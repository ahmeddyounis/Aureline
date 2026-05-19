//! Fixture replay for the public about-destination and capability-boundary
//! card corpus published under
//! `fixtures/public/m3/about_and_boundary_truth/`.
//!
//! Positive cases MUST validate end-to-end; negative cases MUST fail
//! validation with a typed [`AboutAndBoundaryValidationError`].

use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::public_truth::{
    AboutAndBoundaryTruthPage, AboutAndBoundaryValidationError, BoundaryCardSurface,
    DestinationClass, DestinationRole, LocalOnlyParity, RouteState, SupportProminence,
    SurfacePosture, UpgradeHonestyRule,
};

fn fixture_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/public/m3/about_and_boundary_truth")
}

fn load_page(rel: &str) -> AboutAndBoundaryTruthPage {
    let path = fixture_root().join(rel);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn json_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for entry in fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("failed to read directory {}: {err}", dir.display()))
    {
        let entry = entry.expect("dir entry");
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            out.push(path);
        }
    }
    out.sort();
    out
}

#[test]
fn all_positive_fixtures_validate() {
    let dir = fixture_root().join("positive");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected positive fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let page: AboutAndBoundaryTruthPage = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        page.validate().unwrap_or_else(|err| {
            panic!(
                "positive fixture {} should validate but failed: {err}",
                path.display()
            )
        });
        let text = page.render_plaintext();
        assert!(
            text.contains(&page.page_id),
            "plaintext export must mention page id ({})",
            path.display()
        );
    }
}

#[test]
fn all_negative_fixtures_fail_validation() {
    let dir = fixture_root().join("negative");
    let files = json_files(&dir);
    assert!(
        !files.is_empty(),
        "expected negative fixtures under {}",
        dir.display()
    );
    for path in files {
        let body = fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
        let page: AboutAndBoundaryTruthPage = serde_json::from_str(&body)
            .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()));
        let result = page.validate();
        assert!(
            result.is_err(),
            "negative fixture {} should fail validation but passed",
            path.display()
        );
    }
}

#[test]
fn positive_account_optional_page_renders_expected_vocabulary() {
    let page = load_page("positive/account_optional_local_about_page.json");
    page.validate().expect("baseline positive validates");

    let support_routes = [
        DestinationRole::IssueTracker,
        DestinationRole::SupportIntake,
        DestinationRole::SecurityIntake,
        DestinationRole::SourceRepository,
        DestinationRole::ContributingGuide,
    ];
    for destination in &page.destinations {
        if support_routes.contains(&destination.destination_role_class) {
            assert!(
                destination
                    .support_prominence_class
                    .ranks_support_above_upgrade(),
                "support route {} must rank support above upgrade",
                destination.destination_id
            );
        }
    }

    let upgrade_routes = [
        DestinationRole::UpgradeOrHosted,
        DestinationRole::SponsorshipOrFunding,
    ];
    for destination in &page.destinations {
        if upgrade_routes.contains(&destination.destination_role_class) {
            assert!(
                matches!(
                    destination.support_prominence_class,
                    SupportProminence::ParityWithUpgrade | SupportProminence::BelowUpgrade
                ),
                "upgrade/sponsorship route {} must sit at or below upgrade",
                destination.destination_id
            );
        }
    }

    let fallback_count = page
        .destinations
        .iter()
        .filter(|d| d.destination_role_class == DestinationRole::LocalOnlyFallback)
        .count();
    assert!(
        fallback_count >= 1,
        "baseline positive page must publish a local-only fallback"
    );

    for destination in &page.destinations {
        if destination.account_requirement_class.coerces_account()
            && destination
                .local_only_parity_class
                .requires_local_fallback_when_account_coerces()
        {
            assert!(
                destination.local_only_fallback_ref.is_some(),
                "account-coercing destination {} must cite a local fallback",
                destination.destination_id
            );
        }
    }

    let class_counts = page
        .destinations
        .iter()
        .map(|d| d.destination_class)
        .collect::<Vec<_>>();
    assert!(class_counts.contains(&DestinationClass::OfficialPublic));
    assert!(class_counts.contains(&DestinationClass::OfficialPrivate));
    assert!(class_counts.contains(&DestinationClass::Community));

    for card in &page.capability_boundary_cards {
        if card.surface_class.is_upgrade_cta() {
            assert!(
                matches!(
                    card.support_prominence_class,
                    SupportProminence::ParityWithUpgrade | SupportProminence::BelowUpgrade
                ),
                "upgrade CTA surface {} cannot outrank support",
                card.card_id
            );
        }
    }
}

