//! Freeze of the test-item identity, discovery-snapshot, selection-object,
//! session-attempt, verdict, quarantine, and triage-packet matrix for every
//! claimed M5 test-intelligence surface.
//!
//! M5 increasingly depends on trustworthy test discovery, session, retry,
//! coverage, flaky, snapshot, and imported CI evidence across framework packs,
//! notebooks, AI test generation, and review / pipeline flows. Those lanes only
//! stay trustworthy if test items, discovery snapshots, selection objects,
//! session plans, attempt records, verdicts, quarantines, and triage packets are
//! canonical product objects rather than display-name lists and provider
//! dashboards.
//!
//! Where [`crate::testing_identity`] froze the per-record canonical test
//! item / session / attempt *ledger*, [`crate::testing_quality`] froze the
//! coverage / flaky / snapshot / baseline *truth packets*, and
//! [`crate::testing_triage`] froze the watch / flaky / quarantine / trust
//! *triage packets*, this module binds those into a single bounded
//! **qualification matrix**. The matrix is the one canonical answer to "for this
//! claimed M5 test-intelligence surface, what is its test-item identity class,
//! discovery-snapshot class, selection-object class, and session-attempt class —
//! and is the public qualification it claims actually backed by an identified
//! test, discovery, selection, session, verdict, and quarantine truth?"
//!
//! Each [`TestQualificationRow`] reuses the frozen
//! [`TestItemIdentityClass`](crate::testing_identity::TestItemIdentityClass),
//! [`ImportedCiProjectionClass`](crate::tests::ImportedCiProjectionClass), and
//! [`FlakyVerdictState`](crate::tests::FlakyVerdictState) vocabularies rather
//! than minting synonyms, and adds the matrix-level dimensions this freeze owns:
//! [`DiscoverySnapshotClass`], [`SelectionObjectClass`], [`SessionAttemptClass`],
//! and [`ProposalKind`]. The matrix *auto-downgrades*: a claimed row that cannot
//! identify a durable test item, a discovery snapshot, a selection object, or a
//! session-attempt class — or whose verdict still requires review — must carry an
//! `effective_qualification` strictly below its claim, a recorded downgrade
//! trigger, and a precise degraded label, so a test-intelligence claim never
//! outruns the evidence that backs it.
//!
//! [`TestQualificationMatrixPacket::validate`] also refuses a row that lets a
//! display label stand in for durable test identity, collapses a parameterized
//! template into a concrete invocation, hides partial discovery, lets imported or
//! provider-backed results masquerade as live local truth, hides a quarantine or
//! stale coverage behind a generic green state, or lets a snapshot / golden /
//! test-generation proposal bypass the same preview / diff / apply rules used
//! elsewhere.
//!
//! Raw test source, raw provider payloads, raw log bytes, provider cursors,
//! credentials, and raw artifact bodies never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque ids, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json`](../../../../schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json).
//! The contract doc is
//! [`docs/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.md`](../../../../docs/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/`](../../../../fixtures/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::testing_identity::TestItemIdentityClass;
use crate::tests::{FlakyVerdictState, ImportedCiProjectionClass};

/// Stable record-kind tag carried by [`TestQualificationMatrixPacket`].
pub const M5_TEST_QUALIFICATION_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix";

/// Schema version for the test-intelligence qualification matrix.
pub const M5_TEST_QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_TEST_QUALIFICATION_MATRIX_SCHEMA_REF: &str =
    "schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TEST_QUALIFICATION_MATRIX_DOC_REF: &str =
    "docs/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TEST_QUALIFICATION_MATRIX_FIXTURE_DIR: &str =
    "fixtures/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEST_QUALIFICATION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_TEST_QUALIFICATION_MATRIX_SUMMARY_REF: &str =
    "artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.md";

