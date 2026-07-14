//! Canonical seed builders for the M5 display-topology-recovery bounds-recovery and role-continuity registries
//! packet.
//!
//! These builders are the single producer of the checked-in support export and the narrowed fixtures.
//! The headless emitter and the inline tests both call them so the in-code registries, the artifact, and the
//! fixtures never drift. Every resolved example is built by calling the real resolvers so the packet can only
//! carry projections the resolvers actually produce. Clean bounds-recovery and role-continuity entries are built
//! so the one stable bounds-recovery object resolved per window / dialog / sheet, the bounds resolved onto
//! visible bounds before the surface is presented, the monitor-affinity hint and layout intent kept distinct
//! from the keyboard-reach plan, the canonical / accessible / audit resolution forms, and the preserved-role-label
//! / boundary-label / provenance-hint disclosure triple are proven across the shell, recovery, diagnostics,
//! admin, workspace, session, and support surfaces without any hand-copied per-surface assumption, off-screen
//! present, incomplete object, role reset, or resolution-form gap.

use super::*;

/// Stable packet id for the canonical registries packet.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_PACKET_ID: &str =
    "m5-display-topology-recovery-and-role-continuity-registries:stable:0001";

/// Mint / proof-refresh timestamp pinned by the seed builders.
const SEED_TIMESTAMP: &str = "2026-07-14T00:00:00Z";

/// Redaction class token carried by the packet.
const REDACTION_CLASS_TOKEN: &str = "metadata_only_export_safe";

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

fn bounds(input: M5BoundsRecoveryEntryResolutionInput) -> M5ResolvedBoundsRecoveryEntry {
    resolve_bounds_recovery_entry(input).expect("seed bounds-recovery entry resolves")
}

fn fence(input: M5RoleContinuityFenceEntryResolutionInput) -> M5ResolvedRoleContinuityFenceEntry {
    resolve_role_continuity_fence_entry(input).expect("seed role-continuity-fence entry resolves")
}

fn all_forms() -> Vec<M5DisplayTopologyOrchestrationResolutionForm> {
    M5DisplayTopologyOrchestrationResolutionForm::ALL.to_vec()
}

// -- Clean bounds-recovery entries (stable object, bounds-before-present, bound to the registry) -

#[allow(clippy::too_many_arguments)]
fn clean_bounds_base(
    entry_id: &str,
    remap_target_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    bounds_recovery_state: M5BoundsRecoveryState,
    surface_context: M5DisplayTopologyOrchestrationSurfaceContext,
    window_surface_id: &str,
    affinity_monitor_hint: &str,
    resolved_visible_bounds: &str,
    layout_intent: &str,
    provenance_class: &str,
    keyboard_reach_plan: &str,
) -> M5BoundsRecoveryEntryResolutionInput {
    M5BoundsRecoveryEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        remap_target_id: remap_target_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        bounds_recovery_state,
        surface_context,
        resolution_form_coverage: all_forms(),
        window_surface_id: window_surface_id.to_owned(),
        affinity_monitor_hint: affinity_monitor_hint.to_owned(),
        resolved_visible_bounds: resolved_visible_bounds.to_owned(),
        layout_intent: layout_intent.to_owned(),
        provenance_class: provenance_class.to_owned(),
        keyboard_reach_plan: keyboard_reach_plan.to_owned(),
        bound_to_registry: true,
        bounds_resolved_before_present: true,
        is_material_topology_adjustment: false,
        topology_adjustment_recorded_when_material: true,
        proof_fresh: true,
    }
}

fn bounds_shell_affinity_clean() -> M5ResolvedBoundsRecoveryEntry {
    // A window returned to its remembered monitor via the preserved affinity hint — no clamp, no fidelity change.
    bounds(clean_bounds_base(
        "bounds:shell:affinity",
        "window.acme.editor-main",
        "bounds.recovery.affinity_monitor_restored",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::AffinityMonitorRestored,
        M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        "window-surface.editor.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-1440p",
        "layout-intent.split-editor",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    ))
}

