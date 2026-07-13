//! Tests for the M05-1138 decision-feedback-component accessibility parity capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the weakened-never-trusted guarantee, no-loss
//! severity / scope / recovery / durability truth integrity, and the checked-in support export / CSV /
//! report.

use super::*;

fn row(id: &str) -> DecisionFeedbackAccessibilityRow {
    seeded_m5_decision_feedback_component_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5DecisionFeedbackFamily::ALL.len()
    );
    // Eight rows cover the eight families one-to-one (one certified fully green — the state-stated dialog
    // sheet — the other seven narrowed-yellow: six auto-narrowed claims plus the reviewable empty state
    // whose screen-reader traversal narrows to a disclosed linear walk).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5DecisionFeedbackClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5DecisionFeedbackConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5DecisionFeedbackA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5DecisionFeedbackConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_seven_yellow_zero_red() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 7);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5DecisionFeedbackFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_trusted_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
    assert!(packet.summary.all_structure_heavy_have_non_visual_fallback);
    assert_eq!(packet.summary.structure_heavy_family_count, 3);
}

// --- auto-narrowing per family ---

#[test]
fn state_stated_dialog_sheet_is_trusted_and_green() {
    let dialog = row("a11y:dialog-sheet-rationale-scope-actions-stated");
    assert_eq!(
        dialog.full_ready_claim,
        M5DecisionFeedbackA11yClaim::TrustedDecisionSurface
    );
    assert_eq!(
        dialog.effective_claim(),
        M5DecisionFeedbackA11yClaim::TrustedDecisionSurface
    );
    assert!(dialog.claim_narrow.is_none());
    assert_eq!(dialog.status(), DecisionFeedbackAccessibilityStatus::Parity);
    assert!(dialog.effective_claim().asserts_trusted_surface());
}

