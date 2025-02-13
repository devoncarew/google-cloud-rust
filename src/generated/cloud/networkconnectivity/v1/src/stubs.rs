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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

use gax::error::Error;
use std::sync::Arc;

pub(crate) mod dynamic;

/// Defines the trait used to implement [crate::client::HubService].
///
/// Application developers may need to implement this trait to mock
/// `client::HubService`.  In other use-cases, application developers only
/// use `client::HubService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait HubService: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::HubService::list_hubs].
    fn list_hubs(
        &self,
        _req: crate::model::ListHubsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListHubsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListHubsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::get_hub].
    fn get_hub(
        &self,
        _req: crate::model::GetHubRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Hub>> + Send {
        std::future::ready::<crate::Result<crate::model::Hub>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::HubService::create_hub].
    fn create_hub(
        &self,
        _req: crate::model::CreateHubRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::update_hub].
    fn update_hub(
        &self,
        _req: crate::model::UpdateHubRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::delete_hub].
    fn delete_hub(
        &self,
        _req: crate::model::DeleteHubRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::list_hub_spokes].
    fn list_hub_spokes(
        &self,
        _req: crate::model::ListHubSpokesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListHubSpokesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListHubSpokesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::query_hub_status].
    fn query_hub_status(
        &self,
        _req: crate::model::QueryHubStatusRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::QueryHubStatusResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::QueryHubStatusResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::HubService::list_spokes].
    fn list_spokes(
        &self,
        _req: crate::model::ListSpokesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListSpokesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListSpokesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::get_spoke].
    fn get_spoke(
        &self,
        _req: crate::model::GetSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Spoke>> + Send {
        std::future::ready::<crate::Result<crate::model::Spoke>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::HubService::create_spoke].
    fn create_spoke(
        &self,
        _req: crate::model::CreateSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::update_spoke].
    fn update_spoke(
        &self,
        _req: crate::model::UpdateSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::reject_hub_spoke].
    fn reject_hub_spoke(
        &self,
        _req: crate::model::RejectHubSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::accept_hub_spoke].
    fn accept_hub_spoke(
        &self,
        _req: crate::model::AcceptHubSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::delete_spoke].
    fn delete_spoke(
        &self,
        _req: crate::model::DeleteSpokeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::get_route_table].
    fn get_route_table(
        &self,
        _req: crate::model::GetRouteTableRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::RouteTable>> + Send {
        std::future::ready::<crate::Result<crate::model::RouteTable>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::get_route].
    fn get_route(
        &self,
        _req: crate::model::GetRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Route>> + Send {
        std::future::ready::<crate::Result<crate::model::Route>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::HubService::list_routes].
    fn list_routes(
        &self,
        _req: crate::model::ListRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRoutesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRoutesResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::list_route_tables].
    fn list_route_tables(
        &self,
        _req: crate::model::ListRouteTablesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListRouteTablesResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListRouteTablesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::HubService::get_group].
    fn get_group(
        &self,
        _req: crate::model::GetGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::Group>> + Send {
        std::future::ready::<crate::Result<crate::model::Group>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::HubService::list_groups].
    fn list_groups(
        &self,
        _req: crate::model::ListGroupsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListGroupsResponse>> + Send
    {
        std::future::ready::<crate::Result<crate::model::ListGroupsResponse>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::update_group].
    fn update_group(
        &self,
        _req: crate::model::UpdateGroupRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::HubService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::HubService::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::HubService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::HubService::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::HubService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}

/// Defines the trait used to implement [crate::client::PolicyBasedRoutingService].
///
/// Application developers may need to implement this trait to mock
/// `client::PolicyBasedRoutingService`.  In other use-cases, application developers only
/// use `client::PolicyBasedRoutingService` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait PolicyBasedRoutingService: std::fmt::Debug + Send + Sync {
    /// Implements [crate::client::PolicyBasedRoutingService::list_policy_based_routes].
    fn list_policy_based_routes(
        &self,
        _req: crate::model::ListPolicyBasedRoutesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::ListPolicyBasedRoutesResponse>>
           + Send {
        std::future::ready::<crate::Result<crate::model::ListPolicyBasedRoutesResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::get_policy_based_route].
    fn get_policy_based_route(
        &self,
        _req: crate::model::GetPolicyBasedRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<crate::model::PolicyBasedRoute>> + Send
    {
        std::future::ready::<crate::Result<crate::model::PolicyBasedRoute>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::create_policy_based_route].
    fn create_policy_based_route(
        &self,
        _req: crate::model::CreatePolicyBasedRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::delete_policy_based_route].
    fn delete_policy_based_route(
        &self,
        _req: crate::model::DeletePolicyBasedRouteRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::ListLocationsResponse>> + Send
    {
        std::future::ready::<crate::Result<location::model::ListLocationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<location::model::Location>> + Send {
        std::future::ready::<crate::Result<location::model::Location>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::Policy>> + Send {
        std::future::ready::<crate::Result<iam_v1::model::Policy>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<iam_v1::model::TestIamPermissionsResponse>> + Send
    {
        std::future::ready::<crate::Result<iam_v1::model::TestIamPermissionsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::ListOperationsResponse>>
           + Send {
        std::future::ready::<crate::Result<longrunning::model::ListOperationsResponse>>(Err(
            Error::other("unimplemented"),
        ))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<longrunning::model::Operation>> + Send
    {
        std::future::ready::<crate::Result<longrunning::model::Operation>>(Err(Error::other(
            "unimplemented",
        )))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Implements [crate::client::PolicyBasedRoutingService::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<wkt::Empty>> + Send {
        std::future::ready::<crate::Result<wkt::Empty>>(Err(Error::other("unimplemented")))
    }

    /// Returns the polling policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_policy::PollingPolicy> {
        Arc::new(gax::polling_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
