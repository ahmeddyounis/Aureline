//! Canonical seed builders for the M5 no-rerun session-recovery and authority-replay-fence registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean recovery-posture and authority-replay-fence entries
//! are built so the one stable recovery-posture object resolved per session-scoped surface, the explicit posture
//! decided before any replay, the prior authority snapshot and provenance kept distinct from the reauthorization
//! plan, the canonical / accessible / audit resolution forms, and the preserved-surface-role /
//! prior-authority-class / provenance-hint disclosure triple are proven across the shell, recovery, diagnostics,
//! admin, workspace, session, and support surfaces without any hand-copied per-surface assumption, replay-first
//! restore, incomplete object, silent reacquisition, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_PACKET_ID: &str =
    "m5-no-rerun-session-recovery-and-authority-replay-fence-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn posture(
    input: M5SessionRecoveryPostureEntryResolutionInput,
) -> M5ResolvedSessionRecoveryPostureEntry {
    resolve_session_recovery_posture_entry(input).expect("seed recovery-posture entry resolves")
}

fn fence(input: M5AuthorityReplayFenceEntryResolutionInput) -> M5ResolvedAuthorityReplayFenceEntry {
    resolve_authority_replay_fence_entry(input).expect("seed authority-replay-fence entry resolves")
}

fn all_forms() -> Vec<M5SessionRecoveryOrchestrationResolutionForm> {
    M5SessionRecoveryOrchestrationResolutionForm::ALL.to_vec()
}

// -- Clean recovery-posture entries (stable object, explicit-posture, bound to the registry) ----

#[allow(clippy::too_many_arguments)]
fn clean_posture_base(
    entry_id: &str,
    recovery_target_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    recovery_posture_state: M5SessionRecoveryPostureState,
    surface_context: M5SessionRecoveryOrchestrationSurfaceContext,
    session_surface_id: &str,
    session_scope: &str,
    prior_authority_snapshot: &str,
    provenance_class: &str,
    reconnect_plan_ref: &str,
    reauthorization_plan_ref: &str,
) -> M5SessionRecoveryPostureEntryResolutionInput {
    M5SessionRecoveryPostureEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        recovery_target_id: recovery_target_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        recovery_posture_state,
        surface_context,
        resolution_form_coverage: all_forms(),
        session_surface_id: session_surface_id.to_owned(),
        session_scope: session_scope.to_owned(),
        prior_authority_snapshot: prior_authority_snapshot.to_owned(),
        provenance_class: provenance_class.to_owned(),
        reconnect_plan_ref: reconnect_plan_ref.to_owned(),
        reauthorization_plan_ref: reauthorization_plan_ref.to_owned(),
        bound_to_registry: true,
        posture_decided_before_replay: true,
        requires_fresh_user_intent: false,
        reauthorization_disclosed_when_required: true,
        proof_fresh: true,
    }
}

fn posture_shell_transcript_clean() -> M5ResolvedSessionRecoveryPostureEntry {
    // A restored transcript is passive context — no live session, no reconnect, no fresh intent required.
    posture(clean_posture_base(
        "posture:shell:transcript",
        "recovery.acme.warm",
        "recovery.posture.transcript_restored",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::TranscriptRestored,
        M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    ))
}

