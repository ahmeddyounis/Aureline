use super::*;

fn clean_card_input() -> M5MarketplaceAccountBoundaryCardResolutionInput {
    M5MarketplaceAccountBoundaryCardResolutionInput {
        card_id: "boundary-card:test".to_owned(),
        owner_class: WebviewOwnerClass::ProviderOwned,
        owner_origin_disclosed: true,
        account_scope: M5EmbeddedAccountScope::OrgWorkspace,
        account_scope_disclosed: true,
        current_profile: "org member profile".to_owned(),
        region_or_tenant: "eu-west workspace".to_owned(),
        network_state: M5MarketplaceNetworkState::Online,
        browser_fallback_kind: Some(BrowserHandoffKind::ProviderContentView),
        browser_fallback_available: true,
        conceals_identity_behind_generic_chrome: false,
        freshness: M5EmbeddedFreshnessState::LiveFresh,
        proof_fresh: true,
    }
}

fn clean_row_input() -> M5OpenInBrowserHandoffRowResolutionInput {
    M5OpenInBrowserHandoffRowResolutionInput {
        row_id: "handoff-row:test".to_owned(),
        handoff_kind: BrowserHandoffKind::ProviderContentView,
        handoff_reason: HandoffReasonClass::ViewProviderContent,
        object_ref: "object:listing-1042".to_owned(),
        object_label: "Marketplace listing #1042".to_owned(),
        object_identity_preserved: true,
        handoff_reason_stated: true,
        fallback_state: FallbackStateClass::LocalContinuityPreserved,
        local_continuity_explicit: true,
        browser_fallback_available: true,
        lands_on_generic_page: false,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_marketplace_handoff_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_MARKETPLACE_HANDOFF_CONTROLS_PACKET_ID);
}

#[test]
fn card_clean_names_owner_and_scope() {
    let resolved = resolve_marketplace_account_boundary_card(clean_card_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.owner_origin_disclosed);
    assert!(resolved.account_scope_disclosed);
    assert!(!resolved.conceals_identity_behind_generic_chrome);
    assert!(!resolved.hides_identity_region_or_ownership());
    assert_eq!(resolved.owner_origin, "provider_owned");
    assert_eq!(resolved.account_scope, "org_workspace");
    assert_eq!(resolved.network_state, "online");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveProviderOwned
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceHandoffNextAction::NoActionNeeded
    );
}

#[test]
fn card_owner_undisclosed_degrades_ac1() {
    let mut input = clean_card_input();
    input.owner_origin_disclosed = false;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_identity_region_or_ownership());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::OwnerOrOriginUnstated)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn card_untrusted_owner_degrades() {
    let mut input = clean_card_input();
    input.owner_class = WebviewOwnerClass::UnknownUntrusted;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::OwnerOrOriginUnstated)
    );
}

#[test]
fn card_generic_chrome_degrades_ac1() {
    let mut input = clean_card_input();
    input.conceals_identity_behind_generic_chrome = true;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_identity_region_or_ownership());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::GenericChromeConcealsIdentity)
    );
}

#[test]
fn card_account_scope_unstated_degrades() {
    let mut input = clean_card_input();
    input.account_scope = M5EmbeddedAccountScope::AccountScopeUnknown;
    input.account_scope_disclosed = false;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::AccountScopeUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceHandoffNextAction::ReviewAccountScope
    );
}

#[test]
fn card_missing_region_for_org_degrades() {
    let mut input = clean_card_input();
    input.region_or_tenant = "  ".to_owned();
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::ProfileOrRegionUnstated)
    );
}

#[test]
fn card_missing_profile_for_account_degrades() {
    let mut input = clean_card_input();
    input.account_scope = M5EmbeddedAccountScope::PersonalAccount;
    input.current_profile = String::new();
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::ProfileOrRegionUnstated)
    );
}

#[test]
fn card_local_no_account_needs_no_profile() {
    let mut input = clean_card_input();
    input.account_scope = M5EmbeddedAccountScope::NoAccountLocal;
    input.current_profile = String::new();
    input.region_or_tenant = String::new();
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::LiveProviderOwned
    );
}

#[test]
fn card_network_unknown_or_no_fallback_degrades() {
    let mut input = clean_card_input();
    input.network_state = M5MarketplaceNetworkState::NetworkStateUnknown;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::NetworkStateOrFallbackUnstated)
    );

    let mut input = clean_card_input();
    input.browser_fallback_available = false;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5MarketplaceCardDegradeReason::NetworkStateOrFallbackUnstated)
    );
}

