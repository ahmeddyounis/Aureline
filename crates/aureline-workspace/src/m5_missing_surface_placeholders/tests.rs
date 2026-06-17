use super::*;

const EXTENSION_LAYOUT_ONLY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-missing-surface-placeholders/extension_layout_only.json"
));

const FEATURE_PACK_LAYOUT_ONLY_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-missing-surface-placeholders/feature_pack_layout_only.json"
));

const REMOTE_REOPEN_AS_CONTEXT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-missing-surface-placeholders/remote_reopen_as_context.json"
));

const SERVICE_MANUAL_REVIEW_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-missing-surface-placeholders/service_manual_review.json"
));

fn packet() -> M5MissingSurfacePlaceholders {
    current_m5_missing_surface_placeholders().expect("packet parses")
}

fn placeholder(json: &str) -> MissingSurfacePlaceholderCard {
    serde_json::from_str(json).expect("fixture parses")
}

// --- Embedded packet -----------------------------------------------------------------------------

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(
        packet.schema_version,
        M5_MISSING_SURFACE_PLACEHOLDERS_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        M5_MISSING_SURFACE_PLACEHOLDERS_RECORD_KIND
    );
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn embedded_packet_round_trips_byte_stable_shape() {
    let packet = packet();
    let encoded = serde_json::to_string(&packet).expect("serializes");
    let decoded: M5MissingSurfacePlaceholders =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded, packet);
}

#[test]
fn packet_governs_the_placeholder_card_artifact_class() {
    let packet = packet();
    assert_eq!(
        packet.governs_artifact_class,
        RememberedArtifactClass::PlaceholderCard
    );
}

#[test]
fn summary_counts_match_placeholders() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.summary.placeholders, 5);
    assert_eq!(packet.summary.extension_missing, 1);
    assert_eq!(packet.summary.feature_pack_missing, 1);
    assert_eq!(packet.summary.remote_target_missing, 2);
    assert_eq!(packet.summary.backing_service_missing, 1);
    assert_eq!(packet.summary.layout_only_placeholders, 3);
    assert_eq!(packet.summary.manual_review_placeholders, 2);
    assert_eq!(packet.summary.slot_preserved_placeholders, 4);
    assert_eq!(packet.summary.reopened_as_context_placeholders, 1);
    assert_eq!(packet.summary.affected_pane_roles, 5);
}

#[test]
fn every_placeholder_agrees_with_the_gate() {
    let packet = packet();
    assert!(packet.all_placeholders_gate_consistent());
    for p in &packet.placeholders {
        assert!(p.gate_consistent(), "{}", p.placeholder_id);
        assert_eq!(p.published_fidelity, p.achieved_fidelity());
        assert_eq!(p.downgrade_reasons, p.computed_downgrade_reasons());
        assert_eq!(p.recovery_path, p.computed_recovery_path());
    }
}

#[test]
fn vocabularies_are_reused_from_the_matrix_and_restore_provenance() {
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
    assert_eq!(packet.reentry_surfaces, ReentrySurface::ALL.to_vec());
    assert_eq!(
        packet.missing_dependency_behaviors,
        MissingDependencyBehavior::ALL.to_vec()
    );
}

#[test]
fn no_placeholder_publishes_an_exact_restore() {
    let packet = packet();
    for p in &packet.placeholders {
        assert_ne!(
            p.published_fidelity,
            RestoreFidelityClass::ExactRestore,
            "{}",
            p.placeholder_id
        );
        // A missing surface is always narrowed below the fidelity it declared.
        assert!(p.is_downgraded(), "{}", p.placeholder_id);
    }
}

#[test]
fn every_placeholder_records_a_missing_dependency() {
    let packet = packet();
    for p in &packet.placeholders {
        assert!(p.dependency_condition.is_missing(), "{}", p.placeholder_id);
    }
}

#[test]
fn missing_surfaces_never_silently_delete_layout() {
    let packet = packet();
    for p in &packet.placeholders {
        assert!(
            p.substitution_behavior.preserves_slot(),
            "{}",
            p.placeholder_id
        );
    }
}

