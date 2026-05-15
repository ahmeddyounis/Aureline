//! Protected fixture checks for the beta command-parity diff report.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/commands/m3/command_parity/` through the Rust types and
//! asserts the contract invariants. The report fixture is also
//! asserted bit-for-bit equal to the report minted by
//! `seeded_command_parity_diff_report`, and the markdown artifact
//! under `artifacts/ux/m3/command_parity_diff_report.md` is asserted
//! bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::command_parity::{
    seeded_command_parity_diff_report, validate_command_parity_diff_report,
    BetaCommandParityDiffReport, BetaCommandParitySupportExport, BetaSurfaceFamily,
    COMMAND_PARITY_PUBLISHED_REPORT_REF, COMMAND_PARITY_REPORT_RECORD_KIND,
    COMMAND_PARITY_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/commands/m3/command_parity")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m3")
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
    let on_disk: BetaCommandParityDiffReport = load_json("report.json");
    let seeded = seeded_command_parity_diff_report();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded report");
    assert_eq!(seeded.record_kind, COMMAND_PARITY_REPORT_RECORD_KIND);
    assert_eq!(seeded.shared_contract_ref, COMMAND_PARITY_SHARED_CONTRACT_REF);
    assert_eq!(seeded.published_report_ref, COMMAND_PARITY_PUBLISHED_REPORT_REF);
}

#[test]
fn fixture_report_passes_validation() {
    let report: BetaCommandParityDiffReport = load_json("report.json");
    validate_command_parity_diff_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_claims_every_required_surface() {
    let report: BetaCommandParityDiffReport = load_json("report.json");
    assert!(report.every_required_surface_claimed());
    for required in BetaSurfaceFamily::required_surfaces() {
        let any_claimed = report.rows.iter().any(|row| {
            row.surfaces.iter().any(|projection| {
                projection.surface_family == required
                    && projection.coverage_status
                        == aureline_shell::command_parity::BetaCoverageStatus::Claimed
            })
        });
        assert!(
            any_claimed,
            "no claimed row for required surface {}",
            required.as_str()
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: BetaCommandParityDiffReport = load_json("report.json");
    let export: BetaCommandParitySupportExport = load_json("support_export.json");
    let expected = BetaCommandParitySupportExport::from_report(
        export.support_export_id.clone(),
        report.clone(),
    );
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(
            export.case_ids.contains(&row.descriptor.command_id),
            "support export must quote command id {}",
            row.descriptor.command_id
        );
        assert!(
            export.case_ids.contains(&row.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            row.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_command_parity_diff_report();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("command_parity_diff_report.md"))
        .expect("published command_parity_diff_report.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published command_parity_diff_report.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- report-md`",
    );
}

#[test]
fn published_doc_links_every_required_surface_and_artifact() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/ux/m3/command_parity_diff_report.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published command_parity_diff_report doc must exist");
    for required in BetaSurfaceFamily::required_surfaces() {
        assert!(
            body.contains(required.as_str()),
            "doc must quote required surface {}",
            required.as_str()
        );
    }
    assert!(body.contains("/artifacts/ux/m3/command_parity_diff_report.md"));
    assert!(body.contains("/fixtures/commands/m3/command_parity/report.json"));
    assert!(body.contains("/schemas/commands/command_parity.schema.json"));
    assert!(body.contains("tools/ci/m3/command_parity_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path)
        .expect("compact fixture must exist");
    let seeded = seeded_command_parity_diff_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_command_parity -- compact`",
    );
}
