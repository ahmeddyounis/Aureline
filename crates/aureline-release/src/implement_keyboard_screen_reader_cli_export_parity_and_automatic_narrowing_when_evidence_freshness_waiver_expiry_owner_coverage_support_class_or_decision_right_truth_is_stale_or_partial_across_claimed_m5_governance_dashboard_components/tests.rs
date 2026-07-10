//! Tests for the M05-1058 governance-dashboard component accessibility parity
//! capstone: the honest auto-narrowing logic, the per-family parity contract, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> GovernanceAccessibilityRow {
    seeded_m5_governance_dashboard_a11y_parity_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5GovernanceDashboardComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5GovernanceDashboardComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5GovernanceClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5GovernanceSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5GovernanceConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_seven_yellow_zero_red() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC2: a stale / expiring / unresolved lane can no longer keep a clean green pass ---

#[test]
fn current_fitness_tile_is_governed_pass_and_green() {
    let tile = row("a11y:fitness-dashboard-tile");
    assert_eq!(
        tile.full_support_claim,
        M5GovernanceSupportClaim::GovernedPass
    );
    assert_eq!(
        tile.effective_claim(),
        M5GovernanceSupportClaim::GovernedPass
    );
    assert!(tile.claim_narrow.is_none());
    assert_eq!(tile.status(), GovernanceAccessibilityStatus::Parity);
    assert!(tile.effective_claim().asserts_clean_pass());
}

#[test]
fn current_ownership_card_is_governed_resolved_and_green() {
    let card = row("a11y:service-ownership-card");
    assert_eq!(
        card.effective_claim(),
        M5GovernanceSupportClaim::GovernedResolved
    );
    assert!(card.claim_narrow.is_none());
    assert_eq!(card.status(), GovernanceAccessibilityStatus::Parity);
    assert!(card.effective_claim().asserts_full_self_sufficiency());
    assert!(!card.effective_claim().asserts_clean_pass());
}

#[test]
fn stale_evidence_narrows_report_row_to_provisional() {
    let report = row("a11y:governance-report-row");
    assert_eq!(
        report.effective_claim(),
        M5GovernanceSupportClaim::Provisional
    );
    assert!(!report.effective_claim().asserts_clean_pass());
    let narrow = report.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5GovernanceDowngradeTrigger::EvidenceStaleHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5GovernanceClaimDimension::EvidenceFreshness
    );
    assert!(report.claim_is_honest());
}

#[test]
fn expiring_waiver_narrows_queue_item_to_waiver_gated() {
    let item = row("a11y:waiver-expiry-queue-item");
    assert_eq!(
        item.effective_claim(),
        M5GovernanceSupportClaim::WaiverGated
    );
    assert!(!item.effective_claim().asserts_clean_pass());
    let narrow = item.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5GovernanceDowngradeTrigger::WaiverExpiryHidden
    );
    assert!(item.claim_is_honest());
}

#[test]
fn unresolved_forum_narrows_release_gate_to_blocked() {
    let banner = row("a11y:release-gate-banner");
    assert_eq!(banner.effective_claim(), M5GovernanceSupportClaim::Blocked);
    assert!(!banner.effective_claim().asserts_full_self_sufficiency());
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5GovernanceDowngradeTrigger::DecisionForumMasked
    );
    assert!(banner.claim_is_honest());
}

#[test]
fn partial_support_class_narrows_mitigation_to_degraded() {
    let card = row("a11y:mitigation-note-card");
    assert_eq!(card.effective_claim(), M5GovernanceSupportClaim::Degraded);
    let narrow = card.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.binding_dimension,
        M5GovernanceClaimDimension::SupportClass
    );
    assert_eq!(
        narrow.trigger,
        M5GovernanceDowngradeTrigger::MitigationHiddenBehindJargon
    );
    assert!(card.claim_is_honest());
}

#[test]
fn partial_coverage_narrows_on_call_strip_to_degraded() {
    let strip = row("a11y:on-call-strip");
    assert_eq!(strip.effective_claim(), M5GovernanceSupportClaim::Degraded);
    let narrow = strip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5GovernanceDowngradeTrigger::OwnerCoverageOverstated
    );
    assert!(strip.claim_is_honest());
}

#[test]
fn stale_decision_right_narrows_card_to_provisional() {
    let card = row("a11y:decision-right-card");
    assert_eq!(
        card.effective_claim(),
        M5GovernanceSupportClaim::Provisional
    );
    assert!(!card.effective_claim().asserts_clean_pass());
    assert!(card.claim_is_honest());
}

