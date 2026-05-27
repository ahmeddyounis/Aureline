//! Request-workspace and API-request execution-context reuse truth packet
//! for the M4 stable lane.
//!
//! This module pins how the request-workspace, API-request,
//! response-trust, and data-action lanes reuse one canonical
//! `execution_context_id` and one set of route, auth-source, approval,
//! connection-state, and streaming-response vocabularies across the
//! request editor surface, the response timeline surface, the
//! mutation-review sheet, the replay/history surface, the CLI/headless
//! inspect surface, the support export bundle, the Help/About proof
//! card, and the conformance dashboard. Surfaces MUST NOT fork local
//! runtime semantics, paraphrase auth-source modes, collapse streaming
//! states into "done/error", or silently re-enter a deferred queue with
//! a non-idempotent or destructive request without an explicit review.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `execution_context_reuse_quality` row cannot prove:
//!
//! - the four wedges (`route_target_truth`, `auth_source_truth`,
//!   `approval_review_truth`, `execution_context_reuse_truth`) each
//!   have a `wedge_admission` row so the lane explains route, auth
//!   source, approval review, and execution-context reuse posture
//!   without support-only knowledge,
//! - the six auth-source modes (`os_keychain`, `enterprise_vault`,
//!   `delegated_identity`, `session_only`, `workspace_variable`,
//!   `missing`) each have an `auth_source_admission` row so request
//!   workflows disclose where credential material resolves,
//! - the six connection states (`connected`, `constrained`,
//!   `offline_local_safe`, `reauth_required`, `reconciliation_pending`,
//!   `service_unavailable`) each have a `connection_state_admission`
//!   row so offline, reauth, and reconciliation posture stay aligned
//!   across UI, CLI, and support packets,
//! - the eight streaming-response states (`connecting`,
//!   `headers_received`, `streaming`, `truncated`, `complete`,
//!   `partial`, `timed_out`, `policy_blocked`) each have a
//!   `streaming_response_state_admission` row so freshness,
//!   truncation, and policy-block states never collapse into a single
//!   pass/fail bit,
//! - each of the eight consumer surfaces (request editor, response
//!   timeline, mutation-review sheet, replay/history, CLI/headless
//!   inspect, support export, Help/About, conformance dashboard) has
//!   a `consumer_surface_binding` row attesting it reads this packet
//!   verbatim,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through event streams, support packets, approval
//!   tickets, and evidence exports via a `lineage_admission` row.
//!
//! Every row binds a closed `request_execution_lane_class`,
//! `request_execution_row_class`, `support_class`, `wedge_class`,
//! `auth_source_mode`, `connection_state_class`,
//! `streaming_response_state_class`, `consumer_surface_class`,
//! `evidence_class`, `known_limit_class`, `downgrade_automation_class`,
//! and `confidence_class` plus an `evidence_refs` array and a
//! `disclosure_ref` whenever the row is narrowed below launch-stable,
//! declares a non-`none_declared` known limit, or binds a non-`none`
//! downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! request bodies, raw response bodies, raw headers, raw cookies, raw
//! secret values, raw command lines, or ambient credentials past the
//! boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, or evidence class unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`RequestExecutionContextTruthPacket`].
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_RECORD_KIND: &str =
    "finalize_request_workspace_and_api_request_execution_context_truth_stable_packet";

/// Stable record-kind tag for [`RequestExecutionContextTruthSupportExport`].
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "finalize_request_workspace_and_api_request_execution_context_truth_support_export";

/// Integer schema version for the request-workspace execution-context truth packet.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/finalize_request_workspace_and_api_request_execution_context_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/finalize-request-workspace-and-api-request-execution-context.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/finalize-request-workspace-and-api-request-execution-context.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/finalize_request_workspace_and_api_request_execution_context";

/// Repo-relative path of the checked-in stable packet.
pub const REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/finalize_request_workspace_and_api_request_execution_context_truth_packet.json";

/// Closed execution-context lane vocabulary covering the request /
/// API-request domain. Every required lane MUST have at least one row
/// in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestExecutionLaneClass {
    /// Request-workspace lane: authored requests with versionable
    /// files, layered environment, and assertion suites.
    RequestWorkspaceLane,
    /// API-request lane: live send, replay, and streaming-response
    /// dispatch through the runtime.
    ApiRequestLane,
    /// Response-trust lane: response artifacts, preview/redaction
    /// posture, and assertion evidence carried forward into history
    /// and export.
    ResponseTrustLane,
    /// Data-action lane: SQL or result-grid-initiated actions,
    /// browser-runtime continuation, and storage-touching follow-up
    /// flows.
    DataActionLane,
}

impl RequestExecutionLaneClass {
    /// Every required request-execution lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::RequestWorkspaceLane,
        Self::ApiRequestLane,
        Self::ResponseTrustLane,
        Self::DataActionLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestWorkspaceLane => "request_workspace_lane",
            Self::ApiRequestLane => "api_request_lane",
            Self::ResponseTrustLane => "response_trust_lane",
            Self::DataActionLane => "data_action_lane",
        }
    }
}

