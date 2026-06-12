//! Integration coverage for the provider route-resolution beta page.

use std::collections::BTreeSet;

use aureline_provider::{
    audit_route_resolution_beta_page, seeded_route_resolution_beta_page,
    validate_route_resolution_beta_page, AccountScopeBetaProfileClass, ActingIdentityClass,
    AuthorityTruthState, LaneClass, ProviderFallbackMode, RouteChoiceClass,
    RouteDegradedStateClass, RouteResolutionBetaDefectKind, RouteResolutionBetaSupportExport,
    ROUTE_RESOLUTION_BETA_PAGE_RECORD_KIND, ROUTE_RESOLUTION_BETA_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_route_resolution_beta_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: aureline_provider::RouteResolutionBetaPage =
        serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.record_kind, ROUTE_RESOLUTION_BETA_PAGE_RECORD_KIND);
    assert_eq!(parsed.schema_version, ROUTE_RESOLUTION_BETA_SCHEMA_VERSION);
    assert_eq!(parsed.rows.len(), page.rows.len());
    assert_eq!(
        parsed.browser_handoff_panels.len(),
        page.browser_handoff_panels.len()
    );
    assert_eq!(
        parsed.authority_truth_panels.len(),
        page.authority_truth_panels.len()
    );
}

#[test]
fn seeded_page_covers_all_required_profiles() {
    let page = seeded_route_resolution_beta_page();
    validate_route_resolution_beta_page(&page).expect("seeded page validates");

    let profiles: BTreeSet<&str> = page
        .summary
        .profiles_present
        .iter()
        .map(String::as_str)
        .collect();
    for required in AccountScopeBetaProfileClass::ALL {
        assert!(
            profiles.contains(required.as_str()),
            "missing profile coverage: {}",
            required.as_str()
        );
    }
}

#[test]
fn seeded_page_separates_managed_and_external_lanes() {
    let page = seeded_route_resolution_beta_page();
    let lanes: BTreeSet<&str> = page
        .summary
        .lane_classes_present
        .iter()
        .map(String::as_str)
        .collect();
    assert!(lanes.contains(LaneClass::ManagedProviderLane.as_str()));
    assert!(lanes.contains(LaneClass::ManagedMirrorLane.as_str()));
    assert!(lanes.contains(LaneClass::ExternalProviderLane.as_str()));
    assert!(lanes.contains(LaneClass::TunnelExposedExternalLane.as_str()));
    assert!(lanes.contains(LaneClass::OfflineSnapshotLane.as_str()));
}

#[test]
fn seeded_page_covers_all_acting_identity_classes() {
    let page = seeded_route_resolution_beta_page();
    let classes: BTreeSet<&str> = page
        .summary
        .acting_identity_classes_present
        .iter()
        .map(String::as_str)
        .collect();
    assert!(classes.contains(ActingIdentityClass::ConnectedAccount.as_str()));
    assert!(classes.contains(ActingIdentityClass::InstallationGrant.as_str()));
    assert!(classes.contains(ActingIdentityClass::DelegatedCredential.as_str()));
}

#[test]
fn validator_blocks_silent_authority_widening() {
    let mut page = seeded_route_resolution_beta_page();
    page.rows[0].silent_authority_widening_taken = true;
    let defects = audit_route_resolution_beta_page(
        &page.rows,
        &page.browser_handoff_panels,
        &page.authority_truth_panels,
    );
    assert!(
        defects.iter().any(|defect| defect.defect_kind
            == RouteResolutionBetaDefectKind::SilentAuthorityWideningTaken),
        "silent_authority_widening_taken must surface as a typed defect: {defects:#?}",
    );
}

