//! Tests for the M05-898 local-history / write-scope component accessibility fallback
//! capstone: the honest auto-narrowing logic, the per-family parity contract, no-erasure
//! history integrity, and the checked-in support export / CSV / report.

use super::*;

fn row(id: &str) -> HistoryComponentAccessibilityRow {
    seeded_m5_history_component_a11y_fallback_packet()
        .rows
        .into_iter()
        .find(|r| r.row_id == id)
        .unwrap_or_else(|| panic!("row {id} exists"))
}

// --- structural coverage ---

#[test]
fn seeded_packet_validates_clean() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "unexpected violations: {violations:?}"
    );
}

#[test]
fn every_frozen_family_is_certified_once() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert_eq!(
        packet.represented_families().len(),
        M5LocalHistoryWriteScopeComponentFamily::ALL.len()
    );
    assert_eq!(
        packet.rows.len(),
        M5LocalHistoryWriteScopeComponentFamily::ALL.len()
    );
}

#[test]
fn every_dimension_is_exercised() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert_eq!(
        packet.exercised_dimensions().len(),
        M5HistoryClaimDimension::ALL.len()
    );
}

#[test]
fn every_support_claim_tier_appears_as_an_effective_claim() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    let effective = packet.represented_effective_claims();
    for claim in M5HistorySupportClaim::ALL {
        assert!(
            effective.contains(&claim),
            "claim tier {} missing from effective claims",
            claim.as_str()
        );
    }
}

#[test]
fn every_consumer_surface_ingests_a_row() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    let consumers = packet.represented_consumer_surfaces();
    for surface in M5HistoryConsumerSurface::ALL {
        assert!(
            consumers.contains(&surface),
            "consumer surface {} ingests no row",
            surface.as_str()
        );
    }
}

#[test]
fn summary_counts_two_green_five_yellow_zero_red() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert_eq!(packet.summary.green_count, 2);
    assert_eq!(packet.summary.yellow_count, 5);
    assert_eq!(packet.summary.red_count, 0);
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn summary_reports_all_history_preserved() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert!(packet.summary.all_history_preserved);
}

// --- AC1: metadata-only / partial / stale / unavailable can no longer keep a restorable label ---

#[test]
fn available_checkpoint_manifest_is_reviewable_and_green() {
    let manifest = row("a11y:history-export-manifest");
    assert_eq!(
        manifest.effective_claim(),
        M5HistorySupportClaim::ReviewableHistory
    );
    assert!(manifest.claim_narrow.is_none());
    assert_eq!(
        manifest.status(),
        HistoryComponentAccessibilityStatus::Parity
    );
    assert!(manifest.effective_claim().asserts_full_recovery());
    assert!(!manifest.effective_claim().asserts_restorable());
}

#[test]
fn full_scope_selector_is_restorable_and_green() {
    let selector = row("a11y:restore-granularity-selector");
    assert_eq!(
        selector.full_support_claim,
        M5HistorySupportClaim::RestorableCheckpoint
    );
    assert_eq!(
        selector.effective_claim(),
        M5HistorySupportClaim::RestorableCheckpoint
    );
    assert!(selector.claim_narrow.is_none());
    assert_eq!(
        selector.status(),
        HistoryComponentAccessibilityStatus::Parity
    );
    assert!(selector.effective_claim().asserts_restorable());
}

#[test]
fn metadata_only_capture_narrows_to_metadata_only_history() {
    let local = row("a11y:local-history-row");
    assert_eq!(
        local.effective_claim(),
        M5HistorySupportClaim::MetadataOnlyHistory
    );
    assert!(!local.effective_claim().asserts_restorable());
    let narrow = local.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5HistoryDowngradeTrigger::CaptureFidelityMasked
    );
    assert_eq!(
        narrow.binding_dimension,
        M5HistoryClaimDimension::CaptureFidelity
    );
    assert!(local.claim_is_honest());
}

#[test]
fn partial_restore_narrows_to_narrowed_restore() {
    let restore = row("a11y:restore-preview-card");
    assert_eq!(
        restore.effective_claim(),
        M5HistorySupportClaim::NarrowedRestore
    );
    let narrow = restore.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5HistoryDowngradeTrigger::RestoreGranularityCollapsed
    );
    assert!(restore.claim_is_honest());
}

#[test]
fn stale_scope_narrows_to_stale_scope_history() {
    let tree = row("a11y:write-scope-preview-tree");
    assert_eq!(
        tree.effective_claim(),
        M5HistorySupportClaim::StaleScopeHistory
    );
    let narrow = tree.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5HistoryDowngradeTrigger::WriteScopeUnderstated
    );
    assert!(tree.claim_is_honest());
}

