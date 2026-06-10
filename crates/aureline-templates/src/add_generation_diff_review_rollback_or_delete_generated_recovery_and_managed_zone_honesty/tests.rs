use super::*;

const PACKET_ID: &str = "generation-recovery:stable:0001";
const PACKET_LABEL: &str =
    "Generation Diff Review, Rollback/Delete-Generated Recovery, and Managed-Zone Honesty";

const CLEAN_ROW: &str = "generation-recovery-row:rust_cli.clean_generate:2026.05";
const MIXED_ROW: &str = "generation-recovery-row:node_service.mixed_zone:2026.05";
const BRIDGE_ROW: &str = "generation-recovery-row:web_pack.bridge_map:2026.04";
const UNKNOWN_ROW: &str = "generation-recovery-row:imported.unknown_lineage:2026.03";

fn proof_freshness() -> GenerationRecoveryProofFreshness {
    GenerationRecoveryProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-07T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> GenerationRecoveryPacket {
    canonical_generation_recovery(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        "2026-06-07T00:00:00Z".to_owned(),
        proof_freshness(),
    )
}

fn row<'a>(packet: &'a GenerationRecoveryPacket, row_id: &str) -> &'a GenerationRecoveryRow {
    packet
        .rows
        .iter()
        .find(|row| row.row_id == row_id)
        .unwrap_or_else(|| panic!("missing row {row_id}"))
}

#[test]
fn generation_recovery_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_rows_cover_managed_zone_and_recovery_spectrum() {
    let packet = packet();
    let zones: Vec<ManagedZoneClass> = packet
        .rows
        .iter()
        .map(|row| row.managed_zone_class)
        .collect();
    for required in [
        ManagedZoneClass::GeneratedOnly,
        ManagedZoneClass::MixedAuthoredAndGenerated,
        ManagedZoneClass::GeneratedThenUserEdited,
        ManagedZoneClass::ZoneUnknownReviewRequired,
    ] {
        assert!(
            zones.contains(&required),
            "missing zone {}",
            required.as_str()
        );
    }
    let actions: Vec<GenerationRecoveryActionClass> = packet
        .rows
        .iter()
        .map(|row| row.recovery_action_class)
        .collect();
    assert!(actions.contains(&GenerationRecoveryActionClass::RollbackToCheckpoint));
    assert!(actions.contains(&GenerationRecoveryActionClass::DeleteGeneratedOnly));
    assert!(actions.contains(&GenerationRecoveryActionClass::RecoveryBlockedLineageUnknown));
}

#[test]
fn lineage_unknown_row_is_blocked_in_canonical_packet() {
    let packet = packet();
    let unknown = row(&packet, UNKNOWN_ROW);
    assert_eq!(
        unknown.recovery_action_class,
        GenerationRecoveryActionClass::RecoveryBlockedLineageUnknown
    );
    assert!(!unknown.admitted_for_recovery);
}

#[test]
fn bridge_row_discloses_known_issue_and_is_held() {
    let packet = packet();
    let bridge = row(&packet, BRIDGE_ROW);
    assert!(bridge.support_class.requires_disclosure());
    assert!(!bridge.known_issue_refs.is_empty());
    assert!(bridge
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::BridgeBehaviorDisclosed));
    assert!(!bridge.admitted_for_recovery);
}

#[test]
fn rows_empty_fails_validation() {
    let mut packet = packet();
    packet.rows.clear();
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::RowsEmpty));
}

#[test]
fn rollback_without_checkpoint_ref_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == CLEAN_ROW)
        .unwrap()
        .checkpoint_ref = None;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::CheckpointRefMissing));
}

#[test]
fn preview_ready_without_preview_ref_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == CLEAN_ROW)
        .unwrap()
        .diff_preview_ref = None;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::DiffPreviewRefMissing));
}

#[test]
fn destructive_recovery_without_authored_protection_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == MIXED_ROW)
        .unwrap()
        .authored_content_protected = false;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::AuthoredProtectionMissing));
}

#[test]
fn delete_generated_over_authored_only_zone_fails() {
    let mut packet = packet();
    let mixed = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == MIXED_ROW)
        .unwrap();
    mixed.managed_zone_class = ManagedZoneClass::AuthoredOnly;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::DeleteGeneratedScopeInvalid));
}

#[test]
fn bridge_row_without_disclosure_fails() {
    let mut packet = packet();
    let bridge = packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == BRIDGE_ROW)
        .unwrap();
    bridge.known_issue_refs.clear();
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::BridgeBehaviorUndisclosed));
}

