use super::*;

const LOCAL_ONLY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-inspector/local_only_no_export.json"
));

const PORTABLE_EXPORTABLE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-inspector/portable_exportable.json"
));

const BOUNDED_CLEAR_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-remembered-state-inspector/bounded_clear.json"
));

fn packet() -> M5RememberedStateInspector {
    current_m5_remembered_state_inspector().expect("packet parses")
}

fn row(json: &str) -> InspectorRow {
    serde_json::from_str(json).expect("fixture parses")
}

// --- Embedded packet -----------------------------------------------------------------------------

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_REMEMBERED_STATE_INSPECTOR_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_REMEMBERED_STATE_INSPECTOR_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn embedded_packet_round_trips_byte_stable_shape() {
    let packet = packet();
    let encoded = serde_json::to_string(&packet).expect("serializes");
    let decoded: M5RememberedStateInspector = serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded, packet);
}

#[test]
fn summary_counts_match_rows() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.summary.rows, 6);
    assert_eq!(packet.summary.exportable_rows, 3);
    assert_eq!(packet.summary.local_only_rows, 3);
    assert_eq!(packet.summary.clearable_rows, 6);
    assert_eq!(packet.summary.comparable_rows, 6);
}

#[test]
fn one_row_per_artifact_class() {
    let packet = packet();
    for class in RememberedArtifactClass::ALL {
        assert!(packet.row(class).is_some(), "missing row for {class:?}");
    }
}

#[test]
fn ownership_and_fidelity_align_with_matrix() {
    let packet = packet();
    let matrix = crate::m5_serialization_and_restore_matrix::current_m5_serialization_matrix()
        .expect("matrix parses");
    // The inspector must not fork ownership or restore-fidelity labels from the matrix.
    for row in &packet.rows {
        let matrix_row = matrix.row(row.artifact_class).expect("matrix row");
        assert_eq!(
            row.ownership, matrix_row.ownership,
            "{:?}",
            row.artifact_class
        );
        assert_eq!(row.exportable, matrix_row.exportable);
        assert_eq!(
            row.published_fidelity, matrix_row.published_fidelity,
            "{:?}",
            row.artifact_class
        );
    }
}

#[test]
fn portable_and_shared_state_is_exportable() {
    let packet = packet();
    for row in &packet.rows {
        assert_eq!(row.exportable, row.expected_exportable());
        if matches!(
            row.ownership,
            OwnershipClass::Portable | OwnershipClass::Shared
        ) {
            assert!(row.exportable);
            assert!(row.has_action(InspectorActionKind::Export));
        } else {
            assert!(!row.exportable);
            assert!(!row.has_action(InspectorActionKind::Export));
        }
    }
}

#[test]
fn every_row_is_inspectable_clearable_and_comparable() {
    let packet = packet();
    for row in &packet.rows {
        assert!(row.has_action(InspectorActionKind::Inspect));
        assert!(row.has_action(InspectorActionKind::Compare));
        assert!(row.is_clearable());
    }
}

#[test]
fn every_clear_is_bounded_and_confirmed() {
    let packet = packet();
    for row in &packet.rows {
        let clear = row
            .action(InspectorActionKind::Clear)
            .expect("clear present");
        assert_eq!(clear.boundary, ActionBoundary::SelectedStateClassOnly);
        assert!(clear.requires_confirmation);
        assert!(clear.excludes_unrelated_content);
        assert!(clear.excludes_caches);
        assert!(clear.is_safe_clear());
    }
}

#[test]
fn every_affordance_is_accessible_and_keyboard_complete() {
    let packet = packet();
    for row in &packet.rows {
        let mut focus_orders = std::collections::BTreeSet::new();
        for affordance in &row.available_actions {
            assert!(affordance.is_accessible(), "{}", row.row_id);
            assert!(!affordance.keyboard_shortcut.trim().is_empty());
            assert!(!affordance.accessible_label.trim().is_empty());
            // Focus order is unique within a row so keyboard navigation is unambiguous.
            assert!(
                focus_orders.insert(affordance.focus_order),
                "{}",
                row.row_id
            );
        }
    }
}

