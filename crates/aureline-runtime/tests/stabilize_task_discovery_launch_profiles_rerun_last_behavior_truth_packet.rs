//! Fixture-driven coverage for the stable task-discovery / launch-profile
//! / rerun-last / task-event truth packet covering the local, remote_helper,
//! notebook, and imported_provider task-event lanes plus the four-wedge
//! admission coverage, the six envelope-field bindings, the four downstream
//! consumer-surface bindings, the additive-detail-preservation row, and the
//! lineage_admission row binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_task_event_truth_packet, TaskEventTruthConsumerSurface,
    TaskEventTruthDowngradeAutomationClass, TaskEventTruthDownstreamSurfaceClass,
    TaskEventTruthEnvelopeFieldClass, TaskEventTruthEvidenceClass, TaskEventTruthFindingKind,
    TaskEventTruthKnownLimitClass, TaskEventTruthLaneClass, TaskEventTruthPacket,
    TaskEventTruthPacketInput, TaskEventTruthPromotionState, TaskEventTruthRowClass,
    TaskEventTruthSupportClass, TaskEventTruthWedgeClass, TASK_EVENT_TRUTH_ARTIFACT_DOC_REF,
    TASK_EVENT_TRUTH_DOC_REF, TASK_EVENT_TRUTH_FIXTURE_DIR, TASK_EVENT_TRUTH_PACKET_ARTIFACT_REF,
    TASK_EVENT_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct TaskEventTruthFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: TaskEventTruthPacketInput,
    expect: ExpectedFixture,
}

#[derive(Debug, Deserialize)]
struct ExpectedFixture {
    promotion_state: String,
    validation_finding_count: usize,
    row_count: usize,
    lane_tokens: Vec<String>,
    row_class_tokens: Vec<String>,
    support_class_tokens: Vec<String>,
    wedge_tokens: Vec<String>,
    envelope_field_tokens: Vec<String>,
    downstream_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> TaskEventTruthFixture {
    let path = repo_root()
        .join(TASK_EVENT_TRUTH_FIXTURE_DIR)
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
        fixture.record_kind,
        "stabilize_task_discovery_launch_profiles_rerun_last_behavior_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = TaskEventTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(&packet.lane_tokens(), &expect.lane_tokens, "lane");
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
    assert_token_set_matches(&packet.wedge_tokens(), &expect.wedge_tokens, "wedge");
    assert_token_set_matches(
        &packet.envelope_field_tokens(),
        &expect.envelope_field_tokens,
        "envelope_field",
    );
    assert_token_set_matches(
        &packet.downstream_surface_tokens(),
        &expect.downstream_surface_tokens,
        "downstream_surface",
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
    assert_exists(TASK_EVENT_TRUTH_SCHEMA_REF);
    assert_exists(TASK_EVENT_TRUTH_DOC_REF);
    assert_exists(TASK_EVENT_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(TASK_EVENT_TRUTH_FIXTURE_DIR);
    assert_exists(TASK_EVENT_TRUTH_PACKET_ARTIFACT_REF);
}

#[test]
fn baseline_fixture_materializes_stable() {
    assert_fixture_matches("baseline_stable.json");
}

#[test]
fn launch_stable_with_unbound_evidence_blocks_stable() {
    assert_fixture_matches("launch_stable_with_unbound_evidence_blocks_stable.json");
}

#[test]
fn missing_envelope_field_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_envelope_field_for_launch_stable_blocks_stable.json");
}

#[test]
fn additive_detail_admits_flattening_blocks_stable() {
    assert_fixture_matches("additive_detail_admits_flattening_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_envelope_field_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_envelope_field_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_task_event_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, TaskEventTruthPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in TaskEventTruthLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for task-event lane {}",
            required.as_str()
        );
    }
    for surface in TaskEventTruthConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_fields_surfaces_and_admissions_per_launch_stable_lane(
) {
    let packet = current_stable_task_event_truth_packet().expect("checked-in packet validates");
    for required in TaskEventTruthLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == TaskEventTruthRowClass::TaskEventTruthQuality
                && row.support_class == TaskEventTruthSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in TaskEventTruthWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TaskEventTruthRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for field in TaskEventTruthEnvelopeFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TaskEventTruthRowClass::EnvelopeFieldBinding
                    && row.envelope_field_class == field),
                "stable packet must cover the {} envelope field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for surface in TaskEventTruthDownstreamSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == TaskEventTruthRowClass::SurfaceBinding
                    && row.downstream_surface_class == surface),
                "stable packet must cover the {} downstream surface on the {} lane",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == TaskEventTruthRowClass::AdditiveDetailPreservation
                && row.additive_detail_preserved),
            "stable packet must include an additive_detail_preservation row attesting preserved detail on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == TaskEventTruthRowClass::LineageAdmission
                    && row
                        .execution_context_id_binding
                        .as_deref()
                        .map(str::trim)
                        .map(|value| !value.is_empty())
                        .unwrap_or(false)
            }),
            "stable packet must include a lineage_admission row binding execution_context_id on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_task_event_truth_tokens_are_pinned() {
    assert_eq!(TaskEventTruthLaneClass::LocalLane.as_str(), "local_lane");
    assert_eq!(
        TaskEventTruthLaneClass::RemoteHelperLane.as_str(),
        "remote_helper_lane"
    );
    assert_eq!(
        TaskEventTruthLaneClass::NotebookLane.as_str(),
        "notebook_lane"
    );
    assert_eq!(
        TaskEventTruthLaneClass::ImportedProviderLane.as_str(),
        "imported_provider_lane"
    );
    assert_eq!(
        TaskEventTruthRowClass::TaskEventTruthQuality.as_str(),
        "task_event_truth_quality"
    );
    assert_eq!(
        TaskEventTruthSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        TaskEventTruthSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        TaskEventTruthSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        TaskEventTruthWedgeClass::TaskDiscovery.as_str(),
        "task_discovery"
    );
    assert_eq!(TaskEventTruthWedgeClass::RerunLast.as_str(), "rerun_last");
    assert_eq!(
        TaskEventTruthEnvelopeFieldClass::EventId.as_str(),
        "event_id"
    );
    assert_eq!(
        TaskEventTruthEnvelopeFieldClass::FallbackFlag.as_str(),
        "fallback_flag"
    );
    assert_eq!(
        TaskEventTruthDownstreamSurfaceClass::Problems.as_str(),
        "problems"
    );
    assert_eq!(
        TaskEventTruthDownstreamSurfaceClass::RerunSurface.as_str(),
        "rerun_surface"
    );
    assert_eq!(
        TaskEventTruthEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        TaskEventTruthKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        TaskEventTruthDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        TaskEventTruthConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        TaskEventTruthFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        TaskEventTruthFindingKind::AdditiveDetailRowAdmitsFlattening.as_str(),
        "additive_detail_row_admits_flattening"
    );
    assert_eq!(
        TaskEventTruthFindingKind::LineageAdmissionMissingExecutionContextId.as_str(),
        "lineage_admission_missing_execution_context_id"
    );
}
