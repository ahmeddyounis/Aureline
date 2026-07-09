use super::*;

const PACKET_ID: &str = RELATION_STRIP_SYNC_PENDING_PACKET_ID;

fn packet() -> RelationStripSyncPendingControlsPacket {
    seeded_relation_strip_sync_pending_controls()
}

#[test]
fn seed_packet_validates() {
    let packet = packet();
    assert!(
        packet.validate().is_empty(),
        "seed packet failed validation: {:?}",
        packet.validate()
    );
    assert_eq!(packet.packet_id, PACKET_ID);
    assert_eq!(packet.record_kind, RELATION_STRIP_SYNC_PENDING_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        RELATION_STRIP_SYNC_PENDING_SCHEMA_VERSION
    );
}

#[test]
fn relation_health_is_derived_not_asserted() {
    use M5WorkItemRelationKind as Kind;
    use RelationHealthClass as Health;

    // Reachable and current → current.
    let d = resolve_relation_health(Kind::LinkedBranch, true, true);
    assert_eq!(d.health_class, Health::Current);
    assert!(d.is_current);
    assert!(!d.needs_relation_note);

    // Reachable but out of date → stale.
    let d = resolve_relation_health(Kind::LinkedReview, true, false);
    assert_eq!(d.health_class, Health::Stale);
    assert!(!d.is_current);
    assert!(d.needs_relation_note);

    // Unreachable → broken.
    let d = resolve_relation_health(Kind::LinkedTestRun, false, true);
    assert_eq!(d.health_class, Health::Broken);
    assert!(d.needs_relation_note);

    // Unmapped kind → unmapped regardless of reachability.
    let d = resolve_relation_health(Kind::UnmappedRelation, true, true);
    assert_eq!(d.health_class, Health::Unmapped);
    assert!(d.needs_relation_note);
}

#[test]
fn sync_recovery_is_derived_not_asserted() {
    use M5WorkItemLocalState as Local;
    use SyncRecoveryClass as Class;

    // Synced, not blocked, online → provider-confirmed.
    let d = resolve_sync_recovery(Local::SyncedWithProvider, false, false);
    assert_eq!(d.recovery_class, Class::ProviderConfirmed);
    assert!(d.is_provider_confirmed);
    assert!(!d.needs_distinct_style);
    assert!(!d.needs_recovery_action);

    // Queued → pending publish, recoverable, visibly distinct.
    let d = resolve_sync_recovery(Local::QueuedForPublish, false, false);
    assert_eq!(d.recovery_class, Class::PendingPublish);
    assert!(!d.is_provider_confirmed);
    assert!(d.needs_distinct_style);
    assert!(d.needs_recovery_action);
    assert!(d.needs_last_sync_attempt);

    // Publish failed → recoverable failure, stays recoverable.
    let d = resolve_sync_recovery(Local::PublishFailed, false, false);
    assert_eq!(d.recovery_class, Class::RecoverableFailure);
    assert!(d.needs_recovery_action);

    // Offline while unsynced → offline-held, stays recoverable.
    let d = resolve_sync_recovery(Local::LocalOnlyDraft, false, true);
    assert_eq!(d.recovery_class, Class::OfflineHeld);
    assert!(d.needs_recovery_action);

    // Policy-blocked overrides everything.
    let d = resolve_sync_recovery(Local::QueuedForPublish, true, false);
    assert_eq!(d.recovery_class, Class::PolicyBlocked);
    assert!(!d.is_provider_confirmed);
    assert!(d.needs_policy_block_note);
    assert!(!d.needs_recovery_action);
}

#[test]
fn relation_health_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .relation_strips
        .iter()
        .flat_map(|strip| strip.relations.iter())
        .map(|relation| relation.health_disclosure().health_class)
        .collect();
    for class in RelationHealthClass::ALL {
        assert!(covered.contains(&class), "missing health class {class:?}");
    }
}

#[test]
fn sync_recovery_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .sync_pending_pills
        .iter()
        .map(|pill| pill.recovery_disclosure().recovery_class)
        .collect();
    for class in SyncRecoveryClass::ALL {
        assert!(covered.contains(&class), "missing recovery class {class:?}");
    }
}

#[test]
fn pending_change_type_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .sync_pending_pills
        .iter()
        .map(|pill| pill.pending_change_type)
        .collect();
    for change_type in PendingChangeType::ALL {
        assert!(covered.contains(&change_type), "missing type {change_type:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::MissingSourceContracts));
}

#[test]
fn empty_relation_strips_fails() {
    let mut packet = packet();
    packet.relation_strips.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationStripsMissing));
}

#[test]
fn empty_sync_pending_pills_fails() {
    let mut packet = packet();
    packet.sync_pending_pills.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::SyncPendingPillsMissing));
}

#[test]
fn strip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.relation_strips[0].component = M5WorkItemComponentFamily::SyncPendingPill;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationStripWrongComponentClass));
}

#[test]
fn collapsed_linked_label_fails() {
    let mut packet = packet();
    packet.relation_strips[0].collapses_into_generic_linked_label = true;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationsCollapsedIntoVagueLabel));
}