fn bounds_recovery_clamped_clean() -> M5ResolvedBoundsRecoveryEntry {
    // The affinity monitor is gone, so the window is clamped back onto visible bounds — a material adjustment.
    let mut base = clean_bounds_base(
        "bounds:recovery:clamped",
        "window.acme.detached-preview",
        "bounds.recovery.clamped_onto_visible_bounds",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::ClampedOntoVisibleBounds,
        M5DisplayTopologyOrchestrationSurfaceContext::RecoverySurface,
        "window-surface.preview.detached",
        "affinity.monitor.secondary-detached",
        "bounds.visible.primary-clamped",
        "layout-intent.detached-preview",
        "provenance.reduced-fidelity",
        "keyboard-reach.focus-cycle",
    );
    base.is_material_topology_adjustment = true;
    base.topology_adjustment_recorded_when_material = true;
    bounds(base)
}

fn bounds_diagnostics_rescaled_clean() -> M5ResolvedBoundsRecoveryEntry {
    // A DPI change rescaled the geometry so the surface stays legible and on-screen — a material adjustment.
    let mut base = clean_bounds_base(
        "bounds:diagnostics:rescaled",
        "window.acme.dialog-confirm",
        "bounds.recovery.rescaled_for_dpi_change",
        M5WindowRestoreRole::RestoreFidelity,
        M5BoundsRecoveryState::RescaledForDpiChange,
        M5DisplayTopologyOrchestrationSurfaceContext::DiagnosticsSurface,
        "window-surface.dialog.confirm",
        "affinity.monitor.docked-hidpi",
        "bounds.visible.docked-rescaled",
        "layout-intent.blocking-dialog",
        "provenance.reduced-fidelity",
        "keyboard-reach.dialog-trap",
    );
    base.is_material_topology_adjustment = true;
    base.topology_adjustment_recorded_when_material = true;
    bounds(base)
}

fn bounds_admin_relocated_clean() -> M5ResolvedBoundsRecoveryEntry {
    // The affinity monitor is unavailable, so the surface relocates to the primary fallback — a material change.
    let mut base = clean_bounds_base(
        "bounds:admin:relocated",
        "window.acme.aux-inspector",
        "bounds.recovery.relocated_to_primary_fallback",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::RelocatedToPrimaryFallback,
        M5DisplayTopologyOrchestrationSurfaceContext::AdminSurface,
        "window-surface.aux.inspector",
        "affinity.monitor.unplugged",
        "bounds.visible.primary-fallback",
        "layout-intent.auxiliary-inspector",
        "provenance.reduced-fidelity",
        "keyboard-reach.focus-cycle",
    );
    base.is_material_topology_adjustment = true;
    base.topology_adjustment_recorded_when_material = true;
    bounds(base)
}

fn bounds_support_fullscreen_clean() -> M5ResolvedBoundsRecoveryEntry {
    // A fullscreen surface restored to its remembered windowed bounds on a visible monitor — no fidelity change.
    bounds(clean_bounds_base(
        "bounds:support:fullscreen",
        "window.acme.presentation-main",
        "bounds.recovery.restored_fullscreen_to_windowed",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::RestoredFullscreenToWindowed,
        M5DisplayTopologyOrchestrationSurfaceContext::SupportOrExportForm,
        "window-surface.presentation.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-windowed",
        "layout-intent.presentation",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    ))
}

// -- Degraded bounds-recovery entries -----------------------------------------------------------

/// Degraded bounds entry: the behavior is a hand-copied per-surface remap assumption instead of tracing to the
/// registry.
fn bounds_unbound() -> M5ResolvedBoundsRecoveryEntry {
    let mut base = clean_bounds_base(
        "bounds:workspace:unbound",
        "window.acme.aux-inspector",
        "bounds.recovery.relocated_to_primary_fallback",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::RelocatedToPrimaryFallback,
        M5DisplayTopologyOrchestrationSurfaceContext::AdminSurface,
        "window-surface.aux.inspector",
        "affinity.monitor.unplugged",
        "bounds.visible.primary-fallback",
        "layout-intent.auxiliary-inspector",
        "provenance.reduced-fidelity",
        "keyboard-reach.focus-cycle",
    );
    base.is_material_topology_adjustment = true;
    base.bound_to_registry = false;
    bounds(base)
}

