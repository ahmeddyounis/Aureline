//! Fixture-driven coverage for the stable profiler and tracing-hooks
//! truth packet covering the local, remote_helper, container, and
//! ci_import profiler lanes plus the eight-wedge admission coverage,
//! the six capture-state admissions, the four origin-class admissions,
//! the two build-mode admissions, the two run-class admissions,
//! the three confounder admissions, the five replay-state admissions,
//! the six surface bindings, and the lineage_admission row binding
//! `execution_context_id`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_profiler_truth_packet, ProfilerBuildModeClass, ProfilerCaptureStateClass,
    ProfilerConfounderClass, ProfilerConsumerProjectionSurface, ProfilerLaneClass,
    ProfilerOriginClass, ProfilerPromotionState, ProfilerReplayStateClass, ProfilerRowClass,
    ProfilerRunClassClass, ProfilerSurfaceClass, ProfilerTruthPacket, ProfilerTruthPacketInput,
    ProfilerWedgeClass, PROFILER_TRUTH_ARTIFACT_DOC_REF, PROFILER_TRUTH_DOC_REF,
    PROFILER_TRUTH_FIXTURE_DIR, PROFILER_TRUTH_PACKET_ARTIFACT_REF, PROFILER_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ProfilerFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: ProfilerTruthPacketInput,
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
    capture_state_tokens: Vec<String>,
    origin_class_tokens: Vec<String>,
    build_mode_tokens: Vec<String>,
    run_class_tokens: Vec<String>,
    confounder_tokens: Vec<String>,
    replay_state_tokens: Vec<String>,
    profiler_surface_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> ProfilerFixture {
    let path = repo_root()
        .join(PROFILER_TRUTH_FIXTURE_DIR)
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
        "harden_the_stable_profiler_and_tracing_hooks_needed_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = ProfilerTruthPacket::materialize(fixture.input.clone());
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
        &packet.capture_state_tokens(),
        &expect.capture_state_tokens,
        "capture_state",
    );
    assert_token_set_matches(
        &packet.origin_class_tokens(),
        &expect.origin_class_tokens,
        "origin_class",
    );
    assert_token_set_matches(
        &packet.build_mode_tokens(),
        &expect.build_mode_tokens,
        "build_mode",
    );
    assert_token_set_matches(
        &packet.run_class_tokens(),
        &expect.run_class_tokens,
        "run_class",
    );
    assert_token_set_matches(
        &packet.confounder_tokens(),
        &expect.confounder_tokens,
        "confounder",
    );
    assert_token_set_matches(
        &packet.replay_state_tokens(),
        &expect.replay_state_tokens,
        "replay_state",
    );
    assert_token_set_matches(
        &packet.profiler_surface_tokens(),
        &expect.profiler_surface_tokens,
        "profiler_surface",
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
        "2026-05-27T12:00:10Z",
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
    assert_exists(PROFILER_TRUTH_SCHEMA_REF);
    assert_exists(PROFILER_TRUTH_DOC_REF);
    assert_exists(PROFILER_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(PROFILER_TRUTH_FIXTURE_DIR);
    assert_exists(PROFILER_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_capture_state_for_launch_stable_blocks_stable() {
    assert_fixture_matches("missing_capture_state_for_launch_stable_blocks_stable.json");
}

#[test]
fn surface_missing_capture_state_attestation_blocks_stable() {
    assert_fixture_matches("surface_missing_capture_state_attestation_blocks_stable.json");
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_capture_state_vocabulary_blocks_stable() {
    assert_fixture_matches("projection_collapses_capture_state_vocabulary_blocks_stable.json");
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_profiler_truth_packet().expect("checked-in packet validates");
    assert_eq!(packet.promotion_state, ProfilerPromotionState::Stable);
    assert!(packet.validate().is_empty());
    for required in ProfilerLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for profiler lane {}",
            required.as_str()
        );
    }
    for surface in ProfilerConsumerProjectionSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_admissions_per_launch_stable_lane() {
    let packet = current_stable_profiler_truth_packet().expect("checked-in packet validates");
    for required in ProfilerLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == ProfilerRowClass::ProfilerQuality
                && row.support_class == aureline_runtime::ProfilerSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for wedge in ProfilerWedgeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::WedgeAdmission
                    && row.wedge_class == wedge),
                "stable packet must cover the {} wedge admission on the {} lane",
                wedge.as_str(),
                required.as_str()
            );
        }
        for state in ProfilerCaptureStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::CaptureStateAdmission
                    && row.capture_state_class == state),
                "stable packet must cover the {} capture state on the {} lane",
                state.as_str(),
                required.as_str()
            );
        }
        for origin in ProfilerOriginClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::OriginClassAdmission
                    && row.origin_class == origin),
                "stable packet must cover the {} origin class on the {} lane",
                origin.as_str(),
                required.as_str()
            );
        }
        for mode in ProfilerBuildModeClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::BuildModeAdmission
                    && row.build_mode_class == mode),
                "stable packet must cover the {} build mode on the {} lane",
                mode.as_str(),
                required.as_str()
            );
        }
        for run_class in ProfilerRunClassClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::RunClassAdmission
                    && row.run_class_class == run_class),
                "stable packet must cover the {} run class on the {} lane",
                run_class.as_str(),
                required.as_str()
            );
        }
        for confounder in ProfilerConfounderClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::ConfounderAdmission
                    && row.confounder_class == confounder),
                "stable packet must cover the {} confounder on the {} lane",
                confounder.as_str(),
                required.as_str()
            );
        }
        for replay_state in ProfilerReplayStateClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::ReplayStateAdmission
                    && row.replay_state_class == replay_state),
                "stable packet must cover the {} replay state on the {} lane",
                replay_state.as_str(),
                required.as_str()
            );
        }
        for surface in ProfilerSurfaceClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == ProfilerRowClass::SurfaceBinding
                    && row.profiler_surface_class == surface
                    && (!surface.requires_capture_state_attestation()
                        || row.attests_capture_state_preserved)
                    && (!surface.requires_replay_state_attestation()
                        || row.attests_replay_state_preserved)),
                "stable packet must cover the {} profiler surface on the {} lane with required attestations",
                surface.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| {
                row.lane_class == required
                    && row.row_class == ProfilerRowClass::LineageAdmission
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
fn closed_profiler_truth_tokens_are_pinned() {
    assert_eq!(ProfilerLaneClass::LocalLane.as_str(), "local_lane");
    assert_eq!(
        ProfilerLaneClass::CiImportLane.as_str(),
        "ci_import_lane"
    );
    assert_eq!(
        ProfilerRowClass::ProfilerQuality.as_str(),
        "profiler_quality"
    );
    assert_eq!(
        ProfilerRowClass::CaptureStateAdmission.as_str(),
        "capture_state_admission"
    );
    assert_eq!(
        ProfilerRowClass::OriginClassAdmission.as_str(),
        "origin_class_admission"
    );
    assert_eq!(
        ProfilerRowClass::BuildModeAdmission.as_str(),
        "build_mode_admission"
    );
    assert_eq!(
        ProfilerRowClass::RunClassAdmission.as_str(),
        "run_class_admission"
    );
    assert_eq!(
        ProfilerRowClass::ConfounderAdmission.as_str(),
        "confounder_admission"
    );
    assert_eq!(
        ProfilerRowClass::ReplayStateAdmission.as_str(),
        "replay_state_admission"
    );
    assert_eq!(
        ProfilerRowClass::SurfaceBinding.as_str(),
        "surface_binding"
    );
    assert_eq!(
        ProfilerWedgeClass::ProfileSessionDescriptor.as_str(),
        "profile_session_descriptor"
    );
    assert_eq!(
        ProfilerWedgeClass::ExportRedactionSummary.as_str(),
        "export_redaction_summary"
    );
    assert_eq!(ProfilerCaptureStateClass::Live.as_str(), "live");
    assert_eq!(
        ProfilerCaptureStateClass::DisabledWithReason.as_str(),
        "disabled_with_reason"
    );
    assert_eq!(ProfilerOriginClass::LocalOrigin.as_str(), "local_origin");
    assert_eq!(
        ProfilerOriginClass::ImportedBundleOrigin.as_str(),
        "imported_bundle_origin"
    );
    assert_eq!(ProfilerBuildModeClass::DebugMode.as_str(), "debug_mode");
    assert_eq!(ProfilerBuildModeClass::ReleaseMode.as_str(), "release_mode");
    assert_eq!(ProfilerRunClassClass::WarmRun.as_str(), "warm_run");
    assert_eq!(ProfilerRunClassClass::ColdRun.as_str(), "cold_run");
    assert_eq!(ProfilerConfounderClass::HardwareClass.as_str(), "hardware_class");
    assert_eq!(ProfilerConfounderClass::ThermalState.as_str(), "thermal_state");
    assert_eq!(ProfilerReplayStateClass::Supported.as_str(), "supported");
    assert_eq!(
        ProfilerReplayStateClass::ImportViewOnly.as_str(),
        "import_view_only"
    );
    assert_eq!(
        ProfilerSurfaceClass::FlamegraphSurface.as_str(),
        "flamegraph_surface"
    );
    assert_eq!(
        ProfilerSurfaceClass::ProfileSessionSurface.as_str(),
        "profile_session_surface"
    );
    assert_eq!(
        ProfilerConsumerProjectionSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
}