/// One claimed M5 test-intelligence surface a matrix row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIntelligenceSurface {
    /// Framework-pack test explorer / test tree.
    FrameworkTestExplorer,
    /// Notebook test cells and inline notebook test results.
    NotebookTestCells,
    /// AI-assist test-generation proposal surface.
    AiTestGeneration,
    /// Review / pull-request test panel.
    ReviewTestPanel,
    /// Imported CI evidence overlay.
    CiImportOverlay,
    /// Coverage surface (per-target coverage and impact).
    CoverageSurface,
    /// Flaky / quarantine / mute board.
    FlakyQuarantineBoard,
    /// Snapshot / golden / baseline review surface.
    SnapshotGoldenReview,
    /// Support / export projection of the matrix.
    SupportExportProjection,
}

impl TestIntelligenceSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::FrameworkTestExplorer,
        Self::NotebookTestCells,
        Self::AiTestGeneration,
        Self::ReviewTestPanel,
        Self::CiImportOverlay,
        Self::CoverageSurface,
        Self::FlakyQuarantineBoard,
        Self::SnapshotGoldenReview,
        Self::SupportExportProjection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FrameworkTestExplorer => "framework_test_explorer",
            Self::NotebookTestCells => "notebook_test_cells",
            Self::AiTestGeneration => "ai_test_generation",
            Self::ReviewTestPanel => "review_test_panel",
            Self::CiImportOverlay => "ci_import_overlay",
            Self::CoverageSurface => "coverage_surface",
            Self::FlakyQuarantineBoard => "flaky_quarantine_board",
            Self::SnapshotGoldenReview => "snapshot_golden_review",
            Self::SupportExportProjection => "support_export_projection",
        }
    }
}

/// Whether a [`TestItemIdentityClass`] is durable enough to back a public claim.
///
/// Only [`TestItemIdentityClass::Stable`] and
/// [`TestItemIdentityClass::ImportedReadOnly`] back a claim; a row that resolves
/// to remap-review, display-text-only, or unknown must auto-downgrade.
const fn item_identity_backs_claim(class: TestItemIdentityClass) -> bool {
    matches!(
        class,
        TestItemIdentityClass::Stable | TestItemIdentityClass::ImportedReadOnly
    )
}

/// Whether a [`TestItemIdentityClass`] denotes imported / provider-owned truth
/// that must never masquerade as a live local result.
const fn item_identity_is_imported(class: TestItemIdentityClass) -> bool {
    matches!(class, TestItemIdentityClass::ImportedReadOnly)
}

/// Closed discovery-snapshot class vocabulary. Names how a surface establishes
/// its discovered set so a partial or imported discovery never reads as a
/// complete local enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverySnapshotClass {
    /// The local discovery enumerated the full claimed scope.
    CompleteDiscovery,
    /// Discovery is partial; uncovered scope stays visible, not hidden.
    PartialVisibleDiscovery,
    /// Discovery is still streaming; the snapshot is incomplete by construction.
    StreamingDiscovery,
    /// The set is imported from CI or a provider and is read-only locally.
    ProviderImportedDiscovery,
    /// A previously cached snapshot is replayed and is outside its freshness window.
    StaleCachedDiscovery,
}

impl DiscoverySnapshotClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteDiscovery => "complete_discovery",
            Self::PartialVisibleDiscovery => "partial_visible_discovery",
            Self::StreamingDiscovery => "streaming_discovery",
            Self::ProviderImportedDiscovery => "provider_imported_discovery",
            Self::StaleCachedDiscovery => "stale_cached_discovery",
        }
    }

    /// True when the snapshot is not a complete local enumeration and so must keep
    /// its uncovered or imported scope visible.
    pub const fn requires_partial_visibility(self) -> bool {
        !matches!(self, Self::CompleteDiscovery)
    }

    /// True when the snapshot is imported / provider-owned.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ProviderImportedDiscovery)
    }
}

