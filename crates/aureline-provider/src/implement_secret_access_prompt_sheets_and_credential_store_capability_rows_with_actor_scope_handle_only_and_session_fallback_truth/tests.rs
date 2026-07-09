use super::*;

const PACKET_ID: &str = SECRET_ACCESS_PROMPT_STORE_CAPABILITY_PACKET_ID;

fn packet() -> SecretAccessPromptStoreCapabilityControlsPacket {
    seeded_secret_access_prompt_store_capability_controls()
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
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SECRET_ACCESS_PROMPT_STORE_CAPABILITY_SCHEMA_VERSION
    );
}

#[test]
fn handle_availability_is_derived_not_asserted() {
    use HandleAvailabilityClass as Handle;
    use M5CredentialRevealPosture as Reveal;

    // Handle-only / never-revealed → a handle-only path exists.
    for posture in [Reveal::HandleOnly, Reveal::NeverRevealed] {
        let d = resolve_handle_availability(posture);
        assert_eq!(d.handle_availability_class, Handle::HandleOnlyAvailable);
        assert!(d.is_handle_only_available);
        assert!(!d.requests_raw_reveal);
        assert!(d.needs_handle_only_note);
    }

    // Masked / clipboard-scoped → scoped-only, still a handle path, never raw.
    for posture in [Reveal::MaskedLastFour, Reveal::ClipboardScoped] {
        let d = resolve_handle_availability(posture);
        assert_eq!(d.handle_availability_class, Handle::ScopedRevealOnly);
        assert!(!d.is_handle_only_available);
        assert!(d.needs_handle_only_note);
    }

    // Reveal-on-demand → raw reveal requested, must be explicit.
    let d = resolve_handle_availability(Reveal::RevealOnDemand);
    assert_eq!(d.handle_availability_class, Handle::RawRevealRequested);
    assert!(d.requests_raw_reveal);
    assert!(d.needs_raw_reveal_disclosure_note);

    // Policy-blocked → reveal blocked.
    let d = resolve_handle_availability(Reveal::PolicyBlockedReveal);
    assert_eq!(d.handle_availability_class, Handle::RevealPolicyBlocked);
    assert!(d.needs_reveal_blocked_note);
}

#[test]
fn store_trust_is_derived_not_asserted() {
    use CredentialStoreTrustClass as Trust;
    use M5CredentialStoreCapability as Capability;
    use StoreVerificationState as Verification;

    // Verified + persistent → securely stored.
    let d = resolve_store_trust(
        Verification::HardwareAttested,
        &[Capability::HardwareBacked, Capability::PersistAcrossRestart],
    );
    assert_eq!(d.trust_class, Trust::SecurelyStored);
    assert!(d.is_securely_stored);
    assert!(!d.is_unverified_or_unsupported);

    // Verified but session-only → limited assurance, names the fallback.
    let d = resolve_store_trust(Verification::EncryptedVerified, &[Capability::SessionOnly]);
    assert_eq!(d.trust_class, Trust::LimitedAssurance);
    assert!(!d.is_securely_stored);
    assert!(d.needs_session_only_fallback_note);

    // Unverified / verification-failed → unverified store, never securely stored.
    for state in [Verification::Unverified, Verification::VerificationFailed] {
        let d = resolve_store_trust(state, &[Capability::PersistAcrossRestart]);
        assert_eq!(d.trust_class, Trust::UnverifiedStore);
        assert!(!d.is_securely_stored);
        assert!(d.is_unverified_or_unsupported);
        assert!(d.needs_unverified_note);
    }

    // Unsupported → unsupported store, never securely stored.
    let d = resolve_store_trust(
        Verification::Unsupported,
        &[Capability::PersistAcrossRestart],
    );
    assert_eq!(d.trust_class, Trust::UnsupportedStore);
    assert!(!d.is_securely_stored);
    assert!(d.needs_unsupported_note);
}

#[test]
fn actor_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .secret_access_prompts
        .iter()
        .map(|prompt| prompt.actor)
        .collect();
    for actor in SecretRequestActor::ALL {
        assert!(covered.contains(&actor), "missing actor {actor:?}");
    }
}

