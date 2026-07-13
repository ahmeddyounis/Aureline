//! Tests for the M05-1154 motion-layer-iconography accessibility parity capstone: the honest auto-narrowing
//! logic, the per-family parity contract, the weakened-never-trusted guarantee, no-loss motion / scrim /
//! layer / icon / illustration truth integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> VisualInteractionAccessibilityRow {
    seeded_m5_motion_layer_iconography_a11y_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5VisualInteractionFamily::ALL.len()
    );
    // Seven rows cover the seven families one-to-one (one certified fully green — the iconography family
    // whose action icons stay semantic and labeled — the other six narrowed-yellow: five auto-narrowed
    // claims plus the reviewable layer-order surface whose high-zoom traversal narrows to a disclosed reflow
    // walk).
    assert_eq!(packet.rows.len(), 7);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5VisualInteractionClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let states = packet.exercised_condition_states();
    for state in M5VisualInteractionConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5VisualInteractionA11yClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5VisualInteractionConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_one_green_six_yellow_zero_red() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    assert_eq!(packet.summary.green_count, 1);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 7);
    assert_eq!(
        packet.summary.family_count,
        M5VisualInteractionFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_truth_and_trusted_honesty() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
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
fn semantic_labeled_iconography_is_trusted_and_green() {
    let icon = row("a11y:iconography-semantic-labeled-icons");
    assert_eq!(
        icon.full_ready_claim,
        M5VisualInteractionA11yClaim::TrustedInteractionSurface
    );
    assert_eq!(
        icon.effective_claim(),
        M5VisualInteractionA11yClaim::TrustedInteractionSurface
    );
    assert!(icon.claim_narrow.is_none());
    assert_eq!(icon.status(), VisualInteractionAccessibilityStatus::Parity);
    assert!(icon.effective_claim().asserts_trusted_surface());
}

#[test]
fn stale_motion_token_narrows_and_is_never_trusted() {
    let motion = row("a11y:motion-token-protected-path-timing-stale");
    assert_eq!(
        motion.effective_claim(),
        M5VisualInteractionA11yClaim::MotionTimingUnverifiedProjection
    );
    assert!(!motion.effective_claim().asserts_trusted_surface());
    assert!(motion.trusted_honesty_holds());
    assert_eq!(
        motion.status(),
        VisualInteractionAccessibilityStatus::NarrowedDisclosed
    );
    let narrow = motion.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5VisualInteractionDowngradeTrigger::MotionDelayedProtectedInput
    );
    assert_eq!(
        narrow.binding_dimension,
        M5VisualInteractionClaimDimension::MotionTimingClarity
    );
}

#[test]
fn each_weak_family_narrows_to_its_permitted_ceiling() {
    for (id, expected) in [
        (
            "a11y:motion-token-protected-path-timing-stale",
            M5VisualInteractionA11yClaim::MotionTimingUnverifiedProjection,
        ),
        (
            "a11y:reduced-motion-clamp-unconfirmed",
            M5VisualInteractionA11yClaim::ReducedMotionClampUnverifiedProjection,
        ),
        (
            "a11y:opacity-scrim-orientation-contrast-unconfirmed",
            M5VisualInteractionA11yClaim::ScrimOrientationUnverifiedProjection,
        ),
        (
            "a11y:portal-ownership-owning-surface-unconfirmed",
            M5VisualInteractionA11yClaim::PortalOwnershipUnverifiedProjection,
        ),
        (
            "a11y:illustration-boundary-disclosed-partial",
            M5VisualInteractionA11yClaim::IllustrationBoundaryDisclosedProjection,
        ),
    ] {
        let r = row(id);
        assert_eq!(r.effective_claim(), expected, "row {id}");
        assert_eq!(r.permitted_claim(), expected, "row {id} permitted");
        assert!(r.claim_is_honest(), "row {id} honest");
        assert_eq!(
            r.status(),
            VisualInteractionAccessibilityStatus::NarrowedDisclosed,
            "row {id} yellow"
        );
    }
}

