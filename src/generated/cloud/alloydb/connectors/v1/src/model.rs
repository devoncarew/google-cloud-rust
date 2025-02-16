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
extern crate bytes;
extern crate serde;
extern crate serde_with;
extern crate std;
extern crate wkt;

/// Message used by AlloyDB connectors to exchange client and connection metadata
/// with the server after a successful TLS handshake. This metadata includes an
/// IAM token, which is used to authenticate users based on their IAM identity.
/// The sole purpose of this message is for the use of AlloyDB connectors.
/// Clients should not rely on this message directly as there can be breaking
/// changes in the future.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct MetadataExchangeRequest {
    /// Optional. Connector information.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub user_agent: std::string::String,

    /// Authentication type.
    pub auth_type: crate::model::metadata_exchange_request::AuthType,

    /// IAM token used for both IAM user authentiation and
    /// `alloydb.instances.connect` permission check.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub oauth2_token: std::string::String,
}

impl MetadataExchangeRequest {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [user_agent][crate::model::MetadataExchangeRequest::user_agent].
    pub fn set_user_agent<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.user_agent = v.into();
        self
    }

    /// Sets the value of [auth_type][crate::model::MetadataExchangeRequest::auth_type].
    pub fn set_auth_type<
        T: std::convert::Into<crate::model::metadata_exchange_request::AuthType>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.auth_type = v.into();
        self
    }

    /// Sets the value of [oauth2_token][crate::model::MetadataExchangeRequest::oauth2_token].
    pub fn set_oauth2_token<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.oauth2_token = v.into();
        self
    }
}

impl wkt::message::Message for MetadataExchangeRequest {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.alloydb.connectors.v1.MetadataExchangeRequest"
    }
}

/// Defines additional types related to MetadataExchangeRequest
pub mod metadata_exchange_request {
    #[allow(unused_imports)]
    use super::*;

    /// AuthType contains all supported authentication types.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct AuthType(std::borrow::Cow<'static, str>);

    impl AuthType {
        /// Creates a new AuthType instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [AuthType](AuthType)
    pub mod auth_type {
        use super::AuthType;

        /// Authentication type is unspecified and DB_NATIVE is used by default
        pub const AUTH_TYPE_UNSPECIFIED: AuthType = AuthType::new("AUTH_TYPE_UNSPECIFIED");

        /// Database native authentication (user/password)
        pub const DB_NATIVE: AuthType = AuthType::new("DB_NATIVE");

        /// Automatic IAM authentication
        pub const AUTO_IAM: AuthType = AuthType::new("AUTO_IAM");
    }

    impl std::convert::From<std::string::String> for AuthType {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }
}

/// Message for response to metadata exchange request. The sole purpose of this
/// message is for the use of AlloyDB connectors. Clients should not rely on this
/// message directly as there can be breaking changes in the future.
#[serde_with::serde_as]
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(default, rename_all = "camelCase")]
#[non_exhaustive]
pub struct MetadataExchangeResponse {
    /// Response code.
    pub response_code: crate::model::metadata_exchange_response::ResponseCode,

    /// Optional. Error message.
    #[serde(skip_serializing_if = "std::string::String::is_empty")]
    pub error: std::string::String,
}

impl MetadataExchangeResponse {
    pub fn new() -> Self {
        std::default::Default::default()
    }

    /// Sets the value of [response_code][crate::model::MetadataExchangeResponse::response_code].
    pub fn set_response_code<
        T: std::convert::Into<crate::model::metadata_exchange_response::ResponseCode>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.response_code = v.into();
        self
    }

    /// Sets the value of [error][crate::model::MetadataExchangeResponse::error].
    pub fn set_error<T: std::convert::Into<std::string::String>>(mut self, v: T) -> Self {
        self.error = v.into();
        self
    }
}

impl wkt::message::Message for MetadataExchangeResponse {
    fn typename() -> &'static str {
        "type.googleapis.com/google.cloud.alloydb.connectors.v1.MetadataExchangeResponse"
    }
}

/// Defines additional types related to MetadataExchangeResponse
pub mod metadata_exchange_response {
    #[allow(unused_imports)]
    use super::*;

    /// Response code.
    #[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
    pub struct ResponseCode(std::borrow::Cow<'static, str>);

    impl ResponseCode {
        /// Creates a new ResponseCode instance.
        pub const fn new(v: &'static str) -> Self {
            Self(std::borrow::Cow::Borrowed(v))
        }

        /// Gets the enum value.
        pub fn value(&self) -> &str {
            &self.0
        }
    }

    /// Useful constants to work with [ResponseCode](ResponseCode)
    pub mod response_code {
        use super::ResponseCode;

        /// Unknown response code
        pub const RESPONSE_CODE_UNSPECIFIED: ResponseCode =
            ResponseCode::new("RESPONSE_CODE_UNSPECIFIED");

        /// Success
        pub const OK: ResponseCode = ResponseCode::new("OK");

        /// Failure
        pub const ERROR: ResponseCode = ResponseCode::new("ERROR");
    }

    impl std::convert::From<std::string::String> for ResponseCode {
        fn from(value: std::string::String) -> Self {
            Self(std::borrow::Cow::Owned(value))
        }
    }
}