/// Closed selection-object class vocabulary. Names how a surface establishes the
/// portable selection it reruns, debugs, exports, or hands to review, so a
/// display-name list never stands in for a durable selection object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionObjectClass {
    /// A durable selection object keyed by stable test-item identity.
    DurableIdentitySelection,
    /// The currently visible range of discovered items.
    VisibleRangeSelection,
    /// All items matching the active query / filter.
    QueryMatchedSelection,
    /// A provider-scoped selection the provider owns and completes.
    ProviderScopedSelection,
}

impl SelectionObjectClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableIdentitySelection => "durable_identity_selection",
            Self::VisibleRangeSelection => "visible_range_selection",
            Self::QueryMatchedSelection => "query_matched_selection",
            Self::ProviderScopedSelection => "provider_scoped_selection",
        }
    }

    /// True when the provider owns the selection scope.
    pub const fn is_provider_owned(self) -> bool {
        matches!(self, Self::ProviderScopedSelection)
    }
}

/// Closed session-attempt class vocabulary. Names how a surface establishes its
/// session plan and attempt lineage so an imported CI session is never mistaken
/// for a live local rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionAttemptClass {
    /// A live local session with its own attempt records.
    LocalLiveSession,
    /// A rerun-last lineage of attempts over a prior session plan.
    RerunAttemptLineage,
    /// An imported CI session whose attempts are read-only locally.
    ImportedCiSession,
    /// A mixed session reconciling local attempts with imported evidence.
    MixedLocalImportedSession,
}

impl SessionAttemptClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLiveSession => "local_live_session",
            Self::RerunAttemptLineage => "rerun_attempt_lineage",
            Self::ImportedCiSession => "imported_ci_session",
            Self::MixedLocalImportedSession => "mixed_local_imported_session",
        }
    }

    /// True when imported CI evidence participates in this session.
    pub const fn involves_imported(self) -> bool {
        matches!(
            self,
            Self::ImportedCiSession | Self::MixedLocalImportedSession
        )
    }
}

/// Closed proposal-kind vocabulary the freeze owns. Each mutating proposal a
/// surface offers — generated test, accepted snapshot, updated golden, accepted
/// baseline, or applied codemod — declares its kind so it is held to the same
/// preview / diff / apply rules used elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalKind {
    /// Generate a new test (AI-assist or scaffold).
    GenerateTest,
    /// Accept a snapshot mutation.
    AcceptSnapshot,
    /// Update a golden artifact.
    UpdateGolden,
    /// Accept a per-case baseline change.
    AcceptBaseline,
    /// Apply a codemod over test source.
    ApplyCodemod,
}

impl ProposalKind {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GenerateTest => "generate_test",
            Self::AcceptSnapshot => "accept_snapshot",
            Self::UpdateGolden => "update_golden",
            Self::AcceptBaseline => "accept_baseline",
            Self::ApplyCodemod => "apply_codemod",
        }
    }
}

/// One declared proposal a test-intelligence surface offers, with its preview and
/// explicit-apply requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestGenerationProposalDescriptor {
    /// Stable proposal id.
    pub proposal_id: String,
    /// Proposal kind.
    pub proposal_kind: ProposalKind,
    /// True when the proposal mutates source or a checked-in artifact.
    pub mutates_source_or_artifact: bool,
    /// True when the proposal renders a reviewable diff before commit.
    pub requires_preview_diff: bool,
    /// True when the proposal requires an explicit apply step (never auto-applied).
    pub requires_explicit_apply: bool,
}

impl TestGenerationProposalDescriptor {
    /// Whether this proposal must preview before commit. Every test-generation,
    /// snapshot, golden, baseline, or codemod proposal touches source or a
    /// checked-in artifact and so must preview a diff and gate behind an explicit
    /// apply, regardless of which surface offers it.
    pub const fn must_preview(&self) -> bool {
        true
    }

    /// Whether this descriptor satisfies its preview and explicit-apply invariants.
    pub fn is_safe(&self) -> bool {
        !self.proposal_id.trim().is_empty()
            && (!self.must_preview() || self.requires_preview_diff)
            && self.requires_explicit_apply
    }
}

