use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_generated_governance_packet();
    validate_m5_generated_governance_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_certifies_every_claimed_class() {
    let packet = seeded_m5_generated_governance_packet();
    assert_eq!(packet.rows.len(), ArtifactClass::ALL.len());
    for artifact_row in &packet.rows {
        // Every seeded row is fully current, so it certifies at its claim.
        assert_eq!(
            artifact_row.verdict,
            RowVerdict::Certified,
            "{} should certify on current evidence",
            artifact_row.row_id
        );
        assert_eq!(
            artifact_row.effective_maturity,
            artifact_row.claimed_maturity
        );
        assert_eq!(
            artifact_row.effective_edit_posture,
            artifact_row.claimed_edit_posture
        );
        assert!(!artifact_row.narrowed);
        assert!(!artifact_row.edit_posture_downgraded);
        assert!(artifact_row.narrow_reason_tokens.is_empty());
        assert!(artifact_row.edit_posture_downgrade_tokens.is_empty());
        assert!(artifact_row.stale_or_missing_dimension_tokens.is_empty());
        assert_eq!(
            artifact_row.dimensions.len(),
            ProvenanceDimension::ALL.len()
        );
    }
}

#[test]
fn partial_evidence_narrows_to_beta() {
    let dimensions = degraded_dimensions(ProvenanceDimension::DriftState, EvidenceState::Partial);
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::RegenerateOnly,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert!(outcome.narrowed);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec!["drift_state_partial".to_owned()]
    );
    // A non-edit-posture dimension never narrows the writable boundary.
    assert!(!outcome.edit_posture_downgraded);
    assert!(outcome.edit_posture_downgrade_tokens.is_empty());
}

#[test]
fn stale_evidence_narrows_to_preview() {
    let dimensions =
        degraded_dimensions(ProvenanceDimension::GeneratorIdentity, EvidenceState::Stale);
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::ReviewedOverrideRequired,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.stale_or_missing_dimension_tokens,
        vec!["generator_identity".to_owned()]
    );
}

#[test]
fn missing_evidence_withholds_the_claim() {
    let dimensions = degraded_dimensions(
        ProvenanceDimension::RegenerationRoute,
        EvidenceState::Missing,
    );
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Beta,
        EditPosture::RegenerateOnly,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Withheld);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Withdrawn);
    assert!(!outcome.narrowed, "a withheld claim is not merely narrowed");
}

#[test]
fn stale_writable_boundary_forces_regenerate_only() {
    // The marquee guardrail: a stale writable boundary narrows the maturity
    // AND drops a direct-edit claim to a regenerate-only boundary.
    let dimensions =
        degraded_dimensions(ProvenanceDimension::WritableBoundary, EvidenceState::Stale);
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::DirectEditAllowed,
        &dimensions,
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(outcome.effective_edit_posture, EditPosture::RegenerateOnly);
    assert!(outcome.edit_posture_downgraded);
    assert_eq!(
        outcome.edit_posture_downgrade_tokens,
        vec!["writable_boundary_stale".to_owned()]
    );
}

#[test]
fn partial_canonical_source_caps_edit_posture_at_reviewed_override() {
    // A partial canonical source cannot prove a direct edit survives regen.
    let dimensions =
        degraded_dimensions(ProvenanceDimension::CanonicalSource, EvidenceState::Partial);
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::DirectEditAllowed,
        &dimensions,
    );
    assert_eq!(
        outcome.effective_edit_posture,
        EditPosture::ReviewedOverrideRequired
    );
    assert!(outcome.edit_posture_downgraded);
}

#[test]
fn edit_posture_never_widens_above_the_claim() {
    // A regenerate-only claim with all-current evidence stays regenerate-only;
    // edit-posture narrowing never promotes a boundary.
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::RegenerateOnly,
        &current_dimensions(),
    );
    assert_eq!(outcome.effective_edit_posture, EditPosture::RegenerateOnly);
    assert!(!outcome.edit_posture_downgraded);
}

#[test]
fn narrowing_takes_the_worst_floor_across_dimensions() {
    let mut dimensions = current_dimensions();
    for evidence in &mut dimensions {
        match evidence.dimension {
            ProvenanceDimension::ProvenanceClass => {
                evidence.evidence_state = EvidenceState::Partial
            }
            ProvenanceDimension::CheckpointLineage => {
                evidence.evidence_state = EvidenceState::Stale
            }
            _ => {}
        }
    }
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Stable,
        EditPosture::RegenerateOnly,
        &dimensions,
    );
    // Stale (preview) is worse than partial (beta), so preview wins.
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec![
            "checkpoint_lineage_stale".to_owned(),
            "provenance_class_partial".to_owned(),
        ]
    );
}

#[test]
fn certification_never_widens_the_claim() {
    // A beta claim with all-current evidence stays beta, never stable.
    let outcome = certify_artifact_outcome(
        ClaimMaturity::Beta,
        EditPosture::ReviewedOverrideRequired,
        &current_dimensions(),
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert_eq!(outcome.verdict, RowVerdict::Certified);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_verdict() {
    let fixtures = seeded_m5_generated_governance_fixtures();
    assert!(!fixtures.is_empty());
    let mut verdicts = BTreeSet::new();
    let mut saw_edit_posture_downgrade = false;
    for fixture in &fixtures {
        validate_m5_generated_governance_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        verdicts.insert(fixture.expected_verdict);
        if !fixture.expected_edit_posture_downgrade_tokens.is_empty() {
            saw_edit_posture_downgrade = true;
        }
    }
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
    assert!(
        saw_edit_posture_downgrade,
        "fixtures must cover an edit-posture downgrade"
    );
}

#[test]
fn drills_degrade_then_recover() {
    let packet = seeded_m5_generated_governance_packet();
    for artifact_drill in &packet.drills {
        assert_ne!(
            artifact_drill.expected_degraded_verdict,
            RowVerdict::Certified,
            "drill {} must actually degrade",
            artifact_drill.drill_id
        );
        assert_eq!(artifact_drill.recovers_to_verdict, RowVerdict::Certified);
        assert_eq!(
            artifact_drill.steps.first().map(|s| s.phase),
            Some(DrillPhase::Inject)
        );
        assert_eq!(
            artifact_drill.steps.last().map(|s| s.phase),
            Some(DrillPhase::Verify)
        );
    }
}

#[test]
fn every_freshness_rule_floor_matches_the_engine() {
    let packet = seeded_m5_generated_governance_packet();
    for rule in &packet.freshness_rules {
        assert_eq!(
            Some(rule.maturity_floor),
            rule.trigger_evidence_state.qualification_floor(),
            "rule {} floor must match the engine",
            rule.rule_id
        );
    }
    for rule in &packet.edit_boundary_rules {
        assert_eq!(
            Some(rule.edit_posture_floor),
            rule.trigger_evidence_state.edit_posture_floor(),
            "edit-boundary rule {} floor must match the engine",
            rule.rule_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_generated_governance_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: M5GeneratedGovernancePacket =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