/// Closed row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestExecutionRowClass {
    /// The lane's headline execution-context reuse qualification row.
    ExecutionContextReuseQuality,
    /// A row admitting one of the four required wedges
    /// (route_target_truth, auth_source_truth, approval_review_truth,
    /// execution_context_reuse_truth).
    WedgeAdmission,
    /// A row admitting one of the six required auth-source modes
    /// (os_keychain, enterprise_vault, delegated_identity,
    /// session_only, workspace_variable, missing).
    AuthSourceAdmission,
    /// A row admitting one of the six required connection states
    /// (connected, constrained, offline_local_safe, reauth_required,
    /// reconciliation_pending, service_unavailable).
    ConnectionStateAdmission,
    /// A row admitting one of the eight required streaming-response
    /// states (connecting, headers_received, streaming, truncated,
    /// complete, partial, timed_out, policy_blocked).
    StreamingResponseStateAdmission,
    /// A row binding one consumer surface (request editor, response
    /// timeline, mutation-review sheet, replay/history, CLI/headless
    /// inspect, support export, Help/About, conformance dashboard).
    ConsumerSurfaceBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object reference) into event streams, support packets,
    /// approval tickets, and evidence exports.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl RequestExecutionRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionContextReuseQuality => "execution_context_reuse_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::AuthSourceAdmission => "auth_source_admission",
            Self::ConnectionStateAdmission => "connection_state_admission",
            Self::StreamingResponseStateAdmission => "streaming_response_state_admission",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge token.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound auth-source token.
    pub const fn requires_auth_source(self) -> bool {
        matches!(self, Self::AuthSourceAdmission)
    }

    /// True when this row class requires a bound connection-state token.
    pub const fn requires_connection_state(self) -> bool {
        matches!(self, Self::ConnectionStateAdmission)
    }

    /// True when this row class requires a bound streaming-response-state token.
    pub const fn requires_streaming_state(self) -> bool {
        matches!(self, Self::StreamingResponseStateAdmission)
    }

    /// True when this row class requires a bound consumer-surface token.
    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a request-execution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestExecutionSupportClass {
    /// Row claims M4 launch-stable grade for the execution lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable; the narrowing is disclosed.
    LaunchStableBelow,
    /// Row is at beta-grade only (capability sample, not launch-stable).
    BetaGradeOnly,
    /// Row is at preview only (under-review wedge).
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl RequestExecutionSupportClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LaunchStable => "launch_stable",
            Self::LaunchStableBelow => "launch_stable_below",
            Self::BetaGradeOnly => "beta_grade_only",
            Self::PreviewOnly => "preview_only",
            Self::Unsupported => "unsupported",
            Self::SupportUnbound => "support_unbound",
        }
    }

    /// True when this support class satisfies the support-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::SupportUnbound)
    }

    /// True when the support class must surface a disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::LaunchStable)
    }
}

/// Closed wedge vocabulary. Every lane claiming `launch_stable` MUST
/// publish a `wedge_admission` row for each required wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Route + target identity stay visible and never silently widen on rerun.
    RouteTargetTruth,
    /// Auth-source class and credential resolution stay portable and reviewable.
    AuthSourceTruth,
    /// Mutation-review / approval-review posture stays explicit for
    /// non-idempotent and destructive actions.
    ApprovalReviewTruth,
    /// Execution-context object is reused across request, replay,
    /// follow-up debug, and data-action dispatch without forking.
    ExecutionContextReuseTruth,
    /// The row is not bound to a wedge (non-wedge row classes).
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::RouteTargetTruth,
        Self::AuthSourceTruth,
        Self::ApprovalReviewTruth,
        Self::ExecutionContextReuseTruth,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteTargetTruth => "route_target_truth",
            Self::AuthSourceTruth => "auth_source_truth",
            Self::ApprovalReviewTruth => "approval_review_truth",
            Self::ExecutionContextReuseTruth => "execution_context_reuse_truth",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed auth-source mode vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `auth_source_admission` row for each
/// mode so request flows disclose where credential material resolves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthSourceModeClass {
    /// Credential resolves through OS keychain.
    OsKeychain,
    /// Credential resolves through an enterprise vault.
    EnterpriseVault,
    /// Credential resolves through a delegated identity broker.
    DelegatedIdentity,
    /// Credential is bound to the current session only.
    SessionOnly,
    /// Credential resolves through a workspace variable (secret-handle
    /// indirection — never raw inline value).
    WorkspaceVariable,
    /// Credential is missing; the surface MUST refuse to dispatch.
    Missing,
    /// The row is not bound to an auth-source mode (non-auth-source row classes).
    NotApplicable,
}

impl AuthSourceModeClass {
    /// Every required auth-source mode per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::OsKeychain,
        Self::EnterpriseVault,
        Self::DelegatedIdentity,
        Self::SessionOnly,
        Self::WorkspaceVariable,
        Self::Missing,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OsKeychain => "os_keychain",
            Self::EnterpriseVault => "enterprise_vault",
            Self::DelegatedIdentity => "delegated_identity",
            Self::SessionOnly => "session_only",
            Self::WorkspaceVariable => "workspace_variable",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed connection-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `connection_state_admission` row for
/// each state so offline, reauth, and reconciliation posture stay
/// aligned across UI, CLI, support, and imported packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionStateClass {
    /// Service is reachable and credentials are valid.
    Connected,
    /// Service is reachable but limited (e.g. read-only, rate-limited).
    Constrained,
    /// Service is unreachable; local cached reads are safe to surface.
    OfflineLocalSafe,
    /// Auth handshake expired; reauth is required before any dispatch.
    ReauthRequired,
    /// Deferred / queued intents exist; reconciliation review is required.
    ReconciliationPending,
    /// Service is unavailable; no dispatch is permitted.
    ServiceUnavailable,
    /// The row is not bound to a connection state (non-connection-state row classes).
    NotApplicable,
}

impl ConnectionStateClass {
    /// Every required connection state per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::Connected,
        Self::Constrained,
        Self::OfflineLocalSafe,
        Self::ReauthRequired,
        Self::ReconciliationPending,
        Self::ServiceUnavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Constrained => "constrained",
            Self::OfflineLocalSafe => "offline_local_safe",
            Self::ReauthRequired => "reauth_required",
            Self::ReconciliationPending => "reconciliation_pending",
            Self::ServiceUnavailable => "service_unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed streaming-response-state vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `streaming_response_state_admission`
/// row for each state so freshness, truncation, and policy-block
/// states never collapse into a single pass/fail bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StreamingResponseStateClass {
    /// Connection dialing; no headers yet.
    Connecting,
    /// Response headers received; body has not started streaming.
    HeadersReceived,
    /// Response body is actively streaming.
    Streaming,
    /// Response body was truncated mid-stream.
    Truncated,
    /// Response completed cleanly.
    Complete,
    /// Response completed but is a partial view of the resource.
    Partial,
    /// Stream stopped because the deadline expired.
    TimedOut,
    /// Stream stopped because a policy or trust gate blocked the response.
    PolicyBlocked,
    /// The row is not bound to a streaming-response state (non-streaming row classes).
    NotApplicable,
}

