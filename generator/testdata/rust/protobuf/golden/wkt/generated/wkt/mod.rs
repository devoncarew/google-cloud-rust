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

#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::redundant_explicit_links)]
use crate as wkt;

/// `SourceContext` represents information about the source of a
/// protobuf element, like the file in which it is defined.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct SourceContext {

    /// The path-qualified name of the .proto file that contained the associated
    /// protobuf element.  For example: `"google/protobuf/source_context.proto"`.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub file_name: std::string::String,
}

impl SourceContext {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [file_name][crate::SourceContext::file_name].
    pub fn set_file_name<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.file_name = v.into();
        self
    }
}

impl wkt::message::Message for SourceContext {
    fn typename() -> &'static str {
        "type.googleapis.com/google.protobuf.SourceContext"
    }
}