/// Declared selection-object contract for a test-intelligence surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionObjectDeclaration {
    /// Stable selection-object id.
    pub selection_object_id: String,
    /// Identity-basis token (e.g. `stable_node_id_set`, `provider_scope_handle`).
    pub basis_token: String,
    /// True when the selection is portable to a local rerun.
    pub portable_to_rerun: bool,
    /// True when the selection is portable to a CLI / headless selector.
    pub portable_to_cli: bool,
    /// True when the selection is captured as display names only (forbidden).
    pub captures_display_name_only: bool,
    /// True when the selection survives a rediscovery / refresh by stable identity.
    pub survives_rediscovery: bool,
}

impl SelectionObjectDeclaration {
    /// Whether the selection-object declaration is valid: it carries identity,
    /// never captures display names only, survives rediscovery, and is portable to
    /// at least a rerun or a CLI selector.
    pub fn is_valid(&self) -> bool {
        !self.selection_object_id.trim().is_empty()
            && !self.basis_token.trim().is_empty()
            && !self.captures_display_name_only
            && self.survives_rediscovery
            && (self.portable_to_rerun || self.portable_to_cli)
    }
}

/// Declared triage-packet contract for a test-intelligence surface. Re-exports
/// the frozen [`FlakyVerdictState`] so quarantine / mute state stays evidence-based,
/// visible, and exportable rather than hidden behind a generic green state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriagePacketDeclaration {
    /// Stable triage-packet id.
    pub triage_packet_id: String,
    /// Flaky / quarantine / mute state, reusing the frozen verdict vocabulary.
    pub quarantine_state: FlakyVerdictState,
    /// True when the quarantine / mute state is visible and filterable in-product.
    pub quarantine_visible: bool,
    /// True when the quarantine / mute state is exportable.
    pub quarantine_exportable: bool,
    /// True when the quarantine / mute carries renewal / expiry / removal semantics.
    pub has_renewal_or_expiry: bool,
    /// True when the verdict / quarantine state is backed by comparable evidence.
    pub evidence_backed: bool,
}

impl TriagePacketDeclaration {
    /// Whether the triage-packet declaration is valid: it carries identity, keeps
    /// quarantine state visible and exportable, is evidence-backed, and — when the
    /// state is muted or flaky — carries renewal / expiry semantics rather than
    /// hiding indefinitely.
    pub fn is_valid(&self) -> bool {
        let renewal_ok = if matches!(
            self.quarantine_state,
            FlakyVerdictState::Muted | FlakyVerdictState::ReproducedFlaky
        ) {
            self.has_renewal_or_expiry
        } else {
            true
        };
        !self.triage_packet_id.trim().is_empty()
            && self.quarantine_visible
            && self.quarantine_exportable
            && self.evidence_backed
            && renewal_ok
    }
}

/// Closed qualification vocabulary the matrix freezes for claimed rows. Higher
/// means a stronger public claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestMatrixQualificationClass {
    /// Promoted, durable, publicly claimed.
    Stable,
    /// Publicly claimed but still hardening.
    Beta,
    /// Narrow public preview.
    Preview,
    /// Internal / experimental; not a public claim.
    Experimental,
    /// Held below preview pending evidence.
    Held,
    /// Not available on this surface.
    Unavailable,
}

