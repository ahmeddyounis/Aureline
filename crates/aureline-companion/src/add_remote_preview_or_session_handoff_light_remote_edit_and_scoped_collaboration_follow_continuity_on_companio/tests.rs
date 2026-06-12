use super::*;

const PACKET_ID: &str = "companion-continuity-surface:stable:0001";
const PACKET_LABEL: &str =
    "Companion Remote-Preview, Session-Handoff, Light-Remote-Edit, and Collaboration-Follow Continuity";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> CompanionContinuityProofFreshness {
    CompanionContinuityProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> CompanionContinuitySurfacePacket {
    canonical_companion_continuity_surface(
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
        CompanionContinuitySurface::ALL.len()
    );
    for surface in CompanionContinuitySurface::ALL {
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
    assert!(!packet.remote_preview.is_empty());
    assert!(!packet.light_remote_edit.is_empty());
    assert!(!packet.collaboration_follow.is_empty());
}

#[test]
fn canonical_surface_projects_m5_secret_boundary_state() {
    let packet = packet();
    let states = packet.secret_boundary_states();
    assert_eq!(states.len(), 1);
    assert_eq!(
        states[0].matrix_row_id,
        "m5.secret.companion.session_handoff"
    );
    assert_eq!(
        states[0].consumer_identity_receipt.consumer_identity,
        SecretBoundaryConsumerIdentityClass::CompanionHandoff
    );
    assert_eq!(
        states[0].projection_mode_audit.projection_mode,
        SecretBoundaryProjectionMode::BrowserHandoff
    );
    assert!(!states[0].export_safety_banner.raw_secret_values_included);
}

#[test]
fn read_only_surfaces_never_write() {
    let packet = packet();
    assert!(packet
        .remote_preview
        .iter()
        .all(|item| item.read_write_scope == CompanionContinuityScope::ReadOnly));
    assert!(packet
        .collaboration_follow
        .iter()
        .all(|item| item.read_write_scope == CompanionContinuityScope::ReadOnly));
}

#[test]
fn light_remote_edit_is_bounded_and_host_approved() {
    let packet = packet();
    for item in &packet.light_remote_edit {
        assert_eq!(
            item.read_write_scope,
            CompanionContinuityScope::BoundedWriteRelayedToHost
        );
        assert!(item.requires_host_approval);
        assert!(!item.write_bound_summary.trim().is_empty());
    }
}

#[test]
fn collaboration_follow_is_scoped() {
    let packet = packet();
    assert!(packet
        .collaboration_follow
        .iter()
        .all(|item| item.scope_bounded));
}

#[test]
fn local_work_is_never_stranded() {
    let packet = packet();
    assert!(packet.local_work_never_stranded());
}

#[test]
fn stale_items_are_honestly_labeled() {
    let packet = packet();
    assert!(packet.stale_state_honestly_labeled());
    // The canonical corpus carries at least one stale item, and it is labeled.
    let stale = packet
        .collaboration_follow
        .iter()
        .find(|item| item.freshness == CompanionContinuityFreshness::Stale)
        .expect("a stale collaboration item is present");
    assert!(stale.stale_label_shown);
}

#[test]
fn missing_surface_fails_validation() {
    let mut packet = packet();
    packet
        .surface_qualifications
        .retain(|row| row.surface != CompanionContinuitySurface::LightRemoteEdit);
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::RequiredSurfaceMissing));
}

#[test]
fn surface_lane_mismatch_fails() {
    let mut packet = packet();
    packet.surface_qualifications[0].matrix_lane_ref = "managed_sync".to_owned();
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::SurfaceLaneMismatch));
}

#[test]
fn surface_scope_mismatch_fails() {
    let mut packet = packet();
    // Force the read-only remote-preview surface to claim a write scope.
    packet.surface_qualifications[0].read_write_scope =
        CompanionContinuityScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::SurfaceScopeMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.remote_preview[0].read_write_scope = CompanionContinuityScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::ReadOnlyScopeViolated));
}

#[test]
fn light_edit_without_host_approval_fails() {
    let mut packet = packet();
    packet.light_remote_edit[0].requires_host_approval = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::LightEditUnbounded));
}

#[test]
fn unscoped_collaboration_item_fails() {
    let mut packet = packet();
    packet.collaboration_follow[0].scope_bounded = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::CollaborationFollowUnscoped));
}

#[test]
fn stranded_local_work_fails() {
    let mut packet = packet();
    packet.remote_preview[0].local_work_preserved = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::LocalWorkStranded));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.remote_preview[0].freshness = CompanionContinuityFreshness::Stale;
    packet.remote_preview[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.collaboration_follow[0].handoff.deep_link_ref.clear();
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::HandoffRefMissing));
}

#[test]
fn empty_surface_content_fails() {
    let mut packet = packet();
    packet.light_remote_edit.clear();
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::SurfaceContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet.scope_contract.collaboration_follow_scoped = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::ScopeContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::StaleStateHonestyIncomplete));
}

