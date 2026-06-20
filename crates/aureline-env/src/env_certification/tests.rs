use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_env_certification_packet();
    validate_env_certification_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_certifies_every_claimed_target_class() {
    let packet = seeded_env_certification_packet();
    assert_eq!(packet.rows.len(), TargetClass::ALL.len());
    for certification_row in &packet.rows {
        // Every seeded row is fully current, so it certifies at its claim.
        assert_eq!(
            certification_row.verdict,
            RowVerdict::Certified,
            "{} should certify on current evidence",
            certification_row.row_id
        );
        assert_eq!(
            certification_row.effective_maturity,
            certification_row.claimed_maturity
        );
        assert_eq!(
            certification_row.effective_warm_start_posture,
            certification_row.claimed_warm_start_posture
        );
        assert!(!certification_row.narrowed);
        assert!(!certification_row.warm_start_downgraded);
        assert!(certification_row.narrow_reason_tokens.is_empty());
        assert!(certification_row.warm_start_downgrade_tokens.is_empty());
        assert!(certification_row.stale_or_missing_aspect_tokens.is_empty());
        assert_eq!(
            certification_row.aspects.len(),
            CertificationAspect::ALL.len()
        );
    }
}

#[test]
fn seeded_packet_does_not_block_promotion_on_current_evidence() {
    let packet = seeded_env_certification_packet();
    assert!(!packet.promotion.promotion_blocked);
    assert!(packet.promotion.held_target_class_tokens.is_empty());
    assert!(packet.promotion.narrowed_target_class_tokens.is_empty());
    assert_eq!(
        packet.promotion.certified_target_class_tokens.len(),
        TargetClass::ALL.len()
    );
}

#[test]
fn partial_evidence_narrows_to_beta() {
    let aspects = degraded_aspects(
        CertificationAspect::TemplateComposition,
        EvidenceState::Partial,
    );
    let outcome = certify_environment_lane(
        ClaimMaturity::Stable,
        WarmStartPosture::WarmPartialReuse,
        &aspects,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert!(outcome.narrowed);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec!["template_composition_partial".to_owned()]
    );
    // A non-warm-start aspect never narrows the warm-start posture.
    assert!(!outcome.warm_start_downgraded);
    assert!(outcome.warm_start_downgrade_tokens.is_empty());
}

#[test]
fn stale_runtime_parity_narrows_to_preview() {
    let aspects = degraded_aspects(
        CertificationAspect::RuntimeInstanceParity,
        EvidenceState::Stale,
    );
    let outcome = certify_environment_lane(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmPartialReuse,
        &aspects,
    );
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.stale_or_missing_aspect_tokens,
        vec!["runtime_instance_parity".to_owned()]
    );
}

#[test]
fn missing_lifecycle_hook_truth_withholds_the_claim() {
    let aspects = degraded_aspects(
        CertificationAspect::LifecycleHookTruth,
        EvidenceState::Missing,
    );
    let outcome = certify_environment_lane(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmPartialReuse,
        &aspects,
    );
    assert_eq!(outcome.verdict, RowVerdict::Withheld);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Withdrawn);
    assert!(!outcome.narrowed, "a withheld claim is not merely narrowed");
}

