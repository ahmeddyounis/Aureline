use super::*;

const SCHEMA_JUMP_COMPATIBLE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/schema_jump_forward_migrated.json"
));

const SCHEMA_JUMP_UNMIGRATABLE_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/schema_jump_unmigratable.json"
));

const FOREIGN_MACHINE_IMPORT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/foreign_machine_import.json"
));

const MIXED_CHANNEL_IMPORT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/mixed_channel_import.json"
));

const MONITOR_DETACH_REATTACH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-state-migration-and-topology-remap/monitor_detach_reattach.json"
));

fn packet() -> M5StateMigrationAndTopologyRemap {
    current_m5_state_migration_and_topology_remap().expect("packet parses")
}

fn event(json: &str) -> MigrationRemapEvent {
    serde_json::from_str(json).expect("fixture parses")
}

// --- Embedded packet -----------------------------------------------------------------------------

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_STATE_MIGRATION_REMAP_SCHEMA_VERSION
    );
    assert_eq!(packet.record_kind, M5_STATE_MIGRATION_REMAP_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn embedded_packet_round_trips_byte_stable_shape() {
    let packet = packet();
    let encoded = serde_json::to_string(&packet).expect("serializes");
    let decoded: M5StateMigrationAndTopologyRemap =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded, packet);
}

#[test]
fn every_event_kind_is_covered() {
    let packet = packet();
    for kind in MigrationEventKind::ALL {
        assert!(
            packet.events.iter().any(|e| e.event_kind == kind),
            "no event covers {kind:?}"
        );
    }
}

#[test]
fn summary_counts_match_events() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.summary.events, 7);
    assert_eq!(packet.summary.exact_events, 1);
    assert_eq!(packet.summary.compatible_events, 3);
    assert_eq!(packet.summary.layout_only_events, 2);
    assert_eq!(packet.summary.manual_review_events, 1);
    assert_eq!(packet.summary.downgraded_events, 6);
    assert_eq!(packet.summary.narrowed_events, 6);
    assert_eq!(packet.summary.schema_migration_events, 3);
    assert_eq!(packet.summary.imported_package_events, 2);
    assert_eq!(packet.summary.display_topology_remap_events, 2);
    assert_eq!(packet.summary.foreign_origin_events, 2);
}

#[test]
fn every_event_agrees_with_the_gate() {
    let packet = packet();
    assert!(packet.all_events_gate_consistent());
    for event in &packet.events {
        assert!(event.gate_consistent(), "{}", event.event_id);
        assert_eq!(event.published_fidelity, event.achieved_fidelity());
        assert_eq!(event.downgrade_reasons, event.computed_downgrade_reasons());
        assert_eq!(event.recovery_path, event.computed_recovery_path());
    }
}

#[test]
fn fidelity_and_condition_labels_are_reused_from_the_matrix_vocabulary() {
    let packet = packet();
    assert_eq!(
        packet.restore_fidelity_classes,
        RestoreFidelityClass::ALL.to_vec()
    );
    assert_eq!(
        packet.artifact_classes,
        RememberedArtifactClass::ALL.to_vec()
    );
    assert_eq!(
        packet.redaction_exclusions,
        RedactionExclusion::ALL.to_vec()
    );
    assert_eq!(packet.schema_conditions, SchemaCondition::ALL.to_vec());
    assert_eq!(packet.topology_conditions, TopologyCondition::ALL.to_vec());
    assert_eq!(packet.downgrade_reasons, DowngradeReason::ALL.to_vec());
    assert_eq!(packet.recovery_paths, RecoveryPath::ALL.to_vec());
}

#[test]
fn all_four_fidelity_labels_appear_across_events() {
    let packet = packet();
    let published: std::collections::BTreeSet<RestoreFidelityClass> =
        packet.events.iter().map(|e| e.published_fidelity).collect();
    for class in RestoreFidelityClass::ALL {
        assert!(published.contains(&class), "no event publishes {class:?}");
    }
}

