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
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Identity and Access Management (IAM) API.
///
/// # Service Description
///
/// An interface for managing Identity and Access Management (IAM) policies.
///
/// # Configuration
///
/// `Policies` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `Policies` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Policies` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct Policies {
    inner: Arc<dyn crate::stubs::dynamic::Policies>,
}

impl Policies {
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
        T: crate::stubs::Policies + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::Policies>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Policies> {
        crate::transport::Policies::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::Policies> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::Policies::new)
    }

    /// Retrieves the policies of the specified kind that are attached to a
    /// resource.
    ///
    /// The response lists only policy metadata. In particular, policy rules are
    /// omitted.
    pub fn list_policies(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::policies::ListPolicies {
        crate::builders::policies::ListPolicies::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Gets a policy.
    pub fn get_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::policies::GetPolicy {
        crate::builders::policies::GetPolicy::new(self.inner.clone()).set_name(name.into())
    }

    /// Creates a policy.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_policy(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::policies::CreatePolicy {
        crate::builders::policies::CreatePolicy::new(self.inner.clone()).set_parent(parent.into())
    }

    /// Updates the specified policy.
    ///
    /// You can update only the rules and the display name for the policy.
    ///
    /// To update a policy, you should use a read-modify-write loop:
    ///
    /// . Use [GetPolicy][google.iam.v2.Policies.GetPolicy] to read the current version of the policy.
    /// . Modify the policy as needed.
    /// . Use `UpdatePolicy` to write the updated policy.
    ///
    /// This pattern helps prevent conflicts between concurrent updates.
    ///
    /// [google.iam.v2.Policies.GetPolicy]: crate::client::Policies::get_policy
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_policy(
        &self,
        policy: impl Into<crate::model::Policy>,
    ) -> crate::builders::policies::UpdatePolicy {
        crate::builders::policies::UpdatePolicy::new(self.inner.clone()).set_policy(policy.into())
    }

    /// Deletes a policy. This action is permanent.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_policy(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::policies::DeletePolicy {
        crate::builders::policies::DeletePolicy::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::policies::GetOperation {
        crate::builders::policies::GetOperation::new(self.inner.clone()).set_name(name.into())
    }
}
