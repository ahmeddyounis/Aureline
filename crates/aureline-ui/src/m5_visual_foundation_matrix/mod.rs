//! Frozen M5 color-system, semantic-theme-token, syntax / diff / chart-token, typography, and
//! spacing / sizing / radii / elevation visual-foundation matrix.
//!
//! This module locks Aureline's concrete visual foundation into one export-safe packet. Every claimed
//! M5 surface that still describes its own color meaning, semantic theme role, syntax / diff / chart
//! palette, type scale, font stack, or spacing / sizing / radii / elevation geometry — across the shell,
//! editor, review, data, and docs surfaces — is named once here and constrained by the same shared
//! semantic-role taxonomy (brand, interactive, neutral, status, syntax, diff, chart), the same
//! never-hue-alone rule, the same syntax / diff / diagnostics separation, the same theme-pair coverage of
//! dark / light / high-contrast, the same stable typography and font-stack rule, the same density-aware
//! geometry rule, and the same minimum-hit-target rule regardless of the feature family that renders it.
//!
//! The matrix does not re-open theme-package, import, or appearance-session object design — it is the
//! shared reusable visual-foundation contract those flows consume, and it binds back to the already-landed
//! design-system foundations and publication packets instead of leaving the baseline split across prose
//! and screenshots. The controlled vocabularies are frozen in one self-describing
//! [`M5VisualFoundationVocabularySet`] rather than minted per feature. The single controlled
//! semantic-role vocabulary consumers bind to — brand, interactive, neutral, status, syntax, diff, and
//! chart — keeps status and trust meaning from collapsing into color-only cues, keeps syntax and diff
//! palettes from colliding with diagnostics, keeps chart meaning from depending on color alone, keeps hit
//! targets from shrinking below supported minima, and keeps feature-local spacing or elevation from
//! forking away from the shared geometry contract. Raw secret values and private endpoints stay outside
//! the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_visual_foundation_matrix,
    seeded_m5_visual_foundation_matrix_chart_token_preview_narrowed,
    seeded_m5_visual_foundation_matrix_typography_beta_narrowed,
    M5_VISUAL_FOUNDATION_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5VisualFoundationMatrixPacket`].
pub const M5_VISUAL_FOUNDATION_MATRIX_RECORD_KIND: &str =
    "freeze_m5_color_system_semantic_theme_token_syntax_diff_chart_token_typography_and_spacing_sizing_radii_elevation_visual_foundation_matrix";

/// Schema version for M5 visual-foundation matrix records.
pub const M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined visual-foundation matrix schema.
pub const M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF: &str =
    "schemas/design-system/m5-visual-foundation-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_VISUAL_FOUNDATION_MATRIX_DOC_REF: &str =
    "docs/design-system/m5_visual_foundations_contract.md";

/// Repo-relative path of the canonical color-system domain schema.
pub const M5_COLOR_SYSTEM_SCHEMA_REF: &str = "schemas/design-system/m5-color-system.schema.json";

/// Repo-relative path of the canonical syntax / diff / chart token domain schema.
pub const M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF: &str =
    "schemas/design-system/m5-syntax-diff-chart-tokens.schema.json";

/// Repo-relative path of the canonical typography and geometry domain schema.
pub const M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF: &str =
    "schemas/design-system/m5-typography-and-geometry.schema.json";

/// Repo-relative path of the already-landed design-system foundations artifact the matrix binds to.
pub const M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF: &str =
    "schemas/design-system/m5-foundations.schema.json";

/// Repo-relative path of the already-landed design-system publication (foundation-package) schema the
/// matrix binds to.
pub const M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF: &str =
    "schemas/design-system/m5-foundation-package.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_VISUAL_FOUNDATION_FIXTURE_DIR: &str = "fixtures/ui/m5-visual-foundations";

/// Repo-relative path of the checked support-export artifact.
pub const M5_VISUAL_FOUNDATION_ARTIFACT_REF: &str =
    "artifacts/release/m5-visual-foundations-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_VISUAL_FOUNDATION_CSV_REF: &str =
    "artifacts/release/m5-visual-foundations-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_VISUAL_FOUNDATION_REPORT_REF: &str =
    "artifacts/design-system/m5-visual-foundations.md";

/// One of the eight governed visual-foundation families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationFamily {
    /// The color system: brand / interactive / neutral / status palettes that never rely on hue alone.
    ColorSystem,
    /// Semantic theme tokens whose roles stay stable across dark / light / high-contrast pairs.
    SemanticThemeToken,
    /// Syntax highlighting tokens that stay distinct from diagnostics.
    SyntaxToken,
    /// Diff tokens (add / remove / context) that stay distinct from diagnostics.
    DiffToken,
    /// Chart tokens whose meaning never depends on color alone.
    ChartToken,
    /// Typography: type scale, line-height, tabular numerals, and code / UI font stacks.
    Typography,
    /// Spacing, sizing, radii, and elevation geometry that stays density-aware and machine-readable.
    SpacingSizingRadiiElevation,
    /// Minimum hit-target baselines that never shrink below supported minima.
    HitTarget,
}

impl M5VisualFoundationFamily {
    /// Every governed foundation family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ColorSystem,
        Self::SemanticThemeToken,
        Self::SyntaxToken,
        Self::DiffToken,
        Self::ChartToken,
        Self::Typography,
        Self::SpacingSizingRadiiElevation,
        Self::HitTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColorSystem => "color_system",
            Self::SemanticThemeToken => "semantic_theme_token",
            Self::SyntaxToken => "syntax_token",
            Self::DiffToken => "diff_token",
            Self::ChartToken => "chart_token",
            Self::Typography => "typography",
            Self::SpacingSizingRadiiElevation => "spacing_sizing_radii_elevation",
            Self::HitTarget => "hit_target",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's color, token, typography, or geometry meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::ColorSystem | Self::SemanticThemeToken => M5_COLOR_SYSTEM_SCHEMA_REF,
            Self::SyntaxToken | Self::DiffToken | Self::ChartToken => {
                M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF
            }
            Self::Typography | Self::SpacingSizingRadiiElevation | Self::HitTarget => {
                M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF
            }
        }
    }

    /// `true` when this family must name a controlled color role.
    pub const fn declares_color_roles(self) -> bool {
        matches!(self, Self::ColorSystem)
    }

    /// `true` when this family must name a controlled theme-token role.
    pub const fn declares_theme_token_roles(self) -> bool {
        matches!(self, Self::SemanticThemeToken)
    }

    /// `true` when this family must name a controlled syntax-token role.
    pub const fn declares_syntax_roles(self) -> bool {
        matches!(self, Self::SyntaxToken)
    }

    /// `true` when this family must name a controlled diff-token role.
    pub const fn declares_diff_roles(self) -> bool {
        matches!(self, Self::DiffToken)
    }

    /// `true` when this family must name a controlled chart-token role.
    pub const fn declares_chart_roles(self) -> bool {
        matches!(self, Self::ChartToken)
    }

    /// `true` when this family must name a controlled typography role.
    pub const fn declares_typography_roles(self) -> bool {
        matches!(self, Self::Typography)
    }

    /// `true` when this family must name a controlled geometry role.
    pub const fn declares_geometry_roles(self) -> bool {
        matches!(self, Self::SpacingSizingRadiiElevation)
    }

    /// `true` when this family must name a controlled hit-target rule.
    pub const fn declares_hit_target_rules(self) -> bool {
        matches!(self, Self::HitTarget)
    }
}

/// The single controlled semantic-role vocabulary every shell, editor, review, data, or docs consumer
/// binds to. These are the exact acceptance-criteria tokens that keep `brand`, `interactive`, `neutral`,
/// `status`, `syntax`, `diff`, and `chart` meaning the same thing everywhere the visual foundation ships.
/// No feature family invents a parallel word for any of these roles, and none of them may be conveyed by
/// hue alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualSemanticRole {
    /// Brand identity color / role.
    Brand,
    /// Interactive (action / focus / selection) color / role.
    Interactive,
    /// Neutral surface / text / border role.
    Neutral,
    /// Status (info / success / warning / danger) role.
    Status,
    /// Syntax highlighting role.
    Syntax,
    /// Diff (add / remove / context) role.
    Diff,
    /// Chart / data-visualization role.
    Chart,
}

impl M5VisualSemanticRole {
    /// Every semantic role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Brand,
        Self::Interactive,
        Self::Neutral,
        Self::Status,
        Self::Syntax,
        Self::Diff,
        Self::Chart,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Interactive => "interactive",
            Self::Neutral => "neutral",
            Self::Status => "status",
            Self::Syntax => "syntax",
            Self::Diff => "diff",
            Self::Chart => "chart",
        }
    }

    /// Whether this role carries status or data meaning that must never be conveyed by color alone and
    /// must always pair color with a non-color cue (`status`, `syntax`, `diff`, `chart`).
    pub const fn demands_non_color_cue(self) -> bool {
        matches!(self, Self::Status | Self::Syntax | Self::Diff | Self::Chart)
    }
}

/// Controlled color-system role — how a color conveys meaning beyond hue, so brand, interactive, neutral,
/// and status palettes stay distinct and never rely on hue alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorRoleFamily {
    /// The brand palette family.
    BrandPalette,
    /// The interactive (action / focus) palette family.
    InteractivePalette,
    /// The neutral (surface / text / border) palette family.
    NeutralPalette,
    /// The status (info / success / warning / danger) palette family.
    StatusPalette,
    /// Color paired with a non-color cue (icon / label / shape).
    PairedWithNonColorCue,
    /// Meaning conveyed by hue alone, which is disallowed.
    HueAloneMeaningDisallowed,
}

impl M5ColorRoleFamily {
    /// Every color role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BrandPalette,
        Self::InteractivePalette,
        Self::NeutralPalette,
        Self::StatusPalette,
        Self::PairedWithNonColorCue,
        Self::HueAloneMeaningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrandPalette => "brand_palette",
            Self::InteractivePalette => "interactive_palette",
            Self::NeutralPalette => "neutral_palette",
            Self::StatusPalette => "status_palette",
            Self::PairedWithNonColorCue => "paired_with_non_color_cue",
            Self::HueAloneMeaningDisallowed => "hue_alone_meaning_disallowed",
        }
    }
}

/// Controlled semantic theme-token role — how a theme token names a stable semantic role, so surface,
/// text, border, and status roles stay stable across dark / light / high-contrast pairs and never inline a
/// raw hex value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThemeTokenRole {
    /// A surface / background role.
    SurfaceRole,
    /// A text / foreground role.
    TextRole,
    /// A border / divider role.
    BorderRole,
    /// A status accent role.
    StatusRole,
    /// A complete dark / light / high-contrast theme pair.
    ThemePairDarkLightHighContrast,
    /// A raw hex value inlined on a surface, which is disallowed.
    RawHexInSurfaceDisallowed,
}

impl M5ThemeTokenRole {
    /// Every theme-token role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SurfaceRole,
        Self::TextRole,
        Self::BorderRole,
        Self::StatusRole,
        Self::ThemePairDarkLightHighContrast,
        Self::RawHexInSurfaceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceRole => "surface_role",
            Self::TextRole => "text_role",
            Self::BorderRole => "border_role",
            Self::StatusRole => "status_role",
            Self::ThemePairDarkLightHighContrast => "theme_pair_dark_light_high_contrast",
            Self::RawHexInSurfaceDisallowed => "raw_hex_in_surface_disallowed",
        }
    }
}

/// Controlled syntax-token role — how a syntax palette names its scopes, so keyword, string, comment, and
/// identifier scopes stay distinct and never collide with diagnostic colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyntaxTokenRole {
    /// A keyword scope.
    Keyword,
    /// A string / literal scope.
    StringLiteral,
    /// A comment scope.
    Comment,
    /// An identifier / symbol scope.
    Identifier,
    /// Kept distinct from diagnostic colors.
    DistinctFromDiagnostic,
    /// Colliding with a diagnostic color, which is disallowed.
    SyntaxDiagnosticCollisionDisallowed,
}

impl M5SyntaxTokenRole {
    /// Every syntax-token role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Keyword,
        Self::StringLiteral,
        Self::Comment,
        Self::Identifier,
        Self::DistinctFromDiagnostic,
        Self::SyntaxDiagnosticCollisionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyword => "keyword",
            Self::StringLiteral => "string_literal",
            Self::Comment => "comment",
            Self::Identifier => "identifier",
            Self::DistinctFromDiagnostic => "distinct_from_diagnostic",
            Self::SyntaxDiagnosticCollisionDisallowed => "syntax_diagnostic_collision_disallowed",
        }
    }
}

/// Controlled diff-token role — how a diff palette names its regions, so addition, removal, context, and
/// moved regions stay distinct and never collide with diagnostic colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffTokenRole {
    /// An added region.
    Addition,
    /// A removed region.
    Removal,
    /// An unchanged context region.
    Context,
    /// A moved region.
    Moved,
    /// Kept distinct from diagnostic colors.
    DistinctFromDiagnostic,
    /// Colliding with a diagnostic color, which is disallowed.
    DiffDiagnosticCollisionDisallowed,
}

impl M5DiffTokenRole {
    /// Every diff-token role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Addition,
        Self::Removal,
        Self::Context,
        Self::Moved,
        Self::DistinctFromDiagnostic,
        Self::DiffDiagnosticCollisionDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Addition => "addition",
            Self::Removal => "removal",
            Self::Context => "context",
            Self::Moved => "moved",
            Self::DistinctFromDiagnostic => "distinct_from_diagnostic",
            Self::DiffDiagnosticCollisionDisallowed => "diff_diagnostic_collision_disallowed",
        }
    }
}

/// Controlled chart-token role — how a chart palette encodes data, so categorical, sequential, and
/// diverging scales always pair color with a shape or label and never depend on color alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChartTokenRole {
    /// A categorical series palette.
    CategoricalSeries,
    /// A sequential scale.
    SequentialScale,
    /// A diverging scale.
    DivergingScale,
    /// Color paired with a shape or label.
    PairedWithShapeOrLabel,
    /// Meets accessible-contrast requirements.
    AccessibleContrast,
    /// Chart meaning depending on color alone, which is disallowed.
    ChartColorAloneDisallowed,
}

impl M5ChartTokenRole {
    /// Every chart-token role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CategoricalSeries,
        Self::SequentialScale,
        Self::DivergingScale,
        Self::PairedWithShapeOrLabel,
        Self::AccessibleContrast,
        Self::ChartColorAloneDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CategoricalSeries => "categorical_series",
            Self::SequentialScale => "sequential_scale",
            Self::DivergingScale => "diverging_scale",
            Self::PairedWithShapeOrLabel => "paired_with_shape_or_label",
            Self::AccessibleContrast => "accessible_contrast",
            Self::ChartColorAloneDisallowed => "chart_color_alone_disallowed",
        }
    }
}

/// Controlled typography role — how the type system names its scales and stacks, so display and body
/// scales, code and UI font stacks, and tabular numerals stay stable and line-height never drifts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TypographyRole {
    /// The display / heading scale.
    DisplayScale,
    /// The body / paragraph scale.
    BodyScale,
    /// The code / monospace font stack.
    CodeMonoStack,
    /// The UI / sans font stack.
    UiSansStack,
    /// Tabular numerals for numeric data.
    TabularNumerals,
    /// Line-height drift, which is disallowed.
    LineHeightDriftDisallowed,
}

impl M5TypographyRole {
    /// Every typography role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DisplayScale,
        Self::BodyScale,
        Self::CodeMonoStack,
        Self::UiSansStack,
        Self::TabularNumerals,
        Self::LineHeightDriftDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DisplayScale => "display_scale",
            Self::BodyScale => "body_scale",
            Self::CodeMonoStack => "code_mono_stack",
            Self::UiSansStack => "ui_sans_stack",
            Self::TabularNumerals => "tabular_numerals",
            Self::LineHeightDriftDisallowed => "line_height_drift_disallowed",
        }
    }
}

/// Controlled geometry role — how spacing, sizing, radii, and elevation are named, so every geometry step
/// stays density-aware and machine-readable and no surface forks its own local geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GeometryRole {
    /// A spacing scale step.
    SpacingStep,
    /// A sizing scale step.
    SizingStep,
    /// A corner-radius step.
    RadiusStep,
    /// An elevation / shadow level.
    ElevationLevel,
    /// A density-aware geometry step.
    DensityAware,
    /// A feature-local geometry fork, which is disallowed.
    LocalGeometryForkDisallowed,
}

impl M5GeometryRole {
    /// Every geometry role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SpacingStep,
        Self::SizingStep,
        Self::RadiusStep,
        Self::ElevationLevel,
        Self::DensityAware,
        Self::LocalGeometryForkDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpacingStep => "spacing_step",
            Self::SizingStep => "sizing_step",
            Self::RadiusStep => "radius_step",
            Self::ElevationLevel => "elevation_level",
            Self::DensityAware => "density_aware",
            Self::LocalGeometryForkDisallowed => "local_geometry_fork_disallowed",
        }
    }
}

/// Controlled hit-target rule — how minimum interactive-target sizes are named, so comfortable, compact,
/// and coarse-pointer minima stay honored and a target never shrinks below its supported minimum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HitTargetRule {
    /// The comfortable-density minimum.
    ComfortableMinimum,
    /// The compact-density minimum.
    CompactMinimum,
    /// The coarse-pointer minimum.
    PointerCoarseMinimum,
    /// Minimum spacing between adjacent targets.
    SpacingBetweenTargets,
    /// Never below the supported minimum.
    NeverBelowSupportedMinimum,
    /// Shrinking below the supported minimum, which is disallowed.
    ShrinkBelowMinimumDisallowed,
}

impl M5HitTargetRule {
    /// Every hit-target rule, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComfortableMinimum,
        Self::CompactMinimum,
        Self::PointerCoarseMinimum,
        Self::SpacingBetweenTargets,
        Self::NeverBelowSupportedMinimum,
        Self::ShrinkBelowMinimumDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComfortableMinimum => "comfortable_minimum",
            Self::CompactMinimum => "compact_minimum",
            Self::PointerCoarseMinimum => "pointer_coarse_minimum",
            Self::SpacingBetweenTargets => "spacing_between_targets",
            Self::NeverBelowSupportedMinimum => "never_below_supported_minimum",
            Self::ShrinkBelowMinimumDisallowed => "shrink_below_minimum_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a visual-foundation family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationSurfaceFamily {
    /// The shell surface.
    Shell,
    /// The editor surface.
    Editor,
    /// The review surface.
    Review,
    /// The data surface.
    Data,
    /// The docs surface.
    Docs,
    /// The support export.
    SupportExport,
}

impl M5VisualFoundationSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Data,
        Self::Docs,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Editor => "editor",
            Self::Review => "review",
            Self::Data => "data",
            Self::Docs => "docs",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a family must survive with the same truth, so a family's color, token, typography, or
/// geometry meaning never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationDeploymentLine {
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

impl M5VisualFoundationDeploymentLine {
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
pub enum M5VisualFoundationConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The editor UI.
    EditorUi,
    /// The review UI.
    ReviewUi,
    /// The data UI.
    DataUi,
    /// The docs UI.
    DocsUi,
    /// The settings UI.
    SettingsUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5VisualFoundationConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::EditorUi,
        Self::ReviewUi,
        Self::DataUi,
        Self::DocsUi,
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
            Self::DataUi => "data_ui",
            Self::DocsUi => "docs_ui",
            Self::SettingsUi => "settings_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no color, token, typography, or geometry
/// meaning is hover-only, pointer-only, motion-only, or visually encoded alone. Records the keyboard,
/// screen-reader, high-zoom, reduced-motion, CLI/export, and support-packet requirements up front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-color cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5VisualFoundationAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a visual-foundation family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The palette source is unavailable.
    PaletteSourceUnavailable,
    /// The theme pair (dark / light / high-contrast) is incomplete.
    ThemePairIncomplete,
    /// Density metrics are unavailable.
    DensityMetricsUnavailable,
    /// The font stack is unavailable.
    FontStackUnavailable,
    /// Contrast data is unavailable.
    ContrastDataUnavailable,
}

impl M5VisualFoundationDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::PaletteSourceUnavailable,
        Self::ThemePairIncomplete,
        Self::DensityMetricsUnavailable,
        Self::FontStackUnavailable,
        Self::ContrastDataUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PaletteSourceUnavailable => "palette_source_unavailable",
            Self::ThemePairIncomplete => "theme_pair_incomplete",
            Self::DensityMetricsUnavailable => "density_metrics_unavailable",
            Self::FontStackUnavailable => "font_stack_unavailable",
            Self::ContrastDataUnavailable => "contrast_data_unavailable",
        }
    }
}

/// Mandatory label a claimed visual-foundation family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about theme
/// variant, density context, and contrast pairing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's semantic role.
    SemanticRole,
    /// The canonical token reference the family points at.
    TokenReference,
    /// The theme variant (dark / light / high-contrast) the family covers.
    ThemeVariant,
    /// The density context the family applies to.
    DensityContext,
    /// The non-color cue paired with a color role.
    ContrastPairing,
}

impl M5VisualFoundationRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::ThemeVariant,
        Self::DensityContext,
        Self::ContrastPairing,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::ThemeVariant => "theme_variant",
            Self::DensityContext => "density_context",
            Self::ContrastPairing => "contrast_pairing",
        }
    }
}

/// Qualification class for an M5 visual-foundation row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationQualificationClass {
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

impl M5VisualFoundationQualificationClass {
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

/// Downgrade trigger that narrows a visual-foundation family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualFoundationDowngradeTrigger {
    /// Status or trust meaning collapsed into a color-only cue.
    StatusOrTrustCollapsedToColorOnly,
    /// A syntax or diff palette collided with the diagnostics palette.
    SyntaxOrDiffPaletteCollidedWithDiagnostics,
    /// A hit target shrank below its supported minimum.
    HitTargetShrunkBelowMinimum,
    /// Chart meaning depended on color alone.
    ChartMeaningDependedOnColorAlone,
    /// A feature-local spacing / elevation fork drifted from the shared geometry.
    LocalGeometryForkedFromFoundation,
    /// The typography scale drifted from the shared type scale.
    TypographyScaleDrifted,
    /// A code / UI font stack became unstable.
    FontStackUnstable,
    /// A dark / light / high-contrast theme pair was incomplete.
    ThemePairIncomplete,
    /// Tabular numerals were missing for numeric data.
    TabularNumeralsMissing,
    /// A family left its semantic role unstated.
    SemanticRoleUnstated,
    /// A family left its canonical token reference unstated.
    TokenReferenceUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5VisualFoundationDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::StatusOrTrustCollapsedToColorOnly,
        Self::SyntaxOrDiffPaletteCollidedWithDiagnostics,
        Self::HitTargetShrunkBelowMinimum,
        Self::ChartMeaningDependedOnColorAlone,
        Self::LocalGeometryForkedFromFoundation,
        Self::TypographyScaleDrifted,
        Self::FontStackUnstable,
        Self::ThemePairIncomplete,
        Self::TabularNumeralsMissing,
        Self::SemanticRoleUnstated,
        Self::TokenReferenceUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusOrTrustCollapsedToColorOnly => "status_or_trust_collapsed_to_color_only",
            Self::SyntaxOrDiffPaletteCollidedWithDiagnostics => {
                "syntax_or_diff_palette_collided_with_diagnostics"
            }
            Self::HitTargetShrunkBelowMinimum => "hit_target_shrunk_below_minimum",
            Self::ChartMeaningDependedOnColorAlone => "chart_meaning_depended_on_color_alone",
            Self::LocalGeometryForkedFromFoundation => "local_geometry_forked_from_foundation",
            Self::TypographyScaleDrifted => "typography_scale_drifted",
            Self::FontStackUnstable => "font_stack_unstable",
            Self::ThemePairIncomplete => "theme_pair_incomplete",
            Self::TabularNumeralsMissing => "tabular_numerals_missing",
            Self::SemanticRoleUnstated => "semantic_role_unstated",
            Self::TokenReferenceUnstated => "token_reference_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed visual-foundation family bound to the surface-specific truth it
/// must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationRow {
    /// Governed foundation family.
    pub foundation_family: M5VisualFoundationFamily,
    /// Qualification class earned by this family.
    pub qualification: M5VisualFoundationQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5VisualFoundationSurfaceFamily>,
    /// Deployment lines this family keeps the same truth across.
    pub deployment_lines: Vec<M5VisualFoundationDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5VisualFoundationRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5VisualFoundationRequiredLabel>,
    /// Semantic roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5VisualSemanticRole>,
    /// Color roles this family names (color-system family only).
    pub color_roles: Vec<M5ColorRoleFamily>,
    /// Theme-token roles this family names (semantic-theme-token family only).
    pub theme_token_roles: Vec<M5ThemeTokenRole>,
    /// Syntax-token roles this family names (syntax-token family only).
    pub syntax_roles: Vec<M5SyntaxTokenRole>,
    /// Diff-token roles this family names (diff-token family only).
    pub diff_roles: Vec<M5DiffTokenRole>,
    /// Chart-token roles this family names (chart-token family only).
    pub chart_roles: Vec<M5ChartTokenRole>,
    /// Typography roles this family names (typography family only).
    pub typography_roles: Vec<M5TypographyRole>,
    /// Geometry roles this family names (spacing-sizing-radii-elevation family only).
    pub geometry_roles: Vec<M5GeometryRole>,
    /// Hit-target rules this family names (hit-target family only).
    pub hit_target_rules: Vec<M5HitTargetRule>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5VisualFoundationDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5VisualFoundationAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5VisualFoundationConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never collapses status or trust meaning into a color-only cue. MUST be
    /// `false`.
    pub collapses_status_or_trust_into_color_only: bool,
    /// Hard invariant: this family never lets a syntax or diff palette collide with diagnostics. MUST be
    /// `false`.
    pub lets_syntax_or_diff_palette_collide_with_diagnostics: bool,
    /// Hard invariant: this family never shrinks a hit target below its supported minimum. MUST be
    /// `false`.
    pub shrinks_hit_target_below_supported_minimum: bool,
    /// Hard invariant: this family never lets chart meaning depend on color alone. MUST be `false`.
    pub lets_chart_meaning_depend_on_color_alone: bool,
    /// Hard invariant: this family never forks local spacing or elevation from the shared geometry. MUST
    /// be `false`.
    pub forks_local_spacing_or_elevation_from_shared_geometry: bool,
}

impl M5VisualFoundationRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5VisualFoundationRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5VisualFoundationRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_status_or_trust_into_color_only
            && !self.lets_syntax_or_diff_palette_collide_with_diagnostics
            && !self.shrinks_hit_target_below_supported_minimum
            && !self.lets_chart_meaning_depend_on_color_alone
            && !self.forks_local_spacing_or_elevation_from_shared_geometry
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationVocabularySet {
    /// Foundation-family tokens.
    pub foundation_families: Vec<String>,
    /// Semantic-role tokens.
    pub semantic_roles: Vec<String>,
    /// Color-role tokens.
    pub color_roles: Vec<String>,
    /// Theme-token-role tokens.
    pub theme_token_roles: Vec<String>,
    /// Syntax-token-role tokens.
    pub syntax_roles: Vec<String>,
    /// Diff-token-role tokens.
    pub diff_roles: Vec<String>,
    /// Chart-token-role tokens.
    pub chart_roles: Vec<String>,
    /// Typography-role tokens.
    pub typography_roles: Vec<String>,
    /// Geometry-role tokens.
    pub geometry_roles: Vec<String>,
    /// Hit-target-rule tokens.
    pub hit_target_rules: Vec<String>,
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

impl M5VisualFoundationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            foundation_families: tokens(&M5VisualFoundationFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5VisualSemanticRole::ALL, |v| v.as_str()),
            color_roles: tokens(&M5ColorRoleFamily::ALL, |v| v.as_str()),
            theme_token_roles: tokens(&M5ThemeTokenRole::ALL, |v| v.as_str()),
            syntax_roles: tokens(&M5SyntaxTokenRole::ALL, |v| v.as_str()),
            diff_roles: tokens(&M5DiffTokenRole::ALL, |v| v.as_str()),
            chart_roles: tokens(&M5ChartTokenRole::ALL, |v| v.as_str()),
            typography_roles: tokens(&M5TypographyRole::ALL, |v| v.as_str()),
            geometry_roles: tokens(&M5GeometryRole::ALL, |v| v.as_str()),
            hit_target_rules: tokens(&M5HitTargetRule::ALL, |v| v.as_str()),
            surface_families: tokens(&M5VisualFoundationSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5VisualFoundationDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5VisualFoundationConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5VisualFoundationAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5VisualFoundationDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5VisualFoundationRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5VisualFoundationDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5VisualFoundationGovernanceReview {
    /// Brand, interactive, neutral, and status semantics stay distinct.
    pub brand_interactive_neutral_status_stay_distinct: bool,
    /// Status and trust meaning is never conveyed by color alone.
    pub status_meaning_never_color_alone: bool,
    /// Syntax, diff, and chart palettes never collide with diagnostics.
    pub syntax_diff_chart_never_collide_with_diagnostics: bool,
    /// Chart meaning never depends on color alone.
    pub chart_meaning_never_color_alone: bool,
    /// Semantic theme roles bind to the appearance-session / design-system tokens.
    pub semantic_theme_roles_bind_to_appearance_session: bool,
    /// Theme pairs cover dark, light, and high-contrast.
    pub theme_pairs_cover_dark_light_high_contrast: bool,
    /// Typography scale and line-height stay stable.
    pub typography_scale_and_line_height_stable: bool,
    /// Tabular numerals are available for numeric data.
    pub tabular_numerals_available_for_numeric_data: bool,
    /// Code and UI font stacks stay stable.
    pub code_and_ui_font_stacks_stable: bool,
    /// Spacing, sizing, radii, and elevation stay density-aware.
    pub spacing_sizing_radii_elevation_density_aware: bool,
    /// Geometry rules stay machine-readable.
    pub geometry_rules_machine_readable: bool,
    /// Hit targets never shrink below supported minima.
    pub hit_targets_never_below_supported_minimum: bool,
    /// No surface invents local geometry or color meaning.
    pub no_surface_invents_local_geometry_or_color_meaning: bool,
    /// Every family keeps the same truth across every deployment line.
    pub every_family_declares_deployment_lines: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel visual vocabulary.
    pub later_rows_cannot_invent_parallel_visual_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationConsumerProjection {
    /// Shell and editor consume the shared visual foundation.
    pub shell_and_editor_consume_shared_visual_foundation: bool,
    /// Review and data consume the shared token families.
    pub review_and_data_consume_shared_token_families: bool,
    /// Docs consume the shared typography and geometry.
    pub docs_consume_shared_typography_and_geometry: bool,
    /// Syntax, diff, and chart consumers read a single token source.
    pub syntax_diff_chart_consumers_read_single_token_source: bool,
    /// The appearance session binds to the shared theme tokens.
    pub appearance_session_binds_to_shared_theme_tokens: bool,
    /// Support / export reads a single canonical visual-foundation source.
    pub support_export_reads_single_visual_foundation_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the visual-foundation lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-foundation audit for the lane.
    pub foundation_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5VisualFoundationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5VisualFoundationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Foundation rows.
    pub foundation_rows: Vec<M5VisualFoundationRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5VisualFoundationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5VisualFoundationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5VisualFoundationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5VisualFoundationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5VisualFoundationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 visual-foundation matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualFoundationMatrixPacket {
    /// Record kind; must equal [`M5_VISUAL_FOUNDATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Foundation rows.
    pub foundation_rows: Vec<M5VisualFoundationRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5VisualFoundationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5VisualFoundationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5VisualFoundationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5VisualFoundationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5VisualFoundationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5VisualFoundationMatrixPacket {
    /// Builds an M5 visual-foundation matrix packet from stable-lane input.
    pub fn new(input: M5VisualFoundationMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_VISUAL_FOUNDATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            foundation_rows: input.foundation_rows,
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

    /// Validates the M5 visual-foundation matrix invariants.
    pub fn validate(&self) -> Vec<M5VisualFoundationMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_VISUAL_FOUNDATION_MATRIX_RECORD_KIND {
            violations.push(M5VisualFoundationMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_VERSION {
            violations.push(M5VisualFoundationMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5VisualFoundationMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_foundation_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 visual-foundation matrix serializes"),
        ) {
            violations.push(M5VisualFoundationMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 visual-foundation matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "foundation_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.foundation_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.foundation_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.foundation_family.canonical_domain_schema_ref(),
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
            .foundation_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Color-System, Semantic-Theme-Token, Syntax / Diff / Chart-Token, Typography, and Spacing / Sizing / Radii / Elevation Visual-Foundation Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Foundation families: {} ({} stable)\n",
            self.foundation_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Semantic roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Color roles: {}\n",
            self.vocabulary_set.color_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Foundation families\n\n");
        for row in &self.foundation_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.foundation_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.foundation_family.canonical_domain_schema_ref()
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

/// Errors emitted when reading the checked-in M5 visual-foundation matrix export.
#[derive(Debug)]
pub enum M5VisualFoundationMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5VisualFoundationMatrixViolation>),
}

impl fmt::Display for M5VisualFoundationMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 visual-foundation matrix export parse failed: {error}"
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
                    "m5 visual-foundation matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5VisualFoundationMatrixArtifactError {}

/// Validation failures emitted by [`M5VisualFoundationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5VisualFoundationMatrixViolation {
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
    /// A required governed foundation family is missing from the matrix.
    RequiredFamilyMissing,
    /// A foundation row is incomplete.
    FoundationRowIncomplete,
    /// A foundation row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A foundation row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no semantic roles.
    SemanticRoleMissing,
    /// The color-system family declares no color roles.
    ColorRoleMissing,
    /// The semantic-theme-token family declares no theme-token roles.
    ThemeTokenRoleMissing,
    /// The syntax-token family declares no syntax roles.
    SyntaxRoleMissing,
    /// The diff-token family declares no diff roles.
    DiffRoleMissing,
    /// The chart-token family declares no chart roles.
    ChartRoleMissing,
    /// The typography family declares no typography roles.
    TypographyRoleMissing,
    /// The spacing-sizing-radii-elevation family declares no geometry roles.
    GeometryRoleMissing,
    /// The hit-target family declares no hit-target rules.
    HitTargetRuleMissing,
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
    /// A family violates a hard invariant (status/trust color-only collapse, syntax/diff diagnostics
    /// collision, hit target below minimum, chart color-only meaning, or a local geometry fork).
    FoundationInvariantViolated,
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

impl M5VisualFoundationMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::FoundationRowIncomplete => "foundation_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::ColorRoleMissing => "color_role_missing",
            Self::ThemeTokenRoleMissing => "theme_token_role_missing",
            Self::SyntaxRoleMissing => "syntax_role_missing",
            Self::DiffRoleMissing => "diff_role_missing",
            Self::ChartRoleMissing => "chart_role_missing",
            Self::TypographyRoleMissing => "typography_role_missing",
            Self::GeometryRoleMissing => "geometry_role_missing",
            Self::HitTargetRuleMissing => "hit_target_rule_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::FoundationInvariantViolated => "foundation_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 visual-foundation matrix export.
pub fn current_stable_m5_visual_foundation_matrix_export(
) -> Result<M5VisualFoundationMatrixPacket, M5VisualFoundationMatrixArtifactError> {
    let packet: M5VisualFoundationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-visual-foundations-proof/support_export.json"
    )))
    .map_err(M5VisualFoundationMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5VisualFoundationMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_COLOR_SYSTEM_SCHEMA_REF,
        M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5VisualFoundationMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5VisualFoundationMatrixViolation::VocabularySetDrift);
    }
}

fn validate_foundation_rows(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    let present: BTreeSet<M5VisualFoundationFamily> = packet
        .foundation_rows
        .iter()
        .map(|row| row.foundation_family)
        .collect();
    for required in M5VisualFoundationFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5VisualFoundationMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.foundation_rows {
        let family = row.foundation_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5VisualFoundationMatrixViolation::FoundationRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5VisualFoundationMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5VisualFoundationMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_color_roles() && row.color_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::ColorRoleMissing);
        }
        if family.declares_theme_token_roles() && row.theme_token_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::ThemeTokenRoleMissing);
        }
        if family.declares_syntax_roles() && row.syntax_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::SyntaxRoleMissing);
        }
        if family.declares_diff_roles() && row.diff_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::DiffRoleMissing);
        }
        if family.declares_chart_roles() && row.chart_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::ChartRoleMissing);
        }
        if family.declares_typography_roles() && row.typography_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::TypographyRoleMissing);
        }
        if family.declares_geometry_roles() && row.geometry_roles.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::GeometryRoleMissing);
        }
        if family.declares_hit_target_rules() && row.hit_target_rules.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::HitTargetRuleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5VisualFoundationMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5VisualFoundationMatrixViolation::FoundationInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.brand_interactive_neutral_status_stay_distinct,
        review.status_meaning_never_color_alone,
        review.syntax_diff_chart_never_collide_with_diagnostics,
        review.chart_meaning_never_color_alone,
        review.semantic_theme_roles_bind_to_appearance_session,
        review.theme_pairs_cover_dark_light_high_contrast,
        review.typography_scale_and_line_height_stable,
        review.tabular_numerals_available_for_numeric_data,
        review.code_and_ui_font_stacks_stable,
        review.spacing_sizing_radii_elevation_density_aware,
        review.geometry_rules_machine_readable,
        review.hit_targets_never_below_supported_minimum,
        review.no_surface_invents_local_geometry_or_color_meaning,
        review.every_family_declares_deployment_lines,
        review.every_family_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_visual_vocabulary,
    ] {
        if !ok {
            violations.push(M5VisualFoundationMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_editor_consume_shared_visual_foundation,
        projection.review_and_data_consume_shared_token_families,
        projection.docs_consume_shared_typography_and_geometry,
        projection.syntax_diff_chart_consumers_read_single_token_source,
        projection.appearance_session_binds_to_shared_theme_tokens,
        projection.support_export_reads_single_visual_foundation_source,
    ] {
        if !ok {
            violations.push(M5VisualFoundationMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5VisualFoundationMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5VisualFoundationMatrixPacket,
    violations: &mut Vec<M5VisualFoundationMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.foundation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5VisualFoundationMatrixViolation::ReleasePostureIncomplete);
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
/// vocabulary deliberately uses color / token / typography / geometry words; what is rejected is a raw
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