#[test]
fn every_event_has_complete_redaction_and_detail() {
    let packet = packet();
    for event in &packet.events {
        assert!(event.has_required_exclusions(), "{}", event.event_id);
        assert!(event.detail_matches_kind(), "{}", event.event_id);
        assert!(event.schema_detail_consistent(), "{}", event.event_id);
        assert!(event.imported_detail_consistent(), "{}", event.event_id);
        assert!(event.remap_detail_consistent(), "{}", event.event_id);
        // A provenance record is metadata only: it always excludes machine-local anchors.
        assert!(event
            .redaction_class
            .contains(&RedactionExclusion::ExcludesMachineLocalAnchors));
    }
}

#[test]
fn every_event_offers_open_details() {
    let packet = packet();
    for event in &packet.events {
        assert!(
            event.has_action(MigrationRemapActionKind::OpenDetails),
            "{}",
            event.event_id
        );
    }
}

#[test]
fn narrowed_events_preserve_compare_and_recovery_actions() {
    let packet = packet();
    for event in &packet.events {
        if event.requires_recovery_actions() {
            assert!(
                event.has_action(MigrationRemapActionKind::Compare),
                "{}",
                event.event_id
            );
            assert!(
                event.has_action(MigrationRemapActionKind::RecoveryNextStep),
                "{}",
                event.event_id
            );
            assert!(event.recovery_path.is_offered(), "{}", event.event_id);
            assert!(!event.caveats.is_empty(), "{}", event.event_id);
            assert!(!event.narrowed_fields.is_empty(), "{}", event.event_id);
        }
    }
}

#[test]
fn every_affordance_is_accessible_and_keyboard_complete() {
    let packet = packet();
    for event in &packet.events {
        let mut focus = std::collections::BTreeSet::new();
        for affordance in &event.available_actions {
            assert!(affordance.is_accessible(), "{}", event.event_id);
            assert!(affordance.scoped_to_event, "{}", event.event_id);
            assert!(focus.insert(affordance.focus_order), "{}", event.event_id);
        }
    }
}

#[test]
fn no_event_silently_deletes_layout_or_discards_prior_artifact() {
    let packet = packet();
    for event in &packet.events {
        assert!(
            event.missing_dependency_behavior.preserves_slot(),
            "{}",
            event.event_id
        );
        assert!(
            event.prior_artifact_availability.preserves_prior(),
            "{}",
            event.event_id
        );
    }
}

#[test]
fn foreign_imports_disclose_origin_and_machine_local_exclusions() {
    let packet = packet();
    let foreign: Vec<&MigrationRemapEvent> = packet
        .events
        .iter()
        .filter(|e| {
            e.imported_package
                .as_ref()
                .is_some_and(|d| d.origin.is_foreign())
        })
        .collect();
    assert_eq!(foreign.len(), 2);
    for event in foreign {
        let detail = event.imported_package.as_ref().expect("import detail");
        assert!(detail.machine_local_excluded, "{}", event.event_id);
        assert!(detail.disclosed_before_restore, "{}", event.event_id);
        assert!(
            detail.foreign_disclosure_is_complete(),
            "{}",
            event.event_id
        );
        assert!(event
            .redaction_class
            .contains(&RedactionExclusion::ExcludesMachineLocalAnchors));
    }
}

#[test]
fn mixed_channel_imports_never_claim_a_clean_schema_match() {
    let packet = packet();
    for event in &packet.events {
        if let Some(detail) = &event.imported_package {
            if detail.channel_match.is_mixed() {
                assert_ne!(
                    event.schema_condition,
                    SchemaCondition::SchemaMatch,
                    "{}",
                    event.event_id
                );
                assert_ne!(
                    event.published_fidelity,
                    RestoreFidelityClass::ExactRestore,
                    "{}",
                    event.event_id
                );
            }
        }
    }
}

