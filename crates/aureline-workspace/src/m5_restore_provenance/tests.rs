use super::*;

const EXACT_DESKTOP_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-restore-provenance/exact_desktop_restore.json"
));

const COMPATIBLE_IMPORT_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-restore-provenance/compatible_import.json"
));

const MANUAL_REVIEW_CRASH_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-restore-provenance/manual_review_crash_recovery.json"
));

const LAYOUT_ONLY_HANDOFF_FIXTURE: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/workspace/m5/m5-restore-provenance/layout_only_companion_handoff.json"
));

fn packet() -> M5RestoreProvenance {
    current_m5_restore_provenance().expect("packet parses")
}

fn card(json: &str) -> RestoreProvenanceCard {
    serde_json::from_str(json).expect("fixture parses")
}

// --- Embedded packet -----------------------------------------------------------------------------

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_RESTORE_PROVENANCE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_RESTORE_PROVENANCE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn embedded_packet_round_trips_byte_stable_shape() {
    let packet = packet();
    let encoded = serde_json::to_string(&packet).expect("serializes");
    let decoded: M5RestoreProvenance = serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded, packet);
}

#[test]
fn one_card_per_reentry_surface() {
    let packet = packet();
    for surface in ReentrySurface::ALL {
        assert!(
            packet.card(surface).is_some(),
            "missing card for {surface:?}"
        );
    }
}

#[test]
fn summary_counts_match_cards() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
    assert_eq!(packet.summary.cards, 5);
    assert_eq!(packet.summary.exact_restore_cards, 1);
    assert_eq!(packet.summary.compatible_restore_cards, 2);
    assert_eq!(packet.summary.layout_only_cards, 1);
    assert_eq!(packet.summary.manual_review_cards, 1);
    assert_eq!(packet.summary.downgraded_cards, 2);
    assert_eq!(packet.summary.narrowed_cards, 4);
    assert_eq!(packet.summary.handoff_cards, 1);
}

#[test]
fn every_card_agrees_with_the_gate() {
    let packet = packet();
    assert!(packet.all_cards_gate_consistent());
    for card in &packet.cards {
        assert!(card.gate_consistent(), "{}", card.card_id);
        assert_eq!(card.published_fidelity, card.achieved_fidelity());
        assert_eq!(card.downgrade_reasons, card.computed_downgrade_reasons());
        assert_eq!(card.recovery_path, card.computed_recovery_path());
    }
}

#[test]
fn fidelity_labels_are_reused_from_the_matrix_vocabulary() {
    let packet = packet();
    // The four fidelity classes are the canonical matrix vocabulary, in canonical order.
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
}

#[test]
fn all_four_fidelity_labels_appear_across_surfaces() {
    let packet = packet();
    let published: std::collections::BTreeSet<RestoreFidelityClass> =
        packet.cards.iter().map(|c| c.published_fidelity).collect();
    for class in RestoreFidelityClass::ALL {
        assert!(published.contains(&class), "no card publishes {class:?}");
    }
}

#[test]
fn every_card_has_complete_provenance_and_redaction() {
    let packet = packet();
    for card in &packet.cards {
        assert!(card.producer.is_complete(), "{}", card.card_id);
        assert_ne!(card.restored_schema_version, 0);
        assert!(card.has_required_exclusions(), "{}", card.card_id);
        // A provenance record is metadata only: it always excludes machine-local anchors.
        assert!(card
            .redaction_class
            .contains(&RedactionExclusion::ExcludesMachineLocalAnchors));
    }
}

#[test]
fn every_card_offers_open_details() {
    let packet = packet();
    for card in &packet.cards {
        assert!(
            card.has_action(ProvenanceActionKind::OpenDetails),
            "{}",
            card.card_id
        );
    }
}

#[test]
fn narrowed_cards_preserve_compare_and_recovery_actions() {
    let packet = packet();
    for card in &packet.cards {
        if card.requires_recovery_actions() {
            assert!(
                card.has_action(ProvenanceActionKind::Compare),
                "{}",
                card.card_id
            );
            assert!(
                card.has_action(ProvenanceActionKind::RecoveryNextStep),
                "{}",
                card.card_id
            );
            assert!(card.recovery_path.is_offered(), "{}", card.card_id);
            assert!(!card.caveats.is_empty(), "{}", card.card_id);
            assert!(!card.narrowed_fields.is_empty(), "{}", card.card_id);
        }
    }
}

#[test]
fn every_affordance_is_accessible_and_keyboard_complete() {
    let packet = packet();
    for card in &packet.cards {
        let mut focus = std::collections::BTreeSet::new();
        for affordance in &card.available_actions {
            assert!(affordance.is_accessible(), "{}", card.card_id);
            assert!(affordance.scoped_to_event, "{}", card.card_id);
            assert!(focus.insert(affordance.focus_order), "{}", card.card_id);
        }
    }
}

