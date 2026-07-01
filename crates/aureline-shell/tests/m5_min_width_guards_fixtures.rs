//! Protected fixture checks for the M5 min-width-guard capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ui/m5-min-width-guards/` through the Rust types and asserts the contract
//! invariants. The packet, dashboard, and support-export fixtures are also asserted
//! bit-for-bit equal to the records minted by the seed, and the published markdown
//! report under `artifacts/shell/m5-min-width-guards.md`, the published packet under
//! `artifacts/release/m5-min-width-guards-proof/packet.json`, the published dashboard
//! under `artifacts/release/m5-min-width-guards-proof/dashboard.json`, and the published
//! CSV under `artifacts/release/m5-min-width-guards-proof/matrix.csv` are asserted
//! bit-for-bit equal to the rendering, so the headless emitter remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_min_width_guards::{
    seeded_m5_min_width_guards_packet, validate_m5_min_width_guards_packet, MinWidthGuardDashboard,
    MinWidthGuardPacket, MinWidthGuardStatus, MinWidthGuardSupportExport,
    M5_MIN_WIDTH_GUARDS_PACKET_RECORD_KIND, M5_MIN_WIDTH_GUARDS_PUBLISHED_REPORT_REF,
    M5_MIN_WIDTH_GUARDS_SHARED_CONTRACT_REF, REQUIRED_FAMILIES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ui/m5-min-width-guards")
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
    let on_disk: MinWidthGuardPacket = load_json("packet.json");
    let seeded = seeded_m5_min_width_guards_packet();
    assert_eq!(
        on_disk, seeded,
        "fixture packet diverged from seeded packet"
    );
    assert_eq!(seeded.record_kind, M5_MIN_WIDTH_GUARDS_PACKET_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_MIN_WIDTH_GUARDS_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_MIN_WIDTH_GUARDS_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_packet_passes_validation_and_is_clean() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    validate_m5_min_width_guards_packet(&packet).expect("fixture packet must validate");
    assert!(packet.report_clean);
    assert!(packet.blocking_findings.is_empty());
    assert_eq!(packet.red_row_count, 0);
    assert!(packet.all_rows_publishable);
}

#[test]
fn fixture_packet_covers_every_family() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    assert_eq!(packet.rows.len(), REQUIRED_FAMILIES.len());
    for family in REQUIRED_FAMILIES {
        assert!(
            packet.row(family).is_some(),
            "packet omits {}",
            family.as_str()
        );
    }
}

#[test]
fn fixture_status_is_the_derived_auto_narrowed_value() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    for row in &packet.rows {
        assert_eq!(
            row.derived_status,
            row.recompute_status(),
            "row {} declares a stale status",
            row.family.as_str()
        );
        assert_eq!(row.guard_causes, row.recompute_causes());
        if !matches!(row.derived_status, MinWidthGuardStatus::Green) {
            assert!(
                !row.narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed row {} hides its reason",
                row.family.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_row_strategy_set_and_plans_are_valid() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    for row in &packet.rows {
        assert!(
            row.strategy_set_is_ordered() && row.strategy_set_has_safe_terminal(),
            "row {} has an invalid strategy set",
            row.family.as_str()
        );
        assert!(
            !row.min_size_below_floor(),
            "row {} declares a minimum below its floor",
            row.family.as_str()
        );
        assert!(
            row.plans_cover_declared_classes(),
            "row {} does not cover its declared classes",
            row.family.as_str()
        );
        assert!(
            row.plans_strategies_declared() && row.plans_monotonic(),
            "row {} has an invalid plan",
            row.family.as_str()
        );
    }
}

#[test]
fn fixture_dashboard_matches_packet_projection() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    let on_disk: MinWidthGuardDashboard = load_json("dashboard.json");
    assert_eq!(
        on_disk,
        packet.dashboard(),
        "dashboard diverged from projection"
    );
}

#[test]
fn fixture_support_export_quotes_packet_and_waiver_refs() {
    let packet: MinWidthGuardPacket = load_json("packet.json");
    let export: MinWidthGuardSupportExport = load_json("support_export.json");
    let expected =
        MinWidthGuardSupportExport::from_packet(export.support_export_id.clone(), packet.clone());
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
    let seeded = seeded_m5_min_width_guards_packet();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let packet = seeded_m5_min_width_guards_packet();
    let rendered = packet.render_markdown();
    let on_disk =
        std::fs::read_to_string(repo_root().join("artifacts/shell/m5-min-width-guards.md"))
            .expect("published m5-min-width-guards.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published markdown diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- markdown`",
    );
}

#[test]
fn published_packet_json_matches_seed() {
    let packet = seeded_m5_min_width_guards_packet();
    let rendered = packet.export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-min-width-guards-proof/packet.json"),
    )
    .expect("published packet.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published packet diverged from seed -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- packet`",
    );
}

#[test]
fn published_dashboard_json_matches_seeded_projection() {
    let packet = seeded_m5_min_width_guards_packet();
    let rendered = packet.dashboard().export_safe_json();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-min-width-guards-proof/dashboard.json"),
    )
    .expect("published dashboard.json must exist");
    assert_eq!(
        on_disk.trim_end(),
        rendered.trim_end(),
        "published dashboard diverged from seeded projection -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- dashboard`",
    );
}

#[test]
fn published_csv_matches_seeded_rendering() {
    let packet = seeded_m5_min_width_guards_packet();
    let rendered = packet.render_matrix_csv();
    let on_disk = std::fs::read_to_string(
        repo_root().join("artifacts/release/m5-min-width-guards-proof/matrix.csv"),
    )
    .expect("published matrix.csv must exist");
    assert_eq!(
        on_disk, rendered,
        "published CSV diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- csv`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_families() {
    let body =
        std::fs::read_to_string(repo_root().join("docs/shell/m5_min_width_guards_contract.md"))
            .expect("published min-width-guards contract must exist");
    assert!(body.contains("artifacts/shell/m5-min-width-guards.md"));
    assert!(body.contains("artifacts/release/m5-min-width-guards-proof/packet.json"));
    assert!(body.contains("artifacts/release/m5-min-width-guards-proof/dashboard.json"));
    assert!(body.contains("fixtures/ui/m5-min-width-guards/packet.json"));
    assert!(body.contains("schemas/shell/m5-min-width-guards.schema.json"));
    for family in REQUIRED_FAMILIES {
        assert!(
            body.contains(family.as_str()),
            "doc must name family {}",
            family.as_str()
        );
    }
}