impl TestMatrixQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Held => "held",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether this class is a publicly claimed lane (Stable, Beta, or Preview).
    pub const fn is_claimed(self) -> bool {
        matches!(self, Self::Stable | Self::Beta | Self::Preview)
    }

    /// Ordinal rank used to compare claim severity; higher is a stronger claim, so
    /// a downgrade must move strictly lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Held => 1,
            Self::Experimental => 2,
            Self::Preview => 3,
            Self::Beta => 4,
            Self::Stable => 5,
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a claimed row auto-downgraded
/// below its claim; the chrome quotes the trigger verbatim instead of a generic
/// error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestMatrixDowngradeTrigger {
    /// The test-item identity class could not be identified.
    UnidentifiedTestItem,
    /// The row resolves to a display-text-only identity that cannot back a claim.
    DisplayOnlyTestIdentity,
    /// The discovery-snapshot class could not be identified.
    UnidentifiedDiscoverySnapshot,
    /// The selection-object class could not be identified.
    UnidentifiedSelectionObject,
    /// The session-attempt class could not be identified.
    UnidentifiedSessionAttempt,
    /// The verdict projection still requires review and fails closed.
    ImportedVerdictRequiresReview,
    /// Partial or streaming discovery limited the surface below its claim.
    PartialDiscoveryLimited,
    /// Unresolved quarantine / mute debt narrowed the surface below its claim.
    QuarantineDebtUnresolved,
    /// A provider narrowed the surface below its claim.
    ProviderNarrowed,
    /// An upstream dependency narrowed and dragged this row down with it.
    UpstreamDependencyNarrowed,
}

impl TestMatrixDowngradeTrigger {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnidentifiedTestItem => "unidentified_test_item",
            Self::DisplayOnlyTestIdentity => "display_only_test_identity",
            Self::UnidentifiedDiscoverySnapshot => "unidentified_discovery_snapshot",
            Self::UnidentifiedSelectionObject => "unidentified_selection_object",
            Self::UnidentifiedSessionAttempt => "unidentified_session_attempt",
            Self::ImportedVerdictRequiresReview => "imported_verdict_requires_review",
            Self::PartialDiscoveryLimited => "partial_discovery_limited",
            Self::QuarantineDebtUnresolved => "quarantine_debt_unresolved",
            Self::ProviderNarrowed => "provider_narrowed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// One claimed M5 test-intelligence row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed test-intelligence surface.
    pub surface: TestIntelligenceSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Identified test-item identity class, reusing the frozen identity
    /// vocabulary. `None` means it could not be identified and forces
    /// auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub test_item_identity_class: Option<TestItemIdentityClass>,
    /// Identified discovery-snapshot class. `None` means it could not be
    /// identified and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_snapshot_class: Option<DiscoverySnapshotClass>,
    /// Identified selection-object class. `None` means it could not be identified
    /// and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_object_class: Option<SelectionObjectClass>,
    /// Identified session-attempt class. `None` means it could not be identified
    /// and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_attempt_class: Option<SessionAttemptClass>,
    /// Verdict projection class, reusing the frozen imported-CI vocabulary so an
    /// imported verdict never reads as a live local rerun.
    pub verdict_projection_class: ImportedCiProjectionClass,
    /// Declared selection-object contract.
    pub selection_object: SelectionObjectDeclaration,
    /// Declared triage-packet contract.
    pub triage_packet: TriagePacketDeclaration,
    /// Declared proposals the surface offers (test generation, snapshot, golden,
    /// baseline, codemod).
    pub proposal_descriptors: Vec<TestGenerationProposalDescriptor>,
    /// True when the test item is identified independently of any display name.
    pub identity_independent_of_display_name: bool,
    /// True when a parameterized template stays distinct from its concrete
    /// invocations rather than collapsing into one row identity.
    pub template_distinct_from_invocation: bool,
    /// True when partial / streaming / imported discovery keeps its uncovered scope
    /// visible rather than hiding it behind a complete-looking enumeration.
    pub partial_discovery_stays_visible: bool,
    /// True when imported / provider-backed results are never rendered as live
    /// local truth.
    pub imported_results_not_shown_as_local: bool,
    /// True when quarantine / mute state stays visible and exportable rather than
    /// hidden behind a generic green state.
    pub quarantine_visible_and_exportable: bool,
    /// True when every snapshot / golden / test-generation proposal uses the shared
    /// preview / diff / apply rules.
    pub proposals_use_preview_apply: bool,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: TestMatrixQualificationClass,
    /// Effective qualification after auto-downgrade; equals the claim when every
    /// object dimension is identified, and ranks strictly below it otherwise.
    pub effective_qualification: TestMatrixQualificationClass,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<TestMatrixDowngradeTrigger>,
    /// Precise degraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl TestQualificationRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether every required object dimension (durable test item, discovery
    /// snapshot, selection object, session attempt) is identified and the verdict
    /// does not still require review.
    pub fn identity_complete(&self) -> bool {
        self.test_item_identity_class
            .is_some_and(item_identity_backs_claim)
            && self.discovery_snapshot_class.is_some()
            && self.selection_object_class.is_some()
            && self.session_attempt_class.is_some()
            && self.verdict_projection_class
                != ImportedCiProjectionClass::ImportedCiProjectionUnknownRequiresReview
    }

