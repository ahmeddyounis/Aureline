//! Build-target discovery, adapter-confidence label, and target-graph
//! snapshot hardening truth packet for the M4 stable lane.
//!
//! This module pins how the run, test, debug, and target-graph snapshot
//! lanes serialize one canonical build-target truth across the four
//! hardening wedges (`build_target_discovery_truth`,
//! `adapter_confidence_label_truth`, `target_graph_snapshot_truth`,
//! `cross_surface_target_parity_truth`) so the run surface, the test
//! surface, the debug surface, the CLI/headless inspector, the support
//! export bundle, the Help/About proof card, and the conformance
//! dashboard all read one target-graph truth. Surfaces MUST NOT mint
//! local copies, paraphrase adapter-confidence labels, collapse target
//! discovery source / freshness into a single "ok / unknown" bit, or
//! silently widen target identity on rerun, replay, or restore.
//!
//! The packet refuses to certify a launch-stable lane whose
//! `build_target_hardening_quality` row cannot prove:
//!
//! - the four wedges each have a structured `wedge_admission` row so
//!   reviewers can audit discovery, adapter-confidence, snapshot, and
//!   cross-surface posture without support-only knowledge,
//! - the six discovery-source classes (`native_protocol`,
//!   `structured_adapter`, `heuristic_parser`, `imported_metadata`,
//!   `user_declared`, `resolver_unavailable`) each have a structured
//!   `discovery_source_admission` row so the lane discloses where each
//!   target binding originated,
//! - the five discovery-freshness classes (`fresh_probe`,
//!   `recent_within_session`, `imported_authoritative`, `stale_imported`,
//!   `unknown`) each have a structured `discovery_freshness_admission`
//!   row so freshness never collapses into a single "ok" badge,
//! - the five adapter-confidence label classes
//!   (`adapter_authoritative_match`, `adapter_probed_consistent`,
//!   `adapter_probed_divergent`, `adapter_inferred_from_session`,
//!   `adapter_unreachable`) each have a structured
//!   `adapter_confidence_label_admission` row so the labels survive
//!   verbatim into export and support packets,
//! - the five target-graph snapshot classes (`live_snapshot`,
//!   `session_cached_snapshot`, `imported_snapshot`, `archived_snapshot`,
//!   `snapshot_unavailable`) each have a structured
//!   `target_graph_snapshot_admission` row so snapshot provenance stays
//!   explicit through restore and replay,
//! - each of the seven consumer surfaces (run, test, debug,
//!   CLI/headless inspect, support export, Help/About,
//!   conformance dashboard) has a `consumer_surface_binding` row
//!   attesting it reads this packet verbatim,
//! - one stable `execution_context_id` (or equivalent lineage object)
//!   threads through every emitted target-graph snapshot via a
//!   `lineage_admission` row.
//!
//! Every row binds closed `build_target_hardening_lane_class`,
//! `build_target_hardening_row_class`, `support_class`, `wedge_class`,
//! `discovery_source_class`, `discovery_freshness_class`,
//! `adapter_confidence_label_class`, `target_graph_snapshot_class`,
//! `consumer_surface_class`, `evidence_class`, `known_limit_class`,
//! `downgrade_automation_class`, and `confidence_class` vocabularies plus
//! an `evidence_refs` array and a `disclosure_ref` whenever the row is
//! narrowed below launch-stable, declares a non-`none_declared` known
//! limit, or binds a non-`none` downgrade automation.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! discovery payloads, raw command lines, raw adapter handshake bodies,
//! raw process environment bytes, secrets, or ambient credentials past
//! the boundary. A row that claims `launch_stable` while leaving its
//! known limit, downgrade automation, or evidence class unbound is
//! refused; the validator narrows below launch-stable instead of
//! inheriting an adjacent certified row.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`BuildTargetHardeningTruthPacket`].
pub const BUILD_TARGET_HARDENING_TRUTH_PACKET_RECORD_KIND: &str =
    "harden_build_target_discovery_adapter_confidence_labels_and_truth_stable_packet";

/// Stable record-kind tag for [`BuildTargetHardeningTruthSupportExport`].
pub const BUILD_TARGET_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "harden_build_target_discovery_adapter_confidence_labels_and_truth_support_export";

/// Integer schema version for the build-target hardening truth packet.
pub const BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const BUILD_TARGET_HARDENING_TRUTH_SCHEMA_REF: &str =
    "schemas/runtime/harden_build_target_discovery_adapter_confidence_labels_and_truth.schema.json";

/// Repo-relative path of the reviewer contract doc.
pub const BUILD_TARGET_HARDENING_TRUTH_DOC_REF: &str =
    "docs/runtime/m4/harden-build-target-discovery-adapter-confidence-labels-and.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const BUILD_TARGET_HARDENING_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/runtime/m4/harden-build-target-discovery-adapter-confidence-labels-and.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const BUILD_TARGET_HARDENING_TRUTH_FIXTURE_DIR: &str =
    "fixtures/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and";

/// Repo-relative path of the checked-in stable packet.
pub const BUILD_TARGET_HARDENING_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json";

/// Closed build-target hardening lane vocabulary. Every required lane
/// MUST have at least one row in any stable packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTargetHardeningLaneClass {
    /// Run-dispatch lane: run targets exposed by the runtime to the run
    /// surface and rerun flows.
    RunLane,
    /// Test-dispatch lane: test targets exposed by the runtime to the
    /// test explorer, inline results, and rerun flows.
    TestLane,
    /// Debug-dispatch lane: debug targets exposed by the runtime to the
    /// debug surface, adapter negotiation, and attach/launch flows.
    DebugLane,
    /// Target-graph snapshot lane: archived / restored / imported
    /// target-graph snapshots that downstream surfaces consume after the
    /// live discovery surface is gone.
    TargetGraphSnapshotLane,
}

impl BuildTargetHardeningLaneClass {
    /// Every required build-target hardening lane, in declaration order.
    pub const REQUIRED: [Self; 4] = [
        Self::RunLane,
        Self::TestLane,
        Self::DebugLane,
        Self::TargetGraphSnapshotLane,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunLane => "run_lane",
            Self::TestLane => "test_lane",
            Self::DebugLane => "debug_lane",
            Self::TargetGraphSnapshotLane => "target_graph_snapshot_lane",
        }
    }
}

