//! Tests for the M05-1010 learning-component accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family parity contract, the
//! stale/uncited/unprovable/blocked-never-exact guarantee, no-loss learning-lineage integrity,
//! and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> LearningComponentAccessibilityRow {
    seeded_m5_learning_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5LearningComponentFamily::ALL.len()
    );
    // Eight rows cover the six families (the learning-mode toggle is certified live-green and
    // paused-yellow; the safe-explanation banner is certified explain-only-green and
    // unprovable-boundary-yellow).
    assert_eq!(packet.rows.len(), 8);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5LearningComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5LearningComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_learning_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5LearningComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5LearningConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_six_yellow_zero_red() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 6);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 8);
    assert_eq!(
        packet.summary.family_count,
        M5LearningComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_exact_learning_honesty() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_exact_learning_honesty_holds);
}

// --- AC1: paused / snoozed / stale / uncited / unprovable / blocked can no longer keep exact ---

#[test]
fn explain_only_banner_is_reviewable_and_green() {
    let banner = row("a11y:safe-explanation-banner-explain-only");
    assert_eq!(
        banner.effective_claim(),
        M5LearningComponentClaim::ReviewableGuidance
    );
    assert!(banner.claim_narrow.is_none());
    assert_eq!(
        banner.status(),
        LearningComponentAccessibilityStatus::Parity
    );
    assert!(banner.effective_claim().asserts_trustworthy_learning());
    assert!(!banner.effective_claim().asserts_exact_learning());
}

#[test]
fn live_learning_mode_row_is_exact_learning_and_green() {
    let mode = row("a11y:learning-mode-toggle-live");
    assert_eq!(
        mode.full_learning_claim,
        M5LearningComponentClaim::ExactLearning
    );
    assert_eq!(
        mode.effective_claim(),
        M5LearningComponentClaim::ExactLearning
    );
    assert!(mode.claim_narrow.is_none());
    assert_eq!(mode.status(), LearningComponentAccessibilityStatus::Parity);
    assert!(mode.effective_claim().asserts_exact_learning());
}

#[test]
fn paused_mode_narrows_to_paused_mode_projection() {
    let mode = row("a11y:learning-mode-toggle-paused");
    assert_eq!(
        mode.effective_claim(),
        M5LearningComponentClaim::PausedModeProjection
    );
    assert!(!mode.effective_claim().asserts_exact_learning());
    let narrow = mode.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::LearningModeStateUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5LearningComponentClaimDimension::LearningModeDelivery
    );
    assert!(mode.claim_is_honest());
    // A paused mode is a delivery state, not an exactness overstatement.
    assert!(mode.exact_learning_honesty_holds());
}

#[test]
fn snoozed_tip_narrows_to_snoozed_tip_projection() {
    let tip = row("a11y:tip-card-snoozed");
    assert_eq!(
        tip.effective_claim(),
        M5LearningComponentClaim::SnoozedTipProjection
    );
    assert!(!tip.effective_claim().asserts_exact_learning());
    let narrow = tip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::TipCommandBindingUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5LearningComponentClaimDimension::TipDelivery
    );
    assert!(tip.claim_is_honest());
}

#[test]
fn stale_pack_narrows_to_stale_pack_projection() {
    let ex = row("a11y:guided-exercise-step-stale-pack");
    assert_eq!(
        ex.effective_claim(),
        M5LearningComponentClaim::StalePackProjection
    );
    let narrow = ex.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::ExerciseStepStateUnstated
    );
    assert!(ex.claim_is_honest());
    // A drifted pack must never be shown as exact learning.
    assert!(ex.exact_learning_honesty_holds());
}

#[test]
fn stale_citation_narrows_to_uncited_glossary_projection() {
    let gloss = row("a11y:glossary-chip-card-citation-stale");
    assert_eq!(
        gloss.effective_claim(),
        M5LearningComponentClaim::UncitedGlossaryProjection
    );
    let narrow = gloss.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::GlossaryCitationSevered
    );
    assert!(gloss.claim_is_honest());
    assert!(gloss.exact_learning_honesty_holds());
}

