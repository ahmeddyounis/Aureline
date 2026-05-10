//! Restore-job source for the durable activity center.
//!
//! Restore is the first long-running task class wired through the
//! activity-center seed. The shell's recovery flow already mints a
//! [`RestoreProposal`] before any rehydration runs; this module derives
//! a typed sequence of `(NotificationEnvelope, DurableJobObservation)`
//! tuples from that proposal so the activity center can update its
//! durable row without inventing a private timer or a parallel progress
//! field.
//!
//! The lifecycle the source emits per restore pass:
//!
//! 1. `Preparing` — the proposal has been built; counts and downgrade
//!    triggers are known but rehydration has not begun.
//! 2. `Running` — pane plans are being walked and dirty buffers re-read.
//!    Progress numerator/denominator come from the proposal counts so
//!    the row never claims a percentage the proposal cannot prove.
//! 3. `Completed` / `Failed` — terminal lifecycle. Failed rows quote a
//!    typed retryability class so a downstream renderer never has to
//!    decide whether retry is meaningful.
//!
//! Each lifecycle event reuses one shared `canonical_event_id` so the
//! activity center keeps one row across the entire restore pass; only
//! the `notification_envelope_id` differs between phases.

use aureline_recovery::session_restore::proposal::{
    RestoreProposal, RestoreProposalCounts, RestoreProposalPlanKind,
};

use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass, PrivacyPayloadClass,
    QuietHoursMode, RedactionClass, ReopenTarget, ReopenTargetKind, SeverityClass, SourceSubsystem,
    StableAction, SuppressionState, NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
};

use super::{
    ActivityRowLifecycleClass, ActivityRowProgress, ActivityRowRetryability,
    DurableJobObservation,
};

/// Stable command id for opening a restore job's details from the
/// durable row.
pub const RESTORE_OPEN_DETAILS_COMMAND_ID: &str = "cmd:activity.open_job_details";

/// Stable command id for retrying a failed restore.
pub const RESTORE_RETRY_COMMAND_ID: &str = "cmd:workspace.restore_from_checkpoint";

/// Lifecycle-specific seed input for the restore job source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RestoreLifecyclePhase {
    /// Pre-flight: the proposal has been built but rehydration has not
    /// started.
    Preparing,
    /// Active hydration: a numerator out of the proposal's total work
    /// surfaces has been walked.
    Running { numerator: u32 },
    /// Successful terminal lifecycle.
    Completed,
    /// Terminal failure with a typed retryability decision.
    Failed {
        retryability: ActivityRowRetryability,
        detail_label: &'static str,
        evidence_ref: Option<&'static str>,
    },
}

/// Identity inputs the restore-job source needs to build a stable
/// canonical event id and reopen target. The caller supplies the
/// workspace ref so the activity-center row joins back to the workspace.
#[derive(Debug, Clone)]
pub struct RestoreJobIdentity {
    pub workspace_ref: String,
    pub canonical_event_id: String,
    pub reopen_target_ref: String,
    pub canonical_object_target_ref: String,
}

impl RestoreJobIdentity {
    /// Build an identity record from a workspace ref. The canonical
    /// event id and reopen target ref are derived deterministically so
    /// two processes restoring the same workspace mint the same ids.
    pub fn for_workspace(workspace_ref: impl Into<String>) -> Self {
        let workspace_ref = workspace_ref.into();
        Self {
            canonical_event_id: format!("ux:event:restore:{workspace_ref}"),
            reopen_target_ref: format!("ux:reopen:restore:{workspace_ref}"),
            canonical_object_target_ref: format!("obj:restore:{workspace_ref}"),
            workspace_ref,
        }
    }
}

