//! Inline unit coverage for the event-interop certification packet: seed
//! stability, the claimed profile matrix, the eight graded certification
//! dimensions, the freshness/stale-narrowing roll-up, the certification index,
//! and the fail-closed guardrails against private session histories, overclaimed
//! confidence, lost raw payloads, missing fallback reasons, hidden degraded
//! states, broken replay, and broken export parity.

use super::*;

fn seed() -> EventInteropCertificationPacket {
    seeded_event_interop_certification_packet()
}

fn profile_mut(
    input: &mut EventInteropCertificationPacketInput,
    profile: ToolingProfile,
) -> &mut ToolingProfileCertification {
    input
        .profiles
        .iter_mut()
        .find(|row| row.profile == profile)
        .expect("profile present")
}

#[test]
fn seed_materializes_stable() {
    let packet = seed();
    assert!(
        packet.validate().is_empty(),
        "seed must validate clean: {:?}",
        packet.validate()
    );
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::Stable
    );
    assert_eq!(packet.record_kind, EVENT_INTEROP_CERTIFICATION_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        EVENT_INTEROP_CERTIFICATION_SCHEMA_VERSION
    );
}

#[test]
fn seed_carries_every_claimed_profile() {
    let packet = seed();
    assert_eq!(
        packet.profile_tokens(),
        vec![
            "task_center_run",
            "test_session",
            "debug_session",
            "pipeline_overlay",
            "notebook_run",
            "coverage_intelligence",
        ]
    );
}

#[test]
fn every_seed_profile_certifies_on_all_dimensions() {
    let packet = seed();
    for row in &packet.profiles {
        assert!(
            row.certified,
            "profile {} must certify",
            row.profile.as_str()
        );
        assert_eq!(row.claim_state, ProfileClaimState::Claimable);
        assert_eq!(
            row.dimension_outcomes.len(),
            CertificationDimension::ALL.len()
        );
        for outcome in &row.dimension_outcomes {
            assert!(
                outcome.passed,
                "profile {} dimension {} must pass",
                row.profile.as_str(),
                outcome.dimension.as_str()
            );
        }
    }
}

#[test]
fn every_seed_profile_reads_the_canonical_envelope_and_cites_evidence() {
    let packet = seed();
    for row in &packet.profiles {
        assert_eq!(
            row.consumer_truth_source,
            ConsumerTruthSource::CanonicalEventEnvelope
        );
        assert_eq!(
            row.evidence_refs.len(),
            EVENT_INTEROP_CERTIFICATION_EVIDENCE_REFS.len()
        );
        assert!(!row.evidence_refs.is_empty());
    }
}

#[test]
fn pipeline_overlay_certifies_a_degraded_low_confidence_imported_path() {
    let packet = seed();
    let row = packet
        .profile_for(ToolingProfile::PipelineOverlay)
        .expect("pipeline overlay present");
    assert_eq!(
        row.primary_source_kind,
        BuildTestEventSourceKind::HeuristicParser
    );
    assert_eq!(row.negotiated_capability, AdapterCapabilityState::Degraded);
    assert_eq!(row.observed_confidence, BuildTestEventConfidence::Low);
    assert!(row.fallback_reason.is_some());
    assert!(row.degraded_state_disclosed);
    assert!(row.certified);
}

#[test]
fn seed_certification_index_is_current_and_certified() {
    let packet = seed();
    assert!(packet.certification_index.all_profiles_current);
    assert!(packet.certification_index.all_profiles_certified);
    assert_eq!(
        packet.certification_index.claimable_profiles.len(),
        ToolingProfile::ALL.len()
    );
    assert!(packet.certification_index.narrowed_profiles.is_empty());
    assert!(packet.certification_index.blocked_profiles.is_empty());
    assert_eq!(
        packet.certification_index.certification_ref,
        EVENT_INTEROP_CERTIFICATION_INDEX_REF
    );
}

#[test]
fn evidence_joins_explain_consistently() {
    let packet = seed();
    for surface in [
        CertificationEvidenceSurface::SupportBundle,
        CertificationEvidenceSurface::IncidentPacket,
        CertificationEvidenceSurface::AiEvidence,
    ] {
        let view = packet.evidence_join(surface, "view", "2026-06-18T00:01:00Z");
        assert!(
            view.explains_consistently(),
            "{} must explain",
            surface.as_str()
        );
        assert_eq!(view.profile_rows.len(), packet.profiles.len());
        assert_eq!(view.profile_digest, packet.profile_digest);
        assert_eq!(view.certification_index, packet.certification_index);
    }
}

