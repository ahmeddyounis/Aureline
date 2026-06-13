//! Filesystem-identity, watch-fidelity, mutation-lineage, and deferred-intent
//! matrix for the current generation of file-bearing and managed-write
//! surfaces.
//!
//! This module freezes one typed packet that later notebook, request-workspace,
//! preview, profiler, provider-draft, infrastructure-overlay, and offline-safe
//! packet surfaces can quote without inventing parallel vocabulary for
//! filesystem identity, save posture, undo honesty, corruption routing, or
//! reconnect reconciliation.
//!
//! The checked-in packet is mirrored by:
//!
//! - [`/schemas/state/filesystem_mutation_lineage_matrix.schema.json`](../../../../schemas/state/filesystem_mutation_lineage_matrix.schema.json)
//! - [`/docs/state/filesystem_mutation_lineage_matrix.md`](../../../../docs/state/filesystem_mutation_lineage_matrix.md)
//! - [`/artifacts/state/filesystem_mutation_lineage_matrix.json`](../../../../artifacts/state/filesystem_mutation_lineage_matrix.json)
//! - [`/fixtures/state/filesystem_mutation_lineage_matrix/`](../../../../fixtures/state/filesystem_mutation_lineage_matrix/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixture records.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the checked-in packet.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_RECORD_KIND: &str =
    "filesystem_mutation_lineage_matrix_packet_record";

/// Stable record-kind tag carried by fixture records.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND: &str =
    "filesystem_mutation_lineage_matrix_fixture_record";

/// Repo-relative JSON schema reference.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF: &str =
    "schemas/state/filesystem_mutation_lineage_matrix.schema.json";

/// Repo-relative reviewer doc reference.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF: &str =
    "docs/state/filesystem_mutation_lineage_matrix.md";

/// Repo-relative machine-readable artifact packet.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF: &str =
    "artifacts/state/filesystem_mutation_lineage_matrix.json";

/// Repo-relative reviewer artifact report.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF: &str =
    "artifacts/state/filesystem_mutation_lineage_matrix.md";

/// Repo-relative fixture directory.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_DIR: &str =
    "fixtures/state/filesystem_mutation_lineage_matrix";

/// Repo-relative fixture manifest.
pub const FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/filesystem_mutation_lineage_matrix/manifest.yaml";

/// Root-class vocabulary frozen for M5-era file-bearing surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixRootClass {
    LocalFilesystem,
    RemoteAgent,
    ContainerMount,
    ArchivePackaged,
    GeneratedManaged,
    VirtualProviderBacked,
    ManagedOfflineBundle,
}

impl MatrixRootClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalFilesystem => "local_filesystem",
            Self::RemoteAgent => "remote_agent",
            Self::ContainerMount => "container_mount",
            Self::ArchivePackaged => "archive_packaged",
            Self::GeneratedManaged => "generated_managed",
            Self::VirtualProviderBacked => "virtual_provider_backed",
            Self::ManagedOfflineBundle => "managed_offline_bundle",
        }
    }
}

/// Document or surface class carried by one matrix row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    NotebookDocument,
    NotebookOutputArtifact,
    RequestWorkspaceDocument,
    RequestResponseSnapshot,
    DatabaseExportArtifact,
    ProfilerTraceArtifact,
    PreviewOutputArtifact,
    SyncPacketArtifact,
    ProviderLocalDraft,
    InfrastructureOverlayDocument,
    ImportedArchiveCapture,
}

impl SurfaceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookDocument => "notebook_document",
            Self::NotebookOutputArtifact => "notebook_output_artifact",
            Self::RequestWorkspaceDocument => "request_workspace_document",
            Self::RequestResponseSnapshot => "request_response_snapshot",
            Self::DatabaseExportArtifact => "database_export_artifact",
            Self::ProfilerTraceArtifact => "profiler_trace_artifact",
            Self::PreviewOutputArtifact => "preview_output_artifact",
            Self::SyncPacketArtifact => "sync_packet_artifact",
            Self::ProviderLocalDraft => "provider_local_draft",
            Self::InfrastructureOverlayDocument => "infrastructure_overlay_document",
            Self::ImportedArchiveCapture => "imported_archive_capture",
        }
    }
}

/// Identity class a surface uses as its authoritative write or review target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PathIdentityClass {
    CanonicalFilesystemObject,
    GeneratedSourceIdentity,
    ProviderObjectIdentity,
    ImportedSnapshotIdentity,
    LocalDraftIdentity,
    OfflineBundleIdentity,
}

impl PathIdentityClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalFilesystemObject => "canonical_filesystem_object",
            Self::GeneratedSourceIdentity => "generated_source_identity",
            Self::ProviderObjectIdentity => "provider_object_identity",
            Self::ImportedSnapshotIdentity => "imported_snapshot_identity",
            Self::LocalDraftIdentity => "local_draft_identity",
            Self::OfflineBundleIdentity => "offline_bundle_identity",
        }
    }
}

/// Watch-fidelity vocabulary reused by roots and virtualized surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WatchState {
    LiveWatch,
    ReducedFidelityWatch,
    PollingFallback,
    ManualRefreshOnly,
    ProviderRefreshOnly,
    NoExternalWatch,
}

impl WatchState {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWatch => "live_watch",
            Self::ReducedFidelityWatch => "reduced_fidelity_watch",
            Self::PollingFallback => "polling_fallback",
            Self::ManualRefreshOnly => "manual_refresh_only",
            Self::ProviderRefreshOnly => "provider_refresh_only",
            Self::NoExternalWatch => "no_external_watch",
        }
    }
}

