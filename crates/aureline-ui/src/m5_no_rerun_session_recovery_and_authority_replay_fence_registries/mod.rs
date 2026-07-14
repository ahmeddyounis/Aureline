//! Implemented M5 no-rerun session-recovery-posture and authority-replay-fence registries.
//!
//! The frozen [window-restore matrix][matrix] names Aureline's five workspace-restore families and locks their
//! controlled vocabulary. This module is the runtime implement lane for no-hidden-rerun recovery of the
//! session-scoped surfaces — terminals, debug sessions, notebooks, previews, remote shells, and collaboration
//! panes — that the [no-rerun session hydration][family] family governs. It turns the *session-recovery-posture*
//! grammar and the *authority-replay-fence* grammar into registry resolvers that produce export-safe, honest
//! projections. Every claimed M5 restore then resolves each session-scoped surface to one explicit
//! reconnect-or-rerun posture — transcript restored, session ended, reconnect available, rerun required, or
//! context unavailable — instead of silently rerunning commands, and it fences off any silent reacquisition of a
//! privileged ticket, remote-attach authority, publish/deploy flow, notebook execution, or shared-control grant,
//! so context is preserved after restart without replaying mutating or privileged activity, provenance keeps
//! whether a surface is live, stale evidence, or awaiting fresh user intent, and a restore that only reopened
//! context or evidence can never read as truly live continuity.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Resolve every session-scoped surface to one explicit recovery-posture object per restore before any
//!   replay.** [`resolve_session_recovery_posture_entry`] refuses to read as a clean, registry-bound posture
//!   entry unless it names a canonical registry token, a classified
//!   [recovery-posture state][M5SessionRecoveryPostureState], a window-restore role, covers every
//!   [resolution form][M5SessionRecoveryOrchestrationResolutionForm] (the canonical object, the accessible
//!   summary, and the audit record), publishes every posture field (session surface, session scope, prior
//!   authority snapshot, provenance class, reconnect plan, and the distinct reauthorization plan), decides the
//!   explicit posture before any replay, and discloses reauthorization when the posture requires fresh user
//!   intent; otherwise it degrades.
//! * **Keep replay from preceding the explicit posture.** [`posture_precedes_replay`] rejects an entry whose
//!   session-scoped work or authority replayed before the explicit posture was decided so it degrades to
//!   [`M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture`], and the
//!   `reauthorization_disclosed_when_required` invariant degrades a fresh-intent posture that hid that
//!   reauthorization is required.
//! * **Fence off silent reacquisition of privileged authority.** [`resolve_authority_replay_fence_entry`] names
//!   a classified [authority-replay-fence class][M5AuthorityReplayFenceClass], requires the
//!   preserved-surface-role / prior-authority-class / provenance-hint disclosure triple, covers every resolution
//!   form, and degrades to
//!   [`M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims`] when the fence
//!   reruns session-scoped work or reacquires a privileged ticket, remote-attach authority, publish/deploy flow,
//!   notebook execution, or shared-control grant automatically, hides that reauthorization is required, or
//!   overclaims live continuity when only context or evidence restored, so a surface can never read as live when
//!   its session authority never actually returned.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5WindowRestoreRole`] role vocabulary and
//! the [`M5WindowRestoreConsumerSurface`] consumer-surface taxonomy — so the shell, recovery, diagnostics,
//! admin, workspace, session, docs, CLI, and support surfaces can never fork their own recovery-orchestration
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_window_restore_matrix
//! [family]: crate::m5_window_restore_matrix::M5WindowRestoreFamily::NoRerunSessionHydration

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries,
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed,
    seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed,
    M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_window_restore_matrix::{
    M5WindowRestoreAccessibilityRoute, M5WindowRestoreConsumerSurface,
    M5WindowRestoreDeploymentLine, M5WindowRestoreDowngradeTrigger, M5WindowRestoreFamily,
    M5WindowRestoreQualificationClass, M5WindowRestoreRequiredLabel, M5WindowRestoreRole,
    M5_RESTORE_FIDELITY_SCHEMA_REF, M5_WINDOW_RESTORE_MATRIX_DOC_REF,
    M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF, M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket`].
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_no_rerun_session_recovery_and_authority_replay_fence_registries";

/// Schema version for M5 no-rerun session-recovery / authority-replay-fence registry records.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_VERSION: u32 =
    1;

/// Repo-relative path of the combined registries schema.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-no-rerun-session-recovery-and-authority-replay-fence-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_DOC_REF: &str =
    "docs/recovery/m5_no_rerun_session_recovery_and_authority_replay_fence_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-no-rerun-session-recovery-and-authority-replay-fence-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no lane
/// invents a parallel surface set.
pub type M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerSurface =
    M5WindowRestoreConsumerSurface;

/// One of the three resolution forms every recovery-posture or authority-replay-fence entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// no-rerun-session-hydration *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryOrchestrationResolutionForm {
    /// The canonical resolved recovery-posture / authority-replay-fence object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved recovery discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved recovery inspectable off-renderer.
    AuditRecord,
}

impl M5SessionRecoveryOrchestrationResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled explicit recovery-posture state a session-recovery-posture entry resolves a session-scoped surface
/// to, so the canonical recovery model shares one registry rather than a hand-copied per-surface auto-rerun
/// assumption. Minted by this lane because the frozen matrix carries the workspace-restore families but not the
/// concrete transcript-restored / session-ended / reconnect-available / rerun-required / context-unavailable
/// posture model a recovery entry resolves against. Every classified state carries its canonical recovery mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryPostureState {
    /// The surface's transcript / context was restored read-only; no live session and no replay.
    TranscriptRestored,
    /// The prior session ended cleanly and is not being reattached.
    SessionEnded,
    /// A reconnect is available on explicit user intent, gated behind fresh reauthorization.
    ReconnectAvailable,
    /// A rerun is required on explicit user intent; the mutating or privileged work is never replayed silently.
    RerunRequired,
    /// The surface's session context is unavailable and only its shell remains.
    ContextUnavailable,
    /// The recovery-posture state is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5SessionRecoveryPostureState {
    /// Every recovery-posture state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TranscriptRestored,
        Self::SessionEnded,
        Self::ReconnectAvailable,
        Self::RerunRequired,
        Self::ContextUnavailable,
        Self::PostureUnclassified,
    ];

    /// The five canonical recovery-posture states every claimed M5 restore must resolve session-scoped surfaces
    /// to.
    pub const CANONICAL_STATES: [Self; 5] = [
        Self::TranscriptRestored,
        Self::SessionEnded,
        Self::ReconnectAvailable,
        Self::RerunRequired,
        Self::ContextUnavailable,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptRestored => "transcript_restored",
            Self::SessionEnded => "session_ended",
            Self::ReconnectAvailable => "reconnect_available",
            Self::RerunRequired => "rerun_required",
            Self::ContextUnavailable => "context_unavailable",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the state is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }

    /// The canonical recovery mode for this state.
    pub const fn canonical_recovery_posture_mode(self) -> &'static str {
        match self {
            Self::TranscriptRestored => "transcript_restored",
            Self::SessionEnded => "session_ended",
            Self::ReconnectAvailable => "reconnect_available",
            Self::RerunRequired => "rerun_required",
            Self::ContextUnavailable => "context_unavailable",
            Self::PostureUnclassified => "",
        }
    }

    /// Whether this posture would resume session-scoped work or reacquire authority on explicit user intent and
    /// so must disclose that reauthorization is required. Passive postures (a restored transcript, an ended
    /// session) never require fresh intent.
    pub const fn requires_fresh_user_intent(self) -> bool {
        matches!(
            self,
            Self::ReconnectAvailable | Self::RerunRequired | Self::ContextUnavailable
        )
    }
}

/// Controlled authority-replay-fence class an authority-replay-fence entry must resolve its fence from, so a
/// privileged session-scoped authority shares one registry rather than a hand-copied per-surface reacquisition
/// path. Minted by this lane, tracking the privileged flows the acceptance criteria require be blocked by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthorityReplayFenceClass {
    /// The privileged-ticket / remote-attach authority fence (privileged tickets and remote attach authority).
    PrivilegedTicketOrRemoteAttach,
    /// The publish-deploy / notebook-execution fence (publish/deploy flows and notebook execution).
    PublishDeployOrNotebookExecution,
    /// The shared-control-grant fence (collaboration shared-control grants).
    SharedControlGrant,
    /// The authority-replay-fence class is unclassified, which is disallowed.
    FenceClassUnclassified,
}

impl M5AuthorityReplayFenceClass {
    /// Every authority-replay-fence class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PrivilegedTicketOrRemoteAttach,
        Self::PublishDeployOrNotebookExecution,
        Self::SharedControlGrant,
        Self::FenceClassUnclassified,
    ];

    /// The three canonical fence classes every restore must block silent reacquisition across.
    pub const CANONICAL_CLASSES: [Self; 3] = [
        Self::PrivilegedTicketOrRemoteAttach,
        Self::PublishDeployOrNotebookExecution,
        Self::SharedControlGrant,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrivilegedTicketOrRemoteAttach => "privileged_ticket_or_remote_attach",
            Self::PublishDeployOrNotebookExecution => "publish_deploy_or_notebook_execution",
            Self::SharedControlGrant => "shared_control_grant",
            Self::FenceClassUnclassified => "fence_class_unclassified",
        }
    }

    /// Whether the authority-replay-fence class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::FenceClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a recovery-posture or
/// authority-replay-fence token's meaning stays stable whether it appears in the shell, recovery, diagnostics,
/// admin, or a support / export form. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryOrchestrationSurfaceContext {
    /// The shell surface.
    ShellSurface,
    /// The recovery surface.
    RecoverySurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5SessionRecoveryOrchestrationSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellSurface,
        Self::RecoverySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellSurface,
        Self::RecoverySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::RecoverySurface => "recovery_surface",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a recovery-posture or authority-replay-fence entry must be able to show, so no
/// recovery-posture state, session surface, session scope, prior authority, provenance, fence class,
/// reauthorization-plan hint, or registry fact is left implicit behind a hand-copied per-surface recovery
/// assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryOrchestrationAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The recovery-posture state the entry resolves (recovery-posture entry).
    RecoveryPostureState,
    /// The session surface and session scope the entry rebuilds context for (recovery-posture entry).
    SessionSurfaceAndScope,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The prior authority snapshot and provenance the entry publishes (recovery-posture entry).
    PriorAuthorityAndProvenance,
    /// The authority-replay-fence class the entry publishes (authority-replay-fence entry).
    AuthorityFenceClass,
    /// The distinct reauthorization plan kept separate from the passive recovery (both entries).
    ReauthorizationPlanHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved recovery or fence (both entries).
    PlainLanguageMeaning,
}

impl M5SessionRecoveryOrchestrationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RecoveryPostureState,
        Self::SessionSurfaceAndScope,
        Self::ResolutionFormCoverage,
        Self::PriorAuthorityAndProvenance,
        Self::AuthorityFenceClass,
        Self::ReauthorizationPlanHint,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::RecoveryPostureState => "recovery_posture_state",
            Self::SessionSurfaceAndScope => "session_surface_and_scope",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::PriorAuthorityAndProvenance => "prior_authority_and_provenance",
            Self::AuthorityFenceClass => "authority_fence_class",
            Self::ReauthorizationPlanHint => "reauthorization_plan_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// recovery posture, an authority-replay fence, or a degraded recovery-posture / authority-replay-fence entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryOrchestrationNextAction {
    /// Expand the resolved recovery's or fence's plain-language meaning.
    ExpandRecoveryMeaning,
    /// Inspect the recovery-posture state or authority-replay-fence class the entry resolves.
    InspectPostureOrFence,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5SessionRecoveryOrchestrationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRecoveryMeaning,
        Self::InspectPostureOrFence,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRecoveryMeaning => "expand_recovery_meaning",
            Self::InspectPostureOrFence => "inspect_posture_or_fence",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryOrchestrationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The window-restore families covered.
    WindowRestoreFamilies,
    /// The recovery-posture states carried.
    RecoveryPostureStates,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The authority-replay-fence classes carried.
    AuthorityReplayFenceClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The recovery-posture modes carried.
    RecoveryPostureModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5SessionRecoveryOrchestrationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::RecoveryPostureStates,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::AuthorityReplayFenceClasses,
        Self::SurfaceContext,
        Self::RecoveryPostureModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::RecoveryPostureStates,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::WindowRestoreFamilies => "window_restore_families",
            Self::RecoveryPostureStates => "recovery_posture_states",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::AuthorityReplayFenceClasses => "authority_replay_fence_classes",
            Self::SurfaceContext => "surface_context",
            Self::RecoveryPostureModes => "recovery_posture_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a recovery-posture entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, replay-first, field-incomplete, or form-incomplete
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionRecoveryPostureEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the posture means.
    PostureTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The recovery-posture state is unclassified (not in the resolved taxonomy).
    RecoveryPostureStateUnclassified,
    /// The behavior is a hand-copied per-surface recovery assumption instead of tracing to the canonical
    /// registry.
    PostureNotBoundToRegistry,
    /// The resolved recovery-posture object is incomplete: session surface, session scope, prior authority
    /// snapshot, provenance class, reconnect plan, or the distinct reauthorization plan is unstated.
    RecoveryPostureObjectIncomplete,
    /// Session-scoped work or authority replayed before the explicit posture was decided (an auto-rerun restore
    /// instead of an explicit-posture one).
    ReplayPrecededPosture,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// The posture requires fresh user intent but that reauthorization is not disclosed.
    ReauthorizationNotDisclosed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SessionRecoveryPostureEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PostureTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RecoveryPostureStateUnclassified,
        Self::PostureNotBoundToRegistry,
        Self::RecoveryPostureObjectIncomplete,
        Self::ReplayPrecededPosture,
        Self::ResolutionFormCoverageIncomplete,
        Self::ReauthorizationNotDisclosed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostureTokenUnstated => "posture_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RecoveryPostureStateUnclassified => "recovery_posture_state_unclassified",
            Self::PostureNotBoundToRegistry => "posture_not_bound_to_registry",
            Self::RecoveryPostureObjectIncomplete => "recovery_posture_object_incomplete",
            Self::ReplayPrecededPosture => "replay_preceded_posture",
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::ReauthorizationNotDisclosed => "reauthorization_not_disclosed",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SessionRecoveryOrchestrationNextAction {
        match self {
            Self::PostureTokenUnstated | Self::PostureNotBoundToRegistry => {
                M5SessionRecoveryOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::RecoveryPostureStateUnclassified
            | Self::RecoveryPostureObjectIncomplete
            | Self::ReplayPrecededPosture => {
                M5SessionRecoveryOrchestrationNextAction::InspectPostureOrFence
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5SessionRecoveryOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::ReauthorizationNotDisclosed
            | Self::ProofStale => M5SessionRecoveryOrchestrationNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::PostureTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RecoveryPostureStateUnclassified => {
                M5WindowRestoreDowngradeTrigger::RestoreFidelityClassUnstated
            }
            Self::PostureNotBoundToRegistry => {
                M5WindowRestoreDowngradeTrigger::WindowTopologyBoundaryDriftedBySurface
            }
            Self::RecoveryPostureObjectIncomplete => {
                M5WindowRestoreDowngradeTrigger::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget
            }
            Self::ReplayPrecededPosture | Self::ReauthorizationNotDisclosed => {
                M5WindowRestoreDowngradeTrigger::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an authority-replay-fence entry degraded below a clean, no-reacquire state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthorityReplayFenceEntryDegradeReason {
    /// The canonical registry token name is unstated.
    FenceTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The authority-replay-fence class is unclassified (not in the resolved taxonomy).
    AuthorityReplayFenceClassUnclassified,
    /// The fence reacquires or overclaims — it reran session-scoped work or reacquired a privileged ticket,
    /// remote-attach authority, publish/deploy flow, notebook execution, or shared-control grant automatically,
    /// hid that reauthorization is required, or overclaimed live continuity when only context or evidence
    /// restored, or it dropped the preserved-surface-role / prior-authority-class / provenance-hint disclosure
    /// triple.
    AuthorityReplayFenceReacquiresOrOverclaims,
    /// The canonical / accessible / audit resolution-form coverage of the fence is incomplete.
    FenceFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5AuthorityReplayFenceEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FenceTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::AuthorityReplayFenceClassUnclassified,
        Self::AuthorityReplayFenceReacquiresOrOverclaims,
        Self::FenceFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FenceTokenUnstated => "fence_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::AuthorityReplayFenceClassUnclassified => {
                "authority_replay_fence_class_unclassified"
            }
            Self::AuthorityReplayFenceReacquiresOrOverclaims => {
                "authority_replay_fence_reacquires_or_overclaims"
            }
            Self::FenceFormCoverageIncomplete => "fence_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5SessionRecoveryOrchestrationNextAction {
        match self {
            Self::FenceTokenUnstated => {
                M5SessionRecoveryOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::AuthorityReplayFenceClassUnclassified
            | Self::AuthorityReplayFenceReacquiresOrOverclaims => {
                M5SessionRecoveryOrchestrationNextAction::InspectPostureOrFence
            }
            Self::FenceFormCoverageIncomplete => {
                M5SessionRecoveryOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5SessionRecoveryOrchestrationNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::FenceTokenUnstated => M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::AuthorityReplayFenceClassUnclassified => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::AuthorityReplayFenceReacquiresOrOverclaims => {
                M5WindowRestoreDowngradeTrigger::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore
            }
            Self::FenceFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::SessionHydrationRuleUnstated
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_session_recovery_posture_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SessionRecoveryPostureEntryResolutionInput {
    /// Stable identity of the recovery-posture-registry entry.
    pub entry_id: String,
    /// The stable recovery-target ID this posture binds to (e.g. `recovery.acme.warm-reconnect`); empty means
    /// unstated.
    pub recovery_target_id: String,
    /// The canonical registry token name (e.g. `recovery.posture.transcript_restored`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The recovery-posture state this entry resolves.
    pub recovery_posture_state: M5SessionRecoveryPostureState,
    /// The render / surface context.
    pub surface_context: M5SessionRecoveryOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SessionRecoveryOrchestrationResolutionForm>,
    /// The published session surface ID; empty means unstated.
    pub session_surface_id: String,
    /// The published session scope; empty means unstated.
    pub session_scope: String,
    /// The published prior authority snapshot; empty means unstated.
    pub prior_authority_snapshot: String,
    /// The published provenance class (live / stale-evidence / awaiting-fresh-intent); empty means unstated.
    pub provenance_class: String,
    /// The published reconnect-plan reference; empty means unstated.
    pub reconnect_plan_ref: String,
    /// The published reauthorization-plan reference kept distinct from the passive recovery; empty means
    /// unstated.
    pub reauthorization_plan_ref: String,
    /// True when the behavior traces to the recovery-posture registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the explicit posture is decided before any session-scoped work or authority replays (a hard
    /// invariant when `false`).
    pub posture_decided_before_replay: bool,
    /// True when this posture would resume work or reacquire authority on explicit user intent.
    pub requires_fresh_user_intent: bool,
    /// True when reauthorization is disclosed for a posture that requires fresh user intent.
    pub reauthorization_disclosed_when_required: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe recovery-posture-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSessionRecoveryPostureEntry {
    /// Stable identity of the recovery-posture-registry entry.
    pub entry_id: String,
    /// The stable recovery-target ID this posture binds to.
    pub recovery_target_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The recovery-posture-state token named by the entry.
    pub recovery_posture_state: String,
    /// Whether the recovery-posture state is classified into the resolved taxonomy.
    pub recovery_posture_state_is_classified: bool,
    /// The canonical recovery mode for the entry's state.
    pub canonical_recovery_posture_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published session surface ID.
    pub session_surface_id: String,
    /// The published session scope.
    pub session_scope: String,
    /// The published prior authority snapshot.
    pub prior_authority_snapshot: String,
    /// The published provenance class.
    pub provenance_class: String,
    /// The published reconnect-plan reference.
    pub reconnect_plan_ref: String,
    /// The published reauthorization-plan reference.
    pub reauthorization_plan_ref: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved recovery-posture object publishes every required field.
    pub recovery_posture_object_complete: bool,
    /// Whether the entry traces to the recovery-posture registry.
    pub bound_to_registry: bool,
    /// Whether the explicit posture is decided before any replay.
    pub posture_decided_before_replay: bool,
    /// Whether this posture would resume work or reacquire authority on explicit user intent.
    pub requires_fresh_user_intent: bool,
    /// Whether reauthorization is disclosed when the posture requires fresh user intent.
    pub reauthorization_disclosed_when_required: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SessionRecoveryPostureEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SessionRecoveryOrchestrationNextAction,
    /// Whether the posture resolves to one stable object across every claimed recovery (clean entry naming every
    /// fact).
    pub posture_resolves_across_recoveries: bool,
}

impl M5ResolvedSessionRecoveryPostureEntry {
    /// Whether this recovery-posture entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_authority_replay_fence_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AuthorityReplayFenceEntryResolutionInput {
    /// Stable identity of the authority-replay-fence entry.
    pub entry_id: String,
    /// The stable guarded-surface ID this fence binds to; empty means unstated.
    pub guarded_surface_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The authority-replay-fence class this entry must resolve its fence from.
    pub fence_class: M5AuthorityReplayFenceClass,
    /// The render / surface context.
    pub surface_context: M5SessionRecoveryOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5SessionRecoveryOrchestrationResolutionForm>,
    /// The published preserved surface role kept while the authority stays fenced; empty means missing.
    pub preserved_surface_role: String,
    /// The published prior authority class (privileged / remote-attach / publish-deploy / notebook /
    /// shared-control); empty means missing.
    pub prior_authority_class: String,
    /// The published provenance hint (live / stale-evidence / awaiting-fresh-intent) kept distinct from a live
    /// claim; empty means missing.
    pub provenance_hint: String,
    /// True when the fence preserves the surface role and provenance (never a silent reacquisition).
    pub preserves_surface_and_provenance: bool,
    /// True when the fence is truthful (never reruns session-scoped work, reacquires broader authority, or
    /// overclaims live continuity).
    pub fence_is_truthful: bool,
    /// True when the surface previously held a privileged ticket / remote-attach / publish-deploy / notebook /
    /// shared-control authority.
    pub authority_was_held_used: bool,
    /// True when reauthorization is required-and-disclosed rather than silently reacquired for a previously held
    /// authority.
    pub reauthorization_required_disclosed: bool,
    /// True when a privileged flow was deferred to explicit user intent rather than replayed inline.
    pub privileged_flow_deferred: bool,
    /// True when a deferred privileged flow's fresh-intent requirement is disclosed rather than overclaimed.
    pub fresh_intent_required_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe authority-replay-fence projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAuthorityReplayFenceEntry {
    /// Stable identity of the authority-replay-fence entry.
    pub entry_id: String,
    /// The stable guarded-surface ID this fence binds to.
    pub guarded_surface_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The authority-replay-fence-class token named by the entry.
    pub fence_class: String,
    /// Whether the authority-replay-fence class is classified into the resolved taxonomy.
    pub fence_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published preserved surface role.
    pub preserved_surface_role: String,
    /// The published prior authority class.
    pub prior_authority_class: String,
    /// The published provenance hint.
    pub provenance_hint: String,
    /// Whether the fence preserves the surface role and provenance.
    pub preserves_surface_and_provenance: bool,
    /// Whether the fence is truthful.
    pub fence_is_truthful: bool,
    /// Whether the surface previously held a privileged authority.
    pub authority_was_held_used: bool,
    /// Whether reauthorization is required-and-disclosed for a previously held authority.
    pub reauthorization_required_disclosed: bool,
    /// Whether a privileged flow was deferred rather than replayed inline.
    pub privileged_flow_deferred: bool,
    /// Whether a deferred privileged flow's fresh-intent requirement is disclosed.
    pub fresh_intent_required_disclosed: bool,
    /// Whether the fence holds no-reacquire (no silent reacquisition, surface role and provenance preserved,
    /// prior authority reauthorization-disclosed, deferred fresh intent disclosed).
    pub fence_holds_no_reacquire: bool,
    /// Whether the entry provides the complete preserved-surface-role / prior-authority-class / provenance-hint
    /// disclosure triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, no-reacquire state.
    pub degrade_reason: Option<M5AuthorityReplayFenceEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5SessionRecoveryOrchestrationNextAction,
    /// Whether the fence holds on every claimed surface (clean entry naming every fact).
    pub fence_holds_on_every_surface: bool,
}

impl M5ResolvedAuthorityReplayFenceEntry {
    /// Whether this authority-replay-fence entry reads as a clean, no-reacquire state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SessionRecoveryOrchestrationResolutionError {
    /// The recovery-posture-entry id was empty.
    EmptyRecoveryPostureEntryId,
    /// The authority-replay-fence-entry id was empty.
    EmptyAuthorityReplayFenceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SessionRecoveryOrchestrationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRecoveryPostureEntryId => "empty_recovery_posture_entry_id",
            Self::EmptyAuthorityReplayFenceEntryId => "empty_authority_replay_fence_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SessionRecoveryOrchestrationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 recovery-posture / authority-replay-fence registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SessionRecoveryOrchestrationResolutionError {}

fn form_tokens(forms: &[M5SessionRecoveryOrchestrationResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5SessionRecoveryOrchestrationResolutionForm]) -> bool {
    let present: BTreeSet<M5SessionRecoveryOrchestrationResolutionForm> =
        forms.iter().copied().collect();
    M5SessionRecoveryOrchestrationResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved recovery-posture object publishes every required field: recovery mode (via a classified
/// state), session surface, session scope, prior authority snapshot, provenance class, reconnect plan, and the
/// distinct reauthorization plan. An unclassified state or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn recovery_posture_object_is_complete(
    state: M5SessionRecoveryPostureState,
    session_surface_id: &str,
    session_scope: &str,
    prior_authority_snapshot: &str,
    provenance_class: &str,
    reconnect_plan_ref: &str,
    reauthorization_plan_ref: &str,
) -> bool {
    state.is_classified()
        && !session_surface_id.trim().is_empty()
        && !session_scope.trim().is_empty()
        && !prior_authority_snapshot.trim().is_empty()
        && !provenance_class.trim().is_empty()
        && !reconnect_plan_ref.trim().is_empty()
        && !reauthorization_plan_ref.trim().is_empty()
}

/// Whether the explicit posture is decided before any replay: the state must be classified, the posture must be
/// decided before session-scoped work or authority replays, and a posture that requires fresh user intent must
/// disclose reauthorization. An unclassified state, a replay that preceded the posture, or a hidden
/// reauthorization never matches.
pub fn posture_precedes_replay(
    state: M5SessionRecoveryPostureState,
    posture_decided_before_replay: bool,
    requires_fresh_user_intent: bool,
    reauthorization_disclosed_when_required: bool,
) -> bool {
    state.is_classified()
        && posture_decided_before_replay
        && (!requires_fresh_user_intent || reauthorization_disclosed_when_required)
}

/// Whether an authority-replay fence holds no-reacquire and continuity-preserving: the class must be classified,
/// the fence must be truthful, it must preserve the surface role and provenance, any previously held authority
/// must be reauthorization-disclosed rather than silently reacquired, and any deferred privileged flow's
/// fresh-intent requirement must be disclosed rather than overclaimed.
pub fn authority_replay_fence_holds(
    fence_class: M5AuthorityReplayFenceClass,
    fence_is_truthful: bool,
    preserves_surface_and_provenance: bool,
    authority_was_held_used: bool,
    reauthorization_required_disclosed: bool,
    privileged_flow_deferred: bool,
    fresh_intent_required_disclosed: bool,
) -> bool {
    fence_class.is_classified()
        && fence_is_truthful
        && preserves_surface_and_provenance
        && (!authority_was_held_used || reauthorization_required_disclosed)
        && (!privileged_flow_deferred || fresh_intent_required_disclosed)
}

/// Resolves a recovery-posture-registry entry so it stays bound to the recovery-posture registry: the entry names
/// its canonical token, semantic role, and recovery-posture state, covers all three resolution forms, publishes a
/// complete recovery-posture object (session surface, session scope, prior authority snapshot, provenance class,
/// reconnect plan, distinct reauthorization plan), decides the explicit posture before any replay, and discloses
/// reauthorization when the posture requires fresh user intent.
pub fn resolve_session_recovery_posture_entry(
    input: M5SessionRecoveryPostureEntryResolutionInput,
) -> Result<M5ResolvedSessionRecoveryPostureEntry, M5SessionRecoveryOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SessionRecoveryOrchestrationResolutionError::EmptyRecoveryPostureEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.recovery_target_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.session_surface_id)
        || string_is_forbidden(&input.session_scope)
        || string_is_forbidden(&input.prior_authority_snapshot)
        || string_is_forbidden(&input.provenance_class)
        || string_is_forbidden(&input.reconnect_plan_ref)
        || string_is_forbidden(&input.reauthorization_plan_ref)
    {
        return Err(M5SessionRecoveryOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = recovery_posture_object_is_complete(
        input.recovery_posture_state,
        &input.session_surface_id,
        &input.session_scope,
        &input.prior_authority_snapshot,
        &input.provenance_class,
        &input.reconnect_plan_ref,
        &input.reauthorization_plan_ref,
    );
    let posture_ok = posture_precedes_replay(
        input.recovery_posture_state,
        input.posture_decided_before_replay,
        input.requires_fresh_user_intent,
        input.reauthorization_disclosed_when_required,
    );
    let reauth_undisclosed =
        input.requires_fresh_user_intent && !input.reauthorization_disclosed_when_required;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SessionRecoveryPostureEntryDegradeReason::PostureTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SessionRecoveryPostureEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.recovery_posture_state.is_classified() {
        Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureStateUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SessionRecoveryPostureEntryDegradeReason::PostureNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureObjectIncomplete)
    } else if !posture_ok {
        Some(M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture)
    } else if !all_forms {
        Some(M5SessionRecoveryPostureEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if reauth_undisclosed {
        Some(M5SessionRecoveryPostureEntryDegradeReason::ReauthorizationNotDisclosed)
    } else if !input.proof_fresh {
        Some(M5SessionRecoveryPostureEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SessionRecoveryOrchestrationNextAction::ExpandRecoveryMeaning,
    };

    Ok(M5ResolvedSessionRecoveryPostureEntry {
        entry_id: input.entry_id,
        recovery_target_id: input.recovery_target_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        recovery_posture_state: input.recovery_posture_state.as_str().to_owned(),
        recovery_posture_state_is_classified: input.recovery_posture_state.is_classified(),
        canonical_recovery_posture_mode: input
            .recovery_posture_state
            .canonical_recovery_posture_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        session_surface_id: input.session_surface_id,
        session_scope: input.session_scope,
        prior_authority_snapshot: input.prior_authority_snapshot,
        provenance_class: input.provenance_class,
        reconnect_plan_ref: input.reconnect_plan_ref,
        reauthorization_plan_ref: input.reauthorization_plan_ref,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        recovery_posture_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        posture_decided_before_replay: input.posture_decided_before_replay,
        requires_fresh_user_intent: input.requires_fresh_user_intent,
        reauthorization_disclosed_when_required: input.reauthorization_disclosed_when_required,
        degrade_reason,
        next_action,
        posture_resolves_across_recoveries: degrade_reason.is_none(),
    })
}

/// Resolves an authority-replay-fence entry so its fence holds no-reacquire: the entry names its canonical token,
/// semantic role, and authority-replay-fence class, covers all three resolution forms, provides the
/// preserved-surface-role / prior-authority-class / provenance-hint disclosure triple, and degrades honestly when
/// the fence reruns session-scoped work, reacquires broader authority, hides that reauthorization is required, or
/// overclaims live continuity on a deferred privileged flow.
pub fn resolve_authority_replay_fence_entry(
    input: M5AuthorityReplayFenceEntryResolutionInput,
) -> Result<M5ResolvedAuthorityReplayFenceEntry, M5SessionRecoveryOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(
            M5SessionRecoveryOrchestrationResolutionError::EmptyAuthorityReplayFenceEntryId,
        );
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.guarded_surface_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.preserved_surface_role)
        || string_is_forbidden(&input.prior_authority_class)
        || string_is_forbidden(&input.provenance_hint)
    {
        return Err(M5SessionRecoveryOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let fence_holds_no_reacquire = authority_replay_fence_holds(
        input.fence_class,
        input.fence_is_truthful,
        input.preserves_surface_and_provenance,
        input.authority_was_held_used,
        input.reauthorization_required_disclosed,
        input.privileged_flow_deferred,
        input.fresh_intent_required_disclosed,
    );
    let provides_triple = input.fence_class.is_classified()
        && !input.preserved_surface_role.trim().is_empty()
        && !input.prior_authority_class.trim().is_empty()
        && !input.provenance_hint.trim().is_empty()
        && fence_holds_no_reacquire;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5AuthorityReplayFenceEntryDegradeReason::FenceTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5AuthorityReplayFenceEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.fence_class.is_classified() {
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceClassUnclassified)
    } else if !provides_triple {
        Some(M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims)
    } else if !all_forms {
        Some(M5AuthorityReplayFenceEntryDegradeReason::FenceFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5AuthorityReplayFenceEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5SessionRecoveryOrchestrationNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedAuthorityReplayFenceEntry {
        entry_id: input.entry_id,
        guarded_surface_id: input.guarded_surface_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        fence_class: input.fence_class.as_str().to_owned(),
        fence_class_is_classified: input.fence_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        preserved_surface_role: input.preserved_surface_role,
        prior_authority_class: input.prior_authority_class,
        provenance_hint: input.provenance_hint,
        preserves_surface_and_provenance: input.preserves_surface_and_provenance,
        fence_is_truthful: input.fence_is_truthful,
        authority_was_held_used: input.authority_was_held_used,
        reauthorization_required_disclosed: input.reauthorization_required_disclosed,
        privileged_flow_deferred: input.privileged_flow_deferred,
        fresh_intent_required_disclosed: input.fresh_intent_required_disclosed,
        fence_holds_no_reacquire,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        fence_holds_on_every_surface: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved recovery-posture and authority-replay-fence
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5WindowRestoreQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Restore contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5WindowRestoreDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5WindowRestoreRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5WindowRestoreAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5SessionRecoveryOrchestrationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5SessionRecoveryOrchestrationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    /// Resolved recovery-posture-registry examples.
    pub recovery_posture_entries: Vec<M5ResolvedSessionRecoveryPostureEntry>,
    /// Resolved authority-replay-fence examples.
    pub authority_replay_fence_entries: Vec<M5ResolvedAuthorityReplayFenceEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the restore-fidelity and window-topology
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: session-scoped work never reruns or reacquires authority automatically after restore.
    /// MUST be `false`.
    pub reruns_session_scoped_work_or_reacquires_authority_automatically_after_restore: bool,
    /// Hard invariant: the fact that reauthorization is required is never hidden. MUST be `false`.
    pub hides_that_reauthorization_is_required: bool,
    /// Hard invariant: recovery-posture and authority-fence state are never merged into one opaque blob. MUST be
    /// `false`.
    pub merges_recovery_posture_and_authority_fence_into_one_opaque_blob: bool,
    /// Hard invariant: live continuity is never overclaimed when only context or evidence restored. MUST be
    /// `false`.
    pub overclaims_live_continuity_when_only_context_or_evidence_restored: bool,
}

impl M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SessionRecoveryOrchestrationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SessionRecoveryOrchestrationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5SessionRecoveryOrchestrationExportField> =
            self.export_fields.iter().copied().collect();
        M5SessionRecoveryOrchestrationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.reruns_session_scoped_work_or_reacquires_authority_automatically_after_restore
            && !self.hides_that_reauthorization_is_required
            && !self.merges_recovery_posture_and_authority_fence_into_one_opaque_blob
            && !self.overclaims_live_continuity_when_only_context_or_evidence_restored
    }

    /// True when a clean recovery-posture entry preserves registry-bound truth: it traces to the registry, keeps
    /// a classified recovery-posture state, publishes a complete posture object, decides the posture before
    /// replay, covers all three resolution forms, and discloses reauthorization when it requires fresh intent.
    fn posture_is_honest(ex: &M5ResolvedSessionRecoveryPostureEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.recovery_posture_state_is_classified
                && ex.recovery_posture_object_complete
                && ex.posture_decided_before_replay
                && ex.covers_all_resolution_forms
                && (!ex.requires_fresh_user_intent || ex.reauthorization_disclosed_when_required))
    }

    /// True when a clean authority-replay-fence entry preserves no-reacquire continuity: it keeps a classified
    /// class, provides the disclosure triple, holds no-reacquire, and covers all three resolution forms.
    fn fence_is_honest(ex: &M5ResolvedAuthorityReplayFenceEntry) -> bool {
        !ex.is_clean()
            || (ex.fence_class_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.fence_holds_no_reacquire
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.recovery_posture_entries
            .iter()
            .all(Self::posture_is_honest)
            && self
                .authority_replay_fence_entries
                .iter()
                .all(Self::fence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Recovery-posture-state tokens (minted by this lane).
    pub recovery_posture_states: Vec<String>,
    /// Authority-replay-fence-class tokens (minted by this lane).
    pub authority_replay_fence_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Recovery-posture-entry degrade-reason tokens.
    pub recovery_posture_degrade_reasons: Vec<String>,
    /// Authority-replay-fence-entry degrade-reason tokens.
    pub authority_replay_fence_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5WindowRestoreRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5SessionRecoveryOrchestrationResolutionForm::ALL, |v| {
                v.as_str()
            }),
            recovery_posture_states: tokens(&M5SessionRecoveryPostureState::ALL, |v| v.as_str()),
            authority_replay_fence_classes: tokens(&M5AuthorityReplayFenceClass::ALL, |v| {
                v.as_str()
            }),
            surface_contexts: tokens(&M5SessionRecoveryOrchestrationSurfaceContext::ALL, |v| {
                v.as_str()
            }),
            recovery_posture_degrade_reasons: tokens(
                &M5SessionRecoveryPostureEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            authority_replay_fence_degrade_reasons: tokens(
                &M5AuthorityReplayFenceEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5SessionRecoveryOrchestrationAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            next_actions: tokens(&M5SessionRecoveryOrchestrationNextAction::ALL, |v| {
                v.as_str()
            }),
            export_fields: tokens(&M5SessionRecoveryOrchestrationExportField::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5WindowRestoreConsumerSurface::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesGovernanceReview {
    /// The recovery-posture registry names a canonical token, semantic role, and recovery-posture state for
    /// every entry.
    pub posture_registry_names_token_role_and_recovery_state: bool,
    /// Every claimed restore resolves each session-scoped surface to one stable recovery-posture object from the
    /// shared registry, not per-surface reconstruction.
    pub recovery_resolves_to_stable_posture_object_from_shared_registry: bool,
    /// Session surface, session scope, prior authority snapshot, and provenance are published for every resolved
    /// recovery.
    pub session_surface_scope_prior_authority_and_provenance_published: bool,
    /// The explicit posture is decided before any session-scoped work or authority replays.
    pub posture_decided_before_authority_replay: bool,
    /// The authority fence blocks silent reacquisition of privileged tickets, remote attach, publish/deploy,
    /// notebook execution, or shared control, and never reruns session-scoped work.
    pub authority_fence_blocks_silent_reacquisition_and_never_reruns: bool,
    /// The fact that reauthorization is required is never hidden when fresh user intent is needed.
    pub reauthorization_never_hidden_when_required: bool,
    /// Every recovery-posture and authority-replay-fence entry covers the canonical / accessible / audit
    /// resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Recovery-posture and authority-fence behavior stay bound to the shared registries rather than hand-copied
    /// per surface.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shell, recovery, diagnostics, and admin read a single recovery-orchestration source.
    pub shell_recovery_diagnostics_admin_read_single_source: bool,
    /// A replay-first restore, an incomplete object, or a silent reacquisition is caught by fixtures before
    /// release evidence turns green.
    pub posture_or_fence_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerProjection {
    /// Shell and recovery consume the shared recovery-posture registry.
    pub shell_and_recovery_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared authority-replay-fence registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Session and workspace services consume the shared registries.
    pub session_and_workspace_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical restore-fidelity and window-topology domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical recovery-posture / authority-replay-fence registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting window-restore audit for the lane.
    pub window_restore_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review:
        M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 no-rerun session-recovery and authority-replay-fence registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket {
    /// Record kind; must equal
    /// [`M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review:
        M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection:
        M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(
        input: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacketInput,
    ) -> Self {
        Self {
            record_kind:
                M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_RECORD_KIND
                    .to_owned(),
            schema_version:
                M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the registries-packet invariants.
    pub fn validate(
        &self,
    ) -> Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect(
                "m5 recovery-posture / authority-replay-fence registries packet serializes",
            ),
        ) {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RawMaterialInExport,
            );
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 recovery-posture / authority-replay-fence registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,recovery_posture_entries,authority_replay_fence_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .recovery_posture_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.authority_replay_fence_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.recovery_posture_entries.len(),
                row.authority_replay_fence_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 No-Rerun Session-Recovery and Authority-Replay-Fence Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Recovery-posture states: {}\n",
            self.vocabulary_set.recovery_posture_states.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Recovery-posture entries: {} / authority-replay-fence entries: {}\n",
                row.recovery_posture_entries.len(),
                row.authority_replay_fence_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-recovery posture reference table generated from the registry, so docs and admin
    /// runbooks render the same recovery-mode / session-surface / session-scope / prior-authority / provenance /
    /// reconnect-plan truth the resolvers produced rather than a hand-copied recovery table. Only clean,
    /// registry-bound recovery-posture entries are listed.
    pub fn render_recovery_posture_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| recovery_target_id | recovery_posture_mode | session_surface_id | session_scope | prior_authority_snapshot | provenance_class | reconnect_plan_ref |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.recovery_posture_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.recovery_target_id,
                    ex.canonical_recovery_posture_mode,
                    ex.session_surface_id,
                    ex.session_scope,
                    ex.prior_authority_snapshot,
                    ex.provenance_class,
                    ex.reconnect_plan_ref
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>),
}

