//! Promoted test identity, session, and attempt ledger records.
//!
//! This module projects the existing pytest discovery, alpha attempt packets,
//! and beta runner rows into one canonical identity ledger. The ledger is the
//! runtime contract that editor inline markers, test tree rows, CLI selectors,
//! review packets, imported CI overlays, and support exports can all join
//! without matching on display text.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::execution_context::{ExecutionContext, TargetClass};
use crate::testing::{InlineTestResultRow, TestRunnerBetaProjection, TestTreeRow, TestTreeRowKind};
use crate::tests::{
    ImportedCiProjectionClass, TestAttemptAlphaPacket, TestAttemptKind, TestAttemptRecord,
};

/// Schema version for promoted test identity records.
pub const TEST_IDENTITY_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for a canonical test item.
pub const CANONICAL_TEST_ITEM_RECORD_KIND: &str = "canonical_test_item_record";

/// Stable record-kind tag for a canonical test selector binding.
pub const TEST_SELECTOR_BINDING_RECORD_KIND: &str = "test_selector_binding_record";

/// Stable record-kind tag for a canonical test session ledger.
pub const CANONICAL_TEST_SESSION_RECORD_KIND: &str = "canonical_test_session_record";

/// Stable record-kind tag for a canonical test attempt.
pub const CANONICAL_TEST_ATTEMPT_RECORD_KIND: &str = "canonical_test_attempt_record";

/// Stable record-kind tag for a surface identity binding.
pub const TEST_SURFACE_IDENTITY_BINDING_RECORD_KIND: &str = "test_surface_identity_binding_record";

/// Stable record-kind tag for an imported CI truth overlay.
pub const IMPORTED_CI_TRUTH_OVERLAY_RECORD_KIND: &str = "imported_ci_truth_overlay_record";

/// Stable record-kind tag for a support-export packet.
pub const TEST_IDENTITY_SUPPORT_EXPORT_RECORD_KIND: &str = "test_identity_support_export_record";

/// Stable record-kind tag for the top-level promoted identity bundle.
pub const TEST_IDENTITY_BETA_BUNDLE_RECORD_KIND: &str = "test_identity_beta_bundle_record";

/// Test adapter family that produced the canonical identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestAdapterKind {
    /// Pytest adapter.
    Pytest,
}

impl TestAdapterKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pytest => "pytest",
        }
    }
}

/// Canonical test item shape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalTestItemKind {
    /// A runnable test case.
    Case,
    /// A parameterized family root.
    ParameterizedFamily,
    /// One concrete parameterized invocation.
    ParameterizedInstance,
    /// A provider-imported row that has not mapped to local source.
    ImportedProviderInstance,
}

impl CanonicalTestItemKind {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Case => "case",
            Self::ParameterizedFamily => "parameterized_family",
            Self::ParameterizedInstance => "parameterized_instance",
            Self::ImportedProviderInstance => "imported_provider_instance",
        }
    }
}

/// Identity stability for a canonical test item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestItemIdentityClass {
    /// Stable item identity is present.
    Stable,
    /// Stable item identity needs remap review before rerun or debug.
    RemapReviewRequired,
    /// Item was imported from CI or a provider and is read-only locally.
    ImportedReadOnly,
    /// Item lacks a non-display identity and must fail closed.
    DisplayTextOnlyDenied,
    /// Identity state cannot be classified.
    UnknownRequiresReview,
}

impl TestItemIdentityClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::RemapReviewRequired => "remap_review_required",
            Self::ImportedReadOnly => "imported_read_only",
            Self::DisplayTextOnlyDenied => "display_text_only_denied",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Surface or flow that selected a test item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestSelectionOrigin {
    /// Editor inline marker or gutter action.
    EditorInline,
    /// Test tree row action.
    TestTree,
    /// CLI or headless selector.
    CliSelector,
    /// Command palette action.
    CommandPalette,
    /// Review packet or hosted review flow.
    ReviewPacket,
    /// Support export reconstruction flow.
    SupportExport,
    /// Imported CI overlay.
    ImportedCiOverlay,
    /// Rerun-last command.
    RerunLast,
    /// Debug action started from a test row.
    DebugAction,
}

impl TestSelectionOrigin {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorInline => "editor_inline",
            Self::TestTree => "test_tree",
            Self::CliSelector => "cli_selector",
            Self::CommandPalette => "command_palette",
            Self::ReviewPacket => "review_packet",
            Self::SupportExport => "support_export",
            Self::ImportedCiOverlay => "imported_ci_overlay",
            Self::RerunLast => "rerun_last",
            Self::DebugAction => "debug_action",
        }
    }
}

/// Consumer surface bound to one canonical test identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIdentitySurface {
    /// Editor inline marker.
    EditorInlineMarker,
    /// Test tree row.
    TestTreeRow,
    /// CLI selector row.
    CliSelector,
    /// Command palette selection row.
    CommandPalette,
    /// Review packet row.
    ReviewPacket,
    /// Support export row.
    SupportExport,
    /// Imported CI overlay row.
    ImportedCiOverlay,
}

impl TestIdentitySurface {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorInlineMarker => "editor_inline_marker",
            Self::TestTreeRow => "test_tree_row",
            Self::CliSelector => "cli_selector",
            Self::CommandPalette => "command_palette",
            Self::ReviewPacket => "review_packet",
            Self::SupportExport => "support_export",
            Self::ImportedCiOverlay => "imported_ci_overlay",
        }
    }
}

/// Attempt lineage inside a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestAttemptLineageClass {
    /// First local execution attempt in a session.
    Initial,
    /// Attempt produced by watch mode.
    WatchCycle,
    /// Rerun of a selected or failed subset.
    Rerun,
    /// Debug launch derived from a test attempt.
    DebugFromTest,
    /// Provider CI import attempt.
    ImportedCi,
    /// Local attempt used to confirm imported evidence.
    LocalConfirmation,
    /// Reconstruction attempt for support or release evidence.
    Reconstruction,
}

impl TestAttemptLineageClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Initial => "initial",
            Self::WatchCycle => "watch_cycle",
            Self::Rerun => "rerun",
            Self::DebugFromTest => "debug_from_test",
            Self::ImportedCi => "imported_ci",
            Self::LocalConfirmation => "local_confirmation",
            Self::Reconstruction => "reconstruction",
        }
    }
}

/// Evidence authority for one attempt or surface binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestEvidenceClass {
    /// Current local execution evidence.
    LiveLocal,
    /// Current container execution evidence.
    LiveContainer,
    /// Current remote execution evidence.
    LiveRemote,
    /// Current managed execution evidence.
    LiveManaged,
    /// Imported provider evidence that remains read-only locally.
    ImportedCiReadOnly,
    /// Imported provider evidence outside its freshness window.
    ImportedCiStale,
    /// Cached prior local evidence.
    CachedPrior,
    /// Fresh local attempt confirmed an imported signal without erasing it.
    FreshLocalConfirmation,
    /// Evidence class cannot be trusted without review.
    UnknownRequiresReview,
}

impl TestEvidenceClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLocal => "live_local",
            Self::LiveContainer => "live_container",
            Self::LiveRemote => "live_remote",
            Self::LiveManaged => "live_managed",
            Self::ImportedCiReadOnly => "imported_ci_read_only",
            Self::ImportedCiStale => "imported_ci_stale",
            Self::CachedPrior => "cached_prior",
            Self::FreshLocalConfirmation => "fresh_local_confirmation",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Result freshness label shared by test surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestResultFreshnessClass {
    /// Current live or locally confirmed evidence.
    Current,
    /// Imported evidence that has not been locally confirmed.
    Imported,
    /// Evidence is stale.
    Stale,
    /// Evidence is cached prior local state.
    Cached,
    /// Freshness cannot be classified.
    UnknownRequiresReview,
}

