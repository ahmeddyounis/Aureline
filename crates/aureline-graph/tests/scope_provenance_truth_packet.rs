//! Fixture-driven coverage for the stable hidden-scope, partial-scope,
//! archived-item, and imported-provider truth packet.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_scope_provenance_truth_packet, ScopeProvenanceConsumerSurface,
    ScopeProvenanceDowngradeState, ScopeProvenanceFindingKind, ScopeProvenanceItemClass,
    ScopeProvenancePromotionState, ScopeProvenanceTruthPacket,
    ScopeProvenanceTruthPacketInput, SCOPE_PROVENANCE_TRUTH_ARTIFACT_DOC_REF,
    SCOPE_PROVENANCE_TRUTH_DOC_REF, SCOPE_PROVENANCE_TRUTH_FIXTURE_DIR,
    SCOPE_PROVENANCE_TRUTH_PACKET_ARTIFACT_REF, SCOPE_PROVENANCE_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ScopeProvenanceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: ScopeProvenanceTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    item_class_tokens: Vec<String>,
    provenance_tokens: Vec<String>,
    downgrade_tokens: Vec<String>,
    imported_outcome_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ScopeProvenanceFixture {
    let path = repo_root()
        .join(SCOPE_PROVENANCE_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "scope_provenance_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = ScopeProvenanceTruthPacket::materialize(fixture.input.clone());
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
    assert_eq!(
        packet.item_class_tokens(),
        expect
            .item_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} item-class tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.provenance_tokens(),
        expect
            .provenance_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} provenance tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.downgrade_tokens(),
        expect
            .downgrade_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} downgrade tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.imported_outcome_tokens(),
        expect
            .imported_outcome_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} imported-outcome tokens drifted",
        fixture.case_name
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
    assert_exists(SCOPE_PROVENANCE_TRUTH_SCHEMA_REF);
    assert_exists(SCOPE_PROVENANCE_TRUTH_DOC_REF);
    assert_exists(SCOPE_PROVENANCE_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(SCOPE_PROVENANCE_TRUTH_FIXTURE_DIR);
    assert_exists(SCOPE_PROVENANCE_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn imported_missing_diagnostic_fixture_blocks_stable() {
    assert_fixture_matches("imported_missing_diagnostic_blocks_stable.json");
}

#[test]
fn non_canonical_presented_as_canonical_fixture_blocks_stable() {
    assert_fixture_matches("non_canonical_presented_as_canonical_blocks_stable.json");
}

#[test]
fn archived_missing_context_fixture_blocks_stable() {
    assert_fixture_matches("archived_missing_context_blocks_stable.json");
}

#[test]
fn projection_drops_downgrade_fixture_blocks_stable() {
    assert_fixture_matches("projection_drops_downgrade_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_item_class() {
    let packet =
        current_stable_scope_provenance_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        ScopeProvenancePromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in ScopeProvenanceItemClass::REQUIRED {
        assert!(
            packet
                .rows
                .iter()
                .any(|row| row.item_class == required),
            "stable packet must include row for item class {}",
            required.as_str()
        );
    }
    for surface in ScopeProvenanceConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_scope_provenance_tokens_are_pinned() {
    assert_eq!(
        ScopeProvenanceItemClass::HiddenScope.as_str(),
        "hidden_scope"
    );
    assert_eq!(
        ScopeProvenanceItemClass::ImportedProvider.as_str(),
        "imported_provider"
    );
    assert_eq!(
        ScopeProvenanceDowngradeState::ImportedDisclosed.as_str(),
        "imported_disclosed"
    );
    assert_eq!(
        ScopeProvenanceFindingKind::NonCanonicalPresentedAsCanonical.as_str(),
        "non_canonical_presented_as_canonical"
    );
    assert_eq!(
        ScopeProvenancePromotionState::BlocksStable.as_str(),
        "blocks_stable"
    );
}
