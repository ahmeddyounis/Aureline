use super::*;

// ---------------------------------------------------------------------------
// Seeded page invariants
// ---------------------------------------------------------------------------

#[test]
fn seeded_page_qualifies_stable() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.qualifies_stable(),
        "seeded page must qualify stable; got '{}'; defects: {:?}",
        page.summary.overall_qualification_token,
        page.defects
    );
}

#[test]
fn seeded_page_has_zero_defects() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.defects.is_empty(),
        "seeded page must have zero defects; got: {:?}",
        page.defects
    );
}

#[test]
fn seeded_page_has_ten_rows() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert_eq!(
        page.rows.len(),
        10,
        "seeded page must have 10 rows (5 flows × 2 bundle kinds)"
    );
}

#[test]
fn seeded_page_covers_all_import_flows() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.covers_all_required_import_flows(),
        "seeded page must cover all five required import flows"
    );
}

#[test]
fn seeded_page_covers_both_bundle_kinds() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let kinds: std::collections::BTreeSet<&str> = page
        .rows
        .iter()
        .map(|r| r.bundle_kind_token.as_str())
        .collect();
    assert!(
        kinds.contains("policy_bundle"),
        "seeded page must cover policy_bundle kind"
    );
    assert!(
        kinds.contains("entitlement_snapshot"),
        "seeded page must cover entitlement_snapshot kind"
    );
}

#[test]
fn seeded_page_all_rows_are_stable() {
    let page = seeded_finalize_signed_policy_bundle_page();
    for row in &page.rows {
        assert_eq!(
            row.qualification_token, "stable",
            "row '{}' must qualify stable in the seeded page",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_epoch_states_inspectable() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.all_epoch_states_inspectable(),
        "all seeded rows must carry fully inspectable epoch states"
    );
}

#[test]
fn seeded_page_all_simulation_packets_have_affected_surfaces() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.all_simulation_packets_have_affected_surfaces(),
        "all seeded rows must carry at least one affected surface in the simulation packet"
    );
}

#[test]
fn seeded_page_all_rows_explicit_on_local_core_continuity() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.all_rows_explicit_on_local_core_continuity(),
        "all seeded rows must carry local_core_continuity_explicit: true"
    );
}

#[test]
fn seeded_page_stale_rows_are_explicitly_labeled() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.stale_rows_are_explicitly_labeled(),
        "all stale seeded rows must carry an explicit staleness label"
    );
}

#[test]
fn seeded_page_stale_rows_have_declared_grace_windows() {
    let page = seeded_finalize_signed_policy_bundle_page();
    assert!(
        page.stale_rows_have_declared_grace_windows(),
        "all stale seeded rows must declare a bounded grace window"
    );
}

