//! Certification that profile-session, flamegraph, trace, workset-scope,
//! topology, ownership, and explainer component truth holds on every claimed M5
//! performance and codebase-understanding surface.
//!
//! This module is the M05-803 certification capstone over the frozen M5
//! profiler/topology component matrix (the M05-797..802 family set). Where the
//! sibling lanes freeze the reusable families (M05-797..800), harden their
//! accessibility fallback (M05-801), and adopt them across consumer classes
//! (M05-802), this lane certifies, per *claimed consumer surface*, that the
//! surface either passes the shared component packet or narrows its claim state
//! automatically.
//!
//! Each row keys on one [`M5ClaimedSurface`] (17 claim-bearing performance,
//! codebase-understanding, and shared-evidence surfaces) and certifies four
//! component-truth axes that apply to the component families it consumes —
//! capture/execution identity and imported-versus-live truth, compare baseline
//! and confounder disclosure, workset scope and no-silent-widening, and
//! topology/ownership/explainer provenance and citation posture — plus a
//! cross-cutting label/export parity axis. A surface passes (green) when every
//! applicable axis is certified, narrows (yellow) when at least one axis is a
//! disclosed reduction, and is rejected (red) when any axis hides truth or
//! presents partial state as full truth. Every certified surface points back to
//! exactly one certification bundle so release, help, and support packets can
//! cite a single proof of profiler/graph component truth.
//!
//! It reuses the shared copy/export, reduced-capability, support-export, and
//! auto-narrowing disclosure structs from
//! [`crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows`]
//! and the frozen [`M5ComponentFamily`] enum from
//! [`crate::add_component_fallback_parity_keyboard_and_screen_reader_navigation_and_export_safe_summaries`]
//! so exported labels stay byte-identical to the sibling component packets and
//! there is no controlled-vocabulary drift.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_component_fallback_parity_keyboard_and_screen_reader_navigation_and_export_safe_summaries::M5ComponentFamily;
use crate::implement_profile_session_cards_flamegraph_icicle_views_and_call_tree_rows::{
    AutoNarrowingContract, CopyExportProjection, ReducedCapabilityBanner, SupportExportJoin,
};

