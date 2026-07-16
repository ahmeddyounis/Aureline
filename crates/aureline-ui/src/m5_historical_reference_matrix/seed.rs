//! Canonical seed builders for the frozen M5 historical-reference matrix.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code matrix, the artifact, and the
//! fixtures never drift.

use super::*;

/// Stable packet id for the canonical historical-reference matrix.
pub const M5_HISTORICAL_REFERENCE_MATRIX_PACKET_ID: &str = "m5-historical-reference:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn mandatory_labels() -> Vec<M5HistoricalReferenceRequiredLabel> {
    M5HistoricalReferenceRequiredLabel::MANDATORY.to_vec()
}

fn labels_with(
    extra: &[M5HistoricalReferenceRequiredLabel],
) -> Vec<M5HistoricalReferenceRequiredLabel> {
    let mut labels = mandatory_labels();
    labels.extend_from_slice(extra);
    labels
}

#[allow(clippy::too_many_arguments)]
fn base_row(
    object_class: M5HistoricalReferenceObject,
    qualification: M5HistoricalReferenceQualificationClass,
    owner_role: &str,
    backup_owner_role: &str,
    scope_summary: &str,
    closure_ref: &str,
    source_refs: &[&str],
    required_visible_state: M5HistoricalReferenceVisibleState,
) -> M5HistoricalReferenceRow {
    M5HistoricalReferenceRow {
        object_class,
        qualification,
        evidence_state: M5HistoricalReferenceEvidenceState::ArchivedSnapshot,
        owner_role: owner_role.to_owned(),
        backup_owner_role: backup_owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_visible_state,
        surface_families: M5HistoricalReferenceSurfaceFamily::ALL.to_vec(),
        capture_lifecycle_stages: M5HistoricalReferenceCaptureLifecycleStage::ALL.to_vec(),
        required_labels: mandatory_labels(),
        semantic_roles: vec![],
        retirement_snapshot_roles: vec![],
        support_export_evidence_roles: vec![],
        archived_runbook_packet_roles: vec![],
        imported_offline_route_evidence_roles: vec![],
        review_incident_snapshot_roles: vec![],
        degraded_reasons: M5HistoricalReferenceDegradedReason::ALL.to_vec(),
        accessibility_routes: M5HistoricalReferenceAccessibilityRoute::ALL.to_vec(),
        consumer_surfaces: vec![
            M5HistoricalReferenceConsumerSurface::Support,
            M5HistoricalReferenceConsumerSurface::HelpDocs,
        ],
        downgrade_triggers: vec![M5HistoricalReferenceDowngradeTrigger::HistoricalReferenceDescriptorStale],
        required_closure_artifact_refs: strings(&[closure_ref]),
        source_contract_refs: strings(source_refs),
        lets_archived_or_imported_evidence_look_live_writable_or_current_by_omission: false,
        reopens_a_live_target_from_a_snapshot_without_validating_identity_trust_route_and_authority: false,
        dead_links_an_expired_or_removed_artifact_instead_of_showing_metadata_provenance_or_cleanup_state: false,
        leaves_non_live_evidence_unjoined_to_capture_time_provenance_retention_state_or_live_target_mismatch: false,
        presents_a_snapshot_or_imported_packet_as_a_current_live_object_or_reopens_through_an_ambiguous_route: false,
    }
}

fn txn(f: [&str; 8]) -> M5HistoricalReferenceVisibleState {
    M5HistoricalReferenceVisibleState {
        snapshot_label: f[0].to_owned(),
        capture_time: f[1].to_owned(),
        provenance: f[2].to_owned(),
        live_target_availability: f[3].to_owned(),
        imported_offline_status: f[4].to_owned(),
        mutation_blocked_posture: f[5].to_owned(),
        expiry_removal_state: f[6].to_owned(),
        live_target_handoff_or_exit: f[7].to_owned(),
    }
}

