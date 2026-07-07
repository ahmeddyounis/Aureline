//! Canonical seed builders for the M5 compare-export primitive.
//!
//! These builders are the single producer of the checked-in support export and the narrowed
//! fixtures. The headless emitter and the inline tests both call them so the in-code matrix,
//! the artifact, the worked resolutions, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical compare-export primitive packet.
pub const M5_COMPARE_EXPORT_PACKET_ID: &str = "m5-retention-export-card-primitive:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-07T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// Builds a worked retention/export-card resolution case from a full retention / export state.
#[allow(clippy::too_many_arguments)]
fn card_case(
    retention_posture: M5RetentionPosture,
    export_redaction: M5ExportRedactionPosture,
    supported_baselines: Vec<M5CompareBaseline>,
    baseline_comparison_available: bool,
    is_metadata_only: bool,
    export_path_ready: bool,
    card_label: &str,
) -> M5RetentionExportCardResolutionCase {
    M5RetentionExportCardResolutionCase::resolved(M5RetentionExportCardResolutionInput {
        retention_posture,
        export_redaction,
        supported_baselines,
        baseline_comparison_available,
        is_metadata_only,
        export_path_ready,
        card_label: card_label.to_owned(),
    })
}

/// Builds a worked history-export-manifest resolution case from a full manifest state.
#[allow(clippy::too_many_arguments)]
fn manifest_case(
    manifest_class: M5ExportManifestClass,
    export_redaction: M5ExportRedactionPosture,
    primary_baseline: M5CompareBaseline,
    preserves_actor_lineage: bool,
    preserves_checkpoint_identity: bool,
    preserves_scope: bool,
    includes_raw_bodies: bool,
    export_path_ready: bool,
    manifest_label: &str,
) -> M5HistoryExportManifestResolutionCase {
    M5HistoryExportManifestResolutionCase::resolved(M5HistoryExportManifestResolutionInput {
        manifest_class,
        export_redaction,
        primary_baseline,
        preserves_actor_lineage,
        preserves_checkpoint_identity,
        preserves_scope,
        includes_raw_bodies,
        export_path_ready,
        manifest_label: manifest_label.to_owned(),
    })
}

/// A base row with the shared fields filled in and the full card / manifest anatomy,
/// retention, redaction, manifest-class, baseline, card-posture, manifest-disposition, action,
/// export-field, and accessibility parity every consumer carries.
#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5CompareExportConsumerSurface,
    qualification: M5HistoryQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    card_examples: Vec<M5RetentionExportCardResolutionCase>,
    manifest_examples: Vec<M5HistoryExportManifestResolutionCase>,
) -> M5CompareExportRow {
    M5CompareExportRow {
        consumer_surface,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5HistorySurfaceFamily::ALL.to_vec(),
        deployment_lines: M5HistoryDeploymentLine::ALL.to_vec(),
        card_anatomy_parts: M5RetentionExportCardAnatomyPart::ALL.to_vec(),
        manifest_anatomy_parts: M5ExportManifestAnatomyPart::ALL.to_vec(),
        retention_postures: M5RetentionPosture::ALL.to_vec(),
        export_redactions: M5ExportRedactionPosture::ALL.to_vec(),
        manifest_classes: M5ExportManifestClass::ALL.to_vec(),
        compare_baselines: M5CompareBaseline::ALL.to_vec(),
        card_postures: M5RetentionExportCardPosture::ALL.to_vec(),
        manifest_dispositions: M5ExportManifestDisposition::ALL.to_vec(),
        card_actions: M5RetentionExportCardAction::ALL.to_vec(),
        manifest_actions: M5ExportManifestAction::ALL.to_vec(),
        card_export_fields: M5RetentionExportCardExportField::ALL.to_vec(),
        manifest_export_fields: M5ExportManifestExportField::ALL.to_vec(),
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
            M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed,
            M5HistoryDowngradeTrigger::TimestampOrActorUnstated,
            M5HistoryDowngradeTrigger::FileOrObjectIdentityUnstated,
            M5HistoryDowngradeTrigger::GeneratedOrManagedCaveatHidden,
            M5HistoryDowngradeTrigger::ProofStale,
        ],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_COMPARE_EXPORT_CARD_SCHEMA_REF,
            M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
            M5_COMPARE_EXPORT_RETENTION_CARD_REF,
            M5_COMPARE_EXPORT_GIT_HISTORY_REF,
        ]),
        card_examples,
        manifest_examples,
        hides_export_baseline: false,
        hides_retention_or_redaction: false,
        defaults_to_raw_content_bodies: false,
        collapses_export_into_generic_download: false,
    }
}