#[test]
fn topology_remaps_read_as_changed_topology_not_corruption() {
    let packet = packet();
    for event in &packet.events {
        if event.event_kind == MigrationEventKind::DisplayTopologyRemap {
            let detail = event.display_topology_remap.as_ref().expect("remap detail");
            assert!(detail.is_recordable(), "{}", event.event_id);
            assert!(event.topology_condition.is_changed(), "{}", event.event_id);
            // A remap is a deliberate compatibility downgrade, never a manual-review corruption.
            assert_ne!(
                event.published_fidelity,
                RestoreFidelityClass::ManualReview,
                "{}",
                event.event_id
            );
            assert_eq!(
                event.recovery_path,
                RecoveryPath::ReopenAsContext,
                "{}",
                event.event_id
            );
        }
    }
}

#[test]
fn the_exact_event_is_a_clean_no_migration_baseline() {
    let packet = packet();
    let exact = packet
        .events
        .iter()
        .find(|e| e.is_exact())
        .expect("exact event");
    assert_eq!(exact.event_kind, MigrationEventKind::SchemaMigration);
    let detail = exact.schema_migration.as_ref().expect("schema detail");
    assert_eq!(detail.result_class, MigrationResultClass::SchemaUnchanged);
    assert_eq!(detail.from_schema_version, detail.to_schema_version);
    assert_eq!(detail.migration_steps, 0);
    assert_eq!(exact.schema_condition, SchemaCondition::SchemaMatch);
    assert!(exact.downgrade_reasons.is_empty());
    assert!(exact.caveats.is_empty());
    assert_eq!(exact.recovery_path, RecoveryPath::NoneNeeded);
}

#[test]
fn parity_surfaces_carry_the_same_record() {
    let packet = packet();
    for surface in MigrationRemapConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding {surface:?}"
        );
    }
}

#[test]
fn event_view_is_plain_language_and_distinguishes_fidelity() {
    let packet = packet();
    let view = packet.event_view();
    assert_eq!(view.rows.len(), 7);
    assert_eq!(view.exact_count, 1);
    assert_eq!(view.narrowed_count, 6);
    assert_eq!(view.manual_review_count, 1);
    for row in &view.rows {
        assert!(!row.summary.trim().is_empty());
        assert!(!row.actions.is_empty());
        assert!(!row.prior_artifact_availability.trim().is_empty());
    }
    let remap = view
        .rows
        .iter()
        .find(|r| r.event_id == "remap:monitor_detach_reattach")
        .expect("remap row");
    assert_eq!(remap.published_fidelity, "compatible_restore");
    assert!(remap.narrowed);
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:state-migration-remap", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.record_packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5StateMigrationRemapSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.record, packet);
}

// --- Fixtures ------------------------------------------------------------------------------------

