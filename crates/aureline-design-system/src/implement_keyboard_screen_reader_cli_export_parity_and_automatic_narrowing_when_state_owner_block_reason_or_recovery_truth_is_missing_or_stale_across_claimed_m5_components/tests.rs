//! Tests for the M05-938 shared-component-state accessibility fallback capstone: the honest
//! auto-narrowing logic, the per-family keyboard / screen-reader / CLI / export parity contract,
//! the missing-cause / missing-owner / missing-recovery-never-exact guarantee, no-loss
//! state-lineage integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> StateComponentAccessibilityRow {
    seeded_m5_state_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5SharedComponentStateFamily::ALL.len()
    );
    // Six rows cover the four families (interactive-state and the shared taxonomy are each
    // certified both live-green and narrowed-yellow).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5StateComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5StateComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_state_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5StateComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5ComponentStateConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5SharedComponentStateFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_exact_state_honesty() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_exact_state_honesty_holds);
}

// --- AC1: missing-cause / missing-owner / missing-recovery / stale can no longer keep an exact
// label ---

#[test]
fn stated_taxonomy_row_is_reviewable_and_green() {
    let taxonomy = row("a11y:shared-state-taxonomy-reviewable");
    assert_eq!(
        taxonomy.effective_claim(),
        M5StateComponentClaim::ReviewableStateGuidance
    );
    assert!(taxonomy.claim_narrow.is_none());
    assert_eq!(taxonomy.status(), StateComponentAccessibilityStatus::Parity);
    assert!(taxonomy.effective_claim().asserts_trustworthy_state());
    assert!(!taxonomy.effective_claim().asserts_exact_state());
}

#[test]
fn live_interactive_row_is_exact_state_and_green() {
    let control = row("a11y:interactive-state-live");
    assert_eq!(
        control.full_state_claim,
        M5StateComponentClaim::ExactStateTruth
    );
    assert_eq!(
        control.effective_claim(),
        M5StateComponentClaim::ExactStateTruth
    );
    assert!(control.claim_narrow.is_none());
    assert_eq!(control.status(), StateComponentAccessibilityStatus::Parity);
    assert!(control.effective_claim().asserts_exact_state());
}

#[test]
fn cause_unresolved_narrows_to_cause_narrowed_projection() {
    let taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    assert_eq!(
        taxonomy.effective_claim(),
        M5StateComponentClaim::CauseNarrowedProjection
    );
    assert!(!taxonomy.effective_claim().asserts_exact_state());
    let narrow = taxonomy.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComponentStateDowngradeTrigger::StateCauseUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5StateComponentClaimDimension::StateSemantics
    );
    assert!(taxonomy.claim_is_honest());
    assert!(taxonomy.exact_state_honesty_holds());
}

#[test]
fn owner_unresolved_narrows_to_owner_narrowed_projection() {
    let selection = row("a11y:selection-or-lock-state-owner-unresolved");
    assert_eq!(
        selection.effective_claim(),
        M5StateComponentClaim::OwnerNarrowedProjection
    );
    let narrow = selection.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComponentStateDowngradeTrigger::LockOwnerMasked
    );
    assert_eq!(
        narrow.binding_dimension,
        M5StateComponentClaimDimension::SelectionOrLockState
    );
    assert!(selection.claim_is_honest());
    assert!(selection.exact_state_honesty_holds());
}

#[test]
fn recovery_unavailable_narrows_to_recovery_narrowed_projection() {
    let degraded = row("a11y:degraded-state-application-recovery-unavailable");
    assert_eq!(
        degraded.effective_claim(),
        M5StateComponentClaim::RecoveryNarrowedProjection
    );
    let narrow = degraded.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5ComponentStateDowngradeTrigger::ConsequenceOrRecoveryOmitted
    );
    assert_eq!(
        narrow.binding_dimension,
        M5StateComponentClaimDimension::RecoveryReadiness
    );
    assert!(degraded.claim_is_honest());
    assert!(degraded.exact_state_honesty_holds());
}

