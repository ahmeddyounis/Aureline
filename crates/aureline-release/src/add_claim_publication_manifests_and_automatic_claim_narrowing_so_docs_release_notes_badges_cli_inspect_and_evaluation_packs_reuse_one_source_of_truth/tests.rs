use super::*;

fn register() -> M5ClaimPublicationRegister {
    current_m5_claim_publication_manifests().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_CLAIM_PUBLICATION_MANIFESTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_CLAIM_PUBLICATION_MANIFESTS_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.manifests.is_empty());
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.manifests_for_kind(kind).is_empty(),
            "family kind {} must have at least one manifest",
            kind.as_str()
        );
    }
}

#[test]
fn every_manifest_drives_the_required_destinations() {
    let r = register();
    for m in &r.manifests {
        let driven: BTreeSet<M5ClaimDestination> =
            m.destinations.iter().map(|d| d.destination).collect();
        for required in M5ClaimDestination::REQUIRED {
            assert!(
                driven.contains(&required),
                "manifest {} must drive required destination {}",
                m.entry_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn every_destination_reuses_one_source_of_truth() {
    let r = register();
    for m in &r.manifests {
        for d in &m.destinations {
            assert_eq!(
                d.source_manifest_id,
                r.register_id,
                "destination {} on {} must render from the one manifest",
                d.destination.as_str(),
                m.entry_id
            );
            assert_eq!(d.rendered_label, m.published_label);
            assert_eq!(d.rendered_support_class, m.published_claim.support_class);
            assert_eq!(d.rendered_claim_text, m.published_claim.claim_text);
            assert!(d.discloses_freshness);
            if !m.published_claim.scope_caveats.is_empty() {
                assert!(d.discloses_caveats);
            }
        }
    }
}

#[test]
fn published_claim_never_exceeds_the_row_or_claim() {
    let r = register();
    for m in &r.manifests {
        assert!(
            m.published_label.rank() <= m.row_published_label.rank(),
            "manifest {} claim may not exceed the row",
            m.entry_id
        );
        assert!(
            m.row_published_label.rank() <= m.claim_label.rank(),
            "manifest {} row may not exceed the claim",
            m.entry_id
        );
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let r = register();
    assert!(!r.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_manifests()
        .iter()
        .map(|m| m.family_ref.as_str())
        .collect();
    for declared in &r.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking manifest"
        );
    }
}

#[test]
fn summary_counts_match_manifests() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.manifests_published + r.summary.manifests_narrowed,
        r.manifests.len()
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
    let covered: BTreeSet<M5ClaimNarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in M5ClaimNarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn an_inherited_row_narrowing_does_not_block_promotion() {
    let r = register();
    // The companion claim inherits a Beta row but introduces no manifest-layer
    // failure, so it is not a promotion blocker.
    let companion = r
        .manifest("m5-claim-companion")
        .expect("companion manifest");
    assert!(!companion.publishes_stable());
    assert!(companion.has_active_reason(M5ClaimNarrowingReason::QualificationRowNarrowed));
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
}

#[test]
fn a_manifest_layer_failure_blocks_promotion() {
    let r = register();
    // The toolchain claim has a manifest-layer evidence failure -> a blocker.
    let toolchain = r
        .manifest("m5-claim-toolchain")
        .expect("toolchain manifest");
    assert!(toolchain.has_active_reason(M5ClaimNarrowingReason::EvidenceStale));
    assert!(r
        .computed_blocking_claim_ids()
        .contains(&toolchain.entry_id));
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
}

#[test]
fn validate_flags_a_held_manifest_with_active_gap() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| m.publishes_stable())
        .expect("a published manifest exists");
    m.active_narrowing_reasons
        .push(M5ClaimNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5ClaimPublicationViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_claim_over_claiming_the_row() {
    let mut r = register();
    let m = r
        .manifests
        .iter_mut()
        .find(|m| !m.row_published_label.is_at_or_above_cutline())
        .expect("a narrowed-row manifest exists");
    m.published_label = StableClaimLevel::Stable;
    for d in &mut m.destinations {
        d.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ClaimPublicationViolation::ClaimPublishedWiderThanRow { .. }
    )));
}

#[test]
fn validate_flags_destination_copy_drift() {
    let mut r = register();
    r.manifests[0].destinations[0].rendered_claim_text =
        "Hand-edited marketing copy that drifted from the manifest.".to_owned();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, M5ClaimPublicationViolation::DestinationCopyDrift { .. })));
}

#[test]
fn validate_flags_a_destination_hiding_freshness() {
    let mut r = register();
    r.manifests[0].destinations[0].discloses_freshness = false;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ClaimPublicationViolation::DestinationFreshnessNotDisclosed { .. }
    )));
}

#[test]
fn validate_flags_a_missing_required_destination() {
    let mut r = register();
    r.manifests[0]
        .destinations
        .retain(|d| d.destination != M5ClaimDestination::ReleaseNotes);
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ClaimPublicationViolation::RequiredDestinationUncovered { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        M5ClaimPublicationViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_carries_one_wording_per_row() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.manifests.len());
    for (m, proj) in r.manifests.iter().zip(&projection.rows) {
        assert_eq!(m.entry_id, proj.entry_id);
        assert_eq!(m.publishes_stable(), proj.publishes_stable);
        assert_eq!(m.published_claim.claim_text, proj.claim_text);
        assert_eq!(m.proof_packet.slo_state, proj.freshness_state);
        assert_eq!(m.published_claim.scope_caveats, proj.scope_caveats);
    }
}
