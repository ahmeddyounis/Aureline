//! Protected fixture checks for the open/save/reveal path-truth report.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/platform/m5-open-save-reveal/` through the Rust types and asserts
//! the contract invariants. The report fixture is also asserted bit-for-bit
//! equal to the report minted by `seeded_open_save_reveal_report`, the
//! per-incident case exports are asserted equal to
//! `seeded_open_save_reveal_case_exports`, and the markdown artifact under
//! `artifacts/platform/m5-open-save-reveal.md` is asserted bit-for-bit equal to
//! the rendering, so the headless inspector remains the only mint-from-truth
//! path.

use std::path::{Path, PathBuf};

use aureline_workspace::m5_open_save_reveal::{
    seeded_open_save_reveal_case_exports, seeded_open_save_reveal_report,
    validate_open_save_reveal_report, DialogFlowKind, OpenSaveRevealCaseExport,
    OpenSaveRevealReport, OpenSaveRevealSupportExport, OPEN_SAVE_REVEAL_PUBLISHED_REPORT_REF,
    OPEN_SAVE_REVEAL_REPORT_RECORD_KIND, OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/platform/m5-open-save-reveal")
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
    let on_disk: OpenSaveRevealReport = load_json("report.json");
    let seeded = seeded_open_save_reveal_report();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded report"
    );
    assert_eq!(seeded.record_kind, OPEN_SAVE_REVEAL_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        OPEN_SAVE_REVEAL_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        OPEN_SAVE_REVEAL_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: OpenSaveRevealReport = load_json("report.json");
    validate_open_save_reveal_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_covers_every_kind() {
    let report: OpenSaveRevealReport = load_json("report.json");
    assert!(report.every_kind_present());
    for kind in DialogFlowKind::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.flow_kind == kind),
            "no registered flow for required kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn fixture_support_export_matches_seed() {
    let on_disk: OpenSaveRevealSupportExport = load_json("support_export.json");
    let seeded = OpenSaveRevealSupportExport::from_report(
        &on_disk.support_export_id,
        seeded_open_save_reveal_report(),
    );
    assert_eq!(
        on_disk, seeded,
        "fixture support_export.json diverged from the seeded wrapper"
    );
    let report = seeded_open_save_reveal_report();
    for entry in &report.entries {
        assert!(on_disk.case_ids.contains(&entry.descriptor.flow_id));
        assert!(on_disk
            .case_ids
            .contains(&entry.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn fixture_case_exports_match_seed() {
    let seeded = seeded_open_save_reveal_case_exports();
    for export in &seeded {
        let on_disk: OpenSaveRevealCaseExport =
            load_json(&format!("cases/{}.json", export.case_label));
        assert_eq!(
            &on_disk, export,
            "fixture case {} diverged from the seeded export",
            export.case_label
        );
    }
    assert_eq!(
        seeded.len(),
        4,
        "the four required incident cases must exist"
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_open_save_reveal_report();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5-open-save-reveal.md"))
        .expect("published m5-open-save-reveal.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5-open-save-reveal.md diverged from seeded rendering -- \
         regenerate with `cargo run -q -p aureline-workspace --bin \
         aureline_workspace_m5_open_save_reveal -- report-md`",
    );
}

#[test]
fn published_doc_links_every_kind_and_artifact() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/open-save-reveal-path-truth.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published open-save-reveal-path-truth doc must exist");
    for kind in DialogFlowKind::required_kinds() {
        assert!(
            body.contains(kind.as_str()),
            "doc must quote required flow kind {}",
            kind.as_str()
        );
    }
    assert!(body.contains("artifacts/platform/m5-open-save-reveal.md"));
    assert!(body.contains("fixtures/platform/m5-open-save-reveal/report.json"));
    assert!(body.contains("schemas/platform/m5-path-boundary.schema.json"));
    assert!(body.contains("tools/ci/m5/open_save_reveal_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_open_save_reveal_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-workspace --bin aureline_workspace_m5_open_save_reveal -- compact`",
    );
}
