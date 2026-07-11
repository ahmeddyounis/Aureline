use super::*;

use aureline_execution::m5_host_boundary::{
    ConnectionState, HostKind, HostNarrowingReason, OriginReceiptState,
};

fn clean_strip_input() -> M5HostBoundaryStripResolutionInput {
    M5HostBoundaryStripResolutionInput {
        strip_id: "host-strip:test".to_owned(),
        host_kind: HostKind::Local,
        is_devcontainer: false,
        locality_disclosed: true,
        target_label: "web-frontend@local".to_owned(),
        target_label_disclosed: true,
        owning_runtime_lane: "local desktop runtime".to_owned(),
        owning_lane_disclosed: true,
        connection_state: ConnectionState::Connected,
        reconnect_state_disclosed: true,
        open_details_available: true,
        proof_fresh: true,
    }
}

fn clean_receipt_input() -> M5ExecutionOriginReceiptRowResolutionInput {
    M5ExecutionOriginReceiptRowResolutionInput {
        receipt_id: "origin-receipt:test".to_owned(),
        action_class: "run_tests".to_owned(),
        action_class_disclosed: true,
        resolved_target_identity: "web-frontend@local".to_owned(),
        target_identity_disclosed: true,
        host_kind: HostKind::Local,
        receipt_state: OriginReceiptState::Signed,
        connection_state: ConnectionState::Connected,
        provenance_disclosed: true,
        export_safe_lineage_present: true,
        restored_or_handed_off: false,
        ownership_retained: true,
        host_narrowing_reason: None,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_host_origin_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_HOST_ORIGIN_CONTROLS_PACKET_ID);
}

#[test]
fn strip_local_names_full_host_boundary() {
    let resolved = resolve_host_boundary_strip(clean_strip_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.locality_disclosed);
    assert!(resolved.owning_lane_disclosed);
    assert!(!resolved.hides_host_boundary());
    assert_eq!(resolved.locality, M5HostBoundaryLocality::Local);
    assert_eq!(resolved.host_kind, "local");
    assert_eq!(resolved.origin_locus, "local");
    assert!(!resolved.is_degraded);
    assert_eq!(resolved.next_action, M5HostOriginNextAction::NoActionNeeded);
}

#[test]
fn strip_localities_cover_every_class() {
    // SSH
    let mut input = clean_strip_input();
    input.host_kind = HostKind::Ssh;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::Ssh
    );
    // Container
    let mut input = clean_strip_input();
    input.host_kind = HostKind::Container;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::Container
    );
    // Devcontainer
    let mut input = clean_strip_input();
    input.host_kind = HostKind::Container;
    input.is_devcontainer = true;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::Devcontainer
    );
    // Managed
    let mut input = clean_strip_input();
    input.host_kind = HostKind::ManagedWorkspace;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::Managed
    );
    // Browser bridge
    let mut input = clean_strip_input();
    input.host_kind = HostKind::BrowserBridge;
    input.connection_state = ConnectionState::Bridged;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::BrowserBridge
    );
    // Service plane
    let mut input = clean_strip_input();
    input.host_kind = HostKind::ServicePlane;
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap().locality,
        M5HostBoundaryLocality::ServicePlane
    );
}

#[test]
fn strip_locality_undisclosed_degrades_ac1() {
    let mut input = clean_strip_input();
    input.locality_disclosed = false;
    let resolved = resolve_host_boundary_strip(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.hides_host_boundary());
    assert_eq!(
        resolved.degrade_reason,
        Some(M5HostBoundaryStripDegradeReason::LocalityClassUnstated)
    );
}

#[test]
fn strip_owning_lane_undisclosed_degrades() {
    let mut input = clean_strip_input();
    input.owning_lane_disclosed = false;
    let resolved = resolve_host_boundary_strip(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5HostBoundaryStripDegradeReason::OwningLaneUnstated)
    );
    assert_eq!(
        resolved.next_action,
        M5HostOriginNextAction::ViewExecutionOrigin
    );
}

