//! Fixture-driven coverage for the M5 automation surface certification matrix:
//! every claimed M5 notebook/request-API/package/test-debug/incident/AI-linked
//! automation surface, the six graded certification dimensions, the
//! freshness/stale narrowing roll-up, the certification index, and the
//! fail-closed guardrails against ad-hoc authoring, missing builder evidence,
//! unreviewed parameters, missing side-effect previews, lost run history, unsafe
//! macros, invented labels, and shareable claims without proof.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_automation_certification_input, AutomationAuthoringPath,
    AutomationCertificationCliHeadlessView, AutomationCertificationEvidenceJoinView,
    AutomationCertificationEvidenceSurface, AutomationCertificationPacket,
    AutomationCertificationPacketInput, AutomationCertificationSupportExport, AutomationSurface,
    AutomationSurfaceCertification, AUTOMATION_CERTIFICATION_CONTRACT_BASELINE_SCHEMA_REF,
    AUTOMATION_CERTIFICATION_DOC_REF, AUTOMATION_CERTIFICATION_EVIDENCE_REFS,
    AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF, AUTOMATION_CERTIFICATION_SCHEMA_REF,
};
use serde::Deserialize;

const FIXTURE_DIR: &str = "fixtures/automation/m5/automation-certification";

/// Each fixture is (file-name, mutation).
const CASES: [(&str, &str); 11] = [
    ("baseline_stable.json", "none"),
    ("ad_hoc_authoring_blocks_stable.json", "ad_hoc_authoring"),
    (
        "missing_builder_evidence_blocks_stable.json",
        "missing_builder_evidence",
    ),
    (
        "unreviewed_parameters_block_stable.json",
        "unreviewed_parameters",
    ),
    (
        "unsafe_secret_reference_blocks_stable.json",
        "unsafe_secret_reference",
    ),
    (
        "missing_side_effect_preview_blocks_stable.json",
        "missing_side_effect_preview",
    ),
    (
        "run_history_integrity_missing_blocks_stable.json",
        "run_history_integrity_missing",
    ),
    (
        "macro_scope_unsafe_blocks_stable.json",
        "macro_scope_unsafe",
    ),
    (
        "label_reuse_broken_blocks_stable.json",
        "label_reuse_broken",
    ),
    ("missing_surface_blocks_stable.json", "missing_surface"),
    ("evidence_stale_narrows_below_stable.json", "evidence_stale"),
];

#[derive(Debug, Deserialize)]
struct CertificationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    mutation: String,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    expected_finding_kinds: Vec<String>,
    surface_tokens: Vec<String>,
    authoring_path_tokens: Vec<String>,
    dimension_tokens: Vec<String>,
    shareable_surfaces: Vec<String>,
    narrowed_surfaces: Vec<String>,
    blocked_surfaces: Vec<String>,
    support_export_safe: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn assert_exists(rel: &str) {
    let path = repo_root().join(rel);
    assert!(
        path.exists(),
        "expected path to exist on disk: {} ({})",
        rel,
        path.display()
    );
}

fn load_fixture(file_name: &str) -> CertificationFixture {
    let path = repo_root().join(FIXTURE_DIR).join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn surface(
    input: &mut AutomationCertificationPacketInput,
    surface: AutomationSurface,
) -> &mut AutomationSurfaceCertification {
    input
        .surfaces
        .iter_mut()
        .find(|row| row.surface == surface)
        .expect("surface present")
}

/// Mirrors the mutations applied by the `dump_m5_automation_certification`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_input(mutation: &str) -> AutomationCertificationPacketInput {
    let mut input = current_stable_automation_certification_input();
    match mutation {
        "none" => {}
        "ad_hoc_authoring" => {
            surface(&mut input, AutomationSurface::NotebookAutomation).authoring_path =
                AutomationAuthoringPath::AdHocFeatureDialog;
        }
        "missing_builder_evidence" => {
            surface(&mut input, AutomationSurface::RequestApiAutomation).evidence_refs = Vec::new();
        }
        "unreviewed_parameters" => {
            surface(&mut input, AutomationSurface::PackageAutomation).parameters_reviewed = false;
        }
        "unsafe_secret_reference" => {
            surface(&mut input, AutomationSurface::RequestApiAutomation).secret_references_safe =
                false;
        }
        "missing_side_effect_preview" => {
            surface(&mut input, AutomationSurface::TestDebugAutomation).side_effect_preview_shown =
                false;
        }
        "run_history_integrity_missing" => {
            surface(&mut input, AutomationSurface::IncidentAutomation).rerun_under_current_policy =
                false;
        }
        "macro_scope_unsafe" => {
            surface(&mut input, AutomationSurface::NotebookAutomation)
                .macro_fails_closed_on_mismatch = false;
        }
        "label_reuse_broken" => {
            surface(&mut input, AutomationSurface::AiLinkedAutomation).reuses_controlled_labels =
                false;
        }
        "missing_surface" => {
            input
                .surfaces
                .retain(|row| row.surface != AutomationSurface::AiLinkedAutomation);
        }
        "evidence_stale" => {
            let row = surface(&mut input, AutomationSurface::IncidentAutomation);
            row.proof_age_days = row.freshness_window_days + 10;
        }
        other => panic!("unknown mutation {other}"),
    }
    input
}