/// Save fallback or canonical write posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveFallback {
    AtomicReplace,
    ConditionalRemoteWrite,
    InPlaceWrite,
    SaveAsCopy,
    RegenerateFromSource,
    StageLocalDraft,
    CompareOnlyBlocked,
}

impl SaveFallback {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AtomicReplace => "atomic_replace",
            Self::ConditionalRemoteWrite => "conditional_remote_write",
            Self::InPlaceWrite => "in_place_write",
            Self::SaveAsCopy => "save_as_copy",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::StageLocalDraft => "stage_local_draft",
            Self::CompareOnlyBlocked => "compare_only_blocked",
        }
    }
}

/// Undo-honesty vocabulary for material mutations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UndoClass {
    ExactUndo,
    CompensatingUndo,
    RegenerateRecompute,
    RestoreFromCheckpoint,
    AuditOnlyNonUndoable,
}

impl UndoClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingUndo => "compensating_undo",
            Self::RegenerateRecompute => "regenerate_recompute",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::AuditOnlyNonUndoable => "audit_only_non_undoable",
        }
    }
}

/// Corruption-routing class, aligned with the existing state-object inventory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CorruptionState {
    BlockFeatureOnly,
    RebuildAutomatically,
    OpenWithWarning,
    RepairFlow,
    BackupRollback,
    FailClosedForPrivilegedOperations,
}

impl CorruptionState {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockFeatureOnly => "block_feature_only",
            Self::RebuildAutomatically => "rebuild_automatically",
            Self::OpenWithWarning => "open_with_warning",
            Self::RepairFlow => "repair_flow",
            Self::BackupRollback => "backup_rollback",
            Self::FailClosedForPrivilegedOperations => "fail_closed_for_privileged_operations",
        }
    }
}

/// Connectivity-state vocabulary reused by deferred-intent lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityState {
    Connected,
    Constrained,
    OfflineLocalSafe,
    ReauthRequired,
    ReconciliationPending,
    ServiceUnavailable,
}

impl ConnectivityState {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Constrained => "constrained",
            Self::OfflineLocalSafe => "offline_local_safe",
            Self::ReauthRequired => "reauth_required",
            Self::ReconciliationPending => "reconciliation_pending",
            Self::ServiceUnavailable => "service_unavailable",
        }
    }
}

/// Reconnect or publish-later posture. `NotApplicable` is explicit so rows do
/// not imply invisible replay.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconciliationPosture {
    NotApplicable,
    NoInvisibleReplay,
    RevalidateBeforeReplay,
    ManualReviewRequired,
    ExpireWithoutReplay,
}

impl ReconciliationPosture {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::NoInvisibleReplay => "no_invisible_replay",
            Self::RevalidateBeforeReplay => "revalidate_before_replay",
            Self::ManualReviewRequired => "manual_review_required",
            Self::ExpireWithoutReplay => "expire_without_replay",
        }
    }
}

/// Explicit coverage flags required by the task contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CoverageFlags {
    /// True when the row owns a canonical filesystem object identity.
    pub canonical_filesystem_identity: bool,
    /// True when the row can claim timely external-change/watch guarantees.
    pub watch_guarantee: bool,
    /// True when the row owns a writable canonical or governed save target.
    pub writable_save_target: bool,
    /// True when material mutations on the row emit one attributable journal entry.
    pub mutation_journal_coverage: bool,
    /// True when deferred-intent queueing or reconnect reconciliation is in scope.
    pub deferred_intent_or_reconcile_exposure: bool,
}

/// One matrix row describing a file-bearing or managed-write surface class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixRow {
    /// Stable row id used by fixtures and exports.
    pub row_id: String,
    /// Stable surface-class token.
    pub surface_class: SurfaceClass,
    /// Human-readable label for reviewers.
    pub title: String,
    /// Primary root class users should expect to see first.
    pub primary_root_class: MatrixRootClass,
    /// All root classes this row admits without changing the contract vocabulary.
    pub supported_root_classes: Vec<MatrixRootClass>,
    /// Which identity class ultimately owns save, review, or mutation authority.
    pub path_identity_class: PathIdentityClass,
    /// Current watch-state contract.
    pub watch_state: WatchState,
    /// Canonical save or write posture.
    pub save_fallback: SaveFallback,
    /// Honest reversal class for material mutations on this row.
    pub undo_class: UndoClass,
    /// Corruption-routing class if the row's persisted state becomes unreadable.
    pub corruption_state: CorruptionState,
    /// Connectivity state the row must name when managed authority degrades.
    pub connectivity_state: ConnectivityState,
    /// Reconnect posture for queued or drifted work.
    pub reconciliation_posture: ReconciliationPosture,
    /// Explicit capability booleans required by the task contract.
    pub coverage: CoverageFlags,
    /// Real consumers or source-of-truth modules this row binds to.
    pub consumer_refs: Vec<String>,
    /// Checked-in fixture scenarios that prove the row.
    pub fixture_refs: Vec<String>,
    /// Short review note explaining why the row differs from ordinary source files.
    pub notes: String,
}