/// Closed row vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTargetHardeningRowClass {
    /// The lane's headline build-target hardening qualification row.
    BuildTargetHardeningQuality,
    /// A row admitting one of the four required wedges
    /// (`build_target_discovery_truth`, `adapter_confidence_label_truth`,
    /// `target_graph_snapshot_truth`,
    /// `cross_surface_target_parity_truth`).
    WedgeAdmission,
    /// A row admitting one of the six discovery-source classes.
    DiscoverySourceAdmission,
    /// A row admitting one of the five discovery-freshness classes.
    DiscoveryFreshnessAdmission,
    /// A row admitting one of the five adapter-confidence label classes.
    AdapterConfidenceLabelAdmission,
    /// A row admitting one of the five target-graph snapshot classes.
    TargetGraphSnapshotAdmission,
    /// A row binding one consumer surface (run, test, debug,
    /// CLI/headless inspect, support export, Help/About, conformance
    /// dashboard).
    ConsumerSurfaceBinding,
    /// A row binding the stable `execution_context_id` (or equivalent
    /// lineage object reference) into emitted target-graph snapshots,
    /// support packets, and evidence exports.
    LineageAdmission,
    /// Disclosed known-limit row attached to a lane.
    KnownLimit,
    /// Downgrade-automation rule row attached to a lane.
    DowngradeAutomation,
}

impl BuildTargetHardeningRowClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildTargetHardeningQuality => "build_target_hardening_quality",
            Self::WedgeAdmission => "wedge_admission",
            Self::DiscoverySourceAdmission => "discovery_source_admission",
            Self::DiscoveryFreshnessAdmission => "discovery_freshness_admission",
            Self::AdapterConfidenceLabelAdmission => "adapter_confidence_label_admission",
            Self::TargetGraphSnapshotAdmission => "target_graph_snapshot_admission",
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

    /// True when this row class requires a bound discovery-source token.
    pub const fn requires_discovery_source(self) -> bool {
        matches!(self, Self::DiscoverySourceAdmission)
    }

    /// True when this row class requires a bound discovery-freshness token.
    pub const fn requires_discovery_freshness(self) -> bool {
        matches!(self, Self::DiscoveryFreshnessAdmission)
    }

    /// True when this row class requires a bound adapter-confidence label token.
    pub const fn requires_adapter_confidence_label(self) -> bool {
        matches!(self, Self::AdapterConfidenceLabelAdmission)
    }

    /// True when this row class requires a bound target-graph snapshot token.
    pub const fn requires_target_graph_snapshot(self) -> bool {
        matches!(self, Self::TargetGraphSnapshotAdmission)
    }

    /// True when this row class requires a bound consumer-surface token.
    pub const fn requires_consumer_surface(self) -> bool {
        matches!(self, Self::ConsumerSurfaceBinding)
    }
}

/// Closed support-class vocabulary applied to a build-target hardening row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildTargetHardeningSupportClass {
    /// Row claims M4 launch-stable grade for the lane.
    LaunchStable,
    /// Row is intentionally narrowed below launch-stable; the narrowing
    /// is disclosed.
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

impl BuildTargetHardeningSupportClass {
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
    /// Build-target discovery exposes source class, freshness, and
    /// authority on every certified row instead of inferring a single
    /// "ok / unknown" bit.
    BuildTargetDiscoveryTruth,
    /// Adapter-confidence labels are preserved verbatim across run,
    /// test, debug, CLI/headless, support export, and Help/About
    /// surfaces.
    AdapterConfidenceLabelTruth,
    /// Target-graph snapshots are reproducible, lineage-bound, and
    /// honest about live vs. cached vs. imported vs. archived state.
    TargetGraphSnapshotTruth,
    /// Run, test, and debug surfaces see one shared target identity and
    /// target-graph; no surface may fork a local copy.
    CrossSurfaceTargetParityTruth,
    /// The row is not bound to a wedge (non-wedge row classes).
    NotApplicable,
}

impl WedgeClass {
    /// Every required wedge per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 4] = [
        Self::BuildTargetDiscoveryTruth,
        Self::AdapterConfidenceLabelTruth,
        Self::TargetGraphSnapshotTruth,
        Self::CrossSurfaceTargetParityTruth,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildTargetDiscoveryTruth => "build_target_discovery_truth",
            Self::AdapterConfidenceLabelTruth => "adapter_confidence_label_truth",
            Self::TargetGraphSnapshotTruth => "target_graph_snapshot_truth",
            Self::CrossSurfaceTargetParityTruth => "cross_surface_target_parity_truth",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed discovery-source vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `discovery_source_admission` row for
/// each source class so the lane discloses where each target binding
/// originated. The vocabulary is aligned with the canonical
/// [`crate::DiscoverySourceClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoverySourceClass {
    /// Target was minted by a native runtime / language host / DAP.
    NativeProtocol,
    /// Target came from a typed adapter parsing a structured manifest.
    StructuredAdapter,
    /// Target was inferred by a heuristic / regex / fallback parser.
    HeuristicParser,
    /// Target was lifted from imported CI / external metadata.
    ImportedMetadata,
    /// Target was declared by the user (override, saved profile).
    UserDeclared,
    /// The discovery layer was unavailable; protected dispatch refused.
    ResolverUnavailable,
    /// The row is not bound to a discovery source (non-source row classes).
    NotApplicable,
}

impl DiscoverySourceClass {
    /// Every required discovery-source class per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 6] = [
        Self::NativeProtocol,
        Self::StructuredAdapter,
        Self::HeuristicParser,
        Self::ImportedMetadata,
        Self::UserDeclared,
        Self::ResolverUnavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NativeProtocol => "native_protocol",
            Self::StructuredAdapter => "structured_adapter",
            Self::HeuristicParser => "heuristic_parser",
            Self::ImportedMetadata => "imported_metadata",
            Self::UserDeclared => "user_declared",
            Self::ResolverUnavailable => "resolver_unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed discovery-freshness vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `discovery_freshness_admission` row for
/// each freshness class so freshness never collapses into a single "ok"
/// badge. The vocabulary is aligned with the canonical
/// [`crate::DiscoveryFreshnessClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiscoveryFreshnessClass {
    /// Target was probed in the current resolver session and matched.
    FreshProbe,
    /// Target was probed earlier in this session and still trusted.
    RecentWithinSession,
    /// Target was imported from an authoritative external source.
    ImportedAuthoritative,
    /// Target was imported but the resolver observed drift / staleness.
    StaleImported,
    /// Freshness cannot be determined; treat as unsafe for dispatch.
    Unknown,
    /// The row is not bound to a freshness class (non-freshness row classes).
    NotApplicable,
}

impl DiscoveryFreshnessClass {
    /// Every required freshness class per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::FreshProbe,
        Self::RecentWithinSession,
        Self::ImportedAuthoritative,
        Self::StaleImported,
        Self::Unknown,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshProbe => "fresh_probe",
            Self::RecentWithinSession => "recent_within_session",
            Self::ImportedAuthoritative => "imported_authoritative",
            Self::StaleImported => "stale_imported",
            Self::Unknown => "unknown",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed adapter-confidence label vocabulary. Every lane claiming
/// `launch_stable` MUST publish an `adapter_confidence_label_admission`
/// row for each label class so the labels survive verbatim into export
/// and support packets. Aligned with [`crate::AdapterConfidenceClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterConfidenceLabelClass {
    /// Adapter reported an authoritative match.
    AdapterAuthoritativeMatch,
    /// Adapter probe was consistent.
    AdapterProbedConsistent,
    /// Adapter probe diverged.
    AdapterProbedDivergent,
    /// Adapter inferred the target from session context only.
    AdapterInferredFromSession,
    /// Adapter was unreachable; the label MUST be carried as such.
    AdapterUnreachable,
    /// The row is not bound to an adapter-confidence label (non-label row classes).
    NotApplicable,
}

