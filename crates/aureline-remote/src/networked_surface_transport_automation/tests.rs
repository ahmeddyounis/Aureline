use super::*;

fn page() -> TransportAutomationPage {
    seeded_transport_automation_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert!(
        page.defects.is_empty(),
        "seeded page must seed zero defects"
    );
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Stable.as_str()
    );
    assert!(validate_transport_automation_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    assert!(page.covers_all_required_surfaces());
    assert_eq!(
        page.activity_snapshot.records.len(),
        REQUIRED_SURFACES.len()
    );
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
}

#[test]
fn seeded_page_surfaces_every_canonical_denial_code() {
    let page = page();
    assert!(page.surfaces_complete_denial_vocabulary());
    let present = page.activity_snapshot.denial_codes_present();
    for code in REQUIRED_DENIAL_CODES {
        assert!(
            present.contains(code.as_str()),
            "denial code {} must appear in the seeded history",
            code.as_str()
        );
    }
}

#[test]
fn seeded_page_exposes_full_denial_vocabulary_in_summary() {
    let page = page();
    assert!(page.exposes_full_denial_vocabulary());
    assert_eq!(
        page.summary.denial_vocabulary.len(),
        REQUIRED_DENIAL_CODES.len()
    );
}

#[test]
fn seeded_page_satisfies_core_invariants() {
    let page = page();
    assert!(page.replay_queues_are_idempotent_only());
    assert!(page.denied_records_carry_codes());
    assert!(page.all_dispositions_consistent());
    assert!(page.all_records_at_field_parity());
    for record in &page.activity_snapshot.records {
        assert!(record.no_bypass);
        assert!(record.raw_private_material_excluded);
    }
}

#[test]
fn records_render_at_field_catalog_parity() {
    let page = page();
    for record in &page.activity_snapshot.records {
        let rendered = record.render_fields();
        assert_eq!(rendered.len(), ACTIVITY_FIELD_NAMES.len());
        for ((name, _), expected) in rendered.iter().zip(ACTIVITY_FIELD_NAMES.iter()) {
            assert_eq!(name, expected);
        }
    }
}

#[test]
fn cli_and_support_views_quote_identical_codes_and_field_names() {
    let page = page();
    for record in &page.activity_snapshot.records {
        let cli = record.render_cli_lines();
        let support = record.render_support_lines();
        assert_eq!(cli.len(), support.len());
        for (i, name) in ACTIVITY_FIELD_NAMES.iter().enumerate() {
            assert!(cli[i].starts_with(&format!("{name}=")));
            assert!(support[i].starts_with(&format!("{name}: ")));
            // Same value after the separator.
            let cli_val = cli[i].split_once('=').unwrap().1;
            let support_val = support[i].split_once(": ").unwrap().1;
            assert_eq!(cli_val, support_val);
        }
    }
}

#[test]
fn filter_selects_by_disposition() {
    let page = page();
    let denied = ActivityFilter::all()
        .with_disposition(ActivityDispositionClass::Denied)
        .apply(&page.activity_snapshot.records);
    assert_eq!(denied.len(), page.summary.denied_count);
    assert!(denied.iter().all(|r| r.disposition.is_denied()));
}

#[test]
fn filter_selects_by_denial_code() {
    let page = page();
    let ca =
        page.filter(&ActivityFilter::all().with_denial_code(TransportDenialClass::CaUntrusted));
    assert_eq!(ca.len(), 1);
    assert_eq!(ca[0].surface, SurfaceClass::DocsBrowserFetcher);
}

#[test]
fn filter_composes_dimensions() {
    let page = page();
    let f = ActivityFilter::all()
        .with_surface(SurfaceClass::RegistryRead)
        .with_route_choice(RouteChoiceClass::MirrorFirst);
    let hits = page.filter(&f);
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].denial_code, TransportDenialClass::MirrorUnreachable);

    // A non-matching composition yields nothing.
    let miss = page.filter(
        &ActivityFilter::all()
            .with_surface(SurfaceClass::RegistryRead)
            .with_route_choice(RouteChoiceClass::Direct),
    );
    assert!(miss.is_empty());
}

#[test]
fn filter_facets_cover_every_dimension() {
    let page = page();
    assert_eq!(page.filter_facets.len(), ACTIVITY_FILTER_DIMENSIONS.len());
    for facet in &page.filter_facets {
        assert!(!facet.values.is_empty());
        // Values are sorted and unique.
        let mut sorted = facet.values.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(sorted, facet.values);
    }
}

#[test]
fn route_origin_joins_aggregate_history() {
    let page = page();
    assert!(!page.route_origin_joins.is_empty());
    // The join totals must reconstruct the full activity count.
    let total: usize = page.route_origin_joins.iter().map(|j| j.total_count).sum();
    assert_eq!(total, page.activity_snapshot.records.len());
    for join in &page.route_origin_joins {
        assert_eq!(
            join.total_count,
            join.allowed_count + join.denied_count + join.deferred_count
        );
        // Denial codes present never include the `none` sentinel.
        assert!(!join
            .denial_codes_present
            .iter()
            .any(|c| c == TransportDenialClass::None.as_str()));
    }
}

