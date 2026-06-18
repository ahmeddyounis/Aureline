//! Protected fixture checks for the M5 imported-theme mapping & rollback report.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ux/m5/theme-import-corpus/` through the Rust types and asserts the
//! contract invariants. The report fixture is also asserted bit-for-bit equal to
//! the report minted by `seeded_theme_import_report`, and the markdown artifact
//! under `artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md` is
//! asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::theme_import_reports::{
    seeded_theme_import_report, validate_theme_import_report, ImportOutcomeState, ParityClaimState,
    ThemeImportReport, ThemeImportSupportExport, M5_THEME_IMPORT_PUBLISHED_REPORT_REF,
    M5_THEME_IMPORT_REPORT_RECORD_KIND, M5_THEME_IMPORT_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/theme-import-corpus")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/theme-import-reports")
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
    let on_disk: ThemeImportReport = load_json("report.json");
    let seeded = seeded_theme_import_report();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded report"
    );
    assert_eq!(seeded.record_kind, M5_THEME_IMPORT_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_THEME_IMPORT_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_THEME_IMPORT_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation_and_is_clean() {
    let report: ThemeImportReport = load_json("report.json");
    validate_theme_import_report(&report).expect("fixture report must validate");
    assert!(report.is_clean());
}

#[test]
fn fixture_rows_carry_provenance_unresolved_counts_and_rollback() {
    let report: ThemeImportReport = load_json("report.json");
    for row in &report.rows {
        assert!(
            !row.source_tool.source_tool_name.trim().is_empty(),
            "row {} must name its source tool",
            row.row_id
        );
        assert!(
            !row.source_tool.source_theme_identifier.trim().is_empty(),
            "row {} must carry source provenance",
            row.row_id
        );
        assert!(
            !row.parity_note.trim().is_empty(),
            "row {} must carry a parity note",
            row.row_id
        );
        assert!(
            row.import_is_reversible(),
            "row {} must be reversible",
            row.row_id
        );
        if row.unresolved_slot_count() > 0 {
            assert!(
                !row.unresolved_slots.is_empty(),
                "row {} hides its unresolved slots",
                row.row_id
            );
        }
    }
}

#[test]
fn fixture_report_never_overclaims_parity() {
    let report: ThemeImportReport = load_json("report.json");
    for row in &report.rows {
        if row.parity_claim_state == ParityClaimState::ClaimedWithReport {
            assert!(
                row.full_parity_is_backed(),
                "row {} claims parity it does not back",
                row.row_id
            );
        }
    }
}

#[test]
fn fixture_report_covers_the_honesty_spectrum() {
    let report: ThemeImportReport = load_json("report.json");
    let outcomes: Vec<ImportOutcomeState> =
        report.rows.iter().map(|row| row.import_outcome).collect();
    for expected in [
        ImportOutcomeState::Applied,
        ImportOutcomeState::AppliedWithWarnings,
        ImportOutcomeState::RolledBack,
        ImportOutcomeState::ReviewRequired,
        ImportOutcomeState::Blocked,
    ] {
        assert!(
            outcomes.contains(&expected),
            "fixture must cover outcome {}",
            expected.as_str()
        );
    }
    // A rolled-back import proves the rollback path exists for incompatible
    // imports.
    assert!(report
        .rows
        .iter()
        .any(|row| row.import_outcome == ImportOutcomeState::RolledBack));
}

#[test]
fn fixture_support_export_quotes_report_provenance_and_rollback() {
    let report: ThemeImportReport = load_json("report.json");
    let export: ThemeImportSupportExport = load_json("support_export.json");
    let expected =
        ThemeImportSupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for row in &report.rows {
        assert!(
            export.case_ids.contains(&row.row_id),
            "support export must quote row id {}",
            row.row_id
        );
        assert!(
            export
                .case_ids
                .contains(&row.source_tool.source_theme_identifier),
            "support export must quote provenance for {}",
            row.row_id
        );
        assert!(
            export.case_ids.contains(&row.rollback.rollback_ref),
            "support export must quote rollback ref for {}",
            row.row_id
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_theme_import_report();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5_theme_import_report.md"))
        .expect("published m5_theme_import_report.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_theme_import_report.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- markdown`",
    );
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_theme_import_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_import_reports -- compact`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_rows() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/theme-import-and-rollback.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published theme-import-and-rollback doc must exist");
    assert!(body.contains("artifacts/ux/m5/theme-import-reports/m5_theme_import_report.md"));
    assert!(body.contains("fixtures/ux/m5/theme-import-corpus/report.json"));
    assert!(body.contains("schemas/ux/m5-theme-import-report.schema.json"));
    assert!(body.contains("tools/ci/m5/theme_import_report_check.py"));
    let report = seeded_theme_import_report();
    for row in &report.rows {
        for doc_ref in &row.docs_help_refs {
            if let Some(anchor) = doc_ref.strip_prefix("docs/m5/theme-import-and-rollback.md#") {
                assert!(
                    body.contains(&format!("{{#{anchor}}}")),
                    "doc must define anchor #{anchor} cited by row {}",
                    row.row_id
                );
            }
        }
    }
}
