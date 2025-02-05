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

pub mod quota_controller {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::QuotaController] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::QuotaController>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::QuotaController>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a QuotaController::allocate_quota call.
    #[derive(Clone, Debug)]
    pub struct AllocateQuota(RequestBuilder<crate::model::AllocateQuotaRequest>);

    impl AllocateQuota {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::QuotaController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::AllocateQuotaRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::AllocateQuotaResponse> {
            (*self.0.stub)
                .allocate_quota(self.0.request, self.0.options)
                .await
        }

        /// Sets the value of [service_name][crate::model::AllocateQuotaRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [allocate_operation][crate::model::AllocateQuotaRequest::allocate_operation].
        pub fn set_allocate_operation<
            T: Into<std::option::Option<crate::model::QuotaOperation>>,
        >(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.allocate_operation = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::AllocateQuotaRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for AllocateQuota {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}

pub mod service_controller {
    use crate::Result;
    use std::sync::Arc;

    /// Common implementation for [crate::client::ServiceController] request builders.
    #[derive(Clone, Debug)]
    pub struct RequestBuilder<R: std::default::Default> {
        stub: Arc<dyn crate::stubs::dynamic::ServiceController>,
        request: R,
        options: gax::options::RequestOptions,
    }

    impl<R> RequestBuilder<R>
    where
        R: std::default::Default,
    {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceController>) -> Self {
            Self {
                stub,
                request: R::default(),
                options: gax::options::RequestOptions::default(),
            }
        }
    }

    /// The request builder for a ServiceController::check call.
    #[derive(Clone, Debug)]
    pub struct Check(RequestBuilder<crate::model::CheckRequest>);

    impl Check {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::CheckRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::CheckResponse> {
            (*self.0.stub).check(self.0.request, self.0.options).await
        }

        /// Sets the value of [service_name][crate::model::CheckRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [operation][crate::model::CheckRequest::operation].
        pub fn set_operation<T: Into<std::option::Option<crate::model::Operation>>>(
            mut self,
            v: T,
        ) -> Self {
            self.0.request.operation = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::CheckRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }
    }

    impl gax::options::RequestBuilder for Check {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }

    /// The request builder for a ServiceController::report call.
    #[derive(Clone, Debug)]
    pub struct Report(RequestBuilder<crate::model::ReportRequest>);

    impl Report {
        pub(crate) fn new(stub: Arc<dyn crate::stubs::dynamic::ServiceController>) -> Self {
            Self(RequestBuilder::new(stub))
        }

        /// Sets the full request, replacing any prior values.
        pub fn with_request<V: Into<crate::model::ReportRequest>>(mut self, v: V) -> Self {
            self.0.request = v.into();
            self
        }

        /// Sets all the options, replacing any prior values.
        pub fn with_options<V: Into<gax::options::RequestOptions>>(mut self, v: V) -> Self {
            self.0.options = v.into();
            self
        }

        /// Sends the request.
        pub async fn send(self) -> Result<crate::model::ReportResponse> {
            (*self.0.stub).report(self.0.request, self.0.options).await
        }

        /// Sets the value of [service_name][crate::model::ReportRequest::service_name].
        pub fn set_service_name<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_name = v.into();
            self
        }

        /// Sets the value of [service_config_id][crate::model::ReportRequest::service_config_id].
        pub fn set_service_config_id<T: Into<std::string::String>>(mut self, v: T) -> Self {
            self.0.request.service_config_id = v.into();
            self
        }

        /// Sets the value of [operations][crate::model::ReportRequest::operations].
        pub fn set_operations<T, V>(mut self, v: T) -> Self
        where
            T: std::iter::IntoIterator<Item = V>,
            V: std::convert::Into<crate::model::Operation>,
        {
            use std::iter::Iterator;
            self.0.request.operations = v.into_iter().map(|i| i.into()).collect();
            self
        }
    }

    impl gax::options::RequestBuilder for Report {
        fn request_options(&mut self) -> &mut gax::options::RequestOptions {
            &mut self.0.options
        }
    }
}
