//! Coverage, profile, test, debug, and notebook evidence handoff bars with artifact lineage.
//!
//! This module materializes the typed records that keep evidence handoff bars honest
//! about what was captured, from where, with which build/runtime identity, and how
//! comparable the baseline really is. The records and closed vocabularies here mirror
//! the boundary schema at
//! `/schemas/perf/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.schema.json`
//! and reuse the capture-class, provenance, mapping-quality, and environment-identity
//! axes already frozen in `/docs/performance/profiling_trace_replay_contract.md`.
//!
//! The module exposes:
//!
//! - the [`EvidenceHandoffBarRow`] record that binds originating run, artifact/build ID,
//!   commit or revision, capture source, save/share scope, and lineage state so users
//!   always know what evidence they are looking at;
//! - the [`ArtifactLineageRow`] record that carries the deeper lineage detail—evidence
//!   kind, source run ref, build identity, environment fingerprint, capture mode,
//!   mapping quality, freshness, and provenance—so handoff bars never lose attribution;
//! - the [`CaptureSourceRow`] record that classifies capture origin as local live,
//!   remote live, imported, cached, provider-supplied, CI-provided, synthetic, sampled,
//!   instrumented, estimated, or partial;
//! - the [`SaveShareScopeRow`] record that defines what can be done with the evidence
//!   locally, exported, shared, uploaded, or attached to an incident, with redaction
//!   mode and destination class visible;
//! - the [`EvidenceHandoffQualificationPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw payload bytes, raw command lines, secrets, and ambient credentials MUST NOT
//! appear on any record carried here.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::build_the_regression_baseline_store_baseline_selection_ux_and_comparable_environment_guards::BaselineFreshness;

/// Schema version stamped on every evidence-handoff qualification packet carried
/// by this module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const EVIDENCE_HANDOFF_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`EvidenceHandoffQualificationPacket`].
pub const EVIDENCE_HANDOFF_QUALIFICATION_RECORD_KIND: &str =
    "ship_coverage_profile_test_debug_and_notebook_evidence_handoff_bars_with_artifact_lineage";

/// Repo-relative path to the checked-in evidence-handoff qualification packet JSON.
pub const EVIDENCE_HANDOFF_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json";

/// Embedded checked-in qualification packet JSON.
pub const EVIDENCE_HANDOFF_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json"
));

/// Qualification label shown on promoted evidence-handoff surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceHandoffQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl EvidenceHandoffQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Evidence-handoff surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceHandoffSurfaceKind {
    /// Coverage handoff bar surface.
    CoverageHandoffBar,
    /// Profile handoff bar surface.
    ProfileHandoffBar,
    /// Test handoff bar surface.
    TestHandoffBar,
    /// Debug handoff bar surface.
    DebugHandoffBar,
    /// Notebook handoff bar surface.
    NotebookHandoffBar,
    /// Export review surface for evidence handoff.
    ExportReview,
    /// Support export surface for evidence handoff.
    SupportExport,
}

/// Classification of the capture source for an evidence artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CaptureSourceClass {
    /// Live local capture.
    LocalLive,
    /// Live remote capture.
    RemoteLive,
    /// Imported from an external source.
    Imported,
    /// Cached or previously stored.
    Cached,
    /// Supplied by a provider or managed service.
    ProviderSupplied,
    /// Provided by CI/CD pipeline.
    CiProvided,
    /// Synthetic or simulated data.
    Synthetic,
    /// Sampled capture.
    Sampled,
    /// Fully instrumented capture.
    Instrumented,
    /// Estimated or inferred data.
    Estimated,
    /// Partial or incomplete capture.
    Partial,
}

/// Lineage state describing how well the artifact maps to current source/build state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageState {
    /// Exact match between source, build ID, symbols/maps, and runtime artifact.
    ExactMatch,
    /// Nearby source found but build or map drift exists.
    ProbableMismatch,
    /// Source file exists but symbols/maps are partial or absent.
    SourceOnly,
    /// Runtime frame resolves only to disassembly/generated/minified artifact.
    ArtifactOnly,
    /// Evidence exists but current policy blocks retrieval/display.
    RestrictedByPolicy,
    /// No trustworthy mapping available.
    Unavailable,
}