#[test]
fn placeholders_preserve_role_slot_and_provenance() {
    let packet = packet();
    for p in &packet.placeholders {
        assert!(!p.pane_id.trim().is_empty(), "{}", p.placeholder_id);
        assert!(!p.slot_path.trim().is_empty(), "{}", p.placeholder_id);
        assert!(
            p.last_known_provenance.is_complete(),
            "{}",
            p.placeholder_id
        );
        assert!(p.has_required_exclusions(), "{}", p.placeholder_id);
    }
}

#[test]
fn every_placeholder_offers_open_details_and_its_class_recovery_action() {
    let packet = packet();
    for p in &packet.placeholders {
        assert!(
            p.has_action(PlaceholderActionKind::OpenDetails),
            "{}",
            p.placeholder_id
        );
        assert!(
            p.has_action(p.primary_recovery_action()),
            "{}",
            p.placeholder_id
        );
    }
}

#[test]
fn reopened_as_context_placeholders_offer_the_reopen_action() {
    let packet = packet();
    for p in &packet.placeholders {
        if p.substitution_behavior == MissingDependencyBehavior::ReopenAsContext {
            assert!(
                p.has_action(PlaceholderActionKind::ReopenAsContext),
                "{}",
                p.placeholder_id
            );
        }
    }
}

#[test]
fn every_affordance_is_accessible_and_scoped() {
    let packet = packet();
    for p in &packet.placeholders {
        let mut focus = std::collections::BTreeSet::new();
        for affordance in &p.available_actions {
            assert!(affordance.is_accessible(), "{}", p.placeholder_id);
            assert!(affordance.scoped_to_slot, "{}", p.placeholder_id);
            assert!(focus.insert(affordance.focus_order), "{}", p.placeholder_id);
        }
    }
}

#[test]
fn every_placeholder_narration_is_sensible() {
    let packet = packet();
    for p in &packet.placeholders {
        assert!(p.narration.is_sensible(), "{}", p.placeholder_id);
    }
}

#[test]
fn missing_dependency_class_maps_to_a_recovery_action() {
    assert_eq!(
        MissingDependencyClass::Extension.primary_recovery_action(),
        PlaceholderActionKind::InstallDependency
    );
    assert_eq!(
        MissingDependencyClass::FeaturePack.primary_recovery_action(),
        PlaceholderActionKind::InstallDependency
    );
    assert_eq!(
        MissingDependencyClass::RemoteTarget.primary_recovery_action(),
        PlaceholderActionKind::ReconnectRemote
    );
    assert_eq!(
        MissingDependencyClass::BackingService.primary_recovery_action(),
        PlaceholderActionKind::RetryService
    );
}

#[test]
fn diagnostics_view_names_classes_and_roles() {
    let packet = packet();
    let view = packet.diagnostics_view();
    assert_eq!(view.rows.len(), 5);
    assert_eq!(view.total, 5);
    assert_eq!(view.layout_only_count, 3);
    assert_eq!(view.manual_review_count, 2);
    // Every missing-dependency class observed is named with its affected count.
    let remote = view
        .by_missing_class
        .iter()
        .find(|c| c.missing_dependency_class == "remote_target")
        .expect("remote class row");
    assert_eq!(remote.count, 2);
    // Affected pane roles are named for support packets.
    assert_eq!(view.by_pane_role.len(), 5);
    for row in &view.rows {
        assert!(!row.summary.trim().is_empty());
        assert!(!row.actions.is_empty());
    }
}

#[test]
fn placeholders_for_role_filters_by_role() {
    let packet = packet();
    assert_eq!(packet.placeholders_for_role(PaneRole::Profiler).count(), 1);
    assert_eq!(packet.placeholders_for_role(PaneRole::Editor).count(), 0);
}

