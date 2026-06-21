use super::*;

fn packet() -> M5VoiceQualificationMatrixPacket {
    seeded_voice_qualification_matrix_packet()
}

#[test]
fn seeded_packet_validates() {
    let violations = packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn seeded_packet_covers_every_surface_kind() {
    let kinds = packet().represented_surface_kinds();
    for required in VoiceSurfaceKind::ALL {
        assert!(
            kinds.contains(&required),
            "missing surface kind {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_packet_covers_command_and_dictation_modes() {
    let modes = packet().represented_modes();
    assert!(modes.contains(&VoiceModeClass::CommandModeActive));
    assert!(modes.contains(&VoiceModeClass::DictationModeActive));
    // The two modes are distinct, explicit states — never one inferred label.
    assert_ne!(
        VoiceModeClass::CommandModeActive,
        VoiceModeClass::DictationModeActive
    );
}

#[test]
fn seeded_packet_separates_claimed_scope_from_labs() {
    let packet = packet();
    assert!(
        packet.labs_row_count() >= 1,
        "expected a Labs/unadvertised row"
    );
    assert!(packet.claimed_row_count() >= 1, "expected claimed rows");
    let labs = packet
        .rows
        .iter()
        .find(|row| row.claim_posture == VoiceClaimPosture::LabsUnadvertised)
        .expect("a Labs row");
    assert!(!labs.is_claimed());
    assert_eq!(
        labs.claimed_grade,
        VoiceQualificationGrade::LabsUnadvertisedProfile
    );
}

#[test]
fn seeded_packet_has_clean_claimed_and_downgraded_rows() {
    let packet = packet();
    assert_eq!(packet.downgraded_row_count(), 2);
    assert!(packet.rows.iter().any(|row| {
        !row.needs_downgrade()
            && row.is_claimed()
            && row.claimed_grade == VoiceQualificationGrade::QualifiedClaimedProfile
    }));
    let downgraded = packet
        .rows
        .iter()
        .find(|row| row.needs_downgrade())
        .expect("a downgraded row");
    assert!(downgraded.properly_downgraded());
    assert!(downgraded.downgrade_trigger.is_some());
    assert!(downgraded.downgrade_consistent());
}

#[test]
fn unavailable_fallback_row_keeps_keyboard_path() {
    let packet = packet();
    let fallback = packet
        .rows
        .iter()
        .find(|row| row.surface_kind == VoiceSurfaceKind::UnavailableFallback)
        .expect("an unavailable/fallback row");
    assert!(fallback.keyboard_fallback_ok());
    assert!(fallback.needs_downgrade());
    assert_eq!(
        fallback.unavailable_reason,
        Some(VoiceUnavailableReason::NoMicrophone)
    );
}

#[test]
fn claimed_row_losing_current_proof_must_downgrade() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    row.verification.proof_currency = VoiceProofCurrency::StaleExpired;
    assert!(row.needs_downgrade());
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::RowNotDowngradedOnDeniedAxis));
}

#[test]
fn hosted_locality_undisclosed_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:provider-privacy:enterprise:0001")
        .expect("enterprise provider row");
    // A hosted provider that drops its disclosed transport must downgrade.
    row.provider.transport_class = VoiceTransportClass::LocalInProcessOnly;
    assert!(!row.locality_disclosed_ok());
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::ProviderLocalityDeniedNotDowngraded));
}

#[test]
fn incomplete_command_parity_is_rejected_when_not_downgraded() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:dictation-input:local:0001")
        .expect("dictation row");
    row.command_parity.stable_command_ids = false;
    assert!(!row.command_parity.parity_complete());
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::CommandParityDeniedNotDowngraded));
}

#[test]
fn continuous_activation_without_opt_in_is_rejected() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    // Claimed profile defaults to wake/continuous without an opt-in background state.
    row.session.activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
    assert!(!row.push_to_talk_default_ok());
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::ActivationDefaultDeniedNotDowngraded));
}

#[test]
fn missing_keyboard_fallback_is_rejected() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    row.provider.keyboard_fallback_available = false;
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::KeyboardFallbackMissing));
}

#[test]
fn raw_transcript_retention_is_rejected() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    row.provider
        .retention_posture
        .raw_transcripts_excluded_by_default = false;
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::RawTranscriptRetainedByDefault));
}

#[test]
fn background_listening_default_on_is_rejected() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    row.provider
        .capability_disclosure
        .background_listening_default_off = false;
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::BackgroundListeningDefaultOn));
}

#[test]
fn background_inconsistent_with_activation_is_rejected() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    // Background on, but activation is push-to-talk, not an opted-in wake mode.
    row.session.background_listening_state = BackgroundListeningState::OnUserOptedIn;
    let violations = packet.validate();
    assert!(
        violations.contains(&VoiceMatrixViolation::BackgroundListeningInconsistentWithActivation)
    );
}