#[test]
fn unavailable_checkpoint_narrows_to_unavailable_checkpoint() {
    let checkpoint = row("a11y:checkpoint-group-card");
    assert_eq!(
        checkpoint.effective_claim(),
        M5HistorySupportClaim::UnavailableCheckpoint
    );
    assert!(!checkpoint.effective_claim().asserts_full_recovery());
    let narrow = checkpoint.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5HistoryDowngradeTrigger::CheckpointLineageUnstated
    );
    assert!(checkpoint.claim_is_honest());
}

#[test]
fn export_limited_retention_narrows_to_metadata_only_history() {
    let retention = row("a11y:retention-export-card");
    assert_eq!(
        retention.effective_claim(),
        M5HistorySupportClaim::MetadataOnlyHistory
    );
    let narrow = retention.claim_narrow.as_ref().expect("narrow present");
    assert_eq!(
        narrow.trigger,
        M5HistoryDowngradeTrigger::RetentionOrRedactionUndisclosed
    );
    assert!(retention.claim_is_honest());
}

#[test]
fn over_asserting_control_is_rejected() {
    // Force the effective claim above the permitted ceiling: a metadata-only local-history row
    // claiming RestorableCheckpoint.
    let mut local = row("a11y:local-history-row");
    local.claim_narrow = None;
    assert!(!local.claim_is_honest());
    assert_eq!(
        local.status(),
        HistoryComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn spurious_narrow_on_restorable_row_is_rejected() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector.claim_narrow = Some(HistoryClaimAutoNarrow {
        narrowed_to: M5HistorySupportClaim::StaleScopeHistory,
        binding_dimension: M5HistoryClaimDimension::RestoreScopeSelection,
        trigger: M5HistoryDowngradeTrigger::RestoreGranularityCollapsed,
        narrowed_label: "spurious".to_owned(),
        preserves_canonical_identity: true,
        preserves_history_integrity: true,
    });
    assert!(!selector.claim_is_honest());
}

#[test]
fn narrow_must_bind_the_ceiling_imposing_dimension() {
    let mut local = row("a11y:local-history-row");
    if let Some(narrow) = local.claim_narrow.as_mut() {
        narrow.binding_dimension = M5HistoryClaimDimension::ScopeFreshness;
    }
    assert!(!local.claim_is_honest());
}

#[test]
fn narrow_trigger_must_match_binding_dimension() {
    let mut local = row("a11y:local-history-row");
    if let Some(narrow) = local.claim_narrow.as_mut() {
        narrow.trigger = M5HistoryDowngradeTrigger::WriteScopeUnderstated;
    }
    assert!(!local.claim_is_honest());
}

#[test]
fn narrowed_label_must_not_be_generic() {
    let mut local = row("a11y:local-history-row");
    if let Some(narrow) = local.claim_narrow.as_mut() {
        narrow.narrowed_label = "metadata only".to_owned();
    }
    assert!(!local.claim_is_honest());
}

#[test]
fn permitted_ceilings_map_condition_states_one_to_one() {
    use M5HistoryConditionState as C;
    use M5HistorySupportClaim as S;
    assert_eq!(C::Captured.permitted_ceiling(), S::RestorableCheckpoint);
    assert_eq!(C::NarrowedRestore.permitted_ceiling(), S::NarrowedRestore);
    assert_eq!(C::MetadataOnly.permitted_ceiling(), S::MetadataOnlyHistory);
    assert_eq!(C::StaleScope.permitted_ceiling(), S::StaleScopeHistory);
    assert_eq!(C::Unavailable.permitted_ceiling(), S::UnavailableCheckpoint);
}

#[test]
fn dimension_triggers_map_to_frozen_matrix_vocabulary() {
    use M5HistoryClaimDimension as D;
    use M5HistoryDowngradeTrigger as T;
    assert_eq!(
        D::CaptureFidelity.default_trigger(),
        T::CaptureFidelityMasked
    );
    assert_eq!(
        D::CheckpointAvailability.default_trigger(),
        T::CheckpointLineageUnstated
    );
    assert_eq!(
        D::RestoreGranularity.default_trigger(),
        T::RestoreGranularityCollapsed
    );
    assert_eq!(
        D::ExportDisclosure.default_trigger(),
        T::RetentionOrRedactionUndisclosed
    );
    assert_eq!(
        D::ScopeFreshness.default_trigger(),
        T::WriteScopeUnderstated
    );
    assert_eq!(
        D::RestoreScopeSelection.default_trigger(),
        T::RestoreGranularityCollapsed
    );
    assert_eq!(
        D::ManifestExportDisclosure.default_trigger(),
        T::RetentionOrRedactionUndisclosed
    );
}

// --- AC2: accessibility / CLI / export reach the same canonical truth + no-erasure ---

#[test]
fn every_row_reaches_canonical_truth_via_at() {
    for r in seeded_m5_history_component_a11y_fallback_packet().rows {
        assert!(
            r.reaches_canonical_truth_via_at(),
            "row {} strands assistive tech",
            r.row_id
        );
    }
}

#[test]
fn hierarchy_heavy_write_scope_tree_binds_a_non_visual_fallback() {
    let tree = row("a11y:write-scope-preview-tree");
    assert!(tree.is_hierarchy_heavy());
    assert!(tree.has_non_visual_fallback());
    assert!(tree
        .fallback_modalities
        .contains(&M5HistoryFallbackModality::Structured));
}

#[test]
fn view_only_trap_strands_and_reds_a_row() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector.keyboard_reach = HistoryNonVisualReachState::ViewOnlyTrap;
    assert!(!selector.reaches_canonical_truth_via_at());
    assert_eq!(
        selector.status(),
        HistoryComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn empty_history_context_ref_strands_a_row() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector.history_context_ref = "  ".to_owned();
    assert!(!selector.reaches_canonical_truth_via_at());
}

#[test]
fn export_needing_screenshot_is_rejected() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector.export_summary = HistoryExportSummaryState::AbsentNeedsScreenshot;
    assert!(!selector.export_preserves_meaning());
    assert_eq!(
        selector.status(),
        HistoryComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn copy_export_requires_text_json_markdown() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector.copy_export.formats.retain(|f| f != "markdown");
    assert!(!selector.export_preserves_meaning());
}

#[test]
fn erased_history_strands_a_row() {
    let mut local = row("a11y:local-history-row");
    local.history_preserved = false;
    assert!(!local.preserves_history_integrity());
    assert_eq!(
        local.status(),
        HistoryComponentAccessibilityStatus::Stranded
    );
}

#[test]
fn narrow_dropping_history_integrity_strands_a_row() {
    let mut local = row("a11y:local-history-row");
    if let Some(narrow) = local.claim_narrow.as_mut() {
        narrow.preserves_history_integrity = false;
    }
    assert!(!local.preserves_history_integrity());
    assert!(!local.claim_is_honest());
}

// --- AC3: narrowing disclosed across every narrower surface ---

#[test]
fn narrowed_surface_without_disclosure_is_rejected() {
    let mut local = row("a11y:local-history-row");
    local.narrowing_disclosures.clear();
    assert!(!local.narrowing_disclosed());
}

#[test]
fn silently_dropped_surface_is_rejected() {
    let mut local = row("a11y:local-history-row");
    local.narrowing_disclosures[0].state = HistoryNarrowingDisclosureState::SilentlyDropped;
    assert!(!local.narrowing_disclosed());
}

#[test]
fn narrowed_surface_must_preserve_labels() {
    let mut local = row("a11y:local-history-row");
    local.narrowing_disclosures[0].preserved_labels.clear();
    assert!(!local.narrowing_disclosed());
}

#[test]
fn dropping_a_mandatory_label_strands_a_row() {
    let mut selector = row("a11y:restore-granularity-selector");
    selector
        .required_labels
        .retain(|l| *l != M5HistoryRequiredLabel::Identity);
    assert!(!selector.preserves_mandatory_labels());
    assert_eq!(
        selector.status(),
        HistoryComponentAccessibilityStatus::Stranded
    );
}

// --- packet-level validation ---

#[test]
fn missing_family_coverage_is_flagged() {
    let mut packet = seeded_m5_history_component_a11y_fallback_packet();
    packet
        .rows
        .retain(|r| r.row_id != "a11y:restore-granularity-selector");
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        HistoryComponentAccessibilityViolation::MissingFamilyCoverage { .. }
    )));
}

