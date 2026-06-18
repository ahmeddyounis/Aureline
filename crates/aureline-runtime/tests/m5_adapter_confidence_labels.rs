//! Fixture-driven coverage for the M5 adapter-confidence audit: the per-surface
//! confidence-label bindings, the no-lower-confidence-overwrite arbitration, the
//! source-quality-change vocabulary, and the support / CLI / AI projections that
//! keep confidence preservation honest across the trust boundary.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_adapter_confidence_audit_input, seeded_adapter_confidence_audit,
    AdapterConfidenceAiEvidenceView, AdapterConfidenceAudit, AdapterConfidenceAuditSupportExport,
    AdapterConfidenceCliHeadlessView, BuildTestInteropConfidence, OverwriteDecision,
    OverwriteReason, SourceQualityChange, ADAPTER_CONFIDENCE_AUDIT_DOC_REF,
    ADAPTER_CONFIDENCE_AUDIT_ENVELOPE_SCHEMA_REF, ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR,
    ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF, ADAPTER_CONFIDENCE_AUDIT_SCHEMA_REF,
};
use serde::Deserialize;

const STRUCTURED_OVERCLAIM_CLAIM: &str = "claim:coverage:structured";
const BLOCKED_DECISION_CLAIM: &str = "claim:test:finish:heuristic";
const HELD_SUBJECT: &str = "subject:notebook:test";

#[derive(Debug, Deserialize)]
struct AuditFixture {
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
    source_kind_tokens: Vec<String>,
    source_quality_change_tokens: Vec<String>,
    overwrite_decision_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> AuditFixture {
    let path = repo_root()
        .join(ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

/// Mirrors the mutations applied by the `dump_m5_adapter_confidence_labels`
/// regenerator, so the checked-in fixtures stay bit-for-bit derivable.
fn mutated_audit(mutation: &str) -> AdapterConfidenceAudit {
    match mutation {
        "none" => {
            AdapterConfidenceAudit::materialize(current_stable_adapter_confidence_audit_input())
        }
        "binding_missing" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings.remove(0);
            AdapterConfidenceAudit::materialize(input)
        }
        "surface_collapses_label" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings[0].keeps_source_and_confidence_distinct = false;
            AdapterConfidenceAudit::materialize(input)
        }
        "surface_hides_banner" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            input.surface_bindings[0].shows_heuristic_fallback_banner = false;
            AdapterConfidenceAudit::materialize(input)
        }
        "claim_confidence_overclaim" => {
            let mut input = current_stable_adapter_confidence_audit_input();
            for subject in &mut input.subjects {
                for claim in &mut subject.claims {
                    if claim.claim_id == STRUCTURED_OVERCLAIM_CLAIM {
                        claim.label.confidence = BuildTestInteropConfidence::High;
                    }
                }
            }
            AdapterConfidenceAudit::materialize(input)
        }
        "lower_confidence_overwrite_accepted" => {
            let mut audit = AdapterConfidenceAudit::materialize(
                current_stable_adapter_confidence_audit_input(),
            );
            for subject in &mut audit.subjects {
                for decision in &mut subject.overwrite_decisions {
                    if decision.claim_id == BLOCKED_DECISION_CLAIM {
                        decision.decision = OverwriteDecision::EnrichedContextOnly;
                        decision.reason = Some(OverwriteReason::NeverClaimedAuthority);
                    }
                }
            }
            audit.refresh_findings();
            audit
        }
        "source_quality_change_mismatch" => {
            let mut audit = AdapterConfidenceAudit::materialize(
                current_stable_adapter_confidence_audit_input(),
            );
            for subject in &mut audit.subjects {
                if subject.subject.subject_id == HELD_SUBJECT {
                    subject.source_quality_change = SourceQualityChange::UpgradedToAuthoritative;
                }
            }
            audit.refresh_findings();
            audit
        }
        other => panic!("unknown mutation {other}"),
    }
}

