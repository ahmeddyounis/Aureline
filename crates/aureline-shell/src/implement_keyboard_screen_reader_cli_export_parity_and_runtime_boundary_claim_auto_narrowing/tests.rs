//! Tests for the M05-858 runtime-boundary component accessibility fallback capstone:
//! the honest auto-narrowing logic, the per-family parity contract, and the
//! checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> RuntimeAccessibilityRow {
    seeded_m5_runtime_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5RuntimeBoundaryComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5RuntimeBoundaryComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5RuntimeClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5RuntimeSupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5_RUNTIME_A11Y_CONSUMER_SURFACES {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_four_yellow_zero_red() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 4);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

// --- AC1: a stale / partial runtime can no longer keep an old Live / Ready label ---

#[test]
fn intact_remote_pill_is_live_and_green() {
    let pill = row("a11y:remote-target-pill");
    assert_eq!(pill.full_support_claim, M5RuntimeSupportClaim::Live);
    assert_eq!(pill.effective_claim(), M5RuntimeSupportClaim::Live);
    assert!(pill.claim_narrow.is_none());
    assert_eq!(pill.status(), RuntimeAccessibilityStatus::Parity);
    assert!(pill.effective_claim().asserts_live());
}

#[test]
fn intact_environment_strip_is_ready_and_green() {
    let strip = row("a11y:environment-status-strip");
    assert_eq!(strip.effective_claim(), M5RuntimeSupportClaim::Ready);
    assert!(strip.claim_narrow.is_none());
    assert_eq!(strip.status(), RuntimeAccessibilityStatus::Parity);
    assert!(strip.effective_claim().asserts_full_self_sufficiency());
    assert!(!strip.effective_claim().asserts_live());
}

#[test]
fn restored_terminal_tab_narrows_to_restored() {
    let tab = row("a11y:terminal-tab");
    assert_eq!(tab.effective_claim(), M5RuntimeSupportClaim::Restored);
    assert!(!tab.effective_claim().asserts_live());
    assert!(!tab.effective_claim().asserts_full_self_sufficiency());
    let narrow = tab.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5RuntimeBoundaryDowngradeTrigger::ShellIntegrationQualityHidden
    );
    assert_eq!(
        narrow.binding_dimension,
        M5RuntimeClaimDimension::ShellIntegrationConfidence
    );
    assert!(tab.claim_is_honest());
}

#[test]
fn partial_precedence_narrows_toolchain_to_degraded() {
    let row = row("a11y:toolchain-pin-row");
    assert_eq!(row.effective_claim(), M5RuntimeSupportClaim::Degraded);
    assert!(!row.effective_claim().asserts_full_self_sufficiency());
    assert!(row.claim_is_honest());
}

#[test]
fn reconnecting_presence_narrows_to_reconnecting() {
    let stack = row("a11y:presence-avatar-stack");
    assert_eq!(stack.effective_claim(), M5RuntimeSupportClaim::Reconnecting);
    assert!(!stack.effective_claim().asserts_live());
    assert!(stack.claim_is_honest());
}

