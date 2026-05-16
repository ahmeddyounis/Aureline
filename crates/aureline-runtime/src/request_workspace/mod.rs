//! Request-workspace alpha contract.
//!
//! One typed workspace for API / GraphQL request workflows so the product can
//! claim request execution, inspectability, and export parity without
//! inventing a second runtime truth model. The canonical record bundles the
//! request document, the layered environment set, the auth/credential class,
//! the assertion suite, the optional response artifact, and the schema
//! snapshot for one workspace row, and binds them all to one canonical
//! [`ExecutionContext`] reference via `execution_context_ref` and
//! `target_class`.
//!
//! UI and CLI/headless surfaces consume the same [`RequestWorkspaceAlphaRecord`]
//! and project the same [`SendInspectorReport`]; the support-export wrapper
//! is the only object reviewer / support flows need to reopen or compare a
//! request-workspace run truthfully.
//!
//! The cross-tool boundary schema lives at
//! [`/schemas/runtime/request_workspace.schema.json`](../../../../schemas/runtime/request_workspace.schema.json).
//! The reviewer-facing landing page is
//! [`/docs/runtime/m3/request_workspace_alpha.md`](../../../../docs/runtime/m3/request_workspace_alpha.md).

use serde::{Deserialize, Serialize};

use crate::execution_context::TargetClass;

/// Schema version of the request-workspace alpha lane records.
pub const REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable lane id for this bounded alpha contract.
pub const REQUEST_WORKSPACE_ALPHA_LANE_ID: &str = "request_workspace_alpha";

/// Stable record-kind tag for [`RequestWorkspaceAlphaRecord`] payloads.
pub const REQUEST_WORKSPACE_ALPHA_RECORD_KIND: &str = "request_workspace_alpha_record";

/// Stable record-kind tag for [`SendInspectorReport`] payloads.
pub const REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND: &str =
    "request_workspace_send_inspector_record";

/// Stable record-kind tag for [`RequestWorkspaceSupportExport`] payloads.
pub const REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "request_workspace_support_export_record";

/// Closed request-method vocabulary used by both HTTP and GraphQL request
/// documents. GraphQL operations carry their operation kind in
/// [`RequestMethodClass::GraphqlOperation`] so the same envelope can describe
/// HTTP and GraphQL workflows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestMethodClass {
    Get,
    Head,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    GraphqlOperation,
}

impl RequestMethodClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Head => "head",
            Self::Post => "post",
            Self::Put => "put",
            Self::Patch => "patch",
            Self::Delete => "delete",
            Self::Options => "options",
            Self::GraphqlOperation => "graphql_operation",
        }
    }

    /// True when the method is idempotent under HTTP semantics. Used by the
    /// send-inspector to derive the default expected side-effect band.
    pub const fn is_idempotent(self) -> bool {
        matches!(
            self,
            Self::Get | Self::Head | Self::Put | Self::Delete | Self::Options
        )
    }

    /// True when the method is read-only under HTTP semantics. GraphQL
    /// operations never default to read-only because the runtime cannot
    /// inspect the query body locally.
    pub const fn is_read_only(self) -> bool {
        matches!(self, Self::Get | Self::Head | Self::Options)
    }
}

/// Source layer for one resolved environment variable. The send inspector
/// renders the layer attribution so users can see which value came from a
/// request file, a workspace/profile default, a policy injection, an ad-hoc
/// override, or a secret-broker handle without re-deriving precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentLayerKind {
    RequestFile,
    WorkspaceDefault,
    ProfileDefault,
    PolicyInjection,
    AdHocOverride,
    SecretHandle,
}

impl EnvironmentLayerKind {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestFile => "request_file",
            Self::WorkspaceDefault => "workspace_default",
            Self::ProfileDefault => "profile_default",
            Self::PolicyInjection => "policy_injection",
            Self::AdHocOverride => "ad_hoc_override",
            Self::SecretHandle => "secret_handle",
        }
    }
}

/// One layered variable resolved into the environment set. Secret-handle
/// layers MUST set `is_secret_handle = true` and MUST NOT carry a value
/// token; the inspector renders an opaque marker instead.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentVariableLayer {
    /// Variable name as it appears in the resolved environment.
    pub variable_name: String,
    /// Source layer kind.
    pub layer_kind: EnvironmentLayerKind,
    /// Stable layer-kind token.
    pub layer_kind_token: String,
    /// Opaque ref into the source layer (request file id, profile id, policy
    /// id, override session id, or secret handle id).
    pub source_ref: String,
    /// Resolved value token. MUST be `None` when [`Self::is_secret_handle`]
    /// is `true` so secrets never leak through fixtures or support exports.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    /// True when the layer resolves through a secret-broker handle.
    pub is_secret_handle: bool,
}

impl EnvironmentVariableLayer {
    /// Construct a non-secret layer with a plain value token.
    pub fn plain(
        variable_name: impl Into<String>,
        layer_kind: EnvironmentLayerKind,
        source_ref: impl Into<String>,
        value_token: impl Into<String>,
    ) -> Self {
        Self {
            variable_name: variable_name.into(),
            layer_kind,
            layer_kind_token: layer_kind.as_str().to_owned(),
            source_ref: source_ref.into(),
            value_token: Some(value_token.into()),
            is_secret_handle: false,
        }
    }

    /// Construct a secret-handle layer with no value token.
    pub fn secret_handle(
        variable_name: impl Into<String>,
        source_ref: impl Into<String>,
    ) -> Self {
        Self {
            variable_name: variable_name.into(),
            layer_kind: EnvironmentLayerKind::SecretHandle,
            layer_kind_token: EnvironmentLayerKind::SecretHandle.as_str().to_owned(),
            source_ref: source_ref.into(),
            value_token: None,
            is_secret_handle: true,
        }
    }
}

/// Layered environment set resolved before send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentSet {
    /// Opaque environment id.
    pub environment_id: String,
    /// Scope label (e.g. `workspace.profile.dev`).
    pub scope_label: String,
    /// Resolved base URL template.
    pub base_url_template: String,
    /// Layered variable rows in canonical render order.
    #[serde(default)]
    pub layered_variables: Vec<EnvironmentVariableLayer>,
    /// Effective fingerprint over the layered variable set. Used by the
    /// send inspector to prove the environment that backs a send.
    pub effective_fingerprint: String,
}

impl EnvironmentSet {
    /// True when any resolved layer is a secret-handle reference.
    pub fn has_secret_handle(&self) -> bool {
        self.layered_variables
            .iter()
            .any(|layer| layer.is_secret_handle)
    }

    /// Returns the secret-handle source refs in canonical order.
    pub fn secret_handle_refs(&self) -> Vec<String> {
        self.layered_variables
            .iter()
            .filter(|layer| layer.is_secret_handle)
            .map(|layer| layer.source_ref.clone())
            .collect()
    }
}

/// Closed auth-strategy vocabulary. The strategy kind is rendered by the
/// send inspector alongside the credential class so reviewers see both
/// "which scheme" and "how credentials reach the request".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthStrategyKind {
    None,
    BearerBroker,
    BasicBroker,
    Oauth2Broker,
    ApiKeyBroker,
    MutualTls,
    SignedRequest,
}