/// Checked-in scenario fixture bound to one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version mirrored from the packet.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Expected row.
    pub expected_row_id: String,
    /// Scenario label for reviewers.
    pub scenario: String,
    /// Root class under test.
    pub root_class: MatrixRootClass,
    /// Expected identity class.
    pub path_identity_class: PathIdentityClass,
    /// Expected watch posture.
    pub watch_state: WatchState,
    /// Expected save posture.
    pub save_fallback: SaveFallback,
    /// Expected undo posture.
    pub undo_class: UndoClass,
    /// Expected corruption posture.
    pub corruption_state: CorruptionState,
    /// Expected connectivity posture.
    pub connectivity_state: ConnectivityState,
    /// Expected reconnect posture.
    pub reconciliation_posture: ReconciliationPosture,
    /// Capability flags echoed from the row.
    pub coverage: CoverageFlags,
    /// One real consumer that would quote this scenario.
    pub consumer_ref: String,
    /// One-line summary of the safety property under test.
    pub notes: String,
}

/// Source references every packet row shares.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Machine-readable artifact packet.
    pub packet_ref: String,
    /// Reviewer artifact report.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilesystemMutationLineageMatrixPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer summary.
    pub title: String,
    /// Shared references for schema/doc/artifacts/fixtures.
    pub source_contract_refs: SourceContractRefs,
    /// Matrix rows.
    pub rows: Vec<MatrixRow>,
    /// Short invariant summary safe for support export and release review.
    pub invariants: Vec<String>,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the matrix packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatrixValidationReport {
    /// All detected violations.
    pub violations: Vec<MatrixValidationViolation>,
}