#[test]
fn cannot_be_shown_trusted_flags_exactly_the_overclaim_states() {
    use M5VisualInteractionConditionState as C;
    assert!(C::MotionTimingEvidenceStale.cannot_be_shown_trusted());
    assert!(C::ReducedMotionSafetyUnconfirmed.cannot_be_shown_trusted());
    assert!(C::ScrimContrastUnconfirmed.cannot_be_shown_trusted());
    assert!(C::PortalOwnershipUnconfirmed.cannot_be_shown_trusted());
    // An honest disclosed-absence operation (a partial secondary-illustration boundary) is not a truth
    // overstatement.
    assert!(!C::IllustrationBoundaryDisclosedPartial.cannot_be_shown_trusted());
    assert!(!C::FullyQualified.cannot_be_shown_trusted());
}

#[test]
fn layer_order_interaction_is_reviewable_and_yellow() {
    let layer = row("a11y:layer-order-shared-z-order-model-stable");
    assert!(layer.has_non_visual_fallback());
    assert_eq!(
        layer.effective_claim(),
        M5VisualInteractionA11yClaim::ReviewableInteractionSurface
    );
    assert!(layer.claim_narrow.is_none());
    assert_eq!(
        layer.status(),
        VisualInteractionAccessibilityStatus::NarrowedDisclosed
    );
}

#[test]
fn structure_heavy_illustration_binds_a_structured_and_non_visual_path() {
    let illustration = row("a11y:illustration-boundary-disclosed-partial");
    assert!(illustration.is_structure_heavy());
    assert!(illustration.has_non_visual_fallback());
    assert!(illustration
        .fallback_modalities
        .contains(&M5VisualInteractionFallbackModality::Structured));
}

#[test]
fn unconfirmed_scrim_is_never_shown_as_trusted() {
    let scrim = row("a11y:opacity-scrim-orientation-contrast-unconfirmed");
    assert!(!scrim.effective_claim().asserts_trusted_surface());
    assert!(scrim.trusted_honesty_holds());
    let narrow = scrim.claim_narrow.as_ref().unwrap();
    assert_eq!(
        narrow.trigger,
        M5VisualInteractionDowngradeTrigger::ScrimErasedOrientationOrContrast
    );
}

#[test]
fn power_thermal_reach_is_exercised_as_a_disclosed_reduction() {
    let motion = row("a11y:motion-token-protected-path-timing-stale");
    assert!(motion.power_thermal_reach.is_disclosed_reduction());
    assert!(motion.power_thermal_reach.never_traps());
}

// --- negative / red-detection cases ---

#[test]
fn over_asserting_a_weak_state_is_stranded() {
    let mut motion = row("a11y:motion-token-protected-path-timing-stale");
    // Drop the narrow so the stale-timing state keeps a trusted claim.
    motion.claim_narrow = None;
    assert!(!motion.claim_is_honest());
    assert!(!motion.trusted_honesty_holds());
    assert_eq!(
        motion.status(),
        VisualInteractionAccessibilityStatus::Stranded
    );
}

#[test]
fn a_view_only_hover_trap_is_stranded() {
    let mut layer = row("a11y:layer-order-shared-z-order-model-stable");
    layer.keyboard_reach = VisualInteractionNonVisualReachState::ViewOnlyTrap;
    assert!(!layer.reaches_canonical_truth_via_at());
    assert_eq!(
        layer.status(),
        VisualInteractionAccessibilityStatus::Stranded
    );
}

#[test]
fn a_power_thermal_trap_is_stranded() {
    let mut icon = row("a11y:iconography-semantic-labeled-icons");
    icon.power_thermal_reach = VisualInteractionNonVisualReachState::ViewOnlyTrap;
    assert!(!icon.reaches_canonical_truth_via_at());
    assert_eq!(
        icon.status(),
        VisualInteractionAccessibilityStatus::Stranded
    );
}

#[test]
fn a_raw_payload_only_export_is_stranded() {
    let mut icon = row("a11y:iconography-semantic-labeled-icons");
    icon.export_summary = VisualInteractionExportSummaryState::RequiresRawPayload;
    assert!(!icon.export_preserves_meaning());
    assert_eq!(
        icon.status(),
        VisualInteractionAccessibilityStatus::Stranded
    );
}

