//! Canonical test discovery, session, watch, quarantine, and imported-CI truth.
//!
//! This module defines the stable packet consumed by test explorer rows,
//! inline results, CLI/headless output, support exports, and release evidence
//! when a test lane claims stable support. It composes durable discovery item
//! identities, append-only session and attempt history, watch-state lineage,
//! governed mute/quarantine records, triage packets, and imported-CI parity
//! summaries into one export-safe contract.
//!
//! The boundary schema is
//! [`/schemas/runtime/canonical-test-discovery-session-and-watch-truth.schema.json`](../../../../schemas/runtime/canonical-test-discovery-session-and-watch-truth.schema.json).
//! The reviewer contract is
//! [`/docs/runtime/m4/canonical-test-discovery-session-and-watch-truth.md`](../../../../docs/runtime/m4/canonical-test-discovery-session-and-watch-truth.md).

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version emitted by canonical test-truth packets.
pub const CANONICAL_TEST_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`CanonicalTestTruthPacket`].
pub const CANONICAL_TEST_TRUTH_PACKET_RECORD_KIND: &str =
    "canonical_test_discovery_session_and_watch_truth_packet";

/// Repo-relative path of the boundary schema.
pub const CANONICAL_TEST_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/canonical-test-discovery-session-and-watch-truth.schema.json";

/// Repo-relative path of the checked-in proof artifact.
pub const CANONICAL_TEST_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/canonical_test_discovery_session_and_watch_truth_packet.json";

/// Repo-relative path of the protected fixture corpus.
pub const CANONICAL_TEST_TRUTH_FIXTURE_DIR: &str =
    "fixtures/testing/m4/canonical-test-discovery-session-and-watch-truth";

/// Canonical discovery item class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestDiscoveryItemClass {
    /// Suite or container row with stable child-count truth.
    SuiteContainer,
    /// Runnable concrete test case.
    ConcreteCase,
    /// Parameterized template before invocation expansion.
    ParameterizedTemplate,
    /// Concrete invocation of a parameterized template.
    ParameterizedInvocation,
    /// Notebook cell, interactive object, or artifact-backed test row.
    NotebookInteractive,
    /// Partial-discovery record documenting omitted scope.
    PartialDiscovery,
}

impl TestDiscoveryItemClass {
    /// Every discovery item class required by a stable packet.
    pub const REQUIRED: [Self; 6] = [
        Self::SuiteContainer,
        Self::ConcreteCase,
        Self::ParameterizedTemplate,
        Self::ParameterizedInvocation,
        Self::NotebookInteractive,
        Self::PartialDiscovery,
    ];

    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuiteContainer => "suite_container",
            Self::ConcreteCase => "concrete_case",
            Self::ParameterizedTemplate => "parameterized_template",
            Self::ParameterizedInvocation => "parameterized_invocation",
            Self::NotebookInteractive => "notebook_interactive",
            Self::PartialDiscovery => "partial_discovery",
        }
    }
}

/// Source used to derive a durable test identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurableIdentitySourceClass {
    /// Adapter-emitted stable item id.
    AdapterStableId,
    /// Framework-native node id or selector id.
    FrameworkNodeId,
    /// Notebook cell/object id plus kernel or artifact class.
    NotebookCellObjectId,
    /// Snapshot-scoped imported provider id.
    ImportedProviderStableId,
    /// Display label only; stable packets must reject this source.
    DisplayLabelOnly,
}

impl DurableIdentitySourceClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterStableId => "adapter_stable_id",
            Self::FrameworkNodeId => "framework_node_id",
            Self::NotebookCellObjectId => "notebook_cell_object_id",
            Self::ImportedProviderStableId => "imported_provider_stable_id",
            Self::DisplayLabelOnly => "display_label_only",
        }
    }
}

/// Support class for discovery, watch, or execution on a target family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSupportClass {
    /// Live local/current truth is supported.
    Live,
    /// Supported with declared reduction.
    Reduced,
    /// Supported through polling rather than native watch.
    Polling,
    /// Unsupported or unavailable for the target family.
    Unavailable,
}

impl TestSupportClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reduced => "reduced",
            Self::Polling => "polling",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Reason a discovery record omitted scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OmittedScopeReason {
    /// The adapter expands cases only at runtime.
    ExpandsAtRuntime,
    /// Target or helper capability is unavailable.
    TargetUnavailable,
    /// Notebook kernel or artifact is inspect-only.
    NotebookRunDisabled,
    /// Provider artifact is partial or stale.
    ImportedArtifactPartial,
    /// Policy blocks discovery of the scope.
    PolicyBlocked,
}

impl OmittedScopeReason {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandsAtRuntime => "expands_at_runtime",
            Self::TargetUnavailable => "target_unavailable",
            Self::NotebookRunDisabled => "notebook_run_disabled",
            Self::ImportedArtifactPartial => "imported_artifact_partial",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Test session mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSessionMode {
    /// Run a selected scope once.
    RunSelected,
    /// Watch a selected scope as a session series.
    WatchSeries,
    /// Rerun failed scope from a prior attempt.
    RerunFailed,
    /// Debug from a test attempt.
    DebugFromTest,
    /// Import provider CI results as read-only evidence.
    ImportedCiReadOnly,
}

impl TestSessionMode {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunSelected => "run_selected",
            Self::WatchSeries => "watch_series",
            Self::RerunFailed => "rerun_failed",
            Self::DebugFromTest => "debug_from_test",
            Self::ImportedCiReadOnly => "imported_ci_read_only",
        }
    }
}

/// Attempt result class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcomeClass {
    /// Attempt passed.
    Passed,
    /// Attempt failed.
    Failed,
    /// Attempt was blocked before execution.
    Blocked,
    /// Provider-imported attempt failed.
    ImportedFailed,
    /// Outcome is unknown and must stay visible.
    Unknown,
}

impl AttemptOutcomeClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Blocked => "blocked",
            Self::ImportedFailed => "imported_failed",
            Self::Unknown => "unknown",
        }
    }
}

