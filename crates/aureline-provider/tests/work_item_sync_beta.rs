//! Integration coverage for provider-backed work-item link, comment-sync, and
//! publish-review beta records.

use std::collections::BTreeSet;

use aureline_provider::{
    seeded_work_item_sync_beta_page, validate_work_item_sync_beta_page, CommentConflictClass,
    CommentLifecycleClass, CommentOriginClass, CommentPublishPostureClass, CommentSyncStateClass,
    LinkConflictResolutionPostureClass, LinkLocalDraftStateClass, LinkRelationStateClass,
    LinkSourceClass, PublishReviewActionClass, PublishReviewDispositionClass,
    WorkItemLinkKindClass, WorkItemSyncBetaDefectKind, WorkItemSyncBetaPage,
    WorkItemSyncBetaSupportExport, WORK_ITEM_SYNC_BETA_PAGE_RECORD_KIND,
    WORK_ITEM_SYNC_BETA_SCHEMA_VERSION,
};

#[test]
fn seeded_page_round_trips_through_serde() {
    let page = seeded_work_item_sync_beta_page();
    let json = serde_json::to_string(&page).expect("serialize");
    let parsed: WorkItemSyncBetaPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(parsed.record_kind, WORK_ITEM_SYNC_BETA_PAGE_RECORD_KIND);
    assert_eq!(parsed.schema_version, WORK_ITEM_SYNC_BETA_SCHEMA_VERSION);
    assert_eq!(
        parsed.work_item_link_records.len(),
        page.work_item_link_records.len()
    );
    assert_eq!(
        parsed.comment_sync_records.len(),
        page.comment_sync_records.len()
    );
    assert_eq!(parsed.publish_reviews.len(), page.publish_reviews.len());
}

#[test]
fn seeded_page_validates_and_covers_required_truth_axes() {
    let page = seeded_work_item_sync_beta_page();
    validate_work_item_sync_beta_page(&page).expect("seeded page validates");
    let report = page.validate();
    assert!(report.passed, "seeded page defects: {:#?}", report.defects);

    for kind in [
        WorkItemLinkKindClass::BranchOrWorktreeLink,
        WorkItemLinkKindClass::ReviewWorkspaceLink,
    ] {
        assert!(
            report.coverage.link_kinds.contains(&kind),
            "missing link kind: {kind:?}"
        );
    }

    for relation in [
        LinkRelationStateClass::LinkedActiveProviderConfirmed,
        LinkRelationStateClass::LinkedLocalDraftPendingPublish,
        LinkRelationStateClass::LinkedQueuedForPublishLater,
        LinkRelationStateClass::UnlinkRequestedLocalDraft,
        LinkRelationStateClass::ConflictRequiresReview,
        LinkRelationStateClass::LinkedImportedSnapshotNoProviderPath,
    ] {
        assert!(
            report.coverage.link_relation_states.contains(&relation),
            "missing link relation: {relation:?}"
        );
    }

    for origin in [
        CommentOriginClass::ProviderAuthoritativeComment,
        CommentOriginClass::LocalDraftComment,
        CommentOriginClass::OfflineCapturePacketComment,
    ] {
        assert!(
            report.coverage.comment_origin_classes.contains(&origin),
            "missing comment origin: {origin:?}"
        );
    }

    for state in [
        CommentSyncStateClass::InSyncProviderObserved,
        CommentSyncStateClass::PendingPublishLocalDraft,
        CommentSyncStateClass::QueuedPublishDeferred,
        CommentSyncStateClass::PendingDrainOfflineCaptured,
        CommentSyncStateClass::PublishFailedTypedRetry,
        CommentSyncStateClass::ConflictDetectedAwaitingResolution,
    ] {
        assert!(
            report.coverage.comment_sync_states.contains(&state),
            "missing comment sync state: {state:?}"
        );
    }

    for action in [
        PublishReviewActionClass::CreateProviderComment,
        PublishReviewActionClass::EditProviderComment,
        PublishReviewActionClass::DeleteProviderComment,
        PublishReviewActionClass::LinkBranchOrReview,
        PublishReviewActionClass::UnlinkBranchOrReview,
        PublishReviewActionClass::StatusTransitionPlusComment,
        PublishReviewActionClass::RetryAfterConflict,
    ] {
        assert!(
            report
                .coverage
                .publish_review_action_classes
                .contains(&action),
            "missing publish-review action: {action:?}"
        );
    }
}

#[test]
fn support_export_excludes_raw_provider_material() {
    let page = seeded_work_item_sync_beta_page();
    let export = WorkItemSyncBetaSupportExport::from_page(
        "work-item-sync-beta:support-export:test",
        "2026-05-18T09:30:00Z",
        &page,
    );
    assert!(export.raw_provider_material_excluded);

    let actions: BTreeSet<_> = export
        .publish_review_summaries
        .iter()
        .map(|summary| summary.publish_review_action_class)
        .collect();
    assert!(actions.contains(&PublishReviewActionClass::CreateProviderComment));
    assert!(actions.contains(&PublishReviewActionClass::RetryAfterConflict));

    let json = serde_json::to_string(&export).expect("serialize export");
    assert!(!json.contains("https://"));
    assert!(!json.contains("Bearer "));
    assert!(!json.contains("raw_provider_payload"));
    assert!(!json.contains("raw_comment_text"));
}

