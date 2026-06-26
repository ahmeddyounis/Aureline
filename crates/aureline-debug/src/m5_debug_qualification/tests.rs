//! Unit tests for the M5 debug qualification set.

use super::*;

#[test]
fn canonical_set_validates_and_is_export_safe() {
    let set = m5_debug_qualification_set();
    set.validate().expect("canonical set validates");
    assert!(set.is_support_export_safe());
    assert!(set.all_invariants_hold());
    assert!(set.raw_payload_excluded);
}

#[test]
fn canonical_set_round_trips_through_serde() {
    let set = m5_debug_qualification_set();
    let json = serde_json::to_string(&set).expect("serializes");
    let back: DebugQualificationSet = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(set, back);
}

#[test]
fn every_object_class_is_claimed() {
    let set = m5_debug_qualification_set();
    for class in DebugObjectClass::ALL {
        assert!(
            set.covers_object_class(class),
            "object class {} is not claimed by any row",
            class.as_str()
        );
    }
}

#[test]
fn every_category_and_status_is_materialized() {
    let set = m5_debug_qualification_set();
    for category in DebugRowCategory::ALL {
        assert!(
            set.row_in_category(category).is_some(),
            "missing category {}",
            category.as_str()
        );
    }
    for status in DebugQualificationStatus::ALL {
        assert!(
            set.row_with_status(status).is_some(),
            "missing status {}",
            status.as_str()
        );
    }
}

#[test]
fn every_channel_is_published_once() {
    let set = m5_debug_qualification_set();
    for channel in ClaimPublicationChannel::ALL {
        assert!(
            set.publication_for_channel(channel).is_some(),
            "missing channel {}",
            channel.as_str()
        );
    }
}

#[test]
fn stable_is_only_published_with_certified_supported_exact_evidence() {
    let set = m5_debug_qualification_set();
    for r in &set.qualification_rows {
        if r.published_maturity.is_stable() {
            assert_eq!(r.status, DebugQualificationStatus::Certified);
            assert_eq!(r.support_class, DebugSupportClass::Supported);
            assert!(r.mapping_fidelity.preserves_exact_source());
        }
    }
}

#[test]
fn degraded_evidence_always_narrows_below_stable() {
    let set = m5_debug_qualification_set();
    for r in &set.qualification_rows {
        if r.status.triggers_narrowing() {
            assert!(
                !r.published_maturity.is_stable(),
                "row {} has degraded status {} but still publishes stable",
                r.row_id,
                r.status.as_str()
            );
        }
    }
}

#[test]
fn narrowed_rows_carry_a_reason() {
    let set = m5_debug_qualification_set();
    for r in &set.qualification_rows {
        assert_eq!(
            r.narrowed,
            !r.narrowing_reason.is_empty(),
            "row {} narrowing reason disagrees with narrowed flag",
            r.row_id
        );
    }
    // The aging core-debug row was historically claimed stable and must narrow.
    let aging = set
        .row("debug.qual:core_variables_evaluate:0003")
        .expect("aging row exists");
    assert!(aging.narrowed);
    assert_eq!(aging.published_maturity, DebugClaimMaturity::RetestPending);
}

#[test]
fn publications_republish_the_floor_of_their_rows() {
    let set = m5_debug_qualification_set();
    for p in &set.claim_publications {
        let floor = p
            .row_refs
            .iter()
            .filter_map(|id| set.row(id))
            .fold(p.claimed_maturity, |acc, r| {
                acc.narrower(r.published_maturity)
            });
        assert_eq!(p.published_maturity, floor, "{} floor", p.publication_id);
        assert!(p.published_maturity.rank() <= DebugClaimMaturity::Withdrawn.rank());
    }
    // The claim board narrows because aging core evidence floors it to retest-pending.
    let board = set
        .publication_for_channel(ClaimPublicationChannel::ClaimBoard)
        .expect("claim board exists");
    assert!(board.narrowed);
    assert_eq!(board.published_maturity, DebugClaimMaturity::RetestPending);
    // The release packet covers only ship-stable rows and stays stable.
    let release = set
        .publication_for_channel(ClaimPublicationChannel::ReleasePacket)
        .expect("release packet exists");
    assert!(!release.narrowed);
    assert_eq!(release.published_maturity, DebugClaimMaturity::Stable);
}

#[test]
fn active_rules_cover_every_triggered_row() {
    let set = m5_debug_qualification_set();
    for rule in set.downgrade_rules.iter().filter(|r| r.active) {
        for row in &set.qualification_rows {
            if row.degradations().contains(&rule.trigger) {
                assert!(
                    rule.affected_row_refs.contains(&row.row_id),
                    "rule {} omits triggered row {}",
                    rule.rule_id,
                    row.row_id
                );
                assert!(row.published_maturity.rank() >= rule.resulting_maturity.rank());
            }
        }
    }
}

#[test]
fn tampering_with_a_published_maturity_fails_validation() {
    let mut set = m5_debug_qualification_set();
    // Force a degraded row to claim stable without earning it.
    let row = set
        .qualification_rows
        .iter_mut()
        .find(|r| r.status.triggers_narrowing())
        .expect("a degraded row exists");
    row.published_maturity = DebugClaimMaturity::Stable;
    assert!(
        set.validate().is_err(),
        "a row publishing unearned stable must fail validation"
    );
}

#[test]
fn tampering_with_a_publication_floor_fails_validation() {
    let mut set = m5_debug_qualification_set();
    let board = set
        .claim_publications
        .iter_mut()
        .find(|p| p.channel == ClaimPublicationChannel::ClaimBoard)
        .expect("claim board exists");
    board.published_maturity = DebugClaimMaturity::Stable;
    board.narrowed = false;
    board.narrowing_reason = String::new();
    assert!(
        set.validate().is_err(),
        "a publication claiming wider than its floor must fail validation"
    );
}

#[test]
fn dropping_a_triggered_row_from_a_rule_fails_validation() {
    let mut set = m5_debug_qualification_set();
    let rule = set
        .downgrade_rules
        .iter_mut()
        .find(|r| !r.affected_row_refs.is_empty())
        .expect("a rule with rows exists");
    rule.affected_row_refs.clear();
    assert!(
        set.validate().is_err(),
        "an active rule omitting a triggered row must fail validation"
    );
}

#[test]
fn lines_projection_lists_every_family() {
    let set = m5_debug_qualification_set();
    let lines = m5_debug_qualification_lines(&set);
    assert!(lines.iter().any(|l| l.contains("Qualification rows:")));
    assert!(lines.iter().any(|l| l.contains("Claim publications:")));
    assert!(lines.iter().any(|l| l.contains("Downgrade rules:")));
    assert!(lines.iter().any(|l| l.contains("Invariants:")));
    for r in &set.qualification_rows {
        assert!(lines.iter().any(|l| l.contains(&r.row_id)));
    }
}