impl TestResultFreshnessClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::Cached => "cached",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Imported CI truth class for sessions, attempts, and overlays.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportedCiTruthClass {
    /// No imported CI evidence participates.
    NotImported,
    /// Imported CI evidence is visible and read-only.
    ImportedCurrentReadOnly,
    /// Imported CI evidence is stale and read-only.
    ImportedStaleReadOnly,
    /// A local attempt is required before claiming local truth.
    LocalConfirmationRequired,
    /// Fresh local attempt confirmed the imported relationship.
    ConfirmedByLocalRerun,
    /// Imported CI truth cannot be classified.
    UnknownRequiresReview,
}

impl ImportedCiTruthClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotImported => "not_imported",
            Self::ImportedCurrentReadOnly => "imported_current_read_only",
            Self::ImportedStaleReadOnly => "imported_stale_read_only",
            Self::LocalConfirmationRequired => "local_confirmation_required",
            Self::ConfirmedByLocalRerun => "confirmed_by_local_rerun",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Target environment family for a test session or attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestTargetEnvironmentClass {
    /// Local desktop or host environment.
    Local,
    /// Local container or devcontainer environment.
    Container,
    /// Remote host or remote workspace environment.
    Remote,
    /// Managed workspace or managed runtime.
    Managed,
    /// Provider CI environment.
    Ci,
    /// Environment cannot be classified.
    UnknownRequiresReview,
}

impl TestTargetEnvironmentClass {
    /// Stable token used in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Container => "container",
            Self::Remote => "remote",
            Self::Managed => "managed",
            Self::Ci => "ci",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Error returned by append-only ledger mutation helpers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestIdentityLedgerError {
    /// The session id was not present in the bundle.
    MissingSession(String),
    /// The attempt id already exists and cannot be overwritten.
    DuplicateAttempt(String),
    /// The attempt index does not append after the current session tail.
    NonAppendAttemptIndex {
        /// Session receiving the attempted append.
        session_id: String,
        /// Expected next attempt index.
        expected: u32,
        /// Supplied attempt index.
        actual: u32,
    },
}

/// Target and environment identity copied into sessions and attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestTargetEnvironmentIdentity {
    /// Target environment class.
    pub target_environment_class: TestTargetEnvironmentClass,
    /// Stable target environment token.
    pub target_environment_token: String,
    /// Execution-context ref used by the session or attempt.
    pub execution_context_ref: String,
    /// Canonical target id from the execution context.
    pub target_id: String,
    /// Stable target-class token from the execution context.
    pub target_class_token: String,
    /// Opaque environment fingerprint derived from target, toolchain, policy, and capsule refs.
    pub environment_fingerprint: String,
    /// Environment capsule ref.
    pub environment_capsule_ref: String,
    /// Environment capsule hash.
    pub environment_capsule_hash: String,
    /// Toolchain ref used by the attempt.
    pub toolchain_ref: String,
    /// Toolchain version token used by the attempt.
    pub toolchain_version: String,
}

impl TestTargetEnvironmentIdentity {
    /// Projects target and environment identity from an execution context.
    pub fn from_context(context: &ExecutionContext) -> Self {
        let target_environment_class =
            target_environment_class_for(context.target_identity.target_class);
        let environment_fingerprint = digest_token(&format!(
            "{}|{}|{}|{}|{}",
            context.execution_context_id,
            context.target_identity.canonical_target_id,
            context.toolchain_identity.toolchain_id,
            context.environment_capsule_ref.capsule_hash,
            context.policy_and_trust.policy_epoch
        ));
        Self {
            target_environment_class,
            target_environment_token: target_environment_class.as_str().to_owned(),
            execution_context_ref: context.execution_context_id.clone(),
            target_id: context.target_identity.canonical_target_id.clone(),
            target_class_token: context.target_identity.target_class.as_str().to_owned(),
            environment_fingerprint,
            environment_capsule_ref: context.environment_capsule_ref.capsule_id.clone(),
            environment_capsule_hash: context.environment_capsule_ref.capsule_hash.clone(),
            toolchain_ref: context.toolchain_identity.toolchain_id.clone(),
            toolchain_version: context.toolchain_identity.resolved_version.clone(),
        }
    }
}

/// Canonical runnable test item shared across all test surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalTestItem {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Canonical test item id.
    pub canonical_test_item_id: String,
    /// Test adapter kind.
    pub adapter_kind: TestAdapterKind,
    /// Stable adapter token.
    pub adapter_kind_token: String,
    /// Item kind.
    pub item_kind: CanonicalTestItemKind,
    /// Stable item-kind token.
    pub item_kind_token: String,
    /// Adapter-native stable item ref when one exists.
    pub adapter_item_ref: String,
    /// Logical item key independent of display label.
    pub logical_item_key: String,
    /// Source anchor ref safe for review and support surfaces.
    pub source_anchor_ref: String,
    /// Digest of the source anchor used for remap comparison.
    pub source_anchor_digest: String,
    /// Selector ref that addresses this item.
    pub selector_ref: String,
    /// Digest of the display projection; display text is not identity.
    pub display_label_digest: String,
    /// Identity stability class.
    pub identity_class: TestItemIdentityClass,
    /// Stable identity-class token.
    pub identity_class_token: String,
    /// Parameterized family ref when this item is an instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameterized_family_ref: Option<String>,
    /// Parameterized instance key when this item is an instance.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parameterized_instance_key: Option<String>,
    /// Remap records that explain file moves, source changes, or provider mapping.
    pub remap_record_refs: Vec<String>,
}

impl CanonicalTestItem {
    /// Builds a canonical item from a beta inline row.
    pub fn from_inline_row(row: &InlineTestResultRow) -> Self {
        let (parameterized_family_ref, parameterized_instance_key, item_kind) =
            parameterization_from_selector(&row.selector_ref);
        let source_anchor_ref = format!("{}:{}", row.source_file_ref, row.line_number);
        let identity_class = identity_class_from_token(&row.identity_stability_token);
        Self {
            record_kind: CANONICAL_TEST_ITEM_RECORD_KIND.to_owned(),
            schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
            canonical_test_item_id: row.canonical_test_item_ref.clone(),
            adapter_kind: TestAdapterKind::Pytest,
            adapter_kind_token: TestAdapterKind::Pytest.as_str().to_owned(),
            item_kind,
            item_kind_token: item_kind.as_str().to_owned(),
            adapter_item_ref: row.canonical_test_item_ref.clone(),
            logical_item_key: stable_token(&row.canonical_test_item_ref),
            source_anchor_ref: source_anchor_ref.clone(),
            source_anchor_digest: digest_token(&source_anchor_ref),
            selector_ref: row.selector_ref.clone(),
            display_label_digest: digest_token(&row.summary),
            identity_class,
            identity_class_token: identity_class.as_str().to_owned(),
            parameterized_family_ref,
            parameterized_instance_key,
            remap_record_refs: Vec::new(),
        }
    }

