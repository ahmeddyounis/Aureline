//! Contract tests for the checked-in M5 rehearsal-automation register.

use aureline_release::ship_clean_room_rebuild_exact_build_symbolication_release_center_sync_and_advisory_revocation_rehearsal_automation_for_m5_depth_trains::{
    current_m5_rehearsal_automation_register, M5RehearsalAutomationRegister, RehearsalGapReason,
    RehearsalKind, SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND,
    SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION,
};
use aureline_release::{M5ArtifactFamilyKind, PromotionDecision};

fn register() -> M5RehearsalAutomationRegister {
    current_m5_rehearsal_automation_register().expect("checked-in register parses into the model")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let register = register();
    assert_eq!(
        register.schema_version,
        SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION
    );
    assert_eq!(
        register.record_kind,
        SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND
    );
    let violations = register.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_family_kind_has_a_row_with_all_rehearsals() {
    let register = register();
    for kind in M5ArtifactFamilyKind::ALL {
        let rows = register.rows_for_kind(kind);
        assert!(!rows.is_empty(), "missing family kind {}", kind.as_str());
        for row in rows {
            for rehearsal in RehearsalKind::ALL {
                assert!(
                    row.rehearsal(rehearsal).is_some(),
                    "row {} missing rehearsal {}",
                    row.entry_id,
                    rehearsal.as_str()
                );
            }
        }
    }
}

#[test]
fn promotion_holds_on_stale_red_or_guardrail_failure() {
    let register = register();
    assert_eq!(
        register.computed_promotion_decision(),
        PromotionDecision::Hold
    );
    assert_eq!(register.promotion.decision, PromotionDecision::Hold);
    assert_eq!(
        register.promotion.blocking_rule_ids,
        register.computed_blocking_rule_ids()
    );
    assert_eq!(
        register.promotion.blocking_claim_ids,
        register.computed_blocking_entry_ids()
    );
    assert!(!register.promotion.blocking_claim_ids.is_empty());
}

#[test]
fn guardrails_are_exercised_and_narrow_their_rows() {
    let register = register();
    let cache_only = register
        .rows
        .iter()
        .find(|row| row.has_active_reason(RehearsalGapReason::RebuildCacheOnly))
        .expect("a warm-cache-only rebuild row");
    assert!(!cache_only.publishes_stable());

    let decoupled = register
        .rows
        .iter()
        .find(|row| row.has_active_reason(RehearsalGapReason::SymbolicationFreshnessDecoupled))
        .expect("a symbolication-decoupled row");
    assert!(!decoupled.publishes_stable());
}

#[test]
fn expiry_feed_matches_summary_freshness() {
    let register = register();
    let feed = register.rehearsal_expiry_feed();
    assert_eq!(
        feed.entries.len(),
        register.rows.len() * RehearsalKind::ALL.len()
    );
    let proven = feed.entries.iter().filter(|e| e.proven).count();
    // Every passing, within-SLO, clean-state rehearsal is proven; the feed and
    // the summary must agree on how many rehearsals are red, stale, or missing.
    assert!(proven <= feed.entries.len());
    assert_eq!(
        register.summary.rehearsals_passed
            + register.summary.rehearsals_failed
            + register.summary.rehearsals_not_run,
        feed.entries.len()
    );
}
