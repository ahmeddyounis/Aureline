use super::*;
use crate::networked_surface_transport_matrix::DenialReasonClass;

fn page() -> TransportExplainabilityPage {
    seeded_transport_explainability_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(
        page.defects.len(),
        0,
        "seeded page must be clean: {:?}",
        page.defects
    );
    assert!(validate_transport_explainability_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        ExplainQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    assert!(page.covers_all_required_surfaces());
    assert_eq!(page.posture_inspectors.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.explain_sheets.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.event_ledger.entries.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.summary.stable_row_count, REQUIRED_SURFACES.len());
}

#[test]
fn posture_inspector_projects_effective_proxy_trust_and_mirror_state() {
    let snapshot = seeded_transport_explainability_snapshot();
    for decision in &snapshot.decisions {
        let inspector = TransportPostureInspector::from_decision(decision);
        assert_eq!(
            inspector.effective_proxy_mode_token,
            decision.policy.proxy_resolution_source_token
        );
        assert_eq!(
            inspector.trust_source_token,
            decision.policy.trust_material_token
        );
        assert_eq!(
            inspector.mirror_offline_state_token,
            decision.policy.mirror_offline_behavior_token
        );
        assert!(inspector.is_fully_classified());
        assert!(!inspector.trust_proof_ref.is_empty());
        assert!(inspector.raw_private_material_excluded);
    }
}

#[test]
fn explain_sheets_render_at_field_catalog_parity() {
    let page = page();
    assert!(page.all_explain_sheets_at_parity());
    for sheet in &page.explain_sheets {
        assert!(sheet.fields_at_parity());
        let names: Vec<&str> = sheet.fields.iter().map(|f| f.name.as_str()).collect();
        assert_eq!(names, EXPLAIN_FIELD_NAMES.to_vec());
    }
}

#[test]
fn cli_and_support_views_quote_identical_codes_and_field_names() {
    let page = page();
    for sheet in &page.explain_sheets {
        let cli = sheet.render_cli_lines();
        let support = sheet.render_support_lines();
        assert_eq!(cli.len(), EXPLAIN_FIELD_NAMES.len());
        assert_eq!(support.len(), EXPLAIN_FIELD_NAMES.len());
        for ((name, value), (cli_line, support_line)) in sheet
            .explain_fields()
            .into_iter()
            .zip(cli.iter().zip(support.iter()))
        {
            // Same field name and same decision-code value across both views.
            assert_eq!(cli_line, &format!("{name}={value}"));
            assert_eq!(support_line, &format!("{name}: {value}"));
        }
    }
}

#[test]
fn ledger_filters_by_endpoint_class_origin_scope_and_disposition() {
    let page = page();
    let ledger = &page.event_ledger;

    // Filter by endpoint class: the AI gateway uses an inference-gateway endpoint.
    let gateways = ledger.filter_by_endpoint_class(EndpointClass::InferenceGateway);
    assert_eq!(gateways.len(), 1);
    assert_eq!(gateways[0].surface_token, SurfaceClass::AiGateway.as_str());

    // Filter by origin scope: managed-tenant surfaces.
    let managed = ledger.filter_by_origin_scope(OriginScopeClass::ManagedTenant);
    assert!(!managed.is_empty());
    assert!(managed
        .iter()
        .all(|e| e.origin_scope_token == OriginScopeClass::ManagedTenant.as_str()));

    // Filter by allow/deny disposition.
    let allowed = ledger.allowed_events();
    assert!(!allowed.is_empty());
    assert!(allowed.iter().all(|e| e.disposition.is_allow()));
    assert!(ledger.denied_events().is_empty());

    let served = ledger.filter_by_disposition(EventDispositionClass::ServedWithoutEgress);
    assert_eq!(served.len(), 1);
    assert_eq!(served[0].surface_token, SurfaceClass::RegistryRead.as_str());
}

#[test]
fn ledger_entries_never_carry_raw_material() {
    let page = page();
    for entry in &page.event_ledger.entries {
        assert!(entry.raw_private_material_excluded);
        // Closed-vocabulary tokens only: no URLs/hosts in the filterable fields.
        assert!(!entry.endpoint_class_token.contains("://"));
        assert!(!entry.origin_scope_token.contains('.'));
    }
}

#[test]
fn denial_explain_sheet_carries_typed_reason() {
    // Project a denied decision and confirm the explain sheet explains the
    // typed reason rather than a generic error.
    let mut snapshot = seeded_transport_explainability_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::RequestApiClient {
            d.outcome = TransportOutcomeClass::Denied;
            d.outcome_token = TransportOutcomeClass::Denied.as_str().to_owned();
            d.denial_reason = Some(DenialReasonClass::PolicyBlocked);
            d.denial_reason_token = DenialReasonClass::PolicyBlocked.as_str().to_owned();
        }
    }
    let page = TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:test:denied",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(page.denied_events_carry_reasons());
    let sheet = page
        .explain_sheets
        .iter()
        .find(|s| s.surface_token == SurfaceClass::RequestApiClient.as_str())
        .expect("request_api_client sheet present");
    assert_eq!(
        sheet.disposition_token,
        EventDispositionClass::Denied.as_str()
    );
    let denial = sheet.denial_explanation.as_ref().expect("denial explained");
    assert!(denial.contains(DenialReasonClass::PolicyBlocked.as_str()));
}

