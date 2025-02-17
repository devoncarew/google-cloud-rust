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

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Advisory Notifications API.
///
/// # Service Description
///
/// Service to manage Security and Privacy Notifications.
///
/// # Configuration
///
/// `AdvisoryNotificationsService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `AdvisoryNotificationsService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AdvisoryNotificationsService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct AdvisoryNotificationsService {
    inner: Arc<dyn crate::stubs::dynamic::AdvisoryNotificationsService>,
}

impl AdvisoryNotificationsService {
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
        T: crate::stubs::AdvisoryNotificationsService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::AdvisoryNotificationsService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AdvisoryNotificationsService> {
        crate::transport::AdvisoryNotificationsService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AdvisoryNotificationsService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::AdvisoryNotificationsService::new)
    }

    /// Lists notifications under a given parent.
    pub fn list_notifications(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::advisory_notifications_service::ListNotifications {
        crate::builders::advisory_notifications_service::ListNotifications::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a notification.
    pub fn get_notification(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::advisory_notifications_service::GetNotification {
        crate::builders::advisory_notifications_service::GetNotification::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Get notification settings.
    pub fn get_settings(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::advisory_notifications_service::GetSettings {
        crate::builders::advisory_notifications_service::GetSettings::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Update notification settings.
    pub fn update_settings(
        &self,
        settings: impl Into<crate::model::Settings>,
    ) -> crate::builders::advisory_notifications_service::UpdateSettings {
        crate::builders::advisory_notifications_service::UpdateSettings::new(self.inner.clone())
            .set_settings(settings.into())
    }
}