fn assert_token_list(observed: Vec<&str>, expected: &[String], label: &str) {
    let observed: Vec<String> = observed.into_iter().map(str::to_owned).collect();
    assert_eq!(&observed, expected, "{label} token drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_automation_certification_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let packet = AutomationCertificationPacket::materialize(mutated_input(&fixture.mutation));
    assert_eq!(
        packet.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|f| f.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    let observed_kinds: Vec<&str> = packet
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        observed_kinds,
        fixture
            .expect
            .expected_finding_kinds
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} finding kinds drift",
        fixture.case_name
    );
    assert_token_list(
        packet.surface_tokens(),
        &fixture.expect.surface_tokens,
        "surface",
    );
    assert_token_list(
        packet.authoring_path_tokens(),
        &fixture.expect.authoring_path_tokens,
        "authoring path",
    );
    assert_token_list(
        packet.dimension_tokens(),
        &fixture.expect.dimension_tokens,
        "dimension",
    );
    assert_eq!(
        packet.certification_index.shareable_surfaces, fixture.expect.shareable_surfaces,
        "fixture {} shareable drift",
        fixture.case_name
    );
    assert_eq!(
        packet.certification_index.narrowed_surfaces, fixture.expect.narrowed_surfaces,
        "fixture {} narrowed drift",
        fixture.case_name
    );
    assert_eq!(
        packet.certification_index.blocked_surfaces, fixture.expect.blocked_surfaces,
        "fixture {} blocked drift",
        fixture.case_name
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-18T00:01:00Z",
    );
    assert_eq!(
        export.is_export_safe(),
        fixture.expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(AUTOMATION_CERTIFICATION_SCHEMA_REF);
    assert_exists(AUTOMATION_CERTIFICATION_CONTRACT_BASELINE_SCHEMA_REF);
    assert_exists(AUTOMATION_CERTIFICATION_DOC_REF);
    assert_exists(AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF);
    for reference in AUTOMATION_CERTIFICATION_EVIDENCE_REFS {
        assert_exists(reference);
    }
    assert_exists(FIXTURE_DIR);
}

#[test]
fn every_fixture_lives_in_its_dir() {
    for (file_name, _) in CASES {
        let path = repo_root().join(FIXTURE_DIR).join(file_name);
        assert!(
            path.exists(),
            "fixture {file_name} must exist in {FIXTURE_DIR} ({})",
            path.display()
        );
    }
}

#[test]
fn checked_in_packet_validates_clean() {
    let path = repo_root().join(AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must read: {err}"));
    let packet: AutomationCertificationPacket = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("packet artifact {path:?} must parse: {err}"));
    assert!(
        packet.validate().is_empty(),
        "checked-in packet must validate without findings: {:?}",
        packet.validate()
    );
    assert_eq!(packet.promotion_state.as_str(), "stable");
}

#[test]
fn checked_in_artifacts_match_the_seed() {
    let packet =
        AutomationCertificationPacket::materialize(current_stable_automation_certification_input());

    let packet_path = repo_root().join(AUTOMATION_CERTIFICATION_PACKET_ARTIFACT_REF);
    let on_disk: AutomationCertificationPacket =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, packet, "checked-in packet drifted from the seed");

    let support_path = packet_path.with_file_name("support_export.json");
    let support: AutomationCertificationSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.packet, packet);

    let cli_path = packet_path.with_file_name("cli_headless.json");
    let cli: AutomationCertificationCliHeadlessView =
        serde_json::from_str(&std::fs::read_to_string(&cli_path).expect("read cli view"))
            .expect("parse cli view");
    assert!(cli.every_surface_explained());
    assert_eq!(cli.surface_digest, packet.surface_digest);
    assert_eq!(cli.surface_rows.len(), packet.surfaces.len());

    for (file_name, surface) in [
        (
            "ai_evidence.json",
            AutomationCertificationEvidenceSurface::AiEvidence,
        ),
        (
            "incident_packet.json",
            AutomationCertificationEvidenceSurface::IncidentPacket,
        ),
    ] {
        let path = packet_path.with_file_name(file_name);
        let view: AutomationCertificationEvidenceJoinView =
            serde_json::from_str(&std::fs::read_to_string(&path).expect("read evidence join"))
                .expect("parse evidence join");
        assert_eq!(view.surface, surface);
        assert!(
            view.explains_consistently(),
            "{file_name} must explain consistently"
        );
        assert_eq!(view.surface_digest, packet.surface_digest);
        assert_eq!(view.surface_rows.len(), packet.surfaces.len());
        assert_eq!(view.certification_index, packet.certification_index);
    }
}

#[test]
fn all_fixtures_match_the_seed() {
    for (file_name, _) in CASES {
        assert_fixture_matches(file_name);
    }
}
