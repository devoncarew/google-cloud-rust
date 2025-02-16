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

pub mod service_health {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::ServiceHealth] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ServiceHealth::list_events call.
    #[derive(Clone, Debug)]
    pub struct ListEvents(RequestBuilder<crate::model::ListEventsRequest>);

    impl ListEvents {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListEventsRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListEventsResponse> {
            (*self.0.stub)
                .list_events(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<crate::model::ListEventsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListEventsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListEventsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListEventsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListEventsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [view][crate::model::ListEventsRequest::view].
        pub fn set_view<T: Into<crate::model::EventView>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListEvents {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::get_event call.
    #[derive(Clone, Debug)]
    pub struct GetEvent(RequestBuilder<crate::model::GetEventRequest>);

    impl GetEvent {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetEventRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::Event> {
            (*self.0.stub)
                .get_event(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetEventRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetEvent {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::list_organization_events call.
    #[derive(Clone, Debug)]
    pub struct ListOrganizationEvents(RequestBuilder<crate::model::ListOrganizationEventsRequest>);

    impl ListOrganizationEvents {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListOrganizationEventsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListOrganizationEventsResponse> {
            (*self.0.stub)
                .list_organization_events(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::ListOrganizationEventsResponse,
            gax::error::Error,
        > {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListOrganizationEventsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListOrganizationEventsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListOrganizationEventsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListOrganizationEventsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [view][crate::model::ListOrganizationEventsRequest::view].
        pub fn set_view<T: Into<crate::model::OrganizationEventView>>(mut self, v: T) -> Self {
            self.0.request.view = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOrganizationEvents {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::get_organization_event call.
    #[derive(Clone, Debug)]
    pub struct GetOrganizationEvent(RequestBuilder<crate::model::GetOrganizationEventRequest>);

    impl GetOrganizationEvent {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetOrganizationEventRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::OrganizationEvent> {
            (*self.0.stub)
                .get_organization_event(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetOrganizationEventRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOrganizationEvent {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::list_organization_impacts call.
    #[derive(Clone, Debug)]
    pub struct ListOrganizationImpacts(
        RequestBuilder<crate::model::ListOrganizationImpactsRequest>,
    );

    impl ListOrganizationImpacts {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ListOrganizationImpactsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ListOrganizationImpactsResponse> {
            (*self.0.stub)
                .list_organization_impacts(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<
            crate::model::ListOrganizationImpactsResponse,
            gax::error::Error,
        > {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [parent][crate::model::ListOrganizationImpactsRequest::parent].
        pub fn set_parent<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.parent = v.into();
            self
        }

        /// Sets the value of [page_size][crate::model::ListOrganizationImpactsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][crate::model::ListOrganizationImpactsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }

        /// Sets the value of [filter][crate::model::ListOrganizationImpactsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListOrganizationImpacts {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::get_organization_impact call.
    #[derive(Clone, Debug)]
    pub struct GetOrganizationImpact(RequestBuilder<crate::model::GetOrganizationImpactRequest>);

    impl GetOrganizationImpact {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::GetOrganizationImpactRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::OrganizationImpact> {
            (*self.0.stub)
                .get_organization_impact(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][crate::model::GetOrganizationImpactRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetOrganizationImpact {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::list_locations call.
    #[derive(Clone, Debug)]
    pub struct ListLocations(RequestBuilder<location::model::ListLocationsRequest>);

    impl ListLocations {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::ListLocationsRequest>>(
            mut self,
            v: V,
        ) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<location::model::ListLocationsResponse> {
            (*self.0.stub)
                .list_locations(self.0.request, self.0.options)
                .await
        }

        /// Streams the responses back.
        #[cfg(feature = "unstable-stream")]
        pub async fn stream(
            self,
        ) -> gax::paginator::Paginator<location::model::ListLocationsResponse, gax::error::Error>
        {
            let token = gax::paginator::extract_token(&self.0.request.page_token);
            let execute = move |token: String| {
                let mut builder = self.clone();
                builder.0.request = builder.0.request.set_page_token(token);
                builder.send()
            };
            gax::paginator::Paginator::new(token, execute)
        }

        /// Sets the value of [name][location::model::ListLocationsRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }

        /// Sets the value of [filter][location::model::ListLocationsRequest::filter].
        pub fn set_filter<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.filter = v.into();
            self
        }

        /// Sets the value of [page_size][location::model::ListLocationsRequest::page_size].
        pub fn set_page_size<T: Into<i32>>(mut self, v: T) -> Self {
            self.0.request.page_size = v.into();
            self
        }

        /// Sets the value of [page_token][location::model::ListLocationsRequest::page_token].
        pub fn set_page_token<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.page_token = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for ListLocations {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceHealth::get_location call.
    #[derive(Clone, Debug)]
    pub struct GetLocation(RequestBuilder<location::model::GetLocationRequest>);

    impl GetLocation {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceHealth>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<location::model::GetLocationRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<location::model::Location> {
            (*self.0.stub)
                .get_location(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [name][location::model::GetLocationRequest::name].
        pub fn set_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.name = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for GetLocation {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