#[test]
fn single_consumer_surface_fails_parity() {
    let mut packet = seeded_m5_history_component_a11y_fallback_packet();
    packet.rows[0].consumer_surfaces = vec![M5HistoryConsumerSurface::SupportExport];
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        HistoryComponentAccessibilityViolation::MissingConsumerParity { .. }
    )));
}

#[test]
fn duplicate_row_id_is_flagged() {
    let mut packet = seeded_m5_history_component_a11y_fallback_packet();
    let dup = packet.rows[0].clone();
    packet.rows.push(dup);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        HistoryComponentAccessibilityViolation::DuplicateId { .. }
    )));
}

#[test]
fn summary_mismatch_is_flagged() {
    let mut packet = seeded_m5_history_component_a11y_fallback_packet();
    packet.summary.green_count += 1;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, HistoryComponentAccessibilityViolation::SummaryMismatch)));
}

#[test]
fn forbidden_material_in_export_is_flagged() {
    let mut packet = seeded_m5_history_component_a11y_fallback_packet();
    packet.rows[0]
        .source_refs
        .push("api_key=hunter2".to_owned());
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        HistoryComponentAccessibilityViolation::RawHistoryMaterialInExport
    )));
}

// --- rendering ---

#[test]
fn chip_tokens_render_stable_fields() {
    let chip = row("a11y:local-history-row").chip_tokens();
    assert!(chip.contains("family=local_history_row"));
    assert!(chip.contains("effective_claim=metadata_only_history"));
    assert!(chip.contains("status=narrowed_disclosed"));
}