impl AdapterConfidenceLabelClass {
    /// Every required adapter-confidence label per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::AdapterAuthoritativeMatch,
        Self::AdapterProbedConsistent,
        Self::AdapterProbedDivergent,
        Self::AdapterInferredFromSession,
        Self::AdapterUnreachable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterAuthoritativeMatch => "adapter_authoritative_match",
            Self::AdapterProbedConsistent => "adapter_probed_consistent",
            Self::AdapterProbedDivergent => "adapter_probed_divergent",
            Self::AdapterInferredFromSession => "adapter_inferred_from_session",
            Self::AdapterUnreachable => "adapter_unreachable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed target-graph snapshot vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `target_graph_snapshot_admission` row
/// for each class so snapshot provenance stays explicit through restore
/// and replay rather than collapsing into a single "snapshot ok" bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetGraphSnapshotClass {
    /// Snapshot was actively resolved this session against live discovery.
    LiveSnapshot,
    /// Snapshot was cached earlier in this session; resolver still trusts it.
    SessionCachedSnapshot,
    /// Snapshot was imported from an authoritative external source.
    ImportedSnapshot,
    /// Snapshot was read from archive / history.
    ArchivedSnapshot,
    /// Snapshot could not be produced; downstream dispatch is refused.
    SnapshotUnavailable,
    /// The row is not bound to a snapshot class (non-snapshot row classes).
    NotApplicable,
}

