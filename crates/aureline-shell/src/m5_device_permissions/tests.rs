use super::*;

#[test]
fn seeded_set_validates() {
    let set = seeded_m5_device_permission_set();
    assert!(set.validate().is_ok(), "{:?}", set.validate());
    assert_eq!(set.set_id, M5_DEVICE_PERMISSION_SET_ID);
}

#[test]
fn every_device_class_named_once() {
    let set = seeded_m5_device_permission_set();
    assert_eq!(set.permission_rows.len(), DeviceClass::ALL.len());
    for device in DeviceClass::ALL {
        let count = set
            .permission_rows
            .iter()
            .filter(|r| r.device_class == device)
            .count();
        assert_eq!(count, 1, "device {} not named once", device.as_str());
    }
}

#[test]
fn every_pill_state_present() {
    let set = seeded_m5_device_permission_set();
    for state in MicPillState::ALL {
        assert!(
            set.mic_pills.iter().any(|p| p.pill_state == state),
            "pill state {} missing",
            state.as_str()
        );
    }
}

#[test]
fn every_capture_class_covered() {
    let set = seeded_m5_device_permission_set();
    for class in CaptureClass::ALL {
        let covered = set
            .capture_reviews
            .iter()
            .any(|r| r.included_capture_classes.contains(&class));
        assert!(covered, "capture class {} uncovered", class.as_str());
    }
}

#[test]
fn capture_active_only_when_in_use() {
    let set = seeded_m5_device_permission_set();
    for row in &set.permission_rows {
        assert_eq!(
            row.capture_active,
            row.permission_state == PermissionState::GrantedInUse,
            "row {} capture_active does not match in-use state",
            row.row_id
        );
    }
    // Guardrail: capture is not always-on.
    assert!(set.permission_rows.iter().any(|r| !r.capture_active));
}

#[test]
fn local_processing_never_claimed_with_provider() {
    let set = seeded_m5_device_permission_set();
    for row in &set.permission_rows {
        let provider_in_path = row.controlling_actor.is_provider_backed()
            || retention_is_provider_backed(row.retention_mode);
        if provider_in_path {
            assert_ne!(
                row.processing_locality,
                ProcessingLocalityCue::LocalOnDevice,
                "row {} claims local processing with a provider in path",
                row.row_id
            );
        }
    }
}

#[test]
fn both_local_and_provider_processing_represented() {
    let set = seeded_m5_device_permission_set();
    assert!(set
        .permission_rows
        .iter()
        .any(|r| r.processing_locality == ProcessingLocalityCue::LocalOnDevice));
    assert!(set
        .permission_rows
        .iter()
        .any(|r| r.processing_locality == ProcessingLocalityCue::HostedRemoteDisclosed));
}

#[test]
fn high_impact_pill_gates_confirmation_and_correction() {
    let set = seeded_m5_device_permission_set();
    let mut saw_high_impact = false;
    for pill in &set.mic_pills {
        if pill.command_capability_scope.is_high_impact() {
            saw_high_impact = true;
            assert_eq!(pill.pill_state, MicPillState::NeedsConfirmation);
            assert_eq!(
                pill.correction_posture,
                TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit
            );
            assert!(pill.preview_required_before_commit);
        }
    }
    assert!(saw_high_impact, "no high-impact pill present");
}

#[test]
fn granted_rows_offer_revoke_and_system_settings() {
    let set = seeded_m5_device_permission_set();
    for row in &set.permission_rows {
        assert!(row
            .available_actions
            .contains(&PermissionActionClass::OpenSystemSettings));
        if row.permission_state.is_granted() {
            assert!(row
                .available_actions
                .contains(&PermissionActionClass::RevokeInApp));
        }
    }
}