impl LineageState {
    /// Returns true when the state allows the handoff bar to show navigation actions.
    pub const fn allows_navigation(self) -> bool {
        matches!(
            self,
            Self::ExactMatch | Self::ProbableMismatch | Self::SourceOnly
        )
    }

    /// Returns true when the state should show a degraded-state label.
    pub const fn shows_degraded_label(self) -> bool {
        matches!(
            self,
            Self::ProbableMismatch
                | Self::SourceOnly
                | Self::ArtifactOnly
                | Self::RestrictedByPolicy
                | Self::Unavailable
        )
    }
}

/// Kind of evidence represented by a lineage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// Code coverage data.
    Coverage,
    /// CPU or time profile.
    Profile,
    /// Execution trace.
    Trace,
    /// Memory snapshot or heap sample.
    MemorySnapshot,
    /// Test execution result.
    TestResult,
    /// Debug session capture.
    DebugSession,
    /// Notebook cell or output artifact.
    NotebookOutput,
    /// Replay or time-travel timeline.
    ReplayTimeline,
}

/// Scope kind for save/share actions on evidence artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveShareScopeKind {
    /// Remains on the local device only.
    LocalOnly,
    /// May be exported to a file or bundle.
    Exportable,
    /// May be shared through a collaboration session.
    Shareable,
    /// May be uploaded to a remote destination.
    Uploadable,
    /// May be attached to an incident or support ticket.
    AttachToIncident,
}

/// One evidence handoff bar row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffBarRow {
    /// Stable handoff bar row id.
    pub handoff_bar_id: String,
    /// Human-readable title.
    pub title: String,
    /// Surface kind this handoff bar appears on.
    pub surface_kind: EvidenceHandoffSurfaceKind,
    /// Originating run/test/build ref.
    pub originating_run_ref: String,
    /// Artifact/build ID.
    pub artifact_build_id: String,
    /// Commit or revision.
    pub commit_or_revision: String,
    /// Capture source ref.
    pub capture_source_ref: String,
    /// Save/share scope ref.
    pub save_share_scope_ref: String,
    /// Artifact lineage ref.
    pub lineage_ref: String,
    /// Lineage state.
    pub lineage_state: LineageState,
    /// True when the handoff bar is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the handoff bar shows the originating run.
    pub shows_origin: bool,
    /// True when the handoff bar shows the build ID.
    pub shows_build_id: bool,
    /// True when the handoff bar shows the commit/revision.
    pub shows_commit: bool,
    /// True when the handoff bar shows the capture source.
    pub shows_capture_source: bool,
    /// True when the handoff bar shows the save/share scope.
    pub shows_save_share_scope: bool,
    /// True when the handoff bar shows the lineage state.
    pub shows_lineage_state: bool,
    /// True when the handoff bar shows the lineage detail.
    pub shows_lineage_detail: bool,
}

/// One artifact lineage row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactLineageRow {
    /// Stable lineage row id.
    pub lineage_id: String,
    /// Human-readable title.
    pub title: String,
    /// Kind of evidence.
    pub evidence_kind: EvidenceKind,
    /// Source run ref.
    pub source_run_ref: String,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Environment fingerprint ref.
    pub environment_fingerprint_ref: String,
    /// Capture mode ref.
    pub capture_mode_ref: String,
    /// Mapping quality ref.
    pub mapping_quality_ref: String,
    /// Freshness state.
    pub freshness: BaselineFreshness,
    /// Provenance chain refs.
    #[serde(default)]
    pub provenance_refs: Vec<String>,
    /// True when the lineage row is present in the promoted build.
    pub promoted_build_surface: bool,
    /// True when the lineage row shows build identity.
    pub shows_build_identity: bool,
    /// True when the lineage row shows environment fingerprint.
    pub shows_environment_fingerprint: bool,
    /// True when the lineage row shows capture mode.
    pub shows_capture_mode: bool,
    /// True when the lineage row shows mapping quality.
    pub shows_mapping_quality: bool,
    /// True when the lineage row shows freshness.
    pub shows_freshness: bool,
}

