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

/// Implements a client for the BigQuery Migration API.
///
/// # Service Description
///
/// Service to handle EDW migrations.
///
/// # Configuration
///
/// `MigrationService` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `MigrationService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `MigrationService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct MigrationService {
    inner: Arc<dyn crate::stubs::dynamic::MigrationService>,
}

impl MigrationService {
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
        T: crate::stubs::MigrationService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::MigrationService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MigrationService> {
        crate::transport::MigrationService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::MigrationService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::MigrationService::new)
    }

    /// Creates a migration workflow.
    pub fn create_migration_workflow(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::CreateMigrationWorkflow {
        crate::builders::migration_service::CreateMigrationWorkflow::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a previously created migration workflow.
    pub fn get_migration_workflow(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::GetMigrationWorkflow {
        crate::builders::migration_service::GetMigrationWorkflow::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists previously created migration workflow.
    pub fn list_migration_workflows(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::ListMigrationWorkflows {
        crate::builders::migration_service::ListMigrationWorkflows::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes a migration workflow by name.
    pub fn delete_migration_workflow(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::DeleteMigrationWorkflow {
        crate::builders::migration_service::DeleteMigrationWorkflow::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Starts a previously created migration workflow. I.e., the state transitions
    /// from DRAFT to RUNNING. This is a no-op if the state is already RUNNING.
    /// An error will be signaled if the state is anything other than DRAFT or
    /// RUNNING.
    pub fn start_migration_workflow(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::StartMigrationWorkflow {
        crate::builders::migration_service::StartMigrationWorkflow::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a previously created migration subtask.
    pub fn get_migration_subtask(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::GetMigrationSubtask {
        crate::builders::migration_service::GetMigrationSubtask::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists previously created migration subtasks.
    pub fn list_migration_subtasks(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::migration_service::ListMigrationSubtasks {
        crate::builders::migration_service::ListMigrationSubtasks::new(self.inner.clone())
            .set_parent(parent.into())
    }
}
