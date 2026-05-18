//! Shared request-workspace alpha truth contracts.
//!
//! This module contains the smaller value objects used by the request
//! workspace runtime record: endpoint identity, environment fingerprints,
//! auth-source classification, assertion-suite lineage, response safe-preview
//! rules, local history posture, and portable export posture. Keeping these
//! contracts separate lets UI, CLI, and support-export surfaces reuse the same
//! vocabulary without depending on seeded scenario builders.

use serde::{Deserialize, Serialize};

/// Schema identifier for the environment-fingerprint boundary.
pub const REQUEST_ENVIRONMENT_FINGERPRINT_SCHEMA_ID: &str =
    "https://aureline.dev/schemas/request_workspace/request_environment_fingerprint.schema.json";

/// Schema identifier for the assertion-suite boundary.
pub const REQUEST_ASSERTION_SUITE_SCHEMA_ID: &str =
    "https://aureline.dev/schemas/request_workspace/request_assertion_suite.schema.json";

/// Schema identifier for the response-preview boundary.
pub const REQUEST_RESPONSE_PREVIEW_SCHEMA_ID: &str =
    "https://aureline.dev/schemas/request_workspace/request_response_preview.schema.json";

/// Source used to establish the endpoint identity bound to a request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointSourceClass {
    /// Endpoint identity came from the resolved environment layer.
    EnvironmentLayer,
    /// Endpoint identity came from the authored request template.
    RequestTemplate,
    /// Endpoint identity came from a mirrored schema or contract snapshot.
    MirroredSchema,
    /// Endpoint identity came from an imported example or support bundle.
    ImportedExample,
    /// Endpoint identity is unknown and must be reviewed before dispatch.
    UnknownRequiresReview,
}

impl EndpointSourceClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvironmentLayer => "environment_layer",
            Self::RequestTemplate => "request_template",
            Self::MirroredSchema => "mirrored_schema",
            Self::ImportedExample => "imported_example",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Endpoint identity attached to a request before dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointIdentity {
    /// Opaque endpoint identity reference safe for support exports.
    pub endpoint_identity_ref: String,
    /// Human-safe alias rendered by UI, CLI, and support export surfaces.
    pub endpoint_alias: String,
    /// Target endpoint id shared with schema snapshots.
    pub target_endpoint_id: String,
    /// Source class used to establish the endpoint identity.
    pub source_class: EndpointSourceClass,
    /// Stable source-class token.
    pub source_class_token: String,
    /// True when a request can move to another endpoint without a visible
    /// retarget review.
    pub silent_retarget_allowed: bool,
}

impl EndpointIdentity {
    /// Builds an endpoint identity with the canonical source-class token.
    pub fn new(
        endpoint_identity_ref: impl Into<String>,
        endpoint_alias: impl Into<String>,
        target_endpoint_id: impl Into<String>,
        source_class: EndpointSourceClass,
        silent_retarget_allowed: bool,
    ) -> Self {
        Self {
            endpoint_identity_ref: endpoint_identity_ref.into(),
            endpoint_alias: endpoint_alias.into(),
            target_endpoint_id: target_endpoint_id.into(),
            source_class,
            source_class_token: source_class.as_str().to_owned(),
            silent_retarget_allowed,
        }
    }
}

/// Digest algorithm or identity class used for an environment fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FingerprintDigestClass {
    /// The fingerprint was produced with SHA-256.
    Sha256,
    /// The fingerprint was produced with BLAKE3.
    Blake3,
    /// The fingerprint is an opaque stable handle from another trusted store.
    OpaqueStable,
}

impl FingerprintDigestClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Sha256 => "sha256",
            Self::Blake3 => "blake3",
            Self::OpaqueStable => "opaque_stable",
        }
    }
}

/// Resolution state for an environment fingerprint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentFingerprintState {
    /// Fingerprint was resolved locally for the current run.
    CurrentLocalResolution,
    /// Fingerprint was imported from an explicit export bundle.
    ImportedFromExport,
    /// Fingerprint is stale and must be reviewed before trust claims.
    StaleRequiresReview,
    /// Fingerprint cannot resolve because an environment variable is missing.
    BlockedUnresolvedVariable,
}

impl EnvironmentFingerprintState {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocalResolution => "current_local_resolution",
            Self::ImportedFromExport => "imported_from_export",
            Self::StaleRequiresReview => "stale_requires_review",
            Self::BlockedUnresolvedVariable => "blocked_unresolved_variable",
        }
    }
}

