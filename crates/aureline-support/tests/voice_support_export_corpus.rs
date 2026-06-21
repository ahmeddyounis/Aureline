//! Conformance test for the voice support-export artifacts and fixtures.
//!
//! Re-mints the seeded packet, the per-session and per-decision fixtures, the
//! compact summary, the support-export JSON artifact, and the rendered Markdown
//! report, and asserts the on-disk copies are bit-for-bit equal so they can
//! never drift silently from the in-crate builder. The suite also re-validates
//! the packet and confirms the no-raw-by-default invariants hold.

use std::path::{Path, PathBuf};

use aureline_support::voice_redaction::seed::{
    decision_fixture_file_name, session_fixture_file_name,
};
use aureline_support::voice_redaction::{
    current_voice_support_export, fixture_json, seeded_voice_support_export_packet,
    VoiceSupportExportPacket, VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF,
    VOICE_SUPPORT_EXPORT_PACKET_REF, VOICE_SUPPORT_EXPORT_REPORT_REF,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read(path: &Path) -> String {
    std::fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

#[test]
fn seeded_packet_validates() {
    let violations = seeded_voice_support_export_packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let packet = seeded_voice_support_export_packet();
    let on_disk = read(&repo_root().join(VOICE_SUPPORT_EXPORT_PACKET_REF));
    let expected = fixture_json(&packet).expect("serialize");
    assert_eq!(
        on_disk, expected,
        "{VOICE_SUPPORT_EXPORT_PACKET_REF} drifted from the seed; re-run `cargo run -p aureline-support --example dump_voice_support_export -- support`"
    );

    let parsed: VoiceSupportExportPacket = serde_json::from_str(&on_disk).expect("artifact parses");
    assert_eq!(parsed, packet);

    // The in-crate reader validates the same artifact.
    current_voice_support_export().expect("checked-in artifact validates");
}

#[test]
fn checked_report_matches_seed() {
    let packet = seeded_voice_support_export_packet();
    let on_disk = read(&repo_root().join(VOICE_SUPPORT_EXPORT_REPORT_REF));
    assert_eq!(
        on_disk,
        packet.render_markdown(),
        "{VOICE_SUPPORT_EXPORT_REPORT_REF} drifted from the seed; re-run `cargo run -p aureline-support --example dump_voice_support_export -- report`"
    );
}

#[test]
fn checked_fixtures_match_seed() {
    let packet = seeded_voice_support_export_packet();
    let dir = repo_root().join(VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF);

    let packet_on_disk = read(&dir.join("packet.json"));
    assert_eq!(packet_on_disk, fixture_json(&packet).expect("serialize"));

    for session in &packet.sessions {
        let on_disk = read(&dir.join(session_fixture_file_name(&session.session_id)));
        assert_eq!(on_disk, fixture_json(session).expect("serialize"));
    }

    for decision in &packet.transcript_export_decisions {
        let on_disk = read(&dir.join(decision_fixture_file_name(decision)));
        assert_eq!(on_disk, fixture_json(decision).expect("serialize"));
    }

    let telemetry_on_disk = read(&dir.join("telemetry-posture.json"));
    assert_eq!(
        telemetry_on_disk,
        fixture_json(&packet.telemetry_posture).expect("serialize")
    );

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    assert_eq!(read(&dir.join("compact.txt")), compact);
}

#[test]
fn no_fixture_carries_raw_speech_markers() {
    let dir = repo_root().join(VOICE_SUPPORT_EXPORT_FIXTURES_DIR_REF);
    // The sample raw transcript used to mint the redaction summary must never
    // appear anywhere in the checked-in fixtures.
    for needle in [
        "jordan.doe@example.com",
        "sk-AB12cd34EF56gh78ij90",
        "/Users/jordan/secret/config.env",
        "processPayment",
    ] {
        for entry in std::fs::read_dir(&dir).expect("read fixtures dir") {
            let path = entry.expect("dir entry").path();
            if path.extension().and_then(|e| e.to_str()) == Some("json")
                || path.extension().and_then(|e| e.to_str()) == Some("txt")
            {
                let body = read(&path);
                assert!(
                    !body.contains(needle),
                    "{} leaked raw speech marker {needle}",
                    path.display()
                );
            }
        }
    }
}
