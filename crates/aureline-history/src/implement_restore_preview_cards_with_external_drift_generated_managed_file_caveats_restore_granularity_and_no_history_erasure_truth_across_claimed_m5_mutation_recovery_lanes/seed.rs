//! Canonical seed builders for the M5 restore-preview-card / restore-granularity-selector
//! primitive.
//!
//! These builders are the single producer of the checked-in support export and the
//! narrowed fixtures. The headless emitter and the inline tests both call them so the
//! in-code matrix, the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical preview/selector-primitive packet.
pub const M5_RESTORE_PREVIEW_GRANULARITY_PACKET_ID: &str =
    "m5-restore-preview-card-restore-granularity-selector-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked restore-preview-card resolution case from a full restore state.
#[allow(clippy::too_many_arguments)]
fn preview_case(
    mutation_class: M5MutationClass,
    capture_fidelity: M5CaptureFidelity,
    drift_state: M5RestoreDriftState,
    managed_caveat: M5ManagedFileCaveat,
    offered_granularity: M5RestoreGranularity,
    retention_posture: M5RetentionPosture,
    export_posture: M5ExportRedactionPosture,
    past_state_label: &str,
    current_state_label: &str,
    object_identity: &str,
    selection_valid: bool,
    restore_path_ready: bool,
) -> M5RestorePreviewCardResolutionCase {
    M5RestorePreviewCardResolutionCase::resolved(M5RestorePreviewCardResolutionInput {
        mutation_class,
        capture_fidelity,
        drift_state,
        managed_caveat,
        offered_granularity,
        retention_posture,
        export_posture,
        past_state_label: past_state_label.to_owned(),
        current_state_label: current_state_label.to_owned(),
        object_identity: object_identity.to_owned(),
        selection_valid,
        restore_path_ready,
    })
}

/// Builds a worked restore-granularity-selector resolution case from a full restore state.
fn selector_case(
    drift_state: M5RestoreDriftState,
    is_multi_file: bool,
    selection_valid: bool,
    touches_generated_or_managed: bool,
    restore_path_ready: bool,
    scope_label: &str,
) -> M5RestoreGranularitySelectorResolutionCase {
    M5RestoreGranularitySelectorResolutionCase::resolved(
        M5RestoreGranularitySelectorResolutionInput {
            drift_state,
            is_multi_file,
            selection_valid,
            touches_generated_or_managed,
            restore_path_ready,
            scope_label: scope_label.to_owned(),
        },
    )
}

/// A base row with the shared fields filled in and the full preview / selector anatomy,
/// fidelity, mutation, drift, granularity, caveat, retention, export, selection-mode,
/// posture, action, export-field, and accessibility parity every consumer carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5RestorePreviewConsumerSurface,
    qualification: M5HistoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    preview_examples: Vec<M5RestorePreviewCardResolutionCase>,
    selector_examples: Vec<M5RestoreGranularitySelectorResolutionCase>,
) -> M5RestorePreviewGranularityRow {
    M5RestorePreviewGranularityRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        preview_anatomy_parts: M5RestorePreviewAnatomyPart::ALL.to_vec(),
        selector_anatomy_parts: M5RestoreGranularitySelectorAnatomyPart::ALL.to_vec(),
        capture_fidelities: M5CaptureFidelity::ALL.to_vec(),
        mutation_classes: M5MutationClass::ALL.to_vec(),
        restore_drift_states: M5RestoreDriftState::ALL.to_vec(),
        restore_granularities: M5RestoreGranularity::ALL.to_vec(),
        managed_caveats: M5ManagedFileCaveat::ALL.to_vec(),
        retention_postures: M5RetentionPosture::ALL.to_vec(),
        export_redaction_postures: M5ExportRedactionPosture::ALL.to_vec(),
        selection_modes: M5RestoreSelectionMode::ALL.to_vec(),
        preview_postures: M5RestorePreviewPosture::ALL.to_vec(),
        selector_postures: M5RestoreGranularitySelectorPosture::ALL.to_vec(),
        preview_actions: M5RestorePreviewAction::ALL.to_vec(),
        selector_actions: M5RestoreGranularitySelectorAction::ALL.to_vec(),
        preview_export_fields: M5RestorePreviewExportField::ALL.to_vec(),
        selector_export_fields: M5RestoreGranularitySelectorExportField::ALL.to_vec(),
        accessibility_routes: M5HistoryAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5HistoryConsumerSurface::EditorTimelineUi,
            M5HistoryConsumerSurface::CheckpointInspectorUi,
            M5HistoryConsumerSurface::RestoreReviewUi,
            M5HistoryConsumerSurface::RefactorPreviewUi,
            M5HistoryConsumerSurface::AiApplyReviewUi,
            M5HistoryConsumerSurface::RecoveryCenterUi,
            M5HistoryConsumerSurface::SupportExport,
            M5HistoryConsumerSurface::CliInspect,
            M5HistoryConsumerSurface::ProductUi,
        ],
        downgrade_triggers: vec![
            M5HistoryDowngradeTrigger::FileOrObjectIdentityUnstated,
            M5HistoryDowngradeTrigger::CaptureFidelityMasked,
            M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
            M5HistoryDowngradeTrigger::RestoreDriftHidden,
            M5HistoryDowngradeTrigger::GeneratedOrManagedCaveatHidden,
            M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
            M5HistoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF,
            M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_PREVIEW_REF,
            M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_CHOOSER_REF,
        ]),
        preview_examples,
        selector_examples,
        masks_past_or_current_state: false,
        hides_drift_or_managed_caveat: false,
        collapses_restore_granularity: false,
        erases_history_trail: false,
    }
}

