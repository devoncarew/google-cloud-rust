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

/// The response message for Locations.ListLocations.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsResponse {
    /// A list of locations that matches the specified filter in the request.
    pub locations: Vec<crate::model::Location>,

    /// The standard List next-page token.
    pub next_page_token: Option<String>,
}

impl ListLocationsResponse {
    /// Sets the value of `locations`.
    pub fn set_locations<T: Into<Vec<crate::model::Location>>>(mut self, v: T) -> Self {
        self.locations = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }
}

/// A resource that represents a Google Cloud location.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Location {
    /// Resource name for the location, which may vary between implementations.
    /// For example: `"projects/example-project/locations/us-east1"`
    pub name: Option<String>,

    /// The canonical id for this location. For example: `"us-east1"`.
    pub location_id: Option<String>,

    /// The friendly name for this location, typically a nearby city name.
    /// For example, "Tokyo".
    pub display_name: Option<String>,

    /// Cross-service attributes for the location. For example
    ///
    ///     {"cloud.googleapis.com/region": "us-east1"}
    pub labels: std::collections::HashMap<String, String>,

    /// Service-specific metadata. For example the available capacity at the given
    /// location.
    pub metadata: Option<gax_placeholder::Any>,
}

impl Location {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `location_id`.
    pub fn set_location_id<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.location_id = v.into();
        self
    }

    /// Sets the value of `display_name`.
    pub fn set_display_name<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.display_name = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<T: Into<std::collections::HashMap<String, String>>>(mut self, v: T) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `metadata`.
    pub fn set_metadata<T: Into<Option<gax_placeholder::Any>>>(mut self, v: T) -> Self {
        self.metadata = v.into();
        self
    }
}

/// Response message for SecretManagerService.ListSecrets.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretsResponse {
    /// The list of Secrets sorted in reverse by create_time (newest
    /// first).
    pub secrets: Vec<crate::model::Secret>,

    /// A token to retrieve the next page of results. Pass this value in
    /// ListSecretsRequest.page_token to retrieve the next page.
    pub next_page_token: Option<String>,

    /// The total number of Secrets but 0 when the
    /// ListSecretsRequest.filter field is set.
    pub total_size: Option<i32>,
}

impl ListSecretsResponse {
    /// Sets the value of `secrets`.
    pub fn set_secrets<T: Into<Vec<crate::model::Secret>>>(mut self, v: T) -> Self {
        self.secrets = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of `total_size`.
    pub fn set_total_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.total_size = v.into();
        self
    }
}

/// A Secret is a logical secret whose value and versions can
/// be accessed.
///
/// A Secret is made up of zero or more SecretVersions that
/// represent the secret data.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Secret {
    /// Output only. The resource name of the Secret in the format `projects/_*_/secrets/*`.
    pub name: Option<String>,

    /// Optional. Immutable. The replication policy of the secret data attached to the Secret.
    ///
    /// The replication policy cannot be changed after the Secret has been created.
    pub replication: Option<crate::model::Replication>,

    /// Output only. The time at which the Secret was created.
    pub create_time: Option<gax_placeholder::Timestamp>,

    /// The labels assigned to this Secret.
    ///
    /// Label keys must be between 1 and 63 characters long, have a UTF-8 encoding
    /// of maximum 128 bytes, and must conform to the following PCRE regular
    /// expression: `\p{Ll}\p{Lo}{0,62}`
    ///
    /// Label values must be between 0 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, and must conform to the following PCRE
    /// regular expression: `[\p{Ll}\p{Lo}\p{N}_-]{0,63}`
    ///
    /// No more than 64 labels can be assigned to a given resource.
    pub labels: std::collections::HashMap<String, String>,

    /// Optional. A list of up to 10 Pub/Sub topics to which messages are published when
    /// control plane operations are called on the secret or its versions.
    pub topics: Vec<crate::model::Topic>,

    /// Optional. Timestamp in UTC when the Secret is scheduled to expire. This is
    /// always provided on output, regardless of what was sent on input.
    pub expire_time: Option<gax_placeholder::Timestamp>,

    /// Input only. The TTL for the Secret.
    pub ttl: Option<gax_placeholder::Duration>,

    /// Optional. Etag of the currently stored Secret.
    pub etag: Option<String>,

    /// Optional. Rotation policy attached to the Secret. May be excluded if there is no
    /// rotation policy.
    pub rotation: Option<crate::model::Rotation>,

    /// Optional. Mapping from version alias to version name.
    ///
    /// A version alias is a string with a maximum length of 63 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`)
    /// and underscore ('_') characters. An alias string must start with a
    /// letter and cannot be the string 'latest' or 'NEW'.
    /// No more than 50 aliases can be assigned to a given secret.
    ///
    /// Version-Alias pairs will be viewable via GetSecret and modifiable via
    /// UpdateSecret. Access by alias is only be supported on
    /// GetSecretVersion and AccessSecretVersion.
    pub version_aliases: std::collections::HashMap<String, i64>,

    /// Optional. Custom metadata about the secret.
    ///
    /// Annotations are distinct from various forms of labels.
    /// Annotations exist to allow client tools to store their own state
    /// information without requiring a database.
    ///
    /// Annotation keys must be between 1 and 63 characters long, have a UTF-8
    /// encoding of maximum 128 bytes, begin and end with an alphanumeric character
    /// ([a-z0-9A-Z]), and may have dashes (-), underscores (_), dots (.), and
    /// alphanumerics in between these symbols.
    ///
    /// The total size of annotation keys and values must be less than 16KiB.
    pub annotations: std::collections::HashMap<String, String>,

    /// Optional. Secret Version TTL after destruction request
    ///
    /// This is a part of the Delayed secret version destroy feature.
    /// For secret with TTL>0, version destruction doesn't happen immediately
    /// on calling destroy instead the version goes to a disabled state and
    /// destruction happens after the TTL expires.
    pub version_destroy_ttl: Option<gax_placeholder::Duration>,

    /// Optional. The customer-managed encryption configuration of the Regionalised Secrets.
    /// If no configuration is provided, Google-managed default encryption is used.
    ///
    /// Updates to the Secret encryption configuration only apply to
    /// SecretVersions added afterwards. They do not apply
    /// retroactively to existing SecretVersions.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,

    ///
    pub project: String,

    /// Required. This must be unique within the project.
    ///
    /// A secret ID is a string with a maximum length of 255 characters and can
    /// contain uppercase and lowercase letters, numerals, and the hyphen (`-`) and
    /// underscore (`_`) characters.
    pub secret_id: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    /// Required. Specifies the fields to be updated.
    pub update_mask: gax_placeholder::FieldMask,
}

