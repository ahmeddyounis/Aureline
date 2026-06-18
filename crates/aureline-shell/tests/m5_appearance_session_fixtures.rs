//! Protected fixture checks for the M5 appearance-session runtime audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5/live-appearance-change/` through the Rust types and asserts
//! the contract invariants. The report fixture is asserted bit-for-bit equal
//! to the audit minted by `seeded_appearance_session_runtime`, and the markdown
//! artifact under
//! `artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::appearance_session::{
    seeded_appearance_session_runtime, validate_appearance_session_runtime,
    AppearanceSessionRuntimeReport, AppearanceSessionSupportExport,
    APPEARANCE_SESSION_PUBLISHED_REPORT_REF, APPEARANCE_SESSION_REPORT_RECORD_KIND,
    APPEARANCE_SESSION_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/live-appearance-change")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/ux/m5/appearance-session-checkpoints")
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
    let on_disk: AppearanceSessionRuntimeReport = load_json("report.json");
    let seeded = seeded_appearance_session_runtime();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded runtime"
    );
    assert_eq!(seeded.record_kind, APPEARANCE_SESSION_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        APPEARANCE_SESSION_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        APPEARANCE_SESSION_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: AppearanceSessionRuntimeReport = load_json("report.json");
    validate_appearance_session_runtime(&report).expect("fixture report must validate");
    assert!(report.report_clean);
    assert!(report.blocking_findings.is_empty());
}

#[test]
fn fixture_every_transition_resolves_its_checkpoint() {
    let report: AppearanceSessionRuntimeReport = load_json("report.json");
    assert!(report.every_transition_checkpoint_resolved());
    for transition in &report.transitions {
        assert!(
            report.checkpoint(&transition.checkpoint_ref).is_some(),
            "transition {} names an unknown checkpoint {}",
            transition.transition_ref,
            transition.checkpoint_ref
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: AppearanceSessionRuntimeReport = load_json("report.json");
    let export: AppearanceSessionSupportExport = load_json("support_export.json");
    let expected = AppearanceSessionSupportExport::from_report(
        export.support_export_id.clone(),
        report.clone(),
    );
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.session.session_ref));
    for checkpoint in &report.checkpoints {
        assert!(
            export.case_ids.contains(&checkpoint.checkpoint_ref),
            "support export must quote checkpoint {}",
            checkpoint.checkpoint_ref
        );
    }
    for surface in &report.surfaces {
        assert!(
            export.case_ids.contains(&surface.surface_id),
            "support export must quote surface {}",
            surface.surface_id
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_appearance_session_runtime();
    let rendered = report.render_markdown();
    let on_disk =
        std::fs::read_to_string(artifacts_root().join("m5_appearance_session_runtime_audit.md"))
            .expect("published m5_appearance_session_runtime_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_appearance_session_runtime_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- report-md`",
    );
}

#[test]
fn published_doc_links_artifacts_and_gate() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/appearance-session-runtime.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published appearance-session-runtime doc must exist");
    assert!(body.contains(
        "artifacts/ux/m5/appearance-session-checkpoints/m5_appearance_session_runtime_audit.md"
    ));
    assert!(body.contains("fixtures/ux/m5/live-appearance-change/report.json"));
    assert!(body.contains("schemas/ux/appearance-session.schema.json"));
    assert!(body.contains("schemas/ux/appearance_checkpoint.schema.json"));
    assert!(body.contains("tools/ci/m5/appearance_session_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_appearance_session_runtime();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_session -- compact`",
    );
}