#[test]
fn denial_vocabulary_maps_from_per_feature_classes() {
    // Matrix denial.
    assert_eq!(
        TransportDenialClass::from_matrix_denial(DenialReasonClass::ProxyUnreachable),
        TransportDenialClass::ProxyMisconfigured
    );
    assert_eq!(
        TransportDenialClass::from_matrix_denial(DenialReasonClass::OfflineNoFallback),
        TransportDenialClass::OfflineMode
    );
    // Proxy denial.
    assert_eq!(
        TransportDenialClass::from_proxy_denial(ProxyResolutionDenialClass::PacUnreachable),
        TransportDenialClass::ProxyMisconfigured
    );
    assert_eq!(
        TransportDenialClass::from_proxy_denial(
            ProxyResolutionDenialClass::MirrorOnlyNoProxyPermitted
        ),
        TransportDenialClass::EgressBlockedPolicy
    );
    // Trust denial.
    assert_eq!(
        TransportDenialClass::from_trust_denial(TrustDenialClass::HostProofChanged),
        TransportDenialClass::SshHostKeyUnknown
    );
    assert_eq!(
        TransportDenialClass::from_trust_denial(TrustDenialClass::CaBundleMissing),
        TransportDenialClass::CaUntrusted
    );
    assert_eq!(
        TransportDenialClass::from_trust_denial(TrustDenialClass::MirrorRootMismatch),
        TransportDenialClass::MirrorUnreachable
    );
}

#[test]
fn missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_transport_automation_snapshot();
    snapshot
        .records
        .retain(|r| r.surface != SurfaceClass::AiGateway);
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn raw_material_withdraws_packet() {
    let mut snapshot = seeded_transport_automation_snapshot();
    snapshot.records[0].raw_private_material_excluded = false;
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::RawPrivateMaterialExposed));
}

#[test]
fn bypass_withdraws_packet() {
    let mut snapshot = seeded_transport_automation_snapshot();
    snapshot.records[1].no_bypass = false;
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::BypassedSharedGovernance));
}

#[test]
fn non_idempotent_deferred_action_withdraws_packet() {
    let mut snapshot = seeded_transport_automation_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::CompanionHandoff {
            r.action_is_idempotent = false;
        }
    }
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::NonIdempotentReplayQueued));
}

#[test]
fn denied_without_code_narrows_row_to_beta() {
    let mut snapshot = seeded_transport_automation_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::ProviderMutation {
            r.denial_code = TransportDenialClass::None;
            r.denial_code_token = TransportDenialClass::None.as_str().to_owned();
        }
    }
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::DeniedWithoutCanonicalCode));
}

#[test]
fn allowed_with_code_narrows_row_to_beta() {
    let mut snapshot = seeded_transport_automation_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::AiGateway {
            r.denial_code = TransportDenialClass::CaUntrusted;
            r.denial_code_token = TransportDenialClass::CaUntrusted.as_str().to_owned();
        }
    }
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::DispositionDenialCodeMismatch));
}

#[test]
fn missing_canonical_code_narrows_to_beta() {
    let mut snapshot = seeded_transport_automation_snapshot();
    // Drop the only origin_scope_ambiguous denial by making it allowed.
    for r in snapshot.records.iter_mut() {
        if r.denial_code == TransportDenialClass::OriginScopeAmbiguous {
            r.disposition = ActivityDispositionClass::Allowed;
            r.disposition_token = ActivityDispositionClass::Allowed.as_str().to_owned();
            r.denial_code = TransportDenialClass::None;
            r.denial_code_token = TransportDenialClass::None.as_str().to_owned();
        }
    }
    let page = TransportAutomationPage::new("p", "l", "2026-06-01T00:00:00Z", snapshot);
    assert!(!page.surfaces_complete_denial_vocabulary());
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == AutomationNarrowReasonClass::DenialVocabularyIncomplete));
    assert_eq!(
        page.summary.overall_qualification_token,
        AutomationQualificationClass::Beta.as_str()
    );
}

#[test]
fn support_export_rolls_up_defects_without_raw_material() {
    let page = page();
    let export = TransportAutomationSupportExport::from_page(
        "remote:transport_automation:support:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
}

#[test]
fn record_kinds_and_contract_refs_are_stable() {
    let page = page();
    assert_eq!(page.record_kind, TRANSPORT_AUTOMATION_PAGE_RECORD_KIND);
    assert_eq!(
        page.shared_contract_ref,
        TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF
    );
    assert_eq!(page.schema_version, TRANSPORT_AUTOMATION_SCHEMA_VERSION);
    assert_eq!(
        page.evidence_index_ref,
        TRANSPORT_AUTOMATION_EVIDENCE_INDEX_REF
    );
    for record in &page.activity_snapshot.records {
        assert_eq!(
            record.record_kind,
            TRANSPORT_AUTOMATION_ACTIVITY_RECORD_KIND
        );
        assert_eq!(
            record.shared_contract_ref,
            TRANSPORT_AUTOMATION_SHARED_CONTRACT_REF
        );
    }
    for row in &page.rows {
        assert_eq!(row.record_kind, TRANSPORT_AUTOMATION_ROW_RECORD_KIND);
    }
    for join in &page.route_origin_joins {
        assert_eq!(join.record_kind, TRANSPORT_AUTOMATION_JOIN_RECORD_KIND);
    }
}

#[test]
fn records_never_carry_raw_material() {
    let page = page();
    let json = serde_json::to_string(&page).unwrap();
    // The packet must never leak typical raw-secret shapes.
    for needle in ["https://", "http://", "Bearer ", "BEGIN ", "@", "://"] {
        assert!(
            !json.contains(needle),
            "serialized packet must not contain raw material marker {needle:?}"
        );
    }
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).unwrap();
    let back: TransportAutomationPage = serde_json::from_str(&json).unwrap();
    assert_eq!(page, back);
}
