//! Fixture-driven coverage for the stable Node and Express Advanced
//! workflow pack truth packet covering create, open, run, test, debug,
//! rename, and review loops plus the Node/Express server project
//! model (CommonJS / ESM module system; Express
//! `app`/router/middleware/controller boundary) and the
//! launch-profile parity surface (dev / start / debug / test launch
//! profiles plus `node --inspect`, nodemon, and ts-node-dev
//! hot-reload) with known limits, downgrade automation, and evidence
//! binding.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_node_and_express_advanced_workflow_pack_truth_packet,
    NodeAndExpressAdvancedWorkflowLoopClass, NodeAndExpressAdvancedWorkflowPackClass,
    NodeAndExpressAdvancedWorkflowPackConsumerSurface,
    NodeAndExpressAdvancedWorkflowPackDowngradeAutomationClass,
    NodeAndExpressAdvancedWorkflowPackEvidenceClass,
    NodeAndExpressAdvancedWorkflowPackFindingKind,
    NodeAndExpressAdvancedWorkflowPackKnownLimitClass,
    NodeAndExpressAdvancedWorkflowPackPromotionState,
    NodeAndExpressAdvancedWorkflowPackRowClass,
    NodeAndExpressAdvancedWorkflowPackSupportClass,
    NodeAndExpressAdvancedWorkflowPackTruthPacket,
    NodeAndExpressAdvancedWorkflowPackTruthPacketInput,
    NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF,
    NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF,
    NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_FIXTURE_DIR,
    NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF,
    NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NodeAndExpressAdvancedWorkflowPackFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: NodeAndExpressAdvancedWorkflowPackTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    pack_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    workflow_loop_tokens: Vec<String>,
    known_limit_tokens: Vec<String>,
    downgrade_automation_tokens: Vec<String>,
    evidence_class_tokens: Vec<String>,
    support_export_safe: bool,
    #[serde(default)]
    expected_finding_kinds: Vec<String>,
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

