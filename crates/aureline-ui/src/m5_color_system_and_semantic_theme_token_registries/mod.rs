//! Implemented M5 color-system and semantic-theme-token registries.
//!
//! The frozen [visual-foundation matrix][matrix] names Aureline's eight visual-foundation families and
//! locks their controlled vocabulary. This module is the first implement lane over that matrix: it turns
//! the two foundation families that carry semantic *meaning* — the **color system** and the **semantic
//! theme token** — into registry resolvers that produce export-safe, honest projections, so a user can
//! trust that brand, interactive, neutral, and the operational status families (success / warning /
//! danger / info / insight and the trust-sensitive restricted / remote / collaboration / AI / debug
//! states) mean the same thing in dark, light, and high-contrast modes, never rely on hue alone, and
//! never drift by surface family across the shell, editor, review, notebook, and data consumers.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement canonical palette and semantic-theme registries for dark, light, and high-contrast
//!   modes, with explicit mappings for operational statuses that must remain distinct.**
//!   [`resolve_color_entry`] refuses to read as a clean, distinct color-registry entry unless it names a
//!   canonical token, a classified operational-state family, a color role, and a non-color cue, covers
//!   all three theme modes, stays distinguishable in every mode, and traces to a canonical token rather
//!   than an inlined raw color; otherwise it degrades. [`resolve_theme_token_entry`] does the same for a
//!   semantic theme-token role and refuses to inline a raw hex value or leave the dark / light /
//!   high-contrast pair incomplete.
//! * **Require text / icon / border / shape fallbacks where hue alone would otherwise carry meaning.**
//!   Every color-registry entry names an [`M5NonColorCue`] and degrades to
//!   [`M5ColorEntryDegradeReason::MeaningEncodedByColorAlone`] or
//!   [`M5ColorEntryDegradeReason::NonColorCueMissing`] when meaning would otherwise ride on hue alone.
//! * **Wire first shell, editor, review, notebook, and data consumers plus lint or fixture coverage that
//!   catches raw-color drift before release.** Each registry row carries the render [surface
//!   context][M5ColorRegistrySurfaceContext] so a drift across surfaces degrades honestly, and the
//!   acceptance-criteria gate proves a raw-color / raw-hex regression is caught before release evidence
//!   turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualSemanticRole`] role
//! vocabulary, the [`M5ColorRoleFamily`] color-role vocabulary, and the [`M5ThemeTokenRole`]
//! theme-token-role vocabulary — so shell, editor, review, notebook, data, docs, and support surfaces can
//! never fork their own color or theme meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_visual_foundation_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_color_theme_registries, seeded_m5_color_theme_registries_data_ui_preview_narrowed,
    seeded_m5_color_theme_registries_shell_ui_beta_narrowed, M5_COLOR_THEME_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_visual_foundation_matrix::{
    M5ColorRoleFamily, M5ThemeTokenRole, M5VisualFoundationAccessibilityRoute,
    M5VisualFoundationConsumerSurface, M5VisualFoundationDeploymentLine,
    M5VisualFoundationDowngradeTrigger, M5VisualFoundationFamily,
    M5VisualFoundationQualificationClass, M5VisualFoundationRequiredLabel, M5VisualSemanticRole,
    M5_COLOR_SYSTEM_SCHEMA_REF, M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
    M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ColorThemeRegistriesPacket`].
pub const M5_COLOR_THEME_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_color_system_and_semantic_theme_token_registries";

/// Schema version for M5 color / theme registry records.
pub const M5_COLOR_THEME_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_COLOR_THEME_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-color-system-and-semantic-theme-token-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_COLOR_THEME_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_color_system_and_semantic_theme_token_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLOR_THEME_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_COLOR_THEME_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COLOR_THEME_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLOR_THEME_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-color-system-and-semantic-theme-token-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5ColorThemeConsumerSurface = M5VisualFoundationConsumerSurface;

/// One of the three theme modes every semantic color / theme token must cover so meaning is stable in
/// dark, light, and high-contrast. Minted by this lane because the frozen matrix names the theme-pair
/// *rule* but not the concrete mode set a registry entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThemeMode {
    /// The dark theme mode.
    Dark,
    /// The light theme mode.
    Light,
    /// The high-contrast theme mode.
    HighContrast,
}

impl M5ThemeMode {
    /// Every theme mode, in declaration order. A distinct semantic color must cover all three.
    pub const ALL: [Self; 3] = [Self::Dark, Self::Light, Self::HighContrast];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
            Self::HighContrast => "high_contrast",
        }
    }
}

/// Controlled operational state family a color entry maps, so brand, interactive, neutral, the
/// success / warning / danger / info / insight status families, and the trust-sensitive restricted /
/// remote / collaboration / AI / debug states stop drifting by surface family. Minted by this lane
/// because the frozen matrix carries the seven high-level semantic roles but not the finer operational
/// state families the color acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OperationalStateFamily {
    /// Brand identity.
    Brand,
    /// Interactive (action / focus / selection).
    Interactive,
    /// Neutral surface / text / border.
    Neutral,
    /// Success status.
    Success,
    /// Warning status.
    Warning,
    /// Danger / error status.
    Danger,
    /// Informational status.
    Info,
    /// Insight / AI-suggestion emphasis.
    Insight,
    /// Restricted / policy-limited access.
    Restricted,
    /// Remote / networked origin.
    Remote,
    /// Collaboration / multi-user presence.
    Collaboration,
    /// AI / assistant activity.
    Ai,
    /// Debugging / diagnostic state.
    Debug,
    /// The state family is unclassified, which is disallowed.
    StateUnclassified,
}

impl M5OperationalStateFamily {
    /// Every operational state family, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::Brand,
        Self::Interactive,
        Self::Neutral,
        Self::Success,
        Self::Warning,
        Self::Danger,
        Self::Info,
        Self::Insight,
        Self::Restricted,
        Self::Remote,
        Self::Collaboration,
        Self::Ai,
        Self::Debug,
        Self::StateUnclassified,
    ];

    /// The trust-sensitive states the acceptance criteria require to stay distinguishable in every mode.
    pub const TRUST_SENSITIVE: [Self; 5] = [
        Self::Restricted,
        Self::Remote,
        Self::Collaboration,
        Self::Ai,
        Self::Debug,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Interactive => "interactive",
            Self::Neutral => "neutral",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Danger => "danger",
            Self::Info => "info",
            Self::Insight => "insight",
            Self::Restricted => "restricted",
            Self::Remote => "remote",
            Self::Collaboration => "collaboration",
            Self::Ai => "ai",
            Self::Debug => "debug",
            Self::StateUnclassified => "state_unclassified",
        }
    }

    /// Whether the state family is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::StateUnclassified)
    }

    /// Whether this is a trust-sensitive state that must stay distinguishable in every mode.
    pub const fn is_trust_sensitive(self) -> bool {
        matches!(
            self,
            Self::Restricted | Self::Remote | Self::Collaboration | Self::Ai | Self::Debug
        )
    }
}

/// Controlled non-color cue a color entry pairs with hue so meaning is never carried by color alone: a
/// text label, an icon glyph, a border treatment, a shape / pattern, or a screen-reader description.
/// Minted by this lane, tracking the text / icon / border / shape fallback the color acceptance criteria
/// require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NonColorCue {
    /// A text label carries the meaning alongside color.
    TextLabel,
    /// An icon glyph carries the meaning alongside color.
    IconGlyph,
    /// A border / outline treatment carries the meaning alongside color.
    BorderTreatment,
    /// A shape / fill pattern carries the meaning alongside color.
    ShapePattern,
    /// A screen-reader description carries the meaning alongside color.
    ScreenReaderText,
    /// No non-color cue is paired with the hue, which is disallowed.
    NoneDisallowed,
}

impl M5NonColorCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TextLabel,
        Self::IconGlyph,
        Self::BorderTreatment,
        Self::ShapePattern,
        Self::ScreenReaderText,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextLabel => "text_label",
            Self::IconGlyph => "icon_glyph",
            Self::BorderTreatment => "border_treatment",
            Self::ShapePattern => "shape_pattern",
            Self::ScreenReaderText => "screen_reader_text",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether a non-color cue is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a color or theme
/// token's meaning stays stable whether it appears in the shell, editor, review, notebook, or data
/// surface. Minted by this lane, tracking the first-consumer surfaces the implementation requirement
/// names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorRegistrySurfaceContext {
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

impl M5ColorRegistrySurfaceContext {
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

/// One mandatory rendered part a color or theme entry must be able to show, so no meaning, mode, or token
/// fact is left implicit behind hue, hover, or a single theme mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The operational state family the color maps (color entry).
    OperationalState,
    /// The theme-mode coverage (dark / light / high-contrast).
    ThemeModeCoverage,
    /// The non-color cue paired with the hue (color entry).
    NonColorCue,
    /// The color role named by the entry (color entry).
    ColorRole,
    /// The theme-token role named by the entry (theme-token entry).
    ThemeTokenRole,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the token (both entries).
    PlainLanguageMeaning,
}

impl M5ColorRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::OperationalState,
        Self::ThemeModeCoverage,
        Self::NonColorCue,
        Self::ColorRole,
        Self::ThemeTokenRole,
        Self::SurfaceContext,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::OperationalState => "operational_state",
            Self::ThemeModeCoverage => "theme_mode_coverage",
            Self::NonColorCue => "non_color_cue",
            Self::ColorRole => "color_role",
            Self::ThemeTokenRole => "theme_token_role",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect meaning,
/// mode coverage, or a degraded color / theme token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorRegistryNextAction {
    /// Expand the color's plain-language meaning.
    ExpandColorMeaning,
    /// Inspect the operational state family the color maps.
    InspectOperationalState,
    /// Complete the dark / light / high-contrast mode parity.
    CompleteThemeModeParity,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ColorRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandColorMeaning,
        Self::InspectOperationalState,
        Self::CompleteThemeModeParity,
        Self::TraceCanonicalToken,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandColorMeaning => "expand_color_meaning",
            Self::InspectOperationalState => "inspect_operational_state",
            Self::CompleteThemeModeParity => "complete_theme_mode_parity",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The foundation families covered.
    FoundationFamilies,
    /// The operational state families carried.
    OperationalStates,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The theme modes covered.
    ThemeModes,
    /// The non-color cues paired.
    NonColorCues,
    /// The render / surface context.
    SurfaceContext,
    /// The theme-token roles named.
    ThemeTokenRoles,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ColorRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::OperationalStates,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ThemeModes,
        Self::NonColorCues,
        Self::SurfaceContext,
        Self::ThemeTokenRoles,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::OperationalStates,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::FoundationFamilies => "foundation_families",
            Self::OperationalStates => "operational_states",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ThemeModes => "theme_modes",
            Self::NonColorCues => "non_color_cues",
            Self::SurfaceContext => "surface_context",
            Self::ThemeTokenRoles => "theme_token_roles",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a color entry degraded below a clean, distinct state. The degrade-first ladder returns one of
/// these instead of ever letting a color-only, raw-color, or mode-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ColorEntryDegradeReason {
    /// The canonical token name is unstated; a user cannot trace what the color means.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The operational state family is unclassified (not in the preserved taxonomy).
    OperationalStateUnclassified,
    /// The meaning is encoded by color alone rather than paired with a non-color cue.
    MeaningEncodedByColorAlone,
    /// No non-color cue (text / icon / border / shape) is paired with the hue.
    NonColorCueMissing,
    /// A raw color value is inlined instead of tracing to a canonical token.
    RawColorValueInlined,
    /// The dark / light / high-contrast theme-mode coverage is incomplete.
    ThemeModeParityIncomplete,
    /// The state is indistinguishable from another state in at least one mode.
    StateIndistinguishableAcrossModes,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ColorEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::OperationalStateUnclassified,
        Self::MeaningEncodedByColorAlone,
        Self::NonColorCueMissing,
        Self::RawColorValueInlined,
        Self::ThemeModeParityIncomplete,
        Self::StateIndistinguishableAcrossModes,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::OperationalStateUnclassified => "operational_state_unclassified",
            Self::MeaningEncodedByColorAlone => "meaning_encoded_by_color_alone",
            Self::NonColorCueMissing => "non_color_cue_missing",
            Self::RawColorValueInlined => "raw_color_value_inlined",
            Self::ThemeModeParityIncomplete => "theme_mode_parity_incomplete",
            Self::StateIndistinguishableAcrossModes => "state_indistinguishable_across_modes",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ColorRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5ColorRegistryNextAction::TraceCanonicalToken
            }
            Self::OperationalStateUnclassified | Self::StateIndistinguishableAcrossModes => {
                M5ColorRegistryNextAction::InspectOperationalState
            }
            Self::MeaningEncodedByColorAlone | Self::NonColorCueMissing => {
                M5ColorRegistryNextAction::ExpandColorMeaning
            }
            Self::ThemeModeParityIncomplete => M5ColorRegistryNextAction::CompleteThemeModeParity,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ColorRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::MeaningEncodedByColorAlone
            | Self::NonColorCueMissing
            | Self::StateIndistinguishableAcrossModes => {
                M5VisualFoundationDowngradeTrigger::StatusOrTrustCollapsedToColorOnly
            }
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::OperationalStateUnclassified | Self::SurfaceContextUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ThemeModeParityIncomplete => {
                M5VisualFoundationDowngradeTrigger::ThemePairIncomplete
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a semantic theme-token entry degraded below a clean, stable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ThemeTokenEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// A raw hex value is inlined on the surface instead of naming a token.
    RawHexInlinedInSurface,
    /// The dark / light / high-contrast theme pair is incomplete.
    ThemePairIncomplete,
    /// The theme-token role drifted across surfaces.
    ThemeRoleDriftedAcrossSurface,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ThemeTokenEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::RawHexInlinedInSurface,
        Self::ThemePairIncomplete,
        Self::ThemeRoleDriftedAcrossSurface,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RawHexInlinedInSurface => "raw_hex_inlined_in_surface",
            Self::ThemePairIncomplete => "theme_pair_incomplete",
            Self::ThemeRoleDriftedAcrossSurface => "theme_role_drifted_across_surface",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ColorRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawHexInlinedInSurface => {
                M5ColorRegistryNextAction::TraceCanonicalToken
            }
            Self::ThemePairIncomplete => M5ColorRegistryNextAction::CompleteThemeModeParity,
            Self::ThemeRoleDriftedAcrossSurface => {
                M5ColorRegistryNextAction::InspectOperationalState
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ColorRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::TokenNameUnstated | Self::RawHexInlinedInSurface => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::ThemePairIncomplete => M5VisualFoundationDowngradeTrigger::ThemePairIncomplete,
            Self::SurfaceContextUnresolved | Self::ThemeRoleDriftedAcrossSurface => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_color_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ColorEntryResolutionInput {
    /// Stable identity of the color-registry entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `color.status.danger`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The color role (from the frozen matrix vocabulary).
    pub color_role: M5ColorRoleFamily,
    /// The operational state family this color maps.
    pub operational_state: M5OperationalStateFamily,
    /// The non-color cue paired with the hue.
    pub non_color_cue: M5NonColorCue,
    /// The render / surface context.
    pub surface_context: M5ColorRegistrySurfaceContext,
    /// The theme modes this entry defines (must cover dark / light / high-contrast).
    pub defined_modes: Vec<M5ThemeMode>,
    /// True when the meaning is stated with a non-color cue, never hue alone.
    pub meaning_stated_non_color_only: bool,
    /// True when the state stays distinguishable from other states in every mode.
    pub distinguishable_in_all_modes: bool,
    /// True when the entry traces to a canonical token (never an inlined raw color value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe color-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedColorEntry {
    /// Stable identity of the color-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands a non-color cue (status / syntax / diff / chart).
    pub semantic_role_demands_non_color_cue: bool,
    /// The color-role token named by the entry.
    pub color_role: String,
    /// Whether the color role names the disallowed hue-alone-meaning token.
    pub color_role_is_hue_alone: bool,
    /// The operational-state token named by the entry.
    pub operational_state: String,
    /// Whether the operational state is classified into the preserved taxonomy.
    pub operational_state_is_classified: bool,
    /// Whether this is a trust-sensitive state that must stay distinguishable in every mode.
    pub operational_state_is_trust_sensitive: bool,
    /// The non-color-cue token named by the entry.
    pub non_color_cue: String,
    /// Whether a non-color cue is present.
    pub non_color_cue_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The theme-mode tokens covered by the entry.
    pub defined_modes: Vec<String>,
    /// Whether the entry covers all three theme modes.
    pub covers_all_modes: bool,
    /// Whether the meaning is stated with a non-color cue, never hue alone.
    pub meaning_stated_non_color_only: bool,
    /// Whether the state stays distinguishable in every mode.
    pub distinguishable_in_all_modes: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, distinct state.
    pub degrade_reason: Option<M5ColorEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ColorRegistryNextAction,
    /// Whether the meaning stays distinct across every mode (clean entry naming every fact).
    pub meaning_distinct_across_modes: bool,
}

impl M5ResolvedColorEntry {
    /// Whether this color entry reads as a clean, distinct state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_theme_token_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ThemeTokenEntryResolutionInput {
    /// Stable identity of the theme-token entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The theme-token role (from the frozen matrix vocabulary).
    pub theme_token_role: M5ThemeTokenRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The render / surface context.
    pub surface_context: M5ColorRegistrySurfaceContext,
    /// The theme modes this entry defines (must cover dark / light / high-contrast).
    pub defined_modes: Vec<M5ThemeMode>,
    /// True when the entry traces to a canonical token (never an inlined raw hex value).
    pub references_canonical_token: bool,
    /// True when the theme-token role stays stable across surfaces (no drift).
    pub role_stable_across_surfaces: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe theme-token projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedThemeTokenEntry {
    /// Stable identity of the theme-token entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The theme-token-role token named by the entry.
    pub theme_token_role: String,
    /// Whether the theme-token role names the disallowed raw-hex-in-surface token.
    pub theme_token_role_is_raw_hex: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The theme-mode tokens covered by the entry.
    pub defined_modes: Vec<String>,
    /// Whether the entry covers all three theme modes.
    pub covers_all_modes: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Whether the theme-token role stays stable across surfaces.
    pub role_stable_across_surfaces: bool,
    /// Degrade reason, if the entry could not read as a clean, stable state.
    pub degrade_reason: Option<M5ThemeTokenEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ColorRegistryNextAction,
    /// Whether the token stays stable across the dark / light / high-contrast pair (clean entry naming
    /// every fact).
    pub token_stable_across_theme_pair: bool,
}

impl M5ResolvedThemeTokenEntry {
    /// Whether this theme-token entry reads as a clean, stable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ColorThemeResolutionError {
    /// The color-entry id was empty.
    EmptyColorEntryId,
    /// The theme-token-entry id was empty.
    EmptyThemeTokenEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ColorThemeResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyColorEntryId => "empty_color_entry_id",
            Self::EmptyThemeTokenEntryId => "empty_theme_token_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ColorThemeResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 color / theme registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ColorThemeResolutionError {}

fn defined_mode_tokens(modes: &[M5ThemeMode]) -> Vec<String> {
    modes.iter().map(|m| m.as_str().to_owned()).collect()
}

fn covers_all_modes(modes: &[M5ThemeMode]) -> bool {
    let present: BTreeSet<M5ThemeMode> = modes.iter().copied().collect();
    M5ThemeMode::ALL.iter().all(|mode| present.contains(mode))
}

/// Resolves a color-registry entry so its meaning stays distinct across dark, light, and high-contrast:
/// the entry names its canonical token, semantic role, color role, operational state family, and a
/// non-color cue, covers all three theme modes, stays distinguishable in every mode, and traces to a
/// canonical token rather than an inlined raw color value.
pub fn resolve_color_entry(
    input: M5ColorEntryResolutionInput,
) -> Result<M5ResolvedColorEntry, M5ColorThemeResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ColorThemeResolutionError::EmptyColorEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ColorThemeResolutionError::ForbiddenMaterial);
    }

    let color_role_is_hue_alone = matches!(
        input.color_role,
        M5ColorRoleFamily::HueAloneMeaningDisallowed
    );
    let all_modes = covers_all_modes(&input.defined_modes);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ColorEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ColorEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.operational_state.is_classified() {
        Some(M5ColorEntryDegradeReason::OperationalStateUnclassified)
    } else if color_role_is_hue_alone || !input.meaning_stated_non_color_only {
        Some(M5ColorEntryDegradeReason::MeaningEncodedByColorAlone)
    } else if !input.non_color_cue.is_present() {
        Some(M5ColorEntryDegradeReason::NonColorCueMissing)
    } else if !input.references_canonical_token {
        Some(M5ColorEntryDegradeReason::RawColorValueInlined)
    } else if !all_modes {
        Some(M5ColorEntryDegradeReason::ThemeModeParityIncomplete)
    } else if !input.distinguishable_in_all_modes {
        Some(M5ColorEntryDegradeReason::StateIndistinguishableAcrossModes)
    } else if !input.proof_fresh {
        Some(M5ColorEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ColorRegistryNextAction::ExpandColorMeaning,
    };

    Ok(M5ResolvedColorEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_non_color_cue: input.semantic_role.demands_non_color_cue(),
        color_role: input.color_role.as_str().to_owned(),
        color_role_is_hue_alone,
        operational_state: input.operational_state.as_str().to_owned(),
        operational_state_is_classified: input.operational_state.is_classified(),
        operational_state_is_trust_sensitive: input.operational_state.is_trust_sensitive(),
        non_color_cue: input.non_color_cue.as_str().to_owned(),
        non_color_cue_present: input.non_color_cue.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        defined_modes: defined_mode_tokens(&input.defined_modes),
        covers_all_modes: all_modes,
        meaning_stated_non_color_only: input.meaning_stated_non_color_only,
        distinguishable_in_all_modes: input.distinguishable_in_all_modes,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        meaning_distinct_across_modes: degrade_reason.is_none(),
    })
}

/// Resolves a semantic theme-token entry so it stays stable across the dark / light / high-contrast
/// pair: the entry names its canonical token, theme-token role, semantic role, and surface context,
/// covers all three theme modes, keeps its role stable across surfaces, and traces to a canonical token
/// rather than an inlined raw hex value.
pub fn resolve_theme_token_entry(
    input: M5ThemeTokenEntryResolutionInput,
) -> Result<M5ResolvedThemeTokenEntry, M5ColorThemeResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ColorThemeResolutionError::EmptyThemeTokenEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5ColorThemeResolutionError::ForbiddenMaterial);
    }

    let theme_token_role_is_raw_hex = matches!(
        input.theme_token_role,
        M5ThemeTokenRole::RawHexInSurfaceDisallowed
    );
    let all_modes = covers_all_modes(&input.defined_modes);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ThemeTokenEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ThemeTokenEntryDegradeReason::SurfaceContextUnresolved)
    } else if theme_token_role_is_raw_hex || !input.references_canonical_token {
        Some(M5ThemeTokenEntryDegradeReason::RawHexInlinedInSurface)
    } else if !all_modes {
        Some(M5ThemeTokenEntryDegradeReason::ThemePairIncomplete)
    } else if !input.role_stable_across_surfaces {
        Some(M5ThemeTokenEntryDegradeReason::ThemeRoleDriftedAcrossSurface)
    } else if !input.proof_fresh {
        Some(M5ThemeTokenEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ColorRegistryNextAction::TraceCanonicalToken,
    };

    Ok(M5ResolvedThemeTokenEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        theme_token_role: input.theme_token_role.as_str().to_owned(),
        theme_token_role_is_raw_hex,
        semantic_role: input.semantic_role.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        defined_modes: defined_mode_tokens(&input.defined_modes),
        covers_all_modes: all_modes,
        references_canonical_token: input.references_canonical_token,
        role_stable_across_surfaces: input.role_stable_across_surfaces,
        degrade_reason,
        next_action,
        token_stable_across_theme_pair: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved color and theme-token entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ColorThemeConsumerSurface,
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
    pub anatomy_parts: Vec<M5ColorRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ColorRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    /// Resolved color-registry examples.
    pub color_entries: Vec<M5ResolvedColorEntry>,
    /// Resolved theme-token examples.
    pub theme_token_entries: Vec<M5ResolvedThemeTokenEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical color-system domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: status meaning never relies on color alone. MUST be `false`.
    pub status_meaning_relies_on_color_alone: bool,
    /// Hard invariant: a raw color value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_color_value_inlined_instead_of_token: bool,
    /// Hard invariant: an operational state is never indistinguishable across modes. MUST be `false`.
    pub operational_state_indistinguishable_across_modes: bool,
    /// Hard invariant: the dark / light / high-contrast theme-mode parity is never incomplete. MUST be
    /// `false`.
    pub theme_mode_parity_incomplete: bool,
}

impl M5ColorThemeRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ColorRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ColorRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ColorRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5ColorRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.status_meaning_relies_on_color_alone
            && !self.raw_color_value_inlined_instead_of_token
            && !self.operational_state_indistinguishable_across_modes
            && !self.theme_mode_parity_incomplete
    }

    /// True when a clean color entry preserves distinct meaning: it traces to a canonical token, states
    /// meaning with a non-color cue, keeps a classified operational state, covers all three modes, and
    /// stays distinguishable in every mode.
    fn color_is_honest(ex: &M5ResolvedColorEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.meaning_stated_non_color_only
                && !ex.color_role_is_hue_alone
                && ex.non_color_cue_present
                && ex.operational_state_is_classified
                && ex.covers_all_modes
                && ex.distinguishable_in_all_modes)
    }

    /// True when a clean theme-token entry preserves stability: it traces to a canonical token, never
    /// names the disallowed raw-hex role, covers all three modes, and keeps its role stable across
    /// surfaces.
    fn theme_is_honest(ex: &M5ResolvedThemeTokenEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.theme_token_role_is_raw_hex
                && ex.covers_all_modes
                && ex.role_stable_across_surfaces)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.color_entries.iter().all(Self::color_is_honest)
            && self.theme_token_entries.iter().all(Self::theme_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Color-role tokens (bound from the frozen matrix).
    pub color_roles: Vec<String>,
    /// Theme-token-role tokens (bound from the frozen matrix).
    pub theme_token_roles: Vec<String>,
    /// Theme-mode tokens (minted by this lane).
    pub theme_modes: Vec<String>,
    /// Operational-state-family tokens (minted by this lane).
    pub operational_states: Vec<String>,
    /// Non-color-cue tokens (minted by this lane).
    pub non_color_cues: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Color-entry degrade-reason tokens.
    pub color_degrade_reasons: Vec<String>,
    /// Theme-token-entry degrade-reason tokens.
    pub theme_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ColorThemeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualSemanticRole::ALL, |v| v.as_str()),
            color_roles: tokens(&M5ColorRoleFamily::ALL, |v| v.as_str()),
            theme_token_roles: tokens(&M5ThemeTokenRole::ALL, |v| v.as_str()),
            theme_modes: tokens(&M5ThemeMode::ALL, |v| v.as_str()),
            operational_states: tokens(&M5OperationalStateFamily::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5NonColorCue::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ColorRegistrySurfaceContext::ALL, |v| v.as_str()),
            color_degrade_reasons: tokens(&M5ColorEntryDegradeReason::ALL, |v| v.as_str()),
            theme_degrade_reasons: tokens(&M5ThemeTokenEntryDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ColorRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ColorRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ColorRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5ColorThemeGovernanceReview {
    /// The color registry names a canonical token, semantic role, and operational state for every entry.
    pub color_registry_names_token_role_and_state: bool,
    /// Brand, interactive, neutral, and status families stay distinct.
    pub brand_interactive_neutral_status_stay_distinct: bool,
    /// Status meaning never relies on color alone; a non-color cue is always paired.
    pub status_meaning_never_relies_on_color_alone: bool,
    /// Every color entry covers dark, light, and high-contrast modes.
    pub every_color_entry_covers_all_theme_modes: bool,
    /// The trust-sensitive restricted / remote / collaboration / AI / debug states stay distinguishable
    /// in every mode.
    pub trust_sensitive_states_distinguishable_in_every_mode: bool,
    /// Semantic theme tokens name a stable role rather than inlining a raw hex value.
    pub theme_tokens_name_stable_role_not_raw_hex: bool,
    /// Theme tokens cover the dark / light / high-contrast pair.
    pub theme_tokens_cover_dark_light_high_contrast_pair: bool,
    /// Raw-color drift is caught by fixtures or lint before release evidence turns green.
    pub raw_color_drift_caught_before_release: bool,
    /// The first shell / editor / review / notebook / data consumers use the canonical families.
    pub first_consumers_use_canonical_families: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeConsumerProjection {
    /// The shell surface consumes the shared color / theme registries.
    pub shell_consumes_shared_registries: bool,
    /// The editor surface consumes the shared color / theme registries.
    pub editor_consumes_shared_registries: bool,
    /// The review surface consumes the shared color / theme registries.
    pub review_consumes_shared_registries: bool,
    /// The notebook and data surfaces consume the shared color / theme registries.
    pub notebook_and_data_consume_shared_registries: bool,
    /// Color and theme facts trace back to one canonical color-system domain contract.
    pub color_meaning_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical color / theme registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-foundation audit for the lane.
    pub foundation_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ColorThemeRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ColorThemeRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ColorThemeRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ColorThemeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ColorThemeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ColorThemeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ColorThemeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ColorThemeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 color-system and semantic-theme-token registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ColorThemeRegistriesPacket {
    /// Record kind; must equal [`M5_COLOR_THEME_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLOR_THEME_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ColorThemeRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ColorThemeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ColorThemeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ColorThemeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ColorThemeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ColorThemeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ColorThemeRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ColorThemeRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_COLOR_THEME_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_COLOR_THEME_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ColorThemeRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COLOR_THEME_REGISTRIES_RECORD_KIND {
            violations.push(M5ColorThemeRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLOR_THEME_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5ColorThemeRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ColorThemeRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ColorThemeRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 color / theme registries packet serializes"),
        ) {
            violations.push(M5ColorThemeRegistriesViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 color / theme registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,color_entries,theme_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .color_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.theme_token_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.color_entries.len(),
                row.theme_token_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Color-System and Semantic-Theme-Token Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Operational states: {}\n",
            self.vocabulary_set.operational_states.join(", ")
        ));
        out.push_str(&format!(
            "- Theme modes: {}\n",
            self.vocabulary_set.theme_modes.join(", ")
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
                "  - Color entries: {} / theme-token entries: {}\n",
                row.color_entries.len(),
                row.theme_token_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ColorThemeRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ColorThemeRegistriesViolation>),
}

impl fmt::Display for M5ColorThemeRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 color / theme registries export parse failed: {error}"
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
                    "m5 color / theme registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ColorThemeRegistriesArtifactError {}

/// Validation failures emitted by [`M5ColorThemeRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ColorThemeRegistriesViolation {
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
    /// A registry row does not point at the canonical color-system domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (color-only, raw-color, mode-incomplete, or a
    /// theme token that inlines raw hex or drifts its role).
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
    /// First-consumer canonical adoption is not proven: clean color entries do not cover the canonical
    /// semantic-role families or the first shell / editor / review / notebook / data surfaces, no
    /// raw-color example degrades, or a clean entry inlines a raw color.
    FirstConsumersUseCanonicalFamiliesNotProven,
    /// State distinguishability across modes is not proven: clean color entries do not cover the
    /// trust-sensitive restricted / remote / collaboration / AI / debug states with full mode parity and
    /// a non-color cue, no mode-parity-incomplete or color-only example degrades, or a clean entry is
    /// indistinguishable across modes.
    StateDistinguishabilityAcrossModesNotProven,
    /// Raw-color drift is not detectable: no raw-color color example and no raw-hex theme example
    /// degrade, clean entries do not trace to a canonical token, or a clean entry inlines a raw value.
    RawColorDriftNotDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ColorThemeRegistriesViolation {
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
            Self::FirstConsumersUseCanonicalFamiliesNotProven => {
                "first_consumers_use_canonical_families_not_proven"
            }
            Self::StateDistinguishabilityAcrossModesNotProven => {
                "state_distinguishability_across_modes_not_proven"
            }
            Self::RawColorDriftNotDetectableNotProven => {
                "raw_color_drift_not_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_color_theme_registries_export(
) -> Result<M5ColorThemeRegistriesPacket, M5ColorThemeRegistriesArtifactError> {
    let packet: M5ColorThemeRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-color-system-and-semantic-theme-token-registries-proof/support_export.json"
    )))
    .map_err(M5ColorThemeRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ColorThemeRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLOR_THEME_REGISTRIES_SCHEMA_REF,
        M5_COLOR_THEME_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_COLOR_SYSTEM_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ColorThemeRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5ColorThemeRegistriesViolation::NoRegistryRows);
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
            violations.push(M5ColorThemeRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ColorThemeRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ColorThemeRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_COLOR_SYSTEM_SCHEMA_REF) {
            violations.push(M5ColorThemeRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.color_entries.is_empty() || row.theme_token_entries.is_empty() {
            violations.push(M5ColorThemeRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ColorThemeRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ColorThemeRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.color_registry_names_token_role_and_state,
        review.brand_interactive_neutral_status_stay_distinct,
        review.status_meaning_never_relies_on_color_alone,
        review.every_color_entry_covers_all_theme_modes,
        review.trust_sensitive_states_distinguishable_in_every_mode,
        review.theme_tokens_name_stable_role_not_raw_hex,
        review.theme_tokens_cover_dark_light_high_contrast_pair,
        review.raw_color_drift_caught_before_release,
        review.first_consumers_use_canonical_families,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ColorThemeRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.editor_consumes_shared_registries,
        projection.review_consumes_shared_registries,
        projection.notebook_and_data_consume_shared_registries,
        projection.color_meaning_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5ColorThemeRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ColorThemeRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.foundation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ColorThemeRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ColorThemeRegistriesPacket,
    violations: &mut Vec<M5ColorThemeRegistriesViolation>,
) {
    let colors = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.color_entries.iter())
    };
    let themes = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.theme_token_entries.iter())
    };

    // AC1: the first claimed consumers use the canonical color / state families instead of feature-local
    // palettes. Clean color entries cover the brand / interactive / neutral / status semantic-role
    // families and the first shell / editor / review / notebook / data surfaces, a raw-color example
    // degrades, and no clean entry inlines a raw color.
    let clean_semantic_roles: BTreeSet<String> = colors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = colors()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = ["brand", "interactive", "neutral", "status"]
        .iter()
        .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5ColorRegistrySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_color_degrades = colors()
        .any(|ex| ex.degrade_reason == Some(M5ColorEntryDegradeReason::RawColorValueInlined));
    let no_clean_raw_color = !colors().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(semantic_families_covered
        && first_surfaces_covered
        && raw_color_degrades
        && no_clean_raw_color)
    {
        violations
            .push(M5ColorThemeRegistriesViolation::FirstConsumersUseCanonicalFamiliesNotProven);
    }

    // AC2: restricted / remote / AI / collaboration / debug states remain distinguishable in dark,
    // light, and high-contrast modes. Clean color entries cover every trust-sensitive state with full
    // mode parity and a non-color cue, a mode-parity-incomplete example degrades, a color-only example
    // degrades, and no clean entry is indistinguishable across modes or color-only.
    let clean_trust_states: BTreeSet<String> = colors()
        .filter(|ex| {
            ex.is_clean()
                && ex.operational_state_is_trust_sensitive
                && ex.covers_all_modes
                && ex.non_color_cue_present
        })
        .map(|ex| ex.operational_state.clone())
        .collect();
    let trust_states_covered = M5OperationalStateFamily::TRUST_SENSITIVE
        .iter()
        .all(|s| clean_trust_states.contains(s.as_str()));
    let mode_incomplete_degrades = colors()
        .any(|ex| ex.degrade_reason == Some(M5ColorEntryDegradeReason::ThemeModeParityIncomplete));
    let color_only_degrades = colors()
        .any(|ex| ex.degrade_reason == Some(M5ColorEntryDegradeReason::MeaningEncodedByColorAlone));
    let no_clean_indistinct = !colors().any(|ex| {
        ex.is_clean() && (!ex.distinguishable_in_all_modes || !ex.meaning_stated_non_color_only)
    });
    if !(trust_states_covered
        && mode_incomplete_degrades
        && color_only_degrades
        && no_clean_indistinct)
    {
        violations
            .push(M5ColorThemeRegistriesViolation::StateDistinguishabilityAcrossModesNotProven);
    }

    // AC3: raw-color or ambiguous status regressions are detectable by fixtures, linting, or release
    // evidence. A raw-color color example and a raw-hex theme example both degrade, at least one clean
    // color and one clean theme entry trace to a canonical token, and no clean entry inlines a raw value.
    let raw_hex_degrades = themes().any(|ex| {
        ex.degrade_reason == Some(M5ThemeTokenEntryDegradeReason::RawHexInlinedInSurface)
    });
    let traceable_color = colors().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let traceable_theme = themes().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let no_clean_raw_theme = !themes().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(raw_color_degrades
        && raw_hex_degrades
        && traceable_color
        && traceable_theme
        && no_clean_raw_color
        && no_clean_raw_theme)
    {
        violations.push(M5ColorThemeRegistriesViolation::RawColorDriftNotDetectableNotProven);
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
    M5VisualFoundationFamily::ColorSystem,
    M5VisualFoundationFamily::SemanticThemeToken,
];