#[test]
fn schema_jump_compatible_fixture_is_narrowed_to_compatible() {
    let event = event(SCHEMA_JUMP_COMPATIBLE_FIXTURE);
    assert_eq!(event.event_kind, MigrationEventKind::SchemaMigration);
    let detail = event.schema_migration.as_ref().expect("schema detail");
    assert_eq!(detail.result_class, MigrationResultClass::ForwardMigrated);
    assert!(detail.to_schema_version > detail.from_schema_version);
    assert_eq!(
        event.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert!(event.is_downgraded());
    assert!(event.gate_consistent());
    assert!(event.schema_detail_consistent());
    assert_eq!(event.recovery_path, RecoveryPath::RestoreCompatibly);
    assert!(event.prior_artifact_availability.preserves_prior());
}

#[test]
fn schema_jump_unmigratable_fixture_is_held_for_review() {
    let event = event(SCHEMA_JUMP_UNMIGRATABLE_FIXTURE);
    let detail = event.schema_migration.as_ref().expect("schema detail");
    assert_eq!(detail.result_class, MigrationResultClass::Unmigratable);
    assert_eq!(event.published_fidelity, RestoreFidelityClass::ManualReview);
    assert!(event.gate_consistent());
    assert_eq!(event.recovery_path, RecoveryPath::ManualReview);
    assert_eq!(
        event.prior_artifact_availability,
        PriorArtifactAvailability::PriorArtifactRetained
    );
}

#[test]
fn foreign_machine_import_fixture_discloses_origin_and_narrows() {
    let event = event(FOREIGN_MACHINE_IMPORT_FIXTURE);
    assert_eq!(
        event.event_kind,
        MigrationEventKind::ImportedPackageProvenance
    );
    let detail = event.imported_package.as_ref().expect("import detail");
    assert_eq!(detail.origin, PackageOriginClass::ForeignMachine);
    assert!(detail.origin.is_foreign());
    assert!(detail.machine_local_excluded);
    assert!(detail.disclosed_before_restore);
    assert_eq!(
        detail.path_handling,
        PathHandlingPosture::PathsRemappedToLocalRoots
    );
    assert_eq!(event.published_fidelity, RestoreFidelityClass::LayoutOnly);
    assert!(event.imported_detail_consistent());
    assert!(event.gate_consistent());
    assert_eq!(event.recovery_path, RecoveryPath::RelocateDependency);
}

#[test]
fn mixed_channel_import_fixture_is_compatible_not_exact() {
    let event = event(MIXED_CHANNEL_IMPORT_FIXTURE);
    let detail = event.imported_package.as_ref().expect("import detail");
    assert_eq!(detail.channel_match, ChannelMatch::MixedChannel);
    assert!(detail.origin.is_foreign());
    assert_ne!(event.schema_condition, SchemaCondition::SchemaMatch);
    assert_eq!(
        event.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert!(event.gate_consistent());
    assert_eq!(event.recovery_path, RecoveryPath::RestoreCompatibly);
}

#[test]
fn monitor_detach_reattach_fixture_is_a_compatible_remap() {
    let event = event(MONITOR_DETACH_REATTACH_FIXTURE);
    assert_eq!(event.event_kind, MigrationEventKind::DisplayTopologyRemap);
    let detail = event.display_topology_remap.as_ref().expect("remap detail");
    assert!(detail.triggers.contains(&RemapTrigger::MonitorDetached));
    assert!(detail.triggers.contains(&RemapTrigger::MonitorReattached));
    assert!(detail.materially_altered_placement);
    assert_eq!(
        detail.resolution,
        RemapResolution::PlacementAdaptedToAvailableDisplays
    );
    assert_eq!(
        event.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert!(event.remap_detail_consistent());
    assert!(event.gate_consistent());
    // A platform remap is a compatibility downgrade, never a manual-review corruption.
    assert_ne!(event.published_fidelity, RestoreFidelityClass::ManualReview);
}

// --- Fail-closed gate drills ---------------------------------------------------------------------

fn event_index(packet: &M5StateMigrationAndTopologyRemap, event_id: &str) -> usize {
    packet
        .events
        .iter()
        .position(|e| e.event_id == event_id)
        .expect("event present")
}

#[test]
fn overstated_fidelity_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx].published_fidelity = RestoreFidelityClass::ExactRestore;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::OverstatedFidelity { .. })));
}

#[test]
fn discarded_prior_artifact_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx].prior_artifact_availability =
        PriorArtifactAvailability::PriorArtifactDiscarded;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::DiscardedPriorArtifact { .. }
    )));
}

#[test]
fn silent_layout_delete_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "remap:incompatible_topology_remap");
    broken.events[idx].missing_dependency_behavior = MissingDependencyBehavior::SilentDelete;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::SilentLayoutDelete { .. })));
}

#[test]
fn migration_result_disagreeing_with_schema_condition_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    // ForwardMigrated requires schema_forward_migratable; force a match and the gate must reject it.
    broken.events[idx].schema_condition = SchemaCondition::SchemaMatch;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::SchemaMigrationInconsistent { .. }
    )));
}

