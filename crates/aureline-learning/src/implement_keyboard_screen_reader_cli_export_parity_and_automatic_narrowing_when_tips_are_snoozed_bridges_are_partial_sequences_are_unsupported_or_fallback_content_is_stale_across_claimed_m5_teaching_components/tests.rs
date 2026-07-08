//! Tests for the M05-930 contextual-teaching component accessibility fallback capstone: the
//! honest auto-narrowing logic, the per-family parity contract, the
//! partial/unsupported/stale-never-exact guarantee, no-loss teaching-lineage integrity, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> TeachingComponentAccessibilityRow {
    seeded_m5_teaching_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5ContextualTeachingComponentFamily::ALL.len()
    );
    // Six rows cover the five families (the contextual-tip card is certified both live-green and
    // snoozed-yellow).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5TeachingComponentClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5TeachingComponentConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_teaching_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5TeachingComponentClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5TeachingConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5ContextualTeachingComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_and_exact_teaching_honesty() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
    assert!(packet.summary.all_exact_teaching_honesty_holds);
}

// --- AC1: snoozed / partial / unsupported / stale can no longer keep an exact label ---

#[test]
fn stated_why_unavailable_row_is_reviewable_and_green() {
    let why = row("a11y:why-unavailable-explanation-row");
    assert_eq!(
        why.effective_claim(),
        M5TeachingComponentClaim::ReviewableGuidance
    );
    assert!(why.claim_narrow.is_none());
    assert_eq!(why.status(), TeachingComponentAccessibilityStatus::Parity);
    assert!(why.effective_claim().asserts_trustworthy_teaching());
    assert!(!why.effective_claim().asserts_exact_teaching());
}

#[test]
fn live_tip_row_is_exact_teaching_and_green() {
    let tip = row("a11y:contextual-tip-card-live");
    assert_eq!(
        tip.full_teaching_claim,
        M5TeachingComponentClaim::ExactTeaching
    );
    assert_eq!(
        tip.effective_claim(),
        M5TeachingComponentClaim::ExactTeaching
    );
    assert!(tip.claim_narrow.is_none());
    assert_eq!(tip.status(), TeachingComponentAccessibilityStatus::Parity);
    assert!(tip.effective_claim().asserts_exact_teaching());
}

#[test]
fn snoozed_tip_narrows_to_snoozed_tip_projection() {
    let tip = row("a11y:contextual-tip-card-snoozed");
    assert_eq!(
        tip.effective_claim(),
        M5TeachingComponentClaim::SnoozedTipProjection
    );
    assert!(!tip.effective_claim().asserts_exact_teaching());
    let narrow = tip.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TeachingDowngradeTrigger::TipCommandBindingUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5TeachingComponentClaimDimension::TipDelivery
    );
    assert!(tip.claim_is_honest());
}

#[test]
fn partial_bridge_narrows_to_partial_bridge_projection() {
    let bridge = row("a11y:migration-bridge-card-partial");
    assert_eq!(
        bridge.effective_claim(),
        M5TeachingComponentClaim::PartialBridgeProjection
    );
    let narrow = bridge.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TeachingDowngradeTrigger::MigrationMappingUnstated
    );
    assert!(bridge.claim_is_honest());
    // A partial state must never be shown as exact teaching.
    assert!(bridge.exact_teaching_honesty_holds());
}

#[test]
fn unsupported_sequence_narrows_to_unsupported_sequence_projection() {
    let seq = row("a11y:sequence-help-strip-unsupported");
    assert_eq!(
        seq.effective_claim(),
        M5TeachingComponentClaim::UnsupportedSequenceProjection
    );
    let narrow = seq.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TeachingDowngradeTrigger::SequenceHelpStateUnstated
    );
    assert!(seq.claim_is_honest());
    assert!(seq.exact_teaching_honesty_holds());
}

#[test]
fn stale_fallback_narrows_to_stale_fallback_projection() {
    let fallback = row("a11y:source-language-fallback-stale");
    assert_eq!(
        fallback.effective_claim(),
        M5TeachingComponentClaim::StaleFallbackProjection
    );
    assert!(!fallback.effective_claim().asserts_trustworthy_teaching());
    let narrow = fallback.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated
    );
    assert!(fallback.claim_is_honest());
    assert!(fallback.exact_teaching_honesty_holds());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a snoozed tip claiming
    // ExactTeaching.
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    tip.claim_narrow = None;
    assert!(!tip.claim_is_honest());
    assert_eq!(tip.status(), TeachingComponentAccessibilityStatus::Stranded);
}

#[test]
fn partial_unsupported_or_stale_shown_as_exact_is_rejected() {
    // A partial-bridge row whose narrow claims ExactTeaching violates exact-teaching honesty.
    let mut bridge = row("a11y:migration-bridge-card-partial");
    if let Some(narrow) = bridge.claim_narrow.as_mut() {
        narrow.narrowed_to = M5TeachingComponentClaim::ExactTeaching;
    }
    assert!(!bridge.exact_teaching_honesty_holds());
    let violations = {
        let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
        packet.rows[2] = bridge;
        packet.validate()
    };
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::PartialUnsupportedOrStaleShownAsExact { .. }
    )));
}

