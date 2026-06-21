//! Conformance test for the voice provider routing fixtures and artifact.
//!
//! Re-mints the seeded packet, the per-scenario fixtures, the compact summary,
//! and the support-export artifact, and asserts the on-disk copies are bit-for-bit
//! equal so they can never drift silently from the in-crate builder. The suite
//! also re-validates the packet and confirms every resolved outcome matches a
//! fresh resolution of its inputs.

use std::path::{Path, PathBuf};

use aureline_shell::voice_provider_routing::seed::row_fixture_file_name;
use aureline_shell::voice_provider_routing::{
    current_voice_provider_routing_export, fixture_json, seeded_voice_provider_routing_packet,
    VoiceProviderRoutingPacket, VOICE_PROVIDER_ROUTING_ARTIFACT_REF,
    VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF,
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
    let violations = seeded_voice_provider_routing_packet().validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_recorded_outcome_is_honest() {
    for row in &seeded_voice_provider_routing_packet().rows {
        assert!(
            row.outcome_is_honest(),
            "recorded outcome drifted from resolver for {}",
            row.scenario_id
        );
    }
}

#[test]
fn checked_support_export_matches_seed() {
    let packet = seeded_voice_provider_routing_packet();
    let on_disk = read(&repo_root().join(VOICE_PROVIDER_ROUTING_ARTIFACT_REF));
    let expected = fixture_json(&packet).expect("serialize");
    assert_eq!(
        on_disk, expected,
        "{VOICE_PROVIDER_ROUTING_ARTIFACT_REF} drifted from the seed; re-run `cargo run -p aureline-shell --example dump_voice_provider_routing -- support`"
    );

    // And the in-crate reader validates the same artifact.
    let parsed: VoiceProviderRoutingPacket =
        serde_json::from_str(&on_disk).expect("artifact parses");
    assert_eq!(parsed, packet);
    let reread = current_voice_provider_routing_export().expect("artifact validates");
    assert_eq!(reread, packet);
}

#[test]
fn checked_fixtures_match_seed() {
    let packet = seeded_voice_provider_routing_packet();
    let dir = repo_root().join(VOICE_PROVIDER_ROUTING_FIXTURES_DIR_REF);

    let packet_json = fixture_json(&packet).expect("serialize");
    assert_eq!(
        read(&dir.join("packet.json")),
        packet_json,
        "packet.json drifted from the seed; re-run the routing fixtures dump"
    );

    for row in &packet.rows {
        let name = row_fixture_file_name(&row.scenario_id);
        let expected = fixture_json(row).expect("serialize");
        assert_eq!(
            read(&dir.join(&name)),
            expected,
            "{name} drifted from the seed; re-run the routing fixtures dump"
        );
    }

    let mut compact = packet.compact_lines().join("\n");
    compact.push('\n');
    assert_eq!(
        read(&dir.join("compact.txt")),
        compact,
        "compact.txt drifted from the seed; re-run the routing fixtures dump"
    );
}
