//! Protected fixture checks for the M5 critical-journey checkpoint capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/state/m5-critical-journey-checkpoints/` through the Rust types and asserts the contract
//! invariants. The packet, dashboard, and support-export fixtures are also asserted bit-for-bit
//! equal to the records minted by the seed, and the published markdown report under
//! `artifacts/lifecycle/m5-critical-journey-checkpoints.md`, the published packet under
//! `artifacts/release/m5-critical-journey-checkpoints-proof/packet.json`, the published dashboard
//! under `artifacts/release/m5-critical-journey-checkpoints-proof/dashboard.json`, and the published
//! CSV under `artifacts/release/m5-critical-journey-checkpoints-proof/matrix.csv` are asserted
//! bit-for-bit equal to the rendering, so the headless emitter remains the only mint-from-truth
//! path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_critical_journey_checkpoints::{
    seeded_m5_critical_journey_checkpoints_packet, validate_m5_critical_journey_checkpoints_packet,
    CriticalJourneyDashboard, CriticalJourneyPacket, CriticalJourneyStatus,
    CriticalJourneySupportExport, M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_RECORD_KIND,
    M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_REPORT_REF,
    M5_CRITICAL_JOURNEY_CHECKPOINTS_SHARED_CONTRACT_REF, REQUIRED_PROTECTED_JOURNEYS,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/state/m5-critical-journey-checkpoints")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_packet_is_bit_for_bit_equal_to_seed() {
    let on_disk: CriticalJourneyPacket = load_json("packet.json");
    let seeded = seeded_m5_critical_journey_checkpoints_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(
        seeded.record_kind,
        M5_CRITICAL_JOURNEY_CHECKPOINTS_PACKET_RECORD_KIND
    );
    assert_eq!(
        seeded.shared_contract_ref,
        M5_CRITICAL_JOURNEY_CHECKPOINTS_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_CRITICAL_JOURNEY_CHECKPOINTS_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    validate_m5_critical_journey_checkpoints_packet(&packet).expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_protected_journey() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_PROTECTED_JOURNEYS.len());
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        assert!(
            packet.row(journey).is_some(),
            "packet omits {}",
            journey.as_str()
        );
    }
}

#[test]
fn fixture_status_is_the_derived_auto_narrowed_value() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.journey.as_str()
        );
        assert_eq!(row.journey_causes, row.recompute_causes());
        if !matches!(row.derived_status, CriticalJourneyStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.journey.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_row_shows_named_milestones_and_certifies_all_surfaces() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    for row in &packet.rows {
        assert!(
            row.checkpoint_sequence_well_formed(),
            "row {} does not show a well-formed checkpoint sequence",
            row.journey.as_str()
        );
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.journey.as_str()
        );
        assert!(
            row.headless_parity_preserved,
            "row {} lost headless parity",
            row.journey.as_str()
        );
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    let on_disk: CriticalJourneyDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: CriticalJourneyPacket = load_json("packet.json");
    let export: CriticalJourneySupportExport = load_json("support_export.json");
    let expected =
        CriticalJourneySupportExport::from_packet(export.support_export_id.clone(), packet.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&packet.packet_id));
    assert!(export.case_ids.contains(&packet.matrix_packet_ref));
    for waiver in &packet.active_waivers {
        assert!(export.case_ids.contains(&waiver.waiver_id));
    }
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_m5_critical_journey_checkpoints_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let rendered = packet.render_markdown();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/lifecycle/m5-critical-journey-checkpoints.md"),
    )
    .expect("published m5-critical-journey-checkpoints.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- markdown`",
    );
}

#[test]
fn published_packet_json_matches_seed() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let rendered = packet.export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-critical-journey-checkpoints-proof/packet.json"),
    )
    .expect("published packet.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published packet diverged from seed -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- packet`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-critical-journey-checkpoints-proof/dashboard.json"),
    )
    .expect("published dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- dashboard`",
    );
}

#[test]
fn published_csv_matches_seeded_rendering() {
    let packet = seeded_m5_critical_journey_checkpoints_packet();
    let rendered = packet.render_matrix_csv();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-critical-journey-checkpoints-proof/matrix.csv"),
    )
    .expect("published matrix.csv must exist");
    assert_eq!(
        on_disk, rendered,
        "published CSV diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_critical_journey_checkpoints -- csv`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_journeys() {
    let body = std::fs::read_to_string(
        repo_root().join("docs/lifecycle/m5_critical_journey_checkpoints_contract.md"),
    )
    .expect("published critical-journey-checkpoints contract must exist");
    assert!(body.contains("artifacts/lifecycle/m5-critical-journey-checkpoints.md"));
    assert!(body.contains("artifacts/release/m5-critical-journey-checkpoints-proof/packet.json"));
    assert!(body.contains("artifacts/release/m5-critical-journey-checkpoints-proof/dashboard.json"));
    assert!(body.contains("fixtures/state/m5-critical-journey-checkpoints/packet.json"));
    assert!(body.contains("schemas/lifecycle/m5-critical-journey-checkpoints.schema.json"));
    for journey in REQUIRED_PROTECTED_JOURNEYS {
        assert!(
            body.contains(journey.as_str()),
            "doc must name journey {}",
            journey.as_str()
        );
    }
}
