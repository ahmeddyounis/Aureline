//! Canonical seed builders for the M5 skeleton-first-restore and session-hydration registries packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean skeleton-restore and session-hydration entries are
//! built so the one stable restore-skeleton object rebuilt per restore, the layout skeleton rebuilt before any
//! heavy dependency hydrates, the preserved pane roles and placeholder set kept distinct from the
//! deferred-hydration plan, the canonical / accessible / audit resolution forms, and the preserved-pane-role /
//! missing-dependency-class / restore-fidelity-hint disclosure triple are proven across the shell, recovery,
//! diagnostics, admin, workspace, session, and support surfaces without any hand-copied per-pane assumption,
//! hydration-first restore, incomplete object, silent layout collapse, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_PACKET_ID: &str =
    "m5-skeleton-first-restore-and-session-hydration-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn skeleton(input: M5SkeletonRestoreEntryResolutionInput) -> M5ResolvedSkeletonRestoreEntry {
    resolve_skeleton_restore_entry(input).expect("seed skeleton-restore entry resolves")
}

fn hydration(input: M5SessionHydrationEntryResolutionInput) -> M5ResolvedSessionHydrationEntry {
    resolve_session_hydration_entry(input).expect("seed session-hydration entry resolves")
}

fn all_forms() -> Vec<M5RestoreOrchestrationResolutionForm> {
    M5RestoreOrchestrationResolutionForm::ALL.to_vec()
}

// -- Clean skeleton-restore entries (stable object, skeleton-first, bound to the registry) ------

#[allow(clippy::too_many_arguments)]
fn clean_skeleton_base(
    entry_id: &str,
    restore_target_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    restore_fidelity_class: M5RestoreFidelityClass,
    surface_context: M5RestoreOrchestrationSurfaceContext,
    window_shell_id: &str,
    pane_tree_structure: &str,
    pane_role_set: &str,
    placeholder_set: &str,
    layout_skeleton_root: &str,
    hydration_plan_ref: &str,
) -> M5SkeletonRestoreEntryResolutionInput {
    M5SkeletonRestoreEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        restore_target_id: restore_target_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        restore_fidelity_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        window_shell_id: window_shell_id.to_owned(),
        pane_tree_structure: pane_tree_structure.to_owned(),
        pane_role_set: pane_role_set.to_owned(),
        placeholder_set: placeholder_set.to_owned(),
        layout_skeleton_root: layout_skeleton_root.to_owned(),
        hydration_plan_ref: hydration_plan_ref.to_owned(),
        bound_to_registry: true,
        skeleton_rebuilt_before_hydration: true,
        defers_heavy_hydration: false,
        pane_roles_preserved_when_deferred: true,
        proof_fresh: true,
    }
}

fn skeleton_shell_live_clean() -> M5ResolvedSkeletonRestoreEntry {
    skeleton(clean_skeleton_base(
        "skeleton:shell:live",
        "restore.acme.warm",
        "restore.skeleton.live",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::LiveHydratedPane,
        M5RestoreOrchestrationSurfaceContext::ShellSurface,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    ))
}

