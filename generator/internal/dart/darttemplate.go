// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

package dart

import (
	"strings"

	"github.com/googleapis/google-cloud-rust/generator/internal/api"
	"github.com/googleapis/google-cloud-rust/generator/internal/language"
	"github.com/googleapis/google-cloud-rust/generator/internal/license"
	"github.com/iancoleman/strcase"
)

type modelAnnotations struct {
	PackageName       string
	SourcePackageName string
	HasServices       bool
	CopyrightYear     string
	BoilerPlate       []string
	DefaultHost       string
	DartPackage       string
	DocLines          []string
}

type serviceAnnotations struct {
	FieldName   string
	StructName  string
	DocLines    []string
	DefaultHost string
}

type messageAnnotation struct {
	Name           string
	QualifiedName  string
	HasNestedTypes bool
	DocLines       []string
	// The FQN is the source specification
	SourceFQN   string
	BasicFields []*api.Field
}

type methodAnnotation struct {
	Name              string
	NameToPascal      string
	NameToCamelCase   string
	DocLines          []string
	PathParams        []*api.Field
	QueryParams       []*api.Field
	BodyAccessor      string
	ServiceStructName string
	OperationInfo     *operationInfo
}

type pathInfoAnnotation struct {
	Method      string
	PathFmt     string
	PathArgs    []string
	HasPathArgs bool
	HasBody     bool
}

type operationInfo struct {
	MetadataType string
	ResponseType string
}

type oneOfAnnotation struct {
	Name     string
	DocLines []string
}

type fieldAnnotation struct {
	Name             string
	DocLines         []string
	FieldType        string
	AsQueryParameter string
}

type enumAnnotation struct {
	Name     string
	DocLines []string
}

type enumValueAnnotation struct {
	DocLines []string
	Name     string
	EnumType string
}

// annotateModel creates a struct used as input for Mustache templates.
// Fields and methods defined in this struct directly correspond to Mustache
// tags. For example, the Mustache tag {{#Services}} uses the
// [Template.Services] field.
func annotateModel(model *api.API, options map[string]string) (*modelAnnotations, error) {
	var (
		sourceSpecificationPackageName string
		packageNameOverride            string
		generationYear                 string
	)

	for key, definition := range options {
		switch {
		case key == "package-name-override":
			packageNameOverride = definition
		case key == "copyright-year":
			generationYear = definition
		}
	}
	validateModel(model, sourceSpecificationPackageName)

	loadWellKnownTypes(model.State)
	for _, e := range model.State.EnumByID {
		annotateEnum(e, model.State)
	}
	for _, m := range model.State.MessageByID {
		annotateMessage(m, model.State)
	}
	for _, s := range model.Services {
		annotateService(s, model.State)
	}
	ann := &modelAnnotations{
		PackageName:       modelPackageName(model, packageNameOverride),
		SourcePackageName: sourceSpecificationPackageName,
		HasServices:       len(model.Services) > 0,
		CopyrightYear:     generationYear,
		BoilerPlate: append(license.LicenseHeaderBulk(),
			"",
			" Code generated by sidekick. DO NOT EDIT."),
		DefaultHost: func() string {
			if len(model.Services) > 0 {
				return model.Services[0].DefaultHost
			}
			return ""
		}(),
		DartPackage: "google_" + modelPackageName(model, packageNameOverride),
		DocLines:    strings.Split(model.Description, "\n"),
	}

	model.Codec = ann
	return ann, nil
}

func annotateService(s *api.Service, state *api.APIState) {
	// Some methods are skipped.
	methods := language.FilterSlice(s.Methods, func(m *api.Method) bool {
		return generateMethod(m)
	})
	for _, m := range methods {
		annotateMethod(m, s, state)
	}
	ann := &serviceAnnotations{
		FieldName:   strcase.ToLowerCamel(s.Name),
		StructName:  s.Name,
		DocLines:    formatDocComments(s.Documentation, state),
		DefaultHost: s.DefaultHost,
	}
	s.Codec = ann
}

func annotateMessage(m *api.Message, state *api.APIState) {
	for _, f := range m.Fields {
		annotateField(f, state)
	}
	for _, f := range m.OneOfs {
		annotateOneOf(f, state)
	}
	m.Codec = &messageAnnotation{
		Name:           messageName(m),
		QualifiedName:  messageName(m),
		HasNestedTypes: language.HasNestedTypes(m),
		DocLines:       formatDocComments(m.Documentation, state),
		SourceFQN:      strings.TrimPrefix(m.ID, "."),
		BasicFields: language.FilterSlice(m.Fields, func(s *api.Field) bool {
			return !s.IsOneOf
		}),
	}
}

func annotateMethod(m *api.Method, s *api.Service, state *api.APIState) {
	pathInfoAnnotation := &pathInfoAnnotation{
		Method:   m.PathInfo.Verb,
		PathFmt:  httpPathFmt(m.PathInfo),
		PathArgs: httpPathArgs(m.PathInfo),
		HasBody:  m.PathInfo.BodyFieldPath != "",
	}
	m.PathInfo.Codec = pathInfoAnnotation
	annotation := &methodAnnotation{
		BodyAccessor:      bodyAccessor(m),
		DocLines:          formatDocComments(m.Documentation, state),
		Name:              strcase.ToCamel(m.Name),
		NameToPascal:      toPascal(m.Name),
		NameToCamelCase:   strcase.ToLowerCamel(m.Name),
		PathParams:        language.PathParams(m, state),
		QueryParams:       language.QueryParams(m, state),
		ServiceStructName: s.Name,
	}
	if m.OperationInfo != nil {
		annotation.OperationInfo = &operationInfo{
			MetadataType: methodInOutTypeName(m.OperationInfo.MetadataTypeID, state),
			ResponseType: methodInOutTypeName(m.OperationInfo.ResponseTypeID, state),
		}
	}
	m.Codec = annotation
}

func annotateOneOf(field *api.OneOf, state *api.APIState) {
	field.Codec = &oneOfAnnotation{
		Name:     toPascal(field.Name),
		DocLines: formatDocComments(field.Documentation, state),
	}
}

func annotateField(field *api.Field, state *api.APIState) {
	field.Codec = &fieldAnnotation{
		Name:      toPascal(field.Name),
		DocLines:  formatDocComments(field.Documentation, state),
		FieldType: fieldType(field, state),
	}
}

func annotateEnum(e *api.Enum, state *api.APIState) {
	for _, ev := range e.Values {
		annotateEnumValue(ev, e, state)
	}
	e.Codec = &enumAnnotation{
		Name:     enumName(e),
		DocLines: formatDocComments(e.Documentation, state),
	}
}

func annotateEnumValue(ev *api.EnumValue, e *api.Enum, state *api.APIState) {
	ev.Codec = &enumValueAnnotation{
		DocLines: formatDocComments(ev.Documentation, state),
		Name:     enumValueName(ev),
		EnumType: enumName(e),
	}
}
