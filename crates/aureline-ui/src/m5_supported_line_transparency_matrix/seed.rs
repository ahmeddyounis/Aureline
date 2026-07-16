//! Canonical seed builders for the frozen M5 supported-line transparency matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical supported-line transparency matrix.
pub const M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_PACKET_ID: &str =
    "m5-supported-line-transparency:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

/// The three mandatory labels every line must be able to show.
fn mandatory_labels() -> Vec<M5SupportedLineTransparencyRequiredLabel> {
    M5SupportedLineTransparencyRequiredLabel::MANDATORY.to_vec()
}

/// Mandatory labels plus additional truth labels a line carries.
fn labels_with(
    extra: &[M5SupportedLineTransparencyRequiredLabel],
) -> Vec<M5SupportedLineTransparencyRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

/// A base row with the fields shared by every line filled in and every line-specific vocabulary left empty
/// for the caller to populate.
fn base_row(
    proof_object: M5SupportedLineTransparencyObject,
    qualification: M5SupportedLineTransparencyQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    source_refs: &[&str],
) -> M5SupportedLineTransparencyRow {
    M5SupportedLineTransparencyRow {
        proof_object,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        surface_families: M5SupportedLineTransparencySurfaceFamily::ALL.to_vec(),
        widening_stages: M5SupportedLineTransparencyWideningStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        public_proof_ledger_roles: vec![],
        transparency_report_roles: vec![],
        migration_scoreboard_roles: vec![],
        orr_history_event_roles: vec![],
        correction_train_archive_roles: vec![],
        degraded_reasons: M5SupportedLineTransparencyDegradedReason::ALL.to_vec(),
        accessibility_routes: M5SupportedLineTransparencyAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5SupportedLineTransparencyConsumerSurface::SupportExport,
            M5SupportedLineTransparencyConsumerSurface::DocsHelp,
        ],
        downgrade_triggers: vec![M5SupportedLineTransparencyDowngradeTrigger::ProofStale],
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(source_refs),
        widens_a_claim_because_a_report_once_existed_without_current_freshness: false,
        stays_green_on_stale_external_proof_or_opaque_upstream_health: false,
        leaks_internal_only_incident_or_security_detail_into_public_safe_feeds: false,
        leaves_public_proof_migration_or_history_unjoined_to_build_and_release_line_identity: false,
        leaves_migration_pain_or_orr_and_correction_history_unretained: false,
    }
}