#[test]
fn provider_linked_row_proof_never_reads_as_local() {
    let packet = packet();
    let linked = packet
        .rows
        .iter()
        .find(|row| row.origin_class == VoiceProfileOriginClass::ProviderLinkedProfile)
        .expect("a provider-linked row");
    assert!(linked.provider_or_imported());
    assert!(linked.imported_posture_consistent());
    assert!(linked.verification.backs_claim(true));
    assert!(!linked.verification.backs_claim(false));
}

#[test]
fn provider_linked_row_with_local_proof_is_rejected() {
    let mut packet = packet();
    let linked = packet
        .rows
        .iter_mut()
        .find(|row| row.origin_class == VoiceProfileOriginClass::ProviderLinkedProfile)
        .expect("a provider-linked row");
    linked.verification.proof_currency = VoiceProofCurrency::VerifiedCurrent;
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::ImportedReadsAsLocal));
}

#[test]
fn session_must_bind_its_provider() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    row.session.provider_id = "voice.provider.someone_else".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::SessionProviderRefMismatch));
}

#[test]
fn fingerprint_must_be_independent_of_id() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.profile_id == "voice-qual:dictation-input:local:0001")
        .expect("dictation row");
    row.profile_fingerprint_token = row.profile_id.clone();
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::FingerprintSubstitutesIdentity));
}

#[test]
fn downgraded_label_must_not_be_generic() {
    let mut packet = packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|row| row.needs_downgrade())
        .expect("a downgraded row");
    row.downgraded_label = Some("error".to_owned());
    let violations = packet.validate();
    assert!(violations.contains(&VoiceMatrixViolation::DowngradedRowMissingLabelOrTrigger));
}

#[test]
fn wrong_record_kind_is_rejected() {
    let mut packet = packet();
    packet.record_kind = "something_else".to_owned();
    assert!(packet
        .validate()
        .contains(&VoiceMatrixViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|r| r != VOICE_QUALIFICATION_MATRIX_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&VoiceMatrixViolation::MissingSourceContracts));
}

#[test]
fn provider_descriptor_is_a_standalone_well_formed_object() {
    let row = packet()
        .rows
        .into_iter()
        .find(|row| row.profile_id == "voice-qual:command-overlay:local:0001")
        .expect("local command overlay row");
    let provider = row.provider;
    assert_eq!(provider.record_kind, VOICE_PROVIDER_DESCRIPTOR_RECORD_KIND);
    assert!(provider.is_well_formed());
    assert!(provider.locality_disclosed());
    assert!(provider.retention_posture.bounded_for_general_claim());
    // Round-trips through its own boundary shape.
    let json = serde_json::to_string(&provider).expect("serializes");
    let parsed: VoiceProviderDescriptor = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, provider);
}

#[test]
fn enterprise_provider_retention_is_not_a_general_claim() {
    let row = packet()
        .rows
        .into_iter()
        .find(|row| row.profile_id == "voice-qual:provider-privacy:enterprise:0001")
        .expect("enterprise row");
    // Provider-per-contract retention is a deliberate narrowing, not a general claim.
    assert!(!row.provider.retention_posture.bounded_for_general_claim());
    assert!(row.provider.locality_disclosed());
    assert_eq!(
        row.claimed_grade,
        VoiceQualificationGrade::QualifiedNarrowedProfile
    );
    assert!(!row.needs_downgrade());
}

#[test]
fn session_is_a_standalone_well_formed_object() {
    let row = packet()
        .rows
        .into_iter()
        .find(|row| row.profile_id == "voice-qual:dictation-input:local:0001")
        .expect("dictation row");
    let session = row.session;
    assert_eq!(session.record_kind, VOICE_SESSION_STATE_RECORD_KIND);
    assert!(session.is_well_formed());
    assert!(session.mode_is_explicit());
    assert!(session.activation_default_ok());
    let json = serde_json::to_string(&session).expect("serializes");
    let parsed: VoiceSessionState = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, session);
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: M5VoiceQualificationMatrixPacket =
        serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn markdown_summary_names_rows_and_downgrade() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("Qualification Matrix"));
    assert!(summary.contains("command_overlay"));
    assert!(summary.contains("provider"));
    assert!(summary.contains("Downgraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_voice_qualification_matrix_export()
        .expect("checked voice qualification export validates");
    assert_eq!(checked, packet());
}

#[test]
fn grade_ranks_are_strictly_ordered() {
    assert!(
        VoiceQualificationGrade::QualifiedClaimedProfile.rank()
            > VoiceQualificationGrade::QualifiedNarrowedProfile.rank()
    );
    assert!(
        VoiceQualificationGrade::QualifiedNarrowedProfile.rank()
            > VoiceQualificationGrade::LabsUnadvertisedProfile.rank()
    );
    assert!(
        VoiceQualificationGrade::LabsUnadvertisedProfile.rank()
            > VoiceQualificationGrade::QualificationWithdrawn.rank()
    );
    assert!(
        VoiceQualificationGrade::QualificationWithdrawn.rank()
            > VoiceQualificationGrade::NotApplicable.rank()
    );
}
