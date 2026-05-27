//! Test-explorer / inline-results / watch-mode / rerun /
//! debug-from-test stabilization truth packet for the M4 stable lane.
//!
//! This module pins how local, remote/helper, container, and notebook
//! test-explorer sessions serialize one canonical truth across the four
//! test-explorer wedges (`test_explorer_identity_truth`,
//! `inline_results_truth`, `watch_mode_truth`,
//! `rerun_debug_from_test_parity`). Tests, inline results, watch loops,
//! rerun, debug-from-test, saved selectors, AI tool plans, and exported
//! test packets MUST operate on durable suite/case/template/invocation
//! identities, partial-discovery records, loaded-versus-known counts,
//! and snapshot-scoped selectors rather than display labels or transient
//! row order. Watch-mode support classes (`live`, `reduced`, `polling`,
//! `unavailable`) MUST be visible per target family and the watch loop
//! MUST stay attributable through a durable session/attempt lineage
//! instead of mutating one anonymous status row.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `test_explorer_stabilization_quality` row cannot prove:
//!
//! - the four test-explorer wedges
//!   (`test_explorer_identity_truth`, `inline_results_truth`,
//!   `watch_mode_truth`, `rerun_debug_from_test_parity`) each have a
//!   structured `wedge_admission` row,
//! - the four test-identity classes (`suite_identity`,
//!   `case_identity`, `template_identity`, `invocation_identity`) each
//!   have a structured `test_identity_admission` row so reviewers
//!   cannot infer identity from display labels,
//! - the three discovery posture classes
//!   (`partial_discovery_record`, `loaded_versus_known_counts`,
//!   `case_enumeration_at_runtime`) each have a structured
//!   `discovery_posture_admission` row so partial discovery and
//!   case-enumeration-at-runtime stay explicit,
//! - the four watch-mode support classes (`live`, `reduced`,
//!   `polling`, `unavailable`) each have a structured
//!   `watch_mode_support_admission` row so the surface cannot collapse
//!   the vocabulary down to "watch on / off",
//! - the three durable selector classes (`durable_id_selector`,
//!   `trait_selector`, `snapshot_scoped_query_selector`) each have a
//!   structured `selector_durability_admission` row so rerun,
//!   debug-from-test, saved selectors, AI tool plans, and exported
//!   test packets all operate on durable IDs / traits / snapshot-scoped
//!   queries instead of display labels or transient row order,
//! - the five consumer-surface bindings (`test_explorer_surface`,
//!   `inline_results_surface`, `watch_mode_surface`, `rerun_surface`,
//!   `debug_from_test_surface`) each carry a
//!   `consumer_surface_binding` row attesting the durable identity,
//!   watch-mode support, and durable-selector vocabularies they are
//!   required to preserve,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every emitted test-explorer envelope and
//!   downstream consumer surface.
//!
//! Every row binds a closed `test_explorer_lane_class`,
//! `test_explorer_row_class`, `support_class`, `wedge_class`,
//! `test_identity_class`, `discovery_posture_class`,
//! `watch_mode_support_class`, `selector_durability_class`,
//! `consumer_surface_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `test_explorer_confidence_class`
//! plus an `evidence_refs` array and a `disclosure_ref` whenever the
//! row is narrowed below launch-stable, declares a non-`none_declared`
//! known limit, or binds a non-`none` downgrade automation.
//!
//! The packet is metadata-only — it never admits raw test source
//! bodies, raw runner stdout/stderr scrollback, raw stack frames, raw
//! command lines, raw process environment bytes, secrets, or ambient
//! credentials past the boundary. A row that claims `launch_stable`
//! while leaving its known limit, downgrade automation, or evidence
//! class unbound is refused; the validator narrows below launch-stable
//! instead of inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`TestExplorerStabilizationTruthPacket`].
pub const TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_RECORD_KIND: &str =
    "stabilize_the_test_explorer_inline_results_watch_mode_truth_stable_packet";

/// Stable record-kind tag for [`TestExplorerStabilizationTruthSupportExport`].
pub const TEST_EXPLORER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "stabilize_the_test_explorer_inline_results_watch_mode_truth_support_export";

/// Integer schema version for the test-explorer stabilization truth packet.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/stabilize_the_test_explorer_inline_results_watch_mode_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/stabilize-the-test-explorer-inline-results-watch-mode.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode";

/// Repo-relative path of the checked-in stable packet.
pub const TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json";

/// Closed test-explorer lane vocabulary. Every required lane MUST have at
/// least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestExplorerLaneClass {
    /// Local-host test-explorer session lane.
    LocalLane,
    /// Remote / helper attach test-explorer session lane.
    RemoteHelperLane,
    /// Container-attached test-explorer session lane.
    ContainerLane,
    /// Notebook-bridge test-explorer session lane (notebook cell tests).
    NotebookLane,
}

impl TestExplorerLaneClass {
    /// Every required test-explorer lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalLane,
        Self::RemoteHelperLane,
        Self::ContainerLane,
        Self::NotebookLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLane => "local_lane",
            Self::RemoteHelperLane => "remote_helper_lane",
            Self::ContainerLane => "container_lane",
            Self::NotebookLane => "notebook_lane",
        }
    }
}