#[test]
fn reuse_surfaces_are_bound() {
    let packet = packet();
    for surface in InspectorConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding {surface:?}"
        );
    }
}

#[test]
fn inspect_view_is_plain_language_without_raw_json() {
    let packet = packet();
    let view = packet.inspect_view();
    assert_eq!(view.rows.len(), 6);
    assert_eq!(view.exportable_count, 3);
    assert_eq!(view.clearable_count, 6);
    for row in &view.rows {
        assert!(!row.what_is_remembered.trim().is_empty());
        assert!(!row.what_is_not_remembered.trim().is_empty());
        assert!(!row.summary.trim().is_empty());
        assert!(!row.actions.is_empty());
    }
    // The portable package reads as exportable; the topology snapshot reads as local-only.
    let package = view
        .rows
        .iter()
        .find(|r| r.artifact_class == "portable_state_package")
        .expect("package row");
    assert!(package.exportable);
    assert!(package.summary.contains("exportable"));
    let topology = view
        .rows
        .iter()
        .find(|r| r.artifact_class == "window_topology_snapshot")
        .expect("topology row");
    assert!(!topology.exportable);
    assert!(topology.summary.contains("local-only"));
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:inspector", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5RememberedStateInspectorSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.packet, packet);
}

// --- Fixtures ------------------------------------------------------------------------------------

#[test]
fn local_only_fixture_is_visible_but_unexportable() {
    let row = row(LOCAL_ONLY_FIXTURE);
    assert_eq!(row.ownership, OwnershipClass::MachineLocal);
    assert!(!row.exportable);
    assert!(!row.is_portable());
    // Local-only state is still inspectable, comparable, and clearable — just never exported.
    assert!(row.has_action(InspectorActionKind::Inspect));
    assert!(row.has_action(InspectorActionKind::Compare));
    assert!(!row.has_action(InspectorActionKind::Export));
    assert!(row.is_clearable());
}

#[test]
fn portable_fixture_offers_export() {
    let row = row(PORTABLE_EXPORTABLE_FIXTURE);
    assert_eq!(row.ownership, OwnershipClass::Portable);
    assert!(row.exportable);
    assert!(row.is_portable());
    let export = row.action(InspectorActionKind::Export).expect("export");
    assert_eq!(export.boundary, ActionBoundary::SelectedStateClassOnly);
    assert!(export.is_bounded());
}

#[test]
fn bounded_clear_fixture_is_confirmed_and_scoped() {
    let row = row(BOUNDED_CLEAR_FIXTURE);
    let clear = row.action(InspectorActionKind::Clear).expect("clear");
    assert!(clear.is_safe_clear());
    assert_eq!(clear.boundary, ActionBoundary::SelectedStateClassOnly);
    assert!(clear.requires_confirmation);
    assert!(clear.excludes_unrelated_content);
    assert!(clear.excludes_caches);
}

// --- Fail-closed gate drills ---------------------------------------------------------------------

fn row_index(packet: &M5RememberedStateInspector, class: RememberedArtifactClass) -> usize {
    packet
        .rows
        .iter()
        .position(|r| r.artifact_class == class)
        .expect("row present")
}

#[test]
fn exportability_disagreeing_with_ownership_is_rejected() {
    let mut broken = packet();
    let idx = row_index(
        &broken,
        RememberedArtifactClass::WorkspaceAuthorityCheckpoint,
    );
    broken.rows[idx].exportable = true; // local state must not be exportable.
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::ExportabilityMismatch { .. }
    )));
}

#[test]
fn non_exportable_offering_export_is_rejected() {
    let mut broken = packet();
    let idx = row_index(&broken, RememberedArtifactClass::PlaceholderCard);
    broken.rows[idx].available_actions.push(ActionAffordance {
        action: InspectorActionKind::Export,
        command_id: "inspector.placeholder_card.export".to_owned(),
        keyboard_shortcut: "mod+alt+e".to_owned(),
        focus_order: 3,
        accessible_label: "Export the placeholder cards".to_owned(),
        boundary: ActionBoundary::SelectedStateClassOnly,
        requires_confirmation: false,
        excludes_unrelated_content: true,
        excludes_caches: true,
    });
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::NonExportableOffersExport { .. }
    )));
}