#[test]
fn strip_reconnecting_without_state_degrades() {
    let mut input = clean_strip_input();
    input.connection_state = ConnectionState::Reconnecting;
    input.reconnect_state_disclosed = false;
    let resolved = resolve_host_boundary_strip(input).unwrap();
    assert!(resolved.is_degraded);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5HostBoundaryStripDegradeReason::ReconnectDegradedStateUnstated)
    );
}

#[test]
fn strip_reconnecting_with_state_is_clean() {
    let mut input = clean_strip_input();
    input.connection_state = ConnectionState::Reconnecting;
    input.reconnect_state_disclosed = true;
    let resolved = resolve_host_boundary_strip(input).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.is_degraded);
    assert_eq!(
        resolved.next_action,
        M5HostOriginNextAction::ReviewDegradedContext
    );
}

#[test]
fn strip_empty_id_and_forbidden_material_error() {
    let mut input = clean_strip_input();
    input.strip_id = "".to_owned();
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap_err(),
        M5HostOriginResolutionError::EmptyStripId
    );

    let mut input = clean_strip_input();
    input.target_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_host_boundary_strip(input).unwrap_err(),
        M5HostOriginResolutionError::ForbiddenMaterial
    );
}

#[test]
fn receipt_signed_is_clean_and_reusable() {
    let resolved = resolve_execution_origin_receipt_row(clean_receipt_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.lineage_stable_for_reuse);
    assert!(!resolved.drops_ownership_on_restore);
    assert_eq!(resolved.origin_confidence, AttributionConfidence::Confirmed);
    assert_eq!(resolved.origin_locus, "local");
    assert_eq!(resolved.next_action, M5HostOriginNextAction::NoActionNeeded);
}

#[test]
fn receipt_lineage_not_export_safe_degrades_ac2() {
    let mut input = clean_receipt_input();
    input.export_safe_lineage_present = false;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(!resolved.lineage_stable_for_reuse);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExecutionOriginReceiptRowDegradeReason::LineageNotExportSafe)
    );
    assert_eq!(
        resolved.next_action,
        M5HostOriginNextAction::ViewExecutionOrigin
    );
}

#[test]
fn receipt_ownership_dropped_on_restore_degrades() {
    let mut input = clean_receipt_input();
    input.restored_or_handed_off = true;
    input.ownership_retained = false;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert!(!resolved.is_clean());
    assert!(resolved.drops_ownership_on_restore);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExecutionOriginReceiptRowDegradeReason::OwnershipDroppedOnRestore)
    );
    assert_eq!(resolved.next_action, M5HostOriginNextAction::ReconnectHost);
}

#[test]
fn receipt_restore_with_ownership_is_clean() {
    let mut input = clean_receipt_input();
    input.restored_or_handed_off = true;
    input.ownership_retained = true;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert!(resolved.is_clean());
    assert!(!resolved.drops_ownership_on_restore);
}

#[test]
fn receipt_provenance_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.provenance_disclosed = false;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExecutionOriginReceiptRowDegradeReason::ProvenanceUnstated)
    );
}

#[test]
fn receipt_target_identity_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.target_identity_disclosed = false;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExecutionOriginReceiptRowDegradeReason::TargetIdentityUnstated)
    );
}

#[test]
fn receipt_action_class_unstated_degrades() {
    let mut input = clean_receipt_input();
    input.action_class_disclosed = false;
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert_eq!(
        resolved.degrade_reason,
        Some(M5ExecutionOriginReceiptRowDegradeReason::ActionClassUnstated)
    );
}

#[test]
fn receipt_bridged_confidence_caps_at_attributed() {
    let mut input = clean_receipt_input();
    input.host_kind = HostKind::BrowserBridge;
    input.receipt_state = OriginReceiptState::Recorded;
    input.connection_state = ConnectionState::Bridged;
    input.host_narrowing_reason = Some(HostNarrowingReason::BridgedBoundary);
    let resolved = resolve_execution_origin_receipt_row(input).unwrap();
    assert!(resolved.is_clean());
    assert_eq!(
        resolved.origin_confidence,
        AttributionConfidence::Attributed
    );
    assert_eq!(resolved.origin_locus, "bridged");
}