#[test]
fn policy_blocked_repair_narrows_to_policy_blocked() {
    let card = row("a11y:repair-action-card");
    assert_eq!(card.effective_claim(), M5RuntimeSupportClaim::PolicyBlocked);
    assert!(!card.effective_claim().asserts_full_self_sufficiency());
    assert!(card.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a restored terminal
    // claiming Live.
    let mut tab = row("a11y:terminal-tab");
    tab.claim_narrow = None;
    assert!(!tab.claim_is_honest());
    assert_eq!(tab.status(), RuntimeAccessibilityStatus::Stranded);
}

#[test]
fn spurious_narrow_on_intact_row_is_rejected() {
    let mut pill = row("a11y:remote-target-pill");
    pill.claim_narrow = Some(RuntimeClaimAutoNarrow {
        narrowed_to: M5RuntimeSupportClaim::Degraded,
        binding_dimension: M5RuntimeClaimDimension::HostIdentity,
        trigger: M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
    });
    assert!(!pill.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut tab = row("a11y:terminal-tab");
    if let Some(narrow) = tab.claim_narrow.as_mut() {
        narrow.binding_dimension = M5RuntimeClaimDimension::HostIdentity;
    }
    assert!(!tab.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut tab = row("a11y:terminal-tab");
    if let Some(narrow) = tab.claim_narrow.as_mut() {
        narrow.trigger = M5RuntimeBoundaryDowngradeTrigger::HostBoundaryMasked;
    }
    assert!(!tab.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut tab = row("a11y:terminal-tab");
    if let Some(narrow) = tab.claim_narrow.as_mut() {
        narrow.narrowed_label = "degraded".to_owned();
    }
    assert!(!tab.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5RuntimeConditionState as C;
    use M5RuntimeSupportClaim as S;
    assert_eq!(C::Intact.permitted_ceiling(), S::Live);
    assert_eq!(C::Partial.permitted_ceiling(), S::Degraded);
    assert_eq!(C::Reconnecting.permitted_ceiling(), S::Reconnecting);
    assert_eq!(C::Restored.permitted_ceiling(), S::Restored);
    assert_eq!(C::PolicyBlocked.permitted_ceiling(), S::PolicyBlocked);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5RuntimeBoundaryDowngradeTrigger as T;
    use M5RuntimeClaimDimension as D;
    assert_eq!(D::HostIdentity.default_trigger(), T::HostBoundaryMasked);
    assert_eq!(
        D::ShellIntegrationConfidence.default_trigger(),
        T::ShellIntegrationQualityHidden
    );
    assert_eq!(
        D::ContextPrecedence.default_trigger(),
        T::RuntimeSourceUnexplained
    );
    assert_eq!(
        D::CollaborationRole.default_trigger(),
        T::CollaborationRoleMasked
    );
    assert_eq!(
        D::RepairReversibility.default_trigger(),
        T::ReversibilityOverstated
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_runtime_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_toolchain_row_binds_a_non_visual_fallback() {
    let row = row("a11y:toolchain-pin-row");
    assert!(row.is_hierarchy_heavy());
    assert!(row.has_non_visual_fallback());
    assert!(row
        .fallback_modalities
        .contains(&M5RuntimeFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut pill = row("a11y:remote-target-pill");
    pill.keyboard_reach = RuntimeNonVisualReachState::ViewOnlyTrap;
    assert!(!pill.reaches_canonical_truth_via_at());
    assert_eq!(pill.status(), RuntimeAccessibilityStatus::Stranded);
}

#[test]
fn empty_runtime_context_ref_strands_a_row() {
    let mut pill = row("a11y:remote-target-pill");
    pill.runtime_context_ref = "  ".to_owned();
    assert!(!pill.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut pill = row("a11y:remote-target-pill");
    pill.export_summary = RuntimeExportSummaryState::AbsentNeedsScreenshot;
    assert!(!pill.export_preserves_meaning());
    assert_eq!(pill.status(), RuntimeAccessibilityStatus::Stranded);
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut pill = row("a11y:remote-target-pill");
    pill.copy_export.formats.retain(|f| f != "markdown");
    assert!(!pill.export_preserves_meaning());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut tab = row("a11y:terminal-tab");
    tab.narrowing_disclosures.clear();
    assert!(!tab.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut tab = row("a11y:terminal-tab");
    tab.narrowing_disclosures[0].state = RuntimeNarrowingDisclosureState::SilentlyDropped;
    assert!(!tab.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut tab = row("a11y:terminal-tab");
    tab.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!tab.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut pill = row("a11y:remote-target-pill");
    pill.required_labels
        .retain(|l| *l != M5RuntimeBoundaryRequiredLabel::Identity);
    assert!(!pill.preserves_mandatory_labels());
    assert_eq!(pill.status(), RuntimeAccessibilityStatus::Stranded);
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_runtime_a11y_fallback_packet();
    packet.rows.retain(|r| r.row_id != "a11y:terminal-tab");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        RuntimeAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_runtime_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5ShellConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        RuntimeAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_runtime_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, RuntimeAccessibilityViolation::DuplicateId { .. })));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_runtime_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, RuntimeAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_runtime_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        RuntimeAccessibilityViolation::RawBoundaryMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:terminal-tab").chip_tokens();
    assert!(chip.contains("family=terminal_tab"));
    assert!(chip.contains("effective_claim=restored"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert_eq!(packet.record_kind, RUNTIME_A11Y_FALLBACK_RECORD_KIND);
    assert_eq!(packet.schema_version, RUNTIME_A11Y_FALLBACK_SCHEMA_VERSION);
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_runtime_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_runtime_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_runtime_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_runtime_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_runtime_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/report.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var
/// so it never runs in the normal suite. Run with
/// `GEN_RUNTIME_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_RUNTIME_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_runtime_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest)
        .join("../../artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof");
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(art.join("report.md"), &report).expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-runtime-boundary-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
    fs::write(
        fixtures.join("README.md"),
        "# M5 runtime-boundary component accessibility fallback fixtures\n\n\
         Mirror of `artifacts/release/m5-runtime-boundary-component-accessibility-fallback-proof/`.\n\
         Regenerate with `GEN_RUNTIME_A11Y_ARTIFACTS=1 cargo test -p aureline-shell generate_artifacts`.\n",
    )
    .expect("write fixture readme");
}
