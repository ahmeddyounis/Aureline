use super::*;

fn packet() -> ManifestBuildComponentMatrix {
    seeded_manifest_build_component_matrix()
}

fn row_mut<'a>(
    packet: &'a mut ManifestBuildComponentMatrix,
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
    for family in M5ManifestBuildComponentFamily::ALL {
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
        .retain(|r| r.family != M5ManifestBuildComponentFamily::RawEventDrawer);
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::RequiredFamilyMissing));
}

#[test]
fn no_degraded_row_fails() {
    let mut packet = packet();
    for row in &mut packet.components {
        row.degraded = None;
    }
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::DegradedCaseMissing));
}

#[test]
fn wrong_payload_for_family_fails() {
    let mut packet = packet();
    // Attach a stray manifest-header payload to a schema-validator row.
    let row = row_mut(&mut packet, "component:schema-validator-row:0001");
    row.manifest_editor_header = Some(ManifestEditorHeaderDescriptor {
        truth_mode: TruthMode::Plan,
        schema_freshness: M5SchemaFreshness::Fresh,
        edit_posture: M5ManifestEditPosture::ReadOnly,
        target_context_visible: true,
        manifest_ref: "manifest:leak:0001".to_owned(),
    });
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadFamilyMismatch));
}

#[test]
fn manifest_header_hiding_target_context_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001")
        .manifest_editor_header
        .as_mut()
        .expect("header")
        .target_context_visible = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn validator_row_not_blocking_apply_on_errors_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:schema-validator-row:0001")
        .schema_validator_row
        .as_mut()
        .expect("validator row")
        .blocks_apply_on_error = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn resource_link_overwriting_higher_confidence_silently_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:resource-link-row:0001")
        .resource_link_row
        .as_mut()
        .expect("link row")
        .never_overwrites_higher_confidence = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn resource_link_blurring_two_truth_classes_fails() {
    let mut packet = packet();
    // A rendered-to-live link cannot claim both sides are the same truth class.
    row_mut(&mut packet, "component:resource-link-row:0001")
        .resource_link_row
        .as_mut()
        .expect("link row")
        .to_truth = TruthMode::Rendered;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn explorer_showing_live_fresh_as_non_live_truth_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "component:resource-explorer-row:0001");
    // Keep the row/descriptor truth aligned but present live-fresh data as planned
    // truth.
    row.truth_mode = TruthMode::Plan;
    let explorer = row.resource_explorer_row.as_mut().expect("explorer");
    explorer.truth_mode = TruthMode::Plan;
    explorer.freshness = M5ResourceFreshness::LiveFresh;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn heuristic_badge_claiming_high_confidence_fails() {
    let mut packet = packet();
    // A heuristic parse can never claim high confidence.
    row_mut(&mut packet, "component:adapter-source-badge:0002")
        .adapter_source_badge
        .as_mut()
        .expect("badge")
        .confidence = M5DiscoveryConfidence::High;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn badge_disclosing_a_different_adapter_than_its_row_fails() {
    let mut packet = packet();
    let row = row_mut(&mut packet, "component:adapter-source-badge:0001");
    // Keep the badge internally honest but disagree with the row's adapter source.
    row.adapter_source_badge = Some(AdapterSourceBadgeDescriptor {
        adapter_source: M5AdapterSourceKind::ImportedSnapshot,
        confidence: M5DiscoveryConfidence::Low,
        source_kind_explicit: true,
    });
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::DescriptorRowMismatch));
}

#[test]
fn chip_group_disclosing_a_different_truth_than_its_row_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:target-context-chip-group:0001")
        .target_context_chip_group
        .as_mut()
        .expect("chip group")
        .truth_mode = TruthMode::Plan;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::DescriptorRowMismatch));
}