#[test]
fn handle_availability_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .secret_access_prompts
        .iter()
        .map(|prompt| {
            prompt
                .handle_availability_disclosure()
                .handle_availability_class
        })
        .collect();
    for class in HandleAvailabilityClass::ALL {
        assert!(covered.contains(&class), "missing handle class {class:?}");
    }
}

#[test]
fn verification_state_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .store_capability_rows
        .iter()
        .map(|row| row.verification_state)
        .collect();
    for state in StoreVerificationState::ALL {
        assert!(covered.contains(&state), "missing verification {state:?}");
    }
}

#[test]
fn trust_class_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .store_capability_rows
        .iter()
        .map(|row| row.trust_disclosure().trust_class)
        .collect();
    for class in CredentialStoreTrustClass::ALL {
        assert!(covered.contains(&class), "missing trust class {class:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::MissingSourceContracts));
}

#[test]
fn empty_secret_access_prompts_fails() {
    let mut packet = packet();
    packet.secret_access_prompts.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::SecretAccessPromptsMissing));
}

#[test]
fn empty_store_capability_rows_fails() {
    let mut packet = packet();
    packet.store_capability_rows.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::StoreCapabilityRowsMissing));
}

#[test]
fn prompt_wrong_component_class_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].component =
        M5CredentialComponentFamily::CredentialStoreCapabilityRow;
    assert!(packet.validate().contains(
        &SecretAccessPromptStoreCapabilityViolation::SecretAccessPromptWrongComponentClass
    ));
}

#[test]
fn raw_reveal_prompt_claiming_handle_only_fails() {
    let mut packet = packet();
    let prompt = packet
        .secret_access_prompts
        .iter_mut()
        .find(|prompt| {
            prompt.handle_availability_class == HandleAvailabilityClass::RawRevealRequested
        })
        .expect("raw-reveal prompt present");
    prompt.claims_handle_only_path = true;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::HandleAvailabilityMisrepresented));
}

#[test]
fn misdeclared_handle_availability_class_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].handle_availability_class =
        HandleAvailabilityClass::RawRevealRequested;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::HandleAvailabilityMisrepresented));
}

#[test]
fn missing_raw_reveal_disclosure_fails() {
    let mut packet = packet();
    let prompt = packet
        .secret_access_prompts
        .iter_mut()
        .find(|prompt| {
            prompt.handle_availability_class == HandleAvailabilityClass::RawRevealRequested
        })
        .expect("raw-reveal prompt present");
    prompt.raw_reveal_disclosure_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::RawRevealDisclosureMissing));
}

#[test]
fn missing_handle_only_note_fails() {
    let mut packet = packet();
    let prompt = packet
        .secret_access_prompts
        .iter_mut()
        .find(|prompt| {
            prompt.handle_availability_class == HandleAvailabilityClass::HandleOnlyAvailable
        })
        .expect("handle-only prompt present");
    prompt.handle_only_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::HandleOnlyNoteMissing));
}

#[test]
fn missing_retention_note_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].retention_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::RetentionNoteMissing));
}

#[test]
fn missing_denied_fallback_note_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].denied_fallback_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::DeniedFallbackNoteMissing));
}

#[test]
fn missing_allow_deny_once_action_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].default_actions = vec![SecretAccessPromptAction::ReviewScope];
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::PromptActionsIncomplete));
}

#[test]
fn missing_purpose_or_scope_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].requested_scope_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::ActorPurposeOrScopeMissing));
}

#[test]
fn store_wrong_component_class_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].component =
        M5CredentialComponentFamily::SecretAccessPromptSheet;
    assert!(packet.validate().contains(
        &SecretAccessPromptStoreCapabilityViolation::StoreCapabilityRowWrongComponentClass
    ));
}

#[test]
fn unverified_store_claiming_secure_fails() {
    let mut packet = packet();
    let row = packet
        .store_capability_rows
        .iter_mut()
        .find(|row| row.trust_class == CredentialStoreTrustClass::UnverifiedStore)
        .expect("unverified store present");
    row.claims_securely_stored = true;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::TrustClassMisrepresented));
}