/// One capture source row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureSourceRow {
    /// Stable source row id.
    pub source_id: String,
    /// Human-readable title.
    pub title: String,
    /// Capture source class.
    pub source_class: CaptureSourceClass,
    /// Target identity (process, container, runtime, or provider).
    pub target_identity: String,
    /// Trust label.
    pub trust_label: String,
    /// Data class label.
    pub data_class_label: String,
    /// True when the capture source is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// One save/share scope row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveShareScopeRow {
    /// Stable scope row id.
    pub scope_id: String,
    /// Human-readable title.
    pub title: String,
    /// Scope kind.
    pub scope_kind: SaveShareScopeKind,
    /// True when the scope shows redaction mode.
    pub shows_redaction_mode: bool,
    /// True when the scope shows destination class.
    pub shows_destination_class: bool,
    /// True when the scope row is present in the promoted build.
    pub promoted_build_surface: bool,
}

/// Checked-in proof bundle for one surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffQualificationProof {
    /// Packet id.
    pub packet_id: String,
    /// Packet ref path.
    pub packet_ref: String,
    /// Proof index ref path.
    pub proof_index_ref: String,
    /// Captured-at timestamp.
    pub captured_at: String,
    /// Evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Summary projected onto help, release, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffQualificationSummary {
    /// Total number of handoff bar rows.
    pub handoff_bar_count: usize,
    /// Total number of artifact lineage rows.
    pub artifact_lineage_count: usize,
    /// Total number of capture source rows.
    pub capture_source_count: usize,
    /// Total number of save/share scope rows.
    pub save_share_scope_count: usize,
    /// Number of rows claiming stable.
    pub stable_count: usize,
    /// Number of rows below stable.
    pub below_stable_count: usize,
    /// True when every row has a non-empty disclosure ref if below stable.
    pub all_below_stable_have_disclosure: bool,
    /// Number of lineage rows that are usable.
    pub usable_lineage_count: usize,
    /// Number of save/share scope rows that show redaction mode and destination class.
    pub honest_scope_count: usize,
}

/// Guard set for an evidence-handoff surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffSurfaceGuardSet {
    /// Originating run is visible.
    pub origin_visible: bool,
    /// Build ID is visible.
    pub build_id_visible: bool,
    /// Commit/revision is visible.
    pub commit_visible: bool,
    /// Capture source is visible.
    pub capture_source_visible: bool,
    /// Save/share scope is visible.
    pub save_share_scope_visible: bool,
    /// Lineage state is visible.
    pub lineage_state_visible: bool,
    /// Lineage detail is visible.
    pub lineage_detail_visible: bool,
    /// Degraded-state label is visible when applicable.
    pub degraded_state_label_visible: bool,
}

/// One surface qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffSurfaceQualificationRow {
    /// Surface id.
    pub surface_id: String,
    /// Surface title.
    pub title: String,
    /// Surface kind.
    pub surface_kind: EvidenceHandoffSurfaceKind,
    /// True when the surface is present in the promoted build.
    pub promoted_build_surface: bool,
    /// Claim label.
    pub claim_label: EvidenceHandoffQualificationLabel,
    /// Displayed label (may differ from claim when narrowed).
    pub displayed_label: String,
    /// Qualification proof bundle.
    pub qualification_packet: EvidenceHandoffQualificationProof,
    /// Guard set.
    pub guards: EvidenceHandoffSurfaceGuardSet,
    /// True when the surface downgrades if required guards are missing.
    pub downgrade_if_missing: bool,
    /// Rationale string.
    pub rationale: String,
}

