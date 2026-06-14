//! Fixture-driven coverage for the stable provider-status surface truth
//! packet that binds the provider-status strip, capability-negotiation
//! drawer, and result-provenance pill across the M5 framework, notebook,
//! generated-source, preview, docs-linked, and structured-artifact
//! surfaces. Each strip names the acting provider lane, where it runs, and
//! its lifecycle state with an inspectable capability-detail route; each
//! drawer preserves the losing provider, names the selected result, scope
//! limit, freshness, and recovery action; and each pill keeps provenance
//! attached to a definition, reference, completion, rename preview, or
//! framework-aware result.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_language::{
    current_stable_provider_status_surface_truth_packet, ProviderStatusSurfaceClass,
    ProviderStatusSurfaceObjectKind, ProviderStatusSurfaceObjectRowClass,
    ProviderStatusSurfaceTruthPacket, ProviderStatusSurfaceTruthPacketInput,
    PROVIDER_STATUS_SURFACE_TRUTH_ARTIFACT_DOC_REF, PROVIDER_STATUS_SURFACE_TRUTH_DOC_REF,
    PROVIDER_STATUS_SURFACE_TRUTH_FIXTURE_DIR, PROVIDER_STATUS_SURFACE_TRUTH_PACKET_ARTIFACT_REF,
    PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SurfaceFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: ProviderStatusSurfaceTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    surface_lane_tokens: Vec<String>,
    object_kind_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    provider_family_tokens: Vec<String>,
    provider_locality_tokens: Vec<String>,
    provider_lifecycle_state_tokens: Vec<String>,
    provider_display_label_tokens: Vec<String>,
    capability_negotiation_tokens: Vec<String>,
    capability_detail_route_tokens: Vec<String>,
    participant_role_tokens: Vec<String>,
    conflict_tokens: Vec<String>,
    selected_result_form_tokens: Vec<String>,
    scope_limit_tokens: Vec<String>,
    freshness_tokens: Vec<String>,
    recovery_action_tokens: Vec<String>,
    provenance_anchor_target_tokens: Vec<String>,
    result_provenance_tokens: Vec<String>,
    completeness_tokens: Vec<String>,
    downgrade_label_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> SurfaceFixture {
    let path = repo_root()
        .join(PROVIDER_STATUS_SURFACE_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind, "provider_status_surface_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = ProviderStatusSurfaceTruthPacket::materialize(fixture.input.clone());
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

    assert_token_set_matches(
        &packet.surface_lane_tokens(),
        &expect.surface_lane_tokens,
        "surface_lane",
    );
    assert_token_set_matches(
        &packet.object_kind_tokens(),
        &expect.object_kind_tokens,
        "object_kind",
    );
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
        &packet.provider_family_tokens(),
        &expect.provider_family_tokens,
        "provider_family",
    );
    assert_token_set_matches(
        &packet.provider_locality_tokens(),
        &expect.provider_locality_tokens,
        "provider_locality",
    );
    assert_token_set_matches(
        &packet.provider_lifecycle_state_tokens(),
        &expect.provider_lifecycle_state_tokens,
        "provider_lifecycle_state",
    );
    assert_token_set_matches(
        &packet.provider_display_label_tokens(),
        &expect.provider_display_label_tokens,
        "provider_display_label",
    );
    assert_token_set_matches(
        &packet.capability_negotiation_tokens(),
        &expect.capability_negotiation_tokens,
        "capability_negotiation",
    );
    assert_token_set_matches(
        &packet.capability_detail_route_tokens(),
        &expect.capability_detail_route_tokens,
        "capability_detail_route",
    );
    assert_token_set_matches(
        &packet.participant_role_tokens(),
        &expect.participant_role_tokens,
        "participant_role",
    );
    assert_token_set_matches(
        &packet.conflict_tokens(),
        &expect.conflict_tokens,
        "conflict",
    );
    assert_token_set_matches(
        &packet.selected_result_form_tokens(),
        &expect.selected_result_form_tokens,
        "selected_result_form",
    );
    assert_token_set_matches(
        &packet.scope_limit_tokens(),
        &expect.scope_limit_tokens,
        "scope_limit",
    );
    assert_token_set_matches(
        &packet.freshness_tokens(),
        &expect.freshness_tokens,
        "freshness",
    );
    assert_token_set_matches(
        &packet.recovery_action_tokens(),
        &expect.recovery_action_tokens,
        "recovery_action",
    );
    assert_token_set_matches(
        &packet.provenance_anchor_target_tokens(),
        &expect.provenance_anchor_target_tokens,
        "provenance_anchor_target",
    );
    assert_token_set_matches(
        &packet.result_provenance_tokens(),
        &expect.result_provenance_tokens,
        "result_provenance",
    );
    assert_token_set_matches(
        &packet.completeness_tokens(),
        &expect.completeness_tokens,
        "completeness",
    );
    assert_token_set_matches(
        &packet.downgrade_label_tokens(),
        &expect.downgrade_label_tokens,
        "downgrade_label",
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
        "2026-06-14T12:00:10Z",
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
    assert_exists(PROVIDER_STATUS_SURFACE_TRUTH_SCHEMA_REF);
    assert_exists(PROVIDER_STATUS_SURFACE_TRUTH_DOC_REF);
    assert_exists(PROVIDER_STATUS_SURFACE_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(PROVIDER_STATUS_SURFACE_TRUTH_FIXTURE_DIR);
    assert_exists(PROVIDER_STATUS_SURFACE_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn certified_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("certified_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn status_strip_missing_lane_state_blocks_stable() {
    assert_fixture_matches("status_strip_missing_lane_state_blocks_stable.json");
}

#[test]
fn opaque_spinner_detail_route_blocks_stable() {
    assert_fixture_matches("opaque_spinner_detail_route_blocks_stable.json");
}

#[test]
fn losing_provider_not_preserved_blocks_stable() {
    assert_fixture_matches("losing_provider_not_preserved_blocks_stable.json");
}

#[test]
fn raw_process_name_only_label_blocks_stable() {
    assert_fixture_matches("raw_process_name_only_label_blocks_stable.json");
}

#[test]
fn dimension_bound_on_wrong_row_class_blocks_stable() {
    assert_fixture_matches("dimension_bound_on_wrong_row_class_blocks_stable.json");
}

#[test]
fn projection_collapses_result_provenance_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_result_provenance_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_surface() {
    let packet =
        current_stable_provider_status_surface_truth_packet().expect("checked-in packet validates");
    assert!(packet.validate().is_empty());
    for surface in ProviderStatusSurfaceClass::REQUIRED {
        for kind in ProviderStatusSurfaceObjectKind::REQUIRED {
            assert!(
                packet.rows.iter().any(|row| row.surface_lane == surface
                    && row.object_kind == kind
                    && row.row_class == ProviderStatusSurfaceObjectRowClass::SurfaceObjectPresence),
                "stable packet must include a {} presence row for surface {}",
                kind.as_str(),
                surface.as_str()
            );
        }
    }
}