fn rows() -> Vec<M5CompareExportRow> {
    use M5CompareBaseline as Baseline;
    use M5ExportManifestClass as Manifest;
    use M5ExportRedactionPosture as Redaction;
    use M5RetentionPosture as Retention;

    vec![
        // 1. Local history timeline — a fully-shareable, full-metadata card with every
        //    cross-baseline comparison on offer, plus a session-only, bodies-omitted card that
        //    reads as metadata-only survives; and a full-evidence audit-trail manifest.
        base_row(
            M5CompareExportConsumerSurface::LocalHistoryTimeline,
            M5HistoryQualificationClass::Stable,
            "Local history timeline owner",
            "The local-history timeline renders the shared retention/export card and history-export manifest so a workspace-retained checkpoint reads as fully shareable with current-versus-snapshot, snapshot-versus-disk, and snapshot-versus-Git comparisons all explicit, a session-only checkpoint reads honestly as metadata-only survives, and an audit-trail manifest exports as full evidence with actor lineage, checkpoint identity, and scope intact",
            "evidence:m5-compare-export-local-history:001",
            vec![
                card_case(
                    Retention::WorkspaceRetained,
                    Redaction::FullMetadata,
                    M5CompareBaseline::ALL.to_vec(),
                    true,
                    false,
                    true,
                    "history card: main.rs snapshot",
                ),
                card_case(
                    Retention::SessionOnly,
                    Redaction::BodiesOmitted,
                    vec![Baseline::CurrentVsSnapshot],
                    true,
                    true,
                    true,
                    "history card: scratch buffer snapshot",
                ),
            ],
            vec![manifest_case(
                Manifest::AuditTrail,
                Redaction::FullMetadata,
                Baseline::CurrentVsSnapshot,
                true,
                true,
                true,
                false,
                true,
                "history manifest: session audit trail",
            )],
        ),
        // 2. Refactor evidence — a purge-pending, paths-redacted card that reads as
        //    purge-scheduled and offers a retention extension; a redacted-share manifest
        //    measured against snapshot-versus-disk.
        base_row(
            M5CompareExportConsumerSurface::RefactorEvidence,
            M5HistoryQualificationClass::Stable,
            "Refactor evidence owner",
            "The refactor-evidence surface renders the shared card and manifest so a purge-pending refactor checkpoint reads as purge-scheduled, offers a retention extension before it purges, and exports a paths-redacted redacted-share manifest whose snapshot-versus-disk baseline stays explicit",
            "evidence:m5-compare-export-refactor:001",
            vec![card_case(
                Retention::PurgePending,
                Redaction::PathsRedacted,
                vec![Baseline::SnapshotVsDisk],
                true,
                false,
                true,
                "refactor card: extract-module transaction",
            )],
            vec![manifest_case(
                Manifest::RedactedShare,
                Redaction::PathsRedacted,
                Baseline::SnapshotVsDisk,
                true,
                true,
                true,
                false,
                true,
                "refactor manifest: extract-module evidence",
            )],
        ),
        // 3. Import / migration session — an account-synced, policy-restricted card that reads
        //    as policy-restricted; a policy-restricted migration-session manifest measured
        //    against snapshot-versus-Git HEAD that offers an unredacted-export request.
        base_row(
            M5CompareExportConsumerSurface::ImportMigrationSession,
            M5HistoryQualificationClass::Stable,
            "Import/migration session owner",
            "The import/migration-session surface renders the shared card and manifest so an account-synced import reads as policy-restricted, and a migration-session manifest measured against snapshot-versus-Git HEAD stays held behind policy with an explicit unredacted-export request rather than a silent share",
            "evidence:m5-compare-export-import:001",
            vec![card_case(
                Retention::AccountSynced,
                Redaction::PolicyRestricted,
                vec![Baseline::SnapshotVsGitHead],
                true,
                false,
                true,
                "import card: synced settings snapshot",
            )],
            vec![manifest_case(
                Manifest::MigrationSession,
                Redaction::PolicyRestricted,
                Baseline::SnapshotVsGitHead,
                true,
                true,
                true,
                false,
                true,
                "import manifest: migration session bundle",
            )],
        ),
        // 4. AI apply evidence — an export-blocked card whose redaction posture blocks export
        //    outright; a recovery-evidence manifest that would carry raw bodies and is held
        //    back rather than shared.
        base_row(
            M5CompareExportConsumerSurface::AiApplyEvidence,
            M5HistoryQualificationClass::Stable,
            "AI apply evidence owner",
            "The AI-apply evidence surface renders the shared card and manifest so a policy-pinned checkpoint whose redaction posture blocks export reads honestly as export-blocked, and a recovery-evidence manifest that would carry raw content bodies is held back as raw-body-withheld with an unredacted-export request rather than defaulting to a raw sensitive body",
            "evidence:m5-compare-export-ai-apply:001",
            vec![card_case(
                Retention::PolicyPinned,
                Redaction::ExportBlocked,
                vec![Baseline::CurrentVsSnapshot],
                false,
                false,
                true,
                "ai apply card: agent run checkpoint",
            )],
            vec![manifest_case(
                Manifest::RecoveryEvidence,
                Redaction::FullMetadata,
                Baseline::CurrentVsSnapshot,
                true,
                true,
                true,
                true,
                true,
                "ai apply manifest: agent run evidence",
            )],
        ),
        // 5. Recovery center — an expired-and-purged card that reads as nothing-retained but
        //    still offers a compare and a retention-extension request; a recovery-evidence
        //    manifest whose actor lineage is incomplete and is held back.
        base_row(
            M5CompareExportConsumerSurface::RecoveryCenter,
            M5HistoryQualificationClass::Stable,
            "Recovery center owner",
            "The recovery-center surface renders the shared card and manifest so an expired-and-purged checkpoint reads as nothing-retained yet still exposes a snapshot-versus-Git comparison and a retention-extension request, and a recovery-evidence manifest whose actor lineage is not fully preserved is held back as lineage-incomplete rather than shared as full evidence",
            "evidence:m5-compare-export-recovery:001",
            vec![card_case(
                Retention::ExpiredPurged,
                Redaction::FullMetadata,
                vec![Baseline::SnapshotVsGitHead],
                true,
                false,
                true,
                "recovery card: purged checkpoint",
            )],
            vec![manifest_case(
                Manifest::RecoveryEvidence,
                Redaction::FullMetadata,
                Baseline::SnapshotVsGitHead,
                false,
                true,
                true,
                false,
                true,
                "recovery manifest: partial-lineage evidence",
            )],
        ),
        // 6. Support export desk — a workspace-retained, credentials-scrubbed card that reads
        //    as fully shareable; a support-bundle manifest whose export path is unavailable and
        //    reads as export-blocked.
        base_row(
            M5CompareExportConsumerSurface::SupportExportDesk,
            M5HistoryQualificationClass::Stable,
            "Support export desk owner",
            "The support export desk renders the shared card and manifest so a workspace-retained checkpoint whose credentials are scrubbed still reads as fully shareable over a snapshot-versus-disk comparison, and a support-bundle manifest whose export path is unavailable reads as export-blocked rather than presenting a false download",
            "evidence:m5-compare-export-support:001",
            vec![card_case(
                Retention::WorkspaceRetained,
                Redaction::CredentialsScrubbed,
                vec![Baseline::SnapshotVsDisk],
                true,
                false,
                true,
                "support card: scrubbed workspace snapshot",
            )],
            vec![manifest_case(
                Manifest::SupportBundle,
                Redaction::ExportBlocked,
                Baseline::CurrentVsSnapshot,
                true,
                true,
                true,
                false,
                false,
                "support manifest: offline support bundle",
            )],
        ),
    ]
}