#[test]
fn premium_hosted_positive_keeps_local_path_visible() {
    let page = load_page("positive/premium_hosted_keeps_local_path_visible.json");
    page.validate().expect("premium hosted positive validates");
    let card = page
        .capability_boundary_cards
        .iter()
        .find(|c| c.surface_class == BoundaryCardSurface::UpgradeOrHostedCta)
        .expect("upgrade CTA card present");
    assert_eq!(card.posture_class, SurfacePosture::PremiumHosted);
    assert_eq!(
        card.upgrade_honesty_rule_class,
        UpgradeHonestyRule::LocalPathVisible
    );
    assert!(
        card.continue_local_only_path_ref.is_some(),
        "local_path_visible card must cite continue_local_only_path_ref"
    );
}

#[test]
fn archived_positive_carries_replacement_route() {
    let page = load_page("positive/archived_destination_with_replacement.json");
    page.validate().expect("archived positive validates");
    let archived = page
        .destinations
        .iter()
        .find(|d| d.route_state_class == RouteState::Archived)
        .expect("archived row exists");
    let replacement_ref = archived
        .replacement_destination_ref
        .as_deref()
        .expect("archived row must carry replacement ref");
    let resolved = page
        .destinations
        .iter()
        .any(|d| d.destination_id == replacement_ref);
    assert!(resolved, "replacement ref must resolve on the page");
}

#[test]
fn negative_archived_missing_replacement_is_typed_error() {
    let page = load_page("negative/archived_destination_missing_replacement.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::DeadDestinationMissingReplacement { .. }
    ));
}

#[test]
fn negative_account_required_write_missing_local_fallback_is_typed_error() {
    let page = load_page("negative/account_required_write_missing_local_fallback.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::AccountCoercingRouteMissingLocalFallback { .. }
    ));
}

#[test]
fn negative_premium_hosted_hides_local_path_is_typed_error() {
    let page = load_page("negative/premium_hosted_hides_local_path.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::PremiumOrManagedHidesLocalPath { .. }
    ));
}

#[test]
fn negative_support_route_below_upgrade_is_typed_error() {
    let page = load_page("negative/support_route_below_upgrade.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::SupportRouteDeprioritizedBelowUpgrade { .. }
    ));
}

#[test]
fn negative_third_party_vendor_bad_data_exit_is_typed_error() {
    let page = load_page("negative/third_party_vendor_bad_data_exit.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::ThirdPartyVendorWithUnsupportedDataExit { .. }
    ));
}

#[test]
fn negative_local_open_card_with_authenticated_outbound_is_typed_error() {
    let page = load_page("negative/local_open_card_with_authenticated_outbound.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::LocalOpenSurfaceWithIncompatibleAxes { .. }
    ));
}

#[test]
fn negative_card_links_unknown_destination_is_typed_error() {
    let page = load_page("negative/card_links_unknown_destination.json");
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::CardLinkedDestinationMissingFromPage { .. }
    ));
}

#[test]
fn parity_class_helpers_cover_all_destinations() {
    let page = load_page("positive/account_optional_local_about_page.json");
    page.validate().expect("baseline positive validates");
    let mut local_parity_seen = false;
    for destination in &page.destinations {
        match destination.local_only_parity_class {
            LocalOnlyParity::AccountOptionalLocalParity | LocalOnlyParity::LocalOnlyOnly => {
                local_parity_seen = true;
            }
            LocalOnlyParity::MixedLocalOptionalAccount => {}
            LocalOnlyParity::HostedOnlyNoLocalFallback => {
                panic!(
                    "baseline positive page must not publish hosted-only rows: {}",
                    destination.destination_id
                );
            }
        }
    }
    assert!(
        local_parity_seen,
        "baseline positive page must publish at least one row with account-optional or local-only parity"
    );
}