impl MatrixValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(MatrixValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for MatrixValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "filesystem mutation lineage matrix validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for MatrixValidationReport {}

/// Returns the checked-in packet this lane freezes.
pub fn seeded_filesystem_mutation_lineage_matrix_packet() -> FilesystemMutationLineageMatrixPacket {
    FilesystemMutationLineageMatrixPacket {
        record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_RECORD_KIND.to_owned(),
        schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
        packet_id: "state.filesystem_mutation_lineage_matrix.v1".to_owned(),
        title: "Filesystem identity, watch fidelity, mutation lineage, and deferred-intent matrix"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF.to_owned(),
            schema_ref: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF.to_owned(),
            packet_ref: FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF.to_owned(),
            report_ref: FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF.to_owned(),
            fixture_manifest_ref: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF
                .to_owned(),
        },
        rows: vec![
            MatrixRow {
                row_id: "notebook_document".to_owned(),
                surface_class: SurfaceClass::NotebookDocument,
                title: "Notebook document".to_owned(),
                primary_root_class: MatrixRootClass::LocalFilesystem,
                supported_root_classes: vec![
                    MatrixRootClass::LocalFilesystem,
                    MatrixRootClass::RemoteAgent,
                    MatrixRootClass::ContainerMount,
                ],
                path_identity_class: PathIdentityClass::CanonicalFilesystemObject,
                watch_state: WatchState::LiveWatch,
                save_fallback: SaveFallback::AtomicReplace,
                undo_class: UndoClass::ExactUndo,
                corruption_state: CorruptionState::RepairFlow,
                connectivity_state: ConnectivityState::Connected,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: true,
                    watch_guarantee: true,
                    writable_save_target: true,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/mod.rs".to_owned(),
                    "crates/aureline-vfs/src/identity/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/local_notebook_document.json"
                        .to_owned(),
                ],
                notes: "Notebook files inherit ordinary file identity and save coordination, but their structured metadata repair path must stay explicit.".to_owned(),
            },
            MatrixRow {
                row_id: "notebook_output_artifact".to_owned(),
                surface_class: SurfaceClass::NotebookOutputArtifact,
                title: "Notebook output artifact".to_owned(),
                primary_root_class: MatrixRootClass::GeneratedManaged,
                supported_root_classes: vec![
                    MatrixRootClass::GeneratedManaged,
                    MatrixRootClass::RemoteAgent,
                    MatrixRootClass::ContainerMount,
                ],
                path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
                watch_state: WatchState::ReducedFidelityWatch,
                save_fallback: SaveFallback::RegenerateFromSource,
                undo_class: UndoClass::RegenerateRecompute,
                corruption_state: CorruptionState::RebuildAutomatically,
                connectivity_state: ConnectivityState::Connected,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-notebook/src/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/mod.rs".to_owned(),
                    "crates/aureline-notebook/src/integrate_notebook_outputs_with_docs_browser_ai_context_and_retrieval_debug_provenance_export/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/generated_notebook_output.json"
                        .to_owned(),
                ],
                notes: "Notebook outputs are generated artifacts: the canonical source is the notebook cell or runtime result, not the rendered output bytes.".to_owned(),
            },
            MatrixRow {
                row_id: "request_workspace_document".to_owned(),
                surface_class: SurfaceClass::RequestWorkspaceDocument,
                title: "Request workspace document".to_owned(),
                primary_root_class: MatrixRootClass::LocalFilesystem,
                supported_root_classes: vec![
                    MatrixRootClass::LocalFilesystem,
                    MatrixRootClass::RemoteAgent,
                    MatrixRootClass::ContainerMount,
                ],
                path_identity_class: PathIdentityClass::CanonicalFilesystemObject,
                watch_state: WatchState::LiveWatch,
                save_fallback: SaveFallback::AtomicReplace,
                undo_class: UndoClass::ExactUndo,
                corruption_state: CorruptionState::BackupRollback,
                connectivity_state: ConnectivityState::Constrained,
                reconciliation_posture: ReconciliationPosture::RevalidateBeforeReplay,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: true,
                    watch_guarantee: true,
                    writable_save_target: true,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: true,
                },
                consumer_refs: vec![
                    "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                    "crates/aureline-shell/src/m5_entry_routes/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/remote_request_workspace_document.json"
                        .to_owned(),
                ],
                notes: "Request documents are ordinary files, but sends and managed writes must revalidate target, auth, and policy before any replay.".to_owned(),
            },
            MatrixRow {
                row_id: "request_response_snapshot".to_owned(),
                surface_class: SurfaceClass::RequestResponseSnapshot,
                title: "Request response snapshot".to_owned(),
                primary_root_class: MatrixRootClass::VirtualProviderBacked,
                supported_root_classes: vec![
                    MatrixRootClass::VirtualProviderBacked,
                    MatrixRootClass::ArchivePackaged,
                ],
                path_identity_class: PathIdentityClass::ProviderObjectIdentity,
                watch_state: WatchState::ProviderRefreshOnly,
                save_fallback: SaveFallback::SaveAsCopy,
                undo_class: UndoClass::AuditOnlyNonUndoable,
                corruption_state: CorruptionState::OpenWithWarning,
                connectivity_state: ConnectivityState::ServiceUnavailable,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: false,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/provider_response_snapshot.json"
                        .to_owned(),
                ],
                notes: "Provider-backed response bodies are inspect-first and save-as only; they never inherit ordinary file write semantics.".to_owned(),
            },
            MatrixRow {
                row_id: "database_export_artifact".to_owned(),
                surface_class: SurfaceClass::DatabaseExportArtifact,
                title: "Database export artifact".to_owned(),
                primary_root_class: MatrixRootClass::GeneratedManaged,
                supported_root_classes: vec![
                    MatrixRootClass::GeneratedManaged,
                    MatrixRootClass::LocalFilesystem,
                    MatrixRootClass::RemoteAgent,
                ],
                path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
                watch_state: WatchState::ManualRefreshOnly,
                save_fallback: SaveFallback::RegenerateFromSource,
                undo_class: UndoClass::RegenerateRecompute,
                corruption_state: CorruptionState::RebuildAutomatically,
                connectivity_state: ConnectivityState::Constrained,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-data/src/database_qualification.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/database_export_artifact.json"
                        .to_owned(),
                ],
                notes: "Database exports are generated outputs with canonical-source lineage back to query or connection context, not ad hoc editable documents.".to_owned(),
            },
            MatrixRow {
                row_id: "profiler_trace_artifact".to_owned(),
                surface_class: SurfaceClass::ProfilerTraceArtifact,
                title: "Profiler trace artifact".to_owned(),
                primary_root_class: MatrixRootClass::ArchivePackaged,
                supported_root_classes: vec![
                    MatrixRootClass::ArchivePackaged,
                    MatrixRootClass::LocalFilesystem,
                    MatrixRootClass::ManagedOfflineBundle,
                ],
                path_identity_class: PathIdentityClass::ImportedSnapshotIdentity,
                watch_state: WatchState::NoExternalWatch,
                save_fallback: SaveFallback::SaveAsCopy,
                undo_class: UndoClass::AuditOnlyNonUndoable,
                corruption_state: CorruptionState::OpenWithWarning,
                connectivity_state: ConnectivityState::OfflineLocalSafe,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: true,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-profiler/src/integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/archive_profiler_trace_artifact.json"
                        .to_owned(),
                ],
                notes: "Profiler traces may be imported or exported packets; they are attributable artifacts, not ordinary mutable source files.".to_owned(),
            },
            MatrixRow {
                row_id: "preview_output_artifact".to_owned(),
                surface_class: SurfaceClass::PreviewOutputArtifact,
                title: "Preview output artifact".to_owned(),
                primary_root_class: MatrixRootClass::GeneratedManaged,
                supported_root_classes: vec![
                    MatrixRootClass::GeneratedManaged,
                    MatrixRootClass::ContainerMount,
                    MatrixRootClass::VirtualProviderBacked,
                ],
                path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
                watch_state: WatchState::ReducedFidelityWatch,
                save_fallback: SaveFallback::RegenerateFromSource,
                undo_class: UndoClass::RegenerateRecompute,
                corruption_state: CorruptionState::RebuildAutomatically,
                connectivity_state: ConnectivityState::Constrained,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-preview/src/lib.rs".to_owned(),
                    "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/container_preview_output_artifact.json"
                        .to_owned(),
                ],
                notes: "Preview outputs remain source-first generated artifacts; direct byte edits are subordinate to the preview generator and runtime lineage.".to_owned(),
            },
            MatrixRow {
                row_id: "sync_packet_artifact".to_owned(),
                surface_class: SurfaceClass::SyncPacketArtifact,
                title: "Sync packet artifact".to_owned(),
                primary_root_class: MatrixRootClass::ManagedOfflineBundle,
                supported_root_classes: vec![
                    MatrixRootClass::ManagedOfflineBundle,
                    MatrixRootClass::LocalFilesystem,
                ],
                path_identity_class: PathIdentityClass::OfflineBundleIdentity,
                watch_state: WatchState::ManualRefreshOnly,
                save_fallback: SaveFallback::StageLocalDraft,
                undo_class: UndoClass::RestoreFromCheckpoint,
                corruption_state: CorruptionState::RepairFlow,
                connectivity_state: ConnectivityState::OfflineLocalSafe,
                reconciliation_posture: ReconciliationPosture::RevalidateBeforeReplay,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: true,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: true,
                },
                consumer_refs: vec![
                    "crates/aureline-continuity/src/connectivity_state_and_deferred_intent/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/offline_sync_packet_artifact.json"
                        .to_owned(),
                ],
                notes: "Offline-safe packets preserve local continuity, but reconnect must revalidate target and policy before any replay.".to_owned(),
            },
            MatrixRow {
                row_id: "provider_local_draft".to_owned(),
                surface_class: SurfaceClass::ProviderLocalDraft,
                title: "Provider local draft".to_owned(),
                primary_root_class: MatrixRootClass::VirtualProviderBacked,
                supported_root_classes: vec![
                    MatrixRootClass::VirtualProviderBacked,
                    MatrixRootClass::ManagedOfflineBundle,
                ],
                path_identity_class: PathIdentityClass::LocalDraftIdentity,
                watch_state: WatchState::ProviderRefreshOnly,
                save_fallback: SaveFallback::StageLocalDraft,
                undo_class: UndoClass::CompensatingUndo,
                corruption_state: CorruptionState::RepairFlow,
                connectivity_state: ConnectivityState::ReconciliationPending,
                reconciliation_posture: ReconciliationPosture::ManualReviewRequired,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: true,
                    mutation_journal_coverage: true,
                    deferred_intent_or_reconcile_exposure: true,
                },
                consumer_refs: vec![
                    "crates/aureline-provider/src/publish_later/mod.rs".to_owned(),
                    "crates/aureline-provider/src/work_item_sync/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/provider_local_draft.json"
                        .to_owned(),
                ],
                notes: "Provider drafts persist locally first and must never be replayed or published invisibly after reconnect or provider drift.".to_owned(),
            },
            MatrixRow {
                row_id: "infrastructure_overlay_document".to_owned(),
                surface_class: SurfaceClass::InfrastructureOverlayDocument,
                title: "Infrastructure overlay document".to_owned(),
                primary_root_class: MatrixRootClass::VirtualProviderBacked,
                supported_root_classes: vec![
                    MatrixRootClass::VirtualProviderBacked,
                    MatrixRootClass::RemoteAgent,
                    MatrixRootClass::ContainerMount,
                ],
                path_identity_class: PathIdentityClass::ProviderObjectIdentity,
                watch_state: WatchState::ProviderRefreshOnly,
                save_fallback: SaveFallback::CompareOnlyBlocked,
                undo_class: UndoClass::AuditOnlyNonUndoable,
                corruption_state: CorruptionState::BlockFeatureOnly,
                connectivity_state: ConnectivityState::ServiceUnavailable,
                reconciliation_posture: ReconciliationPosture::ManualReviewRequired,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: false,
                    deferred_intent_or_reconcile_exposure: true,
                },
                consumer_refs: vec![
                    "crates/aureline-infra/src/provider_overlay_and_vendor_console_handoff_continuity/mod.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/infrastructure_overlay_document.json"
                        .to_owned(),
                ],
                notes: "Infrastructure overlays are provider truth layers: inspect and review them, but do not pretend they are ordinary filesystem-backed source files.".to_owned(),
            },
            MatrixRow {
                row_id: "imported_archive_capture".to_owned(),
                surface_class: SurfaceClass::ImportedArchiveCapture,
                title: "Imported archive capture".to_owned(),
                primary_root_class: MatrixRootClass::ArchivePackaged,
                supported_root_classes: vec![
                    MatrixRootClass::ArchivePackaged,
                    MatrixRootClass::ManagedOfflineBundle,
                ],
                path_identity_class: PathIdentityClass::ImportedSnapshotIdentity,
                watch_state: WatchState::NoExternalWatch,
                save_fallback: SaveFallback::SaveAsCopy,
                undo_class: UndoClass::AuditOnlyNonUndoable,
                corruption_state: CorruptionState::OpenWithWarning,
                connectivity_state: ConnectivityState::OfflineLocalSafe,
                reconciliation_posture: ReconciliationPosture::NotApplicable,
                coverage: CoverageFlags {
                    canonical_filesystem_identity: false,
                    watch_guarantee: false,
                    writable_save_target: false,
                    mutation_journal_coverage: false,
                    deferred_intent_or_reconcile_exposure: false,
                },
                consumer_refs: vec![
                    "crates/aureline-vfs/src/roots/virtual_documents.rs".to_owned(),
                    "crates/aureline-preview/src/safe_preview.rs".to_owned(),
                ],
                fixture_refs: vec![
                    "fixtures/state/filesystem_mutation_lineage_matrix/imported_archive_capture.json"
                        .to_owned(),
                ],
                notes: "Imported captures and packaged artifacts are inspect-only unless promoted or copied to a new writable target.".to_owned(),
            },
        ],
        invariants: vec![
            "presentation path, logical identity, canonical target, alias set, and save target remain distinct when the root can express them".to_owned(),
            "degraded watch or save semantics are visible before or during mutation, never implied by success copy alone".to_owned(),
            "material mutations emit one attributable journal entry with an explicit undo class rather than silently forking lineage".to_owned(),
            "corruption degrades by class and row, not by whole-application reset".to_owned(),
            "deferred managed actions require revalidation or review and never replay invisibly after reconnect or policy refresh".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture scenarios bound to the packet rows.
pub fn seeded_filesystem_mutation_lineage_matrix_fixtures() -> Vec<MatrixFixture> {
    vec![
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.local_notebook_document".to_owned(),
            expected_row_id: "notebook_document".to_owned(),
            scenario: "Local notebook file keeps canonical file identity, live watch, and exact save lineage even when notebook structure needs a typed repair path.".to_owned(),
            root_class: MatrixRootClass::LocalFilesystem,
            path_identity_class: PathIdentityClass::CanonicalFilesystemObject,
            watch_state: WatchState::LiveWatch,
            save_fallback: SaveFallback::AtomicReplace,
            undo_class: UndoClass::ExactUndo,
            corruption_state: CorruptionState::RepairFlow,
            connectivity_state: ConnectivityState::Connected,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: true,
                watch_guarantee: true,
                writable_save_target: true,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-notebook/src/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/mod.rs".to_owned(),
            notes: "Covers the local file-backed notebook lane.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.generated_notebook_output".to_owned(),
            expected_row_id: "notebook_output_artifact".to_owned(),
            scenario: "Generated notebook output inherits generator lineage and regenerate-only recovery instead of direct-save semantics.".to_owned(),
            root_class: MatrixRootClass::GeneratedManaged,
            path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
            watch_state: WatchState::ReducedFidelityWatch,
            save_fallback: SaveFallback::RegenerateFromSource,
            undo_class: UndoClass::RegenerateRecompute,
            corruption_state: CorruptionState::RebuildAutomatically,
            connectivity_state: ConnectivityState::Connected,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-notebook/src/materialize_notebook_output_trust_classes_sanitized_or_sandboxed_viewer_lanes_and_large_output_virtualization/mod.rs".to_owned(),
            notes: "Covers generated notebook-output refresh truth.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.remote_request_workspace_document".to_owned(),
            expected_row_id: "request_workspace_document".to_owned(),
            scenario: "Remote-backed request workspace files remain canonical documents, but dispatch replay requires target, auth, and policy revalidation after reconnect.".to_owned(),
            root_class: MatrixRootClass::RemoteAgent,
            path_identity_class: PathIdentityClass::CanonicalFilesystemObject,
            watch_state: WatchState::LiveWatch,
            save_fallback: SaveFallback::AtomicReplace,
            undo_class: UndoClass::ExactUndo,
            corruption_state: CorruptionState::BackupRollback,
            connectivity_state: ConnectivityState::Constrained,
            reconciliation_posture: ReconciliationPosture::RevalidateBeforeReplay,
            coverage: CoverageFlags {
                canonical_filesystem_identity: true,
                watch_guarantee: true,
                writable_save_target: true,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: true,
            },
            consumer_ref: "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
            notes: "Covers remote request-workspace document truth.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.provider_response_snapshot".to_owned(),
            expected_row_id: "request_response_snapshot".to_owned(),
            scenario: "Provider-backed response snapshots remain inspect-first virtual records with save-as only fallback.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            path_identity_class: PathIdentityClass::ProviderObjectIdentity,
            watch_state: WatchState::ProviderRefreshOnly,
            save_fallback: SaveFallback::SaveAsCopy,
            undo_class: UndoClass::AuditOnlyNonUndoable,
            corruption_state: CorruptionState::OpenWithWarning,
            connectivity_state: ConnectivityState::ServiceUnavailable,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: false,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-shell/src/request_workspace/mod.rs".to_owned(),
            notes: "Covers provider-backed response viewing without ordinary edit authority.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.database_export_artifact".to_owned(),
            expected_row_id: "database_export_artifact".to_owned(),
            scenario: "Generated database exports are lineage-bound outputs that rebuild from source query context instead of taking direct edits.".to_owned(),
            root_class: MatrixRootClass::GeneratedManaged,
            path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
            watch_state: WatchState::ManualRefreshOnly,
            save_fallback: SaveFallback::RegenerateFromSource,
            undo_class: UndoClass::RegenerateRecompute,
            corruption_state: CorruptionState::RebuildAutomatically,
            connectivity_state: ConnectivityState::Constrained,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-data/src/database_qualification.rs".to_owned(),
            notes: "Covers generated export packets and files.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.archive_profiler_trace_artifact".to_owned(),
            expected_row_id: "profiler_trace_artifact".to_owned(),
            scenario: "Imported profiler traces remain attributable packets that may be exported or attached without becoming mutable source files.".to_owned(),
            root_class: MatrixRootClass::ArchivePackaged,
            path_identity_class: PathIdentityClass::ImportedSnapshotIdentity,
            watch_state: WatchState::NoExternalWatch,
            save_fallback: SaveFallback::SaveAsCopy,
            undo_class: UndoClass::AuditOnlyNonUndoable,
            corruption_state: CorruptionState::OpenWithWarning,
            connectivity_state: ConnectivityState::OfflineLocalSafe,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: true,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-profiler/src/integrate_profile_and_trace_artifacts_into_incident_workspaces_ai_explanations_and_support_bundles/mod.rs".to_owned(),
            notes: "Covers imported profiler trace packets.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.container_preview_output_artifact".to_owned(),
            expected_row_id: "preview_output_artifact".to_owned(),
            scenario: "Container-backed preview outputs stay generator-owned and surface reduced watch fidelity rather than pretending live source equivalence.".to_owned(),
            root_class: MatrixRootClass::ContainerMount,
            path_identity_class: PathIdentityClass::GeneratedSourceIdentity,
            watch_state: WatchState::ReducedFidelityWatch,
            save_fallback: SaveFallback::RegenerateFromSource,
            undo_class: UndoClass::RegenerateRecompute,
            corruption_state: CorruptionState::RebuildAutomatically,
            connectivity_state: ConnectivityState::Constrained,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-shell/src/preview_truth/mod.rs".to_owned(),
            notes: "Covers preview runtime generated outputs.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.offline_sync_packet_artifact".to_owned(),
            expected_row_id: "sync_packet_artifact".to_owned(),
            scenario: "Offline-safe sync packets remain locally writable artifacts, but reconnect requires typed revalidation before replay.".to_owned(),
            root_class: MatrixRootClass::ManagedOfflineBundle,
            path_identity_class: PathIdentityClass::OfflineBundleIdentity,
            watch_state: WatchState::ManualRefreshOnly,
            save_fallback: SaveFallback::StageLocalDraft,
            undo_class: UndoClass::RestoreFromCheckpoint,
            corruption_state: CorruptionState::RepairFlow,
            connectivity_state: ConnectivityState::OfflineLocalSafe,
            reconciliation_posture: ReconciliationPosture::RevalidateBeforeReplay,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: true,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: true,
            },
            consumer_ref: "crates/aureline-continuity/src/connectivity_state_and_deferred_intent/mod.rs".to_owned(),
            notes: "Covers offline-safe managed packet continuity.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.provider_local_draft".to_owned(),
            expected_row_id: "provider_local_draft".to_owned(),
            scenario: "Provider local drafts survive disconnects, but publish-later replay requires manual conflict review after provider drift.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            path_identity_class: PathIdentityClass::LocalDraftIdentity,
            watch_state: WatchState::ProviderRefreshOnly,
            save_fallback: SaveFallback::StageLocalDraft,
            undo_class: UndoClass::CompensatingUndo,
            corruption_state: CorruptionState::RepairFlow,
            connectivity_state: ConnectivityState::ReconciliationPending,
            reconciliation_posture: ReconciliationPosture::ManualReviewRequired,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: true,
                mutation_journal_coverage: true,
                deferred_intent_or_reconcile_exposure: true,
            },
            consumer_ref: "crates/aureline-provider/src/publish_later/mod.rs".to_owned(),
            notes: "Covers local-draft and publish-later continuity.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.infrastructure_overlay_document".to_owned(),
            expected_row_id: "infrastructure_overlay_document".to_owned(),
            scenario: "Infrastructure overlays remain provider truth layers with manual review for control-plane replay and no ordinary save target.".to_owned(),
            root_class: MatrixRootClass::VirtualProviderBacked,
            path_identity_class: PathIdentityClass::ProviderObjectIdentity,
            watch_state: WatchState::ProviderRefreshOnly,
            save_fallback: SaveFallback::CompareOnlyBlocked,
            undo_class: UndoClass::AuditOnlyNonUndoable,
            corruption_state: CorruptionState::BlockFeatureOnly,
            connectivity_state: ConnectivityState::ServiceUnavailable,
            reconciliation_posture: ReconciliationPosture::ManualReviewRequired,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: false,
                deferred_intent_or_reconcile_exposure: true,
            },
            consumer_ref: "crates/aureline-infra/src/provider_overlay_and_vendor_console_handoff_continuity/mod.rs".to_owned(),
            notes: "Covers provider-backed infrastructure overlays.".to_owned(),
        },
        MatrixFixture {
            record_kind: FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION,
            fixture_id: "fixture.imported_archive_capture".to_owned(),
            expected_row_id: "imported_archive_capture".to_owned(),
            scenario: "Imported archive captures remain inspect-only packets until explicitly promoted or copied to a new target.".to_owned(),
            root_class: MatrixRootClass::ArchivePackaged,
            path_identity_class: PathIdentityClass::ImportedSnapshotIdentity,
            watch_state: WatchState::NoExternalWatch,
            save_fallback: SaveFallback::SaveAsCopy,
            undo_class: UndoClass::AuditOnlyNonUndoable,
            corruption_state: CorruptionState::OpenWithWarning,
            connectivity_state: ConnectivityState::OfflineLocalSafe,
            reconciliation_posture: ReconciliationPosture::NotApplicable,
            coverage: CoverageFlags {
                canonical_filesystem_identity: false,
                watch_guarantee: false,
                writable_save_target: false,
                mutation_journal_coverage: false,
                deferred_intent_or_reconcile_exposure: false,
            },
            consumer_ref: "crates/aureline-vfs/src/roots/virtual_documents.rs".to_owned(),
            notes: "Covers packaged import/export archives.".to_owned(),
        },
    ]
}