/// Degraded bounds entry: the resolved bounds object is incomplete — the layout intent is unstated.
fn bounds_object_incomplete() -> M5ResolvedBoundsRecoveryEntry {
    let mut base = clean_bounds_base(
        "bounds:shell:incomplete",
        "window.acme.editor-main",
        "bounds.recovery.affinity_monitor_restored",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::AffinityMonitorRestored,
        M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        "window-surface.editor.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-1440p",
        "layout-intent.split-editor",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    );
    base.layout_intent = "   ".to_owned();
    bounds(base)
}

/// Degraded bounds entry: the window was presented before its bounds were resolved onto visible bounds.
fn bounds_present_preceded() -> M5ResolvedBoundsRecoveryEntry {
    let mut base = clean_bounds_base(
        "bounds:diagnostics:present-first",
        "window.acme.dialog-confirm",
        "bounds.recovery.rescaled_for_dpi_change",
        M5WindowRestoreRole::RestoreFidelity,
        M5BoundsRecoveryState::RescaledForDpiChange,
        M5DisplayTopologyOrchestrationSurfaceContext::DiagnosticsSurface,
        "window-surface.dialog.confirm",
        "affinity.monitor.docked-hidpi",
        "bounds.visible.docked-rescaled",
        "layout-intent.blocking-dialog",
        "provenance.reduced-fidelity",
        "keyboard-reach.dialog-trap",
    );
    base.is_material_topology_adjustment = true;
    base.topology_adjustment_recorded_when_material = true;
    base.bounds_resolved_before_present = false;
    bounds(base)
}

/// Degraded bounds entry: the canonical / accessible / audit resolution-form coverage is incomplete.
fn bounds_form_incomplete() -> M5ResolvedBoundsRecoveryEntry {
    let mut base = clean_bounds_base(
        "bounds:recovery:form-incomplete",
        "window.acme.detached-preview",
        "bounds.recovery.clamped_onto_visible_bounds",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::ClampedOntoVisibleBounds,
        M5DisplayTopologyOrchestrationSurfaceContext::RecoverySurface,
        "window-surface.preview.detached",
        "affinity.monitor.secondary-detached",
        "bounds.visible.primary-clamped",
        "layout-intent.detached-preview",
        "provenance.reduced-fidelity",
        "keyboard-reach.focus-cycle",
    );
    base.is_material_topology_adjustment = true;
    base.topology_adjustment_recorded_when_material = true;
    base.resolution_form_coverage =
        vec![M5DisplayTopologyOrchestrationResolutionForm::CanonicalObject];
    bounds(base)
}

/// Degraded bounds entry: the canonical registry token name is unstated.
fn bounds_token_unstated() -> M5ResolvedBoundsRecoveryEntry {
    let mut base = clean_bounds_base(
        "bounds:support:token-unstated",
        "window.acme.presentation-main",
        "  ",
        M5WindowRestoreRole::DisplayAffinity,
        M5BoundsRecoveryState::RestoredFullscreenToWindowed,
        M5DisplayTopologyOrchestrationSurfaceContext::SupportOrExportForm,
        "window-surface.presentation.main",
        "affinity.monitor.primary",
        "bounds.visible.primary-windowed",
        "layout-intent.presentation",
        "provenance.live-layout",
        "keyboard-reach.focus-cycle",
    );
    base.token_name = "  ".to_owned();
    bounds(base)
}

