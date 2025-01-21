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
#[allow(unused_imports)]
use gax::error::Error;

/// Implements [SecretManagerService](crate::traits::SecretManagerService) using a [gax::http_client::ReqwestClient].
#[derive(Clone)]
pub struct SecretManagerService {
    inner: gax::http_client::ReqwestClient,
}

impl std::fmt::Debug for SecretManagerService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        f.debug_struct("SecretManagerService")
            .field("inner", &self.inner)
            .finish()
    }
}

impl SecretManagerService {
    pub async fn new(config: gax::http_client::ClientConfig) -> Result<Self> {
        let inner = gax::http_client::ReqwestClient::new(config, crate::DEFAULT_HOST).await?;
        Ok(Self { inner })
    }
}

impl crate::traits::SecretManagerService for SecretManagerService {
    async fn list_locations(
        &self,
        req: crate::model::ListLocationsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListLocationsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/locations", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .filter
            .iter()
            .fold(builder, |builder, p| builder.query(&[("filter", p)]));
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = req
            .page_token
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageToken", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn get_location(
        &self,
        req: crate::model::GetLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Location> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/locations/{}", req.project, req.location),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn list_secrets(
        &self,
        req: crate::model::ListSecretsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/secrets", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = req
            .page_token
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageToken", p)]));
        let builder = req
            .filter
            .iter()
            .fold(builder, |builder, p| builder.query(&[("filter", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn create_secret(
        &self,
        req: crate::model::CreateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!("/v1/projects/{}/secrets", req.project),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("secretId", &req.secret_id)]);
        self.inner
            .execute(builder, Some(req.request_body), options)
            .await
    }

    async fn list_secrets_by_project_and_location(
        &self,
        req: crate::model::ListSecretsByProjectAndLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets",
                    req.project, req.location
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = req
            .page_token
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageToken", p)]));
        let builder = req
            .filter
            .iter()
            .fold(builder, |builder, p| builder.query(&[("filter", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn create_secret_by_project_and_location(
        &self,
        req: crate::model::CreateSecretByProjectAndLocationRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets",
                    req.project, req.location
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = builder.query(&[("secretId", &req.secret_id)]);
        self.inner
            .execute(builder, Some(req.request_body), options)
            .await
    }

    async fn add_secret_version(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:addVersion",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn add_secret_version_by_project_and_location_and_secret(
        &self,
        req: crate::model::AddSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:addVersion",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_secret(
        &self,
        req: crate::model::GetSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn delete_secret(
        &self,
        req: crate::model::DeleteSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .etag
            .iter()
            .fold(builder, |builder, p| builder.query(&[("etag", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn update_secret(
        &self,
        req: crate::model::UpdateSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!("/v1/projects/{}/secrets/{}", req.project, req.secret),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = {
            use gax::query_parameter::QueryParameter;
            serde_json::to_value(&req.update_mask)
                .map_err(Error::serde)?
                .add(builder, "updateMask")
        };
        self.inner
            .execute(builder, Some(req.request_body), options)
            .await
    }

    async fn get_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn delete_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::DeleteSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Empty> {
        let options = options.set_default_idempotency(reqwest::Method::DELETE.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::DELETE,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .etag
            .iter()
            .fold(builder, |builder, p| builder.query(&[("etag", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn update_secret_by_project_and_location_and_secret(
        &self,
        req: crate::model::UpdateSecretByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Secret> {
        let options = options.set_default_idempotency(reqwest::Method::PATCH.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::PATCH,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = {
            use gax::query_parameter::QueryParameter;
            serde_json::to_value(&req.update_mask)
                .map_err(Error::serde)?
                .add(builder, "updateMask")
        };
        self.inner
            .execute(builder, Some(req.request_body), options)
            .await
    }

    async fn list_secret_versions(
        &self,
        req: crate::model::ListSecretVersionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = req
            .page_token
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageToken", p)]));
        let builder = req
            .filter
            .iter()
            .fold(builder, |builder, p| builder.query(&[("filter", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn list_secret_versions_by_project_and_location_and_secret(
        &self,
        req: crate::model::ListSecretVersionsByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::ListSecretVersionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .page_size
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageSize", p)]));
        let builder = req
            .page_token
            .iter()
            .fold(builder, |builder, p| builder.query(&[("pageToken", p)]));
        let builder = req
            .filter
            .iter()
            .fold(builder, |builder, p| builder.query(&[("filter", p)]));
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn get_secret_version(
        &self,
        req: crate::model::GetSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn get_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn access_secret_version(
        &self,
        req: crate::model::AccessSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:access",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn access_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::AccessSecretVersionResponse> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:access",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn disable_secret_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:disable",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn disable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DisableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:disable",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn enable_secret_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:enable",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn enable_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::EnableSecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:enable",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn destroy_secret_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}/versions/{}:destroy",
                    req.project, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn destroy_secret_version_by_project_and_location_and_secret_and_version(
        &self,
        req: crate::model::DestroySecretVersionRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::SecretVersion> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}/versions/{}:destroy",
                    req.project, req.location, req.secret, req.version
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:setIamPolicy",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn set_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::SetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:setIamPolicy",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/secrets/{}:getIamPolicy",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .options_requested_policy_version
            .iter()
            .fold(builder, |builder, p| {
                builder.query(&[("options.requestedPolicyVersion", p)])
            });
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn get_iam_policy_by_project_and_location_and_secret(
        &self,
        req: crate::model::GetIamPolicyByProjectAndLocationAndSecretRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::Policy> {
        let options = options.set_default_idempotency(reqwest::Method::GET.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::GET,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:getIamPolicy",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        let builder = req
            .options_requested_policy_version
            .iter()
            .fold(builder, |builder, p| {
                builder.query(&[("options.requestedPolicyVersion", p)])
            });
        self.inner
            .execute(builder, None::<gax::http_client::NoBody>, options)
            .await
    }

    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/secrets/{}:testIamPermissions",
                    req.project, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }

    async fn test_iam_permissions_by_project_and_location_and_secret(
        &self,
        req: crate::model::TestIamPermissionsRequest,
        options: gax::options::RequestOptions,
    ) -> Result<crate::model::TestIamPermissionsResponse> {
        let options = options.set_default_idempotency(reqwest::Method::POST.is_idempotent());
        let builder = self
            .inner
            .builder(
                reqwest::Method::POST,
                format!(
                    "/v1/projects/{}/locations/{}/secrets/{}:testIamPermissions",
                    req.project, req.location, req.secret
                ),
            )
            .query(&[("alt", "json")])
            .header(
                "x-goog-api-client",
                reqwest::header::HeaderValue::from_static(&crate::info::X_GOOG_API_CLIENT_HEADER),
            );
        self.inner.execute(builder, Some(req), options).await
    }
}