fn skeleton_recovery_placeholder_clean() -> M5ResolvedSkeletonRestoreEntry {
    // A placeholder pane defers heavy hydration and keeps its pane roles.
    let mut base = clean_skeleton_base(
        "skeleton:recovery:placeholder",
        "restore.acme.cold-start",
        "restore.skeleton.placeholder",
        M5WindowRestoreRole::PaneRole,
        M5RestoreFidelityClass::PaneRolePlaceholder,
        M5RestoreOrchestrationSurfaceContext::RecoverySurface,
        "window-shell.main",
        "pane-tree.main.v4",
        "pane-roles.editor|terminal|debugger",
        "placeholders.pane.0011",
        "layout-skeleton.acme/cold-start",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.pane_roles_preserved_when_deferred = true;
    skeleton(base)
}

fn skeleton_diagnostics_context_only_clean() -> M5ResolvedSkeletonRestoreEntry {
    // A context-only pane defers heavy hydration and keeps its pane roles distinct.
    let mut base = clean_skeleton_base(
        "skeleton:diagnostics:context-only",
        "restore.acme.crash-loop",
        "restore.skeleton.context_only",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::ContextOnlyPane,
        M5RestoreOrchestrationSurfaceContext::DiagnosticsSurface,
        "window-shell.detached-inspector",
        "pane-tree.detached.v1",
        "pane-roles.notebook|preview",
        "placeholders.pane.0011",
        "layout-skeleton.acme/crash-loop",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.pane_roles_preserved_when_deferred = true;
    skeleton(base)
}

fn skeleton_admin_evidence_only_clean() -> M5ResolvedSkeletonRestoreEntry {
    // An evidence-only pane defers heavy hydration and keeps its pane roles distinct.
    let mut base = clean_skeleton_base(
        "skeleton:admin:evidence-only",
        "restore.acme.remote-reconnect",
        "restore.skeleton.evidence_only",
        M5WindowRestoreRole::PaneRole,
        M5RestoreFidelityClass::EvidenceOnlyPane,
        M5RestoreOrchestrationSurfaceContext::AdminSurface,
        "window-shell.secondary",
        "pane-tree.secondary.v2",
        "pane-roles.remote-shell|collab",
        "placeholders.pane.0019",
        "layout-skeleton.acme/remote-reconnect",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.pane_roles_preserved_when_deferred = true;
    skeleton(base)
}

fn skeleton_support_live_clean() -> M5ResolvedSkeletonRestoreEntry {
    skeleton(clean_skeleton_base(
        "skeleton:support:live",
        "restore.acme.warm",
        "restore.skeleton.live",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::LiveHydratedPane,
        M5RestoreOrchestrationSurfaceContext::SupportOrExportForm,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    ))
}

// -- Degraded skeleton-restore entries ----------------------------------------------------------

/// Degraded skeleton entry: the behavior is a hand-copied per-pane restore assumption instead of tracing to the
/// registry.
fn skeleton_unbound() -> M5ResolvedSkeletonRestoreEntry {
    let mut base = clean_skeleton_base(
        "skeleton:admin:unbound",
        "restore.acme.remote-reconnect",
        "restore.skeleton.evidence_only",
        M5WindowRestoreRole::PaneRole,
        M5RestoreFidelityClass::EvidenceOnlyPane,
        M5RestoreOrchestrationSurfaceContext::AdminSurface,
        "window-shell.secondary",
        "pane-tree.secondary.v2",
        "pane-roles.remote-shell|collab",
        "placeholders.pane.0019",
        "layout-skeleton.acme/remote-reconnect",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.bound_to_registry = false;
    skeleton(base)
}

/// Degraded skeleton entry: the resolved skeleton object is incomplete — the pane-tree structure is unstated.
fn skeleton_object_incomplete() -> M5ResolvedSkeletonRestoreEntry {
    let mut base = clean_skeleton_base(
        "skeleton:shell:incomplete",
        "restore.acme.warm",
        "restore.skeleton.live",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::LiveHydratedPane,
        M5RestoreOrchestrationSurfaceContext::ShellSurface,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    );
    base.pane_tree_structure = "   ".to_owned();
    skeleton(base)
}

/// Degraded skeleton entry: heavy hydration ran before the layout skeleton was rebuilt.
fn skeleton_hydration_preceded() -> M5ResolvedSkeletonRestoreEntry {
    let mut base = clean_skeleton_base(
        "skeleton:diagnostics:hydration-first",
        "restore.acme.crash-loop",
        "restore.skeleton.context_only",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::ContextOnlyPane,
        M5RestoreOrchestrationSurfaceContext::DiagnosticsSurface,
        "window-shell.detached-inspector",
        "pane-tree.detached.v1",
        "pane-roles.notebook|preview",
        "placeholders.pane.0011",
        "layout-skeleton.acme/crash-loop",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.skeleton_rebuilt_before_hydration = false;
    skeleton(base)
}

/// Degraded skeleton entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn skeleton_form_incomplete() -> M5ResolvedSkeletonRestoreEntry {
    let mut base = clean_skeleton_base(
        "skeleton:recovery:form-incomplete",
        "restore.acme.cold-start",
        "restore.skeleton.placeholder",
        M5WindowRestoreRole::PaneRole,
        M5RestoreFidelityClass::PaneRolePlaceholder,
        M5RestoreOrchestrationSurfaceContext::RecoverySurface,
        "window-shell.main",
        "pane-tree.main.v4",
        "pane-roles.editor|terminal|debugger",
        "placeholders.pane.0011",
        "layout-skeleton.acme/cold-start",
        "hydration-plan.deferred",
    );
    base.defers_heavy_hydration = true;
    base.pane_roles_preserved_when_deferred = true;
    base.resolution_form_coverage = vec![M5RestoreOrchestrationResolutionForm::CanonicalObject];
    skeleton(base)
}

/// Degraded skeleton entry: the canonical registry token name is unstated.
fn skeleton_token_unstated() -> M5ResolvedSkeletonRestoreEntry {
    let mut base = clean_skeleton_base(
        "skeleton:support:token-unstated",
        "restore.acme.warm",
        "  ",
        M5WindowRestoreRole::LayoutSkeleton,
        M5RestoreFidelityClass::LiveHydratedPane,
        M5RestoreOrchestrationSurfaceContext::SupportOrExportForm,
        "window-shell.main",
        "pane-tree.main.v3",
        "pane-roles.editor|terminal|preview",
        "placeholders.none.0007",
        "layout-skeleton.acme/warm",
        "hydration-plan.inline",
    );
    base.token_name = "  ".to_owned();
    skeleton(base)
}

// -- Clean session-hydration entries ------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_hydration_base(
    entry_id: &str,
    pane_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    hydration_surface: M5SessionHydrationSurface,
    surface_context: M5RestoreOrchestrationSurfaceContext,
    preserved_pane_role: &str,
    missing_dependency_class: &str,
    restore_fidelity_hint: &str,
) -> M5SessionHydrationEntryResolutionInput {
    M5SessionHydrationEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        pane_id: pane_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        hydration_surface,
        surface_context,
        resolution_form_coverage: all_forms(),
        preserved_pane_role: preserved_pane_role.to_owned(),
        missing_dependency_class: missing_dependency_class.to_owned(),
        restore_fidelity_hint: restore_fidelity_hint.to_owned(),
        preserves_pane_role_and_topology: true,
        hydration_is_truthful: true,
        dependency_missing_used: false,
        placeholder_substituted_on_missing: false,
        heavy_dependency_deferred: false,
        deferred_fidelity_disclosed: false,
        proof_fresh: true,
    }
}

fn hydration_terminal_shell_clean() -> M5ResolvedSessionHydrationEntry {
    hydration(clean_hydration_base(
        "hydration:terminal:shell",
        "pane.terminal.main",
        "hydration.terminal.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::TerminalOrRemoteShellHydration,
        M5RestoreOrchestrationSurfaceContext::ShellSurface,
        "pane-role.terminal.main",
        "dependency.present",
        "restore-fidelity.live",
    ))
}

fn hydration_debugger_recovery_clean() -> M5ResolvedSessionHydrationEntry {
    // A missing debugger dependency is placeholder-substituted rather than collapsing the pane.
    let mut base = clean_hydration_base(
        "hydration:debugger:recovery",
        "pane.debugger.secondary",
        "hydration.debugger.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::DebuggerOrNotebookHydration,
        M5RestoreOrchestrationSurfaceContext::RecoverySurface,
        "pane-role.debugger.secondary",
        "dependency.missing",
        "restore-fidelity.placeholder",
    );
    base.dependency_missing_used = true;
    base.placeholder_substituted_on_missing = true;
    hydration(base)
}

fn hydration_preview_diagnostics_clean() -> M5ResolvedSessionHydrationEntry {
    // A deferred heavy preview dependency discloses its restore fidelity rather than overclaiming live.
    let mut base = clean_hydration_base(
        "hydration:preview:diagnostics",
        "pane.preview.detached",
        "hydration.preview.no_rerun",
        M5WindowRestoreRole::RestoreFidelity,
        M5SessionHydrationSurface::PreviewOrCollaborationHydration,
        M5RestoreOrchestrationSurfaceContext::DiagnosticsSurface,
        "pane-role.preview.detached",
        "dependency.expired",
        "restore-fidelity.context-only",
    );
    base.dependency_missing_used = true;
    base.placeholder_substituted_on_missing = true;
    base.heavy_dependency_deferred = true;
    base.deferred_fidelity_disclosed = true;
    hydration(base)
}

fn hydration_debugger_admin_clean() -> M5ResolvedSessionHydrationEntry {
    hydration(clean_hydration_base(
        "hydration:debugger:admin",
        "pane.debugger.third",
        "hydration.debugger.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::DebuggerOrNotebookHydration,
        M5RestoreOrchestrationSurfaceContext::AdminSurface,
        "pane-role.debugger.third",
        "dependency.present",
        "restore-fidelity.live",
    ))
}

fn hydration_terminal_support_clean() -> M5ResolvedSessionHydrationEntry {
    hydration(clean_hydration_base(
        "hydration:terminal:support",
        "pane.terminal.main",
        "hydration.terminal.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::TerminalOrRemoteShellHydration,
        M5RestoreOrchestrationSurfaceContext::SupportOrExportForm,
        "pane-role.terminal.main",
        "dependency.present",
        "restore-fidelity.live",
    ))
}

// -- Degraded session-hydration entries ---------------------------------------------------------

/// Degraded hydration entry: a missing dependency collapsed the layout instead of substituting a
/// pane-role-preserving placeholder — the pane reads as gone when its heavy dependency merely failed to
/// hydrate.
fn hydration_collapses() -> M5ResolvedSessionHydrationEntry {
    let mut base = clean_hydration_base(
        "hydration:terminal:collapses",
        "pane.terminal.main",
        "hydration.terminal.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::TerminalOrRemoteShellHydration,
        M5RestoreOrchestrationSurfaceContext::ShellSurface,
        "pane-role.terminal.main",
        "dependency.quarantined",
        "restore-fidelity.placeholder",
    );
    base.dependency_missing_used = true;
    base.placeholder_substituted_on_missing = false;
    hydration(base)
}

/// Degraded hydration entry: the canonical / accessible / audit resolution-form coverage of the hydration is
/// incomplete.
fn hydration_form_incomplete() -> M5ResolvedSessionHydrationEntry {
    let mut base = clean_hydration_base(
        "hydration:debugger:form-incomplete",
        "pane.debugger.secondary",
        "hydration.debugger.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::DebuggerOrNotebookHydration,
        M5RestoreOrchestrationSurfaceContext::RecoverySurface,
        "pane-role.debugger.secondary",
        "dependency.present",
        "restore-fidelity.live",
    );
    base.resolution_form_coverage = vec![M5RestoreOrchestrationResolutionForm::CanonicalObject];
    hydration(base)
}

/// Degraded hydration entry: the session-hydration surface is unclassified.
fn hydration_surface_unclassified() -> M5ResolvedSessionHydrationEntry {
    hydration(clean_hydration_base(
        "hydration:admin:surface-unclassified",
        "pane.debugger.third",
        "hydration.unknown.no_rerun",
        M5WindowRestoreRole::SessionHydration,
        M5SessionHydrationSurface::HydrationSurfaceUnclassified,
        M5RestoreOrchestrationSurfaceContext::AdminSurface,
        "pane-role.debugger.third",
        "dependency.present",
        "restore-fidelity.live",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    skeleton_restore_entries: Vec<M5ResolvedSkeletonRestoreEntry>,
    session_hydration_entries: Vec<M5ResolvedSessionHydrationEntry>,
) -> M5SkeletonFirstRestoreSessionHydrationRegistriesRow {
    M5SkeletonFirstRestoreSessionHydrationRegistriesRow {
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
        anatomy_parts: M5RestoreOrchestrationAnatomyPart::ALL.to_vec(),
        export_fields: M5RestoreOrchestrationExportField::ALL.to_vec(),
        downgrade_triggers,
        skeleton_restore_entries,
        session_hydration_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        ]),
        reruns_session_scoped_work_or_reattaches_privileged_sessions_during_restore: false,
        deletes_layout_structure_silently_on_missing_dependency: false,
        merges_skeleton_and_hydration_into_one_opaque_blob: false,
        overclaims_restore_fidelity_when_only_context_or_evidence_reopened: false,
    }
}

fn registry_rows() -> Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesRow> {
    use M5WindowRestoreConsumerSurface as C;
    use M5WindowRestoreDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell rebuilds the per-restore layout skeleton to one stable object — window shell, stable pane-tree structure, preserved pane roles, placeholder set, layout-skeleton root, and the distinct deferred-hydration plan — from the shared registry before any heavy dependency hydrates, and hydrates the terminal session without rerunning it; a skeleton object missing its pane-tree structure and a hydration that collapses the pane on a missing dependency degrade honestly instead of reading as a clean pass",
            "evidence:m5-restore-orchestration-shell-ui:001",
            vec![
                D::RestoreFidelityClassUnstated,
                D::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore,
                D::ProofStale,
            ],
            vec![skeleton_shell_live_clean(), skeleton_object_incomplete()],
            vec![hydration_terminal_shell_clean(), hydration_collapses()],
        ),
        base_row(
            C::RestoreCoordinator,
            "Restore-coordinator owner",
            "The restore coordinator rebuilds a pane-role-preserving placeholder skeleton first and defers heavy hydration, and substitutes a placeholder for a missing debugger dependency rather than collapsing the layout; a resolution-form gap on a skeleton entry and on a hydration entry is caught before a screenshot can reintroduce a false-truth reading",
            "evidence:m5-restore-orchestration-restore-coordinator:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget,
                D::ProofStale,
            ],
            vec![
                skeleton_recovery_placeholder_clean(),
                skeleton_form_incomplete(),
            ],
            vec![
                hydration_debugger_recovery_clean(),
                hydration_form_incomplete(),
            ],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the context-only skeleton and the preview hydration that discloses its restore fidelity rather than overclaiming live, without manual reconstruction; a skeleton whose heavy hydration ran before the layout skeleton was rebuilt is caught as a hydration-first restore",
            "evidence:m5-restore-orchestration-diagnostics:001",
            vec![
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::SessionHydrationRuleUnstated,
                D::ProofStale,
            ],
            vec![
                skeleton_diagnostics_context_only_clean(),
                skeleton_hydration_preceded(),
            ],
            vec![hydration_preview_diagnostics_clean()],
        ),
        base_row(
            C::WorkspaceService,
            "Workspace-service owner",
            "The workspace service rebuilds the evidence-only skeleton object while keeping it bound to the registry; a skeleton that is a hand-copied per-pane restore assumption and a hydration on an unclassified surface degrade honestly",
            "evidence:m5-restore-orchestration-workspace-service:001",
            vec![
                D::SessionHydrationRuleUnstated,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![skeleton_admin_evidence_only_clean(), skeleton_unbound()],
            vec![
                hydration_debugger_admin_clean(),
                hydration_surface_unclassified(),
            ],
        ),
        base_row(
            C::SessionService,
            "Session-service owner",
            "The session service renders the same resolved skeleton-restore and session-hydration truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied restore table",
            "evidence:m5-restore-orchestration-session-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::SessionHydrationRuleUnstated,
                D::ProofStale,
            ],
            vec![
                skeleton_diagnostics_context_only_clean(),
                skeleton_form_incomplete(),
            ],
            vec![
                hydration_debugger_recovery_clean(),
                hydration_form_incomplete(),
            ],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved skeleton-restore and session-hydration truth, so a hand-copied constant, an unstated registry token, a hydration-first restore, or a collapsed layout is visible in evidence rather than hidden behind a screenshot, and it explains which panes restored live, as placeholders, context-only, or evidence-only",
            "evidence:m5-restore-orchestration-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::ProofStale,
            ],
            vec![skeleton_support_live_clean(), skeleton_token_unstated()],
            vec![hydration_terminal_support_clean()],
        ),
    ]
}