impl AuthStrategyKind {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::BearerBroker => "bearer_broker",
            Self::BasicBroker => "basic_broker",
            Self::Oauth2Broker => "oauth2_broker",
            Self::ApiKeyBroker => "api_key_broker",
            Self::MutualTls => "mutual_tls",
            Self::SignedRequest => "signed_request",
        }
    }
}

/// Credential-class vocabulary that answers "how does credential material
/// reach the request?". Raw inline credentials in workspace files are
/// disallowed: the record carries `raw_inline_disallowed` as an explicit
/// violation marker so the send inspector can block dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CredentialClass {
    NoCredentials,
    BrokerHandle,
    DelegatedIdentity,
    MtlsCertificate,
    PolicyInjectedToken,
    RawInlineDisallowed,
}

impl CredentialClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoCredentials => "no_credentials",
            Self::BrokerHandle => "broker_handle",
            Self::DelegatedIdentity => "delegated_identity",
            Self::MtlsCertificate => "mtls_certificate",
            Self::PolicyInjectedToken => "policy_injected_token",
            Self::RawInlineDisallowed => "raw_inline_disallowed",
        }
    }

    /// True when the credential class is safe enough to dispatch without
    /// explicit reauthorisation review.
    pub const fn is_safe_to_dispatch(self) -> bool {
        matches!(
            self,
            Self::NoCredentials
                | Self::BrokerHandle
                | Self::DelegatedIdentity
                | Self::MtlsCertificate
                | Self::PolicyInjectedToken
        )
    }
}

/// Auth profile rendered next to the send inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthProfile {
    /// Strategy kind.
    pub strategy_kind: AuthStrategyKind,
    /// Stable strategy-kind token.
    pub strategy_kind_token: String,
    /// Credential class.
    pub credential_class: CredentialClass,
    /// Stable credential-class token.
    pub credential_class_token: String,
    /// Broker handle refs in canonical order (empty when no credential
    /// material is required).
    #[serde(default)]
    pub broker_handle_refs: Vec<String>,
    /// Opaque ref into refresh/challenge metadata when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_metadata_ref: Option<String>,
}

impl AuthProfile {
    /// Build a no-credential profile.
    pub fn none() -> Self {
        Self::new(
            AuthStrategyKind::None,
            CredentialClass::NoCredentials,
            Vec::<String>::new(),
            None,
        )
    }

    /// Build a profile with the given strategy and credential class.
    pub fn new(
        strategy_kind: AuthStrategyKind,
        credential_class: CredentialClass,
        broker_handle_refs: impl IntoIterator<Item = impl Into<String>>,
        refresh_metadata_ref: Option<String>,
    ) -> Self {
        Self {
            strategy_kind,
            strategy_kind_token: strategy_kind.as_str().to_owned(),
            credential_class,
            credential_class_token: credential_class.as_str().to_owned(),
            broker_handle_refs: broker_handle_refs
                .into_iter()
                .map(Into::into)
                .collect(),
            refresh_metadata_ref,
        }
    }
}

/// Closed request-document vocabulary describing one HTTP / GraphQL request
/// authored in the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestDocument {
    /// Opaque request id.
    pub request_id: String,
    /// Method or GraphQL operation class.
    pub method: RequestMethodClass,
    /// Stable method token.
    pub method_token: String,
    /// URL or path template (e.g. `{{api_base}}/v1/payments/refund`).
    pub url_template: String,
    /// Header-bundle refs (refs into the workspace, not raw header bodies).
    #[serde(default)]
    pub header_refs: Vec<String>,
    /// Optional body-document ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_ref: Option<String>,
    /// Assertion refs that bind to this request.
    #[serde(default)]
    pub assertion_refs: Vec<String>,
    /// Collection / tag refs for this request.
    #[serde(default)]
    pub collection_tags: Vec<String>,
}

impl RequestDocument {
    /// Build a request-document row with the canonical method token derived
    /// from `method`.
    pub fn new(
        request_id: impl Into<String>,
        method: RequestMethodClass,
        url_template: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            method,
            method_token: method.as_str().to_owned(),
            url_template: url_template.into(),
            header_refs: Vec::new(),
            body_ref: None,
            assertion_refs: Vec::new(),
            collection_tags: Vec::new(),
        }
    }
}

/// Closed assertion-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionKind {
    StatusMatch,
    HeaderMatch,
    BodyJsonPath,
    BodyTextContains,
    SchemaValidation,
    LatencyCeiling,
    CustomScript,
}

impl AssertionKind {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusMatch => "status_match",
            Self::HeaderMatch => "header_match",
            Self::BodyJsonPath => "body_json_path",
            Self::BodyTextContains => "body_text_contains",
            Self::SchemaValidation => "schema_validation",
            Self::LatencyCeiling => "latency_ceiling",
            Self::CustomScript => "custom_script",
        }
    }
}

/// One assertion descriptor authored against a request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssertionDescriptor {
    /// Opaque assertion id.
    pub assertion_id: String,
    /// Kind of assertion.
    pub kind: AssertionKind,
    /// Stable assertion-kind token.
    pub kind_token: String,
    /// Pointer into the response (e.g. `$.status`, `$.body.id`, header name).
    pub target_pointer: String,
    /// Expected value token. Token-only so secrets never leak into
    /// fixtures or support exports.
    pub expected_token: String,
}

impl AssertionDescriptor {
    /// Build an assertion descriptor with the canonical kind token.
    pub fn new(
        assertion_id: impl Into<String>,
        kind: AssertionKind,
        target_pointer: impl Into<String>,
        expected_token: impl Into<String>,
    ) -> Self {
        Self {
            assertion_id: assertion_id.into(),
            kind,
            kind_token: kind.as_str().to_owned(),
            target_pointer: target_pointer.into(),
            expected_token: expected_token.into(),
        }
    }
}

/// Closed assertion-outcome vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssertionOutcomeClass {
    Passed,
    Failed,
    SkippedNoResponse,
    ErroredDuringEvaluation,
    NotExecuted,
}

impl AssertionOutcomeClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::SkippedNoResponse => "skipped_no_response",
            Self::ErroredDuringEvaluation => "errored_during_evaluation",
            Self::NotExecuted => "not_executed",
        }
    }
}

/// One assertion result row attached to a response artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssertionResultRow {
    /// Ref into the originating [`AssertionDescriptor::assertion_id`].
    pub assertion_ref: String,
    /// Outcome class.
    pub outcome: AssertionOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// Observed value token. Token-only so secrets never leak.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub observed_token: Option<String>,
    /// Expected value token quoted from the descriptor at evaluation time.
    pub expected_token: String,
}

/// Closed response-preview vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsePreviewClass {
    JsonTree,
    StructuredText,
    PlainText,
    HtmlSafePreview,
    BinarySummary,
    RedactedBody,
}

impl ResponsePreviewClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::JsonTree => "json_tree",
            Self::StructuredText => "structured_text",
            Self::PlainText => "plain_text",
            Self::HtmlSafePreview => "html_safe_preview",
            Self::BinarySummary => "binary_summary",
            Self::RedactedBody => "redacted_body",
        }
    }
}

/// Closed latency-class vocabulary. Concrete millisecond values are not
/// recorded so reviewer / support flows do not embed timing fingerprints
/// that could de-anonymise an environment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LatencyBandClass {
    Under250Ms,
    Under1Second,
    Under5Seconds,
    Over5Seconds,
    TimedOut,
}