/// Watch truth class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchStateClass {
    /// Live watch can claim current truth.
    Live,
    /// Watch is reduced by partial discovery, batching, or scope.
    Reduced,
    /// Watch is backed by polling.
    Polling,
    /// Watch is unavailable.
    Unavailable,
}

impl WatchStateClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reduced => "reduced",
            Self::Polling => "polling",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Stability verdict class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityVerdictClass {
    /// Stable through the evidence window.
    Stable,
    /// Flaky behavior is suspected.
    SuspectedFlaky,
    /// Flaky behavior was reproduced comparably.
    ReproducedFlaky,
    /// Muted by policy or owner action.
    Muted,
    /// Unknown or stale verdict.
    Unknown,
}

impl StabilityVerdictClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::Muted => "muted",
            Self::Unknown => "unknown",
        }
    }
}

/// Governance action class for hidden or deprioritized test scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestGovernanceActionClass {
    /// Quarantine blocks stable readiness unless visible and current.
    Quarantine,
    /// Mute changes notification/noise behavior without hiding failing scope.
    Mute,
}

impl TestGovernanceActionClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Quarantine => "quarantine",
            Self::Mute => "mute",
        }
    }
}

/// Release visibility for a mute or quarantine record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseVisibilityClass {
    /// Record is release-visible and counted separately.
    ReleaseVisible,
    /// Expired quarantine is release-visible and blocks readiness.
    ReleaseVisibleBlocksReadiness,
    /// Record is support-only; stable packets reject this for quarantines.
    SupportOnly,
}

impl ReleaseVisibilityClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseVisible => "release_visible",
            Self::ReleaseVisibleBlocksReadiness => "release_visible_blocks_readiness",
            Self::SupportOnly => "support_only",
        }
    }
}

/// Imported-CI parity state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedCiParityClass {
    /// No imported CI evidence participates.
    NotImported,
    /// Imported evidence is provider-authoritative but read-only locally.
    ReadOnlyLinked,
    /// Imported evidence has a linked local rerun plan.
    LinkedToLocalRerunPlan,
    /// Fresh local evidence confirms imported evidence without overwriting it.
    FreshLocalParity,
    /// Imported evidence is stale or partial.
    StaleOrPartial,
}

impl ImportedCiParityClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotImported => "not_imported",
            Self::ReadOnlyLinked => "read_only_linked",
            Self::LinkedToLocalRerunPlan => "linked_to_local_rerun_plan",
            Self::FreshLocalParity => "fresh_local_parity",
            Self::StaleOrPartial => "stale_or_partial",
        }
    }
}

/// Durable discovery record for one test row, container, invocation, or omission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalDiscoveryRecord {
    /// Stable discovery record id.
    pub discovery_record_id: String,
    /// Discovery item class.
    pub item_class: TestDiscoveryItemClass,
    /// Stable item-class token.
    pub item_class_token: String,
    /// Export-safe durable item id.
    pub durable_test_id: String,
    /// Source used to derive the durable id.
    pub durable_identity_source_class: DurableIdentitySourceClass,
    /// Stable durable-id source token.
    pub durable_identity_source_token: String,
    /// Source anchor ref such as file digest, notebook cell ref, or provider ref.
    pub source_anchor_ref: String,
    /// Target-family support class.
    pub support_class: TestSupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Omitted-scope reasons, required for partial-discovery rows.
    pub omitted_scope_reasons: Vec<OmittedScopeReason>,
    /// Stable omitted-scope reason tokens.
    pub omitted_scope_reason_tokens: Vec<String>,
    /// True when the record can cross support/release boundaries.
    pub export_safe_identity: bool,
}

impl CanonicalDiscoveryRecord {
    /// Builds a discovery record with derived token fields.
    pub fn new(
        discovery_record_id: impl Into<String>,
        item_class: TestDiscoveryItemClass,
        durable_test_id: impl Into<String>,
        durable_identity_source_class: DurableIdentitySourceClass,
        source_anchor_ref: impl Into<String>,
        support_class: TestSupportClass,
        omitted_scope_reasons: Vec<OmittedScopeReason>,
    ) -> Self {
        Self {
            discovery_record_id: discovery_record_id.into(),
            item_class,
            item_class_token: item_class.as_str().to_owned(),
            durable_test_id: durable_test_id.into(),
            durable_identity_source_class,
            durable_identity_source_token: durable_identity_source_class.as_str().to_owned(),
            source_anchor_ref: source_anchor_ref.into(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            omitted_scope_reason_tokens: omitted_scope_reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect(),
            omitted_scope_reasons,
            export_safe_identity: durable_identity_source_class
                != DurableIdentitySourceClass::DisplayLabelOnly,
        }
    }
}

/// Session plan with exact selection, target, and environment lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionPlanRecord {
    /// Stable session plan id.
    pub session_plan_id: String,
    /// Stable test session id.
    pub test_session_id: String,
    /// Session mode.
    pub session_mode: TestSessionMode,
    /// Stable session-mode token.
    pub session_mode_token: String,
    /// Discovery snapshot ref used by the plan.
    pub discovery_snapshot_ref: String,
    /// Selected durable test ids.
    pub selected_durable_test_ids: Vec<String>,
    /// Target ref used at plan time.
    pub target_ref: String,
    /// Environment or capsule ref used at plan time.
    pub environment_ref: String,
    /// Retry policy token.
    pub retry_policy_ref: String,
    /// Watch policy token.
    pub watch_policy_ref: String,
    /// Imported-CI session ref when this plan was derived from provider evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_ci_session_ref: Option<String>,
    /// True when selection widening requires review.
    pub widening_review_required: bool,
}