#[test]
fn exact_teaching_honesty_unproven_when_no_partial_unsupported_or_stale_row() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    // Drop the three partial / unsupported / stale rows.
    packet.rows.retain(|r| {
        r.row_id != "a11y:migration-bridge-card-partial"
            && r.row_id != "a11y:sequence-help-strip-unsupported"
            && r.row_id != "a11y:source-language-fallback-stale"
    });
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::ExactTeachingHonestyUnproven
    )));
}

#[test]
fn spurious_narrow_on_exact_row_is_rejected() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.claim_narrow = Some(TeachingComponentClaimAutoNarrow {
        narrowed_to: M5TeachingComponentClaim::StaleFallbackProjection,
        binding_dimension: M5TeachingComponentClaimDimension::SourceLanguage,
        trigger: M5TeachingDowngradeTrigger::SourceLanguageFallbackUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!tip.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.binding_dimension = M5TeachingComponentClaimDimension::SourceLanguage;
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_condition() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.trigger = M5TeachingDowngradeTrigger::CitationSevered;
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    if let Some(narrow) = tip.claim_narrow.as_mut() {
        narrow.narrowed_label = "snoozed".to_owned();
    }
    assert!(!tip.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5TeachingComponentClaim as S;
    use M5TeachingComponentConditionState as C;
    assert_eq!(C::LiveExactTeaching.permitted_ceiling(), S::ExactTeaching);
    assert_eq!(C::TipSnoozed.permitted_ceiling(), S::SnoozedTipProjection);
    assert_eq!(
        C::BridgePartial.permitted_ceiling(),
        S::PartialBridgeProjection
    );
    assert_eq!(
        C::SequenceUnsupported.permitted_ceiling(),
        S::UnsupportedSequenceProjection
    );
    assert_eq!(
        C::FallbackStale.permitted_ceiling(),
        S::StaleFallbackProjection
    );
}

#[test]
fn condition_triggers_map_to_frozen_matrix_vocabulary() {
    use M5TeachingComponentConditionState as C;
    use M5TeachingDowngradeTrigger as T;
    assert_eq!(
        C::TipSnoozed.default_trigger(),
        T::TipCommandBindingUnstated
    );
    assert_eq!(
        C::BridgePartial.default_trigger(),
        T::MigrationMappingUnstated
    );
    assert_eq!(
        C::SequenceUnsupported.default_trigger(),
        T::SequenceHelpStateUnstated
    );
    assert_eq!(
        C::FallbackStale.default_trigger(),
        T::SourceLanguageFallbackUnstated
    );
}

#[test]
fn partial_unsupported_or_stale_states_are_flagged() {
    use M5TeachingComponentConditionState as C;
    assert!(C::BridgePartial.is_partial_unsupported_or_stale());
    assert!(C::SequenceUnsupported.is_partial_unsupported_or_stale());
    assert!(C::FallbackStale.is_partial_unsupported_or_stale());
    assert!(!C::TipSnoozed.is_partial_unsupported_or_stale());
    assert!(!C::LiveExactTeaching.is_partial_unsupported_or_stale());
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_teaching_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_sequence_help_binds_a_non_visual_fallback() {
    let seq = row("a11y:sequence-help-strip-unsupported");
    assert!(seq.is_hierarchy_heavy());
    assert!(seq.has_non_visual_fallback());
    assert!(seq
        .fallback_modalities
        .contains(&M5TeachingComponentFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.keyboard_reach = TeachingComponentNonVisualReachState::ViewOnlyTrap;
    assert!(!tip.reaches_canonical_truth_via_at());
    assert_eq!(tip.status(), TeachingComponentAccessibilityStatus::Stranded);
}

#[test]
fn empty_teaching_context_ref_strands_a_row() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.teaching_context_ref = "  ".to_owned();
    assert!(!tip.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.export_summary = TeachingComponentExportSummaryState::AbsentNeedsScreenshot;
    assert!(!tip.export_preserves_meaning());
    assert_eq!(tip.status(), TeachingComponentAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.copy_export.formats.retain(|f| f != "markdown");
    assert!(!tip.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut bridge = row("a11y:migration-bridge-card-partial");
    bridge.lineage_preserved = false;
    assert!(!bridge.preserves_lineage_continuity());
    assert_eq!(
        bridge.status(),
        TeachingComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut bridge = row("a11y:migration-bridge-card-partial");
    if let Some(narrow) = bridge.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!bridge.preserves_lineage_continuity());
    assert!(!bridge.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    tip.narrowing_disclosures.clear();
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    tip.narrowing_disclosures[0].state = TeachingComponentNarrowingDisclosureState::SilentlyDropped;
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut tip = row("a11y:contextual-tip-card-snoozed");
    tip.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!tip.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut tip = row("a11y:contextual-tip-card-live");
    tip.required_labels
        .retain(|l| *l != M5TeachingRequiredLabel::Identity);
    assert!(!tip.preserves_mandatory_labels());
    assert_eq!(tip.status(), TeachingComponentAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:why-unavailable-explanation-row");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5TeachingConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, TeachingComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_teaching_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        TeachingComponentAccessibilityViolation::RawTeachingMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:contextual-tip-card-snoozed").chip_tokens();
    assert!(chip.contains("family=contextual_tip_card"));
    assert!(chip.contains("effective_claim=snoozed_tip_projection"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        TEACHING_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        TEACHING_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_teaching_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_teaching_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_teaching_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-teaching-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_teaching_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-teaching-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_TEACHING_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-learning generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_TEACHING_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_teaching_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-contextual-teaching-component-accessibility-fallback");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-contextual-teaching-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-contextual-teaching-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