#[test]
fn exportable_missing_export_is_rejected() {
    let mut broken = packet();
    let idx = row_index(&broken, RememberedArtifactClass::PortableStatePackage);
    broken.rows[idx]
        .available_actions
        .retain(|a| a.action != InspectorActionKind::Export);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::ExportableMissingExport { .. }
    )));
}

#[test]
fn missing_inspect_action_is_rejected() {
    let mut broken = packet();
    let idx = row_index(
        &broken,
        RememberedArtifactClass::WorkspaceAuthorityCheckpoint,
    );
    broken.rows[idx]
        .available_actions
        .retain(|a| a.action != InspectorActionKind::Inspect);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::MissingInspectAction { .. }
    )));
}

#[test]
fn clear_modeled_as_global_reset_is_rejected() {
    let mut broken = packet();
    let idx = row_index(&broken, RememberedArtifactClass::WindowTopologySnapshot);
    let clear = broken.rows[idx]
        .available_actions
        .iter_mut()
        .find(|a| a.action == InspectorActionKind::Clear)
        .expect("clear");
    clear.boundary = ActionBoundary::GlobalReset;
    let violations = broken.validate();
    // A global-reset clear trips both the unbounded-action and unsafe-clear guards.
    assert!(violations.iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::UnboundedAction { .. }
    )));
    assert!(violations
        .iter()
        .any(|v| matches!(v, M5RememberedStateInspectorViolation::UnsafeClear { .. })));
}

#[test]
fn unconfirmed_clear_is_rejected() {
    let mut broken = packet();
    let idx = row_index(&broken, RememberedArtifactClass::WindowTopologySnapshot);
    let clear = broken.rows[idx]
        .available_actions
        .iter_mut()
        .find(|a| a.action == InspectorActionKind::Clear)
        .expect("clear");
    clear.requires_confirmation = false;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateInspectorViolation::UnsafeClear { .. })));
}

#[test]
fn action_touching_unrelated_content_is_rejected() {
    let mut broken = packet();
    let idx = row_index(&broken, RememberedArtifactClass::PortableStatePackage);
    broken.rows[idx].available_actions[0].excludes_unrelated_content = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::UnboundedAction { .. }
    )));
}

#[test]
fn inaccessible_affordance_is_rejected() {
    let mut broken = packet();
    let idx = row_index(
        &broken,
        RememberedArtifactClass::WorkspaceAuthorityCheckpoint,
    );
    broken.rows[idx].available_actions[0].keyboard_shortcut = "  ".to_owned();
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::InaccessibleAffordance { .. }
    )));
}

#[test]
fn duplicate_focus_order_is_rejected() {
    let mut broken = packet();
    let idx = row_index(
        &broken,
        RememberedArtifactClass::WorkspaceAuthorityCheckpoint,
    );
    broken.rows[idx].available_actions[1].focus_order =
        broken.rows[idx].available_actions[0].focus_order;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::DuplicateFocusOrder { .. }
    )));
}

#[test]
fn missing_consumer_binding_is_rejected() {
    let mut broken = packet();
    broken
        .consumer_bindings
        .retain(|b| b.consumer_surface != InspectorConsumerSurface::Diagnostics);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn consumer_binding_drift_is_rejected() {
    let mut broken = packet();
    broken.consumer_bindings[0].preserves_ownership_labels = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::ConsumerBindingDrift { .. }
    )));
}

#[test]
fn closed_vocabulary_drift_is_rejected() {
    let mut broken = packet();
    broken.action_kinds = vec![InspectorActionKind::Inspect];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RememberedStateInspectorViolation::ClosedVocabularyDrift { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut broken = packet();
    broken.summary.exportable_rows = 99;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RememberedStateInspectorViolation::SummaryMismatch)));
}