impl LatencyBandClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Under250Ms => "under_250_ms",
            Self::Under1Second => "under_1_second",
            Self::Under5Seconds => "under_5_seconds",
            Self::Over5Seconds => "over_5_seconds",
            Self::TimedOut => "timed_out",
        }
    }
}

/// Closed response-redaction vocabulary. The default `structured_tokens_only`
/// class is the only value safe to embed in reviewer / support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseRedactionClass {
    StructuredTokensOnly,
    BodyOmittedAtRest,
}

impl ResponseRedactionClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredTokensOnly => "structured_tokens_only",
            Self::BodyOmittedAtRest => "body_omitted_at_rest",
        }
    }
}

/// Response artifact captured for one request send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseArtifact {
    /// HTTP status code (or GraphQL transport status).
    pub status_code: u16,
    /// Opaque header-bundle digest.
    pub header_digest: String,
    /// Opaque body digest.
    pub body_digest: String,
    /// Optional body ref into a workspace-local store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body_ref: Option<String>,
    /// Latency class.
    pub latency_band: LatencyBandClass,
    /// Stable latency-band token.
    pub latency_band_token: String,
    /// Preview class.
    pub preview_class: ResponsePreviewClass,
    /// Stable preview-class token.
    pub preview_class_token: String,
    /// Redaction class.
    pub redaction_class: ResponseRedactionClass,
    /// Stable redaction-class token.
    pub redaction_class_token: String,
    /// Assertion result rows.
    #[serde(default)]
    pub assertion_results: Vec<AssertionResultRow>,
}

impl ResponseArtifact {
    /// True when every assertion result passed.
    pub fn all_assertions_passed(&self) -> bool {
        !self.assertion_results.is_empty()
            && self
                .assertion_results
                .iter()
                .all(|row| row.outcome == AssertionOutcomeClass::Passed)
    }

    /// True when any assertion result failed.
    pub fn any_assertion_failed(&self) -> bool {
        self.assertion_results
            .iter()
            .any(|row| row.outcome == AssertionOutcomeClass::Failed)
    }
}

/// Closed schema-snapshot-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaSnapshotKind {
    Openapi,
    GraphqlSdl,
    JsonSchema,
    NoneDeclared,
}

impl SchemaSnapshotKind {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Openapi => "openapi",
            Self::GraphqlSdl => "graphql_sdl",
            Self::JsonSchema => "json_schema",
            Self::NoneDeclared => "none_declared",
        }
    }
}

/// Closed schema-snapshot-freshness vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaSnapshotFreshness {
    Current,
    StaleUnderDay,
    StaleUnderWeek,
    StaleOverWeek,
    Missing,
}

impl SchemaSnapshotFreshness {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::StaleUnderDay => "stale_under_day",
            Self::StaleUnderWeek => "stale_under_week",
            Self::StaleOverWeek => "stale_over_week",
            Self::Missing => "missing",
        }
    }

    /// True when the schema freshness MUST surface a stale-label cue.
    pub const fn is_stale(self) -> bool {
        !matches!(self, Self::Current)
    }
}

/// Schema snapshot bound to one request workspace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaSnapshot {
    /// Snapshot kind.
    pub kind: SchemaSnapshotKind,
    /// Stable snapshot-kind token.
    pub kind_token: String,
    /// Opaque source ref (workspace file id, registry mirror id, or cache
    /// id). Empty when [`Self::kind`] is `none_declared`.
    pub source_ref: String,
    /// Opaque digest. Empty when the snapshot is missing.
    pub digest: String,
    /// Freshness class.
    pub freshness: SchemaSnapshotFreshness,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Target endpoint id (e.g. `endpoint:payments:v1`).
    pub target_endpoint_id: String,
}

/// Closed side-effect-class vocabulary used by the send inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    NoSideEffect,
    ReadOnlyGet,
    WriteIdempotent,
    WriteNonIdempotent,
    DestructiveDelete,
    FileUpload,
    ExecutesRemoteScript,
    SchemaIntrospection,
}

impl SideEffectClass {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoSideEffect => "no_side_effect",
            Self::ReadOnlyGet => "read_only_get",
            Self::WriteIdempotent => "write_idempotent",
            Self::WriteNonIdempotent => "write_non_idempotent",
            Self::DestructiveDelete => "destructive_delete",
            Self::FileUpload => "file_upload",
            Self::ExecutesRemoteScript => "executes_remote_script",
            Self::SchemaIntrospection => "schema_introspection",
        }
    }

    /// True when the side-effect class MUST require explicit review before
    /// dispatch.
    pub const fn requires_review(self) -> bool {
        matches!(
            self,
            Self::WriteNonIdempotent
                | Self::DestructiveDelete
                | Self::FileUpload
                | Self::ExecutesRemoteScript
        )
    }
}

/// One expected side-effect row carried by the canonical record. The send
/// inspector renders this list verbatim; surfaces do not derive side
/// effects from raw URLs / method strings locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedSideEffectRow {
    /// Side-effect class.
    pub class: SideEffectClass,
    /// Stable side-effect-class token.
    pub class_token: String,
    /// Reviewer-facing one-line summary.
    pub summary: String,
}

impl ExpectedSideEffectRow {
    /// Build a side-effect row with the canonical class token derived from
    /// `class`.
    pub fn new(class: SideEffectClass, summary: impl Into<String>) -> Self {
        Self {
            class,
            class_token: class.as_str().to_owned(),
            summary: summary.into(),
        }
    }
}

/// Closed send-inspector readiness vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SendInspectorReadiness {
    ReadyToSend,
    ReviewRequired,
    BlockedMissingCredential,
    BlockedSchemaStale,
    BlockedPolicy,
}

impl SendInspectorReadiness {
    /// Stable string token recorded in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToSend => "ready_to_send",
            Self::ReviewRequired => "review_required",
            Self::BlockedMissingCredential => "blocked_missing_credential",
            Self::BlockedSchemaStale => "blocked_schema_stale",
            Self::BlockedPolicy => "blocked_policy",
        }
    }

    /// True when the readiness state requires reviewer attention before
    /// dispatch is permitted.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::ReadyToSend)
    }

    /// True when the readiness state outright blocks dispatch.
    pub const fn blocks_dispatch(self) -> bool {
        matches!(
            self,
            Self::BlockedMissingCredential
                | Self::BlockedSchemaStale
                | Self::BlockedPolicy
        )
    }
}

/// One banner surfaced by the send inspector. Banners quote the structured
/// reason; chrome and CLI / headless surfaces render the same banner set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendInspectorBanner {
    /// Stable banner-kind token.
    pub banner_kind: String,
    /// Reviewer-facing one-line summary.
    pub summary: String,
}