fn supported_line_transparency_rows() -> Vec<M5SupportedLineTransparencyRow> {
    use M5SupportedLineTransparencyConsumerSurface as C;
    use M5SupportedLineTransparencyDowngradeTrigger as D;
    use M5SupportedLineTransparencyObject as F;
    use M5SupportedLineTransparencyQualificationClass as Q;
    use M5SupportedLineTransparencyRequiredLabel as L;
    use M5SupportedLineTransparencyRole as R;

    let mut rows = Vec::new();

    // 1. Public-proof ledger.
    let mut row = base_row(
        F::PublicProofLedger,
        Q::Stable,
        "Public-proof ledger owner",
        "One public-proof ledger naming the current public-claim proof, the published compatibility report, the current support-window proof, and the freshness window met so external claims, partner reviews, and procurement checks inherit current rather than tribal truth",
        "evidence:m5-public-proof-ledger-parity:001",
        &[
            M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
    );
    row.public_proof_ledger_roles = M5PublicProofLedgerRole::ALL.to_vec();
    row.semantic_roles = vec![R::FreshnessWindow, R::PublicProofFreshness];
    row.required_labels = labels_with(&[L::FreshnessWindow]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::Diagnostics,
        C::SupportExport,
        C::PublicProof,
    ];
    row.downgrade_triggers = vec![
        D::WidenedClaimOnStalePublicProof,
        D::RanSupportLanguageAheadOfPublicProof,
        D::FreshnessWindowUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 2. Transparency report.
    let mut row = base_row(
        F::TransparencyReport,
        Q::Stable,
        "Transparency-report owner",
        "One transparency report naming the upstream health reported, the compatibility health reported, the maintainer durability reported, and the export-safe public view kept so no internal-only incident or security detail ever leaks into a public-safe or partner/procurement feed",
        "evidence:m5-transparency-report-parity:001",
        &[
            M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
    );
    row.transparency_report_roles = M5TransparencyReportRole::ALL.to_vec();
    row.semantic_roles = vec![R::TransparencyDisclosure, R::FreshnessWindow];
    row.required_labels = labels_with(&[L::ExportClass]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::PublicProof,
        C::Diagnostics,
        C::SupportExport,
    ];
    row.downgrade_triggers = vec![
        D::WidenedClaimWithoutCurrentTransparencyReport,
        D::LeakedInternalDetailIntoPublicProof,
        D::ExportClassUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 3. Migration scoreboard.
    let mut row = base_row(
        F::MigrationScoreboard,
        Q::Stable,
        "Migration-scoreboard owner",
        "One migration scoreboard naming the migration path scored, the migration blockers tracked, the migration-pain deltas recorded, and the scoreboard versioned so migration pain is never forgotten between release trains",
        "evidence:m5-migration-scoreboard-parity:001",
        &[
            M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            M5_MIGRATION_SCOREBOARD_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
    );
    row.migration_scoreboard_roles = M5MigrationScoreboardRole::ALL.to_vec();
    row.semantic_roles = vec![R::MigrationScoreboardCurrency, R::CorrectionHistoryJoin];
    row.required_labels = labels_with(&[L::ExportClass]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ProgramGovernance,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::LeftMigrationPainUnscored,
        D::WidenedClaimWithoutCurrentTransparencyReport,
        D::ExportClassUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 4. ORR-history event.
    let mut row = base_row(
        F::OrrHistoryEvent,
        Q::Stable,
        "ORR-history archive owner",
        "One ORR-history event naming the ORR decision event recorded, the go/no-go outcome preserved, the support-window decision retained, and the history event archived so supported-line decisions are never lost to memory",
        "evidence:m5-orr-history-event-parity:001",
        &[
            M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
    );
    row.orr_history_event_roles = M5OrrHistoryEventRole::ALL.to_vec();
    row.semantic_roles = vec![R::OrrHistoryRetention, R::PublicProofFreshness];
    row.required_labels = labels_with(&[L::LineAssociation]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ProgramGovernance,
        C::Diagnostics,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::OrrHistoryUnretained,
        D::ImpliedGreenWhileProofOrArchiveWasStale,
        D::LineAssociationUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    // 5. Correction-train archive.
    let mut row = base_row(
        F::CorrectionTrainArchive,
        Q::Stable,
        "Correction-train archive owner",
        "One correction-train archive naming the correction-train packet archived, the hotfix/backport packet archived, the advisory packet archived, and the archive packet bound to exact build identity so correction history stays durable and inspectable",
        "evidence:m5-correction-train-archive-parity:001",
        &[
            M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
            M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
    );
    row.correction_train_archive_roles = M5CorrectionTrainArchiveRole::ALL.to_vec();
    row.semantic_roles = vec![R::CorrectionArchiveRetention, R::CorrectionHistoryJoin];
    row.required_labels = labels_with(&[L::LineAssociation]);
    row.consumer_surfaces = vec![
        C::Shiproom,
        C::ReleaseCenter,
        C::ExecutiveSteering,
        C::ProgramGovernance,
        C::SupportExport,
        C::CliExport,
    ];
    row.downgrade_triggers = vec![
        D::ImpliedGreenWhileProofOrArchiveWasStale,
        D::LeakedInternalDetailIntoPublicProof,
        D::LineAssociationUnstated,
        D::RegistryReferenceUnstated,
        D::ProofStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5SupportedLineTransparencyGovernanceReview {
    M5SupportedLineTransparencyGovernanceReview {
        no_supported_line_stays_green_on_stale_external_proof: true,
        every_supported_object_names_owner_freshness_window_and_export_class: true,
        migration_scoreboard_stays_versioned_and_current: true,
        orr_and_correction_history_is_retained_not_forgotten: true,
        transparency_reports_stay_export_safe_public_view: true,
        public_proof_ledgers_stay_current_on_active_lines: true,
        correction_train_packets_are_archived_and_build_bound: true,
        internal_incident_detail_never_leaks_into_public_feeds: true,
        every_object_declares_widening_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_transparency_source: true,
        release_help_and_support_bind_to_single_transparency_source: true,
        later_rows_cannot_invent_parallel_transparency_vocabulary: true,
        transparency_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
        support_language_never_outruns_current_public_proof: true,
    }
}

fn consumer_projection() -> M5SupportedLineTransparencyConsumerProjection {
    M5SupportedLineTransparencyConsumerProjection {
        release_and_help_consume_shared_transparency_truth: true,
        support_and_public_proof_consume_shared_public_proof_and_freshness_truth: true,
        diagnostics_and_cli_export_consume_shared_migration_and_archive_truth: true,
        docs_help_and_screenshots_read_single_transparency_source: true,
        orr_and_correction_archives_bind_to_shared_build_identity: true,
        support_export_reads_single_transparency_source: true,
    }
}

fn proof_freshness() -> M5SupportedLineTransparencyProofFreshness {
    M5SupportedLineTransparencyProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SupportedLineTransparencyReleasePosture {
    M5SupportedLineTransparencyReleasePosture {
        proof_packet_ref: M5_SUPPORTED_LINE_TRANSPARENCY_ARTIFACT_REF.to_owned(),
        supported_line_transparency_audit_ref: M5_SUPPORTED_LINE_TRANSPARENCY_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_SCHEMA_REF,
        M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_DOC_REF,
        M5_PUBLIC_PROOF_FRESHNESS_LEDGER_DOMAIN_SCHEMA_REF,
        M5_MIGRATION_SCOREBOARD_DOMAIN_SCHEMA_REF,
        M5_SUPPORTED_LINE_ORR_HISTORY_DOMAIN_SCHEMA_REF,
        M5_CORRECTION_TRAIN_ARCHIVE_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 supported-line transparency matrix packet.
pub fn seeded_m5_supported_line_transparency_matrix() -> M5SupportedLineTransparencyMatrixPacket {
    M5SupportedLineTransparencyMatrixPacket::new(M5SupportedLineTransparencyMatrixPacketInput {
        packet_id: M5_SUPPORTED_LINE_TRANSPARENCY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 supported-line public-proof, transparency-report, migration-scoreboard, ORR-history, and correction-train-archive matrix"
                .to_owned(),
        supported_line_transparency_rows: supported_line_transparency_rows(),
        vocabulary_set: M5SupportedLineTransparencyVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: the ORR-history event is held at Beta because its ORR / go-no-go decision history is not
/// yet fully archived for the active line; every object stays visible.
pub fn seeded_m5_supported_line_transparency_matrix_orr_history_event_beta_narrowed(
) -> M5SupportedLineTransparencyMatrixPacket {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.packet_id = "m5-supported-line-transparency:orr-history-event-beta:0001".to_owned();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::OrrHistoryEvent)
        .expect("orr-history-event row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Beta;
    packet
}

/// Narrowed variant: the correction-train archive is narrowed to Preview pending correction-train / advisory
/// packets archived and bound to exact build identity; every object stays visible.
pub fn seeded_m5_supported_line_transparency_matrix_correction_train_archive_preview_narrowed(
) -> M5SupportedLineTransparencyMatrixPacket {
    let mut packet = seeded_m5_supported_line_transparency_matrix();
    packet.packet_id =
        "m5-supported-line-transparency:correction-train-archive-preview:0001".to_owned();
    let row = packet
        .supported_line_transparency_rows
        .iter_mut()
        .find(|row| row.proof_object == M5SupportedLineTransparencyObject::CorrectionTrainArchive)
        .expect("correction-train-archive row present");
    row.qualification = M5SupportedLineTransparencyQualificationClass::Preview;
    packet
}
