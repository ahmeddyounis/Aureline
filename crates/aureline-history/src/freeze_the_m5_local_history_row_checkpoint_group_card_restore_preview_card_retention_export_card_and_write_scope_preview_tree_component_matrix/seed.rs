//! Canonical seed builders for the frozen M5 local-history / write-scope component
//! matrix.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical local-history / write-scope component matrix.
pub const M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_PACKET_ID: &str =
    "m5-local-history-write-scope-components:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every component must be able to show.
fn mandatory_labels() -> Vec<M5HistoryRequiredLabel> {
    M5HistoryRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a component carries.
fn labels_with(extra: &[M5HistoryRequiredLabel]) -> Vec<M5HistoryRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every component filled in and every
/// family-specific vocabulary left empty for the caller to populate.
fn base_row(
    component_family: M5LocalHistoryWriteScopeComponentFamily,
    qualification: M5HistoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5LocalHistoryWriteScopeComponentRow {
    M5LocalHistoryWriteScopeComponentRow {
        component_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        required_labels: mandatory_labels(),
        snapshot_origins: vec![],
        actor_classes: vec![],
        capture_fidelities: vec![],
        checkpoint_lineage_classes: vec![],
        mutation_classes: vec![],
        restore_granularities: vec![],
        restore_drift_states: vec![],
        retention_postures: vec![],
        export_redaction_postures: vec![],
        write_scope_classes: vec![],
        managed_file_caveats: vec![],
        restore_selection_modes: vec![],
        export_manifest_classes: vec![],
        accessibility_routes: M5HistoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5HistoryConsumerSurface::EditorTimelineUi,
            M5HistoryConsumerSurface::SupportExport,
            M5HistoryConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![M5HistoryDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        masks_actor_or_timestamp: false,
        hides_generated_or_managed_caveat: false,
        invents_private_history_grammar: false,
        bypasses_restore_scope_review: false,
    }
}

fn component_rows() -> Vec<M5LocalHistoryWriteScopeComponentRow> {
    use M5CaptureFidelity as CF;
    use M5CheckpointLineageClass as CL;
    use M5ExportManifestClass as EM;
    use M5ExportRedactionPosture as ER;
    use M5HistoryActorClass as AC;
    use M5HistoryConsumerSurface as C;
    use M5HistoryDowngradeTrigger as D;
    use M5HistoryQualificationClass as Q;
    use M5HistoryRequiredLabel as L;
    use M5LocalHistoryWriteScopeComponentFamily as F;
    use M5ManagedFileCaveat as MC;
    use M5MutationClass as MU;
    use M5RestoreDriftState as RD;
    use M5RestoreGranularity as RG;
    use M5RestoreSelectionMode as RS;
    use M5RetentionPosture as RP;
    use M5SnapshotOrigin as SO;
    use M5WriteScopeClass as WS;

    let mut rows = Vec::new();

    // 1. Local-history row.
    let mut row = base_row(
        F::LocalHistoryRow,
        Q::Stable,
        "Local-history row owner",
        "One local-history-row model naming when a snapshot was captured, what produced it — manual save, autosave, formatter run, refactor apply, AI apply, or external import — who authored it, and how much was captured, so a user never has to infer who created a snapshot or whether a metadata-only capture could actually restore",
        "evidence:m5-local-history-row-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_HISTORY_ENTRY_REF,
        ],
    );
    row.snapshot_origins = SO::ALL.to_vec();
    row.actor_classes = AC::ALL.to_vec();
    row.capture_fidelities = CF::ALL.to_vec();
    row.required_labels = labels_with(&[L::TimestampAndActor, L::FileOrObjectIdentity]);
    row.consumer_surfaces = vec![
        C::EditorTimelineUi,
        C::CheckpointInspectorUi,
        C::RecoveryCenterUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::TimestampOrActorUnstated,
        D::CaptureFidelityMasked,
        D::FileOrObjectIdentityUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Checkpoint-group card.
    let mut row = base_row(
        F::CheckpointGroupCard,
        Q::Stable,
        "Checkpoint-group card owner",
        "One checkpoint-group-card model naming whether a checkpoint is a single action, a grouped transaction, a session-restore point, a milestone tag, a rollback point, or an imported checkpoint, and what class of mutation it captured, so a grouped transaction or session-restore point is never collapsed into a single edit",
        "evidence:m5-checkpoint-group-card-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CHECKPOINT_REF,
        ],
    );
    row.checkpoint_lineage_classes = CL::ALL.to_vec();
    row.mutation_classes = MU::ALL.to_vec();
    row.required_labels = labels_with(&[L::TimestampAndActor, L::FileOrObjectIdentity]);
    row.consumer_surfaces = vec![
        C::CheckpointInspectorUi,
        C::EditorTimelineUi,
        C::RecoveryCenterUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::CheckpointLineageUnstated,
        D::MutationClassMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Restore-preview card.
    let mut row = base_row(
        F::RestorePreviewCard,
        Q::Stable,
        "Restore-preview card owner",
        "One restore-preview-card model naming how much a restore will restore — the whole snapshot, per-file, per-hunk, per-symbol, the selection only, or a manual merge — and how the target has drifted since capture, so a partial or manual restore is never shown as a whole-snapshot restore and never applies over local edits or a moved / deleted file silently",
        "evidence:m5-restore-preview-card-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RESTORE_PREVIEW_REF,
        ],
    );
    row.restore_granularities = RG::ALL.to_vec();
    row.restore_drift_states = RD::ALL.to_vec();
    row.required_labels = labels_with(&[L::FileOrObjectIdentity, L::ScopeOrRedaction]);
    row.consumer_surfaces = vec![
        C::RestoreReviewUi,
        C::RecoveryCenterUi,
        C::EditorTimelineUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RestoreGranularityCollapsed,
        D::RestoreDriftHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. Retention/export card.
    let mut row = base_row(
        F::RetentionExportCard,
        Q::Stable,
        "Retention/export card owner",
        "One retention/export-card model naming how long local history is kept — session-only, workspace-retained, account-synced, policy-pinned, purge-pending, or expired-purged — and how it redacts on export, so a purge-pending or expired history is never shown as retained and a redacted export is never shown as a full export",
        "evidence:m5-retention-export-card-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RETENTION_REF,
        ],
    );
    row.retention_postures = RP::ALL.to_vec();
    row.export_redaction_postures = ER::ALL.to_vec();
    row.required_labels = labels_with(&[L::ScopeOrRedaction]);
    row.consumer_surfaces = vec![
        C::RecoveryCenterUi,
        C::CheckpointInspectorUi,
        C::SupportExport,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::RetentionOrRedactionUndisclosed, D::ProofStale];
    rows.push(row);

    // 5. Write-scope preview tree.
    let mut row = base_row(
        F::WriteScopePreviewTree,
        Q::Stable,
        "Write-scope preview tree owner",
        "One write-scope-preview-tree model naming how wide an apply reaches — a single file, several files, a whole directory, across packages, a generated tree, or out of the workspace — and which generated, managed, vendored, protected, or ignored files it touches, so a preview never understates the blast radius of a multi-file apply or restores over a generated or managed file without saying so",
        "evidence:m5-write-scope-preview-tree-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_WRITE_BOUNDARY_REF,
        ],
    );
    row.write_scope_classes = WS::ALL.to_vec();
    row.managed_file_caveats = MC::ALL.to_vec();
    row.required_labels = labels_with(&[L::FileOrObjectIdentity, L::ScopeOrRedaction]);
    row.consumer_surfaces = vec![
        C::RefactorPreviewUi,
        C::AiApplyReviewUi,
        C::RestoreReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::WriteScopeUnderstated,
        D::GeneratedOrManagedCaveatHidden,
        D::BranchOrWorktreeContextMasked,
        D::ProofStale,
    ];
    rows.push(row);

    // 6. Restore-granularity selector.
    let mut row = base_row(
        F::RestoreGranularitySelector,
        Q::Stable,
        "Restore-granularity selector owner",
        "One restore-granularity-selector model naming the selectable apply scope — apply all changes, choose files, choose hunks, choose symbols, exclude generated files, or dry-run only — so scope narrowing is a first-class choice and a broad apply is never forced as all-or-nothing",
        "evidence:m5-restore-granularity-selector-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RESTORE_PREVIEW_REF,
        ],
    );
    row.restore_selection_modes = RS::ALL.to_vec();
    row.required_labels = labels_with(&[L::ScopeOrRedaction]);
    row.consumer_surfaces = vec![
        C::RestoreReviewUi,
        C::RefactorPreviewUi,
        C::AiApplyReviewUi,
        C::SupportExport,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![
        D::RestoreGranularityCollapsed,
        D::GeneratedOrManagedCaveatHidden,
        D::ProofStale,
    ];
    rows.push(row);

    // 7. History-export manifest.
    let mut row = base_row(
        F::HistoryExportManifest,
        Q::Stable,
        "History-export manifest owner",
        "One history-export-manifest model naming what an export bundle contains — a support bundle, recovery evidence, an audit trail, a migration session, an offline mirror, or a redacted share — and how it is redacted, so an export is never mislabelled and a redacted share is never shown as a full-metadata export",
        "evidence:m5-history-export-manifest-parity:001",
        &[
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
            M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RETENTION_REF,
        ],
    );
    row.export_manifest_classes = EM::ALL.to_vec();
    row.export_redaction_postures = ER::ALL.to_vec();
    row.required_labels = labels_with(&[L::ScopeOrRedaction]);
    row.consumer_surfaces = vec![
        C::SupportExport,
        C::RecoveryCenterUi,
        C::CliInspect,
        C::ProductUi,
    ];
    row.downgrade_triggers = vec![D::RetentionOrRedactionUndisclosed, D::ProofStale];
    rows.push(row);

    rows
}

fn governance_review() -> M5LocalHistoryWriteScopeComponentGovernanceReview {
    M5LocalHistoryWriteScopeComponentGovernanceReview {
        local_history_row_shows_timestamp_and_actor: true,
        checkpoint_group_card_shows_lineage_and_mutation_class: true,
        restore_preview_card_shows_granularity_and_drift: true,
        retention_export_card_shows_retention_and_redaction: true,
        write_scope_preview_tree_shows_scope_and_managed_caveat: true,
        restore_granularity_selector_shows_selection_modes: true,
        history_export_manifest_shows_class_and_redaction: true,
        generated_or_managed_files_never_silently_restored: true,
        partial_restore_never_shown_as_whole_snapshot: true,
        branch_or_worktree_context_always_explicit: true,
        export_redaction_posture_always_explicit: true,
        no_component_invents_second_history_grammar: true,
        every_component_declares_deployment_lines: true,
        every_component_declares_accessibility_route: true,
        later_rows_cannot_invent_parallel_vocabulary: true,
    }
}

fn consumer_projection() -> M5LocalHistoryWriteScopeComponentConsumerProjection {
    M5LocalHistoryWriteScopeComponentConsumerProjection {
        editor_and_recovery_surfaces_consume_history_vocabulary: true,
        restore_surfaces_consume_granularity_vocabulary: true,
        write_scope_surfaces_consume_managed_caveat_vocabulary: true,
        export_surfaces_consume_redaction_vocabulary: true,
        support_export_reads_single_source: true,
        refactor_and_ai_surfaces_read_single_source: true,
    }
}

fn proof_freshness() -> M5LocalHistoryWriteScopeComponentProofFreshness {
    M5LocalHistoryWriteScopeComponentProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LocalHistoryWriteScopeComponentReleasePosture {
    M5LocalHistoryWriteScopeComponentReleasePosture {
        proof_packet_ref: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_SCHEMA_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_DOC_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_HISTORY_ENTRY_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_CHECKPOINT_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RESTORE_PREVIEW_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_RETENTION_REF,
        M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_WRITE_BOUNDARY_REF,
    ])
}

/// Builds the canonical frozen M5 local-history / write-scope component matrix packet.
pub fn seeded_m5_local_history_write_scope_component_matrix(
) -> M5LocalHistoryWriteScopeComponentMatrixPacket {
    M5LocalHistoryWriteScopeComponentMatrixPacket::new(
        M5LocalHistoryWriteScopeComponentMatrixPacketInput {
            packet_id: M5_LOCAL_HISTORY_WRITE_SCOPE_COMPONENT_MATRIX_PACKET_ID.to_owned(),
            matrix_label:
                "M5 local-history-row, checkpoint-group-card, restore-preview-card, retention/export-card, write-scope-preview-tree, restore-granularity-selector, and history-export-manifest component matrix"
                    .to_owned(),
            component_rows: component_rows(),
            vocabulary_set: M5LocalHistoryWriteScopeComponentVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the write-scope preview tree is held at Beta because a slice of
/// managed-file caveats does not yet round-trip across every mutation / recovery
/// surface; every component stays visible.
pub fn seeded_m5_local_history_write_scope_component_matrix_write_scope_preview_tree_beta_narrowed(
) -> M5LocalHistoryWriteScopeComponentMatrixPacket {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.packet_id =
        "m5-local-history-write-scope-components:write-scope-preview-tree-beta:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5LocalHistoryWriteScopeComponentFamily::WriteScopePreviewTree
        })
        .expect("write-scope-preview-tree row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}

/// Narrowed variant: the history-export manifest is narrowed to Preview pending
/// redacted-share export parity proof across every surface; every component stays
/// visible.
pub fn seeded_m5_local_history_write_scope_component_matrix_history_export_manifest_preview_narrowed(
) -> M5LocalHistoryWriteScopeComponentMatrixPacket {
    let mut packet = seeded_m5_local_history_write_scope_component_matrix();
    packet.packet_id =
        "m5-local-history-write-scope-components:history-export-manifest-preview:0001".to_owned();
    let row = packet
        .component_rows
        .iter_mut()
        .find(|row| {
            row.component_family == M5LocalHistoryWriteScopeComponentFamily::HistoryExportManifest
        })
        .expect("history-export-manifest row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}
