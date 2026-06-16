use super::*;

fn register() -> ContractDiffReportRegister {
    current_m5_public_interface_diff_reports().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        M5_PUBLIC_INTERFACE_DIFF_REPORTS_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, M5_PUBLIC_INTERFACE_DIFF_REPORTS_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.reports.is_empty());
}

#[test]
fn covers_every_contract_kind() {
    let r = register();
    for kind in ContractKind::ALL {
        assert!(
            !r.reports_for_kind(kind).is_empty(),
            "contract kind {} must have at least one report",
            kind.as_str()
        );
    }
}

#[test]
fn covers_every_change_class() {
    let r = register();
    for class in ChangeClass::ALL {
        assert!(
            !r.reports_for_change_class(class).is_empty(),
            "change class {} must have at least one report",
            class.as_str()
        );
    }
}

#[test]
fn breaking_changes_show_their_diff_surface() {
    let r = register();
    for row in &r.reports {
        if row.change_class == ChangeClass::Breaking {
            assert!(
                row.interface_diff.has_incompatible_surface(),
                "breaking report {} must show removed or changed surface",
                row.entry_id
            );
        }
    }
}

#[test]
fn additive_changes_remove_no_surface() {
    let r = register();
    for row in &r.reports {
        if row.change_class == ChangeClass::Additive {
            assert!(
                row.interface_diff.removed.is_empty(),
                "additive report {} must not remove surface",
                row.entry_id
            );
        }
    }
}

#[test]
fn held_breaking_change_carries_a_complete_packet() {
    let r = register();
    let held_breaking = r
        .reports
        .iter()
        .find(|row| row.holds_label() && row.change_class == ChangeClass::Breaking);
    let row = held_breaking.expect("a held breaking change exists");
    let packet = row
        .deprecation_packet
        .as_ref()
        .expect("a held breaking change carries a deprecation packet");
    assert!(
        packet.is_complete(),
        "a held breaking change's deprecation packet must be complete"
    );
    assert!(!packet.removal_overdue);
}

#[test]
fn covers_every_declared_release_blocking_contract() {
    let r = register();
    assert!(!r.release_blocking_contract_refs.is_empty());
    let covered: Vec<&str> = r
        .release_blocking_reports()
        .iter()
        .map(|row| row.contract_ref.as_str())
        .collect();
    for declared in &r.release_blocking_contract_refs {
        assert!(
            covered.contains(&declared.as_str()),
            "{declared} has no covering release-blocking report"
        );
    }
}

#[test]
fn summary_counts_match_reports() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert_eq!(
        r.summary.reports_publishing_stable + r.summary.reports_narrowed,
        r.reports.len()
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
        r.computed_blocking_entry_ids()
    );
}

#[test]
fn every_narrowing_reason_has_a_stop_rule() {
    let r = register();
    let covered: BTreeSet<NarrowingReason> = r
        .stop_rules
        .iter()
        .map(|rule| rule.trigger_reason)
        .collect();
    for reason in NarrowingReason::ALL {
        assert!(covered.contains(&reason), "{}", reason.as_str());
    }
}

#[test]
fn validate_flags_a_held_report_with_active_gap() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held report exists");
    row.active_narrowing_reasons
        .push(NarrowingReason::EvidenceStale);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ContractDiffReportViolation::HeldWithActiveGap { .. })));
}

#[test]
fn validate_flags_a_breaking_change_held_without_packet() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.change_class == ChangeClass::Breaking && row.deprecation_packet.is_none())
        .expect("a breaking unpacketed report exists");
    row.report_state = ReportState::Published;
    row.published_label = StableClaimLevel::Stable;
    row.active_narrowing_reasons.clear();
    row.support_caveat.support_class = SupportClass::SupportedWithCaveats;
    row.support_caveat.caveats = vec!["forced".to_owned()];
    r.summary = r.computed_summary();
    assert!(r.validate().iter().any(|v| matches!(
        v,
        ContractDiffReportViolation::BreakingHeldWithoutPacket { .. }
    )));
}

#[test]
fn validate_flags_an_incomplete_deprecation_packet() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.deprecation_packet.is_some())
        .expect("a packeted report exists");
    if let Some(packet) = row.deprecation_packet.as_mut() {
        packet.rollback_implications = None;
    }
    // Force it to a holding state so the incomplete packet is the live failure.
    row.report_state = ReportState::Published;
    row.published_label = StableClaimLevel::Stable;
    row.active_narrowing_reasons.clear();
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ContractDiffReportViolation::IncompletePacketHeld { .. })));
}

#[test]
fn validate_flags_a_missing_reader_writer_review() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.publishes_stable())
        .expect("a held report exists");
    row.interface_diff.writer_posture = ReviewPosture::Unreviewed;
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ContractDiffReportViolation::ReviewPendingHeld { .. })));
}

#[test]
fn validate_flags_an_inconsistent_promotion_decision() {
    let mut r = register();
    r.promotion.decision = PromotionDecision::Proceed;
    assert!(r.validate().iter().any(|v| matches!(
        v,
        ContractDiffReportViolation::PromotionDecisionInconsistent { .. }
    )));
}

#[test]
fn validate_flags_a_limited_report_without_caveat() {
    let mut r = register();
    let row = r
        .reports
        .iter_mut()
        .find(|row| row.report_state == ReportState::Limited)
        .expect("a limited report exists");
    row.support_caveat.caveats.clear();
    assert!(r
        .validate()
        .iter()
        .any(|v| matches!(v, ContractDiffReportViolation::LimitedWithoutCaveat { .. })));
}

#[test]
fn export_projection_mirrors_reports() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.reports.len());
    for (row, proj) in r.reports.iter().zip(&projection.rows) {
        assert_eq!(row.entry_id, proj.entry_id);
        assert_eq!(row.publishes_stable(), proj.publishes_stable);
        assert_eq!(row.change_class, proj.change_class);
        assert_eq!(row.support_caveat.support_class, proj.support_class);
        assert_eq!(row.compatibility_window.support_state, proj.support_state);
        assert_eq!(
            row.deprecation_packet.as_ref().map(|p| p.status),
            proj.deprecation_status
        );
    }
}
