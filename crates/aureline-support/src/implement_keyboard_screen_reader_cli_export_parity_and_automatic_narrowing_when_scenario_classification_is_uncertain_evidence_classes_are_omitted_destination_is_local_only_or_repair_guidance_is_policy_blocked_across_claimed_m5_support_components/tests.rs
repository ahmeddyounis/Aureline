//! Tests for the M05-906 support-intake / escalation component accessibility fallback capstone:
//! the honest auto-narrowing logic, the per-family parity contract, no-loss case-lineage
//! integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> SupportIntakeComponentAccessibilityRow {
    seeded_m5_support_intake_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5SupportIntakeEscalationComponentFamily::ALL.len()
    );
    // Six rows cover the five families (the issue-report builder step is certified both
    // reviewable-green and evidence-omitted-yellow).
    assert_eq!(packet.rows.len(), 6);
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5SupportIntakeClaimDimension::ALL.len()
    );
}

#[test]
fn every_condition_state_is_exercised() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let states = packet.exercised_condition_states();
    for state in M5SupportIntakeConditionState::ALL {
        assert!(
            states.contains(&state),
            "condition state {} is not exercised",
            state.as_str()
        );
    }
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5SupportIntakeClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5SupportConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary.row_count, 6);
    assert_eq!(
        packet.summary.family_count,
        M5SupportIntakeEscalationComponentFamily::ALL.len()
    );
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_lineage_preserved() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert!(packet.summary.all_lineage_preserved);
}

// --- AC1: uncertain / evidence-omitted / local-only / policy-blocked can no longer keep a ready label ---

#[test]
fn complete_report_is_reviewable_and_green() {
    let report = row("a11y:issue-report-builder-step-reviewable");
    assert_eq!(
        report.effective_claim(),
        M5SupportIntakeClaim::ReviewableCase
    );
    assert!(report.claim_narrow.is_none());
    assert_eq!(
        report.status(),
        SupportIntakeComponentAccessibilityStatus::Parity
    );
    assert!(report.effective_claim().asserts_full_case());
    assert!(!report.effective_claim().asserts_ready_to_escalate());
}

#[test]
fn stated_handoff_is_ready_to_escalate_and_green() {
    let handoff = row("a11y:handoff-timeline-row");
    assert_eq!(
        handoff.full_support_claim,
        M5SupportIntakeClaim::ReadyToEscalate
    );
    assert_eq!(
        handoff.effective_claim(),
        M5SupportIntakeClaim::ReadyToEscalate
    );
    assert!(handoff.claim_narrow.is_none());
    assert_eq!(
        handoff.status(),
        SupportIntakeComponentAccessibilityStatus::Parity
    );
    assert!(handoff.effective_claim().asserts_ready_to_escalate());
}

#[test]
fn uncertain_scenario_narrows_to_unclassified_scenario() {
    let picker = row("a11y:support-scenario-picker-row");
    assert_eq!(
        picker.effective_claim(),
        M5SupportIntakeClaim::UnclassifiedScenario
    );
    assert!(!picker.effective_claim().asserts_ready_to_escalate());
    let narrow = picker.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5SupportDowngradeTrigger::ScenarioOrScopeUnstated
    );
    assert_eq!(
        narrow.binding_dimension,
        M5SupportIntakeClaimDimension::ScenarioClassification
    );
    assert!(picker.claim_is_honest());
}

#[test]
fn omitted_evidence_narrows_to_evidence_incomplete_case() {
    let step = row("a11y:issue-report-builder-step-evidence-omitted");
    assert_eq!(
        step.effective_claim(),
        M5SupportIntakeClaim::EvidenceIncompleteCase
    );
    let narrow = step.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5SupportDowngradeTrigger::EvidenceClassMasked
    );
    assert!(step.claim_is_honest());
}

#[test]
fn local_only_destination_narrows_to_local_only_diagnosis() {
    let packet = row("a11y:escalation-packet-summary");
    assert_eq!(
        packet.effective_claim(),
        M5SupportIntakeClaim::LocalOnlyDiagnosis
    );
    let narrow = packet.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5SupportDowngradeTrigger::PacketDestinationUnstated
    );
    assert!(packet.claim_is_honest());
}

