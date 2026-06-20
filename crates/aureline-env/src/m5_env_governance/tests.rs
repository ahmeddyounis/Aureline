use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_env_governance_packet();
    validate_m5_env_governance_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_certifies_every_claimed_profile() {
    let packet = seeded_m5_env_governance_packet();
    assert_eq!(packet.rows.len(), EnvironmentProfile::ALL.len());
    for capsule_row in &packet.rows {
        // Every seeded row is fully current, so it certifies at its claim.
        assert_eq!(
            capsule_row.verdict,
            RowVerdict::Certified,
            "{} should certify on current evidence",
            capsule_row.row_id
        );
        assert_eq!(capsule_row.effective_maturity, capsule_row.claimed_maturity);
        assert_eq!(
            capsule_row.effective_warm_start_posture,
            capsule_row.claimed_warm_start_posture
        );
        assert!(!capsule_row.narrowed);
        assert!(!capsule_row.warm_start_downgraded);
        assert!(capsule_row.narrow_reason_tokens.is_empty());
        assert!(capsule_row.warm_start_downgrade_tokens.is_empty());
        assert!(capsule_row.stale_or_missing_dimension_tokens.is_empty());
        assert_eq!(capsule_row.dimensions.len(), CapsuleDimension::ALL.len());
    }
}

#[test]
fn partial_evidence_narrows_to_beta() {
    let dimensions = degraded_dimensions(CapsuleDimension::ServiceGraph, EvidenceState::Partial);
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Stable,
        WarmStartPosture::ColdBuild,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert!(outcome.narrowed);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec!["service_graph_partial".to_owned()]
    );
    // A non-warm-start dimension never narrows the warm-start posture.
    assert!(!outcome.warm_start_downgraded);
    assert!(outcome.warm_start_downgrade_tokens.is_empty());
}

#[test]
fn stale_evidence_narrows_to_preview() {
    let dimensions = degraded_dimensions(CapsuleDimension::ToolchainPlan, EvidenceState::Stale);
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Stable,
        WarmStartPosture::WarmPartialReuse,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.stale_or_missing_dimension_tokens,
        vec!["toolchain_plan".to_owned()]
    );
}

#[test]
fn missing_evidence_withholds_the_claim() {
    let dimensions = degraded_dimensions(CapsuleDimension::TrustHooks, EvidenceState::Missing);
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Beta,
        WarmStartPosture::ColdBuild,
        &dimensions,
    );
    assert_eq!(outcome.verdict, RowVerdict::Withheld);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Withdrawn);
    assert!(!outcome.narrowed, "a withheld claim is not merely narrowed");
}

#[test]
fn stale_prebuild_fingerprint_forces_cold_build() {
    // The marquee guardrail: a stale fingerprint narrows the maturity AND
    // drops a warm-full-reuse claim to a cold build.
    let dimensions =
        degraded_dimensions(CapsuleDimension::PrebuildFingerprint, EvidenceState::Stale);
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &dimensions,
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(outcome.warm_start_downgraded);
    assert_eq!(
        outcome.warm_start_downgrade_tokens,
        vec!["prebuild_fingerprint_stale".to_owned()]
    );
}

#[test]
fn partial_source_digest_caps_warm_reuse_at_partial() {
    // A partial source digest cannot prove the whole cached environment.
    let dimensions = degraded_dimensions(CapsuleDimension::SourceDigest, EvidenceState::Partial);
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &dimensions,
    );
    assert_eq!(
        outcome.effective_warm_start_posture,
        WarmStartPosture::WarmPartialReuse
    );
    assert!(outcome.warm_start_downgraded);
}

#[test]
fn warm_start_never_widens_above_the_claim() {
    // A cold-build claim with all-current evidence stays cold; warm-start
    // narrowing never promotes a posture.
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Stable,
        WarmStartPosture::ColdBuild,
        &current_dimensions(),
    );
    assert_eq!(
        outcome.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(!outcome.warm_start_downgraded);
}

#[test]
fn narrowing_takes_the_worst_floor_across_dimensions() {
    let mut dimensions = current_dimensions();
    for evidence in &mut dimensions {
        match evidence.dimension {
            CapsuleDimension::TargetPlan => evidence.evidence_state = EvidenceState::Partial,
            CapsuleDimension::MaterializationParity => {
                evidence.evidence_state = EvidenceState::Stale
            }
            _ => {}
        }
    }
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Stable,
        WarmStartPosture::ColdBuild,
        &dimensions,
    );
    // Stale (preview) is worse than partial (beta), so preview wins.
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec![
            "materialization_parity_stale".to_owned(),
            "target_plan_partial".to_owned(),
        ]
    );
}

#[test]
fn certification_never_widens_the_claim() {
    // A beta claim with all-current evidence stays beta, never stable.
    let outcome = certify_capsule_outcome(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &current_dimensions(),
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert_eq!(outcome.verdict, RowVerdict::Certified);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_verdict() {
    let fixtures = seeded_m5_env_governance_fixtures();
    assert!(!fixtures.is_empty());
    let mut verdicts = BTreeSet::new();
    let mut saw_warm_start_downgrade = false;
    for fixture in &fixtures {
        validate_m5_env_governance_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        verdicts.insert(fixture.expected_verdict);
        if !fixture.expected_warm_start_downgrade_tokens.is_empty() {
            saw_warm_start_downgrade = true;
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
        saw_warm_start_downgrade,
        "fixtures must cover a warm-start downgrade"
    );
}

#[test]
fn drills_degrade_then_recover() {
    let packet = seeded_m5_env_governance_packet();
    for capsule_drill in &packet.drills {
        assert_ne!(
            capsule_drill.expected_degraded_verdict,
            RowVerdict::Certified,
            "drill {} must actually degrade",
            capsule_drill.drill_id
        );
        assert_eq!(capsule_drill.recovers_to_verdict, RowVerdict::Certified);
        assert_eq!(
            capsule_drill.steps.first().map(|s| s.phase),
            Some(DrillPhase::Inject)
        );
        assert_eq!(
            capsule_drill.steps.last().map(|s| s.phase),
            Some(DrillPhase::Verify)
        );
    }
}

#[test]
fn every_freshness_rule_floor_matches_the_engine() {
    let packet = seeded_m5_env_governance_packet();
    for rule in &packet.freshness_rules {
        assert_eq!(
            Some(rule.maturity_floor),
            rule.trigger_evidence_state.qualification_floor(),
            "rule {} floor must match the engine",
            rule.rule_id
        );
    }
    for rule in &packet.warm_start_rules {
        assert_eq!(
            Some(rule.warm_start_floor),
            rule.trigger_evidence_state.warm_start_floor(),
            "warm-start rule {} floor must match the engine",
            rule.rule_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_env_governance_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: M5EnvGovernancePacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
