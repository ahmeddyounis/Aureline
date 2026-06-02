use super::*;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_five_rows() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert_eq!(
        page.rows.len(),
        5,
        "seeded page must have 5 rows (one per trust-store layer)"
    );
}

#[test]
fn seeded_page_covers_all_layers() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.covers_all_required_layers(),
        "seeded page must cover all five required trust-store layers"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_harden_os_keychain_trust_store_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must qualify stable in the seeded page",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_rows_exclude_raw_trust_material() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.all_rows_exclude_raw_trust_material(),
        "all seeded rows must exclude raw trust material"
    );
}

#[test]
fn seeded_page_all_required_local_continuity_declared() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.all_required_local_continuity_declared(),
        "all layers requiring local-continuity declaration must carry it"
    );
}

#[test]
fn seeded_page_all_change_events_have_repair_transaction_ids() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.all_change_events_have_repair_transaction_ids(),
        "all seeded change events must carry a non-empty repair_transaction_id"
    );
}

#[test]
fn seeded_page_all_change_events_have_attribution() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        page.all_change_events_have_attribution(),
        "all seeded change events must carry attribution"
    );
}

#[test]
fn seeded_page_has_five_change_events() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert_eq!(
        page.change_events.len(),
        5,
        "seeded page must have 5 change events (one per trust-store layer)"
    );
}

#[test]
fn seeded_page_all_change_events_exclude_raw_trust_material() {
    let page = seeded_harden_os_keychain_trust_store_page();
    for event in &page.change_events {
        assert!(
            event.raw_trust_material_excluded,
            "change event '{}' must exclude raw trust material",
            event.repair_transaction_id
        );
    }
}

#[test]
fn seeded_page_managed_authority_rows_have_attribution_ref() {
    let page = seeded_harden_os_keychain_trust_store_page();
    for row in &page.rows {
        if row.layer.may_carry_managed_authority() {
            assert!(
                !row.managed_attribution_ref.is_empty(),
                "managed-authority layer row '{}' (layer: {}) must carry a managed_attribution_ref",
                row.row_id,
                row.layer_token
            );
        }
    }
}

#[test]
fn seeded_page_local_continuity_layers_carry_declaration() {
    let page = seeded_harden_os_keychain_trust_store_page();
    for row in &page.rows {
        if row.layer.requires_local_continuity_declaration() {
            assert!(
                row.local_continuity_explicit,
                "row '{}' (layer: {}) requires local_continuity_explicit: true",
                row.row_id, row.layer_token
            );
        }
    }
}

// ---------------------------------------------------------------------------
// Audit guardrail drills
// ---------------------------------------------------------------------------

#[test]
fn drill_raw_trust_material_withdraws_packet() {
    let mut rows = seeded_harden_os_keychain_trust_store_page().rows;
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.layer == TrustStoreLayerClass::OsRoots)
    {
        row.raw_trust_material_excluded = false;
    }
    let page = HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:drill:raw-material",
        "Drill — raw trust material in row (withdrawn)",
        "2026-06-01T00:00:00Z",
        rows,
        seeded_harden_os_keychain_trust_store_page().change_events,
    );
    assert_eq!(
        page.summary.overall_qualification_token, "withdrawn",
        "packet must be withdrawn when a row exposes raw trust material"
    );
    assert!(
        page.defects.iter().any(|d| d.narrow_reason
            == HardenOsKeychainTrustStoreNarrowReasonClass::RawTrustMaterialExposed),
        "audit must emit RawTrustMaterialExposed defect"
    );
}

#[test]
fn drill_missing_layer_narrows_to_preview() {
    let rows: Vec<TrustStoreLayerRow> = seeded_harden_os_keychain_trust_store_page()
        .rows
        .into_iter()
        .filter(|r| {
            r.layer == TrustStoreLayerClass::OsRoots
                || r.layer == TrustStoreLayerClass::CustomCaBundle
        })
        .collect();
    let page = HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:drill:missing-layer",
        "Drill — required trust-store layer missing (preview)",
        "2026-06-01T00:00:00Z",
        rows,
        vec![],
    );
    assert_eq!(
        page.summary.overall_qualification_token, "preview",
        "packet must be narrowed to preview when a required layer is missing"
    );
    assert!(
        page.defects.iter().any(|d| d.narrow_reason
            == HardenOsKeychainTrustStoreNarrowReasonClass::MissingLayerCoverage),
        "audit must emit MissingLayerCoverage defect"
    );
}

