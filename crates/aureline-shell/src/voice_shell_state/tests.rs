//! Unit and fixture-equality coverage for the voice shell-state lane.

use std::path::{Path, PathBuf};

use super::seed::seeded_voice_shell_state_packet;
use super::*;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/voice/mode-and-mic-state")
}

#[test]
fn seed_validates_and_marks_every_invariant_satisfied() {
    let packet = seeded_voice_shell_state_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "seed must validate: {violations:?}");
    assert!(packet.is_well_formed());
    assert_eq!(
        packet.invariants,
        VoiceShellStateInvariantManifest::all_true()
    );
    assert!(packet.raw_audio_or_transcript_bytes_excluded);
}

#[test]
fn seed_envelope_is_stable() {
    let packet = seeded_voice_shell_state_packet();
    assert_eq!(packet.record_kind, VOICE_SHELL_STATE_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, VOICE_SHELL_STATE_SCHEMA_VERSION);
    assert_eq!(
        packet.shared_contract_ref,
        VOICE_SHELL_STATE_SHARED_CONTRACT_REF
    );
    assert_eq!(packet.packet_id, VOICE_SHELL_STATE_PACKET_ID);
    assert_eq!(packet.doc_ref, VOICE_SHELL_STATE_DOC_REF);
    assert_eq!(packet.fixtures_dir_ref, VOICE_SHELL_STATE_FIXTURES_DIR_REF);
    for row in &packet.rows {
        assert_eq!(row.record_kind, VOICE_SHELL_STATE_ROW_RECORD_KIND);
        assert_eq!(row.schema_version, VOICE_SHELL_STATE_SCHEMA_VERSION);
        assert_eq!(
            row.shared_contract_ref,
            VOICE_SHELL_STATE_SHARED_CONTRACT_REF
        );
        assert_eq!(row.redaction_class, REDACTION_CLASS);
    }
}

#[test]
fn seed_covers_every_lifecycle_state() {
    let packet = seeded_voice_shell_state_packet();
    let states: Vec<VoiceShellLifecycleState> =
        packet.rows.iter().map(|r| r.lifecycle_state).collect();
    for expected in [
        VoiceShellLifecycleState::Idle,
        VoiceShellLifecycleState::Listening,
        VoiceShellLifecycleState::Processing,
        VoiceShellLifecycleState::NeedsConfirmation,
        VoiceShellLifecycleState::Unavailable,
        VoiceShellLifecycleState::PolicyBlocked,
    ] {
        assert!(states.contains(&expected), "missing state {expected:?}");
    }
}

#[test]
fn seed_discloses_both_local_and_hosted_localities() {
    let packet = seeded_voice_shell_state_packet();
    let localities: Vec<ProcessingLocalityCue> = packet
        .rows
        .iter()
        .map(|r| r.provider_locality.processing_locality)
        .collect();
    assert!(localities.contains(&ProcessingLocalityCue::LocalOnDevice));
    assert!(localities.contains(&ProcessingLocalityCue::HostedRemoteDisclosed));
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_voice_shell_state_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let parsed: VoiceShellStatePacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(packet, parsed);
}

#[test]
fn capturing_states_require_a_visible_mic_indicator() {
    let mut packet = seeded_voice_shell_state_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.lifecycle_state == VoiceShellLifecycleState::Listening)
        .expect("listening row");
    row.mic_indicator.indicator_class = MicIndicatorClass::PersistentIndicatorHiddenCaptureDisabled;
    row.mic_indicator.capture_active = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceShellStateViolation::MicIndicatorHiddenDuringCapture { .. }
    )));
}

#[test]
fn continuous_listening_without_opt_in_is_rejected() {
    let mut packet = seeded_voice_shell_state_packet();
    let row = &mut packet.rows[0];
    row.activation.default_activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
    row.activation.continuous_requires_opt_in = false;
    row.activation.background_listening_state = BackgroundListeningState::OffDefault;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceShellStateViolation::ContinuousListeningWithoutOptIn { .. }
    )));
}

#[test]
fn continuous_listening_with_explicit_opt_in_is_allowed() {
    let mut packet = seeded_voice_shell_state_packet();
    let row = &mut packet.rows[0];
    row.activation.default_activation_class = VoiceActivationClass::WakePhraseContinuousUserOptedIn;
    row.activation.continuous_requires_opt_in = true;
    row.activation.background_listening_state = BackgroundListeningState::OnUserOptedIn;
    assert!(row.activation_default_ok());
    assert!(packet.validate().iter().all(|v| !matches!(
        v,
        VoiceShellStateViolation::ContinuousListeningWithoutOptIn { .. }
    )));
}

#[test]
fn claimed_row_must_disclose_locality_inline() {
    let mut packet = seeded_voice_shell_state_packet();
    packet.rows[0]
        .provider_locality
        .visible_without_settings_dive = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceShellStateViolation::ProviderLocalityRequiresSettingsDive { .. }
    )));
}

#[test]
fn blocked_row_without_keyboard_recovery_is_rejected() {
    let mut packet = seeded_voice_shell_state_packet();
    let row = packet
        .rows
        .iter_mut()
        .find(|r| r.lifecycle_state == VoiceShellLifecycleState::PolicyBlocked)
        .expect("policy-blocked row");
    row.recovery.keyboard_first_recovery_immediate = false;
    row.recovery.keyboard_fallback_command_id.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceShellStateViolation::BlockedStateMissingKeyboardRecovery { .. }
    )));
}

#[test]
fn implicit_mode_is_rejected() {
    let mut packet = seeded_voice_shell_state_packet();
    packet.rows[0].mode_strip.both_modes_visible = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceShellStateViolation::ModeNotExplicitlyVisible { .. })));
}

#[test]
fn unannounced_state_is_rejected() {
    let mut packet = seeded_voice_shell_state_packet();
    packet.rows[0].screen_reader_announces_state = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceShellStateViolation::StateNotAnnounced { .. })));
}

#[test]
fn render_markdown_lists_every_row() {
    let packet = seeded_voice_shell_state_packet();
    let md = packet.render_markdown();
    assert!(md.starts_with("# Voice shell states"));
    for row in &packet.rows {
        assert!(md.contains(&row.row_id), "markdown missing {}", row.row_id);
    }
}

#[test]
fn compact_lines_summarize_each_row() {
    let packet = seeded_voice_shell_state_packet();
    let lines = packet.compact_lines();
    // One header line plus one per row.
    assert_eq!(lines.len(), packet.rows.len() + 1);
}

#[test]
fn on_disk_fixtures_match_seed_bit_for_bit() {
    let packet = seeded_voice_shell_state_packet();
    let dir = fixtures_dir();

    let expected_packet = fixture_json(&packet).expect("serialize packet");
    let actual_packet =
        std::fs::read_to_string(dir.join("packet.json")).expect("read packet.json fixture");
    assert_eq!(
        actual_packet, expected_packet,
        "packet.json drifted from seed; regenerate with the dump_voice_shell_state example"
    );

    for row in &packet.rows {
        let file = row_fixture_file_name(row.lifecycle_state);
        let expected = fixture_json(row).expect("serialize row");
        let actual =
            std::fs::read_to_string(dir.join(file)).unwrap_or_else(|e| panic!("read {file}: {e}"));
        assert_eq!(actual, expected, "{file} drifted from seed");
    }

    let mut expected_compact = packet.compact_lines().join("\n");
    expected_compact.push('\n');
    let actual_compact =
        std::fs::read_to_string(dir.join("compact.txt")).expect("read compact.txt fixture");
    assert_eq!(
        actual_compact, expected_compact,
        "compact.txt drifted from seed"
    );
}
