//! Protected tests for the stable Project Doctor probe-pack catalog, finding
//! evaluator, explainability, and unsupported-state reporting.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_doctor::stabilize_project_doctor_probes_finding_codes_explainability_and::{
    load_stable_finding, load_stable_probe_pack_catalog, load_stable_unsupported_state_report,
    ProjectDoctorStableEvaluator, ProjectDoctorStableFinding, ProjectDoctorStableProbePackCatalog,
    ProjectDoctorStableUnsupportedStateReport, StableAttributionKindClass,
    StableChainOfCustodyEvent, StableFindingConfidenceClass, StableFindingSeverityClass,
    StableHeadlessAdmissionClass, StableProbePackClass, StableProbePackLifecycleStatus,
    StableReadOnlyPostureClass, StableRecoveryHandoffClass, StableRenderSurfaceClass,
    StableSupportContextClass, StableSupportGuidedAdmissionClass, StableSupportRedactionClass,
    UnsupportedStateClass, DOCTOR_FINDING_PREFIX, PROJECT_DOCTOR_STABLE_DOC_REF,
    PROJECT_DOCTOR_STABLE_SCHEMA_REF, PROJECT_DOCTOR_STABLE_SUPPORT_PACKET_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    catalog_file: String,
    finding_files: Vec<String>,
    unsupported_state_report_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(
        "fixtures/support/m4/stabilize_project_doctor_probes_finding_codes_explainability_and",
    )
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_catalog() -> ProjectDoctorStableProbePackCatalog {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.catalog_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_stable_probe_pack_catalog(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_findings() -> Vec<ProjectDoctorStableFinding> {
    load_manifest()
        .finding_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_stable_finding(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

fn load_unsupported_state_reports() -> Vec<ProjectDoctorStableUnsupportedStateReport> {
    load_manifest()
        .unsupported_state_report_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_stable_unsupported_state_report(&yaml)
                .unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn stable_catalog_packs_are_versioned_named_and_read_only_by_default() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();

    evaluator
        .validate_catalog(&catalog)
        .expect("catalog validates");

    assert!(!catalog.packs.is_empty());

    let mut covered_classes = BTreeSet::new();
    let mut has_stable = false;
    for pack in &catalog.packs {
        assert!(!pack.pack_id.trim().is_empty());
        assert!(!pack.pack_version.trim().is_empty());
        assert!(matches!(
            pack.lifecycle_status,
            StableProbePackLifecycleStatus::Stable
                | StableProbePackLifecycleStatus::Beta
                | StableProbePackLifecycleStatus::Deprecated
        ));
        if matches!(
            pack.lifecycle_status,
            StableProbePackLifecycleStatus::Stable
        ) {
            has_stable = true;
        }
        assert!(matches!(
            pack.read_only_posture,
            StableReadOnlyPostureClass::ReadOnlyByDefaultNoMutation
                | StableReadOnlyPostureClass::MetadataLocalEvidenceOnly
        ));
        assert!(
            !matches!(
                pack.headless_admission,
                StableHeadlessAdmissionClass::Denied
            ) || !matches!(
                pack.support_guided_admission,
                StableSupportGuidedAdmissionClass::Denied
            )
        );
        assert_eq!(
            pack.default_redaction_class,
            StableSupportRedactionClass::MetadataSafeDefault
        );
        assert!(!pack.supported_finding_codes.is_empty());
        for code in &pack.supported_finding_codes {
            assert!(code.starts_with(DOCTOR_FINDING_PREFIX));
        }
        covered_classes.insert(pack.pack_class);
    }

    assert!(has_stable, "catalog must contain at least one stable pack");
    assert!(covered_classes.len() >= 4);
    assert!(covered_classes.contains(&StableProbePackClass::EntryOpenReadiness));
    assert!(covered_classes.contains(&StableProbePackClass::ToolchainResolution));
    assert!(covered_classes.contains(&StableProbePackClass::ProviderAuth));
    assert!(covered_classes.contains(&StableProbePackClass::SupportBundleIntegrity));
}

#[test]
fn stable_findings_are_typed_attributable_confidence_labeled_and_explainable() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let findings = load_findings();
    assert!(!findings.is_empty());

    let mut severities = BTreeSet::new();
    let mut confidences = BTreeSet::new();
    let mut factor_classes = BTreeSet::new();
    for finding in &findings {
        evaluator
            .validate_finding(finding)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", finding.finding_id));
        evaluator
            .validate_finding_against_catalog(&catalog, finding)
            .unwrap_or_else(|err| panic!("{} catalog-bound failed: {err:?}", finding.finding_id));

        assert!(finding.finding_code.starts_with(DOCTOR_FINDING_PREFIX));
        assert!(!finding.evidence_refs.is_empty());
        assert!(!finding.render_surfaces.is_empty());
        assert!(!finding.explainability_factors.is_empty());
        assert_eq!(
            finding.redaction_class,
            StableSupportRedactionClass::MetadataSafeDefault
        );
        assert!(finding.raw_private_material_excluded);
        assert!(finding.attribution_refs.iter().any(|attribution| {
            attribution.attribution_kind == StableAttributionKindClass::ProbePackRef
                && attribution.ref_ == finding.probe_pack_ref
        }));

        for factor in &finding.explainability_factors {
            factor_classes.insert(factor.factor_class);
        }

        let _: StableFindingSeverityClass = finding.severity_class;
        let _: StableFindingConfidenceClass = finding.confidence_class;
        severities.insert(finding.severity_class);
        confidences.insert(finding.confidence_class);
    }

    assert!(severities.len() >= 2);
    assert!(confidences.len() >= 2);
    assert!(
        factor_classes.len() >= 3,
        "findings must cover at least 3 distinct explainability factor classes"
    );
}

#[test]
fn stable_support_packet_renders_one_finding_packet_for_ui_cli_and_support_export() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let findings = load_findings();
    let reports = load_unsupported_state_reports();

    let custody = vec![StableChainOfCustodyEvent {
        event_id: "custody.001".to_owned(),
        sequence: 0,
        actor_class: "doctor_stable_evaluator".to_owned(),
        actor_ref: "project_doctor.stable.evaluator.v1".to_owned(),
        action_class: "packet_created".to_owned(),
        occurred_at: "2026-06-02T10:05:00Z".to_owned(),
        note: "Stable support packet created from catalog and findings.".to_owned(),
    }];

    let packet = evaluator
        .support_packet(
            "support.project_doctor.stable_catalog",
            "2026-06-02T10:05:00Z",
            &catalog,
            &findings,
            &reports,
            &custody,
        )
        .expect("stable packet builds");

    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_STABLE_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.doc_ref, PROJECT_DOCTOR_STABLE_DOC_REF);
    assert_eq!(packet.schema_ref, PROJECT_DOCTOR_STABLE_SCHEMA_REF);
    assert!(packet.is_export_safe());
    assert_eq!(packet.pack_rows.len(), catalog.packs.len());
    assert_eq!(packet.finding_rows.len(), findings.len());
    assert_eq!(packet.unsupported_state_rows.len(), reports.len());
    assert_eq!(packet.chain_of_custody.len(), 1);

    let render_surfaces = [
        StableRenderSurfaceClass::UiFindingCard,
        StableRenderSurfaceClass::CliFindingRow,
        StableRenderSurfaceClass::SupportExportRow,
    ];
    for surface in render_surfaces {
        assert!(
            packet
                .finding_rows
                .iter()
                .any(|row| row.render_surfaces.contains(&surface)),
            "no stable finding row renders on {surface:?}"
        );
    }
}

#[test]
fn stable_unsupported_state_reports_are_typed_and_evidence_backed() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let reports = load_unsupported_state_reports();
    assert!(!reports.is_empty());

    let mut state_classes = BTreeSet::new();
    for report in &reports {
        evaluator
            .validate_unsupported_state_report(report)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", report.report_id));
        evaluator
            .validate_unsupported_state_report_against_catalog(&catalog, report)
            .unwrap_or_else(|err| panic!("{} catalog-bound failed: {err:?}", report.report_id));

        assert!(!report.report_id.trim().is_empty());
        assert!(!report.evidence_refs.is_empty());
        assert!(report.raw_private_material_excluded);
        assert_eq!(
            report.redaction_class,
            StableSupportRedactionClass::MetadataSafeDefault
        );
        state_classes.insert(report.unsupported_state_class);
    }

    assert!(
        state_classes.len() >= 1,
        "reports must cover at least one unsupported state class"
    );
}