#[test]
fn receipt_empty_id_and_forbidden_material_error() {
    let mut input = clean_receipt_input();
    input.receipt_id = "   ".to_owned();
    assert_eq!(
        resolve_execution_origin_receipt_row(input).unwrap_err(),
        M5HostOriginResolutionError::EmptyReceiptId
    );

    let mut input = clean_receipt_input();
    input.resolved_target_identity = "ssh://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_execution_origin_receipt_row(input).unwrap_err(),
        M5HostOriginResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_host_origin_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.vocabulary_set.localities.pop();
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_HOST_BOUNDARY_STRIP_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5HostOriginAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5HostOriginExportField::Localities);
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.controls_rows[0]
        .execution_origin_receipt_row_examples
        .clear();
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    // Force a clean receipt to also read as dropping ownership — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.execution_origin_receipt_row_examples[0].degrade_reason = None;
    row.execution_origin_receipt_row_examples[0].drops_ownership_on_restore = true;
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_host_origin_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.hides_host_locality_or_owning_lane = true,
            1 => row.drops_execution_origin_when_restored_or_degraded = true,
            2 => row.receipt_lineage_not_stable_for_reuse = true,
            _ => row.conceals_boundary_or_origin_in_generic_status_wording = true,
        }
        assert!(packet
            .validate()
            .contains(&M5HostOriginControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn ac1_not_proven_when_a_locality_uncovered() {
    let mut packet = seeded_m5_host_origin_controls();
    // Drop every clean service-plane strip so the required locality coverage breaks.
    for row in &mut packet.controls_rows {
        row.host_boundary_strip_examples
            .retain(|ex| !(ex.is_clean() && ex.locality == M5HostBoundaryLocality::ServicePlane));
    }
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::Ac1NotProven));
}

#[test]
fn ac1_not_proven_when_reconnect_unstated_example_removed() {
    let mut packet = seeded_m5_host_origin_controls();
    for row in &mut packet.controls_rows {
        row.host_boundary_strip_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5HostBoundaryStripDegradeReason::ReconnectDegradedStateUnstated)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::Ac1NotProven));
}

#[test]
fn ac2_not_proven_when_lineage_example_removed() {
    let mut packet = seeded_m5_host_origin_controls();
    for row in &mut packet.controls_rows {
        row.execution_origin_receipt_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5ExecutionOriginReceiptRowDegradeReason::LineageNotExportSafe)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::Ac2NotProven));
}

#[test]
fn ac2_not_proven_when_ownership_example_removed() {
    let mut packet = seeded_m5_host_origin_controls();
    for row in &mut packet.controls_rows {
        row.execution_origin_receipt_row_examples.retain(|ex| {
            ex.degrade_reason
                != Some(M5ExecutionOriginReceiptRowDegradeReason::OwnershipDroppedOnRestore)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::Ac2NotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet
        .governance_review
        .host_ownership_never_disappears_on_restore = false;
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet
        .consumer_projection
        .host_boundary_language_consistent_across_surfaces = false;
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_host_origin_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5HostOriginControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_host_origin_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_host_origin_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_host_origin_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_host_origin_controls_export()
        .expect("checked M5 host-origin controls export validates");
    assert_eq!(from_disk.packet_id, M5_HOST_ORIGIN_CONTROLS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_host_origin_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 5);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::RunTestDebugUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Beta);

    let preview = seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 5);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5BuildRemoteConsumerSurface::PreviewUi)
        .unwrap();
    assert_eq!(row.qualification, M5BuildRemoteQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5HostOriginControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls/host_boundary_strip_beta_narrowed.json"
    )))
    .expect("host-boundary-strip fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_host_origin_controls_host_boundary_strip_beta_narrowed()
    );

    let preview: M5HostOriginControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-host-boundary-strip-execution-origin-receipt-row-controls/execution_origin_receipt_row_preview_narrowed.json"
    )))
    .expect("execution-origin-receipt-row fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_host_origin_controls_execution_origin_receipt_row_preview_narrowed()
    );
}
