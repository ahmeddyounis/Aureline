use super::*;

fn destination(
    id: &str,
    class: DestinationClass,
    role: DestinationRole,
    account: AccountRequirement,
    data_exit: DataExitBoundary,
    prominence: SupportProminence,
    parity: LocalOnlyParity,
) -> AboutDestinationRecord {
    AboutDestinationRecord {
        about_destination_schema_version: ABOUT_DESTINATION_SCHEMA_VERSION,
        record_kind: ABOUT_DESTINATION_RECORD_KIND.to_owned(),
        destination_id: format!("about_destination:{id}"),
        destination_class: class,
        destination_role_class: role,
        route_state_class: RouteState::Current,
        account_requirement_class: account,
        data_exit_boundary_class: data_exit,
        support_prominence_class: prominence,
        local_only_parity_class: parity,
        headline_label: id.to_owned(),
        destination_summary: format!("{id} destination summary."),
        replacement_destination_ref: None,
        local_only_fallback_ref: None,
        source_surface_refs: vec![format!("surface.{id}")],
        build_context_exports: vec![],
        issue_template_refs: vec![],
        contract_doc_ref: ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn open_source_repo() -> AboutDestinationRecord {
    destination(
        "source.repository",
        DestinationClass::OfficialPublic,
        DestinationRole::SourceRepository,
        AccountRequirement::None,
        DataExitBoundary::ExternalPublicBrowse,
        SupportProminence::SourceFirst,
        LocalOnlyParity::AccountOptionalLocalParity,
    )
}

fn issue_tracker() -> AboutDestinationRecord {
    let mut row = destination(
        "issue.tracker",
        DestinationClass::OfficialPublic,
        DestinationRole::IssueTracker,
        AccountRequirement::OptionalForAccountFeatures,
        DataExitBoundary::MetadataSafeObjectRefs,
        SupportProminence::SupportFirst,
        LocalOnlyParity::AccountOptionalLocalParity,
    );
    row.build_context_exports.push(BuildContextExport {
        export_class: BuildContextExportClass::PublicIssueTemplateBlock,
        export_block_ref: "build_context_export:public_issue.v1".to_owned(),
        export_block_schema_version: 1,
        redacted_for_audience: BuildContextExportClass::PublicIssueTemplateBlock,
        raw_screenshots_excluded: true,
        raw_secrets_excluded: true,
        export_summary: "Issue template block carries build identity refs only.".to_owned(),
    });
    row.issue_template_refs
        .push("issue_template:docs_or_compatibility_public".to_owned());
    row
}

fn local_only_fallback() -> AboutDestinationRecord {
    destination(
        "local.fallback",
        DestinationClass::OfficialPublic,
        DestinationRole::LocalOnlyFallback,
        AccountRequirement::None,
        DataExitBoundary::NoPayloadLeavesProduct,
        SupportProminence::SourceFirst,
        LocalOnlyParity::AccountOptionalLocalParity,
    )
}

fn local_open_about_card() -> CapabilityBoundaryCardRecord {
    CapabilityBoundaryCardRecord {
        capability_boundary_card_schema_version: CAPABILITY_BOUNDARY_CARD_SCHEMA_VERSION,
        record_kind: CAPABILITY_BOUNDARY_CARD_RECORD_KIND.to_owned(),
        card_id: "capability_boundary_card:about_pane.local_open".to_owned(),
        surface_class: BoundaryCardSurface::AboutPane,
        surface_ref: "surface.about_pane".to_owned(),
        posture_class: SurfacePosture::LocalOpenAccountOptional,
        identity_requirement_class: IdentityRequirement::OptionalLocalAccount,
        network_requirement_class: NetworkRequirement::AccountFreeMetadataOnly,
        data_boundary_class: SurfaceDataBoundary::MetadataOnlyOutbound,
        rollback_path_class: RollbackPath::ContinueLocalOnly,
        continue_local_only_path_ref: Some("about_destination:local.fallback".to_owned()),
        rollback_or_downgrade_path_ref: Some("about_destination:local.fallback".to_owned()),
        upgrade_honesty_rule_class: UpgradeHonestyRule::LocalPathVisible,
        support_prominence_class: SupportProminence::SourceFirst,
        local_only_parity_class: LocalOnlyParity::AccountOptionalLocalParity,
        linked_destination_refs: vec![
            "about_destination:source.repository".to_owned(),
            "about_destination:issue.tracker".to_owned(),
            "about_destination:local.fallback".to_owned(),
        ],
        linked_about_destination_refs: vec![],
        headline_label: "About — local-open with account-optional parity".to_owned(),
        card_summary: "About pane reads as local-open with account-optional parity.".to_owned(),
        contract_doc_ref: ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

fn baseline_page() -> AboutAndBoundaryTruthPage {
    AboutAndBoundaryTruthPage {
        about_and_boundary_truth_page_schema_version: ABOUT_AND_BOUNDARY_TRUTH_PAGE_SCHEMA_VERSION,
        record_kind: ABOUT_AND_BOUNDARY_TRUTH_PAGE_RECORD_KIND.to_owned(),
        page_id: "about_and_boundary_page:baseline".to_owned(),
        page_summary: "Baseline about/source/community-handoff page.".to_owned(),
        destinations: vec![open_source_repo(), issue_tracker(), local_only_fallback()],
        capability_boundary_cards: vec![local_open_about_card()],
        contract_doc_ref: ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
    }
}

#[test]
fn baseline_page_validates() {
    let page = baseline_page();
    page.validate().expect("baseline page validates");
}

#[test]
fn dead_destination_must_cite_replacement() {
    let mut page = baseline_page();
    page.destinations[0].route_state_class = RouteState::Archived;
    page.destinations[0].replacement_destination_ref = None;
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::DeadDestinationMissingReplacement { .. }
    ));
}

#[test]
fn redirected_destination_with_replacement_resolves_on_page() {
    let mut page = baseline_page();
    page.destinations[0].route_state_class = RouteState::Redirected;
    page.destinations[0].replacement_destination_ref =
        Some("about_destination:source.repository.new".to_owned());
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::ReplacementRefMissingFromPage { .. }
    ));
}