fn posture_recovery_reconnect_clean() -> M5ResolvedSessionRecoveryPostureEntry {
    // A reconnect is available only on explicit user intent, gated behind disclosed reauthorization.
    let mut base = clean_posture_base(
        "posture:recovery:reconnect",
        "recovery.acme.reconnect",
        "recovery.posture.reconnect_available",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::ReconnectAvailable,
        M5SessionRecoveryOrchestrationSurfaceContext::RecoverySurface,
        "session-surface.remote-shell.secondary",
        "session-scope.remote",
        "authority-snapshot.remote-attach",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.available",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.reauthorization_disclosed_when_required = true;
    posture(base)
}

fn posture_diagnostics_context_unavailable_clean() -> M5ResolvedSessionRecoveryPostureEntry {
    // A context-unavailable surface keeps only its shell and requires fresh intent to resume.
    let mut base = clean_posture_base(
        "posture:diagnostics:context-unavailable",
        "recovery.acme.context-loss",
        "recovery.posture.context_unavailable",
        M5WindowRestoreRole::RestoreFidelity,
        M5SessionRecoveryPostureState::ContextUnavailable,
        M5SessionRecoveryOrchestrationSurfaceContext::DiagnosticsSurface,
        "session-surface.notebook.detached",
        "session-scope.detached",
        "authority-snapshot.publish-deploy",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.available",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.reauthorization_disclosed_when_required = true;
    posture(base)
}

fn posture_admin_rerun_clean() -> M5ResolvedSessionRecoveryPostureEntry {
    // A rerun-required surface never replays its mutating work silently; it awaits explicit intent.
    let mut base = clean_posture_base(
        "posture:admin:rerun",
        "recovery.acme.rerun",
        "recovery.posture.rerun_required",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::RerunRequired,
        M5SessionRecoveryOrchestrationSurfaceContext::AdminSurface,
        "session-surface.debugger.third",
        "session-scope.workspace",
        "authority-snapshot.shared-control",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.none",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.reauthorization_disclosed_when_required = true;
    posture(base)
}

fn posture_support_session_ended_clean() -> M5ResolvedSessionRecoveryPostureEntry {
    // An ended session is not being reattached; it is a passive terminal posture.
    posture(clean_posture_base(
        "posture:support:session-ended",
        "recovery.acme.ended",
        "recovery.posture.session_ended",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::SessionEnded,
        M5SessionRecoveryOrchestrationSurfaceContext::SupportOrExportForm,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    ))
}

// -- Degraded recovery-posture entries ----------------------------------------------------------

/// Degraded posture entry: the behavior is a hand-copied per-surface recovery assumption instead of tracing to
/// the registry.
fn posture_unbound() -> M5ResolvedSessionRecoveryPostureEntry {
    let mut base = clean_posture_base(
        "posture:admin:unbound",
        "recovery.acme.rerun",
        "recovery.posture.rerun_required",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::RerunRequired,
        M5SessionRecoveryOrchestrationSurfaceContext::AdminSurface,
        "session-surface.debugger.third",
        "session-scope.workspace",
        "authority-snapshot.shared-control",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.none",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.bound_to_registry = false;
    posture(base)
}

/// Degraded posture entry: the resolved posture object is incomplete — the session scope is unstated.
fn posture_object_incomplete() -> M5ResolvedSessionRecoveryPostureEntry {
    let mut base = clean_posture_base(
        "posture:shell:incomplete",
        "recovery.acme.warm",
        "recovery.posture.transcript_restored",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::TranscriptRestored,
        M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    );
    base.session_scope = "   ".to_owned();
    posture(base)
}

/// Degraded posture entry: session-scoped work replayed before the explicit posture was decided.
fn posture_replay_preceded() -> M5ResolvedSessionRecoveryPostureEntry {
    let mut base = clean_posture_base(
        "posture:diagnostics:replay-first",
        "recovery.acme.context-loss",
        "recovery.posture.context_unavailable",
        M5WindowRestoreRole::RestoreFidelity,
        M5SessionRecoveryPostureState::ContextUnavailable,
        M5SessionRecoveryOrchestrationSurfaceContext::DiagnosticsSurface,
        "session-surface.notebook.detached",
        "session-scope.detached",
        "authority-snapshot.publish-deploy",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.available",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.posture_decided_before_replay = false;
    posture(base)
}

/// Degraded posture entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn posture_form_incomplete() -> M5ResolvedSessionRecoveryPostureEntry {
    let mut base = clean_posture_base(
        "posture:recovery:form-incomplete",
        "recovery.acme.reconnect",
        "recovery.posture.reconnect_available",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::ReconnectAvailable,
        M5SessionRecoveryOrchestrationSurfaceContext::RecoverySurface,
        "session-surface.remote-shell.secondary",
        "session-scope.remote",
        "authority-snapshot.remote-attach",
        "provenance.awaiting-fresh-intent",
        "reconnect-plan.available",
        "reauth-plan.required",
    );
    base.requires_fresh_user_intent = true;
    base.reauthorization_disclosed_when_required = true;
    base.resolution_form_coverage =
        vec![M5SessionRecoveryOrchestrationResolutionForm::CanonicalObject];
    posture(base)
}

/// Degraded posture entry: the canonical registry token name is unstated.
fn posture_token_unstated() -> M5ResolvedSessionRecoveryPostureEntry {
    let mut base = clean_posture_base(
        "posture:support:token-unstated",
        "recovery.acme.ended",
        "  ",
        M5WindowRestoreRole::SessionHydration,
        M5SessionRecoveryPostureState::SessionEnded,
        M5SessionRecoveryOrchestrationSurfaceContext::SupportOrExportForm,
        "session-surface.terminal.main",
        "session-scope.workspace",
        "authority-snapshot.none",
        "provenance.stale-evidence",
        "reconnect-plan.none",
        "reauth-plan.none",
    );
    base.token_name = "  ".to_owned();
    posture(base)
}

// -- Clean authority-replay-fence entries -------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_fence_base(
    entry_id: &str,
    guarded_surface_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    fence_class: M5AuthorityReplayFenceClass,
    surface_context: M5SessionRecoveryOrchestrationSurfaceContext,
    preserved_surface_role: &str,
    prior_authority_class: &str,
    provenance_hint: &str,
) -> M5AuthorityReplayFenceEntryResolutionInput {
    M5AuthorityReplayFenceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        guarded_surface_id: guarded_surface_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        fence_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        preserved_surface_role: preserved_surface_role.to_owned(),
        prior_authority_class: prior_authority_class.to_owned(),
        provenance_hint: provenance_hint.to_owned(),
        preserves_surface_and_provenance: true,
        fence_is_truthful: true,
        authority_was_held_used: false,
        reauthorization_required_disclosed: false,
        privileged_flow_deferred: false,
        fresh_intent_required_disclosed: false,
        proof_fresh: true,
    }
}

fn fence_terminal_privileged_clean() -> M5ResolvedAuthorityReplayFenceEntry {
    fence(clean_fence_base(
        "fence:terminal:privileged",
        "surface.terminal.main",
        "fence.privileged.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PrivilegedTicketOrRemoteAttach,
        M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        "surface-role.terminal.main",
        "authority-class.none",
        "provenance.live-session",
    ))
}

fn fence_debugger_publish_clean() -> M5ResolvedAuthorityReplayFenceEntry {
    // A previously held publish/deploy authority is reauthorization-disclosed rather than silently reacquired.
    let mut base = clean_fence_base(
        "fence:debugger:publish",
        "surface.debugger.secondary",
        "fence.publish.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PublishDeployOrNotebookExecution,
        M5SessionRecoveryOrchestrationSurfaceContext::RecoverySurface,
        "surface-role.debugger.secondary",
        "authority-class.publish-deploy",
        "provenance.awaiting-fresh-intent",
    );
    base.authority_was_held_used = true;
    base.reauthorization_required_disclosed = true;
    fence(base)
}

fn fence_preview_shared_clean() -> M5ResolvedAuthorityReplayFenceEntry {
    // A deferred shared-control grant discloses its fresh-intent requirement rather than overclaiming live.
    let mut base = clean_fence_base(
        "fence:preview:shared",
        "surface.preview.detached",
        "fence.shared.no_reacquire",
        M5WindowRestoreRole::RestoreFidelity,
        M5AuthorityReplayFenceClass::SharedControlGrant,
        M5SessionRecoveryOrchestrationSurfaceContext::DiagnosticsSurface,
        "surface-role.preview.detached",
        "authority-class.shared-control",
        "provenance.awaiting-fresh-intent",
    );
    base.authority_was_held_used = true;
    base.reauthorization_required_disclosed = true;
    base.privileged_flow_deferred = true;
    base.fresh_intent_required_disclosed = true;
    fence(base)
}

fn fence_debugger_admin_clean() -> M5ResolvedAuthorityReplayFenceEntry {
    fence(clean_fence_base(
        "fence:debugger:admin",
        "surface.debugger.third",
        "fence.publish.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PublishDeployOrNotebookExecution,
        M5SessionRecoveryOrchestrationSurfaceContext::AdminSurface,
        "surface-role.debugger.third",
        "authority-class.none",
        "provenance.live-session",
    ))
}

fn fence_terminal_support_clean() -> M5ResolvedAuthorityReplayFenceEntry {
    fence(clean_fence_base(
        "fence:terminal:support",
        "surface.terminal.main",
        "fence.privileged.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PrivilegedTicketOrRemoteAttach,
        M5SessionRecoveryOrchestrationSurfaceContext::SupportOrExportForm,
        "surface-role.terminal.main",
        "authority-class.none",
        "provenance.live-session",
    ))
}

// -- Degraded authority-replay-fence entries ----------------------------------------------------

/// Degraded fence entry: a previously held authority was silently reacquired instead of requiring disclosed
/// reauthorization — the surface reads as live when its session authority never actually returned on fresh
/// intent.
fn fence_reacquires() -> M5ResolvedAuthorityReplayFenceEntry {
    let mut base = clean_fence_base(
        "fence:terminal:reacquires",
        "surface.terminal.main",
        "fence.privileged.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PrivilegedTicketOrRemoteAttach,
        M5SessionRecoveryOrchestrationSurfaceContext::ShellSurface,
        "surface-role.terminal.main",
        "authority-class.remote-attach",
        "provenance.live-session",
    );
    base.authority_was_held_used = true;
    base.reauthorization_required_disclosed = false;
    fence(base)
}

/// Degraded fence entry: the canonical / accessible / audit resolution-form coverage of the fence is incomplete.
fn fence_form_incomplete() -> M5ResolvedAuthorityReplayFenceEntry {
    let mut base = clean_fence_base(
        "fence:debugger:form-incomplete",
        "surface.debugger.secondary",
        "fence.publish.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::PublishDeployOrNotebookExecution,
        M5SessionRecoveryOrchestrationSurfaceContext::RecoverySurface,
        "surface-role.debugger.secondary",
        "authority-class.none",
        "provenance.live-session",
    );
    base.resolution_form_coverage =
        vec![M5SessionRecoveryOrchestrationResolutionForm::CanonicalObject];
    fence(base)
}

/// Degraded fence entry: the authority-replay-fence class is unclassified.
fn fence_class_unclassified() -> M5ResolvedAuthorityReplayFenceEntry {
    fence(clean_fence_base(
        "fence:admin:class-unclassified",
        "surface.debugger.third",
        "fence.unknown.no_reacquire",
        M5WindowRestoreRole::SessionHydration,
        M5AuthorityReplayFenceClass::FenceClassUnclassified,
        M5SessionRecoveryOrchestrationSurfaceContext::AdminSurface,
        "surface-role.debugger.third",
        "authority-class.none",
        "provenance.live-session",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    recovery_posture_entries: Vec<M5ResolvedSessionRecoveryPostureEntry>,
    authority_replay_fence_entries: Vec<M5ResolvedAuthorityReplayFenceEntry>,
) -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow {
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow {
        consumer_surface,
        qualification: M5WindowRestoreQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5WindowRestoreDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5WindowRestoreRequiredLabel::Identity,
            M5WindowRestoreRequiredLabel::SemanticRole,
            M5WindowRestoreRequiredLabel::RegistryReference,
            M5WindowRestoreRequiredLabel::RestoreFidelityClass,
            M5WindowRestoreRequiredLabel::DisplayAffinity,
        ],
        accessibility_routes: M5WindowRestoreAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5SessionRecoveryOrchestrationAnatomyPart::ALL.to_vec(),
        export_fields: M5SessionRecoveryOrchestrationExportField::ALL.to_vec(),
        downgrade_triggers,
        recovery_posture_entries,
        authority_replay_fence_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        ]),
        reruns_session_scoped_work_or_reacquires_authority_automatically_after_restore: false,
        hides_that_reauthorization_is_required: false,
        merges_recovery_posture_and_authority_fence_into_one_opaque_blob: false,
        overclaims_live_continuity_when_only_context_or_evidence_restored: false,
    }
}

fn registry_rows() -> Vec<M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesRow> {
    use M5WindowRestoreConsumerSurface as C;
    use M5WindowRestoreDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves each session-scoped surface to one stable recovery-posture object — session surface, session scope, prior authority snapshot, provenance class, reconnect plan, and the distinct reauthorization plan — from the shared registry, restores the terminal transcript read-only without rerunning it, and fences off any silent reacquisition of a privileged ticket; a posture object missing its session scope and a fence that silently reacquires a held authority degrade honestly instead of reading as a clean pass",
            "evidence:m5-session-recovery-orchestration-shell-ui:001",
            vec![
                D::RestoreFidelityClassUnstated,
                D::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore,
                D::ProofStale,
            ],
            vec![posture_shell_transcript_clean(), posture_object_incomplete()],
            vec![fence_terminal_privileged_clean(), fence_reacquires()],
        ),
        base_row(
            C::RestoreCoordinator,
            "Restore-coordinator owner",
            "The restore coordinator resolves a reconnect-available posture that gates the remote shell behind disclosed reauthorization, and fences a previously held publish/deploy authority behind disclosed reauthorization rather than silently reacquiring it; a resolution-form gap on a posture entry and on a fence entry is caught before a screenshot can reintroduce a false-live reading",
            "evidence:m5-session-recovery-orchestration-restore-coordinator:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
                D::ProofStale,
            ],
            vec![posture_recovery_reconnect_clean(), posture_form_incomplete()],
            vec![fence_debugger_publish_clean(), fence_form_incomplete()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the context-unavailable posture and the deferred shared-control fence that discloses its fresh-intent requirement rather than overclaiming live, without manual reconstruction; a posture whose session-scoped work replayed before the explicit posture was decided is caught as a replay-first restore",
            "evidence:m5-session-recovery-orchestration-diagnostics:001",
            vec![
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::SessionHydrationRuleUnstated,
                D::ProofStale,
            ],
            vec![
                posture_diagnostics_context_unavailable_clean(),
                posture_replay_preceded(),
            ],
            vec![fence_preview_shared_clean()],
        ),
        base_row(
            C::WorkspaceService,
            "Workspace-service owner",
            "The workspace service resolves the rerun-required posture while keeping it bound to the registry, and fences the debug authority; a posture that is a hand-copied per-surface recovery assumption and a fence on an unclassified authority class degrade honestly",
            "evidence:m5-session-recovery-orchestration-workspace-service:001",
            vec![
                D::SessionHydrationRuleUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![posture_admin_rerun_clean(), posture_unbound()],
            vec![fence_debugger_admin_clean(), fence_class_unclassified()],
        ),
        base_row(
            C::SessionService,
            "Session-service owner",
            "The session service renders the same resolved recovery-posture and authority-replay-fence truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied recovery table",
            "evidence:m5-session-recovery-orchestration-session-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SessionHydrationRuleUnstated,
                D::ProofStale,
            ],
            vec![
                posture_diagnostics_context_unavailable_clean(),
                posture_form_incomplete(),
            ],
            vec![fence_debugger_publish_clean(), fence_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved recovery-posture and authority-replay-fence truth, so a hand-copied constant, an unstated registry token, a replay-first restore, or a silent reacquisition is visible in evidence rather than hidden behind a screenshot, and it distinguishes context-only restore from truly live session continuity",
            "evidence:m5-session-recovery-orchestration-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::ProofStale,
            ],
            vec![posture_support_session_ended_clean(), posture_token_unstated()],
            vec![fence_terminal_support_clean()],
        ),
    ]
}

