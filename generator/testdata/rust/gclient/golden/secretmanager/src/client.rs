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

use crate::Result;
use std::sync::Arc;

/// An implementation of [crate::traits::SecretManagerService] to make requests with.
///
/// `SecretManagerService` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `SecretManagerService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `SecretManagerService` in
/// an [Rc](std::sync::Rc) or [Arc](std::async::Arc) to reuse it, because it
/// already uses an `Arc` internally.
///
/// Secret Manager Service
///
/// Manages secrets and operations using those secrets. Implements a REST
/// model with the following objects:
///
/// * [Secret][google.cloud.secretmanager.v1.Secret]
/// * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
#[derive(Clone)]
pub struct SecretManagerService {
    inner: Arc<dyn crate::traits::dyntraits::SecretManagerService>,
}

impl SecretManagerService {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        Ok(Self { 
            inner: Arc::new(crate::transport::SecretManagerService::new(conf).await?)
        })
    }
}

impl crate::traits::SecretManagerService for SecretManagerService {
    /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
    async fn list_secrets(&self, req: crate::model::ListSecretsRequest) -> Result<crate::model::ListSecretsResponse> {
        self.inner.list_secrets(req).await
    }

    /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
    async fn create_secret(&self, req: crate::model::CreateSecretRequest) -> Result<crate::model::Secret> {
        self.inner.create_secret(req).await
    }

    /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
    /// containing secret data and attaches it to an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn add_secret_version(&self, req: crate::model::AddSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.add_secret_version(req).await
    }

    /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
    async fn get_secret(&self, req: crate::model::GetSecretRequest) -> Result<crate::model::Secret> {
        self.inner.get_secret(req).await
    }

    /// Updates metadata of an existing
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn update_secret(&self, req: crate::model::UpdateSecretRequest) -> Result<crate::model::Secret> {
        self.inner.update_secret(req).await
    }

    /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
    async fn delete_secret(&self, req: crate::model::DeleteSecretRequest) -> Result<wkt::Empty> {
        self.inner.delete_secret(req).await
    }

    /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This
    /// call does not return secret data.
    async fn list_secret_versions(&self, req: crate::model::ListSecretVersionsRequest) -> Result<crate::model::ListSecretVersionsResponse> {
        self.inner.list_secret_versions(req).await
    }

    /// Gets metadata for a
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    async fn get_secret_version(&self, req: crate::model::GetSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.get_secret_version(req).await
    }

    /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    /// This call returns the secret data.
    ///
    /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
    /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    async fn access_secret_version(&self, req: crate::model::AccessSecretVersionRequest) -> Result<crate::model::AccessSecretVersionResponse> {
        self.inner.access_secret_version(req).await
    }

    /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
    async fn disable_secret_version(&self, req: crate::model::DisableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.disable_secret_version(req).await
    }

    /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
    async fn enable_secret_version(&self, req: crate::model::EnableSecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.enable_secret_version(req).await
    }

    /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
    ///
    /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
    /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
    /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]
    /// and irrevocably destroys the secret data.
    async fn destroy_secret_version(&self, req: crate::model::DestroySecretVersionRequest) -> Result<crate::model::SecretVersion> {
        self.inner.destroy_secret_version(req).await
    }

    /// Sets the access control policy on the specified secret. Replaces any
    /// existing policy.
    ///
    /// Permissions on
    /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced
    /// according to the policy set on the associated
    /// [Secret][google.cloud.secretmanager.v1.Secret].
    async fn set_iam_policy(&self, req: iam::model::SetIamPolicyRequest) -> Result<iam::model::Policy> {
        self.inner.set_iam_policy(req).await
    }

    /// Gets the access control policy for a secret.
    /// Returns empty policy if the secret exists and does not have a policy set.
    async fn get_iam_policy(&self, req: iam::model::GetIamPolicyRequest) -> Result<iam::model::Policy> {
        self.inner.get_iam_policy(req).await
    }

    /// Returns permissions that a caller has for the specified secret.
    /// If the secret does not exist, this call returns an empty set of
    /// permissions, not a NOT_FOUND error.
    ///
    /// Note: This operation is designed to be used for building permission-aware
    /// UIs and command-line tools, not for authorization checking. This operation
    /// may "fail open" without warning.
    async fn test_iam_permissions(&self, req: iam::model::TestIamPermissionsRequest) -> Result<iam::model::TestIamPermissionsResponse> {
        self.inner.test_iam_permissions(req).await
    }

}

/// An implementation of [crate::traits::Locations] to make requests with.
///
/// `Locations` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `Locations` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `Locations` in
/// an [Rc](std::sync::Rc) or [Arc](std::async::Arc) to reuse it, because it
/// already uses an `Arc` internally.
///
/// Manages location-related information with an API service.
#[derive(Clone)]
pub struct Locations {
    inner: Arc<dyn crate::traits::dyntraits::Locations>,
}

impl Locations {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(crate::ConfigBuilder::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: crate::ConfigBuilder) -> Result<Self> {
        Ok(Self { 
            inner: Arc::new(crate::transport::Locations::new(conf).await?)
        })
    }
}

impl crate::traits::Locations for Locations {
    /// Lists information about the supported locations for this service.
    async fn list_locations(&self, req: location::model::ListLocationsRequest) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req).await
    }

    /// Gets information about a location.
    async fn get_location(&self, req: location::model::GetLocationRequest) -> Result<location::model::Location> {
        self.inner.get_location(req).await
    }

}