fn historical_reference_rows() -> Vec<M5HistoricalReferenceRow> {
    use M5HistoricalReferenceConsumerSurface as C;
    use M5HistoricalReferenceDowngradeTrigger as D;
    use M5HistoricalReferenceObject as O;
    use M5HistoricalReferenceQualificationClass as Q;
    use M5HistoricalReferenceRequiredLabel as L;
    use M5HistoricalReferenceRole as R;

    let mut rows = Vec::new();

    // 1. RetirementSnapshot.
    let mut row = base_row(
        O::RetirementSnapshot,
        Q::Stable,
        "Retirement-snapshot evidence owner",
        "Release-governance backup owner",
        "One retirement / last-supported snapshot is shown as captured evidence, not a live object: it carries the archived-snapshot label, the capture time and last-supported provenance, an explicit open-current-object handoff when a successor still exists, and a mutation-blocked posture so the snapshot can be inspected without being mistaken for current truth",
        "evidence:m5-retirement-snapshot-closure:001",
        &[
            M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "captured evidence / archived snapshot",
            "captured 2026-06-01",
            "provenance last-supported build 6.4.219 retirement record",
            "live target available via validated open-current-object handoff",
            "native capture, not imported",
            "read-only, non-authoritative for mutation",
            "retained with metadata and archival pointer",
            "open current live object via validated handoff to successor line v7.0",
        ]),
    );
    row.retirement_snapshot_roles = M5HistoricalReferenceRetirementSnapshotRole::ALL.to_vec();
    row.semantic_roles = vec![R::SnapshotLabeling, R::LiveTargetHandoff];
    row.required_labels = labels_with(&[L::SnapshotLabel]);
    row.consumer_surfaces = vec![
        C::Shell,
        C::HelpDocs,
        C::Support,
        C::ReleaseCenter,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.evidence_state = M5HistoricalReferenceEvidenceState::ArchivedSnapshot;
    row.downgrade_triggers = vec![
        D::ArchivedSnapshotShownAsLive,
        D::MutationAllowedOnNonLiveEvidence,
        D::LiveTargetAvailabilityUnstated,
        D::EvidenceUnjoinedFromCaptureContext,
        D::HistoricalReferenceDescriptorStale,
    ];
    rows.push(row);

    // 2. SupportExportEvidence.
    let mut row = base_row(
        O::SupportExportEvidence,
        Q::Stable,
        "Support / export evidence owner",
        "Support-governance backup owner",
        "One captured support / export evidence bundle is labeled as a snapshot with its capture context, shows its retention / expiry / removal state, and offers a metadata-only inspection exit when no live object remains so support can reopen the record without pretending it is editable or current",
        "evidence:m5-support-export-evidence-closure:001",
        &[
            M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "captured support / export evidence snapshot",
            "captured 2026-05-20",
            "provenance support export bundle case 4821",
            "no live target: metadata-only inspection exit",
            "native capture, not imported",
            "read-only, non-authoritative for mutation",
            "expired but retained with metadata and cleanup state",
            "metadata-only inspection exit; no current live object to reopen",
        ]),
    );
    row.support_export_evidence_roles =
        M5HistoricalReferenceSupportExportEvidenceRole::ALL.to_vec();
    row.semantic_roles = vec![R::CaptureTimeAttribution, R::ExpiryRemovalHandling];
    row.required_labels = labels_with(&[L::CaptureTime]);
    row.consumer_surfaces = vec![
        C::Support,
        C::HelpDocs,
        C::Shell,
        C::CompanionExport,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.evidence_state = M5HistoricalReferenceEvidenceState::ArchivedSnapshot;
    row.downgrade_triggers = vec![
        D::MutationAllowedOnNonLiveEvidence,
        D::CaptureTimeMissing,
        D::ExpiredArtifactDeadLinked,
        D::RemovalStateUnstated,
        D::HistoricalReferenceDescriptorStale,
    ];
    rows.push(row);

    // 3. ArchivedRunbookPacket.
    let mut row = base_row(
        O::ArchivedRunbookPacket,
        Q::Stable,
        "Archived-runbook evidence owner",
        "Runbook-governance backup owner",
        "One archived runbook execution packet is labeled as a historical run with its capture time and provenance, and any open-live-run action first validates target identity, trust, route, and authority so an archived run is never silently re-executed as if it were live",
        "evidence:m5-archived-runbook-packet-closure:001",
        &[
            M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "archived runbook execution packet",
            "captured 2026-04-11",
            "provenance runbook run id run-2026-0411-07",
            "live run target available via validated open-live-run handoff",
            "native capture, not imported",
            "read-only, non-authoritative for mutation",
            "retained with metadata and archival pointer",
            "open live run via handoff validated for identity, trust, route, and authority",
        ]),
    );
    row.archived_runbook_packet_roles =
        M5HistoricalReferenceArchivedRunbookPacketRole::ALL.to_vec();
    row.semantic_roles = vec![R::ProvenanceAttribution, R::LiveTargetHandoff];
    row.required_labels = labels_with(&[L::LiveTargetAvailability]);
    row.consumer_surfaces = vec![
        C::RunbookArchive,
        C::Shell,
        C::Support,
        C::ReviewIncident,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.evidence_state = M5HistoricalReferenceEvidenceState::ArchivedSnapshot;
    row.downgrade_triggers = vec![
        D::LiveTargetReopenedWithoutValidation,
        D::MutationAllowedOnNonLiveEvidence,
        D::LiveTargetAvailabilityUnstated,
        D::EvidenceUnjoinedFromCaptureContext,
        D::HistoricalReferenceDescriptorStale,
    ];
    rows.push(row);

    // 4. ImportedOfflineRouteEvidence.
    let mut row = base_row(
        O::ImportedOfflineRouteEvidence,
        Q::Stable,
        "Imported / offline route-evidence owner",
        "Continuity-governance backup owner",
        "One imported / offline route-evidence record carries its imported / offline-only warning, its import context, a controlled restore-fidelity disclosure, and any current live-route mismatch so imported route data never masquerades as current live route, service, or workspace truth",
        "evidence:m5-imported-offline-route-evidence-closure:001",
        &[
            M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            M5_IMPORTED_OFFLINE_EVIDENCE_STATE_DOMAIN_SCHEMA_REF,
            M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
        ],
        txn([
            "imported / offline evidence only",
            "captured 2026-03-02",
            "provenance imported offline packet import-2026-0302",
            "live route target unavailable offline: metadata-only inspection exit",
            "imported / offline evidence only",
            "read-only, non-authoritative for mutation",
            "retained with metadata; restore fidelity partial",
            "metadata-only inspection exit; current live route mismatch flagged for validation",
        ]),
    );
    row.imported_offline_route_evidence_roles =
        M5HistoricalReferenceImportedOfflineRouteEvidenceRole::ALL.to_vec();
    row.semantic_roles = vec![R::ImportedOfflineDisclosure, R::ProvenanceAttribution];
    row.required_labels = labels_with(&[L::LiveTargetAvailability]);
    row.consumer_surfaces = vec![
        C::Shell,
        C::Support,
        C::HelpDocs,
        C::CompanionExport,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.evidence_state = M5HistoricalReferenceEvidenceState::ImportedOfflineEvidence;
    row.downgrade_triggers = vec![
        D::ImportedOfflineEvidenceShownAsCurrent,
        D::LiveTargetReopenedWithoutValidation,
        D::ProvenanceUnattributed,
        D::EvidenceUnjoinedFromCaptureContext,
        D::HistoricalReferenceDescriptorStale,
    ];
    rows.push(row);

    // 5. ReviewIncidentSnapshot.
    let mut row = base_row(
        O::ReviewIncidentSnapshot,
        Q::Stable,
        "Review / incident snapshot owner",
        "Incident-governance backup owner",
        "One review / incident snapshot is labeled as captured evidence with its capture time and provenance, holds a mutation-blocked posture, and offers an open-current-object handoff validated for identity, trust, route, and authority so a review snapshot is never edited or reopened as if it were the live object",
        "evidence:m5-review-incident-snapshot-closure:001",
        &[
            M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF,
            M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        ],
        txn([
            "captured review / incident snapshot",
            "captured 2026-02-14",
            "provenance incident record inc-2026-0214-03",
            "current object available via validated open-current-object handoff",
            "native capture, not imported",
            "read-only, non-authoritative for mutation",
            "retained with metadata and archival pointer",
            "open current live object via handoff validated for identity, trust, route, and authority",
        ]),
    );
    row.review_incident_snapshot_roles =
        M5HistoricalReferenceReviewIncidentSnapshotRole::ALL.to_vec();
    row.semantic_roles = vec![R::MutationBlockedPosture, R::LiveTargetHandoff];
    row.required_labels = labels_with(&[L::SnapshotLabel]);
    row.consumer_surfaces = vec![
        C::ReviewIncident,
        C::Shell,
        C::Support,
        C::HelpDocs,
        C::ProgramGovernance,
        C::CliExport,
    ];
    row.evidence_state = M5HistoricalReferenceEvidenceState::ArchivedSnapshot;
    row.downgrade_triggers = vec![
        D::ArchivedSnapshotShownAsLive,
        D::MutationAllowedOnNonLiveEvidence,
        D::LiveTargetReopenedWithoutValidation,
        D::EvidenceUnjoinedFromCaptureContext,
        D::HistoricalReferenceDescriptorStale,
    ];
    rows.push(row);

    rows
}

fn governance_review() -> M5HistoricalReferenceGovernanceReview {
    M5HistoricalReferenceGovernanceReview {
        no_archived_or_imported_evidence_looks_live_writable_or_current_by_omission: true,
        every_covered_object_class_names_owner_backup_owner_and_first_consumer: true,
        non_live_evidence_is_mechanically_distinct_from_live_cached_and_restore_capable_state: true,
        every_snapshot_carries_capture_time_and_provenance_before_it_is_surfaced: true,
        every_live_target_handoff_validates_target_identity_trust_route_and_authority: true,
        metadata_only_exit_is_offered_when_a_current_object_can_no_longer_be_reopened: true,
        expired_or_removed_artifacts_show_metadata_provenance_or_cleanup_state_never_a_dead_link:
            true,
        imported_and_offline_evidence_always_carries_its_non_live_disclosure: true,
        non_live_evidence_stays_joined_to_capture_context_and_live_target_mismatch: true,
        every_object_declares_capture_lifecycle_stages: true,
        every_object_declares_accessibility_route: true,
        support_export_reads_single_historical_reference_source: true,
        shell_help_support_review_runbook_and_companion_bind_to_single_source: true,
        later_rows_cannot_invent_parallel_historical_reference_vocabulary: true,
        historical_reference_truth_survives_zoom_and_high_contrast: true,
        claims_narrow_automatically_when_matrix_row_missing_or_stale: true,
    }
}

fn consumer_projection() -> M5HistoricalReferenceConsumerProjection {
    M5HistoricalReferenceConsumerProjection {
        shell_and_help_consume_shared_historical_reference_truth: true,
        support_and_review_consume_shared_snapshot_and_handoff_truth: true,
        runbook_archive_and_companion_export_consume_shared_non_live_evidence_truth: true,
        docs_help_and_screenshots_read_single_historical_reference_source: true,
        archives_and_snapshots_bind_to_shared_capture_context: true,
        support_export_reads_single_historical_reference_source: true,
    }
}

fn proof_freshness() -> M5HistoricalReferenceProofFreshness {
    M5HistoricalReferenceProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5HistoricalReferenceReleasePosture {
    M5HistoricalReferenceReleasePosture {
        proof_packet_ref: M5_HISTORICAL_REFERENCE_ARTIFACT_REF.to_owned(),
        historical_reference_audit_ref: M5_HISTORICAL_REFERENCE_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
        M5_HISTORICAL_SNAPSHOT_DESCRIPTOR_DOMAIN_SCHEMA_REF,
        M5_LIVE_TARGET_HANDOFF_DOMAIN_SCHEMA_REF,
        M5_IMPORTED_OFFLINE_EVIDENCE_STATE_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ])
}

/// Builds the canonical frozen M5 historical-reference matrix packet.
pub fn seeded_m5_historical_reference_matrix() -> M5HistoricalReferenceMatrixPacket {
    M5HistoricalReferenceMatrixPacket::new(M5HistoricalReferenceMatrixPacketInput {
        packet_id: M5_HISTORICAL_REFERENCE_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 historical-reference, archived-snapshot, imported/offline-evidence, and live-target-handoff matrix"
            .to_owned(),
        historical_reference_rows: historical_reference_rows(),
        vocabulary_set: M5HistoricalReferenceVocabularySet::canonical(),
        governance_review: governance_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    })
}

/// Narrowed variant: imported / offline route evidence is held at Beta because its restore-fidelity
/// disclosure and live-route-mismatch checks are not yet fully proven; every object class stays visible.
pub fn seeded_m5_historical_reference_matrix_imported_offline_route_evidence_beta_narrowed(
) -> M5HistoricalReferenceMatrixPacket {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.packet_id =
        "m5-historical-reference:imported-offline-route-evidence-beta:0001".to_owned();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::ImportedOfflineRouteEvidence)
        .expect("imported-offline-route-evidence row present");
    row.qualification = M5HistoricalReferenceQualificationClass::Beta;
    packet
}

/// Narrowed variant: the review / incident snapshot is narrowed to Preview pending open-current-object
/// handoff validation and capture-provenance proof; every object class stays visible.
pub fn seeded_m5_historical_reference_matrix_review_incident_snapshot_preview_narrowed(
) -> M5HistoricalReferenceMatrixPacket {
    let mut packet = seeded_m5_historical_reference_matrix();
    packet.packet_id = "m5-historical-reference:review-incident-snapshot-preview:0001".to_owned();
    let row = packet
        .historical_reference_rows
        .iter_mut()
        .find(|row| row.object_class == M5HistoricalReferenceObject::ReviewIncidentSnapshot)
        .expect("review-incident-snapshot row present");
    row.qualification = M5HistoricalReferenceQualificationClass::Preview;
    packet
}
