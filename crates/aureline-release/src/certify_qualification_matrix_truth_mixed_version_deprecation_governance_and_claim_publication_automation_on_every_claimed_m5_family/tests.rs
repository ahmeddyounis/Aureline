use super::*;

fn register() -> M5FamilyCertificationRegister {
    current_m5_family_certification().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_FAMILY_CERTIFICATION_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_FAMILY_CERTIFICATION_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.rows.is_empty());
}

#[test]
fn covers_every_family_kind() {
    let r = register();
    for kind in FamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
}

#[test]
fn every_family_binds_the_four_governance_pillars() {
    let r = register();
    for row in &r.rows {
        let kinds: BTreeSet<CertificationPillarKind> = row.pillars.iter().map(|p| p.kind).collect();
        for required in CertificationPillarKind::REQUIRED {
            assert!(
                kinds.contains(&required),
                "row {} must bind governance pillar {}",
                row.entry_id,
                required.as_str()
            );
        }
        // Each pillar reuses the packet's reopen ref so the bound record and the packet
        // agree.
        assert_eq!(
            row.pillar(CertificationPillarKind::QualificationMatrix)
                .map(|p| p.pillar_ref.as_str()),
            Some(row.qualification_row_ref.as_str())
        );
        assert_eq!(
            row.pillar(CertificationPillarKind::ClaimPublication)
                .map(|p| p.pillar_ref.as_str()),
            Some(row.claim_manifest_entry_ref.as_str())
        );
    }
}

#[test]
fn no_family_is_greener_than_its_public_claim() {
    let r = register();
    for row in &r.rows {
        assert!(
            row.certified_label.rank() <= row.source_published_label.rank(),
            "row {} may not certify greener than its public claim",
            row.entry_id
        );
        assert!(
            !row.over_claims_source(),
            "row {} may not over-claim the public label or support class",
            row.entry_id
        );
    }
}

#[test]
fn certified_rows_reuse_the_public_claim_at_parity() {
    let r = register();
    let certified: Vec<&FamilyCertificationPacket> = r
        .rows
        .iter()
        .filter(|row| row.holds_certification())
        .collect();
    assert!(
        !certified.is_empty(),
        "the register must certify some families"
    );
    for row in certified {
        assert_eq!(row.certified_label, row.source_published_label);
        assert_eq!(row.certified_support_class, row.source_support_class);
        assert!(row.certifies_stable());
        assert!(row.active_certification_reasons.is_empty());
        for p in &row.pillars {
            assert!(
                p.state.is_current(),
                "certified row {} pillar {} must be current",
                row.entry_id,
                p.kind.as_str()
            );
        }
    }
}

#[test]
fn covers_every_declared_release_blocking_family() {
    let r = register();
    assert!(!r.release_blocking_family_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_rows()
        .iter()
        .map(|row| row.family_ref.as_str())
        .collect();
    for declared in &r.release_blocking_family_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking row"
        );
    }
}

#[test]
fn summary_counts_match_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.rows_certified + r.summary.rows_narrowed,
        r.rows.len()
    );
}

#[test]
fn does_not_collapse_to_one_global_flag() {
    let r = register();
    // The summary keeps per-state and per-pillar truth, never a single green/red flag.
    let states: BTreeSet<CertificationState> =
        r.rows.iter().map(|row| row.certification_state).collect();
    assert!(
        states.len() >= 3,
        "certification must keep distinct per-row states"
    );
    assert!(states.contains(&CertificationState::Certified));
    assert!(states.contains(&CertificationState::NarrowedRetestPending));
    assert!(states.contains(&CertificationState::NarrowedStale));
    // At least one pillar carries a stale state independent of the others.
    assert!(r.summary.pillars_stale >= 1);
}

