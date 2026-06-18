//! Protected fixture checks for the M5 live-appearance change & evidence-linkage
//! report.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ux/m5/os-appearance-contrast-accent/` through the Rust types and
//! asserts the contract invariants. The report fixture is also asserted
//! bit-for-bit equal to the report minted by
//! `seeded_live_appearance_evidence_report`, and the markdown artifact under
//! `artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! remains the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_shell::live_appearance_evidence::{
    seeded_live_appearance_evidence_report, validate_live_appearance_evidence_report,
    EvidenceCaptureKind, LiveAppearanceEvidenceReport, LiveAppearanceEvidenceSupportExport,
    M5_LIVE_APPEARANCE_PUBLISHED_REPORT_REF, M5_LIVE_APPEARANCE_REPORT_RECORD_KIND,
    M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF, REQUIRED_SURFACE_FAMILIES,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/os-appearance-contrast-accent")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/ux/m5/live-appearance-platform-labs")
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
    let on_disk: LiveAppearanceEvidenceReport = load_json("report.json");
    let seeded = seeded_live_appearance_evidence_report();
    assert_eq!(
        on_disk, seeded,
        "fixture report diverged from seeded report"
    );
    assert_eq!(seeded.record_kind, M5_LIVE_APPEARANCE_REPORT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        M5_LIVE_APPEARANCE_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.published_report_ref,
        M5_LIVE_APPEARANCE_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_report_passes_validation_and_is_clean() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    validate_live_appearance_evidence_report(&report).expect("fixture report must validate");
    assert!(report.report_clean);
    assert!(report.blocking_findings.is_empty());
}

#[test]
fn fixture_every_capture_is_attributable_to_the_exact_build() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    assert!(report.all_captures_build_attributed);
    for row in &report.rows {
        let Some(evidence) = &row.evidence else {
            continue;
        };
        let a = &evidence.attribution;
        assert_eq!(
            a.build_identity_ref, report.build_identity_ref,
            "row {} capture is not attributed to the report build",
            row.row_id
        );
        assert_eq!(a.theme_package_ref, row.theme_package_ref);
        assert_eq!(a.appearance_session_ref, row.appearance_session_ref);
        assert_eq!(a.checkpoint_ref, row.checkpoint_ref);
        assert_eq!(a.platform, row.platform);
        assert_eq!(a.os_signal, row.os_signal);
        assert!(evidence.golden_match.is_attributable());
    }
}

#[test]
fn fixture_axis_signal_and_posture_disclosure_hold() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    for row in &report.rows {
        assert_eq!(
            row.changed_axis,
            row.os_signal.canonical_axis(),
            "row {} axis disagrees with its OS signal",
            row.row_id
        );
        if row.posture_needs_reload_or_restart() {
            assert!(
                row.restart_or_reload_disclosed,
                "row {} hides its reload/restart posture",
                row.row_id
            );
        }
    }
}

#[test]
fn fixture_marketed_axes_are_cross_platform() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    for coverage in &report.axis_platform_coverage {
        assert!(
            coverage.platforms.len() >= 2,
            "axis {} is proven on only {} platform(s)",
            coverage.axis.as_str(),
            coverage.platforms.len()
        );
    }
}

#[test]
fn fixture_required_surface_families_are_covered() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    for family in REQUIRED_SURFACE_FAMILIES {
        assert!(
            report
                .covered_surface_families
                .iter()
                .any(|s| s == family.as_str()),
            "surface family {} is uncovered",
            family.as_str()
        );
    }
}

#[test]
fn fixture_qualified_rows_carry_live_transition_evidence_and_intact_cues() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    for row in &report.rows {
        if !row.is_qualified() {
            continue;
        }
        let evidence = row
            .evidence
            .as_ref()
            .expect("qualified row carries evidence");
        if row.apply_posture.applies_live() {
            assert_eq!(
                evidence.capture_kind,
                EvidenceCaptureKind::LiveTransition,
                "live-applying row {} must carry a live-transition capture",
                row.row_id
            );
        }
        let cues = row.cue_preservation.expect("qualified row carries cues");
        assert!(
            cues.structurally_intact(),
            "row {} loses focus, semantics, or layout",
            row.row_id
        );
    }
}

#[test]
fn fixture_support_export_quotes_build_session_and_capture_refs() {
    let report: LiveAppearanceEvidenceReport = load_json("report.json");
    let export: LiveAppearanceEvidenceSupportExport = load_json("support_export.json");
    let expected = LiveAppearanceEvidenceSupportExport::from_report(
        export.support_export_id.clone(),
        report.clone(),
    );
    assert_eq!(export, expected);
    assert!(export.case_ids.contains(&report.report_id));
    assert!(export.case_ids.contains(&report.build_identity_ref));
    for row in &report.rows {
        assert!(export.case_ids.contains(&row.row_id));
        assert!(export.case_ids.contains(&row.appearance_session_ref));
        assert!(export.case_ids.contains(&row.checkpoint_ref));
        if let Some(evidence) = &row.evidence {
            assert!(export.case_ids.contains(&evidence.screenshot_ref));
            assert!(export.case_ids.contains(&evidence.golden_baseline_ref));
        }
    }
}

#[test]
fn published_report_md_matches_seeded_rendering() {
    let report = seeded_live_appearance_evidence_report();
    let rendered = report.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("m5_live_appearance_evidence.md"))
        .expect("published m5_live_appearance_evidence.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published m5_live_appearance_evidence.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- markdown`",
    );
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let seeded = seeded_live_appearance_evidence_report();
    let mut rendered = seeded.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-shell --bin aureline_shell_m5_live_appearance_evidence -- compact`",
    );
}

#[test]
fn published_doc_links_artifacts_and_quotes_rows() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/live-appearance-and-evidence-linkage.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published live-appearance-and-evidence-linkage doc must exist");
    assert!(body
        .contains("artifacts/ux/m5/live-appearance-platform-labs/m5_live_appearance_evidence.md"));
    assert!(body.contains("fixtures/ux/m5/os-appearance-contrast-accent/report.json"));
    assert!(body.contains("schemas/ux/m5-live-appearance-evidence.schema.json"));
    assert!(body.contains("tools/ci/m5/live_appearance_evidence_check.py"));
    let report = seeded_live_appearance_evidence_report();
    for row in &report.rows {
        for doc_ref in &row.docs_help_refs {
            if let Some(anchor) =
                doc_ref.strip_prefix("docs/m5/live-appearance-and-evidence-linkage.md#")
            {
                assert!(
                    body.contains(&format!("{{#{anchor}}}")),
                    "doc must define anchor #{anchor} cited by row {}",
                    row.row_id
                );
            }
        }
    }
}