#[test]
fn card_offline_network_reads_as_offline_snapshot() {
    let mut input = clean_card_input();
    input.account_scope = M5EmbeddedAccountScope::NoAccountLocal;
    input.current_profile = String::new();
    input.region_or_tenant = String::new();
    input.network_state = M5MarketplaceNetworkState::Offline;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::OfflineSnapshot
    );
}

#[test]
fn card_blocked_network_reads_as_provider_blocked() {
    let mut input = clean_card_input();
    input.network_state = M5MarketplaceNetworkState::CaptivePortalOrBlocked;
    let resolved = resolve_marketplace_account_boundary_card(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::ProviderBlocked
    );
}

#[test]
fn card_empty_id_and_forbidden_material_error() {
    let mut input = clean_card_input();
    input.card_id = "".to_owned();
    assert_eq!(
        resolve_marketplace_account_boundary_card(input).unwrap_err(),
        M5MarketplaceHandoffResolutionError::EmptyCardId
    );

    let mut input = clean_card_input();
    input.region_or_tenant = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_marketplace_account_boundary_card(input).unwrap_err(),
        M5MarketplaceHandoffResolutionError::ForbiddenMaterial
    );
}

#[test]
fn row_clean_preserves_identity_and_reason() {
    let resolved = resolve_open_in_browser_handoff_row(clean_row_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.object_identity_preserved);
    assert!(resolved.handoff_reason_stated);
    assert!(resolved.local_continuity_explicit);
    assert!(!resolved.lands_on_generic_page);
    assert!(!resolved.drops_identity_or_lands_generic());
    assert_eq!(resolved.handoff_reason, "view_provider_content");
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::BrowserHandoffOnly
    );
}

#[test]
fn row_object_identity_dropped_degrades_ac2() {
    let mut input = clean_row_input();
    input.object_ref = String::new();
    let resolved = resolve_open_in_browser_handoff_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.drops_identity_or_lands_generic());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OpenInBrowserRowDegradeReason::ObjectIdentityDropped)
    );
    assert_eq!(
        resolved.boundary_disposition,
        M5EmbeddedBoundaryDisposition::NotEvaluated
    );
}

#[test]
fn row_lands_on_generic_page_degrades_ac2() {
    let mut input = clean_row_input();
    input.lands_on_generic_page = true;
    let resolved = resolve_open_in_browser_handoff_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.drops_identity_or_lands_generic());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OpenInBrowserRowDegradeReason::LandsOnGenericPage)
    );
    assert_eq!(
        resolved.next_action,
        M5MarketplaceHandoffNextAction::OpenInBrowser
    );
}

#[test]
fn row_reason_unstated_degrades() {
    let mut input = clean_row_input();
    input.handoff_reason_stated = false;
    let resolved = resolve_open_in_browser_handoff_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OpenInBrowserRowDegradeReason::HandoffReasonUnstated)
    );
}

#[test]
fn row_continuity_unstated_degrades() {
    let mut input = clean_row_input();
    input.local_continuity_explicit = false;
    let resolved = resolve_open_in_browser_handoff_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OpenInBrowserRowDegradeReason::LocalContinuityUnstated)
    );
}

#[test]
fn row_no_fallback_degrades() {
    let mut input = clean_row_input();
    input.browser_fallback_available = false;
    let resolved = resolve_open_in_browser_handoff_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5OpenInBrowserRowDegradeReason::BrowserFallbackUnavailable)
    );
}

