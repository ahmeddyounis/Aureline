use super::*;

#[test]
fn seeded_packet_validates() {
    let packet = seeded_work_item_object_rows_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(!packet.rows.is_empty());
}

#[test]
fn seeded_packet_covers_issue_task_and_incident_rows() {
    let packet = seeded_work_item_object_rows_packet();
    let object_classes = packet
        .rows
        .iter()
        .map(|row| row.object_class)
        .collect::<std::collections::BTreeSet<_>>();
    assert!(object_classes.contains(&WorkItemObjectClass::IssueOrWorkItem));
    assert!(object_classes.contains(&WorkItemObjectClass::TaskOrSubtask));
    assert!(object_classes.contains(&WorkItemObjectClass::IncidentReport));
}

#[test]
fn seeded_rows_keep_relation_strip_axes_visible() {
    let packet = seeded_work_item_object_rows_packet();
    let row = packet
        .rows
        .iter()
        .find(|row| row.canonical_id == "AUR-241")
        .expect("provider row");
    let kinds = row
        .relation_strip
        .items
        .iter()
        .map(|item| item.relation_kind)
        .collect::<std::collections::BTreeSet<_>>();
    assert!(kinds.contains(&WorkItemRelationKindClass::BranchOrWorktree));
    assert!(kinds.contains(&WorkItemRelationKindClass::Review));
    assert!(kinds.contains(&WorkItemRelationKindClass::Run));
    assert!(kinds.contains(&WorkItemRelationKindClass::Incident));
    assert!(kinds.contains(&WorkItemRelationKindClass::ValidationEvidence));
}

#[test]
fn queued_row_requires_local_draft_marker() {
    let mut packet = seeded_work_item_object_rows_packet();
    let queued_row = packet
        .rows
        .iter_mut()
        .find(|row| row.sync_scope_class == WorkItemSyncScopeClass::QueuedPublish)
        .expect("queued row");
    queued_row.local_draft_marker_visible = false;
    assert!(packet
        .validate()
        .contains(&WorkItemObjectRowsViolation::MissingLocalDraftMarker));
}