/// Fingerprint over the resolved request environment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestEnvironmentFingerprint {
    /// Opaque fingerprint reference shared across UI, CLI, and export lanes.
    pub fingerprint_ref: String,
    /// Digest or identity class used to create the fingerprint.
    pub digest_class: FingerprintDigestClass,
    /// Stable digest-class token.
    pub digest_class_token: String,
    /// Environment id whose resolved layers were fingerprinted.
    pub environment_id: String,
    /// Endpoint identity bound when the fingerprint was computed.
    pub endpoint_identity_ref: String,
    /// Layer refs included in the fingerprint in canonical order.
    pub layer_refs: Vec<String>,
    /// Resolution state for the fingerprint.
    pub state: EnvironmentFingerprintState,
    /// Stable state token.
    pub state_token: String,
    /// Timestamp supplied by the caller.
    pub captured_at: String,
}

impl RequestEnvironmentFingerprint {
    /// Builds an environment fingerprint with canonical tokens.
    pub fn new(
        fingerprint_ref: impl Into<String>,
        digest_class: FingerprintDigestClass,
        environment_id: impl Into<String>,
        endpoint_identity_ref: impl Into<String>,
        layer_refs: impl IntoIterator<Item = impl Into<String>>,
        state: EnvironmentFingerprintState,
        captured_at: impl Into<String>,
    ) -> Self {
        Self {
            fingerprint_ref: fingerprint_ref.into(),
            digest_class,
            digest_class_token: digest_class.as_str().to_owned(),
            environment_id: environment_id.into(),
            endpoint_identity_ref: endpoint_identity_ref.into(),
            layer_refs: layer_refs.into_iter().map(Into::into).collect(),
            state,
            state_token: state.as_str().to_owned(),
            captured_at: captured_at.into(),
        }
    }
}

/// Auth-source class used to explain where request credentials come from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceClass {
    /// Request uses no authentication material.
    NoAuth,
    /// Request resolves credentials through a secret-broker handle.
    SecretBrokerHandle,
    /// Request uses a delegated identity rather than a raw credential.
    DelegatedIdentity,
    /// Request receives credentials from managed policy injection.
    PolicyInjected,
    /// Request resolves an mTLS certificate handle.
    MtlsCertificateHandle,
    /// Request resolves a signing-key handle.
    SignedRequestHandle,
    /// Raw inline credential material was observed and dispatch is blocked.
    RawInlineDisallowed,
    /// Auth source is unsupported and dispatch is blocked.
    UnsupportedBlocked,
}

impl AuthSourceClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoAuth => "no_auth",
            Self::SecretBrokerHandle => "secret_broker_handle",
            Self::DelegatedIdentity => "delegated_identity",
            Self::PolicyInjected => "policy_injected",
            Self::MtlsCertificateHandle => "mtls_certificate_handle",
            Self::SignedRequestHandle => "signed_request_handle",
            Self::RawInlineDisallowed => "raw_inline_disallowed",
            Self::UnsupportedBlocked => "unsupported_blocked",
        }
    }

    /// True when the source class can be exported without raw credential
    /// material.
    pub const fn is_portable_without_secret_material(self) -> bool {
        !matches!(self, Self::RawInlineDisallowed | Self::UnsupportedBlocked)
    }
}

/// Lineage class for a request assertion suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionSuiteLineageClass {
    /// Assertion suite was authored in the current local workspace.
    CurrentLocal,
    /// Assertion suite came from an imported example or support bundle.
    ImportedArtifact,
    /// Assertion suite is stale relative to the bound schema or endpoint.
    StaleArtifact,
    /// Assertion suite was mirrored from a schema or contract source.
    MirroredSchema,
}

impl AssertionSuiteLineageClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocal => "current_local",
            Self::ImportedArtifact => "imported_artifact",
            Self::StaleArtifact => "stale_artifact",
            Self::MirroredSchema => "mirrored_schema",
        }
    }
}

/// Typed assertion suite that binds descriptors to request evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssertionSuite<T> {
    /// Opaque suite id.
    pub suite_id: String,
    /// Human-safe suite alias rendered in UI and support exports.
    pub suite_alias: String,
    /// Lineage class for the suite.
    pub lineage_class: AssertionSuiteLineageClass,
    /// Stable lineage-class token.
    pub lineage_class_token: String,
    /// Descriptor rows in canonical evaluation order.
    pub assertions: Vec<T>,
}

impl<T> AssertionSuite<T> {
    /// Builds an assertion suite with the canonical lineage token.
    pub fn new(
        suite_id: impl Into<String>,
        suite_alias: impl Into<String>,
        lineage_class: AssertionSuiteLineageClass,
        assertions: Vec<T>,
    ) -> Self {
        Self {
            suite_id: suite_id.into(),
            suite_alias: suite_alias.into(),
            lineage_class,
            lineage_class_token: lineage_class.as_str().to_owned(),
            assertions,
        }
    }
}