/// Mint a `(NotificationEnvelope, DurableJobObservation)` pair for one
/// restore lifecycle phase.
///
/// The envelope reuses one `canonical_event_id` per restore pass; phase
/// changes only differ on `notification_envelope_id` and lifecycle
/// observation. The reopen target is always exact and points at the
/// canonical restore object so a deduped row in the activity center
/// still leads back to the same canonical object.
pub fn mint_restore_lifecycle_event(
    identity: &RestoreJobIdentity,
    proposal: &RestoreProposal,
    phase: RestoreLifecyclePhase,
    minted_at: impl Into<String>,
) -> (NotificationEnvelope, DurableJobObservation) {
    let minted_at = minted_at.into();
    let phase_token = lifecycle_phase_token(phase);
    let envelope_id = format!(
        "ux:notif-env:restore:{}:{}",
        identity.workspace_ref, phase_token
    );

    let summary_label = lifecycle_summary_label(phase, &proposal.counts);
    let severity_class = lifecycle_severity(phase);

    let envelope = NotificationEnvelope {
        record_kind: "notification_envelope_record".into(),
        notification_envelope_schema_version: NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: envelope_id,
        canonical_event_id: identity.canonical_event_id.clone(),
        event_lineage_id_ref: format!("ux:lineage:restore:{}", identity.workspace_ref),
        source_subsystem: SourceSubsystem::Shell,
        source_event_ref: format!("shell:restore:{}", identity.workspace_ref),
        actor_identity_ref: "id:actor:system:shell-restore".into(),
        canonical_object_target_ref: identity.canonical_object_target_ref.clone(),
        severity_class,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: identity.canonical_event_id.clone(),
        grouped_burst_id_ref: None,
        recommended_surfaces: vec![
            FanoutSurfaceClass::DurableJobRow,
            FanoutSurfaceClass::StatusItem,
        ],
        summary_label,
        reopen_target: ReopenTarget {
            reopen_target_ref: identity.reopen_target_ref.clone(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some(identity.canonical_object_target_ref.clone()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: format!("ux:action:restore:open:{}", identity.workspace_ref),
            label: "Open restore details".into(),
            command_id: RESTORE_OPEN_DETAILS_COMMAND_ID.into(),
            target_identity_ref: identity.canonical_object_target_ref.clone(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at,
    };

    let observation = lifecycle_observation(phase, proposal);
    (envelope, observation)
}

fn lifecycle_phase_token(phase: RestoreLifecyclePhase) -> &'static str {
    match phase {
        RestoreLifecyclePhase::Preparing => "preparing",
        RestoreLifecyclePhase::Running { .. } => "running",
        RestoreLifecyclePhase::Completed => "completed",
        RestoreLifecyclePhase::Failed { .. } => "failed",
    }
}

fn lifecycle_severity(phase: RestoreLifecyclePhase) -> SeverityClass {
    match phase {
        RestoreLifecyclePhase::Preparing | RestoreLifecyclePhase::Running { .. } => {
            SeverityClass::Info
        }
        RestoreLifecyclePhase::Completed => SeverityClass::Success,
        RestoreLifecyclePhase::Failed { .. } => SeverityClass::Error,
    }
}

fn lifecycle_summary_label(
    phase: RestoreLifecyclePhase,
    counts: &RestoreProposalCounts,
) -> String {
    let total = restore_total_units(counts);
    match phase {
        RestoreLifecyclePhase::Preparing => "Restore preparing".into(),
        RestoreLifecyclePhase::Running { numerator } => {
            format!("Restore running ({numerator}/{total} surfaces)")
        }
        RestoreLifecyclePhase::Completed => "Restore completed".into(),
        RestoreLifecyclePhase::Failed { detail_label, .. } => {
            format!("Restore failed: {detail_label}")
        }
    }
}

fn lifecycle_observation(
    phase: RestoreLifecyclePhase,
    proposal: &RestoreProposal,
) -> DurableJobObservation {
    let total = restore_total_units(&proposal.counts);
    match phase {
        RestoreLifecyclePhase::Preparing => DurableJobObservation::in_flight(
            ActivityRowLifecycleClass::Preparing,
            None,
        ),
        RestoreLifecyclePhase::Running { numerator } => {
            let progress = ActivityRowProgress::new("Hydrating restore surfaces", numerator, total);
            DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, Some(progress))
        }
        RestoreLifecyclePhase::Completed => DurableJobObservation::completed(
            completion_label(proposal),
            None,
        ),
        RestoreLifecyclePhase::Failed {
            retryability,
            detail_label,
            evidence_ref,
        } => DurableJobObservation::failed(
            retryability,
            detail_label,
            evidence_ref.map(|s| s.to_owned()),
        ),
    }
}

fn restore_total_units(counts: &RestoreProposalCounts) -> u32 {
    let total = counts.windows
        + counts.tab_groups
        + counts.tabs
        + counts.dirty_buffer_journals
        + counts.transient_tasks
        + counts.terminals;
    let total = total.max(1);
    u32::try_from(total).unwrap_or(u32::MAX)
}

fn completion_label(proposal: &RestoreProposal) -> String {
    let counts = &proposal.counts;
    let blocked = proposal
        .pane_plans
        .iter()
        .filter(|plan| {
            matches!(
                plan.plan_kind,
                RestoreProposalPlanKind::BlockedSideEffectful
                    | RestoreProposalPlanKind::PlaceholderOnly
            )
        })
        .count();
    format!(
        "Restored {windows} windows, {tabs} tabs, {drafts} drafts; {blocked} surfaces left as \
         placeholders.",
        windows = counts.windows,
        tabs = counts.tabs,
        drafts = counts.dirty_buffer_journals,
        blocked = blocked,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::activity_center::{ActivityCenterStore, ActivityRowLifecycleClass};
    use crate::notifications::router::NotificationRouter;
    use aureline_recovery::crash_journal::CrashJournalStore;
    use aureline_recovery::session_restore::SessionRestoreStore;

    fn build_proposal() -> RestoreProposal {
        let dir = tempfile::tempdir().expect("tempdir");
        let session_store = SessionRestoreStore::new(dir.path());
        let crash_store = CrashJournalStore::new(dir.path());
        RestoreProposal::build(&session_store, &crash_store, true).expect("build proposal")
    }

    #[test]
    fn restore_lifecycle_lands_durable_row_through_router() {
        let proposal = build_proposal();
        let identity = RestoreJobIdentity::for_workspace("ws-protected-walk");
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();

        for (phase, minted_at) in [
            (RestoreLifecyclePhase::Preparing, "2026-05-10T15:00:00Z"),
            (
                RestoreLifecyclePhase::Running { numerator: 1 },
                "2026-05-10T15:00:01Z",
            ),
            (
                RestoreLifecyclePhase::Running { numerator: 2 },
                "2026-05-10T15:00:02Z",
            ),
            (RestoreLifecyclePhase::Completed, "2026-05-10T15:00:05Z"),
        ] {
            let (envelope, observation) =
                mint_restore_lifecycle_event(&identity, &proposal, phase, minted_at);
            let routed = router.route(&envelope).expect("route restore lifecycle");
            store
                .record_observation(&routed, &observation)
                .expect("record restore observation");
        }

        let snapshot = store.snapshot();
        assert_eq!(snapshot.len(), 1);
        let row = snapshot
            .find(&identity.canonical_event_id)
            .expect("row keyed by canonical event id");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert!(row.is_terminal);
        assert_eq!(row.minted_at, "2026-05-10T15:00:00Z");
        assert_eq!(row.last_observed_at, "2026-05-10T15:00:05Z");
    }

    #[test]
    fn failed_lifecycle_records_retryable_row() {
        let proposal = build_proposal();
        let identity = RestoreJobIdentity::for_workspace("ws-failure-drill");
        let mut router = NotificationRouter::new();
        let mut store = ActivityCenterStore::in_memory();

        let (envelope, observation) = mint_restore_lifecycle_event(
            &identity,
            &proposal,
            RestoreLifecyclePhase::Failed {
                retryability: ActivityRowRetryability::Available,
                detail_label: "Snapshot read returned a corrupt frame.",
                evidence_ref: Some("evidence:restore:ws-failure-drill:snapshot-read"),
            },
            "2026-05-10T16:00:00Z",
        );
        let routed = router.route(&envelope).expect("route fail");
        store
            .record_observation(&routed, &observation)
            .expect("record fail");

        let row = store
            .find_by_canonical_event(&identity.canonical_event_id)
            .expect("row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Failed);
        assert_eq!(row.retryability, ActivityRowRetryability::Available);
        assert!(row.detail.is_some());
    }
}