// -- Clean role-continuity-fence entries --------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn clean_fence_base(
    entry_id: &str,
    guarded_window_id: &str,
    token_name: &str,
    semantic_role: M5WindowRestoreRole,
    role_class: M5RoleContinuityClass,
    surface_context: M5DisplayTopologyOrchestrationSurfaceContext,
    preserved_role_label: &str,
    boundary_label: &str,
    provenance_hint: &str,
) -> M5RoleContinuityFenceEntryResolutionInput {
    M5RoleContinuityFenceEntryResolutionInput {
        entry_id: entry_id.to_owned(),
        guarded_window_id: guarded_window_id.to_owned(),
        token_name: token_name.to_owned(),
        semantic_role,
        role_class,
        surface_context,
        resolution_form_coverage: all_forms(),
        preserved_role_label: preserved_role_label.to_owned(),
        boundary_label: boundary_label.to_owned(),
        provenance_hint: provenance_hint.to_owned(),
        preserves_role_and_boundary: true,
        fence_is_truthful: true,
        role_was_present_used: false,
        role_preserved_after_remap: false,
        fidelity_reduced: false,
        fidelity_reduction_disclosed: false,
        proof_fresh: true,
    }
}

fn fence_follow_presentation_clean() -> M5ResolvedRoleContinuityFenceEntry {
    fence(clean_fence_base(
        "fence:presentation:follow",
        "window.presentation.main",
        "fence.follow.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::FollowOrPresentationState,
        M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        "role-label.presenter-following",
        "boundary.presentation-window",
        "provenance.live-layout",
    ))
}

fn fence_collab_badge_clean() -> M5ResolvedRoleContinuityFenceEntry {
    // A collaboration role badge that was present before the remap is preserved after the remap.
    let mut base = clean_fence_base(
        "fence:collab:badge",
        "window.collab.secondary",
        "fence.collab.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::CollaborationRoleBadge,
        M5DisplayTopologyOrchestrationSurfaceContext::RecoverySurface,
        "role-label.observer-badge",
        "boundary.collaboration-window",
        "provenance.reduced-fidelity",
    );
    base.role_was_present_used = true;
    base.role_preserved_after_remap = true;
    fence(base)
}

fn fence_auxiliary_purpose_clean() -> M5ResolvedRoleContinuityFenceEntry {
    // An auxiliary-window purpose is preserved and its reduced layout fidelity is disclosed rather than hidden.
    let mut base = clean_fence_base(
        "fence:auxiliary:purpose",
        "window.aux.inspector",
        "fence.auxiliary.no_reset",
        M5WindowRestoreRole::RestoreFidelity,
        M5RoleContinuityClass::AuxiliaryWindowPurpose,
        M5DisplayTopologyOrchestrationSurfaceContext::DiagnosticsSurface,
        "role-label.auxiliary-inspector",
        "boundary.auxiliary-window",
        "provenance.reduced-fidelity",
    );
    base.role_was_present_used = true;
    base.role_preserved_after_remap = true;
    base.fidelity_reduced = true;
    base.fidelity_reduction_disclosed = true;
    fence(base)
}

fn fence_collab_badge_admin_clean() -> M5ResolvedRoleContinuityFenceEntry {
    fence(clean_fence_base(
        "fence:collab:admin",
        "window.collab.third",
        "fence.collab.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::CollaborationRoleBadge,
        M5DisplayTopologyOrchestrationSurfaceContext::AdminSurface,
        "role-label.presenter-badge",
        "boundary.collaboration-window",
        "provenance.live-layout",
    ))
}

fn fence_follow_support_clean() -> M5ResolvedRoleContinuityFenceEntry {
    fence(clean_fence_base(
        "fence:presentation:support",
        "window.presentation.main",
        "fence.follow.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::FollowOrPresentationState,
        M5DisplayTopologyOrchestrationSurfaceContext::SupportOrExportForm,
        "role-label.presenter-following",
        "boundary.presentation-window",
        "provenance.live-layout",
    ))
}

// -- Degraded role-continuity-fence entries -----------------------------------------------------

