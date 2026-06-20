use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_reactive_certification_packet();
    validate_m5_reactive_certification_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_certifies_every_claimed_profile() {
    let packet = seeded_m5_reactive_certification_packet();
    assert_eq!(packet.rows.len(), ClaimedSurfaceProfile::ALL.len());
    for cert_row in &packet.rows {
        // Every seeded row is fully current, so it certifies at its claim.
        assert_eq!(
            cert_row.verdict,
            RowVerdict::Certified,
            "{} should certify on current evidence",
            cert_row.row_id
        );
        assert_eq!(cert_row.effective_maturity, cert_row.claimed_maturity);
        assert!(!cert_row.narrowed);
        assert!(cert_row.narrow_reason_tokens.is_empty());
        assert!(cert_row.stale_or_missing_dimension_tokens.is_empty());
        assert_eq!(cert_row.dimensions.len(), CertificationDimension::ALL.len());
    }
}

#[test]
fn partial_evidence_narrows_to_beta() {
    let dimensions = degraded_dimensions(
        CertificationDimension::InvalidationBehavior,
        EvidenceState::Partial,
    );
    let outcome = certify_row_outcome(ClaimMaturity::Stable, &dimensions);
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert!(outcome.narrowed);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec!["invalidation_behavior_partial".to_owned()]
    );
}

#[test]
fn stale_evidence_narrows_to_preview() {
    let dimensions = degraded_dimensions(CertificationDimension::EpochParity, EvidenceState::Stale);
    let outcome = certify_row_outcome(ClaimMaturity::Stable, &dimensions);
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.stale_or_missing_dimension_tokens,
        vec!["epoch_parity".to_owned()]
    );
}

#[test]
fn missing_evidence_withholds_the_claim() {
    let dimensions = degraded_dimensions(
        CertificationDimension::AuthorityClass,
        EvidenceState::Missing,
    );
    let outcome = certify_row_outcome(ClaimMaturity::Beta, &dimensions);
    assert_eq!(outcome.verdict, RowVerdict::Withheld);
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Withdrawn);
    assert!(!outcome.narrowed, "a withheld claim is not merely narrowed");
}

#[test]
fn narrowing_takes_the_worst_floor_across_dimensions() {
    let mut dimensions = current_dimensions();
    for evidence in &mut dimensions {
        match evidence.dimension {
            CertificationDimension::EpochParity => evidence.evidence_state = EvidenceState::Partial,
            CertificationDimension::StaleStateLabeling => {
                evidence.evidence_state = EvidenceState::Stale
            }
            _ => {}
        }
    }
    let outcome = certify_row_outcome(ClaimMaturity::Stable, &dimensions);
    // Stale (preview) is worse than partial (beta), so preview wins.
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec![
            "epoch_parity_partial".to_owned(),
            "stale_state_labeling_stale".to_owned(),
        ]
    );
}

#[test]
fn certification_never_widens_the_claim() {
    // A beta claim with all-current evidence stays beta, never stable.
    let outcome = certify_row_outcome(ClaimMaturity::Beta, &current_dimensions());
    assert_eq!(outcome.effective_maturity, ClaimMaturity::Beta);
    assert_eq!(outcome.verdict, RowVerdict::Certified);
}

#[test]
fn seeded_fixtures_validate_and_cover_every_verdict() {
    let fixtures = seeded_m5_reactive_certification_fixtures();
    assert!(!fixtures.is_empty());
    let mut verdicts = BTreeSet::new();
    for fixture in &fixtures {
        validate_m5_reactive_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        verdicts.insert(fixture.expected_verdict);
    }
    for required in [
        RowVerdict::Certified,
        RowVerdict::Narrowed,
        RowVerdict::Withheld,
    ] {
        assert!(
            verdicts.contains(&required),
            "fixtures must cover {:?}",
            required
        );
    }
}

#[test]
fn drills_degrade_then_recover() {
    let packet = seeded_m5_reactive_certification_packet();
    for cert_drill in &packet.drills {
        assert_ne!(
            cert_drill.expected_degraded_verdict,
            RowVerdict::Certified,
            "drill {} must actually degrade",
            cert_drill.drill_id
        );
        assert_eq!(cert_drill.recovers_to_verdict, RowVerdict::Certified);
        assert_eq!(
            cert_drill.steps.first().map(|s| s.phase),
            Some(DrillPhase::Inject)
        );
        assert_eq!(
            cert_drill.steps.last().map(|s| s.phase),
            Some(DrillPhase::Verify)
        );
    }
}

#[test]
fn every_freshness_rule_floor_matches_the_engine() {
    let packet = seeded_m5_reactive_certification_packet();
    for rule in &packet.freshness_rules {
        assert_eq!(
            Some(rule.maturity_floor),
            rule.trigger_evidence_state.qualification_floor(),
            "rule {} floor must match the engine",
            rule.rule_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_reactive_certification_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: M5ReactiveCertificationPacket =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