impl SessionPlanRecord {
    /// Builds a session plan with derived token fields.
    pub fn new(
        session_plan_id: impl Into<String>,
        test_session_id: impl Into<String>,
        session_mode: TestSessionMode,
        discovery_snapshot_ref: impl Into<String>,
        selected_durable_test_ids: Vec<String>,
        target_ref: impl Into<String>,
        environment_ref: impl Into<String>,
        retry_policy_ref: impl Into<String>,
        watch_policy_ref: impl Into<String>,
    ) -> Self {
        Self {
            session_plan_id: session_plan_id.into(),
            test_session_id: test_session_id.into(),
            session_mode,
            session_mode_token: session_mode.as_str().to_owned(),
            discovery_snapshot_ref: discovery_snapshot_ref.into(),
            selected_durable_test_ids,
            target_ref: target_ref.into(),
            environment_ref: environment_ref.into(),
            retry_policy_ref: retry_policy_ref.into(),
            watch_policy_ref: watch_policy_ref.into(),
            imported_ci_session_ref: None,
            widening_review_required: false,
        }
    }
}

/// Append-only attempt record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AttemptRecord {
    /// Stable attempt id.
    pub attempt_id: String,
    /// Parent session id.
    pub test_session_ref: String,
    /// One-based append-only attempt index.
    pub attempt_index: u32,
    /// Outcome class.
    pub outcome_class: AttemptOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// Target ref used by this attempt.
    pub target_ref: String,
    /// Environment or capsule ref used by this attempt.
    pub environment_ref: String,
    /// Runtime or build lineage refs.
    pub runtime_build_refs: Vec<String>,
    /// Artifact refs retained outside raw runner text.
    pub artifact_refs: Vec<String>,
    /// Previous attempt ref when this attempt derives from one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_attempt_ref: Option<String>,
    /// Imported attempt ref when this attempt compares provider evidence.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_attempt_ref: Option<String>,
    /// Durable test ids covered by this attempt.
    pub covered_durable_test_ids: Vec<String>,
}

impl AttemptRecord {
    /// Builds an append-only attempt record with derived token fields.
    pub fn new(
        attempt_id: impl Into<String>,
        test_session_ref: impl Into<String>,
        attempt_index: u32,
        outcome_class: AttemptOutcomeClass,
        target_ref: impl Into<String>,
        environment_ref: impl Into<String>,
        covered_durable_test_ids: Vec<String>,
    ) -> Self {
        Self {
            attempt_id: attempt_id.into(),
            test_session_ref: test_session_ref.into(),
            attempt_index,
            outcome_class,
            outcome_token: outcome_class.as_str().to_owned(),
            target_ref: target_ref.into(),
            environment_ref: environment_ref.into(),
            runtime_build_refs: Vec::new(),
            artifact_refs: Vec::new(),
            predecessor_attempt_ref: None,
            imported_attempt_ref: None,
            covered_durable_test_ids,
        }
    }
}

/// Stability verdict with evidence lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityVerdict {
    /// Stable verdict id.
    pub stability_verdict_id: String,
    /// Verdict class.
    pub verdict_class: StabilityVerdictClass,
    /// Stable verdict token.
    pub verdict_token: String,
    /// Attempt refs backing the verdict.
    pub evidence_attempt_refs: Vec<String>,
    /// Evidence window ref.
    pub evidence_window_ref: String,
    /// Confidence token.
    pub confidence_ref: String,
    /// Manual override note ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manual_override_ref: Option<String>,
}

impl StabilityVerdict {
    /// Builds a stability verdict with derived token fields.
    pub fn new(
        stability_verdict_id: impl Into<String>,
        verdict_class: StabilityVerdictClass,
        evidence_attempt_refs: Vec<String>,
        evidence_window_ref: impl Into<String>,
        confidence_ref: impl Into<String>,
    ) -> Self {
        Self {
            stability_verdict_id: stability_verdict_id.into(),
            verdict_class,
            verdict_token: verdict_class.as_str().to_owned(),
            evidence_attempt_refs,
            evidence_window_ref: evidence_window_ref.into(),
            confidence_ref: confidence_ref.into(),
            manual_override_ref: None,
        }
    }
}

/// Governed mute or quarantine record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantineRecord {
    /// Stable governance record id.
    pub quarantine_record_id: String,
    /// Governance action.
    pub action_class: TestGovernanceActionClass,
    /// Stable action token.
    pub action_token: String,
    /// Durable scope refs covered by the record.
    pub scope_refs: Vec<String>,
    /// Reason ref or controlled reason token.
    pub reason_ref: String,
    /// Owning team or person ref.
    pub owner_ref: String,
    /// Expiry timestamp or date.
    pub expires_at: String,
    /// True when the record is already expired for the packet's date basis.
    pub expired: bool,
    /// Release visibility class.
    pub release_visibility_class: ReleaseVisibilityClass,
    /// Stable release-visibility token.
    pub release_visibility_token: String,
    /// Evidence refs justifying the governance action.
    pub evidence_refs: Vec<String>,
}

impl QuarantineRecord {
    /// Builds a governed mute or quarantine record with derived token fields.
    pub fn new(
        quarantine_record_id: impl Into<String>,
        action_class: TestGovernanceActionClass,
        scope_refs: Vec<String>,
        reason_ref: impl Into<String>,
        owner_ref: impl Into<String>,
        expires_at: impl Into<String>,
        release_visibility_class: ReleaseVisibilityClass,
        evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            quarantine_record_id: quarantine_record_id.into(),
            action_class,
            action_token: action_class.as_str().to_owned(),
            scope_refs,
            reason_ref: reason_ref.into(),
            owner_ref: owner_ref.into(),
            expires_at: expires_at.into(),
            expired: false,
            release_visibility_class,
            release_visibility_token: release_visibility_class.as_str().to_owned(),
            evidence_refs,
        }
    }
}

/// Failure triage packet for a visible failing or governed test scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriagePacket {
    /// Stable triage packet id.
    pub triage_packet_id: String,
    /// Durable scope refs covered by the triage packet.
    pub scope_refs: Vec<String>,
    /// Attempt refs covered by the triage packet.
    pub attempt_refs: Vec<String>,
    /// Diff or assertion summary ref.
    pub assertion_summary_ref: String,
    /// Environment drift note ref.
    pub environment_drift_ref: String,
    /// Reproduce action ref.
    pub reproduce_action_ref: String,
    /// Debug action ref.
    pub debug_action_ref: String,
    /// Governance review ref required before mute/quarantine changes.
    pub governance_review_ref: String,
}

