use super::*;

const PACKET_ID: &str = ROTATION_REVOKE_EXPORT_SAFETY_PACKET_ID;

fn packet() -> RotationRevokeExportSafetyControlsPacket {
    seeded_rotation_revoke_export_safety_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(
        packet.record_kind,
        ROTATION_REVOKE_EXPORT_SAFETY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        ROTATION_REVOKE_EXPORT_SAFETY_SCHEMA_VERSION
    );
}

#[test]
fn continuity_is_derived_not_asserted() {
    use CredentialContinuityClass as Continuity;
    use M5CredentialLifecycleState as Lifecycle;

    // Active-current → still active, usable.
    let d = resolve_credential_continuity(Lifecycle::ActiveCurrent);
    assert_eq!(d.continuity_class, Continuity::StillActive);
    assert!(d.is_still_usable);
    assert!(d.needs_still_active_note);

    // Refresh-needed / rotation-due → action required, still usable.
    for state in [Lifecycle::RefreshNeeded, Lifecycle::RotationDue] {
        let d = resolve_credential_continuity(state);
        assert_eq!(d.continuity_class, Continuity::ActionRequired);
        assert!(d.is_still_usable);
        assert!(d.needs_action_required_note);
    }

    // Revoked / expired → no longer usable.
    for state in [Lifecycle::Revoked, Lifecycle::Expired] {
        let d = resolve_credential_continuity(state);
        assert_eq!(d.continuity_class, Continuity::NoLongerUsable);
        assert!(!d.is_still_usable);
        assert!(d.needs_no_longer_usable_note);
    }

    // Superseded → superseded, not usable.
    let d = resolve_credential_continuity(Lifecycle::Superseded);
    assert_eq!(d.continuity_class, Continuity::Superseded);
    assert!(!d.is_still_usable);
    assert!(d.needs_superseded_note);
}

#[test]
fn export_safety_posture_is_derived_not_asserted() {
    use ExportSafetyPosture as Posture;
    use M5CredentialExportSafetyClass as Safety;

    // Raw-secret-excluded / metadata-only → raw excluded, labels preserved.
    for class in [Safety::RawSecretExcluded, Safety::MetadataOnly] {
        let d = resolve_export_safety_posture(class);
        assert_eq!(d.export_safety_posture, Posture::RawExcludedLabelsPreserved);
        assert!(d.preserves_handle_class_labels);
        assert!(d.needs_handle_label_note);
    }

    // Handle-reference-only → handle reference only, labels preserved.
    let d = resolve_export_safety_posture(Safety::HandleReferenceOnly);
    assert_eq!(d.export_safety_posture, Posture::HandleReferenceOnly);
    assert!(d.preserves_handle_class_labels);

    // Redacted-share / endpoints-masked → redacted or masked.
    for class in [Safety::RedactedShare, Safety::EndpointsMasked] {
        let d = resolve_export_safety_posture(class);
        assert_eq!(d.export_safety_posture, Posture::RedactedOrMasked);
        assert!(d.needs_redaction_note);
    }

    // Export-blocked → fully blocked, no labels preserved.
    let d = resolve_export_safety_posture(Safety::ExportBlocked);
    assert_eq!(d.export_safety_posture, Posture::FullyBlocked);
    assert!(!d.preserves_handle_class_labels);
    assert!(d.needs_blocked_note);
}

#[test]
fn lifecycle_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> =
        packet.event_rows.iter().map(|row| row.new_state).collect();
    for state in M5CredentialLifecycleState::ALL {
        assert!(
            covered.contains(&state),
            "missing lifecycle state {state:?}"
        );
    }
}

#[test]
fn continuity_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .event_rows
        .iter()
        .map(|row| row.continuity_disclosure().continuity_class)
        .collect();
    for class in CredentialContinuityClass::ALL {
        assert!(
            covered.contains(&class),
            "missing continuity class {class:?}"
        );
    }
}

#[test]
fn impacted_workflow_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .event_rows
        .iter()
        .flat_map(|row| row.impacted_workflows.iter().copied())
        .collect();
    for class in ImpactedWorkflowClass::ALL {
        assert!(covered.contains(&class), "missing impacted class {class:?}");
    }
}