/// Send-inspector projection produced from a canonical
/// [`RequestWorkspaceAlphaRecord`].
///
/// The report answers "before I send this request, what target, credential
/// class, execution context, and side effects am I about to commit?" for
/// both the chrome inspector and the headless CLI. The two surfaces consume
/// this record verbatim — they do not re-derive readiness, banners, or
/// side-effect rows locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SendInspectorReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Ref into the source [`RequestWorkspaceAlphaRecord::request_workspace_ref`].
    pub request_workspace_ref: String,
    /// Source request-document id.
    pub request_id: String,
    /// Canonical execution-context ref.
    pub execution_context_ref: String,
    /// Resolved target class.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// True when the local-vs-managed boundary cue MUST be rendered.
    pub boundary_cue_visible: bool,
    /// Method class.
    pub method_token: String,
    /// Effective URL template (the inspector renders the template verbatim;
    /// it does not embed resolved secret material).
    pub effective_url_template: String,
    /// Effective environment fingerprint.
    pub environment_fingerprint: String,
    /// Credential class.
    pub credential_class: CredentialClass,
    /// Stable credential-class token.
    pub credential_class_token: String,
    /// Auth strategy.
    pub auth_strategy: AuthStrategyKind,
    /// Stable auth-strategy token.
    pub auth_strategy_token: String,
    /// Expected side-effect rows.
    pub expected_side_effects: Vec<ExpectedSideEffectRow>,
    /// Schema snapshot freshness token.
    pub schema_freshness_token: String,
    /// Readiness band.
    pub readiness: SendInspectorReadiness,
    /// Stable readiness token.
    pub readiness_token: String,
    /// True when the inspector MUST render a review-required banner.
    pub requires_review_before_dispatch: bool,
    /// True when the inspector MUST block the send action.
    pub blocks_dispatch: bool,
    /// Banner rows.
    pub banners: Vec<SendInspectorBanner>,
    /// Compact summary line suitable for the headless CLI and support
    /// exports.
    pub summary_line: String,
}

/// Canonical alpha record for one request-workspace row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestWorkspaceAlphaRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable lane id.
    pub lane_id: String,
    /// Opaque workspace-row id.
    pub request_workspace_ref: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Timestamp supplied by the caller.
    pub captured_at: String,
    /// Canonical execution-context ref (resolved through the shared
    /// [`ExecutionContext`] model).
    pub execution_context_ref: String,
    /// Resolved target class on the canonical context.
    pub target_class: TargetClass,
    /// Stable target-class token.
    pub target_class_token: String,
    /// True when the local-vs-managed boundary cue MUST be rendered on this
    /// row.
    pub boundary_cue_visible: bool,
    /// Authored request document.
    pub request: RequestDocument,
    /// Layered environment set resolved before send.
    pub environment: EnvironmentSet,
    /// Auth profile.
    pub auth: AuthProfile,
    /// Assertion suite authored against the request.
    #[serde(default)]
    pub assertions: Vec<AssertionDescriptor>,
    /// Schema snapshot bound to the target endpoint.
    pub schema_snapshot: SchemaSnapshot,
    /// Expected side-effect rows. The inspector renders these verbatim.
    #[serde(default)]
    pub expected_side_effects: Vec<ExpectedSideEffectRow>,
    /// Captured response artifact when one exists; `None` for rows that
    /// have not been sent yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_artifact: Option<ResponseArtifact>,
    /// Review-safe summary.
    pub summary: String,
}

impl RequestWorkspaceAlphaRecord {
    /// Project the canonical send-inspector report.
    pub fn send_inspector_report(&self) -> SendInspectorReport {
        let readiness = self.derive_readiness();
        let banners = self.derive_banners(readiness);
        let summary_line = format!(
            "request={} method={} target={} credential={} readiness={} side_effects={}",
            self.request.request_id,
            self.request.method_token,
            self.target_class_token,
            self.auth.credential_class_token,
            readiness.as_str(),
            if self.expected_side_effects.is_empty() {
                "none".to_owned()
            } else {
                self.expected_side_effects
                    .iter()
                    .map(|row| row.class_token.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            }
        );
        SendInspectorReport {
            record_kind: REQUEST_WORKSPACE_SEND_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
            request_workspace_ref: self.request_workspace_ref.clone(),
            request_id: self.request.request_id.clone(),
            execution_context_ref: self.execution_context_ref.clone(),
            target_class: self.target_class,
            target_class_token: self.target_class_token.clone(),
            boundary_cue_visible: self.boundary_cue_visible,
            method_token: self.request.method_token.clone(),
            effective_url_template: self.request.url_template.clone(),
            environment_fingerprint: self.environment.effective_fingerprint.clone(),
            credential_class: self.auth.credential_class,
            credential_class_token: self.auth.credential_class_token.clone(),
            auth_strategy: self.auth.strategy_kind,
            auth_strategy_token: self.auth.strategy_kind_token.clone(),
            expected_side_effects: self.expected_side_effects.clone(),
            schema_freshness_token: self.schema_snapshot.freshness_token.clone(),
            readiness,
            readiness_token: readiness.as_str().to_owned(),
            requires_review_before_dispatch: readiness.requires_review(),
            blocks_dispatch: readiness.blocks_dispatch(),
            banners,
            summary_line,
        }
    }

    /// Returns validation issues that would make this record overclaim
    /// request-workspace truth.
    pub fn validation_issues(&self) -> Vec<RequestWorkspaceAlphaViolation> {
        let mut issues = Vec::new();
        if self.record_kind != REQUEST_WORKSPACE_ALPHA_RECORD_KIND {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "unexpected_record_kind",
                "record_kind",
                "request-workspace record kind must stay canonical",
            ));
        }
        if self.schema_version != REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "unexpected_schema_version",
                "schema_version",
                "request-workspace schema version must match this crate",
            ));
        }
        if self.lane_id != REQUEST_WORKSPACE_ALPHA_LANE_ID {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "unbounded_lane_id",
                "lane_id",
                "request-workspace alpha lane must stay on the bounded lane id",
            ));
        }
        if self.auth.credential_class == CredentialClass::RawInlineDisallowed {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "raw_inline_credentials",
                "auth.credential_class",
                "request workspaces must not persist raw inline credentials",
            ));
        }
        for layer in &self.environment.layered_variables {
            if layer.is_secret_handle && layer.value_token.is_some() {
                issues.push(RequestWorkspaceAlphaViolation::new(
                    "secret_handle_carries_value",
                    "environment.layered_variables",
                    "secret-handle layers must not carry resolved value tokens",
                ));
                break;
            }
            if layer.layer_kind == EnvironmentLayerKind::SecretHandle && !layer.is_secret_handle {
                issues.push(RequestWorkspaceAlphaViolation::new(
                    "secret_handle_flag_mismatch",
                    "environment.layered_variables",
                    "secret_handle layer kind must set is_secret_handle = true",
                ));
                break;
            }
        }
        if self.target_class_token != self.target_class.as_str() {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "target_class_token_mismatch",
                "target_class_token",
                "target_class_token must match the canonical target_class",
            ));
        }
        if self.boundary_cue_visible != self.target_class.is_remote_or_managed() {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "boundary_cue_mismatch",
                "boundary_cue_visible",
                "boundary_cue_visible must follow the canonical target-class rule",
            ));
        }
        if self
            .expected_side_effects
            .iter()
            .any(|row| row.class_token != row.class.as_str())
        {
            issues.push(RequestWorkspaceAlphaViolation::new(
                "side_effect_token_mismatch",
                "expected_side_effects",
                "expected_side_effects rows must carry canonical class tokens",
            ));
        }
        if let Some(response) = &self.response_artifact {
            for row in &response.assertion_results {
                let referenced = self
                    .assertions
                    .iter()
                    .any(|descriptor| descriptor.assertion_id == row.assertion_ref);
                if !referenced {
                    issues.push(RequestWorkspaceAlphaViolation::new(
                        "assertion_result_unbound",
                        "response_artifact.assertion_results",
                        "assertion result must reference an authored assertion",
                    ));
                    break;
                }
            }
        }
        issues
    }

    fn derive_readiness(&self) -> SendInspectorReadiness {
        if !self.auth.credential_class.is_safe_to_dispatch() {
            return SendInspectorReadiness::BlockedMissingCredential;
        }
        if self.schema_snapshot.kind != SchemaSnapshotKind::NoneDeclared
            && matches!(
                self.schema_snapshot.freshness,
                SchemaSnapshotFreshness::StaleOverWeek | SchemaSnapshotFreshness::Missing
            )
            && self.expected_side_effects.iter().any(|row| {
                matches!(
                    row.class,
                    SideEffectClass::WriteNonIdempotent
                        | SideEffectClass::DestructiveDelete
                        | SideEffectClass::FileUpload
                        | SideEffectClass::ExecutesRemoteScript
                )
            })
        {
            return SendInspectorReadiness::BlockedSchemaStale;
        }
        if self
            .expected_side_effects
            .iter()
            .any(|row| row.class.requires_review())
        {
            return SendInspectorReadiness::ReviewRequired;
        }
        if self.schema_snapshot.freshness.is_stale() {
            return SendInspectorReadiness::ReviewRequired;
        }
        if self.auth.credential_class == CredentialClass::NoCredentials
            && !self.request.method.is_read_only()
        {
            return SendInspectorReadiness::ReviewRequired;
        }
        SendInspectorReadiness::ReadyToSend
    }

    fn derive_banners(&self, readiness: SendInspectorReadiness) -> Vec<SendInspectorBanner> {
        let mut banners = Vec::new();
        if readiness == SendInspectorReadiness::BlockedMissingCredential {
            banners.push(SendInspectorBanner {
                banner_kind: "credential_blocked".to_owned(),
                summary: "Raw inline credentials are not allowed; bind a broker handle"
                    .to_owned(),
            });
        }
        if readiness == SendInspectorReadiness::BlockedSchemaStale {
            banners.push(SendInspectorBanner {
                banner_kind: "schema_stale_blocked".to_owned(),
                summary: format!(
                    "Mutating request blocked: schema snapshot is {}",
                    self.schema_snapshot.freshness_token
                ),
            });
        }
        if self.schema_snapshot.freshness.is_stale()
            && readiness != SendInspectorReadiness::BlockedSchemaStale
        {
            banners.push(SendInspectorBanner {
                banner_kind: "schema_stale".to_owned(),
                summary: format!(
                    "Schema snapshot is {} for endpoint {}",
                    self.schema_snapshot.freshness_token, self.schema_snapshot.target_endpoint_id
                ),
            });
        }
        for row in &self.expected_side_effects {
            if row.class.requires_review() {
                banners.push(SendInspectorBanner {
                    banner_kind: format!("side_effect:{}", row.class_token),
                    summary: row.summary.clone(),
                });
            }
        }
        if self.boundary_cue_visible {
            banners.push(SendInspectorBanner {
                banner_kind: "boundary_cue".to_owned(),
                summary: format!(
                    "Request runs against {} (boundary cue visible)",
                    self.target_class_token
                ),
            });
        }
        if self.auth.credential_class == CredentialClass::NoCredentials
            && !self.request.method.is_read_only()
        {
            banners.push(SendInspectorBanner {
                banner_kind: "no_credentials_with_mutation".to_owned(),
                summary: "Mutating request authored with no credentials".to_owned(),
            });
        }
        banners
    }
}