#[test]
fn parity_surfaces_carry_the_same_record() {
    let packet = packet();
    for surface in PlaceholderConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding {surface:?}"
        );
    }
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:missing-surface", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.record_packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5MissingSurfacePlaceholdersSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.record, packet);
}

// --- Fixtures ------------------------------------------------------------------------------------

#[test]
fn extension_fixture_is_slot_preserving_layout_only() {
    let card = placeholder(EXTENSION_LAYOUT_ONLY_FIXTURE);
    assert_eq!(card.pane_role, PaneRole::Preview);
    assert_eq!(
        card.missing_dependency_class,
        MissingDependencyClass::Extension
    );
    assert_eq!(card.published_fidelity, RestoreFidelityClass::LayoutOnly);
    assert_eq!(
        card.substitution_behavior,
        MissingDependencyBehavior::PlaceholderSlotPreserved
    );
    assert!(card.gate_consistent());
    assert_eq!(card.recovery_path, RecoveryPath::RelocateDependency);
    assert!(card.has_action(PlaceholderActionKind::InstallDependency));
}

#[test]
fn feature_pack_fixture_is_capped_at_layout_only_despite_migratable_schema() {
    let card = placeholder(FEATURE_PACK_LAYOUT_ONLY_FIXTURE);
    assert_eq!(
        card.missing_dependency_class,
        MissingDependencyClass::FeaturePack
    );
    assert_eq!(
        card.schema_condition,
        SchemaCondition::SchemaForwardMigratable
    );
    // The dependency ceiling caps it at layout-only even though the schema would migrate.
    assert_eq!(card.published_fidelity, RestoreFidelityClass::LayoutOnly);
    assert!(card.gate_consistent());
    assert_eq!(
        card.downgrade_reasons,
        vec![
            DowngradeReason::SchemaDrift,
            DowngradeReason::DependencyMissing
        ]
    );
}

#[test]
fn remote_fixture_reopens_as_context_with_slot_preserved() {
    let card = placeholder(REMOTE_REOPEN_AS_CONTEXT_FIXTURE);
    assert_eq!(card.pane_role, PaneRole::QueryConsole);
    assert_eq!(
        card.missing_dependency_class,
        MissingDependencyClass::RemoteTarget
    );
    assert_eq!(
        card.substitution_behavior,
        MissingDependencyBehavior::ReopenAsContext
    );
    assert!(card.substitution_behavior.preserves_slot());
    assert!(card.has_action(PlaceholderActionKind::ReopenAsContext));
    assert!(card.has_action(PlaceholderActionKind::ReconnectRemote));
    assert!(card.gate_consistent());
}

#[test]
fn service_fixture_is_held_for_manual_review() {
    let card = placeholder(SERVICE_MANUAL_REVIEW_FIXTURE);
    assert_eq!(card.pane_role, PaneRole::IncidentWorkspace);
    assert_eq!(
        card.missing_dependency_class,
        MissingDependencyClass::BackingService
    );
    assert_eq!(
        card.dependency_condition,
        DependencyCondition::DependencyRootMissing
    );
    assert_eq!(card.published_fidelity, RestoreFidelityClass::ManualReview);
    assert_eq!(card.recovery_path, RecoveryPath::ManualReview);
    assert!(card.substitution_behavior.preserves_slot());
    assert!(card.gate_consistent());
}

// --- Fail-closed gate drills ---------------------------------------------------------------------

fn index(packet: &M5MissingSurfacePlaceholders, id: &str) -> usize {
    packet
        .placeholders
        .iter()
        .position(|p| p.placeholder_id == id)
        .expect("placeholder present")
}

#[test]
fn silent_layout_delete_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].substitution_behavior = MissingDependencyBehavior::SilentDelete;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::SilentLayoutDelete { .. }
    )));
}

#[test]
fn publishing_exact_for_a_missing_surface_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].published_fidelity = RestoreFidelityClass::ExactRestore;
    let violations = broken.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingSurfacePublishedExact { .. }
    )));
}

