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

/// The Google Cloud client for the Google RPC Types.
///
/// Defines RPC types.
library;


/// Describes the cause of the error with structured details.
/// 
/// Example of an error when contacting the "pubsub.googleapis.com" API when it
/// is not enabled:
/// 
///     { "reason": "API_DISABLED"
///       "domain": "googleapis.com"
///       "metadata": {
///         "resource": "projects/123",
///         "service": "pubsub.googleapis.com"
///       }
///     }
/// 
/// This response indicates that the pubsub.googleapis.com API is not enabled.
/// 
/// Example of an error that is returned when attempting to create a Spanner
/// instance in a region that is out of stock:
/// 
///     { "reason": "STOCKOUT"
///       "domain": "spanner.googleapis.com",
///       "metadata": {
///         "availableRegions": "us-central1,us-east2"
///       }
///     }
class ErrorInfo {
  // TODO:
}

/// Describes when the clients can retry a failed request. Clients could ignore
/// the recommendation here or retry when this information is missing from error
/// responses.
/// 
/// It's always recommended that clients should use exponential backoff when
/// retrying.
/// 
/// Clients should wait until `retry_delay` amount of time has passed since
/// receiving the error response before retrying.  If retrying requests also
/// fail, clients should use an exponential backoff scheme to gradually increase
/// the delay between retries based on `retry_delay`, until either a maximum
/// number of retries have been reached or a maximum retry delay cap has been
/// reached.
class RetryInfo {
  // TODO:
}

/// Describes additional debugging info.
class DebugInfo {
  // TODO:
}

/// Describes how a quota check failed.
/// 
/// For example if a daily limit was exceeded for the calling project,
/// a service could respond with a QuotaFailure detail containing the project
/// id and the description of the quota limit that was exceeded.  If the
/// calling project hasn't enabled the service in the developer console, then
/// a service could respond with the project id and set `service_disabled`
/// to true.
/// 
/// Also see RetryInfo and Help types for other details about handling a
/// quota failure.
class QuotaFailure {
  // TODO:
}

/// A message type used to describe a single quota violation.  For example, a
/// daily quota or a custom quota that was exceeded.
class QuotaFailure$Violation {
  // TODO:
}

/// Describes what preconditions have failed.
/// 
/// For example, if an RPC failed because it required the Terms of Service to be
/// acknowledged, it could list the terms of service violation in the
/// PreconditionFailure message.
class PreconditionFailure {
  // TODO:
}

/// A message type used to describe a single precondition failure.
class PreconditionFailure$Violation {
  // TODO:
}

/// Describes violations in a client request. This error type focuses on the
/// syntactic aspects of the request.
class BadRequest {
  // TODO:
}

/// A message type used to describe a single bad request field.
class BadRequest$FieldViolation {
  // TODO:
}

/// Contains metadata about the request that clients can attach when filing a bug
/// or providing other forms of feedback.
class RequestInfo {
  // TODO:
}

/// Describes the resource that is being accessed.
class ResourceInfo {
  // TODO:
}

/// Provides links to documentation or for performing an out of band action.
/// 
/// For example, if a quota check failed with an error indicating the calling
/// project hasn't enabled the accessed service, this can contain a URL pointing
/// directly to the right place in the developer console to flip the bit.
class Help {
  // TODO:
}

/// Describes a URL link.
class Help$Link {
  // TODO:
}

/// Provides a localized error message that is safe to return to the user
/// which can be attached to an RPC error.
class LocalizedMessage {
  // TODO:
}

/// Represents an HTTP request.
class HttpRequest {
  // TODO:
}

/// Represents an HTTP response.
class HttpResponse {
  // TODO:
}

/// Represents an HTTP header.
class HttpHeader {
  // TODO:
}

/// The `Status` type defines a logical error model that is suitable for
/// different programming environments, including REST APIs and RPC APIs. It is
/// used by [gRPC](https://github.com/grpc). Each `Status` message contains
/// three pieces of data: error code, error message, and error details.
/// 
/// You can find out more about this error model and how to work with it in the
/// [API Design Guide](https://cloud.google.com/apis/design/errors).
class Status {
  // TODO:
}

/// The canonical error codes for gRPC APIs.
/// 
/// 
/// Sometimes multiple error codes may apply.  Services should return
/// the most specific error code that applies.  For example, prefer
/// `OUT_OF_RANGE` over `FAILED_PRECONDITION` if both codes apply.
/// Similarly prefer `NOT_FOUND` or `ALREADY_EXISTS` over `FAILED_PRECONDITION`.
class Code {
  /// Not an error; returned on success.
  /// 
  /// HTTP Mapping: 200 OK
  static const Code ok = Code('OK');

  /// The operation was cancelled, typically by the caller.
  /// 
  /// HTTP Mapping: 499 Client Closed Request
  static const Code cancelled = Code('CANCELLED');

  /// Unknown error.  For example, this error may be returned when
  /// a `Status` value received from another address space belongs to
  /// an error space that is not known in this address space.  Also
  /// errors raised by APIs that do not return enough error information
  /// may be converted to this error.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code unknown = Code('UNKNOWN');

  /// The client specified an invalid argument.  Note that this differs
  /// from `FAILED_PRECONDITION`.  `INVALID_ARGUMENT` indicates arguments
  /// that are problematic regardless of the state of the system
  /// (e.g., a malformed file name).
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code invalidArgument = Code('INVALID_ARGUMENT');