#[test]
fn misdeclared_trust_class_fails() {
    let mut packet = packet();
    let row = packet
        .store_capability_rows
        .iter_mut()
        .find(|row| row.trust_class == CredentialStoreTrustClass::UnverifiedStore)
        .expect("unverified store present");
    row.trust_class = CredentialStoreTrustClass::SecurelyStored;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::TrustClassMisrepresented));
}

#[test]
fn missing_unverified_note_fails() {
    let mut packet = packet();
    let row = packet
        .store_capability_rows
        .iter_mut()
        .find(|row| row.trust_class == CredentialStoreTrustClass::UnverifiedStore)
        .expect("unverified store present");
    row.unverified_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::UnverifiedNoteMissing));
}

#[test]
fn missing_unsupported_note_fails() {
    let mut packet = packet();
    let row = packet
        .store_capability_rows
        .iter_mut()
        .find(|row| row.trust_class == CredentialStoreTrustClass::UnsupportedStore)
        .expect("unsupported store present");
    row.unsupported_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::UnsupportedNoteMissing));
}

#[test]
fn missing_session_only_fallback_note_fails() {
    let mut packet = packet();
    let row = packet
        .store_capability_rows
        .iter_mut()
        .find(|row| row.trust_class == CredentialStoreTrustClass::LimitedAssurance)
        .expect("limited-assurance store present");
    row.session_only_fallback_note.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::SessionOnlyFallbackNoteMissing));
}

#[test]
fn missing_platform_limitations_note_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0]
        .platform_limitations_note
        .clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::PlatformLimitationsNoteMissing));
}

#[test]
fn missing_verification_label_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].verification_label.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::VerificationStateMissing));
}

#[test]
fn missing_store_row_actions_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].default_actions =
        vec![CredentialStoreCapabilityRowAction::ExportRow];
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::StoreRowActionsIncomplete));
}

#[test]
fn store_masking_storage_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].masks_storage_or_reveal_posture = true;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::StorageOrRevealMasked));
}

#[test]
fn prompt_normalizing_raw_secret_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].implies_raw_secret_exportable = true;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::RawSecretHandlingNormalized));
}

#[test]
fn vague_saved_securely_wording_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].uses_friendly_connected_wording = true;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::FriendlyConnectedWordingUsed));
}

#[test]
fn missing_required_labels_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].required_labels = vec![M5CredentialRequiredLabel::Identity];
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::RequiredLabelsIncomplete));
}

#[test]
fn missing_degraded_states_fails() {
    let mut packet = packet();
    packet.store_capability_rows[0].degraded_states.clear();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::DegradedStatesMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .unverified_or_unsupported_never_reads_as_secure = false;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .handle_only_path_visible_before_raw_reveal = false;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.secret_access_prompts[0].purpose_note = "see internal://creds".to_owned();
    assert!(packet
        .validate()
        .contains(&SecretAccessPromptStoreCapabilityViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Secret-access prompt sheets"));
    assert!(summary.contains("## Credential-store-capability rows"));
    assert!(summary.contains("raw_reveal_requested"));
    assert!(summary.contains("unverified_store"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 prompts + 6 rows
    assert_eq!(lines, 1 + 6 + 6);
    assert!(csv.contains("secret_access_prompt_sheet"));
    assert!(csv.contains("credential_store_capability_row"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_secret_access_prompt_store_capability_export()
        .expect("checked secret access prompt store capability export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-secret-access-prompt-store-capability-controls/secret_access_prompt_raw_reveal.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-secret-access-prompt-store-capability-controls/store_capability_unverified.json"
        )),
    ] {
        let packet: SecretAccessPromptStoreCapabilityControlsPacket = serde_json::from_str(raw)
            .expect("fixture parses as secret access prompt store capability packet");
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
        seeded_secret_access_prompt_store_capability_controls_secret_access_prompt_raw_reveal(),
        seeded_secret_access_prompt_store_capability_controls_store_capability_unverified(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