/// Watch-state object that groups cycles by session instead of mutating one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchStateRecord {
    /// Stable watch-state id.
    pub watch_state_id: String,
    /// Parent session id.
    pub test_session_ref: String,
    /// Session series ref grouping watch attempts.
    pub session_series_ref: String,
    /// Watch state class.
    pub watch_state_class: WatchStateClass,
    /// Stable watch-state token.
    pub watch_state_token: String,
    /// Target-family support class.
    pub support_class: TestSupportClass,
    /// Stable support-class token.
    pub support_class_token: String,
    /// Attempt refs in append-only watch order.
    pub grouped_attempt_refs: Vec<String>,
    /// Last successful attempt ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_attempt_ref: Option<String>,
    /// Backlog count.
    pub backlog_count: u32,
    /// Degradation reason refs.
    pub degradation_reason_refs: Vec<String>,
    /// True when the watch record may claim current local truth.
    pub claims_current_truth: bool,
}

impl WatchStateRecord {
    /// Builds a watch-state record with derived token fields.
    pub fn new(
        watch_state_id: impl Into<String>,
        test_session_ref: impl Into<String>,
        session_series_ref: impl Into<String>,
        watch_state_class: WatchStateClass,
        support_class: TestSupportClass,
        grouped_attempt_refs: Vec<String>,
    ) -> Self {
        Self {
            watch_state_id: watch_state_id.into(),
            test_session_ref: test_session_ref.into(),
            session_series_ref: session_series_ref.into(),
            watch_state_class,
            watch_state_token: watch_state_class.as_str().to_owned(),
            support_class,
            support_class_token: support_class.as_str().to_owned(),
            grouped_attempt_refs,
            last_successful_attempt_ref: None,
            backlog_count: 0,
            degradation_reason_refs: Vec::new(),
            claims_current_truth: matches!(watch_state_class, WatchStateClass::Live),
        }
    }
}

/// Imported-CI parity summary linked to local rerun plans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedCiParitySummary {
    /// Stable imported-CI parity id.
    pub imported_ci_parity_id: String,
    /// Imported-CI parity class.
    pub parity_class: ImportedCiParityClass,
    /// Stable parity token.
    pub parity_token: String,
    /// Imported provider session refs.
    pub provider_session_refs: Vec<String>,
    /// Current discovery snapshot ref used to compare imported evidence.
    pub current_discovery_snapshot_ref: String,
    /// Local rerun plan refs that can refresh or compare evidence.
    pub local_rerun_plan_refs: Vec<String>,
    /// Imported attempt refs.
    pub imported_attempt_refs: Vec<String>,
    /// Local parity attempt refs.
    pub local_parity_attempt_refs: Vec<String>,
    /// True when imported evidence remains read-only.
    pub read_only_imported_evidence: bool,
    /// True when the row claims current local truth.
    pub claims_current_local_truth: bool,
}

impl ImportedCiParitySummary {
    /// Builds an imported-CI parity summary with derived token fields.
    pub fn new(
        imported_ci_parity_id: impl Into<String>,
        parity_class: ImportedCiParityClass,
        current_discovery_snapshot_ref: impl Into<String>,
    ) -> Self {
        Self {
            imported_ci_parity_id: imported_ci_parity_id.into(),
            parity_class,
            parity_token: parity_class.as_str().to_owned(),
            provider_session_refs: Vec::new(),
            current_discovery_snapshot_ref: current_discovery_snapshot_ref.into(),
            local_rerun_plan_refs: Vec::new(),
            imported_attempt_refs: Vec::new(),
            local_parity_attempt_refs: Vec::new(),
            read_only_imported_evidence: !matches!(
                parity_class,
                ImportedCiParityClass::NotImported
            ),
            claims_current_local_truth: matches!(
                parity_class,
                ImportedCiParityClass::FreshLocalParity
            ),
        }
    }
}

/// Count summary consumed by release and support packets.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTruthStateCounts {
    /// Live watch/support row count.
    pub live_count: u32,
    /// Reduced watch/support row count.
    pub reduced_count: u32,
    /// Polling watch/support row count.
    pub polling_count: u32,
    /// Unavailable watch/support row count.
    pub unavailable_count: u32,
    /// Quarantined scope count.
    pub quarantined_count: u32,
    /// Muted scope count.
    pub muted_count: u32,
    /// Imported-CI row count.
    pub imported_count: u32,
}

/// Stable canonical test-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalTestTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Packet generation timestamp or date.
    pub generated_at: String,
    /// Release proof index row refs that ingest this packet.
    pub stable_proof_index_refs: Vec<String>,
    /// Discovery snapshot ref.
    pub discovery_snapshot_ref: String,
    /// Durable discovery records.
    pub discovery_records: Vec<CanonicalDiscoveryRecord>,
    /// Session plans.
    pub session_plans: Vec<SessionPlanRecord>,
    /// Append-only attempts.
    pub attempts: Vec<AttemptRecord>,
    /// Stability verdicts.
    pub stability_verdicts: Vec<StabilityVerdict>,
    /// Governed quarantine and mute records.
    pub quarantine_records: Vec<QuarantineRecord>,
    /// Triage packets.
    pub triage_packets: Vec<TriagePacket>,
    /// Watch-state records.
    pub watch_states: Vec<WatchStateRecord>,
    /// Imported-CI parity summaries.
    pub imported_ci_parity: Vec<ImportedCiParitySummary>,
    /// Release and support count summary.
    pub state_counts: TestTruthStateCounts,
    /// Support export refs that ingest the packet.
    pub support_export_refs: Vec<String>,
}

