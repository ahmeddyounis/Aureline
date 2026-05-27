//! Fixture-driven coverage for the stable harden-environment-capsule
//! resolution truth packet covering the devcontainer, Nix, Compose,
//! shell/SDK, and template/prebuild lanes plus the nine typed capsule
//! field admissions, the six prebuild-fingerprint admissions, the six
//! invalidation-reason admissions, the five Project Doctor finding
//! admissions, and the materialized-identity admission binding both
//! the requested artifact and the materialized runtime instance.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_runtime::{
    current_stable_capsule_resolution_truth_packet, CapsuleFieldClass,
    CapsuleResolutionConsumerSurface, CapsuleResolutionDowngradeAutomationClass,
    CapsuleResolutionEvidenceClass, CapsuleResolutionFindingKind,
    CapsuleResolutionKnownLimitClass, CapsuleResolutionLaneClass,
    CapsuleResolutionPromotionState, CapsuleResolutionRowClass, CapsuleResolutionSupportClass,
    CapsuleResolutionTruthPacket, CapsuleResolutionTruthPacketInput, InvalidationReasonClass,
    PrebuildFingerprintComponentClass, ProjectDoctorFindingClass,
    CAPSULE_RESOLUTION_TRUTH_ARTIFACT_DOC_REF, CAPSULE_RESOLUTION_TRUTH_DOC_REF,
    CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR, CAPSULE_RESOLUTION_TRUTH_PACKET_ARTIFACT_REF,
    CAPSULE_RESOLUTION_TRUTH_SCHEMA_REF,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CapsuleResolutionFixture {
    record_kind: String,
    schema_version: u32,
    case_name: String,
    scenario: String,
    input: CapsuleResolutionTruthPacketInput,
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
    capsule_field_tokens: Vec<String>,
    prebuild_fingerprint_tokens: Vec<String>,
    invalidation_reason_tokens: Vec<String>,
    project_doctor_finding_tokens: Vec<String>,
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

fn load_fixture(file_name: &str) -> CapsuleResolutionFixture {
    let path = repo_root()
        .join(CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR)
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
        "harden_environment_capsule_resolution_truth_stable_case",
        "fixture {file_name} declares unexpected record_kind",
    );
    assert_eq!(fixture.schema_version, 1);
    assert!(
        !fixture.scenario.trim().is_empty(),
        "fixture {} scenario must describe what the case proves",
        fixture.case_name
    );

    let expect = &fixture.expect;
    let packet = CapsuleResolutionTruthPacket::materialize(fixture.input.clone());
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
    assert_token_set_matches(
        &packet.capsule_field_tokens(),
        &expect.capsule_field_tokens,
        "capsule_field",
    );
    assert_token_set_matches(
        &packet.prebuild_fingerprint_tokens(),
        &expect.prebuild_fingerprint_tokens,
        "prebuild_fingerprint",
    );
    assert_token_set_matches(
        &packet.invalidation_reason_tokens(),
        &expect.invalidation_reason_tokens,
        "invalidation_reason",
    );
    assert_token_set_matches(
        &packet.project_doctor_finding_tokens(),
        &expect.project_doctor_finding_tokens,
        "project_doctor_finding",
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
    assert_exists(CAPSULE_RESOLUTION_TRUTH_SCHEMA_REF);
    assert_exists(CAPSULE_RESOLUTION_TRUTH_DOC_REF);
    assert_exists(CAPSULE_RESOLUTION_TRUTH_ARTIFACT_DOC_REF);
    assert_exists(CAPSULE_RESOLUTION_TRUTH_FIXTURE_DIR);
    assert_exists(CAPSULE_RESOLUTION_TRUTH_PACKET_ARTIFACT_REF);
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
fn missing_prebuild_fingerprint_component_blocks_stable() {
    assert_fixture_matches("missing_prebuild_fingerprint_component_blocks_stable.json");
}

#[test]
fn materialized_identity_admits_silent_prebuild_reuse_blocks_stable() {
    assert_fixture_matches(
        "materialized_identity_admits_silent_prebuild_reuse_blocks_stable.json",
    );
}

#[test]
fn narrowed_row_missing_disclosure_ref_blocks_stable() {
    assert_fixture_matches("narrowed_row_missing_disclosure_ref_blocks_stable.json");
}

#[test]
fn projection_collapses_invalidation_reason_vocabulary_blocks_stable() {
    assert_fixture_matches(
        "projection_collapses_invalidation_reason_vocabulary_blocks_stable.json",
    );
}

#[test]
fn raw_source_material_blocks_stable() {
    assert_fixture_matches("raw_source_material_blocks_stable.json");
}

#[test]
fn checked_in_artifact_packet_validates_and_covers_every_required_lane() {
    let packet = current_stable_capsule_resolution_truth_packet()
        .expect("checked-in packet validates");
    assert_eq!(
        packet.promotion_state,
        CapsuleResolutionPromotionState::Stable
    );
    assert!(packet.validate().is_empty());
    for required in CapsuleResolutionLaneClass::REQUIRED {
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required),
            "stable packet must include row for capsule-resolution lane {}",
            required.as_str()
        );
    }
    for surface in CapsuleResolutionConsumerSurface::REQUIRED {
        assert!(
            packet.has_projection_for(surface),
            "stable packet must preserve {} consumer projection",
            surface.as_str()
        );
    }
}

