//! Implemented M5 spacing / sizing / radii / border / elevation geometry registries plus minimum
//! hit-target rules.
//!
//! The frozen [visual-foundation matrix][matrix] names Aureline's eight visual-foundation families and
//! locks their controlled vocabulary. The [color / theme registries lane][color] turned the two color
//! families into resolvers, the [syntax / diff / chart registries lane][code] turned the three
//! code-and-data families into resolvers, and the [typography registries lane][type] turned the
//! typography family into resolvers. This module is the closing implement lane over that matrix: it turns
//! the **spacing / sizing / radii / border / elevation** family and the **minimum hit-target** family —
//! the last two of the eight — into registry resolvers that produce export-safe, honest projections, so
//! controls and panes share one canonical geometry instead of acquiring private layout rules, so
//! compact / standard / comfortable density changes presentation without shrinking hit targets below the
//! supported minima, so overlays and dialogs keep their elevation hierarchy, and so a local geometry fork
//! is visible in fixtures and proof packets before stable promotion.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement the shared spacing / sizing / radii / border / elevation primitives plus minimum
//!   hit-target rules for interactive controls and resize handles.** [`resolve_geometry_entry`] refuses to
//!   read as a clean geometry entry unless it names a canonical token, resolves its
//!   [primitive kind][M5GeometryPrimitiveKind], names a [geometry role][crate::m5_visual_foundation_matrix::M5GeometryRole]
//!   that matches the kind (never the disallowed local fork), and stays density-aware.
//!   [`resolve_hit_target_entry`] refuses to read as clean unless it names a canonical token, resolves its
//!   [control kind][M5HitTargetControlKind], and meets the supported minimum with adequate spacing between
//!   adjacent targets.
//! * **Enforce density-aware application so compact / standard / comfortable modes change presentation
//!   without violating accessibility or command semantics.** A geometry entry that is not density-aware
//!   degrades to [`M5GeometryDegradeReason::NotDensityAware`], and a hit-target entry that shrinks below
//!   the supported minimum for its density degrades to [`M5HitTargetDegradeReason::ShrinksBelowMinimum`].
//! * **Wire first shell, list / table, editor, dialog, and review consumers plus drift checks that flag
//!   local geometry forks.** The [surface context][M5GeometrySurfaceContext] tracks the first consumers by
//!   name, and a forked or raw-value entry can never read as a clean pass.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualSemanticRole`] role
//! vocabulary, the [`M5GeometryRole`] geometry-role vocabulary, and the [`M5HitTargetRule`] hit-target-rule
//! vocabulary — so the shell, list / table, editor, dialog, review, and support surfaces can never fork
//! their own spacing, sizing, radii, elevation, or hit-target meaning. Raw secret values and private
//! endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_visual_foundation_matrix
//! [color]: crate::m5_color_system_and_semantic_theme_token_registries
//! [code]: crate::m5_syntax_diff_and_chart_token_registries
//! [type]: crate::m5_typography_scale_font_stack_and_overflow_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_geometry_hit_target_registries,
    seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed,
    seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed,
    M5_GEOMETRY_HIT_TARGET_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_visual_foundation_matrix::{
    M5GeometryRole, M5HitTargetRule, M5VisualFoundationAccessibilityRoute,
    M5VisualFoundationConsumerSurface, M5VisualFoundationDeploymentLine,
    M5VisualFoundationDowngradeTrigger, M5VisualFoundationFamily,
    M5VisualFoundationQualificationClass, M5VisualFoundationRequiredLabel, M5VisualSemanticRole,
    M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF, M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
    M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5GeometryHitTargetRegistriesPacket`].
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_spacing_sizing_radii_elevation_and_hit_target_registries";

/// Schema version for M5 geometry / hit-target registry records.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-spacing-sizing-radii-elevation-and-hit-target-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_spacing_sizing_radii_elevation_and_hit_target_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_GEOMETRY_HIT_TARGET_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-spacing-sizing-radii-elevation-and-hit-target-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5GeometryConsumerSurface = M5VisualFoundationConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the registry entry, so spacing, sizing,
/// radii, elevation, and hit-target rules stay stable whether they appear in the shell, a list / table, the
/// editor, a dialog, or the review surface. Minted by this lane, tracking the first-consumer surfaces the
/// goal names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometrySurfaceContext {
    /// The shell surface (chrome, status, tabs).
    Shell,
    /// A list / table surface (dense rows and cells).
    ListTable,
    /// The editor surface (code, gutter, resize handles).
    Editor,
    /// A dialog / overlay surface (modals, sheets, popovers).
    Dialog,
    /// The review surface (diff and annotation controls).
    Review,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5GeometrySurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::ListTable,
        Self::Editor,
        Self::Dialog,
        Self::Review,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the goal names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Shell,
        Self::ListTable,
        Self::Editor,
        Self::Dialog,
        Self::Review,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::ListTable => "list_table",
            Self::Editor => "editor",
            Self::Dialog => "dialog",
            Self::Review => "review",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled geometry primitive a geometry entry names, so spacing, sizing, radii, borders, and elevation
/// each map to a stable step of the canonical geometry scale. Minted by this lane, tracking the primitives
/// the implementation requirement names by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryPrimitiveKind {
    /// A spacing (margin / padding / gap) step.
    Spacing,
    /// A sizing (width / height / min / max) step.
    Sizing,
    /// A corner-radius step.
    Radius,
    /// A border (width / style) step.
    Border,
    /// An elevation / shadow level.
    Elevation,
    /// The geometry primitive kind is unstated.
    KindUnknown,
}

impl M5GeometryPrimitiveKind {
    /// Every geometry primitive kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Spacing,
        Self::Sizing,
        Self::Radius,
        Self::Border,
        Self::Elevation,
        Self::KindUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Spacing => "spacing",
            Self::Sizing => "sizing",
            Self::Radius => "radius",
            Self::Border => "border",
            Self::Elevation => "elevation",
            Self::KindUnknown => "kind_unknown",
        }
    }

    /// Whether the primitive kind is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::KindUnknown)
    }

    /// Whether this kind is the elevation primitive, which must preserve the overlay / dialog hierarchy.
    pub const fn is_elevation(self) -> bool {
        matches!(self, Self::Elevation)
    }

    /// Whether the supplied geometry role matches this primitive kind. The disallowed local-fork role never
    /// matches; the density-aware role is universally acceptable; otherwise each dimensioned kind must name
    /// its matching canonical geometry step.
    pub const fn matches_geometry_role(self, role: M5GeometryRole) -> bool {
        use M5GeometryRole as R;
        if matches!(role, R::LocalGeometryForkDisallowed) {
            return false;
        }
        match self {
            Self::Spacing => matches!(role, R::SpacingStep | R::DensityAware),
            Self::Sizing | Self::Border => matches!(role, R::SizingStep | R::DensityAware),
            Self::Radius => matches!(role, R::RadiusStep | R::DensityAware),
            Self::Elevation => matches!(role, R::ElevationLevel | R::DensityAware),
            Self::KindUnknown => false,
        }
    }
}

/// Controlled density mode a geometry or hit-target entry is applied under, so compact, standard, and
/// comfortable modes change presentation without violating accessibility or command semantics. Minted by
/// this lane, tracking the density modes the implementation requirement names by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryDensityMode {
    /// Compact density.
    Compact,
    /// Standard density.
    Standard,
    /// Comfortable density.
    Comfortable,
    /// The density mode is unstated.
    DensityUnknown,
}

impl M5GeometryDensityMode {
    /// Every density mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Compact,
        Self::Standard,
        Self::Comfortable,
        Self::DensityUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compact => "compact",
            Self::Standard => "standard",
            Self::Comfortable => "comfortable",
            Self::DensityUnknown => "density_unknown",
        }
    }

    /// Whether the density mode is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::DensityUnknown)
    }
}

/// Controlled elevation tier an elevation primitive names, so base content, raised panels, overlays, and
/// dialogs keep the intended stacking hierarchy. Minted by this lane, tracking the elevation hierarchy the
/// acceptance criteria call for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ElevationTier {
    /// Base / flush content.
    Base,
    /// A raised panel or card.
    Raised,
    /// A transient overlay (popover / menu).
    Overlay,
    /// A modal dialog / sheet.
    Dialog,
    /// The elevation tier is unstated.
    TierUnknown,
}

impl M5ElevationTier {
    /// Every elevation tier, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Base,
        Self::Raised,
        Self::Overlay,
        Self::Dialog,
        Self::TierUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Raised => "raised",
            Self::Overlay => "overlay",
            Self::Dialog => "dialog",
            Self::TierUnknown => "tier_unknown",
        }
    }

    /// Whether the elevation tier is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::TierUnknown)
    }

    /// Whether this tier floats above base content (overlay or dialog).
    pub const fn is_overlay_or_dialog(self) -> bool {
        matches!(self, Self::Overlay | Self::Dialog)
    }
}

/// Controlled interactive-control kind a hit-target entry governs, so buttons, icon buttons, resize
/// handles, toggles, and menu items each declare their minimum-target rule. Minted by this lane, tracking
/// the controls and resize handles the implementation requirement names by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HitTargetControlKind {
    /// A standard button.
    Button,
    /// An icon-only button.
    IconButton,
    /// A drag / resize handle.
    ResizeHandle,
    /// A toggle / switch.
    Toggle,
    /// A menu / list item.
    MenuItem,
    /// The control kind cannot currently be resolved.
    ControlUnknown,
}

impl M5HitTargetControlKind {
    /// Every control kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Button,
        Self::IconButton,
        Self::ResizeHandle,
        Self::Toggle,
        Self::MenuItem,
        Self::ControlUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Button => "button",
            Self::IconButton => "icon_button",
            Self::ResizeHandle => "resize_handle",
            Self::Toggle => "toggle",
            Self::MenuItem => "menu_item",
            Self::ControlUnknown => "control_unknown",
        }
    }

    /// Whether the control kind is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ControlUnknown)
    }
}

/// One mandatory rendered part a geometry or hit-target entry must be able to show, so no role, primitive,
/// density, elevation, control, or minimum-target fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The geometry role (geometry entry).
    GeometryRole,
    /// The primitive kind (geometry entry).
    PrimitiveKind,
    /// The elevation tier (geometry entry).
    ElevationTier,
    /// The hit-target rule (hit-target entry).
    HitTargetRule,
    /// The control kind (hit-target entry).
    ControlKind,
    /// The minimum-target guarantee (hit-target entry).
    MinimumTarget,
    /// The density mode.
    DensityMode,
    /// The render / surface context.
    SurfaceContext,
}

impl M5GeometryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::GeometryRole,
        Self::PrimitiveKind,
        Self::ElevationTier,
        Self::HitTargetRule,
        Self::ControlKind,
        Self::MinimumTarget,
        Self::DensityMode,
        Self::SurfaceContext,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::GeometryRole => "geometry_role",
            Self::PrimitiveKind => "primitive_kind",
            Self::ElevationTier => "elevation_tier",
            Self::HitTargetRule => "hit_target_rule",
            Self::ControlKind => "control_kind",
            Self::MinimumTarget => "minimum_target",
            Self::DensityMode => "density_mode",
            Self::SurfaceContext => "surface_context",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect the
/// geometry scale, adjust hit-target sizing, trace a token, verify density / elevation, or review a
/// degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryNextAction {
    /// Inspect the canonical spacing / sizing / radii / elevation scale.
    InspectGeometryScale,
    /// Adjust the hit-target sizing so it meets the supported minimum.
    AdjustHitTargetSizing,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Verify the entry survives density changes and preserves the elevation hierarchy.
    VerifyDensityAndElevation,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5GeometryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectGeometryScale,
        Self::AdjustHitTargetSizing,
        Self::TraceCanonicalToken,
        Self::VerifyDensityAndElevation,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectGeometryScale => "inspect_geometry_scale",
            Self::AdjustHitTargetSizing => "adjust_hit_target_sizing",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::VerifyDensityAndElevation => "verify_density_and_elevation",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The foundation families covered.
    FoundationFamilies,
    /// The semantic roles named.
    SemanticRoles,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The geometry primitive kinds named.
    PrimitiveKinds,
    /// The elevation tiers named.
    ElevationTiers,
    /// The density modes applied.
    DensityModes,
    /// The control kinds governed.
    ControlKinds,
    /// The hit-target rules named.
    HitTargetRules,
    /// The accountable owner role.
    OwnerRole,
}

impl M5GeometryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::SemanticRoles,
        Self::DegradeReasons,
        Self::Qualification,
        Self::PrimitiveKinds,
        Self::ElevationTiers,
        Self::DensityModes,
        Self::ControlKinds,
        Self::HitTargetRules,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::SemanticRoles,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::FoundationFamilies => "foundation_families",
            Self::SemanticRoles => "semantic_roles",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::PrimitiveKinds => "primitive_kinds",
            Self::ElevationTiers => "elevation_tiers",
            Self::DensityModes => "density_modes",
            Self::ControlKinds => "control_kinds",
            Self::HitTargetRules => "hit_target_rules",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a geometry entry degraded below a clean state. The degrade-first ladder returns one of these
/// instead of ever letting a forked, non-density-aware, elevation-broken, or raw-value entry read as clean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The geometry primitive kind is unstated.
    PrimitiveKindUnstated,
    /// The geometry role forks from the shared foundation or does not match the primitive kind.
    GeometryRoleForked,
    /// The density mode cannot currently be resolved.
    DensityModeUnresolved,
    /// The primitive is not density-aware; it applies one geometry regardless of density mode.
    NotDensityAware,
    /// An elevation primitive does not preserve the overlay / dialog hierarchy.
    ElevationHierarchyBroken,
    /// A raw geometry value is inlined instead of tracing to a canonical token.
    RawGeometryValueInlined,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5GeometryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::PrimitiveKindUnstated,
        Self::GeometryRoleForked,
        Self::DensityModeUnresolved,
        Self::NotDensityAware,
        Self::ElevationHierarchyBroken,
        Self::RawGeometryValueInlined,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PrimitiveKindUnstated => "primitive_kind_unstated",
            Self::GeometryRoleForked => "geometry_role_forked",
            Self::DensityModeUnresolved => "density_mode_unresolved",
            Self::NotDensityAware => "not_density_aware",
            Self::ElevationHierarchyBroken => "elevation_hierarchy_broken",
            Self::RawGeometryValueInlined => "raw_geometry_value_inlined",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5GeometryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawGeometryValueInlined => {
                M5GeometryNextAction::TraceCanonicalToken
            }
            Self::PrimitiveKindUnstated | Self::GeometryRoleForked | Self::NotDensityAware => {
                M5GeometryNextAction::InspectGeometryScale
            }
            Self::DensityModeUnresolved | Self::ElevationHierarchyBroken => {
                M5GeometryNextAction::VerifyDensityAndElevation
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5GeometryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::TokenNameUnstated | Self::RawGeometryValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::GeometryRoleForked | Self::NotDensityAware | Self::ElevationHierarchyBroken => {
                M5VisualFoundationDowngradeTrigger::LocalGeometryForkedFromFoundation
            }
            Self::PrimitiveKindUnstated
            | Self::SurfaceContextUnresolved
            | Self::DensityModeUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a hit-target entry degraded below a clean state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HitTargetDegradeReason {
    /// The canonical token / identity is unstated.
    IdentityUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The control kind cannot currently be resolved.
    ControlKindUnresolved,
    /// The density mode cannot currently be resolved.
    DensityModeUnresolved,
    /// The target shrinks below its supported minimum for this density.
    ShrinksBelowMinimum,
    /// The spacing between adjacent targets is inadequate.
    InadequateTargetSpacing,
    /// A raw geometry value is inlined instead of tracing to a canonical token.
    RawGeometryValueInlined,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5HitTargetDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::IdentityUnstated,
        Self::SurfaceContextUnresolved,
        Self::ControlKindUnresolved,
        Self::DensityModeUnresolved,
        Self::ShrinksBelowMinimum,
        Self::InadequateTargetSpacing,
        Self::RawGeometryValueInlined,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityUnstated => "identity_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ControlKindUnresolved => "control_kind_unresolved",
            Self::DensityModeUnresolved => "density_mode_unresolved",
            Self::ShrinksBelowMinimum => "shrinks_below_minimum",
            Self::InadequateTargetSpacing => "inadequate_target_spacing",
            Self::RawGeometryValueInlined => "raw_geometry_value_inlined",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5GeometryNextAction {
        match self {
            Self::IdentityUnstated | Self::RawGeometryValueInlined => {
                M5GeometryNextAction::TraceCanonicalToken
            }
            Self::ShrinksBelowMinimum | Self::InadequateTargetSpacing => {
                M5GeometryNextAction::AdjustHitTargetSizing
            }
            Self::DensityModeUnresolved => M5GeometryNextAction::VerifyDensityAndElevation,
            Self::ControlKindUnresolved | Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5GeometryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::IdentityUnstated | Self::RawGeometryValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::ShrinksBelowMinimum | Self::InadequateTargetSpacing => {
                M5VisualFoundationDowngradeTrigger::HitTargetShrunkBelowMinimum
            }
            Self::ControlKindUnresolved
            | Self::SurfaceContextUnresolved
            | Self::DensityModeUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_geometry_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GeometryEntryResolutionInput {
    /// Stable identity of the geometry entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `space.2`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The geometry role (from the frozen matrix vocabulary).
    pub geometry_role: M5GeometryRole,
    /// The geometry primitive kind the entry names.
    pub primitive_kind: M5GeometryPrimitiveKind,
    /// The density mode the entry is applied under.
    pub density_mode: M5GeometryDensityMode,
    /// The elevation tier (meaningful for elevation primitives).
    pub elevation_tier: M5ElevationTier,
    /// The render / surface context.
    pub surface_context: M5GeometrySurfaceContext,
    /// True when the primitive adapts across density modes without violating command semantics.
    pub density_aware: bool,
    /// True when an elevation primitive keeps overlays / dialogs above base content.
    pub elevation_hierarchy_preserved: bool,
    /// True when the entry traces to a canonical token (never an inlined raw value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe geometry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedGeometryEntry {
    /// Stable identity of the geometry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The geometry-role token named by the entry.
    pub geometry_role: String,
    /// The primitive-kind token named by the entry.
    pub primitive_kind: String,
    /// Whether the geometry role matches the primitive kind (never the disallowed local fork).
    pub geometry_role_matches_kind: bool,
    /// Whether the primitive is the elevation primitive.
    pub is_elevation: bool,
    /// The elevation-tier token named by the entry.
    pub elevation_tier: String,
    /// Whether the elevation hierarchy is preserved (overlays / dialogs above base content).
    pub elevation_hierarchy_preserved: bool,
    /// The density-mode token named by the entry.
    pub density_mode: String,
    /// Whether the primitive is density-aware.
    pub density_aware: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5GeometryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5GeometryNextAction,
    /// Whether the geometry reads as canonical for a clean entry naming every fact.
    pub geometry_is_canonical: bool,
}

impl M5ResolvedGeometryEntry {
    /// Whether this geometry entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_hit_target_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HitTargetEntryResolutionInput {
    /// Stable identity of the hit-target entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The hit-target rule (from the frozen matrix vocabulary).
    pub hit_target_rule: M5HitTargetRule,
    /// The interactive-control kind the entry governs.
    pub control_kind: M5HitTargetControlKind,
    /// The density mode the target is measured under.
    pub density_mode: M5GeometryDensityMode,
    /// The render / surface context.
    pub surface_context: M5GeometrySurfaceContext,
    /// True when the target meets the supported minimum for this density.
    pub meets_supported_minimum: bool,
    /// True when the spacing between adjacent targets is adequate.
    pub adequate_target_spacing: bool,
    /// True when the entry traces to a canonical token (never an inlined raw value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe hit-target projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedHitTargetEntry {
    /// Stable identity of the hit-target entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The hit-target-rule token named by the entry.
    pub hit_target_rule: String,
    /// The control-kind token named by the entry.
    pub control_kind: String,
    /// Whether the control kind is resolved.
    pub control_kind_resolved: bool,
    /// The density-mode token named by the entry.
    pub density_mode: String,
    /// Whether the target meets the supported minimum for this density.
    pub meets_supported_minimum: bool,
    /// Whether the spacing between adjacent targets is adequate.
    pub adequate_target_spacing: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5HitTargetDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5GeometryNextAction,
    /// Whether the target stays above the supported minimum for a clean entry naming every fact.
    pub target_meets_minimum: bool,
}

impl M5ResolvedHitTargetEntry {
    /// Whether this hit-target entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5GeometryHitTargetResolutionError {
    /// The geometry-entry id was empty.
    EmptyGeometryEntryId,
    /// The hit-target-entry id was empty.
    EmptyHitTargetEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5GeometryHitTargetResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyGeometryEntryId => "empty_geometry_entry_id",
            Self::EmptyHitTargetEntryId => "empty_hit_target_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5GeometryHitTargetResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 geometry / hit-target registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5GeometryHitTargetResolutionError {}

/// Resolves a geometry entry so controls and panes share one canonical geometry: the entry names its
/// canonical token, resolves its primitive kind, names a geometry role that matches the kind (never the
/// disallowed local fork), stays density-aware, and — for elevation primitives — preserves the overlay /
/// dialog hierarchy.
pub fn resolve_geometry_entry(
    input: M5GeometryEntryResolutionInput,
) -> Result<M5ResolvedGeometryEntry, M5GeometryHitTargetResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5GeometryHitTargetResolutionError::EmptyGeometryEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5GeometryHitTargetResolutionError::ForbiddenMaterial);
    }

    let role_matches_kind = input
        .primitive_kind
        .matches_geometry_role(input.geometry_role);
    let is_elevation = input.primitive_kind.is_elevation();
    let elevation_ok = !is_elevation
        || (input.elevation_tier.is_resolved() && input.elevation_hierarchy_preserved);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5GeometryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5GeometryDegradeReason::SurfaceContextUnresolved)
    } else if !input.primitive_kind.is_resolved() {
        Some(M5GeometryDegradeReason::PrimitiveKindUnstated)
    } else if !role_matches_kind {
        Some(M5GeometryDegradeReason::GeometryRoleForked)
    } else if !input.density_mode.is_resolved() {
        Some(M5GeometryDegradeReason::DensityModeUnresolved)
    } else if !input.density_aware {
        Some(M5GeometryDegradeReason::NotDensityAware)
    } else if !elevation_ok {
        Some(M5GeometryDegradeReason::ElevationHierarchyBroken)
    } else if !input.references_canonical_token {
        Some(M5GeometryDegradeReason::RawGeometryValueInlined)
    } else if !input.proof_fresh {
        Some(M5GeometryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5GeometryNextAction::InspectGeometryScale,
    };

    Ok(M5ResolvedGeometryEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        geometry_role: input.geometry_role.as_str().to_owned(),
        primitive_kind: input.primitive_kind.as_str().to_owned(),
        geometry_role_matches_kind: role_matches_kind,
        is_elevation,
        elevation_tier: input.elevation_tier.as_str().to_owned(),
        elevation_hierarchy_preserved: input.elevation_hierarchy_preserved,
        density_mode: input.density_mode.as_str().to_owned(),
        density_aware: input.density_aware,
        surface_context: input.surface_context.as_str().to_owned(),
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        geometry_is_canonical: degrade_reason.is_none(),
    })
}

/// Resolves a hit-target entry so interactive controls and resize handles never shrink below the supported
/// minimum: the entry names its canonical token and control kind, meets the supported minimum for its
/// density, and keeps adequate spacing between adjacent targets.
pub fn resolve_hit_target_entry(
    input: M5HitTargetEntryResolutionInput,
) -> Result<M5ResolvedHitTargetEntry, M5GeometryHitTargetResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5GeometryHitTargetResolutionError::EmptyHitTargetEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5GeometryHitTargetResolutionError::ForbiddenMaterial);
    }

    let rule_shrinks = matches!(
        input.hit_target_rule,
        M5HitTargetRule::ShrinkBelowMinimumDisallowed
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5HitTargetDegradeReason::IdentityUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5HitTargetDegradeReason::SurfaceContextUnresolved)
    } else if !input.control_kind.is_resolved() {
        Some(M5HitTargetDegradeReason::ControlKindUnresolved)
    } else if !input.density_mode.is_resolved() {
        Some(M5HitTargetDegradeReason::DensityModeUnresolved)
    } else if rule_shrinks || !input.meets_supported_minimum {
        Some(M5HitTargetDegradeReason::ShrinksBelowMinimum)
    } else if !input.adequate_target_spacing {
        Some(M5HitTargetDegradeReason::InadequateTargetSpacing)
    } else if !input.references_canonical_token {
        Some(M5HitTargetDegradeReason::RawGeometryValueInlined)
    } else if !input.proof_fresh {
        Some(M5HitTargetDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5GeometryNextAction::AdjustHitTargetSizing,
    };

    Ok(M5ResolvedHitTargetEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        hit_target_rule: input.hit_target_rule.as_str().to_owned(),
        control_kind: input.control_kind.as_str().to_owned(),
        control_kind_resolved: input.control_kind.is_resolved(),
        density_mode: input.density_mode.as_str().to_owned(),
        meets_supported_minimum: input.meets_supported_minimum && !rule_shrinks,
        adequate_target_spacing: input.adequate_target_spacing,
        surface_context: input.surface_context.as_str().to_owned(),
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        target_meets_minimum: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved geometry and hit-target entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5GeometryConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5VisualFoundationQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5VisualFoundationDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5VisualFoundationRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5VisualFoundationAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5GeometryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5GeometryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    /// Resolved geometry examples.
    pub geometry_entries: Vec<M5ResolvedGeometryEntry>,
    /// Resolved hit-target examples.
    pub hit_target_entries: Vec<M5ResolvedHitTargetEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical typography / geometry domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: geometry never forks from the shared foundation. MUST be `false`.
    pub local_geometry_forked_from_foundation: bool,
    /// Hard invariant: a hit target never shrinks below its supported minimum. MUST be `false`.
    pub hit_target_shrunk_below_minimum: bool,
    /// Hard invariant: overlays / dialogs never lose their elevation hierarchy. MUST be `false`.
    pub elevation_hierarchy_broken: bool,
    /// Hard invariant: a raw geometry value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_geometry_value_inlined_instead_of_token: bool,
}

impl M5GeometryHitTargetRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5GeometryAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5GeometryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5GeometryExportField> = self.export_fields.iter().copied().collect();
        M5GeometryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.local_geometry_forked_from_foundation
            && !self.hit_target_shrunk_below_minimum
            && !self.elevation_hierarchy_broken
            && !self.raw_geometry_value_inlined_instead_of_token
    }

    fn has_any_entry(&self) -> bool {
        !self.geometry_entries.is_empty() || !self.hit_target_entries.is_empty()
    }

    /// True when a clean geometry entry stays canonical: it traces to a canonical token, its role matches
    /// its primitive kind, it is density-aware, and — for elevation primitives — preserves the hierarchy.
    fn geometry_is_honest(ex: &M5ResolvedGeometryEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.geometry_role_matches_kind
                && ex.density_aware
                && (!ex.is_elevation || ex.elevation_hierarchy_preserved))
    }

    /// True when a clean hit-target entry stays above the supported minimum: it traces to a canonical
    /// token, meets the supported minimum, and keeps adequate spacing between targets.
    fn hit_target_is_honest(ex: &M5ResolvedHitTargetEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.meets_supported_minimum
                && ex.adequate_target_spacing)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.geometry_entries.iter().all(Self::geometry_is_honest)
            && self
                .hit_target_entries
                .iter()
                .all(Self::hit_target_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Geometry-role tokens (bound from the frozen matrix).
    pub geometry_roles: Vec<String>,
    /// Hit-target-rule tokens (bound from the frozen matrix).
    pub hit_target_rules: Vec<String>,
    /// Geometry-primitive-kind tokens (minted by this lane).
    pub primitive_kinds: Vec<String>,
    /// Density-mode tokens (minted by this lane).
    pub density_modes: Vec<String>,
    /// Elevation-tier tokens (minted by this lane).
    pub elevation_tiers: Vec<String>,
    /// Control-kind tokens (minted by this lane).
    pub control_kinds: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Geometry-entry degrade-reason tokens.
    pub geometry_degrade_reasons: Vec<String>,
    /// Hit-target-entry degrade-reason tokens.
    pub hit_target_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5GeometryHitTargetVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualSemanticRole::ALL, |v| v.as_str()),
            geometry_roles: tokens(&M5GeometryRole::ALL, |v| v.as_str()),
            hit_target_rules: tokens(&M5HitTargetRule::ALL, |v| v.as_str()),
            primitive_kinds: tokens(&M5GeometryPrimitiveKind::ALL, |v| v.as_str()),
            density_modes: tokens(&M5GeometryDensityMode::ALL, |v| v.as_str()),
            elevation_tiers: tokens(&M5ElevationTier::ALL, |v| v.as_str()),
            control_kinds: tokens(&M5HitTargetControlKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5GeometrySurfaceContext::ALL, |v| v.as_str()),
            geometry_degrade_reasons: tokens(&M5GeometryDegradeReason::ALL, |v| v.as_str()),
            hit_target_degrade_reasons: tokens(&M5HitTargetDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5GeometryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5GeometryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5GeometryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5VisualFoundationConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5GeometryHitTargetGovernanceReview {
    /// The shell, list / table, editor, dialog, and review surfaces share one canonical geometry.
    pub one_canonical_geometry_across_surfaces: bool,
    /// The spacing / sizing / radii / border / elevation primitives are shared, not forked.
    pub spacing_sizing_radii_border_elevation_primitives_shared: bool,
    /// Density-aware application holds across compact / standard / comfortable modes.
    pub density_aware_application_holds: bool,
    /// Compact density preserves hit-target minima.
    pub compact_density_preserves_hit_target_minima: bool,
    /// Overlays and dialogs preserve the intended elevation hierarchy.
    pub overlays_and_dialogs_preserve_elevation_hierarchy: bool,
    /// Resize handles meet minimum targets.
    pub resize_handles_meet_minimum_targets: bool,
    /// Geometry drift is caught by fixtures before release evidence turns green.
    pub geometry_drift_caught_before_release: bool,
    /// Raw-geometry-value drift is caught before release evidence turns green.
    pub raw_geometry_value_drift_caught_before_release: bool,
    /// The first shell / list-table / editor / dialog / review consumers use the canonical geometry.
    pub first_consumers_use_canonical_geometry: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetConsumerProjection {
    /// The shell and list / table surfaces consume the shared geometry.
    pub shell_and_list_table_consume_shared_geometry: bool,
    /// The editor surface consumes the shared geometry.
    pub editor_consumes_shared_geometry: bool,
    /// The dialog surface consumes the shared elevation hierarchy.
    pub dialog_consumes_elevation_hierarchy: bool,
    /// The review surface consumes the shared geometry.
    pub review_consumes_shared_geometry: bool,
    /// Geometry meaning traces back to one canonical typography / geometry domain contract.
    pub geometry_meaning_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical geometry source.
    pub support_export_reads_single_geometry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-foundation audit for the lane.
    pub foundation_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5GeometryHitTargetRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5GeometryHitTargetRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5GeometryHitTargetRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GeometryHitTargetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GeometryHitTargetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GeometryHitTargetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GeometryHitTargetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GeometryHitTargetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 spacing / sizing / radii / border / elevation geometry and hit-target registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeometryHitTargetRegistriesPacket {
    /// Record kind; must equal [`M5_GEOMETRY_HIT_TARGET_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5GeometryHitTargetRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5GeometryHitTargetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5GeometryHitTargetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5GeometryHitTargetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5GeometryHitTargetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5GeometryHitTargetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5GeometryHitTargetRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5GeometryHitTargetRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_GEOMETRY_HIT_TARGET_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5GeometryHitTargetRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_GEOMETRY_HIT_TARGET_REGISTRIES_RECORD_KIND {
            violations.push(M5GeometryHitTargetRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5GeometryHitTargetRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5GeometryHitTargetRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5GeometryHitTargetRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 geometry / hit-target registries packet serializes"),
        ) {
            violations.push(M5GeometryHitTargetRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 geometry / hit-target registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,geometry_entries,hit_target_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .geometry_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.hit_target_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.geometry_entries.len(),
                row.hit_target_entries.len(),
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
            "# M5 Spacing / Sizing / Radii / Border / Elevation and Hit-Target Registries\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Primitive kinds: {}\n",
            self.vocabulary_set.primitive_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Density modes: {}\n",
            self.vocabulary_set.density_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Control kinds: {}\n",
            self.vocabulary_set.control_kinds.join(", ")
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
                "  - Geometry: {} / hit-target: {}\n",
                row.geometry_entries.len(),
                row.hit_target_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5GeometryHitTargetRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5GeometryHitTargetRegistriesViolation>),
}

impl fmt::Display for M5GeometryHitTargetRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 geometry / hit-target registries export parse failed: {error}"
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
                    "m5 geometry / hit-target registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5GeometryHitTargetRegistriesArtifactError {}

/// Validation failures emitted by [`M5GeometryHitTargetRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5GeometryHitTargetRegistriesViolation {
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
    /// A registry row does not point at the canonical typography / geometry domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (geometry fork, sub-minimum hit target, broken
    /// elevation, or a raw-value inlining that still reads as clean).
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
    /// Canonical geometry primitives are not proven: clean geometry entries do not cover the spacing /
    /// sizing / radius / elevation primitive kinds across the first shell / list-table / editor / dialog /
    /// review surfaces, no forked example degrades, or a clean entry inlines a raw value.
    CanonicalGeometryPrimitivesNotProven,
    /// Compact minima or the elevation hierarchy are not proven: no clean compact hit-target entry meets
    /// the supported minimum, no sub-minimum example degrades, no clean elevation entry preserves the
    /// hierarchy, no elevation-broken example degrades, or a clean hit-target entry shrinks below minimum.
    CompactMinimaOrElevationHierarchyNotProven,
    /// Geometry drift is not caught: no clean geometry entry is density-aware, no not-density-aware example
    /// degrades, no raw-geometry example degrades, or a clean geometry entry forks the foundation.
    GeometryDriftNotCaught,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5GeometryHitTargetRegistriesViolation {
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
            Self::CanonicalGeometryPrimitivesNotProven => {
                "canonical_geometry_primitives_not_proven"
            }
            Self::CompactMinimaOrElevationHierarchyNotProven => {
                "compact_minima_or_elevation_hierarchy_not_proven"
            }
            Self::GeometryDriftNotCaught => "geometry_drift_not_caught",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_geometry_hit_target_registries_export(
) -> Result<M5GeometryHitTargetRegistriesPacket, M5GeometryHitTargetRegistriesArtifactError> {
    let packet: M5GeometryHitTargetRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/support_export.json"
    )))
    .map_err(M5GeometryHitTargetRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5GeometryHitTargetRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_SCHEMA_REF,
        M5_GEOMETRY_HIT_TARGET_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5GeometryHitTargetRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5GeometryHitTargetRegistriesViolation::NoRegistryRows);
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
            violations.push(M5GeometryHitTargetRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5GeometryHitTargetRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5GeometryHitTargetRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF) {
            violations.push(M5GeometryHitTargetRegistriesViolation::DomainSchemaRefMissing);
        }
        if !row.has_any_entry() {
            violations.push(M5GeometryHitTargetRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5GeometryHitTargetRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5GeometryHitTargetRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_canonical_geometry_across_surfaces,
        review.spacing_sizing_radii_border_elevation_primitives_shared,
        review.density_aware_application_holds,
        review.compact_density_preserves_hit_target_minima,
        review.overlays_and_dialogs_preserve_elevation_hierarchy,
        review.resize_handles_meet_minimum_targets,
        review.geometry_drift_caught_before_release,
        review.raw_geometry_value_drift_caught_before_release,
        review.first_consumers_use_canonical_geometry,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5GeometryHitTargetRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_list_table_consume_shared_geometry,
        projection.editor_consumes_shared_geometry,
        projection.dialog_consumes_elevation_hierarchy,
        projection.review_consumes_shared_geometry,
        projection.geometry_meaning_traces_to_single_domain_contract,
        projection.support_export_reads_single_geometry_source,
    ] {
        if !ok {
            violations.push(M5GeometryHitTargetRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5GeometryHitTargetRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.foundation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5GeometryHitTargetRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5GeometryHitTargetRegistriesPacket,
    violations: &mut Vec<M5GeometryHitTargetRegistriesViolation>,
) {
    let geometries = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.geometry_entries.iter())
    };
    let hit_targets = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.hit_target_entries.iter())
    };

    // AC1: the first claimed M5 consumers use canonical spacing / sizing / radii / elevation primitives
    // rather than ad hoc local geometry. Clean geometry entries cover the spacing / sizing / radius /
    // elevation primitive kinds across the first shell / list-table / editor / dialog / review surfaces, a
    // forked example degrades, and no clean entry inlines a raw value.
    let clean_kinds: BTreeSet<String> = geometries()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.primitive_kind.clone())
        .collect();
    let primitives_covered = ["spacing", "sizing", "radius", "elevation"]
        .iter()
        .all(|k| clean_kinds.contains(*k));
    let clean_surfaces: BTreeSet<String> = geometries()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .chain(
            hit_targets()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .collect();
    let first_surfaces_covered = M5GeometrySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let forked_degrades = geometries()
        .any(|ex| ex.degrade_reason == Some(M5GeometryDegradeReason::GeometryRoleForked));
    let no_clean_raw = !geometries().any(|ex| ex.is_clean() && !ex.references_canonical_token)
        && !hit_targets().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(primitives_covered && first_surfaces_covered && forked_degrades && no_clean_raw) {
        violations
            .push(M5GeometryHitTargetRegistriesViolation::CanonicalGeometryPrimitivesNotProven);
    }

    // AC2: compact density does not shrink hit targets below supported minima, and overlays / dialogs
    // preserve the intended elevation hierarchy. A clean compact hit-target entry meets the supported
    // minimum, a sub-minimum example degrades, a clean elevation entry preserves the hierarchy, an
    // elevation-broken example degrades, and no clean hit-target entry shrinks below minimum.
    let clean_compact_meets_minimum = hit_targets()
        .any(|ex| ex.is_clean() && ex.density_mode == "compact" && ex.meets_supported_minimum);
    let sub_minimum_degrades = hit_targets()
        .any(|ex| ex.degrade_reason == Some(M5HitTargetDegradeReason::ShrinksBelowMinimum));
    let clean_elevation_preserves =
        geometries().any(|ex| ex.is_clean() && ex.is_elevation && ex.elevation_hierarchy_preserved);
    let elevation_broken_degrades = geometries()
        .any(|ex| ex.degrade_reason == Some(M5GeometryDegradeReason::ElevationHierarchyBroken));
    let no_clean_hit_target_shrinks =
        !hit_targets().any(|ex| ex.is_clean() && !ex.meets_supported_minimum);
    if !(clean_compact_meets_minimum
        && sub_minimum_degrades
        && clean_elevation_preserves
        && elevation_broken_degrades
        && no_clean_hit_target_shrinks)
    {
        violations.push(
            M5GeometryHitTargetRegistriesViolation::CompactMinimaOrElevationHierarchyNotProven,
        );
    }

    // AC3: geometry drift is visible in fixtures, linting, or proof packets before stable promotion. A
    // clean geometry entry is density-aware, a not-density-aware example degrades, a raw-geometry example
    // degrades, and no clean geometry entry forks the foundation.
    let clean_density_aware = geometries().any(|ex| ex.is_clean() && ex.density_aware);
    let not_density_aware_degrades =
        geometries().any(|ex| ex.degrade_reason == Some(M5GeometryDegradeReason::NotDensityAware));
    let raw_geometry_degrades = geometries()
        .any(|ex| ex.degrade_reason == Some(M5GeometryDegradeReason::RawGeometryValueInlined));
    let no_clean_geometry_forks =
        !geometries().any(|ex| ex.is_clean() && !ex.geometry_role_matches_kind);
    if !(clean_density_aware
        && not_density_aware_degrades
        && raw_geometry_degrades
        && no_clean_geometry_forks)
    {
        violations.push(M5GeometryHitTargetRegistriesViolation::GeometryDriftNotCaught);
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

/// The two foundation families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5VisualFoundationFamily; 2] = [
    M5VisualFoundationFamily::SpacingSizingRadiiElevation,
    M5VisualFoundationFamily::HitTarget,
];