/// Closed test-explorer row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestExplorerRowClass {
    /// The lane's headline test-explorer stabilization quality row.
    TestExplorerStabilizationQuality,
    /// A row admitting one of the four test-explorer wedges.
    WedgeAdmission,
    /// A row admitting one test-identity class (`suite_identity`,
    /// `case_identity`, `template_identity`, `invocation_identity`).
    TestIdentityAdmission,
    /// A row admitting one discovery posture class
    /// (`partial_discovery_record`, `loaded_versus_known_counts`,
    /// `case_enumeration_at_runtime`).
    DiscoveryPostureAdmission,
    /// A row admitting one watch-mode support class (`live`, `reduced`,
    /// `polling`, `unavailable`).
    WatchModeSupportAdmission,
    /// A row admitting one selector durability class
    /// (`durable_id_selector`, `trait_selector`,
    /// `snapshot_scoped_query_selector`).
    SelectorDurabilityAdmission,
    /// A row binding one consumer surface (`test_explorer_surface`,
    /// `inline_results_surface`, `watch_mode_surface`, `rerun_surface`,
    /// `debug_from_test_surface`) and attesting that the surface
    /// preserves the test-identity, watch-mode-support, and
    /// durable-selector vocabularies it is required to preserve.
    ConsumerSurfaceBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object) into emitted test-explorer truth and downstream
    /// consumer surfaces.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl TestExplorerRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerStabilizationQuality => "test_explorer_stabilization_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::TestIdentityAdmission => "test_identity_admission",
            Self::DiscoveryPostureAdmission => "discovery_posture_admission",
            Self::WatchModeSupportAdmission => "watch_mode_support_admission",
            Self::SelectorDurabilityAdmission => "selector_durability_admission",
            Self::ConsumerSurfaceBinding => "consumer_surface_binding",
            Self::LineageAdmission => "lineage_admission",
            Self::KnownLimit => "known_limit",
            Self::DowngradeAutomation => "downgrade_automation",
        }
    }

    /// True when this row class requires a bound wedge.
    pub const fn requires_wedge(self) -> bool {
        matches!(self, Self::WedgeAdmission)
    }

    /// True when this row class requires a bound test identity.
    pub const fn requires_test_identity(self) -> bool {
        matches!(self, Self::TestIdentityAdmission)
    }

    /// True when this row class requires a bound discovery posture.
    pub const fn requires_discovery_posture(self) -> bool {
        matches!(self, Self::DiscoveryPostureAdmission)
    }

    /// True when this row class requires a bound watch-mode support.
    pub const fn requires_watch_mode_support(self) -> bool {
        matches!(self, Self::WatchModeSupportAdmission)
    }

    /// True when this row class requires a bound selector durability.
    pub const fn requires_selector_durability(self) -> bool {
        matches!(self, Self::SelectorDurabilityAdmission)
    }

    /// True when this row class requires a bound consumer surface.
    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a test-explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable.
    LaunchStableBelow,
    /// Row is at beta-grade only.
    BetaGradeOnly,
    /// Row is at preview only.
    PreviewOnly,
    /// Row carries a precisely labeled unsupported gap.
    Unsupported,
    /// Row has no bound support class; this never qualifies stable.
    SupportUnbound,
}

impl SupportClass {
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

/// Closed test-explorer wedge vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `wedge_admission` row for each required
/// wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WedgeClass {
    /// Test-explorer identity truth wedge — stable suite, case, template,
    /// and invocation identities plus partial-discovery records,
    /// loaded-versus-known counts, and explicit
    /// case-enumeration-at-runtime labeling.
    TestExplorerIdentityTruth,
    /// Inline-results truth wedge — inline result rows carry durable
    /// case/invocation identity and a mapping-fidelity badge that
    /// survives export/support packets.
    InlineResultsTruth,
    /// Watch-mode truth wedge — per target family watch-mode support
    /// classification (live/reduced/polling/unavailable) and durable
    /// session/attempt lineage instead of one anonymous status row.
    WatchModeTruth,
    /// Rerun and debug-from-test parity wedge — rerun, debug-from-test,
    /// saved selectors, AI tool plans, and exported test packets all
    /// operate on durable IDs, traits, or snapshot-scoped queries.
    RerunDebugFromTestParity,
    /// The row is not bound to a wedge.
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::TestExplorerIdentityTruth,
        Self::InlineResultsTruth,
        Self::WatchModeTruth,
        Self::RerunDebugFromTestParity,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerIdentityTruth => "test_explorer_identity_truth",
            Self::InlineResultsTruth => "inline_results_truth",
            Self::WatchModeTruth => "watch_mode_truth",
            Self::RerunDebugFromTestParity => "rerun_debug_from_test_parity",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed test-identity vocabulary. Every lane claiming `launch_stable`
/// MUST publish a `test_identity_admission` row for each required
/// identity so rerun, debug-from-test, saved selectors, AI tool plans,
/// and exported test packets all reference durable identities instead of
/// display labels or transient row order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestIdentityClass {
    /// `suite_identity` — durable suite identity (file/path-rooted or
    /// adapter-rooted, stable across reload).
    SuiteIdentity,
    /// `case_identity` — durable case identity (stable across reload,
    /// survives rename via adapter-supplied stable id when available).
    CaseIdentity,
    /// `template_identity` — durable template identity (parameterized
    /// test template) distinct from the per-invocation identity.
    TemplateIdentity,
    /// `invocation_identity` — durable per-invocation identity
    /// (parameterized test invocation, theory data row, generated case).
    InvocationIdentity,
    /// The row is not bound to a test identity.
    NotApplicable,
}

impl TestIdentityClass {
    /// Every required test identity for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::SuiteIdentity,
        Self::CaseIdentity,
        Self::TemplateIdentity,
        Self::InvocationIdentity,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuiteIdentity => "suite_identity",
            Self::CaseIdentity => "case_identity",
            Self::TemplateIdentity => "template_identity",
            Self::InvocationIdentity => "invocation_identity",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed discovery posture vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `discovery_posture_admission` row for
/// each required posture so partial-discovery records, loaded-versus-
/// known counts, and explicit case-enumeration-at-runtime labeling stay
/// observable on the test-explorer surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryPostureClass {
    /// `partial_discovery_record` — partial discovery records that
    /// document which suites/cases are known but not yet loaded.
    PartialDiscoveryRecord,
    /// `loaded_versus_known_counts` — explicit loaded-versus-known
    /// counts surfaced on the explorer chrome.
    LoadedVersusKnownCounts,
    /// `case_enumeration_at_runtime` — explicit labeling that a case
    /// (parameterized invocation, theory row, generated case) is
    /// enumerated at runtime instead of at discovery time.
    CaseEnumerationAtRuntime,
    /// The row is not bound to a discovery posture.
    NotApplicable,
}

impl DiscoveryPostureClass {
    /// Every required discovery posture for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::PartialDiscoveryRecord,
        Self::LoadedVersusKnownCounts,
        Self::CaseEnumerationAtRuntime,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PartialDiscoveryRecord => "partial_discovery_record",
            Self::LoadedVersusKnownCounts => "loaded_versus_known_counts",
            Self::CaseEnumerationAtRuntime => "case_enumeration_at_runtime",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed watch-mode support vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `watch_mode_support_admission` row for
/// each required support class so the surface cannot collapse
/// watch-mode posture down to "watch on / off" or paraphrase the
/// per-target-family classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchModeSupportClass {
    /// `live` — runner reports test results live via an incremental
    /// watcher channel.
    Live,
    /// `reduced` — runner reports a reduced/coalesced subset live; some
    /// fidelity downgrades are surfaced explicitly.
    Reduced,
    /// `polling` — runner is polled on a debounced cadence; the surface
    /// MUST distinguish polling from live.
    Polling,
    /// `unavailable` — watch mode is not supported for the target
    /// family on this lane and the surface MUST disclose the gap.
    Unavailable,
    /// The row is not bound to a watch-mode support class.
    NotApplicable,
}