impl fmt::Display for M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 recovery-posture / authority-replay-fence registries export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 recovery-posture / authority-replay-fence registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError {}

/// Validation failures emitted by
/// [`M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the restore-fidelity and window-topology domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, replay-first, field-incomplete,
    /// form-incomplete, or an authority-replay-fence entry missing the disclosure triple).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Recovery-posture-resolution is not proven: clean posture entries do not cover the canonical recovery
    /// states or the first shell / recovery / diagnostics / admin / support surfaces, no object-incomplete
    /// example degrades, or a clean posture entry published an incomplete object.
    RecoveryPostureResolutionNotProven,
    /// Posture-before-replay is not proven: no replay-first example and no unbound example degrade, no clean
    /// explicit-posture entry is present, or a clean posture entry replayed first or is unbound.
    PostureBeforeReplayNotProven,
    /// Authority-fence continuity is not proven: clean authority-replay-fence entries do not cover the canonical
    /// privileged-ticket / publish-deploy / shared-control classes with full resolution-form coverage while
    /// providing the disclosure triple, no reacquires-or-overclaims or form-incomplete example degrades, or a
    /// clean fence entry is missing the triple.
    AuthorityFenceContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RecoveryPostureResolutionNotProven => "recovery_posture_resolution_not_proven",
            Self::PostureBeforeReplayNotProven => "posture_before_replay_not_proven",
            Self::AuthorityFenceContinuityNotProven => "authority_fence_continuity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_export(
) -> Result<
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError,
> {
    let packet: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-no-rerun-session-recovery-and-authority-replay-fence-registries-proof/support_export.json"
        )
    ))
    .map_err(M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_REF,
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::NoRegistryRows,
        );
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_RESTORE_FIDELITY_SCHEMA_REF)
            || !refs.contains(M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.recovery_posture_entries.is_empty() || row.authority_replay_fence_entries.is_empty()
        {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.posture_registry_names_token_role_and_recovery_state,
        review.recovery_resolves_to_stable_posture_object_from_shared_registry,
        review.session_surface_scope_prior_authority_and_provenance_published,
        review.posture_decided_before_authority_replay,
        review.authority_fence_blocks_silent_reacquisition_and_never_reruns,
        review.reauthorization_never_hidden_when_required,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shell_recovery_diagnostics_admin_read_single_source,
        review.posture_or_fence_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_recovery_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.session_and_workspace_services_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.window_restore_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket,
    violations: &mut Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation>,
) {
    let postures = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.recovery_posture_entries.iter())
    };
    let fences = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.authority_replay_fence_entries.iter())
    };

    // AC (support/export can distinguish context-only from truly live continuity): every claimed restore
    // resolves each session-scoped surface to one stable recovery-posture object with session-surface /
    // session-scope / prior-authority / provenance / distinct-reauth fields. Clean posture entries cover the
    // canonical recovery states and the first shell / recovery / diagnostics / admin / support surfaces, an
    // object-incomplete example degrades, and no clean posture entry published an incomplete object.
    let clean_states: BTreeSet<String> = postures()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.recovery_posture_state.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = postures()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let states_covered = M5SessionRecoveryPostureState::CANONICAL_STATES
        .iter()
        .all(|c| clean_states.contains(c.as_str()));
    let first_surfaces_covered = M5SessionRecoveryOrchestrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = postures().any(|ex| {
        ex.degrade_reason
            == Some(M5SessionRecoveryPostureEntryDegradeReason::RecoveryPostureObjectIncomplete)
    });
    let no_clean_incomplete =
        !postures().any(|ex| ex.is_clean() && !ex.recovery_posture_object_complete);
    if !(states_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::RecoveryPostureResolutionNotProven,
        );
    }

    // AC (surfaces never rerun or regain authority automatically): the explicit posture is decided before any
    // replay. A replay-first example degrades, an unbound example degrades, at least one clean explicit-posture
    // entry is present, and no clean posture entry replayed first or is unbound.
    let preceded_degrades = postures().any(|ex| {
        ex.degrade_reason == Some(M5SessionRecoveryPostureEntryDegradeReason::ReplayPrecededPosture)
    });
    let unbound_degrades = postures().any(|ex| {
        ex.degrade_reason
            == Some(M5SessionRecoveryPostureEntryDegradeReason::PostureNotBoundToRegistry)
    });
    let decided_clean_posture =
        postures().any(|ex| ex.is_clean() && ex.posture_decided_before_replay);
    let no_clean_unbound = !postures().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_replay_first =
        !postures().any(|ex| ex.is_clean() && !ex.posture_decided_before_replay);
    if !(preceded_degrades
        && unbound_degrades
        && decided_clean_posture
        && no_clean_unbound
        && no_clean_replay_first)
    {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::PostureBeforeReplayNotProven,
        );
    }

    // AC (recovery drills fail when restore reacquires authority or hides that reauthorization is required):
    // clean authority-replay-fence entries cover every canonical privileged-ticket / publish-deploy /
    // shared-control class with full resolution-form coverage while providing the disclosure triple, a
    // reacquires-or-overclaims example degrades, a form-incomplete example degrades, and no clean fence entry is
    // missing the triple.
    let clean_fence_classes: BTreeSet<String> = fences()
        .filter(|ex| {
            ex.is_clean()
                && ex.fence_class_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.fence_class.clone())
        .collect();
    let fence_classes_covered = M5AuthorityReplayFenceClass::CANONICAL_CLASSES
        .iter()
        .all(|s| clean_fence_classes.contains(s.as_str()));
    let reacquires_degrades = fences().any(|ex| {
        ex.degrade_reason
            == Some(
                M5AuthorityReplayFenceEntryDegradeReason::AuthorityReplayFenceReacquiresOrOverclaims,
            )
    });
    let form_incomplete_degrades = fences().any(|ex| {
        ex.degrade_reason
            == Some(M5AuthorityReplayFenceEntryDegradeReason::FenceFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !fences().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(fence_classes_covered
        && reacquires_degrades
        && form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesViolation::AuthorityFenceContinuityNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The window-restore family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WindowRestoreFamily; 1] =
    [M5WindowRestoreFamily::NoRerunSessionHydration];
