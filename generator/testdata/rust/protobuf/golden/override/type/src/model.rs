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
//
// Code generated by sidekick. DO NOT EDIT.

#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]
#![no_implicit_prelude]
extern crate std;

/// Represents a textual expression in the Common Expression Language (CEL)
/// syntax. CEL is a C-like expression language. The syntax and semantics of CEL
/// are documented at <https://github.com/google/cel-spec>.
///
/// Example (Comparison):
///
/// ```norust
/// title: "Summary size limit"
/// description: "Determines if a summary is less than 100 chars"
/// expression: "document.summary.size() < 100"
/// ```
///
/// Example (Equality):
///
/// ```norust
/// title: "Requestor is owner"
/// description: "Determines if requestor is the document owner"
/// expression: "document.owner == request.auth.claims.email"
/// ```
///
/// Example (Logic):
///
/// ```norust
/// title: "Public documents"
/// description: "Determine whether the document should be publicly visible"
/// expression: "document.type != 'private' && document.type != 'internal'"
/// ```
///
/// Example (Data Manipulation):
///
/// ```norust
/// title: "Notification string"
/// description: "Create a notification string with a timestamp."
/// expression: "'New message received at ' + string(document.create_time)"
/// ```
///
/// The exact variables and functions that may be referenced within an expression
/// are determined by the service that evaluates it. See the service
/// documentation for additional information.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct Expr {

    /// Textual representation of an expression in Common Expression Language
    /// syntax.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub expression: std::string::String,

    /// Optional. Title for the expression, i.e. a short string describing
    /// its purpose. This can be used e.g. in UIs which allow to enter the
    /// expression.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub title: std::string::String,

    /// Optional. Description of the expression. This is a longer text which
    /// describes the expression, e.g. when hovered over it in a UI.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub description: std::string::String,

    /// Optional. String indicating the location of the expression for error
    /// reporting, e.g. a file name and a position in the file.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub location: std::string::String,
}

impl Expr {

    /// Sets the value of [expression][crate::model::Expr::expression].
    pub fn set_expression<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.expression = v.into();
        self
    }

    /// Sets the value of [title][crate::model::Expr::title].
    pub fn set_title<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.title = v.into();
        self
    }

    /// Sets the value of [description][crate::model::Expr::description].
    pub fn set_description<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }

    /// Sets the value of [location][crate::model::Expr::location].
    pub fn set_location<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

impl wkt::message::Message for Expr {
    fn typename() -> &'static str {
        "type.googleapis.com/google.type.Expr"
    }
}