#[test]
fn mixed_channel_claiming_schema_match_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:mixed_channel_import");
    broken.events[idx].schema_condition = SchemaCondition::SchemaMatch;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::ImportedPackageInconsistent { .. }
    )));
}

#[test]
fn foreign_import_hiding_machine_local_exclusion_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:foreign_machine_import");
    broken.events[idx]
        .imported_package
        .as_mut()
        .expect("import detail")
        .machine_local_excluded = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::ImportedPackageInconsistent { .. }
    )));
}

#[test]
fn remap_not_materially_altering_placement_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "remap:monitor_detach_reattach");
    broken.events[idx]
        .display_topology_remap
        .as_mut()
        .expect("remap detail")
        .materially_altered_placement = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::TopologyRemapInconsistent { .. }
    )));
}

#[test]
fn detail_block_not_matching_kind_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx].schema_migration = None;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::DetailKindMismatch { .. })));
}

#[test]
fn missing_redaction_exclusion_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx]
        .redaction_class
        .retain(|e| *e != RedactionExclusion::ExcludesMachineLocalAnchors);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::MissingRedactionExclusion { .. }
    )));
}

#[test]
fn missing_open_details_action_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx]
        .available_actions
        .retain(|a| a.action != MigrationRemapActionKind::OpenDetails);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::MissingOpenDetailsAction { .. }
    )));
}

#[test]
fn narrowed_event_dropping_recovery_actions_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx]
        .available_actions
        .retain(|a| a.action != MigrationRemapActionKind::RecoveryNextStep);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::MissingRecoveryActions { .. }
    )));
}

#[test]
fn downgrade_reasons_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx].downgrade_reasons = vec![DowngradeReason::TopologyChanged];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::DowngradeReasonsMismatch { .. }
    )));
}

#[test]
fn recovery_path_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:schema_jump_compatible");
    broken.events[idx].recovery_path = RecoveryPath::NoneNeeded;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::RecoveryPathMismatch { .. }
    )));
}

#[test]
fn exact_event_with_a_caveat_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx]
        .caveats
        .push("this should not be here".to_owned());
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::ExactEventNotClean { .. })));
}

#[test]
fn inaccessible_action_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx].available_actions[0].keyboard_shortcut = "  ".to_owned();
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::InaccessibleAction { .. })));
}

#[test]
fn unscoped_action_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx].available_actions[0].scoped_to_event = false;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::UnscopedAction { .. })));
}

#[test]
fn duplicate_focus_order_is_rejected() {
    let mut broken = packet();
    let idx = event_index(&broken, "migration:no_migration_baseline");
    broken.events[idx].available_actions[1].focus_order =
        broken.events[idx].available_actions[0].focus_order;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::DuplicateFocusOrder { .. }
    )));
}

#[test]
fn missing_consumer_binding_is_rejected() {
    let mut broken = packet();
    broken
        .consumer_bindings
        .retain(|b| b.consumer_surface != MigrationRemapConsumerSurface::SupportPacket);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn consumer_binding_drift_is_rejected() {
    let mut broken = packet();
    broken.consumer_bindings[0].preserves_remap_labels = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::ConsumerBindingDrift { .. }
    )));
}

#[test]
fn closed_vocabulary_drift_is_rejected() {
    let mut broken = packet();
    broken.event_kinds = vec![MigrationEventKind::SchemaMigration];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::ClosedVocabularyDrift { .. }
    )));
}

#[test]
fn missing_event_kind_coverage_is_rejected() {
    let mut broken = packet();
    broken
        .events
        .retain(|e| e.event_kind != MigrationEventKind::DisplayTopologyRemap);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5StateMigrationRemapViolation::MissingEventKindCoverage { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut broken = packet();
    broken.summary.exact_events = 99;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5StateMigrationRemapViolation::SummaryMismatch)));
}