#[test]
fn drill_missing_repair_action_on_blocking_event_narrows_to_beta() {
    let mut events = seeded_harden_os_keychain_trust_store_page().change_events;
    // Force a blocking session impact with NoneRequired repair action.
    if let Some(event) = events.first_mut() {
        event.session_impact = SessionImpactClass::RouteBlockedLocalContinuity;
        event.session_impact_token = SessionImpactClass::RouteBlockedLocalContinuity
            .as_str()
            .to_owned();
        event.repair_action = TrustStoreRepairActionClass::NoneRequired;
        event.repair_action_token = TrustStoreRepairActionClass::NoneRequired
            .as_str()
            .to_owned();
        event.affected_route_refs = vec!["tls_enterprise".to_owned()];
    }
    let page = HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:drill:missing-repair-action",
        "Drill — blocking event missing repair action (beta)",
        "2026-06-01T00:00:00Z",
        seeded_harden_os_keychain_trust_store_page().rows,
        events,
    );
    assert_eq!(
        page.summary.overall_qualification_token, "beta",
        "packet must be narrowed to beta when a blocking event lacks a repair action"
    );
    assert!(
        page.defects.iter().any(|d| d.narrow_reason
            == HardenOsKeychainTrustStoreNarrowReasonClass::ChangeEventMissingRepairAction),
        "audit must emit ChangeEventMissingRepairAction defect"
    );
}

#[test]
fn drill_missing_local_continuity_narrows_to_beta() {
    let mut rows = seeded_harden_os_keychain_trust_store_page().rows;
    // Clear local_continuity_explicit on a layer that requires it.
    if let Some(row) = rows
        .iter_mut()
        .find(|r| r.layer == TrustStoreLayerClass::CustomCaBundle)
    {
        row.local_continuity_explicit = false;
    }
    let page = HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:drill:missing-local-continuity",
        "Drill — missing local continuity declaration (beta)",
        "2026-06-01T00:00:00Z",
        rows,
        seeded_harden_os_keychain_trust_store_page().change_events,
    );
    assert_eq!(
        page.summary.overall_qualification_token, "beta",
        "packet must be narrowed to beta when a required local-continuity declaration is absent"
    );
    assert!(
        page.defects.iter().any(|d| d.narrow_reason
            == HardenOsKeychainTrustStoreNarrowReasonClass::LocalContinuityNotExplicit),
        "audit must emit LocalContinuityNotExplicit defect"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_wraps_seeded_page() {
    let page = seeded_harden_os_keychain_trust_store_page();
    let export = HardenOsKeychainTrustStoreSupportExport::from_page(
        "policy:harden-os-keychain-trust-store:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(
        export.raw_trust_material_excluded,
        "support export must always exclude raw trust material"
    );
    assert!(
        export.narrow_reasons_present.is_empty(),
        "support export of the seeded page must have no narrow reasons"
    );
}

// ---------------------------------------------------------------------------
// Validate function
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_harden_os_keychain_trust_store_page();
    assert!(
        validate_harden_os_keychain_trust_store_page(&page).is_ok(),
        "validate must return Ok for the seeded page"
    );
}

#[test]
fn validate_returns_err_for_page_with_defects() {
    let rows: Vec<TrustStoreLayerRow> = seeded_harden_os_keychain_trust_store_page()
        .rows
        .into_iter()
        .take(1)
        .collect();
    let page = HardenOsKeychainTrustStorePage::new(
        "policy:harden-os-keychain-trust-store:drill:validate-err",
        "Drill — validate returns Err",
        "2026-06-01T00:00:00Z",
        rows,
        vec![],
    );
    assert!(
        validate_harden_os_keychain_trust_store_page(&page).is_err(),
        "validate must return Err when the page has defects"
    );
}
