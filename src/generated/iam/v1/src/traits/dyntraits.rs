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

/// A dyn-compatible, crate-private version of `Iampolicy`.
#[async_trait::async_trait]
pub trait Iampolicy: Send + Sync {
    /// Sets the access control policy on the specified resource. Replaces
    /// any existing policy.
    ///
    /// Can return `NOT_FOUND`, `INVALID_ARGUMENT`, and `PERMISSION_DENIED`
    /// errors.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
    ) -> crate::Result<crate::model::Policy>;

    /// Gets the access control policy for a resource. Returns an empty policy
    /// if the resource exists and does not have a policy set.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
    ) -> crate::Result<crate::model::Policy>;

    /// Returns permissions that a caller has on the specified resource. If the
    /// resource does not exist, this will return an empty set of
    /// permissions, not a `NOT_FOUND` error.
    ///
    /// Note: This operation is designed to be used for building
    /// permission-aware UIs and command-line tools, not for authorization
    /// checking. This operation may "fail open" without warning.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
    ) -> crate::Result<crate::model::TestIamPermissionsResponse>;
}

/// All implementations of [crate::traits::Iampolicy] also implement [Iampolicy].
#[async_trait::async_trait]
impl<T: crate::traits::Iampolicy> Iampolicy for T {
    /// Forwards the call to the implementation provided by `T`.
    async fn set_iam_policy(
        &self,
        req: crate::model::SetIamPolicyRequest,
    ) -> crate::Result<crate::model::Policy> {
        let response = T::set_iam_policy(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn get_iam_policy(
        &self,
        req: crate::model::GetIamPolicyRequest,
    ) -> crate::Result<crate::model::Policy> {
        let response = T::get_iam_policy(self, req).await?;
        Ok(response)
    }

    /// Forwards the call to the implementation provided by `T`.
    async fn test_iam_permissions(
        &self,
        req: crate::model::TestIamPermissionsRequest,
    ) -> crate::Result<crate::model::TestIamPermissionsResponse> {
        let response = T::test_iam_permissions(self, req).await?;
        Ok(response)
    }
}
