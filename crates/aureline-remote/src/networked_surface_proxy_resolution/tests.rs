use super::*;

fn page() -> ProxyResolutionGovernancePage {
    seeded_proxy_resolution_page()
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
    assert!(validate_proxy_resolution_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_stability_conditions() {
    let page = page();
    assert!(page.covers_all_required_surfaces());
    assert!(page.no_record_ships_private_proxy_stack());
    assert!(page.no_record_ships_direct_ca_override());
    assert!(page.no_record_allows_silent_direct_fallback());
    assert!(page.all_records_respect_precedence());
    assert!(page.denied_records_carry_reasons());
    assert!(page.egress_classes_have_policy_epoch_refs());
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    let covered = page.resolution_snapshot.covered_surface_tokens();
    assert_eq!(covered.len(), REQUIRED_SURFACES.len());
    for required in &REQUIRED_SURFACES {
        assert!(
            covered.contains(required.as_str()),
            "required surface '{}' must have a record",
            required.as_str()
        );
    }
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.summary.stable_row_count, REQUIRED_SURFACES.len());
}

#[test]
fn every_record_excludes_raw_material_and_uses_handle_only_candidates() {
    let snapshot = seeded_proxy_resolution_snapshot();
    for record in &snapshot.records {
        assert!(
            record.raw_material_excluded(),
            "record '{}' must exclude raw private material",
            record.record_id
        );
        for candidate in &record.candidates {
            assert!(
                candidate.candidate_handle.starts_with("proxy_source:"),
                "candidate for '{}' must be an opaque handle, got '{}'",
                record.record_id,
                candidate.candidate_handle
            );
            assert!(!candidate.is_private_stack);
        }
    }
}

#[test]
fn every_record_respects_precedence_and_is_fully_classified() {
    let snapshot = seeded_proxy_resolution_snapshot();
    for record in &snapshot.records {
        assert!(
            record.is_fully_classified(),
            "record '{}' must be fully classified",
            record.record_id
        );
        assert!(
            record.precedence_respected(),
            "record '{}' must respect precedence (selected '{}')",
            record.record_id,
            record.selected_tier_token
        );
        assert!(
            record.no_bypass(),
            "record '{}' must not bypass",
            record.record_id
        );
    }
}

#[test]
fn seeded_snapshot_exercises_every_in_ladder_tier_plus_mirror_and_denial() {
    let snapshot = seeded_proxy_resolution_snapshot();
    let selected: BTreeSet<&str> = snapshot
        .records
        .iter()
        .map(|r| r.selected_tier_token.as_str())
        .collect();
    for tier in [
        ProxyResolutionTierClass::PacScript,
        ProxyResolutionTierClass::ManualPinned,
        ProxyResolutionTierClass::EnvironmentProxy,
        ProxyResolutionTierClass::SystemProxy,
        ProxyResolutionTierClass::DirectNoProxy,
        ProxyResolutionTierClass::MirrorPinned,
    ] {
        assert!(
            selected.contains(tier.as_str()),
            "seeded snapshot must select tier '{}' at least once",
            tier.as_str()
        );
    }
    // A typed deny_proxy_resolution outcome must be present.
    assert!(snapshot.records.iter().any(|r| {
        r.outcome == ProxyResolutionOutcomeClass::DeniedProxyResolution
            && r.denial_reason == Some(ProxyResolutionDenialClass::ContradictoryProxyState)
    }));
}

#[test]
fn precedence_ranks_are_strictly_ordered_in_the_ladder() {
    assert!(
        ProxyResolutionTierClass::PacScript.precedence_rank()
            < ProxyResolutionTierClass::ManualPinned.precedence_rank()
    );
    assert!(
        ProxyResolutionTierClass::ManualPinned.precedence_rank()
            < ProxyResolutionTierClass::EnvironmentProxy.precedence_rank()
    );
    assert!(
        ProxyResolutionTierClass::EnvironmentProxy.precedence_rank()
            < ProxyResolutionTierClass::SystemProxy.precedence_rank()
    );
    assert!(
        ProxyResolutionTierClass::SystemProxy.precedence_rank()
            < ProxyResolutionTierClass::DirectNoProxy.precedence_rank()
    );
}

#[test]
fn drill_missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    snapshot
        .records
        .retain(|r| r.surface != SurfaceClass::AiGateway);
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:missing-surface",
        "Drill — required surface absent (preview)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.covers_all_required_surfaces());
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn drill_raw_material_withdraws_packet() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RequestApiClient {
            r.raw_private_material_excluded = false;
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:raw-material",
        "Drill — raw private material exposed (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
    assert!(page
        .rows
        .iter()
        .all(|r| r.qualification_token == ProxyQualificationClass::Withdrawn.as_str()));
}

#[test]
fn drill_private_proxy_stack_withdraws_packet() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::DocsBrowserFetcher {
            r.candidates.push(ProxyCandidate::private_stack(
                ProxyResolutionTierClass::ManualPinned,
                "proxy_source:docs_browser_fetcher:private:0001",
                "A bundled private proxy stack attempted to intercept the route.",
            ));
            // Recompute the derived guardrail flag.
            r.no_private_proxy_stack = !r.candidates.iter().any(|c| c.is_private_stack);
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:private-stack",
        "Drill — private proxy stack shipped (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::PrivateProxyStackShipped));
}

#[test]
fn drill_direct_ca_override_withdraws_packet() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::DatabaseCloudConnector {
            r.no_direct_ca_override = false;
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:ca-override",
        "Drill — direct CA override shipped (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::DirectCaOverrideShipped));
}

#[test]
fn drill_silent_direct_fallback_withdraws_packet() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RegistryRead {
            r.no_silent_direct_fallback = false;
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:silent-fallback",
        "Drill — silent direct-to-public fallback (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::SilentDirectFallbackResolved));
}

#[test]
fn drill_denied_without_reason_narrows_to_beta() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::SyncOffboarding {
            r.outcome = ProxyResolutionOutcomeClass::DeniedProxyResolution;
            r.outcome_token = ProxyResolutionOutcomeClass::DeniedProxyResolution
                .as_str()
                .to_owned();
            r.denial_reason = None;
            r.denial_reason_token = String::new();
            // A denial selects no tier.
            for c in r.candidates.iter_mut() {
                c.selected = false;
            }
            r.selected_tier = None;
            r.selected_tier_token = String::new();
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:denied-no-reason",
        "Drill — denied without reason (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::DenyReasonMissing));
}

#[test]
fn drill_precedence_not_respected_narrows_row_to_beta() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    // On request_api_client, select the lower-precedence system proxy while the
    // higher-precedence environment proxy is available — precedence is violated.
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::RequestApiClient {
            for c in r.candidates.iter_mut() {
                c.selected = c.tier == ProxyResolutionTierClass::SystemProxy;
            }
            r.selected_tier = Some(ProxyResolutionTierClass::SystemProxy);
            r.selected_tier_token = ProxyResolutionTierClass::SystemProxy.as_str().to_owned();
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:precedence",
        "Drill — precedence not respected (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Beta.as_str()
    );
    let row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::RequestApiClient.as_str())
        .expect("request row present");
    assert_eq!(
        row.narrow_reason_token,
        ProxyNarrowReasonClass::PrecedenceNotRespected.as_str()
    );
    assert!(!row.precedence_respected);
}