fn governance_review() -> M5CompareExportGovernanceReview {
    M5CompareExportGovernanceReview {
        one_primitive_carries_card_and_manifest_truth: true,
        compare_baseline_always_explicit: true,
        retention_posture_always_disclosed: true,
        export_redaction_always_disclosed: true,
        survival_and_expiry_always_stated: true,
        no_export_defaults_to_raw_bodies: true,
        lineage_identity_and_scope_survive_export: true,
        export_never_generic_download: true,
        support_export_reconstructs_card_and_manifest_truth: true,
        no_surface_invents_parallel_vocabulary: true,
        every_row_declares_accessibility_route: true,
        descriptors_stable_across_ui_export_support: true,
    }
}

fn consumer_projection() -> M5CompareExportConsumerProjection {
    M5CompareExportConsumerProjection {
        compare_export_surfaces_consume_shared_primitive: true,
        card_posture_reads_single_source: true,
        manifest_disposition_reads_single_source: true,
        actions_read_single_source: true,
        support_export_reads_single_source: true,
    }
}

fn proof_freshness() -> M5CompareExportProofFreshness {
    M5CompareExportProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5CompareExportReleasePosture {
    M5CompareExportReleasePosture {
        release_packet_ref: M5_COMPARE_EXPORT_ARTIFACT_REF.to_owned(),
        recovery_audit_ref: M5_COMPARE_EXPORT_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_COMPARE_EXPORT_CARD_SCHEMA_REF,
        M5_COMPARE_EXPORT_MANIFEST_SCHEMA_REF,
        M5_COMPARE_EXPORT_DOC_REF,
        M5_COMPARE_EXPORT_COMPONENT_MATRIX_REF,
        M5_COMPARE_EXPORT_RETENTION_CARD_REF,
        M5_COMPARE_EXPORT_GIT_HISTORY_REF,
    ])
}