fn rows() -> Vec<M5RestorePreviewGranularityRow> {
    use M5CaptureFidelity as Fidelity;
    use M5ExportRedactionPosture as Export;
    use M5ManagedFileCaveat as Caveat;
    use M5MutationClass as Mutation;
    use M5RestoreDriftState as Drift;
    use M5RestoreGranularity as Granularity;
    use M5RetentionPosture as Retention;

    vec![
        // 1. Editor restore — a clean per-hunk restore that offers a selected-range apply,
        //    and an external-drift restore that discloses the diverged baseline; a
        //    whole-scope selector and a range-scoped selector.
        base_row(
            M5RestorePreviewConsumerSurface::EditorRestore,
            M5HistoryQualificationClass::Stable,
            "Editor restore owner",
            "The editor restore surface renders the shared restore-preview card and restore-granularity selector so a clean restore compares past and current state, discloses exact object identity, and offers both a whole-file and a selected-range restore, and an external-drift restore surfaces the diverged baseline before any apply — every restore recording a new attributable checkpoint rather than an invisible rewrite of local history",
            "evidence:m5-restore-preview-editor:001",
            vec![
                preview_case(
                    Mutation::TextEdit,
                    Fidelity::FullBodySnapshot,
                    Drift::CleanApply,
                    Caveat::Unmanaged,
                    Granularity::PerHunk,
                    Retention::WorkspaceRetained,
                    Export::FullMetadata,
                    "buffer.rs at 2026-07-07T09:15:00Z",
                    "buffer.rs working tree",
                    "src/editor/buffer.rs",
                    true,
                    true,
                ),
                preview_case(
                    Mutation::TextEdit,
                    Fidelity::DiffOnly,
                    Drift::ExternalDrift,
                    Caveat::Unmanaged,
                    Granularity::WholeSnapshot,
                    Retention::WorkspaceRetained,
                    Export::PathsRedacted,
                    "view.rs at 2026-07-07T09:12:00Z",
                    "view.rs working tree (changed on disk)",
                    "src/editor/view.rs",
                    false,
                    true,
                ),
            ],
            vec![
                selector_case(
                    Drift::CleanApply,
                    false,
                    false,
                    false,
                    true,
                    "restore scope: buffer.rs",
                ),
                selector_case(
                    Drift::CleanApply,
                    true,
                    true,
                    false,
                    true,
                    "restore scope: editor selection (3 files)",
                ),
            ],
        ),
        // 2. AI apply restore — a generated-artifact restore that discloses it reaches a
        //    managed file; an exclude-generated selector.
        base_row(
            M5RestorePreviewConsumerSurface::AiApplyRestore,
            M5HistoryQualificationClass::Stable,
            "AI apply restore owner",
            "The AI apply restore surface renders the shared restore-preview card and restore-granularity selector so a restore that reaches a generated or managed file discloses the managed caveat and defaults the selector to exclude generated files, never silently overwriting a generated artifact, and always records a new attributable checkpoint",
            "evidence:m5-restore-preview-ai-apply:001",
            vec![preview_case(
                Mutation::MultiFileRefactor,
                Fidelity::DiffOnly,
                Drift::CleanApply,
                Caveat::GeneratedFile,
                Granularity::PerFile,
                Retention::WorkspaceRetained,
                Export::BodiesOmitted,
                "bindings.rs at 2026-07-07T11:20:00Z",
                "bindings.rs working tree",
                "src/ai/bindings.rs",
                false,
                true,
            )],
            vec![selector_case(
                Drift::CleanApply,
                true,
                false,
                true,
                true,
                "restore scope: regenerate bindings (3 files)",
            )],
        ),
        // 3. Import restore — a local-edits restore that discloses it would land over
        //    unsaved work; a file-scoped selector.
        base_row(
            M5RestorePreviewConsumerSurface::ImportRestore,
            M5HistoryQualificationClass::Stable,
            "Import restore owner",
            "The import restore surface renders the shared restore-preview card and restore-granularity selector so an imported restore that would land over local edits discloses the local drift and offers a file-scoped narrowing, preserving the existing history trail without masquerading as an invisible rewrite",
            "evidence:m5-restore-preview-import:001",
            vec![preview_case(
                Mutation::ConfigMigration,
                Fidelity::FullBodySnapshot,
                Drift::LocalEditsPresent,
                Caveat::Unmanaged,
                Granularity::WholeSnapshot,
                Retention::WorkspaceRetained,
                Export::PolicyRestricted,
                "settings.toml at 2026-07-07T08:40:00Z",
                "settings.toml working tree (local edits)",
                "config/settings.toml",
                false,
                true,
            )],
            vec![selector_case(
                Drift::LocalEditsPresent,
                true,
                false,
                false,
                true,
                "restore scope: imported settings (2 files)",
            )],
        ),
        // 4. Repair restore — a conflict-pending restore that must resolve first; a
        //    dry-run-only selector until the conflict clears.
        base_row(
            M5RestorePreviewConsumerSurface::RepairRestore,
            M5HistoryQualificationClass::Stable,
            "Repair restore owner",
            "The repair restore surface renders the shared restore-preview card and restore-granularity selector so a restore blocked behind a pending conflict offers resolve-conflict rather than a false restore, and the selector stays dry-run-only until the conflict clears — the same drift-first vocabulary a support reviewer reads elsewhere",
            "evidence:m5-restore-preview-repair:001",
            vec![preview_case(
                Mutation::RepairTransaction,
                Fidelity::FullBodySnapshot,
                Drift::ConflictPending,
                Caveat::Unmanaged,
                Granularity::ManualMerge,
                Retention::WorkspaceRetained,
                Export::FullMetadata,
                "transaction.rs at 2026-07-06T22:05:00Z",
                "transaction.rs working tree (conflict pending)",
                "src/repair/transaction.rs",
                false,
                true,
            )],
            vec![selector_case(
                Drift::ConflictPending,
                true,
                false,
                false,
                true,
                "restore scope: repair transaction (2 files)",
            )],
        ),
        // 5. Recovery center — a restore-blocked restore whose restore path is unavailable;
        //    a blocked selector that can only dry-run.
        base_row(
            M5RestorePreviewConsumerSurface::RecoveryCenter,
            M5HistoryQualificationClass::Stable,
            "Recovery center owner",
            "The recovery center renders the shared restore-preview card and restore-granularity selector so a restore whose source was deleted and whose restore path is unavailable reads as restore-blocked rather than falsely offering a restore, and the selector can only dry-run — the export/redaction and no-history-erasure vocabulary staying identical across every mutation and recovery surface",
            "evidence:m5-restore-preview-recovery:001",
            vec![preview_case(
                Mutation::TextEdit,
                Fidelity::PointerReference,
                Drift::SourceDeleted,
                Caveat::Unmanaged,
                Granularity::WholeSnapshot,
                Retention::PurgePending,
                Export::ExportBlocked,
                "notes.md at 2026-07-06T18:00:00Z",
                "notes.md (source deleted)",
                "docs/notes.md",
                false,
                false,
            )],
            vec![selector_case(
                Drift::SourceDeleted,
                false,
                false,
                false,
                false,
                "restore scope: recovered notes.md",
            )],
        ),
    ]
}