#[test]
fn continuity_guarantee_incomplete_fails() {
    let mut packet = packet();
    packet.continuity_guarantee.handoff_never_strands_local_work = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::ContinuityGuaranteeIncomplete));
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
        .contains(&CompanionContinuityViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .preview_labs_label_for_unqualified_surfaces = false;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&CompanionContinuityViolation::ProofFreshnessIncomplete));
}

#[test]
fn degradation_on_relay_unavailable_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
        relay_available: false,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        collaboration_scope_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::RelayUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::FreshnessDowngradedToStale));
    // Every item that was live/cached is now stale and labeled.
    assert!(packet.remote_preview.iter().all(|item| item.freshness
        == CompanionContinuityFreshness::Stale
        && item.stale_label_shown));
    // The beta remote-preview surface narrows to preview and staged rollout to early access.
    let preview = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::RemotePreviewHandoff)
        .expect("remote-preview surface present");
    assert_eq!(
        preview.qualification,
        M5CompanionQualificationClass::Preview
    );
    assert_eq!(preview.rollout_stage, M5CompanionRolloutStage::EarlyAccess);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: false,
        trust_intact: true,
        collaboration_scope_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::HandoffTargetUnresolved));
    // No handoff that requires an active host still claims exact resolution.
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    // Collaboration-follow handoffs are host-independent and stay exact.
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    // No remote-preview item still claims an in-flight staged or resumed handoff:
    // each is either local-authoritative (nothing to strand) or handoff-unavailable.
    assert!(packet.remote_preview.iter().all(|item| matches!(
        item.handoff_continuity,
        SessionHandoffContinuity::LocalAuthoritative | SessionHandoffContinuity::HandoffUnavailable
    )));
    // The previously staged/resumed previews degraded to handoff-unavailable.
    assert!(packet
        .remote_preview
        .iter()
        .any(|item| item.handoff_continuity == SessionHandoffContinuity::HandoffUnavailable));
    // The bounded light-remote-edit surface narrows because a write can no longer relay.
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::LightRemoteEdit)
        .expect("light-remote-edit surface present");
    assert_eq!(
        edit.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_collaboration_scope_revoked_revokes_and_narrows() {
    let mut packet = packet();
    packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: true,
        collaboration_scope_intact: false,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::CollaborationScopeRevoked));
    // Every collaboration item narrows to scope-revoked rather than continuing the follow.
    assert!(packet
        .collaboration_follow
        .iter()
        .all(|item| item.follow_scope == CollaborationFollowScope::ScopeRevoked));
    // Only the collaboration-follow surface narrows; remote-preview is untouched.
    let collab = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::CollaborationFollow)
        .expect("collaboration-follow surface present");
    assert_eq!(collab.qualification, M5CompanionQualificationClass::Beta);
    let preview = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::RemotePreviewHandoff)
        .expect("remote-preview surface present");
    assert_eq!(preview.qualification, M5CompanionQualificationClass::Beta);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_trust_narrowing_only_narrows_light_edit() {
    let mut packet = packet();
    packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
        relay_available: true,
        proof_fresh: true,
        host_session_active: true,
        trust_intact: false,
        collaboration_scope_intact: true,
        upstream_matrix_narrowed: false,
    });
    assert!(packet
        .degraded_labels
        .contains(&CompanionContinuityDegradedReason::TrustNarrowed));
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::LightRemoteEdit)
        .expect("light-remote-edit surface present");
    assert_eq!(
        edit.qualification,
        M5CompanionQualificationClass::Experimental
    );
    // Collaboration-follow stays stable: trust only narrows the write surface.
    let collab = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::CollaborationFollow)
        .expect("collaboration-follow surface present");
    assert_eq!(collab.qualification, M5CompanionQualificationClass::Stable);
}

#[test]
fn publishable_surfaces_excludes_withheld() {
    let mut packet = packet();
    let total = packet.surface_qualifications.len();
    assert_eq!(packet.publishable_surfaces().count(), total);
    // Drive the preview light-remote-edit surface down to held/withheld via repeated narrowing.
    for _ in 0..4 {
        packet.apply_companion_continuity_degradation(&CompanionContinuityObservation {
            relay_available: true,
            proof_fresh: true,
            host_session_active: true,
            trust_intact: false,
            collaboration_scope_intact: true,
            upstream_matrix_narrowed: false,
        });
    }
    let edit = packet
        .surface_qualifications
        .iter()
        .find(|row| row.surface == CompanionContinuitySurface::LightRemoteEdit)
        .expect("light-remote-edit surface present");
    assert_eq!(edit.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_surfaces().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&CompanionContinuityViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_companion_continuity_surface_export()
        .expect("checked companion continuity export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked = current_stable_companion_continuity_surface_export()
        .expect("checked companion continuity export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}
