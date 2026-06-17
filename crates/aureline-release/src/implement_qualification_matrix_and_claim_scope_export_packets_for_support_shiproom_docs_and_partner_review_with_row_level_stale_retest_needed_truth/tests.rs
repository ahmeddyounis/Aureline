use super::*;

fn register() -> ClaimScopeExportRegister {
    current_m5_claim_scope_export_packets().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_CLAIM_SCOPE_EXPORT_PACKETS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_CLAIM_SCOPE_EXPORT_PACKETS_RECORD_KIND);
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
fn every_row_drives_the_required_audiences() {
    let r = register();
    for row in &r.rows {
        let driven: BTreeSet<ClaimScopeAudience> =
            row.audiences.iter().map(|a| a.audience).collect();
        for required in ClaimScopeAudience::REQUIRED {
            assert!(
                driven.contains(&required),
                "row {} must drive required audience {}",
                row.entry_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn every_audience_reuses_one_row() {
    let r = register();
    for row in &r.rows {
        for a in &row.audiences {
            assert_eq!(
                a.source_row_id,
                row.entry_id,
                "audience {} on {} must render from the one row",
                a.audience.as_str(),
                row.entry_id
            );
            assert_eq!(a.rendered_label, row.published_label);
            assert_eq!(a.rendered_support_class, row.scope_support_class);
            assert_eq!(a.rendered_claim_text, row.scope_claim_text);
            assert!(a.discloses_freshness);
            if !row.active_scope_reasons.is_empty() {
                assert!(a.discloses_scope_reasons);
            }
            if !row.scope_caveats.is_empty() {
                assert!(a.discloses_caveats);
            }
            if a.audience.must_reopen_authoritative_row() {
                assert!(a.reopens_authoritative_row);
            }
        }
    }
}

#[test]
fn no_row_is_greener_than_its_public_claim() {
    let r = register();
    for row in &r.rows {
        assert!(
            row.published_label.rank() <= row.source_published_label.rank(),
            "row {} may not publish greener than its public claim",
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
fn every_row_carries_reopen_refs() {
    let r = register();
    for row in &r.rows {
        let kinds: BTreeSet<ScopeEvidenceKind> = row.evidence_refs.iter().map(|e| e.kind).collect();
        assert!(
            kinds.contains(&ScopeEvidenceKind::QualificationRow),
            "row {} must point at its qualification row",
            row.entry_id
        );
        assert!(
            kinds.contains(&ScopeEvidenceKind::ClaimManifest),
            "row {} must point at its claim manifest",
            row.entry_id
        );
        assert!(!row.qualification_row_ref.trim().is_empty());
        assert!(!row.deprecation_packet_ref.trim().is_empty());
        assert!(!row.claim_manifest_entry_ref.trim().is_empty());
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
        r.summary.rows_published + r.summary.rows_narrowed,
        r.rows.len()
    );
}

#[test]
fn does_not_collapse_to_one_global_flag() {
    let r = register();
    // The summary keeps per-state and per-row truth, never a single green/red flag.
    let states: BTreeSet<ClaimScopeRowState> = r.rows.iter().map(|row| row.export_state).collect();
    assert!(
        states.len() >= 3,
        "export must keep distinct per-row states"
    );
    assert!(states.contains(&ClaimScopeRowState::Published));
    assert!(states.contains(&ClaimScopeRowState::NarrowedRetestPending));
    assert!(states.contains(&ClaimScopeRowState::NarrowedStale));
}

#[test]
fn preserves_row_level_stale_and_retest_reasons() {
    let r = register();
    let reasons: BTreeSet<ClaimScopeReason> = r
        .rows
        .iter()
        .flat_map(|row| row.active_scope_reasons.iter().copied())
        .collect();
    assert!(reasons.contains(&ClaimScopeReason::RowDowngraded));
    assert!(reasons.contains(&ClaimScopeReason::RetestPending));
    assert!(reasons.contains(&ClaimScopeReason::QualificationStale));
    assert!(reasons.contains(&ClaimScopeReason::EvidenceStale));
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
fn every_scope_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<ClaimScopeReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in ClaimScopeReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn an_inherited_row_downgrade_does_not_block_promotion() {
    let r = register();
    // The companion row inherits a Beta public claim but introduces no export-layer
    // failure, so it is not a promotion blocker.
    let companion = r
        .row("claim-scope-companion-handoff")
        .expect("companion row");
    assert!(!companion.publishes_stable());
    assert!(companion.has_active_reason(ClaimScopeReason::RowDowngraded));
    assert!(!r
        .computed_blocking_claim_ids()
        .contains(&companion.entry_id));
}

#[test]
fn an_export_layer_failure_on_a_stable_public_claim_blocks_promotion() {
    let r = register();
    // The remote-helper row rides a still-Stable public claim but its export evidence
    // went stale -> a blocker.
    let remote = r
        .row("claim-scope-remote-helper-skew")
        .expect("remote helper row");
    assert!(remote.source_holds_stable());
    assert!(remote.has_active_reason(ClaimScopeReason::EvidenceStale));
    assert!(r.computed_blocking_claim_ids().contains(&remote.entry_id));
    assert_eq!(r.promotion.decision, PromotionDecision::Hold);
}

#[test]
fn validate_flags_a_published_row_with_active_gap() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a published row exists");
    row.active_scope_reasons
        .push(ClaimScopeReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::PublishedWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_row_over_claiming_the_public_label() {
    let mut r = register();
    let row = r
        .rows
        .iter_mut()
        .find(|row| !row.source_published_label.is_at_or_above_cutline())
        .expect("a row reusing a below-cutline public claim exists");
    row.published_label = StableClaimLevel::Stable;
    for a in &mut row.audiences {
        a.rendered_label = StableClaimLevel::Stable;
    }
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::RowLabelExceedsSource { .. })));
}

#[test]
fn validate_flags_audience_copy_drift() {
    let mut r = register();
    r.rows[0].audiences[0].rendered_claim_text =
        "Hand-edited shiproom copy that drifted from the public claim.".to_owned();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::AudienceCopyDrift { .. })));
}

#[test]
fn validate_flags_an_audience_hiding_freshness() {
    let mut r = register();
    r.rows[0].audiences[0].discloses_freshness = false;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::AudienceFreshnessNotDisclosed { .. })));
}