#[test]
fn export_safety_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .export_banners
        .iter()
        .map(|banner| banner.export_safety_class)
        .collect();
    for class in M5CredentialExportSafetyClass::ALL {
        assert!(covered.contains(&class), "missing safety class {class:?}");
    }
}

#[test]
fn export_surface_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .export_banners
        .iter()
        .map(|banner| banner.export_surface_class)
        .collect();
    for surface in ExportSurfaceClass::ALL {
        assert!(covered.contains(&surface), "missing surface {surface:?}");
    }
}

#[test]
fn export_safety_posture_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .export_banners
        .iter()
        .map(|banner| banner.export_safety_disclosure().export_safety_posture)
        .collect();
    for posture in ExportSafetyPosture::ALL {
        assert!(covered.contains(&posture), "missing posture {posture:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::MissingSourceContracts));
}

#[test]
fn empty_event_rows_fails() {
    let mut packet = packet();
    packet.event_rows.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::EventRowsMissing));
}

#[test]
fn empty_export_banners_fails() {
    let mut packet = packet();
    packet.export_banners.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExportBannersMissing));
}

#[test]
fn event_row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.event_rows[0].component = M5CredentialComponentFamily::ExportSafetyBanner;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::EventRowWrongComponentClass));
}

#[test]
fn revoked_row_claiming_still_usable_fails() {
    let mut packet = packet();
    let row = packet
        .event_rows
        .iter_mut()
        .find(|row| row.continuity_class == CredentialContinuityClass::NoLongerUsable)
        .expect("no-longer-usable row present");
    row.claims_still_usable = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ContinuityMisrepresented));
}

#[test]
fn misdeclared_continuity_class_fails() {
    let mut packet = packet();
    packet.event_rows[0].continuity_class = CredentialContinuityClass::NoLongerUsable;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ContinuityMisrepresented));
}

#[test]
fn missing_no_longer_usable_note_fails() {
    let mut packet = packet();
    let row = packet
        .event_rows
        .iter_mut()
        .find(|row| row.continuity_class == CredentialContinuityClass::NoLongerUsable)
        .expect("no-longer-usable row present");
    row.no_longer_usable_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::NoLongerUsableNoteMissing));
}

#[test]
fn missing_superseded_note_fails() {
    let mut packet = packet();
    let row = packet
        .event_rows
        .iter_mut()
        .find(|row| row.continuity_class == CredentialContinuityClass::Superseded)
        .expect("superseded row present");
    row.superseded_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::SupersededNoteMissing));
}

#[test]
fn missing_impacted_workflows_fails() {
    let mut packet = packet();
    packet.event_rows[0].impacted_workflows.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ImpactedWorkflowsMissing));
}

#[test]
fn missing_impacted_workflows_note_fails() {
    let mut packet = packet();
    packet.event_rows[0].impacted_workflows_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ImpactedWorkflowsNoteMissing));
}

#[test]
fn missing_recovery_next_step_note_fails() {
    let mut packet = packet();
    packet.event_rows[0].recovery_next_step_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RecoveryNextStepNoteMissing));
}

#[test]
fn missing_audit_note_fails() {
    let mut packet = packet();
    packet.event_rows[0].audit_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::AuditNoteMissing));
}

#[test]
fn missing_prior_or_new_state_note_fails() {
    let mut packet = packet();
    packet.event_rows[0].prior_state_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::PriorOrNewStateNoteMissing));
}

#[test]
fn missing_credential_identity_fails() {
    let mut packet = packet();
    packet.event_rows[0].credential_id_label.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::CredentialIdentityMissing));
}

#[test]
fn missing_recovery_export_action_fails() {
    let mut packet = packet();
    packet.event_rows[0].default_actions = vec![RotationRevokeEventRowAction::RotateNow];
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::EventActionsIncomplete));
}

#[test]
fn impacted_workflows_masked_fails() {
    let mut packet = packet();
    packet.event_rows[0].masks_impacted_workflows = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ImpactedWorkflowsMasked));
}

#[test]
fn banner_wrong_component_class_fails() {
    let mut packet = packet();
    packet.export_banners[0].component = M5CredentialComponentFamily::RotationRevokeEventRow;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExportBannerWrongComponentClass));
}