#[test]
fn reviews_that_export_are_redacted() {
    let set = seeded_m5_device_permission_set();
    for review in &set.capture_reviews {
        if review.export_available {
            assert!(review.redaction_state.allows_export());
            assert!(review
                .available_actions
                .contains(&CaptureReviewActionClass::ExportRedactedCopy));
        }
    }
}

// --- negative cases -------------------------------------------------------

#[test]
fn capture_active_without_grant_fails() {
    let mut set = seeded_m5_device_permission_set();
    let row = set
        .permission_rows
        .iter_mut()
        .find(|r| r.permission_state == PermissionState::GrantedIdle)
        .expect("granted-idle row present");
    row.capture_active = true;
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::CaptureActiveWithoutGrant { .. })
    ));
}

#[test]
fn local_processing_with_provider_fails() {
    let mut set = seeded_m5_device_permission_set();
    let row = set
        .permission_rows
        .iter_mut()
        .find(|r| r.controlling_actor == PermissionActor::ConnectedProvider)
        .expect("provider row present");
    row.processing_locality = ProcessingLocalityCue::LocalOnDevice;
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::LocalProcessingClaimedWithProvider { .. })
    ));
}

#[test]
fn granted_row_without_revoke_fails() {
    let mut set = seeded_m5_device_permission_set();
    let row = set
        .permission_rows
        .iter_mut()
        .find(|r| r.permission_state.is_granted())
        .expect("granted row present");
    row.available_actions
        .retain(|a| *a != PermissionActionClass::RevokeInApp);
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::MissingRevokeAction { .. })
    ));
}

#[test]
fn missing_system_settings_action_fails() {
    let mut set = seeded_m5_device_permission_set();
    set.permission_rows[0]
        .available_actions
        .retain(|a| *a != PermissionActionClass::OpenSystemSettings);
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::MissingSystemSettingsAction { .. })
    ));
}

#[test]
fn high_impact_without_confirmation_fails() {
    let mut set = seeded_m5_device_permission_set();
    let pill = set
        .mic_pills
        .iter_mut()
        .find(|p| p.command_capability_scope.is_high_impact())
        .expect("high-impact pill present");
    pill.pill_state = MicPillState::Listening;
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::HighImpactWithoutConfirmation { .. })
    ));
}

#[test]
fn capturing_without_indicator_fails() {
    let mut set = seeded_m5_device_permission_set();
    let pill = set
        .mic_pills
        .iter_mut()
        .find(|p| p.pill_state == MicPillState::Listening)
        .expect("listening pill present");
    pill.indicator_visible = false;
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::CapturingWithoutIndicator { .. })
    ));
}

#[test]
fn policy_blocked_wrong_reason_fails() {
    let mut set = seeded_m5_device_permission_set();
    let pill = set
        .mic_pills
        .iter_mut()
        .find(|p| p.pill_state == MicPillState::PolicyBlocked)
        .expect("policy-blocked pill present");
    pill.unavailable_reason = Some(VoiceUnavailableReason::NoMicrophone);
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::PolicyBlockedWrongReason { .. })
    ));
}

#[test]
fn export_without_redaction_fails() {
    let mut set = seeded_m5_device_permission_set();
    let review = set
        .capture_reviews
        .iter_mut()
        .find(|r| r.export_available)
        .expect("exporting review present");
    review.redaction_state = CaptureRedactionState::RawNeverExported;
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::ExportWithoutRedaction { .. })
            | Err(DevicePermissionError::RedactionDataExitMismatch { .. })
    ));
}

#[test]
fn always_on_capture_fails() {
    let mut set = seeded_m5_device_permission_set();
    for row in &mut set.permission_rows {
        row.permission_state = PermissionState::GrantedInUse;
        row.capture_active = true;
    }
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::CaptureAlwaysOn)
            | Err(DevicePermissionError::LocalProcessingClaimedWithProvider { .. })
            | Err(DevicePermissionError::ProviderProcessingUnrepresented)
    ));
}