    /// Whether the row must downgrade below its claim because an object dimension
    /// is missing, the identity is not durable, or the verdict still requires
    /// review.
    pub fn needs_downgrade(&self) -> bool {
        !self.identity_complete()
    }

    /// Whether the effective qualification and downgrade evidence are consistent.
    ///
    /// When every object dimension is present the effective qualification equals
    /// the claim; otherwise it must rank strictly below the claim and carry both a
    /// recorded downgrade trigger and a precise degraded label.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.effective_qualification.rank() < self.claimed_qualification.rank()
                && self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether the test identity is durable and never substituted by a display
    /// label, and a parameterized template stays distinct from its invocations.
    pub fn identity_truth_ok(&self) -> bool {
        self.identity_independent_of_display_name && self.template_distinct_from_invocation
    }

    /// Whether partial / streaming / imported discovery keeps its uncovered scope
    /// visible.
    pub fn discovery_visibility_ok(&self) -> bool {
        match self.discovery_snapshot_class {
            Some(class) if class.requires_partial_visibility() => {
                self.partial_discovery_stays_visible
            }
            _ => true,
        }
    }

    /// Whether imported / provider-backed truth is kept separate from live local
    /// truth when the item identity, discovery, session, or verdict is imported.
    pub fn imported_local_separation_ok(&self) -> bool {
        let imported_involved = self
            .test_item_identity_class
            .is_some_and(item_identity_is_imported)
            || self
                .discovery_snapshot_class
                .is_some_and(DiscoverySnapshotClass::is_imported)
            || self
                .session_attempt_class
                .is_some_and(SessionAttemptClass::involves_imported)
            || matches!(
                self.verdict_projection_class,
                ImportedCiProjectionClass::AuthoritativeImportedReadOnly
                    | ImportedCiProjectionClass::StaleImportedReadOnly
            );
        if imported_involved {
            self.imported_results_not_shown_as_local
        } else {
            true
        }
    }

    /// Whether the quarantine / mute state stays visible and exportable.
    pub fn quarantine_disclosure_ok(&self) -> bool {
        self.quarantine_visible_and_exportable && self.triage_packet.is_valid()
    }

    /// Whether every declared proposal previews a diff and gates behind an explicit
    /// apply — never bypassed because of the surface it runs on.
    pub fn proposal_preview_ok(&self) -> bool {
        self.proposals_use_preview_apply
            && self
                .proposal_descriptors
                .iter()
                .all(TestGenerationProposalDescriptor::is_safe)
    }