impl WatchModeSupportClass {
    /// Every required watch-mode support class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::Live,
        Self::Reduced,
        Self::Polling,
        Self::Unavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Reduced => "reduced",
            Self::Polling => "polling",
            Self::Unavailable => "unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed selector durability vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `selector_durability_admission` row
/// for each required durable selector class so rerun, debug-from-test,
/// saved selectors, AI tool plans, and exported test packets operate on
/// durable IDs, traits, or snapshot-scoped queries instead of display
/// labels or transient row order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectorDurabilityClass {
    /// `durable_id_selector` — selector pinned by durable
    /// suite/case/template/invocation id.
    DurableIdSelector,
    /// `trait_selector` — selector pinned by adapter-provided traits
    /// (tags, categories, ownership predicates).
    TraitSelector,
    /// `snapshot_scoped_query_selector` — selector pinned to a query
    /// scoped to a captured discovery snapshot, so the selector stays
    /// reproducible across reload.
    SnapshotScopedQuerySelector,
    /// The row is not bound to a selector durability class.
    NotApplicable,
}

impl SelectorDurabilityClass {
    /// Every required selector durability class for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 3] = [
        Self::DurableIdSelector,
        Self::TraitSelector,
        Self::SnapshotScopedQuerySelector,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableIdSelector => "durable_id_selector",
            Self::TraitSelector => "trait_selector",
            Self::SnapshotScopedQuerySelector => "snapshot_scoped_query_selector",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed consumer-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `consumer_surface_binding` row for
/// each required surface so the test-identity, watch-mode-support, and
/// durable-selector vocabularies survive into product chrome, export
/// bundles, and support packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceBindingClass {
    /// Test-explorer surface (suite / case / template / invocation
    /// tree, partial-discovery records, loaded-versus-known counts,
    /// case-enumeration-at-runtime labels).
    TestExplorerSurface,
    /// Inline-results surface (gutter / editor inline result rows
    /// linked to durable case / invocation identity).
    InlineResultsSurface,
    /// Watch-mode surface (per-target-family watch-mode support
    /// chip, durable session/attempt lineage).
    WatchModeSurface,
    /// Rerun surface (rerun last, rerun failed, rerun by saved
    /// selector — all backed by durable selectors).
    RerunSurface,
    /// Debug-from-test surface (debug-from-test launch backed by the
    /// same durable selectors as rerun).
    DebugFromTestSurface,
    /// The row is not bound to a consumer surface.
    NotApplicable,
}

impl ConsumerSurfaceBindingClass {
    /// Every required consumer surface for a `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::TestExplorerSurface,
        Self::InlineResultsSurface,
        Self::WatchModeSurface,
        Self::RerunSurface,
        Self::DebugFromTestSurface,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerSurface => "test_explorer_surface",
            Self::InlineResultsSurface => "inline_results_surface",
            Self::WatchModeSurface => "watch_mode_surface",
            Self::RerunSurface => "rerun_surface",
            Self::DebugFromTestSurface => "debug_from_test_surface",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this surface MUST attest that it preserves the
    /// test-identity vocabulary.
    pub const fn requires_test_identity_attestation(self) -> bool {
        matches!(
            self,
            Self::TestExplorerSurface
                | Self::InlineResultsSurface
                | Self::WatchModeSurface
                | Self::RerunSurface
                | Self::DebugFromTestSurface
        )
    }

    /// True when this surface MUST attest that it preserves the
    /// watch-mode-support vocabulary.
    pub const fn requires_watch_mode_support_attestation(self) -> bool {
        matches!(self, Self::WatchModeSurface)
    }

    /// True when this surface MUST attest that it preserves the
    /// durable-selector vocabulary.
    pub const fn requires_durable_selector_attestation(self) -> bool {
        matches!(self, Self::RerunSurface | Self::DebugFromTestSurface)
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
    /// The row is backed by a benchmark / fitness-function capture.
    BenchmarkEvidence,
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
            Self::BenchmarkEvidence => "benchmark_evidence",
            Self::DocsDisclosureEvidence => "docs_disclosure_evidence",
            Self::EvidenceUnbound => "evidence_unbound",
        }
    }

    /// True when this evidence class satisfies the evidence-binding invariant.
    pub const fn is_bound(self) -> bool {
        !matches!(self, Self::EvidenceUnbound)
    }
}

/// Closed known-limit vocabulary attached to a test-explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the local subset.
    LocalLaneSubsetOnly,
    /// The lane only certifies the remote/helper subset.
    RemoteHelperSubsetOnly,
    /// The lane only certifies the container subset.
    ContainerSubsetOnly,
    /// The lane only certifies the notebook subset.
    NotebookSubsetOnly,
    /// The lane only certifies a subset of the four test-explorer wedges.
    WedgeAdmissionSubsetOnly,
    /// The lane only certifies a subset of the four test identities.
    TestIdentitySubsetOnly,
    /// The lane only certifies a subset of the three discovery postures.
    DiscoveryPostureSubsetOnly,
    /// The lane only certifies a subset of the four watch-mode support
    /// classes.
    WatchModeSupportSubsetOnly,
    /// The lane only certifies a subset of the three selector durability
    /// classes.
    SelectorDurabilitySubsetOnly,
    /// The lane only certifies a subset of the five consumer surfaces.
    ConsumerSurfaceSubsetOnly,
    /// The lane reports identity attestation skew on one or more
    /// identity-bearing surfaces.
    TestIdentityAttestationSkewDeclared,
    /// The lane reports watch-mode-support attestation skew on the
    /// watch-mode surface.
    WatchModeSupportAttestationSkewDeclared,
    /// The lane reports durable-selector attestation skew on the rerun
    /// or debug-from-test surface.
    DurableSelectorAttestationSkewDeclared,
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
            Self::LocalLaneSubsetOnly => "local_lane_subset_only",
            Self::RemoteHelperSubsetOnly => "remote_helper_subset_only",
            Self::ContainerSubsetOnly => "container_subset_only",
            Self::NotebookSubsetOnly => "notebook_subset_only",
            Self::WedgeAdmissionSubsetOnly => "wedge_admission_subset_only",
            Self::TestIdentitySubsetOnly => "test_identity_subset_only",
            Self::DiscoveryPostureSubsetOnly => "discovery_posture_subset_only",
            Self::WatchModeSupportSubsetOnly => "watch_mode_support_subset_only",
            Self::SelectorDurabilitySubsetOnly => "selector_durability_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::TestIdentityAttestationSkewDeclared => {
                "test_identity_attestation_skew_declared"
            }
            Self::WatchModeSupportAttestationSkewDeclared => {
                "watch_mode_support_attestation_skew_declared"
            }
            Self::DurableSelectorAttestationSkewDeclared => {
                "durable_selector_attestation_skew_declared"
            }
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