fn governance_review() -> M5RestorePreviewGranularityGovernanceReview {
    M5RestorePreviewGranularityGovernanceReview {
        one_primitive_carries_preview_and_selector_truth: true,
        past_and_current_state_always_shown: true,
        preview_posture_never_masks_drift: true,
        external_drift_always_disclosed: true,
        object_identity_always_preserved: true,
        managed_caveat_never_masked: true,
        inspect_diff_always_offered: true,
        restore_granularity_never_collapsed: true,
        restore_creates_new_checkpoint: true,
        history_trail_never_erased: true,
        support_export_reconstructs_preview_and_selector_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5RestorePreviewGranularityConsumerProjection {
    M5RestorePreviewGranularityConsumerProjection {
        recovery_surfaces_consume_shared_primitive: true,
        preview_posture_reads_single_source: true,
        selector_posture_reads_single_source: true,
        actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5RestorePreviewGranularityProofFreshness {
    M5RestorePreviewGranularityProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5RestorePreviewGranularityReleasePosture {
    M5RestorePreviewGranularityReleasePosture {
        release_packet_ref: M5_RESTORE_PREVIEW_GRANULARITY_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_RESTORE_PREVIEW_GRANULARITY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_RESTORE_PREVIEW_GRANULARITY_SCHEMA_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_DOC_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_COMPONENT_MATRIX_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_PREVIEW_REF,
        M5_RESTORE_PREVIEW_GRANULARITY_RESTORE_CHOOSER_REF,
    ])
}

/// Builds the canonical M5 restore-preview-card / restore-granularity-selector packet.
pub fn seeded_m5_restore_preview_granularity_packet() -> M5RestorePreviewGranularityPacket {
    M5RestorePreviewGranularityPacket::new(M5RestorePreviewGranularityPacketInput {
        packet_id: M5_RESTORE_PREVIEW_GRANULARITY_PACKET_ID.to_owned(),
        matrix_label:
            "M5 restore-preview-card and restore-granularity-selector primitive: past-vs-current comparison, exact object identity, external-drift baseline, generated/managed-file caveat, restore granularity, selectable apply scope, retention/export posture, preview and selector postures, and no-history-erasure truth with bounded inspect/restore/resolve/export and inspect/apply/narrow/exclude actions"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5RestorePreviewGranularityVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the import restore consumer is narrowed to Preview pending
/// external-drift-baseline parity proof across every headless import path; every consumer
/// stays visible.
pub fn seeded_m5_restore_preview_granularity_import_restore_preview_narrowed(
) -> M5RestorePreviewGranularityPacket {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.packet_id =
        "m5-restore-preview-card-restore-granularity-selector-primitive:import-restore-preview:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RestorePreviewConsumerSurface::ImportRestore)
        .expect("import-restore row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}

/// Narrowed variant: the AI apply restore consumer is held at Beta because a slice of
/// AI-apply restores do not yet render the managed-caveat cue on every profile; every
/// consumer stays visible.
pub fn seeded_m5_restore_preview_granularity_ai_apply_restore_beta_narrowed(
) -> M5RestorePreviewGranularityPacket {
    let mut packet = seeded_m5_restore_preview_granularity_packet();
    packet.packet_id =
        "m5-restore-preview-card-restore-granularity-selector-primitive:ai-apply-restore-beta:0001"
            .to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5RestorePreviewConsumerSurface::AiApplyRestore)
        .expect("ai-apply-restore row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}
