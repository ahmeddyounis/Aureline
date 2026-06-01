use super::*;

fn page() -> TransportGovernancePage {
    seeded_transport_governance_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0, "seeded page must be clean: {:?}", page.defects);
    assert!(validate_transport_governance_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        TransportGovernanceQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_seven_stability_conditions() {
    let page = page();
    assert!(page.covers_all_required_lanes(), "all seven lanes must be covered");
    assert!(
        page.all_lanes_declare_local_core_continuity(),
        "all lanes must declare local-core continuity"
    );
    assert!(
        page.network_lanes_have_policy_epoch_refs(),
        "all network-dependent lanes must have policy epoch refs"
    );
    assert!(
        page.all_lanes_have_typed_egress_decisions(),
        "all lanes must have typed egress decisions"
    );
    assert!(
        page.all_lanes_distinguish_plane_impairment(),
        "all lanes must distinguish control-plane from data-plane impairment"
    );
}

#[test]
fn seeded_page_covers_all_required_lanes() {
    let page = page();
    let covered = page.policy_snapshot.covered_lane_tokens();
    for required_lane in &REQUIRED_EGRESS_LANES {
        assert!(
            covered.contains(required_lane.as_str()),
            "required lane '{}' must be covered",
            required_lane.as_str()
        );
    }
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty(), "page must have at least one row");
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            TransportGovernanceQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.lane_token,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            TransportGovernanceNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason; got '{}'",
            row.lane_token,
            row.narrow_reason_token
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.preview_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
}

#[test]
fn seeded_page_summary_covers_seven_lanes() {
    let page = page();
    assert_eq!(
        page.summary.lanes_covered.len(),
        REQUIRED_EGRESS_LANES.len(),
        "summary must cover all seven required lanes"
    );
}

#[test]
fn seeded_page_all_rows_have_local_core_continuity_declared() {
    let page = page();
    for row in &page.rows {
        assert!(
            row.local_core_continuity_declared,
            "row '{}' must declare local-core continuity",
            row.lane_token
        );
    }
    assert_eq!(
        page.summary.local_core_continuity_declared_count,
        page.rows.len()
    );
}

#[test]
fn seeded_page_network_rows_have_policy_epoch_present() {
    let page = page();
    for row in &page.rows {
        let record = page
            .policy_snapshot
            .record_for_lane(EgressLaneClass::MirrorOffline);
        if let Some(r) = record {
            if r.lane_token == row.lane_token {
                // MirrorOffline / air-gapped lane does not require epoch ref.
                continue;
            }
        }
        // All network/managed lanes must have policy epoch ref.
        let rec = page
            .policy_snapshot
            .records
            .iter()
            .find(|r| r.lane_token == row.lane_token)
            .unwrap();
        if rec.dependency_class.requires_policy_epoch_ref() {
            assert!(
                row.policy_epoch_present,
                "row '{}' must have policy_epoch_present for {} dependency",
                row.lane_token,
                row.dependency_class_token
            );
        }
    }
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = TransportGovernanceSupportExport::from_page(
        "remote:transport-governance-export:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn raw_private_material_on_any_lane_withdraws_packet() {
    let mut snapshot = seeded_transport_policy_snapshot();
    // Inject raw material exposure on the update lane.
    snapshot.records[0].raw_private_material_excluded = false;
    let page = TransportGovernancePage::new(
        "remote:transport_governance:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        TransportGovernanceQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == TransportGovernanceNarrowReasonClass::RawPrivateMaterialExposed
    }));
    // Hard guardrail: only the withdrawal defect should be present (early return).
    assert_eq!(page.defects.len(), 1);
}

#[test]
fn missing_required_lane_narrows_to_preview() {
    let mut snapshot = seeded_transport_policy_snapshot();
    // Remove the AI lane.
    snapshot.records.retain(|r| r.lane != EgressLaneClass::Ai);
    let page = TransportGovernancePage::new(
        "remote:transport_governance:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        TransportGovernanceQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == TransportGovernanceNarrowReasonClass::RequiredLaneMissing
            && d.source == EgressLaneClass::Ai.as_str()
    }));
}

#[test]
fn lane_without_local_core_continuity_narrows_to_beta() {
    let mut snapshot = seeded_transport_policy_snapshot();
    snapshot.records[1].local_core_continuity_allowed = false;
    let page = TransportGovernancePage::new(
        "remote:transport_governance:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        TransportGovernanceQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == TransportGovernanceNarrowReasonClass::LocalCoreContinuityUndeclared
    }));
}