/// Degraded fence entry: a role that was present before the remap was reset into a generic window instead of
/// being preserved — the window reads as role-correct when its follow / presentation context never survived the
/// remap.
fn fence_resets() -> M5ResolvedRoleContinuityFenceEntry {
    let mut base = clean_fence_base(
        "fence:presentation:resets",
        "window.presentation.main",
        "fence.follow.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::FollowOrPresentationState,
        M5DisplayTopologyOrchestrationSurfaceContext::ShellSurface,
        "role-label.presenter-following",
        "boundary.presentation-window",
        "provenance.reduced-fidelity",
    );
    base.role_was_present_used = true;
    base.role_preserved_after_remap = false;
    fence(base)
}

/// Degraded fence entry: the canonical / accessible / audit resolution-form coverage of the fence is incomplete.
fn fence_form_incomplete() -> M5ResolvedRoleContinuityFenceEntry {
    let mut base = clean_fence_base(
        "fence:collab:form-incomplete",
        "window.collab.secondary",
        "fence.collab.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::CollaborationRoleBadge,
        M5DisplayTopologyOrchestrationSurfaceContext::RecoverySurface,
        "role-label.observer-badge",
        "boundary.collaboration-window",
        "provenance.live-layout",
    );
    base.resolution_form_coverage =
        vec![M5DisplayTopologyOrchestrationResolutionForm::CanonicalObject];
    fence(base)
}

/// Degraded fence entry: the role-continuity class is unclassified.
fn fence_class_unclassified() -> M5ResolvedRoleContinuityFenceEntry {
    fence(clean_fence_base(
        "fence:admin:class-unclassified",
        "window.collab.third",
        "fence.unknown.no_reset",
        M5WindowRestoreRole::DisplayAffinity,
        M5RoleContinuityClass::RoleClassUnclassified,
        M5DisplayTopologyOrchestrationSurfaceContext::AdminSurface,
        "role-label.presenter-badge",
        "boundary.collaboration-window",
        "provenance.live-layout",
    ))
}

// -- Row builders -------------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn base_row(
    consumer_surface: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerSurface,
    owner_role: &str,
    scope_summary: &str,
    proof_ref: &str,
    downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    bounds_recovery_entries: Vec<M5ResolvedBoundsRecoveryEntry>,
    role_continuity_fence_entries: Vec<M5ResolvedRoleContinuityFenceEntry>,
) -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow {
        consumer_surface,
        qualification: M5WindowRestoreQualificationClass::Stable,
        owner_role: owner_role.to_owned(),
        scope_summary: scope_summary.to_owned(),
        deployment_lines: M5WindowRestoreDeploymentLine::ALL.to_vec(),
        required_labels: vec![
            M5WindowRestoreRequiredLabel::Identity,
            M5WindowRestoreRequiredLabel::SemanticRole,
            M5WindowRestoreRequiredLabel::RegistryReference,
            M5WindowRestoreRequiredLabel::DisplayAffinity,
            M5WindowRestoreRequiredLabel::RestoreFidelityClass,
        ],
        accessibility_routes: M5WindowRestoreAccessibilityRoute::ALL.to_vec(),
        anatomy_parts: M5DisplayTopologyOrchestrationAnatomyPart::ALL.to_vec(),
        export_fields: M5DisplayTopologyOrchestrationExportField::ALL.to_vec(),
        downgrade_triggers,
        bounds_recovery_entries,
        role_continuity_fence_entries,
        required_proof_packet_refs: strings(&[proof_ref]),
        source_contract_refs: strings(&[
            M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_REF,
            M5_RESTORE_FIDELITY_SCHEMA_REF,
            M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        ]),
        strands_window_or_dialog_offscreen_after_remap: false,
        resets_auxiliary_window_into_generic_window: false,
        merges_bounds_recovery_and_role_continuity_into_one_opaque_blob: false,
        overclaims_layout_fidelity_when_only_bounds_or_context_recovered: false,
    }
}

