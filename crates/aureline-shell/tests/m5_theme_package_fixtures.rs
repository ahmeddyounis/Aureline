//! Protected fixture checks for the M5 theme-package manifest audit.
//!
//! The integration test replays every JSON fixture under
//! `fixtures/ux/m5/theme-package-modes/` through the Rust types and asserts
//! the contract invariants. The report fixture is asserted bit-for-bit equal
//! to the audit minted by `seeded_theme_package_manifest_audit`, and the
//! markdown artifact under
//! `artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::theme_packages::{
    seeded_theme_package_manifest_audit, validate_theme_package_manifests,
    ThemePackageManifestReport, ThemePackageSupportExport, THEME_PACKAGE_PUBLISHED_REPORT_REF,
    THEME_PACKAGE_REPORT_RECORD_KIND, THEME_PACKAGE_SHARED_CONTRACT_REF,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/theme-package-modes")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/theme-manifest-audit")
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
    let on_disk: ThemePackageManifestReport = load_json("report.json");
    let seeded = seeded_theme_package_manifest_audit();
    assert_eq!(on_disk, seeded, "fixture report diverged from seeded audit");
    assert_eq!(seeded.record_kind, THEME_PACKAGE_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        THEME_PACKAGE_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        THEME_PACKAGE_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation() {
    let report: ThemePackageManifestReport = load_json("report.json");
    validate_theme_package_manifests(&report).expect("fixture report must validate");
    assert!(report.report_clean);
}

#[test]
fn fixture_every_surface_resolves_its_active_package() {
    let report: ThemePackageManifestReport = load_json("report.json");
    assert!(report.every_surface_package_resolved());
    for surface in &report.surfaces {
        assert!(
            report.manifest(&surface.active_package_id).is_some(),
            "surface {} names an unknown package {}",
            surface.descriptor.surface_id,
            surface.active_package_id
        );
    }
}

#[test]
fn fixture_support_export_quotes_report_and_case_ids() {
    let report: ThemePackageManifestReport = load_json("report.json");
    let export: ThemePackageSupportExport = load_json("support_export.json");
    let expected =
        ThemePackageSupportExport::from_report(export.support_export_id.clone(), report.clone());
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    for manifest in &report.manifests {
        assert!(
            export.case_ids.contains(&manifest.package_id),
            "support export must quote package id {}",
            manifest.package_id
        );
    }
    for surface in &report.surfaces {
        assert!(
            export.case_ids.contains(&surface.descriptor.surface_id),
            "support export must quote surface id {}",
            surface.descriptor.surface_id
        );
        assert!(
            export
                .case_ids
                .contains(&surface.descriptor.descriptor_revision_ref),
            "support export must quote descriptor revision {}",
            surface.descriptor.descriptor_revision_ref
        );
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_theme_package_manifest_audit();
    let rendered = report.render_markdown();
    let on_disk =
        std::fs::read_to_string(artifacts_root().join("m5_theme_package_manifest_audit.md"))
            .expect("published m5_theme_package_manifest_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_theme_package_manifest_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- report-md`",
    );
}

#[test]
fn published_doc_links_artifacts_and_gate() {
    let doc_path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/m5/theme-package-manifests.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published theme-package-manifests doc must exist");
    assert!(
        body.contains("artifacts/ux/m5/theme-manifest-audit/m5_theme_package_manifest_audit.md")
    );
    assert!(body.contains("fixtures/ux/m5/theme-package-modes/report.json"));
    assert!(body.contains("schemas/ux/m5-theme-package-manifest.schema.json"));
    assert!(body.contains("schemas/ux/theme_package_manifest.schema.json"));
    assert!(body.contains("tools/ci/m5/theme_package_manifest_check.py"));
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_theme_package_manifest_audit();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_theme_packages -- compact`",
    );
}