#[test]
fn target_graph_row_without_identity_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:target-graph-row:0001")
        .target_graph_row
        .as_mut()
        .expect("graph row")
        .target_identity_ref = "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn capability_supported_from_unknown_confidence_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:capability-matrix:0001")
        .capability_matrix
        .as_mut()
        .expect("capability")
        .confidence = M5DiscoveryConfidence::Unknown;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn raw_event_drawer_without_redaction_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:raw-event-drawer:0001")
        .raw_event_drawer
        .as_mut()
        .expect("drawer")
        .redaction_applied = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn fallback_overwriting_structured_silently_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:fallback-confidence-drawer:0001")
        .fallback_confidence_drawer
        .as_mut()
        .expect("fallback")
        .never_overwrites_structured_silently = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn fallback_state_without_reason_fails() {
    let mut packet = packet();
    // A heuristic fallback must name why it fell.
    row_mut(&mut packet, "component:fallback-confidence-drawer:0001")
        .fallback_confidence_drawer
        .as_mut()
        .expect("fallback")
        .fallback_reason = None;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn structured_state_carrying_a_reason_fails() {
    let mut packet = packet();
    // A structured-high state cannot carry a fallback reason.
    row_mut(&mut packet, "component:fallback-confidence-drawer:0002")
        .fallback_confidence_drawer
        .as_mut()
        .expect("fallback")
        .fallback_reason = Some(M5FallbackReason::SchemaDrift);
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::PayloadDishonest));
}

#[test]
fn missing_target_context_ref_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001").target_context_ref =
        "   ".to_owned();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::RowIncomplete));
}

#[test]
fn missing_mandatory_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001")
        .required_labels
        .retain(|l| *l != M5ManifestBuildRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::MandatoryLabelMissing));
}

#[test]
fn not_export_safe_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001").export_safe = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::ParityMissing));
}

#[test]
fn not_assistive_ready_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001").assistive_ready = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::ParityMissing));
}

#[test]
fn generic_degraded_label_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:schema-validator-row:0002")
        .degraded
        .as_mut()
        .expect("degraded")
        .degraded_label = "stale".to_owned();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::DegradedLabelGeneric));
}

#[test]
fn row_without_evidence_fails() {
    let mut packet = packet();
    packet.components[0].evidence_refs.clear();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::RowEvidenceMissing));
}

#[test]
fn missing_base_source_contract_fails() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != MANIFEST_BUILD_COMPONENT_MATRIX_DOC_REF);
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::MissingSourceContracts));
}

#[test]
fn incomplete_guardrails_fail() {
    let mut packet = packet();
    packet.guardrails.lower_confidence_never_overwrites_silently = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::GuardrailsIncomplete));
}

#[test]
fn incomplete_consumer_projection_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .later_rows_reference_one_canonical_family = false;
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "wrong".to_owned();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::WrongRecordKind));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    row_mut(&mut packet, "component:manifest-editor-header:0001").label_summary =
        "leaked Bearer abc123 token".to_owned();
    assert!(packet
        .validate()
        .contains(&ManifestBuildComponentViolation::RawBoundaryMaterialInExport));
}

#[test]
fn export_safe_json_round_trips() {
    let packet = packet();
    let json = packet.export_safe_json();
    let parsed: ManifestBuildComponentMatrix =
        serde_json::from_str(&json).expect("export json parses back");
    assert_eq!(parsed, packet);
}

#[test]
fn chip_tokens_name_governed_chips() {
    let row = &packet().components[0];
    let chips = row.chip_tokens();
    assert!(chips.contains("family=manifest_editor_header"));
    assert!(chips.contains("truth=authored_desired"));
    assert!(chips.contains("adapter=native_build_server"));
    assert!(chips.contains("export_safe=true"));
    assert!(chips.contains("assistive=true"));
}

#[test]
fn csv_names_every_component() {
    let csv = packet().render_matrix_csv();
    assert!(csv.contains("component_id,family,truth_mode"));
    assert!(csv.contains("component:fallback-confidence-drawer:0001"));
    assert!(csv.contains("structured_channel_lost"));
}

#[test]
fn markdown_summary_names_rows() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("M5 Manifest / Build Component Matrix"));
    assert!(summary.contains("component:adapter-source-badge:0002"));
    assert!(summary.contains("Degraded:"));
}

#[test]
fn checked_support_export_matches_builder() {
    let checked = current_m5_manifest_build_component_matrix_export()
        .expect("checked manifest/build component export validates");
    assert_eq!(checked, packet());
}