#[test]
fn unprovable_boundary_narrows_to_unprovable_boundary_projection() {
    let banner = row("a11y:safe-explanation-banner-unprovable-boundary");
    assert_eq!(
        banner.effective_claim(),
        M5LearningComponentClaim::UnprovableBoundaryProjection
    );
    let narrow = banner.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::ExplanationApplyBoundaryUnstated
    );
    assert!(banner.claim_is_honest());
    assert!(banner.exact_learning_honesty_holds());
}

#[test]
fn blocked_progress_narrows_to_blocked_progress_projection() {
    let marker = row("a11y:progress-marker-portability-blocked");
    assert_eq!(
        marker.effective_claim(),
        M5LearningComponentClaim::BlockedProgressProjection
    );
    assert!(!marker.effective_claim().asserts_trustworthy_learning());
    let narrow = marker.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5LearningDowngradeTrigger::ProgressOwnershipUnstated
    );
    assert!(marker.claim_is_honest());
    assert!(marker.exact_learning_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a snoozed tip claiming
    // ExactLearning.
    let mut tip = row("a11y:tip-card-snoozed");
    tip.claim_narrow = None;
    assert!(!tip.claim_is_honest());
    assert_eq!(tip.status(), LearningComponentAccessibilityStatus::Stranded);
}

#[test]
fn unprovable_state_shown_as_exact_is_rejected() {
    // A stale-citation row whose narrow claims ExactLearning violates exact-learning honesty.
    let mut gloss = row("a11y:glossary-chip-card-citation-stale");
    if let Some(narrow) = gloss.claim_narrow.as_mut() {
        narrow.narrowed_to = M5LearningComponentClaim::ExactLearning;
    }
    assert!(!gloss.exact_learning_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
        let idx = packet
            .rows
            .iter()
            .position(|r| r.row_id == "a11y:glossary-chip-card-citation-stale")
            .expect("row present");
        packet.rows[idx] = gloss;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::UnprovableStateShownAsExact { .. }
    )));
}

