//! Deferred-publish recovery rows for the activity center.
//!
//! This module adapts the canonical deferred-publish recovery packet from
//! `aureline-review` into durable shell activity rows without inventing a new
//! lifecycle vocabulary. The exact deferred-publish state token stays in the
//! activity row phase label, while the row target and reopen identity reuse the
//! same canonical object and durable packet refs carried by queue, local
//! packet, and support-export surfaces.

use aureline_review::{
    canonical_deferred_publish_queue_recovery_packet, DeferredPublishActivityProjectionRow,
    DeferredPublishBlockReasonClass, DeferredPublishLifecycleState,
    DeferredPublishQueueRecoveryPacket, DeferredPublishRetryPostureClass,
};

use crate::notifications::envelope::{
    PrivacyClass, RedactionClass, SeverityClass, SourceSubsystem,
};

use super::alpha::{
    ActivityCancellabilityClass, ActivityCenterAlphaSnapshot, ActivityJobFamily,
    ActivityProgressForm, ActivityRow, ActivityRowAction, ActivityRowActionAvailability,
    ActivityRowActionKind, ActivityRowCollapseState, ActivityRowDisplayState, ActivityRowImpact,
    ActivityRowInput, ActivityRowProgress, ActivityRowStateClass, ActivityRowSupportLink,
    ActivityRowTimeline,
};

/// Builds activity-center rows from a deferred-publish recovery packet.
pub fn project_deferred_publish_activity_rows(
    packet: &DeferredPublishQueueRecoveryPacket,
) -> Vec<ActivityRow> {
    packet
        .activity_rows
        .iter()
        .map(|activity| activity_row_from_projection(packet, activity))
        .collect()
}

/// Builds an activity-center snapshot from the canonical deferred-publish packet.
pub fn seeded_deferred_publish_activity_snapshot() -> ActivityCenterAlphaSnapshot {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    ActivityCenterAlphaSnapshot::from_rows(project_deferred_publish_activity_rows(&packet))
}