#[test]
fn validator_blocks_green_claim_on_managed_lane_without_bundle() {
    let mut page = seeded_route_resolution_beta_page();
    // Force the managed-provider-lane row to be fresh+green but strip the
    // managed-policy bundle ref, then point an authority-truth panel at it
    // claiming green.
    let row_idx = page
        .rows
        .iter()
        .position(|row| row.lane_class == LaneClass::ManagedProviderLane)
        .expect("managed lane row");
    let row = &mut page.rows[row_idx];
    row.freshness.freshness_class = aureline_provider::FreshnessLabel::Fresh;
    row.freshness.freshness_class_token =
        aureline_provider::FreshnessLabel::Fresh.as_str().to_owned();
    row.freshness.degraded_reason = None;
    row.route_degraded_state = RouteDegradedStateClass::Green;
    row.route_degraded_state_token = RouteDegradedStateClass::Green.as_str().to_owned();
    row.owner.managed_policy_bundle_ref = None;
    row.grant.managed_policy_bundle_ref = None;
    let row_id = row.row_id.clone();

    // Point an authority-truth panel at this row and claim green.
    let panel_idx = page
        .authority_truth_panels
        .iter()
        .position(|p| p.bound_row_ref == row_id)
        .expect("authority-truth panel for managed lane row");
    let panel = &mut page.authority_truth_panels[panel_idx];
    panel.truth_state = AuthorityTruthState::GreenClaimHonest;
    panel.truth_state_token = AuthorityTruthState::GreenClaimHonest.as_str().to_owned();
    panel.green_claim_held = true;

    let defects = audit_route_resolution_beta_page(
        &page.rows,
        &page.browser_handoff_panels,
        &page.authority_truth_panels,
    );
    // Both `ManagedLaneWithoutManagedPolicyBundle` (on the row) and
    // `AuthorityTruthPanelGreenClaimWithoutManagedBundle` (on the panel)
    // must surface, proving green truth cannot survive without a managed
    // bundle ref.
    assert!(defects.iter().any(|defect| defect.defect_kind
        == RouteResolutionBetaDefectKind::ManagedLaneWithoutManagedPolicyBundle));
    assert!(defects.iter().any(|defect| defect.defect_kind
        == RouteResolutionBetaDefectKind::AuthorityTruthPanelGreenClaimWithoutManagedBundle));
}

#[test]
fn browser_handoff_panel_routes_match_bound_row() {
    let page = seeded_route_resolution_beta_page();
    for panel in &page.browser_handoff_panels {
        let row = page
            .rows
            .iter()
            .find(|row| row.row_id == panel.bound_row_ref)
            .expect("panel binds known row");
        let row_route = row.route.route_choice;
        let fallback_route = (row.fallback.fallback_mode == ProviderFallbackMode::OpenInProvider)
            .then_some(RouteChoiceClass::SystemBrowserHandoffRoute);
        assert!(
            panel.projected_route_choice == row_route
                || Some(panel.projected_route_choice) == fallback_route,
            "browser-handoff panel {} projects a route that mismatches its bound row {}",
            panel.panel_id,
            row.row_id,
        );
        assert_eq!(panel.projected_owner_class, row.owner.owner_class);
        assert_eq!(
            panel.projected_acting_identity_class,
            row.grant.acting_identity_class
        );
    }
}

#[test]
fn support_export_excludes_raw_material() {
    let page = seeded_route_resolution_beta_page();
    let export = RouteResolutionBetaSupportExport::from_page(
        "route-resolution-beta:support-export:test",
        "2026-05-16T12:00:00Z",
        page,
    );
    assert!(export.raw_tokens_excluded);
    assert!(export.fail_closed_invariant);

    // Round-trip through JSON to assert the export string never carries raw
    // URL hints or `Bearer` markers — the export is metadata-only.
    let json = serde_json::to_string(&export).expect("serialize export");
    assert!(!json.contains("https://"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("ssh://"));
}

#[test]
fn seeded_page_projects_m5_secret_boundary_state() {
    let page = seeded_route_resolution_beta_page();
    let states = page.secret_boundary_states();
    assert_eq!(states.len(), 1);
    assert_eq!(
        states[0].matrix_row_id,
        "m5.secret.provider_model.route_resolution"
    );
    assert!(states[0].delegated_credential_row.is_some());
    assert!(!states[0].export_safety_banner.raw_secret_values_included);
}