#[test]
fn stale_severity_badge_narrows_and_is_never_trusted() {
    let badge = row("a11y:badge-chip-pill-severity-evidence-stale");
    assert_eq!(
        badge.effective_claim(),
        M5DecisionFeedbackA11yClaim::SeverityUnverifiedProjection
    );
    assert!(!badge.effective_claim().asserts_trusted_surface());
    assert!(badge.trusted_honesty_holds());
    assert_eq!(
        badge.status(),
        DecisionFeedbackAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = badge.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5DecisionFeedbackDowngradeTrigger::StateTaxonomyDrifted
    );
    assert_eq!(
        narrow.binding_dimension,
        M5DecisionFeedbackClaimDimension::SeverityMeaningClarity
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:badge-chip-pill-severity-evidence-stale",
            M5DecisionFeedbackA11yClaim::SeverityUnverifiedProjection,
        ),
        (
            "a11y:banner-inline-notice-scope-evidence-stale",
            M5DecisionFeedbackA11yClaim::ScopeUnverifiedProjection,
        ),
        (
            "a11y:popover-focus-return-anchor-stale",
            M5DecisionFeedbackA11yClaim::FocusReturnUnverifiedProjection,
        ),
        (
            "a11y:toast-durable-object-linkage-stale",
            M5DecisionFeedbackA11yClaim::DurableObjectUnverifiedProjection,
        ),
        (
            "a11y:loading-state-partial-capability-unconfirmed",
            M5DecisionFeedbackA11yClaim::PartialCapabilityUnverifiedProjection,
        ),
        (
            "a11y:consequence-block-recovery-path-disclosed-partial",
            M5DecisionFeedbackA11yClaim::RecoveryPathDisclosedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            DecisionFeedbackAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_exactly_the_overclaim_states() {
    use M5DecisionFeedbackConditionState as C;
    assert!(C::SeverityEvidenceStale.cannot_be_shown_trusted());
    assert!(C::ScopeEvidenceStale.cannot_be_shown_trusted());
    assert!(C::FocusReturnAnchorStale.cannot_be_shown_trusted());
    assert!(C::DurableObjectLinkageStale.cannot_be_shown_trusted());
    assert!(C::PartialCapabilityUnconfirmed.cannot_be_shown_trusted());
    // An honest disclosed-absence operation (a partial / redacted recovery posture) is not a truth
    // overstatement.
    assert!(!C::RecoveryPathDisclosedPartial.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn empty_state_is_reviewable_and_yellow() {
    let empty = row("a11y:empty-state-purpose-next-action-stated");
    assert!(empty.has_non_visual_fallback());
    assert_eq!(
        empty.effective_claim(),
        M5DecisionFeedbackA11yClaim::ReviewableDecisionSurface
    );
    assert!(empty.claim_narrow.is_none());
    assert_eq!(
        empty.status(),
        DecisionFeedbackAccessibilityStatus::NarrowedDisclosed
    );
}

#[test]
fn structure_heavy_popover_binds_a_structured_and_non_visual_path() {
    let popover = row("a11y:popover-focus-return-anchor-stale");
    assert!(popover.is_structure_heavy());
    assert!(popover.has_non_visual_fallback());
    assert!(popover
        .fallback_modalities
        .contains(&M5DecisionFeedbackFallbackModality::Structured));
}

#[test]
fn missing_durable_object_toast_is_never_shown_as_trusted() {
    let toast = row("a11y:toast-durable-object-linkage-stale");
    assert!(!toast.effective_claim().asserts_trusted_surface());
    assert!(toast.trusted_honesty_holds());
    let narrow = toast.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5DecisionFeedbackDowngradeTrigger::DurableWorkShownAsToastOnly
    );
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut badge = row("a11y:badge-chip-pill-severity-evidence-stale");
    // Drop the narrow so the stale-severity state keeps a trusted claim.
    badge.claim_narrow = None;
    assert!(!badge.claim_is_honest());
    assert!(!badge.trusted_honesty_holds());
    assert_eq!(
        badge.status(),
        DecisionFeedbackAccessibilityStatus::Stranded
    );
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut empty = row("a11y:empty-state-purpose-next-action-stated");
    empty.keyboard_reach = DecisionFeedbackNonVisualReachState::ViewOnlyTrap;
    assert!(!empty.reaches_canonical_truth_via_at());
    assert_eq!(
        empty.status(),
        DecisionFeedbackAccessibilityStatus::Stranded
    );
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut dialog = row("a11y:dialog-sheet-rationale-scope-actions-stated");
    dialog.export_summary = DecisionFeedbackExportSummaryState::RequiresRawPayload;
    assert!(!dialog.export_preserves_meaning());
    assert_eq!(
        dialog.status(),
        DecisionFeedbackAccessibilityStatus::Stranded
    );
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut badge = row("a11y:badge-chip-pill-severity-evidence-stale");
    badge.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!badge.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_decision_feedback_component_a11y_packet();
    packet.rows.pop();
    let mut packet =
        DecisionFeedbackAccessibilityPacket::new(DecisionFeedbackAccessibilityPacketInput {
            packet_id: packet.packet_id,
            as_of: packet.as_of,
            matrix_ref: packet.matrix_ref,
            rows: packet.rows,
        });
    packet.summary = packet.computed_summary();
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_family_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_decision_feedback_component_a11y_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_component_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_component() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 8 rows + trailing newline
    assert_eq!(csv.lines().count(), 9);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_decision_feedback_component_a11y_export()
        .expect("checked M5 decision-feedback a11y export validates");
    assert_eq!(from_disk.packet_id, DECISION_FEEDBACK_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_decision_feedback_component_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_decision_feedback_component_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-decision-feedback-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_decision_feedback_component_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-decision-feedback-component-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-decision-feedback-component-accessibility-parity/support_export.json"
    ));
    let packet: DecisionFeedbackAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_decision_feedback_component_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-decision-feedback-component-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_decision_feedback_component_a11y_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_DECISION_FEEDBACK_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures
// from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_DECISION_FEEDBACK_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_decision_feedback_component_a11y_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir =
        repo.join("artifacts/release/m5-decision-feedback-component-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-decision-feedback-component-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/ui/m5-decision-feedback-component-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
