// Copyright 2024 Google LLC
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

use bytes::Bytes;

/// An error describing a non-2xx HTTP response.
#[derive(Debug, Default, Clone)]
pub struct HttpError {
    status_code: u16,
    payload: Option<Bytes>,
    headers: std::collections::HashMap<String, String>,
}

impl HttpError {
    /// Creates a new [HttpError] with the given status code, payload, and headers.
    pub fn new(
        status_code: u16,
        headers: std::collections::HashMap<String, String>,
        payload: Option<Bytes>,
    ) -> Self {
        Self {
            status_code,
            headers,
            payload,
        }
    }

    /// Returns the status code associated with the HTTP error response.
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Returns a reference to the payload associated with the HTTP error
    /// response.
    pub fn payload(&self) -> Option<&bytes::Bytes> {
        self.payload.as_ref()
    }

    /// Returns a reference to the headers associated with the HTTP error
    /// response.
    pub fn headers(&self) -> &std::collections::HashMap<String, String> {
        &self.headers
    }
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP Error: code={}, headers={:?}",
            self.status_code, self.headers
        )?;
        if let Some(payload) = self.payload() {
            if let Ok(status) = TryInto::<crate::error::rpc::Status>::try_into(payload.clone()) {
                return write!(f, ", payload:\n{:?}", status);
            }
            write!(f, ", payload:\n{:?}", payload)?;
        };
        Ok(())
    }
}

impl std::error::Error for HttpError {}

#[cfg(feature = "sdk_client")]
/// A helpers to convert [reqwest::header::HeaderMap] to [std::collections::HashMap].
pub fn convert_headers(
    header_map: &reqwest::header::HeaderMap,
) -> std::collections::HashMap<String, String> {
    let mut headers = std::collections::HashMap::new();
    for (key, value) in header_map {
        if let Ok(value) = value.to_str() {
            headers.insert(key.to_string(), value.to_string());
        }
    }
    headers
}