#[test]
fn missing_source_contract_fails() {
    let mut set = seeded_m5_device_permission_set();
    set.source_contract_refs
        .retain(|r| r != M5_MIC_STATE_PILL_SCHEMA_REF);
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::MissingSourceContracts)
    ));
}

#[test]
fn raw_ref_in_minted_at_fails() {
    let mut set = seeded_m5_device_permission_set();
    set.minted_at = "https://example.com/mint".to_owned();
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::RawRefLeak { .. })
            | Err(DevicePermissionError::RawMaterialInExport)
    ));
}

#[test]
fn duplicate_record_id_fails() {
    let mut set = seeded_m5_device_permission_set();
    let dup = set.permission_rows[0].clone();
    set.permission_rows.push(dup);
    assert!(matches!(
        set.validate(),
        Err(DevicePermissionError::DuplicateRecordId { .. })
            | Err(DevicePermissionError::DeviceClassNotNamedOnce { .. })
    ));
}

// --- renderers ------------------------------------------------------------

#[test]
fn matrix_csv_has_a_row_per_record() {
    let set = seeded_m5_device_permission_set();
    let csv = set.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    let records = set.permission_rows.len() + set.mic_pills.len() + set.capture_reviews.len();
    assert_eq!(lines.len(), 1 + records);
    assert!(lines[0].starts_with("record_kind,identity,"));
}

#[test]
fn markdown_summary_lists_every_record() {
    let set = seeded_m5_device_permission_set();
    let summary = set.render_markdown_summary();
    for row in &set.permission_rows {
        assert!(
            summary.contains(&row.row_id),
            "summary missing {}",
            row.row_id
        );
    }
    for pill in &set.mic_pills {
        assert!(
            summary.contains(&pill.pill_id),
            "summary missing {}",
            pill.pill_id
        );
    }
    for review in &set.capture_reviews {
        assert!(
            summary.contains(&review.review_id),
            "summary missing {}",
            review.review_id
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_device_permission_set().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
}

// --- narrowed fixtures ----------------------------------------------------

#[test]
fn narrowed_fixtures_validate() {
    assert!(
        seeded_high_impact_confirmation_pill().validate().is_ok(),
        "{:?}",
        seeded_high_impact_confirmation_pill().validate()
    );
    assert!(
        seeded_provider_backed_capture_review().validate().is_ok(),
        "{:?}",
        seeded_provider_backed_capture_review().validate()
    );
}

#[test]
fn high_impact_fixture_is_gated() {
    let pill = seeded_high_impact_confirmation_pill();
    assert!(pill.command_capability_scope.is_high_impact());
    assert_eq!(pill.pill_state, MicPillState::NeedsConfirmation);
    assert!(pill.preview_required_before_commit);
    assert_eq!(
        pill.correction_posture,
        TranscriptCorrectionPosture::CorrectionRequiredBeforeCommit
    );
}

#[test]
fn provider_backed_fixture_is_hosted_not_local() {
    let review = seeded_provider_backed_capture_review();
    assert!(retention_is_provider_backed(review.retention_mode));
    assert_eq!(
        review.processing_locality,
        ProcessingLocalityCue::HostedRemoteDisclosed
    );
}

// --- on-disk round-trips --------------------------------------------------

#[test]
fn checked_support_export_matches_seed() {
    let from_disk =
        current_stable_m5_device_permission_set().expect("checked device permission set validates");
    assert_eq!(
        from_disk,
        seeded_m5_device_permission_set(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_match_seed_builders() {
    let pill: MicStatePill = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/device-permissions/high_impact_confirmation_pill.json"
    )))
    .expect("high-impact pill fixture parses");
    assert_eq!(pill, seeded_high_impact_confirmation_pill());

    let review: CaptureExportReview = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/help/device-permissions/provider_backed_capture_review.json"
    )))
    .expect("provider-backed review fixture parses");
    assert_eq!(review, seeded_provider_backed_capture_review());
}