    /// Whether the selection object is portable by stable identity.
    pub fn selection_object_ok(&self) -> bool {
        self.selection_object.is_valid()
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.downgrade_consistent()
            && self.identity_truth_ok()
            && self.discovery_visibility_ok()
            && self.imported_local_separation_ok()
            && self.quarantine_disclosure_ok()
            && self.proposal_preview_ok()
            && self.selection_object_ok()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestMatrixGuardrails {
    /// Display labels never stand in for durable test identity.
    pub display_labels_never_substitute_test_identity: bool,
    /// Parameterized templates stay distinct from their concrete invocations.
    pub parameterized_templates_distinct_from_invocations: bool,
    /// Partial discovery stays visible, never hidden behind a complete-looking set.
    pub partial_discovery_stays_visible: bool,
    /// Imported / provider-backed results never masquerade as live local truth.
    pub imported_results_never_masquerade_as_local: bool,
    /// Quarantine / mute states stay visible, filterable, and exportable.
    pub quarantines_visible_filterable_exportable: bool,
    /// Snapshot / golden / test-generation proposals use preview / diff / apply.
    pub proposals_use_preview_diff_apply: bool,
    /// Any row lacking an identified test / discovery / selection / session object
    /// auto-downgrades below its claim.
    pub rows_auto_downgrade_on_unidentified_objects: bool,
}

impl TestMatrixGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.display_labels_never_substitute_test_identity
            && self.parameterized_templates_distinct_from_invocations
            && self.partial_discovery_stays_visible
            && self.imported_results_never_masquerade_as_local
            && self.quarantines_visible_filterable_exportable
            && self.proposals_use_preview_diff_apply
            && self.rows_auto_downgrade_on_unidentified_objects
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestMatrixConsumerProjection {
    /// Product surfaces ingest this matrix instead of cloning test semantics.
    pub product_ingests_matrix: bool,
    /// Docs/help ingests the same matrix.
    pub docs_help_ingests_matrix: bool,
    /// Diagnostics ingests the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// AI/review surfaces ingest the same matrix.
    pub ai_review_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Downgraded rows are visibly labeled below current in every surface.
    pub downgraded_rows_labeled_below_current: bool,
}

impl TestMatrixConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.docs_help_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.ai_review_ingests_matrix
            && self.release_control_ingests_matrix
            && self.downgraded_rows_labeled_below_current
    }
}

/// Evidence freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestMatrixEvidenceFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically downgrades claimed rows.
    pub auto_downgrade_on_stale: bool,
}