/// Validation issue raised when a request-workspace row overclaims truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestWorkspaceAlphaViolation {
    /// Stable violation code.
    pub code: String,
    /// Dotted field path responsible for the issue.
    pub field_path: String,
    /// Review-safe issue summary.
    pub summary: String,
}

impl RequestWorkspaceAlphaViolation {
    /// Builds a validation issue.
    pub fn new(
        code: impl Into<String>,
        field_path: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            code: code.into(),
            field_path: field_path.into(),
            summary: summary.into(),
        }
    }
}

/// Support-export packet bundling one or more request-workspace rows and
/// their canonical send-inspector reports.
///
/// The wrapper is the only object reviewer / support flows need to reopen
/// or compare a request-workspace run truthfully: every record carries the
/// authored request, the resolved environment, the auth posture, the
/// assertion suite, the optional response, the schema snapshot, and the
/// canonical send-inspector report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestWorkspaceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable lane id.
    pub lane_id: String,
    /// Manifest id supplied by the caller.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Bundled records in canonical order.
    pub records: Vec<RequestWorkspaceAlphaRecord>,
    /// Bundled send-inspector reports, one per record, in the same order.
    pub send_inspector_reports: Vec<SendInspectorReport>,
    /// True when any bundled report requires review before dispatch.
    pub any_requires_review: bool,
    /// True when any bundled report blocks dispatch outright.
    pub any_blocks_dispatch: bool,
}

impl RequestWorkspaceSupportExport {
    /// Build a support-export packet from a record collection. The packet
    /// projects one send-inspector report per record in canonical order so
    /// reviewer / support consumers do not have to re-derive the report
    /// locally.
    pub fn from_records(
        manifest_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: Vec<RequestWorkspaceAlphaRecord>,
    ) -> Self {
        let send_inspector_reports: Vec<SendInspectorReport> = records
            .iter()
            .map(RequestWorkspaceAlphaRecord::send_inspector_report)
            .collect();
        let any_requires_review = send_inspector_reports
            .iter()
            .any(|report| report.requires_review_before_dispatch);
        let any_blocks_dispatch = send_inspector_reports
            .iter()
            .any(|report| report.blocks_dispatch);
        Self {
            record_kind: REQUEST_WORKSPACE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
            lane_id: REQUEST_WORKSPACE_ALPHA_LANE_ID.to_owned(),
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            records,
            send_inspector_reports,
            any_requires_review,
            any_blocks_dispatch,
        }
    }

    /// Render the support export as a stable plaintext block suitable for
    /// the CLI / headless surface, the support-export wrapper, and copy-to-
    /// clipboard. The output is identical across UI inspector chrome and
    /// the headless binary: both consume the same canonical send-inspector
    /// summary lines.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("request-workspace support export: {}\n", self.manifest_id);
        out.push_str(&format!("  generated_at: {}\n", self.generated_at));
        out.push_str(&format!("  any_requires_review: {}\n", self.any_requires_review));
        out.push_str(&format!("  any_blocks_dispatch: {}\n", self.any_blocks_dispatch));
        for report in &self.send_inspector_reports {
            out.push_str(&format!("  - workspace_ref={} ", report.request_workspace_ref));
            out.push_str(&report.summary_line);
            out.push('\n');
            for banner in &report.banners {
                out.push_str(&format!("      [{}] {}\n", banner.banner_kind, banner.summary));
            }
        }
        out
    }
}