#[test]
fn duplicate_relation_labels_collapse_fails() {
    let mut packet = packet();
    // Give two relations the same label — that is a vague, collapsed context.
    let label = packet.relation_strips[0].relations[0].reference_label.clone();
    packet.relation_strips[0].relations[1].reference_label = label;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationsCollapsedIntoVagueLabel));
}

#[test]
fn relation_health_misrepresented_fails() {
    let mut packet = packet();
    // Claim a broken relation is current.
    let relation = packet
        .relation_strips
        .iter_mut()
        .flat_map(|strip| strip.relations.iter_mut())
        .find(|relation| relation.health_disclosure().health_class == RelationHealthClass::Broken)
        .expect("broken relation present");
    relation.health_class = RelationHealthClass::Current;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationHealthMisrepresented));
}

#[test]
fn missing_stale_relation_note_fails() {
    let mut packet = packet();
    let relation = packet
        .relation_strips
        .iter_mut()
        .flat_map(|strip| strip.relations.iter_mut())
        .find(|relation| !relation.health_disclosure().is_current)
        .expect("non-current relation present");
    relation.relation_note.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::StaleOrBrokenRelationNoteMissing));
}

#[test]
fn missing_relation_copy_open_action_fails() {
    let mut packet = packet();
    packet.relation_strips[0].relations[0].actions = vec![RelationStripAction::ExportRelation];
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RelationCopyOpenActionsIncomplete));
}

#[test]
fn pending_pill_claiming_provider_confirmed_fails() {
    let mut packet = packet();
    let pill = packet
        .sync_pending_pills
        .iter_mut()
        .find(|pill| !pill.recovery_disclosure().is_provider_confirmed)
        .expect("pending pill present");
    pill.claims_provider_confirmed = true;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::SyncStateMisrepresented));
}

#[test]
fn pending_pill_not_distinct_fails() {
    let mut packet = packet();
    let pill = packet
        .sync_pending_pills
        .iter_mut()
        .find(|pill| pill.recovery_disclosure().needs_distinct_style)
        .expect("pending pill present");
    pill.distinct_from_confirmed_style = false;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::NotVisiblyDistinctFromConfirmed));
}

#[test]
fn misdeclared_recovery_class_fails() {
    let mut packet = packet();
    packet.sync_pending_pills[0].recovery_class = SyncRecoveryClass::RecoverableFailure;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::SyncRecoveryMisrepresented));
}

#[test]
fn missing_recovery_action_fails() {
    let mut packet = packet();
    let pill = packet
        .sync_pending_pills
        .iter_mut()
        .find(|pill| pill.recovery_disclosure().needs_recovery_action)
        .expect("recoverable pill present");
    pill.recovery_actions = vec![SyncPillAction::OpenInProvider];
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RecoveryActionMissing));
}

#[test]
fn missing_last_sync_attempt_fails() {
    let mut packet = packet();
    let pill = packet
        .sync_pending_pills
        .iter_mut()
        .find(|pill| pill.recovery_disclosure().needs_last_sync_attempt)
        .expect("pending pill present");
    pill.last_sync_attempt_label.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::LastSyncAttemptMissing));
}

#[test]
fn missing_policy_block_note_fails() {
    let mut packet = packet();
    let pill = packet
        .sync_pending_pills
        .iter_mut()
        .find(|pill| pill.recovery_disclosure().needs_policy_block_note)
        .expect("policy-blocked pill present");
    pill.policy_block_note.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::PolicyBlockNoteMissing));
}

#[test]
fn missing_pending_change_label_fails() {
    let mut packet = packet();
    packet.sync_pending_pills[0].pending_change_label.clear();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::PendingChangeTypeLabelMissing));
}

#[test]
fn generic_ticket_wording_fails() {
    let mut packet = packet();
    packet.relation_strips[0].uses_generic_ticket_wording = true;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::GenericTicketWordingUsed));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .sync_pending_visibly_distinct_from_confirmed = false;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .list_and_rail_distinguish_pending_from_confirmed = false;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.relation_strips[0].relations[0].reference_label =
        "see https://internal.example/branch".to_owned();
    assert!(packet
        .validate()
        .contains(&RelationStripSyncPendingViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Relation strips"));
    assert!(summary.contains("## Sync-pending pills"));
    assert!(summary.contains("provider_confirmed"));
    assert!(summary.contains("recoverable_failure"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 2 strips + 5 pills
    assert_eq!(lines, 1 + 2 + 5);
    assert!(csv.contains("relation_strip"));
    assert!(csv.contains("sync_pending_pill"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_relation_strip_sync_pending_export()
        .expect("checked relation strip sync pending export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-relation-strip-sync-pending-controls/relation_strip_stale_relation.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-relation-strip-sync-pending-controls/sync_pending_recoverable_failure.json"
        )),
    ] {
        let packet: RelationStripSyncPendingControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as relation strip sync pending packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn scenario_fixtures_stay_valid_and_covered() {
    for packet in [
        seeded_relation_strip_sync_pending_controls_relation_strip_stale_relation(),
        seeded_relation_strip_sync_pending_controls_sync_pending_recoverable_failure(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
