//! Protected fixture checks for the M5 appearance-object certification capstone.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ux/m5/appearance-object-certification/` through the Rust types and
//! asserts the contract invariants. The report fixture is also asserted
//! bit-for-bit equal to the report minted by
//! `seeded_appearance_object_certification_report`, and the markdown artifact
//! under
//! `artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::appearance_object_certification::{
    seeded_appearance_object_certification_report, validate_appearance_object_certification_report,
    AppearanceObjectCertificationReport, AppearanceObjectCertificationSupportExport,
    AppearanceObjectFamily, CertifiedClaimScope, M5_APPEARANCE_CERT_PUBLISHED_REPORT_REF,
    M5_APPEARANCE_CERT_REPORT_RECORD_KIND, M5_APPEARANCE_CERT_SHARED_CONTRACT_REF,
    REQUIRED_SURFACE_FAMILIES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/ux/m5/appearance-object-certification")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/theme-package-certification")
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
    let on_disk: AppearanceObjectCertificationReport = load_json("report.json");
    let seeded = seeded_appearance_object_certification_report();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded report"
    );
    assert_eq!(seeded.record_kind, M5_APPEARANCE_CERT_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_APPEARANCE_CERT_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_APPEARANCE_CERT_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation_and_is_clean() {
    let report: AppearanceObjectCertificationReport = load_json("report.json");
    validate_appearance_object_certification_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
    assert!(report.blocking_findings.is_empty());
    assert_eq!(report.blocked_surface_count, 0);
    assert!(report.all_surfaces_publishable);
}

#[test]
fn fixture_index_registers_every_family_and_surfaces_back_it() {
    let report: AppearanceObjectCertificationReport = load_json("report.json");
    assert_eq!(
        report.object_model_index.len(),
        AppearanceObjectFamily::ALL.len()
    );
    for family in AppearanceObjectFamily::ALL {
        let entry = report
            .index_entry(family)
            .unwrap_or_else(|| panic!("index must register {}", family.as_str()));
        assert_eq!(entry.source_report_id, family.source_report_id());
    }
    for surface in &report.surfaces {
        for certification in &surface.family_certifications {
            let entry = report
                .index_entry(certification.object_family)
                .expect("index entry exists");
            assert_eq!(
                certification.source_report_id,
                entry.source_report_id,
                "surface {} family {} cites an unbacked report",
                surface.certification_id,
                certification.object_family.as_str()
            );
        }
    }
}

#[test]
fn fixture_every_claimed_surface_is_certified_across_all_five_families() {
    let report: AppearanceObjectCertificationReport = load_json("report.json");
    for surface_family in REQUIRED_SURFACE_FAMILIES {
        let surface = report
            .surfaces
            .iter()
            .find(|surface| surface.surface_family == surface_family)
            .unwrap_or_else(|| panic!("surface {} must be certified", surface_family.as_str()));
        for family in AppearanceObjectFamily::ALL {
            assert!(
                surface.family(family).is_some(),
                "surface {} omits family {}",
                surface_family.as_str(),
                family.as_str()
            );
        }
    }
}

#[test]
fn fixture_claim_scope_is_the_derived_auto_narrowed_value() {
    let report: AppearanceObjectCertificationReport = load_json("report.json");
    for surface in &report.surfaces {
        assert_eq!(
            surface.certified_claim_scope,
            surface.recompute_claim_scope(),
            "surface {} declares a stale claim scope",
            surface.certification_id
        );
        if !matches!(
            surface.certified_claim_scope,
            CertifiedClaimScope::CertifiedFull
        ) {
            assert!(
                !surface
                    .narrowing_reason
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty(),
                "narrowed surface {} hides its reason",
                surface.certification_id
            );
        }
    }
}

#[test]
fn fixture_support_export_quotes_index_surface_and_family_refs() {
    let report: AppearanceObjectCertificationReport = load_json("report.json");
    let export: AppearanceObjectCertificationSupportExport = load_json("support_export.json");
    let expected = AppearanceObjectCertificationSupportExport::from_report(
        export.support_export_id.clone(),
        report.clone(),
    );
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.build_identity_ref));
    for entry in &report.object_model_index {
        assert!(export.case_ids.contains(&entry.source_report_id));
        assert!(export.case_ids.contains(&entry.canonical_schema_ref));
    }
    for surface in &report.surfaces {
        assert!(export.case_ids.contains(&surface.certification_id));
    }
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_appearance_object_certification_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- compact`",
    );
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_appearance_object_certification_report();
    let rendered = report.render_markdown();
    let on_disk =
        std::fs::read_to_string(artifacts_root().join("m5_appearance_object_certification.md"))
            .expect("published m5_appearance_object_certification.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_appearance_object_certification.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_object_certification -- markdown`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_surfaces() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/appearance-object-certification.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published appearance-object-certification doc must exist");
    assert!(body.contains(
        "artifacts/ux/m5/theme-package-certification/m5_appearance_object_certification.md"
    ));
    assert!(body.contains("fixtures/ux/m5/appearance-object-certification/report.json"));
    assert!(body.contains("schemas/ux/m5-appearance-object-certification.schema.json"));
    assert!(body.contains("tools/ci/m5/appearance_object_certification_check.py"));
    let report = seeded_appearance_object_certification_report();
    for family in AppearanceObjectFamily::ALL {
        assert!(
            body.contains(family.as_str()),
            "doc must name family {}",
            family.as_str()
        );
    }
    for surface in &report.surfaces {
        assert!(
            body.contains(surface.surface_family.as_str()),
            "doc must name surface {}",
            surface.surface_family.as_str()
        );
    }
}