/// Evidence state for one assertion result row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionEvidenceState {
    /// Result was evaluated for the current local run.
    CurrentLocalRun,
    /// Result was imported from another run or export bundle.
    ImportedRun,
    /// Result is stale relative to the current request or schema snapshot.
    StaleImportedArtifact,
    /// Result did not execute and carries no runtime evidence.
    NotExecuted,
}

impl AssertionEvidenceState {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentLocalRun => "current_local_run",
            Self::ImportedRun => "imported_run",
            Self::StaleImportedArtifact => "stale_imported_artifact",
            Self::NotExecuted => "not_executed",
        }
    }
}

/// Response component whose preview/copy/export posture is being described.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsePreviewComponentClass {
    /// Response body bytes or structured body view.
    Body,
    /// Response header names and safe header values.
    Headers,
    /// Response cookies.
    Cookies,
    /// Token-like values detected in response material.
    Tokens,
    /// Payload-size summary for oversized or streaming responses.
    LargePayload,
}

impl ResponsePreviewComponentClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Body => "body",
            Self::Headers => "headers",
            Self::Cookies => "cookies",
            Self::Tokens => "tokens",
            Self::LargePayload => "large_payload",
        }
    }
}

/// Safe-preview class applied to one response component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseSafePreviewClass {
    /// Structured JSON tree view.
    JsonTree,
    /// Raw text view safe under content-integrity rules.
    RawText,
    /// Sanitized rich rendering with active content disabled.
    SanitizedRich,
    /// Metadata-only view with values omitted.
    MetadataOnly,
    /// Digest-only view for sensitive or oversized payloads.
    DigestOnly,
    /// Large-payload summary view.
    LargePayloadSummary,
    /// Redacted component view.
    Redacted,
}

impl ResponseSafePreviewClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonTree => "json_tree",
            Self::RawText => "raw_text",
            Self::SanitizedRich => "sanitized_rich",
            Self::MetadataOnly => "metadata_only",
            Self::DigestOnly => "digest_only",
            Self::LargePayloadSummary => "large_payload_summary",
            Self::Redacted => "redacted",
        }
    }
}

/// Copy/export behavior for one response representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseCopyExportClass {
    /// Copy/export redacted material with the label preserved.
    RedactedWithLabel,
    /// Copy/export structured summary material with the label preserved.
    StructuredSummaryWithLabel,
    /// Export digest-only material with the label preserved.
    DigestOnlyWithLabel,
    /// Raw material is local-only and requires explicit opt-in.
    LocalOnlyRawOptIn,
    /// Copy/export is blocked because the component is sensitive.
    BlockedSensitive,
}

impl ResponseCopyExportClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedWithLabel => "redacted_with_label",
            Self::StructuredSummaryWithLabel => "structured_summary_with_label",
            Self::DigestOnlyWithLabel => "digest_only_with_label",
            Self::LocalOnlyRawOptIn => "local_only_raw_opt_in",
            Self::BlockedSensitive => "blocked_sensitive",
        }
    }

    /// True when the class can appear in a portable export by default.
    pub const fn is_default_portable(self) -> bool {
        matches!(
            self,
            Self::RedactedWithLabel
                | Self::StructuredSummaryWithLabel
                | Self::DigestOnlyWithLabel
                | Self::BlockedSensitive
        )
    }
}

/// Payload-size class for response retention and preview decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsePayloadSizeClass {
    /// Payload is small enough for normal structured preview.
    Small,
    /// Payload is medium and should remain virtualized.
    MediumVirtualized,
    /// Payload is large and should open in summary or limited mode.
    LargeSafePreviewOnly,
    /// Payload size is unknown because the response is streaming or partial.
    UnknownStreaming,
}

impl ResponsePayloadSizeClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::MediumVirtualized => "medium_virtualized",
            Self::LargeSafePreviewOnly => "large_safe_preview_only",
            Self::UnknownStreaming => "unknown_streaming",
        }
    }
}

/// Safe preview and representation-labeled copy/export rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsePreviewRule {
    /// Response component the rule covers.
    pub component: ResponsePreviewComponentClass,
    /// Stable component token.
    pub component_token: String,
    /// Safe-preview class for the component.
    pub safe_preview_class: ResponseSafePreviewClass,
    /// Stable safe-preview-class token.
    pub safe_preview_class_token: String,
    /// Label that must travel with copy/export actions.
    pub representation_label: String,
    /// Copy/export behavior for the component.
    pub copy_export_class: ResponseCopyExportClass,
    /// Stable copy/export-class token.
    pub copy_export_class_token: String,
}

