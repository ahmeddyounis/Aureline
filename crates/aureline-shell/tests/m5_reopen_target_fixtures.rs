//! Protected fixture checks for the reopen-target report.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/platform/m5-reopen-targets/` through the Rust types and asserts
//! the contract invariants. The report fixture is also asserted bit-for-bit
//! equal to the report minted by `seeded_reopen_target_report`, the
//! per-incident case exports are asserted equal to
//! `seeded_reopen_target_case_exports`, and the markdown artifact under
//! `artifacts/platform/m5-recent-item-and-reopen.md` is asserted bit-for-bit
//! equal to the rendering, so the headless inspector remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_recent_items_and_reopen::{
    seeded_reopen_target_case_exports, seeded_reopen_target_report,
    validate_reopen_target_report, ReopenAvailability, ReopenSurfaceKind, ReopenTargetCaseExport,
    ReopenTargetReport, ReopenTargetSupportExport, REOPEN_TARGET_PUBLISHED_REPORT_REF,
    REOPEN_TARGET_REPORT_RECORD_KIND, REOPEN_TARGET_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/platform/m5-reopen-targets")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/platform")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_report_is_bit_for_bit_equal_to_seed() {
    let on_disk: ReopenTargetReport = load_json("report.json");
    let seeded = seeded_reopen_target_report();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded report");
    assert_eq!(seeded.record_kind, REOPEN_TARGET_REPORT_RECORD_KIND);
    assert_eq!(seeded.shared_contract_ref, REOPEN_TARGET_SHARED_CONTRACT_REF);
    assert_eq!(seeded.published_report_ref, REOPEN_TARGET_PUBLISHED_REPORT_REF);
}

#[test]
fn fixture_report_passes_validation() {
    let report: ReopenTargetReport = load_json("report.json");
    validate_reopen_target_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_covers_every_surface_and_degraded_class() {
    let report: ReopenTargetReport = load_json("report.json");
    assert!(report.every_surface_present());
    assert!(report.every_degraded_class_present());
    for surface in ReopenSurfaceKind::required_surfaces() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.surface_kind == surface),
            "no registered target for required surface {}",
            surface.as_str()
        );
    }
    for availability in ReopenAvailability::degraded_classes() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.availability == availability),
            "no registered target for degraded class {}",
            availability.as_str()
        );
    }
}

#[test]
fn fixture_support_export_matches_seed() {
    let on_disk: ReopenTargetSupportExport = load_json("support_export.json");
    let seeded = ReopenTargetSupportExport::from_report(
        &on_disk.support_export_id,
        seeded_reopen_target_report(),
    );
    assert_eq!(
        on_disk, seeded,
        "fixture support_export.json diverged from the seeded wrapper"
    );
    let report = seeded_reopen_target_report();
    for entry in &report.entries {
        assert!(on_disk.case_ids.contains(&entry.descriptor.reopen_target_id));
        assert!(on_disk
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn fixture_case_exports_match_seed() {
    let seeded = seeded_reopen_target_case_exports();
    for export in &seeded {
        let on_disk: ReopenTargetCaseExport =
            load_json(&format!("cases/{}.json", export.case_label));
        assert_eq!(
            &on_disk, export,
            "fixture case {} diverged from the seeded export",
            export.case_label
        );
    }
    assert_eq!(
        seeded.len(),
        5,
        "the five required reopen incident cases must exist"
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_reopen_target_report();
    let rendered = report.render_markdown();
    let on_disk =
        std::fs::read_to_string(artifacts_root().join("m5-recent-item-and-reopen.md"))
            .expect("published m5-recent-item-and-reopen.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5-recent-item-and-reopen.md diverged from seeded rendering -- \
         regenerate with `cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- \
         report-md`",
    );
}

#[test]
fn published_doc_links_every_surface_and_artifact() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/recent-items-dock-taskbar-jump-list.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published recent-items-dock-taskbar-jump-list doc must exist");
    for surface in ReopenSurfaceKind::required_surfaces() {
        assert!(
            body.contains(surface.as_str()),
            "doc must quote required reopen surface {}",
            surface.as_str()
        );
    }
    assert!(body.contains("artifacts/platform/m5-recent-item-and-reopen.md"));
    assert!(body.contains("fixtures/platform/m5-reopen-targets/report.json"));
    assert!(body.contains("schemas/platform/m5-reopen-target.schema.json"));
    assert!(body.contains("tools/ci/m5/reopen_target_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_reopen_target_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_reopen_target -- compact`",
    );
}
