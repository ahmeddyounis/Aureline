use super::{
    canonical_deferred_publish_queue_recovery_packet,
    current_deferred_publish_queue_recovery_export, DeferredPublishBlockReasonClass,
    DeferredPublishLifecycleState,
};

#[test]
fn canonical_packet_validates() {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn canonical_packet_covers_every_lifecycle_state() {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    let states = packet
        .queue_rows
        .iter()
        .map(|row| row.lifecycle_state)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        states,
        std::collections::BTreeSet::from([
            DeferredPublishLifecycleState::DraftOnly,
            DeferredPublishLifecycleState::QueuedForPublish,
            DeferredPublishLifecycleState::Blocked,
            DeferredPublishLifecycleState::StaleTarget,
            DeferredPublishLifecycleState::ConflictReviewRequired,
            DeferredPublishLifecycleState::Published,
        ])
    );
}

#[test]
fn blocked_rows_cover_required_failure_classes() {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    let reasons = packet
        .queue_rows
        .iter()
        .filter_map(|row| row.block_reason_class)
        .collect::<std::collections::BTreeSet<_>>();
    for required in [
        DeferredPublishBlockReasonClass::AuthDenied,
        DeferredPublishBlockReasonClass::ProviderOutage,
        DeferredPublishBlockReasonClass::ValidationConflict,
        DeferredPublishBlockReasonClass::RedactionPolicyBlocked,
    ] {
        assert!(
            reasons.contains(&required),
            "missing required block reason {}",
            required.as_str()
        );
    }
}

#[test]
fn replay_rows_require_fresh_target_and_scope_and_never_auto_replay_high_impact() {
    let packet = canonical_deferred_publish_queue_recovery_packet();
    for row in packet
        .queue_rows
        .iter()
        .filter(|row| row.lifecycle_state.requires_replay_review())
    {
        assert!(row.replay_requires_fresh_target_identity, "{}", row.row_id);
        assert!(
            row.replay_requires_current_effective_scope,
            "{}",
            row.row_id
        );
        if row.high_impact_provider_mutation || row.changed_boundary_review_required {
            assert!(!row.auto_replay_allowed, "{}", row.row_id);
        }
    }
}

#[test]
fn checked_artifact_matches_the_seeded_packet() {
    let on_disk = current_deferred_publish_queue_recovery_export()
        .expect("checked deferred-publish recovery export must validate");
    let seeded = canonical_deferred_publish_queue_recovery_packet();
    assert_eq!(on_disk, seeded);
}
