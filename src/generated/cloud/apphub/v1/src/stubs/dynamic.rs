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

use std::sync::Arc;

/// A dyn-compatible, crate-private version of [super::AppHub].
#[async_trait::async_trait]
pub trait AppHub: std::fmt::Debug + Send + Sync {
    async fn lookup_service_project_attachment(
        &self,
        req: crate::model::LookupServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupServiceProjectAttachmentResponse>;

    async fn list_service_project_attachments(
        &self,
        req: crate::model::ListServiceProjectAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServiceProjectAttachmentsResponse>;

    async fn create_service_project_attachment(
        &self,
        req: crate::model::CreateServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_service_project_attachment(
        &self,
        req: crate::model::GetServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServiceProjectAttachment>;

    async fn delete_service_project_attachment(
        &self,
        req: crate::model::DeleteServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn detach_service_project_attachment(
        &self,
        req: crate::model::DetachServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DetachServiceProjectAttachmentResponse>;

    async fn list_discovered_services(
        &self,
        req: crate::model::ListDiscoveredServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDiscoveredServicesResponse>;

    async fn get_discovered_service(
        &self,
        req: crate::model::GetDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoveredService>;

    async fn lookup_discovered_service(
        &self,
        req: crate::model::LookupDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupDiscoveredServiceResponse>;

    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse>;

    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service>;

    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_discovered_workloads(
        &self,
        req: crate::model::ListDiscoveredWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDiscoveredWorkloadsResponse>;

    async fn get_discovered_workload(
        &self,
        req: crate::model::GetDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoveredWorkload>;

    async fn lookup_discovered_workload(
        &self,
        req: crate::model::LookupDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupDiscoveredWorkloadResponse>;

    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListWorkloadsResponse>;

    async fn create_workload(
        &self,
        req: crate::model::CreateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_workload(
        &self,
        req: crate::model::GetWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Workload>;

    async fn update_workload(
        &self,
        req: crate::model::UpdateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_workload(
        &self,
        req: crate::model::DeleteWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_applications(
        &self,
        req: crate::model::ListApplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListApplicationsResponse>;

    async fn create_application(
        &self,
        req: crate::model::CreateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn get_application(
        &self,
        req: crate::model::GetApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Application>;

    async fn update_application(
        &self,
        req: crate::model::UpdateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_application(
        &self,
        req: crate::model::DeleteApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse>;

    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location>;

    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy>;

    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse>;

    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse>;

    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation>;

    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy>;

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy>;
}

/// All implementations of [crate::stubs::AppHub] also implement [AppHub].
#[async_trait::async_trait]
impl<T: crate::stubs::AppHub> AppHub for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn lookup_service_project_attachment(
        &self,
        req: crate::model::LookupServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupServiceProjectAttachmentResponse> {
        T::lookup_service_project_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_service_project_attachments(
        &self,
        req: crate::model::ListServiceProjectAttachmentsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServiceProjectAttachmentsResponse> {
        T::list_service_project_attachments(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_service_project_attachment(
        &self,
        req: crate::model::CreateServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_service_project_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_service_project_attachment(
        &self,
        req: crate::model::GetServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ServiceProjectAttachment> {
        T::get_service_project_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_service_project_attachment(
        &self,
        req: crate::model::DeleteServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_service_project_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn detach_service_project_attachment(
        &self,
        req: crate::model::DetachServiceProjectAttachmentRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DetachServiceProjectAttachmentResponse> {
        T::detach_service_project_attachment(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_discovered_services(
        &self,
        req: crate::model::ListDiscoveredServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDiscoveredServicesResponse> {
        T::list_discovered_services(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_discovered_service(
        &self,
        req: crate::model::GetDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoveredService> {
        T::get_discovered_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn lookup_discovered_service(
        &self,
        req: crate::model::LookupDiscoveredServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupDiscoveredServiceResponse> {
        T::lookup_discovered_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_services(
        &self,
        req: crate::model::ListServicesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListServicesResponse> {
        T::list_services(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_service(
        &self,
        req: crate::model::CreateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_service(
        &self,
        req: crate::model::GetServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Service> {
        T::get_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_service(
        &self,
        req: crate::model::UpdateServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_service(
        &self,
        req: crate::model::DeleteServiceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_service(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_discovered_workloads(
        &self,
        req: crate::model::ListDiscoveredWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDiscoveredWorkloadsResponse> {
        T::list_discovered_workloads(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_discovered_workload(
        &self,
        req: crate::model::GetDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DiscoveredWorkload> {
        T::get_discovered_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn lookup_discovered_workload(
        &self,
        req: crate::model::LookupDiscoveredWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::LookupDiscoveredWorkloadResponse> {
        T::lookup_discovered_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_workloads(
        &self,
        req: crate::model::ListWorkloadsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListWorkloadsResponse> {
        T::list_workloads(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_workload(
        &self,
        req: crate::model::CreateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_workload(
        &self,
        req: crate::model::GetWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Workload> {
        T::get_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_workload(
        &self,
        req: crate::model::UpdateWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_workload(
        &self,
        req: crate::model::DeleteWorkloadRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_workload(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_applications(
        &self,
        req: crate::model::ListApplicationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListApplicationsResponse> {
        T::list_applications(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_application(
        &self,
        req: crate::model::CreateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::create_application(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_application(
        &self,
        req: crate::model::GetApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::Application> {
        T::get_application(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn update_application(
        &self,
        req: crate::model::UpdateApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::update_application(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_application(
        &self,
        req: crate::model::DeleteApplicationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::delete_application(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_locations(
        &self,
        req: location::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::ListLocationsResponse> {
        T::list_locations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_location(
        &self,
        req: location::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<location::model::Location> {
        T::get_location(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: iam_v1::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::set_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: iam_v1::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::Policy> {
        T::get_iam_policy(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: iam_v1::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<iam_v1::model::TestIamPermissionsResponse> {
        T::test_iam_permissions(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn list_operations(
        &self,
        req: longrunning::model::ListOperationsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::ListOperationsResponse> {
        T::list_operations(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_operation(
        &self,
        req: longrunning::model::GetOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<longrunning::model::Operation> {
        T::get_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_operation(
        &self,
        req: longrunning::model::DeleteOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_operation(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn cancel_operation(
        &self,
        req: longrunning::model::CancelOperationRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::cancel_operation(self, req, options).await
    }

    fn get_polling_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        T::get_polling_policy(self, options)
    }

    fn get_polling_backoff_policy(
        &self,
        options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        T::get_polling_backoff_policy(self, options)
    }
}