#[test]
fn account_coercing_route_without_local_fallback_is_rejected() {
    let mut page = baseline_page();
    page.destinations[1].account_requirement_class = AccountRequirement::RequiredForWrite;
    page.destinations[1].local_only_fallback_ref = None;
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::AccountCoercingRouteMissingLocalFallback { .. }
    ));
}

#[test]
fn support_route_cannot_drop_below_upgrade() {
    let mut page = baseline_page();
    page.destinations[1].support_prominence_class = SupportProminence::BelowUpgrade;
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::SupportRouteDeprioritizedBelowUpgrade { .. }
    ));
}

#[test]
fn upgrade_route_cannot_outrank_support() {
    let mut page = baseline_page();
    page.destinations.push(AboutDestinationRecord {
        destination_id: "about_destination:upgrade.hosted".to_owned(),
        destination_class: DestinationClass::OfficialPublic,
        destination_role_class: DestinationRole::UpgradeOrHosted,
        account_requirement_class: AccountRequirement::RequiredForPremiumHosted,
        data_exit_boundary_class: DataExitBoundary::ExternalPublicBrowse,
        support_prominence_class: SupportProminence::SupportFirst,
        local_only_parity_class: LocalOnlyParity::AccountOptionalLocalParity,
        headline_label: "Upgrade or hosted".to_owned(),
        destination_summary: "Hosted plan with managed services.".to_owned(),
        replacement_destination_ref: None,
        local_only_fallback_ref: Some("about_destination:local.fallback".to_owned()),
        source_surface_refs: vec!["surface.upgrade".to_owned()],
        build_context_exports: vec![],
        issue_template_refs: vec![],
        contract_doc_ref: ABOUT_AND_BOUNDARY_CONTRACT_DOC_REF.to_owned(),
        notes: None,
        record_kind: ABOUT_DESTINATION_RECORD_KIND.to_owned(),
        about_destination_schema_version: ABOUT_DESTINATION_SCHEMA_VERSION,
        route_state_class: RouteState::Current,
    });
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::UpgradeRouteOutranksSupport { .. }
    ));
}

#[test]
fn premium_card_hiding_local_path_is_rejected() {
    let mut page = baseline_page();
    let mut card = local_open_about_card();
    card.card_id = "capability_boundary_card:upgrade.premium".to_owned();
    card.surface_class = BoundaryCardSurface::UpgradeOrHostedCta;
    card.posture_class = SurfacePosture::PremiumHosted;
    card.identity_requirement_class = IdentityRequirement::RequiredAccountForSubscribe;
    card.network_requirement_class = NetworkRequirement::AuthenticatedPremiumPlane;
    card.data_boundary_class = SurfaceDataBoundary::AuthenticatedPremiumOutbound;
    card.upgrade_honesty_rule_class = UpgradeHonestyRule::LocalPathHiddenViolation;
    card.support_prominence_class = SupportProminence::BelowUpgrade;
    card.local_only_parity_class = LocalOnlyParity::MixedLocalOptionalAccount;
    page.capability_boundary_cards.push(card);
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::PremiumOrManagedHidesLocalPath { .. }
    ));
}

#[test]
fn local_open_card_with_authenticated_outbound_is_rejected() {
    let mut card = local_open_about_card();
    card.data_boundary_class = SurfaceDataBoundary::AuthenticatedManagedOutbound;
    let err = card.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::LocalOpenSurfaceWithIncompatibleAxes { .. }
    ));
}

#[test]
fn handoff_route_requires_build_context_export() {
    let mut page = baseline_page();
    page.destinations[1].build_context_exports.clear();
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::HandoffRouteMissingBuildContextExport { .. }
    ));
}

#[test]
fn render_plaintext_is_stable() {
    let page = baseline_page();
    page.validate().expect("page validates");
    let block = page.render_plaintext();
    assert!(block.contains("about_destination:source.repository"));
    assert!(block.contains("local-only fallback") || block.contains("local.fallback"));
    assert!(block.contains("capability_boundary_card:about_pane.local_open"));
}

#[test]
fn duplicate_destination_id_is_rejected() {
    let mut page = baseline_page();
    page.destinations.push(open_source_repo());
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::DuplicateDestinationId { .. }
    ));
}

#[test]
fn card_links_must_resolve_on_page() {
    let mut page = baseline_page();
    page.capability_boundary_cards[0]
        .linked_destination_refs
        .push("about_destination:unknown.row".to_owned());
    let err = page.validate().unwrap_err();
    assert!(matches!(
        err,
        AboutAndBoundaryValidationError::CardLinkedDestinationMissingFromPage { .. }
    ));
}
