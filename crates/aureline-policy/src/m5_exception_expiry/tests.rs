use super::*;
use std::path::{Path, PathBuf};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_canonical_fixture_matches_seeded_packet() {
    let fixture = repo_root().join("fixtures/governance/m5_exception_expiry/canonical_packet.yaml");
    let raw = std::fs::read_to_string(&fixture).expect("canonical fixture is readable");
    let parsed: M5ExceptionExpiryPacket =
        serde_yaml::from_str(&raw).expect("canonical fixture parses");

    assert!(
        parsed.validate().is_empty(),
        "canonical fixture must validate cleanly: {:?}",
        parsed.validate()
    );
    assert_eq!(
        parsed,
        seeded_m5_exception_expiry_packet(),
        "canonical fixture drifted from the seeded packet; regenerate it"
    );
}

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_exception_expiry_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_exception_is_bounded_and_scoped() {
    let packet = seeded_m5_exception_expiry_packet();
    for row in &packet.rows {
        assert!(row.bounded_by_expiry, "{row:?}");
        assert!(!row.widens_authority, "{row:?}");
        assert!(!row.expires_at.trim().is_empty(), "{row:?}");
        assert!(row.scope_binding.unbound_dimensions().is_empty(), "{row:?}");
        assert!(!row.reapproval_triggers.is_empty(), "{row:?}");
    }
}

#[test]
fn unbounded_exception_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].bounded_by_expiry = false;
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::ExceptionNotBounded { .. }
    )));
}

#[test]
fn authority_widening_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].widens_authority = true;
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::ExceptionWidensAuthority { .. }
    )));
}

#[test]
fn unpinned_scope_dimension_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].scope_binding.actor_ref = String::new();
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::ScopeDimensionUnbound {
            dimension: AuthorityDimension::Actor,
            ..
        }
    )));
}

#[test]
fn missing_reapproval_trigger_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].reapproval_triggers.clear();
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::ReapprovalTriggerMissing { .. }
    )));
}

#[test]
fn exception_ids_are_unique_and_sorted() {
    let packet = seeded_m5_exception_expiry_packet();
    let ids = packet.exception_ids();
    let mut sorted = ids.clone();
    sorted.sort();
    assert_eq!(ids, sorted);
    assert_eq!(ids.len(), packet.rows.len());
}

#[test]
fn request_sheets_show_exact_variance_not_generic_bypass() {
    let packet = seeded_m5_exception_expiry_packet();
    let sheets = packet.request_sheets();
    assert_eq!(sheets.len(), packet.rows.len());
    for sheet in &sheets {
        assert!(!sheet.exact_variance.trim().is_empty(), "{sheet:?}");
        assert!(!sheet.reason.trim().is_empty(), "{sheet:?}");
        assert!(!sheet.owner_or_approver_ref.trim().is_empty(), "{sheet:?}");
        assert!(!sheet.expires_at.trim().is_empty(), "{sheet:?}");
        // The scope summary names every pinned authority dimension.
        for dimension in AuthorityDimension::ALL {
            assert!(
                sheet.scope_summary.contains(dimension.as_str()),
                "scope summary missing {dimension:?}: {sheet:?}"
            );
        }
        assert!(
            sheet.bounded_by_expiry && !sheet.widens_authority,
            "{sheet:?}"
        );
    }
}

#[test]
fn approval_history_carries_lineage_and_consistent_state() {
    let packet = seeded_m5_exception_expiry_packet();
    let rows = packet.approval_history();
    assert_eq!(rows.len(), packet.rows.len());
    for row in &rows {
        assert!(!row.events.is_empty(), "{row:?}");
        assert_eq!(
            row.events.last().unwrap().event_class,
            row.current_state,
            "current state must match the latest event: {row:?}"
        );
    }
}

#[test]
fn expiry_banner_state_tracks_as_of() {
    let row = seeded_m5_exception_expiry_packet().rows.remove(0);
    // created 2026-06-13, review 2026-08-13, expiry 2026-09-13.
    assert_eq!(
        row.expiry_banner("2026-06-20T00:00:00Z").state,
        ExpiryState::Active
    );
    assert_eq!(
        row.expiry_banner("2026-08-20T00:00:00Z").state,
        ExpiryState::ExpiringSoon
    );
    assert_eq!(
        row.expiry_banner("2026-10-01T00:00:00Z").state,
        ExpiryState::Expired
    );
}

#[test]
fn self_revalidation_holds_for_every_seeded_row() {
    let packet = seeded_m5_exception_expiry_packet();
    for outcome in packet.self_revalidation() {
        assert_eq!(
            outcome.outcome,
            RevalidationOutcome::StillValid,
            "{outcome:?}"
        );
        assert!(outcome.drifted_dimensions.is_empty(), "{outcome:?}");
        assert!(!outcome.must_reauthorize, "{outcome:?}");
        assert!(!outcome.widens_authority, "{outcome:?}");
    }
}

#[test]
fn actor_drift_forces_re_review_without_widening() {
    let row = &seeded_m5_exception_expiry_packet().rows[0];
    let mut observed = row.observed_at("2026-06-20T00:00:00Z");
    observed.actor_ref = "actor:someone-else".to_owned();
    let outcome = row.revalidate(&observed);
    assert_eq!(outcome.outcome, RevalidationOutcome::MustReReview);
    assert!(outcome.must_reauthorize);
    assert!(!outcome.widens_authority);
    assert_eq!(outcome.drifted_dimensions, vec![AuthorityDimension::Actor]);
}

#[test]
fn policy_epoch_and_environment_drift_are_each_detected() {
    let row = &seeded_m5_exception_expiry_packet().rows[0];
    let mut observed = row.observed_at("2026-06-20T00:00:00Z");
    observed.policy_epoch = "policy:m5-records:v2".to_owned();
    observed.environment_ref = "env:other".to_owned();
    let outcome = row.revalidate(&observed);
    assert_eq!(outcome.outcome, RevalidationOutcome::MustReReview);
    assert!(outcome
        .drifted_dimensions
        .contains(&AuthorityDimension::PolicyEpoch));
    assert!(outcome
        .drifted_dimensions
        .contains(&AuthorityDimension::Environment));
}

#[test]
fn lapsed_expiry_forces_re_review_even_without_drift() {
    let row = &seeded_m5_exception_expiry_packet().rows[0];
    // Reuse after the 2026-09-13 expiry with the exact pinned scope.
    let observed = row.observed_at("2026-10-01T00:00:00Z");
    let outcome = row.revalidate(&observed);
    assert_eq!(outcome.outcome, RevalidationOutcome::MustReReview);
    assert!(outcome.expired);
    assert!(outcome.drifted_dimensions.is_empty());
    assert!(outcome.must_reauthorize);
}

#[test]
fn missing_approval_history_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].approval_history.clear();
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::ApprovalHistoryMissing { .. }
    )));
}

#[test]
fn inconsistent_current_state_is_rejected() {
    let mut packet = seeded_m5_exception_expiry_packet();
    packet.rows[0].current_state = ApprovalEventClass::Revoked;
    assert!(packet.validate().iter().any(|violation| matches!(
        violation,
        M5ExceptionExpiryViolation::CurrentStateInconsistent { .. }
    )));
}