#[test]
fn blocked_row_admitted_fails() {
    let mut packet = packet();
    packet
        .rows
        .iter_mut()
        .find(|row| row.row_id == UNKNOWN_ROW)
        .unwrap()
        .admitted_for_recovery = true;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::BlockedRecoveryAdmitted));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = packet();
    packet.rows[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = packet();
    packet.rows[0].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::MissingSourceContracts));
}

#[test]
fn review_incomplete_fails() {
    let mut packet = packet();
    packet.review.authored_content_never_deleted_by_recovery = false;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::ReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet.consumer_projection.blocked_rows_labeled_not_hidden = false;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&GenerationRecoveryViolation::ProofFreshnessIncomplete));
}

#[test]
fn unknown_lineage_blocks_a_row() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        diff_preview_available: true,
        checkpoint_available: true,
        lineage_known: false,
        authored_protection_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert_eq!(
        clean.recovery_action_class,
        GenerationRecoveryActionClass::RecoveryBlockedLineageUnknown
    );
    assert_eq!(
        clean.managed_zone_class,
        ManagedZoneClass::ZoneUnknownReviewRequired
    );
    assert!(!clean.admitted_for_recovery);
    assert!(clean
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::LineageUnknown));
    // A blocked-but-labeled row keeps the export valid.
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn unverified_authored_protection_quarantines_destructive_recovery() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
        row_id: MIXED_ROW.to_owned(),
        diff_preview_available: true,
        checkpoint_available: true,
        lineage_known: true,
        authored_protection_verified: false,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let mixed = row(&packet, MIXED_ROW);
    assert_eq!(
        mixed.recovery_action_class,
        GenerationRecoveryActionClass::QuarantineGenerated
    );
    assert!(!mixed.admitted_for_recovery);
    assert!(mixed
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::AuthoredProtectionUnverified));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_diff_preview_forbids_overwrite() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
        row_id: MIXED_ROW.to_owned(),
        diff_preview_available: false,
        checkpoint_available: true,
        lineage_known: true,
        authored_protection_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let mixed = row(&packet, MIXED_ROW);
    assert_eq!(
        mixed.diff_review_class,
        GenerationDiffReviewClass::DiffUnavailableReviewRequired
    );
    assert_eq!(
        mixed.overwrite_guard_class,
        GenerationOverwriteGuardClass::OverwriteForbiddenWithoutPreview
    );
    assert!(!mixed.admitted_for_recovery);
    assert!(mixed
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::DiffPreviewUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_checkpoint_withdraws_rollback() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        diff_preview_available: true,
        checkpoint_available: false,
        lineage_known: true,
        authored_protection_verified: true,
        proof_fresh: true,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert_eq!(
        clean.recovery_action_class,
        GenerationRecoveryActionClass::NoRecoveryAvailable
    );
    assert!(!clean.admitted_for_recovery);
    assert!(clean
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::CheckpointUnavailable));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn stale_proof_withholds_admission() {
    let mut packet = packet();
    packet.apply_downgrade_automation(&[GenerationRecoveryRowObservation {
        row_id: CLEAN_ROW.to_owned(),
        diff_preview_available: true,
        checkpoint_available: true,
        lineage_known: true,
        authored_protection_verified: true,
        proof_fresh: false,
        upstream_narrowed: false,
    }]);
    let clean = row(&packet, CLEAN_ROW);
    assert!(!clean.admitted_for_recovery);
    assert!(clean
        .downgrade_triggers
        .contains(&GenerationRecoveryDowngradeTrigger::ProofStale));
}

#[test]
fn markdown_summary_lists_every_row() {
    let summary = packet().render_markdown_summary();
    for row in &packet().rows {
        assert!(
            summary.contains(&row.template_id),
            "summary missing template {}",
            row.template_id
        );
    }
    assert!(summary.contains("rollback_to_checkpoint"));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_generation_recovery_export().expect("checked generation-recovery export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked =
        current_generation_recovery_export().expect("checked generation-recovery export validates");
    assert_eq!(checked, packet());
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty/lineage_unknown_blocked.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/templates/m5/add_generation_diff_review_rollback_or_delete_generated_recovery_and_managed_zone_honesty/authored_protection_quarantined.json"
        )),
    ] {
        let packet: GenerationRecoveryPacket =
            serde_json::from_str(raw).expect("fixture parses as generation-recovery packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}