#[test]
fn a_generic_narrow_label_is_dishonest() {
    let mut motion = row("a11y:motion-token-protected-path-timing-stale");
    motion.claim_narrow.as_mut().unwrap().narrowed_label = "stale".to_owned();
    assert!(!motion.claim_is_honest());
}

#[test]
fn dropping_a_family_reports_missing_coverage() {
    let mut packet = seeded_m5_motion_layer_iconography_a11y_packet();
    packet.rows.pop();
    let mut packet =
        VisualInteractionAccessibilityPacket::new(VisualInteractionAccessibilityPacketInput {
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
    let mut packet = seeded_m5_motion_layer_iconography_a11y_packet();
    packet.rows[0]
        .copy_export
        .export_fields
        .push("bearer abc".to_owned());
    let tokens: Vec<&str> = packet.validate().iter().map(|v| v.as_str()).collect();
    assert!(tokens.contains(&"raw_interaction_material_in_export"));
}

// --- rendering ---

#[test]
fn csv_has_a_row_per_certified_interaction() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let csv = packet.render_matrix_csv();
    // header + 7 rows + trailing newline
    assert_eq!(csv.lines().count(), 8);
    for row in &packet.rows {
        assert!(csv.contains(&row.row_id));
    }
}

#[test]
fn markdown_summary_lists_every_row_and_narrowing() {
    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
    let md = packet.render_markdown_summary();
    for row in &packet.rows {
        assert!(md.contains(&row.row_id));
    }
    assert!(md.contains("Auto-narrow"));
}

// --- byte-lock against the checked-in artifacts ---

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_m5_motion_layer_iconography_a11y_export()
        .expect("checked M5 motion-layer-iconography a11y export validates");
    assert_eq!(from_disk.packet_id, MOTION_LAYER_ICONOGRAPHY_A11Y_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_motion_layer_iconography_a11y_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_matrix_csv_matches_builder() {
    let expected = seeded_m5_motion_layer_iconography_a11y_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-motion-layer-iconography-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        on_disk, expected,
        "checked matrix CSV drifted from the builder"
    );
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_motion_layer_iconography_a11y_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-motion-layer-iconography-accessibility-parity.md"
    ));
    assert_eq!(on_disk, expected, "checked report drifted from the builder");
}

#[test]
fn checked_fixtures_mirror_the_release_artifacts() {
    let fixture_export = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-motion-layer-iconography-accessibility-parity/support_export.json"
    ));
    let packet: VisualInteractionAccessibilityPacket =
        serde_json::from_str(fixture_export).expect("fixture export parses");
    assert!(packet.validate().is_empty());
    assert_eq!(packet, seeded_m5_motion_layer_iconography_a11y_packet());

    let fixture_csv = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-motion-layer-iconography-accessibility-parity/matrix.csv"
    ));
    assert_eq!(
        fixture_csv,
        seeded_m5_motion_layer_iconography_a11y_packet().render_matrix_csv()
    );
}

// --- gated artifact regeneration ---
//
// Set `GEN_MOTION_LAYER_ICONOGRAPHY_A11Y_ARTIFACTS=1` to (re)write the checked-in release artifacts and
// fixtures from the seed builder. Off by default so a normal `cargo test` never mutates the tree.
#[test]
fn regenerate_checked_artifacts_when_requested() {
    if std::env::var("GEN_MOTION_LAYER_ICONOGRAPHY_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_motion_layer_iconography_a11y_packet();
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
        repo.join("artifacts/release/m5-motion-layer-iconography-accessibility-parity");
    fs::create_dir_all(&release_dir).expect("create release dir");
    fs::write(release_dir.join("support_export.json"), &json).expect("write release export");
    fs::write(release_dir.join("matrix.csv"), &csv).expect("write release csv");
    fs::write(
        repo.join("artifacts/release/m5-motion-layer-iconography-accessibility-parity.md"),
        &md,
    )
    .expect("write release report");

    let fixture_dir = repo.join("fixtures/ui/m5-motion-layer-iconography-accessibility-parity");
    fs::create_dir_all(&fixture_dir).expect("create fixture dir");
    fs::write(fixture_dir.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixture_dir.join("matrix.csv"), &csv).expect("write fixture csv");
}
