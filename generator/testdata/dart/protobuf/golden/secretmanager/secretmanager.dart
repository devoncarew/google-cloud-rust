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

/// The Google Cloud client for the Secret Manager API.
///
/// Stores sensitive data such as API keys, passwords, and certificates.
/// Provides convenience while improving security.
library;

/// Secret Manager Service
/// 
/// Manages secrets and operations using those secrets. Implements a REST
/// model with the following objects:
/// 
/// * [Secret][google.cloud.secretmanager.v1.Secret]
/// * [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
class SecretManagerService {

  /// Lists [Secrets][google.cloud.secretmanager.v1.Secret].
  void listSecrets() {
    // TODO:
  }

  /// Creates a new [Secret][google.cloud.secretmanager.v1.Secret] containing no
  /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion].
  void createSecret() {
    // TODO:
  }

  /// Creates a new [SecretVersion][google.cloud.secretmanager.v1.SecretVersion]
  /// containing secret data and attaches it to an existing
  /// [Secret][google.cloud.secretmanager.v1.Secret].
  void addSecretVersion() {
    // TODO:
  }

  /// Gets metadata for a given [Secret][google.cloud.secretmanager.v1.Secret].
  void getSecret() {
    // TODO:
  }

  /// Updates metadata of an existing
  /// [Secret][google.cloud.secretmanager.v1.Secret].
  void updateSecret() {
    // TODO:
  }

  /// Deletes a [Secret][google.cloud.secretmanager.v1.Secret].
  void deleteSecret() {
    // TODO:
  }

  /// Lists [SecretVersions][google.cloud.secretmanager.v1.SecretVersion]. This
  /// call does not return secret data.
  void listSecretVersions() {
    // TODO:
  }

  /// Gets metadata for a
  /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  /// 
  /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
  /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  void getSecretVersion() {
    // TODO:
  }

  /// Accesses a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  /// This call returns the secret data.
  /// 
  /// `projects/*/secrets/*/versions/latest` is an alias to the most recently
  /// created [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  void accessSecretVersion() {
    // TODO:
  }

  /// Disables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  /// 
  /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
  /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
  /// [DISABLED][google.cloud.secretmanager.v1.SecretVersion.State.DISABLED].
  void disableSecretVersion() {
    // TODO:
  }

  /// Enables a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  /// 
  /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
  /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
  /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED].
  void enableSecretVersion() {
    // TODO:
  }

  /// Destroys a [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
  /// 
  /// Sets the [state][google.cloud.secretmanager.v1.SecretVersion.state] of the
  /// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] to
  /// [DESTROYED][google.cloud.secretmanager.v1.SecretVersion.State.DESTROYED]
  /// and irrevocably destroys the secret data.
  void destroySecretVersion() {
    // TODO:
  }

  /// Sets the access control policy on the specified secret. Replaces any
  /// existing policy.
  /// 
  /// Permissions on
  /// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] are enforced
  /// according to the policy set on the associated
  /// [Secret][google.cloud.secretmanager.v1.Secret].
  void setIamPolicy() {
    // TODO:
  }

  /// Gets the access control policy for a secret.
  /// Returns empty policy if the secret exists and does not have a policy set.
  void getIamPolicy() {
    // TODO:
  }

  /// Returns permissions that a caller has for the specified secret.
  /// If the secret does not exist, this call returns an empty set of
  /// permissions, not a NOT_FOUND error.
  /// 
  /// Note: This operation is designed to be used for building permission-aware
  /// UIs and command-line tools, not for authorization checking. This operation
  /// may "fail open" without warning.
  void testIamPermissions() {
    // TODO:
  }

  /// Lists information about the supported locations for this service.
  void listLocations() {
    // TODO:
  }

  /// Gets information about a location.
  void getLocation() {
    // TODO:
  }
}

/// A [Secret][google.cloud.secretmanager.v1.Secret] is a logical secret whose
/// value and versions can be accessed.
/// 
/// A [Secret][google.cloud.secretmanager.v1.Secret] is made up of zero or more
/// [SecretVersions][google.cloud.secretmanager.v1.SecretVersion] that represent
/// the secret data.
class Secret {
  // TODO:
}

/// A secret version resource in the Secret Manager API.
class SecretVersion {
  // TODO:
}

/// The state of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion], indicating if
/// it can be accessed.
class SecretVersion$State {
  /// Not specified. This value is unused and invalid.
  static const SecretVersion$State stateUnspecified = SecretVersion$State('STATE_UNSPECIFIED');

  /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may be
  /// accessed.
  static const SecretVersion$State enabled = SecretVersion$State('ENABLED');