/// Validates the checked-in packet contract.
pub fn validate_filesystem_mutation_lineage_matrix(
    packet: &FilesystemMutationLineageMatrixPacket,
) -> Result<(), MatrixValidationReport> {
    let mut report = MatrixValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.source_contract_refs.doc_ref != FILESYSTEM_MUTATION_LINEAGE_MATRIX_DOC_REF {
        report.push(
            "packet.doc_ref",
            "doc_ref drifted from the frozen reviewer doc",
        );
    }
    if packet.source_contract_refs.schema_ref != FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen JSON schema",
        );
    }
    if packet.source_contract_refs.packet_ref != FILESYSTEM_MUTATION_LINEAGE_MATRIX_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != FILESYSTEM_MUTATION_LINEAGE_MATRIX_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }

    let mut row_ids = BTreeSet::new();
    let mut supported_roots = BTreeSet::new();
    let mut seen_surfaces = BTreeSet::new();

    for row in &packet.rows {
        if !row_ids.insert(row.row_id.clone()) {
            report.push(
                "row.row_id.unique",
                format!("duplicate row_id {}", row.row_id),
            );
        }
        seen_surfaces.insert(row.surface_class);
        supported_roots.insert(row.primary_root_class);
        for root in &row.supported_root_classes {
            supported_roots.insert(*root);
        }
        if row.consumer_refs.is_empty() {
            report.push(
                "row.consumer_refs",
                format!("row {} must cite at least one consumer ref", row.row_id),
            );
        }
        if row.fixture_refs.is_empty() {
            report.push(
                "row.fixture_refs",
                format!("row {} must cite at least one fixture ref", row.row_id),
            );
        }
        if row.coverage.writable_save_target && !row.coverage.mutation_journal_coverage {
            report.push(
                "row.journal_required_for_writes",
                format!(
                    "row {} claims a writable save target without mutation journal coverage",
                    row.row_id
                ),
            );
        }
        if row.coverage.watch_guarantee
            && matches!(
                row.watch_state,
                WatchState::ManualRefreshOnly
                    | WatchState::ProviderRefreshOnly
                    | WatchState::NoExternalWatch
            )
        {
            report.push(
                "row.watch_state_inconsistent",
                format!(
                    "row {} claims watch_guarantee=true with a non-guaranteed watch state",
                    row.row_id
                ),
            );
        }
        if !row.coverage.deferred_intent_or_reconcile_exposure
            && !matches!(
                row.reconciliation_posture,
                ReconciliationPosture::NotApplicable
            )
        {
            report.push(
                "row.reconciliation_posture_inconsistent",
                format!(
                    "row {} declares a reconciliation posture without deferred/reconcile exposure",
                    row.row_id
                ),
            );
        }
        if row.coverage.deferred_intent_or_reconcile_exposure
            && matches!(
                row.reconciliation_posture,
                ReconciliationPosture::NotApplicable
            )
        {
            report.push(
                "row.reconciliation_posture_missing",
                format!(
                    "row {} must name a reconciliation posture when deferred/reconcile exposure is true",
                    row.row_id
                ),
            );
        }
        if matches!(row.save_fallback, SaveFallback::RegenerateFromSource)
            && !matches!(row.undo_class, UndoClass::RegenerateRecompute)
        {
            report.push(
                "row.regenerate_undo_alignment",
                format!(
                    "row {} uses regenerate_from_source but does not declare regenerate_recompute",
                    row.row_id
                ),
            );
        }
        if matches!(row.save_fallback, SaveFallback::StageLocalDraft)
            && !row.coverage.deferred_intent_or_reconcile_exposure
        {
            report.push(
                "row.stage_local_draft_requires_reconcile",
                format!(
                    "row {} stages a local draft without deferred/reconcile exposure",
                    row.row_id
                ),
            );
        }
    }

    for required_root in [
        MatrixRootClass::LocalFilesystem,
        MatrixRootClass::RemoteAgent,
        MatrixRootClass::ContainerMount,
        MatrixRootClass::ArchivePackaged,
        MatrixRootClass::GeneratedManaged,
        MatrixRootClass::VirtualProviderBacked,
        MatrixRootClass::ManagedOfflineBundle,
    ] {
        if !supported_roots.contains(&required_root) {
            report.push(
                "coverage.root_class",
                format!("missing required root class {}", required_root.as_str()),
            );
        }
    }

    for required_surface in [
        SurfaceClass::NotebookDocument,
        SurfaceClass::NotebookOutputArtifact,
        SurfaceClass::RequestWorkspaceDocument,
        SurfaceClass::RequestResponseSnapshot,
        SurfaceClass::DatabaseExportArtifact,
        SurfaceClass::ProfilerTraceArtifact,
        SurfaceClass::PreviewOutputArtifact,
        SurfaceClass::SyncPacketArtifact,
        SurfaceClass::ProviderLocalDraft,
        SurfaceClass::InfrastructureOverlayDocument,
        SurfaceClass::ImportedArchiveCapture,
    ] {
        if !seen_surfaces.contains(&required_surface) {
            report.push(
                "coverage.surface_class",
                format!(
                    "missing required surface class {}",
                    required_surface.as_str()
                ),
            );
        }
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one fixture against the frozen packet.
pub fn validate_filesystem_mutation_lineage_fixture(
    packet: &FilesystemMutationLineageMatrixPacket,
    fixture: &MatrixFixture,
) -> Result<(), MatrixValidationReport> {
    let mut report = MatrixValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != FILESYSTEM_MUTATION_LINEAGE_MATRIX_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != FILESYSTEM_MUTATION_LINEAGE_MATRIX_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    let Some(row) = packet
        .rows
        .iter()
        .find(|row| row.row_id == fixture.expected_row_id)
    else {
        report.push(
            "fixture.expected_row_id",
            format!("fixture {} references an unknown row", fixture.fixture_id),
        );
        return Err(report);
    };

    if !row.supported_root_classes.contains(&fixture.root_class)
        && row.primary_root_class != fixture.root_class
    {
        report.push(
            "fixture.root_class_supported",
            format!(
                "fixture {} uses unsupported root class {} for row {}",
                fixture.fixture_id,
                fixture.root_class.as_str(),
                row.row_id
            ),
        );
    }
    if row.path_identity_class != fixture.path_identity_class {
        report.push(
            "fixture.path_identity_class",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.watch_state != fixture.watch_state {
        report.push(
            "fixture.watch_state",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.save_fallback != fixture.save_fallback {
        report.push(
            "fixture.save_fallback",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.undo_class != fixture.undo_class {
        report.push(
            "fixture.undo_class",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.corruption_state != fixture.corruption_state {
        report.push(
            "fixture.corruption_state",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.connectivity_state != fixture.connectivity_state {
        report.push(
            "fixture.connectivity_state",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.reconciliation_posture != fixture.reconciliation_posture {
        report.push(
            "fixture.reconciliation_posture",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if row.coverage != fixture.coverage {
        report.push(
            "fixture.coverage",
            format!(
                "fixture {} drifted from row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }
    if !row
        .consumer_refs
        .iter()
        .any(|reference| reference == &fixture.consumer_ref)
    {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} cites a consumer_ref not declared by row {}",
                fixture.fixture_id, row.row_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}
