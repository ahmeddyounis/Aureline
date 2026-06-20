use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_generated_certification_packet();
    validate_m5_generated_certification_packet(&packet)
        .expect("seeded packet must satisfy the frozen contract");
}

#[test]
fn seeded_packet_certifies_every_claimed_profile() {
    let packet = seeded_m5_generated_certification_packet();
    assert_eq!(packet.rows.len(), CertifiedProfile::ALL.len());
    for certification_row in &packet.rows {
        // Every seeded row is fully current, so it certifies at its claim.
        assert_eq!(
            certification_row.verdict,
            RowVerdict::Certified,
            "{} should certify on current evidence",
            certification_row.row_id
        );
        assert_eq!(
            certification_row.certified_maturity,
            certification_row.published_claim_maturity
        );
        assert_eq!(
            certification_row.promotion_decision,
            PromotionDecision::Promote
        );
        assert!(!certification_row.narrowed);
        assert!(certification_row.narrow_reason_tokens.is_empty());
        assert!(certification_row.stale_or_missing_domain_tokens.is_empty());
        assert_eq!(
            certification_row.domains.len(),
            CertificationDomain::ALL.len()
        );
    }
}

#[test]
fn partial_evidence_narrows_to_beta_and_promotes_narrowed() {
    let domains = degraded_domains(
        CertificationDomain::CanonicalSourceVisibility,
        EvidenceState::Partial,
    );
    let outcome = certify_profile_outcome(ClaimMaturity::Stable, &domains);
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.certified_maturity, ClaimMaturity::Beta);
    assert_eq!(
        outcome.promotion_decision,
        PromotionDecision::PromoteNarrowed
    );
    assert!(outcome.narrowed);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec!["canonical_source_visibility_partial".to_owned()]
    );
    assert!(outcome.stale_or_missing_domain_tokens.is_empty());
}

#[test]
fn stale_evidence_narrows_to_preview() {
    let domains = degraded_domains(
        CertificationDomain::WritableBoundaryTruth,
        EvidenceState::Stale,
    );
    let outcome = certify_profile_outcome(ClaimMaturity::Beta, &domains);
    assert_eq!(outcome.verdict, RowVerdict::Narrowed);
    assert_eq!(outcome.certified_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.promotion_decision,
        PromotionDecision::PromoteNarrowed
    );
    assert_eq!(
        outcome.stale_or_missing_domain_tokens,
        vec!["writable_boundary_truth".to_owned()]
    );
}

#[test]
fn missing_evidence_withholds_and_holds_promotion() {
    let domains = degraded_domains(
        CertificationDomain::RegenerationPath,
        EvidenceState::Missing,
    );
    let outcome = certify_profile_outcome(ClaimMaturity::Beta, &domains);
    assert_eq!(outcome.verdict, RowVerdict::Withheld);
    assert_eq!(outcome.certified_maturity, ClaimMaturity::Withdrawn);
    assert_eq!(outcome.promotion_decision, PromotionDecision::Hold);
    assert!(outcome.promotion_decision.holds_promotion());
    assert!(!outcome.narrowed, "a withheld claim is not merely narrowed");
}

#[test]
fn certification_never_widens_the_claim() {
    // A beta claim with all-current evidence stays beta, never stable.
    let outcome = certify_profile_outcome(ClaimMaturity::Beta, &current_domains());
    assert_eq!(outcome.certified_maturity, ClaimMaturity::Beta);
    assert_eq!(outcome.verdict, RowVerdict::Certified);
    assert_eq!(outcome.promotion_decision, PromotionDecision::Promote);
}

#[test]
fn narrowing_takes_the_worst_floor_across_domains() {
    let mut domains = current_domains();
    for evidence in &mut domains {
        match evidence.domain {
            CertificationDomain::WritableBoundaryTruth => {
                evidence.evidence_state = EvidenceState::Partial
            }
            CertificationDomain::RestoreExportHonesty => {
                evidence.evidence_state = EvidenceState::Stale
            }
            _ => {}
        }
    }
    let outcome = certify_profile_outcome(ClaimMaturity::Stable, &domains);
    // Stale (preview) is worse than partial (beta), so preview wins.
    assert_eq!(outcome.certified_maturity, ClaimMaturity::Preview);
    assert_eq!(
        outcome.narrow_reason_tokens,
        vec![
            "restore_export_honesty_stale".to_owned(),
            "writable_boundary_truth_partial".to_owned(),
        ]
    );
}

#[test]
fn every_profile_binds_a_real_claim_publication_object() {
    for profile in CertifiedProfile::ALL {
        assert_eq!(profile.backing_artifact_class().as_str(), profile.as_str());
        assert!(!profile.claim_publication_ref().is_empty());
    }
}

#[test]
fn copy_line_reflects_promotion_decision() {
    let packet = seeded_m5_generated_certification_packet();
    for certification_row in &packet.rows {
        let line = certification_copy_line(certification_row);
        assert!(line.contains(&certification_row.profile_label));
        assert!(line.contains("promotes at"));
    }
}

#[test]
fn seeded_fixtures_validate_and_cover_every_verdict() {
    let fixtures = seeded_m5_generated_certification_fixtures();
    assert!(!fixtures.is_empty());
    let mut verdicts = BTreeSet::new();
    let mut decisions = BTreeSet::new();
    for fixture in &fixtures {
        validate_m5_generated_certification_fixture(fixture)
            .unwrap_or_else(|err| panic!("fixture {} must validate: {err}", fixture.fixture_id));
        verdicts.insert(fixture.expected_verdict);
        decisions.insert(fixture.expected_promotion_decision);
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
    for required in [
        PromotionDecision::Promote,
        PromotionDecision::PromoteNarrowed,
        PromotionDecision::Hold,
    ] {
        assert!(
            decisions.contains(&required),
            "fixtures must cover {required:?}"
        );
    }
}

#[test]
fn drills_degrade_then_recover() {
    let packet = seeded_m5_generated_certification_packet();
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
fn every_freshness_rule_floor_matches_the_engine() {
    let packet = seeded_m5_generated_certification_packet();
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
fn certification_stays_aligned_with_governance_classes() {
    // The certification cannot claim a profile the governance lane does not
    // certify, and cannot publish wider than the governance row's claim.
    let governance = crate::m5_generated_governance::seeded_m5_generated_governance_packet();
    let certification = seeded_m5_generated_certification_packet();
    for certification_row in &certification.rows {
        let governance_row = governance
            .rows
            .iter()
            .find(|row| row.artifact_class == certification_row.backing_artifact_class)
            .unwrap_or_else(|| {
                panic!(
                    "governance must certify the class backing {}",
                    certification_row.row_id
                )
            });
        assert!(
            certification_row.published_claim_maturity.severity()
                >= governance_row.claimed_maturity.severity(),
            "certification for {} must not outrun the governance claim",
            certification_row.row_id
        );
    }
}

#[test]
fn packet_round_trips_through_json() {
    let packet = seeded_m5_generated_certification_packet();
    let json = serde_json::to_string(&packet).expect("packet serializes");
    let back: M5GeneratedCertificationPacket =
        serde_json::from_str(&json).expect("packet deserializes");
    assert_eq!(packet, back);
}