  /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] may not
  /// be accessed, but the secret data is still available and can be placed
  /// back into the
  /// [ENABLED][google.cloud.secretmanager.v1.SecretVersion.State.ENABLED]
  /// state.
  static const SecretVersion$State disabled = SecretVersion$State('DISABLED');

  /// The [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] is
  /// destroyed and the secret data is no longer stored. A version may not
  /// leave this state once entered.
  static const SecretVersion$State destroyed = SecretVersion$State('DESTROYED');

  final String value;

  const SecretVersion$State(this.value);
}

/// A policy that defines the replication and encryption configuration of data.
class Replication {
  // TODO:
}

/// A replication policy that replicates the
/// [Secret][google.cloud.secretmanager.v1.Secret] payload without any
/// restrictions.
class Replication$Automatic {
  // TODO:
}

/// A replication policy that replicates the
/// [Secret][google.cloud.secretmanager.v1.Secret] payload into the locations
/// specified in [Secret.replication.user_managed.replicas][]
class Replication$UserManaged {
  // TODO:
}

/// Represents a Replica for this
/// [Secret][google.cloud.secretmanager.v1.Secret].
class Replication$UserManaged$Replica {
  // TODO:
}

/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
class CustomerManagedEncryption {
  // TODO:
}

/// The replication status of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
class ReplicationStatus {
  // TODO:
}

/// The replication status of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
/// automatic replication.
/// 
/// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
/// has an automatic replication policy.
class ReplicationStatus$AutomaticStatus {
  // TODO:
}

/// The replication status of a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion] using
/// user-managed replication.
/// 
/// Only populated if the parent [Secret][google.cloud.secretmanager.v1.Secret]
/// has a user-managed replication policy.
class ReplicationStatus$UserManagedStatus {
  // TODO:
}

/// Describes the status of a user-managed replica for the
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
class ReplicationStatus$UserManagedStatus$ReplicaStatus {
  // TODO:
}

/// Describes the status of customer-managed encryption.
class CustomerManagedEncryptionStatus {
  // TODO:
}

/// A Pub/Sub topic which Secret Manager will publish to when control plane
/// events occur on this secret.
class Topic {
  // TODO:
}

/// The rotation time and period for a
/// [Secret][google.cloud.secretmanager.v1.Secret]. At next_rotation_time, Secret
/// Manager will send a Pub/Sub notification to the topics configured on the
/// Secret. [Secret.topics][google.cloud.secretmanager.v1.Secret.topics] must be
/// set to configure rotation.
class Rotation {
  // TODO:
}

/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a
/// [SecretVersion][google.cloud.secretmanager.v1.SecretVersion].
class SecretPayload {
  // TODO:
}

/// Request message for
/// [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
class ListSecretsRequest {
  // TODO:
}

/// Response message for
/// [SecretManagerService.ListSecrets][google.cloud.secretmanager.v1.SecretManagerService.ListSecrets].
class ListSecretsResponse {
  // TODO:
}

/// Request message for
/// [SecretManagerService.CreateSecret][google.cloud.secretmanager.v1.SecretManagerService.CreateSecret].
class CreateSecretRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.AddSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AddSecretVersion].
class AddSecretVersionRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.GetSecret][google.cloud.secretmanager.v1.SecretManagerService.GetSecret].
class GetSecretRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
class ListSecretVersionsRequest {
  // TODO:
}

/// Response message for
/// [SecretManagerService.ListSecretVersions][google.cloud.secretmanager.v1.SecretManagerService.ListSecretVersions].
class ListSecretVersionsResponse {
  // TODO:
}

/// Request message for
/// [SecretManagerService.GetSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.GetSecretVersion].
class GetSecretVersionRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.UpdateSecret][google.cloud.secretmanager.v1.SecretManagerService.UpdateSecret].
class UpdateSecretRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
class AccessSecretVersionRequest {
  // TODO:
}

/// Response message for
/// [SecretManagerService.AccessSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.AccessSecretVersion].
class AccessSecretVersionResponse {
  // TODO:
}

/// Request message for
/// [SecretManagerService.DeleteSecret][google.cloud.secretmanager.v1.SecretManagerService.DeleteSecret].
class DeleteSecretRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.DisableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DisableSecretVersion].
class DisableSecretVersionRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.EnableSecretVersion][google.cloud.secretmanager.v1.SecretManagerService.EnableSecretVersion].
class EnableSecretVersionRequest {
  // TODO:
}

/// Request message for
/// [SecretManagerService.DestroySecretVersion][google.cloud.secretmanager.v1.SecretManagerService.DestroySecretVersion].
class DestroySecretVersionRequest {
  // TODO:
}