/// Identifier for one seeded request-workspace scenario the headless CLI,
/// the chrome panel projection, and the integration test all replay
/// verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestWorkspaceSeededScenario {
    /// Read-only `GET` against a local target, broker-handle credential,
    /// fresh schema, no review required.
    LocalReadOnlyGet,
    /// Mutating `POST` against a remote target with a stale-under-week
    /// schema; the send inspector flags the row as review-required.
    RemoteMutatingPostStaleSchema,
    /// Destructive `DELETE` against a managed-workspace target with a
    /// missing schema snapshot; the send inspector outright blocks the
    /// send.
    ManagedDeleteMissingSchema,
    /// GraphQL operation against a remote target with no auth and a
    /// fresh schema; the send inspector flags it as review-required.
    RemoteGraphqlNoAuth,
}

impl RequestWorkspaceSeededScenario {
    /// Every seeded scenario in canonical order.
    pub const ALL: [Self; 4] = [
        Self::LocalReadOnlyGet,
        Self::RemoteMutatingPostStaleSchema,
        Self::ManagedDeleteMissingSchema,
        Self::RemoteGraphqlNoAuth,
    ];

    /// Stable string token recorded in CLI output and reviewer fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReadOnlyGet => "local_read_only_get",
            Self::RemoteMutatingPostStaleSchema => "remote_mutating_post_stale_schema",
            Self::ManagedDeleteMissingSchema => "managed_delete_missing_schema",
            Self::RemoteGraphqlNoAuth => "remote_graphql_no_auth",
        }
    }
}

/// Build the seeded canonical record for one scenario.
pub fn seeded_request_workspace_record(
    scenario: RequestWorkspaceSeededScenario,
) -> RequestWorkspaceAlphaRecord {
    match scenario {
        RequestWorkspaceSeededScenario::LocalReadOnlyGet => local_read_only_get_record(),
        RequestWorkspaceSeededScenario::RemoteMutatingPostStaleSchema => {
            remote_mutating_post_stale_schema_record()
        }
        RequestWorkspaceSeededScenario::ManagedDeleteMissingSchema => {
            managed_delete_missing_schema_record()
        }
        RequestWorkspaceSeededScenario::RemoteGraphqlNoAuth => remote_graphql_no_auth_record(),
    }
}

/// Project a seeded send-inspector report.
pub fn seeded_send_inspector_report(
    scenario: RequestWorkspaceSeededScenario,
) -> SendInspectorReport {
    seeded_request_workspace_record(scenario).send_inspector_report()
}

/// Build a seeded support-export packet bundling every canonical scenario.
pub fn seeded_request_workspace_support_export(
    manifest_id: impl Into<String>,
    generated_at: impl Into<String>,
) -> RequestWorkspaceSupportExport {
    let records: Vec<RequestWorkspaceAlphaRecord> = RequestWorkspaceSeededScenario::ALL
        .into_iter()
        .map(seeded_request_workspace_record)
        .collect();
    RequestWorkspaceSupportExport::from_records(manifest_id, generated_at, records)
}

fn local_read_only_get_record() -> RequestWorkspaceAlphaRecord {
    let mut request = RequestDocument::new(
        "req:local:read_only_get",
        RequestMethodClass::Get,
        "{{api_base}}/v1/payments/{{payment_id}}",
    );
    request.header_refs.push("headers:default_json".to_owned());
    request.assertion_refs = vec![
        "assert:status_200".to_owned(),
        "assert:latency_under_250ms".to_owned(),
    ];
    request.collection_tags = vec!["payments".to_owned(), "read_only".to_owned()];

    let environment = EnvironmentSet {
        environment_id: "env:dev".to_owned(),
        scope_label: "workspace.profile.dev".to_owned(),
        base_url_template: "https://api.dev.example.com".to_owned(),
        layered_variables: vec![
            EnvironmentVariableLayer::plain(
                "api_base",
                EnvironmentLayerKind::ProfileDefault,
                "profile:dev",
                "https://api.dev.example.com",
            ),
            EnvironmentVariableLayer::plain(
                "payment_id",
                EnvironmentLayerKind::AdHocOverride,
                "session:override:001",
                "pay_demo_001",
            ),
            EnvironmentVariableLayer::secret_handle(
                "api_token",
                "secret://payments/api-token",
            ),
        ],
        effective_fingerprint: "env:fp:dev:read_only_get:01".to_owned(),
    };

    let auth = AuthProfile::new(
        AuthStrategyKind::BearerBroker,
        CredentialClass::BrokerHandle,
        ["secret://payments/api-token"],
        Some("refresh:bearer:payments:01".to_owned()),
    );

    let assertions = vec![
        AssertionDescriptor::new(
            "assert:status_200",
            AssertionKind::StatusMatch,
            "$.status",
            "200",
        ),
        AssertionDescriptor::new(
            "assert:latency_under_250ms",
            AssertionKind::LatencyCeiling,
            "$.latency_band",
            "under_250_ms",
        ),
    ];

    let schema_snapshot = SchemaSnapshot {
        kind: SchemaSnapshotKind::Openapi,
        kind_token: SchemaSnapshotKind::Openapi.as_str().to_owned(),
        source_ref: "schema:payments:openapi:v1".to_owned(),
        digest: "sha256:openapi:payments:fresh".to_owned(),
        freshness: SchemaSnapshotFreshness::Current,
        freshness_token: SchemaSnapshotFreshness::Current.as_str().to_owned(),
        target_endpoint_id: "endpoint:payments:v1".to_owned(),
    };

    let expected_side_effects = vec![ExpectedSideEffectRow::new(
        SideEffectClass::ReadOnlyGet,
        "Read-only payment lookup",
    )];

    let response_artifact = Some(ResponseArtifact {
        status_code: 200,
        header_digest: "sha256:resp:headers:read_only_get".to_owned(),
        body_digest: "sha256:resp:body:read_only_get".to_owned(),
        body_ref: Some("response:local:read_only_get:body".to_owned()),
        latency_band: LatencyBandClass::Under250Ms,
        latency_band_token: LatencyBandClass::Under250Ms.as_str().to_owned(),
        preview_class: ResponsePreviewClass::JsonTree,
        preview_class_token: ResponsePreviewClass::JsonTree.as_str().to_owned(),
        redaction_class: ResponseRedactionClass::StructuredTokensOnly,
        redaction_class_token: ResponseRedactionClass::StructuredTokensOnly.as_str().to_owned(),
        assertion_results: vec![
            AssertionResultRow {
                assertion_ref: "assert:status_200".to_owned(),
                outcome: AssertionOutcomeClass::Passed,
                outcome_token: AssertionOutcomeClass::Passed.as_str().to_owned(),
                observed_token: Some("200".to_owned()),
                expected_token: "200".to_owned(),
            },
            AssertionResultRow {
                assertion_ref: "assert:latency_under_250ms".to_owned(),
                outcome: AssertionOutcomeClass::Passed,
                outcome_token: AssertionOutcomeClass::Passed.as_str().to_owned(),
                observed_token: Some("under_250_ms".to_owned()),
                expected_token: "under_250_ms".to_owned(),
            },
        ],
    });

    let target_class = TargetClass::LocalHost;
    RequestWorkspaceAlphaRecord {
        record_kind: REQUEST_WORKSPACE_ALPHA_RECORD_KIND.to_owned(),
        schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
        lane_id: REQUEST_WORKSPACE_ALPHA_LANE_ID.to_owned(),
        request_workspace_ref: "rwsr:local:read_only_get".to_owned(),
        workspace_id: "ws:request-workspace-alpha".to_owned(),
        captured_at: "2026-05-15T00:00:00Z".to_owned(),
        execution_context_ref: "exec:ws-request-workspace-alpha:task:0".to_owned(),
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        boundary_cue_visible: target_class.is_remote_or_managed(),
        request,
        environment,
        auth,
        assertions,
        schema_snapshot,
        expected_side_effects,
        response_artifact,
        summary: "Local read-only GET against payments lookup, broker-handle credential."
            .to_owned(),
    }
}