#[test]
fn network_lane_without_policy_epoch_ref_narrows_to_beta() {
    let mut snapshot = seeded_transport_policy_snapshot();
    // Remove policy epoch ref from the update lane (network dependency).
    snapshot.records[0].last_known_good_policy_epoch_ref = None;
    let page = TransportGovernancePage::new(
        "remote:transport_governance:test",
        "test page",
        "2026-06-01T00:00:00Z",
        snapshot,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        TransportGovernanceQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == TransportGovernanceNarrowReasonClass::PolicyEpochRefMissing
    }));
}

#[test]
fn qualification_class_checks() {
    assert!(TransportGovernanceQualificationClass::Stable.is_stable());
    assert!(!TransportGovernanceQualificationClass::Beta.is_stable());
    assert!(!TransportGovernanceQualificationClass::Withdrawn.is_stable());
    assert!(TransportGovernanceQualificationClass::Stable.is_claimable());
    assert!(TransportGovernanceQualificationClass::Beta.is_claimable());
    assert!(!TransportGovernanceQualificationClass::Withdrawn.is_claimable());
    assert!(!TransportGovernanceQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_sentinel_checks() {
    assert!(
        TransportGovernanceNarrowReasonClass::RawPrivateMaterialExposed.is_withdrawal_reason()
    );
    assert!(
        !TransportGovernanceNarrowReasonClass::RequiredLaneMissing.is_withdrawal_reason()
    );
    assert!(
        TransportGovernanceNarrowReasonClass::RequiredLaneMissing.is_preview_reason()
    );
    assert!(
        !TransportGovernanceNarrowReasonClass::LocalCoreContinuityUndeclared.is_preview_reason()
    );
    assert!(
        !TransportGovernanceNarrowReasonClass::NotNarrowed.is_withdrawal_reason()
    );
}

#[test]
fn egress_lane_tokens_are_stable() {
    assert_eq!(EgressLaneClass::Update.as_str(), "update");
    assert_eq!(EgressLaneClass::Marketplace.as_str(), "marketplace");
    assert_eq!(EgressLaneClass::Ai.as_str(), "ai");
    assert_eq!(EgressLaneClass::Docs.as_str(), "docs");
    assert_eq!(EgressLaneClass::Provider.as_str(), "provider");
    assert_eq!(EgressLaneClass::Remote.as_str(), "remote");
    assert_eq!(EgressLaneClass::MirrorOffline.as_str(), "mirror_offline");
}

#[test]
fn egress_decision_acceptable_checks() {
    assert!(EgressDecisionClass::Allowed.is_acceptable());
    assert!(EgressDecisionClass::MirrorRouted.is_acceptable());
    assert!(EgressDecisionClass::CachedOffline.is_acceptable());
    assert!(EgressDecisionClass::LastKnownGood.is_acceptable());
    assert!(!EgressDecisionClass::BlockedPolicy.is_acceptable());
    assert!(!EgressDecisionClass::BlockedTransport.is_acceptable());
    assert!(!EgressDecisionClass::ControlPlaneImpaired.is_acceptable());
    assert!(!EgressDecisionClass::DataPlaneImpaired.is_acceptable());
}

#[test]
fn record_kind_constants_match_schema_tokens() {
    assert_eq!(
        TRANSPORT_GOVERNANCE_PAGE_RECORD_KIND,
        "remote_transport_governance_page_record"
    );
    assert_eq!(
        TRANSPORT_GOVERNANCE_DEFECT_RECORD_KIND,
        "remote_transport_governance_defect_record"
    );
    assert_eq!(TRANSPORT_GOVERNANCE_SCHEMA_VERSION, 1);
}

#[test]
fn mirror_offline_lane_does_not_require_policy_epoch_ref() {
    // MirrorOffline uses air-gapped dependency, which does not require a
    // policy epoch ref — but it must still pass all other checks.
    let snapshot = seeded_transport_policy_snapshot();
    let mirror = snapshot
        .record_for_lane(EgressLaneClass::MirrorOffline)
        .expect("MirrorOffline lane must be in seeded snapshot");
    assert!(!mirror.dependency_class.requires_policy_epoch_ref());
    // The seeded page has no epoch ref for this lane and is still stable.
    assert!(seeded_transport_governance_page().qualifies_stable());
}

#[test]
fn audit_function_returns_same_defects_as_page() {
    let page = page();
    let audit_defects = audit_transport_governance_page(&page);
    assert_eq!(audit_defects.len(), page.defects.len());
}