/// Builds the canonical M5 compare-export packet.
pub fn seeded_m5_compare_export_packet() -> M5CompareExportPacket {
    M5CompareExportPacket::new(M5CompareExportPacketInput {
        packet_id: M5_COMPARE_EXPORT_PACKET_ID.to_owned(),
        matrix_label:
            "M5 retention/export-card & history-export-manifest primitive: cross-baseline compare (current-versus-snapshot, snapshot-versus-disk, snapshot-versus-Git HEAD), retention posture, export redaction, survival/expiry/metadata-only truth, actor-lineage and scope preservation, and bounded inspect/review/compare/export/request actions with no export defaulting to raw sensitive bodies"
                .to_owned(),
        rows: rows(),
        vocabulary_set: M5CompareExportVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the import/migration-session consumer is narrowed to Preview pending
/// policy-restricted export parity proof across every headless import path; every consumer
/// stays visible.
pub fn seeded_m5_compare_export_import_migration_session_preview_narrowed() -> M5CompareExportPacket {
    let mut packet = seeded_m5_compare_export_packet();
    packet.packet_id = "m5-retention-export-card-primitive:import-migration-session-preview:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CompareExportConsumerSurface::ImportMigrationSession)
        .expect("import/migration-session row present");
    row.qualification = M5HistoryQualificationClass::Preview;
    packet
}

/// Narrowed variant: the AI-apply-evidence consumer is held at Beta because a slice of AI-apply
/// evidence exports do not yet render the raw-body-withheld cue on every profile; every consumer
/// stays visible.
pub fn seeded_m5_compare_export_ai_apply_evidence_beta_narrowed() -> M5CompareExportPacket {
    let mut packet = seeded_m5_compare_export_packet();
    packet.packet_id = "m5-retention-export-card-primitive:ai-apply-evidence-beta:0001".to_owned();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5CompareExportConsumerSurface::AiApplyEvidence)
        .expect("ai-apply-evidence row present");
    row.qualification = M5HistoryQualificationClass::Beta;
    packet
}
