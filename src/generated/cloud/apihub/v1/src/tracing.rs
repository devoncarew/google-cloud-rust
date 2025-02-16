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
use crate::Result;

/// Implements a [ApiHub](crate::stubs::ApiHub) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ApiHub<T>
where
    T: crate::stubs::ApiHub + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ApiHub<T>
where
    T: crate::stubs::ApiHub + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::ApiHub for ApiHub<T>
where
    T: crate::stubs::ApiHub + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_api(
        &self,
        req: crate::model::CreateApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Api> {
        self.inner.create_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_api(
        &self,
        req: crate::model::GetApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Api> {
        self.inner.get_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_apis(
        &self,
        req: crate::model::ListApisRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListApisResponse> {
        self.inner.list_apis(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_api(
        &self,
        req: crate::model::UpdateApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Api> {
        self.inner.update_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_api(
        &self,
        req: crate::model::DeleteApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_version(
        &self,
        req: crate::model::CreateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Version> {
        self.inner.create_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_version(
        &self,
        req: crate::model::GetVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Version> {
        self.inner.get_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_versions(
        &self,
        req: crate::model::ListVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListVersionsResponse> {
        self.inner.list_versions(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_version(
        &self,
        req: crate::model::UpdateVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Version> {
        self.inner.update_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_version(
        &self,
        req: crate::model::DeleteVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_version(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_spec(
        &self,
        req: crate::model::CreateSpecRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Spec> {
        self.inner.create_spec(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_spec(
        &self,
        req: crate::model::GetSpecRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Spec> {
        self.inner.get_spec(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_spec_contents(
        &self,
        req: crate::model::GetSpecContentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SpecContents> {
        self.inner.get_spec_contents(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_specs(
        &self,
        req: crate::model::ListSpecsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSpecsResponse> {
        self.inner.list_specs(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_spec(
        &self,
        req: crate::model::UpdateSpecRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Spec> {
        self.inner.update_spec(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_spec(
        &self,
        req: crate::model::DeleteSpecRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_spec(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_api_operation(
        &self,
        req: crate::model::GetApiOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ApiOperation> {
        self.inner.get_api_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_api_operations(
        &self,
        req: crate::model::ListApiOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListApiOperationsResponse> {
        self.inner.list_api_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_definition(
        &self,
        req: crate::model::GetDefinitionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Definition> {
        self.inner.get_definition(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_deployment(
        &self,
        req: crate::model::CreateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.create_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_deployment(
        &self,
        req: crate::model::GetDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.get_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_deployments(
        &self,
        req: crate::model::ListDeploymentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDeploymentsResponse> {
        self.inner.list_deployments(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_deployment(
        &self,
        req: crate::model::UpdateDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Deployment> {
        self.inner.update_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_deployment(
        &self,
        req: crate::model::DeleteDeploymentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_deployment(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_attribute(
        &self,
        req: crate::model::CreateAttributeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attribute> {
        self.inner.create_attribute(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_attribute(
        &self,
        req: crate::model::GetAttributeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attribute> {
        self.inner.get_attribute(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_attribute(
        &self,
        req: crate::model::UpdateAttributeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Attribute> {
        self.inner.update_attribute(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_attribute(
        &self,
        req: crate::model::DeleteAttributeRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_attribute(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_attributes(
        &self,
        req: crate::model::ListAttributesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListAttributesResponse> {
        self.inner.list_attributes(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn search_resources(
        &self,
        req: crate::model::SearchResourcesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SearchResourcesResponse> {
        self.inner.search_resources(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn create_external_api(
        &self,
        req: crate::model::CreateExternalApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalApi> {
        self.inner.create_external_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_external_api(
        &self,
        req: crate::model::GetExternalApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalApi> {
        self.inner.get_external_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_external_api(
        &self,
        req: crate::model::UpdateExternalApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ExternalApi> {
        self.inner.update_external_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_external_api(
        &self,
        req: crate::model::DeleteExternalApiRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_external_api(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_external_apis(
        &self,
        req: crate::model::ListExternalApisRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListExternalApisResponse> {
        self.inner.list_external_apis(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [ApiHubDependencies](crate::stubs::ApiHubDependencies) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ApiHubDependencies<T>
where
    T: crate::stubs::ApiHubDependencies + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ApiHubDependencies<T>
where
    T: crate::stubs::ApiHubDependencies + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::ApiHubDependencies for ApiHubDependencies<T>
where
    T: crate::stubs::ApiHubDependencies + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_dependency(
        &self,
        req: crate::model::CreateDependencyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dependency> {
        self.inner.create_dependency(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_dependency(
        &self,
        req: crate::model::GetDependencyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dependency> {
        self.inner.get_dependency(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_dependency(
        &self,
        req: crate::model::UpdateDependencyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Dependency> {
        self.inner.update_dependency(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_dependency(
        &self,
        req: crate::model::DeleteDependencyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_dependency(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_dependencies(
        &self,
        req: crate::model::ListDependenciesRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListDependenciesResponse> {
        self.inner.list_dependencies(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [HostProjectRegistrationService](crate::stubs::HostProjectRegistrationService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct HostProjectRegistrationService<T>
where
    T: crate::stubs::HostProjectRegistrationService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> HostProjectRegistrationService<T>
where
    T: crate::stubs::HostProjectRegistrationService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::HostProjectRegistrationService for HostProjectRegistrationService<T>
where
    T: crate::stubs::HostProjectRegistrationService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_host_project_registration(
        &self,
        req: crate::model::CreateHostProjectRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HostProjectRegistration> {
        self.inner
            .create_host_project_registration(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_host_project_registration(
        &self,
        req: crate::model::GetHostProjectRegistrationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::HostProjectRegistration> {
        self.inner.get_host_project_registration(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_host_project_registrations(
        &self,
        req: crate::model::ListHostProjectRegistrationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListHostProjectRegistrationsResponse> {
        self.inner
            .list_host_project_registrations(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [LintingService](crate::stubs::LintingService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct LintingService<T>
where
    T: crate::stubs::LintingService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> LintingService<T>
where
    T: crate::stubs::LintingService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::LintingService for LintingService<T>
where
    T: crate::stubs::LintingService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_style_guide(
        &self,
        req: crate::model::GetStyleGuideRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StyleGuide> {
        self.inner.get_style_guide(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn update_style_guide(
        &self,
        req: crate::model::UpdateStyleGuideRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StyleGuide> {
        self.inner.update_style_guide(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_style_guide_contents(
        &self,
        req: crate::model::GetStyleGuideContentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::StyleGuideContents> {
        self.inner.get_style_guide_contents(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lint_spec(
        &self,
        req: crate::model::LintSpecRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.lint_spec(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [ApiHubPlugin](crate::stubs::ApiHubPlugin) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct ApiHubPlugin<T>
where
    T: crate::stubs::ApiHubPlugin + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> ApiHubPlugin<T>
where
    T: crate::stubs::ApiHubPlugin + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::ApiHubPlugin for ApiHubPlugin<T>
where
    T: crate::stubs::ApiHubPlugin + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn get_plugin(
        &self,
        req: crate::model::GetPluginRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Plugin> {
        self.inner.get_plugin(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn enable_plugin(
        &self,
        req: crate::model::EnablePluginRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Plugin> {
        self.inner.enable_plugin(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn disable_plugin(
        &self,
        req: crate::model::DisablePluginRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Plugin> {
        self.inner.disable_plugin(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}

/// Implements a [Provisioning](crate::stubs::Provisioning) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct Provisioning<T>
where
    T: crate::stubs::Provisioning + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> Provisioning<T>
where
    T: crate::stubs::Provisioning + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::Provisioning for Provisioning<T>
where
    T: crate::stubs::Provisioning + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_api_hub_instance(
        &self,
        req: crate::model::CreateApiHubInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.create_api_hub_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_api_hub_instance(
        &self,
        req: crate::model::GetApiHubInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ApiHubInstance> {
        self.inner.get_api_hub_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn lookup_api_hub_instance(
        &self,
        req: crate::model::LookupApiHubInstanceRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LookupApiHubInstanceResponse> {
        self.inner.lookup_api_hub_instance(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_policy::PollingPolicy> {
        self.inner.get_polling_policy(options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        self.inner.get_polling_backoff_policy(options)
    }
}

/// Implements a [RuntimeProjectAttachmentService](crate::stubs::RuntimeProjectAttachmentService) decorator for logging and tracing.
#[derive(Clone, Debug)]
pub struct RuntimeProjectAttachmentService<T>
where
    T: crate::stubs::RuntimeProjectAttachmentService + std::fmt::Debug + Send + Sync,
{
    inner: T,
}

impl<T> RuntimeProjectAttachmentService<T>
where
    T: crate::stubs::RuntimeProjectAttachmentService + std::fmt::Debug + Send + Sync,
{
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> crate::stubs::RuntimeProjectAttachmentService for RuntimeProjectAttachmentService<T>
where
    T: crate::stubs::RuntimeProjectAttachmentService + std::fmt::Debug + Send + Sync,
{
    #[tracing::instrument(ret)]
    async fn create_runtime_project_attachment(
        &self,
        req: crate::model::CreateRuntimeProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RuntimeProjectAttachment> {
        self.inner
            .create_runtime_project_attachment(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn get_runtime_project_attachment(
        &self,
        req: crate::model::GetRuntimeProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::RuntimeProjectAttachment> {
        self.inner
            .get_runtime_project_attachment(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_runtime_project_attachments(
        &self,
        req: crate::model::ListRuntimeProjectAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListRuntimeProjectAttachmentsResponse> {
        self.inner
            .list_runtime_project_attachments(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn delete_runtime_project_attachment(
        &self,
        req: crate::model::DeleteRuntimeProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner
            .delete_runtime_project_attachment(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn lookup_runtime_project_attachment(
        &self,
        req: crate::model::LookupRuntimeProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::LookupRuntimeProjectAttachmentResponse> {
        self.inner
            .lookup_runtime_project_attachment(req, options)
            .await
    }

    #[tracing::instrument(ret)]
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::ListLocationsResponse> {
        self.inner.list_locations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<location::model::Location> {
        self.inner.get_location(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::ListOperationsResponse> {
        self.inner.list_operations(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<longrunning::model::Operation> {
        self.inner.get_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.delete_operation(req, options).await
    }

    #[tracing::instrument(ret)]
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<wkt::Empty> {
        self.inner.cancel_operation(req, options).await
    }
}