#[test]
fn preserves_row_level_stale_and_retest_reasons() {
    let r = register();
    let reasons: BTreeSet<CertificationReason> = r
        .rows
        .iter()
        .flat_map(|row| row.active_certification_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&CertificationReason::RowDowngraded));
    assert!(reasons.contains(&CertificationReason::RetestPending));
    assert!(reasons.contains(&CertificationReason::QualificationStale));
    assert!(reasons.contains(&CertificationReason::EvidenceStale));
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
fn every_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<CertificationReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in CertificationReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn an_inherited_row_downgrade_does_not_block_promotion() {
    let r = register();
    // The companion family inherits a Beta public claim but introduces no
    // certification-layer failure, so it is not a promotion blocker.
    let companion = r.row("cert-companion-handoff").expect("companion row");
    assert!(!companion.certifies_stable());
    assert!(companion.has_active_reason(CertificationReason::RowDowngraded));
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
}

#[test]
fn a_certification_layer_failure_on_a_stable_public_claim_blocks_promotion() {
    let r = register();
    // The remote-helper family rides a still-Stable public claim but its certification
    // evidence went stale -> a blocker.
    let remote = r.row("cert-remote-helper-skew").expect("remote helper row");
    assert!(remote.source_holds_stable());
    assert!(remote.has_active_reason(CertificationReason::EvidenceStale));
    assert!(r.computed_blocking_claim_ids().contains(&remote.entry_id));
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
}

#[test]
fn validate_flags_a_certified_row_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.holds_certification())
        .expect("a certified row exists");
    row.active_certification_reasons
        .push(CertificationReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, CertificationViolation::CertifiedWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_family_over_claiming_the_public_label() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.source_published_label.is_at_or_above_cutline())
        .expect("a row reusing a below-cutline public claim exists");
    row.certified_label = StableClaimLevel::Stable;
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, CertificationViolation::RowLabelExceedsSource { .. })));
}

#[test]
fn validate_flags_a_missing_required_pillar() {
    let mut r = register();
    r.rows[0]
        .pillars
        .retain(|p| p.kind != CertificationPillarKind::SkewWindow);
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, CertificationViolation::RequiredPillarUncovered { .. })));
}

#[test]
fn validate_flags_a_stale_pillar_without_reason() {
    let mut r = register();
    // Marking a certified row's qualification pillar stale without the matching reason
    // must be rejected, so a pillar that thins out always narrows the certification.
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.holds_certification())
        .expect("a certified row exists");
    if let Some(p) = row
        .pillars
        .iter_mut()
        .find(|p| p.kind == CertificationPillarKind::QualificationMatrix)
    {
        p.state = M5ClaimReportState::Stale;
    }
    r.summary = r.computed_summary();
    let violations = r.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, CertificationViolation::PillarStateWithoutReason { .. })));
}

#[test]
fn validate_flags_a_pillar_ref_drift() {
    let mut r = register();
    if let Some(p) = r.rows[0]
        .pillars
        .iter_mut()
        .find(|p| p.kind == CertificationPillarKind::QualificationMatrix)
    {
        p.pillar_ref = "qualification/some-other-row".to_owned();
    }
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, CertificationViolation::PillarRefDrift { .. })));
}

#[test]
fn validate_flags_a_lost_retest_reason() {
    let mut r = register();
    // Dropping the retest reason from the retest-pending row loses the row-level
    // retest-needed truth -> rejected.
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.row_state == RowState::RetestPending)
        .expect("a retest-pending row exists");
    row.active_certification_reasons
        .retain(|reason| *reason != CertificationReason::RetestPending);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, CertificationViolation::RowStateWithoutReason { .. })));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        CertificationViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn export_projection_carries_one_verdict_per_row() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.certifies_stable(), proj.certifies_stable);
        assert_eq!(row.certified_label, proj.certified_label);
        assert_eq!(row.certified_support_class, proj.certified_support_class);
        assert_eq!(row.freshness_state(), proj.freshness_state);
        assert_eq!(row.certification_caveats, proj.certification_caveats);
        assert_eq!(row.qualification_row_ref, proj.qualification_row_ref);
        assert_eq!(row.skew_window_ref, proj.skew_window_ref);
        assert_eq!(
            row.diff_deprecation_packet_ref,
            proj.diff_deprecation_packet_ref
        );
        assert_eq!(row.pillars.len(), proj.pillar_count);
        assert_eq!(
            row.active_certification_reasons,
            proj.active_certification_reasons
        );
    }
}