fn governance_review() -> M5SkeletonFirstRestoreSessionHydrationRegistriesGovernanceReview {
    M5SkeletonFirstRestoreSessionHydrationRegistriesGovernanceReview {
        skeleton_registry_names_token_role_and_fidelity_class: true,
        restore_resolves_to_stable_skeleton_object_from_shared_registry: true,
        window_shell_pane_tree_roles_and_placeholders_published: true,
        skeleton_rebuilt_before_heavy_hydration: true,
        session_hydration_keeps_pane_roles_and_never_reruns: true,
        missing_dependency_never_collapses_layout_silently: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shell_recovery_diagnostics_admin_read_single_source: true,
        skeleton_or_hydration_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerProjection {
    M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerProjection {
        shell_and_recovery_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        session_and_workspace_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5SkeletonFirstRestoreSessionHydrationRegistriesProofFreshness {
    M5SkeletonFirstRestoreSessionHydrationRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5SkeletonFirstRestoreSessionHydrationRegistriesReleasePosture {
    M5SkeletonFirstRestoreSessionHydrationRegistriesReleasePosture {
        proof_packet_ref: M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        window_restore_audit_ref: M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_REPORT_REF
            .to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_REF,
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 skeleton-first-restore and session-hydration registries packet.
pub fn seeded_m5_skeleton_first_restore_and_session_hydration_registries(
) -> M5SkeletonFirstRestoreSessionHydrationRegistriesPacket {
    M5SkeletonFirstRestoreSessionHydrationRegistriesPacket::new(
        M5SkeletonFirstRestoreSessionHydrationRegistriesPacketInput {
            packet_id: M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_PACKET_ID.to_owned(),
            registries_label:
                "M5 skeleton-first-restore and session-hydration registries with one stable restore-skeleton object rebuilt per restore, the layout skeleton rebuilt before any heavy dependency hydrates, preserved pane roles and placeholder set kept distinct from the deferred-hydration plan, canonical / accessible / audit resolution-form coverage, and the preserved-pane-role / missing-dependency-class / restore-fidelity-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5SkeletonFirstRestoreSessionHydrationRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the restore-coordinator row is held at Beta pending pane-role-preserving placeholder parity
/// on every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed(
) -> M5SkeletonFirstRestoreSessionHydrationRegistriesPacket {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.packet_id =
        "m5-skeleton-first-restore-and-session-hydration-registries:placeholder-pane-continuity-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .expect("restore-coordinator row present");
    row.qualification = M5WindowRestoreQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending context-only hydration parity on every
/// platform; every row stays visible and every example stays honest.
pub fn seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed(
) -> M5SkeletonFirstRestoreSessionHydrationRegistriesPacket {
    let mut packet = seeded_m5_skeleton_first_restore_and_session_hydration_registries();
    packet.packet_id =
        "m5-skeleton-first-restore-and-session-hydration-registries:context-only-hydration-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5WindowRestoreQualificationClass::Preview;
    packet
}
