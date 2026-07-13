//! Frozen M5 shell-metric, minimum-size, density-mode, responsive-geometry, and adaptive-collapse
//! shell-geometry matrix.
//!
//! This module locks Aureline's concrete shell geometry and density behavior into one export-safe
//! packet. Every claimed M5 desktop surface that still describes its own shell-zone widths, minimum
//! editor / chrome sizes, density modes, responsive window classes, collapse priorities, or hit-target
//! rules — across the desktop shell, editor, review, notebook, and data surfaces — is named once here
//! and constrained by the same shared shell-geometry-role taxonomy (zone, metric, hit_target, density,
//! responsive, collapse, workspace_dominance), the same honor-declared-minimum-and-recommended-sizes
//! rule, the same density-changes-presentation-not-information-architecture rule, the same
//! responsive-preserves-task-identity-and-recovery-state rule, the same
//! never-shrink-hit-targets-below-supported-minimums rule, and the same
//! extension-and-embedded-cannot-invent-private-widths rule regardless of the surface that renders it.
//!
//! The matrix does not redesign navigation content, start-center flows, or native protocol-handler
//! ownership — it is the shared reusable shell-geometry contract those flows consume, and it binds back
//! to the already-landed shell-zone and reusable-shell-primitive packets instead of leaving the geometry
//! split across scattered local constants and screenshots. The controlled vocabularies are frozen in one
//! self-describing [`M5ShellGeometryVocabularySet`] rather than minted per surface. The single controlled
//! shell-geometry-role vocabulary consumers bind to — zone, metric, hit_target, density, responsive,
//! collapse, and workspace_dominance — keeps the main workspace dominant, keeps declared minimum and
//! recommended sizes honored, keeps density changing presentation rather than information architecture,
//! keeps responsive collapse preserving task identity and recovery-critical state, and keeps extension
//! and embedded surfaces from inventing private widths that fracture the shell. Raw secret values and
//! private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_metric_density_matrix,
    seeded_m5_shell_metric_density_matrix_collapse_priority_preview_narrowed,
    seeded_m5_shell_metric_density_matrix_responsive_geometry_beta_narrowed,
    M5_SHELL_METRIC_DENSITY_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ShellMetricDensityMatrixPacket`].
pub const M5_SHELL_METRIC_DENSITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_shell_metric_minimum_size_density_mode_responsive_geometry_and_collapse_priority_matrix";

/// Schema version for M5 shell-metric / density matrix records.
pub const M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined shell-geometry matrix schema.
pub const M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-metric-density-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF: &str =
    "docs/design-system/m5_shell_metric_density_contract.md";

/// Repo-relative path of the canonical shell-metric / minimum-size domain schema.
pub const M5_SHELL_METRICS_SCHEMA_REF: &str = "schemas/shell/m5-shell-metrics.schema.json";

/// Repo-relative path of the canonical density-mode / responsive-geometry / collapse domain schema.
pub const M5_DENSITY_MODE_SCHEMA_REF: &str = "schemas/shell/m5-density-mode.schema.json";

/// Repo-relative path of the already-landed shell-zone schema the matrix binds back to.
pub const M5_SHELL_ZONE_SCHEMA_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the already-landed reusable-shell-primitive schema the matrix binds back to.
pub const M5_SHELL_PRIMITIVES_SCHEMA_REF: &str = "schemas/shell/m5-shell-primitives.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHELL_METRIC_DENSITY_FIXTURE_DIR: &str = "fixtures/ui/m5-shell-metric-density";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHELL_METRIC_DENSITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-metric-density-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SHELL_METRIC_DENSITY_CSV_REF: &str =
    "artifacts/release/m5-shell-metric-density-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SHELL_METRIC_DENSITY_REPORT_REF: &str =
    "artifacts/shell/m5-shell-metric-density-matrix.md";

/// One of the five governed shell-geometry families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryFamily {
    /// Shell-zone metrics: default / minimum / recommended sizes for the canonical shell zones.
    ShellMetric,
    /// Minimum sizes and hit targets: tab minimum width, resize-handle hit area, icon-only hit targets.
    MinimumSize,
    /// Density modes: comfortable / standard / compact presentation without changing information
    /// architecture.
    DensityMode,
    /// Responsive geometry: compact / standard / expanded window classes that preserve task identity.
    ResponsiveGeometry,
    /// Collapse priority: adaptive-collapse ordering and no-fracture geometry that keeps the main
    /// workspace dominant.
    CollapsePriority,
}

impl M5ShellGeometryFamily {
    /// Every governed shell-geometry family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ShellMetric,
        Self::MinimumSize,
        Self::DensityMode,
        Self::ResponsiveGeometry,
        Self::CollapsePriority,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellMetric => "shell_metric",
            Self::MinimumSize => "minimum_size",
            Self::DensityMode => "density_mode",
            Self::ResponsiveGeometry => "responsive_geometry",
            Self::CollapsePriority => "collapse_priority",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's shell-metric, minimum-size, density, responsive, or collapse meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ShellMetric | Self::MinimumSize => M5_SHELL_METRICS_SCHEMA_REF,
            Self::DensityMode | Self::ResponsiveGeometry | Self::CollapsePriority => {
                M5_DENSITY_MODE_SCHEMA_REF
            }
        }
    }

    /// `true` when this family must name a controlled shell-metric role.
    pub const fn declares_shell_metric_roles(self) -> bool {
        matches!(self, Self::ShellMetric)
    }

    /// `true` when this family must name a controlled minimum-size role.
    pub const fn declares_minimum_size_roles(self) -> bool {
        matches!(self, Self::MinimumSize)
    }

    /// `true` when this family must name a controlled density-mode role.
    pub const fn declares_density_mode_roles(self) -> bool {
        matches!(self, Self::DensityMode)
    }

    /// `true` when this family must name a controlled responsive-geometry role.
    pub const fn declares_responsive_geometry_roles(self) -> bool {
        matches!(self, Self::ResponsiveGeometry)
    }

    /// `true` when this family must name a controlled collapse-priority role.
    pub const fn declares_collapse_priority_roles(self) -> bool {
        matches!(self, Self::CollapsePriority)
    }
}

/// The single controlled shell-geometry-role vocabulary every desktop, editor, review, notebook, or data
/// consumer binds to. These are the exact acceptance-criteria tokens that keep `zone`, `metric`,
/// `hit_target`, `density`, `responsive`, `collapse`, and `workspace_dominance` meaning the same thing
/// everywhere the shell-geometry grammar ships. No surface invents a parallel word for any of these
/// roles, and the collapse-sensitive roles may never drop task identity or recovery-critical state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryRole {
    /// Shell-zone role (title / context bar, rail, sidebar, main workspace, inspector, panel, status).
    Zone,
    /// Size / spacing metric role (default / minimum / recommended / maximum).
    Metric,
    /// Minimum hit-target role (tab width, resize handle, icon-only target).
    HitTarget,
    /// Density-mode role.
    Density,
    /// Responsive window-class role.
    Responsive,
    /// Adaptive-collapse-priority role.
    Collapse,
    /// Main-workspace-dominance role.
    WorkspaceDominance,
}

impl M5ShellGeometryRole {
    /// Every shell-geometry role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Zone,
        Self::Metric,
        Self::HitTarget,
        Self::Density,
        Self::Responsive,
        Self::Collapse,
        Self::WorkspaceDominance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Zone => "zone",
            Self::Metric => "metric",
            Self::HitTarget => "hit_target",
            Self::Density => "density",
            Self::Responsive => "responsive",
            Self::Collapse => "collapse",
            Self::WorkspaceDominance => "workspace_dominance",
        }
    }

    /// Whether this role carries adaptive behavior that must never drop task identity, focus order, trust
    /// visibility, or recovery-critical state when density changes or the layout collapses (`density`,
    /// `responsive`, `collapse`, `workspace_dominance`). The purely-static geometry roles (`zone`,
    /// `metric`, `hit_target`) do not collapse and so do not carry this requirement.
    pub const fn must_preserve_task_identity_under_collapse(self) -> bool {
        matches!(
            self,
            Self::Density | Self::Responsive | Self::Collapse | Self::WorkspaceDominance
        )
    }
}

/// Controlled shell-metric role — how a shell-zone size is named, so title / context bar, rail, sidebar,
/// main editor group, right inspector, bottom panel, and status bar honor one declared default / minimum
/// / recommended size bound to the registry rather than a hand-copied constant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellMetricRole {
    /// The default size for a shell zone.
    DefaultSize,
    /// The minimum size for a shell zone.
    MinimumSize,
    /// The recommended size for a shell zone.
    RecommendedSize,
    /// The maximum size for a shell zone.
    MaximumSize,
    /// A metric bound to the single shell-metric registry.
    BoundToRegistry,
    /// A metric hand-copied as a scattered local constant, which is disallowed.
    HandCopiedConstantDisallowed,
}

impl M5ShellMetricRole {
    /// Every shell-metric role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DefaultSize,
        Self::MinimumSize,
        Self::RecommendedSize,
        Self::MaximumSize,
        Self::BoundToRegistry,
        Self::HandCopiedConstantDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultSize => "default_size",
            Self::MinimumSize => "minimum_size",
            Self::RecommendedSize => "recommended_size",
            Self::MaximumSize => "maximum_size",
            Self::BoundToRegistry => "bound_to_registry",
            Self::HandCopiedConstantDisallowed => "hand_copied_constant_disallowed",
        }
    }
}

/// Controlled minimum-size role — how minimum hit targets are named, so tab minimum width, resize-handle
/// hit area, and icon-only hit targets stay reachable and never shrink below the supported minimum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MinimumSizeRole {
    /// The tab minimum width.
    TabMinimumWidth,
    /// The resize-handle hit area.
    ResizeHandleHitArea,
    /// The icon-only hit target.
    IconOnlyHitTarget,
    /// A target reachable by both pointer and keyboard.
    PointerAndKeyboardReachable,
    /// A target that meets the supported minimum.
    MeetsSupportedMinimum,
    /// A target shrunk below the supported minimum, which is disallowed.
    ShrinksBelowMinimumDisallowed,
}

impl M5MinimumSizeRole {
    /// Every minimum-size role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TabMinimumWidth,
        Self::ResizeHandleHitArea,
        Self::IconOnlyHitTarget,
        Self::PointerAndKeyboardReachable,
        Self::MeetsSupportedMinimum,
        Self::ShrinksBelowMinimumDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TabMinimumWidth => "tab_minimum_width",
            Self::ResizeHandleHitArea => "resize_handle_hit_area",
            Self::IconOnlyHitTarget => "icon_only_hit_target",
            Self::PointerAndKeyboardReachable => "pointer_and_keyboard_reachable",
            Self::MeetsSupportedMinimum => "meets_supported_minimum",
            Self::ShrinksBelowMinimumDisallowed => "shrinks_below_minimum_disallowed",
        }
    }
}

/// Controlled density-mode role — how density modes are named, so comfortable, standard, and compact
/// modes change presentation only and never rearrange the information architecture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityModeRole {
    /// The comfortable density mode.
    ComfortableMode,
    /// The standard density mode.
    StandardMode,
    /// The compact density mode.
    CompactMode,
    /// A density change that is presentation-only.
    PresentationOnlyChange,
    /// A density change that preserves the information architecture.
    PreservesInformationArchitecture,
    /// A density change that rearranges information architecture, which is disallowed.
    DensityChangesInformationArchitectureDisallowed,
}

impl M5DensityModeRole {
    /// Every density-mode role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComfortableMode,
        Self::StandardMode,
        Self::CompactMode,
        Self::PresentationOnlyChange,
        Self::PreservesInformationArchitecture,
        Self::DensityChangesInformationArchitectureDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComfortableMode => "comfortable_mode",
            Self::StandardMode => "standard_mode",
            Self::CompactMode => "compact_mode",
            Self::PresentationOnlyChange => "presentation_only_change",
            Self::PreservesInformationArchitecture => "preserves_information_architecture",
            Self::DensityChangesInformationArchitectureDisallowed => {
                "density_changes_information_architecture_disallowed"
            }
        }
    }
}

/// Controlled responsive-geometry role — how responsive window classes are named, so compact, standard,
/// and expanded classes preserve task identity and recovery-critical state under snapped or narrow widths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResponsiveGeometryRole {
    /// The compact window class.
    CompactClass,
    /// The standard window class.
    StandardClass,
    /// The expanded window class.
    ExpandedClass,
    /// A responsive change that preserves task identity.
    PreservesTaskIdentity,
    /// A responsive change that preserves recovery-critical state.
    PreservesRecoveryCriticalState,
    /// A responsive change that drops recovery-critical state, which is disallowed.
    ResponsiveChangeDropsRecoveryStateDisallowed,
}

impl M5ResponsiveGeometryRole {
    /// Every responsive-geometry role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CompactClass,
        Self::StandardClass,
        Self::ExpandedClass,
        Self::PreservesTaskIdentity,
        Self::PreservesRecoveryCriticalState,
        Self::ResponsiveChangeDropsRecoveryStateDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompactClass => "compact_class",
            Self::StandardClass => "standard_class",
            Self::ExpandedClass => "expanded_class",
            Self::PreservesTaskIdentity => "preserves_task_identity",
            Self::PreservesRecoveryCriticalState => "preserves_recovery_critical_state",
            Self::ResponsiveChangeDropsRecoveryStateDisallowed => {
                "responsive_change_drops_recovery_state_disallowed"
            }
        }
    }
}

/// Controlled collapse-priority role — how adaptive-collapse ordering is named, so collapse follows one
/// declared priority, keeps the main workspace dominant, restores on re-expand, and never fractures the
/// shell with a private width.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollapsePriorityRole {
    /// A declared collapse order.
    CollapseOrderDeclared,
    /// The main workspace stays dominant through collapse.
    MainWorkspaceStaysDominant,
    /// Collapse keeps no-fracture geometry.
    NoFractureGeometry,
    /// Collapse avoids hiding a primary workflow behind an overlay-only fallback.
    OverlayOnlyFallbackAvoided,
    /// Collapsed zones restore on re-expand.
    RestoreOnReexpand,
    /// A private width that fractures the layout, which is disallowed.
    PrivateWidthThatFracturesLayoutDisallowed,
}

impl M5CollapsePriorityRole {
    /// Every collapse-priority role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CollapseOrderDeclared,
        Self::MainWorkspaceStaysDominant,
        Self::NoFractureGeometry,
        Self::OverlayOnlyFallbackAvoided,
        Self::RestoreOnReexpand,
        Self::PrivateWidthThatFracturesLayoutDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CollapseOrderDeclared => "collapse_order_declared",
            Self::MainWorkspaceStaysDominant => "main_workspace_stays_dominant",
            Self::NoFractureGeometry => "no_fracture_geometry",
            Self::OverlayOnlyFallbackAvoided => "overlay_only_fallback_avoided",
            Self::RestoreOnReexpand => "restore_on_reexpand",
            Self::PrivateWidthThatFracturesLayoutDisallowed => {
                "private_width_that_fractures_layout_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a shell-geometry family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometrySurfaceFamily {
    /// The desktop shell surface.
    Desktop,
    /// The editor surface.
    Editor,
    /// The review surface.
    Review,
    /// The notebook surface.
    Notebook,
    /// The data surface.
    Data,
    /// The support export.
    SupportExport,
}

impl M5ShellGeometrySurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Desktop,
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Editor => "editor",
            Self::Review => "review",
            Self::Notebook => "notebook",
            Self::Data => "data",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a family must survive with the same truth, so a family's shell-metric, minimum-size,
/// density, responsive, or collapse meaning never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5ShellGeometryDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The editor UI.
    EditorUi,
    /// The review UI.
    ReviewUi,
    /// The notebook UI.
    NotebookUi,
    /// The data UI.
    DataUi,
    /// The settings UI.
    SettingsUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5ShellGeometryConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::EditorUi,
        Self::ReviewUi,
        Self::NotebookUi,
        Self::DataUi,
        Self::SettingsUi,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::EditorUi => "editor_ui",
            Self::ReviewUi => "review_ui",
            Self::NotebookUi => "notebook_ui",
            Self::DataUi => "data_ui",
            Self::SettingsUi => "settings_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no shell-geometry meaning disappears under
/// zoom, snapped widths, keyboard-only use, or export. Records the keyboard, screen-reader, high-zoom,
/// snapped-width, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under snapped or narrow window widths.
    SnappedWidthSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5ShellGeometryAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::SnappedWidthSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::SnappedWidthSafe => "snapped_width_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a shell-geometry family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The shell-metric registry source is unavailable.
    ShellMetricSourceUnavailable,
    /// The density-mode contract is unavailable.
    DensityContractUnavailable,
    /// The responsive window-class source is unavailable.
    ResponsiveClassUnavailable,
    /// The hit-target minimum source is unavailable.
    HitTargetSourceUnavailable,
    /// The collapse-priority order is unverified.
    CollapsePriorityUnverified,
}

impl M5ShellGeometryDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::ShellMetricSourceUnavailable,
        Self::DensityContractUnavailable,
        Self::ResponsiveClassUnavailable,
        Self::HitTargetSourceUnavailable,
        Self::CollapsePriorityUnverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::ShellMetricSourceUnavailable => "shell_metric_source_unavailable",
            Self::DensityContractUnavailable => "density_contract_unavailable",
            Self::ResponsiveClassUnavailable => "responsive_class_unavailable",
            Self::HitTargetSourceUnavailable => "hit_target_source_unavailable",
            Self::CollapsePriorityUnverified => "collapse_priority_unverified",
        }
    }
}

/// Mandatory label a claimed shell-geometry family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// size metric, the density mode, and the responsive class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's shell-geometry role.
    SemanticRole,
    /// The canonical registry reference the family points at.
    RegistryReference,
    /// The size metric (default / minimum / recommended) the family covers.
    SizeMetric,
    /// The density mode the family applies to.
    DensityMode,
    /// The responsive window class the family applies to.
    ResponsiveClass,
}

impl M5ShellGeometryRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::SizeMetric,
        Self::DensityMode,
        Self::ResponsiveClass,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::SizeMetric => "size_metric",
            Self::DensityMode => "density_mode",
            Self::ResponsiveClass => "responsive_class",
        }
    }
}

/// Qualification class for an M5 shell-geometry row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5ShellGeometryQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a shell-geometry family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryDowngradeTrigger {
    /// A density change or collapse changed command meaning, focus order, or trust visibility.
    DensityChangedCommandOrFocusOrTrust,
    /// A responsive collapse dropped recovery-critical state.
    ResponsiveCollapseDroppedRecoveryState,
    /// A zone starved the main workspace below its minimum.
    ZoneStarvedMainWorkspace,
    /// An extension or embedded surface set a private fracturing width.
    ExtensionSetPrivateFracturingWidth,
    /// A hit target shrank below the supported minimum.
    HitTargetShrankBelowMinimum,
    /// A primary workflow was hidden behind an overlay-only fallback.
    PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
    /// A metric was copied by hand across packages.
    MetricCopiedByHandAcrossPackages,
    /// A family left its size metric unstated.
    SizeMetricUnstated,
    /// A family left its density mode unstated.
    DensityModeUnstated,
    /// A family left its responsive class unstated.
    ResponsiveClassUnstated,
    /// A family left its canonical registry reference unstated.
    RegistryReferenceUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ShellGeometryDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::DensityChangedCommandOrFocusOrTrust,
        Self::ResponsiveCollapseDroppedRecoveryState,
        Self::ZoneStarvedMainWorkspace,
        Self::ExtensionSetPrivateFracturingWidth,
        Self::HitTargetShrankBelowMinimum,
        Self::PrimaryWorkflowHiddenBehindOverlayOnlyFallback,
        Self::MetricCopiedByHandAcrossPackages,
        Self::SizeMetricUnstated,
        Self::DensityModeUnstated,
        Self::ResponsiveClassUnstated,
        Self::RegistryReferenceUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DensityChangedCommandOrFocusOrTrust => {
                "density_changed_command_or_focus_or_trust"
            }
            Self::ResponsiveCollapseDroppedRecoveryState => {
                "responsive_collapse_dropped_recovery_state"
            }
            Self::ZoneStarvedMainWorkspace => "zone_starved_main_workspace",
            Self::ExtensionSetPrivateFracturingWidth => "extension_set_private_fracturing_width",
            Self::HitTargetShrankBelowMinimum => "hit_target_shrank_below_minimum",
            Self::PrimaryWorkflowHiddenBehindOverlayOnlyFallback => {
                "primary_workflow_hidden_behind_overlay_only_fallback"
            }
            Self::MetricCopiedByHandAcrossPackages => "metric_copied_by_hand_across_packages",
            Self::SizeMetricUnstated => "size_metric_unstated",
            Self::DensityModeUnstated => "density_mode_unstated",
            Self::ResponsiveClassUnstated => "responsive_class_unstated",
            Self::RegistryReferenceUnstated => "registry_reference_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed shell-geometry family bound to the surface-specific truth it must
/// project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometryRow {
    /// Governed shell-geometry family.
    pub geometry_family: M5ShellGeometryFamily,
    /// Qualification class earned by this family.
    pub qualification: M5ShellGeometryQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5ShellGeometrySurfaceFamily>,
    /// Deployment lines this family keeps the same truth across.
    pub deployment_lines: Vec<M5ShellGeometryDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5ShellGeometryRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ShellGeometryRequiredLabel>,
    /// Shell-geometry roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5ShellGeometryRole>,
    /// Shell-metric roles this family names (shell-metric family only).
    pub shell_metric_roles: Vec<M5ShellMetricRole>,
    /// Minimum-size roles this family names (minimum-size family only).
    pub minimum_size_roles: Vec<M5MinimumSizeRole>,
    /// Density-mode roles this family names (density-mode family only).
    pub density_mode_roles: Vec<M5DensityModeRole>,
    /// Responsive-geometry roles this family names (responsive-geometry family only).
    pub responsive_geometry_roles: Vec<M5ResponsiveGeometryRole>,
    /// Collapse-priority roles this family names (collapse-priority family only).
    pub collapse_priority_roles: Vec<M5CollapsePriorityRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5ShellGeometryDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5ShellGeometryAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5ShellGeometryConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never lets density or collapse change command meaning, focus order, or
    /// trust visibility. MUST be `false`.
    pub density_or_collapse_changes_command_focus_or_trust: bool,
    /// Hard invariant: this family never lets an extension or embedded surface set a private width that
    /// fractures the layout. MUST be `false`.
    pub extension_or_embedded_sets_private_fracturing_width: bool,
    /// Hard invariant: this family never shrinks a hit target below the supported minimum. MUST be
    /// `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Hard invariant: this family never hides a primary workflow behind an overlay-only fallback. MUST
    /// be `false`.
    pub hides_primary_workflow_behind_overlay_only_fallback: bool,
    /// Hard invariant: this family never lets a zone starve the main workspace below its minimum. MUST be
    /// `false`.
    pub lets_zone_starve_main_workspace_below_minimum: bool,
}

