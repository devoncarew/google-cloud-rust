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

/// An implementation of [crate::stubs::AccountManagementService] to make requests with.
///
/// `AccountManagementService` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `AccountManagementService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AccountManagementService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Account management for Identity Toolkit
#[derive(Clone, Debug)]
pub struct AccountManagementService {
    inner: Arc<dyn crate::stubs::dynamic::AccountManagementService>,
}

impl AccountManagementService {
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
        T: crate::stubs::AccountManagementService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::AccountManagementService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AccountManagementService> {
        crate::transport::AccountManagementService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AccountManagementService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::AccountManagementService::new)
    }

    /// Finishes enrolling a second factor for the user.
    pub fn finalize_mfa_enrollment(
        &self,
    ) -> crate::builders::account_management_service::FinalizeMfaEnrollment {
        crate::builders::account_management_service::FinalizeMfaEnrollment::new(self.inner.clone())
    }

    /// Step one of the MFA enrollment process. In SMS case, this sends an
    /// SMS verification code to the user.
    pub fn start_mfa_enrollment(
        &self,
    ) -> crate::builders::account_management_service::StartMfaEnrollment {
        crate::builders::account_management_service::StartMfaEnrollment::new(self.inner.clone())
    }

    /// Revokes one second factor from the enrolled second factors for an account.
    pub fn withdraw_mfa(&self) -> crate::builders::account_management_service::WithdrawMfa {
        crate::builders::account_management_service::WithdrawMfa::new(self.inner.clone())
    }
}

/// An implementation of [crate::stubs::AuthenticationService] to make requests with.
///
/// `AuthenticationService` has various configuration parameters, but the defaults
/// are set to work with most applications.
///
/// `AuthenticationService` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `AuthenticationService` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
///
/// Authentication for Identity Toolkit
#[derive(Clone, Debug)]
pub struct AuthenticationService {
    inner: Arc<dyn crate::stubs::dynamic::AuthenticationService>,
}

impl AuthenticationService {
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
        T: crate::stubs::AuthenticationService + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::AuthenticationService>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AuthenticationService> {
        crate::transport::AuthenticationService::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::AuthenticationService> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::AuthenticationService::new)
    }

    /// Verifies the MFA challenge and performs sign-in
    pub fn finalize_mfa_sign_in(
        &self,
    ) -> crate::builders::authentication_service::FinalizeMfaSignIn {
        crate::builders::authentication_service::FinalizeMfaSignIn::new(self.inner.clone())
    }

    /// Sends the MFA challenge
    pub fn start_mfa_sign_in(&self) -> crate::builders::authentication_service::StartMfaSignIn {
        crate::builders::authentication_service::StartMfaSignIn::new(self.inner.clone())
    }
}
