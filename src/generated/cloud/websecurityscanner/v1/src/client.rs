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

/// Implements a client for the Web Security Scanner API.
///
/// # Service Description
///
/// Web Security Scanner Service identifies security vulnerabilities in web
/// applications hosted on Google Cloud. It crawls your application, and
/// attempts to exercise as many user inputs and event handlers as possible.
///
/// # Configuration
///
/// `WebSecurityScanner` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `WebSecurityScanner` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `WebSecurityScanner` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct WebSecurityScanner {
    inner: Arc<dyn crate::stubs::dynamic::WebSecurityScanner>,
}

impl WebSecurityScanner {
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
        T: crate::stubs::WebSecurityScanner + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn crate::stubs::dynamic::WebSecurityScanner>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::WebSecurityScanner> {
        crate::transport::WebSecurityScanner::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl crate::stubs::WebSecurityScanner> {
        Self::build_transport(conf)
            .await
            .map(crate::tracing::WebSecurityScanner::new)
    }

    /// Creates a new ScanConfig.
    pub fn create_scan_config(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::CreateScanConfig {
        crate::builders::web_security_scanner::CreateScanConfig::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Deletes an existing ScanConfig and its child resources.
    pub fn delete_scan_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::DeleteScanConfig {
        crate::builders::web_security_scanner::DeleteScanConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a ScanConfig.
    pub fn get_scan_config(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::GetScanConfig {
        crate::builders::web_security_scanner::GetScanConfig::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ScanConfigs under a given project.
    pub fn list_scan_configs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::ListScanConfigs {
        crate::builders::web_security_scanner::ListScanConfigs::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates a ScanConfig. This method support partial update of a ScanConfig.
    pub fn update_scan_config(
        &self,
        scan_config: impl Into<crate::model::ScanConfig>,
    ) -> crate::builders::web_security_scanner::UpdateScanConfig {
        crate::builders::web_security_scanner::UpdateScanConfig::new(self.inner.clone())
            .set_scan_config(scan_config.into())
    }

    /// Start a ScanRun according to the given ScanConfig.
    pub fn start_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::StartScanRun {
        crate::builders::web_security_scanner::StartScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Gets a ScanRun.
    pub fn get_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::GetScanRun {
        crate::builders::web_security_scanner::GetScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists ScanRuns under a given ScanConfig, in descending order of ScanRun
    /// stop time.
    pub fn list_scan_runs(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::ListScanRuns {
        crate::builders::web_security_scanner::ListScanRuns::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Stops a ScanRun. The stopped ScanRun is returned.
    pub fn stop_scan_run(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::StopScanRun {
        crate::builders::web_security_scanner::StopScanRun::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List CrawledUrls under a given ScanRun.
    pub fn list_crawled_urls(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::ListCrawledUrls {
        crate::builders::web_security_scanner::ListCrawledUrls::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets a Finding.
    pub fn get_finding(
        &self,
        name: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::GetFinding {
        crate::builders::web_security_scanner::GetFinding::new(self.inner.clone())
            .set_name(name.into())
    }

    /// List Findings under a given ScanRun.
    pub fn list_findings(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::ListFindings {
        crate::builders::web_security_scanner::ListFindings::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// List all FindingTypeStats under a given ScanRun.
    pub fn list_finding_type_stats(
        &self,
        parent: impl Into<std::string::String>,
    ) -> crate::builders::web_security_scanner::ListFindingTypeStats {
        crate::builders::web_security_scanner::ListFindingTypeStats::new(self.inner.clone())
            .set_parent(parent.into())
    }
}