fn remote_mutating_post_stale_schema_record() -> RequestWorkspaceAlphaRecord {
    let mut request = RequestDocument::new(
        "req:remote:mutating_post",
        RequestMethodClass::Post,
        "{{api_base}}/v1/payments/refund",
    );
    request.header_refs.push("headers:default_json".to_owned());
    request.body_ref = Some("body:remote:refund".to_owned());
    request.assertion_refs = vec!["assert:status_202".to_owned()];
    request.collection_tags = vec!["payments".to_owned(), "refund".to_owned()];

    let environment = EnvironmentSet {
        environment_id: "env:staging".to_owned(),
        scope_label: "workspace.profile.staging".to_owned(),
        base_url_template: "https://api.staging.example.com".to_owned(),
        layered_variables: vec![
            EnvironmentVariableLayer::plain(
                "api_base",
                EnvironmentLayerKind::ProfileDefault,
                "profile:staging",
                "https://api.staging.example.com",
            ),
            EnvironmentVariableLayer::secret_handle(
                "api_token",
                "secret://payments/api-token",
            ),
        ],
        effective_fingerprint: "env:fp:staging:mutating_post:01".to_owned(),
    };

    let auth = AuthProfile::new(
        AuthStrategyKind::Oauth2Broker,
        CredentialClass::BrokerHandle,
        ["secret://payments/api-token"],
        Some("refresh:oauth:payments:01".to_owned()),
    );

    let assertions = vec![AssertionDescriptor::new(
        "assert:status_202",
        AssertionKind::StatusMatch,
        "$.status",
        "202",
    )];

    let schema_snapshot = SchemaSnapshot {
        kind: SchemaSnapshotKind::Openapi,
        kind_token: SchemaSnapshotKind::Openapi.as_str().to_owned(),
        source_ref: "schema:payments:openapi:v1".to_owned(),
        digest: "sha256:openapi:payments:stale".to_owned(),
        freshness: SchemaSnapshotFreshness::StaleUnderWeek,
        freshness_token: SchemaSnapshotFreshness::StaleUnderWeek.as_str().to_owned(),
        target_endpoint_id: "endpoint:payments:v1".to_owned(),
    };

    let expected_side_effects = vec![ExpectedSideEffectRow::new(
        SideEffectClass::WriteNonIdempotent,
        "Refund mutates payment state and is not idempotent",
    )];

    let target_class = TargetClass::SshRemote;
    RequestWorkspaceAlphaRecord {
        record_kind: REQUEST_WORKSPACE_ALPHA_RECORD_KIND.to_owned(),
        schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
        lane_id: REQUEST_WORKSPACE_ALPHA_LANE_ID.to_owned(),
        request_workspace_ref: "rwsr:remote:mutating_post".to_owned(),
        workspace_id: "ws:request-workspace-alpha".to_owned(),
        captured_at: "2026-05-15T00:00:00Z".to_owned(),
        execution_context_ref: "exec:ws-request-workspace-alpha:task:1".to_owned(),
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        boundary_cue_visible: target_class.is_remote_or_managed(),
        request,
        environment,
        auth,
        assertions,
        schema_snapshot,
        expected_side_effects,
        response_artifact: None,
        summary: "Mutating POST against staging refund endpoint, stale schema snapshot."
            .to_owned(),
    }
}

fn managed_delete_missing_schema_record() -> RequestWorkspaceAlphaRecord {
    let mut request = RequestDocument::new(
        "req:managed:destructive_delete",
        RequestMethodClass::Delete,
        "{{api_base}}/v1/payments/refund/{{refund_id}}",
    );
    request.header_refs.push("headers:default_json".to_owned());
    request.assertion_refs = vec!["assert:status_204".to_owned()];
    request.collection_tags = vec!["payments".to_owned(), "destructive".to_owned()];

    let environment = EnvironmentSet {
        environment_id: "env:managed".to_owned(),
        scope_label: "workspace.profile.managed".to_owned(),
        base_url_template: "https://api.managed.example.com".to_owned(),
        layered_variables: vec![
            EnvironmentVariableLayer::plain(
                "api_base",
                EnvironmentLayerKind::ProfileDefault,
                "profile:managed",
                "https://api.managed.example.com",
            ),
            EnvironmentVariableLayer::plain(
                "refund_id",
                EnvironmentLayerKind::AdHocOverride,
                "session:override:002",
                "rfd_demo_001",
            ),
            EnvironmentVariableLayer::secret_handle(
                "api_token",
                "secret://payments/managed-api-token",
            ),
        ],
        effective_fingerprint: "env:fp:managed:destructive_delete:01".to_owned(),
    };

    let auth = AuthProfile::new(
        AuthStrategyKind::BearerBroker,
        CredentialClass::BrokerHandle,
        ["secret://payments/managed-api-token"],
        None,
    );

    let assertions = vec![AssertionDescriptor::new(
        "assert:status_204",
        AssertionKind::StatusMatch,
        "$.status",
        "204",
    )];

    let schema_snapshot = SchemaSnapshot {
        kind: SchemaSnapshotKind::Openapi,
        kind_token: SchemaSnapshotKind::Openapi.as_str().to_owned(),
        source_ref: "".to_owned(),
        digest: "".to_owned(),
        freshness: SchemaSnapshotFreshness::Missing,
        freshness_token: SchemaSnapshotFreshness::Missing.as_str().to_owned(),
        target_endpoint_id: "endpoint:payments:v1".to_owned(),
    };

    let expected_side_effects = vec![ExpectedSideEffectRow::new(
        SideEffectClass::DestructiveDelete,
        "Destructive refund deletion against managed-workspace endpoint",
    )];

    let target_class = TargetClass::ManagedWorkspace;
    RequestWorkspaceAlphaRecord {
        record_kind: REQUEST_WORKSPACE_ALPHA_RECORD_KIND.to_owned(),
        schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
        lane_id: REQUEST_WORKSPACE_ALPHA_LANE_ID.to_owned(),
        request_workspace_ref: "rwsr:managed:destructive_delete".to_owned(),
        workspace_id: "ws:request-workspace-alpha".to_owned(),
        captured_at: "2026-05-15T00:00:00Z".to_owned(),
        execution_context_ref: "exec:ws-request-workspace-alpha:task:2".to_owned(),
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        boundary_cue_visible: target_class.is_remote_or_managed(),
        request,
        environment,
        auth,
        assertions,
        schema_snapshot,
        expected_side_effects,
        response_artifact: None,
        summary: "Destructive DELETE against managed-workspace endpoint with missing schema."
            .to_owned(),
    }
}