/// Schema version stamped on the M05-803 certification packet.
pub const COMPONENT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ComponentCertificationPacket`].
pub const COMPONENT_CERTIFICATION_RECORD_KIND: &str =
    "m5_profiler_topology_component_certification";

/// Stable record-kind tag for each [`ComponentCertificationRow`].
pub const COMPONENT_CERTIFICATION_ROW_RECORD_KIND: &str =
    "m5_profiler_topology_component_certification_row";

/// Repo-relative path to the checked-in M05-803 certification packet.
pub const COMPONENT_CERTIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/m5-profiler-topology-component-certification.json";

/// Embedded checked-in M05-803 certification packet JSON.
pub const COMPONENT_CERTIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/m5-profiler-topology-component-certification.json"
));

/// The domain a claimed surface belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceDomain {
    /// Profiling / performance-depth surfaces.
    Performance,
    /// Codebase-understanding surfaces (topology, ownership, explainer, search).
    CodebaseUnderstanding,
    /// Shared evidence surfaces that consume both families (review, AI,
    /// incident, docs/help, CLI, support export, release proof).
    SharedEvidence,
}

/// A claim-bearing M5 consumer surface, matching the matrix `consumer_surface`
/// vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimedSurface {
    DesktopProfilerWorkspace,
    HotspotWorkspace,
    TraceViewer,
    HeapAnalysis,
    ProfileCompare,
    TopologyMap,
    OwnershipBrowser,
    ArchitectureExplainer,
    SearchResults,
    ReviewWorkspace,
    OnboardingTour,
    AiContextPanel,
    IncidentWorkspace,
    DocsHelp,
    CliHeadless,
    SupportExport,
    ReleaseProof,
}

impl M5ClaimedSurface {
    /// Every claimed surface, in matrix order.
    pub const ALL: [M5ClaimedSurface; 17] = [
        M5ClaimedSurface::DesktopProfilerWorkspace,
        M5ClaimedSurface::HotspotWorkspace,
        M5ClaimedSurface::TraceViewer,
        M5ClaimedSurface::HeapAnalysis,
        M5ClaimedSurface::ProfileCompare,
        M5ClaimedSurface::TopologyMap,
        M5ClaimedSurface::OwnershipBrowser,
        M5ClaimedSurface::ArchitectureExplainer,
        M5ClaimedSurface::SearchResults,
        M5ClaimedSurface::ReviewWorkspace,
        M5ClaimedSurface::OnboardingTour,
        M5ClaimedSurface::AiContextPanel,
        M5ClaimedSurface::IncidentWorkspace,
        M5ClaimedSurface::DocsHelp,
        M5ClaimedSurface::CliHeadless,
        M5ClaimedSurface::SupportExport,
        M5ClaimedSurface::ReleaseProof,
    ];

    /// Surfaces that release, help, and support packets must be able to cite a
    /// single certification bundle from.
    pub const EVIDENCE_SURFACES: [M5ClaimedSurface; 3] = [
        M5ClaimedSurface::DocsHelp,
        M5ClaimedSurface::SupportExport,
        M5ClaimedSurface::ReleaseProof,
    ];

    /// The domain this surface belongs to.
    pub const fn domain(self) -> M5SurfaceDomain {
        match self {
            M5ClaimedSurface::DesktopProfilerWorkspace
            | M5ClaimedSurface::HotspotWorkspace
            | M5ClaimedSurface::TraceViewer
            | M5ClaimedSurface::HeapAnalysis
            | M5ClaimedSurface::ProfileCompare => M5SurfaceDomain::Performance,
            M5ClaimedSurface::TopologyMap
            | M5ClaimedSurface::OwnershipBrowser
            | M5ClaimedSurface::ArchitectureExplainer
            | M5ClaimedSurface::SearchResults
            | M5ClaimedSurface::OnboardingTour => M5SurfaceDomain::CodebaseUnderstanding,
            M5ClaimedSurface::ReviewWorkspace
            | M5ClaimedSurface::AiContextPanel
            | M5ClaimedSurface::IncidentWorkspace
            | M5ClaimedSurface::DocsHelp
            | M5ClaimedSurface::CliHeadless
            | M5ClaimedSurface::SupportExport
            | M5ClaimedSurface::ReleaseProof => M5SurfaceDomain::SharedEvidence,
        }
    }
}

/// Returns true when a family carries capture/execution and imported-versus-live
/// truth (profile-session and profiler cost families).
const fn is_capture_family(family: M5ComponentFamily) -> bool {
    matches!(
        family,
        M5ComponentFamily::ProfileSessionCard
            | M5ComponentFamily::FlamegraphView
            | M5ComponentFamily::IcicleView
            | M5ComponentFamily::CallTreeRow
            | M5ComponentFamily::HeapProfileCompareCard
            | M5ComponentFamily::TraceTimeline
    )
}

/// Returns true when a family carries baseline/environment/threshold compare
/// truth that must be disclosed before a regression is claimed.
const fn is_compare_family(family: M5ComponentFamily) -> bool {
    matches!(
        family,
        M5ComponentFamily::FlamegraphView
            | M5ComponentFamily::IcicleView
            | M5ComponentFamily::HeapProfileCompareCard
    )
}

/// Returns true when a family carries workset scope truth (scope mode, excluded
/// roots, index coverage, no-silent-widening).
const fn is_workset_family(family: M5ComponentFamily) -> bool {
    matches!(
        family,
        M5ComponentFamily::WorksetSwitcherRow
            | M5ComponentFamily::TopologyNodeCard
            | M5ComponentFamily::ExplainerSectionCard
    )
}

/// Returns true when a family carries graph provenance / citation truth
/// (freshness, confidence, provenance, role separation, generated-vs-curated).
const fn is_graph_family(family: M5ComponentFamily) -> bool {
    matches!(
        family,
        M5ComponentFamily::TopologyNodeCard
            | M5ComponentFamily::OwnershipCard
            | M5ComponentFamily::ExplainerSectionCard
    )
}

/// Capture/execution identity and imported-versus-live truth for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureExecutionTruthState {
    /// Capture mode, execution origin, build/runtime identity, and
    /// imported-versus-live artifact origin are certified visible.
    IdentityCertified,
    /// Some identity truth narrows to inspect-only, but the reduction is
    /// disclosed (yellow).
    DisclosedReducedIdentity,
    /// Identity is hidden or an imported artifact is presented as live (red).
    IdentityHiddenOrImportedAsLive,
    /// The surface consumes no capture family.
    NotApplicable,
}

/// Baseline / environment / threshold / confounder disclosure for compare views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareBaselineTruthState {
    /// Baseline identity, environment deltas, threshold/waiver state, and
    /// confounders are certified visible before any regression is claimed.
    BaselineAndConfoundersCertified,
    /// Comparison is deferred or reduced, but disclosed (yellow).
    DisclosedDeferredComparison,
    /// A regression is claimed before baseline/confounder truth is visible (red).
    RegressionClaimedBeforeBaselineTruth,
    /// The surface consumes no compare family.
    NotApplicable,
}

/// Workset scope and no-silent-widening truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorksetScopeTruthState {
    /// Scope mode, excluded roots, index coverage, and no-silent-widening are
    /// certified visible.
    ScopeAndNoWideningCertified,
    /// Scope narrows to a sparse slice, but disclosed (yellow).
    DisclosedNarrowedScope,
    /// Scope silently widens or hides excluded roots / index coverage (red).
    SilentWideningOrHiddenScope,
    /// The surface consumes no workset family.
    NotApplicable,
}

/// Topology / ownership / explainer provenance and citation posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphProvenanceTruthState {
    /// Freshness, confidence, provenance, role separation, and concrete
    /// citations (generated-vs-curated) are certified visible.
    ProvenanceAndCitationsCertified,
    /// The claim narrows to available evidence only, but disclosed (yellow).
    DisclosedNarrowedToAvailableEvidence,
    /// Partial graph state is presented as full truth, or roles collapse, or a
    /// generated summary masquerades as uncited primary truth (red).
    PartialGraphPresentedAsFullTruth,
    /// The surface consumes no topology/ownership/explainer family.
    NotApplicable,
}

/// Cross-cutting controlled-label / copy-export / export-safe-summary parity.
/// Applies to every surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimExportParityState {
    /// Controlled labels and export-safe summaries reconstruct the claim in
    /// text/JSON/Markdown without a screenshot.
    LabelsAndSummariesCertified,
    /// Export narrows to a partial capture, but disclosed (yellow).
    DisclosedPartialExport,
    /// Labels are renamed/dropped or the export relies on a screenshot (red).
    LabelsDroppedOrScreenshotOnly,
}

/// Shared tri-state helpers so the derivation logic reads uniformly.
macro_rules! truth_state_helpers {
    ($ty:ty, $red:path, $disclosed:path, $na:path) => {
        impl $ty {
            /// Returns true unless the state hides truth (red).
            pub const fn never_violates(self) -> bool {
                !matches!(self, $red)
            }

            /// Returns true when the state is a disclosed reduction (yellow).
            pub const fn is_disclosed_reduction(self) -> bool {
                matches!(self, $disclosed)
            }

            /// Returns true when the axis does not apply to the surface.
            pub const fn is_not_applicable(self) -> bool {
                matches!(self, $na)
            }
        }
    };
}

truth_state_helpers!(
    CaptureExecutionTruthState,
    CaptureExecutionTruthState::IdentityHiddenOrImportedAsLive,
    CaptureExecutionTruthState::DisclosedReducedIdentity,
    CaptureExecutionTruthState::NotApplicable
);
truth_state_helpers!(
    CompareBaselineTruthState,
    CompareBaselineTruthState::RegressionClaimedBeforeBaselineTruth,
    CompareBaselineTruthState::DisclosedDeferredComparison,
    CompareBaselineTruthState::NotApplicable
);
truth_state_helpers!(
    WorksetScopeTruthState,
    WorksetScopeTruthState::SilentWideningOrHiddenScope,
    WorksetScopeTruthState::DisclosedNarrowedScope,
    WorksetScopeTruthState::NotApplicable
);
truth_state_helpers!(
    GraphProvenanceTruthState,
    GraphProvenanceTruthState::PartialGraphPresentedAsFullTruth,
    GraphProvenanceTruthState::DisclosedNarrowedToAvailableEvidence,
    GraphProvenanceTruthState::NotApplicable
);

impl ClaimExportParityState {
    /// Returns true unless labels drop or the export needs a screenshot (red).
    pub const fn never_violates(self) -> bool {
        !matches!(self, ClaimExportParityState::LabelsDroppedOrScreenshotOnly)
    }

    /// Returns true when the state is a disclosed reduction (yellow).
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, ClaimExportParityState::DisclosedPartialExport)
    }
}

/// Derived certification status for a claimed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceCertificationStatus {
    /// Passes the shared component packet on every applicable axis (green).
    Certified,
    /// Auto-narrowed its claim on at least one disclosed axis (yellow).
    NarrowedDisclosed,
    /// Hides truth or presents partial state as full truth (red).
    Blocked,
}

/// Certification row for one claimed consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentCertificationRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub row_id: String,
    pub surface: M5ClaimedSurface,
    pub surface_domain: M5SurfaceDomain,
    /// The canonical component families this surface consumes.
    #[serde(default)]
    pub consumed_families: Vec<M5ComponentFamily>,
    /// The single certification bundle this surface cites.
    pub certification_bundle_ref: String,
    /// True when the surface consumes families by canonical reference rather
    /// than cloning surface-local prose.
    pub references_canonical_families: bool,
    pub capture_execution_truth: CaptureExecutionTruthState,
    pub compare_baseline_truth: CompareBaselineTruthState,
    pub workset_scope_truth: WorksetScopeTruthState,
    pub graph_provenance_truth: GraphProvenanceTruthState,
    pub claim_export_parity: ClaimExportParityState,
    /// True when the surface auto-narrowed its claim; must match the derived
    /// status.
    pub claim_narrowed: bool,
    #[serde(default)]
    pub source_refs: Vec<String>,
    pub copy_export: CopyExportProjection,
    pub reduced_capability_banner: ReducedCapabilityBanner,
    pub support_export_join: SupportExportJoin,
    pub auto_narrowing_contract: AutoNarrowingContract,
}

impl ComponentCertificationRow {
    /// True when the surface consumes at least one capture family.
    pub fn consumes_capture_family(&self) -> bool {
        self.consumed_families
            .iter()
            .copied()
            .any(is_capture_family)
    }

    /// True when the surface consumes at least one compare family.
    pub fn consumes_compare_family(&self) -> bool {
        self.consumed_families
            .iter()
            .copied()
            .any(is_compare_family)
    }

    /// True when the surface consumes at least one workset family.
    pub fn consumes_workset_family(&self) -> bool {
        self.consumed_families
            .iter()
            .copied()
            .any(is_workset_family)
    }

    /// True when the surface consumes at least one graph provenance family.
    pub fn consumes_graph_family(&self) -> bool {
        self.consumed_families.iter().copied().any(is_graph_family)
    }

    /// Each truth axis is present exactly when the surface consumes a family it
    /// applies to, and absent (`NotApplicable`) otherwise.
    pub fn axes_match_consumed_families(&self) -> bool {
        self.capture_execution_truth.is_not_applicable() != self.consumes_capture_family()
            && self.compare_baseline_truth.is_not_applicable() != self.consumes_compare_family()
            && self.workset_scope_truth.is_not_applicable() != self.consumes_workset_family()
            && self.graph_provenance_truth.is_not_applicable() != self.consumes_graph_family()
    }

    /// True when no applicable axis hides truth (no red axis).
    pub fn all_axes_never_violate(&self) -> bool {
        self.capture_execution_truth.never_violates()
            && self.compare_baseline_truth.never_violates()
            && self.workset_scope_truth.never_violates()
            && self.graph_provenance_truth.never_violates()
            && self.claim_export_parity.never_violates()
    }

    /// True when at least one axis is a disclosed reduction.
    pub fn any_axis_disclosed(&self) -> bool {
        self.capture_execution_truth.is_disclosed_reduction()
            || self.compare_baseline_truth.is_disclosed_reduction()
            || self.workset_scope_truth.is_disclosed_reduction()
            || self.graph_provenance_truth.is_disclosed_reduction()
            || self.claim_export_parity.is_disclosed_reduction()
    }

    /// Derived certification status.
    pub fn status(&self) -> SurfaceCertificationStatus {
        if !self.all_axes_never_violate() {
            SurfaceCertificationStatus::Blocked
        } else if self.any_axis_disclosed() {
            SurfaceCertificationStatus::NarrowedDisclosed
        } else {
            SurfaceCertificationStatus::Certified
        }
    }

    /// AC1: the surface either passes (green) or narrows its claim (yellow); it
    /// is never left in a truth-hiding state.
    pub fn certifies_or_narrows(&self) -> bool {
        self.status() != SurfaceCertificationStatus::Blocked
    }

    /// AC1 coherence: the `claim_narrowed` flag matches the derived status.
    pub fn claim_narrowing_is_coherent(&self) -> bool {
        self.claim_narrowed == (self.status() == SurfaceCertificationStatus::NarrowedDisclosed)
    }

    /// A narrowed surface discloses the reduction with a reduced-capability
    /// banner and a non-empty auto-narrowing contract.
    pub fn discloses_narrowing(&self) -> bool {
        if !self.claim_narrowed {
            return true;
        }
        self.reduced_capability_banner.capability_state != "full"
            && !self
                .reduced_capability_banner
                .capability_state
                .trim()
                .is_empty()
            && !self
                .reduced_capability_banner
                .missing_capabilities
                .is_empty()
            && !self
                .auto_narrowing_contract
                .narrow_on_missing_or_stale
                .is_empty()
    }

    /// AC2/AC3: the surface points back to the one certification bundle by
    /// canonical reference.
    pub fn references_one_bundle(&self, packet_bundle_ref: &str) -> bool {
        self.references_canonical_families
            && !self.certification_bundle_ref.trim().is_empty()
            && self.certification_bundle_ref == packet_bundle_ref
    }

    /// Raw profile/trace material is excluded from the support export.
    pub fn excludes_raw_material(&self) -> bool {
        !self.support_export_join.raw_profile_samples_exported
            && !self.support_export_join.raw_trace_events_exported
            && self.support_export_join.gui_cli_support_label_parity
    }
}

/// Rolled-up summary of an M05-803 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentCertificationSummary {
    pub surface_count: usize,
    pub performance_surface_count: usize,
    pub codebase_understanding_surface_count: usize,
    pub shared_evidence_surface_count: usize,
    pub consumed_family_count: usize,
    pub family_coverage_complete: bool,
    pub all_surfaces_certify_or_narrow: bool,
    pub all_reference_one_bundle: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
}

/// Checked-in M05-803 profiler/topology component certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    /// The single certification bundle every surface cites (AC2).
    pub certification_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ComponentCertificationRow>,
    pub summary: ComponentCertificationSummary,
}

impl ComponentCertificationPacket {
    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComponentCertificationSummary {
        let mut consumed = BTreeSet::new();
        let mut performance = 0;
        let mut codebase = 0;
        let mut shared = 0;
        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            consumed.extend(row.consumed_families.iter().copied());
            match row.surface.domain() {
                M5SurfaceDomain::Performance => performance += 1,
                M5SurfaceDomain::CodebaseUnderstanding => codebase += 1,
                M5SurfaceDomain::SharedEvidence => shared += 1,
            }
            match row.status() {
                SurfaceCertificationStatus::Certified => green += 1,
                SurfaceCertificationStatus::NarrowedDisclosed => yellow += 1,
                SurfaceCertificationStatus::Blocked => red += 1,
            }
        }

        let family_coverage_complete = M5ComponentFamily::ALL
            .iter()
            .all(|family| consumed.contains(family));

        ComponentCertificationSummary {
            surface_count: self.rows.len(),
            performance_surface_count: performance,
            codebase_understanding_surface_count: codebase,
            shared_evidence_surface_count: shared,
            consumed_family_count: consumed.len(),
            family_coverage_complete,
            all_surfaces_certify_or_narrow: self
                .rows
                .iter()
                .all(ComponentCertificationRow::certifies_or_narrows),
            all_reference_one_bundle: self
                .rows
                .iter()
                .all(|row| row.references_one_bundle(&self.certification_bundle_ref)),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComponentCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPONENT_CERTIFICATION_SCHEMA_VERSION {
            violations.push(ComponentCertificationViolation::SchemaVersion {
                expected: COMPONENT_CERTIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPONENT_CERTIFICATION_RECORD_KIND {
            violations.push(ComponentCertificationViolation::RecordKind {
                expected: COMPONENT_CERTIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.certification_bundle_ref.trim().is_empty() {
            violations.push(ComponentCertificationViolation::MissingBundleRef);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        let mut consumed_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComponentCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_surfaces.insert(row.surface);
            consumed_families.extend(row.consumed_families.iter().copied());

            if row.record_kind != COMPONENT_CERTIFICATION_ROW_RECORD_KIND
                || row.schema_version != COMPONENT_CERTIFICATION_SCHEMA_VERSION
                || row.consumed_families.is_empty()
                || row.surface_domain != row.surface.domain()
            {
                violations.push(ComponentCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Duplicate family in one row is a drift signal.
            let mut families = BTreeSet::new();
            for family in &row.consumed_families {
                if !families.insert(*family) {
                    violations.push(ComponentCertificationViolation::DuplicateFamily {
                        id: row.row_id.clone(),
                        family: *family,
                    });
                }
            }

            // Truth axes must apply exactly to the families the surface consumes.
            if !row.axes_match_consumed_families() {
                violations.push(ComponentCertificationViolation::AxisApplicabilityMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: a claim-bearing surface passes or auto-narrows; it is never
            // left hiding truth.
            if !row.certifies_or_narrows() {
                violations.push(ComponentCertificationViolation::BlockedSurface {
                    id: row.row_id.clone(),
                });
            }
            if !row.claim_narrowing_is_coherent() {
                violations.push(ComponentCertificationViolation::ClaimNarrowingMismatch {
                    id: row.row_id.clone(),
                });
            }
            if !row.discloses_narrowing() {
                violations.push(ComponentCertificationViolation::UndisclosedNarrowing {
                    id: row.row_id.clone(),
                });
            }

            // AC2/AC3: one bundle, cited by canonical reference.
            if !row.references_one_bundle(&self.certification_bundle_ref) {
                violations.push(ComponentCertificationViolation::BundleReferenceDrift {
                    id: row.row_id.clone(),
                });
            }

            // Copy/export parity: text/JSON/Markdown, screenshot prohibited.
            if !has_copy_export(&row.copy_export) {
                violations.push(ComponentCertificationViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Raw material stays out of the support export.
            if !row.excludes_raw_material() {
                violations.push(ComponentCertificationViolation::RawMaterialLeak {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == SurfaceCertificationStatus::Blocked {
                violations.push(ComponentCertificationViolation::BlockedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every claimed surface is certified exactly once.
        for surface in M5ClaimedSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations
                    .push(ComponentCertificationViolation::MissingSurfaceCoverage { surface });
            }
        }

        // Family coverage: every frozen family is consumed by some certified
        // surface, so the certification spans all component types.
        for family in M5ComponentFamily::ALL {
            if !consumed_families.contains(&family) {
                violations.push(ComponentCertificationViolation::MissingFamilyCoverage { family });
            }
        }

        // AC2: release/help/support evidence surfaces are present so they can
        // cite the one bundle.
        for surface in M5ClaimedSurface::EVIDENCE_SURFACES {
            if !seen_surfaces.contains(&surface) {
                violations
                    .push(ComponentCertificationViolation::MissingEvidenceSurface { surface });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ComponentCertificationViolation::SummaryMismatch);
        }

        violations
    }
}

fn has_copy_export(copy_export: &CopyExportProjection) -> bool {
    copy_export.screenshot_only_prohibited
        && copy_export.formats.iter().any(|format| format == "text")
        && copy_export.formats.iter().any(|format| format == "json")
        && copy_export
            .formats
            .iter()
            .any(|format| format == "markdown")
        && !copy_export.export_fields.is_empty()
}

/// Loads the checked-in M05-803 certification packet.
pub fn current_component_certification_packet(
) -> Result<ComponentCertificationPacket, serde_json::Error> {
    serde_json::from_str(COMPONENT_CERTIFICATION_PACKET_JSON)
}

/// Validation failure for M05-803 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentCertificationViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingBundleRef,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    DuplicateFamily {
        id: String,
        family: M5ComponentFamily,
    },
    AxisApplicabilityMismatch {
        id: String,
    },
    BlockedSurface {
        id: String,
    },
    ClaimNarrowingMismatch {
        id: String,
    },
    UndisclosedNarrowing {
        id: String,
    },
    BundleReferenceDrift {
        id: String,
    },
    MissingCopyExportParity {
        id: String,
    },
    RawMaterialLeak {
        id: String,
    },
    BlockedRow {
        id: String,
    },
    MissingSurfaceCoverage {
        surface: M5ClaimedSurface,
    },
    MissingFamilyCoverage {
        family: M5ComponentFamily,
    },
    MissingEvidenceSurface {
        surface: M5ClaimedSurface,
    },
    SummaryMismatch,
}

impl fmt::Display for ComponentCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingBundleRef => write!(f, "packet is missing a certification bundle ref"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::DuplicateFamily { id, family } => {
                write!(f, "row {id} consumes family {family:?} more than once")
            }
            Self::AxisApplicabilityMismatch { id } => {
                write!(
                    f,
                    "row {id} truth axes do not match its consumed component families"
                )
            }
            Self::BlockedSurface { id } => {
                write!(
                    f,
                    "row {id} neither passes nor narrows: it hides component truth"
                )
            }
            Self::ClaimNarrowingMismatch { id } => {
                write!(
                    f,
                    "row {id} claim_narrowed flag disagrees with its derived status"
                )
            }
            Self::UndisclosedNarrowing { id } => {
                write!(
                    f,
                    "row {id} narrows its claim without a reduced-capability banner and contract"
                )
            }
            Self::BundleReferenceDrift { id } => {
                write!(
                    f,
                    "row {id} does not cite the one certification bundle by canonical reference"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text/JSON/Markdown copy-export parity"
                )
            }
            Self::RawMaterialLeak { id } => {
                write!(
                    f,
                    "row {id} support export leaks raw profile/trace material"
                )
            }
            Self::BlockedRow { id } => {
                write!(f, "row {id} is blocked (red) and may not ship")
            }
            Self::MissingSurfaceCoverage { surface } => {
                write!(
                    f,
                    "claimed surface {surface:?} is not certified in the packet"
                )
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not consumed by any certified surface"
                )
            }
            Self::MissingEvidenceSurface { surface } => {
                write!(
                    f,
                    "evidence surface {surface:?} must be certified so it can cite the one bundle"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
        }
    }
}

impl Error for ComponentCertificationViolation {}
