use super::*;

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
