use super::*;

const PACKET_ID: &str = BROWSER_HANDOFF_DELEGATED_CREDENTIAL_PACKET_ID;

fn packet() -> BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket {
    seeded_browser_handoff_delegated_credential_controls()
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
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        BROWSER_HANDOFF_DELEGATED_CREDENTIAL_SCHEMA_VERSION
    );
}

#[test]
fn handoff_boundary_is_derived_not_asserted() {
    use HandoffBoundaryClass as Boundary;
    use M5AuthHandoffClass as Handoff;

    // System-browser redirect → out-of-app system-browser boundary.
    let d = resolve_handoff_boundary(Handoff::SystemBrowserRedirect);
    assert_eq!(d.handoff_boundary_class, Boundary::SystemBrowserBoundary);
    assert!(d.is_out_of_app_system_browser);
    assert!(d.needs_system_browser_note);

    // Device-code poll → device-code boundary, names its code / expiry.
    let d = resolve_handoff_boundary(Handoff::DeviceCodePoll);
    assert_eq!(d.handoff_boundary_class, Boundary::DeviceCodeBoundary);
    assert!(!d.is_out_of_app_system_browser);
    assert!(d.needs_device_code_note);

    // Embedded / passkey → local capture, a safer boundary is preferred.
    for handoff in [Handoff::EmbeddedPrompt, Handoff::PasskeyStepUp] {
        let d = resolve_handoff_boundary(handoff);
        assert_eq!(d.handoff_boundary_class, Boundary::LocalCaptureBoundary);
        assert!(d.is_local_capture);
        assert!(d.needs_local_capture_disclosure_note);
    }

    // Delegated forward / offline deferred → delegated-or-deferred boundary.
    for handoff in [Handoff::DelegatedForward, Handoff::OfflineDeferred] {
        let d = resolve_handoff_boundary(handoff);
        assert_eq!(
            d.handoff_boundary_class,
            Boundary::DelegatedOrDeferredBoundary
        );
        assert!(d.needs_delegated_deferred_note);
    }
}

#[test]
fn delegated_identity_origin_is_derived_not_asserted() {
    use DelegatedIdentityOrigin as Origin;
    use M5CredentialStorageMode as Storage;
    use M5DelegatedIdentityState as Identity;

    // Local identity + local storage → locally stored.
    let d = resolve_delegated_identity_origin(Identity::LocalIdentity, Storage::OsKeychain);
    assert_eq!(d.identity_origin, Origin::LocallyStored);
    assert!(d.is_locally_stored);
    assert!(!d.is_forwarded_or_delegated);

    // Forwarded / delegated / impersonation with local storage → forwarded.
    for state in [
        Identity::ForwardedIdentity,
        Identity::DelegatedOnBehalf,
        Identity::ImpersonationScoped,
    ] {
        let d = resolve_delegated_identity_origin(state, Storage::OsKeychain);
        assert_eq!(d.identity_origin, Origin::Forwarded);
        assert!(!d.is_locally_stored);
        assert!(d.needs_forwarded_note);
    }

    // Broker handle / external reference → remote vault (precedence over forwarded state).
    for storage in [Storage::SecretBrokerHandle, Storage::ExternalReference] {
        let d = resolve_delegated_identity_origin(Identity::DelegatedOnBehalf, storage);
        assert_eq!(d.identity_origin, Origin::RemoteVault);
        assert!(d.needs_remote_vault_note);
    }

    // Service account → service-issued, regardless of storage.
    let d = resolve_delegated_identity_origin(
        Identity::ServiceAccountActing,
        Storage::SessionMemoryOnly,
    );
    assert_eq!(d.identity_origin, Origin::ServiceIssued);
    assert!(d.needs_service_issued_note);

    // Revoked delegation → forwarded origin, names its revoked state.
    let d = resolve_delegated_identity_origin(Identity::DelegationRevoked, Storage::EncryptedVault);
    assert_eq!(d.identity_origin, Origin::Forwarded);
    assert!(d.needs_revoked_note);
}