    /// True when the item has a non-display identity suitable for claimed adapters.
    pub fn has_claimable_identity(&self) -> bool {
        !self.canonical_test_item_id.is_empty()
            && !self.selector_ref.is_empty()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// Selector binding that can re-address a canonical test item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSelectorBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable selector binding id.
    pub selector_binding_id: String,
    /// Selector ref shared with sessions, attempts, and surfaces.
    pub selector_ref: String,
    /// Canonical test item id selected by this binding.
    pub canonical_test_item_id: String,
    /// Import-safe selector expression.
    pub selector_expression: String,
    /// Digest of the normalized selector expression.
    pub selector_digest: String,
    /// Selection origin that produced this binding.
    pub selection_origin: TestSelectionOrigin,
    /// Stable selection-origin token.
    pub selection_origin_token: String,
    /// Surfaces that can use this selector without matching display text.
    pub readdressable_surface_tokens: Vec<String>,
    /// True when display text was not used as the selector key.
    pub display_text_matching_forbidden: bool,
}

impl TestSelectorBinding {
    /// Builds an exact canonical-id selector binding.
    pub fn exact_item_selector(item: &CanonicalTestItem, origin: TestSelectionOrigin) -> Self {
        let selector_expression =
            format!("id:{}", escape_selector_value(&item.canonical_test_item_id));
        Self {
            record_kind: TEST_SELECTOR_BINDING_RECORD_KIND.to_owned(),
            schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
            selector_binding_id: format!(
                "test-selector-binding:{}:{}",
                origin.as_str(),
                stable_token(&item.canonical_test_item_id)
            ),
            selector_ref: item.selector_ref.clone(),
            canonical_test_item_id: item.canonical_test_item_id.clone(),
            selector_expression: selector_expression.clone(),
            selector_digest: digest_token(&selector_expression),
            selection_origin: origin,
            selection_origin_token: origin.as_str().to_owned(),
            readdressable_surface_tokens: canonical_surface_tokens(),
            display_text_matching_forbidden: true,
        }
    }
}

/// Canonical session ledger for one selected test item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalTestSession {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Canonical session id.
    pub test_session_id: String,
    /// Session plan ref from the alpha packet.
    pub session_plan_ref: String,
    /// Selection origin for the session.
    pub selection_origin: TestSelectionOrigin,
    /// Stable selection-origin token.
    pub selection_origin_token: String,
    /// Canonical test item refs in this session.
    pub canonical_test_item_refs: Vec<String>,
    /// Selector refs in this session.
    pub selector_refs: Vec<String>,
    /// Target and environment identity for the session.
    pub target_environment: TestTargetEnvironmentIdentity,
    /// Imported CI truth for the session.
    pub imported_ci_truth_class: ImportedCiTruthClass,
    /// Stable imported-CI truth token.
    pub imported_ci_truth_token: String,
    /// Session creation timestamp.
    pub created_at: String,
    /// Session update timestamp.
    pub updated_at: String,
    /// Ordered append-only attempt refs.
    pub attempt_refs: Vec<String>,
    /// Latest attempt ref visible on surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Surface binding refs that cite this session.
    pub surface_binding_refs: Vec<String>,
    /// Export-safe summary.
    pub support_summary: String,
}

impl CanonicalTestSession {
    /// True when the ordered attempt refs contain no duplicates.
    pub fn attempt_refs_are_unique(&self) -> bool {
        let mut seen = BTreeSet::new();
        self.attempt_refs.iter().all(|attempt| seen.insert(attempt))
    }
}

/// Canonical append-only attempt record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalTestAttempt {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Canonical attempt id.
    pub test_attempt_id: String,
    /// Parent session ref.
    pub parent_test_session_ref: String,
    /// One-based attempt index.
    pub attempt_index: u32,
    /// Attempt lineage class.
    pub lineage_class: TestAttemptLineageClass,
    /// Stable lineage token.
    pub lineage_token: String,
    /// Predecessor attempt ref for rerun/debug/local confirmation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub predecessor_attempt_ref: Option<String>,
    /// Origin attempt ref for imported or debug-derived attempts.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_attempt_ref: Option<String>,
    /// Generic execution attempt ref when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_attempt_ref: Option<String>,
    /// Canonical test item refs covered by the attempt.
    pub canonical_test_item_refs: Vec<String>,
    /// Selector ref covered by the attempt.
    pub selector_ref: String,
    /// Selection origin for this attempt.
    pub selection_origin: TestSelectionOrigin,
    /// Stable selection-origin token.
    pub selection_origin_token: String,
    /// Target and environment identity captured at attempt open.
    pub target_environment: TestTargetEnvironmentIdentity,
    /// Evidence authority class.
    pub evidence_class: TestEvidenceClass,
    /// Stable evidence-class token.
    pub evidence_class_token: String,
    /// Result freshness class.
    pub freshness_class: TestResultFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Imported CI truth class.
    pub imported_ci_truth_class: ImportedCiTruthClass,
    /// Stable imported-CI truth token.
    pub imported_ci_truth_token: String,
    /// Result state token from the attempt ledger.
    pub result_state_token: String,
    /// Attempt start timestamp when known.
    pub started_at: String,
    /// Attempt end timestamp when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ended_at: Option<String>,
    /// Attempt capture timestamp.
    pub captured_at: String,
    /// Artifact refs retained on governed rails.
    pub artifact_refs: Vec<String>,
    /// Raw event refs retained on governed rails.
    pub raw_event_refs: Vec<String>,
    /// Export-safe summary.
    pub support_summary: String,
}

impl CanonicalTestAttempt {
    /// Builds a follow-up attempt that appends to an existing session.
    pub fn follow_up(
        session: &CanonicalTestSession,
        previous_attempt: &CanonicalTestAttempt,
        lineage_class: TestAttemptLineageClass,
        result_state_token: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> Self {
        let attempt_index = previous_attempt.attempt_index.saturating_add(1);
        let captured_at = captured_at.into();
        let test_attempt_id = format!(
            "test-attempt:{}:{}",
            stable_token(&session.test_session_id),
            attempt_index
        );
        let evidence_class =
            if session.imported_ci_truth_class == ImportedCiTruthClass::ConfirmedByLocalRerun {
                TestEvidenceClass::FreshLocalConfirmation
            } else {
                previous_attempt.evidence_class
            };
        let freshness_class = freshness_for_evidence(evidence_class);
        Self {
            record_kind: CANONICAL_TEST_ATTEMPT_RECORD_KIND.to_owned(),
            schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
            test_attempt_id,
            parent_test_session_ref: session.test_session_id.clone(),
            attempt_index,
            lineage_class,
            lineage_token: lineage_class.as_str().to_owned(),
            predecessor_attempt_ref: Some(previous_attempt.test_attempt_id.clone()),
            origin_attempt_ref: previous_attempt.origin_attempt_ref.clone(),
            execution_attempt_ref: None,
            canonical_test_item_refs: session.canonical_test_item_refs.clone(),
            selector_ref: session.selector_refs.first().cloned().unwrap_or_default(),
            selection_origin: TestSelectionOrigin::RerunLast,
            selection_origin_token: TestSelectionOrigin::RerunLast.as_str().to_owned(),
            target_environment: session.target_environment.clone(),
            evidence_class,
            evidence_class_token: evidence_class.as_str().to_owned(),
            freshness_class,
            freshness_token: freshness_class.as_str().to_owned(),
            imported_ci_truth_class: session.imported_ci_truth_class,
            imported_ci_truth_token: session.imported_ci_truth_token.clone(),
            result_state_token: result_state_token.into(),
            started_at: captured_at.clone(),
            ended_at: None,
            captured_at,
            artifact_refs: Vec::new(),
            raw_event_refs: Vec::new(),
            support_summary:
                "Follow-up attempt appended without overwriting prior attempt history.".to_owned(),
        }
    }
}

/// Binding between one consumer surface and one canonical identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestSurfaceIdentityBinding {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable binding id.
    pub surface_binding_id: String,
    /// Surface kind.
    pub surface: TestIdentitySurface,
    /// Stable surface token.
    pub surface_token: String,
    /// Surface-local row ref.
    pub surface_ref: String,
    /// Canonical test item id.
    pub canonical_test_item_id: String,
    /// Canonical test session ref.
    pub test_session_ref: String,
    /// Latest attempt ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_attempt_ref: Option<String>,
    /// Selector ref the surface uses.
    pub selector_ref: String,
    /// Selection origin represented by this surface binding.
    pub selection_origin: TestSelectionOrigin,
    /// Stable selection-origin token.
    pub selection_origin_token: String,
    /// Evidence authority class.
    pub evidence_class: TestEvidenceClass,
    /// Stable evidence-class token.
    pub evidence_class_token: String,
    /// Result freshness class.
    pub freshness_class: TestResultFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Imported CI truth class.
    pub imported_ci_truth_class: ImportedCiTruthClass,
    /// Stable imported-CI truth token.
    pub imported_ci_truth_token: String,
    /// True when this surface may dispatch live rerun/debug without review.
    pub live_dispatch_allowed: bool,
}