#[test]
fn local_draft_comment_cannot_claim_provider_publish_posture() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .comment_sync_records
        .iter_mut()
        .find(|record| record.comment_sync_id == "work_item_sync:comment:local-draft-pending")
        .expect("local-draft comment record");
    record.comment_publish_posture_class = CommentPublishPostureClass::ProviderPublishedObserved;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == WorkItemSyncBetaDefectKind::CommentSyncTruthIncoherent
            && defect.check_id == "work_item_sync_beta.comment_local_draft_truth"
    }));
}

#[test]
fn provider_confirmed_link_cannot_claim_local_draft_state() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .work_item_link_records
        .iter_mut()
        .find(|record| record.link_id == "work_item_sync:link:branch-provider-confirmed")
        .expect("provider-confirmed link");
    record.link_local_draft_state_class =
        LinkLocalDraftStateClass::LocalDraftCreateLinkPendingPublish;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == WorkItemSyncBetaDefectKind::LinkTruthIncoherent
            && defect.check_id == "work_item_sync_beta.link_active_truth_incoherent"
    }));
}

#[test]
fn conflict_link_must_declare_a_conflict_posture() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .work_item_link_records
        .iter_mut()
        .find(|record| record.link_id == "work_item_sync:link:branch-conflict")
        .expect("conflict link");
    record.link_conflict_resolution_posture_class =
        LinkConflictResolutionPostureClass::NoConflictDetected;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| { defect.check_id == "work_item_sync_beta.link_conflict_truth" }));
}

#[test]
fn publish_review_requires_side_effects_and_escape_actions() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| review.publish_review_id == "work_item_sync:publish_review:create-comment")
        .expect("create-comment publish-review");
    review.side_effect_rows.clear();
    review.action_affordances.export_action_available = false;

    let report = page.validate();
    assert!(!report.passed);
    assert!(
        report
            .defects
            .iter()
            .any(|defect| defect.check_id
                == "work_item_sync_beta.publish_review_side_effects_missing")
    );
    assert!(report.defects.iter().any(|defect| {
        defect.check_id == "work_item_sync_beta.publish_review_export_cancel_missing"
    }));
}

#[test]
fn link_action_publish_review_must_cite_link_record() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| review.publish_review_id == "work_item_sync:publish_review:link-review")
        .expect("link-review publish-review");
    review.work_item_link_state_record_id_ref = None;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.check_id == "work_item_sync_beta.publish_review_link_ref_missing"
    }));
}

#[test]
fn comment_action_publish_review_must_bind_existing_comment() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| review.publish_review_id == "work_item_sync:publish_review:edit-comment")
        .expect("edit-comment publish-review");
    review.comment_sync_state_record_id_ref =
        Some("work_item_sync:comment:does-not-exist".to_string());

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.defect_kind == WorkItemSyncBetaDefectKind::UnknownRecordReference
            && defect.check_id == "work_item_sync_beta.publish_review_comment_unknown"
    }));
}

#[test]
fn retry_disposition_requires_a_retry_action() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| {
            review.publish_review_id == "work_item_sync:publish_review:retry-after-conflict"
        })
        .expect("retry publish-review");
    review.publish_review_action_class = PublishReviewActionClass::CreateProviderComment;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| { defect.check_id == "work_item_sync_beta.publish_review_retry_truth" }));
}

#[test]
fn queued_disposition_requires_queue_item_reference() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| review.publish_review_id == "work_item_sync:publish_review:unlink-branch")
        .expect("unlink publish-review");
    review.linked_publish_later_queue_item_record_id_ref = None;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "work_item_sync_beta.publish_review_queue_truth"));
}

#[test]
fn blocked_disposition_requires_block_reason_summary() {
    let mut page = seeded_work_item_sync_beta_page();
    let review = page
        .publish_reviews
        .iter_mut()
        .find(|review| review.publish_review_id == "work_item_sync:publish_review:create-comment")
        .expect("create-comment publish-review");
    review.publish_review_disposition_class = PublishReviewDispositionClass::BlockedByPolicy;
    review.block_reason_summary = None;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report.defects.iter().any(|defect| {
        defect.check_id == "work_item_sync_beta.publish_review_block_reason_missing"
    }));
}

#[test]
fn comment_origin_lifecycle_must_match_provider_observation() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .comment_sync_records
        .iter_mut()
        .find(|record| record.comment_sync_id == "work_item_sync:comment:provider-active")
        .expect("provider comment");
    record.comment_lifecycle_class = CommentLifecycleClass::LocalDraftCreateNeverPublished;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "work_item_sync_beta.comment_in_sync_truth"));
}

#[test]
fn failed_publish_comment_must_declare_typed_retry_route() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .comment_sync_records
        .iter_mut()
        .find(|record| record.comment_sync_id == "work_item_sync:comment:publish-failed")
        .expect("failed publish comment");
    record.typed_retry_route_class = None;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| { defect.check_id == "work_item_sync_beta.comment_failed_retry_truth" }));
}

#[test]
fn link_source_class_drives_imported_truth() {
    let mut page = seeded_work_item_sync_beta_page();
    let record = page
        .work_item_link_records
        .iter_mut()
        .find(|record| record.link_id == "work_item_sync:link:imported-handoff")
        .expect("imported handoff link");
    record.link_source_class = LinkSourceClass::ProviderAuthoritativeOverlay;

    let report = page.validate();
    assert!(!report.passed);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "work_item_sync_beta.link_imported_truth"));
}

#[test]
fn comment_conflict_class_helper_distinguishes_no_conflict_path() {
    assert!(!CommentConflictClass::NoConflict.is_conflict());
    assert!(CommentConflictClass::ConflictProviderEditedAfterDraft.is_conflict());
}
