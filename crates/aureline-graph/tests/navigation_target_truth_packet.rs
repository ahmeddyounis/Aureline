//! Fixture-driven coverage for the stable navigation-target truth packet
//! that hardens find-references, rename-preview, and impact surfacing
//! across launch languages.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_graph::{
    current_stable_navigation_target_truth_packet, NavigationAccessKind,
    NavigationTargetConsumerSurface, NavigationTargetFindingKind, NavigationTargetPromotionState,
    NavigationTargetRowClass, NavigationTargetTruthPacket, NavigationTargetTruthPacketInput,
    NAVIGATION_TARGET_TRUTH_ARTIFACT_DOC_REF, NAVIGATION_TARGET_TRUTH_DOC_REF,
    NAVIGATION_TARGET_TRUTH_FIXTURE_DIR, NAVIGATION_TARGET_TRUTH_PACKET_ARTIFACT_REF,
    NAVIGATION_TARGET_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct NavigationTargetFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: NavigationTargetTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    row_class_tokens: Vec<String>,
    access_kind_tokens: Vec<String>,
    downgrade_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> NavigationTargetFixture {
    let path = repo_root()
        .join(NAVIGATION_TARGET_TRUTH_FIXTURE_DIR)
        .join(file_name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {path:?} must read: {err}"));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("fixture {path:?} must parse: {err}"))
}

fn assert_fixture_matches(file_name: &str) {
    let fixture = load_fixture(file_name);
    assert_eq!(
        fixture.record_kind, "navigation_target_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = NavigationTargetTruthPacket::materialize(fixture.input.clone());
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
        packet.row_class_tokens(),
        expect
            .row_class_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} row-class tokens drifted",
        fixture.case_name
    );
    assert_eq!(
        packet.access_kind_tokens(),
        expect
            .access_kind_tokens
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        "fixture {} access-kind tokens drifted",
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
    assert_exists(NAVIGATION_TARGET_TRUTH_SCHEMA_REF);
    assert_exists(NAVIGATION_TARGET_TRUTH_DOC_REF);
    assert_exists(NAVIGATION_TARGET_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(NAVIGATION_TARGET_TRUTH_FIXTURE_DIR);
    assert_exists(NAVIGATION_TARGET_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_stable_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn silent_relation_alias_fixture_blocks_stable() {
    assert_fixture_matches("silent_relation_alias_blocks_stable.json");
}

#[test]
fn reference_missing_access_context_fixture_blocks_stable() {
    assert_fixture_matches("reference_missing_access_context_blocks_stable.json");
}

#[test]
fn aliased_due_to_shallow_provider_missing_context_fixture_blocks_stable() {
    assert_fixture_matches("aliased_due_to_shallow_provider_missing_context_blocks_stable.json");
}

#[test]
fn consumer_projection_drops_access_kind_fixture_blocks_stable() {
    assert_fixture_matches("consumer_projection_drops_access_kind_blocks_stable.json");
}

#[test]
fn rename_preview_missing_context_fixture_blocks_stable() {
    assert_fixture_matches("rename_preview_missing_context_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_row_class() {
    let packet =
        current_stable_navigation_target_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        NavigationTargetPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in NavigationTargetRowClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.row_class == required),
            "stable packet must include row for row class {}",
            required.as_str()
        );
    }
    for required_access in NavigationAccessKind::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row
                .reference_context
                .as_ref()
                .is_some_and(|context| context.access_kind == required_access)),
            "stable packet must include reference row preserving access kind {}",
            required_access.as_str()
        );
    }
    for surface in NavigationTargetConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn closed_navigation_target_tokens_are_pinned() {
    assert_eq!(
        NavigationTargetRowClass::RenamePreview.as_str(),
        "rename_preview"
    );
    assert_eq!(
        NavigationAccessKind::RuntimeObserved.as_str(),
        "runtime_observed"
    );
    assert_eq!(
        NavigationTargetFindingKind::SilentRelationAliasPresent.as_str(),
        "silent_relation_alias_present"
    );
    assert_eq!(
        NavigationTargetPromotionState::BlocksStable.as_str(),
        "blocks_stable"
    );
}
