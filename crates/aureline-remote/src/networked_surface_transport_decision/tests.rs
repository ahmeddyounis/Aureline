use super::*;

fn page() -> TransportDecisionLogPage {
    seeded_transport_decision_page()
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
    assert!(validate_transport_decision_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_stability_conditions() {
    let page = page();
    assert!(
        page.covers_all_required_surfaces(),
        "all required surfaces must have a decision"
    );
    assert!(
        page.no_decision_bypasses_governance(),
        "no decision may bypass the shared governance layer"
    );
    assert!(
        page.no_decision_allows_silent_public_fallback(),
        "no decision may permit a silent public fall-through"
    );
    assert!(
        page.replay_queues_are_idempotent_only(),
        "offline/replay queues must be idempotent-only"
    );
    assert!(
        page.egress_classes_have_policy_epoch_refs(),
        "egress classes that require a policy epoch must carry one"
    );
    assert!(
        page.denied_decisions_carry_reasons(),
        "denied decisions must carry a typed reason"
    );
    assert!(
        page.all_decisions_declare_trust_proof(),
        "all decisions must carry a trust-proof ref"
    );
    assert!(
        page.all_decision_proofs_usable(),
        "all decision proofs must be usable"
    );
}

#[test]
fn seeded_page_covers_all_required_surfaces() {
    let page = page();
    let covered = page.decision_snapshot.covered_surface_tokens();
    assert_eq!(covered.len(), REQUIRED_SURFACES.len());
    for required in &REQUIRED_SURFACES {
        assert!(
            covered.contains(required.as_str()),
            "required surface '{}' must have a decision",
            required.as_str()
        );
    }
    assert_eq!(page.rows.len(), REQUIRED_SURFACES.len());
    assert_eq!(page.summary.stable_row_count, REQUIRED_SURFACES.len());
}

#[test]
fn every_decision_excludes_raw_material_and_uses_handle_only_auth() {
    let snapshot = seeded_transport_decision_snapshot();
    for decision in &snapshot.decisions {
        assert!(
            decision.raw_material_excluded(),
            "decision '{}' must exclude raw private material",
            decision.decision_id
        );
        if decision.auth_posture != AuthPostureClass::Anonymous {
            assert!(
                decision.auth_posture_token.ends_with("_handle"),
                "decision '{}' auth posture '{}' must be handle-only",
                decision.decision_id,
                decision.auth_posture_token
            );
        }
        // No raw URL ever leaks: the endpoint is named by opaque handle only.
        assert!(decision.endpoint.endpoint_handle.starts_with("endpoint:"));
    }
}

#[test]
fn every_decision_route_choice_matches_resolution_tier() {
    let snapshot = seeded_transport_decision_snapshot();
    for decision in &snapshot.decisions {
        assert!(
            decision.policy.is_route_consistent(),
            "decision '{}' route '{}' must agree with proxy-resolution tier '{}'",
            decision.decision_id,
            decision.policy.route_choice_token,
            decision.policy.proxy_resolution_source_token
        );
    }
}

#[test]
fn offline_deferred_decisions_are_idempotent_only() {
    let snapshot = seeded_transport_decision_snapshot();
    for decision in &snapshot.decisions {
        if decision.outcome.is_offline_deferred() {
            assert!(
                decision.action_is_idempotent,
                "offline-deferred decision '{}' must queue an idempotent action only",
                decision.decision_id
            );
        }
    }
}

#[test]
fn drill_missing_surface_narrows_to_preview() {
    let mut snapshot = seeded_transport_decision_snapshot();
    snapshot
        .decisions
        .retain(|d| d.surface != SurfaceClass::AiGateway);
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:missing-surface",
        "Drill — required surface absent (preview)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.covers_all_required_surfaces());
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::RequiredSurfaceMissing));
}

#[test]
fn drill_raw_material_withdraws_packet() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::RequestApiClient {
            d.raw_private_material_excluded = false;
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:raw-material",
        "Drill — raw private material exposed (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.no_withdrawn_rows());
    assert!(page
        .rows
        .iter()
        .all(|r| r.qualification_token == DecisionQualificationClass::Withdrawn.as_str()));
}

#[test]
fn drill_bypass_withdraws_packet() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::DatabaseCloudConnector {
            d.no_bypass = false;
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:bypass",
        "Drill — governance bypass (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::BypassedSharedGovernance));
}

