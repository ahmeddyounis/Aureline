use super::*;

fn packet() -> VisualDesignerComponentMatrix {
    seeded_visual_designer_component_matrix()
}

fn row_mut<'a>(
    packet: &'a mut VisualDesignerComponentMatrix,
    component_id: &str,
) -> &'a mut ComponentRow {
    packet
        .components
        .iter_mut()
        .find(|r| r.component_id == component_id)
        .unwrap_or_else(|| panic!("component {component_id}"))
}

#[test]
fn packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn every_family_is_defined() {
    let families = packet().represented_families();
    for family in M5VisualDesignerComponentFamily::ALL {
        assert!(
            families.contains(&family),
            "missing family: {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_carries_degraded_rows() {
    assert!(packet().degraded_row_count() >= 1);
}

#[test]
fn payload_is_present_only_for_its_family() {
    // Each row carries exactly its family's payload and no other.
    for row in &packet().components {
        assert!(
            row.payload_matches_family(),
            "payload mismatch for {}",
            row.component_id
        );
    }
}

#[test]
fn missing_family_fails() {
    let mut packet = packet();
    packet
        .components
        .retain(|r| r.family != M5VisualDesignerComponentFamily::RoundTripConflictBanner);
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::RequiredFamilyMissing));
}

#[test]
fn no_degraded_row_fails() {
    let mut packet = packet();
    for row in &mut packet.components {
        row.degraded = None;
    }
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::DegradedCaseMissing));
}

#[test]
fn wrong_payload_for_family_fails() {
    let mut packet = packet();
    // Attach a stray design-canvas payload to a property-inspector row.
    let row = row_mut(&mut packet, "component:property-inspector-row:0001");
    row.design_canvas = Some(DesignCanvasDescriptor {
        canvas_state: M5CanvasState::SourceBoundEditable,
        is_derivative_of_source: true,
        selection_synced_with_tree_and_source: true,
        source_revision_ref: "source_revision:leak:0001".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadFamilyMismatch));
}

#[test]
fn canvas_not_derivative_of_source_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001")
        .design_canvas
        .as_mut()
        .expect("canvas")
        .is_derivative_of_source = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn canvas_desynced_selection_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001")
        .design_canvas
        .as_mut()
        .expect("canvas")
        .selection_synced_with_tree_and_source = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn tree_row_faking_a_mapping_fails() {
    let mut packet = packet();
    // An unmapped node that claims it maps to a source span.
    row_mut(&mut packet, "component:structure-tree-row:0002")
        .structure_tree_row
        .as_mut()
        .expect("tree row")
        .maps_to_source_span = true;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn property_row_widening_write_scope_silently_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:property-inspector-row:0001")
        .property_inspector_row
        .as_mut()
        .expect("property row")
        .widens_write_scope_silently = true;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn design_token_edit_recorded_as_single_literal_span_fails() {
    let mut packet = packet();
    // A design-token value can never be recorded as a single literal span.
    row_mut(&mut packet, "component:property-inspector-row:0001")
        .property_inspector_row
        .as_mut()
        .expect("property row")
        .write_scope = M5PropertyWriteScope::SingleLiteralSpan;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn property_write_without_source_diff_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:property-inspector-row:0001")
        .property_inspector_row
        .as_mut()
        .expect("property row")
        .preview_diff = PreviewDiffClass::NoDiffInspectOnly;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn chip_recovery_route_inconsistent_with_sync_fails() {
    let mut packet = packet();
    // A drifted chip cannot claim there is nothing to recover.
    row_mut(&mut packet, "component:source-sync-chip:0001")
        .source_sync_chip
        .as_mut()
        .expect("chip")
        .recovery_route = M5SyncRecoveryRoute::NoneInSync;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn chip_disclosing_a_different_sync_than_its_row_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "component:source-sync-chip:0001");
    // Keep the chip internally honest but make it disagree with the row's sync.
    row.source_sync_chip = Some(SourceSyncChipDescriptor {
        sync_class: SourceSyncClass::InSyncFromSource,
        recovery_route: M5SyncRecoveryRoute::NoneInSync,
    });
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::ChipSyncMismatch));
}

#[test]
fn breakpoint_without_runtime_origin_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:breakpoint-preview-row:0001")
        .breakpoint_preview_row
        .as_mut()
        .expect("breakpoint")
        .runtime_origin_token = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn conflict_banner_permitting_silent_writeback_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:round-trip-conflict-banner:0001")
        .round_trip_conflict_banner
        .as_mut()
        .expect("banner")
        .never_silent_writeback = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn unsupported_card_dropping_selection_context_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:unsupported-construct-card:0001")
        .unsupported_construct_card
        .as_mut()
        .expect("card")
        .preserves_selection_context = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::PayloadDishonest));
}

#[test]
fn missing_mandatory_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001")
        .required_labels
        .retain(|l| *l != M5VisualDesignerRequiredLabel::KeyboardRoute);
    let violations = packet.validate();
    assert!(violations.contains(&VisualDesignerComponentViolation::MandatoryLabelMissing));
}

#[test]
fn not_export_safe_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001").export_safe = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::ParityMissing));
}

#[test]
fn not_assistive_ready_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001").assistive_ready = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::ParityMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:source-sync-chip:0001")
        .degraded
        .as_mut()
        .expect("degraded")
        .degraded_label = "unavailable".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::DegradedLabelGeneric));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.components[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != VISUAL_DESIGNER_COMPONENT_MATRIX_DOC_REF);
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet
        .guardrails
        .unsupported_generated_protected_conflicts_never_silent = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .later_rows_reference_one_canonical_family = false;
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:design-canvas:0001").label_summary =
        "leaked Bearer abc123 token".to_owned();
    assert!(packet
        .validate()
        .contains(&VisualDesignerComponentViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: VisualDesignerComponentMatrix =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().components[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("family=design_canvas"));
    assert!(chips.contains("surface=visual_surface_mapping"));
    assert!(chips.contains("sync=in_sync_from_source"));
    assert!(chips.contains("round_trip=exact_source_round_trip"));
    assert!(chips.contains("export_safe=true"));
    assert!(chips.contains("assistive=true"));
}

#[test]
fn csv_names_every_component() {
    let csv = packet().render_matrix_csv();
    assert!(csv.contains("component_id,family,preview_surface"));
    assert!(csv.contains("component:round-trip-conflict-banner:0001"));
    assert!(csv.contains("drifted_from_source"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Visual-Designer Component Matrix"));
    assert!(summary.contains("component:unsupported-construct-card:0001"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_visual_designer_component_matrix_export()
        .expect("checked visual-designer component export validates");
    assert_eq!(checked, packet());
}