#[test]
fn policy_blocked_repair_narrows_to_policy_blocked_repair() {
    let note = row("a11y:unsafe-fix-blocked-note");
    assert_eq!(
        note.effective_claim(),
        M5SupportIntakeClaim::PolicyBlockedRepair
    );
    assert!(!note.effective_claim().asserts_full_case());
    let narrow = note.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5SupportDowngradeTrigger::ApprovedRepairClassMasked
    );
    assert!(note.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: an uncertain scenario picker
    // claiming ReadyToEscalate.
    let mut picker = row("a11y:support-scenario-picker-row");
    picker.claim_narrow = None;
    assert!(!picker.claim_is_honest());
    assert_eq!(
        picker.status(),
        SupportIntakeComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn spurious_narrow_on_ready_row_is_rejected() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff.claim_narrow = Some(SupportIntakeClaimAutoNarrow {
        narrowed_to: M5SupportIntakeClaim::LocalOnlyDiagnosis,
        binding_dimension: M5SupportIntakeClaimDimension::DestinationReach,
        trigger: M5SupportDowngradeTrigger::PacketDestinationUnstated,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_lineage_continuity: true,
    });
    assert!(!handoff.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut picker = row("a11y:support-scenario-picker-row");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.binding_dimension = M5SupportIntakeClaimDimension::RepairGuidance;
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut picker = row("a11y:support-scenario-picker-row");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.trigger = M5SupportDowngradeTrigger::ApprovedRepairClassMasked;
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut picker = row("a11y:support-scenario-picker-row");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.narrowed_label = "unclassified".to_owned();
    }
    assert!(!picker.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5SupportIntakeClaim as S;
    use M5SupportIntakeConditionState as C;
    assert_eq!(C::Classified.permitted_ceiling(), S::ReadyToEscalate);
    assert_eq!(
        C::LocalOnlyDestination.permitted_ceiling(),
        S::LocalOnlyDiagnosis
    );
    assert_eq!(
        C::EvidenceOmitted.permitted_ceiling(),
        S::EvidenceIncompleteCase
    );
    assert_eq!(
        C::ScenarioUncertain.permitted_ceiling(),
        S::UnclassifiedScenario
    );
    assert_eq!(
        C::RepairPolicyBlocked.permitted_ceiling(),
        S::PolicyBlockedRepair
    );
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5SupportDowngradeTrigger as T;
    use M5SupportIntakeClaimDimension as D;
    assert_eq!(
        D::ScenarioClassification.default_trigger(),
        T::ScenarioOrScopeUnstated
    );
    assert_eq!(
        D::EvidenceCompleteness.default_trigger(),
        T::EvidenceClassMasked
    );
    assert_eq!(
        D::DestinationReach.default_trigger(),
        T::PacketDestinationUnstated
    );
    assert_eq!(
        D::HandoffContinuity.default_trigger(),
        T::NextHumanStepUnstated
    );
    assert_eq!(
        D::RepairGuidance.default_trigger(),
        T::ApprovedRepairClassMasked
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-loss ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_support_intake_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_escalation_packet_binds_a_non_visual_fallback() {
    let packet = row("a11y:escalation-packet-summary");
    assert!(packet.is_hierarchy_heavy());
    assert!(packet.has_non_visual_fallback());
    assert!(packet
        .fallback_modalities
        .contains(&M5SupportIntakeFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff.keyboard_reach = SupportIntakeNonVisualReachState::ViewOnlyTrap;
    assert!(!handoff.reaches_canonical_truth_via_at());
    assert_eq!(
        handoff.status(),
        SupportIntakeComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_support_context_ref_strands_a_row() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff.support_context_ref = "  ".to_owned();
    assert!(!handoff.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff.export_summary = SupportIntakeExportSummaryState::AbsentNeedsScreenshot;
    assert!(!handoff.export_preserves_meaning());
    assert_eq!(
        handoff.status(),
        SupportIntakeComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff.copy_export.formats.retain(|f| f != "markdown");
    assert!(!handoff.export_preserves_meaning());
}

#[test]
fn dropped_lineage_strands_a_row() {
    let mut picker = row("a11y:support-scenario-picker-row");
    picker.lineage_preserved = false;
    assert!(!picker.preserves_lineage_continuity());
    assert_eq!(
        picker.status(),
        SupportIntakeComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_lineage_continuity_strands_a_row() {
    let mut picker = row("a11y:support-scenario-picker-row");
    if let Some(narrow) = picker.claim_narrow.as_mut() {
        narrow.preserves_lineage_continuity = false;
    }
    assert!(!picker.preserves_lineage_continuity());
    assert!(!picker.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut picker = row("a11y:support-scenario-picker-row");
    picker.narrowing_disclosures.clear();
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut picker = row("a11y:support-scenario-picker-row");
    picker.narrowing_disclosures[0].state = SupportIntakeNarrowingDisclosureState::SilentlyDropped;
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut picker = row("a11y:support-scenario-picker-row");
    picker.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!picker.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut handoff = row("a11y:handoff-timeline-row");
    handoff
        .required_labels
        .retain(|l| *l != M5SupportRequiredLabel::Identity);
    assert!(!handoff.preserves_mandatory_labels());
    assert_eq!(
        handoff.status(),
        SupportIntakeComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:unsafe-fix-blocked-note");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        SupportIntakeComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5SupportConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        SupportIntakeComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        SupportIntakeComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        SupportIntakeComponentAccessibilityViolation::SummaryMismatch
    )));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        SupportIntakeComponentAccessibilityViolation::RawSupportMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:support-scenario-picker-row").chip_tokens();
    assert!(chip.contains("family=support_scenario_picker_row"));
    assert!(chip.contains("effective_claim=unclassified_scenario"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        SUPPORT_INTAKE_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_support_intake_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(
        packet,
        seeded_m5_support_intake_component_a11y_fallback_packet()
    );
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_support_intake_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected =
        seeded_m5_support_intake_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_SUPPORT_INTAKE_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-support generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_SUPPORT_INTAKE_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_support_intake_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest).join(
        "../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback",
    );
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-support-intake-escalation-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-support-intake-escalation-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