/// Imported CI overlay preserving provider truth separately from local truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedCiTruthOverlay {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable overlay id.
    pub imported_ci_overlay_id: String,
    /// Canonical test item id.
    pub canonical_test_item_id: String,
    /// Test session ref.
    pub test_session_ref: String,
    /// Provider run ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_run_ref: Option<String>,
    /// Imported attempt refs.
    pub imported_attempt_refs: Vec<String>,
    /// Local confirmation attempt refs.
    pub local_confirmation_attempt_refs: Vec<String>,
    /// Imported CI truth class.
    pub imported_ci_truth_class: ImportedCiTruthClass,
    /// Stable imported-CI truth token.
    pub imported_ci_truth_token: String,
    /// Result freshness class rendered by the overlay.
    pub freshness_class: TestResultFreshnessClass,
    /// Stable freshness token rendered by the overlay.
    pub freshness_token: String,
    /// True when this overlay may be treated as live local truth.
    pub current_local_truth_claim_allowed: bool,
    /// Export-safe overlay summary.
    pub support_summary: String,
}

/// Support-export packet for the promoted identity bundle.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestIdentitySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub support_export_id: String,
    /// Export generation timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Canonical item refs included in the export.
    pub canonical_test_item_refs: Vec<String>,
    /// Session refs included in the export.
    pub test_session_refs: Vec<String>,
    /// Attempt refs included in the export.
    pub test_attempt_refs: Vec<String>,
    /// Selector refs included in the export.
    pub selector_refs: Vec<String>,
    /// Surface binding refs included in the export.
    pub surface_binding_refs: Vec<String>,
    /// Imported CI overlay refs included in the export.
    pub imported_ci_overlay_refs: Vec<String>,
    /// True when support can reconstruct identity/session/attempt lineage without raw logs.
    pub reconstructable_without_raw_logs: bool,
    /// Export-safe summary lines.
    pub summary_lines: Vec<String>,
}

impl TestIdentitySupportExport {
    /// Renders stable plaintext lines for support review.
    pub fn render_plaintext(&self) -> String {
        let mut out = format!("Test identity support export: {}\n", self.support_export_id);
        out.push_str(&format!("Workspace: {}\n", self.workspace_id));
        out.push_str(&format!("Generated at: {}\n", self.generated_at));
        out.push_str(&format!(
            "Canonical items: {}\n",
            self.canonical_test_item_refs.len()
        ));
        out.push_str(&format!("Sessions: {}\n", self.test_session_refs.len()));
        out.push_str(&format!("Attempts: {}\n", self.test_attempt_refs.len()));
        out.push_str(&format!("Selectors: {}\n", self.selector_refs.len()));
        out.push_str(&format!("Surfaces: {}\n", self.surface_binding_refs.len()));
        out.push_str(&format!(
            "Imported CI overlays: {}\n",
            self.imported_ci_overlay_refs.len()
        ));
        for line in &self.summary_lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }
}

/// Top-level promoted identity bundle for one beta test projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestIdentityBetaBundle {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable bundle id.
    pub bundle_id: String,
    /// Bundle generation timestamp.
    pub generated_at: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Canonical test item records.
    pub items: Vec<CanonicalTestItem>,
    /// Selector binding records.
    pub selectors: Vec<TestSelectorBinding>,
    /// Session ledgers.
    pub sessions: Vec<CanonicalTestSession>,
    /// Append-only attempt records.
    pub attempts: Vec<CanonicalTestAttempt>,
    /// Surface identity bindings.
    pub surface_bindings: Vec<TestSurfaceIdentityBinding>,
    /// Imported CI truth overlays.
    pub imported_ci_overlays: Vec<ImportedCiTruthOverlay>,
    /// Support export for the bundle.
    pub support_export: TestIdentitySupportExport,
}