/// Constructor input for [`TestQualificationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestQualificationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<TestQualificationRow>,
    /// Guardrail invariants block.
    pub guardrails: TestMatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestMatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: TestMatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe test-intelligence qualification matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestQualificationMatrixPacket {
    /// Record kind; must equal [`M5_TEST_QUALIFICATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEST_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<TestQualificationRow>,
    /// Guardrail invariants block.
    pub guardrails: TestMatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: TestMatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: TestMatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl TestQualificationMatrixPacket {
    /// Builds a test-intelligence qualification matrix packet.
    pub fn new(input: TestQualificationMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_TEST_QUALIFICATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_TEST_QUALIFICATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this matrix.
    pub fn represented_surfaces(&self) -> BTreeSet<TestIntelligenceSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows whose effective qualification was downgraded below its claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the test-intelligence qualification matrix invariants.
    pub fn validate(&self) -> Vec<TestQualificationMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEST_QUALIFICATION_MATRIX_RECORD_KIND {
            violations.push(TestQualificationMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEST_QUALIFICATION_MATRIX_SCHEMA_VERSION {
            violations.push(TestQualificationMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(TestQualificationMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("test qualification matrix packet serializes"),
        ) {
            violations.push(TestQualificationMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("test qualification matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Test-Intelligence Qualification Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} downgraded)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            TestIntelligenceSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_qualification.as_str(),
                row.effective_qualification.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - item=`{}` discovery=`{}` selection=`{}` session=`{}` verdict=`{}`\n",
                row.test_item_identity_class
                    .map_or("unidentified", TestItemIdentityClass::as_str),
                row.discovery_snapshot_class
                    .map_or("unidentified", DiscoverySnapshotClass::as_str),
                row.selection_object_class
                    .map_or("unidentified", SelectionObjectClass::as_str),
                row.session_attempt_class
                    .map_or("unidentified", SessionAttemptClass::as_str),
                row.verdict_projection_class.as_str(),
            ));
            out.push_str(&format!(
                "  - quarantine=`{}` proposals: {}\n",
                row.triage_packet.quarantine_state.as_str(),
                if row.proposal_descriptors.is_empty() {
                    "none".to_owned()
                } else {
                    row.proposal_descriptors
                        .iter()
                        .map(|descriptor| descriptor.proposal_kind.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum TestQualificationMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<TestQualificationMatrixViolation>),
}

impl fmt::Display for TestQualificationMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "test qualification matrix export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "test qualification matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TestQualificationMatrixArtifactError {}

/// Validation failures emitted by [`TestQualificationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TestQualificationMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required test-intelligence surface is represented by no row.
    RequiredSurfaceMissing,
    /// No row demonstrates auto-downgrade on an unidentified object dimension.
    DowngradedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not downgraded below its claim despite a missing object.
    RowNotDowngradedOnUnidentifiedObjects,
    /// A downgraded row lacks a precise degraded label or downgrade trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// A display label stands in for durable test identity.
    DisplayLabelSubstitutesTestIdentity,
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// Partial / streaming / imported discovery was hidden.
    PartialDiscoveryHidden,
    /// Imported / provider-backed results were shown as live local truth.
    ImportedResultsShownAsLocal,
    /// A quarantine / mute state was hidden behind a generic green state.
    QuarantineHidden,
    /// A snapshot / golden / test-generation proposal bypassed preview / apply.
    ProposalBypassesPreview,
    /// A selection object is not portable by stable identity.
    SelectionObjectNotPortable,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl TestQualificationMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnUnidentifiedObjects => {
                "row_not_downgraded_on_unidentified_objects"
            }
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::DisplayLabelSubstitutesTestIdentity => "display_label_substitutes_test_identity",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::PartialDiscoveryHidden => "partial_discovery_hidden",
            Self::ImportedResultsShownAsLocal => "imported_results_shown_as_local",
            Self::QuarantineHidden => "quarantine_hidden",
            Self::ProposalBypassesPreview => "proposal_bypasses_preview",
            Self::SelectionObjectNotPortable => "selection_object_not_portable",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable matrix export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_test_qualification_matrix_export(
) -> Result<TestQualificationMatrixPacket, TestQualificationMatrixArtifactError> {
    let packet: TestQualificationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/support_export.json"
    )))
    .map_err(TestQualificationMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(TestQualificationMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEST_QUALIFICATION_MATRIX_SCHEMA_REF,
        M5_TEST_QUALIFICATION_MATRIX_DOC_REF,
        M5_TEST_QUALIFICATION_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(TestQualificationMatrixViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in TestIntelligenceSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(TestQualificationMatrixViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(TestQualificationMatrixViolation::DowngradedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(TestQualificationMatrixViolation::RowIncomplete);
        }
        if row.needs_downgrade()
            && row.effective_qualification.rank() >= row.claimed_qualification.rank()
        {
            violations
                .push(TestQualificationMatrixViolation::RowNotDowngradedOnUnidentifiedObjects);
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(TestQualificationMatrixViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.identity_independent_of_display_name {
            violations.push(TestQualificationMatrixViolation::DisplayLabelSubstitutesTestIdentity);
        }
        if !row.template_distinct_from_invocation {
            violations.push(TestQualificationMatrixViolation::TemplateCollapsedWithInvocation);
        }
        if !row.discovery_visibility_ok() {
            violations.push(TestQualificationMatrixViolation::PartialDiscoveryHidden);
        }
        if !row.imported_local_separation_ok() {
            violations.push(TestQualificationMatrixViolation::ImportedResultsShownAsLocal);
        }
        if !row.quarantine_disclosure_ok() {
            violations.push(TestQualificationMatrixViolation::QuarantineHidden);
        }
        if !row.proposal_preview_ok() {
            violations.push(TestQualificationMatrixViolation::ProposalBypassesPreview);
        }
        if !row.selection_object_ok() {
            violations.push(TestQualificationMatrixViolation::SelectionObjectNotPortable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(TestQualificationMatrixViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(TestQualificationMatrixViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(TestQualificationMatrixViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &TestQualificationMatrixPacket,
    violations: &mut Vec<TestQualificationMatrixViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(TestQualificationMatrixViolation::EvidenceFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "downgraded"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