/// Closed downgrade-automation vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a required wedge admission is missing.
    AutoNarrowOnWedgeAdmissionGap,
    /// Automatically narrow when a required test-identity admission is
    /// missing.
    AutoNarrowOnTestIdentityGap,
    /// Automatically narrow when a required discovery-posture admission
    /// is missing.
    AutoNarrowOnDiscoveryPostureGap,
    /// Automatically narrow when a required watch-mode support admission
    /// is missing.
    AutoNarrowOnWatchModeSupportGap,
    /// Automatically narrow when a required selector-durability
    /// admission is missing.
    AutoNarrowOnSelectorDurabilityGap,
    /// Automatically narrow when a required consumer-surface binding is
    /// missing.
    AutoNarrowOnConsumerSurfaceGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required test-identity attestation.
    AutoNarrowOnTestIdentityAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required watch-mode-support attestation.
    AutoNarrowOnWatchModeSupportAttestationGap,
    /// Automatically narrow when a consumer-surface row drops a
    /// required durable-selector attestation.
    AutoNarrowOnDurableSelectorAttestationGap,
    /// Automatically narrow when the lineage object breaks
    /// (`execution_context_id` does not thread through emitted truth).
    AutoNarrowOnLineageBreak,
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
            Self::AutoNarrowOnWedgeAdmissionGap => "auto_narrow_on_wedge_admission_gap",
            Self::AutoNarrowOnTestIdentityGap => "auto_narrow_on_test_identity_gap",
            Self::AutoNarrowOnDiscoveryPostureGap => "auto_narrow_on_discovery_posture_gap",
            Self::AutoNarrowOnWatchModeSupportGap => "auto_narrow_on_watch_mode_support_gap",
            Self::AutoNarrowOnSelectorDurabilityGap => {
                "auto_narrow_on_selector_durability_gap"
            }
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnTestIdentityAttestationGap => {
                "auto_narrow_on_test_identity_attestation_gap"
            }
            Self::AutoNarrowOnWatchModeSupportAttestationGap => {
                "auto_narrow_on_watch_mode_support_attestation_gap"
            }
            Self::AutoNarrowOnDurableSelectorAttestationGap => {
                "auto_narrow_on_durable_selector_attestation_gap"
            }
            Self::AutoNarrowOnLineageBreak => "auto_narrow_on_lineage_break",
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

/// Closed confidence-class vocabulary for a test-explorer row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TestExplorerConfidenceClass {
    /// High confidence — the lane can certify launch-stable.
    HighConfidence,
    /// Medium confidence — the lane narrows below launch-stable.
    MediumConfidence,
    /// Low confidence — the lane narrows below launch-stable until evidence grows.
    LowConfidence,
}

