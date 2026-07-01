//! Canonical seed builders for the frozen M5 lifecycle-state and
//! journey-checkpoint matrix.
//!
//! These builders are the single producer of the checked-in support export and
//! the narrowed fixtures. The headless emitter and the inline tests both call
//! them so the in-code matrix, the artifact, and the fixtures never drift.

use super::*;

/// Stable packet id for the canonical lifecycle matrix.
pub const M5_LIFECYCLE_MATRIX_PACKET_ID: &str = "m5-lifecycle-matrix:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-06-30T00:00:00Z";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn object_row(
    object_family: M5LifecycleObjectFamily,
    qualification: M5LifecycleQualificationClass,
    owner_role: &str,
    scope_summary: &str,
    required_fields: &[&str],
    admitted_states: Vec<M5LifecycleState>,
    primary_status_surface: M5PrimaryStatusSurface,
    status_code_export_field: &str,
    last_failure_reason_field: &str,
    last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
    recovery_affordance: M5RecoveryAffordance,
    required_proof_packet_refs: &[&str],
    downgrade_triggers: Vec<M5LifecycleDowngradeTrigger>,
    consumer_surfaces: Vec<M5LifecycleConsumerSurface>,
) -> M5ObjectStateRow {
    M5ObjectStateRow {
        object_family,
        qualification,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        required_fields: strings(required_fields),
        admitted_states,
        primary_status_surface,
        status_code_export_field: status_code_export_field.to_owned(),
        last_failure_reason_field: last_failure_reason_field.to_owned(),
        last_failure_reason_classes,
        recovery_affordance,
        evidence_requirement: M5LifecycleEvidenceRequirement::Required,
        required_proof_packet_refs: strings(required_proof_packet_refs),
        downgrade_triggers,
        source_contract_refs: strings(&[
            M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF,
            M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF,
        ]),
        consumer_surfaces,
    }
}

