//! Inline unit coverage for the browser/provider-console handoff packet.

use super::*;

fn stable_packet() -> BrowserHandoffPacket {
    BrowserHandoffPacket::materialize(seeded_stable_browser_handoff_input())
}

#[test]
fn seeded_packet_is_clean_stable() {
    let packet = stable_packet();
    assert_eq!(packet.record_kind, BROWSER_HANDOFF_OBJECTS_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        BROWSER_HANDOFF_OBJECTS_SCHEMA_VERSION
    );
    assert_eq!(packet.promotion_state, BrowserHandoffPromotionState::Stable);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet.is_clean_stable());
    assert!(packet.is_stable());
}

#[test]
fn seeded_packet_routes_every_required_exit() {
    let packet = stable_packet();
    let covered = packet.covered_exits();
    for exit in HandoffSourceSurface::REQUIRED_EXITS {
        assert!(
            covered.contains(&exit),
            "missing handoff for required exit {}",
            exit.as_str()
        );
    }
}

#[test]
fn seeded_packet_reconstructs_on_help_support_history() {
    let packet = stable_packet();
    for surface in BrowserHandoffConsumerSurface::REQUIRED_RECONSTRUCTION {
        assert!(
            packet.has_projection_for(surface),
            "missing reconstruction projection for {}",
            surface.as_str()
        );
    }
}

#[test]
fn destination_class_tokens_are_pinned() {
    let expected = [
        "docs_or_portal_web",
        "code_host_web",
        "issue_tracker_web",
        "package_registry_web",
        "ai_provider_web",
        "managed_admin_web",
        "external_generic_web",
    ];
    let observed: Vec<&str> = HandoffDestinationClass::ALL
        .iter()
        .map(|destination| destination.as_str())
        .collect();
    assert_eq!(observed, expected);
}

#[test]
fn no_seeded_handoff_leaks_raw_workspace_context() {
    let packet = stable_packet();
    for handoff in &packet.handoffs {
        assert!(
            !handoff.leaks_hidden_context(),
            "handoff {} must not exfiltrate raw workspace context",
            handoff.handoff_id
        );
        assert!(handoff.privacy_consequence_consistent());
    }
}

#[test]
fn ordinary_navigation_handoff_shares_only_resolved_ref() {
    let packet = stable_packet();
    let ordinary = packet
        .handoffs
        .iter()
        .find(|handoff| handoff.ordinary_navigation)
        .expect("seed carries an ordinary-navigation handoff");
    assert!(!ordinary.ordinary_navigation_overshares());
    assert!(ordinary.shared_context.shares_resolved_destination_ref);
    assert!(!ordinary.shared_context.shares_user_query_terms);
}

#[test]
fn support_and_history_reconstruct_every_handoff() {
    let packet = stable_packet();
    for surface in BrowserHandoffConsumerSurface::FULL_COVERAGE {
        let projection = packet
            .consumer_projections
            .iter()
            .find(|projection| projection.surface == surface)
            .unwrap_or_else(|| panic!("seed carries a {} projection", surface.as_str()));
        for handoff in &packet.handoffs {
            assert!(
                projection.handoff_id_refs.contains(&handoff.handoff_id),
                "{} drops handoff {}",
                surface.as_str(),
                handoff.handoff_id
            );
        }
    }
}

#[test]
fn hidden_context_share_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::DocsBrowser)
    {
        handoff.shared_context.shares_raw_code_selection = true;
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == BrowserHandoffValidationKind::HiddenContextShareDetected));
}

#[test]
fn ordinary_navigation_sharing_query_terms_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::DocsBrowser)
    {
        handoff.shared_context.shares_user_query_terms = true;
        handoff.privacy_consequence =
            DocsContractBrowserHandoffPrivacyConsequence::QueryTermsDisclosed;
        handoff.user_initiated = true;
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == BrowserHandoffValidationKind::OrdinaryNavigationSharesContext));
}

#[test]
fn raw_browser_open_bypass_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.routed_through_handoff_review = false;
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BrowserHandoffValidationKind::RawBrowserOpenBypass));
}

#[test]
fn missing_return_anchor_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input.handoffs.first_mut() {
        handoff.return_anchor.anchor_ref.clear();
        handoff.return_anchor.label.clear();
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BrowserHandoffValidationKind::ReturnAnchorMissing));
}

#[test]
fn exit_coverage_missing_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    input
        .handoffs
        .retain(|handoff| handoff.source_surface != HandoffSourceSurface::HelpAbout);
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind == BrowserHandoffValidationKind::ExitCoverageMissing));
}

#[test]
fn history_dropping_a_handoff_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(projection) = input
        .consumer_projections
        .iter_mut()
        .find(|projection| projection.surface == BrowserHandoffConsumerSurface::DocsHistory)
    {
        projection
            .handoff_id_refs
            .retain(|handoff_ref| handoff_ref != "handoff:docs_browser:tokio-spawn-anchor");
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == BrowserHandoffValidationKind::HistoryReconstructionDropsHandoff));
}

#[test]
fn blocked_handoff_presented_available_blocks() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.policy_posture = HandoffPolicyPosture::BlockedByPolicy;
        handoff.offered_as_actionable = true;
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::BlocksStable
    );
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == BrowserHandoffValidationKind::BlockedHandoffPresentedAvailable));
}

#[test]
fn honestly_blocked_handoff_narrows_without_blocking() {
    let mut input = seeded_stable_browser_handoff_input();
    if let Some(handoff) = input
        .handoffs
        .iter_mut()
        .find(|handoff| handoff.source_surface == HandoffSourceSurface::ProviderConsolePivot)
    {
        handoff.policy_posture = HandoffPolicyPosture::BlockedByPolicy;
        handoff.offered_as_actionable = false;
    }
    let packet = BrowserHandoffPacket::materialize(input);
    assert_eq!(
        packet.promotion_state,
        BrowserHandoffPromotionState::NarrowedBelowStable
    );
    assert!(packet.is_stable(), "narrowing must not block");
    assert!(packet
        .validation_findings
        .iter()
        .any(|finding| finding.finding_kind
            == BrowserHandoffValidationKind::HandoffUnavailableNarrowed));
}

#[test]
fn promotion_state_mismatch_is_detected() {
    let mut packet = stable_packet();
    packet.promotion_state = BrowserHandoffPromotionState::BlocksStable;
    assert!(packet.validate().iter().any(
        |finding| finding.finding_kind == BrowserHandoffValidationKind::PromotionStateMismatch
    ));
}

#[test]
fn support_export_round_trips_and_is_export_safe() {
    let packet = stable_packet();
    let export = packet.support_export("export:test", "2026-06-26T00:00:00Z");
    assert!(export.is_export_safe());
    let json = serde_json::to_string(&export).expect("export serializes");
    let parsed: BrowserHandoffSupportExport =
        serde_json::from_str(&json).expect("export round-trips");
    assert_eq!(parsed, export);
}

#[test]
fn checked_in_packet_validates() {
    let packet = current_stable_browser_handoff_packet().expect("seeded packet certifies stable");
    assert!(packet.validate().is_empty());
}
