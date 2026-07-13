//! Implemented M5 shell-metric and minimum-size registries.
//!
//! The frozen [shell-metric / density matrix][matrix] names Aureline's five shell-geometry families and
//! locks their controlled vocabulary. This module is the first implement lane over that matrix: it turns
//! the two families that carry the concrete *size* grammar — the **shell metric** (title / context bar,
//! activity rail, sidebar, main editor group, right inspector, bottom panel, and status bar default /
//! minimum / recommended / maximum sizes) and the **minimum size** guards (tab minimum width, resize-handle
//! hit area, and icon-only control hit targets) — into registry resolvers that produce export-safe, honest
//! projections. A user can then trust that every claimed M5 shell surface resolves its geometry from one
//! shared metric registry rather than a hand-copied constant, that the main editor group stays dominant and
//! is never starved below its minimum, that hit targets never shrink below the supported minimum under any
//! density mode, and that a metric drifting outside the canonical B138 envelope degrades honestly instead of
//! reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Encode the reference metrics as logical-pixel contracts before OS scaling.** [`resolve_shell_metric_entry`]
//!   refuses to read as a clean, registry-bound metric entry unless it names a canonical registry token, a
//!   classified [shell zone][M5ShellZone], a shell-metric role, covers every [density mode][M5ShellDensityMode],
//!   declares logical-pixel bounds that stay inside the zone's canonical envelope, never starves the main
//!   workspace, and preserves task identity under snapped widths; otherwise it degrades.
//! * **Enforce collapse-before-break and minimum-size rules so hit targets never shrink below the supported
//!   minimum.** [`resolve_minimum_size_entry`] names a classified [control class][M5ShellControlClass],
//!   requires the declared minimum hit dimension to meet the control's canonical minimum, requires the
//!   target to be reachable by pointer and keyboard, covers every density mode, and degrades to
//!   [`M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum`] when a target drops below its
//!   supported minimum.
//! * **Wire first shell, editor, review, notebook, and data consumers plus fixtures that catch metric
//!   drift.** Each registry row carries the render [surface context][M5ShellSurfaceContext] so a
//!   drift-outside-envelope or below-minimum regression degrades honestly, and the acceptance-criteria gate
//!   proves that drift is caught before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ShellGeometryRole`] role vocabulary,
//! the [`M5ShellMetricRole`] shell-metric-role vocabulary, and the [`M5MinimumSizeRole`] minimum-size-role
//! vocabulary — so shell, editor, review, notebook, data, and support surfaces can never fork their own
//! shell-metric or hit-target meaning. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_shell_metric_density_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_metric_minimum_size_registries,
    seeded_m5_shell_metric_minimum_size_registries_data_ui_preview_narrowed,
    seeded_m5_shell_metric_minimum_size_registries_editor_ui_beta_narrowed,
    M5_SHELL_METRIC_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shell_metric_density_matrix::{
    M5MinimumSizeRole, M5ShellGeometryAccessibilityRoute, M5ShellGeometryConsumerSurface,
    M5ShellGeometryDeploymentLine, M5ShellGeometryDowngradeTrigger, M5ShellGeometryFamily,
    M5ShellGeometryQualificationClass, M5ShellGeometryRequiredLabel, M5ShellGeometryRole,
    M5ShellMetricRole, M5_SHELL_METRICS_SCHEMA_REF, M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
    M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ShellMetricRegistriesPacket`].
pub const M5_SHELL_METRIC_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_shell_metric_and_minimum_size_registries";

/// Schema version for M5 shell-metric / minimum-size registry records.
pub const M5_SHELL_METRIC_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SHELL_METRIC_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-metric-and-minimum-size-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SHELL_METRIC_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_shell_metric_and_minimum_size_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHELL_METRIC_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SHELL_METRIC_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SHELL_METRIC_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHELL_METRIC_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-shell-metric-and-minimum-size-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5ShellMetricRegistriesConsumerSurface = M5ShellGeometryConsumerSurface;

/// Canonical logical-pixel bounds for a shell zone, before OS scaling. `maximum_px == 0` means the zone
/// has no fixed cap because it is the dominant zone (fills the remaining width) or its cap is a percentage
/// of the window rather than a fixed logical-pixel value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5ShellZoneBounds {
    /// Minimum logical-pixel size.
    pub minimum_px: u32,
    /// Default logical-pixel size.
    pub default_px: u32,
    /// Recommended logical-pixel size.
    pub recommended_px: u32,
    /// Maximum logical-pixel size (`0` means no fixed cap).
    pub maximum_px: u32,
}

/// One of the three density modes every shell-metric / minimum-size entry must hold across so a metric or
/// hit target keeps its meaning under comfortable, standard, and compact presentation. Minted by this lane
/// because the frozen matrix names the density-mode *family* but not the concrete mode set an entry must
/// cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellDensityMode {
    /// The comfortable density mode.
    Comfortable,
    /// The standard density mode.
    Standard,
    /// The compact density mode.
    Compact,
}

impl M5ShellDensityMode {
    /// Every density mode, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [Self::Comfortable, Self::Standard, Self::Compact];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Standard => "standard",
            Self::Compact => "compact",
        }
    }
}

/// Controlled shell zone a metric entry maps, so the canonical title / context bar, activity rail, sidebar,
/// main editor group, right inspector, bottom panel, and status bar sizes share one registry rather than a
/// hand-copied constant. Minted by this lane because the frozen matrix carries the high-level geometry roles
/// but not the finer zones the shell-metric acceptance criteria require by name. Every classified zone
/// carries its canonical logical-pixel envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellZone {
    /// Title / context bar (height 32–40 px).
    TitleContextBar,
    /// Activity rail (width 44–56 px, 48 px default).
    ActivityRail,
    /// Sidebar (width 220–420 px, 260–320 px default).
    Sidebar,
    /// Main editor group (width minimum 420 px, 720+ px recommended; dominant, fills the remainder).
    MainEditorGroup,
    /// Right inspector (width 280–420 px, 320–360 px default).
    RightInspector,
    /// Bottom panel (height minimum 180 px, 240–320 px default, capped at 45% of window height).
    BottomPanel,
    /// Status bar (height 24–28 px).
    StatusBar,
    /// The shell zone is unclassified, which is disallowed.
    ZoneUnclassified,
}

impl M5ShellZone {
    /// Every shell zone, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TitleContextBar,
        Self::ActivityRail,
        Self::Sidebar,
        Self::MainEditorGroup,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
        Self::ZoneUnclassified,
    ];

    /// The canonical shell zones every claimed M5 desktop surface resolves from the registry.
    pub const CANONICAL_ZONES: [Self; 7] = [
        Self::TitleContextBar,
        Self::ActivityRail,
        Self::Sidebar,
        Self::MainEditorGroup,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::ActivityRail => "activity_rail",
            Self::Sidebar => "sidebar",
            Self::MainEditorGroup => "main_editor_group",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
            Self::ZoneUnclassified => "zone_unclassified",
        }
    }

    /// Whether the zone is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ZoneUnclassified)
    }

    /// Whether this is the dominant main-workspace zone that must never be starved below its minimum.
    pub const fn is_workspace_dominant(self) -> bool {
        matches!(self, Self::MainEditorGroup)
    }

    /// Canonical logical-pixel envelope for this zone, before OS scaling. The unclassified sentinel has no
    /// bounds.
    pub const fn canonical_bounds(self) -> M5ShellZoneBounds {
        match self {
            Self::TitleContextBar => M5ShellZoneBounds {
                minimum_px: 32,
                default_px: 36,
                recommended_px: 40,
                maximum_px: 40,
            },
            Self::ActivityRail => M5ShellZoneBounds {
                minimum_px: 44,
                default_px: 48,
                recommended_px: 48,
                maximum_px: 56,
            },
            Self::Sidebar => M5ShellZoneBounds {
                minimum_px: 220,
                default_px: 260,
                recommended_px: 320,
                maximum_px: 420,
            },
            Self::MainEditorGroup => M5ShellZoneBounds {
                minimum_px: 420,
                default_px: 720,
                recommended_px: 720,
                maximum_px: 0,
            },
            Self::RightInspector => M5ShellZoneBounds {
                minimum_px: 280,
                default_px: 320,
                recommended_px: 360,
                maximum_px: 420,
            },
            Self::BottomPanel => M5ShellZoneBounds {
                minimum_px: 180,
                default_px: 240,
                recommended_px: 320,
                maximum_px: 0,
            },
            Self::StatusBar => M5ShellZoneBounds {
                minimum_px: 24,
                default_px: 24,
                recommended_px: 26,
                maximum_px: 28,
            },
            Self::ZoneUnclassified => M5ShellZoneBounds {
                minimum_px: 0,
                default_px: 0,
                recommended_px: 0,
                maximum_px: 0,
            },
        }
    }
}

/// Controlled control class a minimum-size entry maps, so the canonical tab minimum width, resize-handle hit
/// area, and icon-only control hit targets share one registry and never shrink below the supported minimum.
/// Minted by this lane, tracking the minimum-size hit targets the acceptance criteria require by name. Every
/// classified control carries its canonical minimum logical-pixel hit dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellControlClass {
    /// A tab (minimum width 96–160 px).
    Tab,
    /// A resize handle (hit area 4–8 px).
    ResizeHandle,
    /// An icon-only control (hit target 28–36 px).
    IconOnlyControl,
    /// The control class is unclassified, which is disallowed.
    ControlUnclassified,
}

impl M5ShellControlClass {
    /// Every control class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Tab,
        Self::ResizeHandle,
        Self::IconOnlyControl,
        Self::ControlUnclassified,
    ];

    /// The three canonical control classes whose hit targets must hold above their supported minimum.
    pub const CANONICAL_CONTROLS: [Self; 3] =
        [Self::Tab, Self::ResizeHandle, Self::IconOnlyControl];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tab => "tab",
            Self::ResizeHandle => "resize_handle",
            Self::IconOnlyControl => "icon_only_control",
            Self::ControlUnclassified => "control_unclassified",
        }
    }

    /// Whether the control class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ControlUnclassified)
    }

    /// The canonical minimum logical-pixel hit dimension for this control, before OS scaling. A declared
    /// minimum below this value shrinks the target below its supported minimum.
    pub const fn canonical_minimum_px(self) -> u32 {
        match self {
            Self::Tab => 96,
            Self::ResizeHandle => 4,
            Self::IconOnlyControl => 28,
            Self::ControlUnclassified => 0,
        }
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a shell-metric or
/// minimum-size token's meaning stays stable whether it appears in the shell, editor, review, notebook, or
/// data surface. Minted by this lane, tracking the first-consumer surfaces the implementation requirement
/// names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellSurfaceContext {
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

impl M5ShellSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
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

/// One mandatory rendered part a shell-metric or minimum-size entry must be able to show, so no size,
/// density, or registry fact is left implicit behind a hand-copied constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The shell zone the entry maps (shell-metric entry).
    ShellZone,
    /// The logical-pixel envelope (default / minimum / recommended / maximum).
    LogicalPixelEnvelope,
    /// The density coverage (comfortable / standard / compact).
    DensityCoverage,
    /// The control class the entry maps (minimum-size entry).
    ControlClass,
    /// The minimum hit dimension the entry declares (minimum-size entry).
    MinimumHitDimension,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the metric (both entries).
    PlainLanguageMeaning,
}

impl M5ShellRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::ShellZone,
        Self::LogicalPixelEnvelope,
        Self::DensityCoverage,
        Self::ControlClass,
        Self::MinimumHitDimension,
        Self::SurfaceContext,
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
            Self::ShellZone => "shell_zone",
            Self::LogicalPixelEnvelope => "logical_pixel_envelope",
            Self::DensityCoverage => "density_coverage",
            Self::ControlClass => "control_class",
            Self::MinimumHitDimension => "minimum_hit_dimension",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a size,
/// density coverage, or a degraded shell-metric / minimum-size entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellRegistryNextAction {
    /// Expand the metric's plain-language meaning.
    ExpandMetricMeaning,
    /// Inspect the shell zone or control class the entry maps.
    InspectZoneOrControl,
    /// Complete the comfortable / standard / compact density coverage.
    CompleteDensityCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ShellRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandMetricMeaning,
        Self::InspectZoneOrControl,
        Self::CompleteDensityCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandMetricMeaning => "expand_metric_meaning",
            Self::InspectZoneOrControl => "inspect_zone_or_control",
            Self::CompleteDensityCoverage => "complete_density_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The geometry families covered.
    GeometryFamilies,
    /// The shell zones carried.
    ShellZones,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The density modes covered.
    DensityModes,
    /// The control classes carried.
    ControlClasses,
    /// The render / surface context.
    SurfaceContext,
    /// The logical-pixel envelopes carried.
    LogicalPixelEnvelopes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ShellRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::ShellZones,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::DensityModes,
        Self::ControlClasses,
        Self::SurfaceContext,
        Self::LogicalPixelEnvelopes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::ShellZones,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::GeometryFamilies => "geometry_families",
            Self::ShellZones => "shell_zones",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::DensityModes => "density_modes",
            Self::ControlClasses => "control_classes",
            Self::SurfaceContext => "surface_context",
            Self::LogicalPixelEnvelopes => "logical_pixel_envelopes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a shell-metric entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, out-of-envelope, workspace-starving, or
/// density-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellMetricEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the metric means.
    MetricTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The shell zone is unclassified (not in the preserved taxonomy).
    ZoneUnclassified,
    /// The metric is a hand-copied constant instead of tracing to the canonical registry.
    MetricNotBoundToRegistry,
    /// The declared logical-pixel bounds fall outside the zone's canonical envelope.
    MetricOutsideCanonicalEnvelope,
    /// The metric starves the main workspace below its minimum.
    ZoneStarvesMainWorkspace,
    /// The comfortable / standard / compact density coverage is incomplete.
    DensityCoverageIncomplete,
    /// The metric does not preserve task identity under snapped or narrow window widths.
    SnappedWidthUnsafe,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ShellMetricEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::MetricTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ZoneUnclassified,
        Self::MetricNotBoundToRegistry,
        Self::MetricOutsideCanonicalEnvelope,
        Self::ZoneStarvesMainWorkspace,
        Self::DensityCoverageIncomplete,
        Self::SnappedWidthUnsafe,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetricTokenUnstated => "metric_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ZoneUnclassified => "zone_unclassified",
            Self::MetricNotBoundToRegistry => "metric_not_bound_to_registry",
            Self::MetricOutsideCanonicalEnvelope => "metric_outside_canonical_envelope",
            Self::ZoneStarvesMainWorkspace => "zone_starves_main_workspace",
            Self::DensityCoverageIncomplete => "density_coverage_incomplete",
            Self::SnappedWidthUnsafe => "snapped_width_unsafe",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ShellRegistryNextAction {
        match self {
            Self::MetricTokenUnstated | Self::MetricNotBoundToRegistry => {
                M5ShellRegistryNextAction::TraceCanonicalRegistry
            }
            Self::ZoneUnclassified
            | Self::MetricOutsideCanonicalEnvelope
            | Self::ZoneStarvesMainWorkspace => M5ShellRegistryNextAction::InspectZoneOrControl,
            Self::DensityCoverageIncomplete => M5ShellRegistryNextAction::CompleteDensityCoverage,
            Self::SurfaceContextUnresolved | Self::SnappedWidthUnsafe | Self::ProofStale => {
                M5ShellRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::MetricTokenUnstated => M5ShellGeometryDowngradeTrigger::SizeMetricUnstated,
            Self::SurfaceContextUnresolved | Self::ZoneUnclassified => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::MetricNotBoundToRegistry => {
                M5ShellGeometryDowngradeTrigger::MetricCopiedByHandAcrossPackages
            }
            Self::MetricOutsideCanonicalEnvelope | Self::ZoneStarvesMainWorkspace => {
                M5ShellGeometryDowngradeTrigger::ZoneStarvedMainWorkspace
            }
            Self::DensityCoverageIncomplete => M5ShellGeometryDowngradeTrigger::DensityModeUnstated,
            Self::SnappedWidthUnsafe => M5ShellGeometryDowngradeTrigger::ResponsiveClassUnstated,
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a minimum-size entry degraded below a clean, minimum-safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MinimumSizeEntryDegradeReason {
    /// The canonical registry token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The control class is unclassified (not in the preserved taxonomy).
    ControlUnclassified,
    /// The hit target shrinks below its supported minimum, or is not reachable by pointer and keyboard.
    HitTargetShrinksBelowMinimum,
    /// The comfortable / standard / compact density coverage is incomplete.
    DensityCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5MinimumSizeEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::ControlUnclassified,
        Self::HitTargetShrinksBelowMinimum,
        Self::DensityCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ControlUnclassified => "control_unclassified",
            Self::HitTargetShrinksBelowMinimum => "hit_target_shrinks_below_minimum",
            Self::DensityCoverageIncomplete => "density_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ShellRegistryNextAction {
        match self {
            Self::TokenNameUnstated => M5ShellRegistryNextAction::TraceCanonicalRegistry,
            Self::ControlUnclassified | Self::HitTargetShrinksBelowMinimum => {
                M5ShellRegistryNextAction::InspectZoneOrControl
            }
            Self::DensityCoverageIncomplete => M5ShellRegistryNextAction::CompleteDensityCoverage,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ShellRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenNameUnstated => M5ShellGeometryDowngradeTrigger::SizeMetricUnstated,
            Self::SurfaceContextUnresolved | Self::ControlUnclassified => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::HitTargetShrinksBelowMinimum => {
                M5ShellGeometryDowngradeTrigger::HitTargetShrankBelowMinimum
            }
            Self::DensityCoverageIncomplete => M5ShellGeometryDowngradeTrigger::DensityModeUnstated,
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_shell_metric_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellMetricEntryResolutionInput {
    /// Stable identity of the shell-metric-registry entry.
    pub entry_id: String,
    /// The canonical registry token name (e.g. `shell.metric.sidebar.default`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The shell-metric role (from the frozen matrix vocabulary).
    pub metric_role: M5ShellMetricRole,
    /// The shell zone this entry maps.
    pub zone: M5ShellZone,
    /// The render / surface context.
    pub surface_context: M5ShellSurfaceContext,
    /// The density modes this entry holds across (must cover comfortable / standard / compact).
    pub density_coverage: Vec<M5ShellDensityMode>,
    /// The declared minimum logical-pixel size.
    pub minimum_px: u32,
    /// The declared default logical-pixel size.
    pub default_px: u32,
    /// The declared recommended logical-pixel size.
    pub recommended_px: u32,
    /// The declared maximum logical-pixel size (`0` means no fixed cap).
    pub maximum_px: u32,
    /// True when the entry traces to a canonical registry token (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the metric starves the main workspace below its minimum (a hard invariant when `true`).
    pub starves_main_workspace: bool,
    /// True when the metric preserves task identity under snapped or narrow window widths.
    pub preserves_task_identity_under_snapped_width: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe shell-metric-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedShellMetricEntry {
    /// Stable identity of the shell-metric-registry entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve task identity when density changes or the layout collapses.
    pub semantic_role_preserves_task_identity_under_collapse: bool,
    /// The shell-metric-role token named by the entry.
    pub metric_role: String,
    /// Whether the metric role names the disallowed hand-copied-constant token.
    pub metric_role_hand_copied: bool,
    /// The shell-zone token named by the entry.
    pub zone: String,
    /// Whether the shell zone is classified into the preserved taxonomy.
    pub zone_is_classified: bool,
    /// Whether the shell zone is the dominant main-workspace zone.
    pub zone_is_workspace_dominant: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The density-mode tokens covered by the entry.
    pub density_coverage: Vec<String>,
    /// Whether the entry covers all three density modes.
    pub covers_all_density_modes: bool,
    /// The declared minimum logical-pixel size.
    pub minimum_px: u32,
    /// The declared default logical-pixel size.
    pub default_px: u32,
    /// The declared recommended logical-pixel size.
    pub recommended_px: u32,
    /// The declared maximum logical-pixel size.
    pub maximum_px: u32,
    /// Whether the declared bounds stay inside the zone's canonical envelope.
    pub within_canonical_envelope: bool,
    /// Whether the entry traces to a canonical registry token.
    pub bound_to_registry: bool,
    /// Whether the metric starves the main workspace below its minimum.
    pub starves_main_workspace: bool,
    /// Whether the metric preserves task identity under snapped or narrow window widths.
    pub preserves_task_identity_under_snapped_width: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5ShellMetricEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ShellRegistryNextAction,
    /// Whether the metric holds across every density mode and snapped width (clean entry naming every fact).
    pub metric_holds_across_density_and_snapped_widths: bool,
}

impl M5ResolvedShellMetricEntry {
    /// Whether this shell-metric entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_minimum_size_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MinimumSizeEntryResolutionInput {
    /// Stable identity of the minimum-size entry.
    pub entry_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The minimum-size role (from the frozen matrix vocabulary).
    pub minimum_size_role: M5MinimumSizeRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The control class this entry maps.
    pub control: M5ShellControlClass,
    /// The render / surface context.
    pub surface_context: M5ShellSurfaceContext,
    /// The density modes this entry holds across (must cover comfortable / standard / compact).
    pub density_coverage: Vec<M5ShellDensityMode>,
    /// The declared minimum logical-pixel hit dimension.
    pub declared_minimum_px: u32,
    /// True when the target is reachable by both pointer and keyboard.
    pub pointer_and_keyboard_reachable: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe minimum-size projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMinimumSizeEntry {
    /// Stable identity of the minimum-size entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The minimum-size-role token named by the entry.
    pub minimum_size_role: String,
    /// Whether the minimum-size role names the disallowed shrinks-below-minimum token.
    pub minimum_size_role_shrinks_below_minimum: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The control-class token named by the entry.
    pub control: String,
    /// Whether the control class is classified into the preserved taxonomy.
    pub control_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The density-mode tokens covered by the entry.
    pub density_coverage: Vec<String>,
    /// Whether the entry covers all three density modes.
    pub covers_all_density_modes: bool,
    /// The declared minimum logical-pixel hit dimension.
    pub declared_minimum_px: u32,
    /// The canonical minimum logical-pixel hit dimension for this control.
    pub canonical_minimum_px: u32,
    /// Whether the declared minimum meets the supported minimum.
    pub meets_supported_minimum: bool,
    /// Whether the target is reachable by both pointer and keyboard.
    pub pointer_and_keyboard_reachable: bool,
    /// Degrade reason, if the entry could not read as a clean, minimum-safe state.
    pub degrade_reason: Option<M5MinimumSizeEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ShellRegistryNextAction,
    /// Whether the hit target holds above its minimum across every density mode (clean entry naming every
    /// fact).
    pub hit_target_holds_across_density: bool,
}

impl M5ResolvedMinimumSizeEntry {
    /// Whether this minimum-size entry reads as a clean, minimum-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ShellMetricResolutionError {
    /// The shell-metric-entry id was empty.
    EmptyShellMetricEntryId,
    /// The minimum-size-entry id was empty.
    EmptyMinimumSizeEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ShellMetricResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyShellMetricEntryId => "empty_shell_metric_entry_id",
            Self::EmptyMinimumSizeEntryId => "empty_minimum_size_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ShellMetricResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 shell-metric / minimum-size registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ShellMetricResolutionError {}

fn density_tokens(modes: &[M5ShellDensityMode]) -> Vec<String> {
    modes.iter().map(|m| m.as_str().to_owned()).collect()
}

fn covers_all_density_modes(modes: &[M5ShellDensityMode]) -> bool {
    let present: BTreeSet<M5ShellDensityMode> = modes.iter().copied().collect();
    M5ShellDensityMode::ALL
        .iter()
        .all(|mode| present.contains(mode))
}

/// Whether the declared logical-pixel bounds stay inside the zone's canonical envelope. A metric may declare
/// exactly the canonical values or a tighter (larger-minimum, smaller-maximum) window, but never a minimum
/// below the canonical floor or an ordering that inverts min / default / recommended / max.
fn within_canonical_envelope(
    zone: M5ShellZone,
    minimum_px: u32,
    default_px: u32,
    recommended_px: u32,
    maximum_px: u32,
) -> bool {
    if !zone.is_classified() {
        return false;
    }
    let bounds = zone.canonical_bounds();
    // The declared minimum must never fall below the canonical floor.
    if minimum_px < bounds.minimum_px {
        return false;
    }
    // Ordering: minimum <= default <= recommended (ignoring unset zeros for default / recommended).
    if default_px != 0 && default_px < minimum_px {
        return false;
    }
    if recommended_px != 0 && default_px != 0 && recommended_px < default_px {
        return false;
    }
    // When the zone declares a fixed cap, the declared maximum must not exceed it and every declared value
    // must stay at or below the effective cap.
    if bounds.maximum_px != 0 {
        if maximum_px != 0 && maximum_px > bounds.maximum_px {
            return false;
        }
        let cap = if maximum_px != 0 {
            maximum_px
        } else {
            bounds.maximum_px
        };
        if minimum_px > cap || default_px > cap || recommended_px > cap {
            return false;
        }
    }
    true
}

/// Resolves a shell-metric-registry entry so it stays bound to the shared registry: the entry names its
/// canonical token, semantic role, metric role, and shell zone, covers all three density modes, declares
/// logical-pixel bounds inside the zone's canonical envelope, never starves the main workspace, and
/// preserves task identity under snapped widths.
pub fn resolve_shell_metric_entry(
    input: M5ShellMetricEntryResolutionInput,
) -> Result<M5ResolvedShellMetricEntry, M5ShellMetricResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ShellMetricResolutionError::EmptyShellMetricEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ShellMetricResolutionError::ForbiddenMaterial);
    }

    let metric_role_hand_copied = matches!(
        input.metric_role,
        M5ShellMetricRole::HandCopiedConstantDisallowed
    );
    let all_density = covers_all_density_modes(&input.density_coverage);
    let within_envelope = within_canonical_envelope(
        input.zone,
        input.minimum_px,
        input.default_px,
        input.recommended_px,
        input.maximum_px,
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ShellMetricEntryDegradeReason::MetricTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ShellMetricEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.zone.is_classified() {
        Some(M5ShellMetricEntryDegradeReason::ZoneUnclassified)
    } else if metric_role_hand_copied || !input.bound_to_registry {
        Some(M5ShellMetricEntryDegradeReason::MetricNotBoundToRegistry)
    } else if !within_envelope {
        Some(M5ShellMetricEntryDegradeReason::MetricOutsideCanonicalEnvelope)
    } else if input.starves_main_workspace {
        Some(M5ShellMetricEntryDegradeReason::ZoneStarvesMainWorkspace)
    } else if !all_density {
        Some(M5ShellMetricEntryDegradeReason::DensityCoverageIncomplete)
    } else if !input.preserves_task_identity_under_snapped_width {
        Some(M5ShellMetricEntryDegradeReason::SnappedWidthUnsafe)
    } else if !input.proof_fresh {
        Some(M5ShellMetricEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ShellRegistryNextAction::ExpandMetricMeaning,
    };

    Ok(M5ResolvedShellMetricEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        metric_role: input.metric_role.as_str().to_owned(),
        metric_role_hand_copied,
        zone: input.zone.as_str().to_owned(),
        zone_is_classified: input.zone.is_classified(),
        zone_is_workspace_dominant: input.zone.is_workspace_dominant(),
        surface_context: input.surface_context.as_str().to_owned(),
        density_coverage: density_tokens(&input.density_coverage),
        covers_all_density_modes: all_density,
        minimum_px: input.minimum_px,
        default_px: input.default_px,
        recommended_px: input.recommended_px,
        maximum_px: input.maximum_px,
        within_canonical_envelope: within_envelope,
        bound_to_registry: input.bound_to_registry,
        starves_main_workspace: input.starves_main_workspace,
        preserves_task_identity_under_snapped_width: input
            .preserves_task_identity_under_snapped_width,
        degrade_reason,
        next_action,
        metric_holds_across_density_and_snapped_widths: degrade_reason.is_none(),
    })
}

/// Resolves a minimum-size entry so it stays safe above its supported minimum: the entry names its canonical
/// token, minimum-size role, semantic role, and control class, covers all three density modes, declares a
/// minimum hit dimension at or above the control's canonical minimum, and stays reachable by pointer and
/// keyboard.
pub fn resolve_minimum_size_entry(
    input: M5MinimumSizeEntryResolutionInput,
) -> Result<M5ResolvedMinimumSizeEntry, M5ShellMetricResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ShellMetricResolutionError::EmptyMinimumSizeEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ShellMetricResolutionError::ForbiddenMaterial);
    }

    let role_shrinks_below_minimum = matches!(
        input.minimum_size_role,
        M5MinimumSizeRole::ShrinksBelowMinimumDisallowed
    );
    let all_density = covers_all_density_modes(&input.density_coverage);
    let canonical_minimum_px = input.control.canonical_minimum_px();
    let meets_supported_minimum =
        input.control.is_classified() && input.declared_minimum_px >= canonical_minimum_px;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5MinimumSizeEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5MinimumSizeEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.control.is_classified() {
        Some(M5MinimumSizeEntryDegradeReason::ControlUnclassified)
    } else if role_shrinks_below_minimum
        || !meets_supported_minimum
        || !input.pointer_and_keyboard_reachable
    {
        Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
    } else if !all_density {
        Some(M5MinimumSizeEntryDegradeReason::DensityCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5MinimumSizeEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ShellRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedMinimumSizeEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        minimum_size_role: input.minimum_size_role.as_str().to_owned(),
        minimum_size_role_shrinks_below_minimum: role_shrinks_below_minimum,
        semantic_role: input.semantic_role.as_str().to_owned(),
        control: input.control.as_str().to_owned(),
        control_is_classified: input.control.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        density_coverage: density_tokens(&input.density_coverage),
        covers_all_density_modes: all_density,
        declared_minimum_px: input.declared_minimum_px,
        canonical_minimum_px,
        meets_supported_minimum,
        pointer_and_keyboard_reachable: input.pointer_and_keyboard_reachable,
        degrade_reason,
        next_action,
        hit_target_holds_across_density: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved shell-metric and minimum-size entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ShellMetricRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5ShellRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ShellRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    /// Resolved shell-metric-registry examples.
    pub shell_metric_entries: Vec<M5ResolvedShellMetricEntry>,
    /// Resolved minimum-size examples.
    pub minimum_size_entries: Vec<M5ResolvedMinimumSizeEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical shell-metrics domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a zone never starves the main workspace below its minimum. MUST be `false`.
    pub lets_zone_starve_main_workspace_below_minimum: bool,
    /// Hard invariant: a hit target never shrinks below the supported minimum. MUST be `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Hard invariant: an extension or embedded surface never sets a private fracturing width. MUST be
    /// `false`.
    pub extension_or_embedded_sets_private_fracturing_width: bool,
    /// Hard invariant: a metric is never hand-copied instead of tracing to the registry. MUST be `false`.
    pub metric_hand_copied_instead_of_registry: bool,
}