impl TestIdentityBetaBundle {
    /// Projects a promoted identity bundle from the beta test-runner projection.
    pub fn from_runner_projection(
        projection: &TestRunnerBetaProjection,
        generated_at: impl Into<String>,
    ) -> Self {
        let generated_at = generated_at.into();
        let workspace_id = projection.workspace_id.clone();
        let bundle_id = format!(
            "test-identity-beta:{}:{}",
            stable_token(&workspace_id),
            stable_token(&generated_at)
        );
        let support_export_id = format!("test-identity-support:{}", stable_token(&bundle_id));
        let target_environment = target_environment_from_projection(projection);
        let case_rows = case_rows_by_item(&projection.tree.rows);
        let mut items = Vec::new();
        let mut selectors = Vec::new();
        let mut sessions = Vec::new();
        let mut attempts = Vec::new();
        let mut surface_bindings = Vec::new();
        let mut imported_ci_overlays = Vec::new();

        for inline in &projection.inline.rows {
            let item = CanonicalTestItem::from_inline_row(inline);
            let packet = matching_packet(&projection.attempt_packets, &item.canonical_test_item_id);
            let session_id = packet
                .map(|packet| packet.session_plan.test_session_id.clone())
                .unwrap_or_else(|| {
                    format!(
                        "test-session:{}",
                        stable_token(&item.canonical_test_item_id)
                    )
                });
            let imported_truth = packet
                .map(|packet| {
                    imported_truth_for_projection(packet.imported_ci_projection.projection_class)
                })
                .unwrap_or(ImportedCiTruthClass::UnknownRequiresReview);
            let session_attempts = packet
                .map(|packet| {
                    packet
                        .attempts
                        .iter()
                        .map(|attempt| {
                            canonical_attempt_for_alpha(
                                attempt,
                                &target_environment,
                                TestSelectionOrigin::TestTree,
                                &packet.generated_at,
                            )
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            let attempt_refs = session_attempts
                .iter()
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect::<Vec<_>>();
            let latest_attempt_ref = inline
                .latest_attempt_ref
                .clone()
                .or_else(|| attempt_refs.last().cloned());

            let mut surface_refs = Vec::new();
            let tree_ref = case_rows
                .get(&item.canonical_test_item_id)
                .map(|row| row.tree_row_id.clone())
                .unwrap_or_else(|| {
                    format!(
                        "test-tree-row:{}:missing",
                        stable_token(&item.canonical_test_item_id)
                    )
                });
            let surface_specs = [
                (
                    TestIdentitySurface::EditorInlineMarker,
                    inline.inline_row_id.clone(),
                ),
                (TestIdentitySurface::TestTreeRow, tree_ref),
                (
                    TestIdentitySurface::CliSelector,
                    format!(
                        "cli-selector:{}",
                        stable_token(&item.canonical_test_item_id)
                    ),
                ),
                (
                    TestIdentitySurface::CommandPalette,
                    format!(
                        "command-palette-selector:{}",
                        stable_token(&item.canonical_test_item_id)
                    ),
                ),
                (
                    TestIdentitySurface::ReviewPacket,
                    format!(
                        "review-test-row:{}",
                        stable_token(&item.canonical_test_item_id)
                    ),
                ),
                (
                    TestIdentitySurface::SupportExport,
                    support_export_id.clone(),
                ),
                (
                    TestIdentitySurface::ImportedCiOverlay,
                    format!(
                        "imported-ci-overlay:{}",
                        stable_token(&item.canonical_test_item_id)
                    ),
                ),
            ];
            let latest_attempt = latest_attempt_ref
                .as_deref()
                .and_then(|attempt_ref| {
                    session_attempts
                        .iter()
                        .find(|attempt| attempt.test_attempt_id == attempt_ref)
                })
                .or_else(|| session_attempts.last());
            let evidence_class = latest_attempt
                .map(|attempt| attempt.evidence_class)
                .unwrap_or_else(|| evidence_for_target_class(&target_environment));
            let freshness_class = latest_attempt
                .map(|attempt| attempt.freshness_class)
                .unwrap_or_else(|| freshness_for_imported_truth(imported_truth));
            for (surface, surface_ref) in surface_specs {
                let surface_binding_id = format!(
                    "test-surface-binding:{}:{}",
                    surface.as_str(),
                    stable_token(&item.canonical_test_item_id)
                );
                let selection_origin = selection_origin_for_surface(surface);
                surface_refs.push(surface_binding_id.clone());
                surface_bindings.push(TestSurfaceIdentityBinding {
                    record_kind: TEST_SURFACE_IDENTITY_BINDING_RECORD_KIND.to_owned(),
                    schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
                    surface_binding_id,
                    surface,
                    surface_token: surface.as_str().to_owned(),
                    surface_ref,
                    canonical_test_item_id: item.canonical_test_item_id.clone(),
                    test_session_ref: session_id.clone(),
                    latest_attempt_ref: latest_attempt_ref.clone(),
                    selector_ref: item.selector_ref.clone(),
                    selection_origin,
                    selection_origin_token: selection_origin.as_str().to_owned(),
                    evidence_class,
                    evidence_class_token: evidence_class.as_str().to_owned(),
                    freshness_class,
                    freshness_token: freshness_class.as_str().to_owned(),
                    imported_ci_truth_class: imported_truth,
                    imported_ci_truth_token: imported_truth.as_str().to_owned(),
                    live_dispatch_allowed: imported_truth_allows_live_dispatch(imported_truth),
                });
            }

            let overlay = imported_ci_overlay_for_packet(
                &item,
                &session_id,
                packet,
                imported_truth,
                freshness_class,
            );
            imported_ci_overlays.push(overlay);

            sessions.push(CanonicalTestSession {
                record_kind: CANONICAL_TEST_SESSION_RECORD_KIND.to_owned(),
                schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
                test_session_id: session_id,
                session_plan_ref: packet
                    .map(|packet| packet.session_plan.session_plan_id.clone())
                    .unwrap_or_else(|| {
                        format!(
                            "test-session-plan:{}",
                            stable_token(&item.canonical_test_item_id)
                        )
                    }),
                selection_origin: TestSelectionOrigin::TestTree,
                selection_origin_token: TestSelectionOrigin::TestTree.as_str().to_owned(),
                canonical_test_item_refs: vec![item.canonical_test_item_id.clone()],
                selector_refs: vec![item.selector_ref.clone()],
                target_environment: target_environment.clone(),
                imported_ci_truth_class: imported_truth,
                imported_ci_truth_token: imported_truth.as_str().to_owned(),
                created_at: packet
                    .map(|packet| packet.generated_at.clone())
                    .unwrap_or_else(|| generated_at.clone()),
                updated_at: generated_at.clone(),
                attempt_refs,
                latest_attempt_ref,
                surface_binding_refs: surface_refs,
                support_summary: format!(
                    "Session preserves item {}, selector {}, target {}, and environment {}.",
                    item.canonical_test_item_id,
                    item.selector_ref,
                    target_environment.target_id,
                    target_environment.environment_fingerprint
                ),
            });
            selectors.push(TestSelectorBinding::exact_item_selector(
                &item,
                TestSelectionOrigin::CliSelector,
            ));
            attempts.extend(session_attempts);
            items.push(item);
        }

        let support_export = TestIdentitySupportExport {
            record_kind: TEST_IDENTITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
            support_export_id,
            generated_at: generated_at.clone(),
            workspace_id: workspace_id.clone(),
            canonical_test_item_refs: items
                .iter()
                .map(|item| item.canonical_test_item_id.clone())
                .collect(),
            test_session_refs: sessions
                .iter()
                .map(|session| session.test_session_id.clone())
                .collect(),
            test_attempt_refs: attempts
                .iter()
                .map(|attempt| attempt.test_attempt_id.clone())
                .collect(),
            selector_refs: selectors
                .iter()
                .map(|selector| selector.selector_ref.clone())
                .collect(),
            surface_binding_refs: surface_bindings
                .iter()
                .map(|binding| binding.surface_binding_id.clone())
                .collect(),
            imported_ci_overlay_refs: imported_ci_overlays
                .iter()
                .map(|overlay| overlay.imported_ci_overlay_id.clone())
                .collect(),
            reconstructable_without_raw_logs: true,
            summary_lines: support_summary_lines(&items, &sessions, &attempts),
        };

        Self {
            record_kind: TEST_IDENTITY_BETA_BUNDLE_RECORD_KIND.to_owned(),
            schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
            bundle_id,
            generated_at,
            workspace_id,
            items,
            selectors,
            sessions,
            attempts,
            surface_bindings,
            imported_ci_overlays,
            support_export,
        }
    }

    /// Returns surface bindings for one canonical item.
    pub fn surface_bindings_for_item(
        &self,
        canonical_test_item_id: &str,
    ) -> Vec<&TestSurfaceIdentityBinding> {
        self.surface_bindings
            .iter()
            .filter(|binding| binding.canonical_test_item_id == canonical_test_item_id)
            .collect()
    }

    /// True when all surfaces for an item point at the same item, session, selector, and latest attempt.
    pub fn surface_bindings_resolve_to_same_ids(&self, canonical_test_item_id: &str) -> bool {
        let bindings = self.surface_bindings_for_item(canonical_test_item_id);
        if bindings.is_empty() {
            return false;
        }
        let first = bindings[0];
        bindings.iter().all(|binding| {
            binding.canonical_test_item_id == first.canonical_test_item_id
                && binding.test_session_ref == first.test_session_ref
                && binding.selector_ref == first.selector_ref
                && binding.latest_attempt_ref == first.latest_attempt_ref
        })
    }

    /// Returns true when all sessions preserve append-only attempt ordering.
    pub fn attempt_history_is_append_only(&self) -> bool {
        self.sessions.iter().all(|session| {
            if !session.attempt_refs_are_unique() {
                return false;
            }
            let mut expected = 1;
            for attempt_ref in &session.attempt_refs {
                let Some(attempt) = self
                    .attempts
                    .iter()
                    .find(|attempt| &attempt.test_attempt_id == attempt_ref)
                else {
                    return false;
                };
                if attempt.parent_test_session_ref != session.test_session_id
                    || attempt.attempt_index != expected
                {
                    return false;
                }
                expected += 1;
            }
            true
        })
    }

    /// Returns rows that cannot satisfy the no-anonymous claimed-adapter rule.
    pub fn anonymous_or_display_text_only_items(&self) -> Vec<&CanonicalTestItem> {
        self.items
            .iter()
            .filter(|item| !item.has_claimable_identity())
            .collect()
    }

    /// True when imported evidence remains labelled imported or stale unless a local rerun confirms it.
    pub fn imported_evidence_preserves_truth_class(&self) -> bool {
        self.imported_ci_overlays
            .iter()
            .all(|overlay| match overlay.imported_ci_truth_class {
                ImportedCiTruthClass::NotImported => true,
                ImportedCiTruthClass::ConfirmedByLocalRerun => {
                    overlay.current_local_truth_claim_allowed
                        && overlay.freshness_class == TestResultFreshnessClass::Current
                }
                ImportedCiTruthClass::ImportedCurrentReadOnly
                | ImportedCiTruthClass::LocalConfirmationRequired => {
                    !overlay.current_local_truth_claim_allowed
                        && overlay.freshness_class == TestResultFreshnessClass::Imported
                }
                ImportedCiTruthClass::ImportedStaleReadOnly => {
                    !overlay.current_local_truth_claim_allowed
                        && overlay.freshness_class == TestResultFreshnessClass::Stale
                }
                ImportedCiTruthClass::UnknownRequiresReview => {
                    !overlay.current_local_truth_claim_allowed
                        && overlay.freshness_class
                            == TestResultFreshnessClass::UnknownRequiresReview
                }
            })
    }

    /// Appends an attempt to the session without overwriting existing history.
    pub fn append_attempt(
        &mut self,
        attempt: CanonicalTestAttempt,
    ) -> Result<(), TestIdentityLedgerError> {
        if self
            .attempts
            .iter()
            .any(|existing| existing.test_attempt_id == attempt.test_attempt_id)
        {
            return Err(TestIdentityLedgerError::DuplicateAttempt(
                attempt.test_attempt_id,
            ));
        }
        let session = self
            .sessions
            .iter_mut()
            .find(|session| session.test_session_id == attempt.parent_test_session_ref)
            .ok_or_else(|| {
                TestIdentityLedgerError::MissingSession(attempt.parent_test_session_ref.clone())
            })?;
        let expected = session.attempt_refs.len() as u32 + 1;
        if attempt.attempt_index != expected {
            return Err(TestIdentityLedgerError::NonAppendAttemptIndex {
                session_id: session.test_session_id.clone(),
                expected,
                actual: attempt.attempt_index,
            });
        }
        let attempt_id = attempt.test_attempt_id.clone();
        session.latest_attempt_ref = Some(attempt_id.clone());
        session.attempt_refs.push(attempt_id.clone());
        for binding in &mut self.surface_bindings {
            if binding.test_session_ref == session.test_session_id {
                binding.latest_attempt_ref = Some(attempt_id.clone());
                binding.evidence_class = attempt.evidence_class;
                binding.evidence_class_token = attempt.evidence_class_token.clone();
                binding.freshness_class = attempt.freshness_class;
                binding.freshness_token = attempt.freshness_token.clone();
                binding.imported_ci_truth_class = attempt.imported_ci_truth_class;
                binding.imported_ci_truth_token = attempt.imported_ci_truth_token.clone();
                binding.live_dispatch_allowed =
                    imported_truth_allows_live_dispatch(attempt.imported_ci_truth_class);
            }
        }
        self.support_export.test_attempt_refs.push(attempt_id);
        self.attempts.push(attempt);
        Ok(())
    }
}

fn canonical_attempt_for_alpha(
    attempt: &TestAttemptRecord,
    target_environment: &TestTargetEnvironmentIdentity,
    selection_origin: TestSelectionOrigin,
    captured_at: &str,
) -> CanonicalTestAttempt {
    let lineage_class = lineage_for_attempt_kind(attempt.attempt_kind);
    let imported_ci_truth_class =
        imported_truth_for_projection(attempt.imported_ci_projection_class);
    let evidence_class = evidence_for_attempt(attempt, target_environment);
    let freshness_class = freshness_for_attempt(attempt);
    CanonicalTestAttempt {
        record_kind: CANONICAL_TEST_ATTEMPT_RECORD_KIND.to_owned(),
        schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
        test_attempt_id: attempt.test_attempt_id.clone(),
        parent_test_session_ref: attempt.parent_test_session_ref.clone(),
        attempt_index: attempt.attempt_index,
        lineage_class,
        lineage_token: lineage_class.as_str().to_owned(),
        predecessor_attempt_ref: attempt.predecessor_attempt_ref.clone(),
        origin_attempt_ref: attempt.origin_attempt_ref.clone(),
        execution_attempt_ref: attempt.execution_attempt_ref.clone(),
        canonical_test_item_refs: attempt.canonical_test_item_refs.clone(),
        selector_ref: attempt.selector_ref.clone(),
        selection_origin,
        selection_origin_token: selection_origin.as_str().to_owned(),
        target_environment: target_environment.clone(),
        evidence_class,
        evidence_class_token: evidence_class.as_str().to_owned(),
        freshness_class,
        freshness_token: freshness_class.as_str().to_owned(),
        imported_ci_truth_class,
        imported_ci_truth_token: imported_ci_truth_class.as_str().to_owned(),
        result_state_token: attempt.result_state_token.clone(),
        started_at: captured_at.to_owned(),
        ended_at: None,
        captured_at: captured_at.to_owned(),
        artifact_refs: attempt.artifact_refs.clone(),
        raw_event_refs: attempt.raw_event_refs.clone(),
        support_summary: attempt.support_summary.clone(),
    }
}

fn imported_ci_overlay_for_packet(
    item: &CanonicalTestItem,
    session_id: &str,
    packet: Option<&TestAttemptAlphaPacket>,
    imported_truth: ImportedCiTruthClass,
    fallback_freshness: TestResultFreshnessClass,
) -> ImportedCiTruthOverlay {
    let (provider_run_ref, imported_attempt_refs, local_confirmation_attempt_refs) =
        if let Some(packet) = packet {
            (
                packet.imported_ci_projection.provider_run_ref.clone(),
                packet.imported_ci_projection.imported_attempt_refs.clone(),
                packet
                    .imported_ci_projection
                    .local_reconfirmation_attempt_refs
                    .clone(),
            )
        } else {
            (None, Vec::new(), Vec::new())
        };
    let freshness_class = freshness_for_imported_truth(imported_truth);
    let freshness_class = if freshness_class == TestResultFreshnessClass::UnknownRequiresReview {
        fallback_freshness
    } else {
        freshness_class
    };
    let current_local_truth_claim_allowed =
        imported_truth == ImportedCiTruthClass::ConfirmedByLocalRerun;
    ImportedCiTruthOverlay {
        record_kind: IMPORTED_CI_TRUTH_OVERLAY_RECORD_KIND.to_owned(),
        schema_version: TEST_IDENTITY_BETA_SCHEMA_VERSION,
        imported_ci_overlay_id: format!(
            "imported-ci-overlay:{}",
            stable_token(&item.canonical_test_item_id)
        ),
        canonical_test_item_id: item.canonical_test_item_id.clone(),
        test_session_ref: session_id.to_owned(),
        provider_run_ref,
        imported_attempt_refs,
        local_confirmation_attempt_refs,
        imported_ci_truth_class: imported_truth,
        imported_ci_truth_token: imported_truth.as_str().to_owned(),
        freshness_class,
        freshness_token: freshness_class.as_str().to_owned(),
        current_local_truth_claim_allowed,
        support_summary: match imported_truth {
            ImportedCiTruthClass::NotImported => {
                "No imported CI evidence participates for this item.".to_owned()
            }
            ImportedCiTruthClass::ImportedCurrentReadOnly => {
                "Imported CI evidence is current for the provider and read-only locally.".to_owned()
            }
            ImportedCiTruthClass::ImportedStaleReadOnly => {
                "Imported CI evidence is stale and read-only locally.".to_owned()
            }
            ImportedCiTruthClass::LocalConfirmationRequired => {
                "Imported CI evidence needs a local confirmation attempt before local truth claims."
                    .to_owned()
            }
            ImportedCiTruthClass::ConfirmedByLocalRerun => {
                "Fresh local evidence confirms the imported relationship without erasing provider truth."
                    .to_owned()
            }
            ImportedCiTruthClass::UnknownRequiresReview => {
                "Imported CI evidence requires review before it can be interpreted.".to_owned()
            }
        },
    }
}

fn target_environment_from_projection(
    projection: &TestRunnerBetaProjection,
) -> TestTargetEnvironmentIdentity {
    if let Some(packet) = projection.attempt_packets.first() {
        if let Some(provenance) = &packet.session_plan.context_provenance {
            let target_environment_class = target_environment_class_for(provenance.target_class);
            return TestTargetEnvironmentIdentity {
                target_environment_class,
                target_environment_token: target_environment_class.as_str().to_owned(),
                execution_context_ref: packet.session_plan.execution_context_ref.clone(),
                target_id: packet.session_plan.target_id.clone(),
                target_class_token: packet.session_plan.target_class_token.clone(),
                environment_fingerprint: digest_token(&format!(
                    "{}|{}|{}|{}",
                    packet.session_plan.execution_context_ref,
                    packet.session_plan.target_id,
                    packet.session_plan.target_class_token,
                    provenance.context_provenance_id
                )),
                environment_capsule_ref: provenance.environment_capsule_ref.clone(),
                environment_capsule_hash: provenance.environment_capsule_hash.clone(),
                toolchain_ref: provenance.toolchain_id.clone(),
                toolchain_version: provenance.resolved_version.clone(),
            };
        }
    }
    TestTargetEnvironmentIdentity {
        target_environment_class: TestTargetEnvironmentClass::UnknownRequiresReview,
        target_environment_token: TestTargetEnvironmentClass::UnknownRequiresReview
            .as_str()
            .to_owned(),
        execution_context_ref: projection.tree.execution_context_ref.clone(),
        target_id: projection.tree.target_id.clone(),
        target_class_token: "unknown_requires_review".to_owned(),
        environment_fingerprint: digest_token(&format!(
            "{}|{}",
            projection.tree.execution_context_ref, projection.tree.target_id
        )),
        environment_capsule_ref: "environment-capsule:unknown".to_owned(),
        environment_capsule_hash: "sha256:unknown".to_owned(),
        toolchain_ref: "toolchain:unknown".to_owned(),
        toolchain_version: "unknown".to_owned(),
    }
}

fn target_environment_class_for(target_class: TargetClass) -> TestTargetEnvironmentClass {
    match target_class {
        TargetClass::LocalHost | TargetClass::NotebookKernelLocal => {
            TestTargetEnvironmentClass::Local
        }
        TargetClass::ContainerLocal | TargetClass::Devcontainer => {
            TestTargetEnvironmentClass::Container
        }
        TargetClass::SshRemote
        | TargetClass::RemoteWorkspaceVm
        | TargetClass::NotebookKernelRemote => TestTargetEnvironmentClass::Remote,
        TargetClass::PrebuildRuntime | TargetClass::ManagedWorkspace | TargetClass::AiSandbox => {
            TestTargetEnvironmentClass::Managed
        }
    }
}

fn identity_class_from_token(token: &str) -> TestItemIdentityClass {
    match token {
        "stable" => TestItemIdentityClass::Stable,
        "imported_read_only" => TestItemIdentityClass::ImportedReadOnly,
        "remap_review_required" => TestItemIdentityClass::RemapReviewRequired,
        "display_text_only_denied" => TestItemIdentityClass::DisplayTextOnlyDenied,
        _ => TestItemIdentityClass::UnknownRequiresReview,
    }
}

fn parameterization_from_selector(
    selector_ref: &str,
) -> (Option<String>, Option<String>, CanonicalTestItemKind) {
    if let Some((family, instance)) = selector_ref.split_once('[') {
        let instance = instance.trim_end_matches(']');
        (
            Some(family.to_owned()),
            Some(instance.to_owned()),
            CanonicalTestItemKind::ParameterizedInstance,
        )
    } else {
        (None, None, CanonicalTestItemKind::Case)
    }
}

fn matching_packet<'a>(
    packets: &'a [TestAttemptAlphaPacket],
    canonical_test_item_id: &str,
) -> Option<&'a TestAttemptAlphaPacket> {
    packets.iter().find(|packet| {
        packet
            .identity_projection
            .canonical_test_item_ref
            .as_deref()
            == Some(canonical_test_item_id)
            || packet.attempts.iter().any(|attempt| {
                attempt
                    .canonical_test_item_refs
                    .iter()
                    .any(|item_ref| item_ref == canonical_test_item_id)
            })
    })
}

fn case_rows_by_item(rows: &[TestTreeRow]) -> BTreeMap<String, &TestTreeRow> {
    let mut out = BTreeMap::new();
    for row in rows {
        if row.row_kind == TestTreeRowKind::TestCase {
            if let Some(canonical) = &row.canonical_test_item_ref {
                out.insert(canonical.clone(), row);
            }
        }
    }
    out
}

fn lineage_for_attempt_kind(kind: TestAttemptKind) -> TestAttemptLineageClass {
    match kind {
        TestAttemptKind::InitialTestRun => TestAttemptLineageClass::Initial,
        TestAttemptKind::WatchCycle => TestAttemptLineageClass::WatchCycle,
        TestAttemptKind::RerunFailed => TestAttemptLineageClass::Rerun,
        TestAttemptKind::DebugFromTest => TestAttemptLineageClass::DebugFromTest,
        TestAttemptKind::ProviderCiImport => TestAttemptLineageClass::ImportedCi,
        TestAttemptKind::LocalParityRerun => TestAttemptLineageClass::LocalConfirmation,
        TestAttemptKind::Reconstruction => TestAttemptLineageClass::Reconstruction,
    }
}

fn imported_truth_for_projection(class: ImportedCiProjectionClass) -> ImportedCiTruthClass {
    match class {
        ImportedCiProjectionClass::NotImportedCi => ImportedCiTruthClass::NotImported,
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly => {
            ImportedCiTruthClass::ImportedCurrentReadOnly
        }
        ImportedCiProjectionClass::StaleImportedReadOnly => {
            ImportedCiTruthClass::ImportedStaleReadOnly
        }
        ImportedCiProjectionClass::FreshLocalReconfirmation => {
            ImportedCiTruthClass::ConfirmedByLocalRerun
        }
        ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview => {
            ImportedCiTruthClass::UnknownRequiresReview
        }
    }
}

fn imported_truth_allows_live_dispatch(class: ImportedCiTruthClass) -> bool {
    matches!(
        class,
        ImportedCiTruthClass::NotImported | ImportedCiTruthClass::ConfirmedByLocalRerun
    )
}

fn evidence_for_attempt(
    attempt: &TestAttemptRecord,
    target_environment: &TestTargetEnvironmentIdentity,
) -> TestEvidenceClass {
    match attempt.imported_ci_projection_class {
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly => {
            TestEvidenceClass::ImportedCiReadOnly
        }
        ImportedCiProjectionClass::StaleImportedReadOnly => TestEvidenceClass::ImportedCiStale,
        ImportedCiProjectionClass::FreshLocalReconfirmation => {
            TestEvidenceClass::FreshLocalConfirmation
        }
        ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview => {
            TestEvidenceClass::UnknownRequiresReview
        }
        ImportedCiProjectionClass::NotImportedCi => evidence_for_target_class(target_environment),
    }
}

fn evidence_for_target_class(
    target_environment: &TestTargetEnvironmentIdentity,
) -> TestEvidenceClass {
    match target_environment.target_environment_class {
        TestTargetEnvironmentClass::Local => TestEvidenceClass::LiveLocal,
        TestTargetEnvironmentClass::Container => TestEvidenceClass::LiveContainer,
        TestTargetEnvironmentClass::Remote => TestEvidenceClass::LiveRemote,
        TestTargetEnvironmentClass::Managed => TestEvidenceClass::LiveManaged,
        TestTargetEnvironmentClass::Ci => TestEvidenceClass::ImportedCiReadOnly,
        TestTargetEnvironmentClass::UnknownRequiresReview => {
            TestEvidenceClass::UnknownRequiresReview
        }
    }
}

fn freshness_for_attempt(attempt: &TestAttemptRecord) -> TestResultFreshnessClass {
    match attempt.imported_ci_projection_class {
        ImportedCiProjectionClass::NotImportedCi => TestResultFreshnessClass::Current,
        ImportedCiProjectionClass::AuthoritativeImportedReadOnly => {
            TestResultFreshnessClass::Imported
        }
        ImportedCiProjectionClass::StaleImportedReadOnly => TestResultFreshnessClass::Stale,
        ImportedCiProjectionClass::FreshLocalReconfirmation => TestResultFreshnessClass::Current,
        ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview => {
            TestResultFreshnessClass::UnknownRequiresReview
        }
    }
}

fn freshness_for_evidence(evidence_class: TestEvidenceClass) -> TestResultFreshnessClass {
    match evidence_class {
        TestEvidenceClass::LiveLocal
        | TestEvidenceClass::LiveContainer
        | TestEvidenceClass::LiveRemote
        | TestEvidenceClass::LiveManaged
        | TestEvidenceClass::FreshLocalConfirmation => TestResultFreshnessClass::Current,
        TestEvidenceClass::ImportedCiReadOnly => TestResultFreshnessClass::Imported,
        TestEvidenceClass::ImportedCiStale => TestResultFreshnessClass::Stale,
        TestEvidenceClass::CachedPrior => TestResultFreshnessClass::Cached,
        TestEvidenceClass::UnknownRequiresReview => TestResultFreshnessClass::UnknownRequiresReview,
    }
}

fn freshness_for_imported_truth(class: ImportedCiTruthClass) -> TestResultFreshnessClass {
    match class {
        ImportedCiTruthClass::NotImported | ImportedCiTruthClass::ConfirmedByLocalRerun => {
            TestResultFreshnessClass::Current
        }
        ImportedCiTruthClass::ImportedCurrentReadOnly
        | ImportedCiTruthClass::LocalConfirmationRequired => TestResultFreshnessClass::Imported,
        ImportedCiTruthClass::ImportedStaleReadOnly => TestResultFreshnessClass::Stale,
        ImportedCiTruthClass::UnknownRequiresReview => {
            TestResultFreshnessClass::UnknownRequiresReview
        }
    }
}

fn canonical_surface_tokens() -> Vec<String> {
    [
        TestIdentitySurface::EditorInlineMarker,
        TestIdentitySurface::TestTreeRow,
        TestIdentitySurface::CliSelector,
        TestIdentitySurface::CommandPalette,
        TestIdentitySurface::ReviewPacket,
        TestIdentitySurface::SupportExport,
        TestIdentitySurface::ImportedCiOverlay,
    ]
    .into_iter()
    .map(|surface| surface.as_str().to_owned())
    .collect()
}

fn selection_origin_for_surface(surface: TestIdentitySurface) -> TestSelectionOrigin {
    match surface {
        TestIdentitySurface::EditorInlineMarker => TestSelectionOrigin::EditorInline,
        TestIdentitySurface::TestTreeRow => TestSelectionOrigin::TestTree,
        TestIdentitySurface::CliSelector => TestSelectionOrigin::CliSelector,
        TestIdentitySurface::CommandPalette => TestSelectionOrigin::CommandPalette,
        TestIdentitySurface::ReviewPacket => TestSelectionOrigin::ReviewPacket,
        TestIdentitySurface::SupportExport => TestSelectionOrigin::SupportExport,
        TestIdentitySurface::ImportedCiOverlay => TestSelectionOrigin::ImportedCiOverlay,
    }
}

fn support_summary_lines(
    items: &[CanonicalTestItem],
    sessions: &[CanonicalTestSession],
    attempts: &[CanonicalTestAttempt],
) -> Vec<String> {
    items
        .iter()
        .map(|item| {
            let session = sessions.iter().find(|session| {
                session
                    .canonical_test_item_refs
                    .iter()
                    .any(|item_ref| item_ref == &item.canonical_test_item_id)
            });
            let attempt_count = session
                .map(|session| session.attempt_refs.len())
                .unwrap_or_default();
            let latest = session
                .and_then(|session| session.latest_attempt_ref.as_deref())
                .and_then(|latest| {
                    attempts
                        .iter()
                        .find(|attempt| attempt.test_attempt_id == latest)
                });
            format!(
                "item={} session={} attempts={} latest={} freshness={} imported_truth={}",
                item.canonical_test_item_id,
                session
                    .map(|session| session.test_session_id.as_str())
                    .unwrap_or("missing"),
                attempt_count,
                latest
                    .map(|attempt| attempt.test_attempt_id.as_str())
                    .unwrap_or("none"),
                latest
                    .map(|attempt| attempt.freshness_token.as_str())
                    .unwrap_or("unknown_requires_review"),
                session
                    .map(|session| session.imported_ci_truth_token.as_str())
                    .unwrap_or("unknown_requires_review")
            )
        })
        .collect()
}

fn escape_selector_value(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '\\' | ':' | '/' | '[' | ']' | '(' | ')' | ',' | '*' | '?' => {
                escaped.push('\\');
                escaped.push(ch);
            }
            ch if ch.is_ascii_whitespace() => {
                escaped.push('\\');
                escaped.push(ch);
            }
            ch if ch.is_control() => {
                escaped.push_str(&format!("\\u{{{:x}}}", ch as u32));
            }
            ch => escaped.push(ch),
        }
    }
    escaped
}

fn stable_token(raw: &str) -> String {
    let mut token = String::new();
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            token.push(ch.to_ascii_lowercase());
        } else if !token.ends_with('_') {
            token.push('_');
        }
    }
    let token = token.trim_matches('_').to_owned();
    if token.is_empty() {
        "unnamed".to_owned()
    } else {
        token
    }
}

fn digest_token(payload: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in payload.bytes() {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("sha256:{hash:064x}")
}