impl TargetGraphSnapshotClass {
    /// Every required snapshot class per `launch_stable` lane.
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 5] = [
        Self::LiveSnapshot,
        Self::SessionCachedSnapshot,
        Self::ImportedSnapshot,
        Self::ArchivedSnapshot,
        Self::SnapshotUnavailable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveSnapshot => "live_snapshot",
            Self::SessionCachedSnapshot => "session_cached_snapshot",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ArchivedSnapshot => "archived_snapshot",
            Self::SnapshotUnavailable => "snapshot_unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Closed consumer-surface vocabulary. Every lane claiming
/// `launch_stable` MUST publish a `consumer_surface_binding` row for
/// each surface so the run surface, test surface, debug surface,
/// CLI/headless inspector, support export bundle, Help/About proof
/// card, and conformance dashboard all read this packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Run surface (run target picker, run cards).
    RunSurface,
    /// Test surface (test explorer, inline results).
    TestSurface,
    /// Debug surface (debug session panel, adapter chips).
    DebugSurface,
    /// CLI / headless inspect surface (`aureline runtime inspect`).
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
    pub const REQUIRED_FOR_LAUNCH_STABLE: [Self; 7] = [
        Self::RunSurface,
        Self::TestSurface,
        Self::DebugSurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunSurface => "run_surface",
            Self::TestSurface => "test_surface",
            Self::DebugSurface => "debug_surface",
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

/// Closed known-limit vocabulary attached to a build-target hardening row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No known limit beyond canonical truth.
    NoneDeclared,
    /// The lane only certifies the run-dispatch subset.
    RunSubsetOnly,
    /// The lane only certifies the test-dispatch subset.
    TestSubsetOnly,
    /// The lane only certifies the debug-dispatch subset.
    DebugSubsetOnly,
    /// The lane only certifies the target-graph snapshot subset.
    TargetGraphSnapshotSubsetOnly,
    /// The lane only certifies a subset of the four required wedges.
    WedgeSubsetOnly,
    /// The lane only certifies a subset of the discovery-source classes.
    DiscoverySourceSubsetOnly,
    /// The lane only certifies a subset of the discovery-freshness classes.
    DiscoveryFreshnessSubsetOnly,
    /// The lane only certifies a subset of the adapter-confidence labels.
    AdapterConfidenceLabelSubsetOnly,
    /// The lane only certifies a subset of the target-graph snapshot classes.
    TargetGraphSnapshotClassSubsetOnly,
    /// The lane only certifies a subset of the seven consumer surfaces.
    ConsumerSurfaceSubsetOnly,
    /// The lane certifies an unsupported discovery / adapter gap.
    UnsupportedDiscoveryOrAdapter,
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
            Self::RunSubsetOnly => "run_subset_only",
            Self::TestSubsetOnly => "test_subset_only",
            Self::DebugSubsetOnly => "debug_subset_only",
            Self::TargetGraphSnapshotSubsetOnly => "target_graph_snapshot_subset_only",
            Self::WedgeSubsetOnly => "wedge_subset_only",
            Self::DiscoverySourceSubsetOnly => "discovery_source_subset_only",
            Self::DiscoveryFreshnessSubsetOnly => "discovery_freshness_subset_only",
            Self::AdapterConfidenceLabelSubsetOnly => "adapter_confidence_label_subset_only",
            Self::TargetGraphSnapshotClassSubsetOnly => "target_graph_snapshot_class_subset_only",
            Self::ConsumerSurfaceSubsetOnly => "consumer_surface_subset_only",
            Self::UnsupportedDiscoveryOrAdapter => "unsupported_discovery_or_adapter",
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

/// Closed downgrade-automation vocabulary attached to a build-target row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeAutomationClass {
    /// No downgrade automation is required for the row.
    None,
    /// Automatically narrow when a discovery-source admission is unbound.
    AutoNarrowOnDiscoverySourceGap,
    /// Automatically narrow when a discovery-freshness admission is unbound.
    AutoNarrowOnDiscoveryFreshnessGap,
    /// Automatically narrow when an adapter-confidence label admission is unbound.
    AutoNarrowOnAdapterConfidenceLabelGap,
    /// Automatically narrow when a target-graph snapshot admission is unbound.
    AutoNarrowOnTargetGraphSnapshotGap,
    /// Automatically narrow when the consumer-surface binding is missing.
    AutoNarrowOnConsumerSurfaceGap,
    /// Automatically narrow when target identity drifts between run,
    /// test, debug, and snapshot surfaces.
    AutoNarrowOnCrossSurfaceTargetDrift,
    /// Automatically narrow when the lineage object breaks (no
    /// `execution_context_id` binding survives across snapshots, support
    /// packets, or evidence exports).
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
            Self::AutoNarrowOnDiscoverySourceGap => "auto_narrow_on_discovery_source_gap",
            Self::AutoNarrowOnDiscoveryFreshnessGap => "auto_narrow_on_discovery_freshness_gap",
            Self::AutoNarrowOnAdapterConfidenceLabelGap => {
                "auto_narrow_on_adapter_confidence_label_gap"
            }
            Self::AutoNarrowOnTargetGraphSnapshotGap => "auto_narrow_on_target_graph_snapshot_gap",
            Self::AutoNarrowOnConsumerSurfaceGap => "auto_narrow_on_consumer_surface_gap",
            Self::AutoNarrowOnCrossSurfaceTargetDrift => {
                "auto_narrow_on_cross_surface_target_drift"
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

/// Closed confidence-class vocabulary for a build-target hardening row.
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

/// Closed validation-finding vocabulary for the build-target hardening
/// truth packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    WrongRecordKind,
    WrongSchemaVersion,
    MissingIdentity,
    MissingHardeningLaneCoverage,
    MissingWedgeCoverage,
    MissingDiscoverySourceCoverage,
    MissingDiscoveryFreshnessCoverage,
    MissingAdapterConfidenceLabelCoverage,
    MissingTargetGraphSnapshotCoverage,
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
    DiscoverySourceNotApplicable,
    DiscoverySourceNotPermittedOnRowClass,
    DiscoveryFreshnessNotApplicable,
    DiscoveryFreshnessNotPermittedOnRowClass,
    AdapterConfidenceLabelNotApplicable,
    AdapterConfidenceLabelNotPermittedOnRowClass,
    TargetGraphSnapshotNotApplicable,
    TargetGraphSnapshotNotPermittedOnRowClass,
    ConsumerSurfaceNotApplicable,
    ConsumerSurfaceNotPermittedOnRowClass,
    LineageAdmissionMissingExecutionContextId,
    CrossSurfaceTargetParityNotAttested,
    RawSourceMaterialPresent,
    SecretsPresent,
    AmbientAuthorityPresent,
    MissingConsumerProjection,
    ConsumerProjectionDrift,
    LaneVocabularyCollapsed,
    RowClassVocabularyCollapsed,
    SupportClassVocabularyCollapsed,
    WedgeVocabularyCollapsed,
    DiscoverySourceVocabularyCollapsed,
    DiscoveryFreshnessVocabularyCollapsed,
    AdapterConfidenceLabelVocabularyCollapsed,
    TargetGraphSnapshotVocabularyCollapsed,
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
            Self::MissingHardeningLaneCoverage => "missing_hardening_lane_coverage",
            Self::MissingWedgeCoverage => "missing_wedge_coverage",
            Self::MissingDiscoverySourceCoverage => "missing_discovery_source_coverage",
            Self::MissingDiscoveryFreshnessCoverage => "missing_discovery_freshness_coverage",
            Self::MissingAdapterConfidenceLabelCoverage => {
                "missing_adapter_confidence_label_coverage"
            }
            Self::MissingTargetGraphSnapshotCoverage => "missing_target_graph_snapshot_coverage",
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
            Self::DiscoverySourceNotApplicable => "discovery_source_not_applicable",
            Self::DiscoverySourceNotPermittedOnRowClass => {
                "discovery_source_not_permitted_on_row_class"
            }
            Self::DiscoveryFreshnessNotApplicable => "discovery_freshness_not_applicable",
            Self::DiscoveryFreshnessNotPermittedOnRowClass => {
                "discovery_freshness_not_permitted_on_row_class"
            }
            Self::AdapterConfidenceLabelNotApplicable => "adapter_confidence_label_not_applicable",
            Self::AdapterConfidenceLabelNotPermittedOnRowClass => {
                "adapter_confidence_label_not_permitted_on_row_class"
            }
            Self::TargetGraphSnapshotNotApplicable => "target_graph_snapshot_not_applicable",
            Self::TargetGraphSnapshotNotPermittedOnRowClass => {
                "target_graph_snapshot_not_permitted_on_row_class"
            }
            Self::ConsumerSurfaceNotApplicable => "consumer_surface_not_applicable",
            Self::ConsumerSurfaceNotPermittedOnRowClass => {
                "consumer_surface_not_permitted_on_row_class"
            }
            Self::LineageAdmissionMissingExecutionContextId => {
                "lineage_admission_missing_execution_context_id"
            }
            Self::CrossSurfaceTargetParityNotAttested => "cross_surface_target_parity_not_attested",
            Self::RawSourceMaterialPresent => "raw_source_material_present",
            Self::SecretsPresent => "secrets_present",
            Self::AmbientAuthorityPresent => "ambient_authority_present",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::LaneVocabularyCollapsed => "lane_vocabulary_collapsed",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::SupportClassVocabularyCollapsed => "support_class_vocabulary_collapsed",
            Self::WedgeVocabularyCollapsed => "wedge_vocabulary_collapsed",
            Self::DiscoverySourceVocabularyCollapsed => "discovery_source_vocabulary_collapsed",
            Self::DiscoveryFreshnessVocabularyCollapsed => {
                "discovery_freshness_vocabulary_collapsed"
            }
            Self::AdapterConfidenceLabelVocabularyCollapsed => {
                "adapter_confidence_label_vocabulary_collapsed"
            }
            Self::TargetGraphSnapshotVocabularyCollapsed => {
                "target_graph_snapshot_vocabulary_collapsed"
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

/// Consumer surface that must inherit the build-target hardening packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerProjectionSurface {
    /// Run surface (run target picker, run cards).
    RunSurface,
    /// Test surface (test explorer, inline results).
    TestSurface,
    /// Debug surface (debug session panel, adapter chips).
    DebugSurface,
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
    pub const REQUIRED: [Self; 7] = [
        Self::RunSurface,
        Self::TestSurface,
        Self::DebugSurface,
        Self::CliHeadlessInspect,
        Self::SupportExport,
        Self::HelpAbout,
        Self::ConformanceDashboard,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RunSurface => "run_surface",
            Self::TestSurface => "test_surface",
            Self::DebugSurface => "debug_surface",
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

/// One build-target hardening truth row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTargetHardeningRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Lane this row certifies.
    pub lane_class: BuildTargetHardeningLaneClass,
    /// Row class.
    pub row_class: BuildTargetHardeningRowClass,
    /// Support class claimed by the row.
    pub support_class: BuildTargetHardeningSupportClass,
    /// Wedge certified by the row (or `not_applicable`).
    pub wedge_class: WedgeClass,
    /// Discovery source certified by the row (or `not_applicable`).
    pub discovery_source_class: DiscoverySourceClass,
    /// Discovery freshness certified by the row (or `not_applicable`).
    pub discovery_freshness_class: DiscoveryFreshnessClass,
    /// Adapter-confidence label certified by the row (or `not_applicable`).
    pub adapter_confidence_label_class: AdapterConfidenceLabelClass,
    /// Target-graph snapshot class certified by the row (or `not_applicable`).
    pub target_graph_snapshot_class: TargetGraphSnapshotClass,
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
    /// For `lineage_admission` rows, the bound `execution_context_id`
    /// token (or equivalent lineage object reference). Required when
    /// `row_class == LineageAdmission`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id_binding: Option<String>,
    /// For `cross_surface_target_parity_truth` wedge admissions, true
    /// when the row attests that run, test, debug, and snapshot
    /// surfaces all read the same target identity and target-graph.
    #[serde(default)]
    pub cross_surface_target_parity_attested: bool,
    /// True when raw discovery payloads, raw adapter handshake bodies,
    /// raw command lines, or raw process environment bytes are excluded
    /// from this row.
    pub raw_source_material_excluded: bool,
    /// True when raw secret values are excluded from this row.
    pub secrets_excluded: bool,
    /// True when ambient authority/credentials are excluded from this row.
    pub ambient_authority_excluded: bool,
    /// Capture timestamp for the row.
    pub captured_at: String,
}

impl BuildTargetHardeningRow {
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
    pub build_target_hardening_packet_id_ref: String,
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
    /// True when the discovery-source vocabulary is preserved verbatim.
    pub preserves_discovery_source_vocabulary: bool,
    /// True when the discovery-freshness vocabulary is preserved verbatim.
    pub preserves_discovery_freshness_vocabulary: bool,
    /// True when the adapter-confidence label vocabulary is preserved verbatim.
    pub preserves_adapter_confidence_label_vocabulary: bool,
    /// True when the target-graph snapshot vocabulary is preserved verbatim.
    pub preserves_target_graph_snapshot_vocabulary: bool,
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
        self.build_target_hardening_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_lane_vocabulary
            && self.preserves_row_class_vocabulary
            && self.preserves_support_class_vocabulary
            && self.preserves_wedge_vocabulary
            && self.preserves_discovery_source_vocabulary
            && self.preserves_discovery_freshness_vocabulary
            && self.preserves_adapter_confidence_label_vocabulary
            && self.preserves_target_graph_snapshot_vocabulary
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

/// Constructor input for [`BuildTargetHardeningTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTargetHardeningTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet.
    pub generated_at: String,
    /// Build-target hardening lanes the packet covers.
    #[serde(default)]
    pub covered_lanes: Vec<BuildTargetHardeningLaneClass>,
    /// Build-target hardening rows.
    #[serde(default)]
    pub rows: Vec<BuildTargetHardeningRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Runtime-owned packet certifying build-target discovery,
/// adapter-confidence label, and target-graph snapshot hardening at the
/// M4 launch-stable grade across run, test, debug, and target-graph
/// snapshot lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildTargetHardeningTruthPacket {
    pub record_kind: String,
    pub schema_version: u32,
    pub packet_id: String,
    pub workflow_or_surface_id: String,
    pub generated_at: String,
    #[serde(default)]
    pub covered_lanes: Vec<BuildTargetHardeningLaneClass>,
    #[serde(default)]
    pub rows: Vec<BuildTargetHardeningRow>,
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    pub promotion_state: PromotionState,
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl BuildTargetHardeningTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: BuildTargetHardeningTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: BUILD_TARGET_HARDENING_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION,
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
        set.into_iter()
            .map(BuildTargetHardeningLaneClass::as_str)
            .collect()
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter()
            .map(BuildTargetHardeningRowClass::as_str)
            .collect()
    }

    /// Returns the unique support-class tokens observed across rows.
    pub fn support_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.support_class);
        }
        set.into_iter()
            .map(BuildTargetHardeningSupportClass::as_str)
            .collect()
    }

    /// Returns the unique wedge tokens observed across rows.
    pub fn wedge_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.wedge_class);
        }
        set.into_iter().map(WedgeClass::as_str).collect()
    }

    /// Returns the unique discovery-source tokens observed across rows.
    pub fn discovery_source_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.discovery_source_class);
        }
        set.into_iter().map(DiscoverySourceClass::as_str).collect()
    }

    /// Returns the unique discovery-freshness tokens observed across rows.
    pub fn discovery_freshness_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.discovery_freshness_class);
        }
        set.into_iter()
            .map(DiscoveryFreshnessClass::as_str)
            .collect()
    }

    /// Returns the unique adapter-confidence label tokens observed across rows.
    pub fn adapter_confidence_label_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.adapter_confidence_label_class);
        }
        set.into_iter()
            .map(AdapterConfidenceLabelClass::as_str)
            .collect()
    }

    /// Returns the unique target-graph snapshot tokens observed across rows.
    pub fn target_graph_snapshot_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.target_graph_snapshot_class);
        }
        set.into_iter()
            .map(TargetGraphSnapshotClass::as_str)
            .collect()
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
        set.into_iter()
            .map(DowngradeAutomationClass::as_str)
            .collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> BuildTargetHardeningTruthSupportExport {
        BuildTargetHardeningTruthSupportExport {
            record_kind: BUILD_TARGET_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            build_target_hardening_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            build_target_hardening_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != BUILD_TARGET_HARDENING_TRUTH_PACKET_RECORD_KIND
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "build-target hardening packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION
        {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "build-target hardening packet has the wrong schema version",
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
                FindingKind::MissingHardeningLaneCoverage,
                FindingSeverity::Blocker,
                "packet must declare at least one covered build-target hardening lane",
            ));
        }

        for lane in &self.covered_lanes {
            let present = self.rows.iter().any(|row| row.lane_class == *lane);
            if !present {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingHardeningLaneCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "no row covers build-target hardening lane {}",
                        lane.as_str()
                    ),
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
                        "row {} admits raw discovery payloads or adapter handshake bodies past the boundary",
                        row.row_id
                    ),
                ));
            }
            if !row.secrets_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::SecretsPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw secret values past the boundary",
                        row.row_id
                    ),
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

            if matches!(
                row.support_class,
                BuildTargetHardeningSupportClass::LaunchStable
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

            // discovery-source binding rules
            if row.row_class.requires_discovery_source()
                && matches!(
                    row.discovery_source_class,
                    DiscoverySourceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoverySourceNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a discovery_source_admission but has no bound source",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_discovery_source()
                && !matches!(
                    row.discovery_source_class,
                    DiscoverySourceClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoverySourceNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds discovery source {}; only discovery_source_admission rows may bind a source",
                        row.row_id,
                        row.row_class.as_str(),
                        row.discovery_source_class.as_str()
                    ),
                ));
            }

            // discovery-freshness binding rules
            if row.row_class.requires_discovery_freshness()
                && matches!(
                    row.discovery_freshness_class,
                    DiscoveryFreshnessClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryFreshnessNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a discovery_freshness_admission but has no bound freshness",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_discovery_freshness()
                && !matches!(
                    row.discovery_freshness_class,
                    DiscoveryFreshnessClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryFreshnessNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds discovery freshness {}; only discovery_freshness_admission rows may bind a freshness class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.discovery_freshness_class.as_str()
                    ),
                ));
            }

            // adapter-confidence label binding rules
            if row.row_class.requires_adapter_confidence_label()
                && matches!(
                    row.adapter_confidence_label_class,
                    AdapterConfidenceLabelClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterConfidenceLabelNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is an adapter_confidence_label_admission but has no bound label",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_adapter_confidence_label()
                && !matches!(
                    row.adapter_confidence_label_class,
                    AdapterConfidenceLabelClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterConfidenceLabelNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds adapter-confidence label {}; only adapter_confidence_label_admission rows may bind a label",
                        row.row_id,
                        row.row_class.as_str(),
                        row.adapter_confidence_label_class.as_str()
                    ),
                ));
            }

            // target-graph snapshot binding rules
            if row.row_class.requires_target_graph_snapshot()
                && matches!(
                    row.target_graph_snapshot_class,
                    TargetGraphSnapshotClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TargetGraphSnapshotNotApplicable,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is a target_graph_snapshot_admission but has no bound snapshot class",
                        row.row_id
                    ),
                ));
            }
            if !row.row_class.requires_target_graph_snapshot()
                && !matches!(
                    row.target_graph_snapshot_class,
                    TargetGraphSnapshotClass::NotApplicable
                )
            {
                findings.push(ValidationFinding::new(
                    FindingKind::TargetGraphSnapshotNotPermittedOnRowClass,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} has row class {} but binds target-graph snapshot {}; only target_graph_snapshot_admission rows may bind a snapshot class",
                        row.row_id,
                        row.row_class.as_str(),
                        row.target_graph_snapshot_class.as_str()
                    ),
                ));
            }

            // consumer-surface binding rules
            if row.row_class.requires_consumer_surface()
                && matches!(
                    row.consumer_surface_class,
                    ConsumerSurfaceClass::NotApplicable
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
                    ConsumerSurfaceClass::NotApplicable
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

            // lineage admission rules
            if matches!(
                row.row_class,
                BuildTargetHardeningRowClass::LineageAdmission
            ) && row
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

            // cross-surface target-parity attestation rule
            if matches!(row.row_class, BuildTargetHardeningRowClass::WedgeAdmission)
                && matches!(row.wedge_class, WedgeClass::CrossSurfaceTargetParityTruth)
                && !row.cross_surface_target_parity_attested
            {
                findings.push(ValidationFinding::new(
                    FindingKind::CrossSurfaceTargetParityNotAttested,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} binds cross_surface_target_parity_truth but does not attest cross-surface target parity",
                        row.row_id
                    ),
                ));
            }

            if matches!(row.confidence_class, ConfidenceClass::LowConfidence)
                && matches!(
                    row.support_class,
                    BuildTargetHardeningSupportClass::LaunchStable
                )
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
                        BuildTargetHardeningRowClass::BuildTargetHardeningQuality
                    )
                    && matches!(
                        row.support_class,
                        BuildTargetHardeningSupportClass::LaunchStable
                    )
            });
            if !lane_claims_launch {
                continue;
            }

            for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(row.row_class, BuildTargetHardeningRowClass::WedgeAdmission)
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

            for source in DiscoverySourceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            BuildTargetHardeningRowClass::DiscoverySourceAdmission
                        )
                        && row.discovery_source_class == source
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingDiscoverySourceCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no discovery_source_admission row for {}",
                            lane.as_str(),
                            source.as_str()
                        ),
                    ));
                }
            }

            for freshness in DiscoveryFreshnessClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            BuildTargetHardeningRowClass::DiscoveryFreshnessAdmission
                        )
                        && row.discovery_freshness_class == freshness
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingDiscoveryFreshnessCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no discovery_freshness_admission row for {}",
                            lane.as_str(),
                            freshness.as_str()
                        ),
                    ));
                }
            }

            for label in AdapterConfidenceLabelClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            BuildTargetHardeningRowClass::AdapterConfidenceLabelAdmission
                        )
                        && row.adapter_confidence_label_class == label
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingAdapterConfidenceLabelCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no adapter_confidence_label_admission row for {}",
                            lane.as_str(),
                            label.as_str()
                        ),
                    ));
                }
            }

            for snapshot in TargetGraphSnapshotClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            BuildTargetHardeningRowClass::TargetGraphSnapshotAdmission
                        )
                        && row.target_graph_snapshot_class == snapshot
                });
                if !covered {
                    findings.push(ValidationFinding::new(
                        FindingKind::MissingTargetGraphSnapshotCoverage,
                        FindingSeverity::Blocker,
                        format!(
                            "lane {} claims launch_stable but has no target_graph_snapshot_admission row for {}",
                            lane.as_str(),
                            snapshot.as_str()
                        ),
                    ));
                }
            }

            for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
                let covered = self.rows.iter().any(|row| {
                    row.lane_class == *lane
                        && matches!(
                            row.row_class,
                            BuildTargetHardeningRowClass::ConsumerSurfaceBinding
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
                    && matches!(
                        row.row_class,
                        BuildTargetHardeningRowClass::LineageAdmission
                    )
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
                        "projection {} does not preserve build-target hardening truth",
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
            if !projection.preserves_discovery_source_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoverySourceVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the discovery-source vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_discovery_freshness_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DiscoveryFreshnessVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the discovery-freshness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_adapter_confidence_label_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AdapterConfidenceLabelVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the adapter-confidence label vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_target_graph_snapshot_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::TargetGraphSnapshotVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the target-graph snapshot vocabulary",
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
pub struct BuildTargetHardeningTruthSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub export_id: String,
    pub build_target_hardening_packet_id_ref: String,
    pub exported_at: String,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub build_target_hardening_packet: BuildTargetHardeningTruthPacket,
}

impl BuildTargetHardeningTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == BUILD_TARGET_HARDENING_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == BUILD_TARGET_HARDENING_TRUTH_SCHEMA_VERSION
            && self.build_target_hardening_packet_id_ref
                == self.build_target_hardening_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.build_target_hardening_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum BuildTargetHardeningTruthArtifactError {
    Packet(serde_json::Error),
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for BuildTargetHardeningTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "build-target hardening packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "build-target hardening packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for BuildTargetHardeningTruthArtifactError {}

/// Returns the checked-in stable build-target hardening truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_build_target_hardening_truth_packet(
) -> Result<BuildTargetHardeningTruthPacket, BuildTargetHardeningTruthArtifactError> {
    let packet: BuildTargetHardeningTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/runtime/m4/harden_build_target_discovery_adapter_confidence_labels_and_truth_packet.json"
    )))
    .map_err(BuildTargetHardeningTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(BuildTargetHardeningTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn doc_ref() -> String {
        BUILD_TARGET_HARDENING_TRUTH_DOC_REF.to_owned()
    }

    fn fixture_ref() -> String {
        BUILD_TARGET_HARDENING_TRUTH_FIXTURE_DIR.to_owned()
    }

    fn quality_row(prefix: &str, lane: BuildTargetHardeningLaneClass) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:quality"),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::BuildTargetHardeningQuality,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ReleaseEvidenceReview,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoBlockOnMissingEvidence,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![doc_ref(), fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_block_on_missing_evidence", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn wedge_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        wedge: WedgeClass,
    ) -> BuildTargetHardeningRow {
        let parity_attested = matches!(wedge, WedgeClass::CrossSurfaceTargetParityTruth);
        let automation = match wedge {
            WedgeClass::BuildTargetDiscoveryTruth => {
                DowngradeAutomationClass::AutoNarrowOnDiscoverySourceGap
            }
            WedgeClass::AdapterConfidenceLabelTruth => {
                DowngradeAutomationClass::AutoNarrowOnAdapterConfidenceLabelGap
            }
            WedgeClass::TargetGraphSnapshotTruth => {
                DowngradeAutomationClass::AutoNarrowOnTargetGraphSnapshotGap
            }
            WedgeClass::CrossSurfaceTargetParityTruth => {
                DowngradeAutomationClass::AutoNarrowOnCrossSurfaceTargetDrift
            }
            WedgeClass::NotApplicable => DowngradeAutomationClass::None,
        };
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:wedge:{}", wedge.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::WedgeAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: wedge,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: automation,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#{}", doc_ref(), automation.as_str())),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: parity_attested,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn discovery_source_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        source: DiscoverySourceClass,
    ) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:discovery_source:{}", source.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::DiscoverySourceAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: source,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnDiscoverySourceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_discovery_source_gap", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn discovery_freshness_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        freshness: DiscoveryFreshnessClass,
    ) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:discovery_freshness:{}", freshness.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::DiscoveryFreshnessAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: freshness,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FailureRecoveryDrillEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnDiscoveryFreshnessGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_discovery_freshness_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn adapter_label_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        label: AdapterConfidenceLabelClass,
    ) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:adapter_label:{}", label.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::AdapterConfidenceLabelAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: label,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnAdapterConfidenceLabelGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_adapter_confidence_label_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn snapshot_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        snapshot: TargetGraphSnapshotClass,
    ) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:target_graph_snapshot:{}", snapshot.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::TargetGraphSnapshotAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: snapshot,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::FixtureRepoEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class:
                DowngradeAutomationClass::AutoNarrowOnTargetGraphSnapshotGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!(
                "{}#auto_narrow_on_target_graph_snapshot_gap",
                doc_ref()
            )),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn consumer_surface_row(
        prefix: &str,
        lane: BuildTargetHardeningLaneClass,
        surface: ConsumerSurfaceClass,
    ) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:consumer_surface:{}", surface.as_str()),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::ConsumerSurfaceBinding,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: surface,
            evidence_class: EvidenceClass::ConformanceSuiteEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnConsumerSurfaceGap,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_consumer_surface_gap", doc_ref())),
            execution_context_id_binding: None,
            cross_surface_target_parity_attested: false,
            raw_source_material_excluded: true,
            secrets_excluded: true,
            ambient_authority_excluded: true,
            captured_at: "2026-05-27T12:00:00Z".to_owned(),
        }
    }

    fn lineage_row(prefix: &str, lane: BuildTargetHardeningLaneClass) -> BuildTargetHardeningRow {
        BuildTargetHardeningRow {
            row_id: format!("row:{prefix}:lineage_admission"),
            lane_class: lane,
            row_class: BuildTargetHardeningRowClass::LineageAdmission,
            support_class: BuildTargetHardeningSupportClass::LaunchStable,
            wedge_class: WedgeClass::NotApplicable,
            discovery_source_class: DiscoverySourceClass::NotApplicable,
            discovery_freshness_class: DiscoveryFreshnessClass::NotApplicable,
            adapter_confidence_label_class: AdapterConfidenceLabelClass::NotApplicable,
            target_graph_snapshot_class: TargetGraphSnapshotClass::NotApplicable,
            consumer_surface_class: ConsumerSurfaceClass::NotApplicable,
            evidence_class: EvidenceClass::AutomatedFunctionalEvidence,
            known_limit_class: KnownLimitClass::NoneDeclared,
            downgrade_automation_class: DowngradeAutomationClass::AutoNarrowOnLineageBreak,
            confidence_class: ConfidenceClass::HighConfidence,
            evidence_refs: vec![fixture_ref()],
            disclosure_ref: Some(format!("{}#auto_narrow_on_lineage_break", doc_ref())),
            execution_context_id_binding: Some(format!("exec:m4:build_target:{prefix}:lineage")),
            cross_surface_target_parity_attested: false,
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
            build_target_hardening_packet_id_ref:
                "packet:m4:harden_build_target_discovery_adapter_confidence_labels_and".to_owned(),
            rendered_at: "2026-05-27T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_lane_vocabulary: true,
            preserves_row_class_vocabulary: true,
            preserves_support_class_vocabulary: true,
            preserves_wedge_vocabulary: true,
            preserves_discovery_source_vocabulary: true,
            preserves_discovery_freshness_vocabulary: true,
            preserves_adapter_confidence_label_vocabulary: true,
            preserves_target_graph_snapshot_vocabulary: true,
            preserves_consumer_surface_vocabulary: true,
            preserves_known_limit_vocabulary: true,
            preserves_downgrade_automation_vocabulary: true,
            preserves_evidence_class_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn lane_rows(
        lane: BuildTargetHardeningLaneClass,
        prefix: &str,
    ) -> Vec<BuildTargetHardeningRow> {
        let mut out = vec![quality_row(prefix, lane)];
        for wedge in WedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(wedge_row(prefix, lane, wedge));
        }
        for source in DiscoverySourceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(discovery_source_row(prefix, lane, source));
        }
        for freshness in DiscoveryFreshnessClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(discovery_freshness_row(prefix, lane, freshness));
        }
        for label in AdapterConfidenceLabelClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(adapter_label_row(prefix, lane, label));
        }
        for snapshot in TargetGraphSnapshotClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(snapshot_row(prefix, lane, snapshot));
        }
        for surface in ConsumerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            out.push(consumer_surface_row(prefix, lane, surface));
        }
        out.push(lineage_row(prefix, lane));
        out
    }

    fn sample_input() -> BuildTargetHardeningTruthPacketInput {
        let mut rows = Vec::new();
        rows.extend(lane_rows(BuildTargetHardeningLaneClass::RunLane, "run"));
        rows.extend(lane_rows(BuildTargetHardeningLaneClass::TestLane, "test"));
        rows.extend(lane_rows(BuildTargetHardeningLaneClass::DebugLane, "debug"));
        rows.extend(lane_rows(
            BuildTargetHardeningLaneClass::TargetGraphSnapshotLane,
            "target_graph_snapshot",
        ));
        BuildTargetHardeningTruthPacketInput {
            packet_id: "packet:m4:harden_build_target_discovery_adapter_confidence_labels_and"
                .to_owned(),
            workflow_or_surface_id:
                "workflow.runtime.harden_build_target_discovery_adapter_confidence_labels_and"
                    .to_owned(),
            generated_at: "2026-05-27T12:00:00Z".to_owned(),
            covered_lanes: BuildTargetHardeningLaneClass::REQUIRED.to_vec(),
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
        assert_eq!(BuildTargetHardeningLaneClass::RunLane.as_str(), "run_lane");
        assert_eq!(
            BuildTargetHardeningLaneClass::TargetGraphSnapshotLane.as_str(),
            "target_graph_snapshot_lane"
        );
        assert_eq!(
            BuildTargetHardeningRowClass::BuildTargetHardeningQuality.as_str(),
            "build_target_hardening_quality"
        );
        assert_eq!(
            BuildTargetHardeningRowClass::LineageAdmission.as_str(),
            "lineage_admission"
        );
        assert_eq!(
            WedgeClass::BuildTargetDiscoveryTruth.as_str(),
            "build_target_discovery_truth"
        );
        assert_eq!(
            WedgeClass::CrossSurfaceTargetParityTruth.as_str(),
            "cross_surface_target_parity_truth"
        );
        assert_eq!(
            DiscoverySourceClass::NativeProtocol.as_str(),
            "native_protocol"
        );
        assert_eq!(
            DiscoverySourceClass::ResolverUnavailable.as_str(),
            "resolver_unavailable"
        );
        assert_eq!(DiscoveryFreshnessClass::FreshProbe.as_str(), "fresh_probe");
        assert_eq!(DiscoveryFreshnessClass::Unknown.as_str(), "unknown");
        assert_eq!(
            AdapterConfidenceLabelClass::AdapterAuthoritativeMatch.as_str(),
            "adapter_authoritative_match"
        );
        assert_eq!(
            AdapterConfidenceLabelClass::AdapterUnreachable.as_str(),
            "adapter_unreachable"
        );
        assert_eq!(
            TargetGraphSnapshotClass::LiveSnapshot.as_str(),
            "live_snapshot"
        );
        assert_eq!(
            TargetGraphSnapshotClass::SnapshotUnavailable.as_str(),
            "snapshot_unavailable"
        );
        assert_eq!(ConsumerSurfaceClass::RunSurface.as_str(), "run_surface");
        assert_eq!(
            ConsumerSurfaceClass::ConformanceDashboard.as_str(),
            "conformance_dashboard"
        );
        assert_eq!(
            FindingKind::CrossSurfaceTargetParityNotAttested.as_str(),
            "cross_surface_target_parity_not_attested"
        );
        assert_eq!(
            FindingKind::MissingAdapterConfidenceLabelCoverage.as_str(),
            "missing_adapter_confidence_label_coverage"
        );
    }

    #[test]
    fn baseline_materialization_is_stable() {
        let packet = BuildTargetHardeningTruthPacket::materialize(sample_input());
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
                "support:m4:harden_build_target_discovery_adapter_confidence_labels_and",
                "2026-05-27T12:00:10Z"
            )
            .is_export_safe());
    }

    #[test]
    fn launch_stable_with_unbound_evidence_blocks() {
        let mut input = sample_input();
        input.rows[0].evidence_class = EvidenceClass::EvidenceUnbound;
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
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
    fn missing_discovery_source_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                BuildTargetHardeningRowClass::DiscoverySourceAdmission
            ) && row.discovery_source_class == DiscoverySourceClass::ResolverUnavailable
                && row.lane_class == BuildTargetHardeningLaneClass::RunLane)
        });
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDiscoverySourceCoverage));
    }

    #[test]
    fn missing_adapter_confidence_label_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                BuildTargetHardeningRowClass::AdapterConfidenceLabelAdmission
            ) && row.adapter_confidence_label_class
                == AdapterConfidenceLabelClass::AdapterUnreachable
                && row.lane_class == BuildTargetHardeningLaneClass::DebugLane)
        });
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::MissingAdapterConfidenceLabelCoverage
        }));
    }

    #[test]
    fn missing_target_graph_snapshot_for_launch_stable_blocks() {
        let mut input = sample_input();
        input.rows.retain(|row| {
            !(matches!(
                row.row_class,
                BuildTargetHardeningRowClass::TargetGraphSnapshotAdmission
            ) && row.target_graph_snapshot_class == TargetGraphSnapshotClass::SnapshotUnavailable
                && row.lane_class == BuildTargetHardeningLaneClass::TargetGraphSnapshotLane)
        });
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(
            packet
                .validation_findings
                .iter()
                .any(|finding| finding.finding_kind
                    == FindingKind::MissingTargetGraphSnapshotCoverage)
        );
    }

    #[test]
    fn cross_surface_target_parity_without_attestation_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(row.row_class, BuildTargetHardeningRowClass::WedgeAdmission)
                && row.wedge_class == WedgeClass::CrossSurfaceTargetParityTruth
                && row.lane_class == BuildTargetHardeningLaneClass::DebugLane
            {
                row.cross_surface_target_parity_attested = false;
                break;
            }
        }
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::CrossSurfaceTargetParityNotAttested
        }));
    }

    #[test]
    fn lineage_admission_without_execution_context_id_blocks() {
        let mut input = sample_input();
        for row in &mut input.rows {
            if matches!(
                row.row_class,
                BuildTargetHardeningRowClass::LineageAdmission
            ) && row.lane_class == BuildTargetHardeningLaneClass::RunLane
            {
                row.execution_context_id_binding = None;
                break;
            }
        }
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::LineageAdmissionMissingExecutionContextId
        }));
    }

    #[test]
    fn narrowed_row_without_disclosure_ref_blocks() {
        let mut input = sample_input();
        input.rows[0].support_class = BuildTargetHardeningSupportClass::LaunchStableBelow;
        input.rows[0].disclosure_ref = None;
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
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
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn collapsed_adapter_label_vocabulary_blocks() {
        let mut input = sample_input();
        for projection in &mut input.consumer_projections {
            if projection.consumer_surface == ConsumerProjectionSurface::HelpAbout {
                projection.preserves_adapter_confidence_label_vocabulary = false;
            }
        }
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::AdapterConfidenceLabelVocabularyCollapsed
        }));
    }

    #[test]
    fn raw_source_material_blocks_promotion() {
        let mut input = sample_input();
        input.rows[0].raw_source_material_excluded = false;
        let packet = BuildTargetHardeningTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RawSourceMaterialPresent));
    }
}