#[test]
fn auth_handoff_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .handoff_cards
        .iter()
        .map(|card| card.auth_handoff_class)
        .collect();
    for class in M5AuthHandoffClass::ALL {
        assert!(covered.contains(&class), "missing handoff class {class:?}");
    }
}

#[test]
fn handoff_boundary_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .handoff_cards
        .iter()
        .map(|card| card.handoff_boundary_disclosure().handoff_boundary_class)
        .collect();
    for class in HandoffBoundaryClass::ALL {
        assert!(covered.contains(&class), "missing boundary class {class:?}");
    }
}

#[test]
fn delegated_identity_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .delegated_rows
        .iter()
        .map(|row| row.delegated_identity_state)
        .collect();
    for state in M5DelegatedIdentityState::ALL {
        assert!(covered.contains(&state), "missing identity state {state:?}");
    }
}

#[test]
fn delegated_identity_origin_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .delegated_rows
        .iter()
        .map(|row| row.identity_disclosure().identity_origin)
        .collect();
    for origin in DelegatedIdentityOrigin::ALL {
        assert!(covered.contains(&origin), "missing origin {origin:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::MissingSourceContracts));
}

#[test]
fn empty_handoff_cards_fails() {
    let mut packet = packet();
    packet.handoff_cards.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffCardsMissing));
}

#[test]
fn empty_delegated_rows_fails() {
    let mut packet = packet();
    packet.delegated_rows.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DelegatedRowsMissing));
}

#[test]
fn card_wrong_component_class_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].component = M5CredentialComponentFamily::DelegatedCredentialRow;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffCardWrongComponentClass));
}

#[test]
fn local_capture_card_claiming_out_of_app_fails() {
    let mut packet = packet();
    let card = packet
        .handoff_cards
        .iter_mut()
        .find(|card| card.handoff_boundary_class == HandoffBoundaryClass::LocalCaptureBoundary)
        .expect("local-capture card present");
    card.claims_out_of_app_boundary = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffBoundaryMisrepresented));
}

#[test]
fn misdeclared_handoff_boundary_class_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].handoff_boundary_class = HandoffBoundaryClass::LocalCaptureBoundary;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffBoundaryMisrepresented));
}

#[test]
fn missing_device_code_note_fails() {
    let mut packet = packet();
    let card = packet
        .handoff_cards
        .iter_mut()
        .find(|card| card.handoff_boundary_class == HandoffBoundaryClass::DeviceCodeBoundary)
        .expect("device-code card present");
    card.device_code_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DeviceCodeNoteMissing));
}

#[test]
fn missing_local_capture_disclosure_fails() {
    let mut packet = packet();
    let card = packet
        .handoff_cards
        .iter_mut()
        .find(|card| card.handoff_boundary_class == HandoffBoundaryClass::LocalCaptureBoundary)
        .expect("local-capture card present");
    card.local_capture_disclosure_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::LocalCaptureDisclosureMissing));
}

#[test]
fn missing_safer_boundary_rationale_fails() {
    let mut packet = packet();
    packet.handoff_cards[0]
        .safer_boundary_rationale_note
        .clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::SaferBoundaryRationaleMissing));
}

#[test]
fn missing_fallback_state_note_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].fallback_state_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::FallbackStateNoteMissing));
}

#[test]
fn missing_local_continuity_note_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].local_continuity_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::LocalContinuityNoteMissing));
}

#[test]
fn missing_continue_cancel_action_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].default_actions =
        vec![BrowserDeviceCodeHandoffAction::SwitchToDeviceCode];
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffActionsIncomplete));
}

#[test]
fn missing_provider_org_or_flow_kind_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].provider_org_label.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ProviderOrgOrFlowKindMissing));
}

#[test]
fn handoff_blurred_into_generic_sign_in_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].blurs_handoff_into_generic_sign_in = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::HandoffBlurredIntoGenericSignIn));
}

#[test]
fn card_masking_storage_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].masks_storage_or_reveal_posture = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::StorageOrRevealMasked));
}

#[test]
fn row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].component = M5CredentialComponentFamily::BrowserDeviceCodeHandoffCard;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DelegatedRowWrongComponentClass));
}