impl TestExplorerConfidenceClass {
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
    /// Packet certifies a stable claim.
    Stable,
    /// Packet narrows below stable.
    NarrowedBelowStable,
    /// Packet has a blocker finding.
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

/// Closed validation-finding vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required lane has no row.
    MissingLaneCoverage,
    /// A lane claiming launch_stable is missing a required wedge admission.
    MissingWedgeAdmissionCoverage,
    /// A lane claiming launch_stable is missing a required test-identity admission.
    MissingTestIdentityCoverage,
    /// A lane claiming launch_stable is missing a required discovery-posture admission.
    MissingDiscoveryPostureCoverage,
    /// A lane claiming launch_stable is missing a required watch-mode support admission.
    MissingWatchModeSupportCoverage,
    /// A lane claiming launch_stable is missing a required selector-durability admission.
    MissingSelectorDurabilityCoverage,
    /// A lane claiming launch_stable is missing a required consumer-surface binding.
    MissingConsumerSurfaceCoverage,
    /// A lane claiming launch_stable is missing the required lineage admission row.
    MissingLineageAdmission,
    /// A row has no bound support class.
    MissingSupportClass,
    /// A row has no bound known-limit class.
    MissingKnownLimit,
    /// A row has no bound downgrade-automation class.
    MissingDowngradeAutomation,
    /// A row has no bound evidence class.
    MissingEvidenceClass,
    /// A row claims launch_stable while one or more bindings is unbound.
    LaunchStableWithUnboundBinding,
    /// A row narrowed below launch_stable drops its disclosure ref.
    NarrowedRowMissingDisclosureRef,
    /// A row with a non-`none_declared` known limit drops its disclosure ref.
    KnownLimitMissingDisclosureRef,
    /// A row with a non-`none` downgrade automation drops its disclosure ref.
    DowngradeAutomationMissingDisclosureRef,
    /// A row carries no evidence refs.
    MissingEvidenceRefs,
    /// A wedge-admission row drops its wedge binding.
    WedgeNotApplicable,
    /// A non-wedge row binds a wedge it cannot certify.
    WedgeNotPermittedOnRowClass,
    /// A test-identity-admission row drops its identity binding.
    TestIdentityNotApplicable,
    /// A non-identity row binds a test identity it cannot certify.
    TestIdentityNotPermittedOnRowClass,
    /// A discovery-posture-admission row drops its posture binding.
    DiscoveryPostureNotApplicable,
    /// A non-discovery-posture row binds a posture it cannot certify.
    DiscoveryPostureNotPermittedOnRowClass,
    /// A watch-mode-support-admission row drops its support binding.
    WatchModeSupportNotApplicable,
    /// A non-watch-mode-support row binds a support class it cannot certify.
    WatchModeSupportNotPermittedOnRowClass,
    /// A selector-durability-admission row drops its selector binding.
    SelectorDurabilityNotApplicable,
    /// A non-selector-durability row binds a selector class it cannot certify.
    SelectorDurabilityNotPermittedOnRowClass,
    /// A consumer-surface-binding row drops its surface binding.
    ConsumerSurfaceNotApplicable,
    /// A non-consumer-surface row binds a surface it cannot certify.
    ConsumerSurfaceNotPermittedOnRowClass,
    /// A consumer-surface row fails to attest the test-identity vocabulary it must preserve.
    ConsumerSurfaceMissingTestIdentityAttestation,
    /// A consumer-surface row fails to attest the watch-mode-support vocabulary it must preserve.
    ConsumerSurfaceMissingWatchModeSupportAttestation,
    /// A consumer-surface row fails to attest the durable-selector vocabulary it must preserve.
    ConsumerSurfaceMissingDurableSelectorAttestation,
    /// A lineage-admission row does not bind a lineage object id.
    LineageAdmissionMissingExecutionContextId,
    /// A row admits raw test source bodies, raw stack frames, raw runner
    /// scrollback bodies, raw command lines, or raw process environment
    /// bytes past the boundary.
    RawSourceMaterialPresent,
    /// A row admits secrets past the boundary.
    SecretsPresent,
    /// A row admits ambient authority/credentials past the boundary.
    AmbientAuthorityPresent,
    /// A required consumer projection is missing for this packet.
    MissingConsumerProjection,
    /// A consumer projection remints or drops truth.
    ConsumerProjectionDrift,
    /// A projection collapses the lane vocabulary.
    LaneVocabularyCollapsed,
    /// A projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A projection collapses the support-class vocabulary.
    SupportClassVocabularyCollapsed,
    /// A projection collapses the wedge vocabulary.
    WedgeVocabularyCollapsed,
    /// A projection collapses the test-identity vocabulary.
    TestIdentityVocabularyCollapsed,
    /// A projection collapses the discovery-posture vocabulary.
    DiscoveryPostureVocabularyCollapsed,
    /// A projection collapses the watch-mode-support vocabulary.
    WatchModeSupportVocabularyCollapsed,
    /// A projection collapses the selector-durability vocabulary.
    SelectorDurabilityVocabularyCollapsed,
    /// A projection collapses the consumer-surface vocabulary.
    ConsumerSurfaceVocabularyCollapsed,
    /// A projection collapses the known-limit vocabulary.
    KnownLimitVocabularyCollapsed,
    /// A projection collapses the downgrade-automation vocabulary.
    DowngradeAutomationVocabularyCollapsed,
    /// A projection collapses the evidence-class vocabulary.
    EvidenceClassVocabularyCollapsed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingLaneCoverage => "missing_lane_coverage",
            Self::MissingWedgeAdmissionCoverage => "missing_wedge_admission_coverage",
            Self::MissingTestIdentityCoverage => "missing_test_identity_coverage",
            Self::MissingDiscoveryPostureCoverage => "missing_discovery_posture_coverage",
            Self::MissingWatchModeSupportCoverage => "missing_watch_mode_support_coverage",
            Self::MissingSelectorDurabilityCoverage => "missing_selector_durability_coverage",
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
            Self::TestIdentityNotApplicable => "test_identity_not_applicable",
            Self::TestIdentityNotPermittedOnRowClass => "test_identity_not_permitted_on_row_class",
            Self::DiscoveryPostureNotApplicable => "discovery_posture_not_applicable",
            Self::DiscoveryPostureNotPermittedOnRowClass => {
                "discovery_posture_not_permitted_on_row_class"
            }
            Self::WatchModeSupportNotApplicable => "watch_mode_support_not_applicable",
            Self::WatchModeSupportNotPermittedOnRowClass => {
                "watch_mode_support_not_permitted_on_row_class"
            }
            Self::SelectorDurabilityNotApplicable => "selector_durability_not_applicable",
            Self::SelectorDurabilityNotPermittedOnRowClass => {
                "selector_durability_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceMissingTestIdentityAttestation => {
                "consumer_surface_missing_test_identity_attestation"
            }
            Self::ConsumerSurfaceMissingWatchModeSupportAttestation => {
                "consumer_surface_missing_watch_mode_support_attestation"
            }
            Self::ConsumerSurfaceMissingDurableSelectorAttestation => {
                "consumer_surface_missing_durable_selector_attestation"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::TestIdentityVocabularyCollapsed => "test_identity_vocabulary_collapsed",
            Self::DiscoveryPostureVocabularyCollapsed => "discovery_posture_vocabulary_collapsed",
            Self::WatchModeSupportVocabularyCollapsed => "watch_mode_support_vocabulary_collapsed",
            Self::SelectorDurabilityVocabularyCollapsed => {
                "selector_durability_vocabulary_collapsed"
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

/// Consumer surface that must inherit the packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Test-explorer surface (suite/case/template/invocation tree).
    TestExplorerSurface,
    /// Inline-results surface (gutter / editor inline result rows).
    InlineResultsSurface,
    /// Watch-mode surface (per-target watch-mode chip, lineage row).
    WatchModeSurface,
    /// Rerun surface (rerun last, rerun failed, rerun by saved selector).
    RerunSurface,
    /// Debug-from-test surface.
    DebugFromTestSurface,
    /// AI tool surface (AI tool plans operating on durable selectors).
    AiToolSurface,
    /// CLI / headless inspection surface (`aureline test ...`).
    CliHeadless,
    /// Evidence export bundle surface.
    EvidenceExport,
    /// Support export bundle surface.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
    /// Help/About proof card surface.
    HelpAbout,
    /// Conformance dashboard surface.
    ConformanceDashboard,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 12] = [
        Self::TestExplorerSurface,
        Self::InlineResultsSurface,
        Self::WatchModeSurface,
        Self::RerunSurface,
        Self::DebugFromTestSurface,
        Self::AiToolSurface,
        Self::CliHeadless,
        Self::EvidenceExport,
        Self::SupportExport,
        Self::ReleaseProofIndex,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestExplorerSurface => "test_explorer_surface",
            Self::InlineResultsSurface => "inline_results_surface",
            Self::WatchModeSurface => "watch_mode_surface",
            Self::RerunSurface => "rerun_surface",
            Self::DebugFromTestSurface => "debug_from_test_surface",
            Self::AiToolSurface => "ai_tool_surface",
            Self::CliHeadless => "cli_headless",
            Self::EvidenceExport => "evidence_export",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
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

/// One test-explorer truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestExplorerRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Test-explorer lane this row certifies.
    pub lane_class: TestExplorerLaneClass,
    /// Row class.
    pub row_class: TestExplorerRowClass,
    /// Support class claimed by the row.
    pub support_class: SupportClass,
    /// Wedge bound by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Test identity bound by the row (or `not_applicable`).
    pub test_identity_class: TestIdentityClass,
    /// Discovery posture bound by the row (or `not_applicable`).
    pub discovery_posture_class: DiscoveryPostureClass,
    /// Watch-mode support bound by the row (or `not_applicable`).
    pub watch_mode_support_class: WatchModeSupportClass,
    /// Selector durability bound by the row (or `not_applicable`).
    pub selector_durability_class: SelectorDurabilityClass,
    /// Consumer surface bound by the row (or `not_applicable`).
    pub consumer_surface_class: ConsumerSurfaceBindingClass,
    /// Evidence class backing the row.
    pub evidence_class: EvidenceClass,
    /// Known-limit class disclosed by the row.
    pub known_limit_class: KnownLimitClass,
    /// Downgrade-automation class bound to the row.
    pub downgrade_automation_class: DowngradeAutomationClass,
    /// Confidence class for the row.
    pub confidence_class: TestExplorerConfidenceClass,
    /// Evidence refs cited by the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Optional disclosure ref required whenever the row is not
    /// `launch_stable`, declares a non-`none_declared` known limit,
    /// or binds a non-`none` automation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_ref: Option<String>,
    /// For lineage_admission rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the test-identity vocabulary verbatim.
    #[serde(default)]
    pub attests_test_identity_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the watch-mode-support vocabulary verbatim.
    #[serde(default)]
    pub attests_watch_mode_support_preserved: bool,
    /// For consumer_surface_binding rows, true when the surface
    /// preserves the durable-selector vocabulary verbatim.
    #[serde(default)]
    pub attests_durable_selector_preserved: bool,
    /// True when raw test source bodies, raw runner scrollback bodies,
    /// raw stack frames, raw command lines, or raw process environment
    /// bytes are excluded from this row.
    pub raw_source_material_excluded: bool,
    /// True when secrets are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl TestExplorerRow {
    fn all_bindings_satisfied(&self) -> bool {
        self.support_class.is_bound()
            && self.known_limit_class.is_bound()
            && self.downgrade_automation_class.is_bound()
            && self.evidence_class.is_bound()
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestExplorerConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Test-explorer packet id consumed by the projection.
    pub test_explorer_stabilization_truth_packet_id_ref: String,
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
    /// True when the test-identity vocabulary is preserved verbatim.
    pub preserves_test_identity_vocabulary: bool,
    /// True when the discovery-posture vocabulary is preserved verbatim.
    pub preserves_discovery_posture_vocabulary: bool,
    /// True when the watch-mode-support vocabulary is preserved verbatim.
    pub preserves_watch_mode_support_vocabulary: bool,
    /// True when the selector-durability vocabulary is preserved verbatim.
    pub preserves_selector_durability_vocabulary: bool,
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

impl TestExplorerConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.test_explorer_stabilization_truth_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_test_identity_vocabulary
            && self.preserves_discovery_posture_vocabulary
            && self.preserves_watch_mode_support_vocabulary
            && self.preserves_selector_durability_vocabulary
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

/// Constructor input for [`TestExplorerStabilizationTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestExplorerStabilizationTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Test-explorer lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TestExplorerLaneClass>,
    /// Test-explorer rows.
    #[serde(default)]
    pub rows: Vec<TestExplorerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TestExplorerConsumerProjection>,
    /// Source contracts consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying local, remote/helper, container, and
/// notebook test-explorer / inline-results / watch-mode truth at the M4
/// launch-stable grade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TestExplorerStabilizationTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Test-explorer lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<TestExplorerLaneClass>,
    /// Test-explorer rows.
    #[serde(default)]
    pub rows: Vec<TestExplorerRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<TestExplorerConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl TestExplorerStabilizationTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: TestExplorerStabilizationTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION,
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

    /// Re-validates the packet against stable test-explorer invariants.
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
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
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
        set.into_iter().map(TestExplorerLaneClass::as_str).collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(TestExplorerRowClass::as_str).collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter().map(SupportClass::as_str).collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique test-identity tokens observed across rows.
    pub fn test_identity_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.test_identity_class);
        }
        set.into_iter().map(TestIdentityClass::as_str).collect()
    }