impl Secret {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `replication`.
    pub fn set_replication<T: Into<Option<crate::model::Replication>>>(mut self, v: T) -> Self {
        self.replication = v.into();
        self
    }

    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<gax_placeholder::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `labels`.
    pub fn set_labels<T: Into<std::collections::HashMap<String, String>>>(mut self, v: T) -> Self {
        self.labels = v.into();
        self
    }

    /// Sets the value of `topics`.
    pub fn set_topics<T: Into<Vec<crate::model::Topic>>>(mut self, v: T) -> Self {
        self.topics = v.into();
        self
    }

    /// Sets the value of `expire_time`.
    pub fn set_expire_time<T: Into<Option<gax_placeholder::Timestamp>>>(mut self, v: T) -> Self {
        self.expire_time = v.into();
        self
    }

    /// Sets the value of `ttl`.
    pub fn set_ttl<T: Into<Option<gax_placeholder::Duration>>>(mut self, v: T) -> Self {
        self.ttl = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `rotation`.
    pub fn set_rotation<T: Into<Option<crate::model::Rotation>>>(mut self, v: T) -> Self {
        self.rotation = v.into();
        self
    }

    /// Sets the value of `version_aliases`.
    pub fn set_version_aliases<T: Into<std::collections::HashMap<String, i64>>>(
        mut self,
        v: T,
    ) -> Self {
        self.version_aliases = v.into();
        self
    }

    /// Sets the value of `annotations`.
    pub fn set_annotations<T: Into<std::collections::HashMap<String, String>>>(
        mut self,
        v: T,
    ) -> Self {
        self.annotations = v.into();
        self
    }

    /// Sets the value of `version_destroy_ttl`.
    pub fn set_version_destroy_ttl<T: Into<Option<gax_placeholder::Duration>>>(
        mut self,
        v: T,
    ) -> Self {
        self.version_destroy_ttl = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryption>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret_id`.
    pub fn set_secret_id<T: Into<String>>(mut self, v: T) -> Self {
        self.secret_id = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `update_mask`.
    pub fn set_update_mask<T: Into<gax_placeholder::FieldMask>>(mut self, v: T) -> Self {
        self.update_mask = v.into();
        self
    }
}

/// A policy that defines the replication and encryption configuration of data.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Replication {
    /// The Secret will automatically be replicated without any restrictions.
    pub automatic: Option<crate::model::Automatic>,

    /// The Secret will only be replicated into the locations specified.
    pub user_managed: Option<crate::model::UserManaged>,
}

impl Replication {
    /// Sets the value of `automatic`.
    pub fn set_automatic<T: Into<Option<crate::model::Automatic>>>(mut self, v: T) -> Self {
        self.automatic = v.into();
        self
    }

    /// Sets the value of `user_managed`.
    pub fn set_user_managed<T: Into<Option<crate::model::UserManaged>>>(mut self, v: T) -> Self {
        self.user_managed = v.into();
        self
    }
}

/// A replication policy that replicates the Secret payload without any
/// restrictions.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Automatic {
    /// Optional. The customer-managed encryption configuration of the Secret. If no
    /// configuration is provided, Google-managed default encryption is used.
    ///
    /// Updates to the Secret encryption configuration only apply to
    /// SecretVersions added afterwards. They do not apply
    /// retroactively to existing SecretVersions.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,
}

impl Automatic {
    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryption>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// Configuration for encrypting secret payloads using customer-managed
/// encryption keys (CMEK).
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryption {
    /// Required. The resource name of the Cloud KMS CryptoKey used to encrypt secret
    /// payloads.
    ///
    /// For secrets using the UserManaged replication
    /// policy type, Cloud KMS CryptoKeys must reside in the same location as the
    /// replica location.
    ///
    /// For secrets using the Automatic replication policy
    /// type, Cloud KMS CryptoKeys must reside in `global`.
    ///
    /// The expected format is `projects/_*_/locations/_*_/keyRings/_*_/cryptoKeys/*`.
    pub kms_key_name: String,
}

impl CustomerManagedEncryption {
    /// Sets the value of `kms_key_name`.
    pub fn set_kms_key_name<T: Into<String>>(mut self, v: T) -> Self {
        self.kms_key_name = v.into();
        self
    }
}

/// A replication policy that replicates the Secret payload into the
/// locations specified in Secret.replication.user_managed.replicas
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserManaged {
    /// Required. The list of Replicas for this Secret.
    ///
    /// Cannot be empty.
    pub replicas: Vec<crate::model::Replica>,
}

impl UserManaged {
    /// Sets the value of `replicas`.
    pub fn set_replicas<T: Into<Vec<crate::model::Replica>>>(mut self, v: T) -> Self {
        self.replicas = v.into();
        self
    }
}

/// Represents a Replica for this Secret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Replica {
    /// The canonical IDs of the location to replicate data.
    /// For example: `"us-east1"`.
    pub location: Option<String>,

    /// Optional. The customer-managed encryption configuration of the User-Managed
    /// Replica. If no configuration is
    /// provided, Google-managed default encryption is used.
    ///
    /// Updates to the Secret encryption configuration only apply to
    /// SecretVersions added afterwards. They do not apply
    /// retroactively to existing SecretVersions.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryption>,
}

impl Replica {
    /// Sets the value of `location`.
    pub fn set_location<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryption>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// A Pub/Sub topic which Secret Manager will publish to when control plane
/// events occur on this secret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Topic {
    /// Required. The resource name of the Pub/Sub topic that will be published to, in the
    /// following format: `projects/_*_/topics/*`. For publication to succeed, the
    /// Secret Manager service agent must have the `pubsub.topic.publish`
    /// permission on the topic. The Pub/Sub Publisher role
    /// (`roles/pubsub.publisher`) includes this permission.
    pub name: String,
}

impl Topic {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<String>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }
}

/// The rotation time and period for a Secret. At next_rotation_time, Secret
/// Manager will send a Pub/Sub notification to the topics configured on the
/// Secret. Secret.topics must be set to configure rotation.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Rotation {
    /// Optional. Timestamp in UTC at which the Secret is scheduled to rotate. Cannot be
    /// set to less than 300s (5 min) in the future and at most 3153600000s (100
    /// years).
    ///
    /// next_rotation_time MUST  be set if rotation_period is set.
    pub next_rotation_time: Option<gax_placeholder::Timestamp>,

    /// Input only. The Duration between rotation notifications. Must be in seconds
    /// and at least 3600s (1h) and at most 3153600000s (100 years).
    ///
    /// If rotation_period is set, next_rotation_time must be set.
    /// next_rotation_time will be advanced by this period when the service
    /// automatically sends rotation notifications.
    pub rotation_period: Option<gax_placeholder::Duration>,
}

impl Rotation {
    /// Sets the value of `next_rotation_time`.
    pub fn set_next_rotation_time<T: Into<Option<gax_placeholder::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.next_rotation_time = v.into();
        self
    }

    /// Sets the value of `rotation_period`.
    pub fn set_rotation_period<T: Into<Option<gax_placeholder::Duration>>>(mut self, v: T) -> Self {
        self.rotation_period = v.into();
        self
    }
}

/// Request message for SecretManagerService.AddSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AddSecretVersionRequest {
    /// Required. The secret payload of the SecretVersion.
    pub payload: Option<crate::model::SecretPayload>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub location: String,
}

impl AddSecretVersionRequest {
    /// Sets the value of `payload`.
    pub fn set_payload<T: Into<Option<crate::model::SecretPayload>>>(mut self, v: T) -> Self {
        self.payload = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// A secret payload resource in the Secret Manager API. This contains the
/// sensitive secret payload that is associated with a SecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretPayload {
    /// The secret data. Must be no larger than 64KiB.
    pub data: Option<bytes::Bytes>,

    /// Optional. If specified, SecretManagerService will verify the integrity of the
    /// received data on SecretManagerService.AddSecretVersion calls using
    /// the crc32c checksum and store it to include in future
    /// SecretManagerService.AccessSecretVersion responses. If a checksum is
    /// not provided in the SecretManagerService.AddSecretVersion request, the
    /// SecretManagerService will generate and store one for you.
    ///
    /// The CRC32C value is encoded as a Int64 for compatibility, and can be
    /// safely downconverted to uint32 in languages that support this type.
    /// https://cloud.google.com/apis/design/design_patterns#integer_types
    pub data_crc_32_c: Option<i64>,
}

impl SecretPayload {
    /// Sets the value of `data`.
    pub fn set_data<T: Into<Option<bytes::Bytes>>>(mut self, v: T) -> Self {
        self.data = v.into();
        self
    }

    /// Sets the value of `data_crc_32_c`.
    pub fn set_data_crc_32_c<T: Into<Option<i64>>>(mut self, v: T) -> Self {
        self.data_crc_32_c = v.into();
        self
    }
}

/// A secret version resource in the Secret Manager API.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SecretVersion {
    /// Output only. The resource name of the SecretVersion in the
    /// format `projects/_*_/secrets/_*_/versions/*`.
    ///
    /// SecretVersion IDs in a Secret start at 1 and
    /// are incremented for each subsequent version of the secret.
    pub name: Option<String>,

    /// Output only. The time at which the SecretVersion was created.
    pub create_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The time this SecretVersion was destroyed.
    /// Only present if state is
    /// DESTROYED.
    pub destroy_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The current state of the SecretVersion.
    pub state: Option<String>,

    /// The replication status of the SecretVersion.
    pub replication_status: Option<crate::model::ReplicationStatus>,

    /// Output only. Etag of the currently stored SecretVersion.
    pub etag: Option<String>,

    /// Output only. True if payload checksum specified in SecretPayload object has been
    /// received by SecretManagerService on
    /// SecretManagerService.AddSecretVersion.
    pub client_specified_payload_checksum: Option<bool>,

    /// Optional. Output only. Scheduled destroy time for secret version.
    /// This is a part of the Delayed secret version destroy feature. For a
    /// Secret with a valid version destroy TTL, when a secert version is
    /// destroyed, version is moved to disabled state and it is scheduled for
    /// destruction Version is destroyed only after the scheduled_destroy_time.
    pub scheduled_destroy_time: Option<gax_placeholder::Timestamp>,

    /// Output only. The customer-managed encryption status of the SecretVersion. Only
    /// populated if customer-managed encryption is used and Secret is
    /// a Regionalised Secret.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
}

impl SecretVersion {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `create_time`.
    pub fn set_create_time<T: Into<Option<gax_placeholder::Timestamp>>>(mut self, v: T) -> Self {
        self.create_time = v.into();
        self
    }

    /// Sets the value of `destroy_time`.
    pub fn set_destroy_time<T: Into<Option<gax_placeholder::Timestamp>>>(mut self, v: T) -> Self {
        self.destroy_time = v.into();
        self
    }

    /// Sets the value of `state`.
    pub fn set_state<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.state = v.into();
        self
    }

    /// Sets the value of `replication_status`.
    pub fn set_replication_status<T: Into<Option<crate::model::ReplicationStatus>>>(
        mut self,
        v: T,
    ) -> Self {
        self.replication_status = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `client_specified_payload_checksum`.
    pub fn set_client_specified_payload_checksum<T: Into<Option<bool>>>(mut self, v: T) -> Self {
        self.client_specified_payload_checksum = v.into();
        self
    }

    /// Sets the value of `scheduled_destroy_time`.
    pub fn set_scheduled_destroy_time<T: Into<Option<gax_placeholder::Timestamp>>>(
        mut self,
        v: T,
    ) -> Self {
        self.scheduled_destroy_time = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// The replication status of a SecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReplicationStatus {
    /// Describes the replication status of a SecretVersion with
    /// automatic replication.
    ///
    /// Only populated if the parent Secret has an automatic replication
    /// policy.
    pub automatic: Option<crate::model::AutomaticStatus>,

    /// Describes the replication status of a SecretVersion with
    /// user-managed replication.
    ///
    /// Only populated if the parent Secret has a user-managed replication
    /// policy.
    pub user_managed: Option<crate::model::UserManagedStatus>,
}

impl ReplicationStatus {
    /// Sets the value of `automatic`.
    pub fn set_automatic<T: Into<Option<crate::model::AutomaticStatus>>>(mut self, v: T) -> Self {
        self.automatic = v.into();
        self
    }

    /// Sets the value of `user_managed`.
    pub fn set_user_managed<T: Into<Option<crate::model::UserManagedStatus>>>(
        mut self,
        v: T,
    ) -> Self {
        self.user_managed = v.into();
        self
    }
}

/// The replication status of a SecretVersion using automatic replication.
///
/// Only populated if the parent Secret has an automatic replication
/// policy.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AutomaticStatus {
    /// Output only. The customer-managed encryption status of the SecretVersion. Only
    /// populated if customer-managed encryption is used.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
}

impl AutomaticStatus {
    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// Describes the status of customer-managed encryption.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct CustomerManagedEncryptionStatus {
    /// Required. The resource name of the Cloud KMS CryptoKeyVersion used to encrypt the
    /// secret payload, in the following format:
    /// `projects/_*_/locations/_*_/keyRings/_*_/cryptoKeys/_*_/versions/*`.
    pub kms_key_version_name: String,
}

impl CustomerManagedEncryptionStatus {
    /// Sets the value of `kms_key_version_name`.
    pub fn set_kms_key_version_name<T: Into<String>>(mut self, v: T) -> Self {
        self.kms_key_version_name = v.into();
        self
    }
}

/// The replication status of a SecretVersion using user-managed
/// replication.
///
/// Only populated if the parent Secret has a user-managed replication
/// policy.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct UserManagedStatus {
    /// Output only. The list of replica statuses for the SecretVersion.
    pub replicas: Vec<crate::model::ReplicaStatus>,
}

impl UserManagedStatus {
    /// Sets the value of `replicas`.
    pub fn set_replicas<T: Into<Vec<crate::model::ReplicaStatus>>>(mut self, v: T) -> Self {
        self.replicas = v.into();
        self
    }
}

/// Describes the status of a user-managed replica for the SecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ReplicaStatus {
    /// Output only. The canonical ID of the replica location.
    /// For example: `"us-east1"`.
    pub location: Option<String>,

    /// Output only. The customer-managed encryption status of the SecretVersion. Only
    /// populated if customer-managed encryption is used.
    pub customer_managed_encryption: Option<crate::model::CustomerManagedEncryptionStatus>,
}

impl ReplicaStatus {
    /// Sets the value of `location`.
    pub fn set_location<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `customer_managed_encryption`.
    pub fn set_customer_managed_encryption<
        T: Into<Option<crate::model::CustomerManagedEncryptionStatus>>,
    >(
        mut self,
        v: T,
    ) -> Self {
        self.customer_managed_encryption = v.into();
        self
    }
}

/// A generic empty message that you can re-use to avoid defining duplicated
/// empty messages in your APIs. A typical example is to use it as the request
/// or the response type of an API method. For instance:
///
///     service Foo {
///       rpc Bar(google.protobuf.Empty) returns (google.protobuf.Empty);
///     }
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Empty {}

impl Empty {}

/// Response message for SecretManagerService.ListSecretVersions.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretVersionsResponse {
    /// The list of SecretVersions sorted in reverse by
    /// create_time (newest first).
    pub versions: Vec<crate::model::SecretVersion>,

    /// A token to retrieve the next page of results. Pass this value in
    /// ListSecretVersionsRequest.page_token to retrieve the next page.
    pub next_page_token: Option<String>,

    /// The total number of SecretVersions but 0 when the
    /// ListSecretsRequest.filter field is set.
    pub total_size: Option<i32>,
}

impl ListSecretVersionsResponse {
    /// Sets the value of `versions`.
    pub fn set_versions<T: Into<Vec<crate::model::SecretVersion>>>(mut self, v: T) -> Self {
        self.versions = v.into();
        self
    }

    /// Sets the value of `next_page_token`.
    pub fn set_next_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.next_page_token = v.into();
        self
    }

    /// Sets the value of `total_size`.
    pub fn set_total_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.total_size = v.into();
        self
    }
}

/// Response message for SecretManagerService.AccessSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccessSecretVersionResponse {
    /// The resource name of the SecretVersion in the format
    /// `projects/_*_/secrets/_*_/versions/*` or
    /// `projects/_*_/locations/_*_/secrets/_*_/versions/*`.
    pub name: Option<String>,

    /// Secret payload
    pub payload: Option<crate::model::SecretPayload>,
}

impl AccessSecretVersionResponse {
    /// Sets the value of `name`.
    pub fn set_name<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.name = v.into();
        self
    }

    /// Sets the value of `payload`.
    pub fn set_payload<T: Into<Option<crate::model::SecretPayload>>>(mut self, v: T) -> Self {
        self.payload = v.into();
        self
    }
}

/// Request message for SecretManagerService.DisableSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DisableSecretVersionRequest {
    /// Optional. Etag of the SecretVersion. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    pub etag: Option<String>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub version: String,

    ///
    pub location: String,
}

impl DisableSecretVersionRequest {
    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// Request message for SecretManagerService.EnableSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct EnableSecretVersionRequest {
    /// Optional. Etag of the SecretVersion. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    pub etag: Option<String>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub version: String,

    ///
    pub location: String,
}

impl EnableSecretVersionRequest {
    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// Request message for SecretManagerService.DestroySecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DestroySecretVersionRequest {
    /// Optional. Etag of the SecretVersion. The request succeeds if it matches
    /// the etag of the currently stored secret version object. If the etag is
    /// omitted, the request succeeds.
    pub etag: Option<String>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub version: String,

    ///
    pub location: String,
}

impl DestroySecretVersionRequest {
    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// Request message for `SetIamPolicy` method.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct SetIamPolicyRequest {
    /// REQUIRED: The complete policy to be applied to the `resource`. The size of
    /// the policy is limited to a few 10s of KB. An empty policy is a
    /// valid policy but certain Google Cloud services (such as Projects)
    /// might reject them.
    pub policy: Option<crate::model::Policy>,

    /// OPTIONAL: A FieldMask specifying which fields of the policy to modify. Only
    /// the fields in the mask will be modified. If no mask is provided, the
    /// following default mask is used:
    ///
    /// `paths: "bindings, etag"`
    pub update_mask: Option<gax_placeholder::FieldMask>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub location: String,
}

impl SetIamPolicyRequest {
    /// Sets the value of `policy`.
    pub fn set_policy<T: Into<Option<crate::model::Policy>>>(mut self, v: T) -> Self {
        self.policy = v.into();
        self
    }

    /// Sets the value of `update_mask`.
    pub fn set_update_mask<T: Into<Option<gax_placeholder::FieldMask>>>(mut self, v: T) -> Self {
        self.update_mask = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// An Identity and Access Management (IAM) policy, which specifies access
/// controls for Google Cloud resources.
///
///
/// A `Policy` is a collection of `bindings`. A `binding` binds one or more
/// `members`, or principals, to a single `role`. Principals can be user
/// accounts, service accounts, Google groups, and domains (such as G Suite). A
/// `role` is a named list of permissions; each `role` can be an IAM predefined
/// role or a user-created custom role.
///
/// For some types of Google Cloud resources, a `binding` can also specify a
/// `condition`, which is a logical expression that allows access to a resource
/// only if the expression evaluates to `true`. A condition can add constraints
/// based on attributes of the request, the resource, or both. To learn which
/// resources support conditions in their IAM policies, see the
/// [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
///
/// **JSON example:**
///
/// ```norust
///     {
///       "bindings": [
///         {
///           "role": "roles/resourcemanager.organizationAdmin",
///           "members": [
///             "user:mike@example.com",
///             "group:admins@example.com",
///             "domain:google.com",
///             "serviceAccount:my-project-id@appspot.gserviceaccount.com"
///           ]
///         },
///         {
///           "role": "roles/resourcemanager.organizationViewer",
///           "members": [
///             "user:eve@example.com"
///           ],
///           "condition": {
///             "title": "expirable access",
///             "description": "Does not grant access after Sep 2020",
///             "expression": "request.time < timestamp('2020-10-01T00:00:00.000Z')",
///           }
///         }
///       ],
///       "etag": "BwWWja0YfJA=",
///       "version": 3
///     }
/// ```
///
/// **YAML example:**
///
/// ```norust
///     bindings:
///     - members:
///       - user:mike@example.com
///       - group:admins@example.com
///       - domain:google.com
///       - serviceAccount:my-project-id@appspot.gserviceaccount.com
///       role: roles/resourcemanager.organizationAdmin
///     - members:
///       - user:eve@example.com
///       role: roles/resourcemanager.organizationViewer
///       condition:
///         title: expirable access
///         description: Does not grant access after Sep 2020
///         expression: request.time < timestamp('2020-10-01T00:00:00.000Z')
///     etag: BwWWja0YfJA=
///     version: 3
/// ```
///
/// For a description of IAM and its features, see the
/// [IAM documentation](https://cloud.google.com/iam/docs/).
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Policy {
    /// Specifies the format of the policy.
    ///
    /// Valid values are `0`, `1`, and `3`. Requests that specify an invalid value
    /// are rejected.
    ///
    /// Any operation that affects conditional role bindings must specify version
    /// `3`. This requirement applies to the following operations:
    ///
    /// * Getting a policy that includes a conditional role binding
    /// * Adding a conditional role binding to a policy
    /// * Changing a conditional role binding in a policy
    /// * Removing any role binding, with or without a condition, from a policy
    ///   that includes conditions
    ///
    /// **Important:** If you use IAM Conditions, you must include the `etag` field
    /// whenever you call `setIamPolicy`. If you omit this field, then IAM allows
    /// you to overwrite a version `3` policy with a version `1` policy, and all of
    /// the conditions in the version `3` policy are lost.
    ///
    /// If a policy does not include any conditions, operations on that policy may
    /// specify any valid version or leave the field unset.
    ///
    /// To learn which resources support conditions in their IAM policies, see the
    /// [IAM documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    pub version: Option<i32>,

    /// Associates a list of `members`, or principals, with a `role`. Optionally,
    /// may specify a `condition` that determines how and when the `bindings` are
    /// applied. Each of the `bindings` must contain at least one principal.
    ///
    /// The `bindings` in a `Policy` can refer to up to 1,500 principals; up to 250
    /// of these principals can be Google groups. Each occurrence of a principal
    /// counts towards these limits. For example, if the `bindings` grant 50
    /// different roles to `user:alice@example.com`, and not to any other
    /// principal, then you can add another 1,450 principals to the `bindings` in
    /// the `Policy`.
    pub bindings: Vec<crate::model::Binding>,

    /// Specifies cloud audit logging configuration for this policy.
    pub audit_configs: Vec<crate::model::AuditConfig>,

    /// `etag` is used for optimistic concurrency control as a way to help
    /// prevent simultaneous updates of a policy from overwriting each other.
    /// It is strongly suggested that systems make use of the `etag` in the
    /// read-modify-write cycle to perform policy updates in order to avoid race
    /// conditions: An `etag` is returned in the response to `getIamPolicy`, and
    /// systems are expected to put that etag in the request to `setIamPolicy` to
    /// ensure that their change will be applied to the same version of the policy.
    ///
    /// **Important:** If you use IAM Conditions, you must include the `etag` field
    /// whenever you call `setIamPolicy`. If you omit this field, then IAM allows
    /// you to overwrite a version `3` policy with a version `1` policy, and all of
    /// the conditions in the version `3` policy are lost.
    pub etag: Option<bytes::Bytes>,
}

impl Policy {
    /// Sets the value of `version`.
    pub fn set_version<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }

    /// Sets the value of `bindings`.
    pub fn set_bindings<T: Into<Vec<crate::model::Binding>>>(mut self, v: T) -> Self {
        self.bindings = v.into();
        self
    }

    /// Sets the value of `audit_configs`.
    pub fn set_audit_configs<T: Into<Vec<crate::model::AuditConfig>>>(mut self, v: T) -> Self {
        self.audit_configs = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<bytes::Bytes>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// Associates `members`, or principals, with a `role`.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Binding {
    /// Role that is assigned to the list of `members`, or principals.
    /// For example, `roles/viewer`, `roles/editor`, or `roles/owner`.
    ///
    /// For an overview of the IAM roles and permissions, see the
    /// [IAM documentation](https://cloud.google.com/iam/docs/roles-overview). For
    /// a list of the available pre-defined roles, see
    /// [here](https://cloud.google.com/iam/docs/understanding-roles).
    pub role: Option<String>,

    /// Specifies the principals requesting access for a Google Cloud resource.
    /// `members` can have the following values:
    ///
    /// * `allUsers`: A special identifier that represents anyone who is
    ///    on the internet; with or without a Google account.
    ///
    /// * `allAuthenticatedUsers`: A special identifier that represents anyone
    ///    who is authenticated with a Google account or a service account.
    ///    Does not include identities that come from external identity providers
    ///    (IdPs) through identity federation.
    ///
    /// * `user:{emailid}`: An email address that represents a specific Google
    ///    account. For example, `alice@example.com` .
    ///
    ///
    /// * `serviceAccount:{emailid}`: An email address that represents a Google
    ///    service account. For example,
    ///    `my-other-app@appspot.gserviceaccount.com`.
    ///
    /// * `serviceAccount:{projectid}.svc.id.goog[{namespace}/{kubernetes-sa}]`: An
    ///    identifier for a
    ///    [Kubernetes service
    ///    account](https://cloud.google.com/kubernetes-engine/docs/how-to/kubernetes-service-accounts).
    ///    For example, `my-project.svc.id.goog[my-namespace/my-kubernetes-sa]`.
    ///
    /// * `group:{emailid}`: An email address that represents a Google group.
    ///    For example, `admins@example.com`.
    ///
    ///
    /// * `domain:{domain}`: The G Suite domain (primary) that represents all the
    ///    users of that domain. For example, `google.com` or `example.com`.
    ///
    ///
    ///
    ///
    /// * `principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`:
    ///   A single identity in a workforce identity pool.
    ///
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/group/{group_id}`:
    ///   All workforce identities in a group.
    ///
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/attribute.{attribute_name}/{attribute_value}`:
    ///   All workforce identities with a specific attribute value.
    ///
    /// * `principalSet://iam.googleapis.com/locations/global/workforcePools/{pool_id}/*`:
    ///   All identities in a workforce identity pool.
    ///
    /// * `principal://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/subject/{subject_attribute_value}`:
    ///   A single identity in a workload identity pool.
    ///
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/group/{group_id}`:
    ///   A workload identity pool group.
    ///
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/attribute.{attribute_name}/{attribute_value}`:
    ///   All identities in a workload identity pool with a certain attribute.
    ///
    /// * `principalSet://iam.googleapis.com/projects/{project_number}/locations/global/workloadIdentityPools/{pool_id}/*`:
    ///   All identities in a workload identity pool.
    ///
    /// * `deleted:user:{emailid}?uid={uniqueid}`: An email address (plus unique
    ///    identifier) representing a user that has been recently deleted. For
    ///    example, `alice@example.com?uid=123456789012345678901`. If the user is
    ///    recovered, this value reverts to `user:{emailid}` and the recovered user
    ///    retains the role in the binding.
    ///
    /// * `deleted:serviceAccount:{emailid}?uid={uniqueid}`: An email address (plus
    ///    unique identifier) representing a service account that has been recently
    ///    deleted. For example,
    ///    `my-other-app@appspot.gserviceaccount.com?uid=123456789012345678901`.
    ///    If the service account is undeleted, this value reverts to
    ///    `serviceAccount:{emailid}` and the undeleted service account retains the
    ///    role in the binding.
    ///
    /// * `deleted:group:{emailid}?uid={uniqueid}`: An email address (plus unique
    ///    identifier) representing a Google group that has been recently
    ///    deleted. For example, `admins@example.com?uid=123456789012345678901`. If
    ///    the group is recovered, this value reverts to `group:{emailid}` and the
    ///    recovered group retains the role in the binding.
    ///
    /// * `deleted:principal://iam.googleapis.com/locations/global/workforcePools/{pool_id}/subject/{subject_attribute_value}`:
    ///   Deleted single identity in a workforce identity pool. For example,
    ///   `deleted:principal://iam.googleapis.com/locations/global/workforcePools/my-pool-id/subject/my-subject-attribute-value`.
    pub members: Vec<String>,

    /// The condition that is associated with this binding.
    ///
    /// If the condition evaluates to `true`, then this binding applies to the
    /// current request.
    ///
    /// If the condition evaluates to `false`, then this binding does not apply to
    /// the current request. However, a different role binding might grant the same
    /// role to one or more of the principals in this binding.
    ///
    /// To learn which resources support conditions in their IAM policies, see the
    /// [IAM
    /// documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    pub condition: Option<crate::model::Expr>,
}

impl Binding {
    /// Sets the value of `role`.
    pub fn set_role<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.role = v.into();
        self
    }

    /// Sets the value of `members`.
    pub fn set_members<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.members = v.into();
        self
    }

    /// Sets the value of `condition`.
    pub fn set_condition<T: Into<Option<crate::model::Expr>>>(mut self, v: T) -> Self {
        self.condition = v.into();
        self
    }
}

/// Represents a textual expression in the Common Expression Language (CEL)
/// syntax. CEL is a C-like expression language. The syntax and semantics of CEL
/// are documented at https://github.com/google/cel-spec.
///
/// Example (Comparison):
///
///     title: "Summary size limit"
///     description: "Determines if a summary is less than 100 chars"
///     expression: "document.summary.size() < 100"
///
/// Example (Equality):
///
///     title: "Requestor is owner"
///     description: "Determines if requestor is the document owner"
///     expression: "document.owner == request.auth.claims.email"
///
/// Example (Logic):
///
///     title: "Public documents"
///     description: "Determine whether the document should be publicly visible"
///     expression: "document.type != 'private' && document.type != 'internal'"
///
/// Example (Data Manipulation):
///
///     title: "Notification string"
///     description: "Create a notification string with a timestamp."
///     expression: "'New message received at ' + string(document.create_time)"
///
/// The exact variables and functions that may be referenced within an expression
/// are determined by the service that evaluates it. See the service
/// documentation for additional information.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Expr {
    /// Textual representation of an expression in Common Expression Language
    /// syntax.
    pub expression: Option<String>,

    /// Optional. Title for the expression, i.e. a short string describing
    /// its purpose. This can be used e.g. in UIs which allow to enter the
    /// expression.
    pub title: Option<String>,

    /// Optional. Description of the expression. This is a longer text which
    /// describes the expression, e.g. when hovered over it in a UI.
    pub description: Option<String>,

    /// Optional. String indicating the location of the expression for error
    /// reporting, e.g. a file name and a position in the file.
    pub location: Option<String>,
}

impl Expr {
    /// Sets the value of `expression`.
    pub fn set_expression<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.expression = v.into();
        self
    }

    /// Sets the value of `title`.
    pub fn set_title<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.title = v.into();
        self
    }

    /// Sets the value of `description`.
    pub fn set_description<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.description = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// Specifies the audit configuration for a service.
/// The configuration determines which permission types are logged, and what
/// identities, if any, are exempted from logging.
/// An AuditConfig must have one or more AuditLogConfigs.
///
/// If there are AuditConfigs for both `allServices` and a specific service,
/// the union of the two AuditConfigs is used for that service: the log_types
/// specified in each AuditConfig are enabled, and the exempted_members in each
/// AuditLogConfig are exempted.
///
/// Example Policy with multiple AuditConfigs:
///
///     {
///       "audit_configs": [
///         {
///           "service": "allServices",
///           "audit_log_configs": [
///             {
///               "log_type": "DATA_READ",
///               "exempted_members": [
///                 "user:jose@example.com"
///               ]
///             },
///             {
///               "log_type": "DATA_WRITE"
///             },
///             {
///               "log_type": "ADMIN_READ"
///             }
///           ]
///         },
///         {
///           "service": "sampleservice.googleapis.com",
///           "audit_log_configs": [
///             {
///               "log_type": "DATA_READ"
///             },
///             {
///               "log_type": "DATA_WRITE",
///               "exempted_members": [
///                 "user:aliya@example.com"
///               ]
///             }
///           ]
///         }
///       ]
///     }
///
/// For sampleservice, this policy enables DATA_READ, DATA_WRITE and ADMIN_READ
/// logging. It also exempts `jose@example.com` from DATA_READ logging, and
/// `aliya@example.com` from DATA_WRITE logging.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuditConfig {
    /// Specifies a service that will be enabled for audit logging.
    /// For example, `storage.googleapis.com`, `cloudsql.googleapis.com`.
    /// `allServices` is a special value that covers all services.
    pub service: Option<String>,

    /// The configuration for logging of each type of permission.
    pub audit_log_configs: Vec<crate::model::AuditLogConfig>,
}

impl AuditConfig {
    /// Sets the value of `service`.
    pub fn set_service<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.service = v.into();
        self
    }

    /// Sets the value of `audit_log_configs`.
    pub fn set_audit_log_configs<T: Into<Vec<crate::model::AuditLogConfig>>>(
        mut self,
        v: T,
    ) -> Self {
        self.audit_log_configs = v.into();
        self
    }
}

/// Provides the configuration for logging a type of permissions.
/// Example:
///
///     {
///       "audit_log_configs": [
///         {
///           "log_type": "DATA_READ",
///           "exempted_members": [
///             "user:jose@example.com"
///           ]
///         },
///         {
///           "log_type": "DATA_WRITE"
///         }
///       ]
///     }
///
/// This enables 'DATA_READ' and 'DATA_WRITE' logging, while exempting
/// jose@example.com from DATA_READ logging.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AuditLogConfig {
    /// The log type that this config enables.
    pub log_type: Option<String>,

    /// Specifies the identities that do not cause logging for this type of
    /// permission.
    /// Follows the same format of Binding.members.
    pub exempted_members: Vec<String>,
}

impl AuditLogConfig {
    /// Sets the value of `log_type`.
    pub fn set_log_type<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.log_type = v.into();
        self
    }

    /// Sets the value of `exempted_members`.
    pub fn set_exempted_members<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.exempted_members = v.into();
        self
    }
}

/// Request message for `TestIamPermissions` method.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TestIamPermissionsRequest {
    /// The set of permissions to check for the `resource`. Permissions with
    /// wildcards (such as `*` or `storage.*`) are not allowed. For more
    /// information see
    /// [IAM Overview](https://cloud.google.com/iam/docs/overview#permissions).
    pub permissions: Vec<String>,

    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub location: String,
}

impl TestIamPermissionsRequest {
    /// Sets the value of `permissions`.
    pub fn set_permissions<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.permissions = v.into();
        self
    }

    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// Response message for `TestIamPermissions` method.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct TestIamPermissionsResponse {
    /// A subset of `TestPermissionsRequest.permissions` that the caller is
    /// allowed.
    pub permissions: Vec<String>,
}

impl TestIamPermissionsResponse {
    /// Sets the value of `permissions`.
    pub fn set_permissions<T: Into<Vec<String>>>(mut self, v: T) -> Self {
        self.permissions = v.into();
        self
    }
}

/// The request message for ListLocations.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListLocationsRequest {
    ///
    pub project: String,

    /// A filter to narrow down results to a preferred subset.
    /// The filtering language accepts strings like `"displayName=tokyo"`, and
    /// is documented in more detail in [AIP-160](https://google.aip.dev/160).
    pub filter: Option<String>,

    /// The maximum number of results to return.
    /// If not set, the service selects a default.
    pub page_size: Option<i32>,

    /// A page token received from the `next_page_token` field in the response.
    /// Send that page token to receive the subsequent page.
    pub page_token: Option<String>,
}

impl ListLocationsRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }
}

/// The request message for GetLocation.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetLocationRequest {
    ///
    pub project: String,

    ///
    pub location: String,
}

impl GetLocationRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }
}

/// The request message for ListSecrets.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretsRequest {
    ///
    pub project: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: Option<i32>,

    /// Optional. Pagination token, returned earlier via
    /// ListSecretsResponse.next_page_token.
    pub page_token: Option<String>,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secrets matching the filter. If filter is empty, all secrets are
    /// listed.
    pub filter: Option<String>,
}

impl ListSecretsRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// The request message for ListSecretsByProjectAndLocation.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretsByProjectAndLocationRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: Option<i32>,

    /// Optional. Pagination token, returned earlier via
    /// ListSecretsResponse.next_page_token.
    pub page_token: Option<String>,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secrets matching the filter. If filter is empty, all secrets are
    /// listed.
    pub filter: Option<String>,
}

impl ListSecretsByProjectAndLocationRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// The request message for GetSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretRequest {
    ///
    pub project: String,

    ///
    pub secret: String,
}

impl GetSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }
}

/// The request message for DeleteSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteSecretRequest {
    ///
    pub project: String,

    ///
    pub secret: String,

    /// Optional. Etag of the Secret. The request succeeds if it matches
    /// the etag of the currently stored secret object. If the etag is omitted,
    /// the request succeeds.
    pub etag: Option<String>,
}

impl DeleteSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// The request message for GetSecretByProjectAndLocationAndSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretByProjectAndLocationAndSecretRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,
}

impl GetSecretByProjectAndLocationAndSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }
}

/// The request message for DeleteSecretByProjectAndLocationAndSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct DeleteSecretByProjectAndLocationAndSecretRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    /// Optional. Etag of the Secret. The request succeeds if it matches
    /// the etag of the currently stored secret object. If the etag is omitted,
    /// the request succeeds.
    pub etag: Option<String>,
}

impl DeleteSecretByProjectAndLocationAndSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `etag`.
    pub fn set_etag<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.etag = v.into();
        self
    }
}

/// The request message for ListSecretVersions.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretVersionsRequest {
    ///
    pub project: String,

    ///
    pub secret: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: Option<i32>,

    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    pub page_token: Option<String>,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secret versions matching the filter. If filter is empty, all secret
    /// versions are listed.
    pub filter: Option<String>,
}

impl ListSecretVersionsRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// The request message for ListSecretVersionsByProjectAndLocationAndSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ListSecretVersionsByProjectAndLocationAndSecretRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    /// Optional. The maximum number of results to be returned in a single page. If
    /// set to 0, the server decides the number of results to return. If the
    /// number is greater than 25000, it is capped at 25000.
    pub page_size: Option<i32>,

    /// Optional. Pagination token, returned earlier via
    /// ListSecretVersionsResponse.next_page_token][].
    pub page_token: Option<String>,

    /// Optional. Filter string, adhering to the rules in
    /// [List-operation
    /// filtering](https://cloud.google.com/secret-manager/docs/filtering). List
    /// only secret versions matching the filter. If filter is empty, all secret
    /// versions are listed.
    pub filter: Option<String>,
}

impl ListSecretVersionsByProjectAndLocationAndSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `page_size`.
    pub fn set_page_size<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.page_size = v.into();
        self
    }

    /// Sets the value of `page_token`.
    pub fn set_page_token<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.page_token = v.into();
        self
    }

    /// Sets the value of `filter`.
    pub fn set_filter<T: Into<Option<String>>>(mut self, v: T) -> Self {
        self.filter = v.into();
        self
    }
}

/// The request message for GetSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretVersionRequest {
    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub version: String,
}

impl GetSecretVersionRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }
}

/// The request message for GetSecretVersionByProjectAndLocationAndSecretAndVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    ///
    pub version: String,
}

impl GetSecretVersionByProjectAndLocationAndSecretAndVersionRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }
}

/// The request message for AccessSecretVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccessSecretVersionRequest {
    ///
    pub project: String,

    ///
    pub secret: String,

    ///
    pub version: String,
}

impl AccessSecretVersionRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }
}

/// The request message for AccessSecretVersionByProjectAndLocationAndSecretAndVersion.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    ///
    pub version: String,
}

impl AccessSecretVersionByProjectAndLocationAndSecretAndVersionRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `version`.
    pub fn set_version<T: Into<String>>(mut self, v: T) -> Self {
        self.version = v.into();
        self
    }
}

/// The request message for GetIamPolicy.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetIamPolicyRequest {
    ///
    pub project: String,

    ///
    pub secret: String,

    /// Optional. The maximum policy version that will be used to format the
    /// policy.
    ///
    /// Valid values are 0, 1, and 3. Requests specifying an invalid value will be
    /// rejected.
    ///
    /// Requests for policies with any conditional role bindings must specify
    /// version 3. Policies with no conditional role bindings may specify any valid
    /// value or leave the field unset.
    ///
    /// The policy in the response might use the policy version that you specified,
    /// or it might use a lower policy version. For example, if you specify version
    /// 3, but the policy has no conditional role bindings, the response uses
    /// version 1.
    ///
    /// To learn which resources support conditions in their IAM policies, see the
    /// [IAM
    /// documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    pub options_requested_policy_version: Option<i32>,
}

impl GetIamPolicyRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `options_requested_policy_version`.
    pub fn set_options_requested_policy_version<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.options_requested_policy_version = v.into();
        self
    }
}

/// The request message for GetIamPolicyByProjectAndLocationAndSecret.
#[derive(Clone, Debug, Default, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct GetIamPolicyByProjectAndLocationAndSecretRequest {
    ///
    pub project: String,

    ///
    pub location: String,

    ///
    pub secret: String,

    /// Optional. The maximum policy version that will be used to format the
    /// policy.
    ///
    /// Valid values are 0, 1, and 3. Requests specifying an invalid value will be
    /// rejected.
    ///
    /// Requests for policies with any conditional role bindings must specify
    /// version 3. Policies with no conditional role bindings may specify any valid
    /// value or leave the field unset.
    ///
    /// The policy in the response might use the policy version that you specified,
    /// or it might use a lower policy version. For example, if you specify version
    /// 3, but the policy has no conditional role bindings, the response uses
    /// version 1.
    ///
    /// To learn which resources support conditions in their IAM policies, see the
    /// [IAM
    /// documentation](https://cloud.google.com/iam/help/conditions/resource-policies).
    pub options_requested_policy_version: Option<i32>,
}

impl GetIamPolicyByProjectAndLocationAndSecretRequest {
    /// Sets the value of `project`.
    pub fn set_project<T: Into<String>>(mut self, v: T) -> Self {
        self.project = v.into();
        self
    }

    /// Sets the value of `location`.
    pub fn set_location<T: Into<String>>(mut self, v: T) -> Self {
        self.location = v.into();
        self
    }

    /// Sets the value of `secret`.
    pub fn set_secret<T: Into<String>>(mut self, v: T) -> Self {
        self.secret = v.into();
        self
    }

    /// Sets the value of `options_requested_policy_version`.
    pub fn set_options_requested_policy_version<T: Into<Option<i32>>>(mut self, v: T) -> Self {
        self.options_requested_policy_version = v.into();
        self
    }
}
