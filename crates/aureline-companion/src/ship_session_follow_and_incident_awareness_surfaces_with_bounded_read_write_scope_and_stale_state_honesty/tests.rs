use super::*;

const PACKET_ID: &str = "companion-scope-surface:stable:0001";
const PACKET_LABEL: &str = "Companion Session-Follow and Incident-Awareness Surfaces";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> CompanionScopeProofFreshness {
    CompanionScopeProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> CompanionScopeSurfacePacket {
    canonical_companion_scope_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn canonical_surface_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_surface_covers_every_surface() {
    let packet = packet();
    assert_eq!(
        packet.surface_qualifications.len(),
        CompanionScopeSurface::ALL.len()
    );
    for surface in CompanionScopeSurface::ALL {
        let row = packet
            .surface_qualifications
            .iter()
            .find(|row| row.surface == surface)
            .expect("surface present");
        assert_eq!(row.matrix_lane_ref, surface.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, surface.bounded_scope());
    }
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.session_follow.is_empty());
    assert!(!packet.incident_awareness.is_empty());
    assert!(!packet.bounded_light_edit.is_empty());
}

#[test]
fn read_only_surfaces_never_write() {
    let packet = packet();
    assert!(packet
        .session_follow
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .incident_awareness
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn light_edit_is_bounded_and_host_approved() {
    let packet = packet();
    for item in &packet.bounded_light_edit {
        assert_eq!(
            item.read_write_scope,
            CompanionReadWriteScope::BoundedWriteRelayedToHost
        );
        assert!(item.requires_host_approval);
        assert!(!item.write_bound_summary.trim().is_empty());
    }
}

#[test]
fn stale_items_are_honestly_labeled() {
    let packet = packet();
    assert!(packet.stale_state_honestly_labeled());
    // The canonical corpus carries at least one stale item, and it is labeled.
    let stale = packet
        .incident_awareness
        .iter()
        .find(|item| item.freshness == CompanionFreshnessState::Stale)
        .expect("a stale incident item is present");
    assert!(stale.stale_label_shown);
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surface_qualifications
        .retain(|row| row.surface != CompanionScopeSurface::BoundedLightEdit);
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::RequiredSurfaceMissing));
}

#[test]
fn surface_lane_mismatch_fails() {
    let mut packet = packet();
    packet.surface_qualifications[0].matrix_lane_ref = "managed_sync".to_owned();
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::SurfaceLaneMismatch));
}

#[test]
fn surface_scope_mismatch_fails() {
    let mut packet = packet();
    // Force the read-only session-follow surface to claim a write scope.
    packet.surface_qualifications[0].read_write_scope =
        CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::SurfaceScopeMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.session_follow[0].read_write_scope = CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::ReadOnlyScopeViolated));
}

#[test]
fn light_edit_without_host_approval_fails() {
    let mut packet = packet();
    packet.bounded_light_edit[0].requires_host_approval = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::LightEditUnbounded));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.session_follow[0].freshness = CompanionFreshnessState::Stale;
    packet.session_follow[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.incident_awareness[0].handoff.deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::HandoffRefMissing));
}

#[test]
fn empty_surface_content_fails() {
    let mut packet = packet();
    packet.bounded_light_edit.clear();
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::SurfaceContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet.scope_contract.no_unbounded_companion_write = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::ScopeContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::StaleStateHonestyIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet
        .locality_disclosure
        .requires_provider_or_admin_continuity
        .clear();
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.no_unbounded_companion_write = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_surfaces = false;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CompanionScopeViolation::ProofFreshnessIncomplete));
}

#[test]
fn degradation_on_relay_unavailable_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_companion_scope_degradation(&CompanionScopeObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::RelayUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::FreshnessDowngradedToStale));
    // Every item that was live/cached is now stale and labeled.
    assert!(packet
        .session_follow
        .iter()
        .all(|item| item.freshness == CompanionFreshnessState::Stale && item.stale_label_shown));
    // The stable incident-awareness surface narrows to beta and GA to staged rollout.
    let incident = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::IncidentAwareness)
        .expect("incident surface present");
    assert_eq!(incident.qualification, M5CompanionQualificationClass::Beta);
    assert_eq!(
        incident.rollout_stage,
        M5CompanionRolloutStage::StagedRollout
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.apply_companion_scope_degradation(&CompanionScopeObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        incident_attribution_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::HandoffTargetUnresolved));
    // No handoff that requires an active host still claims exact resolution.
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    // Incident-workspace handoffs are host-independent and stay exact.
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    // The bounded light-edit surface narrows because a write can no longer relay.
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::BoundedLightEdit)
        .expect("light-edit surface present");
    assert_eq!(
        edit.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_incident_attribution_loss_unattributes_and_narrows() {
    let mut packet = packet();
    packet.apply_companion_scope_degradation(&CompanionScopeObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        incident_attribution_intact: false,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::IncidentAttributionLost));
    // Every incident item narrows to unattributed rather than claiming provenance.
    assert!(packet
        .incident_awareness
        .iter()
        .all(|item| item.attribution == IncidentAttributionState::Unattributed));
    // Only the incident-awareness surface narrows; session-follow is untouched.
    let incident = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::IncidentAwareness)
        .expect("incident surface present");
    assert_eq!(incident.qualification, M5CompanionQualificationClass::Beta);
    let follow = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::SessionFollow)
        .expect("session-follow surface present");
    assert_eq!(follow.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_trust_narrowing_only_narrows_light_edit() {
    let mut packet = packet();
    packet.apply_companion_scope_degradation(&CompanionScopeObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: false,
        incident_attribution_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionScopeDegradedReason::TrustNarrowed));
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::BoundedLightEdit)
        .expect("light-edit surface present");
    assert_eq!(
        edit.qualification,
        M5CompanionQualificationClass::Experimental
    );
    // Incident-awareness stays stable: trust only narrows the write surface.
    let incident = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::IncidentAwareness)
        .expect("incident surface present");
    assert_eq!(
        incident.qualification,
        M5CompanionQualificationClass::Stable
    );
}

#[test]
fn publishable_surfaces_excludes_withheld() {
    let mut packet = packet();
    let total = packet.surface_qualifications.len();
    assert_eq!(packet.publishable_surfaces().count(), total);
    // Drive the preview light-edit surface down to held/withheld via repeated narrowing.
    for _ in 0..4 {
        packet.apply_companion_scope_degradation(&CompanionScopeObservation {
            relay_available: true,
            proof_fresh: true,
            host_session_active: true,
            trust_intact: false,
            incident_attribution_intact: true,
            upstream_matrix_narrowed: false,
        });
    }
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionScopeSurface::BoundedLightEdit)
        .expect("light-edit surface present");
    assert_eq!(edit.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_surfaces().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&CompanionScopeViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_companion_scope_surface_export()
        .expect("checked companion scope export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_companion_scope_surface_export()
        .expect("checked companion scope export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}