impl StreamingResponseStateClass {
    /// Every required streaming-response state per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 8] = [
        Self::Connecting,
        Self::HeadersReceived,
        Self::Streaming,
        Self::Truncated,
        Self::Complete,
        Self::Partial,
        Self::TimedOut,
        Self::PolicyBlocked,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connecting => "connecting",
            Self::HeadersReceived => "headers_received",
            Self::Streaming => "streaming",
            Self::Truncated => "truncated",
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::TimedOut => "timed_out",
            Self::PolicyBlocked => "policy_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed consumer-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `consumer_surface_binding` row for
/// each surface so the request editor, response timeline,
/// mutation-review sheet, replay/history surface, CLI/headless
/// inspect, support export bundle, Help/About proof card, and the
/// conformance dashboard all read this packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Request editor surface (authored request workspace).
    RequestEditorSurface,
    /// Response timeline surface (streaming-response bar, redaction posture).
    ResponseTimelineSurface,
    /// Mutation-review sheet (target identity, auth scope, side-effect class).
    MutationReviewSheet,
    /// Replay / history surface (current vs. archived run posture).
    ReplayHistorySurface,
    /// CLI / headless inspect surface (`aureline request inspect`).
    CliHeadlessInspect,
    /// Support export bundle surface.
    SupportExport,
    /// Help / About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
    /// The row is not bound to a consumer surface (non-surface row classes).
    NotApplicable,
}

impl ConsumerSurfaceClass {
    /// Every required consumer surface per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 8] = [
        Self::RequestEditorSurface,
        Self::ResponseTimelineSurface,
        Self::MutationReviewSheet,
        Self::ReplayHistorySurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestEditorSurface => "request_editor_surface",
            Self::ResponseTimelineSurface => "response_timeline_surface",
            Self::MutationReviewSheet => "mutation_review_sheet",
            Self::ReplayHistorySurface => "replay_history_surface",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed evidence-class vocabulary describing what backs a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceClass {
    /// The row is backed by an automated functional / unit suite.
    AutomatedFunctionalEvidence,
    /// The row is backed by a conformance / interoperability suite.
    ConformanceSuiteEvidence,
    /// The row is backed by a failure / recovery drill.
    FailureRecoveryDrillEvidence,
    /// The row is backed by design-QA / UX validation.
    DesignQaEvidence,
    /// The row is backed by release-evidence review.
    ReleaseEvidenceReview,
    /// The row is backed by a fixture-repo capture.
    FixtureRepoEvidence,
    /// The row is backed by a docs/help disclosure (gap label only).
    DocsDisclosureEvidence,
    /// The row has no bound evidence class; this never qualifies stable.
    EvidenceUnbound,
}

impl EvidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AutomatedFunctionalEvidence => "automated_functional_evidence",
            Self::ConformanceSuiteEvidence => "conformance_suite_evidence",
            Self::FailureRecoveryDrillEvidence => "failure_recovery_drill_evidence",
            Self::DesignQaEvidence => "design_qa_evidence",
            Self::ReleaseEvidenceReview => "release_evidence_review",
            Self::FixtureRepoEvidence => "fixture_repo_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a request-execution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the request-workspace subset.
    RequestWorkspaceSubsetOnly,
    /// The lane only certifies the API-request subset.
    ApiRequestSubsetOnly,
    /// The lane only certifies the response-trust subset.
    ResponseTrustSubsetOnly,
    /// The lane only certifies the data-action subset.
    DataActionSubsetOnly,
    /// The lane only certifies a subset of the four required wedges.
    WedgeSubsetOnly,
    /// The lane only certifies a subset of the six required auth-source modes.
    AuthSourceModeSubsetOnly,
    /// The lane only certifies a subset of the six required connection states.
    ConnectionStateSubsetOnly,
    /// The lane only certifies a subset of the eight required streaming states.
    StreamingResponseStateSubsetOnly,
    /// The lane only certifies a subset of the eight required consumer surfaces.
    ConsumerSurfaceSubsetOnly,
    /// The lane certifies an unsupported request route or auth gap.
    UnsupportedRequestRoute,
    /// The lane is at beta-grade-only capability sample.
    BetaCapabilitySampleOnly,
    /// The row has no bound known limit class; this never qualifies stable.
    LimitUnbound,
}

impl KnownLimitClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneDeclared => "none_declared",
            Self::RequestWorkspaceSubsetOnly => "request_workspace_subset_only",
            Self::ApiRequestSubsetOnly => "api_request_subset_only",
            Self::ResponseTrustSubsetOnly => "response_trust_subset_only",
            Self::DataActionSubsetOnly => "data_action_subset_only",
            Self::WedgeSubsetOnly => "wedge_subset_only",
            Self::AuthSourceModeSubsetOnly => "auth_source_mode_subset_only",
            Self::ConnectionStateSubsetOnly => "connection_state_subset_only",
            Self::StreamingResponseStateSubsetOnly => "streaming_response_state_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::UnsupportedRequestRoute => "unsupported_request_route",
            Self::BetaCapabilitySampleOnly => "beta_capability_sample_only",
            Self::LimitUnbound => "limit_unbound",
        }
    }

    /// True when this known-limit class satisfies the limit-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::LimitUnbound)
    }

    /// True when this known-limit class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::NoneDeclared | Self::LimitUnbound)
    }
}

/// Closed downgrade-automation vocabulary attached to a request-execution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when route / target identity is unbound.
    AutoNarrowOnRouteTargetGap,
    /// Automatically narrow when an auth-source mode is unbound.
    AutoNarrowOnAuthSourceGap,
    /// Automatically narrow when a connection-state admission is unbound.
    AutoNarrowOnConnectionStateGap,
    /// Automatically narrow when a streaming-response-state admission is unbound.
    AutoNarrowOnStreamingStateGap,
    /// Automatically narrow when the consumer-surface binding is missing.
    AutoNarrowOnConsumerSurfaceGap,
    /// Automatically narrow when the approval-review posture drifts
    /// between dispatch and replay.
    AutoNarrowOnApprovalReviewDrift,
    /// Automatically narrow when the lineage object breaks (no
    /// `execution_context_id` binding survives across event streams,
    /// support packets, approval tickets, or evidence exports).
    AutoNarrowOnLineageBreak,
    /// Automatically narrow when a deferred queue would silently
    /// dispatch a non-idempotent or destructive intent.
    AutoNarrowOnSilentQueueDispatch,
    /// Automatically block when required evidence is missing.
    AutoBlockOnMissingEvidence,
    /// Manual-only review required until automation lands.
    ManualOnlyPendingReview,
    /// Automation is unbound; this never qualifies stable.
    AutomationUnbound,
}