fn object_rows() -> Vec<M5ObjectStateRow> {
    use M5LastFailureReasonClass as R;
    use M5LifecycleConsumerSurface as C;
    use M5LifecycleDowngradeTrigger as D;
    use M5LifecycleObjectFamily as F;
    use M5LifecycleState as S;
    use M5PrimaryStatusSurface as P;
    use M5RecoveryAffordance as A;

    let default_triggers = || {
        vec![
            D::StateVocabularyDrift,
            D::StatusSurfaceMissing,
            D::LastFailureReasonMissing,
            D::ProofStale,
        ]
    };

    vec![
        object_row(
            F::Workspace,
            M5LifecycleQualificationClass::Stable,
            "Shell/workspace owner",
            "Workspace / window session that restores its identity, trust, and layout; its state machine reports readiness, restore progress, and degraded read-only fallback, and it always names a restore recovery affordance rather than a blank window",
            &[
                "workspace_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Stale,
                S::Rebuilding,
                S::Restricted,
                S::Reconnecting,
                S::Degraded,
                S::ReadOnlyDegraded,
                S::Unavailable,
            ],
            P::StatusBarIndicator,
            "workspace_status_code",
            "workspace_last_failure_reason",
            vec![
                R::NoFailure,
                R::DependencyUnavailable,
                R::IntegrityCheckFailed,
                R::ResourceExhausted,
            ],
            A::RestoreAction,
            &["evidence:workspace-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::Extension,
            M5LifecycleQualificationClass::Stable,
            "Extensions owner",
            "Installed extension / capability whose lifecycle spans warming, restricted, policy-blocked, deprecated, experimental, and retest-pending; it names a reinstall/re-enable recovery affordance and never hides a disabled capability",
            &[
                "extension_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Restricted,
                S::PolicyBlocked,
                S::Degraded,
                S::Unavailable,
                S::Deprecated,
                S::Experimental,
                S::RetestPending,
            ],
            P::ObjectHeaderBadge,
            "extension_status_code",
            "extension_last_failure_reason",
            vec![
                R::NoFailure,
                R::VersionIncompatible,
                R::PermissionDenied,
                R::PolicyBlock,
            ],
            A::ReinstallAction,
            &["evidence:extension-lifecycle-conformance:m5"],
            vec![
                D::StateVocabularyDrift,
                D::StatusSurfaceMissing,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::RemoteSession,
            M5LifecycleQualificationClass::Stable,
            "Remote owner",
            "Remote / tunnel session whose machine reports reconnecting, degraded, and read-only-degraded states with a named reconnect affordance, so a dropped connection is never an anonymous stall",
            &[
                "session_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Reconnecting,
                S::Degraded,
                S::ReadOnlyDegraded,
                S::Restricted,
                S::Unavailable,
            ],
            P::RemotePresenceIndicator,
            "remote_session_status_code",
            "remote_session_last_failure_reason",
            vec![
                R::NoFailure,
                R::ConnectionLost,
                R::TimeoutExceeded,
                R::PermissionDenied,
            ],
            A::ReconnectAction,
            &["evidence:remote-session-lifecycle-conformance:m5"],
            vec![
                D::StateVocabularyDrift,
                D::StatusSurfaceMissing,
                D::RecoveryAffordanceMissing,
                D::ProofStale,
            ],
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::CollaborationSession,
            M5LifecycleQualificationClass::Beta,
            "Collaboration owner",
            "Live collaboration session whose machine reports reconnecting, partial, and restricted states with a reconnect affordance; a lost co-editing link narrows the presence badge rather than silently dropping edits",
            &[
                "session_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Reconnecting,
                S::Partial,
                S::Degraded,
                S::Restricted,
                S::Unavailable,
            ],
            P::CollaborationPresenceBadge,
            "collaboration_session_status_code",
            "collaboration_session_last_failure_reason",
            vec![
                R::NoFailure,
                R::ConnectionLost,
                R::PermissionDenied,
                R::UpstreamDegraded,
            ],
            A::ReconnectAction,
            &["evidence:collaboration-session-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
        object_row(
            F::AiAction,
            M5LifecycleQualificationClass::Beta,
            "AI owner",
            "AI assistant action whose machine reports warming, partial, restricted, policy-blocked, and retest-pending states with a retry affordance; a blocked action narrows to a named reason rather than an anonymous spinner",
            &[
                "action_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Restricted,
                S::PolicyBlocked,
                S::Degraded,
                S::Unavailable,
                S::RetestPending,
                S::Experimental,
            ],
            P::InlineActionStatus,
            "ai_action_status_code",
            "ai_action_last_failure_reason",
            vec![
                R::NoFailure,
                R::PolicyBlock,
                R::TimeoutExceeded,
                R::UpstreamDegraded,
                R::UserCancelled,
            ],
            A::RetryAction,
            &["evidence:ai-action-lifecycle-conformance:m5"],
            vec![
                D::StateVocabularyDrift,
                D::AnonymousCheckpoint,
                D::PolicyBlocked,
                D::ProofStale,
            ],
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
        object_row(
            F::UpdateRollback,
            M5LifecycleQualificationClass::Stable,
            "Update owner",
            "Update / rollback lifecycle whose machine reports warming, rebuilding, rollback-available, and deprecated states with a named update-now or rollback affordance, so a failed update always offers a safe return path",
            &[
                "update_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Rebuilding,
                S::RollbackAvailable,
                S::Degraded,
                S::Unavailable,
                S::Deprecated,
            ],
            P::UpdateCenterRow,
            "update_rollback_status_code",
            "update_rollback_last_failure_reason",
            vec![
                R::NoFailure,
                R::IntegrityCheckFailed,
                R::VersionIncompatible,
                R::DependencyUnavailable,
            ],
            A::RollbackAction,
            &["evidence:update-rollback-lifecycle-conformance:m5"],
            vec![
                D::StateVocabularyDrift,
                D::StatusSurfaceMissing,
                D::RecoveryAffordanceMissing,
                D::ProofStale,
            ],
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
                C::ReleaseNotes,
            ],
        ),
        object_row(
            F::NotebookRuntime,
            M5LifecycleQualificationClass::Stable,
            "Notebook owner",
            "Notebook kernel runtime whose machine reports warming, partial, rebuilding, and read-only-degraded states with a rebuild affordance, so a crashed kernel narrows rather than losing the notebook",
            &[
                "runtime_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Rebuilding,
                S::Degraded,
                S::ReadOnlyDegraded,
                S::Unavailable,
                S::RetestPending,
            ],
            P::PanelHeaderStatus,
            "notebook_runtime_status_code",
            "notebook_runtime_last_failure_reason",
            vec![
                R::NoFailure,
                R::ResourceExhausted,
                R::DependencyUnavailable,
                R::TimeoutExceeded,
            ],
            A::RebuildAction,
            &["evidence:notebook-runtime-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::RequestApiRun,
            M5LifecycleQualificationClass::Stable,
            "API client owner",
            "Request / API run whose machine reports warming, partial, restricted, and policy-blocked states with a retry affordance; a failed request names a controlled reason instead of a raw error body",
            &[
                "run_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Restricted,
                S::PolicyBlocked,
                S::Degraded,
                S::Unavailable,
            ],
            P::InlineActionStatus,
            "request_api_run_status_code",
            "request_api_run_last_failure_reason",
            vec![
                R::NoFailure,
                R::TimeoutExceeded,
                R::ConnectionLost,
                R::PermissionDenied,
                R::PolicyBlock,
            ],
            A::RetryAction,
            &["evidence:request-api-run-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::PreviewSession,
            M5LifecycleQualificationClass::Beta,
            "Preview owner",
            "Preview / live-server session whose machine reports warming, rebuilding, and partial states with a rebuild affordance, so a broken build narrows the preview badge rather than blanking the panel",
            &[
                "preview_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Rebuilding,
                S::Partial,
                S::Degraded,
                S::Unavailable,
                S::Experimental,
            ],
            P::PanelHeaderStatus,
            "preview_session_status_code",
            "preview_session_last_failure_reason",
            vec![
                R::NoFailure,
                R::DependencyUnavailable,
                R::ResourceExhausted,
                R::TimeoutExceeded,
            ],
            A::RebuildAction,
            &["evidence:preview-session-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
        object_row(
            F::PipelineRun,
            M5LifecycleQualificationClass::Beta,
            "Pipeline owner",
            "Pipeline / task run whose machine reports warming, partial, and rebuilding states with a retry affordance and named checkpoints, so a long run never shows an anonymous progress bar",
            &[
                "run_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Rebuilding,
                S::Degraded,
                S::Unavailable,
                S::RetestPending,
            ],
            P::ActivityCenterRow,
            "pipeline_run_status_code",
            "pipeline_run_last_failure_reason",
            vec![
                R::NoFailure,
                R::DependencyUnavailable,
                R::ResourceExhausted,
                R::IntegrityCheckFailed,
            ],
            A::RetryAction,
            &["evidence:pipeline-run-lifecycle-conformance:m5"],
            vec![
                D::StateVocabularyDrift,
                D::AnonymousCheckpoint,
                D::StatusSurfaceMissing,
                D::ProofStale,
            ],
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
        object_row(
            F::DataSession,
            M5LifecycleQualificationClass::Stable,
            "Data owner",
            "Data / database session whose machine reports reconnecting, partial, stale, and read-only-degraded states with a reconnect affordance, so a dropped connection preserves a safe read path",
            &[
                "session_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Reconnecting,
                S::Partial,
                S::Stale,
                S::Degraded,
                S::ReadOnlyDegraded,
                S::Restricted,
                S::Unavailable,
            ],
            P::PanelHeaderStatus,
            "data_session_status_code",
            "data_session_last_failure_reason",
            vec![
                R::NoFailure,
                R::ConnectionLost,
                R::TimeoutExceeded,
                R::PermissionDenied,
            ],
            A::ReconnectAction,
            &["evidence:data-session-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
                C::ClaimTooling,
            ],
        ),
        object_row(
            F::ProfilerCapture,
            M5LifecycleQualificationClass::Preview,
            "Profiler owner",
            "Profiler / trace capture whose machine reports warming, partial, and experimental states with a retest affordance, so an interrupted capture narrows to a named reason rather than a silent failure",
            &[
                "capture_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Partial,
                S::Degraded,
                S::Unavailable,
                S::Experimental,
                S::RetestPending,
            ],
            P::ActivityCenterRow,
            "profiler_capture_status_code",
            "profiler_capture_last_failure_reason",
            vec![
                R::NoFailure,
                R::ResourceExhausted,
                R::TimeoutExceeded,
                R::UserCancelled,
            ],
            A::RetestAction,
            &["evidence:profiler-capture-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
        object_row(
            F::CompanionSession,
            M5LifecycleQualificationClass::Experimental,
            "Companion owner",
            "Companion / paired device session whose machine reports warming, reconnecting, restricted, and experimental states with a reconnect affordance, so a paired device that drops narrows the companion badge rather than vanishing",
            &[
                "session_id",
                "lifecycle_status_code",
                "primary_status_surface",
                "last_failure_reason",
                "recovery_affordance",
            ],
            vec![
                S::Ready,
                S::Warming,
                S::Reconnecting,
                S::Restricted,
                S::Degraded,
                S::Unavailable,
                S::Experimental,
            ],
            P::CollaborationPresenceBadge,
            "companion_session_status_code",
            "companion_session_last_failure_reason",
            vec![
                R::NoFailure,
                R::ConnectionLost,
                R::PermissionDenied,
                R::TimeoutExceeded,
            ],
            A::ReconnectAction,
            &["evidence:companion-session-lifecycle-conformance:m5"],
            default_triggers(),
            vec![
                C::ProductUi,
                C::Cli,
                C::DocsHelp,
                C::Diagnostics,
                C::SupportExport,
                C::Telemetry,
            ],
        ),
    ]
}

#[allow(clippy::too_many_arguments)]
fn journey_row(
    journey: M5CriticalJourney,
    object_family: M5LifecycleObjectFamily,
    owner_role: &str,
    scope_summary: &str,
    checkpoints: Vec<M5JourneyCheckpoint>,
    success_state: M5LifecycleState,
    recovery_affordance: M5RecoveryAffordance,
    last_failure_reason_classes: Vec<M5LastFailureReasonClass>,
) -> M5JourneyCheckpointRow {
    M5JourneyCheckpointRow {
        journey,
        object_family,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        checkpoints,
        shows_named_checkpoints: true,
        success_state,
        recovery_affordance,
        last_failure_reason_classes,
        source_contract_refs: strings(&[
            M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF,
            M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF,
        ]),
    }
}

fn journey_rows() -> Vec<M5JourneyCheckpointRow> {
    use M5CriticalJourney as J;
    use M5JourneyCheckpoint as K;
    use M5LastFailureReasonClass as R;
    use M5LifecycleObjectFamily as F;
    use M5LifecycleState as S;
    use M5RecoveryAffordance as A;

    vec![
        journey_row(
            J::WorkspaceRestore,
            F::Workspace,
            "Shell/workspace owner",
            "Workspace restore shows named checkpoints instead of a blank window while it rehydrates layout, trust, and open editors",
            vec![
                K::Queued,
                K::Authorizing,
                K::Restoring,
                K::Warming,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::RestoreAction,
            vec![R::NoFailure, R::IntegrityCheckFailed, R::DependencyUnavailable],
        ),
        journey_row(
            J::RemoteReconnect,
            F::RemoteSession,
            "Remote owner",
            "Remote reconnect shows named checkpoints while it re-establishes the tunnel and re-verifies trust",
            vec![
                K::Queued,
                K::Connecting,
                K::Authorizing,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::ReconnectAction,
            vec![R::NoFailure, R::ConnectionLost, R::TimeoutExceeded],
        ),
        journey_row(
            J::ExtensionActivation,
            F::Extension,
            "Extensions owner",
            "Extension activation shows named checkpoints while it authorizes, warms, and verifies the capability",
            vec![K::Queued, K::Authorizing, K::Warming, K::Verifying, K::Ready],
            S::Ready,
            A::ReinstallAction,
            vec![R::NoFailure, R::VersionIncompatible, R::PermissionDenied],
        ),
        journey_row(
            J::CollaborationJoin,
            F::CollaborationSession,
            "Collaboration owner",
            "Collaboration join shows named checkpoints while it connects, authorizes, and syncs presence",
            vec![
                K::Queued,
                K::Connecting,
                K::Authorizing,
                K::Warming,
                K::Ready,
            ],
            S::Ready,
            A::ReconnectAction,
            vec![R::NoFailure, R::ConnectionLost, R::PermissionDenied],
        ),
        journey_row(
            J::AiActionRun,
            F::AiAction,
            "AI owner",
            "AI action run shows named checkpoints while it authorizes, prepares context, and verifies output instead of an anonymous spinner",
            vec![
                K::Queued,
                K::Authorizing,
                K::Preparing,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::RetryAction,
            vec![R::NoFailure, R::PolicyBlock, R::TimeoutExceeded],
        ),
        journey_row(
            J::UpdateRollbackJourney,
            F::UpdateRollback,
            "Update owner",
            "Update / rollback shows named checkpoints while it prepares, verifies, and finalizes, always exposing a rollback terminal",
            vec![
                K::Queued,
                K::Preparing,
                K::Building,
                K::Verifying,
                K::Finalizing,
                K::Ready,
            ],
            S::Ready,
            A::RollbackAction,
            vec![R::NoFailure, R::IntegrityCheckFailed, R::VersionIncompatible],
        ),
        journey_row(
            J::NotebookExecution,
            F::NotebookRuntime,
            "Notebook owner",
            "Notebook execution shows named checkpoints while it warms the kernel, runs, and verifies cells",
            vec![K::Queued, K::Warming, K::Building, K::Verifying, K::Ready],
            S::Ready,
            A::RebuildAction,
            vec![R::NoFailure, R::ResourceExhausted, R::TimeoutExceeded],
        ),
        journey_row(
            J::RequestRun,
            F::RequestApiRun,
            "API client owner",
            "Request run shows named checkpoints while it authorizes, connects, and verifies the response",
            vec![
                K::Queued,
                K::Authorizing,
                K::Connecting,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::RetryAction,
            vec![R::NoFailure, R::TimeoutExceeded, R::ConnectionLost],
        ),
        journey_row(
            J::PreviewBuild,
            F::PreviewSession,
            "Preview owner",
            "Preview build shows named checkpoints while it prepares, builds, and warms the live server",
            vec![K::Queued, K::Preparing, K::Building, K::Warming, K::Ready],
            S::Ready,
            A::RebuildAction,
            vec![R::NoFailure, R::DependencyUnavailable, R::ResourceExhausted],
        ),
        journey_row(
            J::PipelineRunJourney,
            F::PipelineRun,
            "Pipeline owner",
            "Pipeline run shows named checkpoints while it prepares, builds, and verifies each stage instead of an anonymous progress bar",
            vec![
                K::Queued,
                K::Preparing,
                K::Building,
                K::Verifying,
                K::Finalizing,
                K::Ready,
            ],
            S::Ready,
            A::RetryAction,
            vec![R::NoFailure, R::DependencyUnavailable, R::IntegrityCheckFailed],
        ),
        journey_row(
            J::DataSessionConnect,
            F::DataSession,
            "Data owner",
            "Data session connect shows named checkpoints while it connects, authorizes, and verifies the schema",
            vec![
                K::Queued,
                K::Connecting,
                K::Authorizing,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::ReconnectAction,
            vec![R::NoFailure, R::ConnectionLost, R::PermissionDenied],
        ),
        journey_row(
            J::ProfilerCaptureJourney,
            F::ProfilerCapture,
            "Profiler owner",
            "Profiler capture shows named checkpoints while it prepares, captures, and finalizes the trace",
            vec![
                K::Queued,
                K::Preparing,
                K::Warming,
                K::Finalizing,
                K::Ready,
            ],
            S::Ready,
            A::RetestAction,
            vec![R::NoFailure, R::ResourceExhausted, R::UserCancelled],
        ),
        journey_row(
            J::CompanionAttach,
            F::CompanionSession,
            "Companion owner",
            "Companion attach shows named checkpoints while it connects, authorizes, and verifies the paired device",
            vec![
                K::Queued,
                K::Connecting,
                K::Authorizing,
                K::Verifying,
                K::Ready,
            ],
            S::Ready,
            A::ReconnectAction,
            vec![R::NoFailure, R::ConnectionLost, R::PermissionDenied],
        ),
    ]
}

fn state_binding_review() -> M5LifecycleStateBindingReview {
    M5LifecycleStateBindingReview {
        every_object_has_explicit_state_machine: true,
        every_object_has_one_primary_status_surface: true,
        every_object_has_one_exportable_status_code: true,
        every_object_has_one_last_failure_reason: true,
        every_object_has_one_named_recovery_affordance: true,
        controlled_terms_shared_across_ui_cli_docs_support_telemetry: true,
        protected_journeys_show_named_checkpoints: true,
        ready_state_defined_for_every_object: true,
        downgrade_narrows_instead_of_hides: true,
        later_rows_cannot_invent_private_state_vocabulary: true,
    }
}

fn consumer_projection() -> M5LifecycleConsumerProjection {
    M5LifecycleConsumerProjection {
        product_ui_consumes_lifecycle_matrix: true,
        cli_shows_status_code: true,
        docs_help_use_controlled_state_vocabulary: true,
        diagnostics_show_last_failure_reason: true,
        support_export_shows_object_state_model: true,
        telemetry_uses_controlled_state_codes: true,
        claim_tooling_reads_single_source: true,
        release_notes_use_controlled_vocabulary: true,
    }
}

fn proof_freshness() -> M5LifecycleProofFreshness {
    M5LifecycleProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5LifecycleReleasePosture {
    M5LifecycleReleasePosture {
        release_packet_ref: "evidence:lifecycle-matrix-release-packet:m5".to_owned(),
        recovery_drill_packet_ref: "evidence:lifecycle-matrix-recovery-drill-packet:m5".to_owned(),
        support_export_parity_required: true,
        telemetry_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_LIFECYCLE_OBJECT_STATE_SCHEMA_REF,
        M5_LIFECYCLE_JOURNEY_CHECKPOINT_SCHEMA_REF,
        M5_LIFECYCLE_MATRIX_DOC_REF,
        M5_LIFECYCLE_STATE_OBJECT_INVENTORY_REF,
        M5_LIFECYCLE_STATE_CLASS_RECOVERY_REF,
    ])
}

fn base_input() -> M5LifecycleMatrixPacketInput {
    M5LifecycleMatrixPacketInput {
        packet_id: M5_LIFECYCLE_MATRIX_PACKET_ID.to_owned(),
        matrix_label: "M5 Lifecycle-State, Degraded-Vocabulary, and Critical-Journey Checkpoint Matrix"
            .to_owned(),
        object_state_rows: object_rows(),
        journey_checkpoint_rows: journey_rows(),
        vocabulary_set: M5LifecycleVocabularySet::canonical(),
        state_binding_review: state_binding_review(),
        consumer_projection: consumer_projection(),
        proof_freshness: proof_freshness(),
        release_posture: release_posture(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: SEED_TIMESTAMP.to_owned(),
    }
}

/// Builds the canonical stable M5 lifecycle matrix packet.
///
/// This is the single producer of the checked-in support export.
pub fn seeded_m5_lifecycle_matrix() -> M5LifecycleMatrixPacket {
    M5LifecycleMatrixPacket::new(base_input())
}

/// Builds a narrowed variant where the remote session is pulled to Beta after a
/// degraded finding, proving downgrade narrows the claim rather than hiding the
/// object.
pub fn seeded_m5_lifecycle_matrix_remote_session_degraded_narrowed() -> M5LifecycleMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-lifecycle-matrix:remote-session-degraded-narrowed:0001".to_owned();
    for row in &mut input.object_state_rows {
        if row.object_family == M5LifecycleObjectFamily::RemoteSession {
            row.qualification = M5LifecycleQualificationClass::Beta;
        }
    }
    M5LifecycleMatrixPacket::new(input)
}

/// Builds a narrowed variant where the notebook runtime is pulled to Preview
/// after a retest-pending finding, proving auto-narrowing keeps the object
/// visible.
pub fn seeded_m5_lifecycle_matrix_notebook_runtime_retest_narrowed() -> M5LifecycleMatrixPacket {
    let mut input = base_input();
    input.packet_id = "m5-lifecycle-matrix:notebook-runtime-retest-narrowed:0001".to_owned();
    for row in &mut input.object_state_rows {
        if row.object_family == M5LifecycleObjectFamily::NotebookRuntime {
            row.qualification = M5LifecycleQualificationClass::Preview;
        }
    }
    M5LifecycleMatrixPacket::new(input)
}
