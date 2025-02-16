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

/// A dyn-compatible, crate-private version of [super::TimeseriesInsightsController].
#[async_trait::async_trait]
pub trait TimeseriesInsightsController: std::fmt::Debug + Send + Sync {
    async fn list_data_sets(
        &self,
        req: crate::model::ListDataSetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDataSetsResponse>;

    async fn create_data_set(
        &self,
        req: crate::model::CreateDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DataSet>;

    async fn delete_data_set(
        &self,
        req: crate::model::DeleteDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty>;

    async fn append_events(
        &self,
        req: crate::model::AppendEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AppendEventsResponse>;

    async fn query_data_set(
        &self,
        req: crate::model::QueryDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryDataSetResponse>;

    async fn evaluate_slice(
        &self,
        req: crate::model::EvaluateSliceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EvaluatedSlice>;

    async fn evaluate_timeseries(
        &self,
        req: crate::model::EvaluateTimeseriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EvaluatedSlice>;
}

/// All implementations of [crate::stubs::TimeseriesInsightsController] also implement [TimeseriesInsightsController].
#[async_trait::async_trait]
impl<T: crate::stubs::TimeseriesInsightsController> TimeseriesInsightsController for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn list_data_sets(
        &self,
        req: crate::model::ListDataSetsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::ListDataSetsResponse> {
        T::list_data_sets(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn create_data_set(
        &self,
        req: crate::model::CreateDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::DataSet> {
        T::create_data_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn delete_data_set(
        &self,
        req: crate::model::DeleteDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<wkt::Empty> {
        T::delete_data_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn append_events(
        &self,
        req: crate::model::AppendEventsRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::AppendEventsResponse> {
        T::append_events(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn query_data_set(
        &self,
        req: crate::model::QueryDataSetRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::QueryDataSetResponse> {
        T::query_data_set(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn evaluate_slice(
        &self,
        req: crate::model::EvaluateSliceRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EvaluatedSlice> {
        T::evaluate_slice(self, req, options).await
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn evaluate_timeseries(
        &self,
        req: crate::model::EvaluateTimeseriesRequest,
        options: gax::options::RequestOptions,
    ) -> crate::Result<crate::model::EvaluatedSlice> {
        T::evaluate_timeseries(self, req, options).await
    }
}
