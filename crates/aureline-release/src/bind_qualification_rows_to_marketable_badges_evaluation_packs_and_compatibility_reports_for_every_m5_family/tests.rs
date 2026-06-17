use super::*;

fn register() -> QualificationBadgeBindingRegister {
    current_m5_qualification_badge_bindings().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        BIND_M5_QUALIFICATION_BADGE_BINDINGS_SCHEMA_VERSION
    );
    assert_eq!(
        r.record_kind,
        BIND_M5_QUALIFICATION_BADGE_BINDINGS_RECORD_KIND
    );
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.bindings.is_empty());
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.bindings_for_kind(kind).is_empty(),
            "family kind {} must have at least one binding",
            kind.as_str()
        );
    }
}

#[test]
fn every_binding_renders_on_the_truth_surfaces() {
    let r = register();
    for b in &r.bindings {
        for surface in BadgeSurface::TRUTH_SURFACES {
            assert!(
                b.surfaces.contains(&surface),
                "binding {} must render the badge on truth surface {}",
                b.entry_id,
                surface.as_str()
            );
        }
    }
}

#[test]
fn every_badge_discloses_freshness_and_never_exceeds_the_row() {
    let r = register();
    for b in &r.bindings {
        assert!(
            b.badge.freshness_disclosed,
            "binding {} badge must disclose freshness",
            b.entry_id
        );
        assert_eq!(b.badge.badge_label, b.published_label);
        assert_eq!(b.badge.freshness_state, b.proof_packet.slo_state);
        assert!(
            b.published_label.rank() <= b.row_published_label.rank(),
            "binding {} badge may not exceed the row",
            b.entry_id
        );
        assert!(
            b.row_published_label.rank() <= b.claim_label.rank(),
            "binding {} row may not exceed the claim",
            b.entry_id
        );
        if !b.badge.caveat_summary.is_empty() {
            assert!(b.badge.caveats_disclosed);
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let r = register();
    assert!(!r.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_bindings()
        .iter()
        .map(|b| b.family_ref.as_str())
        .collect();
    for declared in &r.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking binding"
        );
    }
}

#[test]
fn summary_counts_match_bindings() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.bindings_published + r.summary.bindings_narrowed,
        r.bindings.len()
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
    let covered: BTreeSet<BindingNarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in BindingNarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn an_inherited_row_narrowing_does_not_block_promotion() {
    let r = register();
    // The companion badge inherits a Beta row but introduces no binding-layer
    // failure, so it is not a promotion blocker.
    let companion = r.binding("m5-badge-companion").expect("companion binding");
    assert!(!companion.publishes_stable());
    assert!(companion.has_active_reason(BindingNarrowingReason::QualificationRowNarrowed));
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
}

#[test]
fn validate_flags_a_held_binding_with_active_gap() {
    let mut r = register();
    let b = r
        .bindings
        .iter_mut()
        .find(|b| b.publishes_stable())
        .expect("a published binding exists");
    b.active_narrowing_reasons
        .push(BindingNarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        QualificationBadgeBindingViolation::HeldWithActiveGap { .. }
    )));
}

#[test]
fn validate_flags_a_badge_over_claiming_the_row() {
    let mut r = register();
    let b = r
        .bindings
        .iter_mut()
        .find(|b| !b.row_published_label.is_at_or_above_cutline())
        .expect("a narrowed-row binding exists");
    b.published_label = StableClaimLevel::Stable;
    b.badge.badge_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        QualificationBadgeBindingViolation::BadgePublishedWiderThanRow { .. }
    )));
}

#[test]
fn validate_flags_an_undisclosed_freshness_badge() {
    let mut r = register();
    r.bindings[0].badge.freshness_disclosed = false;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        QualificationBadgeBindingViolation::FreshnessNotDisclosed { .. }
    )));
}

#[test]
fn validate_flags_a_missing_truth_surface() {
    let mut r = register();
    r.bindings[0]
        .surfaces
        .retain(|s| *s != BadgeSurface::ServiceHealth);
    assert!(r.validate().iter().any(|v| matches!(
        v,
        QualificationBadgeBindingViolation::TruthSurfaceUncovered { .. }
    )));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        QualificationBadgeBindingViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_carries_freshness_and_caveats() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.bindings.len());
    for (b, proj) in r.bindings.iter().zip(&projection.rows) {
        assert_eq!(b.entry_id, proj.entry_id);
        assert_eq!(b.publishes_stable(), proj.publishes_stable);
        assert_eq!(b.badge.freshness_state, proj.freshness_state);
        assert_eq!(b.badge.caveat_summary, proj.caveat_summary);
    }
}