#[test]
fn stable_evaluator_refuses_unknown_pack_ref_or_unsupported_finding_code() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let mut findings = load_findings();
    let mut finding = findings.remove(0);

    let original_ref = finding.probe_pack_ref.clone();
    finding.probe_pack_ref = "project_doctor.pack.does_not_exist".to_owned();
    finding.attribution_refs = finding
        .attribution_refs
        .into_iter()
        .map(|mut attribution| {
            if attribution.attribution_kind == StableAttributionKindClass::ProbePackRef {
                attribution.ref_ = finding.probe_pack_ref.clone();
            }
            attribution
        })
        .collect();
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err("unknown pack ref must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "project_doctor_stable.finding_pack_ref_unknown"));

    // Restore pack ref and make finding_code unsupported.
    finding.probe_pack_ref = original_ref;
    finding.attribution_refs = finding
        .attribution_refs
        .into_iter()
        .map(|mut attribution| {
            if attribution.attribution_kind == StableAttributionKindClass::ProbePackRef {
                attribution.ref_ = finding.probe_pack_ref.clone();
            }
            attribution
        })
        .collect();
    finding.finding_code = "doctor.finding.not_in_catalog".to_owned();
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err("uncataloged finding_code must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "project_doctor_stable.finding_code_not_supported"));
}

#[test]
fn stable_evaluator_refuses_finding_without_explainability_factors() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let mut finding = load_findings().remove(0);

    finding.explainability_factors.clear();
    let report = evaluator
        .validate_finding(&finding)
        .expect_err("missing explainability factors must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "project_doctor_stable.finding_explainability_missing"
    }));
}