fn registry_rows() -> Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow> {
    use M5WindowRestoreConsumerSurface as C;
    use M5WindowRestoreDowngradeTrigger as D;

    vec![
        base_row(
            C::ShellUi,
            "Shell surface owner",
            "The shell resolves each window, dialog, or sheet to one stable bounds-recovery object — window surface, monitor-affinity hint, resolved visible bounds, layout intent, provenance class, and the distinct keyboard-reach plan — from the shared registry, returns a window to its remembered monitor via the preserved affinity hint, and fences a follow / presentation state so it is never reset into a generic window; a bounds object missing its layout intent and a fence that resets a present role degrade honestly instead of reading as a clean pass",
            "evidence:m5-display-topology-orchestration-shell-ui:001",
            vec![
                D::DisplayAffinityUnstated,
                D::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
                D::ProofStale,
            ],
            vec![bounds_shell_affinity_clean(), bounds_object_incomplete()],
            vec![fence_follow_presentation_clean(), fence_resets()],
        ),
        base_row(
            C::RestoreCoordinator,
            "Restore-coordinator owner",
            "The restore coordinator resolves a clamped-onto-visible-bounds recovery that records the material topology adjustment in provenance, and fences a collaboration role badge that was present before the remap so it stays visible rather than resetting to generic; a resolution-form gap on a bounds entry and on a fence entry is caught before a screenshot can hide a reduced fidelity",
            "evidence:m5-display-topology-orchestration-restore-coordinator:001",
            vec![
                D::RegistryReferenceUnstated,
                D::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
                D::ProofStale,
            ],
            vec![bounds_recovery_clamped_clean(), bounds_form_incomplete()],
            vec![fence_collab_badge_clean(), fence_form_incomplete()],
        ),
        base_row(
            C::Diagnostics,
            "Diagnostics surface owner",
            "Diagnostics reports the DPI-rescaled bounds recovery and the auxiliary-window purpose fence that discloses its reduced layout fidelity rather than hiding it, without manual reconstruction; a dialog that was presented before its bounds were resolved onto visible bounds is caught as an off-screen present",
            "evidence:m5-display-topology-orchestration-diagnostics:001",
            vec![
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap,
                D::ProofStale,
            ],
            vec![
                bounds_diagnostics_rescaled_clean(),
                bounds_present_preceded(),
            ],
            vec![fence_auxiliary_purpose_clean()],
        ),
        base_row(
            C::WorkspaceService,
            "Workspace-service owner",
            "The workspace service resolves the relocated-to-primary-fallback recovery while keeping it bound to the registry, and fences the collaboration authority window; a bounds recovery that is a hand-copied per-surface remap assumption and a fence on an unclassified role class degrade honestly",
            "evidence:m5-display-topology-orchestration-workspace-service:001",
            vec![
                D::WindowTopologyBoundaryDriftedBySurface,
                D::RegistryReferenceUnstated,
                D::ProofStale,
            ],
            vec![bounds_admin_relocated_clean(), bounds_unbound()],
            vec![fence_collab_badge_admin_clean(), fence_class_unclassified()],
        ),
        base_row(
            C::SessionService,
            "Session-service owner",
            "The session service renders the same resolved bounds-recovery and role-continuity truth the resolvers produced across the canonical, accessible, and audit resolution forms rather than a hand-copied bounds table",
            "evidence:m5-display-topology-orchestration-session-service:001",
            vec![
                D::RegistryReferenceUnstated,
                D::DisplayAffinityUnstated,
                D::ProofStale,
            ],
            vec![
                bounds_diagnostics_rescaled_clean(),
                bounds_form_incomplete(),
            ],
            vec![fence_collab_badge_clean(), fence_form_incomplete()],
        ),
        base_row(
            C::SupportExport,
            "Support/export owner",
            "The support export carries the same resolved bounds-recovery and role-continuity truth, so a hand-copied constant, an unstated registry token, an off-screen present, or a role reset is visible in evidence rather than hidden behind a screenshot, and it distinguishes a full-fidelity restore from a bounds-only or reduced-fidelity remap",
            "evidence:m5-display-topology-orchestration-support-export:001",
            vec![
                D::RegistryReferenceUnstated,
                D::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened,
                D::ProofStale,
            ],
            vec![bounds_support_fullscreen_clean(), bounds_token_unstated()],
            vec![fence_follow_support_clean()],
        ),
    ]
}