impl CanonicalTestTruthPacket {
    /// Builds a packet with derived state counts.
    pub fn new(
        packet_id: impl Into<String>,
        generated_at: impl Into<String>,
        discovery_snapshot_ref: impl Into<String>,
        discovery_records: Vec<CanonicalDiscoveryRecord>,
        session_plans: Vec<SessionPlanRecord>,
        attempts: Vec<AttemptRecord>,
        stability_verdicts: Vec<StabilityVerdict>,
        quarantine_records: Vec<QuarantineRecord>,
        triage_packets: Vec<TriagePacket>,
        watch_states: Vec<WatchStateRecord>,
        imported_ci_parity: Vec<ImportedCiParitySummary>,
    ) -> Self {
        let mut packet = Self {
            record_kind: CANONICAL_TEST_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: CANONICAL_TEST_TRUTH_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            generated_at: generated_at.into(),
            stable_proof_index_refs: Vec::new(),
            discovery_snapshot_ref: discovery_snapshot_ref.into(),
            discovery_records,
            session_plans,
            attempts,
            stability_verdicts,
            quarantine_records,
            triage_packets,
            watch_states,
            imported_ci_parity,
            state_counts: TestTruthStateCounts::default(),
            support_export_refs: Vec::new(),
        };
        packet.state_counts = packet.derive_state_counts();
        packet
    }

    /// Validates the packet against stable test-lane guardrails.
    pub fn validate(&self) -> Result<(), CanonicalTestTruthValidationError> {
        if self.record_kind != CANONICAL_TEST_TRUTH_PACKET_RECORD_KIND {
            return Err(CanonicalTestTruthValidationError::WrongRecordKind);
        }
        if self.schema_version != CANONICAL_TEST_TRUTH_SCHEMA_VERSION {
            return Err(
                CanonicalTestTruthValidationError::UnsupportedSchemaVersion {
                    schema_version: self.schema_version,
                },
            );
        }
        if self.stable_proof_index_refs.is_empty() {
            return Err(CanonicalTestTruthValidationError::MissingStableProofIndexRef);
        }
        self.validate_discovery_records()?;
        self.validate_sessions_and_attempts()?;
        self.validate_watch_states()?;
        self.validate_quarantine_governance()?;
        self.validate_imported_ci_parity()?;
        self.validate_state_counts()?;
        Ok(())
    }

