//! Implemented M5 responsive-geometry and collapse-priority registries.
//!
//! The frozen [shell-metric / density matrix][matrix] names Aureline's five shell-geometry families and
//! locks their controlled vocabulary. This module is the responsive-geometry / collapse-priority implement
//! lane over that matrix: it turns the two families that carry the concrete *adaptive-layout* grammar — the
//! **responsive geometry** family (the Compact / Standard / Expanded desktop window classes, their canonical
//! logical-pixel width bounds, and the surface coexistence every claimed shell zone keeps across a class
//! change) and the **collapse priority** family (the declared adaptive-collapse order, the identity-stable
//! docked / sheet / overlay / temporary-panel transitions, and the no-fracture geometry that keeps the main
//! workspace dominant) — into registry resolvers that produce export-safe, honest projections. A user can
//! then trust that a snapped window, a larger text size, or a secondary display never fractures a daily-driver
//! flow: every claimed M5 surface resolves the same tokenized window-class bounds rather than a hand-picked
//! local breakpoint; every responsive change preserves task identity and recovery-critical state; a surface
//! that moves between docked, sheet, overlay, or temporary-panel form stays the same task surface with the
//! same state, history, and keyboard route; the declared collapse order moves optional right-inspector detail
//! into sheets or inline disclosures first, collapses secondary bottom-panel tabs before starving the editor,
//! converts low-frequency side tools to overflow before collapsing primary navigation, and never lets a zone
//! starve the main workspace or hide a primary workflow behind an overlay-only fallback; and no essential
//! action becomes hover-only and no compare / editor group silently narrows into an unusable pane.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Encode the canonical window classes and adaptive behavior as logical-pixel tokens.**
//!   [`resolve_window_class_entry`] refuses to read as a clean, registry-bound window-class entry unless it
//!   names a canonical registry token, a classified [window class][M5WindowClass], a responsive-geometry
//!   role, declares the exact canonical width bounds for that class (Compact desktop 1024–1279 px, Standard
//!   desktop 1280–1599 px, Expanded desktop 1600+ px), covers every coexisting [shell zone][M5ResponsiveShellZone]
//!   (title / context bar, rail, sidebar, main workspace, right inspector, bottom panel, status bar),
//!   preserves task identity and recovery-critical state, and never makes an essential action hover-only or
//!   narrows a compare / editor group into an unusable pane.
//! * **Implement the declared responsive priority order and keep transitions identity-stable.**
//!   [`resolve_collapse_step_entry`] names a classified [collapse target][M5CollapseTarget] and a classified
//!   [identity-transition form][M5IdentityTransitionForm], declares the canonical collapse-order rank for that
//!   target, keeps the main workspace dominant, never collapses a protected target (path / branch / trust /
//!   target identity or the editor workspace), never hides a primary workflow behind an overlay-only fallback,
//!   and degrades to [`M5CollapseStepEntryDegradeReason::DropsIdentityStateOrRoute`] when a docked / sheet /
//!   overlay / temporary-panel transition would drop the surface's identity, state, history, or keyboard route.
//! * **Wire first shell, editor, review, notebook, and data consumers plus fixtures that catch drift.**
//!   Each registry row carries the render [surface context][M5ResponsiveSurfaceContext] so a private-width or
//!   identity-dropping regression degrades honestly, and the acceptance-criteria gate proves that drift is
//!   caught before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ShellGeometryRole`] role vocabulary,
//! the [`M5ResponsiveGeometryRole`] responsive-geometry-role vocabulary, and the [`M5CollapsePriorityRole`]
//! collapse-priority-role vocabulary — so shell, editor, review, notebook, data, and support surfaces can
//! never fork their own adaptive-layout meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_shell_metric_density_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_responsive_geometry_and_collapse_priority_registries,
    seeded_m5_responsive_geometry_and_collapse_priority_registries_editor_ui_beta_narrowed,
    seeded_m5_responsive_geometry_and_collapse_priority_registries_settings_ui_preview_narrowed,
    M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shell_metric_density_matrix::{
    M5CollapsePriorityRole, M5ResponsiveGeometryRole, M5ShellGeometryAccessibilityRoute,
    M5ShellGeometryConsumerSurface, M5ShellGeometryDeploymentLine, M5ShellGeometryDowngradeTrigger,
    M5ShellGeometryFamily, M5ShellGeometryQualificationClass, M5ShellGeometryRequiredLabel,
    M5ShellGeometryRole, M5_DENSITY_MODE_SCHEMA_REF, M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
    M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket`].
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_responsive_geometry_and_collapse_priority_registries";

/// Schema version for M5 responsive-geometry / collapse-priority registry records.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the registries schema.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-responsive-geometry-and-collapse-priority-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_responsive_geometry_and_collapse_priority_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-responsive-geometry-and-collapse-priority-registries";

/// Canonical minimum supported desktop width in logical pixels (the Compact-desktop lower bound); a window
/// class whose lower bound falls below this floor is below the supported minimum width.
pub const CANONICAL_MINIMUM_SUPPORTED_WIDTH_PX: u32 = 1024;

/// Canonical Compact-desktop lower / upper width bound in logical pixels.
pub const CANONICAL_COMPACT_MIN_PX: u32 = 1024;
/// Canonical Compact-desktop upper width bound in logical pixels.
pub const CANONICAL_COMPACT_MAX_PX: u32 = 1279;
/// Canonical Standard-desktop lower width bound in logical pixels.
pub const CANONICAL_STANDARD_MIN_PX: u32 = 1280;
/// Canonical Standard-desktop upper width bound in logical pixels.
pub const CANONICAL_STANDARD_MAX_PX: u32 = 1599;
/// Canonical Expanded-desktop lower width bound in logical pixels.
pub const CANONICAL_EXPANDED_MIN_PX: u32 = 1600;
/// Sentinel upper bound for the unbounded Expanded-desktop class.
pub const EXPANDED_UPPER_BOUND_SENTINEL: u32 = u32::MAX;

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5ResponsiveGeometryRegistriesConsumerSurface = M5ShellGeometryConsumerSurface;

/// Canonical logical-pixel width bounds for one responsive window class, before OS scaling.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5WindowClassBounds {
    /// Inclusive lower width bound in logical pixels.
    pub min_width_px: u32,
    /// Inclusive upper width bound in logical pixels ([`EXPANDED_UPPER_BOUND_SENTINEL`] when unbounded).
    pub max_width_px: u32,
}

/// One of the three responsive desktop window classes every claimed M5 surface resolves from the shared
/// registry so a snapped or resized window changes layout predictably without fracturing task identity.
/// Minted by this lane because the frozen matrix names the responsive-geometry *family* but not the concrete
/// window-class set and its canonical logical-pixel bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowClass {
    /// The Compact desktop class (1024–1279 px).
    CompactDesktop,
    /// The Standard desktop class (1280–1599 px).
    StandardDesktop,
    /// The Expanded desktop class (1600+ px).
    ExpandedDesktop,
    /// The window class is unclassified, which is disallowed.
    ClassUnclassified,
}

impl M5WindowClass {
    /// Every window class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CompactDesktop,
        Self::StandardDesktop,
        Self::ExpandedDesktop,
        Self::ClassUnclassified,
    ];

    /// The three canonical window classes every claimed M5 surface resolves from the registry.
    pub const CANONICAL_CLASSES: [Self; 3] = [
        Self::CompactDesktop,
        Self::StandardDesktop,
        Self::ExpandedDesktop,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactDesktop => "compact_desktop",
            Self::StandardDesktop => "standard_desktop",
            Self::ExpandedDesktop => "expanded_desktop",
            Self::ClassUnclassified => "class_unclassified",
        }
    }

    /// Whether the window class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ClassUnclassified)
    }

    /// Canonical logical-pixel width bounds for this class. The unclassified sentinel has no bounds.
    pub const fn canonical_bounds(self) -> M5WindowClassBounds {
        match self {
            Self::CompactDesktop => M5WindowClassBounds {
                min_width_px: CANONICAL_COMPACT_MIN_PX,
                max_width_px: CANONICAL_COMPACT_MAX_PX,
            },
            Self::StandardDesktop => M5WindowClassBounds {
                min_width_px: CANONICAL_STANDARD_MIN_PX,
                max_width_px: CANONICAL_STANDARD_MAX_PX,
            },
            Self::ExpandedDesktop => M5WindowClassBounds {
                min_width_px: CANONICAL_EXPANDED_MIN_PX,
                max_width_px: EXPANDED_UPPER_BOUND_SENTINEL,
            },
            Self::ClassUnclassified => M5WindowClassBounds {
                min_width_px: 0,
                max_width_px: 0,
            },
        }
    }
}

/// Shell zone a window-class entry must keep coexisting across a class change, so a Compact / Standard /
/// Expanded change reflows the shell predictably rather than dropping a zone. Minted by this lane, tracking
/// the canonical shell zones the track invariant names directly. Every clean window-class entry covers all
/// seven.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveShellZone {
    /// The title / context bar.
    TitleContextBar,
    /// The activity rail.
    Rail,
    /// The sidebar.
    Sidebar,
    /// The dominant main workspace.
    MainWorkspace,
    /// The right inspector.
    RightInspector,
    /// The bottom panel.
    BottomPanel,
    /// The status bar.
    StatusBar,
}

impl M5ResponsiveShellZone {
    /// Every shell zone, in declaration order. A clean window-class entry must cover all of them.
    pub const ALL: [Self; 7] = [
        Self::TitleContextBar,
        Self::Rail,
        Self::Sidebar,
        Self::MainWorkspace,
        Self::RightInspector,
        Self::BottomPanel,
        Self::StatusBar,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TitleContextBar => "title_context_bar",
            Self::Rail => "rail",
            Self::Sidebar => "sidebar",
            Self::MainWorkspace => "main_workspace",
            Self::RightInspector => "right_inspector",
            Self::BottomPanel => "bottom_panel",
            Self::StatusBar => "status_bar",
        }
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a window-class or
/// collapse-step token's meaning stays stable whether it appears in the shell, editor, review, notebook, or
/// data surface. Minted by this lane, tracking the first-consumer surfaces the implementation requirement
/// names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveSurfaceContext {
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

impl M5ResponsiveSurfaceContext {
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

/// What an adaptive-collapse step operates on, in the declared responsive priority order. Optional detail
/// collapses first and primary navigation last; the path / branch / trust / target identity and the editor
/// workspace are protected and never collapse. Minted by this lane, tracking the collapse targets the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollapseTarget {
    /// Optional right-inspector detail (moves into sheets or inline disclosures first; rank 0).
    OptionalRightInspectorDetail,
    /// Secondary bottom-panel tabs (collapse before starving the editor; rank 1).
    SecondaryBottomPanelTabs,
    /// Low-frequency side tools (convert to overflow before collapsing primary navigation; rank 2).
    LowFrequencySideTools,
    /// Primary navigation (collapses last; rank 3).
    PrimaryNavigation,
    /// Path / branch / trust / target identity (protected: never collapses before optional content).
    PathBranchTrustTargetIdentity,
    /// The dominant editor workspace (protected: never starved by a collapse).
    EditorWorkspace,
    /// The collapse target is unclassified, which is disallowed.
    TargetUnclassified,
}

impl M5CollapseTarget {
    /// Every collapse target, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::OptionalRightInspectorDetail,
        Self::SecondaryBottomPanelTabs,
        Self::LowFrequencySideTools,
        Self::PrimaryNavigation,
        Self::PathBranchTrustTargetIdentity,
        Self::EditorWorkspace,
        Self::TargetUnclassified,
    ];

    /// The four collapse targets that carry a canonical priority rank, in declared collapse order.
    pub const ORDERED_COLLAPSE_TARGETS: [Self; 4] = [
        Self::OptionalRightInspectorDetail,
        Self::SecondaryBottomPanelTabs,
        Self::LowFrequencySideTools,
        Self::PrimaryNavigation,
    ];

    /// The two protected collapse targets that must never collapse.
    pub const PROTECTED_TARGETS: [Self; 2] =
        [Self::PathBranchTrustTargetIdentity, Self::EditorWorkspace];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OptionalRightInspectorDetail => "optional_right_inspector_detail",
            Self::SecondaryBottomPanelTabs => "secondary_bottom_panel_tabs",
            Self::LowFrequencySideTools => "low_frequency_side_tools",
            Self::PrimaryNavigation => "primary_navigation",
            Self::PathBranchTrustTargetIdentity => "path_branch_trust_target_identity",
            Self::EditorWorkspace => "editor_workspace",
            Self::TargetUnclassified => "target_unclassified",
        }
    }

    /// Whether the collapse target is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::TargetUnclassified)
    }

    /// Whether this target is protected and must never collapse.
    pub const fn is_protected(self) -> bool {
        matches!(
            self,
            Self::PathBranchTrustTargetIdentity | Self::EditorWorkspace
        )
    }

    /// Whether this target is a primary workflow that must never be hidden behind an overlay-only fallback.
    pub const fn is_primary_workflow(self) -> bool {
        matches!(
            self,
            Self::PrimaryNavigation | Self::EditorWorkspace | Self::PathBranchTrustTargetIdentity
        )
    }

    /// Canonical collapse-order rank for this target, or `None` when the target carries no rank (a protected
    /// or unclassified target).
    pub const fn canonical_collapse_rank(self) -> Option<u32> {
        match self {
            Self::OptionalRightInspectorDetail => Some(0),
            Self::SecondaryBottomPanelTabs => Some(1),
            Self::LowFrequencySideTools => Some(2),
            Self::PrimaryNavigation => Some(3),
            Self::PathBranchTrustTargetIdentity
            | Self::EditorWorkspace
            | Self::TargetUnclassified => None,
        }
    }
}

/// Form a task surface takes as it moves through an adaptive collapse, so it stays the same task surface with
/// the same state, history, and keyboard route whether it is docked, in a sheet, in an overlay, in a
/// temporary panel, inline, or in overflow. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IdentityTransitionForm {
    /// Docked in its canonical zone.
    Docked,
    /// Presented as a sheet.
    Sheet,
    /// Presented as an inline disclosure.
    InlineDisclosure,
    /// Presented as an overlay.
    Overlay,
    /// Presented as a temporary panel.
    TemporaryPanel,
    /// Moved into an overflow menu.
    Overflow,
    /// The transition form is unclassified, which is disallowed.
    FormUnclassified,
}

impl M5IdentityTransitionForm {
    /// Every transition form, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Docked,
        Self::Sheet,
        Self::InlineDisclosure,
        Self::Overlay,
        Self::TemporaryPanel,
        Self::Overflow,
        Self::FormUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Docked => "docked",
            Self::Sheet => "sheet",
            Self::InlineDisclosure => "inline_disclosure",
            Self::Overlay => "overlay",
            Self::TemporaryPanel => "temporary_panel",
            Self::Overflow => "overflow",
            Self::FormUnclassified => "form_unclassified",
        }
    }

    /// Whether the transition form is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::FormUnclassified)
    }

    /// Whether this form is an overlay-only fallback.
    pub const fn is_overlay_only(self) -> bool {
        matches!(self, Self::Overlay)
    }
}

/// One mandatory rendered part a window-class or collapse-step entry must be able to show, so no responsive,
/// bound, collapse, or transition fact is left implicit behind a private width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The window class the entry maps (window-class entry).
    WindowClass,
    /// The canonical width bounds (window-class entry).
    WidthBounds,
    /// The shell-zone coexistence set (window-class entry).
    ShellZoneCoexistence,
    /// The collapse target the entry maps (collapse-step entry).
    CollapseTarget,
    /// The declared collapse order (collapse-step entry).
    CollapseOrder,
    /// The identity-transition form (collapse-step entry).
    IdentityTransition,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the responsive change (both entries).
    PlainLanguageMeaning,
}

impl M5ResponsiveRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WindowClass,
        Self::WidthBounds,
        Self::ShellZoneCoexistence,
        Self::CollapseTarget,
        Self::CollapseOrder,
        Self::IdentityTransition,
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
            Self::WindowClass => "window_class",
            Self::WidthBounds => "width_bounds",
            Self::ShellZoneCoexistence => "shell_zone_coexistence",
            Self::CollapseTarget => "collapse_target",
            Self::CollapseOrder => "collapse_order",
            Self::IdentityTransition => "identity_transition",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a window
/// class, shell-zone coexistence, collapse step, or a degraded responsive entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveRegistryNextAction {
    /// Expand the responsive change's plain-language meaning.
    ExpandResponsiveMeaning,
    /// Inspect the window class, width bounds, or collapse step the entry maps.
    InspectClassOrCollapse,
    /// Complete the title / rail / sidebar / workspace / inspector / panel / status shell-zone coexistence.
    CompleteShellZoneCoexistence,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ResponsiveRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandResponsiveMeaning,
        Self::InspectClassOrCollapse,
        Self::CompleteShellZoneCoexistence,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandResponsiveMeaning => "expand_responsive_meaning",
            Self::InspectClassOrCollapse => "inspect_class_or_collapse",
            Self::CompleteShellZoneCoexistence => "complete_shell_zone_coexistence",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The geometry families covered.
    GeometryFamilies,
    /// The window classes carried.
    WindowClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The width bounds carried.
    WidthBounds,
    /// The shell zones carried.
    ShellZones,
    /// The collapse targets carried.
    CollapseTargets,
    /// The render / surface context.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ResponsiveRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::WindowClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::WidthBounds,
        Self::ShellZones,
        Self::CollapseTargets,
        Self::SurfaceContext,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::WindowClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::GeometryFamilies => "geometry_families",
            Self::WindowClasses => "window_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::WidthBounds => "width_bounds",
            Self::ShellZones => "shell_zones",
            Self::CollapseTargets => "collapse_targets",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a window-class entry degraded below a clean, registry-bound state. The degrade-first ladder returns
/// one of these instead of ever letting a recovery-dropping, identity-dropping, hover-only, unusable-pane,
/// private-bound, or zone-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowClassEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the window class means.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The window class is unclassified (not in the preserved taxonomy).
    ClassUnclassified,
    /// The responsive change drops recovery-critical state.
    DropsRecoveryCriticalState,
    /// The responsive change drops task identity.
    DropsTaskIdentity,
    /// An essential action became hover-only at this class.
    EssentialActionBecomesHoverOnly,
    /// A compare / editor group silently narrowed into an unusable pane at this class.
    EditorGroupNarrowsIntoUnusablePane,
    /// The declared width bounds drift from the canonical class bounds (a private breakpoint).
    BoundsOutsideCanonicalClass,
    /// The title / rail / sidebar / workspace / inspector / panel / status shell-zone coexistence is
    /// incomplete.
    ShellZoneCoexistenceIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WindowClassEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ClassUnclassified,
        Self::DropsRecoveryCriticalState,
        Self::DropsTaskIdentity,
        Self::EssentialActionBecomesHoverOnly,
        Self::EditorGroupNarrowsIntoUnusablePane,
        Self::BoundsOutsideCanonicalClass,
        Self::ShellZoneCoexistenceIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ClassUnclassified => "class_unclassified",
            Self::DropsRecoveryCriticalState => "drops_recovery_critical_state",
            Self::DropsTaskIdentity => "drops_task_identity",
            Self::EssentialActionBecomesHoverOnly => "essential_action_becomes_hover_only",
            Self::EditorGroupNarrowsIntoUnusablePane => "editor_group_narrows_into_unusable_pane",
            Self::BoundsOutsideCanonicalClass => "bounds_outside_canonical_class",
            Self::ShellZoneCoexistenceIncomplete => "shell_zone_coexistence_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ResponsiveRegistryNextAction {
        match self {
            Self::TokenUnstated => M5ResponsiveRegistryNextAction::TraceCanonicalRegistry,
            Self::ClassUnclassified
            | Self::DropsRecoveryCriticalState
            | Self::DropsTaskIdentity
            | Self::EssentialActionBecomesHoverOnly
            | Self::EditorGroupNarrowsIntoUnusablePane
            | Self::BoundsOutsideCanonicalClass => {
                M5ResponsiveRegistryNextAction::InspectClassOrCollapse
            }
            Self::ShellZoneCoexistenceIncomplete => {
                M5ResponsiveRegistryNextAction::CompleteShellZoneCoexistence
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ResponsiveRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated | Self::SurfaceContextUnresolved => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ClassUnclassified | Self::ShellZoneCoexistenceIncomplete => {
                M5ShellGeometryDowngradeTrigger::ResponsiveClassUnstated
            }
            Self::DropsRecoveryCriticalState | Self::DropsTaskIdentity => {
                M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState
            }
            Self::EssentialActionBecomesHoverOnly => {
                M5ShellGeometryDowngradeTrigger::PrimaryWorkflowHiddenBehindOverlayOnlyFallback
            }
            Self::EditorGroupNarrowsIntoUnusablePane => {
                M5ShellGeometryDowngradeTrigger::ZoneStarvedMainWorkspace
            }
            Self::BoundsOutsideCanonicalClass => {
                M5ShellGeometryDowngradeTrigger::ExtensionSetPrivateFracturingWidth
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a collapse-step entry degraded below a clean, identity-stable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollapseStepEntryDegradeReason {
    /// The canonical registry token name is unstated.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The collapse target is unclassified (not in the preserved taxonomy).
    TargetUnclassified,
    /// The identity-transition form is unclassified (not in the preserved taxonomy).
    FormUnclassified,
    /// An extension or embedded surface set a private width that fractures the layout.
    FracturesLayoutWithPrivateWidth,
    /// A protected target (path / branch / trust / target identity or the editor workspace) collapsed.
    CollapsesProtectedTarget,
    /// A docked / sheet / overlay / temporary-panel transition dropped the surface's identity, state,
    /// history, or keyboard route.
    DropsIdentityStateOrRoute,
    /// The collapse starved the main workspace below its minimum.
    StarvesMainWorkspace,
    /// A primary workflow was hidden behind an overlay-only fallback.
    OverlayOnlyPrimaryFallback,
    /// The declared collapse-order rank drifts from the canonical priority order.
    CollapseOrderOutsideCanonical,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CollapseStepEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::TargetUnclassified,
        Self::FormUnclassified,
        Self::FracturesLayoutWithPrivateWidth,
        Self::CollapsesProtectedTarget,
        Self::DropsIdentityStateOrRoute,
        Self::StarvesMainWorkspace,
        Self::OverlayOnlyPrimaryFallback,
        Self::CollapseOrderOutsideCanonical,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::TargetUnclassified => "target_unclassified",
            Self::FormUnclassified => "form_unclassified",
            Self::FracturesLayoutWithPrivateWidth => "fractures_layout_with_private_width",
            Self::CollapsesProtectedTarget => "collapses_protected_target",
            Self::DropsIdentityStateOrRoute => "drops_identity_state_or_route",
            Self::StarvesMainWorkspace => "starves_main_workspace",
            Self::OverlayOnlyPrimaryFallback => "overlay_only_primary_fallback",
            Self::CollapseOrderOutsideCanonical => "collapse_order_outside_canonical",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ResponsiveRegistryNextAction {
        match self {
            Self::TokenUnstated => M5ResponsiveRegistryNextAction::TraceCanonicalRegistry,
            Self::TargetUnclassified
            | Self::FormUnclassified
            | Self::FracturesLayoutWithPrivateWidth
            | Self::CollapsesProtectedTarget
            | Self::DropsIdentityStateOrRoute
            | Self::StarvesMainWorkspace
            | Self::OverlayOnlyPrimaryFallback
            | Self::CollapseOrderOutsideCanonical => {
                M5ResponsiveRegistryNextAction::InspectClassOrCollapse
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ResponsiveRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::TargetUnclassified
            | Self::FormUnclassified => M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated,
            Self::FracturesLayoutWithPrivateWidth => {
                M5ShellGeometryDowngradeTrigger::ExtensionSetPrivateFracturingWidth
            }
            Self::CollapsesProtectedTarget | Self::DropsIdentityStateOrRoute => {
                M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState
            }
            Self::StarvesMainWorkspace => M5ShellGeometryDowngradeTrigger::ZoneStarvedMainWorkspace,
            Self::OverlayOnlyPrimaryFallback => {
                M5ShellGeometryDowngradeTrigger::PrimaryWorkflowHiddenBehindOverlayOnlyFallback
            }
            Self::CollapseOrderOutsideCanonical => {
                M5ShellGeometryDowngradeTrigger::MetricCopiedByHandAcrossPackages
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_window_class_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WindowClassEntryResolutionInput {
    /// Stable identity of the window-class-registry entry.
    pub entry_id: String,
    /// The canonical registry token name (e.g. `shell.responsive.standard.class`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The responsive-geometry role (from the frozen matrix vocabulary).
    pub responsive_geometry_role: M5ResponsiveGeometryRole,
    /// The window class this entry maps.
    pub window_class: M5WindowClass,
    /// The render / surface context.
    pub surface_context: M5ResponsiveSurfaceContext,
    /// The declared lower width bound in logical pixels.
    pub min_width_px: u32,
    /// The declared upper width bound in logical pixels.
    pub max_width_px: u32,
    /// The shell zones this entry keeps coexisting (must cover every zone).
    pub coexisting_zones: Vec<M5ResponsiveShellZone>,
    /// True when the responsive change preserves task identity.
    pub preserves_task_identity: bool,
    /// True when the responsive change preserves recovery-critical state.
    pub preserves_recovery_critical_state: bool,
    /// True when an essential action became hover-only at this class (a hard invariant when `true`).
    pub makes_essential_action_hover_only: bool,
    /// True when a compare / editor group narrowed into an unusable pane (a hard invariant when `true`).
    pub narrows_editor_group_into_unusable_pane: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe window-class-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWindowClassEntry {
    /// Stable identity of the window-class-registry entry.
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
    /// The window-class token named by the entry.
    pub window_class: String,
    /// Whether the window class is classified into the preserved taxonomy.
    pub window_class_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The declared lower width bound in logical pixels.
    pub min_width_px: u32,
    /// The declared upper width bound in logical pixels.
    pub max_width_px: u32,
    /// The canonical lower width bound for this window class.
    pub canonical_min_width_px: u32,
    /// The canonical upper width bound for this window class.
    pub canonical_max_width_px: u32,
    /// Whether the declared bounds match the canonical class bounds.
    pub matches_canonical_bounds: bool,
    /// The shell-zone tokens covered by the entry.
    pub coexisting_zones: Vec<String>,
    /// Whether the entry covers every shell zone.
    pub covers_all_zones: bool,
    /// Whether the responsive change preserves task identity.
    pub preserves_task_identity: bool,
    /// Whether the responsive change preserves recovery-critical state.
    pub preserves_recovery_critical_state: bool,
    /// Whether an essential action became hover-only at this class.
    pub makes_essential_action_hover_only: bool,
    /// Whether a compare / editor group narrowed into an unusable pane at this class.
    pub narrows_editor_group_into_unusable_pane: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5WindowClassEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ResponsiveRegistryNextAction,
    /// Whether the responsive change preserves task identity and recovery-critical state at this class
    /// (clean entry naming every fact).
    pub responsive_change_preserves_task_identity: bool,
}

impl M5ResolvedWindowClassEntry {
    /// Whether this window-class entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_collapse_step_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollapseStepEntryResolutionInput {
    /// Stable identity of the collapse-step entry.
    pub entry_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The collapse-priority role (from the frozen matrix vocabulary).
    pub collapse_priority_role: M5CollapsePriorityRole,
    /// The collapse target this entry maps.
    pub collapse_target: M5CollapseTarget,
    /// The identity-transition form this entry maps.
    pub transition_form: M5IdentityTransitionForm,
    /// The render / surface context.
    pub surface_context: M5ResponsiveSurfaceContext,
    /// True when this step actually collapses the target (false for a protected / dominant target).
    pub collapses: bool,
    /// The declared collapse-order rank for the target.
    pub declared_collapse_rank: u32,
    /// True when the transition preserves the surface's identity, state, history, and keyboard route.
    pub preserves_identity_state_and_keyboard_route: bool,
    /// True when the collapse starved the main workspace below its minimum (a hard invariant when `true`).
    pub starves_main_workspace: bool,
    /// True when the step used a private width that fractures the layout (a hard invariant when `true`).
    pub uses_private_fracturing_width: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe collapse-step-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCollapseStepEntry {
    /// Stable identity of the collapse-step entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve task identity when density changes or the layout collapses.
    pub semantic_role_preserves_task_identity_under_collapse: bool,
    /// The collapse-priority-role token named by the entry.
    pub collapse_priority_role: String,
    /// Whether the collapse-priority role names the disallowed private-width-that-fractures-layout token.
    pub collapse_role_fractures_layout: bool,
    /// The collapse-target token named by the entry.
    pub collapse_target: String,
    /// Whether the collapse target is classified into the preserved taxonomy.
    pub target_is_classified: bool,
    /// Whether the collapse target is protected and must never collapse.
    pub target_is_protected: bool,
    /// The identity-transition-form token named by the entry.
    pub transition_form: String,
    /// Whether the transition form is classified into the preserved taxonomy.
    pub form_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether this step collapses the target.
    pub collapses: bool,
    /// The declared collapse-order rank for the target.
    pub declared_collapse_rank: u32,
    /// The canonical collapse-order rank for the target, if any.
    pub canonical_collapse_rank: Option<u32>,
    /// Whether the declared collapse-order rank matches the canonical priority order.
    pub matches_canonical_order: bool,
    /// Whether the transition preserves identity, state, history, and keyboard route.
    pub preserves_identity_state_and_keyboard_route: bool,
    /// Whether the collapse keeps the main workspace dominant.
    pub keeps_main_workspace_dominant: bool,
    /// Whether a primary workflow was hidden behind an overlay-only fallback.
    pub hides_primary_workflow_behind_overlay_only: bool,
    /// Degrade reason, if the entry could not read as a clean, identity-stable state.
    pub degrade_reason: Option<M5CollapseStepEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ResponsiveRegistryNextAction,
    /// Whether the transition is identity-stable (clean entry naming every fact).
    pub transition_is_identity_stable: bool,
}

impl M5ResolvedCollapseStepEntry {
    /// Whether this collapse-step entry reads as a clean, identity-stable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ResponsiveResolutionError {
    /// The window-class-entry id was empty.
    EmptyWindowClassEntryId,
    /// The collapse-step-entry id was empty.
    EmptyCollapseStepEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ResponsiveResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWindowClassEntryId => "empty_window_class_entry_id",
            Self::EmptyCollapseStepEntryId => "empty_collapse_step_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ResponsiveResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 responsive-geometry / collapse-priority registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ResponsiveResolutionError {}

fn zone_tokens(zones: &[M5ResponsiveShellZone]) -> Vec<String> {
    zones.iter().map(|z| z.as_str().to_owned()).collect()
}

fn covers_all_zones(zones: &[M5ResponsiveShellZone]) -> bool {
    let present: BTreeSet<M5ResponsiveShellZone> = zones.iter().copied().collect();
    M5ResponsiveShellZone::ALL
        .iter()
        .all(|zone| present.contains(zone))
}

/// Whether the declared width bounds exactly match the canonical bounds for this class. Window classes are
/// tokenized, not free-form: a class either declares its canonical lower / upper bound or it drifts into a
/// private breakpoint that must degrade.
fn matches_canonical_bounds(class: M5WindowClass, min_width_px: u32, max_width_px: u32) -> bool {
    if !class.is_classified() {
        return false;
    }
    let bounds = class.canonical_bounds();
    bounds.min_width_px == min_width_px && bounds.max_width_px == max_width_px
}

/// Resolves a window-class-registry entry so it stays bound to the shared registry: the entry names its
/// canonical token, semantic role, responsive-geometry role, and window class, declares the exact canonical
/// width bounds for that class, covers every shell zone, preserves task identity and recovery-critical state,
/// and never makes an essential action hover-only or narrows a compare / editor group into an unusable pane.
pub fn resolve_window_class_entry(
    input: M5WindowClassEntryResolutionInput,
) -> Result<M5ResolvedWindowClassEntry, M5ResponsiveResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ResponsiveResolutionError::EmptyWindowClassEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ResponsiveResolutionError::ForbiddenMaterial);
    }

    let role_drops_recovery = matches!(
        input.responsive_geometry_role,
        M5ResponsiveGeometryRole::ResponsiveChangeDropsRecoveryStateDisallowed
    );
    let bounds = input.window_class.canonical_bounds();
    let matches_bounds =
        matches_canonical_bounds(input.window_class, input.min_width_px, input.max_width_px);
    let all_zones = covers_all_zones(&input.coexisting_zones);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WindowClassEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WindowClassEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.window_class.is_classified() {
        Some(M5WindowClassEntryDegradeReason::ClassUnclassified)
    } else if role_drops_recovery || !input.preserves_recovery_critical_state {
        Some(M5WindowClassEntryDegradeReason::DropsRecoveryCriticalState)
    } else if !input.preserves_task_identity {
        Some(M5WindowClassEntryDegradeReason::DropsTaskIdentity)
    } else if input.makes_essential_action_hover_only {
        Some(M5WindowClassEntryDegradeReason::EssentialActionBecomesHoverOnly)
    } else if input.narrows_editor_group_into_unusable_pane {
        Some(M5WindowClassEntryDegradeReason::EditorGroupNarrowsIntoUnusablePane)
    } else if !matches_bounds {
        Some(M5WindowClassEntryDegradeReason::BoundsOutsideCanonicalClass)
    } else if !all_zones {
        Some(M5WindowClassEntryDegradeReason::ShellZoneCoexistenceIncomplete)
    } else if !input.proof_fresh {
        Some(M5WindowClassEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ResponsiveRegistryNextAction::ExpandResponsiveMeaning,
    };

    Ok(M5ResolvedWindowClassEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        responsive_geometry_role: input.responsive_geometry_role.as_str().to_owned(),
        responsive_role_drops_recovery_state: role_drops_recovery,
        window_class: input.window_class.as_str().to_owned(),
        window_class_is_classified: input.window_class.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        min_width_px: input.min_width_px,
        max_width_px: input.max_width_px,
        canonical_min_width_px: bounds.min_width_px,
        canonical_max_width_px: bounds.max_width_px,
        matches_canonical_bounds: matches_bounds,
        coexisting_zones: zone_tokens(&input.coexisting_zones),
        covers_all_zones: all_zones,
        preserves_task_identity: input.preserves_task_identity,
        preserves_recovery_critical_state: input.preserves_recovery_critical_state,
        makes_essential_action_hover_only: input.makes_essential_action_hover_only,
        narrows_editor_group_into_unusable_pane: input.narrows_editor_group_into_unusable_pane,
        degrade_reason,
        next_action,
        responsive_change_preserves_task_identity: degrade_reason.is_none(),
    })
}

/// Resolves a collapse-step entry so a docked / sheet / overlay / temporary-panel transition stays
/// identity-stable: the entry names its canonical token, collapse-priority role, collapse target, and
/// transition form, declares the canonical collapse-order rank for that target, keeps the main workspace
/// dominant, never collapses a protected target or hides a primary workflow behind an overlay-only fallback,
/// and preserves the surface's identity, state, history, and keyboard route.
pub fn resolve_collapse_step_entry(
    input: M5CollapseStepEntryResolutionInput,
) -> Result<M5ResolvedCollapseStepEntry, M5ResponsiveResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ResponsiveResolutionError::EmptyCollapseStepEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ResponsiveResolutionError::ForbiddenMaterial);
    }

    let role_fractures = matches!(
        input.collapse_priority_role,
        M5CollapsePriorityRole::PrivateWidthThatFracturesLayoutDisallowed
    );
    let canonical_rank = input.collapse_target.canonical_collapse_rank();
    let matches_order = match canonical_rank {
        Some(rank) => input.declared_collapse_rank == rank,
        None => true,
    };
    let target_is_protected = input.collapse_target.is_protected();
    let hides_primary = input.transition_form.is_overlay_only()
        && input.collapse_target.is_primary_workflow()
        && input.collapses;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CollapseStepEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CollapseStepEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.collapse_target.is_classified() {
        Some(M5CollapseStepEntryDegradeReason::TargetUnclassified)
    } else if !input.transition_form.is_classified() {
        Some(M5CollapseStepEntryDegradeReason::FormUnclassified)
    } else if role_fractures || input.uses_private_fracturing_width {
        Some(M5CollapseStepEntryDegradeReason::FracturesLayoutWithPrivateWidth)
    } else if target_is_protected && input.collapses {
        Some(M5CollapseStepEntryDegradeReason::CollapsesProtectedTarget)
    } else if !input.preserves_identity_state_and_keyboard_route {
        Some(M5CollapseStepEntryDegradeReason::DropsIdentityStateOrRoute)
    } else if input.starves_main_workspace {
        Some(M5CollapseStepEntryDegradeReason::StarvesMainWorkspace)
    } else if hides_primary {
        Some(M5CollapseStepEntryDegradeReason::OverlayOnlyPrimaryFallback)
    } else if !matches_order {
        Some(M5CollapseStepEntryDegradeReason::CollapseOrderOutsideCanonical)
    } else if !input.proof_fresh {
        Some(M5CollapseStepEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ResponsiveRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCollapseStepEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        collapse_priority_role: input.collapse_priority_role.as_str().to_owned(),
        collapse_role_fractures_layout: role_fractures,
        collapse_target: input.collapse_target.as_str().to_owned(),
        target_is_classified: input.collapse_target.is_classified(),
        target_is_protected,
        transition_form: input.transition_form.as_str().to_owned(),
        form_is_classified: input.transition_form.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        collapses: input.collapses,
        declared_collapse_rank: input.declared_collapse_rank,
        canonical_collapse_rank: canonical_rank,
        matches_canonical_order: matches_order,
        preserves_identity_state_and_keyboard_route: input
            .preserves_identity_state_and_keyboard_route,
        keeps_main_workspace_dominant: !input.starves_main_workspace,
        hides_primary_workflow_behind_overlay_only: hides_primary,
        degrade_reason,
        next_action,
        transition_is_identity_stable: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved window-class and collapse-step entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ResponsiveGeometryRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5ResponsiveRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ResponsiveRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    /// Resolved window-class-registry examples.
    pub window_class_entries: Vec<M5ResolvedWindowClassEntry>,
    /// Resolved collapse-step examples.
    pub collapse_step_entries: Vec<M5ResolvedCollapseStepEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical density-mode domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a responsive change or collapse never alters command meaning, focus order, or trust
    /// visibility. MUST be `false`.
    pub responsive_or_collapse_alters_command_focus_or_trust: bool,
    /// Hard invariant: an extension or embedded surface never sets a private width that fractures the layout.
    /// MUST be `false`.
    pub extension_sets_private_fracturing_width: bool,
    /// Hard invariant: a collapse never lets a zone starve the main workspace below its minimum. MUST be
    /// `false`.
    pub lets_zone_starve_main_workspace_below_minimum: bool,
    /// Hard invariant: a primary workflow is never hidden behind an overlay-only fallback. MUST be `false`.
    pub hides_primary_workflow_behind_overlay_only_fallback: bool,
}

impl M5ResponsiveGeometryAndCollapsePriorityRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ResponsiveRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ResponsiveRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ResponsiveRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5ResponsiveRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.responsive_or_collapse_alters_command_focus_or_trust
            && !self.extension_sets_private_fracturing_width
            && !self.lets_zone_starve_main_workspace_below_minimum
            && !self.hides_primary_workflow_behind_overlay_only_fallback
    }

    /// True when a clean window-class entry preserves registry-bound geometry: it keeps a classified class,
    /// never names the disallowed drops-recovery role, matches the canonical bounds, covers every shell
    /// zone, preserves task identity and recovery-critical state, and never makes an essential action
    /// hover-only or narrows a compare / editor group into an unusable pane.
    fn window_class_is_honest(ex: &M5ResolvedWindowClassEntry) -> bool {
        !ex.is_clean()
            || (ex.window_class_is_classified
                && !ex.responsive_role_drops_recovery_state
                && ex.matches_canonical_bounds
                && ex.covers_all_zones
                && ex.preserves_task_identity
                && ex.preserves_recovery_critical_state
                && !ex.makes_essential_action_hover_only
                && !ex.narrows_editor_group_into_unusable_pane)
    }

    /// True when a clean collapse-step entry preserves identity-stable geometry: it keeps a classified
    /// target and form, never fractures the layout, never collapses a protected target, preserves identity /
    /// state / route, keeps the main workspace dominant, never hides a primary workflow behind an
    /// overlay-only fallback, and matches the canonical collapse order.
    fn collapse_step_is_honest(ex: &M5ResolvedCollapseStepEntry) -> bool {
        !ex.is_clean()
            || (ex.target_is_classified
                && ex.form_is_classified
                && !ex.collapse_role_fractures_layout
                && !(ex.target_is_protected && ex.collapses)
                && ex.preserves_identity_state_and_keyboard_route
                && ex.keeps_main_workspace_dominant
                && !ex.hides_primary_workflow_behind_overlay_only
                && ex.matches_canonical_order)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.window_class_entries
            .iter()
            .all(Self::window_class_is_honest)
            && self
                .collapse_step_entries
                .iter()
                .all(Self::collapse_step_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Responsive-geometry-role tokens (bound from the frozen matrix).
    pub responsive_geometry_roles: Vec<String>,
    /// Collapse-priority-role tokens (bound from the frozen matrix).
    pub collapse_priority_roles: Vec<String>,
    /// Window-class tokens (minted by this lane).
    pub window_classes: Vec<String>,
    /// Shell-zone tokens (minted by this lane).
    pub shell_zones: Vec<String>,
    /// Collapse-target tokens (minted by this lane).
    pub collapse_targets: Vec<String>,
    /// Identity-transition-form tokens (minted by this lane).
    pub transition_forms: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Window-class-entry degrade-reason tokens.
    pub window_class_degrade_reasons: Vec<String>,
    /// Collapse-step-entry degrade-reason tokens.
    pub collapse_step_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ResponsiveGeometryAndCollapsePriorityRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ShellGeometryRole::ALL, |v| v.as_str()),
            responsive_geometry_roles: tokens(&M5ResponsiveGeometryRole::ALL, |v| v.as_str()),
            collapse_priority_roles: tokens(&M5CollapsePriorityRole::ALL, |v| v.as_str()),
            window_classes: tokens(&M5WindowClass::ALL, |v| v.as_str()),
            shell_zones: tokens(&M5ResponsiveShellZone::ALL, |v| v.as_str()),
            collapse_targets: tokens(&M5CollapseTarget::ALL, |v| v.as_str()),
            transition_forms: tokens(&M5IdentityTransitionForm::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ResponsiveSurfaceContext::ALL, |v| v.as_str()),
            window_class_degrade_reasons: tokens(&M5WindowClassEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            collapse_step_degrade_reasons: tokens(&M5CollapseStepEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ResponsiveRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ResponsiveRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ResponsiveRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesGovernanceReview {
    /// The registry names a canonical token, role, and window class / collapse target for every entry.
    pub registry_names_token_role_and_class: bool,
    /// The canonical window-class bounds are encoded as logical-pixel tokens before OS scaling.
    pub window_class_bounds_encoded_as_logical_pixel_tokens: bool,
    /// Every claimed surface resolves its window class from the shared registry.
    pub every_surface_resolves_from_shared_registry: bool,
    /// Responsive behavior preserves task identity and recovery-critical state.
    pub responsive_preserves_task_identity_and_recovery_state: bool,
    /// No essential action becomes hover-only and no compare / editor group narrows into an unusable pane.
    pub no_hover_only_action_or_unusable_pane: bool,
    /// The declared collapse priority order is honored across every surface.
    pub declared_collapse_priority_order_honored: bool,
    /// Docked / sheet / overlay / temporary-panel transitions stay identity-stable.
    pub transitions_stay_identity_stable: bool,
    /// The main workspace stays dominant and is never starved by a collapse.
    pub main_workspace_stays_dominant: bool,
    /// No primary workflow is hidden behind an overlay-only fallback.
    pub no_primary_workflow_hidden_behind_overlay_only_fallback: bool,
    /// Extension or embedded surfaces cannot invent private widths that fracture the layout.
    pub extension_cannot_invent_private_fracturing_width: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesConsumerProjection {
    /// The shell surface consumes the shared responsive registries.
    pub shell_consumes_shared_registries: bool,
    /// The editor surface consumes the shared responsive registries.
    pub editor_consumes_shared_registries: bool,
    /// The review surface consumes the shared responsive registries.
    pub review_consumes_shared_registries: bool,
    /// The notebook and data surfaces consume the shared responsive registries.
    pub notebook_and_data_consume_shared_registries: bool,
    /// Responsive geometry resolves back to one canonical density-mode domain contract.
    pub geometry_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical responsive registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting shell-geometry audit for the lane.
    pub geometry_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ResponsiveGeometryAndCollapsePriorityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ResponsiveGeometryAndCollapsePriorityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ResponsiveGeometryAndCollapsePriorityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ResponsiveGeometryAndCollapsePriorityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ResponsiveGeometryAndCollapsePriorityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 responsive-geometry / collapse-priority registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket {
    /// Record kind; must equal [`M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ResponsiveGeometryAndCollapsePriorityRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ResponsiveGeometryAndCollapsePriorityRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ResponsiveGeometryAndCollapsePriorityRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ResponsiveGeometryAndCollapsePriorityRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ResponsiveGeometryAndCollapsePriorityRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ResponsiveGeometryAndCollapsePriorityRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_RECORD_KIND {
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::VocabularySetDrift,
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
                .expect("m5 responsive-geometry / collapse-priority registries packet serializes"),
        ) {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::RawMaterialInExport,
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
            .expect("m5 responsive-geometry / collapse-priority registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,window_class_entries,collapse_step_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .window_class_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.collapse_step_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.window_class_entries.len(),
                row.collapse_step_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Responsive-Geometry and Collapse-Priority Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Window classes: {}\n",
            self.vocabulary_set.window_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Collapse targets: {}\n",
            self.vocabulary_set.collapse_targets.join(", ")
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
                "  - Window-class entries: {} / collapse-step entries: {}\n",
                row.window_class_entries.len(),
                row.collapse_step_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>),
}

impl fmt::Display for M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 responsive-geometry / collapse-priority registries export parse failed: {error}"
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
                    "m5 responsive-geometry / collapse-priority registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError {}

/// Validation failures emitted by [`M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation {
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
    /// A registry row carries a dishonest clean example (recovery-dropping, identity-dropping, hover-only,
    /// unusable-pane, private-bound, protected-collapse, or overlay-only-fallback).
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
    /// Tokenized window classes across surfaces are not proven: clean window-class entries do not cover the
    /// three canonical window classes or the first shell / editor / review / notebook / data surfaces, no
    /// private-bound example degrades, or a clean entry drifts from the canonical bounds.
    ResponsiveWindowClassesAcrossSurfacesNotProven,
    /// Identity-stable transitions are not proven: clean collapse-step entries do not cover the canonical
    /// collapse order, no identity-dropping example degrades, no hover-only or unusable-pane window example
    /// degrades, or a clean entry drops identity / state / route.
    IdentityStableTransitionsNotProven,
    /// Honest extension degradation is not proven: no private-bound window example and no
    /// starves-workspace collapse example degrade, no clean main-workspace-dominant collapse step exists, or
    /// a clean window entry carries a private bound.
    ExtensionCannotFracturePrivateWidthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation {
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
            Self::ResponsiveWindowClassesAcrossSurfacesNotProven => {
                "responsive_window_classes_across_surfaces_not_proven"
            }
            Self::IdentityStableTransitionsNotProven => "identity_stable_transitions_not_proven",
            Self::ExtensionCannotFracturePrivateWidthNotProven => {
                "extension_cannot_fracture_private_width_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_responsive_geometry_and_collapse_priority_registries_export() -> Result<
    M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError,
> {
    let packet: M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-responsive-geometry-and-collapse-priority-registries-proof/support_export.json"
        )))
        .map_err(M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ResponsiveGeometryAndCollapsePriorityRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_SCHEMA_REF,
        M5_RESPONSIVE_GEOMETRY_AND_COLLAPSE_PRIORITY_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::NoRegistryRows);
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
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DENSITY_MODE_SCHEMA_REF) {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.window_class_entries.is_empty() || row.collapse_step_entries.is_empty() {
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.registry_names_token_role_and_class,
        review.window_class_bounds_encoded_as_logical_pixel_tokens,
        review.every_surface_resolves_from_shared_registry,
        review.responsive_preserves_task_identity_and_recovery_state,
        review.no_hover_only_action_or_unusable_pane,
        review.declared_collapse_priority_order_honored,
        review.transitions_stay_identity_stable,
        review.main_workspace_stays_dominant,
        review.no_primary_workflow_hidden_behind_overlay_only_fallback,
        review.extension_cannot_invent_private_fracturing_width,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
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
            violations
                .push(M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.geometry_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ResponsiveGeometryAndCollapsePriorityRegistriesPacket,
    violations: &mut Vec<M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation>,
) {
    let windows = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.window_class_entries.iter())
    };
    let collapses = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.collapse_step_entries.iter())
    };

    // AC1: Compact / Standard / Expanded window classes produce predictable, tokenized layout across every
    // surface. Clean window-class entries cover the three canonical classes and the first shell / editor /
    // review / notebook / data surfaces, a private-bound example degrades, and no clean entry drifts.
    let clean_classes: BTreeSet<String> = windows()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.window_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = windows()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5WindowClass::CANONICAL_CLASSES
        .iter()
        .all(|c| clean_classes.contains(c.as_str()));
    let first_surfaces_covered = M5ResponsiveSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let bounds_drift_degrades = windows().any(|ex| {
        ex.degrade_reason == Some(M5WindowClassEntryDegradeReason::BoundsOutsideCanonicalClass)
    });
    let no_clean_drift = !windows().any(|ex| ex.is_clean() && !ex.matches_canonical_bounds);
    if !(classes_covered && first_surfaces_covered && bounds_drift_degrades && no_clean_drift) {
        violations.push(
            M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ResponsiveWindowClassesAcrossSurfacesNotProven,
        );
    }

    // AC2: docked / sheet / overlay / temporary-panel transitions stay identity-stable, no essential action
    // becomes hover-only, and no compare / editor group narrows into an unusable pane. Clean collapse-step
    // entries cover the canonical collapse order, an identity-dropping collapse example degrades, a
    // hover-only and an unusable-pane window example degrade, and no clean collapse entry drops identity.
    let clean_ranks: BTreeSet<u32> = collapses()
        .filter(|ex| ex.is_clean())
        .filter_map(|ex| ex.canonical_collapse_rank)
        .collect();
    let order_covered = M5CollapseTarget::ORDERED_COLLAPSE_TARGETS.iter().all(|t| {
        match t.canonical_collapse_rank() {
            Some(rank) => clean_ranks.contains(&rank),
            None => true,
        }
    });
    let drops_identity_degrades = collapses().any(|ex| {
        ex.degrade_reason == Some(M5CollapseStepEntryDegradeReason::DropsIdentityStateOrRoute)
    });
    let no_clean_drops_identity =
        !collapses().any(|ex| ex.is_clean() && !ex.preserves_identity_state_and_keyboard_route);
    let hover_only_degrades = windows().any(|ex| {
        ex.degrade_reason == Some(M5WindowClassEntryDegradeReason::EssentialActionBecomesHoverOnly)
    });
    let unusable_pane_degrades = windows().any(|ex| {
        ex.degrade_reason
            == Some(M5WindowClassEntryDegradeReason::EditorGroupNarrowsIntoUnusablePane)
    });
    if !(order_covered
        && drops_identity_degrades
        && no_clean_drops_identity
        && hover_only_degrades
        && unusable_pane_degrades)
    {
        violations.push(
            M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::IdentityStableTransitionsNotProven,
        );
    }

    // AC3: extension or embedded surfaces that cannot honor the canonical geometry degrade honestly instead
    // of inventing private widths. A private-bound window example and a starves-workspace collapse example
    // both degrade, at least one clean main-workspace-dominant collapse step exists, and no clean window
    // entry carries a private bound.
    let starves_workspace_degrades = collapses().any(|ex| {
        ex.degrade_reason == Some(M5CollapseStepEntryDegradeReason::StarvesMainWorkspace)
    });
    let clean_dominant_exists =
        collapses().any(|ex| ex.is_clean() && ex.keeps_main_workspace_dominant);
    if !(bounds_drift_degrades
        && starves_workspace_degrades
        && clean_dominant_exists
        && no_clean_drift)
    {
        violations.push(
            M5ResponsiveGeometryAndCollapsePriorityRegistriesViolation::ExtensionCannotFracturePrivateWidthNotProven,
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
    M5ShellGeometryFamily::ResponsiveGeometry,
    M5ShellGeometryFamily::CollapsePriority,
];