#[test]
fn handoff_card_cannot_imply_a_full_restore() {
    let packet = packet();
    let handoff = packet
        .card(ReentrySurface::CompanionBrowserReentry)
        .expect("companion card");
    assert_eq!(handoff.source, RestoreSource::BrowserCompanionHandoff);
    assert!(handoff.source.is_contextual_only());
    // A contextual reopen is never published as an exact (full) restore.
    assert_ne!(
        handoff.published_fidelity,
        RestoreFidelityClass::ExactRestore
    );
    assert!(handoff.published_fidelity.rank() <= handoff.source.fidelity_ceiling().rank());
    assert_eq!(handoff.recovery_path, RecoveryPath::ReopenAsContext);
}

#[test]
fn the_exact_card_is_genuinely_clean() {
    let packet = packet();
    let exact = packet
        .card(ReentrySurface::DesktopRestore)
        .expect("desktop card");
    assert!(exact.is_exact());
    assert!(!exact.is_narrowed());
    assert!(!exact.source.is_contextual_only());
    assert_eq!(exact.schema_condition, SchemaCondition::SchemaMatch);
    assert_eq!(
        exact.dependency_condition,
        DependencyCondition::DependenciesPresent
    );
    assert_eq!(
        exact.topology_condition,
        TopologyCondition::TopologyIdentical
    );
    assert_eq!(exact.evidence_freshness, EvidenceFreshness::Current);
    assert!(exact.downgrade_reasons.is_empty());
    assert!(exact.caveats.is_empty());
    assert_eq!(exact.recovery_path, RecoveryPath::NoneNeeded);
}

#[test]
fn missing_dependency_behavior_never_silently_deletes_layout() {
    let packet = packet();
    for card in &packet.cards {
        assert!(
            card.missing_dependency_behavior.preserves_slot(),
            "{}",
            card.card_id
        );
    }
}

#[test]
fn parity_surfaces_carry_the_same_record() {
    let packet = packet();
    for surface in ProvenanceConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding {surface:?}"
        );
    }
}

#[test]
fn card_view_is_plain_language_and_distinguishes_fidelity() {
    let packet = packet();
    let view = packet.card_view();
    assert_eq!(view.rows.len(), 5);
    assert_eq!(view.exact_count, 1);
    assert_eq!(view.narrowed_count, 4);
    assert_eq!(view.manual_review_count, 1);
    for row in &view.rows {
        assert!(!row.summary.trim().is_empty());
        assert!(!row.actions.is_empty());
        assert!(!row.redaction_class.is_empty());
    }
    let handoff = view
        .rows
        .iter()
        .find(|r| r.reentry_surface == "companion_browser_reentry")
        .expect("handoff row");
    assert_eq!(handoff.published_fidelity, "layout_only");
    assert!(handoff.narrowed);
    let desktop = view
        .rows
        .iter()
        .find(|r| r.reentry_surface == "desktop_restore")
        .expect("desktop row");
    assert_eq!(desktop.published_fidelity, "exact_restore");
    assert!(!desktop.narrowed);
}

#[test]
fn support_export_round_trips_and_is_safe() {
    let packet = packet();
    let export = packet.support_export("export:restore-provenance", "2026-06-16");
    assert!(export.is_export_safe());
    assert_eq!(export.record_packet_id_ref, packet.packet_id);
    let encoded = serde_json::to_string(&export).expect("serializes");
    let decoded: M5RestoreProvenanceSupportExport =
        serde_json::from_str(&encoded).expect("round-trips");
    assert_eq!(decoded.record, packet);
}

// --- Fixtures ------------------------------------------------------------------------------------

#[test]
fn exact_desktop_fixture_is_clean_exact() {
    let card = card(EXACT_DESKTOP_FIXTURE);
    assert_eq!(card.source, RestoreSource::AutoCheckpoint);
    assert!(card.is_exact());
    assert!(card.gate_consistent());
    assert_eq!(card.published_fidelity, RestoreFidelityClass::ExactRestore);
}

#[test]
fn compatible_import_fixture_is_narrowed_to_compatible() {
    let card = card(COMPATIBLE_IMPORT_FIXTURE);
    assert_eq!(card.source, RestoreSource::Import);
    assert_eq!(
        card.published_fidelity,
        RestoreFidelityClass::CompatibleRestore
    );
    assert!(card.is_downgraded());
    assert!(card.gate_consistent());
    assert_eq!(card.recovery_path, RecoveryPath::RestoreCompatibly);
}

#[test]
fn manual_review_crash_fixture_is_held_for_review() {
    let card = card(MANUAL_REVIEW_CRASH_FIXTURE);
    assert_eq!(card.published_fidelity, RestoreFidelityClass::ManualReview);
    assert!(card.gate_consistent());
    assert_eq!(card.recovery_path, RecoveryPath::ManualReview);
    assert!(card.missing_dependency_behavior.preserves_slot());
}