#[test]
fn overstated_fidelity_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    // Claim compatible where the dependency ceiling only permits layout-only.
    broken.placeholders[idx].published_fidelity = RestoreFidelityClass::CompatibleRestore;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::OverstatedFidelity { .. }
    )));
}

#[test]
fn a_present_dependency_placeholder_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].dependency_condition = DependencyCondition::DependenciesPresent;
    let violations = broken.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::PlaceholderWithoutMissingDependency { .. }
    )));
}

#[test]
fn erased_provenance_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx]
        .last_known_provenance
        .last_attached_ref = "  ".to_owned();
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::ProvenanceErased { .. }
    )));
}

#[test]
fn missing_redaction_exclusion_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx]
        .redaction_class
        .retain(|e| *e != RedactionExclusion::ExcludesMachineLocalAnchors);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingRedactionExclusion { .. }
    )));
}

#[test]
fn missing_open_details_action_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx]
        .available_actions
        .retain(|a| a.action != PlaceholderActionKind::OpenDetails);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingOpenDetailsAction { .. }
    )));
}

#[test]
fn dropping_the_class_recovery_action_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx]
        .available_actions
        .retain(|a| a.action != PlaceholderActionKind::InstallDependency);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingRecoveryAction { .. }
    )));
}

#[test]
fn reopen_as_context_without_the_action_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:query_console_remote_missing");
    broken.placeholders[idx]
        .available_actions
        .retain(|a| a.action != PlaceholderActionKind::ReopenAsContext);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingReopenAsContextAction { .. }
    )));
}

#[test]
fn inaccessible_narration_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].narration.keyboard_reachable = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::InaccessibleNarration { .. }
    )));
}

#[test]
fn unscoped_action_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].available_actions[0].scoped_to_slot = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::UnscopedAction { .. }
    )));
}

#[test]
fn duplicate_focus_order_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:query_console_remote_missing");
    broken.placeholders[idx].available_actions[1].focus_order =
        broken.placeholders[idx].available_actions[0].focus_order;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::DuplicateFocusOrder { .. }
    )));
}

#[test]
fn downgrade_reasons_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].downgrade_reasons = vec![DowngradeReason::SchemaDrift];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::DowngradeReasonsMismatch { .. }
    )));
}

#[test]
fn recovery_path_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = index(&broken, "placeholder:preview_extension_missing");
    broken.placeholders[idx].recovery_path = RecoveryPath::NoneNeeded;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::RecoveryPathMismatch { .. }
    )));
}

#[test]
fn wrong_governed_artifact_class_is_rejected() {
    let mut broken = packet();
    broken.governs_artifact_class = RememberedArtifactClass::WindowTopologySnapshot;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::WrongGovernedArtifactClass { .. }
    )));
}

#[test]
fn closed_vocabulary_drift_is_rejected() {
    let mut broken = packet();
    broken.missing_dependency_classes = vec![MissingDependencyClass::Extension];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::ClosedVocabularyDrift { .. }
    )));
}

#[test]
fn missing_consumer_binding_is_rejected() {
    let mut broken = packet();
    broken
        .consumer_bindings
        .retain(|b| b.consumer_surface != PlaceholderConsumerSurface::SupportPacket);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn consumer_binding_drift_is_rejected() {
    let mut broken = packet();
    broken.consumer_bindings[0].preserves_pane_role_labels = false;
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::ConsumerBindingDrift { .. }
    )));
}

#[test]
fn duplicate_pane_slot_is_rejected() {
    let mut broken = packet();
    let a = index(&broken, "placeholder:preview_extension_missing");
    let b = index(&broken, "placeholder:notebook_feature_pack_missing");
    broken.placeholders[b].pane_id = broken.placeholders[a].pane_id.clone();
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5MissingSurfacePlaceholdersViolation::DuplicatePaneSlot { .. }
    )));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut broken = packet();
    broken.summary.remote_target_missing = 99;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5MissingSurfacePlaceholdersViolation::SummaryMismatch)));
}
