//! Implemented M5 monitor-topology geometry-remap and restore-bounds registries.
//!
//! The frozen [shell-metric / density matrix][matrix] names Aureline's five shell-geometry families and
//! locks their controlled vocabulary. The responsive-geometry / collapse-priority implement lane turned the
//! adaptive-layout grammar into registry resolvers; this module is the monitor-topology continuation of that
//! responsive-geometry family. It ties concrete shell geometry to real desktop topology changes so a monitor
//! attach or detach, a DPI change, an undock, a fullscreen transition, or a snapped-layout recovery preserves
//! a usable on-screen layout rather than replaying stale absolute coordinates. It carries two registry
//! resolvers that produce export-safe, honest projections:
//!
//! * **Monitor-aware restore bounds.** [`resolve_restore_bounds_entry`] refuses to read as a clean,
//!   registry-bound restore entry unless it names a canonical registry token, a classified
//!   [restore-surface kind][M5RestoreSurfaceKind] (window, approval sheet, dialog, docked panel, split
//!   layout), a classified [topology change][M5TopologyChange], clamps the restored surface into visible
//!   bounds, never reopens fully off-screen or traps focus after a monitor or DPI change, preserves usable
//!   editor / panel / inspector geometry, persists layout intent and monitor-affinity hints instead of brittle
//!   absolute coordinates, and — whenever fidelity must be reduced — offers a one-click or command-backed
//!   recenter / reset affordance.
//! * **Geometry-remap provenance.** [`resolve_geometry_remap_provenance_entry`] keeps geometry continuity
//!   distinct from surface continuity: it preserves the user's workspace, focus chain, and critical state
//!   while honestly recording every remap or fallback in restore provenance, naming the remap trigger, the
//!   before / after monitor topology and DPI scale, the [fidelity outcome][M5RemapFidelityOutcome], and enough
//!   detail to diagnose why fidelity changed. It degrades to
//!   [`M5GeometryRemapProvenanceEntryDegradeReason::SilentlyDropsWorkspaceOrState`] when a remap would silently
//!   drop the workspace, focus chain, or recovery-critical state.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Persist layout intent, proportions, and monitor-affinity hints instead of brittle absolute
//!   coordinates.** A restore entry that replays stale absolute coordinates rather than resolving persisted
//!   intent degrades to [`M5RestoreBoundsEntryDegradeReason::ReplaysStaleAbsoluteCoordinates`].
//! * **Clamp every restored window, sheet, and dialog into visible bounds, and provide recenter / reset
//!   affordances when fidelity must be reduced.** A restore entry that reopens off-screen, traps focus, or
//!   omits the recenter affordance under reduced fidelity degrades honestly.
//! * **Keep geometry continuity distinct from surface continuity, recording any remap or fallback in restore
//!   provenance.** Each registry row carries the render [surface context][M5RemapSurfaceContext] so an
//!   off-screen, focus-trapping, or provenance-losing regression degrades honestly, and the acceptance-criteria
//!   gate proves the drift is caught before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ShellGeometryRole`] role vocabulary and
//! the [`M5ResponsiveGeometryRole`] responsive-geometry-role vocabulary — so shell, editor, review, notebook,
//! data, and support surfaces can never fork their own monitor-topology meaning. Raw secret values and private
//! endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_shell_metric_density_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_monitor_geometry_remap_and_restore_bounds,
    seeded_m5_monitor_geometry_remap_and_restore_bounds_editor_ui_beta_narrowed,
    seeded_m5_monitor_geometry_remap_and_restore_bounds_settings_ui_preview_narrowed,
    M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shell_metric_density_matrix::{
    M5ResponsiveGeometryRole, M5ShellGeometryAccessibilityRoute, M5ShellGeometryConsumerSurface,
    M5ShellGeometryDeploymentLine, M5ShellGeometryDowngradeTrigger, M5ShellGeometryFamily,
    M5ShellGeometryQualificationClass, M5ShellGeometryRequiredLabel, M5ShellGeometryRole,
    M5_DENSITY_MODE_SCHEMA_REF, M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
    M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5MonitorGeometryRemapAndRestoreBoundsPacket`].
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_RECORD_KIND: &str =
    "implement_m5_monitor_geometry_remap_and_restore_bounds";

/// Schema version for M5 monitor-topology geometry-remap / restore-bounds records.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the registries schema.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_REF: &str =
    "schemas/shell/m5-monitor-geometry-remap-and-restore-bounds.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_DOC_REF: &str =
    "docs/design-system/m5_monitor_geometry_remap_and_restore_bounds.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_ARTIFACT_REF: &str =
    "artifacts/release/m5-monitor-geometry-remap-and-restore-bounds-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_CSV_REF: &str =
    "artifacts/release/m5-monitor-geometry-remap-and-restore-bounds-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_REPORT_REF: &str =
    "artifacts/release/m5-monitor-geometry-remap-and-restore-bounds-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-monitor-geometry-remap-and-restore-bounds";

/// Canonical minimum supported desktop width in logical pixels; a restored surface narrower than this floor is
/// below the supported minimum usable width.
pub const CANONICAL_MINIMUM_SUPPORTED_WIDTH_PX: u32 = 1024;

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5MonitorGeometryRegistriesConsumerSurface = M5ShellGeometryConsumerSurface;

/// One of the surface kinds a restore entry clamps into visible bounds after a topology change, so a restored
/// window, approval sheet, dialog, docked panel, or split layout can never reopen off-screen or trap focus.
/// Minted by this lane because the frozen matrix names the responsive-geometry *family* but not the concrete
/// restore-surface set. The approval sheet is called out directly by the acceptance criteria.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreSurfaceKind {
    /// A restorable top-level window.
    RestorableWindow,
    /// An approval sheet (a modal decision surface that must never reopen off-screen or trap focus).
    ApprovalSheet,
    /// A dialog.
    Dialog,
    /// A docked panel restored into a shell zone.
    DockedPanel,
    /// A restored split layout.
    SplitLayout,
    /// The restore-surface kind is unclassified, which is disallowed.
    KindUnclassified,
}

impl M5RestoreSurfaceKind {
    /// Every restore-surface kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RestorableWindow,
        Self::ApprovalSheet,
        Self::Dialog,
        Self::DockedPanel,
        Self::SplitLayout,
        Self::KindUnclassified,
    ];

    /// The five canonical restore-surface kinds every claimed M5 surface restores through the shared registry.
    pub const CANONICAL_KINDS: [Self; 5] = [
        Self::RestorableWindow,
        Self::ApprovalSheet,
        Self::Dialog,
        Self::DockedPanel,
        Self::SplitLayout,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestorableWindow => "restorable_window",
            Self::ApprovalSheet => "approval_sheet",
            Self::Dialog => "dialog",
            Self::DockedPanel => "docked_panel",
            Self::SplitLayout => "split_layout",
            Self::KindUnclassified => "kind_unclassified",
        }
    }

    /// Whether the restore-surface kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::KindUnclassified)
    }
}

/// A desktop topology change that forces a geometry remap, so the same restore grammar covers monitor attach /
/// detach, undock, DPI change, fullscreen transition, and snapped-layout recovery. Minted by this lane,
/// tracking the topology changes the implementation requirements name directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TopologyChange {
    /// A monitor was attached.
    MonitorAttach,
    /// A monitor was detached.
    MonitorDetach,
    /// The device was undocked.
    Undock,
    /// The DPI / scale factor changed (a mixed-DPI transition).
    DpiChange,
    /// A fullscreen transition entered or left.
    FullscreenTransition,
    /// A snapped-layout recovery.
    SnappedLayoutRecovery,
    /// The topology change is unclassified, which is disallowed.
    ChangeUnclassified,
}

impl M5TopologyChange {
    /// Every topology change, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MonitorAttach,
        Self::MonitorDetach,
        Self::Undock,
        Self::DpiChange,
        Self::FullscreenTransition,
        Self::SnappedLayoutRecovery,
        Self::ChangeUnclassified,
    ];

    /// The six canonical topology changes every claimed M5 surface remaps through the shared registry.
    pub const CANONICAL_CHANGES: [Self; 6] = [
        Self::MonitorAttach,
        Self::MonitorDetach,
        Self::Undock,
        Self::DpiChange,
        Self::FullscreenTransition,
        Self::SnappedLayoutRecovery,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MonitorAttach => "monitor_attach",
            Self::MonitorDetach => "monitor_detach",
            Self::Undock => "undock",
            Self::DpiChange => "dpi_change",
            Self::FullscreenTransition => "fullscreen_transition",
            Self::SnappedLayoutRecovery => "snapped_layout_recovery",
            Self::ChangeUnclassified => "change_unclassified",
        }
    }

    /// Whether the topology change is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ChangeUnclassified)
    }

    /// Whether this change is a mixed-DPI transition (used by the mixed-DPI acceptance-criteria gate).
    pub const fn is_dpi_change(self) -> bool {
        matches!(self, Self::DpiChange)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a restore or remap
/// token's meaning stays stable whether it appears in the shell, editor, review, notebook, or data surface.
/// Minted by this lane, tracking the first-consumer surfaces the implementation requirements name directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemapSurfaceContext {
    /// The shell surface.
    Shell,
    /// The editor surface.
    Editor,
    /// The review surface.
    Review,
    /// The notebook surface.
    Notebook,
    /// The data surface.
    Data,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5RemapSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirements name.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Editor => "editor",
            Self::Review => "review",
            Self::Notebook => "notebook",
            Self::Data => "data",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// The fidelity outcome of a geometry remap, in descending fidelity order: an exact restore is highest
/// fidelity, a proportional-intent remap or monitor-affinity fallback reduces fidelity, and a recenter / reset
/// is the lowest-fidelity failure-safe recovery. Minted by this lane. A reduced-fidelity outcome must be
/// surfaced as recoverable product truth (a recenter / reset affordance) rather than a silent regression.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemapFidelityOutcome {
    /// The exact persisted bounds were restored (full fidelity).
    ExactBoundsRestored,
    /// Layout intent and proportions were remapped onto the new topology (reduced fidelity).
    ProportionalIntentRemap,
    /// The monitor-affinity hint could not be honored; the surface fell back to a nearby monitor (reduced
    /// fidelity).
    MonitorAffinityFallback,
    /// The surface was recentered / reset into visible bounds (lowest-fidelity failure-safe recovery).
    RecenterReset,
    /// The fidelity outcome is unclassified, which is disallowed.
    OutcomeUnclassified,
}

impl M5RemapFidelityOutcome {
    /// Every fidelity outcome, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExactBoundsRestored,
        Self::ProportionalIntentRemap,
        Self::MonitorAffinityFallback,
        Self::RecenterReset,
        Self::OutcomeUnclassified,
    ];

    /// The four canonical fidelity outcomes the provenance registry records across surfaces.
    pub const CANONICAL_OUTCOMES: [Self; 4] = [
        Self::ExactBoundsRestored,
        Self::ProportionalIntentRemap,
        Self::MonitorAffinityFallback,
        Self::RecenterReset,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBoundsRestored => "exact_bounds_restored",
            Self::ProportionalIntentRemap => "proportional_intent_remap",
            Self::MonitorAffinityFallback => "monitor_affinity_fallback",
            Self::RecenterReset => "recenter_reset",
            Self::OutcomeUnclassified => "outcome_unclassified",
        }
    }

    /// Whether the fidelity outcome is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::OutcomeUnclassified)
    }

    /// Whether this outcome reduced fidelity below an exact restore and so must offer a recenter / reset
    /// affordance and be recorded as recoverable product truth.
    pub const fn is_reduced_fidelity(self) -> bool {
        matches!(
            self,
            Self::ProportionalIntentRemap | Self::MonitorAffinityFallback | Self::RecenterReset
        )
    }
}

/// One provenance field a geometry-remap event records so support and restore diagnostics can explain why
/// fidelity changed. Minted by this lane; the mandatory subset is the minimum a diagnosable remap must record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RemapProvenanceField {
    /// The topology change that triggered the remap.
    RemapTrigger,
    /// The monitor topology before the remap.
    FromMonitorTopology,
    /// The monitor topology after the remap.
    ToMonitorTopology,
    /// The DPI / scale factor before the remap.
    FromDpiScale,
    /// The DPI / scale factor after the remap.
    ToDpiScale,
    /// The fidelity outcome of the remap.
    FidelityOutcome,
    /// Whether a visible-bounds clamp was applied.
    VisibleBoundsClampApplied,
    /// Whether a recenter / reset affordance was offered.
    RecenterAffordanceOffered,
    /// Whether the workspace, focus chain, and critical state were preserved.
    PreservedWorkspaceState,
}

impl M5RemapProvenanceField {
    /// Every provenance field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RemapTrigger,
        Self::FromMonitorTopology,
        Self::ToMonitorTopology,
        Self::FromDpiScale,
        Self::ToDpiScale,
        Self::FidelityOutcome,
        Self::VisibleBoundsClampApplied,
        Self::RecenterAffordanceOffered,
        Self::PreservedWorkspaceState,
    ];

    /// The five mandatory provenance fields a diagnosable remap must record.
    pub const MANDATORY: [Self; 5] = [
        Self::RemapTrigger,
        Self::ToMonitorTopology,
        Self::ToDpiScale,
        Self::FidelityOutcome,
        Self::PreservedWorkspaceState,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RemapTrigger => "remap_trigger",
            Self::FromMonitorTopology => "from_monitor_topology",
            Self::ToMonitorTopology => "to_monitor_topology",
            Self::FromDpiScale => "from_dpi_scale",
            Self::ToDpiScale => "to_dpi_scale",
            Self::FidelityOutcome => "fidelity_outcome",
            Self::VisibleBoundsClampApplied => "visible_bounds_clamp_applied",
            Self::RecenterAffordanceOffered => "recenter_affordance_offered",
            Self::PreservedWorkspaceState => "preserved_workspace_state",
        }
    }
}

/// One mandatory rendered part a restore-bounds or remap-provenance entry must be able to show, so no restore,
/// clamp, remap, or provenance fact is left implicit behind a private geometry rule.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The restore-surface kind the entry maps (restore-bounds entry).
    RestoreSurfaceKind,
    /// The topology change the entry maps (both entries).
    TopologyChange,
    /// The visible-bounds clamp applied (restore-bounds entry).
    VisibleBoundsClamp,
    /// The persisted monitor-affinity hint (restore-bounds entry).
    MonitorAffinityHint,
    /// The fidelity outcome of the remap (both entries).
    FidelityOutcome,
    /// The recorded remap provenance (remap-provenance entry).
    RemapProvenance,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the restore or remap (both entries).
    PlainLanguageMeaning,
}

impl M5RestoreRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RestoreSurfaceKind,
        Self::TopologyChange,
        Self::VisibleBoundsClamp,
        Self::MonitorAffinityHint,
        Self::FidelityOutcome,
        Self::RemapProvenance,
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
            Self::RestoreSurfaceKind => "restore_surface_kind",
            Self::TopologyChange => "topology_change",
            Self::VisibleBoundsClamp => "visible_bounds_clamp",
            Self::MonitorAffinityHint => "monitor_affinity_hint",
            Self::FidelityOutcome => "fidelity_outcome",
            Self::RemapProvenance => "remap_provenance",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a restore
/// entry, recenter a window, trace a remap, or review a degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreRegistryNextAction {
    /// Expand the restore / remap's plain-language meaning.
    ExpandRemapMeaning,
    /// Inspect the restore-bounds entry or the recorded remap provenance.
    InspectRestoreOrProvenance,
    /// Recenter or reset the window into visible bounds.
    RecenterOrResetWindow,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5RestoreRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRemapMeaning,
        Self::InspectRestoreOrProvenance,
        Self::RecenterOrResetWindow,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRemapMeaning => "expand_remap_meaning",
            Self::InspectRestoreOrProvenance => "inspect_restore_or_provenance",
            Self::RecenterOrResetWindow => "recenter_or_reset_window",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The geometry families covered.
    GeometryFamilies,
    /// The topology changes covered.
    TopologyChanges,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The restore-surface kinds carried.
    RestoreSurfaceKinds,
    /// The fidelity outcomes carried.
    FidelityOutcomes,
    /// The provenance fields carried.
    ProvenanceFields,
    /// The render / surface context.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RestoreRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::TopologyChanges,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::RestoreSurfaceKinds,
        Self::FidelityOutcomes,
        Self::ProvenanceFields,
        Self::SurfaceContext,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::TopologyChanges,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::GeometryFamilies => "geometry_families",
            Self::TopologyChanges => "topology_changes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::RestoreSurfaceKinds => "restore_surface_kinds",
            Self::FidelityOutcomes => "fidelity_outcomes",
            Self::ProvenanceFields => "provenance_fields",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a restore-bounds entry degraded below a clean, on-screen-continuity-preserving state. The
/// degrade-first ladder returns one of these instead of ever letting an off-screen, focus-trapping,
/// unclamped, unusable, stale-coordinate, or affordance-missing entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreBoundsEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the restore entry means.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The restore-surface kind is unclassified (not in the preserved taxonomy).
    SurfaceKindUnclassified,
    /// The topology change is unclassified (not in the preserved taxonomy).
    TopologyChangeUnclassified,
    /// The restored surface reopened fully off-screen after the topology change.
    ReopensFullyOffScreen,
    /// The restored surface trapped focus after the topology change.
    TrapsFocusAfterRemap,
    /// The restored surface was not clamped into visible bounds.
    NotClampedIntoVisibleBounds,
    /// The restore lost usable editor / panel / inspector geometry.
    LosesUsableGeometry,
    /// The restore replayed stale absolute coordinates instead of resolving persisted layout intent and
    /// monitor-affinity hints.
    ReplaysStaleAbsoluteCoordinates,
    /// Fidelity was reduced but no recenter / reset affordance was offered.
    NoRecenterResetAffordance,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RestoreBoundsEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::SurfaceKindUnclassified,
        Self::TopologyChangeUnclassified,
        Self::ReopensFullyOffScreen,
        Self::TrapsFocusAfterRemap,
        Self::NotClampedIntoVisibleBounds,
        Self::LosesUsableGeometry,
        Self::ReplaysStaleAbsoluteCoordinates,
        Self::NoRecenterResetAffordance,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SurfaceKindUnclassified => "surface_kind_unclassified",
            Self::TopologyChangeUnclassified => "topology_change_unclassified",
            Self::ReopensFullyOffScreen => "reopens_fully_off_screen",
            Self::TrapsFocusAfterRemap => "traps_focus_after_remap",
            Self::NotClampedIntoVisibleBounds => "not_clamped_into_visible_bounds",
            Self::LosesUsableGeometry => "loses_usable_geometry",
            Self::ReplaysStaleAbsoluteCoordinates => "replays_stale_absolute_coordinates",
            Self::NoRecenterResetAffordance => "no_recenter_reset_affordance",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RestoreRegistryNextAction {
        match self {
            Self::TokenUnstated => M5RestoreRegistryNextAction::TraceCanonicalRegistry,
            Self::ReopensFullyOffScreen
            | Self::TrapsFocusAfterRemap
            | Self::NotClampedIntoVisibleBounds
            | Self::NoRecenterResetAffordance => M5RestoreRegistryNextAction::RecenterOrResetWindow,
            Self::SurfaceKindUnclassified
            | Self::TopologyChangeUnclassified
            | Self::LosesUsableGeometry
            | Self::ReplaysStaleAbsoluteCoordinates => {
                M5RestoreRegistryNextAction::InspectRestoreOrProvenance
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RestoreRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated | Self::SurfaceContextUnresolved => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceKindUnclassified | Self::TopologyChangeUnclassified => {
                M5ShellGeometryDowngradeTrigger::ResponsiveClassUnstated
            }
            Self::ReopensFullyOffScreen
            | Self::TrapsFocusAfterRemap
            | Self::NotClampedIntoVisibleBounds => {
                M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState
            }
            Self::LosesUsableGeometry => M5ShellGeometryDowngradeTrigger::ZoneStarvedMainWorkspace,
            Self::ReplaysStaleAbsoluteCoordinates => {
                M5ShellGeometryDowngradeTrigger::MetricCopiedByHandAcrossPackages
            }
            Self::NoRecenterResetAffordance => {
                M5ShellGeometryDowngradeTrigger::PrimaryWorkflowHiddenBehindOverlayOnlyFallback
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a geometry-remap-provenance entry degraded below a clean, diagnosable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryRemapProvenanceEntryDegradeReason {
    /// The canonical registry token name is unstated.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The remap trigger (topology change) is unclassified (not in the preserved taxonomy).
    TriggerUnclassified,
    /// The fidelity outcome is unclassified (not in the preserved taxonomy).
    FidelityOutcomeUnclassified,
    /// The remap silently dropped the workspace, focus chain, or critical state.
    SilentlyDropsWorkspaceOrState,
    /// The remap dropped the workspace, focus chain, or recovery-critical state (surface continuity broken).
    DropsWorkspaceFocusOrCriticalState,
    /// The remap reason was not recorded in provenance.
    RemapReasonUnrecorded,
    /// The recorded provenance omits a mandatory field and cannot diagnose why fidelity changed.
    ProvenanceDetailIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5GeometryRemapProvenanceEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::TriggerUnclassified,
        Self::FidelityOutcomeUnclassified,
        Self::SilentlyDropsWorkspaceOrState,
        Self::DropsWorkspaceFocusOrCriticalState,
        Self::RemapReasonUnrecorded,
        Self::ProvenanceDetailIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::TriggerUnclassified => "trigger_unclassified",
            Self::FidelityOutcomeUnclassified => "fidelity_outcome_unclassified",
            Self::SilentlyDropsWorkspaceOrState => "silently_drops_workspace_or_state",
            Self::DropsWorkspaceFocusOrCriticalState => "drops_workspace_focus_or_critical_state",
            Self::RemapReasonUnrecorded => "remap_reason_unrecorded",
            Self::ProvenanceDetailIncomplete => "provenance_detail_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RestoreRegistryNextAction {
        match self {
            Self::TokenUnstated => M5RestoreRegistryNextAction::TraceCanonicalRegistry,
            Self::TriggerUnclassified
            | Self::FidelityOutcomeUnclassified
            | Self::SilentlyDropsWorkspaceOrState
            | Self::DropsWorkspaceFocusOrCriticalState
            | Self::RemapReasonUnrecorded
            | Self::ProvenanceDetailIncomplete => {
                M5RestoreRegistryNextAction::InspectRestoreOrProvenance
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RestoreRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::TriggerUnclassified
            | Self::FidelityOutcomeUnclassified
            | Self::RemapReasonUnrecorded
            | Self::ProvenanceDetailIncomplete => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SilentlyDropsWorkspaceOrState | Self::DropsWorkspaceFocusOrCriticalState => {
                M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_restore_bounds_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RestoreBoundsEntryResolutionInput {
    /// Stable identity of the restore-bounds entry.
    pub entry_id: String,
    /// The canonical registry token name (e.g. `shell.restore.window.bounds`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The responsive-geometry role (from the frozen matrix vocabulary).
    pub responsive_geometry_role: M5ResponsiveGeometryRole,
    /// The restore-surface kind this entry maps.
    pub restore_surface_kind: M5RestoreSurfaceKind,
    /// The topology change this entry restores across.
    pub topology_change: M5TopologyChange,
    /// The render / surface context.
    pub surface_context: M5RemapSurfaceContext,
    /// The fidelity outcome of the restore.
    pub fidelity_outcome: M5RemapFidelityOutcome,
    /// True when the restored surface reopened fully off-screen (a hard invariant when `true`).
    pub reopens_fully_off_screen: bool,
    /// True when the restored surface trapped focus after the remap (a hard invariant when `true`).
    pub traps_focus_after_remap: bool,
    /// True when the restored surface was clamped into visible bounds.
    pub clamped_into_visible_bounds: bool,
    /// True when the restore preserved usable editor / panel / inspector geometry.
    pub preserves_usable_geometry: bool,
    /// True when the restore replayed stale absolute coordinates instead of persisted intent (a hard invariant
    /// when `true`).
    pub uses_absolute_coordinates_instead_of_intent: bool,
    /// True when a recenter / reset affordance was offered.
    pub offers_recenter_reset_affordance: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe restore-bounds projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRestoreBoundsEntry {
    /// Stable identity of the restore-bounds entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve task identity when density changes or the layout collapses.
    pub semantic_role_preserves_task_identity_under_collapse: bool,
    /// The responsive-geometry-role token named by the entry.
    pub responsive_geometry_role: String,
    /// Whether the responsive-geometry role names the disallowed drops-recovery-state token.
    pub responsive_role_drops_recovery_state: bool,
    /// The restore-surface-kind token named by the entry.
    pub restore_surface_kind: String,
    /// Whether the restore-surface kind is classified into the preserved taxonomy.
    pub restore_surface_kind_is_classified: bool,
    /// The topology-change token named by the entry.
    pub topology_change: String,
    /// Whether the topology change is classified into the preserved taxonomy.
    pub topology_change_is_classified: bool,
    /// Whether the topology change is a mixed-DPI transition.
    pub topology_is_dpi_change: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The fidelity-outcome token named by the entry.
    pub fidelity_outcome: String,
    /// Whether the fidelity outcome reduced fidelity below an exact restore.
    pub fidelity_is_reduced: bool,
    /// Whether the restored surface reopened fully off-screen.
    pub reopens_fully_off_screen: bool,
    /// Whether the restored surface trapped focus after the remap.
    pub traps_focus_after_remap: bool,
    /// Whether the restored surface was clamped into visible bounds.
    pub clamped_into_visible_bounds: bool,
    /// Whether the restore preserved usable editor / panel / inspector geometry.
    pub preserves_usable_geometry: bool,
    /// Whether the restore replayed stale absolute coordinates instead of persisted intent.
    pub uses_absolute_coordinates_instead_of_intent: bool,
    /// Whether a recenter / reset affordance was offered.
    pub offers_recenter_reset_affordance: bool,
    /// Degrade reason, if the entry could not read as a clean, on-screen-continuity-preserving state.
    pub degrade_reason: Option<M5RestoreBoundsEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RestoreRegistryNextAction,
    /// Whether the restore preserves on-screen continuity (clean entry naming every fact).
    pub restore_preserves_on_screen_continuity: bool,
}

impl M5ResolvedRestoreBoundsEntry {
    /// Whether this restore-bounds entry reads as a clean, on-screen-continuity-preserving state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_geometry_remap_provenance_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GeometryRemapProvenanceEntryResolutionInput {
    /// Stable identity of the remap-provenance entry.
    pub entry_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The responsive-geometry role (from the frozen matrix vocabulary).
    pub responsive_geometry_role: M5ResponsiveGeometryRole,
    /// The topology change that triggered the remap.
    pub topology_change: M5TopologyChange,
    /// The fidelity outcome recorded for the remap.
    pub fidelity_outcome: M5RemapFidelityOutcome,
    /// The render / surface context.
    pub surface_context: M5RemapSurfaceContext,
    /// The provenance fields recorded for the remap (must cover the mandatory set).
    pub recorded_provenance_fields: Vec<M5RemapProvenanceField>,
    /// True when the remap preserved the workspace, focus chain, and critical state.
    pub preserves_workspace_focus_and_critical_state: bool,
    /// True when the remap reason was recorded in provenance.
    pub records_remap_reason: bool,
    /// True when the remap silently dropped workspace / focus / state (a hard invariant when `true`).
    pub silently_drops_workspace_or_state: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe geometry-remap-provenance projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGeometryRemapProvenanceEntry {
    /// Stable identity of the remap-provenance entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve task identity when density changes or the layout collapses.
    pub semantic_role_preserves_task_identity_under_collapse: bool,
    /// The responsive-geometry-role token named by the entry.
    pub responsive_geometry_role: String,
    /// Whether the responsive-geometry role names the disallowed drops-recovery-state token.
    pub responsive_role_drops_recovery_state: bool,
    /// The topology-change token named by the entry.
    pub topology_change: String,
    /// Whether the topology change is classified into the preserved taxonomy.
    pub topology_change_is_classified: bool,
    /// The fidelity-outcome token named by the entry.
    pub fidelity_outcome: String,
    /// Whether the fidelity outcome is classified into the preserved taxonomy.
    pub fidelity_outcome_is_classified: bool,
    /// Whether the fidelity outcome reduced fidelity below an exact restore.
    pub fidelity_is_reduced: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The provenance-field tokens recorded by the entry.
    pub recorded_provenance_fields: Vec<String>,
    /// Whether the entry records every mandatory provenance field.
    pub records_mandatory_provenance: bool,
    /// Whether the remap preserved the workspace, focus chain, and critical state.
    pub preserves_workspace_focus_and_critical_state: bool,
    /// Whether the remap reason was recorded in provenance.
    pub records_remap_reason: bool,
    /// Whether the remap silently dropped workspace / focus / state.
    pub silently_drops_workspace_or_state: bool,
    /// Degrade reason, if the entry could not read as a clean, diagnosable state.
    pub degrade_reason: Option<M5GeometryRemapProvenanceEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RestoreRegistryNextAction,
    /// Whether the recorded provenance is diagnosable (clean entry naming every fact).
    pub provenance_is_diagnosable: bool,
}

impl M5ResolvedGeometryRemapProvenanceEntry {
    /// Whether this remap-provenance entry reads as a clean, diagnosable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5MonitorGeometryResolutionError {
    /// The restore-bounds-entry id was empty.
    EmptyRestoreBoundsEntryId,
    /// The remap-provenance-entry id was empty.
    EmptyRemapProvenanceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5MonitorGeometryResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRestoreBoundsEntryId => "empty_restore_bounds_entry_id",
            Self::EmptyRemapProvenanceEntryId => "empty_remap_provenance_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5MonitorGeometryResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 monitor-geometry-remap / restore-bounds registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MonitorGeometryResolutionError {}

fn field_tokens(fields: &[M5RemapProvenanceField]) -> Vec<String> {
    fields.iter().map(|f| f.as_str().to_owned()).collect()
}

fn records_mandatory_provenance(fields: &[M5RemapProvenanceField]) -> bool {
    let present: BTreeSet<M5RemapProvenanceField> = fields.iter().copied().collect();
    M5RemapProvenanceField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// Resolves a restore-bounds entry so a restored window, sheet, dialog, docked panel, or split layout stays
/// on-screen: the entry names its canonical token, semantic role, responsive-geometry role, restore-surface
/// kind, and topology change, clamps into visible bounds, never reopens off-screen or traps focus, preserves
/// usable geometry, persists layout intent instead of stale absolute coordinates, and offers a recenter /
/// reset affordance whenever fidelity is reduced.
pub fn resolve_restore_bounds_entry(
    input: M5RestoreBoundsEntryResolutionInput,
) -> Result<M5ResolvedRestoreBoundsEntry, M5MonitorGeometryResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5MonitorGeometryResolutionError::EmptyRestoreBoundsEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5MonitorGeometryResolutionError::ForbiddenMaterial);
    }

    let role_drops_recovery = matches!(
        input.responsive_geometry_role,
        M5ResponsiveGeometryRole::ResponsiveChangeDropsRecoveryStateDisallowed
    );
    let fidelity_is_reduced = input.fidelity_outcome.is_reduced_fidelity();

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5RestoreBoundsEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5RestoreBoundsEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.restore_surface_kind.is_classified() {
        Some(M5RestoreBoundsEntryDegradeReason::SurfaceKindUnclassified)
    } else if !input.topology_change.is_classified() {
        Some(M5RestoreBoundsEntryDegradeReason::TopologyChangeUnclassified)
    } else if input.reopens_fully_off_screen {
        Some(M5RestoreBoundsEntryDegradeReason::ReopensFullyOffScreen)
    } else if input.traps_focus_after_remap {
        Some(M5RestoreBoundsEntryDegradeReason::TrapsFocusAfterRemap)
    } else if !input.clamped_into_visible_bounds {
        Some(M5RestoreBoundsEntryDegradeReason::NotClampedIntoVisibleBounds)
    } else if role_drops_recovery || !input.preserves_usable_geometry {
        Some(M5RestoreBoundsEntryDegradeReason::LosesUsableGeometry)
    } else if input.uses_absolute_coordinates_instead_of_intent {
        Some(M5RestoreBoundsEntryDegradeReason::ReplaysStaleAbsoluteCoordinates)
    } else if fidelity_is_reduced && !input.offers_recenter_reset_affordance {
        Some(M5RestoreBoundsEntryDegradeReason::NoRecenterResetAffordance)
    } else if !input.proof_fresh {
        Some(M5RestoreBoundsEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestoreRegistryNextAction::ExpandRemapMeaning,
    };

    Ok(M5ResolvedRestoreBoundsEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        responsive_geometry_role: input.responsive_geometry_role.as_str().to_owned(),
        responsive_role_drops_recovery_state: role_drops_recovery,
        restore_surface_kind: input.restore_surface_kind.as_str().to_owned(),
        restore_surface_kind_is_classified: input.restore_surface_kind.is_classified(),
        topology_change: input.topology_change.as_str().to_owned(),
        topology_change_is_classified: input.topology_change.is_classified(),
        topology_is_dpi_change: input.topology_change.is_dpi_change(),
        surface_context: input.surface_context.as_str().to_owned(),
        fidelity_outcome: input.fidelity_outcome.as_str().to_owned(),
        fidelity_is_reduced,
        reopens_fully_off_screen: input.reopens_fully_off_screen,
        traps_focus_after_remap: input.traps_focus_after_remap,
        clamped_into_visible_bounds: input.clamped_into_visible_bounds,
        preserves_usable_geometry: input.preserves_usable_geometry,
        uses_absolute_coordinates_instead_of_intent: input
            .uses_absolute_coordinates_instead_of_intent,
        offers_recenter_reset_affordance: input.offers_recenter_reset_affordance,
        degrade_reason,
        next_action,
        restore_preserves_on_screen_continuity: degrade_reason.is_none(),
    })
}

/// Resolves a geometry-remap-provenance entry so a remap keeps geometry continuity distinct from surface
/// continuity: the entry names its canonical token, responsive-geometry role, topology change, and fidelity
/// outcome, preserves the workspace / focus chain / critical state, records the remap reason, records every
/// mandatory provenance field, and never silently drops workspace or state.
pub fn resolve_geometry_remap_provenance_entry(
    input: M5GeometryRemapProvenanceEntryResolutionInput,
) -> Result<M5ResolvedGeometryRemapProvenanceEntry, M5MonitorGeometryResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5MonitorGeometryResolutionError::EmptyRemapProvenanceEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5MonitorGeometryResolutionError::ForbiddenMaterial);
    }

    let role_drops_recovery = matches!(
        input.responsive_geometry_role,
        M5ResponsiveGeometryRole::ResponsiveChangeDropsRecoveryStateDisallowed
    );
    let records_mandatory = records_mandatory_provenance(&input.recorded_provenance_fields);
    let fidelity_is_reduced = input.fidelity_outcome.is_reduced_fidelity();

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.topology_change.is_classified() {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::TriggerUnclassified)
    } else if !input.fidelity_outcome.is_classified() {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::FidelityOutcomeUnclassified)
    } else if input.silently_drops_workspace_or_state {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::SilentlyDropsWorkspaceOrState)
    } else if role_drops_recovery || !input.preserves_workspace_focus_and_critical_state {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::DropsWorkspaceFocusOrCriticalState)
    } else if !input.records_remap_reason {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::RemapReasonUnrecorded)
    } else if !records_mandatory {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::ProvenanceDetailIncomplete)
    } else if !input.proof_fresh {
        Some(M5GeometryRemapProvenanceEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestoreRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedGeometryRemapProvenanceEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        responsive_geometry_role: input.responsive_geometry_role.as_str().to_owned(),
        responsive_role_drops_recovery_state: role_drops_recovery,
        topology_change: input.topology_change.as_str().to_owned(),
        topology_change_is_classified: input.topology_change.is_classified(),
        fidelity_outcome: input.fidelity_outcome.as_str().to_owned(),
        fidelity_outcome_is_classified: input.fidelity_outcome.is_classified(),
        fidelity_is_reduced,
        surface_context: input.surface_context.as_str().to_owned(),
        recorded_provenance_fields: field_tokens(&input.recorded_provenance_fields),
        records_mandatory_provenance: records_mandatory,
        preserves_workspace_focus_and_critical_state: input
            .preserves_workspace_focus_and_critical_state,
        records_remap_reason: input.records_remap_reason,
        silently_drops_workspace_or_state: input.silently_drops_workspace_or_state,
        degrade_reason,
        next_action,
        provenance_is_diagnosable: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved restore-bounds and remap-provenance entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5MonitorGeometryRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5ShellGeometryQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ShellGeometryDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5ShellGeometryRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5ShellGeometryAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5RestoreRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RestoreRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    /// Resolved restore-bounds examples.
    pub restore_bounds_entries: Vec<M5ResolvedRestoreBoundsEntry>,
    /// Resolved remap-provenance examples.
    pub remap_provenance_entries: Vec<M5ResolvedGeometryRemapProvenanceEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical density-mode domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a restore never reopens off-screen or traps focus. MUST be `false`.
    pub restore_reopens_off_screen_or_traps_focus: bool,
    /// Hard invariant: a remap never replays stale absolute coordinates without a visible-bounds clamp. MUST
    /// be `false`.
    pub remap_replays_stale_absolute_coordinates_without_clamp: bool,
    /// Hard invariant: a remap never silently drops the workspace, focus chain, or critical state. MUST be
    /// `false`.
    pub remap_silently_drops_workspace_focus_or_critical_state: bool,
    /// Hard invariant: reduced fidelity is never left without a recenter / reset affordance or recorded
    /// provenance. MUST be `false`.
    pub reduced_fidelity_without_recenter_or_provenance: bool,
}

impl M5MonitorGeometryRemapAndRestoreBoundsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RestoreRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RestoreRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RestoreRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5RestoreRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.restore_reopens_off_screen_or_traps_focus
            && !self.remap_replays_stale_absolute_coordinates_without_clamp
            && !self.remap_silently_drops_workspace_focus_or_critical_state
            && !self.reduced_fidelity_without_recenter_or_provenance
    }

    /// True when a clean restore-bounds entry preserves on-screen continuity: it keeps a classified kind and
    /// topology change, never names the disallowed drops-recovery role, clamps into visible bounds, never
    /// reopens off-screen or traps focus, preserves usable geometry, never replays stale absolute
    /// coordinates, and offers a recenter / reset affordance whenever fidelity is reduced.
    fn restore_bounds_is_honest(ex: &M5ResolvedRestoreBoundsEntry) -> bool {
        !ex.is_clean()
            || (ex.restore_surface_kind_is_classified
                && ex.topology_change_is_classified
                && !ex.responsive_role_drops_recovery_state
                && ex.clamped_into_visible_bounds
                && !ex.reopens_fully_off_screen
                && !ex.traps_focus_after_remap
                && ex.preserves_usable_geometry
                && !ex.uses_absolute_coordinates_instead_of_intent
                && (!ex.fidelity_is_reduced || ex.offers_recenter_reset_affordance))
    }

    /// True when a clean remap-provenance entry is diagnosable: it keeps a classified topology change and
    /// fidelity outcome, never silently drops workspace or state, preserves the workspace / focus / critical
    /// state, records the remap reason, and records every mandatory provenance field.
    fn provenance_is_honest(ex: &M5ResolvedGeometryRemapProvenanceEntry) -> bool {
        !ex.is_clean()
            || (ex.topology_change_is_classified
                && ex.fidelity_outcome_is_classified
                && !ex.silently_drops_workspace_or_state
                && ex.preserves_workspace_focus_and_critical_state
                && ex.records_remap_reason
                && ex.records_mandatory_provenance)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.restore_bounds_entries
            .iter()
            .all(Self::restore_bounds_is_honest)
            && self
                .remap_provenance_entries
                .iter()
                .all(Self::provenance_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Responsive-geometry-role tokens (bound from the frozen matrix).
    pub responsive_geometry_roles: Vec<String>,
    /// Restore-surface-kind tokens (minted by this lane).
    pub restore_surface_kinds: Vec<String>,
    /// Topology-change tokens (minted by this lane).
    pub topology_changes: Vec<String>,
    /// Fidelity-outcome tokens (minted by this lane).
    pub fidelity_outcomes: Vec<String>,
    /// Provenance-field tokens (minted by this lane).
    pub provenance_fields: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Restore-bounds-entry degrade-reason tokens.
    pub restore_bounds_degrade_reasons: Vec<String>,
    /// Remap-provenance-entry degrade-reason tokens.
    pub remap_provenance_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5MonitorGeometryRemapAndRestoreBoundsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ShellGeometryRole::ALL, |v| v.as_str()),
            responsive_geometry_roles: tokens(&M5ResponsiveGeometryRole::ALL, |v| v.as_str()),
            restore_surface_kinds: tokens(&M5RestoreSurfaceKind::ALL, |v| v.as_str()),
            topology_changes: tokens(&M5TopologyChange::ALL, |v| v.as_str()),
            fidelity_outcomes: tokens(&M5RemapFidelityOutcome::ALL, |v| v.as_str()),
            provenance_fields: tokens(&M5RemapProvenanceField::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RemapSurfaceContext::ALL, |v| v.as_str()),
            restore_bounds_degrade_reasons: tokens(&M5RestoreBoundsEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            remap_provenance_degrade_reasons: tokens(
                &M5GeometryRemapProvenanceEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5RestoreRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RestoreRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RestoreRegistryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ShellGeometryConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5MonitorGeometryRemapAndRestoreBoundsGovernanceReview {
    /// The registry names a canonical token, role, and topology change for every entry.
    pub registry_names_token_role_and_topology_change: bool,
    /// Every restored window, sheet, and dialog is clamped into visible bounds.
    pub restore_clamps_every_surface_into_visible_bounds: bool,
    /// No restored window or approval sheet reopens fully off-screen or traps focus.
    pub no_restored_window_or_sheet_reopens_off_screen_or_traps_focus: bool,
    /// Restore persists layout intent and monitor-affinity hints instead of brittle absolute coordinates.
    pub persists_layout_intent_and_monitor_affinity_not_absolute_coordinates: bool,
    /// Mixed-DPI and topology-change drills preserve usable editor / panel / inspector geometry.
    pub mixed_dpi_and_topology_drills_preserve_usable_geometry: bool,
    /// Reduced fidelity offers a one-click or command-backed recenter / reset affordance.
    pub reduced_fidelity_offers_recenter_or_reset_affordance: bool,
    /// Geometry continuity stays distinct from surface continuity.
    pub geometry_continuity_distinct_from_surface_continuity: bool,
    /// The workspace, focus chain, and critical state are preserved across every remap.
    pub workspace_focus_and_critical_state_preserved_across_remap: bool,
    /// Remap provenance records enough detail to diagnose why fidelity changed.
    pub remap_provenance_records_enough_detail_to_diagnose: bool,
    /// Every claimed surface resolves its restore geometry from the shared registry.
    pub every_surface_resolves_from_shared_registry: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsConsumerProjection {
    /// The shell surface consumes the shared restore registries.
    pub shell_consumes_shared_registries: bool,
    /// The editor surface consumes the shared restore registries.
    pub editor_consumes_shared_registries: bool,
    /// The review surface consumes the shared restore registries.
    pub review_consumes_shared_registries: bool,
    /// The notebook and data surfaces consume the shared restore registries.
    pub notebook_and_data_consume_shared_registries: bool,
    /// Restore geometry resolves back to one canonical density-mode domain contract.
    pub geometry_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical restore registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting shell-geometry audit for the lane.
    pub geometry_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MonitorGeometryRemapAndRestoreBoundsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5MonitorGeometryRemapAndRestoreBoundsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MonitorGeometryRemapAndRestoreBoundsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MonitorGeometryRemapAndRestoreBoundsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MonitorGeometryRemapAndRestoreBoundsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MonitorGeometryRemapAndRestoreBoundsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MonitorGeometryRemapAndRestoreBoundsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 monitor-topology geometry-remap / restore-bounds registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MonitorGeometryRemapAndRestoreBoundsPacket {
    /// Record kind; must equal [`M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5MonitorGeometryRemapAndRestoreBoundsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MonitorGeometryRemapAndRestoreBoundsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MonitorGeometryRemapAndRestoreBoundsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MonitorGeometryRemapAndRestoreBoundsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MonitorGeometryRemapAndRestoreBoundsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MonitorGeometryRemapAndRestoreBoundsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MonitorGeometryRemapAndRestoreBoundsPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5MonitorGeometryRemapAndRestoreBoundsPacketInput) -> Self {
        Self {
            record_kind: M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_RECORD_KIND.to_owned(),
            schema_version: M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_RECORD_KIND {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_VERSION {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 monitor-geometry-remap / restore-bounds registries packet serializes"),
        ) {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::RawMaterialInExport);
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
            .expect("m5 monitor-geometry-remap / restore-bounds registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,restore_bounds_entries,remap_provenance_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .restore_bounds_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.remap_provenance_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.restore_bounds_entries.len(),
                row.remap_provenance_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Monitor-Topology Geometry-Remap and Restore-Bounds Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Topology changes: {}\n",
            self.vocabulary_set.topology_changes.join(", ")
        ));
        out.push_str(&format!(
            "- Fidelity outcomes: {}\n",
            self.vocabulary_set.fidelity_outcomes.join(", ")
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
                "  - Restore-bounds entries: {} / remap-provenance entries: {}\n",
                row.restore_bounds_entries.len(),
                row.remap_provenance_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5MonitorGeometryRemapAndRestoreBoundsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>),
}

impl fmt::Display for M5MonitorGeometryRemapAndRestoreBoundsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 monitor-geometry-remap / restore-bounds registries export parse failed: {error}"
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
                    "m5 monitor-geometry-remap / restore-bounds registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MonitorGeometryRemapAndRestoreBoundsArtifactError {}

/// Validation failures emitted by [`M5MonitorGeometryRemapAndRestoreBoundsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MonitorGeometryRemapAndRestoreBoundsViolation {
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
    /// A registry row does not point at the canonical density-mode domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (off-screen, focus-trapping, unclamped, unusable,
    /// stale-coordinate, silent-drop, or provenance-losing).
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
    /// No-off-screen / no-focus-trap continuity is not proven: clean restore entries do not cover the
    /// canonical topology changes or restore-surface kinds, no off-screen or focus-trap example degrades, or a
    /// clean entry reopens off-screen or traps focus.
    NoOffScreenOrFocusTrapNotProven,
    /// Mixed-DPI usable geometry is not proven: clean restore entries do not cover the DPI-change trigger or
    /// the first shell / editor / review / notebook / data surfaces, no loses-usable-geometry example
    /// degrades, no clean reduced-fidelity restore surfaces a recoverable recenter affordance, or a clean
    /// entry loses usable geometry.
    MixedDpiUsableGeometryNotProven,
    /// Remap provenance is not proven recordable: clean provenance entries do not cover the canonical fidelity
    /// outcomes, no provenance-incomplete or reason-unrecorded example degrades, or a clean entry silently
    /// drops workspace or state.
    RemapProvenanceRecordedNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MonitorGeometryRemapAndRestoreBoundsViolation {
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
            Self::NoOffScreenOrFocusTrapNotProven => "no_off_screen_or_focus_trap_not_proven",
            Self::MixedDpiUsableGeometryNotProven => "mixed_dpi_usable_geometry_not_proven",
            Self::RemapProvenanceRecordedNotProven => "remap_provenance_recorded_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_monitor_geometry_remap_and_restore_bounds_export() -> Result<
    M5MonitorGeometryRemapAndRestoreBoundsPacket,
    M5MonitorGeometryRemapAndRestoreBoundsArtifactError,
> {
    let packet: M5MonitorGeometryRemapAndRestoreBoundsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-monitor-geometry-remap-and-restore-bounds-proof/support_export.json"
        )))
        .map_err(M5MonitorGeometryRemapAndRestoreBoundsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MonitorGeometryRemapAndRestoreBoundsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_SCHEMA_REF,
        M5_MONITOR_GEOMETRY_REMAP_AND_RESTORE_BOUNDS_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::NoRegistryRows);
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
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations
                .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DENSITY_MODE_SCHEMA_REF) {
            violations
                .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::DomainSchemaRefMissing);
        }
        if row.restore_bounds_entries.is_empty() || row.remap_provenance_entries.is_empty() {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.registry_names_token_role_and_topology_change,
        review.restore_clamps_every_surface_into_visible_bounds,
        review.no_restored_window_or_sheet_reopens_off_screen_or_traps_focus,
        review.persists_layout_intent_and_monitor_affinity_not_absolute_coordinates,
        review.mixed_dpi_and_topology_drills_preserve_usable_geometry,
        review.reduced_fidelity_offers_recenter_or_reset_affordance,
        review.geometry_continuity_distinct_from_surface_continuity,
        review.workspace_focus_and_critical_state_preserved_across_remap,
        review.remap_provenance_records_enough_detail_to_diagnose,
        review.every_surface_resolves_from_shared_registry,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.editor_consumes_shared_registries,
        projection.review_consumes_shared_registries,
        projection.notebook_and_data_consume_shared_registries,
        projection.geometry_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5MonitorGeometryRemapAndRestoreBoundsViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.geometry_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MonitorGeometryRemapAndRestoreBoundsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5MonitorGeometryRemapAndRestoreBoundsPacket,
    violations: &mut Vec<M5MonitorGeometryRemapAndRestoreBoundsViolation>,
) {
    let restores = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.restore_bounds_entries.iter())
    };
    let provenances = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.remap_provenance_entries.iter())
    };

    // AC1: no restored window or approval sheet reopens fully off-screen or traps focus after a monitor or DPI
    // change. Clean restore entries cover the canonical topology changes and restore-surface kinds, an
    // off-screen and a focus-trap example degrade, and no clean entry reopens off-screen or traps focus.
    let clean_changes: BTreeSet<String> = restores()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.topology_change.clone())
        .collect();
    let clean_kinds: BTreeSet<String> = restores()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.restore_surface_kind.clone())
        .collect();
    let changes_covered = M5TopologyChange::CANONICAL_CHANGES
        .iter()
        .all(|c| clean_changes.contains(c.as_str()));
    let kinds_covered = M5RestoreSurfaceKind::CANONICAL_KINDS
        .iter()
        .all(|k| clean_kinds.contains(k.as_str()));
    let off_screen_degrades = restores().any(|ex| {
        ex.degrade_reason == Some(M5RestoreBoundsEntryDegradeReason::ReopensFullyOffScreen)
    });
    let focus_trap_degrades = restores().any(|ex| {
        ex.degrade_reason == Some(M5RestoreBoundsEntryDegradeReason::TrapsFocusAfterRemap)
    });
    let no_clean_off_screen_or_trap = !restores()
        .any(|ex| ex.is_clean() && (ex.reopens_fully_off_screen || ex.traps_focus_after_remap));
    if !(changes_covered
        && kinds_covered
        && off_screen_degrades
        && focus_trap_degrades
        && no_clean_off_screen_or_trap)
    {
        violations
            .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::NoOffScreenOrFocusTrapNotProven);
    }

    // AC2: mixed-DPI and topology-change drills preserve usable editor / panel / inspector geometry while
    // surfacing any unavoidable fallback as recoverable product truth. Clean restore entries cover the
    // DPI-change trigger and the first shell / editor / review / notebook / data surfaces, a
    // loses-usable-geometry example degrades, at least one clean reduced-fidelity restore surfaces a
    // recoverable recenter affordance, and no clean entry loses usable geometry.
    let clean_surfaces: BTreeSet<String> = restores()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let dpi_covered = restores().any(|ex| ex.is_clean() && ex.topology_is_dpi_change);
    let first_surfaces_covered = M5RemapSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let loses_geometry_degrades = restores().any(|ex| {
        ex.degrade_reason == Some(M5RestoreBoundsEntryDegradeReason::LosesUsableGeometry)
    });
    let clean_recoverable_fallback = restores()
        .any(|ex| ex.is_clean() && ex.fidelity_is_reduced && ex.offers_recenter_reset_affordance);
    let no_clean_loses_geometry =
        !restores().any(|ex| ex.is_clean() && !ex.preserves_usable_geometry);
    if !(dpi_covered
        && first_surfaces_covered
        && loses_geometry_degrades
        && clean_recoverable_fallback
        && no_clean_loses_geometry)
    {
        violations
            .push(M5MonitorGeometryRemapAndRestoreBoundsViolation::MixedDpiUsableGeometryNotProven);
    }

    // AC3: restore and support exports record geometry-remap events with enough detail to diagnose why
    // fidelity changed. Clean provenance entries cover the canonical fidelity outcomes, a provenance-incomplete
    // and a reason-unrecorded example degrade, and no clean provenance entry silently drops workspace or state.
    let clean_outcomes: BTreeSet<String> = provenances()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.fidelity_outcome.clone())
        .collect();
    let outcomes_covered = M5RemapFidelityOutcome::CANONICAL_OUTCOMES
        .iter()
        .all(|o| clean_outcomes.contains(o.as_str()));
    let provenance_incomplete_degrades = provenances().any(|ex| {
        ex.degrade_reason
            == Some(M5GeometryRemapProvenanceEntryDegradeReason::ProvenanceDetailIncomplete)
    });
    let reason_unrecorded_degrades = provenances().any(|ex| {
        ex.degrade_reason
            == Some(M5GeometryRemapProvenanceEntryDegradeReason::RemapReasonUnrecorded)
    });
    let no_clean_silent_drop =
        !provenances().any(|ex| ex.is_clean() && ex.silently_drops_workspace_or_state);
    if !(outcomes_covered
        && provenance_incomplete_degrades
        && reason_unrecorded_degrades
        && no_clean_silent_drop)
    {
        violations.push(
            M5MonitorGeometryRemapAndRestoreBoundsViolation::RemapProvenanceRecordedNotProven,
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

/// The shell-geometry family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5ShellGeometryFamily; 1] =
    [M5ShellGeometryFamily::ResponsiveGeometry];
