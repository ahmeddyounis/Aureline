//! Implemented M5 density-mode registries.
//!
//! The frozen [shell-metric / density matrix][matrix] names Aureline's five shell-geometry families and
//! locks their controlled vocabulary. This module is the density-mode implement lane over that matrix: it
//! turns the family that carries the concrete *presentation-density* grammar — the **density mode** family
//! (the comfortable / standard / compact row heights, control heights, tab / chip spacing, panel padding,
//! and gutter spacing, plus the profile-scope persistence of a chosen density) — into registry resolvers
//! that produce export-safe, honest projections. A user can then trust that changing density changes
//! presentation and never information architecture, command meaning, focus order, or trust visibility;
//! that every claimed M5 surface resolves the same tokenized density scale rather than a hand-picked local
//! scale; that hit targets never shrink below the supported minimum under any density mode or at high zoom;
//! and that a density preference persists at profile scope by default, allowing only explicitly explained
//! local overrides and never silently switching because a provider, theme, or workflow changed.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Encode the canonical density scale as logical-pixel tokens before OS scaling.**
//!   [`resolve_density_scale_entry`] refuses to read as a clean, registry-bound density-scale entry unless
//!   it names a canonical registry token, a classified [density mode][M5DensityMode], a density-mode role,
//!   declares the exact canonical row / control / spacing / padding / gutter tokens for that mode, covers
//!   every [surface element][M5DensitySurfaceElement] (lists, trees, tables, tabs, panels, editors, and
//!   inspectors), keeps hit targets at or above their supported minimum, changes presentation only (never
//!   information architecture), and preserves command meaning, focus order, and trust visibility.
//! * **Persist density at profile scope and forbid silent switching.**
//!   [`resolve_density_persistence_entry`] names a classified [persistence scope][M5DensityPersistenceScope],
//!   allows a local override only when it is explicitly explained (a presentation or accessibility viewer),
//!   and degrades to [`M5DensityPersistenceEntryDegradeReason::SilentDensitySwitch`] when a density change
//!   is triggered silently by a provider, theme, or workflow change.
//! * **Wire first shell, editor, review, notebook, and data consumers plus fixtures that catch scale drift.**
//!   Each registry row carries the render [surface context][M5DensitySurfaceContext] so a private-scale or
//!   below-minimum regression degrades honestly, and the acceptance-criteria gate proves that drift is
//!   caught before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5ShellGeometryRole`] role vocabulary
//! and the [`M5DensityModeRole`] density-mode-role vocabulary — so shell, editor, review, notebook, data,
//! and support surfaces can never fork their own density meaning. Raw secret values and private endpoints
//! stay outside the export boundary.
//!
//! [matrix]: crate::m5_shell_metric_density_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_density_mode_registries, seeded_m5_density_mode_registries_editor_ui_beta_narrowed,
    seeded_m5_density_mode_registries_settings_ui_preview_narrowed,
    M5_DENSITY_MODE_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_shell_metric_density_matrix::{
    M5DensityModeRole, M5ShellGeometryAccessibilityRoute, M5ShellGeometryConsumerSurface,
    M5ShellGeometryDeploymentLine, M5ShellGeometryDowngradeTrigger, M5ShellGeometryFamily,
    M5ShellGeometryQualificationClass, M5ShellGeometryRequiredLabel, M5ShellGeometryRole,
    M5_DENSITY_MODE_SCHEMA_REF, M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
    M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5DensityModeRegistriesPacket`].
pub const M5_DENSITY_MODE_REGISTRIES_RECORD_KIND: &str = "implement_m5_density_mode_registries";

/// Schema version for M5 density-mode registry records.
pub const M5_DENSITY_MODE_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the density-mode registries schema.
pub const M5_DENSITY_MODE_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-density-mode-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_DENSITY_MODE_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_density_mode_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DENSITY_MODE_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-density-mode-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_DENSITY_MODE_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-density-mode-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_DENSITY_MODE_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-density-mode-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_DENSITY_MODE_REGISTRIES_FIXTURE_DIR: &str = "fixtures/ui/m5-density-mode-registries";

/// Canonical minimum row height in logical pixels (the compact-mode row height); a declared row height
/// below this floor shrinks the row below its supported minimum.
pub const CANONICAL_ROW_MINIMUM_PX: u32 = 24;

/// Canonical minimum control height in logical pixels (the compact-mode control height and the icon-only
/// control hit-target minimum); a declared control height below this floor shrinks the hit target below its
/// supported minimum.
pub const CANONICAL_CONTROL_MINIMUM_PX: u32 = 28;

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5DensityModeRegistriesConsumerSurface = M5ShellGeometryConsumerSurface;

/// Canonical tokenized presentation scale for one density mode, before OS scaling. Density changes these
/// presentation dimensions and nothing else: it never changes command semantics, focus order, shell
/// zoning, state vocabulary, or hit-target minimums.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5DensityScale {
    /// Row height in logical pixels (lists, trees, tables).
    pub row_height_px: u32,
    /// Control height in logical pixels (buttons, inputs, tabs).
    pub control_height_px: u32,
    /// Tab / chip spacing in logical pixels.
    pub tab_chip_spacing_px: u32,
    /// Panel padding in logical pixels.
    pub panel_padding_px: u32,
    /// Gutter spacing in logical pixels.
    pub gutter_spacing_px: u32,
}

/// One of the three density modes every claimed M5 surface resolves from the shared registry so a user can
/// change presentation density without changing product meaning. Minted by this lane because the frozen
/// matrix names the density-mode *family* but not the concrete mode set and its canonical logical-pixel
/// scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityMode {
    /// The comfortable density mode (32 px rows, 36 px controls).
    Comfortable,
    /// The standard density mode (28 px rows, 32 px controls).
    Standard,
    /// The compact density mode (24 px rows, 28 px controls).
    Compact,
    /// The density mode is unclassified, which is disallowed.
    ModeUnclassified,
}

impl M5DensityMode {
    /// Every density mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Comfortable,
        Self::Standard,
        Self::Compact,
        Self::ModeUnclassified,
    ];

    /// The three canonical density modes every claimed M5 surface resolves from the registry.
    pub const CANONICAL_MODES: [Self; 3] = [Self::Comfortable, Self::Standard, Self::Compact];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::ModeUnclassified => "mode_unclassified",
        }
    }

    /// Whether the density mode is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ModeUnclassified)
    }

    /// Canonical tokenized presentation scale for this mode, before OS scaling. The unclassified sentinel
    /// has no scale.
    pub const fn canonical_scale(self) -> M5DensityScale {
        match self {
            Self::Comfortable => M5DensityScale {
                row_height_px: 32,
                control_height_px: 36,
                tab_chip_spacing_px: 8,
                panel_padding_px: 16,
                gutter_spacing_px: 16,
            },
            Self::Standard => M5DensityScale {
                row_height_px: 28,
                control_height_px: 32,
                tab_chip_spacing_px: 6,
                panel_padding_px: 12,
                gutter_spacing_px: 12,
            },
            Self::Compact => M5DensityScale {
                row_height_px: 24,
                control_height_px: 28,
                tab_chip_spacing_px: 4,
                panel_padding_px: 8,
                gutter_spacing_px: 8,
            },
            Self::ModeUnclassified => M5DensityScale {
                row_height_px: 0,
                control_height_px: 0,
                tab_chip_spacing_px: 0,
                panel_padding_px: 0,
                gutter_spacing_px: 0,
            },
        }
    }
}

/// Surface element a density-scale entry must apply to, so comfortable / standard / compact modes produce
/// predictable, tokenized changes across every list, tree, table, tab, panel, editor, and inspector rather
/// than a private per-widget scale. Minted by this lane, tracking the element types the acceptance criteria
/// name directly. Every clean density-scale entry covers all seven.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensitySurfaceElement {
    /// A list.
    List,
    /// A tree.
    Tree,
    /// A table.
    Table,
    /// A tab strip.
    Tab,
    /// A panel.
    Panel,
    /// An editor.
    Editor,
    /// An inspector.
    Inspector,
}

impl M5DensitySurfaceElement {
    /// Every surface element, in declaration order. A clean density-scale entry must cover all of them.
    pub const ALL: [Self; 7] = [
        Self::List,
        Self::Tree,
        Self::Table,
        Self::Tab,
        Self::Panel,
        Self::Editor,
        Self::Inspector,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Tree => "tree",
            Self::Table => "table",
            Self::Tab => "tab",
            Self::Panel => "panel",
            Self::Editor => "editor",
            Self::Inspector => "inspector",
        }
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a density-scale or
/// persistence token's meaning stays stable whether it appears in the shell, editor, review, notebook, or
/// data surface. Minted by this lane, tracking the first-consumer surfaces the implementation requirement
/// names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensitySurfaceContext {
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

impl M5DensitySurfaceContext {
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

/// Controlled persistence scope a density preference is stored at, so a chosen density persists at profile
/// scope by default and only an explicitly explained local override may narrow it. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityPersistenceScope {
    /// The density preference persists at profile scope (the canonical default).
    ProfileScoped,
    /// The density preference is a local override that must be explicitly explained.
    ExplainedLocalOverride,
    /// The persistence scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5DensityPersistenceScope {
    /// Every persistence scope, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ProfileScoped,
        Self::ExplainedLocalOverride,
        Self::ScopeUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileScoped => "profile_scoped",
            Self::ExplainedLocalOverride => "explained_local_override",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the persistence scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }

    /// Whether this is the canonical profile-scoped default.
    pub const fn is_profile_scoped(self) -> bool {
        matches!(self, Self::ProfileScoped)
    }

    /// Whether this scope requires an explicit explanation (a local override).
    pub const fn requires_explanation(self) -> bool {
        matches!(self, Self::ExplainedLocalOverride)
    }
}

/// Controlled reason a local density override is explained, so an override is only ever a presentation or
/// accessibility viewer rather than a silent, unexplained divergence from the profile-scoped default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityOverrideReason {
    /// The preference is not a local override.
    NotOverridden,
    /// The local override is a presentation viewer.
    PresentationViewer,
    /// The local override is an accessibility viewer.
    AccessibilityViewer,
    /// The local override is unexplained, which is disallowed.
    UnexplainedDisallowed,
}

impl M5DensityOverrideReason {
    /// Every override reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotOverridden,
        Self::PresentationViewer,
        Self::AccessibilityViewer,
        Self::UnexplainedDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotOverridden => "not_overridden",
            Self::PresentationViewer => "presentation_viewer",
            Self::AccessibilityViewer => "accessibility_viewer",
            Self::UnexplainedDisallowed => "unexplained_disallowed",
        }
    }

    /// Whether this override reason counts as an explicit explanation.
    pub const fn is_explained(self) -> bool {
        matches!(self, Self::PresentationViewer | Self::AccessibilityViewer)
    }
}

/// One mandatory rendered part a density-scale or persistence entry must be able to show, so no density,
/// scale, persistence, or registry fact is left implicit behind a private scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The density mode the entry maps (density-scale entry).
    DensityMode,
    /// The tokenized presentation scale (row / control / spacing / padding / gutter).
    PresentationScale,
    /// The surface-element coverage (lists / trees / tables / tabs / panels / editors / inspectors).
    SurfaceElementCoverage,
    /// The persistence scope the entry maps (persistence entry).
    PersistenceScope,
    /// The override reason the entry declares (persistence entry).
    OverrideReason,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the density change (both entries).
    PlainLanguageMeaning,
}

impl M5DensityRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::DensityMode,
        Self::PresentationScale,
        Self::SurfaceElementCoverage,
        Self::PersistenceScope,
        Self::OverrideReason,
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
            Self::DensityMode => "density_mode",
            Self::PresentationScale => "presentation_scale",
            Self::SurfaceElementCoverage => "surface_element_coverage",
            Self::PersistenceScope => "persistence_scope",
            Self::OverrideReason => "override_reason",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a density
/// mode, surface-element coverage, persistence scope, or a degraded density entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityRegistryNextAction {
    /// Expand the density change's plain-language meaning.
    ExpandDensityMeaning,
    /// Inspect the density mode, presentation scale, or persistence scope the entry maps.
    InspectModeOrScale,
    /// Complete the list / tree / table / tab / panel / editor / inspector surface-element coverage.
    CompleteSurfaceElementCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5DensityRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandDensityMeaning,
        Self::InspectModeOrScale,
        Self::CompleteSurfaceElementCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandDensityMeaning => "expand_density_meaning",
            Self::InspectModeOrScale => "inspect_mode_or_scale",
            Self::CompleteSurfaceElementCoverage => "complete_surface_element_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The geometry families covered.
    GeometryFamilies,
    /// The density modes carried.
    DensityModes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The tokenized presentation scales carried.
    PresentationScales,
    /// The surface elements carried.
    SurfaceElements,
    /// The persistence scopes carried.
    PersistenceScopes,
    /// The render / surface context.
    SurfaceContext,
    /// The accountable owner role.
    OwnerRole,
}

impl M5DensityRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::DensityModes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationScales,
        Self::SurfaceElements,
        Self::PersistenceScopes,
        Self::SurfaceContext,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::GeometryFamilies,
        Self::DensityModes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::GeometryFamilies => "geometry_families",
            Self::DensityModes => "density_modes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationScales => "presentation_scales",
            Self::SurfaceElements => "surface_elements",
            Self::PersistenceScopes => "persistence_scopes",
            Self::SurfaceContext => "surface_context",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a density-scale entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting an information-architecture-changing, focus-changing,
/// below-minimum, private-scale, or element-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityScaleEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the density scale means.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The density mode is unclassified (not in the preserved taxonomy).
    ModeUnclassified,
    /// The density change rearranges information architecture instead of only presentation.
    ChangesInformationArchitecture,
    /// The density change alters command meaning, focus order, or trust visibility.
    ChangesCommandFocusOrTrust,
    /// A row or control height shrinks the hit target below its supported minimum.
    HitTargetShrinksBelowMinimum,
    /// The declared scale drifts from the canonical density tokens (a private scale).
    ScaleOutsideCanonicalTokens,
    /// The list / tree / table / tab / panel / editor / inspector surface-element coverage is incomplete.
    SurfaceElementCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DensityScaleEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ModeUnclassified,
        Self::ChangesInformationArchitecture,
        Self::ChangesCommandFocusOrTrust,
        Self::HitTargetShrinksBelowMinimum,
        Self::ScaleOutsideCanonicalTokens,
        Self::SurfaceElementCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ModeUnclassified => "mode_unclassified",
            Self::ChangesInformationArchitecture => "changes_information_architecture",
            Self::ChangesCommandFocusOrTrust => "changes_command_focus_or_trust",
            Self::HitTargetShrinksBelowMinimum => "hit_target_shrinks_below_minimum",
            Self::ScaleOutsideCanonicalTokens => "scale_outside_canonical_tokens",
            Self::SurfaceElementCoverageIncomplete => "surface_element_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DensityRegistryNextAction {
        match self {
            Self::TokenUnstated => M5DensityRegistryNextAction::TraceCanonicalRegistry,
            Self::ModeUnclassified
            | Self::ChangesInformationArchitecture
            | Self::ChangesCommandFocusOrTrust
            | Self::HitTargetShrinksBelowMinimum
            | Self::ScaleOutsideCanonicalTokens => M5DensityRegistryNextAction::InspectModeOrScale,
            Self::SurfaceElementCoverageIncomplete => {
                M5DensityRegistryNextAction::CompleteSurfaceElementCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5DensityRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated => M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ModeUnclassified => M5ShellGeometryDowngradeTrigger::DensityModeUnstated,
            Self::ChangesInformationArchitecture | Self::ChangesCommandFocusOrTrust => {
                M5ShellGeometryDowngradeTrigger::DensityChangedCommandOrFocusOrTrust
            }
            Self::HitTargetShrinksBelowMinimum => {
                M5ShellGeometryDowngradeTrigger::HitTargetShrankBelowMinimum
            }
            Self::ScaleOutsideCanonicalTokens => {
                M5ShellGeometryDowngradeTrigger::MetricCopiedByHandAcrossPackages
            }
            Self::SurfaceElementCoverageIncomplete => {
                M5ShellGeometryDowngradeTrigger::DensityModeUnstated
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a density-persistence entry degraded below a clean, profile-scoped state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityPersistenceEntryDegradeReason {
    /// The canonical registry token name is unstated.
    TokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The persistence scope is unclassified (not in the preserved taxonomy).
    PersistenceScopeUnclassified,
    /// The density switched silently because a provider, theme, or workflow changed.
    SilentDensitySwitch,
    /// The local override is not explicitly explained (not a presentation or accessibility viewer).
    UnexplainedLocalOverride,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DensityPersistenceEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::PersistenceScopeUnclassified,
        Self::SilentDensitySwitch,
        Self::UnexplainedLocalOverride,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenUnstated => "token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PersistenceScopeUnclassified => "persistence_scope_unclassified",
            Self::SilentDensitySwitch => "silent_density_switch",
            Self::UnexplainedLocalOverride => "unexplained_local_override",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5DensityRegistryNextAction {
        match self {
            Self::TokenUnstated => M5DensityRegistryNextAction::TraceCanonicalRegistry,
            Self::PersistenceScopeUnclassified
            | Self::SilentDensitySwitch
            | Self::UnexplainedLocalOverride => M5DensityRegistryNextAction::InspectModeOrScale,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5DensityRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            Self::TokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::PersistenceScopeUnclassified => {
                M5ShellGeometryDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SilentDensitySwitch | Self::UnexplainedLocalOverride => {
                M5ShellGeometryDowngradeTrigger::DensityChangedCommandOrFocusOrTrust
            }
            Self::ProofStale => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_density_scale_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DensityScaleEntryResolutionInput {
    /// Stable identity of the density-scale-registry entry.
    pub entry_id: String,
    /// The canonical registry token name (e.g. `shell.density.standard.scale`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The density-mode role (from the frozen matrix vocabulary).
    pub density_mode_role: M5DensityModeRole,
    /// The density mode this entry maps.
    pub density_mode: M5DensityMode,
    /// The render / surface context.
    pub surface_context: M5DensitySurfaceContext,
    /// The declared row height in logical pixels.
    pub row_height_px: u32,
    /// The declared control height in logical pixels.
    pub control_height_px: u32,
    /// The declared tab / chip spacing in logical pixels.
    pub tab_chip_spacing_px: u32,
    /// The declared panel padding in logical pixels.
    pub panel_padding_px: u32,
    /// The declared gutter spacing in logical pixels.
    pub gutter_spacing_px: u32,
    /// The surface elements this entry applies to (must cover every element).
    pub surface_elements: Vec<M5DensitySurfaceElement>,
    /// True when the density change rearranges information architecture (a hard invariant when `true`).
    pub changes_information_architecture: bool,
    /// True when the density change preserves command meaning, focus order, and trust visibility.
    pub preserves_command_focus_and_trust: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe density-scale-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDensityScaleEntry {
    /// Stable identity of the density-scale-registry entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve task identity when density changes or the layout collapses.
    pub semantic_role_preserves_task_identity_under_collapse: bool,
    /// The density-mode-role token named by the entry.
    pub density_mode_role: String,
    /// Whether the density-mode role names the disallowed changes-information-architecture token.
    pub density_mode_role_changes_information_architecture: bool,
    /// The density-mode token named by the entry.
    pub density_mode: String,
    /// Whether the density mode is classified into the preserved taxonomy.
    pub density_mode_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The declared row height in logical pixels.
    pub row_height_px: u32,
    /// The declared control height in logical pixels.
    pub control_height_px: u32,
    /// The declared tab / chip spacing in logical pixels.
    pub tab_chip_spacing_px: u32,
    /// The declared panel padding in logical pixels.
    pub panel_padding_px: u32,
    /// The declared gutter spacing in logical pixels.
    pub gutter_spacing_px: u32,
    /// The canonical row height for this density mode.
    pub canonical_row_height_px: u32,
    /// The canonical control height for this density mode.
    pub canonical_control_height_px: u32,
    /// Whether the declared scale matches the canonical density tokens for this mode.
    pub matches_canonical_scale: bool,
    /// The surface-element tokens covered by the entry.
    pub surface_elements: Vec<String>,
    /// Whether the entry covers every surface element.
    pub covers_all_surface_elements: bool,
    /// Whether the density change rearranges information architecture.
    pub changes_information_architecture: bool,
    /// Whether the density change preserves command meaning, focus order, and trust visibility.
    pub preserves_command_focus_and_trust: bool,
    /// Whether the row and control heights stay at or above their supported minimum.
    pub meets_hit_target_minimum: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5DensityScaleEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DensityRegistryNextAction,
    /// Whether the density change is presentation-only across every surface element (clean entry naming
    /// every fact).
    pub density_change_is_presentation_only: bool,
}

impl M5ResolvedDensityScaleEntry {
    /// Whether this density-scale entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_density_persistence_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DensityPersistenceEntryResolutionInput {
    /// Stable identity of the persistence entry.
    pub entry_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The density-mode role (from the frozen matrix vocabulary).
    pub density_mode_role: M5DensityModeRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5ShellGeometryRole,
    /// The persistence scope this entry maps.
    pub persistence_scope: M5DensityPersistenceScope,
    /// The override reason (meaningful only for a local override).
    pub override_reason: M5DensityOverrideReason,
    /// The render / surface context.
    pub surface_context: M5DensitySurfaceContext,
    /// True when the density switched silently because a provider, theme, or workflow changed (a hard
    /// invariant when `true`).
    pub switched_silently_by_provider_theme_or_workflow: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe density-persistence projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDensityPersistenceEntry {
    /// Stable identity of the persistence entry.
    pub entry_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The density-mode-role token named by the entry.
    pub density_mode_role: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The persistence-scope token named by the entry.
    pub persistence_scope: String,
    /// Whether the persistence scope is classified into the preserved taxonomy.
    pub scope_is_classified: bool,
    /// Whether the persistence scope is the canonical profile-scoped default.
    pub scope_is_profile_scoped: bool,
    /// The override-reason token named by the entry.
    pub override_reason: String,
    /// Whether the override reason counts as an explicit explanation.
    pub override_is_explained: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the density switched silently because a provider, theme, or workflow changed.
    pub switched_silently_by_provider_theme_or_workflow: bool,
    /// Degrade reason, if the entry could not read as a clean, profile-scoped state.
    pub degrade_reason: Option<M5DensityPersistenceEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5DensityRegistryNextAction,
    /// Whether the density preference holds at profile scope (clean entry naming every fact).
    pub persistence_holds_at_profile_scope: bool,
}

impl M5ResolvedDensityPersistenceEntry {
    /// Whether this persistence entry reads as a clean, profile-scoped state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5DensityResolutionError {
    /// The density-scale-entry id was empty.
    EmptyDensityScaleEntryId,
    /// The persistence-entry id was empty.
    EmptyDensityPersistenceEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5DensityResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyDensityScaleEntryId => "empty_density_scale_entry_id",
            Self::EmptyDensityPersistenceEntryId => "empty_density_persistence_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5DensityResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 density-mode registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DensityResolutionError {}

fn element_tokens(elements: &[M5DensitySurfaceElement]) -> Vec<String> {
    elements.iter().map(|e| e.as_str().to_owned()).collect()
}

fn covers_all_surface_elements(elements: &[M5DensitySurfaceElement]) -> bool {
    let present: BTreeSet<M5DensitySurfaceElement> = elements.iter().copied().collect();
    M5DensitySurfaceElement::ALL
        .iter()
        .all(|element| present.contains(element))
}

/// Whether the declared scale exactly matches the canonical density tokens for this mode. Density modes
/// are tokenized, not free-form: a mode either declares its canonical row / control / spacing / padding /
/// gutter or it drifts into a private scale that must degrade.
fn matches_canonical_scale(
    mode: M5DensityMode,
    row_height_px: u32,
    control_height_px: u32,
    tab_chip_spacing_px: u32,
    panel_padding_px: u32,
    gutter_spacing_px: u32,
) -> bool {
    if !mode.is_classified() {
        return false;
    }
    let scale = mode.canonical_scale();
    scale.row_height_px == row_height_px
        && scale.control_height_px == control_height_px
        && scale.tab_chip_spacing_px == tab_chip_spacing_px
        && scale.panel_padding_px == panel_padding_px
        && scale.gutter_spacing_px == gutter_spacing_px
}

/// Resolves a density-scale-registry entry so it stays bound to the shared registry: the entry names its
/// canonical token, semantic role, density-mode role, and density mode, declares the exact canonical scale
/// tokens for that mode, covers every surface element, keeps hit targets at or above their supported
/// minimum, changes presentation only, and preserves command meaning, focus order, and trust visibility.
pub fn resolve_density_scale_entry(
    input: M5DensityScaleEntryResolutionInput,
) -> Result<M5ResolvedDensityScaleEntry, M5DensityResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5DensityResolutionError::EmptyDensityScaleEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5DensityResolutionError::ForbiddenMaterial);
    }

    let role_changes_information_architecture = matches!(
        input.density_mode_role,
        M5DensityModeRole::DensityChangesInformationArchitectureDisallowed
    );
    let scale = input.density_mode.canonical_scale();
    let matches_scale = matches_canonical_scale(
        input.density_mode,
        input.row_height_px,
        input.control_height_px,
        input.tab_chip_spacing_px,
        input.panel_padding_px,
        input.gutter_spacing_px,
    );
    let all_elements = covers_all_surface_elements(&input.surface_elements);
    let meets_hit_target_minimum = input.row_height_px >= CANONICAL_ROW_MINIMUM_PX
        && input.control_height_px >= CANONICAL_CONTROL_MINIMUM_PX;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5DensityScaleEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5DensityScaleEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.density_mode.is_classified() {
        Some(M5DensityScaleEntryDegradeReason::ModeUnclassified)
    } else if role_changes_information_architecture || input.changes_information_architecture {
        Some(M5DensityScaleEntryDegradeReason::ChangesInformationArchitecture)
    } else if !input.preserves_command_focus_and_trust {
        Some(M5DensityScaleEntryDegradeReason::ChangesCommandFocusOrTrust)
    } else if !meets_hit_target_minimum {
        Some(M5DensityScaleEntryDegradeReason::HitTargetShrinksBelowMinimum)
    } else if !matches_scale {
        Some(M5DensityScaleEntryDegradeReason::ScaleOutsideCanonicalTokens)
    } else if !all_elements {
        Some(M5DensityScaleEntryDegradeReason::SurfaceElementCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5DensityScaleEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DensityRegistryNextAction::ExpandDensityMeaning,
    };

    Ok(M5ResolvedDensityScaleEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_task_identity_under_collapse: input
            .semantic_role
            .must_preserve_task_identity_under_collapse(),
        density_mode_role: input.density_mode_role.as_str().to_owned(),
        density_mode_role_changes_information_architecture: role_changes_information_architecture,
        density_mode: input.density_mode.as_str().to_owned(),
        density_mode_is_classified: input.density_mode.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        row_height_px: input.row_height_px,
        control_height_px: input.control_height_px,
        tab_chip_spacing_px: input.tab_chip_spacing_px,
        panel_padding_px: input.panel_padding_px,
        gutter_spacing_px: input.gutter_spacing_px,
        canonical_row_height_px: scale.row_height_px,
        canonical_control_height_px: scale.control_height_px,
        matches_canonical_scale: matches_scale,
        surface_elements: element_tokens(&input.surface_elements),
        covers_all_surface_elements: all_elements,
        changes_information_architecture: input.changes_information_architecture,
        preserves_command_focus_and_trust: input.preserves_command_focus_and_trust,
        meets_hit_target_minimum,
        degrade_reason,
        next_action,
        density_change_is_presentation_only: degrade_reason.is_none(),
    })
}

/// Resolves a density-persistence entry so a chosen density persists safely at profile scope: the entry
/// names its canonical token, density-mode role, and semantic role, keeps a classified persistence scope,
/// explains any local override, and never switches silently because a provider, theme, or workflow changed.
pub fn resolve_density_persistence_entry(
    input: M5DensityPersistenceEntryResolutionInput,
) -> Result<M5ResolvedDensityPersistenceEntry, M5DensityResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5DensityResolutionError::EmptyDensityPersistenceEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5DensityResolutionError::ForbiddenMaterial);
    }

    let override_is_explained = input.override_reason.is_explained();
    let local_override_unexplained =
        input.persistence_scope.requires_explanation() && !override_is_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5DensityPersistenceEntryDegradeReason::TokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5DensityPersistenceEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.persistence_scope.is_classified() {
        Some(M5DensityPersistenceEntryDegradeReason::PersistenceScopeUnclassified)
    } else if input.switched_silently_by_provider_theme_or_workflow {
        Some(M5DensityPersistenceEntryDegradeReason::SilentDensitySwitch)
    } else if local_override_unexplained {
        Some(M5DensityPersistenceEntryDegradeReason::UnexplainedLocalOverride)
    } else if !input.proof_fresh {
        Some(M5DensityPersistenceEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5DensityRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedDensityPersistenceEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        density_mode_role: input.density_mode_role.as_str().to_owned(),
        semantic_role: input.semantic_role.as_str().to_owned(),
        persistence_scope: input.persistence_scope.as_str().to_owned(),
        scope_is_classified: input.persistence_scope.is_classified(),
        scope_is_profile_scoped: input.persistence_scope.is_profile_scoped(),
        override_reason: input.override_reason.as_str().to_owned(),
        override_is_explained,
        surface_context: input.surface_context.as_str().to_owned(),
        switched_silently_by_provider_theme_or_workflow: input
            .switched_silently_by_provider_theme_or_workflow,
        degrade_reason,
        next_action,
        persistence_holds_at_profile_scope: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved density-scale and persistence entries it
/// must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5DensityModeRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5DensityRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5DensityRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5ShellGeometryDowngradeTrigger>,
    /// Resolved density-scale-registry examples.
    pub density_scale_entries: Vec<M5ResolvedDensityScaleEntry>,
    /// Resolved density-persistence examples.
    pub density_persistence_entries: Vec<M5ResolvedDensityPersistenceEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical density-mode domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a density change never rearranges information architecture. MUST be `false`.
    pub density_change_alters_information_architecture: bool,
    /// Hard invariant: a density change never alters command meaning, focus order, or trust visibility.
    /// MUST be `false`.
    pub density_change_alters_command_focus_or_trust: bool,
    /// Hard invariant: a hit target never shrinks below the supported minimum. MUST be `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Hard invariant: density is never switched silently outside its profile scope. MUST be `false`.
    pub silently_switches_density_outside_profile_scope: bool,
}

