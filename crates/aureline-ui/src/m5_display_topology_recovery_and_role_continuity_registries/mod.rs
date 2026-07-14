//! Implemented M5 display-topology-recovery bounds-recovery and role-continuity registries.
//!
//! The frozen [window-restore matrix][matrix] names Aureline's five workspace-restore families and locks their
//! controlled vocabulary. This module is the runtime implement lane for the
//! [display-topology recovery][family] family — the multi-monitor geometry recovery, auxiliary-window
//! continuity, and presentation / follow state preservation that keep restored and live windows visible,
//! attributable, and role-correct when the desktop topology changes. It turns the *on-screen bounds-recovery*
//! grammar and the *role-continuity-fence* grammar into registry resolvers that produce export-safe, honest
//! projections. Every claimed M5 restore then resolves each window, dialog, or sheet to one explicit
//! bounds-recovery posture — affinity monitor restored, clamped onto visible bounds, rescaled for a DPI change,
//! relocated to a primary fallback, or a fullscreen surface restored to windowed bounds — that preserves the
//! monitor-affinity hint and layout intent while clamping the surface back onto visible bounds after display
//! detach, dock / undock, DPI change, or fullscreen / desktop moves, instead of stranding a window off-screen or
//! a blocking dialog beyond keyboard reach, and it fences off any reset of an auxiliary window into a generic
//! window, so follow / presentation state, collaboration role badges, and auxiliary-window purpose stay visible
//! after remap, material topology adjustments are recorded in restore provenance and diagnostics, and a remap
//! that only recovered bounds or context can never overclaim that layout fidelity was unchanged.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Resolve every window, dialog, or sheet to one explicit bounds-recovery object per topology change before
//!   it is presented.** [`resolve_bounds_recovery_entry`] refuses to read as a clean, registry-bound bounds
//!   entry unless it names a canonical registry token, a classified
//!   [bounds-recovery state][M5BoundsRecoveryState], a window-restore role, covers every
//!   [resolution form][M5DisplayTopologyOrchestrationResolutionForm] (the canonical object, the accessible
//!   summary, and the audit record), publishes every bounds field (window surface, monitor-affinity hint,
//!   resolved visible bounds, layout intent, provenance class, and the distinct keyboard-reach plan), resolves
//!   the bounds before the surface is presented, and records a material topology adjustment in provenance when
//!   one happened; otherwise it degrades.
//! * **Keep the surface from being presented before its bounds are resolved onto visible bounds.**
//!   [`bounds_precede_present`] rejects an entry whose window or dialog was presented before its bounds were
//!   resolved so it degrades to
//!   [`M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds`], and the
//!   `topology_adjustment_recorded_when_material` invariant degrades a material-adjustment entry that hid that
//!   layout fidelity changed.
//! * **Fence off resetting an auxiliary window into a generic window.** [`resolve_role_continuity_fence_entry`]
//!   names a classified [role-continuity class][M5RoleContinuityClass], requires the
//!   preserved-role-label / boundary-label / provenance-hint disclosure triple, covers every resolution form, and
//!   degrades to [`M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims`] when the fence resets a
//!   follow / presentation state, a collaboration role badge, or an auxiliary-window purpose into a generic
//!   window, drops a role that was present before the remap, or hides that layout fidelity was reduced, so a
//!   remapped window can never read as role-correct when its follow / presentation / collaboration context never
//!   actually survived the remap.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5WindowRestoreRole`] role vocabulary and
//! the [`M5WindowRestoreConsumerSurface`] consumer-surface taxonomy — so the shell, recovery, diagnostics,
//! admin, workspace, session, docs, CLI, and support surfaces can never fork their own display-topology-recovery
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_window_restore_matrix
//! [family]: crate::m5_window_restore_matrix::M5WindowRestoreFamily::DisplayTopologyRecovery

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_display_topology_recovery_and_role_continuity_registries,
    seeded_m5_display_topology_recovery_and_role_continuity_registries_dpi_rescale_beta_narrowed,
    seeded_m5_display_topology_recovery_and_role_continuity_registries_reduced_fidelity_preview_narrowed,
    M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket`].
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_display_topology_recovery_and_role_continuity_registries";

/// Schema version for M5 display-topology bounds-recovery / role-continuity registry records.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-display-topology-recovery-and-role-continuity-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_DOC_REF: &str =
    "docs/recovery/m5_display_topology_recovery_and_role_continuity_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-display-topology-recovery-and-role-continuity-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no lane
/// invents a parallel surface set.
pub type M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerSurface =
    M5WindowRestoreConsumerSurface;

/// One of the three resolution forms every bounds-recovery or role-continuity entry must hold across so its
/// truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// display-topology-recovery *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayTopologyOrchestrationResolutionForm {
    /// The canonical resolved bounds-recovery / role-continuity object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved recovery discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved recovery inspectable off-renderer.
    AuditRecord,
}

impl M5DisplayTopologyOrchestrationResolutionForm {
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

/// Controlled explicit bounds-recovery state a bounds-recovery entry resolves a window, dialog, or sheet to, so
/// the canonical display-topology-recovery model shares one registry rather than a hand-copied per-surface
/// remap assumption. Minted by this lane because the frozen matrix carries the workspace-restore families but
/// not the concrete affinity-restored / clamped / rescaled / relocated / fullscreen-restored bounds model a
/// recovery entry resolves against. Every classified state carries its canonical bounds-recovery mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundsRecoveryState {
    /// The surface returned to its remembered monitor via the preserved affinity hint; no clamp was needed.
    AffinityMonitorRestored,
    /// The affinity monitor is gone or smaller, so the window / dialog / sheet was clamped back onto visible
    /// bounds.
    ClampedOntoVisibleBounds,
    /// A DPI change rescaled the geometry so the surface stays legible and on-screen.
    RescaledForDpiChange,
    /// The affinity monitor is unavailable, so the surface was relocated to the primary / available monitor as a
    /// disclosed fallback.
    RelocatedToPrimaryFallback,
    /// A fullscreen or desktop-move surface was restored to its remembered windowed bounds on a visible monitor.
    RestoredFullscreenToWindowed,
    /// The bounds-recovery state is unclassified, which is disallowed.
    BoundsUnclassified,
}

impl M5BoundsRecoveryState {
    /// Every bounds-recovery state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AffinityMonitorRestored,
        Self::ClampedOntoVisibleBounds,
        Self::RescaledForDpiChange,
        Self::RelocatedToPrimaryFallback,
        Self::RestoredFullscreenToWindowed,
        Self::BoundsUnclassified,
    ];

    /// The five canonical bounds-recovery states every claimed M5 restore must resolve windows, dialogs, and
    /// sheets to after a display-topology change.
    pub const CANONICAL_STATES: [Self; 5] = [
        Self::AffinityMonitorRestored,
        Self::ClampedOntoVisibleBounds,
        Self::RescaledForDpiChange,
        Self::RelocatedToPrimaryFallback,
        Self::RestoredFullscreenToWindowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AffinityMonitorRestored => "affinity_monitor_restored",
            Self::ClampedOntoVisibleBounds => "clamped_onto_visible_bounds",
            Self::RescaledForDpiChange => "rescaled_for_dpi_change",
            Self::RelocatedToPrimaryFallback => "relocated_to_primary_fallback",
            Self::RestoredFullscreenToWindowed => "restored_fullscreen_to_windowed",
            Self::BoundsUnclassified => "bounds_unclassified",
        }
    }

    /// Whether the state is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::BoundsUnclassified)
    }

    /// The canonical bounds-recovery mode for this state.
    pub const fn canonical_bounds_recovery_mode(self) -> &'static str {
        match self {
            Self::AffinityMonitorRestored => "affinity_monitor_restored",
            Self::ClampedOntoVisibleBounds => "clamped_onto_visible_bounds",
            Self::RescaledForDpiChange => "rescaled_for_dpi_change",
            Self::RelocatedToPrimaryFallback => "relocated_to_primary_fallback",
            Self::RestoredFullscreenToWindowed => "restored_fullscreen_to_windowed",
            Self::BoundsUnclassified => "",
        }
    }

    /// Whether this state is a material topology adjustment — the affinity monitor was gone, smaller, or rescaled
    /// so the surface's layout fidelity changed and the adjustment must be recorded in provenance. Passive
    /// recoveries (the affinity monitor restored, a fullscreen surface restored to its remembered windowed
    /// bounds) never change fidelity.
    pub const fn is_material_topology_adjustment(self) -> bool {
        matches!(
            self,
            Self::ClampedOntoVisibleBounds
                | Self::RescaledForDpiChange
                | Self::RelocatedToPrimaryFallback
        )
    }
}

/// Controlled role-continuity class a role-continuity-fence entry must resolve its fence from, so a remapped
/// window's follow / presentation / collaboration continuity shares one registry rather than a hand-copied
/// per-surface reset path. Minted by this lane, tracking the role facets the acceptance criteria require be kept
/// visible after remap by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoleContinuityClass {
    /// The follow / presentation state fence (follow-mode and presentation state).
    FollowOrPresentationState,
    /// The collaboration role-badge fence (participant / presenter / observer role badges).
    CollaborationRoleBadge,
    /// The auxiliary-window purpose fence (secondary-window purpose and boundary labels).
    AuxiliaryWindowPurpose,
    /// The role-continuity class is unclassified, which is disallowed.
    RoleClassUnclassified,
}

impl M5RoleContinuityClass {
    /// Every role-continuity class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FollowOrPresentationState,
        Self::CollaborationRoleBadge,
        Self::AuxiliaryWindowPurpose,
        Self::RoleClassUnclassified,
    ];

    /// The three canonical role-continuity classes every restore must keep visible after remap.
    pub const CANONICAL_CLASSES: [Self; 3] = [
        Self::FollowOrPresentationState,
        Self::CollaborationRoleBadge,
        Self::AuxiliaryWindowPurpose,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowOrPresentationState => "follow_or_presentation_state",
            Self::CollaborationRoleBadge => "collaboration_role_badge",
            Self::AuxiliaryWindowPurpose => "auxiliary_window_purpose",
            Self::RoleClassUnclassified => "role_class_unclassified",
        }
    }

    /// Whether the role-continuity class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::RoleClassUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a bounds-recovery or
/// role-continuity token's meaning stays stable whether it appears in the shell, recovery, diagnostics, admin,
/// or a support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayTopologyOrchestrationSurfaceContext {
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

impl M5DisplayTopologyOrchestrationSurfaceContext {
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

/// One mandatory rendered part a bounds-recovery or role-continuity entry must be able to show, so no
/// bounds-recovery state, window surface, resolved bounds, affinity hint, provenance, role-continuity class,
/// keyboard-reach hint, or registry fact is left implicit behind a hand-copied per-surface remap assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayTopologyOrchestrationAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The bounds-recovery state the entry resolves (bounds-recovery entry).
    BoundsRecoveryState,
    /// The window surface and resolved visible bounds the entry recovers (bounds-recovery entry).
    WindowSurfaceAndBounds,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The monitor-affinity hint and provenance the entry publishes (bounds-recovery entry).
    AffinityHintAndProvenance,
    /// The role-continuity class the entry publishes (role-continuity entry).
    RoleContinuityClass,
    /// The distinct keyboard-reach plan kept separate from the recovered bounds (both entries).
    KeyboardReachPlanHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved recovery or fence (both entries).
    PlainLanguageMeaning,
}

impl M5DisplayTopologyOrchestrationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::BoundsRecoveryState,
        Self::WindowSurfaceAndBounds,
        Self::ResolutionFormCoverage,
        Self::AffinityHintAndProvenance,
        Self::RoleContinuityClass,
        Self::KeyboardReachPlanHint,
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
            Self::BoundsRecoveryState => "bounds_recovery_state",
            Self::WindowSurfaceAndBounds => "window_surface_and_bounds",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::AffinityHintAndProvenance => "affinity_hint_and_provenance",
            Self::RoleContinuityClass => "role_continuity_class",
            Self::KeyboardReachPlanHint => "keyboard_reach_plan_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// bounds-recovery posture, a role-continuity fence, or a degraded bounds-recovery / role-continuity entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisplayTopologyOrchestrationNextAction {
    /// Expand the resolved recovery's or fence's plain-language layout meaning.
    ExpandLayoutMeaning,
    /// Inspect the bounds-recovery state or role-continuity class the entry resolves.
    InspectBoundsOrRoleContinuity,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5DisplayTopologyOrchestrationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandLayoutMeaning,
        Self::InspectBoundsOrRoleContinuity,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandLayoutMeaning => "expand_layout_meaning",
            Self::InspectBoundsOrRoleContinuity => "inspect_bounds_or_role_continuity",
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
pub enum M5DisplayTopologyOrchestrationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The window-restore families covered.
    WindowRestoreFamilies,
    /// The bounds-recovery states carried.
    BoundsRecoveryStates,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The role-continuity classes carried.
    RoleContinuityClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The bounds-recovery modes carried.
    BoundsRecoveryModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DisplayTopologyOrchestrationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::BoundsRecoveryStates,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::RoleContinuityClasses,
        Self::SurfaceContext,
        Self::BoundsRecoveryModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::BoundsRecoveryStates,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::WindowRestoreFamilies => "window_restore_families",
            Self::BoundsRecoveryStates => "bounds_recovery_states",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::RoleContinuityClasses => "role_continuity_classes",
            Self::SurfaceContext => "surface_context",
            Self::BoundsRecoveryModes => "bounds_recovery_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a bounds-recovery entry degraded below a clean, registry-bound state. The degrade-first ladder returns
/// one of these instead of ever letting a hand-copied, present-first, field-incomplete, or form-incomplete entry
/// read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundsRecoveryEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the bounds recovery means.
    BoundsTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The bounds-recovery state is unclassified (not in the resolved taxonomy).
    BoundsRecoveryStateUnclassified,
    /// The behavior is a hand-copied per-surface remap assumption instead of tracing to the canonical registry.
    BoundsNotBoundToRegistry,
    /// The resolved bounds-recovery object is incomplete: window surface, monitor-affinity hint, resolved
    /// visible bounds, layout intent, provenance class, or the distinct keyboard-reach plan is unstated.
    BoundsRecoveryObjectIncomplete,
    /// The window or dialog was presented before its bounds were resolved onto visible bounds (an off-screen
    /// present instead of a clamp-first recovery).
    PresentPrecededBounds,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// A material topology adjustment happened but was not recorded in restore provenance.
    TopologyAdjustmentNotRecorded,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BoundsRecoveryEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::BoundsTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::BoundsRecoveryStateUnclassified,
        Self::BoundsNotBoundToRegistry,
        Self::BoundsRecoveryObjectIncomplete,
        Self::PresentPrecededBounds,
        Self::ResolutionFormCoverageIncomplete,
        Self::TopologyAdjustmentNotRecorded,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundsTokenUnstated => "bounds_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::BoundsRecoveryStateUnclassified => "bounds_recovery_state_unclassified",
            Self::BoundsNotBoundToRegistry => "bounds_not_bound_to_registry",
            Self::BoundsRecoveryObjectIncomplete => "bounds_recovery_object_incomplete",
            Self::PresentPrecededBounds => "present_preceded_bounds",
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::TopologyAdjustmentNotRecorded => "topology_adjustment_not_recorded",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DisplayTopologyOrchestrationNextAction {
        match self {
            Self::BoundsTokenUnstated | Self::BoundsNotBoundToRegistry => {
                M5DisplayTopologyOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::BoundsRecoveryStateUnclassified
            | Self::BoundsRecoveryObjectIncomplete
            | Self::PresentPrecededBounds => {
                M5DisplayTopologyOrchestrationNextAction::InspectBoundsOrRoleContinuity
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5DisplayTopologyOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::TopologyAdjustmentNotRecorded
            | Self::ProofStale => M5DisplayTopologyOrchestrationNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::BoundsTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::BoundsRecoveryStateUnclassified => {
                M5WindowRestoreDowngradeTrigger::DisplayAffinityUnstated
            }
            Self::BoundsNotBoundToRegistry => {
                M5WindowRestoreDowngradeTrigger::WindowTopologyBoundaryDriftedBySurface
            }
            Self::BoundsRecoveryObjectIncomplete | Self::PresentPrecededBounds => {
                M5WindowRestoreDowngradeTrigger::LeftWindowsOrDialogsUnreachableAfterDisplayTopologyRemap
            }
            Self::TopologyAdjustmentNotRecorded => {
                M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a role-continuity-fence entry degraded below a clean, no-reset state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoleContinuityEntryDegradeReason {
    /// The canonical registry token name is unstated.
    RoleTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The role-continuity class is unclassified (not in the resolved taxonomy).
    RoleContinuityClassUnclassified,
    /// The fence resets or overclaims — it reset a follow / presentation state, a collaboration role badge, or an
    /// auxiliary-window purpose into a generic window, dropped a role that was present before the remap, or hid
    /// that layout fidelity was reduced, or it dropped the preserved-role-label / boundary-label / provenance-hint
    /// disclosure triple.
    RoleContinuityResetsOrOverclaims,
    /// The canonical / accessible / audit resolution-form coverage of the fence is incomplete.
    RoleFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RoleContinuityEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RoleTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RoleContinuityClassUnclassified,
        Self::RoleContinuityResetsOrOverclaims,
        Self::RoleFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoleTokenUnstated => "role_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RoleContinuityClassUnclassified => "role_continuity_class_unclassified",
            Self::RoleContinuityResetsOrOverclaims => "role_continuity_resets_or_overclaims",
            Self::RoleFormCoverageIncomplete => "role_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DisplayTopologyOrchestrationNextAction {
        match self {
            Self::RoleTokenUnstated => {
                M5DisplayTopologyOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::RoleContinuityClassUnclassified | Self::RoleContinuityResetsOrOverclaims => {
                M5DisplayTopologyOrchestrationNextAction::InspectBoundsOrRoleContinuity
            }
            Self::RoleFormCoverageIncomplete => {
                M5DisplayTopologyOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5DisplayTopologyOrchestrationNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::RoleTokenUnstated => M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::RoleContinuityClassUnclassified => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RoleContinuityResetsOrOverclaims => {
                M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened
            }
            Self::RoleFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::DisplayAffinityUnstated
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_bounds_recovery_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BoundsRecoveryEntryResolutionInput {
    /// Stable identity of the bounds-recovery-registry entry.
    pub entry_id: String,
    /// The stable remap-target ID this bounds recovery binds to (e.g. `window.acme.editor-main`); empty means
    /// unstated.
    pub remap_target_id: String,
    /// The canonical registry token name (e.g. `bounds.recovery.clamped_onto_visible_bounds`); empty means
    /// unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The bounds-recovery state this entry resolves.
    pub bounds_recovery_state: M5BoundsRecoveryState,
    /// The render / surface context.
    pub surface_context: M5DisplayTopologyOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5DisplayTopologyOrchestrationResolutionForm>,
    /// The published window / dialog / sheet surface ID; empty means unstated.
    pub window_surface_id: String,
    /// The published monitor-affinity hint; empty means unstated.
    pub affinity_monitor_hint: String,
    /// The published resolved visible bounds; empty means unstated.
    pub resolved_visible_bounds: String,
    /// The published layout intent preserved across the remap; empty means unstated.
    pub layout_intent: String,
    /// The published provenance class (live / reduced-fidelity / remapped); empty means unstated.
    pub provenance_class: String,
    /// The published keyboard-reach plan kept distinct so a blocking dialog stays reachable; empty means
    /// unstated.
    pub keyboard_reach_plan: String,
    /// True when the behavior traces to the bounds-recovery registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the bounds are resolved onto visible bounds before the surface is presented (a hard invariant
    /// when `false`).
    pub bounds_resolved_before_present: bool,
    /// True when this recovery is a material topology adjustment that changed layout fidelity.
    pub is_material_topology_adjustment: bool,
    /// True when a material topology adjustment is recorded in restore provenance.
    pub topology_adjustment_recorded_when_material: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe bounds-recovery-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBoundsRecoveryEntry {
    /// Stable identity of the bounds-recovery-registry entry.
    pub entry_id: String,
    /// The stable remap-target ID this bounds recovery binds to.
    pub remap_target_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The bounds-recovery-state token named by the entry.
    pub bounds_recovery_state: String,
    /// Whether the bounds-recovery state is classified into the resolved taxonomy.
    pub bounds_recovery_state_is_classified: bool,
    /// The canonical bounds-recovery mode for the entry's state.
    pub canonical_bounds_recovery_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published window / dialog / sheet surface ID.
    pub window_surface_id: String,
    /// The published monitor-affinity hint.
    pub affinity_monitor_hint: String,
    /// The published resolved visible bounds.
    pub resolved_visible_bounds: String,
    /// The published layout intent.
    pub layout_intent: String,
    /// The published provenance class.
    pub provenance_class: String,
    /// The published keyboard-reach plan.
    pub keyboard_reach_plan: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved bounds-recovery object publishes every required field.
    pub bounds_recovery_object_complete: bool,
    /// Whether the entry traces to the bounds-recovery registry.
    pub bound_to_registry: bool,
    /// Whether the bounds are resolved before the surface is presented.
    pub bounds_resolved_before_present: bool,
    /// Whether this recovery is a material topology adjustment.
    pub is_material_topology_adjustment: bool,
    /// Whether a material topology adjustment is recorded in provenance.
    pub topology_adjustment_recorded_when_material: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5BoundsRecoveryEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DisplayTopologyOrchestrationNextAction,
    /// Whether the bounds recovery resolves to one stable object across every claimed remap (clean entry naming
    /// every fact).
    pub bounds_resolve_across_remaps: bool,
}

impl M5ResolvedBoundsRecoveryEntry {
    /// Whether this bounds-recovery entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_role_continuity_fence_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RoleContinuityFenceEntryResolutionInput {
    /// Stable identity of the role-continuity-fence entry.
    pub entry_id: String,
    /// The stable guarded-window ID this fence binds to; empty means unstated.
    pub guarded_window_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The role-continuity class this entry must resolve its fence from.
    pub role_class: M5RoleContinuityClass,
    /// The render / surface context.
    pub surface_context: M5DisplayTopologyOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5DisplayTopologyOrchestrationResolutionForm>,
    /// The published preserved role label kept visible after remap; empty means missing.
    pub preserved_role_label: String,
    /// The published boundary label (auxiliary-window purpose / role badge boundary); empty means missing.
    pub boundary_label: String,
    /// The published provenance hint (live / reduced-fidelity / remapped) kept distinct from a full-fidelity
    /// claim; empty means missing.
    pub provenance_hint: String,
    /// True when the fence preserves the role label and boundary (never a reset into a generic window).
    pub preserves_role_and_boundary: bool,
    /// True when the fence is truthful (never resets an auxiliary window into a generic window or hides that
    /// fidelity was reduced).
    pub fence_is_truthful: bool,
    /// True when the window carried a follow / presentation state, a collaboration role badge, or an
    /// auxiliary-window purpose before the remap.
    pub role_was_present_used: bool,
    /// True when a role that was present before the remap is preserved after the remap rather than reset to
    /// generic.
    pub role_preserved_after_remap: bool,
    /// True when the remap reduced layout fidelity for this window.
    pub fidelity_reduced: bool,
    /// True when a reduced layout fidelity is disclosed rather than hidden.
    pub fidelity_reduction_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe role-continuity-fence projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRoleContinuityFenceEntry {
    /// Stable identity of the role-continuity-fence entry.
    pub entry_id: String,
    /// The stable guarded-window ID this fence binds to.
    pub guarded_window_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The role-continuity-class token named by the entry.
    pub role_class: String,
    /// Whether the role-continuity class is classified into the resolved taxonomy.
    pub role_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published preserved role label.
    pub preserved_role_label: String,
    /// The published boundary label.
    pub boundary_label: String,
    /// The published provenance hint.
    pub provenance_hint: String,
    /// Whether the fence preserves the role label and boundary.
    pub preserves_role_and_boundary: bool,
    /// Whether the fence is truthful.
    pub fence_is_truthful: bool,
    /// Whether the window carried a role before the remap.
    pub role_was_present_used: bool,
    /// Whether a role that was present before the remap is preserved after the remap.
    pub role_preserved_after_remap: bool,
    /// Whether the remap reduced layout fidelity.
    pub fidelity_reduced: bool,
    /// Whether a reduced layout fidelity is disclosed.
    pub fidelity_reduction_disclosed: bool,
    /// Whether the fence holds no-reset (no reset into a generic window, role label and boundary preserved, a
    /// present role preserved after remap, a reduced fidelity disclosed).
    pub fence_holds_no_reset: bool,
    /// Whether the entry provides the complete preserved-role-label / boundary-label / provenance-hint disclosure
    /// triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, no-reset state.
    pub degrade_reason: Option<M5RoleContinuityEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DisplayTopologyOrchestrationNextAction,
    /// Whether the fence holds on every claimed surface (clean entry naming every fact).
    pub fence_holds_on_every_surface: bool,
}

impl M5ResolvedRoleContinuityFenceEntry {
    /// Whether this role-continuity-fence entry reads as a clean, no-reset state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DisplayTopologyOrchestrationResolutionError {
    /// The bounds-recovery-entry id was empty.
    EmptyBoundsRecoveryEntryId,
    /// The role-continuity-fence-entry id was empty.
    EmptyRoleContinuityFenceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DisplayTopologyOrchestrationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBoundsRecoveryEntryId => "empty_bounds_recovery_entry_id",
            Self::EmptyRoleContinuityFenceEntryId => "empty_role_continuity_fence_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DisplayTopologyOrchestrationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 bounds-recovery / role-continuity registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DisplayTopologyOrchestrationResolutionError {}

fn form_tokens(forms: &[M5DisplayTopologyOrchestrationResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5DisplayTopologyOrchestrationResolutionForm]) -> bool {
    let present: BTreeSet<M5DisplayTopologyOrchestrationResolutionForm> =
        forms.iter().copied().collect();
    M5DisplayTopologyOrchestrationResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved bounds-recovery object publishes every required field: bounds-recovery mode (via a
/// classified state), window surface, monitor-affinity hint, resolved visible bounds, layout intent, provenance
/// class, and the distinct keyboard-reach plan. An unclassified state or any empty field never resolves to a
/// complete object.
#[allow(clippy::too_many_arguments)]
pub fn bounds_recovery_object_is_complete(
    state: M5BoundsRecoveryState,
    window_surface_id: &str,
    affinity_monitor_hint: &str,
    resolved_visible_bounds: &str,
    layout_intent: &str,
    provenance_class: &str,
    keyboard_reach_plan: &str,
) -> bool {
    state.is_classified()
        && !window_surface_id.trim().is_empty()
        && !affinity_monitor_hint.trim().is_empty()
        && !resolved_visible_bounds.trim().is_empty()
        && !layout_intent.trim().is_empty()
        && !provenance_class.trim().is_empty()
        && !keyboard_reach_plan.trim().is_empty()
}

/// Whether the bounds are resolved before the surface is presented: the state must be classified, the bounds
/// must be resolved before the window or dialog is presented, and a material topology adjustment must be recorded
/// in provenance. An unclassified state, a present that preceded the bounds, or an unrecorded material adjustment
/// never matches.
pub fn bounds_precede_present(
    state: M5BoundsRecoveryState,
    bounds_resolved_before_present: bool,
    is_material_topology_adjustment: bool,
    topology_adjustment_recorded_when_material: bool,
) -> bool {
    state.is_classified()
        && bounds_resolved_before_present
        && (!is_material_topology_adjustment || topology_adjustment_recorded_when_material)
}

/// Whether a role-continuity fence holds no-reset and continuity-preserving: the class must be classified, the
/// fence must be truthful, it must preserve the role label and boundary, any role that was present before the
/// remap must be preserved after the remap rather than reset to generic, and any reduced layout fidelity must be
/// disclosed rather than hidden.
pub fn role_continuity_fence_holds(
    role_class: M5RoleContinuityClass,
    fence_is_truthful: bool,
    preserves_role_and_boundary: bool,
    role_was_present_used: bool,
    role_preserved_after_remap: bool,
    fidelity_reduced: bool,
    fidelity_reduction_disclosed: bool,
) -> bool {
    role_class.is_classified()
        && fence_is_truthful
        && preserves_role_and_boundary
        && (!role_was_present_used || role_preserved_after_remap)
        && (!fidelity_reduced || fidelity_reduction_disclosed)
}

/// Resolves a bounds-recovery-registry entry so it stays bound to the bounds-recovery registry: the entry names
/// its canonical token, semantic role, and bounds-recovery state, covers all three resolution forms, publishes a
/// complete bounds-recovery object (window surface, monitor-affinity hint, resolved visible bounds, layout
/// intent, provenance class, distinct keyboard-reach plan), resolves the bounds before the surface is presented,
/// and records a material topology adjustment in provenance when one happened.
pub fn resolve_bounds_recovery_entry(
    input: M5BoundsRecoveryEntryResolutionInput,
) -> Result<M5ResolvedBoundsRecoveryEntry, M5DisplayTopologyOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5DisplayTopologyOrchestrationResolutionError::EmptyBoundsRecoveryEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.remap_target_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.window_surface_id)
        || string_is_forbidden(&input.affinity_monitor_hint)
        || string_is_forbidden(&input.resolved_visible_bounds)
        || string_is_forbidden(&input.layout_intent)
        || string_is_forbidden(&input.provenance_class)
        || string_is_forbidden(&input.keyboard_reach_plan)
    {
        return Err(M5DisplayTopologyOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = bounds_recovery_object_is_complete(
        input.bounds_recovery_state,
        &input.window_surface_id,
        &input.affinity_monitor_hint,
        &input.resolved_visible_bounds,
        &input.layout_intent,
        &input.provenance_class,
        &input.keyboard_reach_plan,
    );
    let bounds_ok = bounds_precede_present(
        input.bounds_recovery_state,
        input.bounds_resolved_before_present,
        input.is_material_topology_adjustment,
        input.topology_adjustment_recorded_when_material,
    );
    let adjustment_unrecorded =
        input.is_material_topology_adjustment && !input.topology_adjustment_recorded_when_material;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5BoundsRecoveryEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.bounds_recovery_state.is_classified() {
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryStateUnclassified)
    } else if !input.bound_to_registry {
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsNotBoundToRegistry)
    } else if !object_complete {
        Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryObjectIncomplete)
    } else if !bounds_ok {
        Some(M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds)
    } else if !all_forms {
        Some(M5BoundsRecoveryEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if adjustment_unrecorded {
        Some(M5BoundsRecoveryEntryDegradeReason::TopologyAdjustmentNotRecorded)
    } else if !input.proof_fresh {
        Some(M5BoundsRecoveryEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DisplayTopologyOrchestrationNextAction::ExpandLayoutMeaning,
    };

    Ok(M5ResolvedBoundsRecoveryEntry {
        entry_id: input.entry_id,
        remap_target_id: input.remap_target_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        bounds_recovery_state: input.bounds_recovery_state.as_str().to_owned(),
        bounds_recovery_state_is_classified: input.bounds_recovery_state.is_classified(),
        canonical_bounds_recovery_mode: input
            .bounds_recovery_state
            .canonical_bounds_recovery_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        window_surface_id: input.window_surface_id,
        affinity_monitor_hint: input.affinity_monitor_hint,
        resolved_visible_bounds: input.resolved_visible_bounds,
        layout_intent: input.layout_intent,
        provenance_class: input.provenance_class,
        keyboard_reach_plan: input.keyboard_reach_plan,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        bounds_recovery_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        bounds_resolved_before_present: input.bounds_resolved_before_present,
        is_material_topology_adjustment: input.is_material_topology_adjustment,
        topology_adjustment_recorded_when_material: input
            .topology_adjustment_recorded_when_material,
        degrade_reason,
        next_action,
        bounds_resolve_across_remaps: degrade_reason.is_none(),
    })
}

/// Resolves a role-continuity-fence entry so its fence holds no-reset: the entry names its canonical token,
/// semantic role, and role-continuity class, covers all three resolution forms, provides the preserved-role-label
/// / boundary-label / provenance-hint disclosure triple, and degrades honestly when the fence resets a follow /
/// presentation state, a collaboration role badge, or an auxiliary-window purpose into a generic window, drops a
/// role that was present before the remap, or hides that layout fidelity was reduced.
pub fn resolve_role_continuity_fence_entry(
    input: M5RoleContinuityFenceEntryResolutionInput,
) -> Result<M5ResolvedRoleContinuityFenceEntry, M5DisplayTopologyOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5DisplayTopologyOrchestrationResolutionError::EmptyRoleContinuityFenceEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.guarded_window_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.preserved_role_label)
        || string_is_forbidden(&input.boundary_label)
        || string_is_forbidden(&input.provenance_hint)
    {
        return Err(M5DisplayTopologyOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let fence_holds_no_reset = role_continuity_fence_holds(
        input.role_class,
        input.fence_is_truthful,
        input.preserves_role_and_boundary,
        input.role_was_present_used,
        input.role_preserved_after_remap,
        input.fidelity_reduced,
        input.fidelity_reduction_disclosed,
    );
    let provides_triple = input.role_class.is_classified()
        && !input.preserved_role_label.trim().is_empty()
        && !input.boundary_label.trim().is_empty()
        && !input.provenance_hint.trim().is_empty()
        && fence_holds_no_reset;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RoleContinuityEntryDegradeReason::RoleTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RoleContinuityEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.role_class.is_classified() {
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityClassUnclassified)
    } else if !provides_triple {
        Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
    } else if !all_forms {
        Some(M5RoleContinuityEntryDegradeReason::RoleFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5RoleContinuityEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DisplayTopologyOrchestrationNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedRoleContinuityFenceEntry {
        entry_id: input.entry_id,
        guarded_window_id: input.guarded_window_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        role_class: input.role_class.as_str().to_owned(),
        role_class_is_classified: input.role_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        preserved_role_label: input.preserved_role_label,
        boundary_label: input.boundary_label,
        provenance_hint: input.provenance_hint,
        preserves_role_and_boundary: input.preserves_role_and_boundary,
        fence_is_truthful: input.fence_is_truthful,
        role_was_present_used: input.role_was_present_used,
        role_preserved_after_remap: input.role_preserved_after_remap,
        fidelity_reduced: input.fidelity_reduced,
        fidelity_reduction_disclosed: input.fidelity_reduction_disclosed,
        fence_holds_no_reset,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        fence_holds_on_every_surface: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved bounds-recovery and role-continuity entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5DisplayTopologyOrchestrationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DisplayTopologyOrchestrationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    /// Resolved bounds-recovery-registry examples.
    pub bounds_recovery_entries: Vec<M5ResolvedBoundsRecoveryEntry>,
    /// Resolved role-continuity-fence examples.
    pub role_continuity_fence_entries: Vec<M5ResolvedRoleContinuityFenceEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the restore-fidelity and window-topology
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: no restored or remapped window or dialog is stranded off-screen after a display-topology
    /// change. MUST be `false`.
    pub strands_window_or_dialog_offscreen_after_remap: bool,
    /// Hard invariant: an auxiliary window is never reset into a generic window after remap. MUST be `false`.
    pub resets_auxiliary_window_into_generic_window: bool,
    /// Hard invariant: bounds-recovery and role-continuity state are never merged into one opaque blob. MUST be
    /// `false`.
    pub merges_bounds_recovery_and_role_continuity_into_one_opaque_blob: bool,
    /// Hard invariant: layout fidelity is never overclaimed when only bounds or context recovered. MUST be
    /// `false`.
    pub overclaims_layout_fidelity_when_only_bounds_or_context_recovered: bool,
}

impl M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DisplayTopologyOrchestrationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DisplayTopologyOrchestrationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DisplayTopologyOrchestrationExportField> =
            self.export_fields.iter().copied().collect();
        M5DisplayTopologyOrchestrationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.strands_window_or_dialog_offscreen_after_remap
            && !self.resets_auxiliary_window_into_generic_window
            && !self.merges_bounds_recovery_and_role_continuity_into_one_opaque_blob
            && !self.overclaims_layout_fidelity_when_only_bounds_or_context_recovered
    }

    /// True when a clean bounds-recovery entry preserves registry-bound truth: it traces to the registry, keeps a
    /// classified bounds-recovery state, publishes a complete bounds object, resolves the bounds before present,
    /// covers all three resolution forms, and records a material topology adjustment in provenance.
    fn bounds_is_honest(ex: &M5ResolvedBoundsRecoveryEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.bounds_recovery_state_is_classified
                && ex.bounds_recovery_object_complete
                && ex.bounds_resolved_before_present
                && ex.covers_all_resolution_forms
                && (!ex.is_material_topology_adjustment
                    || ex.topology_adjustment_recorded_when_material))
    }

    /// True when a clean role-continuity-fence entry preserves no-reset continuity: it keeps a classified class,
    /// provides the disclosure triple, holds no-reset, and covers all three resolution forms.
    fn fence_is_honest(ex: &M5ResolvedRoleContinuityFenceEntry) -> bool {
        !ex.is_clean()
            || (ex.role_class_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.fence_holds_no_reset
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.bounds_recovery_entries
            .iter()
            .all(Self::bounds_is_honest)
            && self
                .role_continuity_fence_entries
                .iter()
                .all(Self::fence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Bounds-recovery-state tokens (minted by this lane).
    pub bounds_recovery_states: Vec<String>,
    /// Role-continuity-class tokens (minted by this lane).
    pub role_continuity_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Bounds-recovery-entry degrade-reason tokens.
    pub bounds_recovery_degrade_reasons: Vec<String>,
    /// Role-continuity-fence-entry degrade-reason tokens.
    pub role_continuity_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DisplayTopologyRecoveryAndRoleContinuityRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5WindowRestoreRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5DisplayTopologyOrchestrationResolutionForm::ALL, |v| {
                v.as_str()
            }),
            bounds_recovery_states: tokens(&M5BoundsRecoveryState::ALL, |v| v.as_str()),
            role_continuity_classes: tokens(&M5RoleContinuityClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5DisplayTopologyOrchestrationSurfaceContext::ALL, |v| {
                v.as_str()
            }),
            bounds_recovery_degrade_reasons: tokens(
                &M5BoundsRecoveryEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            role_continuity_degrade_reasons: tokens(
                &M5RoleContinuityEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5DisplayTopologyOrchestrationAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            next_actions: tokens(&M5DisplayTopologyOrchestrationNextAction::ALL, |v| {
                v.as_str()
            }),
            export_fields: tokens(&M5DisplayTopologyOrchestrationExportField::ALL, |v| {
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
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesGovernanceReview {
    /// The bounds-recovery registry names a canonical token, semantic role, and bounds-recovery state for every
    /// entry.
    pub bounds_registry_names_token_role_and_bounds_state: bool,
    /// Every claimed remap resolves each window, dialog, or sheet to one stable bounds-recovery object from the
    /// shared registry, not per-surface reconstruction.
    pub remap_resolves_to_stable_bounds_object_from_shared_registry: bool,
    /// Window surface, resolved visible bounds, monitor-affinity hint, and provenance are published for every
    /// resolved recovery.
    pub window_surface_bounds_affinity_and_provenance_published: bool,
    /// The bounds are resolved onto visible bounds before the surface is presented.
    pub bounds_resolved_before_surface_presented: bool,
    /// The role-continuity fence keeps follow / presentation state, collaboration role badges, and
    /// auxiliary-window purpose visible after remap, and never resets a window into generic.
    pub role_fence_keeps_role_visible_and_never_resets_to_generic: bool,
    /// The fact that layout fidelity was reduced is never hidden when a material topology adjustment happened.
    pub fidelity_reduction_never_hidden_when_material: bool,
    /// Every bounds-recovery and role-continuity entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Bounds-recovery and role-continuity behavior stay bound to the shared registries rather than hand-copied
    /// per surface.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shell, recovery, diagnostics, and admin read a single display-topology-recovery source.
    pub shell_recovery_diagnostics_admin_read_single_source: bool,
    /// An off-screen present, an incomplete object, or a role reset is caught by fixtures before release evidence
    /// turns green.
    pub bounds_or_fence_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerProjection {
    /// Shell and recovery consume the shared bounds-recovery registry.
    pub shell_and_recovery_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared role-continuity registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Session and workspace services consume the shared registries.
    pub session_and_workspace_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical restore-fidelity and window-topology domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical bounds-recovery / role-continuity registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting window-restore audit for the lane.
    pub window_restore_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 display-topology-recovery bounds-recovery and role-continuity registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket {
    /// Record kind; must equal
    /// [`M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version:
                M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind
            != M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_RECORD_KIND
        {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::WrongRecordKind,
            );
        }
        if self.schema_version
            != M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MissingIdentity,
            );
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 bounds-recovery / role-continuity registries packet serializes"),
        ) {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RawMaterialInExport,
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
            .expect("m5 bounds-recovery / role-continuity registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,bounds_recovery_entries,role_continuity_fence_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .bounds_recovery_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.role_continuity_fence_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.bounds_recovery_entries.len(),
                row.role_continuity_fence_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Display-Topology-Recovery Bounds-Recovery and Role-Continuity Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Bounds-recovery states: {}\n",
            self.vocabulary_set.bounds_recovery_states.join(", ")
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
                "  - Bounds-recovery entries: {} / role-continuity-fence entries: {}\n",
                row.bounds_recovery_entries.len(),
                row.role_continuity_fence_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-remap bounds-recovery reference table generated from the registry, so docs and admin
    /// runbooks render the same bounds-recovery-mode / window-surface / affinity-hint / resolved-bounds /
    /// provenance / keyboard-reach truth the resolvers produced rather than a hand-copied bounds table. Only
    /// clean, registry-bound bounds-recovery entries are listed.
    pub fn render_bounds_recovery_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| remap_target_id | bounds_recovery_mode | window_surface_id | affinity_monitor_hint | resolved_visible_bounds | provenance_class | keyboard_reach_plan |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.bounds_recovery_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.remap_target_id,
                    ex.canonical_bounds_recovery_mode,
                    ex.window_surface_id,
                    ex.affinity_monitor_hint,
                    ex.resolved_visible_bounds,
                    ex.provenance_class,
                    ex.keyboard_reach_plan
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>),
}