#[test]
fn misdeclared_export_posture_fails() {
    let mut packet = packet();
    packet.export_banners[0].export_safety_posture = ExportSafetyPosture::FullyBlocked;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExportSafetyMisrepresented));
}

#[test]
fn banner_misclaiming_preserved_labels_fails() {
    let mut packet = packet();
    let banner = packet
        .export_banners
        .iter_mut()
        .find(|banner| banner.export_safety_posture == ExportSafetyPosture::FullyBlocked)
        .expect("fully-blocked banner present");
    banner.claims_preserves_handle_labels = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExportSafetyMisrepresented));
}

#[test]
fn missing_raw_secret_excluded_note_fails() {
    let mut packet = packet();
    packet.export_banners[0].raw_secret_excluded_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RawSecretExcludedNoteMissing));
}

#[test]
fn missing_redaction_note_fails() {
    let mut packet = packet();
    let banner = packet
        .export_banners
        .iter_mut()
        .find(|banner| banner.export_safety_posture == ExportSafetyPosture::RedactedOrMasked)
        .expect("redacted banner present");
    banner.redaction_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RedactionNoteMissing));
}

#[test]
fn missing_blocked_note_fails() {
    let mut packet = packet();
    let banner = packet
        .export_banners
        .iter_mut()
        .find(|banner| banner.export_safety_posture == ExportSafetyPosture::FullyBlocked)
        .expect("fully-blocked banner present");
    banner.blocked_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::BlockedNoteMissing));
}

#[test]
fn missing_export_surface_note_fails() {
    let mut packet = packet();
    packet.export_banners[0].export_surface_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExportSurfaceNoteMissing));
}

#[test]
fn missing_reveal_posture_note_fails() {
    let mut packet = packet();
    packet.export_banners[0].reveal_posture_note.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RevealPostureNoteMissing));
}

#[test]
fn missing_policy_excluded_action_fails() {
    let mut packet = packet();
    packet.export_banners[0].default_actions =
        vec![ExportSafetyBannerAction::ReportUnexpectedExposure];
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::BannerActionsIncomplete));
}

#[test]
fn exclusion_left_to_implication_fails() {
    let mut packet = packet();
    packet.export_banners[0].leaves_exclusion_to_implication = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ExclusionLeftToImplication));
}

#[test]
fn banner_normalizing_raw_secret_fails() {
    let mut packet = packet();
    packet.export_banners[0].implies_raw_secret_exportable = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RawSecretHandlingNormalized));
}

#[test]
fn friendly_connected_wording_fails() {
    let mut packet = packet();
    packet.event_rows[0].uses_friendly_connected_wording = true;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::FriendlyConnectedWordingUsed));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.event_rows[0].required_labels = vec![M5CredentialRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_states_fails() {
    let mut packet = packet();
    packet.export_banners[0].degraded_states.clear();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::DegradedStatesMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.event_rows[0].accessibility_routes =
        vec![M5CredentialAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::AccessibilityRouteMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .revoked_expired_never_reads_as_still_usable = false;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .banner_shows_exclusion_posture_inline = false;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.event_rows[0].credential_id_label = "see internal://creds".to_owned();
    assert!(packet
        .validate()
        .contains(&RotationRevokeExportSafetyViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Rotation/revoke-event rows"));
    assert!(summary.contains("## Export-safety banners"));
    assert!(summary.contains("no_longer_usable"));
    assert!(summary.contains("raw secrets excluded"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 event rows + 6 export banners
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("rotation_revoke_event_row"));
    assert!(csv.contains("export_safety_banner"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_rotation_revoke_export_safety_export()
        .expect("checked rotation revoke export safety export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-rotation-revoke-export-safety-controls/revoke_event_impacted_workflows.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-rotation-revoke-export-safety-controls/export_banner_raw_excluded.json"
        )),
    ] {
        let packet: RotationRevokeExportSafetyControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as rotation revoke export safety packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_rotation_revoke_export_safety_controls_revoke_event_impacted_workflows(),
        seeded_rotation_revoke_export_safety_controls_export_banner_raw_excluded(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