fn remote_graphql_no_auth_record() -> RequestWorkspaceAlphaRecord {
    let mut request = RequestDocument::new(
        "req:remote:graphql_no_auth",
        RequestMethodClass::GraphqlOperation,
        "{{graphql_endpoint}}",
    );
    request.body_ref = Some("body:remote:graphql:query".to_owned());
    request.assertion_refs = vec!["assert:status_200".to_owned()];
    request.collection_tags = vec!["graphql".to_owned(), "public".to_owned()];

    let environment = EnvironmentSet {
        environment_id: "env:public".to_owned(),
        scope_label: "workspace.profile.public".to_owned(),
        base_url_template: "https://graphql.public.example.com".to_owned(),
        layered_variables: vec![EnvironmentVariableLayer::plain(
            "graphql_endpoint",
            EnvironmentLayerKind::ProfileDefault,
            "profile:public",
            "https://graphql.public.example.com/v1",
        )],
        effective_fingerprint: "env:fp:public:graphql:01".to_owned(),
    };

    let auth = AuthProfile::none();

    let assertions = vec![AssertionDescriptor::new(
        "assert:status_200",
        AssertionKind::StatusMatch,
        "$.status",
        "200",
    )];

    let schema_snapshot = SchemaSnapshot {
        kind: SchemaSnapshotKind::GraphqlSdl,
        kind_token: SchemaSnapshotKind::GraphqlSdl.as_str().to_owned(),
        source_ref: "schema:public:graphql:sdl".to_owned(),
        digest: "sha256:graphql:public:fresh".to_owned(),
        freshness: SchemaSnapshotFreshness::Current,
        freshness_token: SchemaSnapshotFreshness::Current.as_str().to_owned(),
        target_endpoint_id: "endpoint:public:graphql".to_owned(),
    };

    let expected_side_effects = vec![ExpectedSideEffectRow::new(
        SideEffectClass::SchemaIntrospection,
        "GraphQL introspection / read query against the public schema",
    )];

    let target_class = TargetClass::SshRemote;
    RequestWorkspaceAlphaRecord {
        record_kind: REQUEST_WORKSPACE_ALPHA_RECORD_KIND.to_owned(),
        schema_version: REQUEST_WORKSPACE_ALPHA_SCHEMA_VERSION,
        lane_id: REQUEST_WORKSPACE_ALPHA_LANE_ID.to_owned(),
        request_workspace_ref: "rwsr:remote:graphql_no_auth".to_owned(),
        workspace_id: "ws:request-workspace-alpha".to_owned(),
        captured_at: "2026-05-15T00:00:00Z".to_owned(),
        execution_context_ref: "exec:ws-request-workspace-alpha:task:3".to_owned(),
        target_class,
        target_class_token: target_class.as_str().to_owned(),
        boundary_cue_visible: target_class.is_remote_or_managed(),
        request,
        environment,
        auth,
        assertions,
        schema_snapshot,
        expected_side_effects,
        response_artifact: None,
        summary: "GraphQL operation against public endpoint with no auth.".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_only_local_get_is_ready_to_send() {
        let record = seeded_request_workspace_record(
            RequestWorkspaceSeededScenario::LocalReadOnlyGet,
        );
        assert!(record.validation_issues().is_empty());
        let report = record.send_inspector_report();
        assert_eq!(report.readiness, SendInspectorReadiness::ReadyToSend);
        assert!(!report.requires_review_before_dispatch);
        assert!(!report.blocks_dispatch);
        assert_eq!(report.target_class, TargetClass::LocalHost);
        assert!(!report.boundary_cue_visible);
    }

    #[test]
    fn remote_mutating_post_is_review_required() {
        let record = seeded_request_workspace_record(
            RequestWorkspaceSeededScenario::RemoteMutatingPostStaleSchema,
        );
        let report = record.send_inspector_report();
        assert_eq!(report.readiness, SendInspectorReadiness::ReviewRequired);
        assert!(report.requires_review_before_dispatch);
        assert!(!report.blocks_dispatch);
        assert!(report.boundary_cue_visible);
        assert!(report
            .banners
            .iter()
            .any(|banner| banner.banner_kind.starts_with("side_effect:")));
    }

    #[test]
    fn managed_destructive_delete_is_blocked_when_schema_missing() {
        let record = seeded_request_workspace_record(
            RequestWorkspaceSeededScenario::ManagedDeleteMissingSchema,
        );
        let report = record.send_inspector_report();
        assert_eq!(report.readiness, SendInspectorReadiness::BlockedSchemaStale);
        assert!(report.requires_review_before_dispatch);
        assert!(report.blocks_dispatch);
    }

    #[test]
    fn remote_graphql_no_auth_is_review_required_due_to_no_credentials() {
        let record = seeded_request_workspace_record(
            RequestWorkspaceSeededScenario::RemoteGraphqlNoAuth,
        );
        let report = record.send_inspector_report();
        assert_eq!(report.readiness, SendInspectorReadiness::ReviewRequired);
        assert!(report
            .banners
            .iter()
            .any(|banner| banner.banner_kind == "no_credentials_with_mutation"));
    }

    #[test]
    fn raw_inline_credential_flags_violation_and_blocks_dispatch() {
        let mut record = seeded_request_workspace_record(
            RequestWorkspaceSeededScenario::LocalReadOnlyGet,
        );
        record.auth = AuthProfile::new(
            AuthStrategyKind::ApiKeyBroker,
            CredentialClass::RawInlineDisallowed,
            Vec::<String>::new(),
            None,
        );
        let violations = record.validation_issues();
        assert!(violations.iter().any(|v| v.code == "raw_inline_credentials"));
        let report = record.send_inspector_report();
        assert_eq!(
            report.readiness,
            SendInspectorReadiness::BlockedMissingCredential
        );
        assert!(report.blocks_dispatch);
    }

    #[test]
    fn support_export_round_trips_through_serde() {
        let export = seeded_request_workspace_support_export(
            "request-workspace-alpha:test",
            "2026-05-15T00:00:00Z",
        );
        let json = serde_json::to_string(&export).expect("serialize");
        let round: RequestWorkspaceSupportExport =
            serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, export);
        assert_eq!(round.records.len(), RequestWorkspaceSeededScenario::ALL.len());
        assert_eq!(
            round.send_inspector_reports.len(),
            RequestWorkspaceSeededScenario::ALL.len()
        );
        assert!(round.any_requires_review);
        assert!(round.any_blocks_dispatch);
    }

    #[test]
    fn support_export_render_plaintext_is_deterministic() {
        let export = seeded_request_workspace_support_export(
            "request-workspace-alpha:plaintext",
            "2026-05-15T00:00:00Z",
        );
        let first = export.render_plaintext();
        let second = export.render_plaintext();
        assert_eq!(first, second);
        assert!(first.contains("request-workspace support export:"));
        for report in &export.send_inspector_reports {
            assert!(first.contains(&report.summary_line));
        }
    }

    #[test]
    fn seeded_records_pass_validation() {
        for scenario in RequestWorkspaceSeededScenario::ALL {
            let record = seeded_request_workspace_record(scenario);
            assert!(
                record.validation_issues().is_empty(),
                "{}: validation must pass",
                scenario.as_str()
            );
        }
    }

    #[test]
    fn boundary_cue_visibility_follows_target_class_rule() {
        for scenario in RequestWorkspaceSeededScenario::ALL {
            let record = seeded_request_workspace_record(scenario);
            assert_eq!(
                record.boundary_cue_visible,
                record.target_class.is_remote_or_managed(),
                "{}: boundary cue rule",
                scenario.as_str()
            );
        }
    }
}