impl fmt::Display for M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 bounds-recovery / role-continuity registries export parse failed: {error}"
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
                    "m5 bounds-recovery / role-continuity registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError {}

/// Validation failures emitted by
/// [`M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation {
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
    /// A registry row carries a dishonest clean example (hand-copied, present-first, field-incomplete,
    /// form-incomplete, or a role-continuity entry missing the disclosure triple).
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
    /// Bounds-recovery-resolution is not proven: clean bounds entries do not cover the canonical bounds states or
    /// the first shell / recovery / diagnostics / admin / support surfaces, no object-incomplete example
    /// degrades, or a clean bounds entry published an incomplete object.
    BoundsRecoveryResolutionNotProven,
    /// Bounds-before-present is not proven: no present-first example and no unbound example degrade, no clean
    /// bounds-before-present entry is present, or a clean bounds entry presented first or is unbound.
    BoundsBeforePresentNotProven,
    /// Role-continuity is not proven: clean role-continuity entries do not cover the canonical follow-presentation
    /// / collaboration-badge / auxiliary-purpose classes with full resolution-form coverage while providing the
    /// disclosure triple, no resets-or-overclaims or form-incomplete example degrades, or a clean fence entry is
    /// missing the triple.
    RoleContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation {
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
            Self::BoundsRecoveryResolutionNotProven => "bounds_recovery_resolution_not_proven",
            Self::BoundsBeforePresentNotProven => "bounds_before_present_not_proven",
            Self::RoleContinuityNotProven => "role_continuity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_display_topology_recovery_and_role_continuity_registries_export() -> Result<
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError,
> {
    let packet: M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-display-topology-recovery-and-role-continuity-registries-proof/support_export.json"
        )
    ))
    .map_err(M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesArtifactError::Validation(
                violations,
            ),
        )
    }
}