impl DowngradeAutomationClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AutoNarrowOnRouteTargetGap => "auto_narrow_on_route_target_gap",
            Self::AutoNarrowOnAuthSourceGap => "auto_narrow_on_auth_source_gap",
            Self::AutoNarrowOnConnectionStateGap => "auto_narrow_on_connection_state_gap",
            Self::AutoNarrowOnStreamingStateGap => "auto_narrow_on_streaming_state_gap",
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnApprovalReviewDrift => "auto_narrow_on_approval_review_drift",
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
            Self::AutoNarrowOnSilentQueueDispatch => "auto_narrow_on_silent_queue_dispatch",
            Self::AutoBlockOnMissingEvidence => "auto_block_on_missing_evidence",
            Self::ManualOnlyPendingReview => "manual_only_pending_review",
            Self::AutomationUnbound => "automation_unbound",
        }
    }

    /// True when this automation class satisfies the automation-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::AutomationUnbound)
    }

    /// True when this automation class must surface an explicit disclosure ref.
    pub const fn requires_explicit_disclosure(self) -> bool {
        !matches!(self, Self::None | Self::AutomationUnbound)
    }
}

/// Closed confidence-class vocabulary for a request-execution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidence => "high_confidence",
            Self::MediumConfidence => "medium_confidence",
            Self::LowConfidence => "low_confidence",
        }
    }
}