#[test]
fn checked_in_artifact_covers_required_admissions_per_launch_stable_lane() {
    let packet = current_stable_capsule_resolution_truth_packet()
        .expect("checked-in packet validates");
    for required in CapsuleResolutionLaneClass::REQUIRED {
        let lane_claims_launch = packet.rows.iter().any(|row| {
            row.lane_class == required
                && row.row_class == CapsuleResolutionRowClass::CapsuleResolutionQuality
                && row.support_class == CapsuleResolutionSupportClass::LaunchStable
        });
        if !lane_claims_launch {
            continue;
        }
        for field in CapsuleFieldClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CapsuleResolutionRowClass::CapsuleFieldAdmission
                    && row.capsule_field_class == field),
                "stable packet must cover the {} capsule field on the {} lane",
                field.as_str(),
                required.as_str()
            );
        }
        for component in PrebuildFingerprintComponentClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CapsuleResolutionRowClass::PrebuildFingerprintAdmission
                    && row.prebuild_fingerprint_component_class == component),
                "stable packet must cover the {} prebuild fingerprint component on the {} lane",
                component.as_str(),
                required.as_str()
            );
        }
        for reason in InvalidationReasonClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class == CapsuleResolutionRowClass::InvalidationReasonAdmission
                    && row.invalidation_reason_class == reason),
                "stable packet must cover the {} invalidation reason on the {} lane",
                reason.as_str(),
                required.as_str()
            );
        }
        for finding in ProjectDoctorFindingClass::REQUIRED_FOR_LAUNCH_STABLE {
            assert!(
                packet.rows.iter().any(|row| row.lane_class == required
                    && row.row_class
                        == CapsuleResolutionRowClass::ProjectDoctorFindingAdmission
                    && row.project_doctor_finding_class == finding),
                "stable packet must cover the {} Project Doctor finding on the {} lane",
                finding.as_str(),
                required.as_str()
            );
        }
        assert!(
            packet.rows.iter().any(|row| row.lane_class == required
                && row.row_class == CapsuleResolutionRowClass::MaterializedIdentityAdmission
                && row.no_silent_prebuild_reuse
                && row
                    .requested_artifact_identity_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)
                && row
                    .materialized_runtime_identity_binding
                    .as_deref()
                    .map(str::trim)
                    .map(|value| !value.is_empty())
                    .unwrap_or(false)),
            "stable packet must include a materialized_identity_admission row binding both requested + materialized identity and attesting no_silent_prebuild_reuse on the {} lane",
            required.as_str()
        );
    }
}

#[test]
fn closed_capsule_resolution_tokens_are_pinned() {
    assert_eq!(
        CapsuleResolutionLaneClass::DevcontainerLane.as_str(),
        "devcontainer_lane"
    );
    assert_eq!(
        CapsuleResolutionLaneClass::TemplatePrebuildLane.as_str(),
        "template_prebuild_lane"
    );
    assert_eq!(
        CapsuleResolutionRowClass::CapsuleResolutionQuality.as_str(),
        "capsule_resolution_quality"
    );
    assert_eq!(
        CapsuleResolutionRowClass::MaterializedIdentityAdmission.as_str(),
        "materialized_identity_admission"
    );
    assert_eq!(
        CapsuleResolutionSupportClass::LaunchStable.as_str(),
        "launch_stable"
    );
    assert_eq!(
        CapsuleResolutionSupportClass::SupportUnbound.as_str(),
        "support_unbound"
    );
    assert_eq!(
        CapsuleFieldClass::HostOrBaseImageIdentity.as_str(),
        "host_or_base_image_identity"
    );
    assert_eq!(CapsuleFieldClass::Provenance.as_str(), "provenance");
    assert_eq!(
        PrebuildFingerprintComponentClass::CommitOrTreeIdentity.as_str(),
        "commit_or_tree_identity"
    );
    assert_eq!(
        PrebuildFingerprintComponentClass::CriticalToolchainDigest.as_str(),
        "critical_toolchain_digest"
    );
    assert_eq!(InvalidationReasonClass::ColdPath.as_str(), "cold_path");
    assert_eq!(
        InvalidationReasonClass::StalePrebuild.as_str(),
        "stale_prebuild"
    );
    assert_eq!(
        ProjectDoctorFindingClass::WrongInterpreter.as_str(),
        "wrong_interpreter"
    );
    assert_eq!(
        ProjectDoctorFindingClass::UntrustedTemplateMetadata.as_str(),
        "untrusted_template_metadata"
    );
    assert_eq!(
        CapsuleResolutionEvidenceClass::EvidenceUnbound.as_str(),
        "evidence_unbound"
    );
    assert_eq!(
        CapsuleResolutionKnownLimitClass::LimitUnbound.as_str(),
        "limit_unbound"
    );
    assert_eq!(
        CapsuleResolutionDowngradeAutomationClass::AutomationUnbound.as_str(),
        "automation_unbound"
    );
    assert_eq!(
        CapsuleResolutionConsumerSurface::ProjectDoctor.as_str(),
        "project_doctor"
    );
    assert_eq!(
        CapsuleResolutionConsumerSurface::ConformanceDashboard.as_str(),
        "conformance_dashboard"
    );
    assert_eq!(
        CapsuleResolutionFindingKind::LaunchStableWithUnboundBinding.as_str(),
        "launch_stable_with_unbound_binding"
    );
    assert_eq!(
        CapsuleResolutionFindingKind::MaterializedIdentityAdmissionAdmitsSilentPrebuildReuse
            .as_str(),
        "materialized_identity_admission_admits_silent_prebuild_reuse"
    );
    assert_eq!(
        CapsuleResolutionFindingKind::InvalidationReasonVocabularyCollapsed.as_str(),
        "invalidation_reason_vocabulary_collapsed"
    );
}