#[test]
fn forwarded_row_claiming_local_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.identity_origin == DelegatedIdentityOrigin::Forwarded)
        .expect("forwarded row present");
    row.claims_locally_stored = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DelegatedIdentityMisrepresented));
}

#[test]
fn misdeclared_identity_origin_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.identity_origin == DelegatedIdentityOrigin::Forwarded)
        .expect("forwarded row present");
    row.identity_origin = DelegatedIdentityOrigin::LocallyStored;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DelegatedIdentityMisrepresented));
}

#[test]
fn missing_forwarded_note_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.identity_origin == DelegatedIdentityOrigin::Forwarded)
        .expect("forwarded row present");
    row.forwarded_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ForwardedNoteMissing));
}

#[test]
fn missing_remote_vault_note_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.identity_origin == DelegatedIdentityOrigin::RemoteVault)
        .expect("remote-vault row present");
    row.remote_vault_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::RemoteVaultNoteMissing));
}

#[test]
fn missing_service_issued_note_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.identity_origin == DelegatedIdentityOrigin::ServiceIssued)
        .expect("service-issued row present");
    row.service_issued_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ServiceIssuedNoteMissing));
}

#[test]
fn missing_revoked_note_fails() {
    let mut packet = packet();
    let row = packet
        .delegated_rows
        .iter_mut()
        .find(|row| row.delegated_identity_state == M5DelegatedIdentityState::DelegationRevoked)
        .expect("revoked row present");
    row.revoked_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::RevokedNoteMissing));
}

#[test]
fn missing_expiration_note_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].expiration_note.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ExpirationNoteMissing));
}

#[test]
fn missing_policy_owner_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].policy_owner_label.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::PolicyOwnerMissing));
}

#[test]
fn missing_source_identity_or_scope_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].source_identity_label.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::SourceIdentityOrScopeMissing));
}

#[test]
fn missing_stop_forward_rotate_action_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].default_actions = vec![DelegatedCredentialRowAction::ExportRow];
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DelegatedActionsIncomplete));
}

#[test]
fn forwarded_identity_masked_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].masks_forwarded_or_delegated_identity = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ForwardedOrDelegatedIdentityMasked));
}

#[test]
fn row_normalizing_raw_secret_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].implies_raw_secret_exportable = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::RawSecretHandlingNormalized));
}

#[test]
fn friendly_connected_wording_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].uses_friendly_connected_wording = true;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::FriendlyConnectedWordingUsed));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].required_labels = vec![M5CredentialRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_states_fails() {
    let mut packet = packet();
    packet.delegated_rows[0].degraded_states.clear();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::DegradedStatesMissing));
}

#[test]
fn missing_accessibility_route_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].accessibility_routes =
        vec![M5CredentialAccessibilityRoute::ScreenReaderAnnounced];
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::AccessibilityRouteMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.forwarded_delegated_never_reads_as_local = false;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .safer_boundary_visible_before_local_capture = false;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.handoff_cards[0].flow_kind_label = "see internal://creds".to_owned();
    assert!(packet
        .validate()
        .contains(&BrowserHandoffDelegatedCredentialViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Browser-or-device-code handoff cards"));
    assert!(summary.contains("## Delegated-credential rows"));
    assert!(summary.contains("local_capture_boundary"));
    assert!(summary.contains("forwarded"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 handoff cards + 6 delegated rows
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("browser_device_code_handoff_card"));
    assert!(csv.contains("delegated_credential_row"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_browser_handoff_delegated_credential_export()
        .expect("checked browser handoff delegated credential export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-browser-device-code-handoff-delegated-credential-controls/handoff_local_capture.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-browser-device-code-handoff-delegated-credential-controls/delegated_forwarded_identity.json"
        )),
    ] {
        let packet: BrowserDeviceCodeHandoffDelegatedCredentialControlsPacket =
            serde_json::from_str(raw)
                .expect("fixture parses as browser handoff delegated credential packet");
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
        seeded_browser_handoff_delegated_credential_controls_handoff_local_capture(),
        seeded_browser_handoff_delegated_credential_controls_delegated_forwarded_identity(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