/// Stable promotion state derived from packet validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim across all required rows.
    Stable,
    /// Packet narrows below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker finding that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for the request-execution-context truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    WrongRecordKind,
    WrongSchemaVersion,
    MissingIdentity,
    MissingExecutionLaneCoverage,
    MissingWedgeCoverage,
    MissingAuthSourceCoverage,
    MissingConnectionStateCoverage,
    MissingStreamingResponseStateCoverage,
    MissingConsumerSurfaceCoverage,
    MissingLineageAdmission,
    MissingSupportClass,
    MissingKnownLimit,
    MissingDowngradeAutomation,
    MissingEvidenceClass,
    LaunchStableWithUnboundBinding,
    NarrowedRowMissingDisclosureRef,
    KnownLimitMissingDisclosureRef,
    DowngradeAutomationMissingDisclosureRef,
    MissingEvidenceRefs,
    WedgeNotApplicable,
    WedgeNotPermittedOnRowClass,
    AuthSourceNotApplicable,
    AuthSourceNotPermittedOnRowClass,
    ConnectionStateNotApplicable,
    ConnectionStateNotPermittedOnRowClass,
    StreamingStateNotApplicable,
    StreamingStateNotPermittedOnRowClass,
    ConsumerSurfaceNotApplicable,
    ConsumerSurfaceNotPermittedOnRowClass,
    LineageAdmissionMissingExecutionContextId,
    MutationReviewBindingMissingApproval,
    SilentDeferredQueueAdmitted,
    RawSourceMaterialPresent,
    SecretsPresent,
    AmbientAuthorityPresent,
    MissingConsumerProjection,
    ConsumerProjectionDrift,
    LaneVocabularyCollapsed,
    RowClassVocabularyCollapsed,
    SupportClassVocabularyCollapsed,
    WedgeVocabularyCollapsed,
    AuthSourceVocabularyCollapsed,
    ConnectionStateVocabularyCollapsed,
    StreamingResponseStateVocabularyCollapsed,
    ConsumerSurfaceVocabularyCollapsed,
    KnownLimitVocabularyCollapsed,
    DowngradeAutomationVocabularyCollapsed,
    EvidenceClassVocabularyCollapsed,
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingExecutionLaneCoverage => "missing_execution_lane_coverage",
            Self::MissingWedgeCoverage => "missing_wedge_coverage",
            Self::MissingAuthSourceCoverage => "missing_auth_source_coverage",
            Self::MissingConnectionStateCoverage => "missing_connection_state_coverage",
            Self::MissingStreamingResponseStateCoverage => {
                "missing_streaming_response_state_coverage"
            }
            Self::MissingConsumerSurfaceCoverage => "missing_consumer_surface_coverage",
            Self::MissingLineageAdmission => "missing_lineage_admission",
            Self::MissingSupportClass => "missing_support_class",
            Self::MissingKnownLimit => "missing_known_limit",
            Self::MissingDowngradeAutomation => "missing_downgrade_automation",
            Self::MissingEvidenceClass => "missing_evidence_class",
            Self::LaunchStableWithUnboundBinding => "launch_stable_with_unbound_binding",
            Self::NarrowedRowMissingDisclosureRef => "narrowed_row_missing_disclosure_ref",
            Self::KnownLimitMissingDisclosureRef => "known_limit_missing_disclosure_ref",
            Self::DowngradeAutomationMissingDisclosureRef => {
                "downgrade_automation_missing_disclosure_ref"
            }
            Self::MissingEvidenceRefs => "missing_evidence_refs",
            Self::WedgeNotApplicable => "wedge_not_applicable",
            Self::WedgeNotPermittedOnRowClass => "wedge_not_permitted_on_row_class",
            Self::AuthSourceNotApplicable => "auth_source_not_applicable",
            Self::AuthSourceNotPermittedOnRowClass => "auth_source_not_permitted_on_row_class",
            Self::ConnectionStateNotApplicable => "connection_state_not_applicable",
            Self::ConnectionStateNotPermittedOnRowClass => {
                "connection_state_not_permitted_on_row_class"
            }
            Self::StreamingStateNotApplicable => "streaming_state_not_applicable",
            Self::StreamingStateNotPermittedOnRowClass => {
                "streaming_state_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::MutationReviewBindingMissingApproval => {
                "mutation_review_binding_missing_approval"
            }
            Self::SilentDeferredQueueAdmitted => "silent_deferred_queue_admitted",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::AuthSourceVocabularyCollapsed => "auth_source_vocabulary_collapsed",
            Self::ConnectionStateVocabularyCollapsed => "connection_state_vocabulary_collapsed",
            Self::StreamingResponseStateVocabularyCollapsed => {
                "streaming_response_state_vocabulary_collapsed"
            }
            Self::ConsumerSurfaceVocabularyCollapsed => "consumer_surface_vocabulary_collapsed",
            Self::KnownLimitVocabularyCollapsed => "known_limit_vocabulary_collapsed",
            Self::DowngradeAutomationVocabularyCollapsed => {
                "downgrade_automation_vocabulary_collapsed"
            }
            Self::EvidenceClassVocabularyCollapsed => "evidence_class_vocabulary_collapsed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// Consumer surface that must inherit the request-execution-context packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerProjectionSurface {
    /// Request editor surface (authored request workspace).
    RequestEditorSurface,
    /// Response timeline surface (streaming-response bar, redaction posture).
    ResponseTimelineSurface,
    /// Mutation-review sheet (target identity, auth scope, side-effect class).
    MutationReviewSheet,
    /// Replay / history surface (current vs. archived run posture).
    ReplayHistorySurface,
    /// CLI / headless inspect surface.
    CliHeadlessInspect,
    /// Support export bundle surface.
    SupportExport,
    /// Help / About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerProjectionSurface {
    /// Every required consumer projection surface, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::RequestEditorSurface,
        Self::ResponseTimelineSurface,
        Self::MutationReviewSheet,
        Self::ReplayHistorySurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestEditorSurface => "request_editor_surface",
            Self::ResponseTimelineSurface => "response_timeline_surface",
            Self::MutationReviewSheet => "mutation_review_sheet",
            Self::ReplayHistorySurface => "replay_history_surface",
            Self::CliHeadlessInspect => "cli_headless_inspect",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
            Self::ConformanceDashboard => "conformance_dashboard",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// One request-execution-context truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestExecutionContextRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Lane this row certifies.
    pub lane_class: RequestExecutionLaneClass,
    /// Row class.
    pub row_class: RequestExecutionRowClass,
    /// Support class claimed by the row.
    pub support_class: RequestExecutionSupportClass,
    /// Wedge certified by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Auth-source mode certified by the row (or `not_applicable`).
    pub auth_source_mode: AuthSourceModeClass,
    /// Connection state certified by the row (or `not_applicable`).
    pub connection_state_class: ConnectionStateClass,
    /// Streaming-response state certified by the row (or `not_applicable`).
    pub streaming_response_state_class: StreamingResponseStateClass,
    /// Consumer surface certified by the row (or `not_applicable`).
    pub consumer_surface_class: ConsumerSurfaceClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit, or
    /// binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For approval-review-truth wedge admissions on mutating lanes,
    /// true when the row attests that the lane routes non-idempotent
    /// or destructive actions through an explicit mutation-review
    /// sheet before dispatch and re-enters review after auth, route,
    /// policy, or target drift.
    #[serde(default)]
    pub approval_review_attested: bool,
    /// True when the row attests that no deferred queue silently
    /// dispatches a non-idempotent or destructive intent without
    /// explicit review.
    #[serde(default)]
    pub silent_deferred_queue_blocked: bool,
    /// True when raw request bodies / raw response bodies / raw
    /// headers / raw cookies / raw command lines are excluded from
    /// this row.
    pub raw_source_material_excluded: bool,
    /// True when raw secret values are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl RequestExecutionContextRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerProjectionSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub request_execution_context_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the lane vocabulary is preserved verbatim.
    pub preserves_lane_vocabulary: bool,
    /// True when the row-class vocabulary is preserved verbatim.
    pub preserves_row_class_vocabulary: bool,
    /// True when the support-class vocabulary is preserved verbatim.
    pub preserves_support_class_vocabulary: bool,
    /// True when the wedge vocabulary is preserved verbatim.
    pub preserves_wedge_vocabulary: bool,
    /// True when the auth-source vocabulary is preserved verbatim.
    pub preserves_auth_source_vocabulary: bool,
    /// True when the connection-state vocabulary is preserved verbatim.
    pub preserves_connection_state_vocabulary: bool,
    /// True when the streaming-response-state vocabulary is preserved verbatim.
    pub preserves_streaming_response_state_vocabulary: bool,
    /// True when the consumer-surface vocabulary is preserved verbatim.
    pub preserves_consumer_surface_vocabulary: bool,
    /// True when the known-limit vocabulary is preserved verbatim.
    pub preserves_known_limit_vocabulary: bool,
    /// True when the downgrade-automation vocabulary is preserved verbatim.
    pub preserves_downgrade_automation_vocabulary: bool,
    /// True when the evidence-class vocabulary is preserved verbatim.
    pub preserves_evidence_class_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl ConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.request_execution_context_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_auth_source_vocabulary
            && self.preserves_connection_state_vocabulary
            && self.preserves_streaming_response_state_vocabulary
            && self.preserves_consumer_surface_vocabulary
            && self.preserves_known_limit_vocabulary
            && self.preserves_downgrade_automation_vocabulary
            && self.preserves_evidence_class_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`RequestExecutionContextTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestExecutionContextTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Execution lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<RequestExecutionLaneClass>,
    /// Request-execution-context rows.
    #[serde(default)]
    pub rows: Vec<RequestExecutionContextRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying request-workspace, API-request,
/// response-trust, and data-action execution-context reuse at the M4
/// launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestExecutionContextTruthPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub workflow_or_surface_id: String,
    pub generated_at: String,
    #[serde(default)]
    pub covered_lanes: Vec<RequestExecutionLaneClass>,
    #[serde(default)]
    pub rows: Vec<RequestExecutionContextRow>,
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    pub promotion_state: PromotionState,
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl RequestExecutionContextTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: RequestExecutionContextTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_lanes: input.covered_lanes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerProjectionSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique lane tokens observed across rows.
    pub fn lane_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.lane_class);
        }
        set.into_iter().map(RequestExecutionLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(RequestExecutionRowClass::as_str).collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(RequestExecutionSupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique auth-source tokens observed across rows.
    pub fn auth_source_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.auth_source_mode);
        }
        set.into_iter().map(AuthSourceModeClass::as_str).collect()
    }

    /// Returns the unique connection-state tokens observed across rows.
    pub fn connection_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.connection_state_class);
        }
        set.into_iter().map(ConnectionStateClass::as_str).collect()
    }

    /// Returns the unique streaming-response-state tokens observed across rows.
    pub fn streaming_response_state_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.streaming_response_state_class);
        }
        set.into_iter().map(StreamingResponseStateClass::as_str).collect()
    }

    /// Returns the unique consumer-surface tokens observed across rows.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface_class);
        }
        set.into_iter().map(ConsumerSurfaceClass::as_str).collect()
    }

    /// Returns the unique evidence-class tokens observed across rows.
    pub fn evidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.evidence_class);
        }
        set.into_iter().map(EvidenceClass::as_str).collect()
    }

    /// Returns the unique known-limit tokens observed across rows.
    pub fn known_limit_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.known_limit_class);
        }
        set.into_iter().map(KnownLimitClass::as_str).collect()
    }

    /// Returns the unique downgrade-automation tokens observed across rows.
    pub fn downgrade_automation_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_automation_class);
        }
        set.into_iter().map(DowngradeAutomationClass::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> RequestExecutionContextTruthSupportExport {
        RequestExecutionContextTruthSupportExport {
            record_kind: REQUEST_EXECUTION_CONTEXT_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            request_execution_context_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            request_execution_context_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != REQUEST_EXECUTION_CONTEXT_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "request-execution-context packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "request-execution-context packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }
        if self.covered_lanes.is_empty() {
            findings.push(ValidationFinding::new(
                FindingKind::MissingExecutionLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered execution-context lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingExecutionLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers execution-context lane {}", lane.as_str()),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} identity or timestamp is empty", row.row_id),
                ));
            }
            if !row.raw_source_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawSourceMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw request/response bodies, headers, or cookies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits raw secret values past the boundary", row.row_id),
                ));
            }
            if !row.ambient_authority_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbientAuthorityPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits ambient authority/credentials past the boundary",
                        row.row_id
                    ),
                ));
            }

            if !row.support_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingSupportClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound support class", row.row_id),
                ));
            }
            if !row.known_limit_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingKnownLimit,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound known-limit class", row.row_id),
                ));
            }
            if !row.downgrade_automation_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDowngradeAutomation,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has no bound downgrade-automation class",
                        row.row_id
                    ),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(
                row.support_class,
                RequestExecutionSupportClass::LaunchStable
            ) && !row.all_bindings_satisfied()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} claims launch_stable while a binding (support, known limit, downgrade automation, or evidence) is unbound",
                        row.row_id
                    ),
                ));
            }

            if row.support_class.requires_explicit_disclosure() && row.disclosure_ref.is_none() {
                findings.push(ValidationFinding::new(
                    FindingKind::NarrowedRowMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has support class {} without a disclosure ref",
                        row.row_id,
                        row.support_class.as_str()
                    ),
                ));
            }
            if row.known_limit_class.requires_explicit_disclosure() && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} discloses known limit {} without a disclosure ref",
                        row.row_id,
                        row.known_limit_class.as_str()
                    ),
                ));
            }
            if row.downgrade_automation_class.requires_explicit_disclosure()
                && row.disclosure_ref.is_none()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationMissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds downgrade automation {} without a disclosure ref",
                        row.row_id,
                        row.downgrade_automation_class.as_str()
                    ),
                ));
            }

            if row.evidence_refs.is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceRefs,
                    FindingSeverity::Blocker,
                    format!("row {} carries no evidence refs", row.row_id),
                ));
            }

            // wedge binding rules
            if row.row_class.requires_wedge()
                && matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a wedge_admission but has no bound wedge",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_wedge()
                && !matches!(row.wedge_class, WedgeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds wedge {}; only wedge_admission rows may bind a wedge",
                        row.row_id,
                        row.row_class.as_str(),
                        row.wedge_class.as_str()
                    ),
                ));
            }

            // auth-source binding rules
            if row.row_class.requires_auth_source()
                && matches!(row.auth_source_mode, AuthSourceModeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AuthSourceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an auth_source_admission but has no bound mode",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_auth_source()
                && !matches!(row.auth_source_mode, AuthSourceModeClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AuthSourceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds auth source {}; only auth_source_admission rows may bind a mode",
                        row.row_id,
                        row.row_class.as_str(),
                        row.auth_source_mode.as_str()
                    ),
                ));
            }

            // connection-state binding rules
            if row.row_class.requires_connection_state()
                && matches!(row.connection_state_class, ConnectionStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConnectionStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a connection_state_admission but has no bound state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_connection_state()
                && !matches!(row.connection_state_class, ConnectionStateClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConnectionStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds connection state {}; only connection_state_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.connection_state_class.as_str()
                    ),
                ));
            }

            // streaming-state binding rules
            if row.row_class.requires_streaming_state()
                && matches!(
                    row.streaming_response_state_class,
                    StreamingResponseStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::StreamingStateNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a streaming_response_state_admission but has no bound state",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_streaming_state()
                && !matches!(
                    row.streaming_response_state_class,
                    StreamingResponseStateClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::StreamingStateNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds streaming response state {}; only streaming_response_state_admission rows may bind a state",
                        row.row_id,
                        row.row_class.as_str(),
                        row.streaming_response_state_class.as_str()
                    ),
                ));
            }

            // consumer-surface binding rules
            if row.row_class.requires_consumer_surface()
                && matches!(row.consumer_surface_class, ConsumerSurfaceClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a consumer_surface_binding but has no bound surface",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_consumer_surface()
                && !matches!(row.consumer_surface_class, ConsumerSurfaceClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds consumer surface {}; only consumer_surface_binding rows may bind a surface",
                        row.row_id,
                        row.row_class.as_str(),
                        row.consumer_surface_class.as_str()
                    ),
                ));
            }

            // lineage admission rules
            if matches!(row.row_class, RequestExecutionRowClass::LineageAdmission)
                && row
                    .execution_context_id_binding
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    .unwrap_or(true)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LineageAdmissionMissingExecutionContextId,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a lineage_admission but has no bound execution_context_id",
                        row.row_id
                    ),
                ));
            }

            // approval-review and silent-deferred queue rules
            if matches!(row.row_class, RequestExecutionRowClass::WedgeAdmission)
                && matches!(row.wedge_class, WedgeClass::ApprovalReviewTruth)
                && !row.approval_review_attested
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MutationReviewBindingMissingApproval,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds approval_review_truth but does not attest mutation-review enforcement",
                        row.row_id
                    ),
                ));
            }

            if matches!(
                row.row_class,
                RequestExecutionRowClass::ConnectionStateAdmission
            ) && matches!(
                row.connection_state_class,
                ConnectionStateClass::ReconciliationPending
            ) && !row.silent_deferred_queue_blocked
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SilentDeferredQueueAdmitted,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits reconciliation_pending without attesting silent-deferred-queue blocking",
                        row.row_id
                    ),
                ));
            }

            if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
                && matches!(row.support_class, RequestExecutionSupportClass::LaunchStable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::LaunchStableWithUnboundBinding,
                    FindingSeverity::Warning,
                    format!(
                        "row {} claims launch_stable at low_confidence; narrowing until evidence grows",
                        row.row_id
                    ),
                ));
            }
        }

        // per-lane coverage for lanes claiming launch_stable
        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        RequestExecutionRowClass::ExecutionContextReuseQuality
                    )
                    && matches!(row.support_class, RequestExecutionSupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, RequestExecutionRowClass::WedgeAdmission)
                        && row.wedge_class == wedge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWedgeCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no wedge_admission row for {}",
                            lane.as_str(),
                            wedge.as_str()
                        ),
                    ));
                }
            }

            for mode in AuthSourceModeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, RequestExecutionRowClass::AuthSourceAdmission)
                        && row.auth_source_mode == mode
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingAuthSourceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no auth_source_admission row for {}",
                            lane.as_str(),
                            mode.as_str()
                        ),
                    ));
                }
            }

            for state in ConnectionStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            RequestExecutionRowClass::ConnectionStateAdmission
                        )
                        && row.connection_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConnectionStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no connection_state_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for state in StreamingResponseStateClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            RequestExecutionRowClass::StreamingResponseStateAdmission
                        )
                        && row.streaming_response_state_class == state
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingStreamingResponseStateCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no streaming_response_state_admission row for {}",
                            lane.as_str(),
                            state.as_str()
                        ),
                    ));
                }
            }

            for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            RequestExecutionRowClass::ConsumerSurfaceBinding
                        )
                        && row.consumer_surface_class == surface
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConsumerSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no consumer_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, RequestExecutionRowClass::LineageAdmission)
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            });
            if !has_lineage {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLineageAdmission,
                    FindingSeverity::Blocker,
                    format!(
                        "lane {} claims launch_stable but has no lineage_admission row binding execution_context_id",
                        lane.as_str()
                    ),
                ));
            }
        }

        // consumer projections
        for required_surface in ConsumerProjectionSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve request-execution-context truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_lane_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::LaneVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the lane vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_support_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SupportClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the support-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_wedge_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WedgeVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the wedge vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_auth_source_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AuthSourceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the auth-source vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_connection_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ConnectionStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the connection-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_streaming_response_state_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::StreamingResponseStateVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the streaming-response-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_consumer_surface_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerSurfaceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the consumer-surface vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_known_limit_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::KnownLimitVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the known-limit vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_automation_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeAutomationVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the downgrade-automation vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_evidence_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::EvidenceClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the evidence-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != FindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RequestExecutionContextTruthSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub export_id: String,
    pub request_execution_context_packet_id_ref: String,
    pub exported_at: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub request_execution_context_packet: RequestExecutionContextTruthPacket,
}

impl RequestExecutionContextTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == REQUEST_EXECUTION_CONTEXT_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == REQUEST_EXECUTION_CONTEXT_TRUTH_SCHEMA_VERSION
            && self.request_execution_context_packet_id_ref
                == self.request_execution_context_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.request_execution_context_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum RequestExecutionContextTruthArtifactError {
    Packet(serde_json::Error),
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for RequestExecutionContextTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "request-execution-context packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "request-execution-context packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RequestExecutionContextTruthArtifactError {}

/// Returns the checked-in stable request-execution-context truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_request_execution_context_truth_packet(
) -> Result<RequestExecutionContextTruthPacket, RequestExecutionContextTruthArtifactError> {
    let packet: RequestExecutionContextTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/finalize_request_workspace_and_api_request_execution_context_truth_packet.json"
    )))
    .map_err(RequestExecutionContextTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(RequestExecutionContextTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        REQUEST_EXECUTION_CONTEXT_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        REQUEST_EXECUTION_CONTEXT_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: RequestExecutionLaneClass) -> RequestExecutionContextRow {
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: RequestExecutionRowClass::ExecutionContextReuseQuality,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            approval_review_attested: false,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: RequestExecutionLaneClass,
        wedge: WedgeClass,
    ) -> RequestExecutionContextRow {
        let approval_attested = matches!(wedge, WedgeClass::ApprovalReviewTruth);
        let automation = match wedge {
            WedgeClass::RouteTargetTruth => DowngradeAutomationClass::AutoNarrowOnRouteTargetGap,
            WedgeClass::AuthSourceTruth => DowngradeAutomationClass::AutoNarrowOnAuthSourceGap,
            WedgeClass::ApprovalReviewTruth => DowngradeAutomationClass::AutoNarrowOnApprovalReviewDrift,
            WedgeClass::ExecutionContextReuseTruth => DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            WedgeClass::NotApplicable => DowngradeAutomationClass::None,
        };
        let disclosure = Some(format!("{}#{}", doc_ref(), automation.as_str()));
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: RequestExecutionRowClass::WedgeAdmission,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: wedge,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: automation,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: disclosure,
            execution_context_id_binding: None,
            approval_review_attested: approval_attested,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn auth_source_row(
        prefix: &str,
        lane: RequestExecutionLaneClass,
        mode: AuthSourceModeClass,
    ) -> RequestExecutionContextRow {
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:auth_source:{}", mode.as_str()),
            lane_class: lane,
            row_class: RequestExecutionRowClass::AuthSourceAdmission,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: mode,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnAuthSourceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_auth_source_gap", doc_ref())),
            execution_context_id_binding: None,
            approval_review_attested: false,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn connection_state_row(
        prefix: &str,
        lane: RequestExecutionLaneClass,
        state: ConnectionStateClass,
    ) -> RequestExecutionContextRow {
        let blocks_queue = matches!(state, ConnectionStateClass::ReconciliationPending);
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:connection_state:{}", state.as_str()),
            lane_class: lane,
            row_class: RequestExecutionRowClass::ConnectionStateAdmission,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: state,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: if blocks_queue {
                DowngradeAutomationClass::AutoNarrowOnSilentQueueDispatch
            } else {
                DowngradeAutomationClass::AutoNarrowOnConnectionStateGap
            },
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#{}",
                doc_ref(),
                if blocks_queue {
                    "auto_narrow_on_silent_queue_dispatch"
                } else {
                    "auto_narrow_on_connection_state_gap"
                }
            )),
            execution_context_id_binding: None,
            approval_review_attested: false,
            silent_deferred_queue_blocked: blocks_queue,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn streaming_state_row(
        prefix: &str,
        lane: RequestExecutionLaneClass,
        state: StreamingResponseStateClass,
    ) -> RequestExecutionContextRow {
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:streaming_state:{}", state.as_str()),
            lane_class: lane,
            row_class: RequestExecutionRowClass::StreamingResponseStateAdmission,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: state,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnStreamingStateGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_streaming_state_gap", doc_ref())),
            execution_context_id_binding: None,
            approval_review_attested: false,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn consumer_surface_row(
        prefix: &str,
        lane: RequestExecutionLaneClass,
        surface: ConsumerSurfaceClass,
    ) -> RequestExecutionContextRow {
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:consumer_surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: RequestExecutionRowClass::ConsumerSurfaceBinding,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_consumer_surface_gap", doc_ref())),
            execution_context_id_binding: None,
            approval_review_attested: false,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: RequestExecutionLaneClass) -> RequestExecutionContextRow {
        RequestExecutionContextRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: RequestExecutionRowClass::LineageAdmission,
            support_class: RequestExecutionSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            auth_source_mode: AuthSourceModeClass::NotApplicable,
            connection_state_class: ConnectionStateClass::NotApplicable,
            streaming_response_state_class: StreamingResponseStateClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:request:{prefix}:lineage")),
            approval_review_attested: false,
            silent_deferred_queue_blocked: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerProjectionSurface) -> ConsumerProjection {
        ConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            request_execution_context_packet_id_ref:
                "packet:m4:finalize_request_workspace_and_api_request_execution_context".to_owned(),
            rendered_at: "2026-05-27T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_auth_source_vocabulary: true,
            preserves_connection_state_vocabulary: true,
            preserves_streaming_response_state_vocabulary: true,
            preserves_consumer_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: RequestExecutionLaneClass, prefix: &str) -> Vec<RequestExecutionContextRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for mode in AuthSourceModeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(auth_source_row(prefix, lane, mode));
        }
        for state in ConnectionStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(connection_state_row(prefix, lane, state));
        }
        for state in StreamingResponseStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(streaming_state_row(prefix, lane, state));
        }
        for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(consumer_surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> RequestExecutionContextTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(
            RequestExecutionLaneClass::RequestWorkspaceLane,
            "request_workspace",
        ));
        rows.extend(lane_rows(
            RequestExecutionLaneClass::ApiRequestLane,
            "api_request",
        ));
        rows.extend(lane_rows(
            RequestExecutionLaneClass::ResponseTrustLane,
            "response_trust",
        ));
        rows.extend(lane_rows(
            RequestExecutionLaneClass::DataActionLane,
            "data_action",
        ));
        RequestExecutionContextTruthPacketInput {
            packet_id: "packet:m4:finalize_request_workspace_and_api_request_execution_context"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.finalize_request_workspace_and_api_request_execution_context"
                    .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: RequestExecutionLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerProjectionSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(
            RequestExecutionLaneClass::RequestWorkspaceLane.as_str(),
            "request_workspace_lane"
        );
        assert_eq!(
            RequestExecutionLaneClass::DataActionLane.as_str(),
            "data_action_lane"
        );
        assert_eq!(
            RequestExecutionRowClass::ExecutionContextReuseQuality.as_str(),
            "execution_context_reuse_quality"
        );
        assert_eq!(
            RequestExecutionRowClass::LineageAdmission.as_str(),
            "lineage_admission"
        );
        assert_eq!(WedgeClass::RouteTargetTruth.as_str(), "route_target_truth");
        assert_eq!(
            WedgeClass::ExecutionContextReuseTruth.as_str(),
            "execution_context_reuse_truth"
        );
        assert_eq!(AuthSourceModeClass::OsKeychain.as_str(), "os_keychain");
        assert_eq!(AuthSourceModeClass::Missing.as_str(), "missing");
        assert_eq!(ConnectionStateClass::Connected.as_str(), "connected");
        assert_eq!(
            ConnectionStateClass::ReconciliationPending.as_str(),
            "reconciliation_pending"
        );
        assert_eq!(
            StreamingResponseStateClass::Connecting.as_str(),
            "connecting"
        );
        assert_eq!(
            StreamingResponseStateClass::PolicyBlocked.as_str(),
            "policy_blocked"
        );
        assert_eq!(
            ConsumerSurfaceClass::MutationReviewSheet.as_str(),
            "mutation_review_sheet"
        );
        assert_eq!(
            ConsumerSurfaceClass::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            FindingKind::MutationReviewBindingMissingApproval.as_str(),
            "mutation_review_binding_missing_approval"
        );
        assert_eq!(
            FindingKind::SilentDeferredQueueAdmitted.as_str(),
            "silent_deferred_queue_admitted"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = RequestExecutionContextTruthPacket::materialize(sample_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "expected stable but got findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|f| f.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
        assert!(packet
            .support_export(
                "support:m4:finalize_request_workspace_and_api_request_execution_context",
                "2026-05-27T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingEvidenceClass));
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::LaunchStableWithUnboundBinding));
    }

    #[test]
    fn missing_auth_source_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(row.row_class, RequestExecutionRowClass::AuthSourceAdmission)
                && row.auth_source_mode == AuthSourceModeClass::Missing
                && row.lane_class == RequestExecutionLaneClass::ApiRequestLane)
        });
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingAuthSourceCoverage));
    }

    #[test]
    fn missing_streaming_state_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                RequestExecutionRowClass::StreamingResponseStateAdmission
            ) && row.streaming_response_state_class
                == StreamingResponseStateClass::PolicyBlocked
                && row.lane_class == RequestExecutionLaneClass::ResponseTrustLane)
        });
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingStreamingResponseStateCoverage
        }));
    }

    #[test]
    fn approval_review_truth_without_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, RequestExecutionRowClass::WedgeAdmission)
                && row.wedge_class == WedgeClass::ApprovalReviewTruth
                && row.lane_class == RequestExecutionLaneClass::DataActionLane
            {
                row.approval_review_attested = false;
                break;
            }
        }
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MutationReviewBindingMissingApproval
        }));
    }

    #[test]
    fn reconciliation_pending_without_silent_queue_block_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                RequestExecutionRowClass::ConnectionStateAdmission
            ) && row.connection_state_class == ConnectionStateClass::ReconciliationPending
                && row.lane_class == RequestExecutionLaneClass::ApiRequestLane
            {
                row.silent_deferred_queue_blocked = false;
                break;
            }
        }
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::SilentDeferredQueueAdmitted));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, RequestExecutionRowClass::LineageAdmission)
                && row.lane_class == RequestExecutionLaneClass::RequestWorkspaceLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = RequestExecutionSupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::NarrowedRowMissingDisclosureRef));
    }

    #[test]
    fn projection_drop_blocks_promotion() {
        let mut input = sample_input();
        input.consumer_projections.retain(|projection| {
            projection.consumer_surface != ConsumerProjectionSurface::ConformanceDashboard
        });
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_streaming_state_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerProjectionSurface::HelpAbout {
                projection.preserves_streaming_response_state_vocabulary = false;
            }
        }
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::StreamingResponseStateVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = RequestExecutionContextTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