#[test]
fn exact_learning_honesty_unproven_when_no_unprovable_row() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    // Drop the four stale / uncited / unprovable / blocked rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:guided-exercise-step-stale-pack"
            && r.row_id != "a11y:glossary-chip-card-citation-stale"
            && r.row_id != "a11y:safe-explanation-banner-unprovable-boundary"
            && r.row_id != "a11y:progress-marker-portability-blocked"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::ExactLearningHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_exact_row_is_rejected() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.claim_narrow = Some(LearningComponentClaimAutoNarrow {
        narrowed_to: M5LearningComponentClaim::BlockedProgressProjection,
        binding_dimension: M5LearningComponentClaimDimension::ProgressPortability,
        trigger: M5LearningDowngradeTrigger::ProgressOwnershipUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!mode.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut tip = row("a11y:tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.binding_dimension = M5LearningComponentClaimDimension::CitationFreshness;
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut tip = row("a11y:tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.trigger = M5LearningDowngradeTrigger::GlossaryCitationSevered;
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut tip = row("a11y:tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.narrowed_label = "snoozed".to_owned();
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5LearningComponentClaim as S;
    use M5LearningComponentConditionState as C;
    assert_eq!(C::LiveExactLearning.permitted_ceiling(), S::ExactLearning);
    assert_eq!(
        C::LearningModePaused.permitted_ceiling(),
        S::PausedModeProjection
    );
    assert_eq!(C::TipSnoozed.permitted_ceiling(), S::SnoozedTipProjection);
    assert_eq!(
        C::ExercisePackStale.permitted_ceiling(),
        S::StalePackProjection
    );
    assert_eq!(
        C::CitationStale.permitted_ceiling(),
        S::UncitedGlossaryProjection
    );
    assert_eq!(
        C::ExplainDoUnprovable.permitted_ceiling(),
        S::UnprovableBoundaryProjection
    );
    assert_eq!(
        C::ProgressPortabilityBlocked.permitted_ceiling(),
        S::BlockedProgressProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5LearningComponentConditionState as C;
    use M5LearningDowngradeTrigger as T;
    assert_eq!(
        C::LearningModePaused.default_trigger(),
        T::LearningModeStateUnstated
    );
    assert_eq!(
        C::TipSnoozed.default_trigger(),
        T::TipCommandBindingUnstated
    );
    assert_eq!(
        C::ExercisePackStale.default_trigger(),
        T::ExerciseStepStateUnstated
    );
    assert_eq!(
        C::CitationStale.default_trigger(),
        T::GlossaryCitationSevered
    );
    assert_eq!(
        C::ExplainDoUnprovable.default_trigger(),
        T::ExplanationApplyBoundaryUnstated
    );
    assert_eq!(
        C::ProgressPortabilityBlocked.default_trigger(),
        T::ProgressOwnershipUnstated
    );
}

#[test]
fn cannot_be_proven_states_are_flagged() {
    use M5LearningComponentConditionState as C;
    assert!(C::ExercisePackStale.cannot_be_proven_exact());
    assert!(C::CitationStale.cannot_be_proven_exact());
    assert!(C::ExplainDoUnprovable.cannot_be_proven_exact());
    assert!(C::ProgressPortabilityBlocked.cannot_be_proven_exact());
    assert!(!C::LearningModePaused.cannot_be_proven_exact());
    assert!(!C::TipSnoozed.cannot_be_proven_exact());
    assert!(!C::LiveExactLearning.cannot_be_proven_exact());
}

// --- AC2: accessibility / localization / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_learning_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_guided_exercise_binds_a_non_visual_fallback() {
    let ex = row("a11y:guided-exercise-step-stale-pack");
    assert!(ex.is_hierarchy_heavy());
    assert!(ex.has_non_visual_fallback());
    assert!(ex
        .fallback_modalities
        .contains(&M5LearningComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.keyboard_reach = LearningComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!mode.reaches_canonical_truth_via_at());
    assert_eq!(
        mode.status(),
        LearningComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn localization_trap_strands_a_row() {
    let mut gloss = row("a11y:glossary-chip-card-citation-stale");
    gloss.localization_reach = LearningComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!gloss.reaches_canonical_truth_via_at());
    assert_eq!(
        gloss.status(),
        LearningComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_learning_context_ref_strands_a_row() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.learning_context_ref = "  ".to_owned();
    assert!(!mode.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.export_summary = LearningComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!mode.export_preserves_meaning());
    assert_eq!(
        mode.status(),
        LearningComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.copy_export.formats.retain(|f| f != "markdown");
    assert!(!mode.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut gloss = row("a11y:glossary-chip-card-citation-stale");
    gloss.lineage_preserved = false;
    assert!(!gloss.preserves_lineage_continuity());
    assert_eq!(
        gloss.status(),
        LearningComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut gloss = row("a11y:glossary-chip-card-citation-stale");
    if let Some(narrow) = gloss.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!gloss.preserves_lineage_continuity());
    assert!(!gloss.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut tip = row("a11y:tip-card-snoozed");
    tip.narrowing_disclosures.clear();
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut tip = row("a11y:tip-card-snoozed");
    tip.narrowing_disclosures[0].state = LearningComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut tip = row("a11y:tip-card-snoozed");
    tip.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut mode = row("a11y:learning-mode-toggle-live");
    mode.required_labels
        .retain(|l| *l != M5LearningRequiredLabel::Identity);
    assert!(!mode.preserves_mandatory_labels());
    assert_eq!(
        mode.status(),
        LearningComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.component_family != M5LearningComponentFamily::ProgressMarker);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5LearningConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, LearningComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_learning_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        LearningComponentAccessibilityViolation::RawLearningMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:tip-card-snoozed").chip_tokens();
    assert!(chip.contains("family=tip_card"));
    assert!(chip.contains("effective_claim=snoozed_tip_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        LEARNING_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        LEARNING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_learning_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_learning_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_learning_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_learning_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_LEARNING_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-learning generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_LEARNING_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_learning_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-learning-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-learning-component-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures =
        Path::new(manifest).join("../../fixtures/ui/m5-learning-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