/// The checked-in evidence-handoff qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffQualificationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record kind.
    pub record_kind: String,
    /// Packet id.
    pub packet_id: String,
    /// As-of timestamp.
    pub as_of: String,
    /// Release doc ref.
    pub release_doc_ref: String,
    /// Help doc ref.
    pub help_doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Surface qualification rows.
    pub surfaces: Vec<EvidenceHandoffSurfaceQualificationRow>,
    /// Evidence handoff bar rows.
    pub handoff_bars: Vec<EvidenceHandoffBarRow>,
    /// Artifact lineage rows.
    pub artifact_lineages: Vec<ArtifactLineageRow>,
    /// Capture source rows.
    pub capture_sources: Vec<CaptureSourceRow>,
    /// Save/share scope rows.
    pub save_share_scopes: Vec<SaveShareScopeRow>,
    /// Summary.
    pub summary: EvidenceHandoffQualificationSummary,
}

impl EvidenceHandoffQualificationPacket {
    /// Computes the summary from current rows.
    pub fn computed_summary(&self) -> EvidenceHandoffQualificationSummary {
        let stable_count = self
            .surfaces
            .iter()
            .filter(|s| s.claim_label.is_stable())
            .count();
        let below_stable_count = self.surfaces.len().saturating_sub(stable_count);
        let all_below_stable_have_disclosure = self
            .surfaces
            .iter()
            .filter(|s| !s.claim_label.is_stable())
            .all(|s| !s.rationale.is_empty());
        let usable_lineage_count = self
            .artifact_lineages
            .iter()
            .filter(|l| l.freshness.is_usable())
            .count();
        let honest_scope_count = self
            .save_share_scopes
            .iter()
            .filter(|s| s.shows_redaction_mode && s.shows_destination_class)
            .count();

        EvidenceHandoffQualificationSummary {
            handoff_bar_count: self.handoff_bars.len(),
            artifact_lineage_count: self.artifact_lineages.len(),
            capture_source_count: self.capture_sources.len(),
            save_share_scope_count: self.save_share_scopes.len(),
            stable_count,
            below_stable_count,
            all_below_stable_have_disclosure,
            usable_lineage_count,
            honest_scope_count,
        }
    }

