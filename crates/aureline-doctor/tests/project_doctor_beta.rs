//! Protected tests for the beta Project Doctor probe-pack catalog and finding
//! evaluator.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_doctor::probes::beta::{
    load_beta_finding, load_probe_pack_catalog, AttributionKindClass, FindingConfidenceClass,
    FindingSeverityClass, HeadlessAdmissionClass, ProbePackClass, ProbePackLifecycleStatus,
    ProjectDoctorBetaEvaluator, ProjectDoctorBetaFinding, ProjectDoctorProbePackCatalog,
    ReadOnlyPostureClass, RecoveryHandoffClass, RenderSurfaceClass, SupportContextClass,
    SupportGuidedAdmissionClass, SupportRedactionClass, DOCTOR_FINDING_PREFIX,
    PROJECT_DOCTOR_BETA_DOC_REF, PROJECT_DOCTOR_BETA_SCHEMA_REF,
    PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND, PROJECT_DOCTOR_FINDING_BETA_RECORD_KIND,
    PROJECT_DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    catalog_file: String,
    finding_files: Vec<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/support/project_doctor_beta")
}

fn load_manifest() -> Manifest {
    let path = fixture_dir().join("manifest.yaml");
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_yaml::from_str(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_catalog() -> ProjectDoctorProbePackCatalog {
    let manifest = load_manifest();
    let path = fixture_dir().join(manifest.catalog_file);
    let yaml = std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    load_probe_pack_catalog(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn load_findings() -> Vec<ProjectDoctorBetaFinding> {
    load_manifest()
        .finding_files
        .into_iter()
        .map(|file| {
            let path = fixture_dir().join(file);
            let yaml =
                std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
            load_beta_finding(&yaml).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
        })
        .collect()
}

#[test]
fn beta_catalog_packs_are_versioned_named_and_read_only_by_default() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();

    evaluator
        .validate_catalog(&catalog)
        .expect("catalog validates");

    assert_eq!(
        catalog.record_kind,
        PROJECT_DOCTOR_PROBE_PACK_CATALOG_RECORD_KIND
    );
    assert_eq!(catalog.doc_ref, PROJECT_DOCTOR_BETA_DOC_REF);
    assert_eq!(catalog.schema_ref, PROJECT_DOCTOR_BETA_SCHEMA_REF);
    assert!(!catalog.packs.is_empty());

    let mut covered_classes = BTreeSet::new();
    for pack in &catalog.packs {
        assert!(!pack.pack_id.trim().is_empty());
        assert!(!pack.pack_version.trim().is_empty());
        assert!(matches!(
            pack.lifecycle_status,
            ProbePackLifecycleStatus::Beta | ProbePackLifecycleStatus::Deprecated
        ));
        assert!(matches!(
            pack.read_only_posture,
            ReadOnlyPostureClass::ReadOnlyByDefaultNoMutation
                | ReadOnlyPostureClass::MetadataLocalEvidenceOnly
        ));
        // A pack is admitted under headless or under support-guided mode.
        assert!(
            !matches!(pack.headless_admission, HeadlessAdmissionClass::Denied)
                || !matches!(
                    pack.support_guided_admission,
                    SupportGuidedAdmissionClass::Denied
                )
        );
        assert_eq!(
            pack.default_redaction_class,
            SupportRedactionClass::MetadataSafeDefault
        );
        assert!(!pack.supported_finding_codes.is_empty());
        for code in &pack.supported_finding_codes {
            assert!(code.starts_with(DOCTOR_FINDING_PREFIX));
        }
        covered_classes.insert(pack.pack_class);
    }

    // Beta catalog exercises at least four distinct pack classes.
    assert!(covered_classes.len() >= 4);
    assert!(covered_classes.contains(&ProbePackClass::EntryOpenReadiness));
    assert!(covered_classes.contains(&ProbePackClass::ToolchainResolution));
    assert!(covered_classes.contains(&ProbePackClass::ProviderAuth));
    assert!(covered_classes.contains(&ProbePackClass::SupportBundleIntegrity));
}

#[test]
fn beta_findings_are_typed_attributable_and_confidence_labeled() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();
    let findings = load_findings();
    assert!(!findings.is_empty());

    let mut severities = BTreeSet::new();
    let mut confidences = BTreeSet::new();
    for finding in &findings {
        evaluator
            .validate_finding(finding)
            .unwrap_or_else(|err| panic!("{} failed: {err:?}", finding.finding_id));
        evaluator
            .validate_finding_against_catalog(&catalog, finding)
            .unwrap_or_else(|err| panic!("{} catalog-bound failed: {err:?}", finding.finding_id));

        assert_eq!(finding.record_kind, PROJECT_DOCTOR_FINDING_BETA_RECORD_KIND);
        assert!(finding.finding_code.starts_with(DOCTOR_FINDING_PREFIX));
        assert!(!finding.evidence_refs.is_empty());
        assert!(!finding.render_surfaces.is_empty());
        assert_eq!(
            finding.redaction_class,
            SupportRedactionClass::MetadataSafeDefault
        );
        assert!(finding.raw_private_material_excluded);
        assert!(finding.attribution_refs.iter().any(|attribution| {
            attribution.attribution_kind == AttributionKindClass::ProbePackRef
                && attribution.ref_ == finding.probe_pack_ref
        }));
        let _: FindingSeverityClass = finding.severity_class;
        let _: FindingConfidenceClass = finding.confidence_class;
        severities.insert(finding.severity_class);
        confidences.insert(finding.confidence_class);
    }

    assert!(severities.len() >= 2);
    assert!(confidences.len() >= 2);
}

#[test]
fn beta_support_packet_renders_one_finding_packet_for_ui_cli_and_support_export() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();
    let findings = load_findings();

    let packet = evaluator
        .support_packet(
            "support.project_doctor.beta_catalog",
            "2026-05-15T12:00:00Z",
            &catalog,
            &findings,
        )
        .expect("beta packet builds");

    assert_eq!(
        packet.record_kind,
        PROJECT_DOCTOR_BETA_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(packet.doc_ref, PROJECT_DOCTOR_BETA_DOC_REF);
    assert_eq!(packet.schema_ref, PROJECT_DOCTOR_BETA_SCHEMA_REF);
    assert!(packet.is_export_safe());
    assert_eq!(packet.pack_rows.len(), catalog.packs.len());
    assert_eq!(packet.finding_rows.len(), findings.len());

    let render_surfaces = [
        RenderSurfaceClass::UiFindingCard,
        RenderSurfaceClass::CliFindingRow,
        RenderSurfaceClass::SupportExportRow,
    ];
    for surface in render_surfaces {
        assert!(
            packet
                .finding_rows
                .iter()
                .any(|row| row.render_surfaces.contains(&surface)),
            "no beta finding row renders on {surface:?}"
        );
    }
}

#[test]
fn beta_evaluator_refuses_unknown_pack_ref_or_unsupported_finding_code() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();
    let mut findings = load_findings();
    let mut finding = findings.remove(0);

    let original_ref = finding.probe_pack_ref.clone();
    finding.probe_pack_ref = "project_doctor.pack.does_not_exist".to_owned();
    finding.attribution_refs = finding
        .attribution_refs
        .into_iter()
        .map(|mut attribution| {
            if attribution.attribution_kind == AttributionKindClass::ProbePackRef {
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
        .any(|violation| violation.check_id == "project_doctor.finding_pack_ref_unknown"));

    // Restore pack ref and make finding_code unsupported.
    finding.probe_pack_ref = original_ref;
    finding.attribution_refs = finding
        .attribution_refs
        .into_iter()
        .map(|mut attribution| {
            if attribution.attribution_kind == AttributionKindClass::ProbePackRef {
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
        .any(|violation| violation.check_id == "project_doctor.finding_code_not_supported"));
}

#[test]
fn beta_evaluator_refuses_finding_without_pack_attribution_or_with_raw_material_present() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let mut finding = load_findings().remove(0);

    // Drop the probe_pack_ref attribution.
    let mut without_pack_attribution = finding.clone();
    without_pack_attribution
        .attribution_refs
        .retain(|attribution| attribution.attribution_kind != AttributionKindClass::ProbePackRef);
    let report = evaluator
        .validate_finding(&without_pack_attribution)
        .expect_err("missing pack attribution must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "project_doctor.finding_pack_attribution_missing"
    }));

    // Declare raw private material present.
    finding.raw_private_material_excluded = false;
    let report = evaluator
        .validate_finding(&finding)
        .expect_err("raw material present must be rejected");
    assert!(report
        .violations
        .iter()
        .any(|violation| violation.check_id == "project_doctor.finding_raw_material_present"));
}

#[test]
fn beta_evaluator_refuses_headless_finding_against_support_guided_only_pack() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();
    let provider_auth = catalog
        .packs
        .iter()
        .find(|pack| pack.pack_class == ProbePackClass::ProviderAuth)
        .expect("provider_auth pack present");
    assert!(matches!(
        provider_auth.headless_admission,
        HeadlessAdmissionClass::AdmittedSafeForSupportGuidedOnly
    ));

    let mut finding = load_findings()
        .into_iter()
        .find(|finding| finding.probe_pack_ref == provider_auth.pack_id)
        .expect("provider_auth beta finding present");
    finding.support_context_class = SupportContextClass::CliHeadless;
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err(
            "cli_headless support context against support_guided_only pack must be rejected",
        );
    assert!(
        report.violations.iter().any(|violation| {
            violation.check_id == "project_doctor.finding_support_context_not_supported"
        }),
        "expected support_context_not_supported, got: {:?}",
        report.violations
    );
}

#[test]
fn beta_evaluator_refuses_recovery_handoff_not_declared_by_pack() {
    let evaluator = ProjectDoctorBetaEvaluator::new();
    let catalog = load_catalog();
    let mut finding = load_findings()
        .into_iter()
        .find(|finding| finding.probe_pack_class == ProbePackClass::EntryOpenReadiness)
        .expect("entry_open beta finding present");
    // entry_open pack admits reviewed_repair_available and handoff_only, not preview_only.
    finding.recovery_handoff_class = RecoveryHandoffClass::PreviewOnly;
    let report = evaluator
        .validate_finding_against_catalog(&catalog, &finding)
        .expect_err("undeclared recovery_handoff must be rejected");
    assert!(report.violations.iter().any(|violation| {
        violation.check_id == "project_doctor.finding_recovery_handoff_not_supported"
    }));
}