fn activity_row_from_projection(
    packet: &DeferredPublishQueueRecoveryPacket,
    activity: &DeferredPublishActivityProjectionRow,
) -> ActivityRow {
    let queue_row = packet
        .queue_rows
        .iter()
        .find(|row| row.row_id == activity.queue_row_id_ref)
        .expect("activity projection must resolve its queue row");
    let support_row = packet
        .support_export
        .rows
        .iter()
        .find(|row| row.row_id == activity.support_export_row_ref)
        .expect("activity projection must resolve its support row");
    let state_class = state_class_for(activity.lifecycle_state);
    let retry_requires_revalidation = queue_row.replay_requires_fresh_target_identity
        || queue_row.replay_requires_current_effective_scope
        || queue_row.changed_boundary_review_required;
    let mut actions = vec![ActivityRowAction::open_details(
        format!(
            "action.activity.deferred_publish.open.{}",
            sanitize_id(&activity.row_id)
        ),
        "Open deferred publish details",
        activity.exact_reopen_ref.clone(),
    )];
    actions.push(ActivityRowAction {
        action_id: format!(
            "action.activity.deferred_publish.retry.{}",
            sanitize_id(&activity.row_id)
        ),
        action_kind: ActivityRowActionKind::RetryJob,
        label: "Retry deferred publish".to_owned(),
        command_id: Some("cmd:deferred_publish.retry".to_owned()),
        availability_class: retry_action_availability(
            activity.retry_posture_class,
            retry_requires_revalidation,
        ),
        disabled_reason_label: retry_disabled_reason(activity),
        target_identity_ref: activity.canonical_object_ref.clone(),
        preserves_durable_history: true,
        reissues_original_side_effect: true,
    });

    ActivityRow::from_input(ActivityRowInput {
        activity_row_id: activity.row_id.clone(),
        durable_job_id: format!("durable_job:{}", activity.queue_row_id_ref),
        canonical_event_id: format!("event:deferred_publish:{}", activity.queue_row_id_ref),
        canonical_object_target_ref: activity.canonical_object_ref.clone(),
        exact_reopen_identity_ref: activity.exact_reopen_ref.clone(),
        job_family: ActivityJobFamily::GitReview,
        source_subsystem: SourceSubsystem::ProviderBearing,
        actor_identity_ref: "id:actor:system:deferred-publish-recovery".to_owned(),
        actor_or_subsystem_label: "Deferred publish recovery".to_owned(),
        execution_origin_class: "deferred_publish_queue_recovery".to_owned(),
        severity_class: severity_for(activity.lifecycle_state),
        privacy_class: PrivacyClass::WorkspaceSensitive,
        summary_label: activity.summary_label.clone(),
        target_label: queue_row.canonical_object_label.clone(),
        target_scope_label: queue_row.queue_id.clone(),
        state_class,
        progress: ActivityRowProgress {
            forms: vec![progress_form_for(activity.lifecycle_state)],
            phase_label: activity.lifecycle_state.as_str().to_owned(),
            progress_bar: None,
            queue_reason_label: queue_reason_label(activity),
            approval_source_label: approval_source_label(activity.block_reason_class),
            actor_or_subsystem_label: "Deferred publish recovery".to_owned(),
            age_label: "Recorded".to_owned(),
            indeterminate_reason_label: None,
            expected_boundary_class: "provider_mutation_replay_review".to_owned(),
            cancellability_class: ActivityCancellabilityClass::AlreadyTerminal,
            detail_or_evidence_ref: Some(activity.support_export_row_ref.clone()),
        },
        timeline: ActivityRowTimeline {
            minted_at: packet.minted_at.clone(),
            queued_at: Some(packet.minted_at.clone()),
            started_at: None,
            last_observed_at: packet.minted_at.clone(),
            finished_at: matches!(activity.lifecycle_state, DeferredPublishLifecycleState::Published)
                .then(|| packet.minted_at.clone()),
            archived_at: None,
            superseded_by_row_id_ref: None,
            retention_label:
                "Retained until replay, publish, discard, or archive review completes".to_owned(),
        },
        impact: ActivityRowImpact {
            affects_cost: false,
            affects_policy: activity.block_reason_class
                == Some(DeferredPublishBlockReasonClass::RedactionPolicyBlocked),
            affects_network: true,
            affects_trust: true,
            affects_provider_state: true,
            affects_recovery_posture: true,
            detail_or_evidence_required: true,
            impact_summary_sentence:
                "Deferred publish rows preserve provider-mutation replay state, object identity, and durable recovery detail."
                    .to_owned(),
        },
        actions,
        display: ActivityRowDisplayState {
            collapse_state: ActivityRowCollapseState::CollapsedSummary,
            can_expand_inline: true,
            expand_reveals_label:
                "queue identity, replay policy, and durable support details".to_owned(),
        },
        support_link: ActivityRowSupportLink {
            exportable: true,
            support_pack_item_id: Some(support_row.row_id.clone()),
            bundle_member_path_ref: Some(activity.support_export_row_ref.clone()),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_private_material_excluded: true,
            export_field_refs: vec![
                activity.queue_row_id_ref.clone(),
                activity.local_packet_id_ref.clone(),
                activity.support_export_row_ref.clone(),
            ],
        },
        git_review_context: None,
        occurrence_count: 1,
    })
}

fn state_class_for(lifecycle_state: DeferredPublishLifecycleState) -> ActivityRowStateClass {
    match lifecycle_state {
        DeferredPublishLifecycleState::DraftOnly
        | DeferredPublishLifecycleState::QueuedForPublish => ActivityRowStateClass::QueuedWaiting,
        DeferredPublishLifecycleState::Blocked
        | DeferredPublishLifecycleState::StaleTarget
        | DeferredPublishLifecycleState::ConflictReviewRequired => {
            ActivityRowStateClass::NeedsApproval
        }
        DeferredPublishLifecycleState::Published => ActivityRowStateClass::Completed,
    }
}

fn severity_for(lifecycle_state: DeferredPublishLifecycleState) -> SeverityClass {
    match lifecycle_state {
        DeferredPublishLifecycleState::Blocked
        | DeferredPublishLifecycleState::StaleTarget
        | DeferredPublishLifecycleState::ConflictReviewRequired => SeverityClass::Warning,
        DeferredPublishLifecycleState::DraftOnly
        | DeferredPublishLifecycleState::QueuedForPublish
        | DeferredPublishLifecycleState::Published => SeverityClass::Info,
    }
}

fn progress_form_for(lifecycle_state: DeferredPublishLifecycleState) -> ActivityProgressForm {
    match lifecycle_state {
        DeferredPublishLifecycleState::DraftOnly
        | DeferredPublishLifecycleState::QueuedForPublish => ActivityProgressForm::QueueReason,
        DeferredPublishLifecycleState::Blocked
        | DeferredPublishLifecycleState::StaleTarget
        | DeferredPublishLifecycleState::ConflictReviewRequired => {
            ActivityProgressForm::FailureOrPartialSummary
        }
        DeferredPublishLifecycleState::Published => ActivityProgressForm::CompletionSummary,
    }
}