#[test]
fn drill_missing_policy_epoch_on_required_egress_narrows_to_beta() {
    let mut snapshot = seeded_proxy_resolution_snapshot();
    for r in snapshot.records.iter_mut() {
        if r.surface == SurfaceClass::AiGateway {
            r.policy_epoch_ref = None;
        }
    }
    let page = ProxyResolutionGovernancePage::new(
        "remote:networked_surface_proxy_resolution:drill:missing-epoch",
        "Drill — missing policy epoch (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == ProxyNarrowReasonClass::PolicyEpochRefMissing));
    assert_eq!(
        page.summary.overall_qualification_token,
        ProxyQualificationClass::Beta.as_str()
    );
}

#[test]
fn summary_rolls_up_tiers_and_outcomes() {
    let page = page();
    assert_eq!(
        page.summary.no_private_proxy_stack_count,
        REQUIRED_SURFACES.len()
    );
    assert_eq!(
        page.summary.precedence_respected_count,
        REQUIRED_SURFACES.len()
    );
    let denied = page
        .summary
        .outcome_counts
        .get(ProxyResolutionOutcomeClass::DeniedProxyResolution.as_str())
        .copied()
        .unwrap_or(0);
    assert_eq!(denied, 1, "exactly one denied record is seeded");
    let env = page
        .summary
        .selected_tier_counts
        .get(ProxyResolutionTierClass::EnvironmentProxy.as_str())
        .copied()
        .unwrap_or(0);
    assert!(
        env >= 1,
        "environment proxy tier must be selected at least once"
    );
}

#[test]
fn support_export_rolls_up_defects_and_excludes_raw_material() {
    let page = page();
    let export = ProxyResolutionSupportExport::from_page(
        "remote:networked_surface_proxy_resolution:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.record_kind,
        PROXY_RESOLUTION_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn cli_view_quotes_stable_tier_and_outcome_tokens() {
    let page = page();
    let view = page.render_cli_view();
    assert!(view.contains(ProxyResolutionTierClass::EnvironmentProxy.as_str()));
    assert!(view.contains(ProxyResolutionOutcomeClass::DeniedProxyResolution.as_str()));
    assert!(view.contains(ProxyResolutionDenialClass::ContradictoryProxyState.as_str()));
    assert!(view.contains("ai_gateway"));
}

#[test]
fn page_carries_stable_metadata_and_evidence_index_ref() {
    let page = page();
    assert_eq!(page.record_kind, PROXY_RESOLUTION_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, PROXY_RESOLUTION_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        PROXY_RESOLUTION_SHARED_CONTRACT_REF
    );
    assert_eq!(page.evidence_index_ref, PROXY_RESOLUTION_EVIDENCE_INDEX_REF);
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: ProxyResolutionGovernancePage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
