use super::*;

const PACKET_ID: &str = WORK_ITEM_ROW_PROVIDER_CHIP_PACKET_ID;

fn packet() -> WorkItemRowProviderChipControlsPacket {
    seeded_work_item_row_provider_chip_controls()
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
    assert_eq!(packet.record_kind, WORK_ITEM_ROW_PROVIDER_CHIP_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        WORK_ITEM_ROW_PROVIDER_CHIP_SCHEMA_VERSION
    );
}

#[test]
fn state_authority_is_derived_not_asserted() {
    use M5WorkItemLocalState as Local;
    use M5WorkItemProviderAuthority as Authority;
    use WorkItemStateAuthorityClass as Class;

    // Provider-owned + synced → provider-authoritative.
    let d = resolve_work_item_state_authority(Authority::ProviderOwned, Local::SyncedWithProvider);
    assert_eq!(d.authority_class, Class::ProviderAuthoritative);
    assert!(d.is_provider_authoritative);
    assert!(!d.needs_local_state_note);

    // A local draft never reads as provider-authoritative.
    let d = resolve_work_item_state_authority(Authority::LocalDraft, Local::LocalOnlyDraft);
    assert_eq!(d.authority_class, Class::LocalOnlyDraft);
    assert!(!d.is_provider_authoritative);
    assert!(d.is_local_only);
    assert!(d.needs_local_state_note);

    // Queued / conflict-held → publish-pending, not reconciled.
    for state in [
        Local::QueuedForPublish,
        Local::ConflictHeld,
        Local::PublishFailed,
    ] {
        let d = resolve_work_item_state_authority(Authority::ProviderOwned, state);
        assert_eq!(d.authority_class, Class::PublishPending);
        assert!(d.needs_publish_pending_note);
        assert!(!d.is_provider_authoritative);
    }

    // Policy-pinned → blocked capability regardless of local state.
    let d = resolve_work_item_state_authority(Authority::PolicyPinned, Local::SyncedWithProvider);
    assert_eq!(d.authority_class, Class::BlockedCapability);
    assert!(d.is_blocked);
    assert!(d.needs_blocked_note);

    // Mirrored → snapshot only.
    let d =
        resolve_work_item_state_authority(Authority::MirroredReadOnly, Local::SyncedWithProvider);
    assert_eq!(d.authority_class, Class::SnapshotOnly);
    assert!(!d.is_provider_authoritative);
}

#[test]
fn chip_posture_consistency_is_derived() {
    use M5WorkItemProviderAuthority as Authority;
    use ProviderChipWritePosture as Posture;

    // Full-edit requires provider-owned authority.
    assert!(Posture::FullEdit.is_consistent_with_authority(Authority::ProviderOwned));
    assert!(!Posture::FullEdit.is_consistent_with_authority(Authority::LocalDraft));
    // Offline-capture requires a local authority.
    assert!(Posture::OfflineCapture.is_consistent_with_authority(Authority::LocalDraft));
    assert!(!Posture::OfflineCapture.is_consistent_with_authority(Authority::ProviderOwned));
    // Policy-blocked requires a policy-pinned authority.
    assert!(Posture::PolicyBlocked.is_consistent_with_authority(Authority::PolicyPinned));
    assert!(!Posture::PolicyBlocked.is_consistent_with_authority(Authority::ProviderOwned));

    assert!(Posture::FullEdit.is_writable());
    assert!(Posture::CommentLink.is_writable());
    assert!(!Posture::ReadOnly.is_writable());
    assert!(!Posture::OfflineCapture.is_writable());
}

#[test]
fn state_authority_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .work_item_rows
        .iter()
        .map(|row| row.state_authority_disclosure().authority_class)
        .collect();
    for class in WorkItemStateAuthorityClass::ALL {
        assert!(covered.contains(&class), "missing class {class:?}");
    }
}

#[test]
fn write_posture_coverage_is_complete() {
    let packet = packet();
    let covered: std::collections::BTreeSet<_> = packet
        .provider_chip_groups
        .iter()
        .map(|group| group.write_posture)
        .collect();
    for posture in ProviderChipWritePosture::ALL {
        assert!(covered.contains(&posture), "missing posture {posture:?}");
    }
}

#[test]
fn wrong_record_kind_fails() {
    let mut packet = packet();
    packet.record_kind = "bogus".to_owned();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::WrongRecordKind));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::MissingSourceContracts));
}

#[test]
fn empty_work_item_rows_fails() {
    let mut packet = packet();
    packet.work_item_rows.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::WorkItemRowsMissing));
}

#[test]
fn empty_provider_chip_groups_fails() {
    let mut packet = packet();
    packet.provider_chip_groups.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ProviderChipGroupsMissing));
}

#[test]
fn row_wrong_component_class_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].component = M5WorkItemComponentFamily::ProviderChipGroup;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::WorkItemRowWrongComponentClass));
}

#[test]
fn non_copyable_canonical_id_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].canonical_id_copyable = false;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::CanonicalIdNotCopyable));
}

#[test]
fn missing_copy_action_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].default_actions = vec![WorkItemRowAction::ExportRow];
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::DefaultActionsIncomplete));
}

