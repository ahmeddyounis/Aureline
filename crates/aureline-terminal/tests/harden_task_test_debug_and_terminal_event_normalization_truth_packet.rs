//! Fixture-driven coverage for the stable event-normalization truth
//! packet covering the task, test, debug, and terminal lanes plus the
//! four-wedge admission coverage, the ten envelope-field bindings, the
//! five source-kind bindings, the nine lifecycle-event bindings, the
//! nine consumer-surface bindings, the raw_payload_retention_attestation
//! row, and the lineage_admission row binding `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_terminal::{
    current_stable_event_normalization_truth_packet, EventNormalizationConsumerSurface,
    EventNormalizationConsumerSurfaceBindingClass, EventNormalizationDowngradeAutomationClass,
    EventNormalizationEnvelopeFieldClass, EventNormalizationEvidenceClass,
    EventNormalizationFindingKind, EventNormalizationKnownLimitClass, EventNormalizationLaneClass,
    EventNormalizationLifecycleEventClass, EventNormalizationPromotionState,
    EventNormalizationRowClass, EventNormalizationSourceKindClass, EventNormalizationSupportClass,
    EventNormalizationTruthPacket, EventNormalizationTruthPacketInput,
    EventNormalizationWedgeClass, EVENT_NORMALIZATION_TRUTH_ARTIFACT_DOC_REF,
    EVENT_NORMALIZATION_TRUTH_DOC_REF, EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR,
    EVENT_NORMALIZATION_TRUTH_PACKET_ARTIFACT_REF, EVENT_NORMALIZATION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EventNormalizationFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: EventNormalizationTruthPacketInput,
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
    source_kind_tokens: Vec<String>,
    lifecycle_event_tokens: Vec<String>,
    consumer_surface_binding_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> EventNormalizationFixture {
    let path = repo_root()
        .join(EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR)
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
        "harden_task_test_debug_and_terminal_event_normalization_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = EventNormalizationTruthPacket::materialize(fixture.input.clone());
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
        &packet.source_kind_tokens(),
        &expect.source_kind_tokens,
        "source_kind",
    );
    assert_token_set_matches(
        &packet.lifecycle_event_tokens(),
        &expect.lifecycle_event_tokens,
        "lifecycle_event",
    );
    assert_token_set_matches(
        &packet.consumer_surface_binding_tokens(),
        &expect.consumer_surface_binding_tokens,
        "consumer_surface_binding",
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
    assert_exists(EVENT_NORMALIZATION_TRUTH_SCHEMA_REF);
    assert_exists(EVENT_NORMALIZATION_TRUTH_DOC_REF);
    assert_exists(EVENT_NORMALIZATION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(EVENT_NORMALIZATION_TRUTH_FIXTURE_DIR);
    assert_exists(EVENT_NORMALIZATION_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_source_kind_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_source_kind_for_launch_stable_blocks_stable.json");
}

#[test]
fn missing_lifecycle_event_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_lifecycle_event_for_launch_stable_blocks_stable.json");
}

#[test]
fn retention_admits_flattening_blocks_stable() {
    assert_fixture_matches("retention_admits_flattening_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_source_kind_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_source_kind_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet =
        current_stable_event_normalization_truth_packet().expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        EventNormalizationPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in EventNormalizationLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for event-normalization lane {}",
            required.as_str()
        );
    }
    for surface in EventNormalizationConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_wedges_fields_kinds_events_surfaces_and_admissions_per_launch_stable_lane(
) {
    let packet =
        current_stable_event_normalization_truth_packet().expect("checked-in packet validates");
    for required in EventNormalizationLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == EventNormalizationRowClass::EventNormalizationQuality
                && row.support_class == EventNormalizationSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in EventNormalizationWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for field in EventNormalizationEnvelopeFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::EnvelopeFieldBinding
                    && row.envelope_field_class == field),
                "stable packet must cover the {} envelope field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for kind in EventNormalizationSourceKindClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::SourceKindBinding
                    && row.source_kind_class == kind),
                "stable packet must cover the {} source kind on the {} lane",
                kind.as_str(),
                required.as_str()
            );
        }
        for event in EventNormalizationLifecycleEventClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::LifecycleEventBinding
                    && row.lifecycle_event_class == event),
                "stable packet must cover the {} lifecycle event on the {} lane",
                event.as_str(),
                required.as_str()
            );
        }
        for surface in EventNormalizationConsumerSurfaceBindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::ConsumerSurfaceBinding
                    && row.consumer_surface_class == surface),
                "stable packet must cover the {} consumer surface on the {} lane",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == EventNormalizationRowClass::RawPayloadRetentionAttestation
                && row.attests_raw_payload_retained),
            "stable packet must include a raw_payload_retention_attestation row attesting preserved retention on the {} lane",
            required.as_str()
        );
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == EventNormalizationRowClass::LineageAdmission
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
fn closed_event_normalization_tokens_are_pinned() {
    assert_eq!(EventNormalizationLaneClass::TaskLane.as_str(), "task_lane");
    assert_eq!(EventNormalizationLaneClass::TestLane.as_str(), "test_lane");
    assert_eq!(
        EventNormalizationLaneClass::DebugLane.as_str(),
        "debug_lane"
    );
    assert_eq!(
        EventNormalizationLaneClass::TerminalLane.as_str(),
        "terminal_lane"
    );
    assert_eq!(
        EventNormalizationRowClass::EventNormalizationQuality.as_str(),
        "event_normalization_quality"
    );
    assert_eq!(
        EventNormalizationSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        EventNormalizationSupportClass::LaunchStableBelow.as_str(),
        "launch_stable_below"
    );
    assert_eq!(
        EventNormalizationSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        EventNormalizationWedgeClass::EnvelopeCanonicalization.as_str(),
        "envelope_canonicalization"
    );
    assert_eq!(
        EventNormalizationWedgeClass::ExportPreservation.as_str(),
        "export_preservation"
    );
    assert_eq!(
        EventNormalizationEnvelopeFieldClass::EventId.as_str(),
        "event_id"
    );
    assert_eq!(
        EventNormalizationEnvelopeFieldClass::RawPayloadRef.as_str(),
        "raw_payload_ref"
    );
    assert_eq!(EventNormalizationSourceKindClass::Native.as_str(), "native");
    assert_eq!(
        EventNormalizationSourceKindClass::BazelBep.as_str(),
        "bazel_bep"
    );
    assert_eq!(
        EventNormalizationSourceKindClass::HeuristicParser.as_str(),
        "heuristic_parser"
    );
    assert_eq!(
        EventNormalizationLifecycleEventClass::TaskQueued.as_str(),
        "task_queued"
    );
    assert_eq!(
        EventNormalizationLifecycleEventClass::TaskFinished.as_str(),
        "task_finished"
    );
    assert_eq!(
        EventNormalizationConsumerSurfaceBindingClass::AiToolSurface.as_str(),
        "ai_tool_surface"
    );
    assert_eq!(
        EventNormalizationEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        EventNormalizationKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        EventNormalizationDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        EventNormalizationConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        EventNormalizationFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        EventNormalizationFindingKind::RawPayloadRetentionAttestationAdmitsFlattening.as_str(),
        "raw_payload_retention_attestation_admits_flattening"
    );
    assert_eq!(
        EventNormalizationFindingKind::LineageAdmissionMissingExecutionContextId.as_str(),
        "lineage_admission_missing_execution_context_id"
    );
}
