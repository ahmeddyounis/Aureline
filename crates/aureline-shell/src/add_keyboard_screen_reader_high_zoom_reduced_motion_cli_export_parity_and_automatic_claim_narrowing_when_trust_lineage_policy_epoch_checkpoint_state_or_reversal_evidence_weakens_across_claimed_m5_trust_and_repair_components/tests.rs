//! Tests for the M05-1098 workspace-trust / repair-component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the stale/expired/mixed/partial-never-full-trust
//! guarantee, no-loss trust / repair-truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> TrustRepairComponentAccessibilityRow {
    seeded_m5_workspace_trust_repair_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let violations = packet.validate();
    assert!(violations.is_empty(), "unexpected violations: {violations:?}");
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5WorkspaceTrustRepairComponentFamily::ALL.len()
    );
    // Eight rows cover the eight families one-to-one (one certified fully green — the fully attributed
    // repair-result receipt row — the other seven narrowed-yellow: six auto-narrowed claims plus the
    // dense trust-fact grid whose screen-reader traversal narrows to a disclosed linear walk).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5TrustRepairComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5TrustRepairComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5TrustRepairComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5WorkspaceTrustRepairConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_seven_yellow_zero_red() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5WorkspaceTrustRepairComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_full_trust_honesty() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_full_trust_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
}

// --- auto-narrowing per family ---

#[test]
fn attributed_receipt_row_is_full_trust_and_green() {
    let receipt = row("a11y:repair-result-receipt-row-attributed");
    assert_eq!(
        receipt.full_trust_claim,
        M5TrustRepairComponentClaim::FullTrustReviewedResult
    );
    assert_eq!(
        receipt.effective_claim(),
        M5TrustRepairComponentClaim::FullTrustReviewedResult
    );
    assert!(receipt.claim_narrow.is_none());
    assert_eq!(
        receipt.status(),
        TrustRepairComponentAccessibilityStatus::Parity
    );
    assert!(receipt
        .effective_claim()
        .asserts_full_trust_reviewed_result());
}

#[test]
fn stale_lineage_banner_narrows_and_is_never_full_trust() {
    let banner = row("a11y:workspace-trust-banner-stale-lineage");
    assert_eq!(
        banner.effective_claim(),
        M5TrustRepairComponentClaim::StaleLineageProjection
    );
    assert!(!banner
        .effective_claim()
        .asserts_full_trust_reviewed_result());
    assert!(banner.full_trust_honesty_holds());
    assert_eq!(
        banner.status(),
        TrustRepairComponentAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = banner.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5TrustRepairComponentClaimDimension::TrustGrantLineage
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:workspace-trust-banner-stale-lineage",
            M5TrustRepairComponentClaim::StaleLineageProjection,
        ),
        (
            "a11y:trust-elevation-sheet-expired-epoch",
            M5TrustRepairComponentClaim::ExpiredEpochProjection,
        ),
        (
            "a11y:restricted-capability-row-narrowed",
            M5TrustRepairComponentClaim::NarrowedCapabilityProjection,
        ),
        (
            "a11y:root-trust-strip-mixed-root",
            M5TrustRepairComponentClaim::MixedRootProjection,
        ),
        (
            "a11y:repair-transaction-preview-card-missing-checkpoint",
            M5TrustRepairComponentClaim::MissingCheckpointProjection,
        ),
        (
            "a11y:rollback-class-strip-unproven-reversal",
            M5TrustRepairComponentClaim::UnprovenReversalProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            TrustRepairComponentAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_full_trust_flags_exactly_the_evidence_states() {
    use M5TrustRepairComponentConditionState as C;
    assert!(C::TrustLineageStale.cannot_be_shown_full_trust());
    assert!(C::PolicyEpochExpired.cannot_be_shown_full_trust());
    assert!(C::PerRootTrustMixed.cannot_be_shown_full_trust());
    assert!(C::ReversalEvidencePartial.cannot_be_shown_full_trust());
    // Honest restricted-mode / disclosed-absence operations are not truth overstatements.
    assert!(!C::CapabilityNarrowed.cannot_be_shown_full_trust());
    assert!(!C::CheckpointMissing.cannot_be_shown_full_trust());
    assert!(!C::FullTrustReviewed.cannot_be_shown_full_trust());
}

#[test]
fn trust_fact_grid_is_reviewable_hierarchy_heavy_and_yellow() {
    let grid = row("a11y:trust-fact-grid-scoped");
    assert!(grid.is_hierarchy_heavy());
    assert!(grid.has_non_visual_fallback());
    assert!(grid
        .fallback_modalities
        .contains(&M5TrustRepairComponentFallbackModality::Structured));
    assert_eq!(
        grid.effective_claim(),
        M5TrustRepairComponentClaim::ReviewableResult
    );
    assert!(grid.claim_narrow.is_none());
    assert_eq!(
        grid.status(),
        TrustRepairComponentAccessibilityStatus::NarrowedDisclosed
    );
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut banner = row("a11y:workspace-trust-banner-stale-lineage");
    // Drop the narrow so the stale-lineage state keeps a full-trust claim.
    banner.claim_narrow = None;
    assert!(!banner.claim_is_honest());
    assert!(!banner.full_trust_honesty_holds());
    assert_eq!(
        banner.status(),
        TrustRepairComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut grid = row("a11y:trust-fact-grid-scoped");
    grid.keyboard_reach = TrustRepairComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!grid.reaches_canonical_truth_via_at());
    assert_eq!(
        grid.status(),
        TrustRepairComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut receipt = row("a11y:repair-result-receipt-row-attributed");
    receipt.export_summary = TrustRepairComponentExportSummaryState::RequiresRawPayload;
    assert!(!receipt.export_preserves_meaning());
    assert_eq!(
        receipt.status(),
        TrustRepairComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut banner = row("a11y:workspace-trust-banner-stale-lineage");
    banner.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!banner.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    packet.rows.pop();
    let mut packet = TrustRepairComponentAccessibilityPacket::new(
        TrustRepairComponentAccessibilityPacketInput {
            packet_id: packet.packet_id,
            as_of: packet.as_of,
            matrix_ref: packet.matrix_ref,
            rows: packet.rows,
        },
    );
    packet.summary = packet.computed_summary();
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_family_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    packet.rows[0].copy_export.export_fields.push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_trust_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_component() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 8 rows + trailing newline
    assert_eq!(csv.lines().count(), 9);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_workspace_trust_repair_component_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_workspace_trust_repair_component_a11y_export()
        .expect("checked M5 workspace-trust-repair a11y export validates");
    assert_eq!(from_disk.packet_id, WORKSPACE_TRUST_REPAIR_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_workspace_trust_repair_component_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_workspace_trust_repair_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-trust-repair-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(on_disk, expected, "checked matrix CSV drifted from the builder");
}

#[test]
fn checked_report_matches_builder() {
    let expected =
        seeded_m5_workspace_trust_repair_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-trust-repair-component-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-repair-component-accessibility-parity/support_export.json"
    ));
    let packet: TrustRepairComponentAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_workspace_trust_repair_component_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-workspace-trust-repair-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_workspace_trust_repair_component_a11y_packet().render_matrix_csv()
    );
}
