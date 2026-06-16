use super::*;

fn register() -> M5RehearsalAutomationRegister {
    current_m5_rehearsal_automation_register().expect("register parses")
}

#[test]
fn embedded_register_parses_and_validates() {
    let r = register();
    assert_eq!(
        r.schema_version,
        SHIP_M5_REHEARSAL_AUTOMATION_SCHEMA_VERSION
    );
    assert_eq!(r.record_kind, SHIP_M5_REHEARSAL_AUTOMATION_RECORD_KIND);
    assert_eq!(r.validate(), Vec::new());
    assert!(!r.rows.is_empty());
}

#[test]
fn covers_every_family_kind_with_four_rehearsals() {
    let r = register();
    for kind in M5ArtifactFamilyKind::ALL {
        assert!(
            !r.rows_for_kind(kind).is_empty(),
            "family kind {} must have at least one row",
            kind.as_str()
        );
    }
    for row in &r.rows {
        for kind in RehearsalKind::ALL {
            assert!(
                row.rehearsal(kind).is_some(),
                "row {} missing rehearsal {}",
                row.entry_id,
                kind.as_str()
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
        .into_iter()
        .map(|row| row.subject_ref.as_str())
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
        r.summary.entries_holding_stable + r.summary.entries_narrowed,
        r.rows.len()
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
fn stale_or_red_rehearsal_holds_promotion() {
    let r = register();
    // The automation must actually fail promotion when a release-blocking row has
    // a stale, red, missing, or guardrail-tripped rehearsal.
    let narrowed = r.rows_narrowed();
    assert!(
        !narrowed.is_empty(),
        "fixture should exercise at least one narrowed family"
    );
    assert_eq!(r.computed_promotion_decision(), PromotionDecision::Hold);
}

#[test]
fn warm_cache_only_rebuild_never_counts_as_proof() {
    let r = register();
    let cache_only: Vec<&M5RehearsalRow> = r
        .rows
        .iter()
        .filter(|row| row.rebuild_cache_only())
        .collect();
    assert!(
        !cache_only.is_empty(),
        "fixture should exercise the warm-cache-only guardrail"
    );
    for row in cache_only {
        assert!(!row.publishes_stable(), "cache-only rebuild must narrow");
        assert!(row.has_active_reason(RehearsalGapReason::RebuildCacheOnly));
        let rebuild = row.rehearsal(RehearsalKind::CleanRoomRebuild).unwrap();
        assert!(!rebuild.is_proven());
    }
}

#[test]
fn symbolication_freshness_stays_coupled_to_release_center() {
    let r = register();
    let decoupled: Vec<&M5RehearsalRow> = r
        .rows
        .iter()
        .filter(|row| row.symbolication_decoupled())
        .collect();
    assert!(
        !decoupled.is_empty(),
        "fixture should exercise the symbolication-decoupling guardrail"
    );
    for row in decoupled {
        assert!(!row.publishes_stable());
        assert!(row.has_active_reason(RehearsalGapReason::SymbolicationFreshnessDecoupled));
    }
}

#[test]
fn expiry_feed_has_one_entry_per_rehearsal() {
    let r = register();
    let feed = r.rehearsal_expiry_feed();
    assert_eq!(feed.entries.len(), r.rows.len() * RehearsalKind::ALL.len());
    assert_eq!(feed.promotion_decision, r.promotion.decision);
    // The feed's freshness rollup must match the summary the dashboards also read.
    let breached = feed
        .entries
        .iter()
        .filter(|e| e.slo_state == FreshnessSloState::Breached)
        .count();
    assert_eq!(breached, r.summary.rehearsals_breached);
}

#[test]
fn export_projection_is_lossless_on_labels() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.rows.len(), r.rows.len());
    for (row, exported) in r.rows.iter().zip(projection.rows.iter()) {
        assert_eq!(row.entry_id, exported.entry_id);
        assert_eq!(row.published_label, exported.published_label);
        assert_eq!(row.publishes_stable(), exported.publishes_stable);
    }
}
