//! Fixture-driven coverage for the stable audit of topology, explainer,
//! and companion-adjacent surfaces truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_audit_topology_explainer_companion_truth_packet, AuditConsumerSurface,
    AuditFindingKind, AuditPromotionState, AuditSurfaceClass,
    AuditTopologyExplainerCompanionTruthPacket, AuditTopologyExplainerCompanionTruthPacketInput,
    DowngradeStateDisclosureClass, FreshnessDisclosureClass, ProvenanceDisclosureClass,
    QualificationState, ScopeDisclosureClass,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_ARTIFACT_DOC_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_DOC_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_FIXTURE_DIR,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_ARTIFACT_REF,
    AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AuditFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: AuditTopologyExplainerCompanionTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    surface_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    qualification_tokens: Vec<String>,
    scope_disclosure_tokens: Vec<String>,
    freshness_disclosure_tokens: Vec<String>,
    provenance_disclosure_tokens: Vec<String>,
    downgrade_state_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> AuditFixture {
    let path = repo_root()
        .join(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "audit_topology_explainer_companion_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = AuditTopologyExplainerCompanionTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(&packet.surface_tokens(), &expect.surface_tokens, "surface");
    assert_token_set_matches(
        &packet.row_class_tokens(),
        &expect.row_class_tokens,
        "row_class",
    );
    assert_token_set_matches(
        &packet.qualification_tokens(),
        &expect.qualification_tokens,
        "qualification",
    );
    assert_token_set_matches(
        &packet.scope_disclosure_tokens(),
        &expect.scope_disclosure_tokens,
        "scope_disclosure",
    );
    assert_token_set_matches(
        &packet.freshness_disclosure_tokens(),
        &expect.freshness_disclosure_tokens,
        "freshness_disclosure",
    );
    assert_token_set_matches(
        &packet.provenance_disclosure_tokens(),
        &expect.provenance_disclosure_tokens,
        "provenance_disclosure",
    );
    assert_token_set_matches(
        &packet.downgrade_state_tokens(),
        &expect.downgrade_state_tokens,
        "downgrade_state",
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
    assert_exists(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_SCHEMA_REF);
    assert_exists(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_DOC_REF);
    assert_exists(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_FIXTURE_DIR);
    assert_exists(AUDIT_TOPOLOGY_EXPLAINER_COMPANION_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn scope_unbound_fixture_blocks_stable() {
    assert_fixture_matches("scope_unbound_blocks_stable.json");
}

#[test]
fn non_qualified_row_masquerading_stable_fixture_blocks_stable() {
    assert_fixture_matches("non_qualified_row_masquerading_stable_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_fixture_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn audit_pillar_collapsed_fixture_blocks_stable() {
    assert_fixture_matches("audit_pillar_collapsed_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_surface() {
    let packet = current_stable_audit_topology_explainer_companion_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, AuditPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in AuditSurfaceClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.surface_class == required),
            "stable packet must include row for audit surface {}",
            required.as_str()
        );
    }
    for surface in AuditConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_audit_topology_explainer_companion_tokens_are_pinned() {
    assert_eq!(
        AuditSurfaceClass::CompanionHistory.as_str(),
        "companion_history"
    );
    assert_eq!(
        QualificationState::NotQualifiedStable.as_str(),
        "not_qualified_stable"
    );
    assert_eq!(ScopeDisclosureClass::ScopeUnbound.as_str(), "scope_unbound");
    assert_eq!(
        FreshnessDisclosureClass::FreshnessUnbound.as_str(),
        "freshness_unbound"
    );
    assert_eq!(
        ProvenanceDisclosureClass::ProvenanceUnbound.as_str(),
        "provenance_unbound"
    );
    assert_eq!(
        DowngradeStateDisclosureClass::DowngradeStateUnbound.as_str(),
        "downgrade_state_unbound"
    );
    assert_eq!(
        AuditFindingKind::NonQualifiedRowMasqueradingStable.as_str(),
        "non_qualified_row_masquerading_stable"
    );
    assert_eq!(
        AuditConsumerSurface::ReleaseProofIndex.as_str(),
        "release_proof_index"
    );
    assert_eq!(AuditPromotionState::BlocksStable.as_str(), "blocks_stable");
}