impl M5ShellMetricRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ShellRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ShellRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ShellRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5ShellRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.lets_zone_starve_main_workspace_below_minimum
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.extension_or_embedded_sets_private_fracturing_width
            && !self.metric_hand_copied_instead_of_registry
    }

    /// True when a clean shell-metric entry preserves registry-bound geometry: it traces to the registry,
    /// never names the disallowed hand-copied role, keeps a classified zone, stays inside the canonical
    /// envelope, never starves the main workspace, covers all three density modes, and preserves task
    /// identity under snapped widths.
    fn metric_is_honest(ex: &M5ResolvedShellMetricEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && !ex.metric_role_hand_copied
                && ex.zone_is_classified
                && ex.within_canonical_envelope
                && !ex.starves_main_workspace
                && ex.covers_all_density_modes
                && ex.preserves_task_identity_under_snapped_width)
    }

    /// True when a clean minimum-size entry preserves hit-target safety: it keeps a classified control,
    /// never names the disallowed shrinks-below-minimum role, meets the supported minimum, stays reachable,
    /// and covers all three density modes.
    fn minimum_is_honest(ex: &M5ResolvedMinimumSizeEntry) -> bool {
        !ex.is_clean()
            || (ex.control_is_classified
                && !ex.minimum_size_role_shrinks_below_minimum
                && ex.meets_supported_minimum
                && ex.pointer_and_keyboard_reachable
                && ex.covers_all_density_modes)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.shell_metric_entries.iter().all(Self::metric_is_honest)
            && self
                .minimum_size_entries
                .iter()
                .all(Self::minimum_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Shell-metric-role tokens (bound from the frozen matrix).
    pub shell_metric_roles: Vec<String>,
    /// Minimum-size-role tokens (bound from the frozen matrix).
    pub minimum_size_roles: Vec<String>,
    /// Density-mode tokens (minted by this lane).
    pub density_modes: Vec<String>,
    /// Shell-zone tokens (minted by this lane).
    pub shell_zones: Vec<String>,
    /// Control-class tokens (minted by this lane).
    pub control_classes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Shell-metric-entry degrade-reason tokens.
    pub shell_metric_degrade_reasons: Vec<String>,
    /// Minimum-size-entry degrade-reason tokens.
    pub minimum_size_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ShellMetricRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ShellGeometryRole::ALL, |v| v.as_str()),
            shell_metric_roles: tokens(&M5ShellMetricRole::ALL, |v| v.as_str()),
            minimum_size_roles: tokens(&M5MinimumSizeRole::ALL, |v| v.as_str()),
            density_modes: tokens(&M5ShellDensityMode::ALL, |v| v.as_str()),
            shell_zones: tokens(&M5ShellZone::ALL, |v| v.as_str()),
            control_classes: tokens(&M5ShellControlClass::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ShellSurfaceContext::ALL, |v| v.as_str()),
            shell_metric_degrade_reasons: tokens(&M5ShellMetricEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            minimum_size_degrade_reasons: tokens(&M5MinimumSizeEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ShellRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ShellRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ShellRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5ShellMetricRegistriesGovernanceReview {
    /// The shell-metric registry names a canonical token, metric role, and shell zone for every entry.
    pub shell_metric_registry_names_token_role_and_zone: bool,
    /// The reference metrics are encoded as logical-pixel contracts before OS scaling.
    pub reference_metrics_encoded_as_logical_pixel_contracts: bool,
    /// Every claimed surface resolves its geometry from the shared metric registry.
    pub every_surface_resolves_from_shared_registry: bool,
    /// The main editor group stays dominant and is never starved below its minimum.
    pub main_editor_group_stays_dominant: bool,
    /// Hit targets never shrink below the supported minimum under any density mode.
    pub hit_targets_never_shrink_below_supported_minimum: bool,
    /// Every metric and hit target covers the comfortable / standard / compact density modes.
    pub every_entry_covers_all_density_modes: bool,
    /// Metrics stay bound to one registry rather than hand-copied across packages.
    pub metrics_bound_to_single_registry_not_hand_copied: bool,
    /// Metric drift outside the canonical envelope is caught by fixtures before release evidence turns
    /// green.
    pub metric_drift_caught_before_release: bool,
    /// The first shell / editor / review / notebook / data consumers use the canonical shell-metric
    /// grammar.
    pub first_consumers_use_canonical_metric_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesConsumerProjection {
    /// The shell surface consumes the shared shell-metric / minimum-size registries.
    pub shell_consumes_shared_registries: bool,
    /// The editor surface consumes the shared shell-metric / minimum-size registries.
    pub editor_consumes_shared_registries: bool,
    /// The review surface consumes the shared shell-metric / minimum-size registries.
    pub review_consumes_shared_registries: bool,
    /// The notebook and data surfaces consume the shared shell-metric / minimum-size registries.
    pub notebook_and_data_consume_shared_registries: bool,
    /// Shell geometry traces back to one canonical shell-metrics domain contract.
    pub shell_geometry_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical shell-metric / minimum-size registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting shell-geometry audit for the lane.
    pub geometry_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ShellMetricRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellMetricRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ShellMetricRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellMetricRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellMetricRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellMetricRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellMetricRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellMetricRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 shell-metric and minimum-size registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricRegistriesPacket {
    /// Record kind; must equal [`M5_SHELL_METRIC_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHELL_METRIC_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ShellMetricRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellMetricRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellMetricRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellMetricRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellMetricRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellMetricRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShellMetricRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ShellMetricRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SHELL_METRIC_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_METRIC_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ShellMetricRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHELL_METRIC_REGISTRIES_RECORD_KIND {
            violations.push(M5ShellMetricRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHELL_METRIC_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5ShellMetricRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShellMetricRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ShellMetricRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 shell-metric / minimum-size registries packet serializes"),
        ) {
            violations.push(M5ShellMetricRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 shell-metric / minimum-size registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,shell_metric_entries,minimum_size_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .shell_metric_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.minimum_size_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_metric_entries.len(),
                row.minimum_size_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Shell-Metric and Minimum-Size Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Shell zones: {}\n",
            self.vocabulary_set.shell_zones.join(", ")
        ));
        out.push_str(&format!(
            "- Density modes: {}\n",
            self.vocabulary_set.density_modes.join(", ")
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
                "  - Shell-metric entries: {} / minimum-size entries: {}\n",
                row.shell_metric_entries.len(),
                row.minimum_size_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ShellMetricRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShellMetricRegistriesViolation>),
}

impl fmt::Display for M5ShellMetricRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 shell-metric / minimum-size registries export parse failed: {error}"
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
                    "m5 shell-metric / minimum-size registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShellMetricRegistriesArtifactError {}

/// Validation failures emitted by [`M5ShellMetricRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShellMetricRegistriesViolation {
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
    /// A registry row does not point at the canonical shell-metrics domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, out-of-envelope,
    /// workspace-starving, density-incomplete, or a minimum-size entry that shrinks below its minimum).
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
    /// First-consumer canonical adoption is not proven: clean shell-metric entries do not cover the
    /// canonical semantic-role families or the first shell / editor / review / notebook / data surfaces, no
    /// hand-copied example degrades, or a clean entry is not bound to the registry.
    FirstConsumersResolveFromSharedRegistryNotProven,
    /// Minimum guarantees across density and snapped widths are not proven: clean minimum-size entries do
    /// not cover the tab / resize-handle / icon-only control classes with full density coverage while
    /// meeting the supported minimum, no below-minimum or density-incomplete example degrades, or a clean
    /// entry shrinks below its minimum.
    MinimumGuaranteesAcrossDensityAndSnappedWidthsNotProven,
    /// Drift outside the canonical envelope is not detectable: no metric-outside-envelope example and no
    /// below-minimum example degrade, clean entries do not trace to the registry, or a clean entry drifts.
    DriftOutsideCanonicalEnvelopeDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ShellMetricRegistriesViolation {
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
            Self::FirstConsumersResolveFromSharedRegistryNotProven => {
                "first_consumers_resolve_from_shared_registry_not_proven"
            }
            Self::MinimumGuaranteesAcrossDensityAndSnappedWidthsNotProven => {
                "minimum_guarantees_across_density_and_snapped_widths_not_proven"
            }
            Self::DriftOutsideCanonicalEnvelopeDetectableNotProven => {
                "drift_outside_canonical_envelope_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_shell_metric_minimum_size_registries_export(
) -> Result<M5ShellMetricRegistriesPacket, M5ShellMetricRegistriesArtifactError> {
    let packet: M5ShellMetricRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-metric-and-minimum-size-registries-proof/support_export.json"
    )))
    .map_err(M5ShellMetricRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShellMetricRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHELL_METRIC_REGISTRIES_SCHEMA_REF,
        M5_SHELL_METRIC_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_SHELL_METRICS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ShellMetricRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5ShellMetricRegistriesViolation::NoRegistryRows);
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
            violations.push(M5ShellMetricRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ShellMetricRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ShellMetricRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SHELL_METRICS_SCHEMA_REF) {
            violations.push(M5ShellMetricRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.shell_metric_entries.is_empty() || row.minimum_size_entries.is_empty() {
            violations.push(M5ShellMetricRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ShellMetricRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ShellMetricRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.shell_metric_registry_names_token_role_and_zone,
        review.reference_metrics_encoded_as_logical_pixel_contracts,
        review.every_surface_resolves_from_shared_registry,
        review.main_editor_group_stays_dominant,
        review.hit_targets_never_shrink_below_supported_minimum,
        review.every_entry_covers_all_density_modes,
        review.metrics_bound_to_single_registry_not_hand_copied,
        review.metric_drift_caught_before_release,
        review.first_consumers_use_canonical_metric_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ShellMetricRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.editor_consumes_shared_registries,
        projection.review_consumes_shared_registries,
        projection.notebook_and_data_consume_shared_registries,
        projection.shell_geometry_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5ShellMetricRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ShellMetricRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.geometry_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ShellMetricRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ShellMetricRegistriesPacket,
    violations: &mut Vec<M5ShellMetricRegistriesViolation>,
) {
    let metrics = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.shell_metric_entries.iter())
    };
    let minimums = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.minimum_size_entries.iter())
    };

    // AC1: all claimed M5 shell surfaces resolve their geometry from the shared metric registry. Clean
    // shell-metric entries cover the zone / metric semantic-role families and the first shell / editor /
    // review / notebook / data surfaces, a hand-copied example degrades, and no clean entry is unbound.
    let clean_semantic_roles: BTreeSet<String> = metrics()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = metrics()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = [
        M5ShellGeometryRole::Zone.as_str(),
        M5ShellGeometryRole::Metric.as_str(),
    ]
    .iter()
    .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5ShellSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let hand_copied_degrades = metrics().any(|ex| {
        ex.degrade_reason == Some(M5ShellMetricEntryDegradeReason::MetricNotBoundToRegistry)
    });
    let no_clean_unbound = !metrics().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    if !(semantic_families_covered
        && first_surfaces_covered
        && hand_copied_degrades
        && no_clean_unbound)
    {
        violations.push(
            M5ShellMetricRegistriesViolation::FirstConsumersResolveFromSharedRegistryNotProven,
        );
    }

    // AC2: minimum editor and control hit-target guarantees hold under supported density modes and
    // snapped-window widths. Clean minimum-size entries cover every canonical control class with full
    // density coverage while meeting the supported minimum, a below-minimum example degrades, a
    // density-incomplete example degrades, and no clean entry shrinks below its minimum.
    let clean_control_classes: BTreeSet<String> = minimums()
        .filter(|ex| {
            ex.is_clean()
                && ex.control_is_classified
                && ex.meets_supported_minimum
                && ex.covers_all_density_modes
        })
        .map(|ex| ex.control.clone())
        .collect();
    let control_classes_covered = M5ShellControlClass::CANONICAL_CONTROLS
        .iter()
        .all(|c| clean_control_classes.contains(c.as_str()));
    let below_minimum_degrades = minimums().any(|ex| {
        ex.degrade_reason == Some(M5MinimumSizeEntryDegradeReason::HitTargetShrinksBelowMinimum)
    });
    let density_incomplete_degrades = minimums().any(|ex| {
        ex.degrade_reason == Some(M5MinimumSizeEntryDegradeReason::DensityCoverageIncomplete)
    });
    let no_clean_below_minimum = !minimums().any(|ex| ex.is_clean() && !ex.meets_supported_minimum);
    if !(control_classes_covered
        && below_minimum_degrades
        && density_incomplete_degrades
        && no_clean_below_minimum)
    {
        violations.push(
            M5ShellMetricRegistriesViolation::MinimumGuaranteesAcrossDensityAndSnappedWidthsNotProven,
        );
    }

    // AC3: regression suites fail when a surface drifts outside the canonical B138 metric envelope. A
    // metric-outside-envelope example and a below-minimum example both degrade, at least one clean
    // shell-metric and one clean minimum-size entry trace to the registry, and no clean entry drifts.
    let envelope_drift_degrades = metrics().any(|ex| {
        ex.degrade_reason == Some(M5ShellMetricEntryDegradeReason::MetricOutsideCanonicalEnvelope)
    });
    let bound_metric = metrics().any(|ex| ex.is_clean() && ex.bound_to_registry);
    let bound_minimum = minimums().any(|ex| ex.is_clean() && ex.meets_supported_minimum);
    let no_clean_drift = !metrics().any(|ex| ex.is_clean() && !ex.within_canonical_envelope);
    if !(envelope_drift_degrades
        && below_minimum_degrades
        && bound_metric
        && bound_minimum
        && no_clean_unbound
        && no_clean_drift)
    {
        violations.push(
            M5ShellMetricRegistriesViolation::DriftOutsideCanonicalEnvelopeDetectableNotProven,
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

/// The two shell-geometry families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5ShellGeometryFamily; 2] = [
    M5ShellGeometryFamily::ShellMetric,
    M5ShellGeometryFamily::MinimumSize,
];
