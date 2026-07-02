//! Protected fixture checks for the M5 discoverability-affordance-parity certification.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/commands/m5-discoverability-affordance-parity/` through the Rust types and asserts the contract
//! invariants. The packet, dashboard, and support-export fixtures are also asserted bit-for-bit equal to the
//! records minted by the seed, and the published markdown report under
//! `artifacts/commands/m5-discoverability-affordance-parity.md`, the published packet under
//! `artifacts/release/m5-discoverability-affordance-parity-proof/packet.json`, the published dashboard under
//! `artifacts/release/m5-discoverability-affordance-parity-proof/dashboard.json`, and the published CSV
//! under `artifacts/release/m5-discoverability-affordance-parity-proof/matrix.csv` are asserted bit-for-bit
//! equal to the rendering, so the headless emitter remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_discoverability_affordance_parity::{
    seeded_m5_discoverability_affordance_parity_packet, validate_m5_affordance_parity_packet,
    AffordanceParityDashboard, AffordanceParityPacket, AffordanceParityStatus,
    AffordanceParitySupportExport, M5_AFFORDANCE_PARITY_PACKET_RECORD_KIND,
    M5_AFFORDANCE_PARITY_PUBLISHED_REPORT_REF, M5_AFFORDANCE_PARITY_SHARED_CONTRACT_REF,
    REQUIRED_AFFORDANCES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/commands/m5-discoverability-affordance-parity")
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
    let on_disk: AffordanceParityPacket = load_json("packet.json");
    let seeded = seeded_m5_discoverability_affordance_parity_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(seeded.record_kind, M5_AFFORDANCE_PARITY_PACKET_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_AFFORDANCE_PARITY_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_AFFORDANCE_PARITY_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    validate_m5_affordance_parity_packet(&packet).expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_affordance() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_AFFORDANCES.len());
    for affordance in REQUIRED_AFFORDANCES {
        assert!(
            packet.row(affordance).is_some(),
            "packet omits {}",
            affordance.as_str()
        );
    }
}

#[test]
fn fixture_status_is_the_derived_value() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.affordance.as_str()
        );
        assert_eq!(row.conformance_causes, row.recompute_causes());
        if !matches!(row.derived_status, AffordanceParityStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.affordance.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_row_reuses_fields_modes_and_consumer_surfaces() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    for row in &packet.rows {
        assert!(
            row.record_fields_complete(),
            "row {} does not reuse all six canonical record fields",
            row.affordance.as_str()
        );
        assert!(
            row.reach_modes_complete(),
            "row {} does not stay reachable in all five reach modes",
            row.affordance.as_str()
        );
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.affordance.as_str()
        );
        assert!(
            row.headless_parity_preserved,
            "row {} lost headless parity",
            row.affordance.as_str()
        );
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    let on_disk: AffordanceParityDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: AffordanceParityPacket = load_json("packet.json");
    let export: AffordanceParitySupportExport = load_json("support_export.json");
    let expected = AffordanceParitySupportExport::from_packet(
        export.support_export_id.clone(),
        packet.clone(),
    );
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
    let seeded = seeded_m5_discoverability_affordance_parity_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let rendered = packet.render_markdown();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/commands/m5-discoverability-affordance-parity.md"),
    )
    .expect("published m5-discoverability-affordance-parity.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- markdown`",
    );
}

#[test]
fn published_packet_json_matches_seed() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let rendered = packet.export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root()
            .join("artifacts/release/m5-discoverability-affordance-parity-proof/packet.json"),
    )
    .expect("published packet.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published packet diverged from seed -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- packet`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root()
            .join("artifacts/release/m5-discoverability-affordance-parity-proof/dashboard.json"),
    )
    .expect("published dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- dashboard`",
    );
}

#[test]
fn published_csv_matches_seeded_rendering() {
    let packet = seeded_m5_discoverability_affordance_parity_packet();
    let rendered = packet.render_matrix_csv();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-discoverability-affordance-parity-proof/matrix.csv"),
    )
    .expect("published matrix.csv must exist");
    assert_eq!(
        on_disk, rendered,
        "published CSV diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_discoverability_affordance_parity -- csv`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_affordances() {
    let body = std::fs::read_to_string(
        repo_root().join("docs/commands/m5_discoverability_affordance_parity_contract.md"),
    )
    .expect("published affordance-parity contract must exist");
    assert!(body.contains("artifacts/commands/m5-discoverability-affordance-parity.md"));
    assert!(
        body.contains("artifacts/release/m5-discoverability-affordance-parity-proof/packet.json")
    );
    assert!(body
        .contains("artifacts/release/m5-discoverability-affordance-parity-proof/dashboard.json"));
    assert!(body.contains("fixtures/commands/m5-discoverability-affordance-parity/packet.json"));
    assert!(body.contains("schemas/commands/m5-discoverability-affordance-parity.schema.json"));
    for affordance in REQUIRED_AFFORDANCES {
        assert!(
            body.contains(affordance.as_str()),
            "doc must name affordance {}",
            affordance.as_str()
        );
    }
}