    /// Validates the packet and returns any violations.
    pub fn validate(&self) -> Vec<EvidenceHandoffQualificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EVIDENCE_HANDOFF_QUALIFICATION_SCHEMA_VERSION {
            violations.push(EvidenceHandoffQualificationViolation::SchemaVersion {
                expected: EVIDENCE_HANDOFF_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }

        if self.record_kind != EVIDENCE_HANDOFF_QUALIFICATION_RECORD_KIND {
            violations.push(EvidenceHandoffQualificationViolation::RecordKind {
                expected: EVIDENCE_HANDOFF_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        let mut surface_ids = BTreeSet::new();
        for surface in &self.surfaces {
            if !surface_ids.insert(surface.surface_id.clone()) {
                violations.push(EvidenceHandoffQualificationViolation::DuplicateId {
                    kind: EvidenceHandoffQualificationViolationKind::Surface,
                    id: surface.surface_id.clone(),
                });
            }
            if surface.promoted_build_surface
                && surface.claim_label.is_stable()
                && (!surface.guards.origin_visible
                    || !surface.guards.build_id_visible
                    || !surface.guards.commit_visible
                    || !surface.guards.capture_source_visible
                    || !surface.guards.save_share_scope_visible
                    || !surface.guards.lineage_state_visible
                    || !surface.guards.lineage_detail_visible
                    || !surface.guards.degraded_state_label_visible)
            {
                violations.push(EvidenceHandoffQualificationViolation::IncompleteGuardSet {
                    surface_id: surface.surface_id.clone(),
                });
            }
        }

        let mut handoff_bar_ids = BTreeSet::new();
        for bar in &self.handoff_bars {
            if !handoff_bar_ids.insert(bar.handoff_bar_id.clone()) {
                violations.push(EvidenceHandoffQualificationViolation::DuplicateId {
                    kind: EvidenceHandoffQualificationViolationKind::HandoffBar,
                    id: bar.handoff_bar_id.clone(),
                });
            }
            if bar.handoff_bar_id.trim().is_empty()
                || bar.title.trim().is_empty()
                || bar.originating_run_ref.trim().is_empty()
                || bar.artifact_build_id.trim().is_empty()
                || bar.commit_or_revision.trim().is_empty()
                || bar.capture_source_ref.trim().is_empty()
                || bar.save_share_scope_ref.trim().is_empty()
                || bar.lineage_ref.trim().is_empty()
            {
                violations.push(
                    EvidenceHandoffQualificationViolation::IncompleteHandoffBar {
                        handoff_bar_id: bar.handoff_bar_id.clone(),
                    },
                );
            }
            if !bar.shows_origin
                || !bar.shows_build_id
                || !bar.shows_commit
                || !bar.shows_capture_source
                || !bar.shows_save_share_scope
                || !bar.shows_lineage_state
                || !bar.shows_lineage_detail
            {
                violations.push(
                    EvidenceHandoffQualificationViolation::HandoffBarMissingTruthLabels {
                        handoff_bar_id: bar.handoff_bar_id.clone(),
                    },
                );
            }
        }

        let mut lineage_ids = BTreeSet::new();
        for lineage in &self.artifact_lineages {
            if !lineage_ids.insert(lineage.lineage_id.clone()) {
                violations.push(EvidenceHandoffQualificationViolation::DuplicateId {
                    kind: EvidenceHandoffQualificationViolationKind::ArtifactLineage,
                    id: lineage.lineage_id.clone(),
                });
            }
            if lineage.lineage_id.trim().is_empty()
                || lineage.title.trim().is_empty()
                || lineage.source_run_ref.trim().is_empty()
                || lineage.build_identity_ref.trim().is_empty()
                || lineage.environment_fingerprint_ref.trim().is_empty()
                || lineage.capture_mode_ref.trim().is_empty()
                || lineage.mapping_quality_ref.trim().is_empty()
            {
                violations.push(
                    EvidenceHandoffQualificationViolation::IncompleteArtifactLineage {
                        lineage_id: lineage.lineage_id.clone(),
                    },
                );
            }
            if !lineage.shows_build_identity
                || !lineage.shows_environment_fingerprint
                || !lineage.shows_capture_mode
                || !lineage.shows_mapping_quality
                || !lineage.shows_freshness
            {
                violations.push(
                    EvidenceHandoffQualificationViolation::ArtifactLineageMissingTruthLabels {
                        lineage_id: lineage.lineage_id.clone(),
                    },
                );
            }
        }

        let mut source_ids = BTreeSet::new();
        for source in &self.capture_sources {
            if !source_ids.insert(source.source_id.clone()) {
                violations.push(EvidenceHandoffQualificationViolation::DuplicateId {
                    kind: EvidenceHandoffQualificationViolationKind::CaptureSource,
                    id: source.source_id.clone(),
                });
            }
            if source.source_id.trim().is_empty()
                || source.title.trim().is_empty()
                || source.target_identity.trim().is_empty()
                || source.trust_label.trim().is_empty()
                || source.data_class_label.trim().is_empty()
            {
                violations.push(
                    EvidenceHandoffQualificationViolation::IncompleteCaptureSource {
                        source_id: source.source_id.clone(),
                    },
                );
            }
        }

        let mut scope_ids = BTreeSet::new();
        for scope in &self.save_share_scopes {
            if !scope_ids.insert(scope.scope_id.clone()) {
                violations.push(EvidenceHandoffQualificationViolation::DuplicateId {
                    kind: EvidenceHandoffQualificationViolationKind::SaveShareScope,
                    id: scope.scope_id.clone(),
                });
            }
            if scope.scope_id.trim().is_empty() || scope.title.trim().is_empty() {
                violations.push(
                    EvidenceHandoffQualificationViolation::IncompleteSaveShareScope {
                        scope_id: scope.scope_id.clone(),
                    },
                );
            }
            if !scope.shows_redaction_mode || !scope.shows_destination_class {
                violations.push(
                    EvidenceHandoffQualificationViolation::SaveShareScopeMissingBehavior {
                        scope_id: scope.scope_id.clone(),
                    },
                );
            }
        }

        // Cross-reference: every handoff bar must point to a known capture source.
        let source_id_set: BTreeSet<String> = self
            .capture_sources
            .iter()
            .map(|s| s.source_id.clone())
            .collect();
        for bar in &self.handoff_bars {
            if !source_id_set.contains(&bar.capture_source_ref) {
                violations.push(
                    EvidenceHandoffQualificationViolation::HandoffBarCaptureSourceRefUnknown {
                        handoff_bar_id: bar.handoff_bar_id.clone(),
                        capture_source_ref: bar.capture_source_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every handoff bar must point to a known save/share scope.
        let scope_id_set: BTreeSet<String> = self
            .save_share_scopes
            .iter()
            .map(|s| s.scope_id.clone())
            .collect();
        for bar in &self.handoff_bars {
            if !scope_id_set.contains(&bar.save_share_scope_ref) {
                violations.push(
                    EvidenceHandoffQualificationViolation::HandoffBarSaveShareScopeRefUnknown {
                        handoff_bar_id: bar.handoff_bar_id.clone(),
                        save_share_scope_ref: bar.save_share_scope_ref.clone(),
                    },
                );
            }
        }

        // Cross-reference: every handoff bar must point to a known artifact lineage.
        let lineage_id_set: BTreeSet<String> = self
            .artifact_lineages
            .iter()
            .map(|l| l.lineage_id.clone())
            .collect();
        for bar in &self.handoff_bars {
            if !lineage_id_set.contains(&bar.lineage_ref) {
                violations.push(
                    EvidenceHandoffQualificationViolation::HandoffBarLineageRefUnknown {
                        handoff_bar_id: bar.handoff_bar_id.clone(),
                        lineage_ref: bar.lineage_ref.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(EvidenceHandoffQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in evidence-handoff qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_evidence_handoff_qualification(
) -> Result<EvidenceHandoffQualificationPacket, serde_json::Error> {
    serde_json::from_str(EVIDENCE_HANDOFF_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceHandoffQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Handoff bar rows.
    HandoffBar,
    /// Artifact lineage rows.
    ArtifactLineage,
    /// Capture source rows.
    CaptureSource,
    /// Save/share scope rows.
    SaveShareScope,
}

impl fmt::Display for EvidenceHandoffQualificationViolationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface => write!(f, "surface"),
            Self::HandoffBar => write!(f, "handoff_bar"),
            Self::ArtifactLineage => write!(f, "artifact_lineage"),
            Self::CaptureSource => write!(f, "capture_source"),
            Self::SaveShareScope => write!(f, "save_share_scope"),
        }
    }
}

/// Validation failure for evidence-handoff qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceHandoffQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Record kind does not match the model.
    RecordKind {
        /// Expected record kind.
        expected: String,
        /// Actual record kind.
        actual: String,
    },
    /// IDs must be unique inside an object family.
    DuplicateId {
        /// Kind of object family.
        kind: EvidenceHandoffQualificationViolationKind,
        /// Duplicate id.
        id: String,
    },
    /// A surface with a stable claim has an incomplete guard set.
    IncompleteGuardSet {
        /// Surface id.
        surface_id: String,
    },
    /// A handoff bar row is incomplete.
    IncompleteHandoffBar {
        /// Handoff bar id.
        handoff_bar_id: String,
    },
    /// A handoff bar row must show origin, build ID, commit, capture source, save/share scope, and lineage state.
    HandoffBarMissingTruthLabels {
        /// Handoff bar id.
        handoff_bar_id: String,
    },
    /// An artifact lineage row is incomplete.
    IncompleteArtifactLineage {
        /// Lineage id.
        lineage_id: String,
    },
    /// An artifact lineage row must show build identity, environment fingerprint, capture mode, mapping quality, and freshness.
    ArtifactLineageMissingTruthLabels {
        /// Lineage id.
        lineage_id: String,
    },
    /// A capture source row is incomplete.
    IncompleteCaptureSource {
        /// Source id.
        source_id: String,
    },
    /// A save/share scope row is incomplete.
    IncompleteSaveShareScope {
        /// Scope id.
        scope_id: String,
    },
    /// A save/share scope row must show redaction mode and destination class.
    SaveShareScopeMissingBehavior {
        /// Scope id.
        scope_id: String,
    },
    /// A handoff bar references an unknown capture source.
    HandoffBarCaptureSourceRefUnknown {
        /// Handoff bar id.
        handoff_bar_id: String,
        /// Unknown capture source ref.
        capture_source_ref: String,
    },
    /// A handoff bar references an unknown save/share scope.
    HandoffBarSaveShareScopeRefUnknown {
        /// Handoff bar id.
        handoff_bar_id: String,
        /// Unknown save/share scope ref.
        save_share_scope_ref: String,
    },
    /// A handoff bar references an unknown artifact lineage.
    HandoffBarLineageRefUnknown {
        /// Handoff bar id.
        handoff_bar_id: String,
        /// Unknown lineage ref.
        lineage_ref: String,
    },
    /// Computed summary does not match the stored summary.
    SummaryMismatch,
}

impl fmt::Display for EvidenceHandoffQualificationViolation {
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
            Self::DuplicateId { kind, id } => {
                write!(f, "duplicate {kind} id: {id}")
            }
            Self::IncompleteGuardSet { surface_id } => {
                write!(
                    f,
                    "surface {surface_id} claims stable but guard set is incomplete"
                )
            }
            Self::IncompleteHandoffBar { handoff_bar_id } => {
                write!(f, "incomplete handoff bar row: {handoff_bar_id}")
            }
            Self::HandoffBarMissingTruthLabels { handoff_bar_id } => {
                write!(
                    f,
                    "handoff bar row {handoff_bar_id} must show origin, build ID, commit, capture source, save/share scope, and lineage state"
                )
            }
            Self::IncompleteArtifactLineage { lineage_id } => {
                write!(f, "incomplete artifact lineage row: {lineage_id}")
            }
            Self::ArtifactLineageMissingTruthLabels { lineage_id } => {
                write!(
                    f,
                    "artifact lineage row {lineage_id} must show build identity, environment fingerprint, capture mode, mapping quality, and freshness"
                )
            }
            Self::IncompleteCaptureSource { source_id } => {
                write!(f, "incomplete capture source row: {source_id}")
            }
            Self::IncompleteSaveShareScope { scope_id } => {
                write!(f, "incomplete save/share scope row: {scope_id}")
            }
            Self::SaveShareScopeMissingBehavior { scope_id } => {
                write!(
                    f,
                    "save/share scope row {scope_id} must show redaction mode and destination class"
                )
            }
            Self::HandoffBarCaptureSourceRefUnknown {
                handoff_bar_id,
                capture_source_ref,
            } => {
                write!(
                    f,
                    "handoff bar {handoff_bar_id} references unknown capture source {capture_source_ref}"
                )
            }
            Self::HandoffBarSaveShareScopeRefUnknown {
                handoff_bar_id,
                save_share_scope_ref,
            } => {
                write!(
                    f,
                    "handoff bar {handoff_bar_id} references unknown save/share scope {save_share_scope_ref}"
                )
            }
            Self::HandoffBarLineageRefUnknown {
                handoff_bar_id,
                lineage_ref,
            } => {
                write!(
                    f,
                    "handoff bar {handoff_bar_id} references unknown artifact lineage {lineage_ref}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "computed summary does not match stored summary")
            }
        }
    }
}

impl Error for EvidenceHandoffQualificationViolation {}