#[test]
fn local_row_claiming_provider_authoritative_fails() {
    let mut packet = packet();
    // Find the local-only draft row and claim it is provider-authoritative.
    let row = packet
        .work_item_rows
        .iter_mut()
        .find(|row| row.state_authority_class == WorkItemStateAuthorityClass::LocalOnlyDraft)
        .expect("local-only row present");
    row.claims_provider_authoritative = true;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::StateAuthorityMisrepresented));
}

#[test]
fn misdeclared_state_authority_class_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].state_authority_class = WorkItemStateAuthorityClass::BlockedCapability;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::StateAuthorityMisrepresented));
}

#[test]
fn missing_publish_pending_note_fails() {
    let mut packet = packet();
    let row = packet
        .work_item_rows
        .iter_mut()
        .find(|row| row.state_authority_class == WorkItemStateAuthorityClass::PublishPending)
        .expect("publish-pending row present");
    row.publish_pending_note.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::PublishPendingNoteMissing));
}

#[test]
fn missing_blocked_note_fails() {
    let mut packet = packet();
    let row = packet
        .work_item_rows
        .iter_mut()
        .find(|row| row.state_authority_class == WorkItemStateAuthorityClass::BlockedCapability)
        .expect("blocked row present");
    row.blocked_capability_note.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::BlockedCapabilityNoteMissing));
}

#[test]
fn missing_linked_change_label_fails() {
    let mut packet = packet();
    let row = packet
        .work_item_rows
        .iter_mut()
        .find(|row| row.linked_change_count > 0)
        .expect("linked-change row present");
    row.linked_change_label.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::LinkedChangeLabelMissing));
}

#[test]
fn generic_ticket_wording_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].uses_generic_ticket_wording = true;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::GenericTicketWordingUsed));
}

#[test]
fn chip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.provider_chip_groups[0].component = M5WorkItemComponentFamily::WorkItemRow;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ProviderChipGroupWrongComponentClass));
}

#[test]
fn chip_posture_inconsistent_with_authority_fails() {
    let mut packet = packet();
    // Claim full-edit on a policy-pinned authority.
    let group = &mut packet.provider_chip_groups[4];
    group.write_posture = ProviderChipWritePosture::FullEdit;
    group.is_writable = true;
    group.policy_block_note.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ChipPostureMisrepresented));
}

#[test]
fn chip_writability_misrepresented_fails() {
    let mut packet = packet();
    // A read-only chip that claims to be writable.
    let group = packet
        .provider_chip_groups
        .iter_mut()
        .find(|group| group.write_posture == ProviderChipWritePosture::ReadOnly)
        .expect("read-only chip present");
    group.is_writable = true;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ChipWritabilityMisrepresented));
}

#[test]
fn missing_offline_capture_note_fails() {
    let mut packet = packet();
    let group = packet
        .provider_chip_groups
        .iter_mut()
        .find(|group| group.write_posture == ProviderChipWritePosture::OfflineCapture)
        .expect("offline-capture chip present");
    group.offline_capture_note.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::OfflineCaptureNoteMissing));
}

#[test]
fn missing_project_or_space_scope_fails() {
    let mut packet = packet();
    packet.provider_chip_groups[0]
        .project_or_space_label
        .clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ProjectOrSpaceScopeMissing));
}

#[test]
fn missing_tenant_scope_note_fails() {
    let mut packet = packet();
    let group = packet
        .provider_chip_groups
        .iter_mut()
        .find(|group| group.has_tenant_scope)
        .expect("tenant-scoped chip present");
    group.tenant_scope_note.clear();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::TenantScopeNoteMissing));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet
        .trust_review
        .local_or_blocked_never_reads_as_provider_authoritative = false;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .list_rows_distinguish_authority_without_inspector = false;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::ProofFreshnessIncomplete));
}

#[test]
fn forbidden_material_in_export_fails() {
    let mut packet = packet();
    packet.work_item_rows[0].title = "see https://internal.example/board".to_owned();
    assert!(packet
        .validate()
        .contains(&WorkItemRowProviderChipViolation::RawBoundaryMaterialInExport));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Work-item rows"));
    assert!(summary.contains("## Provider chip groups"));
    assert!(summary.contains("provider_authoritative"));
    assert!(summary.contains("offline_capture"));
}

#[test]
fn matrix_csv_has_a_line_per_control() {
    let csv = packet().render_matrix_csv();
    let lines = csv.lines().count();
    // header + 6 rows + 5 chip groups
    assert_eq!(lines, 1 + 6 + 5);
    assert!(csv.contains("work_item_row"));
    assert!(csv.contains("provider_chip_group"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_work_item_row_provider_chip_export()
        .expect("checked work item row provider chip export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_scenario_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-work-item-row-provider-chip-controls/work_item_row_local_only.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-work-item-row-provider-chip-controls/provider_chip_offline_capture.json"
        )),
    ] {
        let packet: WorkItemRowProviderChipControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as work item row provider chip packet");
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
        seeded_work_item_row_provider_chip_controls_work_item_row_local_only(),
        seeded_work_item_row_provider_chip_controls_provider_chip_offline_capture(),
    ] {
        assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    }
}