fn queue_reason_label(activity: &DeferredPublishActivityProjectionRow) -> Option<String> {
    match activity.lifecycle_state {
        DeferredPublishLifecycleState::DraftOnly => {
            Some("Draft remains local-only until publish is reviewed.".to_owned())
        }
        DeferredPublishLifecycleState::QueuedForPublish => {
            Some("Queued for later publish after provider recovery.".to_owned())
        }
        _ => None,
    }
}

fn approval_source_label(
    block_reason_class: Option<DeferredPublishBlockReasonClass>,
) -> Option<String> {
    block_reason_class.map(|reason| match reason {
        DeferredPublishBlockReasonClass::AuthDenied => {
            "Replay requires fresh auth and scope review.".to_owned()
        }
        DeferredPublishBlockReasonClass::ProviderOutage => {
            "Replay waits for provider health and target refresh.".to_owned()
        }
        DeferredPublishBlockReasonClass::ValidationConflict => {
            "Replay waits for compare-and-reconcile review.".to_owned()
        }
        DeferredPublishBlockReasonClass::RedactionPolicyBlocked => {
            "Replay waits for redaction review.".to_owned()
        }
        DeferredPublishBlockReasonClass::FreshTargetRequired => {
            "Replay waits for a refreshed target identity.".to_owned()
        }
        DeferredPublishBlockReasonClass::ChangedBoundaryNeedsReview => {
            "Replay waits for the changed boundary to be reviewed again.".to_owned()
        }
    })
}

fn retry_action_availability(
    retry_posture_class: DeferredPublishRetryPostureClass,
    retry_requires_revalidation: bool,
) -> ActivityRowActionAvailability {
    match retry_posture_class {
        DeferredPublishRetryPostureClass::NotApplicableDraftOrPublished => {
            ActivityRowActionAvailability::NotApplicable
        }
        DeferredPublishRetryPostureClass::OpenExternalOnly => {
            ActivityRowActionAvailability::Disabled
        }
        _ if retry_requires_revalidation => ActivityRowActionAvailability::RequiresRevalidation,
        _ => ActivityRowActionAvailability::Enabled,
    }
}

fn retry_disabled_reason(activity: &DeferredPublishActivityProjectionRow) -> Option<String> {
    match activity.retry_posture_class {
        DeferredPublishRetryPostureClass::NotApplicableDraftOrPublished => {
            Some("Retry is not meaningful for this lifecycle state.".to_owned())
        }
        DeferredPublishRetryPostureClass::OpenExternalOnly => {
            Some("Continuation is only safe through the reviewed external path.".to_owned())
        }
        _ => approval_source_label(activity.block_reason_class),
    }
}

fn sanitize_id(raw: &str) -> String {
    raw.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '_' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projected_rows_preserve_phase_vocabulary_and_object_identity() {
        let packet = canonical_deferred_publish_queue_recovery_packet();
        let rows = project_deferred_publish_activity_rows(&packet);

        assert_eq!(rows.len(), packet.activity_rows.len());
        for row in &rows {
            let source = packet
                .activity_rows
                .iter()
                .find(|activity| activity.row_id == row.activity_row_id)
                .expect("source activity row exists");
            assert_eq!(row.progress.phase_label, source.lifecycle_state.as_str());
            assert_eq!(row.canonical_object_target_ref, source.canonical_object_ref);
            assert_eq!(
                row.reopen_target.exact_target_identity_ref.as_deref(),
                Some(source.exact_reopen_ref.as_str())
            );
        }
    }

    #[test]
    fn attention_rows_keep_blocked_stale_and_conflict_states_distinct() {
        let snapshot = seeded_deferred_publish_activity_snapshot();
        let attention_rows = snapshot
            .rows
            .iter()
            .filter(|row| row.state_class == ActivityRowStateClass::NeedsApproval)
            .collect::<Vec<_>>();

        assert!(attention_rows
            .iter()
            .any(|row| row.progress.phase_label == "blocked"));
        assert!(attention_rows
            .iter()
            .any(|row| row.progress.phase_label == "stale_target"));
        assert!(attention_rows
            .iter()
            .any(|row| row.progress.phase_label == "conflict_review_required"));
    }
}