#[test]
fn row_empty_id_and_forbidden_material_error() {
    let mut input = clean_row_input();
    input.row_id = "   ".to_owned();
    assert_eq!(
        resolve_open_in_browser_handoff_row(input).unwrap_err(),
        M5MarketplaceHandoffResolutionError::EmptyRowId
    );

    let mut input = clean_row_input();
    input.object_ref = "object://leak".to_owned();
    assert_eq!(
        resolve_open_in_browser_handoff_row(input).unwrap_err(),
        M5MarketplaceHandoffResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_marketplace_handoff_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.vocabulary_set.network_states.pop();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_OPEN_IN_BROWSER_HANDOFF_ROW_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5MarketplaceHandoffAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5MarketplaceHandoffExportField::BoundaryDispositions);
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.controls_rows[0]
        .open_in_browser_handoff_row_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    // Force a clean card to also read as concealed by generic chrome — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.marketplace_account_boundary_card_examples[0].degrade_reason = None;
    row.marketplace_account_boundary_card_examples[0].conceals_identity_behind_generic_chrome =
        true;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_marketplace_handoff_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.masquerades_as_native_approval_chrome = true,
            1 => row.hides_owner_origin_or_handoff_in_menus_only = true,
            2 => row.renders_stale_or_blocked_as_fresh_first_party_truth = true,
            _ => row.embeds_high_risk_approval_without_native_step_up = true,
        }
        assert!(packet
            .validate()
            .contains(&M5MarketplaceHandoffControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_generic_chrome_example_removed() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    for row in &mut packet.controls_rows {
        row.marketplace_account_boundary_card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5MarketplaceCardDegradeReason::GenericChromeConcealsIdentity)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_owner_unstated_example_removed() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    for row in &mut packet.controls_rows {
        row.marketplace_account_boundary_card_examples.retain(|ex| {
            ex.degrade_reason != Some(M5MarketplaceCardDegradeReason::OwnerOrOriginUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_identity_dropped_example_removed() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    for row in &mut packet.controls_rows {
        row.open_in_browser_handoff_row_examples.retain(|ex| {
            ex.degrade_reason != Some(M5OpenInBrowserRowDegradeReason::ObjectIdentityDropped)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_generic_landing_example_removed() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    for row in &mut packet.controls_rows {
        row.open_in_browser_handoff_row_examples.retain(|ex| {
            ex.degrade_reason != Some(M5OpenInBrowserRowDegradeReason::LandsOnGenericPage)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet
        .governance_review
        .generic_chrome_never_conceals_identity = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet
        .consumer_projection
        .support_export_reads_single_boundary_source = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_marketplace_handoff_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MarketplaceHandoffControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_marketplace_handoff_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_marketplace_handoff_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_marketplace_handoff_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_marketplace_handoff_controls_export()
        .expect("checked M5 marketplace/handoff controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_MARKETPLACE_HANDOFF_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_marketplace_handoff_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::MarketplaceUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Beta);

    let preview = seeded_m5_marketplace_handoff_controls_account_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5EmbeddedConsumerSurface::AccountUi)
        .unwrap();
    assert_eq!(row.qualification, M5EmbeddedQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5MarketplaceHandoffControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls/marketplace_beta_narrowed.json"
    )))
    .expect("marketplace fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed()
    );

    let preview: M5MarketplaceHandoffControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls/account_preview_narrowed.json"
    )))
    .expect("account fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_marketplace_handoff_controls_account_preview_narrowed()
    );
}

/// One-shot generator for the checked proof bundle and narrowed fixtures. Run
/// with `GEN_MARKETPLACE_HANDOFF_CONTROL_ARTIFACTS=1 cargo test -p
/// aureline-shell marketplace_account_boundary::tests::generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_MARKETPLACE_HANDOFF_CONTROL_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_marketplace_handoff_controls();
    assert!(packet.validate().is_empty());
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let artifact_dir = manifest.join(
        "../../artifacts/release/m5-marketplace-account-boundary-open-in-browser-handoff-controls-proof",
    );
    fs::create_dir_all(&artifact_dir).expect("create marketplace-handoff artifact directory");
    fs::write(
        artifact_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write marketplace-handoff support export");
    fs::write(artifact_dir.join("matrix.csv"), packet.render_matrix_csv())
        .expect("write marketplace-handoff matrix");
    fs::write(
        artifact_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write marketplace-handoff summary");

    let fixture_dir = manifest
        .join("../../fixtures/ui/m5-marketplace-account-boundary-open-in-browser-handoff-controls");
    fs::create_dir_all(&fixture_dir).expect("create marketplace-handoff fixture directory");
    for (name, narrowed) in [
        (
            "marketplace_beta_narrowed.json",
            seeded_m5_marketplace_handoff_controls_marketplace_beta_narrowed(),
        ),
        (
            "account_preview_narrowed.json",
            seeded_m5_marketplace_handoff_controls_account_preview_narrowed(),
        ),
    ] {
        assert!(narrowed.validate().is_empty());
        fs::write(
            fixture_dir.join(name),
            format!(
                "{}\n",
                serde_json::to_string_pretty(&narrowed)
                    .expect("marketplace-handoff fixture serializes")
            ),
        )
        .expect("write marketplace-handoff fixture");
    }
}