impl M5DensityModeRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5DensityRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5DensityRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DensityRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5DensityRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.density_change_alters_information_architecture
            && !self.density_change_alters_command_focus_or_trust
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.silently_switches_density_outside_profile_scope
    }

    /// True when a clean density-scale entry preserves registry-bound density: it keeps a classified mode,
    /// never names the disallowed changes-information-architecture role, matches the canonical scale,
    /// covers every surface element, keeps hit targets above their minimum, changes presentation only, and
    /// preserves command / focus / trust.
    fn scale_is_honest(ex: &M5ResolvedDensityScaleEntry) -> bool {
        !ex.is_clean()
            || (ex.density_mode_is_classified
                && !ex.density_mode_role_changes_information_architecture
                && ex.matches_canonical_scale
                && ex.covers_all_surface_elements
                && ex.meets_hit_target_minimum
                && !ex.changes_information_architecture
                && ex.preserves_command_focus_and_trust)
    }

    /// True when a clean persistence entry preserves profile-scope safety: it keeps a classified scope,
    /// never switches silently, and either is profile-scoped or is an explained local override.
    fn persistence_is_honest(ex: &M5ResolvedDensityPersistenceEntry) -> bool {
        !ex.is_clean()
            || (ex.scope_is_classified
                && !ex.switched_silently_by_provider_theme_or_workflow
                && (ex.scope_is_profile_scoped || ex.override_is_explained))
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.density_scale_entries.iter().all(Self::scale_is_honest)
            && self
                .density_persistence_entries
                .iter()
                .all(Self::persistence_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Density-mode-role tokens (bound from the frozen matrix).
    pub density_mode_roles: Vec<String>,
    /// Density-mode tokens (minted by this lane).
    pub density_modes: Vec<String>,
    /// Surface-element tokens (minted by this lane).
    pub surface_elements: Vec<String>,
    /// Persistence-scope tokens (minted by this lane).
    pub persistence_scopes: Vec<String>,
    /// Override-reason tokens (minted by this lane).
    pub override_reasons: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Density-scale-entry degrade-reason tokens.
    pub density_scale_degrade_reasons: Vec<String>,
    /// Persistence-entry degrade-reason tokens.
    pub density_persistence_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5DensityModeRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5ShellGeometryRole::ALL, |v| v.as_str()),
            density_mode_roles: tokens(&M5DensityModeRole::ALL, |v| v.as_str()),
            density_modes: tokens(&M5DensityMode::ALL, |v| v.as_str()),
            surface_elements: tokens(&M5DensitySurfaceElement::ALL, |v| v.as_str()),
            persistence_scopes: tokens(&M5DensityPersistenceScope::ALL, |v| v.as_str()),
            override_reasons: tokens(&M5DensityOverrideReason::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5DensitySurfaceContext::ALL, |v| v.as_str()),
            density_scale_degrade_reasons: tokens(&M5DensityScaleEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            density_persistence_degrade_reasons: tokens(
                &M5DensityPersistenceEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5DensityRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5DensityRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DensityRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5DensityModeRegistriesGovernanceReview {
    /// The density registry names a canonical token, density-mode role, and mode for every entry.
    pub density_registry_names_token_role_and_mode: bool,
    /// The canonical density scale is encoded as logical-pixel tokens before OS scaling.
    pub density_scale_encoded_as_logical_pixel_tokens: bool,
    /// Every claimed surface resolves its density scale from the shared registry.
    pub every_surface_resolves_from_shared_registry: bool,
    /// Density changes presentation only and never information architecture, command, focus, or trust.
    pub density_changes_presentation_only: bool,
    /// Hit targets never shrink below the supported minimum under any density mode or at high zoom.
    pub hit_targets_never_shrink_below_supported_minimum: bool,
    /// Every claimed density mode covers the list / tree / table / tab / panel / editor / inspector
    /// surface elements.
    pub every_mode_covers_all_surface_elements: bool,
    /// Density persists at profile scope by default with only explicitly explained local overrides.
    pub density_persists_at_profile_scope_by_default: bool,
    /// Density is never switched silently because a provider, theme, or workflow changed.
    pub density_never_switches_silently: bool,
    /// The first shell / editor / review / notebook / data consumers use the canonical density grammar.
    pub first_consumers_use_canonical_density_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesConsumerProjection {
    /// The shell surface consumes the shared density registries.
    pub shell_consumes_shared_registries: bool,
    /// The editor surface consumes the shared density registries.
    pub editor_consumes_shared_registries: bool,
    /// The review surface consumes the shared density registries.
    pub review_consumes_shared_registries: bool,
    /// The notebook and data surfaces consume the shared density registries.
    pub notebook_and_data_consume_shared_registries: bool,
    /// Density resolves back to one canonical density-mode domain contract.
    pub density_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical density registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting shell-geometry audit for the lane.
    pub geometry_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DensityModeRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DensityModeRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5DensityModeRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DensityModeRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DensityModeRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DensityModeRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DensityModeRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DensityModeRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 density-mode registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DensityModeRegistriesPacket {
    /// Record kind; must equal [`M5_DENSITY_MODE_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DENSITY_MODE_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5DensityModeRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DensityModeRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DensityModeRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DensityModeRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DensityModeRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DensityModeRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DensityModeRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5DensityModeRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_DENSITY_MODE_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_DENSITY_MODE_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5DensityModeRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DENSITY_MODE_REGISTRIES_RECORD_KIND {
            violations.push(M5DensityModeRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DENSITY_MODE_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5DensityModeRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DensityModeRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5DensityModeRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 density-mode registries packet serializes"),
        ) {
            violations.push(M5DensityModeRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 density-mode registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,density_scale_entries,density_persistence_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .density_scale_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.density_persistence_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.density_scale_entries.len(),
                row.density_persistence_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Density-Mode Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Density modes: {}\n",
            self.vocabulary_set.density_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Surface elements: {}\n",
            self.vocabulary_set.surface_elements.join(", ")
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
                "  - Density-scale entries: {} / persistence entries: {}\n",
                row.density_scale_entries.len(),
                row.density_persistence_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5DensityModeRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DensityModeRegistriesViolation>),
}

impl fmt::Display for M5DensityModeRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 density-mode registries export parse failed: {error}"
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
                    "m5 density-mode registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DensityModeRegistriesArtifactError {}

/// Validation failures emitted by [`M5DensityModeRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DensityModeRegistriesViolation {
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
    /// A registry row carries a dishonest clean example (information-architecture-changing, private-scale,
    /// below-minimum, element-incomplete, or a persistence entry that switches silently or is an
    /// unexplained override).
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
    /// Tokenized density changes across surfaces are not proven: clean density-scale entries do not cover
    /// the three canonical density modes or the first shell / editor / review / notebook / data surfaces,
    /// no private-scale example degrades, or a clean entry drifts from the canonical scale.
    TokenizedDensityChangesAcrossSurfacesNotProven,
    /// Density operability under zoom is not proven: clean density-scale entries do not meet the hit-target
    /// minimum across the canonical modes, no below-minimum example degrades, or a clean entry shrinks
    /// below its minimum.
    DensityOperableUnderZoomWithoutShrinkingHitTargetsNotProven,
    /// Honest extension degradation is not proven: no private-scale and no silent-switch example degrade,
    /// no clean profile-scoped persistence entry exists, or a clean entry carries a private scale.
    ExtensionPrivateScaleDegradesHonestlyNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5DensityModeRegistriesViolation {
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
            Self::TokenizedDensityChangesAcrossSurfacesNotProven => {
                "tokenized_density_changes_across_surfaces_not_proven"
            }
            Self::DensityOperableUnderZoomWithoutShrinkingHitTargetsNotProven => {
                "density_operable_under_zoom_without_shrinking_hit_targets_not_proven"
            }
            Self::ExtensionPrivateScaleDegradesHonestlyNotProven => {
                "extension_private_scale_degrades_honestly_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_density_mode_registries_export(
) -> Result<M5DensityModeRegistriesPacket, M5DensityModeRegistriesArtifactError> {
    let packet: M5DensityModeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-density-mode-registries-proof/support_export.json"
    )))
    .map_err(M5DensityModeRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DensityModeRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DENSITY_MODE_REGISTRIES_SCHEMA_REF,
        M5_DENSITY_MODE_REGISTRIES_DOC_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
        M5_SHELL_METRIC_DENSITY_MATRIX_DOC_REF,
        M5_DENSITY_MODE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DensityModeRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5DensityModeRegistriesViolation::NoRegistryRows);
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
            violations.push(M5DensityModeRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5DensityModeRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DensityModeRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_DENSITY_MODE_SCHEMA_REF) {
            violations.push(M5DensityModeRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.density_scale_entries.is_empty() || row.density_persistence_entries.is_empty() {
            violations.push(M5DensityModeRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5DensityModeRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5DensityModeRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.density_registry_names_token_role_and_mode,
        review.density_scale_encoded_as_logical_pixel_tokens,
        review.every_surface_resolves_from_shared_registry,
        review.density_changes_presentation_only,
        review.hit_targets_never_shrink_below_supported_minimum,
        review.every_mode_covers_all_surface_elements,
        review.density_persists_at_profile_scope_by_default,
        review.density_never_switches_silently,
        review.first_consumers_use_canonical_density_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5DensityModeRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.editor_consumes_shared_registries,
        projection.review_consumes_shared_registries,
        projection.notebook_and_data_consume_shared_registries,
        projection.density_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5DensityModeRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DensityModeRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.geometry_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DensityModeRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5DensityModeRegistriesPacket,
    violations: &mut Vec<M5DensityModeRegistriesViolation>,
) {
    let scales = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.density_scale_entries.iter())
    };
    let persistences = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.density_persistence_entries.iter())
    };

    // AC1: comfortable / standard / compact modes produce predictable, tokenized changes across every
    // surface element. Clean density-scale entries cover the three canonical modes and the first shell /
    // editor / review / notebook / data surfaces, a private-scale example degrades, and no clean entry
    // drifts from the canonical scale.
    let clean_modes: BTreeSet<String> = scales()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.density_mode.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = scales()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let modes_covered = M5DensityMode::CANONICAL_MODES
        .iter()
        .all(|m| clean_modes.contains(m.as_str()));
    let first_surfaces_covered = M5DensitySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let private_scale_degrades = scales().any(|ex| {
        ex.degrade_reason == Some(M5DensityScaleEntryDegradeReason::ScaleOutsideCanonicalTokens)
    });
    let no_clean_drift = !scales().any(|ex| ex.is_clean() && !ex.matches_canonical_scale);
    if !(modes_covered && first_surfaces_covered && private_scale_degrades && no_clean_drift) {
        violations
            .push(M5DensityModeRegistriesViolation::TokenizedDensityChangesAcrossSurfacesNotProven);
    }

    // AC2: at 400% zoom or equivalent assistive use, density preferences remain operable without shrinking
    // hit targets below their supported minimum. Clean density-scale entries meet the hit-target minimum
    // across the canonical modes, a below-minimum example degrades, and no clean entry shrinks below its
    // minimum.
    let clean_min_modes: BTreeSet<String> = scales()
        .filter(|ex| ex.is_clean() && ex.meets_hit_target_minimum)
        .map(|ex| ex.density_mode.clone())
        .collect();
    let min_modes_covered = M5DensityMode::CANONICAL_MODES
        .iter()
        .all(|m| clean_min_modes.contains(m.as_str()));
    let below_minimum_degrades = scales().any(|ex| {
        ex.degrade_reason == Some(M5DensityScaleEntryDegradeReason::HitTargetShrinksBelowMinimum)
    });
    let no_clean_below_minimum = !scales().any(|ex| ex.is_clean() && !ex.meets_hit_target_minimum);
    if !(min_modes_covered && below_minimum_degrades && no_clean_below_minimum) {
        violations.push(
            M5DensityModeRegistriesViolation::DensityOperableUnderZoomWithoutShrinkingHitTargetsNotProven,
        );
    }

    // AC3: extension surfaces that cannot honor the canonical density tokens degrade honestly instead of
    // inventing private scales. A private-scale example and a silent-switch persistence example both
    // degrade, at least one clean profile-scoped persistence entry exists, and no clean entry carries a
    // private scale.
    let silent_switch_degrades = persistences().any(|ex| {
        ex.degrade_reason == Some(M5DensityPersistenceEntryDegradeReason::SilentDensitySwitch)
    });
    let clean_profile_scoped = persistences().any(|ex| ex.is_clean() && ex.scope_is_profile_scoped);
    if !(private_scale_degrades && silent_switch_degrades && clean_profile_scoped && no_clean_drift)
    {
        violations
            .push(M5DensityModeRegistriesViolation::ExtensionPrivateScaleDegradesHonestlyNotProven);
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

/// The single shell-geometry family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5ShellGeometryFamily; 1] = [M5ShellGeometryFamily::DensityMode];