#[test]
fn layout_only_handoff_fixture_is_contextual_reopen() {
    let card = card(LAYOUT_ONLY_HANDOFF_FIXTURE);
    assert_eq!(card.source, RestoreSource::BrowserCompanionHandoff);
    assert_eq!(card.published_fidelity, RestoreFidelityClass::LayoutOnly);
    assert!(card.gate_consistent());
    assert_ne!(card.published_fidelity, RestoreFidelityClass::ExactRestore);
}

// --- Fail-closed gate drills ---------------------------------------------------------------------

fn card_index(packet: &M5RestoreProvenance, surface: ReentrySurface) -> usize {
    packet
        .cards
        .iter()
        .position(|c| c.reentry_surface == surface)
        .expect("card present")
}

#[test]
fn overstated_fidelity_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::PortableStateImport);
    broken.cards[idx].published_fidelity = RestoreFidelityClass::ExactRestore;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::OverstatedFidelity { .. })));
}

#[test]
fn handoff_implying_full_restore_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::CompanionBrowserReentry);
    // Force the handoff card to pristine conditions and an exact claim; the source ceiling must
    // still bar it from publishing a full restore.
    let card = &mut broken.cards[idx];
    card.declared_resulting_fidelity = RestoreFidelityClass::ExactRestore;
    card.schema_condition = SchemaCondition::SchemaMatch;
    card.dependency_condition = DependencyCondition::DependenciesPresent;
    card.topology_condition = TopologyCondition::TopologyIdentical;
    card.evidence_freshness = EvidenceFreshness::Current;
    card.published_fidelity = RestoreFidelityClass::ExactRestore;
    let violations = broken.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::SourceFidelityCeilingExceeded { .. }
    )));
}

#[test]
fn silent_layout_delete_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::CrashRecovery);
    broken.cards[idx].missing_dependency_behavior = MissingDependencyBehavior::SilentDelete;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::SilentLayoutDelete { .. })));
}

#[test]
fn missing_redaction_exclusion_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx]
        .redaction_class
        .retain(|e| *e != RedactionExclusion::ExcludesMachineLocalAnchors);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::MissingRedactionExclusion { .. }
    )));
}

#[test]
fn missing_open_details_action_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx]
        .available_actions
        .retain(|a| a.action != ProvenanceActionKind::OpenDetails);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::MissingOpenDetailsAction { .. }
    )));
}

#[test]
fn narrowed_card_dropping_recovery_actions_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::PortableStateImport);
    broken.cards[idx]
        .available_actions
        .retain(|a| a.action != ProvenanceActionKind::RecoveryNextStep);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::MissingRecoveryActions { .. }
    )));
}

#[test]
fn downgrade_reasons_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::PortableStateImport);
    broken.cards[idx].downgrade_reasons = vec![DowngradeReason::SchemaDrift];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::DowngradeReasonsMismatch { .. }
    )));
}

#[test]
fn recovery_path_mismatch_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::PortableStateImport);
    broken.cards[idx].recovery_path = RecoveryPath::NoneNeeded;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::RecoveryPathMismatch { .. })));
}

#[test]
fn exact_card_with_a_caveat_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx]
        .caveats
        .push("this should not be here".to_owned());
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::ExactCardNotClean { .. })));
}

#[test]
fn inaccessible_action_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx].available_actions[0].keyboard_shortcut = "  ".to_owned();
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::InaccessibleAction { .. })));
}

#[test]
fn unscoped_action_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx].available_actions[0].scoped_to_event = false;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::UnscopedAction { .. })));
}

#[test]
fn duplicate_focus_order_is_rejected() {
    let mut broken = packet();
    let idx = card_index(&broken, ReentrySurface::DesktopRestore);
    broken.cards[idx].available_actions[1].focus_order =
        broken.cards[idx].available_actions[0].focus_order;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::DuplicateFocusOrder { .. })));
}

#[test]
fn missing_consumer_binding_is_rejected() {
    let mut broken = packet();
    broken
        .consumer_bindings
        .retain(|b| b.consumer_surface != ProvenanceConsumerSurface::SupportPacket);
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn consumer_binding_drift_is_rejected() {
    let mut broken = packet();
    broken.consumer_bindings[0].preserves_fidelity_labels = false;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::ConsumerBindingDrift { .. })));
}

#[test]
fn closed_vocabulary_drift_is_rejected() {
    let mut broken = packet();
    broken.restore_sources = vec![RestoreSource::AutoCheckpoint];
    assert!(broken.validate().iter().any(|v| matches!(
        v,
        M5RestoreProvenanceViolation::ClosedVocabularyDrift { .. }
    )));
}

#[test]
fn missing_surface_card_is_rejected() {
    let mut broken = packet();
    broken
        .cards
        .retain(|c| c.reentry_surface != ReentrySurface::CrashRecovery);
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::MissingSurfaceCard { .. })));
}

#[test]
fn summary_mismatch_is_rejected() {
    let mut broken = packet();
    broken.summary.exact_restore_cards = 99;
    assert!(broken
        .validate()
        .iter()
        .any(|v| matches!(v, M5RestoreProvenanceViolation::SummaryMismatch)));
}