impl ResponsePreviewRule {
    /// Builds a response-preview rule with canonical tokens.
    pub fn new(
        component: ResponsePreviewComponentClass,
        safe_preview_class: ResponseSafePreviewClass,
        representation_label: impl Into<String>,
        copy_export_class: ResponseCopyExportClass,
    ) -> Self {
        Self {
            component,
            component_token: component.as_str().to_owned(),
            safe_preview_class,
            safe_preview_class_token: safe_preview_class.as_str().to_owned(),
            representation_label: representation_label.into(),
            copy_export_class,
            copy_export_class_token: copy_export_class.as_str().to_owned(),
        }
    }
}

/// Local history retention class for request runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestHistoryRetentionClass {
    /// Local history keeps structured metadata and can redact retained data.
    LocalOnlyRedactable,
    /// Local history omits body material at rest by default.
    LocalOnlyBodyOmitted,
    /// Imported history is read-only and cannot be treated as current truth.
    ImportedReadOnly,
    /// Export carries summary and evidence refs only.
    ExportSummaryOnly,
}

impl RequestHistoryRetentionClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyRedactable => "local_only_redactable",
            Self::LocalOnlyBodyOmitted => "local_only_body_omitted",
            Self::ImportedReadOnly => "imported_read_only",
            Self::ExportSummaryOnly => "export_summary_only",
        }
    }
}

/// History posture attached to a request-workspace run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestHistoryPosture {
    /// Retention class for the run.
    pub retention_class: RequestHistoryRetentionClass,
    /// Stable retention-class token.
    pub retention_class_token: String,
    /// True when history sync is enabled by default.
    pub sync_enabled_by_default: bool,
    /// True when raw payloads are retained by default.
    pub raw_payload_retained_by_default: bool,
    /// True when the history item can be redacted after capture.
    pub redactable: bool,
}

impl RequestHistoryPosture {
    /// Builds a history posture with the canonical retention token.
    pub fn new(
        retention_class: RequestHistoryRetentionClass,
        sync_enabled_by_default: bool,
        raw_payload_retained_by_default: bool,
        redactable: bool,
    ) -> Self {
        Self {
            retention_class,
            retention_class_token: retention_class.as_str().to_owned(),
            sync_enabled_by_default,
            raw_payload_retained_by_default,
            redactable,
        }
    }

    /// Default posture for local-first request history.
    pub fn local_redactable() -> Self {
        Self::new(
            RequestHistoryRetentionClass::LocalOnlyRedactable,
            false,
            false,
            true,
        )
    }
}

/// Portable export class for request-workspace evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableExportClass {
    /// Export includes metadata, aliases, posture, and evidence refs only.
    MetadataAliasesAndEvidence,
    /// Export includes a redacted response summary.
    RedactedResponseSummary,
    /// Export preserves an imported stale artifact as read-only evidence.
    ImportedStaleEvidenceOnly,
    /// Raw export is local-only and disabled by default.
    LocalOnlyRawOptInBlocked,
}

impl PortableExportClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataAliasesAndEvidence => "metadata_aliases_and_evidence",
            Self::RedactedResponseSummary => "redacted_response_summary",
            Self::ImportedStaleEvidenceOnly => "imported_stale_evidence_only",
            Self::LocalOnlyRawOptInBlocked => "local_only_raw_opt_in_blocked",
        }
    }
}

/// Portable export posture for one request-workspace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableExportContract {
    /// Portable export class.
    pub export_class: PortableExportClass,
    /// Stable export-class token.
    pub export_class_token: String,
    /// Alias bundle ref that lets importers remap local labels safely.
    pub alias_bundle_ref: String,
    /// Redaction profile applied to the export.
    pub redaction_profile: String,
    /// Evidence refs carried by the export.
    pub evidence_refs: Vec<String>,
    /// True when raw credential material is included.
    pub includes_raw_credentials: bool,
    /// True when raw cookie material is included.
    pub includes_raw_cookies: bool,
    /// True when raw token material is included.
    pub includes_raw_tokens: bool,
}

impl PortableExportContract {
    /// Builds a portable export contract with canonical tokens.
    pub fn new(
        export_class: PortableExportClass,
        alias_bundle_ref: impl Into<String>,
        redaction_profile: impl Into<String>,
        evidence_refs: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        Self {
            export_class,
            export_class_token: export_class.as_str().to_owned(),
            alias_bundle_ref: alias_bundle_ref.into(),
            redaction_profile: redaction_profile.into(),
            evidence_refs: evidence_refs.into_iter().map(Into::into).collect(),
            includes_raw_credentials: false,
            includes_raw_cookies: false,
            includes_raw_tokens: false,
        }
    }

    /// True when the export omits all raw credential, cookie, and token
    /// material by default.
    pub const fn excludes_raw_secret_material(&self) -> bool {
        !self.includes_raw_credentials && !self.includes_raw_cookies && !self.includes_raw_tokens
    }
}