fn validate_source_contracts(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_SCHEMA_REF,
        M5_DISPLAY_TOPOLOGY_RECOVERY_AND_ROLE_CONTINUITY_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations
            .push(M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::NoRegistryRows);
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
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::MandatoryExportFieldMissing,
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
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.bounds_recovery_entries.is_empty() || row.role_continuity_fence_entries.is_empty() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ExamplesMissing,
            );
        }
        if !row.examples_are_honest() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::DishonestExample,
            );
        }
        if !row.honours_invariants() {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.bounds_registry_names_token_role_and_bounds_state,
        review.remap_resolves_to_stable_bounds_object_from_shared_registry,
        review.window_surface_bounds_affinity_and_provenance_published,
        review.bounds_resolved_before_surface_presented,
        review.role_fence_keeps_role_visible_and_never_resets_to_generic,
        review.fidelity_reduction_never_hidden_when_material,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shell_recovery_diagnostics_admin_read_single_source,
        review.bounds_or_fence_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
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
                M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.window_restore_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5DisplayTopologyRecoveryAndRoleContinuityRegistriesPacket,
    violations: &mut Vec<M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation>,
) {
    let bounds = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.bounds_recovery_entries.iter())
    };
    let fences = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.role_continuity_fence_entries.iter())
    };

    // AC (auxiliary windows preserve their intended role and boundary labels after monitor or DPI changes): every
    // claimed remap resolves each window, dialog, or sheet to one stable bounds-recovery object with
    // window-surface / affinity-hint / resolved-bounds / provenance / distinct-keyboard-reach fields. Clean
    // bounds entries cover the canonical bounds states and the first shell / recovery / diagnostics / admin /
    // support surfaces, an object-incomplete example degrades, and no clean bounds entry published an incomplete
    // object.
    let clean_states: BTreeSet<String> = bounds()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.bounds_recovery_state.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = bounds()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let states_covered = M5BoundsRecoveryState::CANONICAL_STATES
        .iter()
        .all(|c| clean_states.contains(c.as_str()));
    let first_surfaces_covered = M5DisplayTopologyOrchestrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = bounds().any(|ex| {
        ex.degrade_reason
            == Some(M5BoundsRecoveryEntryDegradeReason::BoundsRecoveryObjectIncomplete)
    });
    let no_clean_incomplete =
        !bounds().any(|ex| ex.is_clean() && !ex.bounds_recovery_object_complete);
    if !(states_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsRecoveryResolutionNotProven,
        );
    }

    // AC (no restored or remapped window can open wholly off-screen or strand a blocking dialog beyond keyboard
    // reach): the bounds are resolved before the surface is presented. A present-first example degrades, an
    // unbound example degrades, at least one clean bounds-before-present entry is present, and no clean bounds
    // entry presented first or is unbound.
    let preceded_degrades = bounds().any(|ex| {
        ex.degrade_reason == Some(M5BoundsRecoveryEntryDegradeReason::PresentPrecededBounds)
    });
    let unbound_degrades = bounds().any(|ex| {
        ex.degrade_reason == Some(M5BoundsRecoveryEntryDegradeReason::BoundsNotBoundToRegistry)
    });
    let resolved_clean_bounds =
        bounds().any(|ex| ex.is_clean() && ex.bounds_resolved_before_present);
    let no_clean_unbound = !bounds().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_present_first =
        !bounds().any(|ex| ex.is_clean() && !ex.bounds_resolved_before_present);
    if !(preceded_degrades
        && unbound_degrades
        && resolved_clean_bounds
        && no_clean_unbound
        && no_clean_present_first)
    {
        violations.push(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::BoundsBeforePresentNotProven,
        );
    }

    // AC (display-topology drills fail when remap loses collaboration/presentation context or hides that fidelity
    // was reduced): clean role-continuity-fence entries cover every canonical follow-presentation /
    // collaboration-badge / auxiliary-purpose class with full resolution-form coverage while providing the
    // disclosure triple, a resets-or-overclaims example degrades, a form-incomplete example degrades, and no clean
    // fence entry is missing the triple.
    let clean_fence_classes: BTreeSet<String> = fences()
        .filter(|ex| {
            ex.is_clean()
                && ex.role_class_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.role_class.clone())
        .collect();
    let fence_classes_covered = M5RoleContinuityClass::CANONICAL_CLASSES
        .iter()
        .all(|s| clean_fence_classes.contains(s.as_str()));
    let resets_degrades = fences().any(|ex| {
        ex.degrade_reason
            == Some(M5RoleContinuityEntryDegradeReason::RoleContinuityResetsOrOverclaims)
    });
    let form_incomplete_degrades = fences().any(|ex| {
        ex.degrade_reason == Some(M5RoleContinuityEntryDegradeReason::RoleFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !fences().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(fence_classes_covered
        && resets_degrades
        && form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5DisplayTopologyRecoveryAndRoleContinuityRegistriesViolation::RoleContinuityNotProven,
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
    [M5WindowRestoreFamily::DisplayTopologyRecovery];