#[test]
fn stale_prebuild_compatibility_forces_cold_build() {
    // The marquee guardrail: a stale prebuild fingerprint narrows the
    // maturity AND drops a warm-full-reuse claim to a cold build.
    let aspects = degraded_aspects(
        CertificationAspect::PrebuildCompatibility,
        EvidenceState::Stale,
    );
    let outcome = certify_environment_lane(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &aspects,
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(outcome.warm_start_downgraded);
    assert_eq!(
        outcome.warm_start_downgrade_tokens,
        vec!["prebuild_compatibility_stale".to_owned()]
    );
}

#[test]
fn stale_capsule_identity_forces_cold_build() {
    // Capsule identity also governs warm start: a stale capsule cannot
    // prove the cached environment matches the current source.
    let aspects = degraded_aspects(CertificationAspect::CapsuleIdentity, EvidenceState::Stale);
    let outcome = certify_environment_lane(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &aspects,
    );
    assert_eq!(
        outcome.effective_warm_start_posture,
        WarmStartPosture::ColdBuild
    );
    assert!(outcome.warm_start_downgraded);
}

#[test]
fn certification_never_widens_the_claim() {
    // A beta claim with all-current evidence stays beta, never stable.
    let outcome = certify_environment_lane(
        ClaimMaturity::Beta,
        WarmStartPosture::WarmFullReuse,
        &current_aspects(),
    );
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert_eq!(outcome.verdict, RowVerdict::Certified);
}

#[test]
fn narrowing_takes_the_worst_floor_across_aspects() {
    let mut aspects = current_aspects();
    for evidence in &mut aspects {
        match evidence.aspect {
            CertificationAspect::TemplateComposition => {
                evidence.evidence_state = EvidenceState::Partial
            }
            CertificationAspect::RuntimeInstanceParity => {
                evidence.evidence_state = EvidenceState::Stale
            }
            _ => {}
        }
    }
    let outcome = certify_environment_lane(
        ClaimMaturity::Stable,
        WarmStartPosture::WarmPartialReuse,
        &aspects,
    );
    // Stale (preview) is worse than partial (beta), so preview wins.
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec![
            "runtime_instance_parity_stale".to_owned(),
            "template_composition_partial".to_owned(),
        ]
    );
}

#[test]
fn seeded_fixtures_validate_and_cover_every_verdict() {
    let fixtures = seeded_env_certification_fixtures();
    assert!(!fixtures.is_empty());
    let mut verdicts = BTreeSet::new();
    let mut saw_warm_start_downgrade = false;
    let mut saw_promotion_block = false;
    for fixture in &fixtures {
        validate_env_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        verdicts.insert(fixture.expected_verdict);
        if !fixture.expected_warm_start_downgrade_tokens.is_empty() {
            saw_warm_start_downgrade = true;
        }
        if fixture.blocks_promotion {
            saw_promotion_block = true;
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
    assert!(
        saw_promotion_block,
        "fixtures must cover a promotion-blocking verdict"
    );
}

#[test]
fn drills_degrade_then_recover() {
    let packet = seeded_env_certification_packet();
    assert_eq!(packet.drills.len(), TargetClass::ALL.len());
    for certification_drill in &packet.drills {
        assert_ne!(
            certification_drill.expected_degraded_verdict,
            RowVerdict::Certified,
            "drill {} must actually degrade",
            certification_drill.drill_id
        );
        assert_eq!(
            certification_drill.recovers_to_verdict,
            RowVerdict::Certified
        );
        assert_eq!(
            certification_drill.steps.first().map(|s| s.phase),
            Some(DrillPhase::Inject)
        );
        assert_eq!(
            certification_drill.steps.last().map(|s| s.phase),
            Some(DrillPhase::Verify)
        );
    }
}

#[test]
fn drills_cover_narrowed_withheld_and_warm_start_downgrade() {
    let packet = seeded_env_certification_packet();
    let mut verdicts = BTreeSet::new();
    let mut saw_warm_start_downgrade = false;
    let mut saw_promotion_block = false;
    for certification_drill in &packet.drills {
        verdicts.insert(certification_drill.expected_degraded_verdict);
        if certification_drill.expected_degraded_warm_start_posture
            != certification_drill.claimed_warm_start_posture
        {
            saw_warm_start_downgrade = true;
        }
        if certification_drill.blocks_promotion_while_degraded {
            saw_promotion_block = true;
        }
    }
    assert!(verdicts.contains(&RowVerdict::Narrowed));
    assert!(verdicts.contains(&RowVerdict::Withheld));
    assert!(saw_warm_start_downgrade);
    assert!(saw_promotion_block);
}

#[test]
fn every_freshness_rule_floor_matches_the_engine() {
    let packet = seeded_env_certification_packet();
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
fn every_aspect_binds_a_real_lane_ref() {
    // Each aspect must cite at least one upstream lane artifact so the
    // certification is a composition of the frozen lanes.
    for aspect in CertificationAspect::ALL {
        assert!(
            !aspect_evidence_refs(aspect).is_empty(),
            "aspect {} must bind an upstream lane ref",
            aspect.as_str()
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_env_certification_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: EnvCertificationPacket = serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
