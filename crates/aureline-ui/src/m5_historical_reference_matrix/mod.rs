//! Frozen M5 historical-reference, archived-snapshot, imported/offline-evidence, and live-target-handoff matrix.
//!
//! This module locks Aureline's non-live-evidence object model — the retirement / last-supported snapshots,
//! captured support / export evidence bundles, archived runbook execution packets, imported / offline route
//! evidence, and review / incident snapshots that no longer point at live mutable state — into one
//! export-safe packet. Every covered object class is named once here and constrained by the same shared
//! historical-reference role taxonomy (snapshot_labeling, capture_time_attribution, provenance_attribution,
//! mutation_blocked_posture, live_target_handoff, imported_offline_disclosure, expiry_removal_handling), the
//! same required visible state (snapshot label, capture time, provenance, live-target availability,
//! imported / offline status, mutation-blocked posture, and expiry / removed handling), the same
//! no-archived-or-imported-evidence-looks-live-writable-or-current-by-omission rule, the same
//! no-live-target-reopened-from-a-snapshot-without-validating-identity-trust-route-and-authority rule, the same
//! no-dead-linking-an-expired-or-removed-artifact-when-metadata-provenance-or-cleanup-state-can-be-shown rule,
//! the same non-live-evidence-stays-joined-to-capture-time-provenance-retention-state-and-live-target-mismatch
//! rule, and the same no-snapshot-or-imported-packet-presented-as-a-current-live-object-or-reopened-through-an-ambiguous-route
//! rule regardless of the surface that renders it.
//!
//! The matrix makes captured / archived and imported / offline evidence mechanically distinct from ordinary
//! live objects, read-only cached current state, and restore-capable workspaces (see
//! [`M5HistoricalReferenceEvidenceState`]) so downstream automation can key off the non-live evidence state
//! rather than guessing from a stale label. It does not build every consumer — later rows implement the
//! archive viewers, compare / open-live-target plumbing, and support / help / review / companion consumers —
//! it is the shared reusable non-live-evidence contract those rows consume, and it binds back to the
//! already-landed stable-proof-index and migration-task-row packets so historical-reference truth is not
//! split across scattered internal notes. The controlled vocabularies are frozen in one self-describing
//! [`M5HistoricalReferenceVocabularySet`] rather than minted per surface. Raw secret values and private
//! endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_historical_reference_matrix,
    seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed,
    seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed,
    M5_HISTORICAL_REFERENCE_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5HistoricalReferenceMatrixPacket`].
pub const M5_HISTORICAL_REFERENCE_MATRIX_RECORD_KIND: &str =
    "freeze_m5_historical_reference_archived_snapshot_imported_offline_evidence_and_live_target_handoff_matrix";

/// Schema version for M5 historical-reference matrix records.
pub const M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined historical-reference matrix schema.
pub const M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF: &str =
    "schemas/program/m5-historical-reference-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF: &str =
    "docs/support/m5-historical-evidence-ops.md";

/// Repo-relative path of the canonical historical-snapshot-descriptor domain schema (retirement snapshot and
/// support / export evidence: snapshot label, capture time, provenance, retention / removal state, and the
/// mutation-blocked posture of a captured, non-live object).
pub const M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-historical-snapshot-descriptor.schema.json";

/// Repo-relative path of the canonical live-target-handoff domain schema (archived runbook packet and
/// review / incident snapshot: the explicit open-live-target action plus the target identity, trust, route,
/// and authority it validates before reopening a current object, or the metadata-only exit when none remains).
pub const M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-live-target-handoff.schema.json";

/// Repo-relative path of the canonical imported-offline-evidence-state domain schema (imported / offline
/// route evidence: the imported / offline disclosure, controlled restore-fidelity vocabulary, and any current
/// live-route mismatch so imported data never masquerades as current live route truth).
pub const M5_IMPORTED_OFFLINE_EVIDENCE_STATE_DOMAIN_SCHEMA_REF: &str =
    "schemas/program/m5-imported-offline-evidence-state.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_HISTORICAL_REFERENCE_FIXTURE_DIR: &str = "fixtures/recovery/m5-historical-snapshots";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HISTORICAL_REFERENCE_ARTIFACT_REF: &str =
    "artifacts/support/m5-historical-evidence/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_HISTORICAL_REFERENCE_CSV_REF: &str =
    "artifacts/support/m5-historical-evidence/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_HISTORICAL_REFERENCE_REPORT_REF: &str =
    "artifacts/program/m5-historical-reference-matrix.md";

/// Repo-relative path of the checked historical-evidence-health dashboard.
pub const M5_HISTORICAL_REFERENCE_DASHBOARD_REF: &str =
    "dashboards/m5-historical-evidence-health.json";

/// One of the five governed historical-reference object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceObject {
    /// A retirement / last-supported snapshot of an object that no longer points at live mutable state.
    RetirementSnapshot,
    /// A captured support / export evidence bundle preserved for historical inspection.
    SupportExportEvidence,
    /// An archived runbook execution packet kept for historical / export purposes only.
    ArchivedRunbookPacket,
    /// Imported / offline route evidence that must never masquerade as current live route truth.
    ImportedOfflineRouteEvidence,
    /// A review / incident snapshot that no longer points at a live mutable object.
    ReviewIncidentSnapshot,
}

impl M5HistoricalReferenceObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RetirementSnapshot,
        Self::SupportExportEvidence,
        Self::ArchivedRunbookPacket,
        Self::ImportedOfflineRouteEvidence,
        Self::ReviewIncidentSnapshot,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetirementSnapshot => "retirement_snapshot",
            Self::SupportExportEvidence => "support_export_evidence",
            Self::ArchivedRunbookPacket => "archived_runbook_packet",
            Self::ImportedOfflineRouteEvidence => "imported_offline_route_evidence",
            Self::ReviewIncidentSnapshot => "review_incident_snapshot",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's historical-snapshot-descriptor, live-target-handoff, or imported-offline-evidence-state meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::RetirementSnapshot | Self::SupportExportEvidence => {
                M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF
            }
            Self::ArchivedRunbookPacket | Self::ReviewIncidentSnapshot => {
                M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF
            }
            Self::ImportedOfflineRouteEvidence => {
                M5_IMPORTED_OFFLINE_EVIDENCE_STATE_DOMAIN_SCHEMA_REF
            }
        }
    }

    /// `true` when this class must name a controlled retirement snapshot role.
    pub const fn declares_retirement_snapshot_roles(self) -> bool {
        matches!(self, Self::RetirementSnapshot)
    }

    /// `true` when this class must name a controlled support export evidence role.
    pub const fn declares_support_export_evidence_roles(self) -> bool {
        matches!(self, Self::SupportExportEvidence)
    }

    /// `true` when this class must name a controlled archived runbook packet role.
    pub const fn declares_archived_runbook_packet_roles(self) -> bool {
        matches!(self, Self::ArchivedRunbookPacket)
    }

    /// `true` when this class must name a controlled imported offline route evidence role.
    pub const fn declares_imported_offline_route_evidence_roles(self) -> bool {
        matches!(self, Self::ImportedOfflineRouteEvidence)
    }

    /// `true` when this class must name a controlled review incident snapshot role.
    pub const fn declares_review_incident_snapshot_roles(self) -> bool {
        matches!(self, Self::ReviewIncidentSnapshot)
    }
}

/// The single controlled historical-reference role vocabulary every shell, help, docs, support, review, runbook-archive, or companion/export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceRole {
    /// The captured-evidence / archived-snapshot label that keeps the object visibly non-live.
    SnapshotLabeling,
    /// The capture time the evidence is attributed to.
    CaptureTimeAttribution,
    /// The capture context / provenance the evidence is attributed to.
    ProvenanceAttribution,
    /// The read-only, non-authoritative-for-mutation posture the object holds.
    MutationBlockedPosture,
    /// The explicit open-live-target handoff (or metadata-only exit) back to a current object.
    LiveTargetHandoff,
    /// The imported / offline-evidence-only disclosure the object carries.
    ImportedOfflineDisclosure,
    /// The expired / removed-artifact handling that shows metadata instead of a dead link.
    ExpiryRemovalHandling,
}

impl M5HistoricalReferenceRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SnapshotLabeling,
        Self::CaptureTimeAttribution,
        Self::ProvenanceAttribution,
        Self::MutationBlockedPosture,
        Self::LiveTargetHandoff,
        Self::ImportedOfflineDisclosure,
        Self::ExpiryRemovalHandling,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotLabeling => "snapshot_labeling",
            Self::CaptureTimeAttribution => "capture_time_attribution",
            Self::ProvenanceAttribution => "provenance_attribution",
            Self::MutationBlockedPosture => "mutation_blocked_posture",
            Self::LiveTargetHandoff => "live_target_handoff",
            Self::ImportedOfflineDisclosure => "imported_offline_disclosure",
            Self::ExpiryRemovalHandling => "expiry_removal_handling",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as non-live evidence (`snapshot_labeling`, `capture_time_attribution`,
    /// `provenance_attribution`, `mutation_blocked_posture`). The contextual roles (`live_target_handoff`,
    /// `imported_offline_disclosure`, `expiry_removal_handling`) apply where the object class calls for them.
    pub const fn must_be_present_before_surfacing_as_non_live_evidence(self) -> bool {
        matches!(
            self,
            Self::SnapshotLabeling
                | Self::CaptureTimeAttribution
                | Self::ProvenanceAttribution
                | Self::MutationBlockedPosture
        )
    }
}

/// Evidence state that makes captured / archived and imported / offline evidence mechanically distinct from ordinary live objects, read-only cached current state, and restore-capable workspaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceEvidenceState {
    /// An ordinary editable / live object, not historical evidence.
    LiveObject,
    /// A read-only cached view of current live state, not historical evidence.
    CachedCurrentState,
    /// A restore-capable workspace that can still mutate, not historical evidence.
    RestoreCapableWorkspace,
    /// Captured evidence / archived snapshot: visibly non-live and non-authoritative for mutation.
    ArchivedSnapshot,
    /// Imported / offline evidence only: visibly non-live and attributable to its import context.
    ImportedOfflineEvidence,
}

impl M5HistoricalReferenceEvidenceState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveObject,
        Self::CachedCurrentState,
        Self::RestoreCapableWorkspace,
        Self::ArchivedSnapshot,
        Self::ImportedOfflineEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveObject => "live_object",
            Self::CachedCurrentState => "cached_current_state",
            Self::RestoreCapableWorkspace => "restore_capable_workspace",
            Self::ArchivedSnapshot => "archived_snapshot",
            Self::ImportedOfflineEvidence => "imported_offline_evidence",
        }
    }
    /// `true` only for the captured / archived and imported / offline evidence states, so downstream
    /// automation can key off non-live evidence rather than confusing it with a live object, a read-only
    /// cached current view, or a restore-capable workspace.
    pub const fn is_non_live_evidence(self) -> bool {
        matches!(self, Self::ArchivedSnapshot | Self::ImportedOfflineEvidence)
    }
}

/// Controlled non-live-evidence role for a retirement / last-supported snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceRetirementSnapshotRole {
    /// Captured-evidence / archived-snapshot label shown so the object never looks live.
    SnapshotLabelShown,
    /// Capture time recorded for the snapshot.
    CaptureTimeRecorded,
    /// Last-supported provenance / capture context attributed.
    LastSupportedProvenanceAttributed,
    /// Explicit open-live-target handoff (or metadata-only exit) offered.
    LiveTargetHandoffOffered,
    /// A role bound to the single historical-reference registry.
    BoundToHistoricalReferenceRegistry,
    /// Mutation of a retirement snapshot in place, which is disallowed.
    MutationOnSnapshotDisallowed,
}

impl M5HistoricalReferenceRetirementSnapshotRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SnapshotLabelShown,
        Self::CaptureTimeRecorded,
        Self::LastSupportedProvenanceAttributed,
        Self::LiveTargetHandoffOffered,
        Self::BoundToHistoricalReferenceRegistry,
        Self::MutationOnSnapshotDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotLabelShown => "snapshot_label_shown",
            Self::CaptureTimeRecorded => "capture_time_recorded",
            Self::LastSupportedProvenanceAttributed => "last_supported_provenance_attributed",
            Self::LiveTargetHandoffOffered => "live_target_handoff_offered",
            Self::BoundToHistoricalReferenceRegistry => "bound_to_historical_reference_registry",
            Self::MutationOnSnapshotDisallowed => "mutation_on_snapshot_disallowed",
        }
    }
}

/// Controlled non-live-evidence role for a captured support / export evidence bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceSupportExportEvidenceRole {
    /// Support / export evidence labeled as a captured snapshot.
    EvidenceSnapshotLabeled,
    /// Capture context / provenance attributed for the evidence.
    CaptureContextAttributed,
    /// Retention / expiry / removal state shown for the evidence.
    ExportRetentionStateShown,
    /// Metadata-only inspection exit offered when no live target remains.
    MetadataOnlyExitOffered,
    /// A role bound to the single historical-reference registry.
    BoundToHistoricalReferenceRegistry,
    /// Treating captured export evidence as editable / authoritative, which is disallowed.
    EditableExportEvidenceDisallowed,
}

impl M5HistoricalReferenceSupportExportEvidenceRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvidenceSnapshotLabeled,
        Self::CaptureContextAttributed,
        Self::ExportRetentionStateShown,
        Self::MetadataOnlyExitOffered,
        Self::BoundToHistoricalReferenceRegistry,
        Self::EditableExportEvidenceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceSnapshotLabeled => "evidence_snapshot_labeled",
            Self::CaptureContextAttributed => "capture_context_attributed",
            Self::ExportRetentionStateShown => "export_retention_state_shown",
            Self::MetadataOnlyExitOffered => "metadata_only_exit_offered",
            Self::BoundToHistoricalReferenceRegistry => "bound_to_historical_reference_registry",
            Self::EditableExportEvidenceDisallowed => "editable_export_evidence_disallowed",
        }
    }
}

/// Controlled non-live-evidence role for an archived runbook execution packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceArchivedRunbookPacketRole {
    /// Archived runbook execution labeled as a historical packet.
    ArchivedRunLabeled,
    /// Capture time recorded for the archived run.
    RunCaptureTimeRecorded,
    /// Provenance / capture context attributed for the archived run.
    RunProvenanceAttributed,
    /// Open-live-run handoff validated for target identity, trust, route, and authority.
    OpenLiveRunHandoffValidated,
    /// A role bound to the single historical-reference registry.
    BoundToHistoricalReferenceRegistry,
    /// Re-running an archived packet in place without validated handoff, which is disallowed.
    RerunArchivedPacketInPlaceDisallowed,
}

impl M5HistoricalReferenceArchivedRunbookPacketRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ArchivedRunLabeled,
        Self::RunCaptureTimeRecorded,
        Self::RunProvenanceAttributed,
        Self::OpenLiveRunHandoffValidated,
        Self::BoundToHistoricalReferenceRegistry,
        Self::RerunArchivedPacketInPlaceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchivedRunLabeled => "archived_run_labeled",
            Self::RunCaptureTimeRecorded => "run_capture_time_recorded",
            Self::RunProvenanceAttributed => "run_provenance_attributed",
            Self::OpenLiveRunHandoffValidated => "open_live_run_handoff_validated",
            Self::BoundToHistoricalReferenceRegistry => "bound_to_historical_reference_registry",
            Self::RerunArchivedPacketInPlaceDisallowed => {
                "rerun_archived_packet_in_place_disallowed"
            }
        }
    }
}

/// Controlled non-live-evidence role for imported / offline route evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceImportedOfflineRouteEvidenceRole {
    /// Imported / offline-evidence-only warning shown so it never reads as live route truth.
    ImportedOfflineWarningShown,
    /// Import context / provenance attributed for the evidence.
    ImportContextAttributed,
    /// Controlled restore-fidelity vocabulary disclosed for the evidence.
    RestoreFidelityDisclosed,
    /// Any current live-route mismatch flagged against the imported evidence.
    LiveRouteMismatchFlagged,
    /// A role bound to the single historical-reference registry.
    BoundToHistoricalReferenceRegistry,
    /// Treating imported / offline route evidence as a current live route, which is disallowed.
    ImportedRouteTreatedAsLiveDisallowed,
}

impl M5HistoricalReferenceImportedOfflineRouteEvidenceRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ImportedOfflineWarningShown,
        Self::ImportContextAttributed,
        Self::RestoreFidelityDisclosed,
        Self::LiveRouteMismatchFlagged,
        Self::BoundToHistoricalReferenceRegistry,
        Self::ImportedRouteTreatedAsLiveDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedOfflineWarningShown => "imported_offline_warning_shown",
            Self::ImportContextAttributed => "import_context_attributed",
            Self::RestoreFidelityDisclosed => "restore_fidelity_disclosed",
            Self::LiveRouteMismatchFlagged => "live_route_mismatch_flagged",
            Self::BoundToHistoricalReferenceRegistry => "bound_to_historical_reference_registry",
            Self::ImportedRouteTreatedAsLiveDisallowed => {
                "imported_route_treated_as_live_disallowed"
            }
        }
    }
}

/// Controlled non-live-evidence role for a review / incident snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceReviewIncidentSnapshotRole {
    /// Review / incident snapshot labeled as captured, non-live evidence.
    IncidentSnapshotLabeled,
    /// Capture time recorded for the snapshot.
    SnapshotCaptureTimeRecorded,
    /// Provenance / capture context attributed for the snapshot.
    IncidentProvenanceAttributed,
    /// Open-current-object handoff validated for target identity, trust, route, and authority.
    OpenCurrentObjectHandoffValidated,
    /// A role bound to the single historical-reference registry.
    BoundToHistoricalReferenceRegistry,
    /// Mutating a review / incident snapshot as if it were the live object, which is disallowed.
    MutationOnReviewSnapshotDisallowed,
}

impl M5HistoricalReferenceReviewIncidentSnapshotRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IncidentSnapshotLabeled,
        Self::SnapshotCaptureTimeRecorded,
        Self::IncidentProvenanceAttributed,
        Self::OpenCurrentObjectHandoffValidated,
        Self::BoundToHistoricalReferenceRegistry,
        Self::MutationOnReviewSnapshotDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentSnapshotLabeled => "incident_snapshot_labeled",
            Self::SnapshotCaptureTimeRecorded => "snapshot_capture_time_recorded",
            Self::IncidentProvenanceAttributed => "incident_provenance_attributed",
            Self::OpenCurrentObjectHandoffValidated => "open_current_object_handoff_validated",
            Self::BoundToHistoricalReferenceRegistry => "bound_to_historical_reference_registry",
            Self::MutationOnReviewSnapshotDisallowed => "mutation_on_review_snapshot_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a historical-reference object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceSurfaceFamily {
    /// The shell / archive-viewer surface.
    Shell,
    /// The help / docs surface.
    HelpDocs,
    /// The support surface.
    Support,
    /// The review / incident surface.
    ReviewIncident,
    /// The runbook-archive surface.
    RunbookArchive,
    /// The companion / export surface.
    CompanionExport,
}

impl M5HistoricalReferenceSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::HelpDocs,
        Self::Support,
        Self::ReviewIncident,
        Self::RunbookArchive,
        Self::CompanionExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::HelpDocs => "help_docs",
            Self::Support => "support",
            Self::ReviewIncident => "review_incident",
            Self::RunbookArchive => "runbook_archive",
            Self::CompanionExport => "companion_export",
        }
    }
}

/// Capture-lifecycle stage a class passes through from live capture to a labeled, provenance-attributed, live-target-resolved, and retention/removal-marked non-live reference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceCaptureLifecycleStage {
    /// The evidence-captured stage: a snapshot is taken from a live object.
    Captured,
    /// The snapshot-labeled stage: the captured evidence is marked visibly non-live.
    SnapshotLabeled,
    /// The provenance-attributed stage: capture time and capture context are attached.
    ProvenanceAttributed,
    /// The live-target-resolved stage: live-target availability is resolved to a handoff or a metadata-only exit.
    LiveTargetResolved,
    /// The retention-or-removal-marked stage: retention, expiry, or removal state is recorded.
    RetentionOrRemovalMarked,
}

impl M5HistoricalReferenceCaptureLifecycleStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Captured,
        Self::SnapshotLabeled,
        Self::ProvenanceAttributed,
        Self::LiveTargetResolved,
        Self::RetentionOrRemovalMarked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Captured => "captured",
            Self::SnapshotLabeled => "snapshot_labeled",
            Self::ProvenanceAttributed => "provenance_attributed",
            Self::LiveTargetResolved => "live_target_resolved",
            Self::RetentionOrRemovalMarked => "retention_or_removal_marked",
        }
    }
}

/// Subsystem that consumes a class's historical-reference projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceConsumerSurface {
    /// The shell / archive viewer.
    Shell,
    /// The help / docs surface.
    HelpDocs,
    /// The support export.
    Support,
    /// The review / incident surface.
    ReviewIncident,
    /// The runbook-archive surface.
    RunbookArchive,
    /// The release center.
    ReleaseCenter,
    /// The companion / export path.
    CompanionExport,
    /// The program-governance review.
    ProgramGovernance,
    /// The CLI / export path.
    CliExport,
}

impl M5HistoricalReferenceConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Shell,
        Self::HelpDocs,
        Self::Support,
        Self::ReviewIncident,
        Self::RunbookArchive,
        Self::ReleaseCenter,
        Self::CompanionExport,
        Self::ProgramGovernance,
        Self::CliExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::HelpDocs => "help_docs",
            Self::Support => "support",
            Self::ReviewIncident => "review_incident",
            Self::RunbookArchive => "runbook_archive",
            Self::ReleaseCenter => "release_center",
            Self::CompanionExport => "companion_export",
            Self::ProgramGovernance => "program_governance",
            Self::CliExport => "cli_export",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no historical-reference meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5HistoricalReferenceAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified historical-evidence-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceDegradedReason {
    /// The historical-snapshot descriptor has gone stale.
    SnapshotDescriptorStale,
    /// The evidence's provenance / capture context is unattributed.
    ProvenanceUnattributed,
    /// The live-target availability is unresolved.
    LiveTargetUnresolved,
    /// The imported / offline status is unknown.
    ImportedOfflineStateUnknown,
    /// The retention / expiry / removal state is unknown.
    RetentionStateUnknown,
    /// The capture owner is unknown.
    CaptureOwnerUnknown,
}

impl M5HistoricalReferenceDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SnapshotDescriptorStale,
        Self::ProvenanceUnattributed,
        Self::LiveTargetUnresolved,
        Self::ImportedOfflineStateUnknown,
        Self::RetentionStateUnknown,
        Self::CaptureOwnerUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SnapshotDescriptorStale => "snapshot_descriptor_stale",
            Self::ProvenanceUnattributed => "provenance_unattributed",
            Self::LiveTargetUnresolved => "live_target_unresolved",
            Self::ImportedOfflineStateUnknown => "imported_offline_state_unknown",
            Self::RetentionStateUnknown => "retention_state_unknown",
            Self::CaptureOwnerUnknown => "capture_owner_unknown",
        }
    }
}

/// Mandatory label a claimed historical-reference class must be able to show. The first three are hard requirements; the remaining three close the acceptance-criteria ambiguity about the non-live snapshot label, the capture time, and the live-target availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's historical-reference role.
    HistoricalRole,
    /// The canonical historical-reference descriptor the class points at.
    CanonicalReference,
    /// The captured-evidence / archived-snapshot label the class must show.
    SnapshotLabel,
    /// The capture time the class must attribute the evidence to.
    CaptureTime,
    /// The live-target availability (handoff or metadata-only exit) the class must state.
    LiveTargetAvailability,
}

impl M5HistoricalReferenceRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::HistoricalRole,
        Self::CanonicalReference,
        Self::SnapshotLabel,
        Self::CaptureTime,
        Self::LiveTargetAvailability,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::HistoricalRole => "historical_role",
            Self::CanonicalReference => "canonical_reference",
            Self::SnapshotLabel => "snapshot_label",
            Self::CaptureTime => "capture_time",
            Self::LiveTargetAvailability => "live_target_availability",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [
        Self::Identity,
        Self::HistoricalRole,
        Self::CanonicalReference,
    ];
}

/// Qualification class for an M5 historical-reference row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceQualificationClass {
    /// Class historical-evidence handling qualifies for the Stable claim.
    Stable,
    /// Class historical-evidence handling is narrowed to Beta.
    Beta,
    /// Class historical-evidence handling is narrowed to Preview.
    Preview,
    /// Class historical-evidence handling is experimental and not claimed.
    Experimental,
    /// Class historical-evidence handling is unavailable on this build.
    Unavailable,
    /// Class historical-evidence handling is held pending review.
    Held,
}

impl M5HistoricalReferenceQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
    /// Whether the class may carry a public Stable historical-evidence-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a historical-reference object class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HistoricalReferenceDowngradeTrigger {
    /// An archived snapshot was shown as a live, writable, or current object.
    ArchivedSnapshotShownAsLive,
    /// Imported / offline evidence was shown as current route, service, or workspace truth.
    ImportedOfflineEvidenceShownAsCurrent,
    /// Mutation was allowed on non-live captured evidence.
    MutationAllowedOnNonLiveEvidence,
    /// A live target was reopened from a snapshot without validating identity, trust, route, and authority.
    LiveTargetReopenedWithoutValidation,
    /// A class left its captured-evidence / archived-snapshot label missing.
    SnapshotLabelMissing,
    /// A class left its capture time missing.
    CaptureTimeMissing,
    /// A class left its provenance / capture context unattributed.
    ProvenanceUnattributed,
    /// A class left its live-target availability unstated.
    LiveTargetAvailabilityUnstated,
    /// An expired / removed artifact was dead-linked instead of showing metadata or cleanup state.
    ExpiredArtifactDeadLinked,
    /// A class left its retention / removal state unstated.
    RemovalStateUnstated,
    /// Non-live evidence was left unjoined from capture time, provenance, or live-target mismatch.
    EvidenceUnjoinedFromCaptureContext,
    /// The historical-reference descriptor packet has gone stale.
    HistoricalReferenceDescriptorStale,
}

impl M5HistoricalReferenceDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ArchivedSnapshotShownAsLive,
        Self::ImportedOfflineEvidenceShownAsCurrent,
        Self::MutationAllowedOnNonLiveEvidence,
        Self::LiveTargetReopenedWithoutValidation,
        Self::SnapshotLabelMissing,
        Self::CaptureTimeMissing,
        Self::ProvenanceUnattributed,
        Self::LiveTargetAvailabilityUnstated,
        Self::ExpiredArtifactDeadLinked,
        Self::RemovalStateUnstated,
        Self::EvidenceUnjoinedFromCaptureContext,
        Self::HistoricalReferenceDescriptorStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchivedSnapshotShownAsLive => "archived_snapshot_shown_as_live",
            Self::ImportedOfflineEvidenceShownAsCurrent => {
                "imported_offline_evidence_shown_as_current"
            }
            Self::MutationAllowedOnNonLiveEvidence => "mutation_allowed_on_non_live_evidence",
            Self::LiveTargetReopenedWithoutValidation => "live_target_reopened_without_validation",
            Self::SnapshotLabelMissing => "snapshot_label_missing",
            Self::CaptureTimeMissing => "capture_time_missing",
            Self::ProvenanceUnattributed => "provenance_unattributed",
            Self::LiveTargetAvailabilityUnstated => "live_target_availability_unstated",
            Self::ExpiredArtifactDeadLinked => "expired_artifact_dead_linked",
            Self::RemovalStateUnstated => "removal_state_unstated",
            Self::EvidenceUnjoinedFromCaptureContext => "evidence_unjoined_from_capture_context",
            Self::HistoricalReferenceDescriptorStale => "historical_reference_descriptor_stale",
        }
    }
}

/// Required visible state a class must carry so its non-live evidence never reads as a current live object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceVisibleState {
    /// Captured-evidence / archived-snapshot label shown on the surface.
    pub snapshot_label: String,
    /// Capture time the evidence is attributed to.
    pub capture_time: String,
    /// Provenance / capture context the evidence is attributed to.
    pub provenance: String,
    /// Live-target availability (available, unavailable, or metadata-only).
    pub live_target_availability: String,
    /// Imported / offline status (native capture or imported / offline evidence only).
    pub imported_offline_status: String,
    /// Read-only, non-authoritative-for-mutation posture the object holds.
    pub mutation_blocked_posture: String,
    /// Expiry / removed handling (retained, expired-with-metadata, or removed-with-cleanup-state).
    pub expiry_removal_state: String,
    /// Explicit live-target handoff (or metadata-only exit) back to a current object.
    pub live_target_handoff_or_exit: String,
}

impl M5HistoricalReferenceVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.snapshot_label.trim().is_empty()
            && !self.capture_time.trim().is_empty()
            && !self.provenance.trim().is_empty()
            && !self.live_target_availability.trim().is_empty()
            && !self.imported_offline_status.trim().is_empty()
            && !self.mutation_blocked_posture.trim().is_empty()
            && !self.expiry_removal_state.trim().is_empty()
            && !self.live_target_handoff_or_exit.trim().is_empty()
    }
}

/// One row in the matrix: one governed historical-reference object class bound to the surface-specific
/// non-live-evidence truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceRow {
    /// Governed historical-reference object class.
    pub object_class: M5HistoricalReferenceObject,
    /// Qualification class earned by this class's historical-evidence handling.
    pub qualification: M5HistoricalReferenceQualificationClass,
    /// Evidence state this row governs (distinguishes archived / imported evidence from live, cached, and restore-capable state).
    pub evidence_state: M5HistoricalReferenceEvidenceState,
    /// Owner role accountable for keeping this class's non-live evidence governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's evidence non-live and attributable.
    pub required_visible_state: M5HistoricalReferenceVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5HistoricalReferenceSurfaceFamily>,
    /// Capture-lifecycle stages this class passes through from live capture to non-live reference.
    pub capture_lifecycle_stages: Vec<M5HistoricalReferenceCaptureLifecycleStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5HistoricalReferenceRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5HistoricalReferenceRequiredLabel>,
    /// Historical-reference roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5HistoricalReferenceRole>,
    /// RetirementSnapshot non-live-evidence roles this class names (RetirementSnapshot only).
    pub retirement_snapshot_roles: Vec<M5HistoricalReferenceRetirementSnapshotRole>,
    /// SupportExportEvidence non-live-evidence roles this class names (SupportExportEvidence only).
    pub support_export_evidence_roles: Vec<M5HistoricalReferenceSupportExportEvidenceRole>,
    /// ArchivedRunbookPacket non-live-evidence roles this class names (ArchivedRunbookPacket only).
    pub archived_runbook_packet_roles: Vec<M5HistoricalReferenceArchivedRunbookPacketRole>,
    /// ImportedOfflineRouteEvidence non-live-evidence roles this class names (ImportedOfflineRouteEvidence only).
    pub imported_offline_route_evidence_roles:
        Vec<M5HistoricalReferenceImportedOfflineRouteEvidenceRole>,
    /// ReviewIncidentSnapshot non-live-evidence roles this class names (ReviewIncidentSnapshot only).
    pub review_incident_snapshot_roles: Vec<M5HistoricalReferenceReviewIncidentSnapshotRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5HistoricalReferenceDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// First consumer surfaces that consume this class's historical-reference projection.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5HistoricalReferenceDowngradeTrigger>,
    /// Required evidence-artifact refs that keep this class's non-live evidence provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets archived or imported / offline evidence look live, writable, or current by omission. MUST be `false`.
    pub lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: bool,
    /// Hard invariant: this class never reopens a live target from a snapshot without validating target identity, trust, route, and authority first. MUST be `false`.
    pub reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority:
        bool,
    /// Hard invariant: this class never dead-links an expired / removed historical artifact when it can still show metadata, provenance, or safe cleanup state. MUST be `false`.
    pub dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state:
        bool,
    /// Hard invariant: this class never leaves non-live evidence unjoined to capture time, provenance, retention / removal state, or any current live-target mismatch. MUST be `false`.
    pub leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch:
        bool,
    /// Hard invariant: this class never presents a snapshot or imported / offline packet as a current live object or reopens through an ambiguous route. MUST be `false`.
    pub presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route:
        bool,
}

impl M5HistoricalReferenceRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5HistoricalReferenceRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5HistoricalReferenceRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission
            && !self.reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority
            && !self.dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state
            && !self.leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch
            && !self.presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Evidence states tokens.
    pub evidence_states: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Retirement snapshot roles tokens.
    pub retirement_snapshot_roles: Vec<String>,
    /// Support export evidence roles tokens.
    pub support_export_evidence_roles: Vec<String>,
    /// Archived runbook packet roles tokens.
    pub archived_runbook_packet_roles: Vec<String>,
    /// Imported offline route evidence roles tokens.
    pub imported_offline_route_evidence_roles: Vec<String>,
    /// Review incident snapshot roles tokens.
    pub review_incident_snapshot_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Capture lifecycle stages tokens.
    pub capture_lifecycle_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5HistoricalReferenceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5HistoricalReferenceObject::ALL, |v| v.as_str()),
            evidence_states: tokens(&M5HistoricalReferenceEvidenceState::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5HistoricalReferenceRole::ALL, |v| v.as_str()),
            retirement_snapshot_roles: tokens(
                &M5HistoricalReferenceRetirementSnapshotRole::ALL,
                |v| v.as_str(),
            ),
            support_export_evidence_roles: tokens(
                &M5HistoricalReferenceSupportExportEvidenceRole::ALL,
                |v| v.as_str(),
            ),
            archived_runbook_packet_roles: tokens(
                &M5HistoricalReferenceArchivedRunbookPacketRole::ALL,
                |v| v.as_str(),
            ),
            imported_offline_route_evidence_roles: tokens(
                &M5HistoricalReferenceImportedOfflineRouteEvidenceRole::ALL,
                |v| v.as_str(),
            ),
            review_incident_snapshot_roles: tokens(
                &M5HistoricalReferenceReviewIncidentSnapshotRole::ALL,
                |v| v.as_str(),
            ),
            surface_families: tokens(&M5HistoricalReferenceSurfaceFamily::ALL, |v| v.as_str()),
            capture_lifecycle_stages: tokens(
                &M5HistoricalReferenceCaptureLifecycleStage::ALL,
                |v| v.as_str(),
            ),
            consumer_surfaces: tokens(&M5HistoricalReferenceConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5HistoricalReferenceAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5HistoricalReferenceDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5HistoricalReferenceRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5HistoricalReferenceDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceGovernanceReview {
    /// No archived or imported evidence looks live writable or current by omission.
    pub no_archived_or_imported_evidence_looks_live_writable_or_current_by_omission: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Non live evidence is mechanically distinct from live cached and restore capable state.
    pub non_live_evidence_is_mechanically_distinct_from_live_cached_and_restore_capable_state: bool,
    /// Every snapshot carries capture time and provenance before it is surfaced.
    pub every_snapshot_carries_capture_time_and_provenance_before_it_is_surfaced: bool,
    /// Every live target handoff validates target identity trust route and authority.
    pub every_live_target_handoff_validates_target_identity_trust_route_and_authority: bool,
    /// Metadata only exit is offered when a current object can no longer be reopened.
    pub metadata_only_exit_is_offered_when_a_current_object_can_no_longer_be_reopened: bool,
    /// Expired or removed artifacts show metadata provenance or cleanup state never a dead link.
    pub expired_or_removed_artifacts_show_metadata_provenance_or_cleanup_state_never_a_dead_link:
        bool,
    /// Imported and offline evidence always carries its non live disclosure.
    pub imported_and_offline_evidence_always_carries_its_non_live_disclosure: bool,
    /// Non live evidence stays joined to capture context and live target mismatch.
    pub non_live_evidence_stays_joined_to_capture_context_and_live_target_mismatch: bool,
    /// Every object declares capture lifecycle stages.
    pub every_object_declares_capture_lifecycle_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single historical reference source.
    pub support_export_reads_single_historical_reference_source: bool,
    /// Shell help support review runbook and companion bind to single source.
    pub shell_help_support_review_runbook_and_companion_bind_to_single_source: bool,
    /// Later rows cannot invent parallel historical reference vocabulary.
    pub later_rows_cannot_invent_parallel_historical_reference_vocabulary: bool,
    /// Historical reference truth survives zoom and high contrast.
    pub historical_reference_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceConsumerProjection {
    /// Shell and help consume shared historical reference truth.
    pub shell_and_help_consume_shared_historical_reference_truth: bool,
    /// Support and review consume shared snapshot and handoff truth.
    pub support_and_review_consume_shared_snapshot_and_handoff_truth: bool,
    /// Runbook archive and companion export consume shared non live evidence truth.
    pub runbook_archive_and_companion_export_consume_shared_non_live_evidence_truth: bool,
    /// Docs help and screenshots read single historical reference source.
    pub docs_help_and_screenshots_read_single_historical_reference_source: bool,
    /// Archives and snapshots bind to shared capture context.
    pub archives_and_snapshots_bind_to_shared_capture_context: bool,
    /// Support export reads single historical reference source.
    pub support_export_reads_single_historical_reference_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the historical-reference lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting historical-reference audit for the lane.
    pub historical_reference_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5HistoricalReferenceMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HistoricalReferenceMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Historical-reference rows.
    pub historical_reference_rows: Vec<M5HistoricalReferenceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HistoricalReferenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HistoricalReferenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HistoricalReferenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HistoricalReferenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HistoricalReferenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 historical-reference matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalReferenceMatrixPacket {
    /// Record kind; must equal [`M5_HISTORICAL_REFERENCE_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Historical-reference rows.
    pub historical_reference_rows: Vec<M5HistoricalReferenceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5HistoricalReferenceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5HistoricalReferenceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5HistoricalReferenceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5HistoricalReferenceProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5HistoricalReferenceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HistoricalReferenceMatrixPacket {
    /// Builds an M5 historical-reference matrix packet from input.
    pub fn new(input: M5HistoricalReferenceMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_HISTORICAL_REFERENCE_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            historical_reference_rows: input.historical_reference_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 historical-reference matrix invariants.
    pub fn validate(&self) -> Vec<M5HistoricalReferenceMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_HISTORICAL_REFERENCE_MATRIX_RECORD_KIND {
            violations.push(M5HistoricalReferenceMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_VERSION {
            violations.push(M5HistoricalReferenceMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HistoricalReferenceMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_historical_reference_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 historical-reference matrix serializes"),
        ) {
            violations.push(M5HistoricalReferenceMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 historical-reference matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed historical-reference class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,evidence_state,owner,backup_owner,canonical_schema,surface_families,capture_lifecycle_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.historical_reference_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.evidence_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.capture_lifecycle_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic historical-evidence-health dashboard JSON that shell and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .historical_reference_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "evidence_state": row.evidence_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "capture_lifecycle_stages": row
                        .capture_lifecycle_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_historical_evidence_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_HISTORICAL_REFERENCE_ARTIFACT_REF,
            "capture_lifecycle_stages": self.vocabulary_set.capture_lifecycle_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 historical-evidence-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .historical_reference_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Historical-Reference, Archived-Snapshot, Imported/Offline-Evidence, and Live-Target-Handoff Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.historical_reference_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- Historical-reference roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Capture-lifecycle stages: {}\n",
            self.vocabulary_set.capture_lifecycle_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.historical_reference_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (evidence_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.evidence_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Live-target availability: {}\n",
                row.required_visible_state.live_target_availability
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 historical-reference matrix export.
#[derive(Debug)]
pub enum M5HistoricalReferenceMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HistoricalReferenceMatrixViolation>),
}

impl fmt::Display for M5HistoricalReferenceMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 historical-reference matrix export parse failed: {error}"
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
                    "m5 historical-reference matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5HistoricalReferenceMatrixArtifactError {}

/// Validation failures emitted by [`M5HistoricalReferenceMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HistoricalReferenceMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// A historical-reference row is incomplete.
    HistoricalReferenceRowIncomplete,
    /// A historical-reference row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A historical-reference row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no historical-reference roles.
    SemanticRoleMissing,
    /// The RetirementSnapshot class declares no RetirementSnapshot non-live-evidence roles.
    RetirementSnapshotRoleMissing,
    /// The SupportExportEvidence class declares no SupportExportEvidence non-live-evidence roles.
    SupportExportEvidenceRoleMissing,
    /// The ArchivedRunbookPacket class declares no ArchivedRunbookPacket non-live-evidence roles.
    ArchivedRunbookPacketRoleMissing,
    /// The ImportedOfflineRouteEvidence class declares no ImportedOfflineRouteEvidence non-live-evidence roles.
    ImportedOfflineRouteEvidenceRoleMissing,
    /// The ReviewIncidentSnapshot class declares no ReviewIncidentSnapshot non-live-evidence roles.
    ReviewIncidentSnapshotRoleMissing,
    /// A class omits required transition metadata.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no removal-horizon stages.
    CaptureLifecycleStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard historical-reference invariant.
    HistoricalReferenceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5HistoricalReferenceMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::HistoricalReferenceRowIncomplete => "historical_reference_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::RetirementSnapshotRoleMissing => "retirement_snapshot_role_missing",
            Self::SupportExportEvidenceRoleMissing => "support_export_evidence_role_missing",
            Self::ArchivedRunbookPacketRoleMissing => "archived_runbook_packet_role_missing",
            Self::ImportedOfflineRouteEvidenceRoleMissing => {
                "imported_offline_route_evidence_role_missing"
            }
            Self::ReviewIncidentSnapshotRoleMissing => "review_incident_snapshot_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::CaptureLifecycleStageMissing => "capture_lifecycle_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::HistoricalReferenceInvariantViolated => "historical_reference_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 historical-reference matrix export.
pub fn current_stable_m5_historical_reference_matrix_export(
) -> Result<M5HistoricalReferenceMatrixPacket, M5HistoricalReferenceMatrixArtifactError> {
    let packet: M5HistoricalReferenceMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-historical-evidence/support_export.json"
    )))
    .map_err(M5HistoricalReferenceMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HistoricalReferenceMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
        M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF,
        M5_IMPORTED_OFFLINE_EVIDENCE_STATE_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5HistoricalReferenceMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5HistoricalReferenceMatrixViolation::VocabularySetDrift);
    }
}

fn validate_historical_reference_rows(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    let present: BTreeSet<M5HistoricalReferenceObject> = packet
        .historical_reference_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5HistoricalReferenceObject::ALL {
        if !present.contains(&required) {
            violations.push(M5HistoricalReferenceMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.historical_reference_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5HistoricalReferenceMatrixViolation::HistoricalReferenceRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5HistoricalReferenceMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5HistoricalReferenceMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_retirement_snapshot_roles() && row.retirement_snapshot_roles.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::RetirementSnapshotRoleMissing);
        }
        if class.declares_support_export_evidence_roles()
            && row.support_export_evidence_roles.is_empty()
        {
            violations.push(M5HistoricalReferenceMatrixViolation::SupportExportEvidenceRoleMissing);
        }
        if class.declares_archived_runbook_packet_roles()
            && row.archived_runbook_packet_roles.is_empty()
        {
            violations.push(M5HistoricalReferenceMatrixViolation::ArchivedRunbookPacketRoleMissing);
        }
        if class.declares_imported_offline_route_evidence_roles()
            && row.imported_offline_route_evidence_roles.is_empty()
        {
            violations.push(
                M5HistoricalReferenceMatrixViolation::ImportedOfflineRouteEvidenceRoleMissing,
            );
        }
        if class.declares_review_incident_snapshot_roles()
            && row.review_incident_snapshot_roles.is_empty()
        {
            violations
                .push(M5HistoricalReferenceMatrixViolation::ReviewIncidentSnapshotRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5HistoricalReferenceMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::SurfaceFamilyMissing);
        }
        if row.capture_lifecycle_stages.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::CaptureLifecycleStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5HistoricalReferenceMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations
                .push(M5HistoricalReferenceMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations
                .push(M5HistoricalReferenceMatrixViolation::HistoricalReferenceInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_archived_or_imported_evidence_looks_live_writable_or_current_by_omission,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.non_live_evidence_is_mechanically_distinct_from_live_cached_and_restore_capable_state,
        review.every_snapshot_carries_capture_time_and_provenance_before_it_is_surfaced,
        review.every_live_target_handoff_validates_target_identity_trust_route_and_authority,
        review.metadata_only_exit_is_offered_when_a_current_object_can_no_longer_be_reopened,
        review.expired_or_removed_artifacts_show_metadata_provenance_or_cleanup_state_never_a_dead_link,
        review.imported_and_offline_evidence_always_carries_its_non_live_disclosure,
        review.non_live_evidence_stays_joined_to_capture_context_and_live_target_mismatch,
        review.every_object_declares_capture_lifecycle_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_historical_reference_source,
        review.shell_help_support_review_runbook_and_companion_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_historical_reference_vocabulary,
        review.historical_reference_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5HistoricalReferenceMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_help_consume_shared_historical_reference_truth,
        projection.support_and_review_consume_shared_snapshot_and_handoff_truth,
        projection.runbook_archive_and_companion_export_consume_shared_non_live_evidence_truth,
        projection.docs_help_and_screenshots_read_single_historical_reference_source,
        projection.archives_and_snapshots_bind_to_shared_capture_context,
        projection.support_export_reads_single_historical_reference_source,
    ] {
        if !ok {
            violations.push(M5HistoricalReferenceMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5HistoricalReferenceMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5HistoricalReferenceMatrixPacket,
    violations: &mut Vec<M5HistoricalReferenceMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.historical_reference_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5HistoricalReferenceMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses snapshot / provenance / handoff / archival words; what is rejected is a raw secret
/// *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
