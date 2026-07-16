//! Tests for the M05-1272 AI-review-assist accessibility parity capstone: the honest auto-narrowing logic,
//! the per-object parity contract, the weakened-never-trusted guarantee, no-loss finding / scope / publish /
//! lifecycle truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> AiReviewAccessibilityRow {
    seeded_m5_ai_review_accessibility_parity_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_object_is_certified() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    assert_eq!(
        packet.represented_objects().len(),
        M5AiReviewAssistObject::ALL.len()
    );
    // Six rows cover the four objects (two certified fully green — the fresh finding row and the live
    // resolution memory row — the other four auto-narrowed yellow across the four spec narrowing axes:
    // provider freshness, diff drift, publish target, and finding lifecycle).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5AiReviewClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let states = packet.exercised_condition_states();
    for state in M5AiReviewConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5AiReviewA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5AiReviewAssistConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.object_count,
        M5AiReviewAssistObject::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    assert!(packet.summary.all_truth_preserved);
    assert!(packet.summary.all_trusted_honesty_holds);
    assert!(packet.summary.all_claims_honest);
    assert!(packet.summary.all_export_summaries_preserve_meaning);
    assert!(packet.summary.all_reach_canonical_truth_via_at);
    assert!(packet.summary.all_narrowing_disclosed);
    assert!(packet.summary.all_structure_heavy_have_non_visual_fallback);
    // Rows 2, 5, and 6 key on structure-heavy objects (resolution memory row ×2, publish sheet ×1).
    assert_eq!(packet.summary.structure_heavy_object_count, 3);
}

// --- auto-narrowing per object ---

#[test]
fn fresh_finding_row_is_trusted_and_green() {
    let finding = row("a11y:ai-review-finding-row-fresh-and-scoped");
    assert_eq!(
        finding.full_ready_claim,
        M5AiReviewA11yClaim::TrustedReviewSurface
    );
    assert_eq!(
        finding.effective_claim(),
        M5AiReviewA11yClaim::TrustedReviewSurface
    );
    assert!(finding.claim_narrow.is_none());
    assert_eq!(finding.status(), AiReviewAccessibilityStatus::Parity);
    assert!(finding.effective_claim().asserts_trusted_surface());
}

#[test]
fn stale_provider_finding_narrows_and_is_never_trusted() {
    let finding = row("a11y:ai-review-finding-row-provider-freshness-stale");
    assert_eq!(
        finding.effective_claim(),
        M5AiReviewA11yClaim::ProviderFreshnessUnverifiedProjection
    );
    assert!(!finding.effective_claim().asserts_trusted_surface());
    assert!(finding.trusted_honesty_holds());
    assert_eq!(
        finding.status(),
        AiReviewAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = finding.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5AiReviewAssistDowngradeTrigger::StaleFindingShownAsCurrent
    );
    assert_eq!(
        narrow.binding_dimension,
        M5AiReviewClaimDimension::ProviderFreshnessClarity
    );
}