  /// The deadline expired before the operation could complete. For operations
  /// that change the state of the system, this error may be returned
  /// even if the operation has completed successfully.  For example, a
  /// successful response from a server could have been delayed long
  /// enough for the deadline to expire.
  /// 
  /// HTTP Mapping: 504 Gateway Timeout
  static const Code deadlineExceeded = Code('DEADLINE_EXCEEDED');

  /// Some requested entity (e.g., file or directory) was not found.
  /// 
  /// Note to server developers: if a request is denied for an entire class
  /// of users, such as gradual feature rollout or undocumented allowlist,
  /// `NOT_FOUND` may be used. If a request is denied for some users within
  /// a class of users, such as user-based access control, `PERMISSION_DENIED`
  /// must be used.
  /// 
  /// HTTP Mapping: 404 Not Found
  static const Code notFound = Code('NOT_FOUND');

  /// The entity that a client attempted to create (e.g., file or directory)
  /// already exists.
  /// 
  /// HTTP Mapping: 409 Conflict
  static const Code alreadyExists = Code('ALREADY_EXISTS');

  /// The caller does not have permission to execute the specified
  /// operation. `PERMISSION_DENIED` must not be used for rejections
  /// caused by exhausting some resource (use `RESOURCE_EXHAUSTED`
  /// instead for those errors). `PERMISSION_DENIED` must not be
  /// used if the caller can not be identified (use `UNAUTHENTICATED`
  /// instead for those errors). This error code does not imply the
  /// request is valid or the requested entity exists or satisfies
  /// other pre-conditions.
  /// 
  /// HTTP Mapping: 403 Forbidden
  static const Code permissionDenied = Code('PERMISSION_DENIED');

  /// The request does not have valid authentication credentials for the
  /// operation.
  /// 
  /// HTTP Mapping: 401 Unauthorized
  static const Code unauthenticated = Code('UNAUTHENTICATED');

  /// Some resource has been exhausted, perhaps a per-user quota, or
  /// perhaps the entire file system is out of space.
  /// 
  /// HTTP Mapping: 429 Too Many Requests
  static const Code resourceExhausted = Code('RESOURCE_EXHAUSTED');

  /// The operation was rejected because the system is not in a state
  /// required for the operation's execution.  For example, the directory
  /// to be deleted is non-empty, an rmdir operation is applied to
  /// a non-directory, etc.
  /// 
  /// Service implementors can use the following guidelines to decide
  /// between `FAILED_PRECONDITION`, `ABORTED`, and `UNAVAILABLE`:
  ///  (a) Use `UNAVAILABLE` if the client can retry just the failing call.
  ///  (b) Use `ABORTED` if the client should retry at a higher level. For
  ///      example, when a client-specified test-and-set fails, indicating the
  ///      client should restart a read-modify-write sequence.
  ///  (c) Use `FAILED_PRECONDITION` if the client should not retry until
  ///      the system state has been explicitly fixed. For example, if an "rmdir"
  ///      fails because the directory is non-empty, `FAILED_PRECONDITION`
  ///      should be returned since the client should not retry unless
  ///      the files are deleted from the directory.
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code failedPrecondition = Code('FAILED_PRECONDITION');

  /// The operation was aborted, typically due to a concurrency issue such as
  /// a sequencer check failure or transaction abort.
  /// 
  /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
  /// `ABORTED`, and `UNAVAILABLE`.
  /// 
  /// HTTP Mapping: 409 Conflict
  static const Code aborted = Code('ABORTED');

  /// The operation was attempted past the valid range.  E.g., seeking or
  /// reading past end-of-file.
  /// 
  /// Unlike `INVALID_ARGUMENT`, this error indicates a problem that may
  /// be fixed if the system state changes. For example, a 32-bit file
  /// system will generate `INVALID_ARGUMENT` if asked to read at an
  /// offset that is not in the range [0,2^32-1], but it will generate
  /// `OUT_OF_RANGE` if asked to read from an offset past the current
  /// file size.
  /// 
  /// There is a fair bit of overlap between `FAILED_PRECONDITION` and
  /// `OUT_OF_RANGE`.  We recommend using `OUT_OF_RANGE` (the more specific
  /// error) when it applies so that callers who are iterating through
  /// a space can easily look for an `OUT_OF_RANGE` error to detect when
  /// they are done.
  /// 
  /// HTTP Mapping: 400 Bad Request
  static const Code outOfRange = Code('OUT_OF_RANGE');

  /// The operation is not implemented or is not supported/enabled in this
  /// service.
  /// 
  /// HTTP Mapping: 501 Not Implemented
  static const Code unimplemented = Code('UNIMPLEMENTED');

  /// Internal errors.  This means that some invariants expected by the
  /// underlying system have been broken.  This error code is reserved
  /// for serious errors.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code internal = Code('INTERNAL');

  /// The service is currently unavailable.  This is most likely a
  /// transient condition, which can be corrected by retrying with
  /// a backoff. Note that it is not always safe to retry
  /// non-idempotent operations.
  /// 
  /// See the guidelines above for deciding between `FAILED_PRECONDITION`,
  /// `ABORTED`, and `UNAVAILABLE`.
  /// 
  /// HTTP Mapping: 503 Service Unavailable
  static const Code unavailable = Code('UNAVAILABLE');

  /// Unrecoverable data loss or corruption.
  /// 
  /// HTTP Mapping: 500 Internal Server Error
  static const Code dataLoss = Code('DATA_LOSS');

  final String value;

  const Code(this.value);
}