#[test]
fn missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_transport_explainability_snapshot();
    snapshot
        .decisions
        .retain(|d| d.surface != SurfaceClass::AiGateway);
    let page = TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:test:missing",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ExplainQualificationClass::Preview.as_str()
    );
    assert!(!page.covers_all_required_surfaces());
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ExplainNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn raw_material_withdraws_packet() {
    let mut snapshot = seeded_transport_explainability_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::DatabaseCloudConnector {
            d.raw_private_material_excluded = false;
        }
    }
    let page = TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:test:raw",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ExplainQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
}

#[test]
fn bypass_withdraws_packet() {
    let mut snapshot = seeded_transport_explainability_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::ProviderMutation {
            d.no_bypass = false;
        }
    }
    let page = TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:test:bypass",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ExplainQualificationClass::Withdrawn.as_str()
    );
}

#[test]
fn stale_proof_narrows_to_beta() {
    let mut snapshot = seeded_transport_explainability_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::DocsBrowserFetcher {
            d.policy.trust_proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
            d.policy.trust_proof_freshness_token =
                ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
        }
    }
    let page = TransportExplainabilityPage::from_decision_snapshot(
        "remote:networked_surface_transport_explainability:test:stale",
        "test",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ExplainQualificationClass::Beta.as_str()
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::DocsBrowserFetcher.as_str())
        .expect("docs row present");
    assert_eq!(
        row.qualification_token,
        ExplainQualificationClass::Beta.as_str()
    );
    assert_eq!(
        row.narrow_reason_token,
        ExplainNarrowReasonClass::ProofStaleBeyondWindow.as_str()
    );
}

#[test]
fn support_export_rolls_up_defects_without_raw_material() {
    let page = page();
    let export = TransportExplainabilitySupportExport::from_page(
        "remote:networked_surface_transport_explainability:support-export:test",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.shared_contract_ref,
        TRANSPORT_EXPLAINABILITY_SHARED_CONTRACT_REF
    );
}

#[test]
fn record_kinds_and_contract_refs_are_stable() {
    let page = page();
    assert_eq!(page.record_kind, TRANSPORT_EXPLAINABILITY_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, TRANSPORT_EXPLAINABILITY_SCHEMA_VERSION);
    assert_eq!(
        page.evidence_index_ref,
        TRANSPORT_EXPLAINABILITY_EVIDENCE_INDEX_REF
    );
    for inspector in &page.posture_inspectors {
        assert_eq!(
            inspector.record_kind,
            TRANSPORT_EXPLAINABILITY_POSTURE_RECORD_KIND
        );
    }
    for entry in &page.event_ledger.entries {
        assert_eq!(
            entry.record_kind,
            TRANSPORT_EXPLAINABILITY_EVENT_RECORD_KIND
        );
    }
    for sheet in &page.explain_sheets {
        assert_eq!(
            sheet.record_kind,
            TRANSPORT_EXPLAINABILITY_EXPLAIN_SHEET_RECORD_KIND
        );
    }
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let back: TransportExplainabilityPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, back);
}