#[test]
fn proof_stale_narrows_to_stale_proof_projection() {
    let control = row("a11y:interactive-state-proof-stale");
    assert_eq!(
        control.effective_claim(),
        M5StateComponentClaim::StaleProofProjection
    );
    assert!(!control.effective_claim().asserts_trustworthy_state());
    let narrow = control.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(narrow.trigger, M5ComponentStateDowngradeTrigger::ProofStale);
    assert_eq!(
        narrow.binding_dimension,
        M5StateComponentClaimDimension::InteractionState
    );
    assert!(control.claim_is_honest());
    // A stale proof is a freshness reduction, not a missing-truth overstatement.
    assert!(control.exact_state_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a cause-unresolved state claiming
    // ExactStateTruth.
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    taxonomy.claim_narrow = None;
    assert!(!taxonomy.claim_is_honest());
    assert_eq!(
        taxonomy.status(),
        StateComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn missing_state_truth_shown_as_exact_is_rejected() {
    // A cause-unresolved row whose narrow claims ExactStateTruth violates exact-state honesty.
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    if let Some(narrow) = taxonomy.claim_narrow.as_mut() {
        narrow.narrowed_to = M5StateComponentClaim::ExactStateTruth;
    }
    assert!(!taxonomy.exact_state_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_state_component_a11y_fallback_packet();
        packet.rows[2] = taxonomy;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        StateComponentAccessibilityViolation::MissingStateTruthShownAsExact { .. }
    )));
}

#[test]
fn exact_state_honesty_unproven_when_no_missing_truth_row() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    // Drop the three missing-cause / missing-owner / missing-recovery rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:shared-state-taxonomy-cause-unresolved"
            && r.row_id != "a11y:selection-or-lock-state-owner-unresolved"
            && r.row_id != "a11y:degraded-state-application-recovery-unavailable"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        StateComponentAccessibilityViolation::ExactStateHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_exact_row_is_rejected() {
    let mut control = row("a11y:interactive-state-live");
    control.claim_narrow = Some(StateComponentClaimAutoNarrow {
        narrowed_to: M5StateComponentClaim::StaleProofProjection,
        binding_dimension: M5StateComponentClaimDimension::InteractionState,
        trigger: M5ComponentStateDowngradeTrigger::ProofStale,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!control.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    if let Some(narrow) = taxonomy.claim_narrow.as_mut() {
        narrow.binding_dimension = M5StateComponentClaimDimension::RecoveryReadiness;
    }
    assert!(!taxonomy.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    if let Some(narrow) = taxonomy.claim_narrow.as_mut() {
        narrow.trigger = M5ComponentStateDowngradeTrigger::ProofStale;
    }
    assert!(!taxonomy.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    if let Some(narrow) = taxonomy.claim_narrow.as_mut() {
        narrow.narrowed_label = "unresolved".to_owned();
    }
    assert!(!taxonomy.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5StateComponentClaim as S;
    use M5StateComponentConditionState as C;
    assert_eq!(C::LiveExactState.permitted_ceiling(), S::ExactStateTruth);
    assert_eq!(
        C::StateCauseUnresolved.permitted_ceiling(),
        S::CauseNarrowedProjection
    );
    assert_eq!(
        C::LockOwnerUnresolved.permitted_ceiling(),
        S::OwnerNarrowedProjection
    );
    assert_eq!(
        C::RecoveryUnavailable.permitted_ceiling(),
        S::RecoveryNarrowedProjection
    );
    assert_eq!(C::ProofStale.permitted_ceiling(), S::StaleProofProjection);
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5ComponentStateDowngradeTrigger as T;
    use M5StateComponentConditionState as C;
    assert_eq!(
        C::StateCauseUnresolved.default_trigger(),
        T::StateCauseUnstated
    );
    assert_eq!(C::LockOwnerUnresolved.default_trigger(), T::LockOwnerMasked);
    assert_eq!(
        C::RecoveryUnavailable.default_trigger(),
        T::ConsequenceOrRecoveryOmitted
    );
    assert_eq!(C::ProofStale.default_trigger(), T::ProofStale);
}

#[test]
fn missing_state_truth_states_are_flagged() {
    use M5StateComponentConditionState as C;
    assert!(C::StateCauseUnresolved.is_missing_state_truth());
    assert!(C::LockOwnerUnresolved.is_missing_state_truth());
    assert!(C::RecoveryUnavailable.is_missing_state_truth());
    // A stale proof is a freshness reduction, not a missing-truth overstatement.
    assert!(!C::ProofStale.is_missing_state_truth());
    assert!(!C::LiveExactState.is_missing_state_truth());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_state_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_selection_binds_a_non_visual_fallback() {
    let selection = row("a11y:selection-or-lock-state-owner-unresolved");
    assert!(selection.is_hierarchy_heavy());
    assert!(selection.has_non_visual_fallback());
    assert!(selection
        .fallback_modalities
        .contains(&M5StateComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut control = row("a11y:interactive-state-live");
    control.keyboard_reach = StateComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!control.reaches_canonical_truth_via_at());
    assert_eq!(
        control.status(),
        StateComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_state_context_ref_strands_a_row() {
    let mut control = row("a11y:interactive-state-live");
    control.state_context_ref = "  ".to_owned();
    assert!(!control.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut control = row("a11y:interactive-state-live");
    control.export_summary = StateComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!control.export_preserves_meaning());
    assert_eq!(
        control.status(),
        StateComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut control = row("a11y:interactive-state-live");
    control.copy_export.formats.retain(|f| f != "markdown");
    assert!(!control.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut degraded = row("a11y:degraded-state-application-recovery-unavailable");
    degraded.lineage_preserved = false;
    assert!(!degraded.preserves_lineage_continuity());
    assert_eq!(
        degraded.status(),
        StateComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut degraded = row("a11y:degraded-state-application-recovery-unavailable");
    if let Some(narrow) = degraded.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!degraded.preserves_lineage_continuity());
    assert!(!degraded.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    taxonomy.narrowing_disclosures.clear();
    assert!(!taxonomy.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    taxonomy.narrowing_disclosures[0].state =
        StateComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!taxonomy.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut taxonomy = row("a11y:shared-state-taxonomy-cause-unresolved");
    taxonomy.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!taxonomy.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut control = row("a11y:interactive-state-live");
    control
        .required_labels
        .retain(|l| *l != M5ComponentStateRequiredLabel::Identity);
    assert!(!control.preserves_mandatory_labels());
    assert_eq!(
        control.status(),
        StateComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:degraded-state-application-recovery-unavailable");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        StateComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ComponentStateConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        StateComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, StateComponentAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, StateComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_state_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        StateComponentAccessibilityViolation::RawStateMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:shared-state-taxonomy-cause-unresolved").chip_tokens();
    assert!(chip.contains("family=shared_component_state_taxonomy"));
    assert!(chip.contains("effective_claim=cause_narrowed_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        STATE_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        STATE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_state_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_state_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_state_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_state_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-state-taxonomy-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_state_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-state-taxonomy-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_STATE_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-design-system generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_STATE_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_state_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-shared-state-taxonomy-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest)
            .join("../../artifacts/release/m5-shared-state-taxonomy-accessibility-fallback.md"),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-shared-state-taxonomy-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