    /// Returns the unique discovery-posture tokens observed across rows.
    pub fn discovery_posture_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.discovery_posture_class);
        }
        set.into_iter().map(DiscoveryPostureClass::as_str).collect()
    }

    /// Returns the unique watch-mode-support tokens observed across rows.
    pub fn watch_mode_support_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.watch_mode_support_class);
        }
        set.into_iter().map(WatchModeSupportClass::as_str).collect()
    }

    /// Returns the unique selector-durability tokens observed across rows.
    pub fn selector_durability_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.selector_durability_class);
        }
        set.into_iter()
            .map(SelectorDurabilityClass::as_str)
            .collect()
    }

    /// Returns the unique consumer-surface tokens observed across rows.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface_class);
        }
        set.into_iter()
            .map(ConsumerSurfaceBindingClass::as_str)
            .collect()
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
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> TestExplorerStabilizationTruthSupportExport {
        TestExplorerStabilizationTruthSupportExport {
            record_kind: TEST_EXPLORER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            test_explorer_stabilization_truth_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            test_explorer_stabilization_truth_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != TEST_EXPLORER_STABILIZATION_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "test-explorer stabilization truth packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "test-explorer stabilization truth packet has the wrong schema version",
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
                FindingKind::MissingLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered test-explorer lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingLaneCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers test-explorer lane {}", lane.as_str()),
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
                        "row {} admits raw test source bodies, raw runner scrollback bodies, raw stack frames, raw command lines, or raw env bytes past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!("row {} admits secrets past the boundary", row.row_id),
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
                    format!("row {} has no bound downgrade-automation class", row.row_id),
                ));
            }
            if !row.evidence_class.is_bound() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingEvidenceClass,
                    FindingSeverity::Blocker,
                    format!("row {} has no bound evidence class", row.row_id),
                ));
            }

            if matches!(row.support_class, SupportClass::LaunchStable)
                && !row.all_bindings_satisfied()
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
            if row
                .downgrade_automation_class
                .requires_explicit_disclosure()
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

            if row.row_class.requires_test_identity()
                && matches!(row.test_identity_class, TestIdentityClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TestIdentityNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a test_identity_admission but has no bound identity",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_test_identity()
                && !matches!(row.test_identity_class, TestIdentityClass::NotApplicable)
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TestIdentityNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds test identity {}; only test_identity_admission rows may bind an identity",
                        row.row_id,
                        row.row_class.as_str(),
                        row.test_identity_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_discovery_posture()
                && matches!(
                    row.discovery_posture_class,
                    DiscoveryPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryPostureNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a discovery_posture_admission but has no bound posture",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_discovery_posture()
                && !matches!(
                    row.discovery_posture_class,
                    DiscoveryPostureClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryPostureNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds discovery posture {}; only discovery_posture_admission rows may bind a posture",
                        row.row_id,
                        row.row_class.as_str(),
                        row.discovery_posture_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_watch_mode_support()
                && matches!(
                    row.watch_mode_support_class,
                    WatchModeSupportClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WatchModeSupportNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a watch_mode_support_admission but has no bound support class",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_watch_mode_support()
                && !matches!(
                    row.watch_mode_support_class,
                    WatchModeSupportClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::WatchModeSupportNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds watch-mode support {}; only watch_mode_support_admission rows may bind a support class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.watch_mode_support_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_selector_durability()
                && matches!(
                    row.selector_durability_class,
                    SelectorDurabilityClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SelectorDurabilityNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a selector_durability_admission but has no bound selector class",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_selector_durability()
                && !matches!(
                    row.selector_durability_class,
                    SelectorDurabilityClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SelectorDurabilityNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds selector durability {}; only selector_durability_admission rows may bind a selector class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.selector_durability_class.as_str()
                    ),
                ));
            }

            if row.row_class.requires_consumer_surface()
                && matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceBindingClass::NotApplicable
                )
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
                && !matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceBindingClass::NotApplicable
                )
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

            if matches!(row.row_class, TestExplorerRowClass::ConsumerSurfaceBinding) {
                if row
                    .consumer_surface_class
                    .requires_test_identity_attestation()
                    && !row.attests_test_identity_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::ConsumerSurfaceMissingTestIdentityAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds consumer surface {} but does not attest test-identity preservation",
                            row.row_id,
                            row.consumer_surface_class.as_str()
                        ),
                    ));
                }
                if row
                    .consumer_surface_class
                    .requires_watch_mode_support_attestation()
                    && !row.attests_watch_mode_support_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::ConsumerSurfaceMissingWatchModeSupportAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds consumer surface {} but does not attest watch-mode-support preservation",
                            row.row_id,
                            row.consumer_surface_class.as_str()
                        ),
                    ));
                }
                if row
                    .consumer_surface_class
                    .requires_durable_selector_attestation()
                    && !row.attests_durable_selector_preserved
                {
                    findings.push(ValidationFinding::new(
                        FindingKind::ConsumerSurfaceMissingDurableSelectorAttestation,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} binds consumer surface {} but does not attest durable-selector preservation",
                            row.row_id,
                            row.consumer_surface_class.as_str()
                        ),
                    ));
                }
            }

            if matches!(row.row_class, TestExplorerRowClass::LineageAdmission)
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

            if matches!(
                row.confidence_class,
                TestExplorerConfidenceClass::LowConfidence
            ) && matches!(row.support_class, SupportClass::LaunchStable)
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

        for lane in &self.covered_lanes {
            let lane_claims_launch = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(
                        row.row_class,
                        TestExplorerRowClass::TestExplorerStabilizationQuality
                    )
                    && matches!(row.support_class, SupportClass::LaunchStable)
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TestExplorerRowClass::WedgeAdmission)
                        && row.wedge_class == wedge
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWedgeAdmissionCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no wedge_admission row for {}",
                            lane.as_str(),
                            wedge.as_str()
                        ),
                    ));
                }
            }

            for identity in TestIdentityClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TestExplorerRowClass::TestIdentityAdmission)
                        && row.test_identity_class == identity
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingTestIdentityCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no test_identity_admission row for {}",
                            lane.as_str(),
                            identity.as_str()
                        ),
                    ));
                }
            }

            for posture in DiscoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TestExplorerRowClass::DiscoveryPostureAdmission
                        )
                        && row.discovery_posture_class == posture
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingDiscoveryPostureCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no discovery_posture_admission row for {}",
                            lane.as_str(),
                            posture.as_str()
                        ),
                    ));
                }
            }

            for support in WatchModeSupportClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TestExplorerRowClass::WatchModeSupportAdmission
                        )
                        && row.watch_mode_support_class == support
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingWatchModeSupportCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no watch_mode_support_admission row for {}",
                            lane.as_str(),
                            support.as_str()
                        ),
                    ));
                }
            }

            for durability in SelectorDurabilityClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            TestExplorerRowClass::SelectorDurabilityAdmission
                        )
                        && row.selector_durability_class == durability
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingSelectorDurabilityCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no selector_durability_admission row for {}",
                            lane.as_str(),
                            durability.as_str()
                        ),
                    ));
                }
            }

            for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, TestExplorerRowClass::ConsumerSurfaceBinding)
                        && row.consumer_surface_class == surface
                        && (!surface.requires_test_identity_attestation()
                            || row.attests_test_identity_preserved)
                        && (!surface.requires_watch_mode_support_attestation()
                            || row.attests_watch_mode_support_preserved)
                        && (!surface.requires_durable_selector_attestation()
                            || row.attests_durable_selector_preserved)
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingConsumerSurfaceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no fully-attested consumer_surface_binding row for {}",
                            lane.as_str(),
                            surface.as_str()
                        ),
                    ));
                }
            }

            let has_lineage = self.rows.iter().any(|row| {
                row.lane_class == *lane
                    && matches!(row.row_class, TestExplorerRowClass::LineageAdmission)
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

        for required_surface in ConsumerSurface::REQUIRED {
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
                        "projection {} does not preserve test-explorer truth",
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
            if !projection.preserves_test_identity_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::TestIdentityVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the test-identity vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_discovery_posture_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryPostureVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the discovery-posture vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_watch_mode_support_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::WatchModeSupportVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the watch-mode-support vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_selector_durability_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::SelectorDurabilityVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the selector-durability vocabulary",
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
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
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
pub struct TestExplorerStabilizationTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub test_explorer_stabilization_truth_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub test_explorer_stabilization_truth_packet: TestExplorerStabilizationTruthPacket,
}

impl TestExplorerStabilizationTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == TEST_EXPLORER_STABILIZATION_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == TEST_EXPLORER_STABILIZATION_TRUTH_SCHEMA_VERSION
            && self.test_explorer_stabilization_truth_packet_id_ref
                == self.test_explorer_stabilization_truth_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self
                .test_explorer_stabilization_truth_packet
                .validate()
                .is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum TestExplorerStabilizationTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for TestExplorerStabilizationTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(
                    formatter,
                    "test-explorer stabilization truth packet parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "test-explorer stabilization truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for TestExplorerStabilizationTruthArtifactError {}

/// Returns the checked-in stable test-explorer stabilization truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_test_explorer_stabilization_truth_packet(
) -> Result<TestExplorerStabilizationTruthPacket, TestExplorerStabilizationTruthArtifactError> {
    let packet: TestExplorerStabilizationTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/stabilize_the_test_explorer_inline_results_watch_mode_truth_packet.json"
    )))
    .map_err(TestExplorerStabilizationTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(TestExplorerStabilizationTruthArtifactError::Validation(
            findings,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        TEST_EXPLORER_STABILIZATION_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        TEST_EXPLORER_STABILIZATION_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: TestExplorerLaneClass) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: TestExplorerRowClass::TestExplorerStabilizationQuality,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        wedge: WedgeClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::WedgeAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: wedge,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWedgeAdmissionGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_wedge_admission_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn test_identity_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        identity: TestIdentityClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:test_identity:{}", identity.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::TestIdentityAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: identity,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnTestIdentityGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_test_identity_gap", doc_ref())),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn discovery_posture_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        posture: DiscoveryPostureClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:discovery_posture:{}", posture.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::DiscoveryPostureAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: posture,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnDiscoveryPostureGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_discovery_posture_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn watch_mode_support_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        support: WatchModeSupportClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:watch_mode_support:{}", support.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::WatchModeSupportAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: support,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnWatchModeSupportGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_watch_mode_support_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn selector_durability_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        durability: SelectorDurabilityClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:selector_durability:{}", durability.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::SelectorDurabilityAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: durability,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnSelectorDurabilityGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_selector_durability_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn consumer_surface_row(
        prefix: &str,
        lane: TestExplorerLaneClass,
        surface: ConsumerSurfaceBindingClass,
    ) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:consumer_surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: TestExplorerRowClass::ConsumerSurfaceBinding,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_consumer_surface_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            attests_test_identity_preserved: surface.requires_test_identity_attestation(),
            attests_watch_mode_support_preserved: surface
                .requires_watch_mode_support_attestation(),
            attests_durable_selector_preserved: surface.requires_durable_selector_attestation(),
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: TestExplorerLaneClass) -> TestExplorerRow {
        TestExplorerRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: TestExplorerRowClass::LineageAdmission,
            support_class: SupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            test_identity_class: TestIdentityClass::NotApplicable,
            discovery_posture_class: DiscoveryPostureClass::NotApplicable,
            watch_mode_support_class: WatchModeSupportClass::NotApplicable,
            selector_durability_class: SelectorDurabilityClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceBindingClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: TestExplorerConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!(
                "exec:m4:{prefix}:test_explorer_lineage"
            )),
            attests_test_identity_preserved: false,
            attests_watch_mode_support_preserved: false,
            attests_durable_selector_preserved: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn projection(surface: ConsumerSurface) -> TestExplorerConsumerProjection {
        TestExplorerConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            test_explorer_stabilization_truth_packet_id_ref:
                "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_test_identity_vocabulary: true,
            preserves_discovery_posture_vocabulary: true,
            preserves_watch_mode_support_vocabulary: true,
            preserves_selector_durability_vocabulary: true,
            preserves_consumer_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(lane: TestExplorerLaneClass, prefix: &str) -> Vec<TestExplorerRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for identity in TestIdentityClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(test_identity_row(prefix, lane, identity));
        }
        for posture in DiscoveryPostureClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(discovery_posture_row(prefix, lane, posture));
        }
        for support in WatchModeSupportClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(watch_mode_support_row(prefix, lane, support));
        }
        for durability in SelectorDurabilityClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(selector_durability_row(prefix, lane, durability));
        }
        for surface in ConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(consumer_surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> TestExplorerStabilizationTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(TestExplorerLaneClass::LocalLane, "local"));
        rows.extend(lane_rows(
            TestExplorerLaneClass::RemoteHelperLane,
            "remote",
        ));
        rows.extend(lane_rows(TestExplorerLaneClass::ContainerLane, "container"));
        rows.extend(lane_rows(TestExplorerLaneClass::NotebookLane, "notebook"));
        TestExplorerStabilizationTruthPacketInput {
            packet_id: "packet:m4:stabilize_the_test_explorer_inline_results_watch_mode"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.stabilize_the_test_explorer_inline_results_watch_mode"
                    .to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_lanes: TestExplorerLaneClass::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection)
                .collect(),
            source_contract_refs: vec![doc_ref()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(TestExplorerLaneClass::LocalLane.as_str(), "local_lane");
        assert_eq!(
            TestExplorerLaneClass::NotebookLane.as_str(),
            "notebook_lane"
        );
        assert_eq!(
            TestExplorerRowClass::TestExplorerStabilizationQuality.as_str(),
            "test_explorer_stabilization_quality"
        );
        assert_eq!(
            TestExplorerRowClass::TestIdentityAdmission.as_str(),
            "test_identity_admission"
        );
        assert_eq!(
            TestExplorerRowClass::WatchModeSupportAdmission.as_str(),
            "watch_mode_support_admission"
        );
        assert_eq!(SupportClass::LaunchStable.as_str(), "launch_stable");
        assert_eq!(
            WedgeClass::TestExplorerIdentityTruth.as_str(),
            "test_explorer_identity_truth"
        );
        assert_eq!(
            WedgeClass::RerunDebugFromTestParity.as_str(),
            "rerun_debug_from_test_parity"
        );
        assert_eq!(TestIdentityClass::CaseIdentity.as_str(), "case_identity");
        assert_eq!(
            TestIdentityClass::InvocationIdentity.as_str(),
            "invocation_identity"
        );
        assert_eq!(WatchModeSupportClass::Live.as_str(), "live");
        assert_eq!(WatchModeSupportClass::Polling.as_str(), "polling");
        assert_eq!(
            SelectorDurabilityClass::DurableIdSelector.as_str(),
            "durable_id_selector"
        );
        assert_eq!(
            SelectorDurabilityClass::SnapshotScopedQuerySelector.as_str(),
            "snapshot_scoped_query_selector"
        );
        assert_eq!(
            ConsumerSurfaceBindingClass::RerunSurface.as_str(),
            "rerun_surface"
        );
        assert_eq!(EvidenceClass::EvidenceUnbound.as_str(), "evidence_unbound");
        assert_eq!(KnownLimitClass::LimitUnbound.as_str(), "limit_unbound");
        assert_eq!(
            DowngradeAutomationClass::AutomationUnbound.as_str(),
            "automation_unbound"
        );
        assert_eq!(
            ConsumerSurface::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(ConsumerSurface::AiToolSurface.as_str(), "ai_tool_surface");
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
        assert_eq!(
            FindingKind::LaunchStableWithUnboundBinding.as_str(),
            "launch_stable_with_unbound_binding"
        );
        assert_eq!(
            FindingKind::ConsumerSurfaceMissingTestIdentityAttestation.as_str(),
            "consumer_surface_missing_test_identity_attestation"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = TestExplorerStabilizationTruthPacket::materialize(sample_input());
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
                "support:m4:stabilize_the_test_explorer_inline_results_watch_mode",
                "2026-05-26T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
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
    fn missing_watch_mode_support_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                TestExplorerRowClass::WatchModeSupportAdmission
            ) && row.watch_mode_support_class == WatchModeSupportClass::Polling
                && row.lane_class == TestExplorerLaneClass::LocalLane)
        });
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingWatchModeSupportCoverage));
    }

    #[test]
    fn missing_selector_durability_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                TestExplorerRowClass::SelectorDurabilityAdmission
            ) && row.selector_durability_class
                == SelectorDurabilityClass::SnapshotScopedQuerySelector
                && row.lane_class == TestExplorerLaneClass::LocalLane)
        });
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingSelectorDurabilityCoverage
        }));
    }

    #[test]
    fn consumer_surface_missing_durable_selector_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, TestExplorerRowClass::ConsumerSurfaceBinding)
                && row.lane_class == TestExplorerLaneClass::LocalLane
                && row.consumer_surface_class == ConsumerSurfaceBindingClass::RerunSurface
            {
                row.attests_durable_selector_preserved = false;
                break;
            }
        }
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::ConsumerSurfaceMissingDurableSelectorAttestation
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, TestExplorerRowClass::LineageAdmission)
                && row.lane_class == TestExplorerLaneClass::LocalLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = SupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
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
            projection.consumer_surface != ConsumerSurface::ConformanceDashboard
        });
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_selector_durability_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerSurface::HelpAbout {
                projection.preserves_selector_durability_vocabulary = false;
            }
        }
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::SelectorDurabilityVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = TestExplorerStabilizationTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