impl M5ShellGeometryRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ShellGeometryRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ShellGeometryRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.density_or_collapse_changes_command_focus_or_trust
            && !self.extension_or_embedded_sets_private_fracturing_width
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.hides_primary_workflow_behind_overlay_only_fallback
            && !self.lets_zone_starve_main_workspace_below_minimum
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometryVocabularySet {
    /// Geometry-family tokens.
    pub geometry_families: Vec<String>,
    /// Shell-geometry-role tokens.
    pub semantic_roles: Vec<String>,
    /// Shell-metric-role tokens.
    pub shell_metric_roles: Vec<String>,
    /// Minimum-size-role tokens.
    pub minimum_size_roles: Vec<String>,
    /// Density-mode-role tokens.
    pub density_mode_roles: Vec<String>,
    /// Responsive-geometry-role tokens.
    pub responsive_geometry_roles: Vec<String>,
    /// Collapse-priority-role tokens.
    pub collapse_priority_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5ShellGeometryVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            geometry_families: tokens(&M5ShellGeometryFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5ShellGeometryRole::ALL, |v| v.as_str()),
            shell_metric_roles: tokens(&M5ShellMetricRole::ALL, |v| v.as_str()),
            minimum_size_roles: tokens(&M5MinimumSizeRole::ALL, |v| v.as_str()),
            density_mode_roles: tokens(&M5DensityModeRole::ALL, |v| v.as_str()),
            responsive_geometry_roles: tokens(&M5ResponsiveGeometryRole::ALL, |v| v.as_str()),
            collapse_priority_roles: tokens(&M5CollapsePriorityRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ShellGeometrySurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ShellGeometryDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ShellGeometryConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ShellGeometryAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5ShellGeometryDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ShellGeometryRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5ShellGeometryDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5ShellGeometryGovernanceReview {
    /// The main workspace remains dominant across every surface.
    pub main_workspace_remains_dominant: bool,
    /// Every zone honors its declared minimum and recommended sizes.
    pub zones_honor_declared_minimum_and_recommended_sizes: bool,
    /// Density changes presentation, not information architecture.
    pub density_changes_presentation_not_information_architecture: bool,
    /// Responsive behavior preserves task identity.
    pub responsive_preserves_task_identity: bool,
    /// Responsive behavior preserves recovery-critical state.
    pub responsive_preserves_recovery_critical_state: bool,
    /// Hit targets meet the supported minimums.
    pub hit_targets_meet_supported_minimums: bool,
    /// Resize handles meet the hit-area minimum.
    pub resize_handles_meet_hit_area_minimum: bool,
    /// The tab minimum width is enforced.
    pub tab_minimum_width_enforced: bool,
    /// Extension or embedded surfaces cannot invent private widths.
    pub extension_or_embedded_cannot_invent_private_widths: bool,
    /// No primary workflow is hidden behind an overlay-only fallback.
    pub no_primary_workflow_hidden_behind_overlay_only_fallback: bool,
    /// Metrics are bound to one registry, not hand-copied.
    pub metrics_bound_to_single_registry_not_hand_copied: bool,
    /// Every family keeps the same truth across every deployment line.
    pub every_family_declares_deployment_lines: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical shell-geometry source.
    pub support_export_reads_single_shell_geometry_source: bool,
    /// Later M5 rows cannot invent parallel metric / density vocabulary.
    pub later_rows_cannot_invent_parallel_metric_or_density_vocabulary: bool,
    /// Geometry survives zoom and snapped widths.
    pub geometry_survives_zoom_and_snapped_widths: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometryConsumerProjection {
    /// Shell and editor consume the shared metric and density grammar.
    pub shell_and_editor_consume_shared_metric_and_density_grammar: bool,
    /// Review and notebook consume the shared responsive geometry.
    pub review_and_notebook_consume_shared_responsive_geometry: bool,
    /// Data and embedded surfaces consume the shared collapse model.
    pub data_and_embedded_surfaces_consume_shared_collapse_model: bool,
    /// Metric / density consumers read a single registry source.
    pub metric_density_consumers_read_single_registry_source: bool,
    /// Appearance and layout bind to the shared shell metrics.
    pub appearance_and_layout_bind_to_shared_shell_metrics: bool,
    /// Support / export reads a single canonical shell-geometry source.
    pub support_export_reads_single_shell_geometry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometryProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the shell-geometry lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellGeometryReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting shell-geometry audit for the lane.
    pub geometry_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ShellMetricDensityMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShellMetricDensityMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Geometry rows.
    pub geometry_rows: Vec<M5ShellGeometryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellGeometryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellGeometryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellGeometryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellGeometryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellGeometryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 shell-metric / density matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShellMetricDensityMatrixPacket {
    /// Record kind; must equal [`M5_SHELL_METRIC_DENSITY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Geometry rows.
    pub geometry_rows: Vec<M5ShellGeometryRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShellGeometryVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShellGeometryGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShellGeometryConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShellGeometryProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShellGeometryReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShellMetricDensityMatrixPacket {
    /// Builds an M5 shell-metric / density matrix packet from stable-lane input.
    pub fn new(input: M5ShellMetricDensityMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_SHELL_METRIC_DENSITY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            geometry_rows: input.geometry_rows,
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

    /// Validates the M5 shell-metric / density matrix invariants.
    pub fn validate(&self) -> Vec<M5ShellMetricDensityMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHELL_METRIC_DENSITY_MATRIX_RECORD_KIND {
            violations.push(M5ShellMetricDensityMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_VERSION {
            violations.push(M5ShellMetricDensityMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShellMetricDensityMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_geometry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 shell-metric / density matrix serializes"),
        ) {
            violations.push(M5ShellMetricDensityMatrixViolation::RawMaterialInExport);
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
            .expect("m5 shell-metric / density matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "geometry_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.geometry_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.geometry_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.geometry_family.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .geometry_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Shell-Metric, Minimum-Size, Density-Mode, Responsive-Geometry, and Collapse-Priority Shell-Geometry Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Geometry families: {} ({} stable)\n",
            self.geometry_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Shell-geometry roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Shell-metric roles: {}\n",
            self.vocabulary_set.shell_metric_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Geometry families\n\n");
        for row in &self.geometry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.geometry_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.geometry_family.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 shell-metric / density matrix export.
#[derive(Debug)]
pub enum M5ShellMetricDensityMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShellMetricDensityMatrixViolation>),
}

impl fmt::Display for M5ShellMetricDensityMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 shell-metric / density matrix export parse failed: {error}"
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
                    "m5 shell-metric / density matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShellMetricDensityMatrixArtifactError {}

/// Validation failures emitted by [`M5ShellMetricDensityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShellMetricDensityMatrixViolation {
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
    /// A required governed shell-geometry family is missing from the matrix.
    RequiredFamilyMissing,
    /// A geometry row is incomplete.
    GeometryRowIncomplete,
    /// A geometry row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A geometry row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no shell-geometry roles.
    SemanticRoleMissing,
    /// The shell-metric family declares no shell-metric roles.
    ShellMetricRoleMissing,
    /// The minimum-size family declares no minimum-size roles.
    MinimumSizeRoleMissing,
    /// The density-mode family declares no density-mode roles.
    DensityModeRoleMissing,
    /// The responsive-geometry family declares no responsive-geometry roles.
    ResponsiveGeometryRoleMissing,
    /// The collapse-priority family declares no collapse-priority roles.
    CollapsePriorityRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no deployment lines.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (density or collapse changing command / focus / trust, an
    /// extension setting a private fracturing width, a hit target shrinking below the minimum, a primary
    /// workflow hidden behind an overlay-only fallback, or a zone starving the main workspace).
    GeometryInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ShellMetricDensityMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::GeometryRowIncomplete => "geometry_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ShellMetricRoleMissing => "shell_metric_role_missing",
            Self::MinimumSizeRoleMissing => "minimum_size_role_missing",
            Self::DensityModeRoleMissing => "density_mode_role_missing",
            Self::ResponsiveGeometryRoleMissing => "responsive_geometry_role_missing",
            Self::CollapsePriorityRoleMissing => "collapse_priority_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::GeometryInvariantViolated => "geometry_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 shell-metric / density matrix export.
pub fn current_stable_m5_shell_metric_density_matrix_export(
) -> Result<M5ShellMetricDensityMatrixPacket, M5ShellMetricDensityMatrixArtifactError> {
    let packet: M5ShellMetricDensityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-metric-density-proof/support_export.json"
    )))
    .map_err(M5ShellMetricDensityMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShellMetricDensityMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_SHELL_METRICS_SCHEMA_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
        M5_SHELL_ZONE_SCHEMA_REF,
        M5_SHELL_PRIMITIVES_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ShellMetricDensityMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ShellMetricDensityMatrixViolation::VocabularySetDrift);
    }
}

fn validate_geometry_rows(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    let present: BTreeSet<M5ShellGeometryFamily> = packet
        .geometry_rows
        .iter()
        .map(|row| row.geometry_family)
        .collect();
    for required in M5ShellGeometryFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ShellMetricDensityMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.geometry_rows {
        let family = row.geometry_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ShellMetricDensityMatrixViolation::GeometryRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ShellMetricDensityMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5ShellMetricDensityMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_shell_metric_roles() && row.shell_metric_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::ShellMetricRoleMissing);
        }
        if family.declares_minimum_size_roles() && row.minimum_size_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::MinimumSizeRoleMissing);
        }
        if family.declares_density_mode_roles() && row.density_mode_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::DensityModeRoleMissing);
        }
        if family.declares_responsive_geometry_roles() && row.responsive_geometry_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::ResponsiveGeometryRoleMissing);
        }
        if family.declares_collapse_priority_roles() && row.collapse_priority_roles.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::CollapsePriorityRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ShellMetricDensityMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ShellMetricDensityMatrixViolation::GeometryInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.main_workspace_remains_dominant,
        review.zones_honor_declared_minimum_and_recommended_sizes,
        review.density_changes_presentation_not_information_architecture,
        review.responsive_preserves_task_identity,
        review.responsive_preserves_recovery_critical_state,
        review.hit_targets_meet_supported_minimums,
        review.resize_handles_meet_hit_area_minimum,
        review.tab_minimum_width_enforced,
        review.extension_or_embedded_cannot_invent_private_widths,
        review.no_primary_workflow_hidden_behind_overlay_only_fallback,
        review.metrics_bound_to_single_registry_not_hand_copied,
        review.every_family_declares_deployment_lines,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_shell_geometry_source,
        review.later_rows_cannot_invent_parallel_metric_or_density_vocabulary,
        review.geometry_survives_zoom_and_snapped_widths,
    ] {
        if !ok {
            violations.push(M5ShellMetricDensityMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_editor_consume_shared_metric_and_density_grammar,
        projection.review_and_notebook_consume_shared_responsive_geometry,
        projection.data_and_embedded_surfaces_consume_shared_collapse_model,
        projection.metric_density_consumers_read_single_registry_source,
        projection.appearance_and_layout_bind_to_shared_shell_metrics,
        projection.support_export_reads_single_shell_geometry_source,
    ] {
        if !ok {
            violations.push(M5ShellMetricDensityMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ShellMetricDensityMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ShellMetricDensityMatrixPacket,
    violations: &mut Vec<M5ShellMetricDensityMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.geometry_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ShellMetricDensityMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses shell / zone / density / responsive words; what is rejected is a raw
/// secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
