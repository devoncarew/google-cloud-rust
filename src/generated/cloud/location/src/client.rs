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
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// An implementation of [crate::stubs::Locations] to make requests with.
///
/// `Locations` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Locations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Locations` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// An abstract interface that provides location-related information for
/// a service. Service-specific metadata is provided through the
/// [Location.metadata][google.cloud.location.Location.metadata] field.
///
/// [google.cloud.location.Location.metadata]: crate::model::Location::metadata
#[derive(Clone, Debug)]
pub struct Locations {
    inner: Arc<dyn crate::stubs::dynamic::Locations>,
}

impl Locations {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: crate::stubs::Locations + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Locations>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Locations> {
        crate::transport::Locations::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Locations> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Locations::new)
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::locations::ListLocations {
        crate::builders::locations::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::locations::GetLocation {
        crate::builders::locations::GetLocation::new(self.inner.clone()).set_name(name.into())
    }
}