fn governance_review() -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesGovernanceReview
{
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesGovernanceReview {
        posture_registry_names_token_role_and_recovery_state: true,
        recovery_resolves_to_stable_posture_object_from_shared_registry: true,
        session_surface_scope_prior_authority_and_provenance_published: true,
        posture_decided_before_authority_replay: true,
        authority_fence_blocks_silent_reacquisition_and_never_reruns: true,
        reauthorization_never_hidden_when_required: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shell_recovery_diagnostics_admin_read_single_source: true,
        posture_or_fence_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection(
) -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerProjection {
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesConsumerProjection {
        shell_and_recovery_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        session_and_workspace_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesProofFreshness {
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesReleasePosture {
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesReleasePosture {
        proof_packet_ref:
            M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_ARTIFACT_REF
                .to_owned(),
        window_restore_audit_ref:
            M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_SCHEMA_REF,
        M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 no-rerun session-recovery and authority-replay-fence registries packet.
pub fn seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries(
) -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket {
    M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket::new(
        M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacketInput {
            packet_id:
                M5_NO_RERUN_SESSION_RECOVERY_AND_AUTHORITY_REPLAY_FENCE_REGISTRIES_PACKET_ID
                    .to_owned(),
            registries_label:
                "M5 no-rerun session-recovery and authority-replay-fence registries with one stable recovery-posture object resolved per session-scoped surface, the explicit posture decided before any replay, the prior authority snapshot and provenance kept distinct from the reauthorization plan, canonical / accessible / audit resolution-form coverage, and the preserved-surface-role / prior-authority-class / provenance-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesVocabularySet::canonical(),
            governance_review: governance_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            release_posture: release_posture(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: REDACTION_CLASS_TOKEN.to_owned(),
            minted_at: SEED_TIMESTAMP.to_owned(),
        },
    )
}

/// Narrowed variant: the restore-coordinator row is held at Beta pending reconnect-posture parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_reconnect_posture_beta_narrowed(
) -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.packet_id =
        "m5-no-rerun-session-recovery-and-authority-replay-fence-registries:reconnect-posture-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .expect("restore-coordinator row present");
    row.qualification = M5WindowRestoreQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending context-only continuity parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries_context_only_continuity_preview_narrowed(
) -> M5NoRerunSessionRecoveryAndAuthorityReplayFenceRegistriesPacket {
    let mut packet = seeded_m5_no_rerun_session_recovery_and_authority_replay_fence_registries();
    packet.packet_id =
        "m5-no-rerun-session-recovery-and-authority-replay-fence-registries:context-only-continuity-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5WindowRestoreQualificationClass::Preview;
    packet
}