#[test]
fn validate_flags_a_shiproom_audience_without_reopen_ref() {
    let mut r = register();
    let row = &mut r.rows[0];
    let shiproom = row
        .audiences
        .iter_mut()
        .find(|a| a.audience == ClaimScopeAudience::Shiproom)
        .expect("a shiproom rendering exists");
    shiproom.reopens_authoritative_row = false;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::ReopenRefNotDisclosed { .. })));
}

#[test]
fn validate_flags_a_missing_required_audience() {
    let mut r = register();
    r.rows[0]
        .audiences
        .retain(|a| a.audience != ClaimScopeAudience::Support);
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::RequiredAudienceUncovered { .. })));
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
    row.active_scope_reasons
        .retain(|reason| *reason != ClaimScopeReason::RetestPending);
    for a in &mut row.audiences {
        a.discloses_scope_reasons = !row.active_scope_reasons.is_empty();
    }
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::RowStateWithoutReason { .. })));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ClaimScopeViolation::PromotionDecisionInconsistent { .. })));
}

#[test]
fn export_projection_carries_one_wording_per_row() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, proj) in r.rows.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.scope_claim_text, proj.scope_claim_text);
        assert_eq!(row.freshness_state(), proj.freshness_state);
        assert_eq!(row.scope_caveats, proj.scope_caveats);
        assert_eq!(row.qualification_row_ref, proj.qualification_row_ref);
        assert_eq!(row.deprecation_packet_ref, proj.deprecation_packet_ref);
        assert_eq!(row.evidence_refs.len(), proj.evidence_ref_count);
        assert_eq!(row.active_scope_reasons, proj.active_scope_reasons);
    }
}