#[test]
fn drill_silent_public_fallback_withdraws_packet() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::RegistryRead {
            d.policy.no_silent_public_fallback = false;
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:silent-fallback",
        "Drill — silent public fall-through permitted (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::SilentPublicFallbackResolved));
}

#[test]
fn drill_non_idempotent_replay_withdraws_packet() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::SyncOffboarding {
            d.outcome = TransportOutcomeClass::OfflineDeferred;
            d.outcome_token = TransportOutcomeClass::OfflineDeferred.as_str().to_owned();
            d.action_is_idempotent = false;
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:non-idempotent-replay",
        "Drill — non-idempotent replay queued (withdrawn)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::NonIdempotentReplayQueued));
}

#[test]
fn drill_denied_without_reason_narrows_to_beta() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::ProviderMutation {
            d.outcome = TransportOutcomeClass::Denied;
            d.outcome_token = TransportOutcomeClass::Denied.as_str().to_owned();
            d.denial_reason = None;
            d.denial_reason_token = String::new();
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:denied-no-reason",
        "Drill — denied without reason (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::DenialReasonMissing));
}

#[test]
fn drill_stale_proof_narrows_row_to_beta() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::DocsBrowserFetcher {
            d.policy.trust_proof_freshness = ProofFreshnessClass::ExpiredBeyondWindow;
            d.policy.trust_proof_freshness_token =
                ProofFreshnessClass::ExpiredBeyondWindow.as_str().to_owned();
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:stale-proof",
        "Drill — stale proof beyond window (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Beta.as_str()
    );
    let docs_row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::DocsBrowserFetcher.as_str())
        .expect("docs row present");
    assert_eq!(
        docs_row.narrow_reason_token,
        DecisionNarrowReasonClass::ProofStaleBeyondWindow.as_str()
    );
    assert_eq!(
        docs_row.qualification_token,
        DecisionQualificationClass::Beta.as_str()
    );
    let ai_row = page
        .rows
        .iter()
        .find(|r| r.surface_token == SurfaceClass::AiGateway.as_str())
        .expect("ai row present");
    assert_eq!(
        ai_row.qualification_token,
        DecisionQualificationClass::Stable.as_str()
    );
}

#[test]
fn missing_policy_epoch_on_required_egress_narrows_to_beta() {
    let mut snapshot = seeded_transport_decision_snapshot();
    for d in snapshot.decisions.iter_mut() {
        if d.surface == SurfaceClass::AiGateway {
            d.policy.policy_epoch_ref = None;
        }
    }
    let page = TransportDecisionLogPage::new(
        "remote:networked_surface_transport_decision:drill:missing-epoch",
        "Drill — missing policy epoch (beta)",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == DecisionNarrowReasonClass::PolicyEpochRefMissing));
    assert_eq!(
        page.summary.overall_qualification_token,
        DecisionQualificationClass::Beta.as_str()
    );
}

#[test]
fn summary_rolls_up_outcomes() {
    let page = page();
    let allowed = page
        .summary
        .outcome_counts
        .get(TransportOutcomeClass::Allowed.as_str())
        .copied()
        .unwrap_or(0);
    assert!(allowed >= 1, "seeded page should have allowed outcomes");
    let deferred = page
        .summary
        .outcome_counts
        .get(TransportOutcomeClass::OfflineDeferred.as_str())
        .copied()
        .unwrap_or(0);
    assert_eq!(
        deferred, 1,
        "exactly one offline-deferred decision is seeded"
    );
    assert_eq!(page.summary.no_bypass_count, REQUIRED_SURFACES.len());
}

#[test]
fn support_export_rolls_up_defects_and_excludes_raw_material() {
    let page = page();
    let export = TransportDecisionSupportExport::from_page(
        "remote:networked_surface_transport_decision:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(
        export.record_kind,
        TRANSPORT_DECISION_SUPPORT_EXPORT_RECORD_KIND
    );
}

#[test]
fn page_carries_stable_metadata_and_evidence_index_ref() {
    let page = page();
    assert_eq!(page.record_kind, TRANSPORT_DECISION_PAGE_RECORD_KIND);
    assert_eq!(page.schema_version, TRANSPORT_DECISION_SCHEMA_VERSION);
    assert_eq!(
        page.shared_contract_ref,
        TRANSPORT_DECISION_SHARED_CONTRACT_REF
    );
    assert_eq!(
        page.evidence_index_ref,
        TRANSPORT_DECISION_EVIDENCE_INDEX_REF
    );
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: TransportDecisionLogPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
