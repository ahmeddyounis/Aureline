//! Inline tests for the M5 assurance-claim-reducer lane.

use super::*;

fn packet() -> M5AssuranceClaimReducer {
    seeded_m5_assurance_claim_reducer()
}

#[test]
fn canonical_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_ASSURANCE_CLAIM_REDUCER_PACKET_ID);
    assert_eq!(packet.record_kind, M5_ASSURANCE_CLAIM_REDUCER_RECORD_KIND);
    assert_eq!(
        packet.reduced_claims.len(),
        AssuranceClaimSubject::ALL.len()
    );
    assert_eq!(
        packet.precondition_states.len(),
        ClaimPrecondition::ALL.len()
    );
    assert!(packet.conformance.all_hold());
    assert!(packet.vocabulary.matches_canonical());
}

#[test]
fn canonical_packet_proves_every_claim() {
    // Acceptance: with every precondition satisfied, every claim stays proven and no claim narrows.
    let packet = packet();
    for claim in &packet.reduced_claims {
        assert!(
            claim.is_governed(),
            "claim `{}` not governed",
            claim.subject.as_str()
        );
        assert_eq!(claim.reduced_state, AssuranceClaimState::Proven);
        assert!(claim.nearest_truthful.is_none());
        assert!(claim.drifts.is_empty());
        assert_eq!(claim.effective_qualification, QualificationClass::Stable);
    }
    assert_eq!(
        packet.summary.proven_claims,
        AssuranceClaimSubject::ALL.len() as u32
    );
    assert_eq!(packet.summary.total_drifts, 0);
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn every_reduced_claim_carries_a_consumer_projection_per_surface() {
    let packet = packet();
    for claim in &packet.reduced_claims {
        assert_eq!(claim.consumer_projections.len(), ReducerConsumer::ALL.len());
        let consumers: Vec<ReducerConsumer> = claim
            .consumer_projections
            .iter()
            .map(|p| p.consumer)
            .collect();
        assert_eq!(consumers, ReducerConsumer::ALL.to_vec());
    }
}

#[test]
fn stale_evidence_narrows_every_dependent_claim() {
    // Acceptance: stale proof visibly narrows claims instead of leaving prior states in place.
    let packet = seeded_m5_assurance_claim_reducer_stale_evidence_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.precondition_status(ClaimPrecondition::EvidenceFreshness),
        Some(PreconditionStatus::Drifted)
    );
    // Every claim depends on fresh evidence, so every claim narrows to under_review.
    for claim in &packet.reduced_claims {
        assert!(
            claim.is_narrowed(),
            "claim `{}` did not narrow",
            claim.subject.as_str()
        );
        assert_eq!(claim.reduced_state, AssuranceClaimState::UnderReview);
        // The narrowing is attributed to the stale-evidence precondition drift.
        assert!(claim
            .drifts
            .iter()
            .any(|d| d.drift == DriftToken::StaleEvidence));
        // The nearest truthful state never overstates.
        let nearest = claim.nearest_truthful.as_ref().expect("fallback present");
        assert_ne!(nearest.fallback_state, AssuranceClaimState::Proven);
    }
    assert!(!packet.blocks_stable_promotion());
}

#[test]
fn hosted_dependency_drift_narrows_only_dependent_claims() {
    let packet = seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    for claim in &packet.reduced_claims {
        let depends = required_preconditions(claim.subject)
            .contains(&ClaimPrecondition::HostedDependencyBoundary);
        if depends {
            assert!(claim.is_narrowed(), "`{}`", claim.subject.as_str());
            assert!(claim
                .drifts
                .iter()
                .any(|d| d.drift == DriftToken::HostedDependencyDrift));
        } else {
            assert!(claim.is_governed(), "`{}`", claim.subject.as_str());
            assert_eq!(claim.reduced_state, AssuranceClaimState::Proven);
        }
    }
}

#[test]
fn key_residency_mismatch_blocks_dependent_claims() {
    // Acceptance: a key/residency path change blocks the claims that depend on it.
    let packet = seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    for claim in &packet.reduced_claims {
        let depends =
            required_preconditions(claim.subject).contains(&ClaimPrecondition::KeyResidency);
        if depends {
            assert!(claim.is_blocked(), "`{}`", claim.subject.as_str());
            assert_eq!(claim.reduced_state, AssuranceClaimState::Unproven);
            assert!(claim
                .drifts
                .iter()
                .any(|d| d.drift == DriftToken::KeyResidencyMismatch && d.blocking));
        } else {
            assert!(claim.is_governed(), "`{}`", claim.subject.as_str());
        }
    }
}

#[test]
fn policy_path_regression_blocks_dependent_claims() {
    let packet = seeded_m5_assurance_claim_reducer_policy_path_regression_blocked();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.blocks_stable_promotion());
    for claim in &packet.reduced_claims {
        let depends =
            required_preconditions(claim.subject).contains(&ClaimPrecondition::PolicyControlPath);
        if depends {
            assert!(claim.is_blocked(), "`{}`", claim.subject.as_str());
            assert!(claim
                .drifts
                .iter()
                .any(|d| d.drift == DriftToken::PolicyPathRegression));
        } else {
            assert!(claim.is_governed(), "`{}`", claim.subject.as_str());
        }
    }
}