#[test]
fn seeded_page_offline_grace_rows_have_in_grace_posture() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let grace_rows: Vec<_> = page
        .rows
        .iter()
        .filter(|r| r.import_flow == BundleImportFlowClass::OfflineGrace)
        .collect();
    assert_eq!(grace_rows.len(), 2, "expected two offline_grace rows");
    for row in grace_rows {
        assert_eq!(
            row.grace_state.grace_posture,
            GracePostureClass::InGrace,
            "offline_grace row '{}' must have InGrace posture",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_no_row_widens_managed_claims() {
    let page = seeded_finalize_signed_policy_bundle_page();
    for row in &page.rows {
        assert!(
            !row.simulation_packet.widens_managed_claims,
            "seeded row '{}' must not widen managed claims",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_all_packets_inspectable_before_apply() {
    let page = seeded_finalize_signed_policy_bundle_page();
    for row in &page.rows {
        assert!(
            row.simulation_packet.inspectable_before_apply,
            "seeded row '{}' simulation packet must be inspectable before apply",
            row.row_id
        );
    }
}

// ---------------------------------------------------------------------------
// Audit: staleness disguised as auth failure triggers withdrawal
// ---------------------------------------------------------------------------

#[test]
fn staleness_disguised_as_auth_failure_triggers_withdrawal() {
    let mut rows = vec![row_offline_grace_policy()];
    // Remove the staleness label to trigger the guardrail.
    rows[0].grace_state.staleness_label.clear();
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:withdrawal:staleness-disguised",
        "Test: staleness disguised as auth failure",
        "2026-06-01T00:00:00Z",
        rows,
        verifier_page,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Withdrawn.as_str(),
        "staleness disguised as auth failure must withdraw the packet"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == FinalizeSignedPolicyBundleNarrowReasonClass::StalenessDisguisedAsAuthFailure
        }),
        "defects must include staleness_disguised_as_auth_failure"
    );
}

// ---------------------------------------------------------------------------
// Audit: missing import flow triggers preview narrowing
// ---------------------------------------------------------------------------

#[test]
fn missing_import_flow_narrows_to_preview() {
    // Only include online rows — mirror, manual, air-gapped, offline_grace missing.
    let rows = vec![row_online_policy(), row_online_entitlement()];
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:preview:missing-flow",
        "Test: missing import flow",
        "2026-06-01T00:00:00Z",
        rows,
        verifier_page,
    );
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeSignedPolicyBundleQualificationClass::Preview.as_str(),
        "missing import flow must narrow to preview"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason == FinalizeSignedPolicyBundleNarrowReasonClass::ImportFlowCoverageGap
        }),
        "defects must include import_flow_coverage_gap"
    );
}

// ---------------------------------------------------------------------------
// Audit: empty affected surfaces narrows to beta
// ---------------------------------------------------------------------------

#[test]
fn simulation_packet_with_no_affected_surfaces_narrows_to_beta() {
    let mut rows = seeded_rows();
    rows[0].simulation_packet.affected_surfaces.clear();
    let verifier_page = seeded_offline_entitlement_verifier_beta_page();
    let page = FinalizeSignedPolicyBundlePage::new(
        "test:beta:no-surfaces",
        "Test: simulation packet missing affected surfaces",
        "2026-06-01T00:00:00Z",
        rows,
        verifier_page,
    );
    assert!(
        page.summary.overall_qualification_token == FinalizeSignedPolicyBundleQualificationClass::Beta.as_str()
            || page.summary.overall_qualification_token
                == FinalizeSignedPolicyBundleQualificationClass::Withdrawn.as_str(),
        "missing affected surfaces must narrow to at least beta"
    );
    assert!(
        page.defects.iter().any(|d| {
            d.narrow_reason
                == FinalizeSignedPolicyBundleNarrowReasonClass::SimulationPacketMissingBeforeApply
        }),
        "defects must include simulation_packet_missing_before_apply"
    );
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

#[test]
fn support_export_always_excludes_raw_private_material() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let export = FinalizeSignedPolicyBundleSupportExport::from_page(
        "export:test:001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(
        export.raw_private_material_excluded,
        "support export must always report raw_private_material_excluded: true"
    );
}

#[test]
fn support_export_has_correct_record_kind() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let export = FinalizeSignedPolicyBundleSupportExport::from_page(
        "export:test:002",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert_eq!(
        export.record_kind,
        SIGNED_POLICY_BUNDLE_FINALIZE_SUPPORT_EXPORT_RECORD_KIND
    );
}

// ---------------------------------------------------------------------------
// Re-audit helpers are consistent
// ---------------------------------------------------------------------------

#[test]
fn validate_returns_ok_for_seeded_page() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let result = validate_finalize_signed_policy_bundle_page(&page);
    assert!(
        result.is_ok(),
        "validate must return Ok for the seeded page; defects: {:?}",
        result.err()
    );
}

#[test]
fn audit_returns_zero_defects_for_seeded_page() {
    let page = seeded_finalize_signed_policy_bundle_page();
    let defects = audit_finalize_signed_policy_bundle_page(&page);
    assert!(
        defects.is_empty(),
        "re-audit of seeded page must return zero defects; got: {:?}",
        defects
    );
}
