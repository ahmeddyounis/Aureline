use super::*;

fn register() -> EvalPackRegister {
    current_m5_evaluation_pilot_packs().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_EVALUATION_PILOT_PACKS_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.packs.is_empty());
}

#[test]
fn covers_every_lane_kind() {
    let r = register();
    for lane in EvalPackLaneKind::ALL {
        assert!(
            !r.packs_for_lane(lane).is_empty(),
            "lane kind {} must have at least one pack",
            lane.as_str()
        );
    }
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.packs_for_kind(kind).is_empty(),
            "family kind {} must have at least one pack",
            kind.as_str()
        );
    }
}

#[test]
fn every_pack_drives_the_required_destinations() {
    let r = register();
    for p in &r.packs {
        let driven: BTreeSet<EvalPackDestination> =
            p.destinations.iter().map(|d| d.destination).collect();
        for required in EvalPackDestination::REQUIRED {
            assert!(
                driven.contains(&required),
                "pack {} must drive required destination {}",
                p.entry_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn every_destination_reuses_one_pack() {
    let r = register();
    for p in &r.packs {
        for d in &p.destinations {
            assert_eq!(
                d.source_pack_id,
                r.register_id,
                "destination {} on {} must render from the one pack",
                d.destination.as_str(),
                p.entry_id
            );
            assert_eq!(d.rendered_label, p.pack_published_label);
            assert_eq!(d.rendered_support_class, p.pack_support_class);
            assert_eq!(d.rendered_claim_text, p.pack_claim_text);
            assert!(d.discloses_freshness);
            if !p.known_issues_delta.is_empty() {
                assert!(d.discloses_known_issues);
            }
            if !p.deployment_caveats.is_empty() {
                assert!(d.discloses_caveats);
            }
        }
    }
}

#[test]
fn no_pack_is_greener_than_its_public_claim() {
    let r = register();
    for p in &r.packs {
        assert!(
            p.pack_published_label.rank() <= p.public_claim_label.rank(),
            "pack {} may not publish greener than its public claim",
            p.entry_id
        );
        assert!(
            !p.over_claims_public(),
            "pack {} may not over-claim the public label or support class",
            p.entry_id
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let r = register();
    assert!(!r.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_packs()
        .iter()
        .map(|p| p.family_ref.as_str())
        .collect();
    for declared in &r.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking pack"
        );
    }
}

#[test]
fn summary_counts_match_packs() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.packs_published + r.summary.packs_narrowed,
        r.packs.len()
    );
}

#[test]
fn promotion_decision_matches_computed() {
    let r = register();
    assert_eq!(r.promotion.decision, r.computed_promotion_decision());
    assert_eq!(
        r.promotion.blocking_rule_ids,
        r.computed_blocking_rule_ids()
    );
    assert_eq!(
        r.promotion.blocking_claim_ids,
        r.computed_blocking_claim_ids()
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<EvalPackNarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in EvalPackNarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn an_inherited_public_claim_narrowing_does_not_block_promotion() {
    let r = register();
    // The companion partner pack inherits a Beta public claim but introduces no
    // pack-layer failure, so it is not a promotion blocker.
    let companion = r
        .pack("eval-pack-companion-ecosystem-partner")
        .expect("companion pack");
    assert!(!companion.publishes_stable());
    assert!(companion.has_active_reason(EvalPackNarrowingReason::PublicClaimNarrowed));
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
}

#[test]
fn a_pack_layer_failure_on_a_stable_public_claim_blocks_promotion() {
    let r = register();
    // The notebook pilot pack rides a still-Stable public claim but its offline
    // bundle mirror went stale -> a blocker.
    let pilot = r
        .pack("eval-pack-notebook-enterprise-pilot")
        .expect("notebook pilot pack");
    assert!(pilot.public_claim_holds_stable());
    assert!(pilot.has_active_reason(EvalPackNarrowingReason::MirrorStale));
    assert!(r.computed_blocking_claim_ids().contains(&pilot.entry_id));
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
}

#[test]
fn validate_flags_a_published_pack_with_active_gap() {
    let mut r = register();
    let p = r
        .packs
        .iter_mut()
        .find(|p| p.publishes_stable())
        .expect("a published pack exists");
    p.active_narrowing_reasons
        .push(EvalPackNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::PublishedWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_pack_over_claiming_the_public_label() {
    let mut r = register();
    let p = r
        .packs
        .iter_mut()
        .find(|p| !p.public_claim_label.is_at_or_above_cutline())
        .expect("a pack reusing a below-cutline public claim exists");
    p.pack_published_label = StableClaimLevel::Stable;
    for d in &mut p.destinations {
        d.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::PackLabelExceedsPublicClaim { .. })));
}

#[test]
fn validate_flags_a_pack_over_claiming_the_support_class() {
    let mut r = register();
    // The air-gapped managed pilot reuses a security_only public claim; broadening
    // it to full_support is a no-overclaim violation.
    let p = r
        .pack("eval-pack-managed-airgapped-managed-pilot")
        .expect("managed pilot pack");
    assert_eq!(p.public_support_class, SupportClass::SecurityOnly);
    let p = r
        .packs
        .iter_mut()
        .find(|p| p.entry_id == "eval-pack-managed-airgapped-managed-pilot")
        .unwrap();
    p.pack_support_class = SupportClass::FullSupport;
    for d in &mut p.destinations {
        d.rendered_support_class = SupportClass::FullSupport;
    }
    assert!(r.validate().iter().any(|v| matches!(
        v,
        EvalPackViolation::PackSupportClassExceedsPublicClaim { .. }
    )));
}

#[test]
fn validate_flags_destination_copy_drift() {
    let mut r = register();
    r.packs[0].destinations[0].rendered_claim_text =
        "Hand-edited pilot-only marketing copy that drifted from the public claim.".to_owned();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::DestinationCopyDrift { .. })));
}

#[test]
fn validate_flags_an_undisclosed_known_issue() {
    let mut r = register();
    let p = r
        .packs
        .iter_mut()
        .find(|p| !p.known_issues_delta.is_empty())
        .expect("a pack with a known-issues delta exists");
    p.known_issues_delta[0].disclosed = false;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::KnownIssueNotDisclosed { .. })));
}

#[test]
fn validate_flags_a_destination_hiding_freshness() {
    let mut r = register();
    r.packs[0].destinations[0].discloses_freshness = false;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        EvalPackViolation::DestinationFreshnessNotDisclosed { .. }
    )));
}

#[test]
fn validate_flags_a_missing_required_destination() {
    let mut r = register();
    r.packs[0]
        .destinations
        .retain(|d| d.destination != EvalPackDestination::EvaluationPack);
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::RequiredDestinationUncovered { .. })));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, EvalPackViolation::PromotionDecisionInconsistent { .. })));
}

#[test]
fn export_projection_carries_one_wording_per_pack() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.packs.len());
    for (p, proj) in r.packs.iter().zip(&projection.rows) {
        assert_eq!(p.entry_id, proj.entry_id);
        assert_eq!(p.publishes_stable(), proj.publishes_stable);
        assert_eq!(p.pack_claim_text, proj.pack_claim_text);
        assert_eq!(p.proof_packet.slo_state, proj.freshness_state);
        assert_eq!(p.deployment_caveats, proj.deployment_caveats);
        assert_eq!(p.bundle_id, proj.bundle_id);
        assert_eq!(p.known_issues_delta.len(), proj.known_issue_count);
        assert_eq!(p.mirror_refs.len(), proj.mirror_count);
    }
}