#[test]
fn one_reducer_output_drives_all_consumers_identically() {
    // Guardrail: a claim narrowed in one consumer can never read stronger in another.
    for packet in [
        seeded_m5_assurance_claim_reducer(),
        seeded_m5_assurance_claim_reducer_stale_evidence_narrowed(),
        seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed(),
        seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked(),
        seeded_m5_assurance_claim_reducer_policy_path_regression_blocked(),
    ] {
        for claim in &packet.reduced_claims {
            for projection in &claim.consumer_projections {
                assert_eq!(
                    projection.claim_state,
                    claim.reduced_state,
                    "consumer `{}` diverged on claim `{}`",
                    projection.consumer.as_str(),
                    claim.subject.as_str()
                );
                assert_eq!(
                    projection.effective_qualification,
                    claim.effective_qualification
                );
                assert!(projection.converges_with_reduced);
            }
        }
        assert!(packet.conformance.consumers_converge_on_reduced_state);
        assert!(packet.conformance.no_consumer_strengthens_after_narrowing);
    }
}

#[test]
fn every_drift_names_the_precondition_that_changed() {
    // Acceptance: claim-state changes remain attributable to evidence/boundary changes.
    for packet in [
        seeded_m5_assurance_claim_reducer_stale_evidence_narrowed(),
        seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed(),
        seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked(),
        seeded_m5_assurance_claim_reducer_policy_path_regression_blocked(),
    ] {
        assert!(packet.conformance.drift_attributed_to_precondition);
        for claim in &packet.reduced_claims {
            // A non-governed claim records at least one drift; a governed claim records none.
            assert_eq!(claim.is_governed(), claim.drifts.is_empty());
            for drift in &claim.drifts {
                assert!(claim
                    .preconditions
                    .iter()
                    .any(|r| r.precondition == drift.precondition && r.status == drift.status));
                assert!(!drift.evidence_ref.trim().is_empty());
            }
        }
    }
}

#[test]
fn export_preview_is_refs_only_and_carries_no_raw_material() {
    let packet = seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked();
    let preview = &packet.export_preview;
    assert_eq!(preview.entries.len(), AssuranceClaimSubject::ALL.len());
    assert_eq!(
        preview.record_kind,
        M5_ASSURANCE_NARROWING_EXPORT_RECORD_KIND
    );
    let value = serde_json::to_value(packet.clone()).expect("serialize");
    assert!(!json_contains_forbidden_material(&value));
    // The preview reuses the same reduced states the live claims show.
    for entry in &preview.entries {
        let claim = packet.claim(entry.subject).expect("claim present");
        assert_eq!(entry.reduced_state, claim.reduced_state);
        assert_eq!(entry.effective_qualification, claim.effective_qualification);
    }
}

#[test]
fn renders_are_deterministic_and_channel_independent() {
    let packet = packet();
    assert_eq!(
        packet.render_for_channel(AssuranceClaimReducerChannel::Desktop),
        packet.render_for_channel(AssuranceClaimReducerChannel::Headless)
    );
    assert_eq!(
        packet.render_for_channel(AssuranceClaimReducerChannel::Headless),
        packet.render_for_channel(AssuranceClaimReducerChannel::OfflineMirror)
    );
    // Smoke: every render produces non-empty, stable output.
    assert!(packet
        .render_overview_markdown()
        .contains("Assurance-Claim Reducer"));
    assert!(packet.render_markdown_summary().contains("Proof"));
    assert!(packet
        .render_claims_csv()
        .starts_with("claim,claimed_posture,reduced_state"));
    assert!(
        packet.render_export_preview().contains("export_preview")
            || packet.render_export_preview().contains("entries")
    );
}

#[test]
fn tampering_with_a_reduced_state_is_caught() {
    let mut packet = seeded_m5_assurance_claim_reducer_stale_evidence_narrowed();
    // Forge a stronger state on a narrowed claim — validation must catch the overstatement.
    packet.reduced_claims[0].reduced_state = AssuranceClaimState::Proven;
    packet.reduced_claims[0].reduced_gate = DescriptorGate::Governed;
    let violations = packet.validate();
    assert!(!violations.is_empty());
}

#[test]
fn detects_a_consumer_strengthening_after_narrowing() {
    let mut packet = seeded_m5_assurance_claim_reducer_stale_evidence_narrowed();
    // A consumer reads the claim stronger than the reduced output — the guardrail must fire.
    packet.reduced_claims[0].consumer_projections[0].claim_state = AssuranceClaimState::Proven;
    let violations = packet.validate();
    assert!(violations.contains(&M5AssuranceClaimReducerViolation::ConsumerDivergence));
}

#[test]
fn vocabulary_lists_every_frozen_token() {
    let vocab = AssuranceClaimReducerVocabulary::canonical();
    assert_eq!(vocab.preconditions.len(), ClaimPrecondition::ALL.len());
    assert_eq!(
        vocab.precondition_statuses.len(),
        PreconditionStatus::ALL.len()
    );
    assert_eq!(vocab.drift_tokens.len(), DriftToken::ALL.len());
    assert_eq!(vocab.consumers.len(), ReducerConsumer::ALL.len());
    assert_eq!(
        vocab.restoration_actions.len(),
        RestorationAction::ALL.len()
    );
}