#[test]
fn each_weak_object_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:ai-review-finding-row-provider-freshness-stale",
            M5AiReviewA11yClaim::ProviderFreshnessUnverifiedProjection,
        ),
        (
            "a11y:review-scope-selector-diff-drift-invalidates-findings",
            M5AiReviewA11yClaim::DiffScopeUnverifiedProjection,
        ),
        (
            "a11y:publish-to-review-sheet-publish-target-unavailable",
            M5AiReviewA11yClaim::PublishTargetUnverifiedProjection,
        ),
        (
            "a11y:resolution-memory-row-lifecycle-outside-publish-safe",
            M5AiReviewA11yClaim::FindingLifecycleUnverifiedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            AiReviewAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_every_weak_state() {
    use M5AiReviewConditionState as C;
    assert!(C::ProviderFreshnessStale.cannot_be_shown_trusted());
    assert!(C::DiffDriftInvalidatesFindings.cannot_be_shown_trusted());
    assert!(C::PublishTargetUnavailable.cannot_be_shown_trusted());
    assert!(C::LifecycleOutsidePublishSafe.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn live_resolution_memory_row_is_reviewable_and_green() {
    let resolution = row("a11y:resolution-memory-row-live-lifecycle");
    assert!(resolution.is_structure_heavy());
    assert!(resolution.has_non_visual_fallback());
    assert_eq!(
        resolution.effective_claim(),
        M5AiReviewA11yClaim::ReviewableReviewSurface
    );
    assert!(resolution.claim_narrow.is_none());
    assert_eq!(resolution.status(), AiReviewAccessibilityStatus::Parity);
}

#[test]
fn structure_heavy_publish_sheet_binds_a_structured_and_non_visual_path() {
    let publish = row("a11y:publish-to-review-sheet-publish-target-unavailable");
    assert!(publish.is_structure_heavy());
    assert!(publish.has_non_visual_fallback());
    assert!(publish
        .fallback_modalities
        .contains(&M5AiReviewFallbackModality::Structured));
}

#[test]
fn lost_publish_target_never_shown_as_provider_committed() {
    let publish = row("a11y:publish-to-review-sheet-publish-target-unavailable");
    assert!(!publish.effective_claim().asserts_trusted_surface());
    assert!(publish.trusted_honesty_holds());
    let narrow = publish.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5AiReviewAssistDowngradeTrigger::PublishModeUnstated
    );
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut finding = row("a11y:ai-review-finding-row-provider-freshness-stale");
    // Drop the narrow so the stale-provider state keeps a trusted claim.
    finding.claim_narrow = None;
    assert!(!finding.claim_is_honest());
    assert!(!finding.trusted_honesty_holds());
    assert_eq!(finding.status(), AiReviewAccessibilityStatus::Stranded);
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut resolution = row("a11y:resolution-memory-row-live-lifecycle");
    resolution.keyboard_reach = AiReviewNonVisualReachState::ViewOnlyTrap;
    assert!(!resolution.reaches_canonical_truth_via_at());
    assert_eq!(resolution.status(), AiReviewAccessibilityStatus::Stranded);
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut finding = row("a11y:ai-review-finding-row-fresh-and-scoped");
    finding.export_summary = AiReviewExportSummaryState::RequiresRawPayload;
    assert!(!finding.export_preserves_meaning());
    assert_eq!(finding.status(), AiReviewAccessibilityStatus::Stranded);
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut finding = row("a11y:ai-review-finding-row-provider-freshness-stale");
    finding.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!finding.claim_is_honest());
}

#[test]
fn dropping_an_object_reports_missing_coverage() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    // Drop both rows that key on the review scope selector to remove that object entirely.
    let rows: Vec<AiReviewAccessibilityRow> = packet
        .rows
        .into_iter()
        .filter(|r| r.object != M5AiReviewAssistObject::ReviewScopeSelector)
        .collect();
    let packet = AiReviewAccessibilityPacket::new(AiReviewAccessibilityPacketInput {
        packet_id: packet.packet_id,
        as_of: packet.as_of,
        matrix_ref: packet.matrix_ref,
        rows,
    });
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"missing_object_coverage"));
}

// --- forbidden material ---

#[test]
fn forbidden_material_in_export_is_rejected() {
    let mut packet = seeded_m5_ai_review_accessibility_parity_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_object_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_object() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let csv = packet.render_matrix_csv();
    // header + 6 rows + trailing newline
    assert_eq!(csv.lines().count(), 7);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_ai_review_accessibility_parity_export()
        .expect("checked M5 ai-review a11y export validates");
    assert_eq!(from_disk.packet_id, AI_REVIEW_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_ai_review_accessibility_parity_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_ai_review_accessibility_parity_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-ai-review-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_ai_review_accessibility_parity_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-ai-review-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-accessibility-parity/support_export.json"
    ));
    let packet: AiReviewAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_ai_review_accessibility_parity_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/review/m5-ai-review-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_ai_review_accessibility_parity_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_AI_REVIEW_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and fixtures from the
// seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_AI_REVIEW_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_ai_review_accessibility_parity_packet();
    assert!(
        packet.validate().is_empty(),
        "seed must validate before write"
    );

    let manifest = env!("CARGO_MANIFEST_DIR");
    let repo = Path::new(manifest).join("../..");

    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let md = packet.render_markdown_summary();

    let release_dir = repo.join("artifacts/review/m5-ai-review-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/review/m5-ai-review-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/review/m5-ai-review-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
