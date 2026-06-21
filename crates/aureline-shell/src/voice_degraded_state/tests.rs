//! Unit and fixture-equality coverage for the voice degraded-state lane.

use std::path::{Path, PathBuf};

use super::seed::seeded_voice_degraded_state_packet;
use super::*;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/voice/fallback-and-noisy-env")
}

#[test]
fn seed_validates_and_marks_every_invariant_satisfied() {
    let packet = seeded_voice_degraded_state_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "seed must validate: {violations:?}");
    assert!(packet.is_well_formed());
    assert_eq!(
        packet.invariants,
        VoiceDegradedStateInvariantManifest::all_true()
    );
    assert!(packet.raw_audio_or_transcript_bytes_excluded);
}

#[test]
fn seed_envelope_is_stable() {
    let packet = seeded_voice_degraded_state_packet();
    assert_eq!(packet.record_kind, VOICE_DEGRADED_STATE_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, VOICE_DEGRADED_STATE_SCHEMA_VERSION);
    assert_eq!(
        packet.shared_contract_ref,
        VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF
    );
    assert_eq!(packet.packet_id, VOICE_DEGRADED_STATE_PACKET_ID);
    assert_eq!(packet.doc_ref, VOICE_DEGRADED_STATE_DOC_REF);
    assert_eq!(packet.matrix_ref, VOICE_DEGRADED_STATE_MATRIX_REF);
    assert_eq!(
        packet.fixtures_dir_ref,
        VOICE_DEGRADED_STATE_FIXTURES_DIR_REF
    );
    assert_eq!(
        packet.voice_shell_state_contract_ref,
        VOICE_SHELL_STATE_CONTRACT_REF
    );
    for flow in &packet.flows {
        assert_eq!(flow.record_kind, VOICE_DEGRADED_FLOW_RECORD_KIND);
        assert_eq!(flow.schema_version, VOICE_DEGRADED_STATE_SCHEMA_VERSION);
        assert_eq!(
            flow.shared_contract_ref,
            VOICE_DEGRADED_STATE_SHARED_CONTRACT_REF
        );
        assert_eq!(flow.redaction_class, REDACTION_CLASS);
    }
}

#[test]
fn seed_covers_every_failure_class() {
    let packet = seeded_voice_degraded_state_packet();
    for cause in VoiceDegradedCause::ALL {
        assert!(
            packet.flow(cause).is_some(),
            "missing flow for cause {cause:?}"
        );
    }
    assert!(packet.invariants.all_failure_classes_covered);
}

#[test]
fn every_flow_lands_on_a_controlled_state() {
    let packet = seeded_voice_degraded_state_packet();
    for flow in &packet.flows {
        assert!(
            flow.lands_on_controlled_state(),
            "{} did not land on a controlled state",
            flow.flow_id
        );
    }
}

#[test]
fn every_flow_offers_a_keyboard_first_recovery_action() {
    let packet = seeded_voice_degraded_state_packet();
    for flow in &packet.flows {
        assert!(
            flow.has_keyboard_first_recovery_action(),
            "{} missing keyboard-first recovery action",
            flow.flow_id
        );
        assert!(flow.keyboard_fallback.preserves_continuity());
    }
}

#[test]
fn canonical_reason_matches_cause_or_is_absent() {
    let packet = seeded_voice_degraded_state_packet();
    for flow in &packet.flows {
        assert_eq!(
            flow.canonical_unavailable_reason,
            flow.cause.canonical_unavailable_reason()
        );
    }
    // Language-pack-missing has no canonical reason token.
    assert_eq!(
        packet
            .flow(VoiceDegradedCause::LanguagePackMissing)
            .unwrap()
            .canonical_unavailable_reason,
        None
    );
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_voice_degraded_state_packet();
    let json = serde_json::to_string(&packet).expect("serialize");
    let parsed: VoiceDegradedStatePacket = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(packet, parsed);
}

#[test]
fn generic_or_transient_banner_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    let flow = &mut packet.flows[0];
    flow.banner.names_specific_cause = false;
    flow.banner.cause_detail_label_ref.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::GenericOrTransientBanner { .. }
    )));
}

#[test]
fn flow_without_a_recovery_action_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].recovery_actions.clear();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::NoConcreteRecoveryAction { .. }
    )));
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::MissingKeyboardFirstFallback { .. }
    )));
}

#[test]
fn keyboard_fallback_that_loses_work_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].keyboard_fallback.preserves_focus_and_work = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::KeyboardFallbackLosesContinuity { .. }
    )));
}

#[test]
fn uncontrolled_state_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].lifecycle_state = VoiceShellLifecycleState::Listening;
    packet.flows[0].enters_controlled_state = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceDegradedStateViolation::NotAControlledState { .. })));
}

#[test]
fn policy_state_inconsistency_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    // A non-policy cause must not claim a policy-blocked state.
    let flow = packet
        .flows
        .iter_mut()
        .find(|f| f.cause == VoiceDegradedCause::MissingMicrophoneHardware)
        .unwrap();
    flow.policy_state = VoicePolicyState::PolicyBlocked;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::PolicyStateInconsistent { .. }
    )));
}

#[test]
fn canonical_reason_mismatch_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].canonical_unavailable_reason = Some(VoiceUnavailableReason::NoisyEnvironment);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::CanonicalReasonMismatch { .. }
    )));
}

#[test]
fn unsafe_narration_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].narration.announced_once_per_transition = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, VoiceDegradedStateViolation::NarrationUnsafe { .. })));
}

#[test]
fn suppressing_nonvoice_recovery_is_rejected() {
    let mut packet = seeded_voice_degraded_state_packet();
    packet.flows[0].preserves_nonvoice_recovery = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        VoiceDegradedStateViolation::SuppressesNonVoiceRecovery { .. }
    )));
}

#[test]
fn render_markdown_lists_every_cause() {
    let packet = seeded_voice_degraded_state_packet();
    let md = packet.render_markdown();
    assert!(md.starts_with("# Voice degraded-state and recovery matrix"));
    for cause in VoiceDegradedCause::ALL {
        assert!(md.contains(cause.as_str()), "markdown missing {cause:?}");
    }
}

#[test]
fn compact_lines_summarize_each_flow() {
    let packet = seeded_voice_degraded_state_packet();
    let lines = packet.compact_lines();
    // One header line plus one per flow.
    assert_eq!(lines.len(), packet.flows.len() + 1);
}

#[test]
fn on_disk_fixtures_match_seed_bit_for_bit() {
    let packet = seeded_voice_degraded_state_packet();
    let dir = fixtures_dir();

    let expected_packet = fixture_json(&packet).expect("serialize packet");
    let actual_packet =
        std::fs::read_to_string(dir.join("packet.json")).expect("read packet.json fixture");
    assert_eq!(
        actual_packet, expected_packet,
        "packet.json drifted from seed; regenerate with the dump_voice_degraded_state example"
    );

    for flow in &packet.flows {
        let file = flow_fixture_file_name(flow.cause);
        let expected = fixture_json(flow).expect("serialize flow");
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