#[test]
fn unresolved_forum_narrows_milestone_row_to_blocked() {
    let milestone = row("a11y:milestone-dashboard-row");
    assert_eq!(
        milestone.effective_claim(),
        M5GovernanceSupportClaim::Blocked
    );
    assert!(!milestone.effective_claim().asserts_full_self_sufficiency());
    assert!(milestone.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an unresolved release-gate
    // banner claiming its full governed-resolved posture with no narrow block.
    let mut banner = row("a11y:release-gate-banner");
    banner.claim_narrow = None;
    assert!(!banner.claim_is_honest());
    assert_eq!(banner.status(), GovernanceAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_current_row_is_rejected() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.claim_narrow = Some(GovernanceClaimAutoNarrow {
        narrowed_to: M5GovernanceSupportClaim::Degraded,
        binding_dimension: M5GovernanceClaimDimension::EvidenceFreshness,
        trigger: M5GovernanceDowngradeTrigger::EvidenceStaleHidden,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!tile.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut report = row("a11y:governance-report-row");
    if let Some(narrow) = report.claim_narrow.as_mut() {
        narrow.binding_dimension = M5GovernanceClaimDimension::WaiverExpiry;
    }
    assert!(!report.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut report = row("a11y:governance-report-row");
    if let Some(narrow) = report.claim_narrow.as_mut() {
        narrow.trigger = M5GovernanceDowngradeTrigger::WaiverExpiryHidden;
    }
    assert!(!report.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut report = row("a11y:governance-report-row");
    if let Some(narrow) = report.claim_narrow.as_mut() {
        narrow.narrowed_label = "warning".to_owned();
    }
    assert!(!report.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5GovernanceConditionState as C;
    use M5GovernanceSupportClaim as S;
    assert_eq!(C::Current.permitted_ceiling(), S::GovernedPass);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Stale.permitted_ceiling(), S::Provisional);
    assert_eq!(C::Waived.permitted_ceiling(), S::WaiverGated);
    assert_eq!(C::Unresolved.permitted_ceiling(), S::Blocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5GovernanceClaimDimension as D;
    use M5GovernanceDowngradeTrigger as T;
    assert_eq!(
        D::EvidenceFreshness.default_trigger(),
        T::EvidenceStaleHidden
    );
    assert_eq!(D::WaiverExpiry.default_trigger(), T::WaiverExpiryHidden);
    assert_eq!(
        D::OwnerCoverage.default_trigger(),
        T::OwnerCoverageOverstated
    );
    assert_eq!(
        D::SupportClass.default_trigger(),
        T::MitigationHiddenBehindJargon
    );
    assert_eq!(
        D::DecisionRightTruth.default_trigger(),
        T::DecisionForumMasked
    );
}

// --- AC1: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_governance_dashboard_a11y_parity_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_milestone_row_binds_a_non_visual_fallback() {
    let milestone = row("a11y:milestone-dashboard-row");
    assert!(milestone.is_hierarchy_heavy());
    assert!(milestone.has_non_visual_fallback());
    assert!(milestone
        .fallback_modalities
        .contains(&M5GovernanceFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.keyboard_reach = GovernanceNonVisualReachState::ViewOnlyTrap;
    assert!(!tile.reaches_canonical_truth_via_at());
    assert_eq!(tile.status(), GovernanceAccessibilityStatus::Stranded);
}

#[test]
fn empty_governance_context_ref_strands_a_row() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.governance_context_ref = "  ".to_owned();
    assert!(!tile.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.export_summary = GovernanceExportSummaryState::AbsentNeedsScreenshot;
    assert!(!tile.export_preserves_meaning());
    assert_eq!(tile.status(), GovernanceAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.copy_export.formats.retain(|f| f != "markdown");
    assert!(!tile.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut report = row("a11y:governance-report-row");
    report.narrowing_disclosures.clear();
    assert!(!report.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut report = row("a11y:governance-report-row");
    report.narrowing_disclosures[0].state = GovernanceNarrowingDisclosureState::SilentlyDropped;
    assert!(!report.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut report = row("a11y:governance-report-row");
    report.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!report.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut tile = row("a11y:fitness-dashboard-tile");
    tile.required_labels
        .retain(|l| *l != M5GovernanceRequiredLabel::Identity);
    assert!(!tile.preserves_mandatory_labels());
    assert_eq!(tile.status(), GovernanceAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:governance-report-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        GovernanceAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    packet.rows[0].consumer_surfaces = vec![M5GovernanceConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        GovernanceAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, GovernanceAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, GovernanceAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        GovernanceAccessibilityViolation::RawGovernanceMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:release-gate-banner").chip_tokens();
    assert!(chip.contains("family=release_gate_banner"));
    assert!(chip.contains("effective_claim=blocked"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert_eq!(packet.record_kind, GOVERNANCE_A11Y_RECORD_KIND);
    assert_eq!(packet.schema_version, GOVERNANCE_A11Y_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_governance_dashboard_a11y_parity_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_governance_dashboard_a11y_parity_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_governance_dashboard_a11y_parity_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-component-accessibility-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_governance_dashboard_a11y_parity_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-governance-dashboard-component-accessibility-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so
/// it never runs in the normal suite. Run with
/// `GEN_GOVERNANCE_DASHBOARD_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_GOVERNANCE_DASHBOARD_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_governance_dashboard_a11y_parity_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-governance-dashboard-component-accessibility-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-governance-dashboard-component-accessibility-parity");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 governance-dashboard component accessibility parity fixtures\n\n\
         Mirror of `artifacts/release/m5-governance-dashboard-component-accessibility-proof/`.\n\
         Regenerate with `GEN_GOVERNANCE_DASHBOARD_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-release generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