fn load_fixture(file_name: &str) -> NodeAndExpressAdvancedWorkflowPackFixture {
    let path = repo_root()
        .join(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_token_set_matches(observed: &[&str], expected: &[String], label: &str) {
    let observed: BTreeSet<&str> = observed.iter().copied().collect();
    let expected: BTreeSet<&str> = expected.iter().map(String::as_str).collect();
    assert_eq!(
        observed, expected,
        "{label} token set drift: observed={observed:?}, expected={expected:?}"
    );
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "node_and_express_advanced_workflow_pack_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet =
        NodeAndExpressAdvancedWorkflowPackTruthPacket::materialize(fixture.input.clone());
    assert_eq!(
        packet.promotion_state.as_str(),
        expect.promotion_state,
        "fixture {} expected promotion {}, got {:?}",
        fixture.case_name,
        expect.promotion_state,
        packet.promotion_state
    );
    assert_eq!(
        packet.rows.len(),
        expect.row_count,
        "fixture {} row count drift",
        fixture.case_name
    );
    assert_eq!(
        packet.validation_findings.len(),
        expect.validation_finding_count,
        "fixture {} finding count drift; got {:?}",
        fixture.case_name,
        packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect::<Vec<_>>()
    );
    assert_token_set_matches(&packet.pack_tokens(), &expect.pack_tokens, "pack");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.support_class_tokens(),
        &expect.support_class_tokens,
        "support_class",
    );
    assert_token_set_matches(
        &packet.workflow_loop_tokens(),
        &expect.workflow_loop_tokens,
        "workflow_loop",
    );
    assert_token_set_matches(
        &packet.known_limit_tokens(),
        &expect.known_limit_tokens,
        "known_limit",
    );
    assert_token_set_matches(
        &packet.downgrade_automation_tokens(),
        &expect.downgrade_automation_tokens,
        "downgrade_automation",
    );
    assert_token_set_matches(
        &packet.evidence_class_tokens(),
        &expect.evidence_class_tokens,
        "evidence_class",
    );

    let export = packet.support_export(
        format!("support-export:{}", fixture.case_name),
        "2026-05-26T12:00:10Z",
    );
    assert_eq!(
        export.is_export_safe(),
        expect.support_export_safe,
        "fixture {} support-export safety drift",
        fixture.case_name
    );

    if !expect.expected_finding_kinds.is_empty() {
        let observed: BTreeSet<&str> = packet
            .validation_findings
            .iter()
            .map(|finding| finding.finding_kind.as_str())
            .collect();
        for kind in &expect.expected_finding_kinds {
            assert!(
                observed.contains(kind.as_str()),
                "fixture {} expected finding kind {kind}; observed {:?}",
                fixture.case_name,
                observed
            );
        }
    }
}

#[test]
fn schema_doc_fixture_and_artifact_exist_on_disk() {
    assert_exists(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_SCHEMA_REF);
    assert_exists(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_DOC_REF);
    assert_exists(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_FIXTURE_DIR);
    assert_exists(NODE_AND_EXPRESS_ADVANCED_WORKFLOW_PACK_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn expert_grade_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("expert_grade_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_workflow_loop_for_expert_grade_blocks_stable() {
    assert_fixture_matches("missing_workflow_loop_for_expert_grade_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_workflow_loop_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_workflow_loop_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_pack() {
    let packet = current_stable_node_and_express_advanced_workflow_pack_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        NodeAndExpressAdvancedWorkflowPackPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in NodeAndExpressAdvancedWorkflowPackClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.pack_class == required),
            "stable packet must include row for workflow pack {}",
            required.as_str()
        );
    }
    for surface in NodeAndExpressAdvancedWorkflowPackConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_migration_server_project_model_and_launch_profile_parity() {
    let packet = current_stable_node_and_express_advanced_workflow_pack_truth_packet()
        .expect("checked-in packet validates");
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == NodeAndExpressAdvancedWorkflowPackRowClass::FrameworkMigrationRow),
        "stable packet must include a framework_migration_row binding the Express 4 → Express 5 migration"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == NodeAndExpressAdvancedWorkflowPackRowClass::ServerProjectModelRow),
        "stable packet must include a server_project_model_row"
    );
    assert!(
        packet.rows.iter().any(|row| row.row_class
            == NodeAndExpressAdvancedWorkflowPackRowClass::LaunchProfileParityRow),
        "stable packet must include a launch_profile_parity_row"
    );
}

#[test]
fn closed_node_and_express_advanced_workflow_pack_tokens_are_pinned() {
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackClass::NodeAndExpressAdvancedWorkflowPack.as_str(),
        "node_and_express_advanced_workflow_pack"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackRowClass::ServerProjectModelRow.as_str(),
        "server_project_model_row"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackRowClass::LaunchProfileParityRow.as_str(),
        "launch_profile_parity_row"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackRowClass::DowngradeAutomation.as_str(),
        "downgrade_automation"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(NodeAndExpressAdvancedWorkflowLoopClass::Review.as_str(), "review");
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackEvidenceClass::ServerProjectModelEvidence.as_str(),
        "server_project_model_evidence"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackEvidenceClass::LaunchProfileParityEvidence.as_str(),
        "launch_profile_parity_evidence"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackKnownLimitClass::ServerProjectModelSubsetOnly.as_str(),
        "server_project_model_subset_only"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackKnownLimitClass::LaunchProfileParitySubsetOnly.as_str(),
        "launch_profile_parity_subset_only"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackDowngradeAutomationClass::AutoNarrowOnUnprovenServerProjectModel
            .as_str(),
        "auto_narrow_on_unproven_server_project_model"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackDowngradeAutomationClass::AutoNarrowOnUnprovenLaunchProfileParity
            .as_str(),
        "auto_narrow_on_unproven_launch_profile_parity"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        NodeAndExpressAdvancedWorkflowPackFindingKind::WorkflowLoopVocabularyCollapsed.as_str(),
        "workflow_loop_vocabulary_collapsed"
    );
}