#[test]
fn record_kind_and_schema_version_are_stable() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert_eq!(
        packet.record_kind,
        HISTORY_COMPONENT_A11Y_FALLBACK_RECORD_KIND
    );
    assert_eq!(
        packet.schema_version,
        HISTORY_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
    );
}

#[test]
fn export_is_free_of_forbidden_material() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert!(!json_contains_forbidden_material(
        &serde_json::to_value(&packet).expect("serializes")
    ));
}

#[test]
fn csv_has_one_header_and_one_line_per_row() {
    let packet = seeded_m5_history_component_a11y_fallback_packet();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), packet.rows.len() + 1);
    assert!(lines[0].starts_with("row_id,component_family"));
}

// --- checked-in artifacts ---

#[test]
fn checked_support_export_matches_builder() {
    let packet = current_m5_history_component_a11y_fallback_export()
        .expect("checked-in support export parses and validates");
    assert_eq!(packet, seeded_m5_history_component_a11y_fallback_packet());
}

#[test]
fn checked_csv_matches_builder() {
    let expected = seeded_m5_history_component_a11y_fallback_packet().render_matrix_csv();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-write-scope-component-accessibility-fallback/matrix.csv"
    ));
    assert_eq!(expected, on_disk);
}

#[test]
fn checked_report_matches_builder() {
    let expected = seeded_m5_history_component_a11y_fallback_packet().render_markdown_summary();
    let on_disk = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-local-history-write-scope-component-accessibility-fallback.md"
    ));
    assert_eq!(expected, on_disk);
}

/// One-shot generator for the checked-in artifacts + fixtures. Gated on an env var so it never
/// runs in the normal suite. Run with
/// `GEN_HISTORY_COMPONENT_A11Y_ARTIFACTS=1 cargo test -p aureline-history generate_artifacts`.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_HISTORY_COMPONENT_A11Y_ARTIFACTS").is_err() {
        return;
    }
    use std::fs;
    use std::path::Path;

    let packet = seeded_m5_history_component_a11y_fallback_packet();
    assert!(packet.validate().is_empty());

    let manifest = env!("CARGO_MANIFEST_DIR");
    let json = format!("{}\n", packet.export_safe_json());
    let csv = packet.render_matrix_csv();
    let report = packet.render_markdown_summary();

    let art = Path::new(manifest).join(
        "../../artifacts/release/m5-local-history-write-scope-component-accessibility-fallback",
    );
    fs::create_dir_all(&art).expect("create artifact dir");
    fs::write(art.join("support_export.json"), &json).expect("write support export");
    fs::write(art.join("matrix.csv"), &csv).expect("write csv");
    fs::write(
        Path::new(manifest).join(
            "../../artifacts/release/m5-local-history-write-scope-component-accessibility-fallback.md",
        ),
        &report,
    )
    .expect("write report");

    let fixtures = Path::new(manifest)
        .join("../../fixtures/ui/m5-local-history-write-scope-component-accessibility-fallback");
    fs::create_dir_all(&fixtures).expect("create fixture dir");
    fs::write(fixtures.join("support_export.json"), &json).expect("write fixture export");
    fs::write(fixtures.join("matrix.csv"), &csv).expect("write fixture csv");
}