#[test]
fn stable_evaluator_refuses_headless_finding_against_support_guided_only_pack() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let provider_auth = catalog
        .packs
        .iter()
        .find(|pack| pack.pack_class == StableProbePackClass::ProviderAuth)
        .expect("provider_auth pack present");
    assert!(matches!(
        provider_auth.headless_admission,
        StableHeadlessAdmissionClass::AdmittedSafeForSupportGuidedOnly
    ));

    let mut finding = load_findings()
        .into_iter()
        .find(|finding| finding.probe_pack_ref == provider_auth.pack_id)
        .expect("provider_auth stable finding present");
    finding.support_context_class = StableSupportContextClass::CliHeadless;
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err(
            "cli_headless support context against support_guided_only pack must be rejected",
        );
    assert!(
        report.violations.iter().any(|violation| {
            violation.check_id == "project_doctor_stable.finding_support_context_not_supported"
        }),
        "expected support_context_not_supported, got: {:?}",
        report.violations
    );
}

#[test]
fn stable_evaluator_refuses_recovery_handoff_not_declared_by_pack() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let catalog = load_catalog();
    let mut finding = load_findings()
        .into_iter()
        .find(|finding| finding.probe_pack_class == StableProbePackClass::EntryOpenReadiness)
        .expect("entry_open stable finding present");
    // entry_open pack admits reviewed_repair_available and handoff_only, not preview_only.
    finding.recovery_handoff_class = StableRecoveryHandoffClass::PreviewOnly;
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err("undeclared recovery_handoff must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "project_doctor_stable.finding_recovery_handoff_not_supported"
    }));
}

#[test]
fn stable_evaluator_refuses_none_unsupported_state_with_finding_code() {
    let evaluator = ProjectDoctorStableEvaluator::new();
    let mut reports = load_unsupported_state_reports();
    let report = reports
        .iter_mut()
        .find(|r| !matches!(r.unsupported_state_class, UnsupportedStateClass::None))
        .expect("a non-None unsupported state report is present");

    report.unsupported_state_class = UnsupportedStateClass::None;
    report.unsupported_finding_code = "doctor.finding.entry_open.scope_out_of_bounds".to_owned();
    let validation = evaluator
        .validate_unsupported_state_report(report)
        .expect_err("None state with non-empty finding code must be rejected");
    assert!(validation.violations.iter().any(|violation| {
        violation.check_id == "project_doctor_stable.report_none_with_finding_code"
    }));
}
