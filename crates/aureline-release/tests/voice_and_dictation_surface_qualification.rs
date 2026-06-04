//! Protected tests for the voice/dictation surface qualification packet.

use std::path::{Path, PathBuf};

use aureline_release::stable_claim_matrix::StableClaimLevel;
use aureline_release::voice_and_dictation_surface_qualification::{
    current_voice_and_dictation_surface_qualification, ActivationDefault,
    VoiceAndDictationSurfaceQualification, VoiceQualificationViolation, VoiceSurfaceKind,
    VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND,
    VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION,
};
use serde::Deserialize;

fn packet() -> VoiceAndDictationSurfaceQualification {
    current_voice_and_dictation_surface_qualification()
        .expect("checked-in voice/dictation packet parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    case_id: String,
    expected_check_id: String,
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        VOICE_DICTATION_SURFACE_QUALIFICATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        VOICE_DICTATION_SURFACE_QUALIFICATION_RECORD_KIND
    );
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in voice/dictation packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_matches_row_state_and_preview_rows_stay_narrow() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.stable_surfaces().len(), 6);
    assert_eq!(packet.narrowed_surfaces().len(), 1);

    for surface in &packet.surfaces {
        assert!(
            surface.displayed_label.rank() <= surface.claim_label.rank(),
            "{} displays wider than its claim",
            surface.surface_id
        );
        if surface.processing_class == aureline_release::ProcessingClass::ThirdPartyProvider
            || matches!(
                surface.activation_default,
                ActivationDefault::ContinuousOptIn | ActivationDefault::WakeWordOptIn
            )
        {
            assert_ne!(
                surface.displayed_label,
                StableClaimLevel::Stable,
                "{} must not imply stable third-party or background-listening support",
                surface.surface_id
            );
        }
    }
}

#[test]
fn stable_rows_have_mode_privacy_parity_and_fallback_evidence() {
    let packet = packet();
    let stable_kinds = packet
        .stable_surfaces()
        .into_iter()
        .map(|surface| surface.surface_kind)
        .collect::<std::collections::BTreeSet<_>>();
    assert!(stable_kinds.contains(&VoiceSurfaceKind::CommandOverlay));
    assert!(stable_kinds.contains(&VoiceSurfaceKind::DictationInput));
    assert!(stable_kinds.contains(&VoiceSurfaceKind::TranscriptCorrection));
    assert!(stable_kinds.contains(&VoiceSurfaceKind::UnavailableFallback));
    assert!(stable_kinds.contains(&VoiceSurfaceKind::HighImpactActionReview));

    for surface in packet.stable_surfaces() {
        assert!(
            surface.has_green_packet(),
            "{} lacks green packet",
            surface.surface_id
        );
        assert!(
            surface.owner_signoff.signed_off,
            "{} lacks owner sign-off",
            surface.surface_id
        );
        assert!(
            surface.explicit_mode_visible
                && surface.mute_or_stop_action
                && surface.keyboard_fallback,
            "{} lacks explicit mode, stop, or keyboard fallback",
            surface.surface_id
        );
        assert!(
            !surface.accessibility_refs.is_empty()
                && !surface.regression_refs.is_empty()
                && !surface.privacy_review_refs.is_empty()
                && !surface.support_export_refs.is_empty(),
            "{} lacks evidence refs",
            surface.surface_id
        );
    }
}

#[test]
fn fixture_manifest_is_present_and_negative_drills_fire_expected_checks() {
    let fixture_path = repo_root()
        .join("fixtures/release/m4/voice-and-dictation-surface-qualification/cases.json");
    let payload = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_path.display()));
    let manifest: FixtureManifest =
        serde_json::from_str(&payload).expect("fixture manifest parses");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.cases.is_empty());

    for case in manifest.cases {
        let mut packet = packet();
        match case.case_id.as_str() {
            "stable_surface_without_green_packet" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.qualification_packet = None;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    VoiceQualificationViolation::StableSurfaceWithoutGreenPacket { surface_id },
                );
            }
            "missing_mode_truth" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.explicit_mode_visible = false;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    VoiceQualificationViolation::MissingExplicitModeOrFallback { surface_id },
                );
            }
            "unsafe_activation_default" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.activation_default = ActivationDefault::ContinuousOptIn;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    VoiceQualificationViolation::UnsafeActivationDefault { surface_id },
                );
            }
            "incomplete_transcript_privacy" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface
                        .transcript_privacy
                        .raw_transcripts_excluded_by_default = false;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    VoiceQualificationViolation::IncompleteTranscriptPrivacy { surface_id },
                );
            }
            "missing_command_parity" => {
                let surface_id = {
                    let surface = packet
                        .surfaces
                        .iter_mut()
                        .find(|surface| surface.renders_stable())
                        .expect("stable surface exists");
                    surface.command_parity.approval_requirements = false;
                    surface.surface_id.clone()
                };
                assert_expected(
                    &case.expected_check_id,
                    packet.validate(),
                    VoiceQualificationViolation::IncompleteCommandParity { surface_id },
                );
            }
            other => panic!("unknown fixture case: {other}"),
        }
    }
}

fn assert_expected(
    expected_check_id: &str,
    violations: Vec<VoiceQualificationViolation>,
    expected: VoiceQualificationViolation,
) {
    assert!(
        violations.contains(&expected),
        "expected {expected_check_id} to fire {expected:?}, got {violations:#?}"
    );
}
