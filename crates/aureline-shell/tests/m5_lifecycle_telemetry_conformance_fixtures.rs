//! Protected fixture checks for the M5 lifecycle-telemetry-conformance capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/state/m5-lifecycle-telemetry-conformance/` through the Rust types and asserts the
//! contract invariants. The packet, dashboard, and support-export fixtures are also asserted
//! bit-for-bit equal to the records minted by the seed, and the published markdown report under
//! `artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md`, the published packet under
//! `artifacts/release/m5-lifecycle-telemetry-conformance-proof/packet.json`, the published dashboard
//! under `artifacts/release/m5-lifecycle-telemetry-conformance-proof/dashboard.json`, and the
//! published CSV under `artifacts/release/m5-lifecycle-telemetry-conformance-proof/matrix.csv` are
//! asserted bit-for-bit equal to the rendering, so the headless emitter remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_lifecycle_telemetry_conformance::{
    seeded_m5_lifecycle_telemetry_conformance_packet,
    validate_m5_lifecycle_telemetry_conformance_packet, TelemetryConformanceDashboard,
    TelemetryConformancePacket, TelemetryConformanceStatus, TelemetryConformanceSupportExport,
    M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_RECORD_KIND,
    M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_REPORT_REF,
    M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SHARED_CONTRACT_REF, REQUIRED_OBJECT_FAMILIES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/state/m5-lifecycle-telemetry-conformance")
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
    let on_disk: TelemetryConformancePacket = load_json("packet.json");
    let seeded = seeded_m5_lifecycle_telemetry_conformance_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(
        seeded.record_kind,
        M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PACKET_RECORD_KIND
    );
    assert_eq!(
        seeded.shared_contract_ref,
        M5_LIFECYCLE_TELEMETRY_CONFORMANCE_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_LIFECYCLE_TELEMETRY_CONFORMANCE_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    validate_m5_lifecycle_telemetry_conformance_packet(&packet)
        .expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_object_family() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_OBJECT_FAMILIES.len());
    for family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            packet.row(family).is_some(),
            "packet omits {}",
            family.as_str()
        );
    }
}

#[test]
fn fixture_status_is_the_derived_auto_narrowed_value() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.object_family.as_str()
        );
        assert_eq!(row.conformance_causes, row.recompute_causes());
        if !matches!(row.derived_status, TelemetryConformanceStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.object_family.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_row_emits_all_sinks_keeps_all_fields_and_certifies_all_surfaces() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    for row in &packet.rows {
        assert!(
            row.telemetry_sinks_complete(),
            "row {} does not emit into all four telemetry sinks",
            row.object_family.as_str()
        );
        assert!(
            row.mandatory_fields_complete(),
            "row {} does not keep all three mandatory fields conformant",
            row.object_family.as_str()
        );
        assert!(
            row.consumer_surfaces_complete(),
            "row {} does not certify all declared consumer surfaces",
            row.object_family.as_str()
        );
        assert!(
            row.headless_parity_preserved,
            "row {} lost headless parity",
            row.object_family.as_str()
        );
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    let on_disk: TelemetryConformanceDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: TelemetryConformancePacket = load_json("packet.json");
    let export: TelemetryConformanceSupportExport = load_json("support_export.json");
    let expected = TelemetryConformanceSupportExport::from_packet(
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
    let seeded = seeded_m5_lifecycle_telemetry_conformance_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let rendered = packet.render_markdown();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md"),
    )
    .expect("published m5-lifecycle-telemetry-conformance.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- markdown`",
    );
}

#[test]
fn published_packet_json_matches_seed() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let rendered = packet.export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-lifecycle-telemetry-conformance-proof/packet.json"),
    )
    .expect("published packet.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published packet diverged from seed -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- packet`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root()
            .join("artifacts/release/m5-lifecycle-telemetry-conformance-proof/dashboard.json"),
    )
    .expect("published dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- dashboard`",
    );
}

#[test]
fn published_csv_matches_seeded_rendering() {
    let packet = seeded_m5_lifecycle_telemetry_conformance_packet();
    let rendered = packet.render_matrix_csv();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-lifecycle-telemetry-conformance-proof/matrix.csv"),
    )
    .expect("published matrix.csv must exist");
    assert_eq!(
        on_disk, rendered,
        "published CSV diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_lifecycle_telemetry_conformance -- csv`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_families() {
    let body = std::fs::read_to_string(
        repo_root().join("docs/lifecycle/m5_lifecycle_telemetry_conformance_contract.md"),
    )
    .expect("published telemetry-conformance contract must exist");
    assert!(body.contains("artifacts/lifecycle/m5-lifecycle-telemetry-conformance.md"));
    assert!(body.contains("artifacts/release/m5-lifecycle-telemetry-conformance-proof/packet.json"));
    assert!(
        body.contains("artifacts/release/m5-lifecycle-telemetry-conformance-proof/dashboard.json")
    );
    assert!(body.contains("fixtures/state/m5-lifecycle-telemetry-conformance/packet.json"));
    assert!(body.contains("schemas/lifecycle/m5-lifecycle-telemetry-conformance.schema.json"));
    for family in REQUIRED_OBJECT_FAMILIES {
        assert!(
            body.contains(family.as_str()),
            "doc must name family {}",
            family.as_str()
        );
    }
}