fn governance_review() -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesGovernanceReview {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesGovernanceReview {
        bounds_registry_names_token_role_and_bounds_state: true,
        remap_resolves_to_stable_bounds_object_from_shared_registry: true,
        window_surface_bounds_affinity_and_provenance_published: true,
        bounds_resolved_before_surface_presented: true,
        role_fence_keeps_role_visible_and_never_resets_to_generic: true,
        fidelity_reduction_never_hidden_when_material: true,
        every_entry_covers_all_resolution_forms: true,
        behavior_bound_to_registry_not_hand_copied: true,
        shell_recovery_diagnostics_admin_read_single_source: true,
        bounds_or_fence_drift_caught_before_release: true,
        every_row_declares_mandatory_anatomy: true,
        reuses_frozen_matrix_vocabulary: true,
    }
}

fn consumer_projection() -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerProjection {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerProjection {
        shell_and_recovery_consume_shared_registries: true,
        diagnostics_and_admin_consume_shared_registries: true,
        session_and_workspace_services_consume_shared_registries: true,
        docs_help_and_cli_consume_shared_registries: true,
        behavior_traces_to_domain_contracts: true,
        support_export_reads_single_registry_source: true,
    }
}

fn proof_freshness() -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesProofFreshness {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesProofFreshness {
        proof_freshness_slo_hours: 720,
        last_proof_refresh: SEED_TIMESTAMP.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn release_posture() -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesReleasePosture {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesReleasePosture {
        proof_packet_ref: M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_ARTIFACT_REF
            .to_owned(),
        window_restore_audit_ref:
            M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_REPORT_REF.to_owned(),
        support_export_parity_required: true,
        accessibility_parity_required: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    strings(&[
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_REF,
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ])
}

/// Builds the canonical M5 display-topology-recovery bounds-recovery and role-continuity registries packet.
pub fn seeded_m5_display_topology_recovery_and_role_continuity_registries(
) -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket {
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket::new(
        M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacketInput {
            packet_id: M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_PACKET_ID
                .to_owned(),
            registries_label:
                "M5 display-topology-recovery bounds-recovery and role-continuity registries with one stable bounds-recovery object resolved per window / dialog / sheet, the bounds resolved onto visible bounds before the surface is presented, the monitor-affinity hint and layout intent kept distinct from the keyboard-reach plan, canonical / accessible / audit resolution-form coverage, and the preserved-role-label / boundary-label / provenance-hint disclosure triple across shell, recovery, diagnostics, admin, workspace, session, and support surfaces"
                    .to_owned(),
            registry_rows: registry_rows(),
            vocabulary_set:
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesVocabularySet::canonical(),
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

/// Narrowed variant: the restore-coordinator row is held at Beta pending DPI-rescale parity on every platform;
/// every row stays visible and every example stays honest.
pub fn seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed(
) -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.packet_id =
        "m5-display-topology-recovery-and-role-continuity-registries:dpi-rescale-beta:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::RestoreCoordinator)
        .expect("restore-coordinator row present");
    row.qualification = M5WindowRestoreQualificationClass::Beta;
    packet
}

/// Narrowed variant: the diagnostics row is narrowed to Preview pending reduced-fidelity disclosure parity on
/// every platform; every row stays visible and every example stays honest.
pub fn seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed(
) -> M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket {
    let mut packet = seeded_m5_display_topology_recovery_and_role_continuity_registries();
    packet.packet_id =
        "m5-display-topology-recovery-and-role-continuity-registries:reduced-fidelity-preview:0001"
            .to_owned();
    let row = packet
        .registry_rows
        .iter_mut()
        .find(|row| row.consumer_surface == M5WindowRestoreConsumerSurface::Diagnostics)
        .expect("diagnostics row present");
    row.qualification = M5WindowRestoreQualificationClass::Preview;
    packet
}