    fn validate_discovery_records(&self) -> Result<(), CanonicalTestTruthValidationError> {
        let classes = self
            .discovery_records
            .iter()
            .map(|record| record.item_class)
            .collect::<BTreeSet<_>>();
        for required in TestDiscoveryItemClass::REQUIRED {
            if !classes.contains(&required) {
                return Err(CanonicalTestTruthValidationError::MissingDiscoveryClass {
                    item_class: required.as_str(),
                });
            }
        }
        for record in &self.discovery_records {
            if record.durable_identity_source_class == DurableIdentitySourceClass::DisplayLabelOnly
                || !record.export_safe_identity
            {
                return Err(
                    CanonicalTestTruthValidationError::DisplayLabelIdentityDenied {
                        discovery_record_id: record.discovery_record_id.clone(),
                    },
                );
            }
            if record.item_class == TestDiscoveryItemClass::PartialDiscovery
                && record.omitted_scope_reasons.is_empty()
            {
                return Err(
                    CanonicalTestTruthValidationError::PartialDiscoveryMissingOmittedScope {
                        discovery_record_id: record.discovery_record_id.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_sessions_and_attempts(&self) -> Result<(), CanonicalTestTruthValidationError> {
        let selected_ids = self
            .session_plans
            .iter()
            .flat_map(|plan| plan.selected_durable_test_ids.iter().cloned())
            .collect::<BTreeSet<_>>();
        let discovered_ids = self
            .discovery_records
            .iter()
            .map(|record| record.durable_test_id.clone())
            .collect::<BTreeSet<_>>();
        for selected in selected_ids {
            if !discovered_ids.contains(&selected) {
                return Err(
                    CanonicalTestTruthValidationError::SessionSelectsUnknownDiscovery {
                        durable_test_id: selected,
                    },
                );
            }
        }

        let mut attempts_by_session: BTreeMap<&str, Vec<&AttemptRecord>> = BTreeMap::new();
        for attempt in &self.attempts {
            attempts_by_session
                .entry(&attempt.test_session_ref)
                .or_default()
                .push(attempt);
        }
        for attempts in attempts_by_session.values_mut() {
            attempts.sort_by_key(|attempt| attempt.attempt_index);
            for (offset, attempt) in attempts.iter().enumerate() {
                let expected = offset as u32 + 1;
                if attempt.attempt_index != expected {
                    return Err(
                        CanonicalTestTruthValidationError::NonAppendOnlyAttemptLedger {
                            test_session_ref: attempt.test_session_ref.clone(),
                            expected_index: expected,
                            actual_index: attempt.attempt_index,
                        },
                    );
                }
            }
        }
        Ok(())
    }

    fn validate_watch_states(&self) -> Result<(), CanonicalTestTruthValidationError> {
        let attempt_refs = self
            .attempts
            .iter()
            .map(|attempt| attempt.attempt_id.as_str())
            .collect::<BTreeSet<_>>();
        for watch_state in &self.watch_states {
            if watch_state.session_series_ref.is_empty() {
                return Err(
                    CanonicalTestTruthValidationError::WatchMissingSessionSeries {
                        watch_state_id: watch_state.watch_state_id.clone(),
                    },
                );
            }
            if watch_state.watch_state_class != WatchStateClass::Unavailable
                && watch_state.grouped_attempt_refs.is_empty()
            {
                return Err(
                    CanonicalTestTruthValidationError::WatchMissingGroupedAttempts {
                        watch_state_id: watch_state.watch_state_id.clone(),
                    },
                );
            }
            for attempt_ref in &watch_state.grouped_attempt_refs {
                if !attempt_refs.contains(attempt_ref.as_str()) {
                    return Err(
                        CanonicalTestTruthValidationError::WatchReferencesUnknownAttempt {
                            watch_state_id: watch_state.watch_state_id.clone(),
                            attempt_ref: attempt_ref.clone(),
                        },
                    );
                }
            }
            if watch_state.claims_current_truth
                && watch_state.watch_state_class != WatchStateClass::Live
            {
                return Err(
                    CanonicalTestTruthValidationError::DegradedWatchClaimsCurrentTruth {
                        watch_state_id: watch_state.watch_state_id.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_quarantine_governance(&self) -> Result<(), CanonicalTestTruthValidationError> {
        for record in &self.quarantine_records {
            if record.owner_ref.is_empty()
                || record.reason_ref.is_empty()
                || record.expires_at.is_empty()
            {
                return Err(
                    CanonicalTestTruthValidationError::QuarantineMissingGovernance {
                        quarantine_record_id: record.quarantine_record_id.clone(),
                    },
                );
            }
            if record.action_class == TestGovernanceActionClass::Quarantine
                && record.release_visibility_class == ReleaseVisibilityClass::SupportOnly
            {
                return Err(
                    CanonicalTestTruthValidationError::QuarantineHiddenFromRelease {
                        quarantine_record_id: record.quarantine_record_id.clone(),
                    },
                );
            }
            if record.action_class == TestGovernanceActionClass::Quarantine
                && record.expired
                && record.release_visibility_class
                    != ReleaseVisibilityClass::ReleaseVisibleBlocksReadiness
            {
                return Err(
                    CanonicalTestTruthValidationError::ExpiredQuarantineDoesNotBlockReadiness {
                        quarantine_record_id: record.quarantine_record_id.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_imported_ci_parity(&self) -> Result<(), CanonicalTestTruthValidationError> {
        for parity in &self.imported_ci_parity {
            if parity.parity_class != ImportedCiParityClass::NotImported {
                if !parity.read_only_imported_evidence {
                    return Err(CanonicalTestTruthValidationError::ImportedCiNotReadOnly {
                        imported_ci_parity_id: parity.imported_ci_parity_id.clone(),
                    });
                }
                if parity.provider_session_refs.is_empty()
                    || parity.current_discovery_snapshot_ref.is_empty()
                    || parity.local_rerun_plan_refs.is_empty()
                {
                    return Err(
                        CanonicalTestTruthValidationError::ImportedCiMissingParityLinkage {
                            imported_ci_parity_id: parity.imported_ci_parity_id.clone(),
                        },
                    );
                }
                if parity.claims_current_local_truth
                    && parity.parity_class != ImportedCiParityClass::FreshLocalParity
                {
                    return Err(
                        CanonicalTestTruthValidationError::ImportedCiMasqueradesAsLocalRun {
                            imported_ci_parity_id: parity.imported_ci_parity_id.clone(),
                        },
                    );
                }
            }
        }
        Ok(())
    }

    fn validate_state_counts(&self) -> Result<(), CanonicalTestTruthValidationError> {
        let expected = self.derive_state_counts();
        if self.state_counts != expected {
            return Err(CanonicalTestTruthValidationError::StateCountsMismatch {
                expected,
                actual: self.state_counts.clone(),
            });
        }
        Ok(())
    }

    fn derive_state_counts(&self) -> TestTruthStateCounts {
        let mut counts = TestTruthStateCounts::default();
        for watch_state in &self.watch_states {
            match watch_state.support_class {
                TestSupportClass::Live => counts.live_count += 1,
                TestSupportClass::Reduced => counts.reduced_count += 1,
                TestSupportClass::Polling => counts.polling_count += 1,
                TestSupportClass::Unavailable => counts.unavailable_count += 1,
            }
        }
        for record in &self.quarantine_records {
            match record.action_class {
                TestGovernanceActionClass::Quarantine => counts.quarantined_count += 1,
                TestGovernanceActionClass::Mute => counts.muted_count += 1,
            }
        }
        counts.imported_count = self
            .imported_ci_parity
            .iter()
            .filter(|parity| parity.parity_class != ImportedCiParityClass::NotImported)
            .count() as u32;
        counts
    }
}

/// Validation error for canonical test-truth packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CanonicalTestTruthValidationError {
    /// Record kind does not match the canonical packet contract.
    WrongRecordKind,
    /// Schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Unsupported schema version.
        schema_version: u32,
    },
    /// Packet is not wired to release proof index evidence.
    MissingStableProofIndexRef,
    /// Required discovery class is missing.
    MissingDiscoveryClass {
        /// Missing discovery item class.
        item_class: &'static str,
    },
    /// Discovery identity is based only on display text.
    DisplayLabelIdentityDenied {
        /// Discovery record id.
        discovery_record_id: String,
    },
    /// Partial-discovery row has no omitted-scope reason.
    PartialDiscoveryMissingOmittedScope {
        /// Discovery record id.
        discovery_record_id: String,
    },
    /// Session selects a durable id not in the discovery snapshot.
    SessionSelectsUnknownDiscovery {
        /// Unknown durable test id.
        durable_test_id: String,
    },
    /// Attempt ledger is not append-only for a session.
    NonAppendOnlyAttemptLedger {
        /// Test session ref.
        test_session_ref: String,
        /// Expected one-based attempt index.
        expected_index: u32,
        /// Actual attempt index.
        actual_index: u32,
    },
    /// Watch state is missing its session series ref.
    WatchMissingSessionSeries {
        /// Watch-state id.
        watch_state_id: String,
    },
    /// Watch state has no grouped attempts.
    WatchMissingGroupedAttempts {
        /// Watch-state id.
        watch_state_id: String,
    },
    /// Watch state references an unknown attempt.
    WatchReferencesUnknownAttempt {
        /// Watch-state id.
        watch_state_id: String,
        /// Unknown attempt ref.
        attempt_ref: String,
    },
    /// Degraded watch state claims current truth.
    DegradedWatchClaimsCurrentTruth {
        /// Watch-state id.
        watch_state_id: String,
    },
    /// Mute or quarantine record is missing owner, reason, or expiry.
    QuarantineMissingGovernance {
        /// Governance record id.
        quarantine_record_id: String,
    },
    /// Quarantine is hidden from release evidence.
    QuarantineHiddenFromRelease {
        /// Governance record id.
        quarantine_record_id: String,
    },
    /// Expired quarantine does not block stable readiness.
    ExpiredQuarantineDoesNotBlockReadiness {
        /// Governance record id.
        quarantine_record_id: String,
    },
    /// Imported-CI evidence is mutable locally.
    ImportedCiNotReadOnly {
        /// Imported-CI parity id.
        imported_ci_parity_id: String,
    },
    /// Imported-CI evidence lacks provider, discovery, or rerun linkage.
    ImportedCiMissingParityLinkage {
        /// Imported-CI parity id.
        imported_ci_parity_id: String,
    },
    /// Imported-CI evidence claims current local truth without local parity.
    ImportedCiMasqueradesAsLocalRun {
        /// Imported-CI parity id.
        imported_ci_parity_id: String,
    },
    /// State counts do not match packet rows.
    StateCountsMismatch {
        /// Expected counts.
        expected: TestTruthStateCounts,
        /// Actual counts.
        actual: TestTruthStateCounts,
    },
}

impl fmt::Display for CanonicalTestTruthValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind => f.write_str("canonical test-truth packet has the wrong record kind"),
            Self::UnsupportedSchemaVersion { schema_version } => {
                write!(f, "unsupported canonical test-truth schema version {schema_version}")
            }
            Self::MissingStableProofIndexRef => {
                f.write_str("canonical test-truth packet is missing stable proof index refs")
            }
            Self::MissingDiscoveryClass { item_class } => {
                write!(f, "canonical test-truth packet is missing discovery class {item_class}")
            }
            Self::DisplayLabelIdentityDenied { discovery_record_id } => {
                write!(f, "discovery record {discovery_record_id} uses display-label identity")
            }
            Self::PartialDiscoveryMissingOmittedScope { discovery_record_id } => {
                write!(f, "partial discovery record {discovery_record_id} is missing omitted-scope reasons")
            }
            Self::SessionSelectsUnknownDiscovery { durable_test_id } => {
                write!(f, "session selects unknown durable test id {durable_test_id}")
            }
            Self::NonAppendOnlyAttemptLedger {
                test_session_ref,
                expected_index,
                actual_index,
            } => write!(
                f,
                "attempt ledger for {test_session_ref} expected index {expected_index} but found {actual_index}"
            ),
            Self::WatchMissingSessionSeries { watch_state_id } => {
                write!(f, "watch state {watch_state_id} is missing a session series ref")
            }
            Self::WatchMissingGroupedAttempts { watch_state_id } => {
                write!(f, "watch state {watch_state_id} has no grouped attempts")
            }
            Self::WatchReferencesUnknownAttempt {
                watch_state_id,
                attempt_ref,
            } => write!(f, "watch state {watch_state_id} references unknown attempt {attempt_ref}"),
            Self::DegradedWatchClaimsCurrentTruth { watch_state_id } => {
                write!(f, "degraded watch state {watch_state_id} claims current truth")
            }
            Self::QuarantineMissingGovernance {
                quarantine_record_id,
            } => write!(f, "governance record {quarantine_record_id} is missing owner, reason, or expiry"),
            Self::QuarantineHiddenFromRelease {
                quarantine_record_id,
            } => write!(f, "quarantine record {quarantine_record_id} is hidden from release evidence"),
            Self::ExpiredQuarantineDoesNotBlockReadiness {
                quarantine_record_id,
            } => write!(f, "expired quarantine record {quarantine_record_id} does not block readiness"),
            Self::ImportedCiNotReadOnly {
                imported_ci_parity_id,
            } => write!(f, "imported-CI parity {imported_ci_parity_id} is not read-only"),
            Self::ImportedCiMissingParityLinkage {
                imported_ci_parity_id,
            } => write!(f, "imported-CI parity {imported_ci_parity_id} is missing provider, discovery, or rerun linkage"),
            Self::ImportedCiMasqueradesAsLocalRun {
                imported_ci_parity_id,
            } => write!(f, "imported-CI parity {imported_ci_parity_id} masquerades as current local truth"),
            Self::StateCountsMismatch { .. } => {
                f.write_str("canonical test-truth state counts do not match packet rows")
            }
        }
    }
}

impl Error for CanonicalTestTruthValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stable_packet_validates_required_truth() {
        let packet = sample_packet();
        assert_eq!(packet.validate(), Ok(()));
    }

    #[test]
    fn display_label_identity_is_denied() {
        let mut packet = sample_packet();
        packet.discovery_records[1].durable_identity_source_class =
            DurableIdentitySourceClass::DisplayLabelOnly;
        packet.discovery_records[1].export_safe_identity = false;

        assert!(matches!(
            packet.validate(),
            Err(CanonicalTestTruthValidationError::DisplayLabelIdentityDenied { .. })
        ));
    }

    #[test]
    fn expired_quarantine_must_block_readiness() {
        let mut packet = sample_packet();
        packet.quarantine_records[0].expired = true;
        packet.quarantine_records[0].release_visibility_class =
            ReleaseVisibilityClass::ReleaseVisible;

        assert!(matches!(
            packet.validate(),
            Err(CanonicalTestTruthValidationError::ExpiredQuarantineDoesNotBlockReadiness { .. })
        ));
    }

    #[test]
    fn imported_ci_cannot_claim_local_truth_without_parity() {
        let mut packet = sample_packet();
        packet.imported_ci_parity[0].parity_class = ImportedCiParityClass::ReadOnlyLinked;
        packet.imported_ci_parity[0].claims_current_local_truth = true;

        assert!(matches!(
            packet.validate(),
            Err(CanonicalTestTruthValidationError::ImportedCiMasqueradesAsLocalRun { .. })
        ));
    }

    #[test]
    fn checked_in_packet_validates() {
        let packet: CanonicalTestTruthPacket = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/runtime/m4/canonical_test_discovery_session_and_watch_truth_packet.json"
        )))
        .expect("checked-in canonical test truth packet parses");

        assert_eq!(packet.validate(), Ok(()));
    }

    fn sample_packet() -> CanonicalTestTruthPacket {
        let discovery_records = vec![
            CanonicalDiscoveryRecord::new(
                "disc:suite:orders",
                TestDiscoveryItemClass::SuiteContainer,
                "test:suite:orders",
                DurableIdentitySourceClass::AdapterStableId,
                "src-anchor:digest:orders-tests",
                TestSupportClass::Live,
                Vec::new(),
            ),
            CanonicalDiscoveryRecord::new(
                "disc:case:orders-create",
                TestDiscoveryItemClass::ConcreteCase,
                "test:case:orders-create",
                DurableIdentitySourceClass::FrameworkNodeId,
                "src-anchor:digest:orders-create",
                TestSupportClass::Live,
                Vec::new(),
            ),
            CanonicalDiscoveryRecord::new(
                "disc:template:tax",
                TestDiscoveryItemClass::ParameterizedTemplate,
                "test:template:tax",
                DurableIdentitySourceClass::FrameworkNodeId,
                "src-anchor:digest:tax-template",
                TestSupportClass::Reduced,
                Vec::new(),
            ),
            CanonicalDiscoveryRecord::new(
                "disc:invocation:tax:vat",
                TestDiscoveryItemClass::ParameterizedInvocation,
                "test:invocation:tax:vat",
                DurableIdentitySourceClass::FrameworkNodeId,
                "src-anchor:digest:tax-vat",
                TestSupportClass::Reduced,
                Vec::new(),
            ),
            CanonicalDiscoveryRecord::new(
                "disc:notebook:forecast:cell-7",
                TestDiscoveryItemClass::NotebookInteractive,
                "test:notebook:forecast:cell-7",
                DurableIdentitySourceClass::NotebookCellObjectId,
                "notebook-cell:forecast:cell-7",
                TestSupportClass::Unavailable,
                vec![OmittedScopeReason::NotebookRunDisabled],
            ),
            CanonicalDiscoveryRecord::new(
                "disc:partial:provider-shard",
                TestDiscoveryItemClass::PartialDiscovery,
                "test:partial:provider-shard",
                DurableIdentitySourceClass::ImportedProviderStableId,
                "provider-ci:run:1842:shard-3",
                TestSupportClass::Polling,
                vec![OmittedScopeReason::ImportedArtifactPartial],
            ),
        ];
        let session = SessionPlanRecord::new(
            "session-plan:orders:watch",
            "test-session:orders:watch",
            TestSessionMode::WatchSeries,
            "discovery-snapshot:orders:2026-06-04",
            vec!["test:case:orders-create".to_owned()],
            "target:local:macos",
            "environment:uv:orders",
            "retry-policy:failed-only",
            "watch-policy:native-or-polling",
        );
        let attempts = vec![
            AttemptRecord::new(
                "attempt:orders:watch:1",
                "test-session:orders:watch",
                1,
                AttemptOutcomeClass::Failed,
                "target:local:macos",
                "environment:uv:orders",
                vec!["test:case:orders-create".to_owned()],
            ),
            AttemptRecord::new(
                "attempt:orders:watch:2",
                "test-session:orders:watch",
                2,
                AttemptOutcomeClass::Passed,
                "target:local:macos",
                "environment:uv:orders",
                vec!["test:case:orders-create".to_owned()],
            ),
        ];
        let verdict = StabilityVerdict::new(
            "verdict:orders-create",
            StabilityVerdictClass::SuspectedFlaky,
            vec![
                "attempt:orders:watch:1".to_owned(),
                "attempt:orders:watch:2".to_owned(),
            ],
            "evidence-window:orders:7d",
            "confidence:comparable-local",
        );
        let quarantine = QuarantineRecord::new(
            "quarantine:orders-create",
            TestGovernanceActionClass::Quarantine,
            vec!["test:case:orders-create".to_owned()],
            "reason:known-flaky-checkout-boundary",
            "owner:test-tooling",
            "2026-06-30",
            ReleaseVisibilityClass::ReleaseVisibleBlocksReadiness,
            vec!["attempt:orders:watch:1".to_owned()],
        );
        let triage = TriagePacket {
            triage_packet_id: "triage:orders-create".to_owned(),
            scope_refs: vec!["test:case:orders-create".to_owned()],
            attempt_refs: vec!["attempt:orders:watch:1".to_owned()],
            assertion_summary_ref: "assertion-summary:orders-create".to_owned(),
            environment_drift_ref: "env-drift:none".to_owned(),
            reproduce_action_ref: "action:rerun:orders-create".to_owned(),
            debug_action_ref: "action:debug:orders-create".to_owned(),
            governance_review_ref: "review:quarantine:orders-create".to_owned(),
        };
        let mut watch_state = WatchStateRecord::new(
            "watch:orders",
            "test-session:orders:watch",
            "watch-series:orders",
            WatchStateClass::Live,
            TestSupportClass::Live,
            vec![
                "attempt:orders:watch:1".to_owned(),
                "attempt:orders:watch:2".to_owned(),
            ],
        );
        watch_state.last_successful_attempt_ref = Some("attempt:orders:watch:2".to_owned());
        let mut imported = ImportedCiParitySummary::new(
            "imported-ci:orders:1842",
            ImportedCiParityClass::LinkedToLocalRerunPlan,
            "discovery-snapshot:orders:2026-06-04",
        );
        imported.provider_session_refs = vec!["provider-ci:run:1842".to_owned()];
        imported.local_rerun_plan_refs = vec!["session-plan:orders:watch".to_owned()];
        imported.imported_attempt_refs = vec!["provider-ci:attempt:1842:1".to_owned()];
        let mut packet = CanonicalTestTruthPacket::new(
            "canonical-test-truth:orders:2026-06-04",
            "2026-06-04T18:20:00Z",
            "discovery-snapshot:orders:2026-06-04",
            discovery_records,
            vec![session],
            attempts,
            vec![verdict],
            vec![quarantine],
            vec![triage],
            vec![watch_state],
            vec![imported],
        );
        packet.stable_proof_index_refs =
            vec!["stable-proof-index:runtime:test-discovery-session-watch-truth".to_owned()];
        packet.support_export_refs = vec!["support-export:test-truth:orders".to_owned()];
        packet
    }
}
