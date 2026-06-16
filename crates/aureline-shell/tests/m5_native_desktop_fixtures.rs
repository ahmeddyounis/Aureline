//! Protected fixture checks for the native-desktop matrix.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/platform/m5_os_entry_and_reopen/` through the Rust types and
//! asserts the contract invariants. The report fixture is also asserted
//! bit-for-bit equal to the matrix minted by `seeded_native_desktop_matrix`,
//! and the markdown artifact under
//! `artifacts/platform/m5-native-desktop-matrix.md` is asserted bit-for-bit
//! equal to the rendering, so the headless inspector remains the only
//! mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::m5_native_desktop::{
    seeded_native_desktop_matrix, validate_native_desktop_matrix, NativeDesktopControl,
    NativeDesktopControlStatus, NativeDesktopMatrixReport, NativeDesktopSupportExport,
    NativeDesktopSurfaceKind, NATIVE_DESKTOP_PUBLISHED_REPORT_REF,
    NATIVE_DESKTOP_REPORT_RECORD_KIND, NATIVE_DESKTOP_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/platform/m5_os_entry_and_reopen")
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
    let on_disk: NativeDesktopMatrixReport = load_json("report.json");
    let seeded = seeded_native_desktop_matrix();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded matrix"
    );
    assert_eq!(seeded.record_kind, NATIVE_DESKTOP_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        NATIVE_DESKTOP_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        NATIVE_DESKTOP_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: NativeDesktopMatrixReport = load_json("report.json");
    validate_native_desktop_matrix(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_report_covers_every_kind_and_control() {
    let report: NativeDesktopMatrixReport = load_json("report.json");
    assert!(report.every_kind_present());
    assert!(report.every_control_satisfied());
    for kind in NativeDesktopSurfaceKind::required_kinds() {
        assert!(
            report
                .entries
                .iter()
                .any(|entry| entry.descriptor.surface_kind == kind),
            "no registered surface for required kind {}",
            kind.as_str()
        );
    }
    for control in NativeDesktopControl::required_controls() {
        let any_satisfied = report.entries.iter().any(|entry| {
            entry.bindings.iter().any(|binding| {
                binding.control == control
                    && binding.status == NativeDesktopControlStatus::Satisfied
            })
        });
        assert!(
            any_satisfied,
            "no satisfied surface for required control {}",
            control.as_str()
        );
    }
}

#[test]
fn fixture_reopen_anchor_index_is_complete_and_non_empty() {
    let report: NativeDesktopMatrixReport = load_json("report.json");
    assert_eq!(report.reopen_anchor_index.len(), report.entries.len());
    for entry in &report.reopen_anchor_index {
        assert!(
            !entry.reopen_anchor_ref.trim().is_empty(),
            "reopen anchor for {} must be non-empty",
            entry.entry_id
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: NativeDesktopMatrixReport = load_json("report.json");
    let export: NativeDesktopSupportExport = load_json("support_export.json");
    let expected =
        NativeDesktopSupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for entry in &report.entries {
        assert!(
            export.case_ids.contains(&entry.descriptor.entry_id),
            "support export must quote entry id {}",
            entry.descriptor.entry_id
        );
        assert!(
            export
                .case_ids
                .contains(&entry.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            entry.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn published_matrix_md_matches_seeded_rendering() {
    let report = seeded_native_desktop_matrix();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5-native-desktop-matrix.md"))
        .expect("published m5-native-desktop-matrix.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5-native-desktop-matrix.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- report-md`",
    );
}

#[test]
fn published_doc_links_every_kind_control_and_artifact() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/native-desktop-integration-and-reopen.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published native-desktop-integration-and-reopen doc must exist");
    for kind in NativeDesktopSurfaceKind::required_kinds() {
        assert!(
            body.contains(kind.as_str()),
            "doc must quote required kind {}",
            kind.as_str()
        );
    }
    for control in NativeDesktopControl::required_controls() {
        assert!(
            body.contains(control.as_str()),
            "doc must quote required control {}",
            control.as_str()
        );
    }
    assert!(body.contains("artifacts/platform/m5-native-desktop-matrix.md"));
    assert!(body.contains("fixtures/platform/m5_os_entry_and_reopen/report.json"));
    assert!(body.contains("schemas/platform/m5-native-desktop-matrix.schema.json"));
    assert!(body.contains("tools/ci/m5/native_desktop_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_native_desktop_matrix();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_native_desktop -- compact`",
    );
}