#[test]
fn cli_headless_view_explains_every_profile() {
    let packet = seed();
    let view = packet.cli_headless_view(
        EVENT_INTEROP_CERTIFICATION_CLI_HEADLESS_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(view.every_profile_explained());
    assert_eq!(view.profile_rows.len(), packet.profiles.len());
    assert_eq!(view.profile_digest, packet.profile_digest);
    assert_eq!(view.promotion_state, packet.promotion_state);
}

#[test]
fn support_export_round_trips_and_stays_safe() {
    let packet = seed();
    let export = packet.support_export(
        EVENT_INTEROP_CERTIFICATION_SUPPORT_EXPORT_ID,
        "2026-06-18T00:01:00Z",
    );
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("serialize");
    let parsed: EventInteropCertificationSupportExport =
        serde_json::from_str(&json).expect("parse");
    assert_eq!(parsed, export);
    assert!(parsed.packet.is_stable());
    assert_eq!(parsed.packet.profile_digest, packet.profile_digest);
}

#[test]
fn private_session_history_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::TaskCenterRun).consumer_truth_source =
        ConsumerTruthSource::PrivateSessionHistory;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::EventEnvelopeNotReused));
    assert!(packet
        .certification_index
        .blocked_profiles
        .contains(&ToolingProfile::TaskCenterRun.as_str().to_owned()));
}

#[test]
fn missing_evidence_ref_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::TestSession).evidence_refs = Vec::new();
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MissingEvidenceRef));
}

#[test]
fn confidence_overclaim_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::PipelineOverlay).observed_confidence =
        BuildTestEventConfidence::High;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ConfidenceOverclaim));
}

#[test]
fn missing_fallback_reason_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::PipelineOverlay).fallback_reason = None;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::FallbackReasonMissing));
}

#[test]
fn spurious_fallback_reason_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::TaskCenterRun).fallback_reason =
        Some("unexpected".to_owned());
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::FallbackReasonUnexpected));
}

#[test]
fn degraded_state_not_disclosed_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::PipelineOverlay).degraded_state_disclosed = false;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::DegradedStateNotDisclosed));
}

#[test]
fn raw_payload_not_retained_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::TestSession).raw_private_material_excluded = false;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::RawPayloadNotRetained));
}

#[test]
fn replay_unstable_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::DebugSession).replay_stable = false;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ReplayUnstable));
}

#[test]
fn export_parity_broken_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::NotebookRun).export_parity_preserved = false;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ExportParityBroken));
}

#[test]
fn adapter_hierarchy_missing_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    profile_mut(&mut input, ToolingProfile::TaskCenterRun).capability_packet_ref = String::new();
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::AdapterHierarchyMissing));
}

#[test]
fn missing_profile_blocks_stable() {
    let mut input = current_stable_event_interop_certification_input();
    input
        .profiles
        .retain(|row| row.profile != ToolingProfile::CoverageIntelligence);
    let packet = EventInteropCertificationPacket::materialize(input);
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::MissingProfile));
}

#[test]
fn stale_profile_narrows_below_stable_without_blocking() {
    let mut input = current_stable_event_interop_certification_input();
    let row = profile_mut(&mut input, ToolingProfile::CoverageIntelligence);
    row.proof_age_days = row.freshness_window_days + 10;
    let packet = EventInteropCertificationPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BuildTestInteropPromotionState::NarrowedBelowStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ProfileEvidenceStale));
    // Narrowing is a warning, not a blocker.
    assert!(packet.is_stable());
    assert!(!packet.certification_index.all_profiles_current);
    assert!(packet
        .certification_index
        .narrowed_profiles
        .contains(&ToolingProfile::CoverageIntelligence.as_str().to_owned()));
}

#[test]
fn profile_digest_drift_is_caught() {
    let mut packet = seed();
    packet.profile_digest = "fnv1a64:0000000000000000".to_owned();
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ProfileDigestDrift));
}

#[test]
fn certification_index_drift_is_caught() {
    let mut packet = seed();
    packet.certification_index.all_profiles_certified = false;
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::CertificationIndexDrift));
}

#[test]
fn profile_certification_drift_is_caught() {
    let mut packet = seed();
    if let Some(row) = packet.profiles.first_mut() {
        row.certified = false;
    }
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ProfileCertificationDrift));
}

#[test]
fn claim_state_drift_is_caught() {
    let mut packet = seed();
    if let Some(row) = packet.profiles.first_mut() {
        row.claim_state = ProfileClaimState::Blocked;
    }
    assert!(packet
        .validate()
        .iter()
        .any(|f| f.finding_kind == CertificationFindingKind::ProfileClaimStateDrift));
}
