use std::fs;
use std::path::PathBuf;

use aureline_chronology::{
    seeded_accessibility_fixture, seeded_chronology_export_packet, seeded_chronology_packet,
    validate_accessibility_fixture, validate_chronology_export_packet, validate_chronology_packet,
    AccessibilityChronologyFixture, ChronologyExportPacket, ChronologyHistoryPacket,
    ChronologySurfaceClass, FollowUpTransitionKind, LocalAuthorityEffectClass,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/ux/m4/stabilize-chronology-grammar-and-history-row-truth")
}

#[test]
fn seeded_packet_covers_stable_attention_surfaces() {
    let packet = seeded_chronology_packet();
    let report = validate_chronology_packet(&packet);
    assert!(report.passed, "{:#?}", report.findings);

    for surface in [
        ChronologySurfaceClass::ActivityCenter,
        ChronologySurfaceClass::DurableJob,
        ChronologySurfaceClass::DebugHistory,
        ChronologySurfaceClass::ProviderEvent,
        ChronologySurfaceClass::AiRun,
        ChronologySurfaceClass::PolicyAdminNotice,
        ChronologySurfaceClass::RecoveryTimeline,
    ] {
        assert!(
            packet.rows.iter().any(|row| row.surface_class == surface),
            "missing {surface:?}"
        );
    }
}

#[test]
fn checked_in_packet_fixture_matches_canonical_seed_and_validates() {
    let body = fs::read_to_string(fixture_dir().join("chronology_packet.json")).expect("fixture");
    let fixture: ChronologyHistoryPacket = serde_json::from_str(&body).expect("json");
    assert_eq!(fixture, seeded_chronology_packet());

    let report = validate_chronology_packet(&fixture);
    assert!(report.passed, "{:#?}", report.findings);
}

#[test]
fn checked_in_export_fixture_preserves_live_chronology_rows() {
    let body =
        fs::read_to_string(fixture_dir().join("support_export_packet.json")).expect("fixture");
    let export: ChronologyExportPacket = serde_json::from_str(&body).expect("json");
    assert_eq!(export, seeded_chronology_export_packet());

    let packet = seeded_chronology_packet();
    let report = validate_chronology_export_packet(&packet, &export);
    assert!(report.passed, "{:#?}", report.findings);
}

#[test]
fn checked_in_accessibility_fixture_distinguishes_identity_time_and_provenance() {
    let body =
        fs::read_to_string(fixture_dir().join("accessibility_fixture.json")).expect("fixture");
    let fixture: AccessibilityChronologyFixture = serde_json::from_str(&body).expect("json");
    assert_eq!(fixture, seeded_accessibility_fixture());

    let packet = seeded_chronology_packet();
    let report = validate_accessibility_fixture(&packet, &fixture);
    assert!(report.passed, "{:#?}", report.findings);
}

#[test]
fn provider_owned_rows_keep_local_follow_up_local() {
    let packet = seeded_chronology_packet();
    let provider_row = packet
        .rows
        .iter()
        .find(|row| row.provider_owned_object)
        .expect("provider row");

    for transition in &provider_row.allowed_transitions {
        if matches!(
            transition.transition,
            FollowUpTransitionKind::Acknowledge
                | FollowUpTransitionKind::Resolve
                | FollowUpTransitionKind::Dismiss
                | FollowUpTransitionKind::Snooze
                | FollowUpTransitionKind::Mute
        ) {
            assert_eq!(
                transition.local_authority_effect,
                LocalAuthorityEffectClass::LocalOnly
            );
            assert!(transition.reviewed_provider_command_ref.is_none());
        }
    }
}