fn assert_token_list(observed: Vec<&str>, expected: &[String], label: &str) {
    let observed: Vec<String> = observed.into_iter().map(str::to_owned).collect();
    assert_eq!(&observed, expected, "{label} token drift");
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "m5_adapter_confidence_audit_case",
        "fixture {file_name} declares unexpected record_kind"
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.case_name.trim().is_empty() && !fixture.scenario.trim().is_empty(),
        "fixture must describe its case and scenario"
    );

    let audit = mutated_audit(&fixture.mutation);
    assert_eq!(
        audit.promotion_state.as_str(),
        fixture.expect.promotion_state,
        "fixture {} promotion drift",
        fixture.case_name
    );
    let observed_kinds: Vec<&str> = audit
        .validation_findings
        .iter()
        .map(|f| f.finding_kind.as_str())
        .collect();
    assert_eq!(
        audit.validation_findings.len(),
        fixture.expect.validation_finding_count,
        "fixture {} finding count drift; got {observed_kinds:?}",
        fixture.case_name
    );
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
        audit.surface_tokens(),
        &fixture.expect.surface_tokens,
        "surface",
    );
    assert_token_list(
        audit.source_kind_tokens(),
        &fixture.expect.source_kind_tokens,
        "source kind",
    );
    assert_token_list(
        audit.source_quality_change_tokens(),
        &fixture.expect.source_quality_change_tokens,
        "source quality change",
    );
    assert_token_list(
        audit.overwrite_decision_tokens(),
        &fixture.expect.overwrite_decision_tokens,
        "overwrite decision",
    );

    let export = audit.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-06-17T00:01:00Z",
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
    assert_exists(ADAPTER_CONFIDENCE_AUDIT_SCHEMA_REF);
    assert_exists(ADAPTER_CONFIDENCE_AUDIT_ENVELOPE_SCHEMA_REF);
    assert_exists(ADAPTER_CONFIDENCE_AUDIT_DOC_REF);
    assert_exists(ADAPTER_CONFIDENCE_AUDIT_FIXTURE_DIR);
    assert_exists(ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF);
}

#[test]
fn checked_in_audit_validates_clean() {
    let path = repo_root().join(ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("audit artifact {path:?} must read: {err}"));
    let audit: AdapterConfidenceAudit = serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("audit artifact {path:?} must parse: {err}"));
    assert!(
        audit.validate().is_empty(),
        "checked-in audit must validate without findings: {:?}",
        audit.validate()
    );
    assert_eq!(audit.promotion_state.as_str(), "stable");
}

#[test]
fn checked_in_artifacts_match_the_seed() {
    let audit = seeded_adapter_confidence_audit();

    let packet_path = repo_root().join(ADAPTER_CONFIDENCE_AUDIT_PACKET_ARTIFACT_REF);
    let on_disk: AdapterConfidenceAudit =
        serde_json::from_str(&std::fs::read_to_string(&packet_path).expect("read packet"))
            .expect("parse packet");
    assert_eq!(on_disk, audit, "checked-in packet drifted from the seed");

    let support_path = packet_path.with_file_name("support_export.json");
    let support: AdapterConfidenceAuditSupportExport =
        serde_json::from_str(&std::fs::read_to_string(&support_path).expect("read support export"))
            .expect("parse support export");
    assert!(support.is_export_safe());
    assert_eq!(support.audit, audit);

    let cli_path = packet_path.with_file_name("cli_headless.json");
    let cli: AdapterConfidenceCliHeadlessView =
        serde_json::from_str(&std::fs::read_to_string(&cli_path).expect("read cli view"))
            .expect("parse cli view");
    assert!(cli.every_row_keeps_label());
    assert_eq!(cli.label_digest, audit.label_digest);

    let ai_path = packet_path.with_file_name("ai_evidence.json");
    let ai: AdapterConfidenceAiEvidenceView =
        serde_json::from_str(&std::fs::read_to_string(&ai_path).expect("read ai view"))
            .expect("parse ai view");
    assert!(ai.keeps_lineage());
    assert_eq!(ai.label_digest, audit.label_digest);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn binding_missing_blocks_stable() {
    assert_fixture_matches("binding_missing_blocks_stable.json");
}

#[test]
fn surface_collapses_label_blocks_stable() {
    assert_fixture_matches("surface_collapses_label_blocks_stable.json");
}

#[test]
fn surface_hides_banner_blocks_stable() {
    assert_fixture_matches("surface_hides_banner_blocks_stable.json");
}

#[test]
fn claim_confidence_overclaim_blocks_stable() {
    assert_fixture_matches("claim_confidence_overclaim_blocks_stable.json");
}

#[test]
fn lower_confidence_overwrite_accepted_blocks_stable() {
    assert_fixture_matches("lower_confidence_overwrite_accepted_blocks_stable.json");
}

#[test]
fn source_quality_change_mismatch_blocks_stable() {
    assert_fixture_matches("source_quality_change_mismatch_blocks_stable.json");
}
