//! Implemented M5 syntax-token, diff-token, and chart-token registries.
//!
//! The frozen [visual-foundation matrix][matrix] names Aureline's eight visual-foundation families and
//! locks their controlled vocabulary. The [color / theme registries lane][color] already turned the two
//! meaning-carrying color families into registry resolvers. This module is the next implement lane over
//! that matrix: it turns the three foundation families that carry *code and data* meaning — the **syntax
//! token**, the **diff token**, and the **chart token** — into registry resolvers that produce
//! export-safe, honest projections, so a user can trust that source-code highlighting, diff regions, and
//! chart series mean the same thing across the editor, review, notebook, data, and docs consumers, never
//! collide with the diagnostics palette, never depend on hue alone, and never lose their meaning under
//! high contrast or export.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement canonical syntax-role tokens, diff-role tokens, and chart series families with explicit
//!   notes for moved-block confidence, historical-vs-current emphasis, and diagnostics precedence over
//!   syntax color.** [`resolve_syntax_entry`] refuses to read as a clean syntax-registry entry unless it
//!   names a canonical token, keeps its scope distinct from the diagnostics palette, and lets diagnostics
//!   outrank syntax wherever they overlap. [`resolve_diff_entry`] refuses to read as clean unless it names
//!   a canonical token, stays distinct from diagnostics, states its moved-block confidence and its
//!   historical-vs-current emphasis, and pairs a non-color cue. [`resolve_chart_entry`] refuses to read as
//!   clean unless it names a canonical token, pairs a legend / pattern / marker cue, and meets accessible
//!   contrast.
//! * **Require non-color-only chart / diff cues such as labels, patterns, markers, or legends wherever
//!   meaning would otherwise collapse under high contrast or export.** Every diff and chart entry names an
//!   [`M5CodeDataNonColorCue`] and degrades to
//!   [`M5DiffEntryDegradeReason::NonColorCueMissing`] or
//!   [`M5ChartEntryDegradeReason::LegendOrPatternMissing`] when meaning would otherwise ride on hue alone.
//! * **Wire export / render fixtures so charts, diffs, and code views preserve meaning in screenshots,
//!   PDFs, and support packets.** Each entry carries the [export channels][M5MeaningExportChannel] it
//!   survives, and an entry that cannot survive the required screenshot / PDF / support-packet /
//!   high-contrast channels degrades to `ExportMeaningLost` before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualSemanticRole`] role
//! vocabulary and the [`M5SyntaxTokenRole`] / [`M5DiffTokenRole`] / [`M5ChartTokenRole`] family-role
//! vocabularies — so the editor, review, notebook, data, docs, and support surfaces can never fork their
//! own syntax, diff, or chart meaning. Raw secret values and private endpoints stay outside the export
//! boundary.
//!
//! [matrix]: crate::m5_visual_foundation_matrix
//! [color]: crate::m5_color_system_and_semantic_theme_token_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_syntax_diff_chart_registries,
    seeded_m5_syntax_diff_chart_registries_data_ui_preview_narrowed,
    seeded_m5_syntax_diff_chart_registries_editor_ui_beta_narrowed,
    M5_SYNTAX_DIFF_CHART_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_visual_foundation_matrix::{
    M5ChartTokenRole, M5DiffTokenRole, M5SyntaxTokenRole, M5VisualFoundationAccessibilityRoute,
    M5VisualFoundationConsumerSurface, M5VisualFoundationDeploymentLine,
    M5VisualFoundationDowngradeTrigger, M5VisualFoundationFamily,
    M5VisualFoundationQualificationClass, M5VisualFoundationRequiredLabel, M5VisualSemanticRole,
    M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF, M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
    M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5SyntaxDiffChartRegistriesPacket`].
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_syntax_diff_and_chart_token_registries";

/// Schema version for M5 syntax / diff / chart registry records.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-syntax-diff-and-chart-token-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_syntax_diff_and_chart_token_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-syntax-diff-and-chart-token-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-syntax-diff-and-chart-token-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-syntax-diff-and-chart-token-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SYNTAX_DIFF_CHART_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-syntax-diff-and-chart-token-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5CodeDataConsumerSurface = M5VisualFoundationConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the registry entry, so syntax, diff, and
/// chart meaning stays stable whether it appears in the editor, review, notebook, data, or docs surface.
/// Minted by this lane, tracking the first-consumer surfaces the implementation requirement names
/// directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeDataSurfaceContext {
    /// The editor surface (source-code highlighting).
    Editor,
    /// The review surface (diff regions).
    Review,
    /// The notebook surface (code cells and inline diffs).
    Notebook,
    /// The data surface (charts and data-visualization series).
    Data,
    /// The docs surface (rendered code, diffs, and charts).
    Docs,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5CodeDataSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
        Self::Docs,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Editor,
        Self::Review,
        Self::Notebook,
        Self::Data,
        Self::Docs,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Review => "review",
            Self::Notebook => "notebook",
            Self::Data => "data",
            Self::Docs => "docs",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled non-color cue a diff or chart entry pairs with hue so meaning is never carried by color
/// alone: a text label, a fill pattern, a series marker, a legend, or a screen-reader description. Minted
/// by this lane, tracking the labels / patterns / markers / legends the acceptance criteria require by
/// name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeDataNonColorCue {
    /// A text label carries the meaning alongside color.
    TextLabel,
    /// A fill / hatch pattern carries the meaning alongside color.
    FillPattern,
    /// A series / point marker carries the meaning alongside color.
    SeriesMarker,
    /// A legend carries the meaning alongside color.
    Legend,
    /// A screen-reader description carries the meaning alongside color.
    ScreenReaderText,
    /// No non-color cue is paired with the hue, which is disallowed.
    NoneDisallowed,
}

impl M5CodeDataNonColorCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TextLabel,
        Self::FillPattern,
        Self::SeriesMarker,
        Self::Legend,
        Self::ScreenReaderText,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextLabel => "text_label",
            Self::FillPattern => "fill_pattern",
            Self::SeriesMarker => "series_marker",
            Self::Legend => "legend",
            Self::ScreenReaderText => "screen_reader_text",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether a non-color cue is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled export / render channel a syntax, diff, or chart entry must keep its meaning across, so a
/// code view, a diff, or a chart preserves meaning in a screenshot, a PDF, a support packet, and under
/// high contrast. Minted by this lane, tracking the export requirement the implementation calls out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MeaningExportChannel {
    /// A rendered screenshot.
    Screenshot,
    /// An exported PDF.
    Pdf,
    /// A support / evidence packet.
    SupportPacket,
    /// A high-contrast rendering.
    HighContrast,
    /// A monochrome / grayscale print.
    MonochromePrint,
    /// The channel cannot currently be resolved.
    ChannelUnknown,
}

impl M5MeaningExportChannel {
    /// Every export channel, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Screenshot,
        Self::Pdf,
        Self::SupportPacket,
        Self::HighContrast,
        Self::MonochromePrint,
        Self::ChannelUnknown,
    ];

    /// The four channels a clean entry must keep its meaning across.
    pub const REQUIRED: [Self; 4] = [
        Self::Screenshot,
        Self::Pdf,
        Self::SupportPacket,
        Self::HighContrast,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Screenshot => "screenshot",
            Self::Pdf => "pdf",
            Self::SupportPacket => "support_packet",
            Self::HighContrast => "high_contrast",
            Self::MonochromePrint => "monochrome_print",
            Self::ChannelUnknown => "channel_unknown",
        }
    }
}

/// Controlled diagnostics-precedence posture a syntax entry declares, so a diagnostic underline, squiggle,
/// or gutter marker visually outranks syntax color wherever the two overlap. Minted by this lane, tracking
/// the diagnostics-precedence note the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyntaxDiagnosticsPosture {
    /// Diagnostics outrank syntax color where they overlap.
    DiagnosticsOutrankSyntax,
    /// Syntax and diagnostics never occupy the same channel.
    DistinctNoOverlap,
    /// Syntax color outranks diagnostics, which is disallowed.
    SyntaxOutranksDiagnosticsDisallowed,
    /// The precedence posture is unstated.
    PostureUnknown,
}

impl M5SyntaxDiagnosticsPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::DiagnosticsOutrankSyntax,
        Self::DistinctNoOverlap,
        Self::SyntaxOutranksDiagnosticsDisallowed,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsOutrankSyntax => "diagnostics_outrank_syntax",
            Self::DistinctNoOverlap => "distinct_no_overlap",
            Self::SyntaxOutranksDiagnosticsDisallowed => "syntax_outranks_diagnostics_disallowed",
            Self::PostureUnknown => "posture_unknown",
        }
    }

    /// Whether diagnostics precedence is honored: diagnostics either outrank syntax or never overlap it.
    pub const fn honors_diagnostics_precedence(self) -> bool {
        matches!(
            self,
            Self::DiagnosticsOutrankSyntax | Self::DistinctNoOverlap
        )
    }
}

/// Controlled moved-block confidence a diff entry declares, so a moved region is honestly labeled by how
/// confidently it was detected rather than silently colored like an ordinary add / remove. Minted by this
/// lane, tracking the moved-block-confidence note the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffMovedConfidence {
    /// A high-confidence, exact move.
    HighConfidenceMove,
    /// A likely move.
    LikelyMove,
    /// A heuristic / low-confidence move.
    HeuristicMove,
    /// Not a moved region.
    NotAMove,
    /// The moved-block confidence is unstated.
    ConfidenceUnknown,
}

impl M5DiffMovedConfidence {
    /// Every moved-block confidence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::HighConfidenceMove,
        Self::LikelyMove,
        Self::HeuristicMove,
        Self::NotAMove,
        Self::ConfidenceUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HighConfidenceMove => "high_confidence_move",
            Self::LikelyMove => "likely_move",
            Self::HeuristicMove => "heuristic_move",
            Self::NotAMove => "not_a_move",
            Self::ConfidenceUnknown => "confidence_unknown",
        }
    }

    /// Whether the moved-block confidence is stated (never the unknown sentinel).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::ConfidenceUnknown)
    }
}

/// Controlled historical-vs-current emphasis a diff entry declares, so a review can tell whether a region
/// emphasizes the current revision, the historical baseline, or holds them in balance. Minted by this
/// lane, tracking the historical-vs-current-emphasis note the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffEmphasis {
    /// The current revision is emphasized.
    CurrentEmphasis,
    /// The historical baseline is emphasized.
    HistoricalEmphasis,
    /// Current and historical are held in balance.
    BalancedEmphasis,
    /// The emphasis is unstated.
    EmphasisUnknown,
}

impl M5DiffEmphasis {
    /// Every emphasis, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CurrentEmphasis,
        Self::HistoricalEmphasis,
        Self::BalancedEmphasis,
        Self::EmphasisUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentEmphasis => "current_emphasis",
            Self::HistoricalEmphasis => "historical_emphasis",
            Self::BalancedEmphasis => "balanced_emphasis",
            Self::EmphasisUnknown => "emphasis_unknown",
        }
    }

    /// Whether the emphasis is stated (never the unknown sentinel).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::EmphasisUnknown)
    }
}

/// One mandatory rendered part a syntax, diff, or chart entry must be able to show, so no meaning, cue, or
/// token fact is left implicit behind hue, hover, or a single render channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeDataAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The family role the entry names (syntax scope, diff region, or chart series).
    FamilyRole,
    /// The render / surface context.
    SurfaceContext,
    /// The non-color cue paired with the hue (diff / chart entry).
    NonColorCue,
    /// The diagnostics-precedence posture (syntax entry).
    DiagnosticsPosture,
    /// The moved-block confidence (diff entry).
    MovedConfidence,
    /// The historical-vs-current emphasis (diff entry).
    HistoricalEmphasis,
    /// The export channels the entry survives.
    ExportChannels,
    /// The plain-language meaning of the token.
    PlainLanguageMeaning,
}

impl M5CodeDataAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::FamilyRole,
        Self::SurfaceContext,
        Self::NonColorCue,
        Self::DiagnosticsPosture,
        Self::MovedConfidence,
        Self::HistoricalEmphasis,
        Self::ExportChannels,
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
            Self::FamilyRole => "family_role",
            Self::SurfaceContext => "surface_context",
            Self::NonColorCue => "non_color_cue",
            Self::DiagnosticsPosture => "diagnostics_posture",
            Self::MovedConfidence => "moved_confidence",
            Self::HistoricalEmphasis => "historical_emphasis",
            Self::ExportChannels => "export_channels",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect meaning,
/// precedence, parity, or a degraded syntax / diff / chart token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeDataNextAction {
    /// Inspect the diagnostics-precedence posture over syntax color.
    InspectDiagnosticsPrecedence,
    /// Add a legend / pattern / marker / label cue.
    AddLegendOrPattern,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Verify the entry survives every required export channel.
    VerifyExportSurvival,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5CodeDataNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectDiagnosticsPrecedence,
        Self::AddLegendOrPattern,
        Self::TraceCanonicalToken,
        Self::VerifyExportSurvival,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectDiagnosticsPrecedence => "inspect_diagnostics_precedence",
            Self::AddLegendOrPattern => "add_legend_or_pattern",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::VerifyExportSurvival => "verify_export_survival",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CodeDataExportField {
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
    /// The syntax roles named.
    SyntaxRoles,
    /// The diff roles named.
    DiffRoles,
    /// The chart roles named.
    ChartRoles,
    /// The non-color cues paired.
    NonColorCues,
    /// The export channels the entries survive.
    ExportChannels,
    /// The accountable owner role.
    OwnerRole,
}

impl M5CodeDataExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::SemanticRoles,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SyntaxRoles,
        Self::DiffRoles,
        Self::ChartRoles,
        Self::NonColorCues,
        Self::ExportChannels,
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
            Self::SyntaxRoles => "syntax_roles",
            Self::DiffRoles => "diff_roles",
            Self::ChartRoles => "chart_roles",
            Self::NonColorCues => "non_color_cues",
            Self::ExportChannels => "export_channels",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a syntax entry degraded below a clean state. The degrade-first ladder returns one of these
/// instead of ever letting a diagnostics-colliding, precedence-losing, raw-color, or export-losing entry
/// read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SyntaxEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The syntax scope collides with a diagnostics color.
    SyntaxCollidesWithDiagnostics,
    /// Diagnostics do not outrank syntax color where they overlap.
    DiagnosticsPrecedenceMissing,
    /// A raw color value is inlined instead of tracing to a canonical token.
    RawColorValueInlined,
    /// The meaning is lost under at least one required export / high-contrast channel.
    ExportMeaningLost,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SyntaxEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::SyntaxCollidesWithDiagnostics,
        Self::DiagnosticsPrecedenceMissing,
        Self::RawColorValueInlined,
        Self::ExportMeaningLost,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SyntaxCollidesWithDiagnostics => "syntax_collides_with_diagnostics",
            Self::DiagnosticsPrecedenceMissing => "diagnostics_precedence_missing",
            Self::RawColorValueInlined => "raw_color_value_inlined",
            Self::ExportMeaningLost => "export_meaning_lost",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CodeDataNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5CodeDataNextAction::TraceCanonicalToken
            }
            Self::SyntaxCollidesWithDiagnostics | Self::DiagnosticsPrecedenceMissing => {
                M5CodeDataNextAction::InspectDiagnosticsPrecedence
            }
            Self::ExportMeaningLost => M5CodeDataNextAction::VerifyExportSurvival,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5CodeDataNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::SyntaxCollidesWithDiagnostics | Self::DiagnosticsPrecedenceMissing => {
                M5VisualFoundationDowngradeTrigger::SyntaxOrDiffPaletteCollidedWithDiagnostics
            }
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::ExportMeaningLost => {
                M5VisualFoundationDowngradeTrigger::StatusOrTrustCollapsedToColorOnly
            }
            Self::SurfaceContextUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a diff entry degraded below a clean state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiffEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The diff region collides with a diagnostics color.
    DiffCollidesWithDiagnostics,
    /// No non-color cue (label / pattern / marker) is paired with the hue.
    NonColorCueMissing,
    /// The moved-block confidence is unstated.
    MovedConfidenceUnstated,
    /// The historical-vs-current emphasis is unstated.
    HistoricalEmphasisUnstated,
    /// A raw color value is inlined instead of tracing to a canonical token.
    RawColorValueInlined,
    /// The meaning is lost under at least one required export / high-contrast channel.
    ExportMeaningLost,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5DiffEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::DiffCollidesWithDiagnostics,
        Self::NonColorCueMissing,
        Self::MovedConfidenceUnstated,
        Self::HistoricalEmphasisUnstated,
        Self::RawColorValueInlined,
        Self::ExportMeaningLost,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DiffCollidesWithDiagnostics => "diff_collides_with_diagnostics",
            Self::NonColorCueMissing => "non_color_cue_missing",
            Self::MovedConfidenceUnstated => "moved_confidence_unstated",
            Self::HistoricalEmphasisUnstated => "historical_emphasis_unstated",
            Self::RawColorValueInlined => "raw_color_value_inlined",
            Self::ExportMeaningLost => "export_meaning_lost",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CodeDataNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5CodeDataNextAction::TraceCanonicalToken
            }
            Self::DiffCollidesWithDiagnostics => M5CodeDataNextAction::InspectDiagnosticsPrecedence,
            Self::NonColorCueMissing => M5CodeDataNextAction::AddLegendOrPattern,
            Self::ExportMeaningLost => M5CodeDataNextAction::VerifyExportSurvival,
            Self::MovedConfidenceUnstated
            | Self::HistoricalEmphasisUnstated
            | Self::SurfaceContextUnresolved
            | Self::ProofStale => M5CodeDataNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::DiffCollidesWithDiagnostics => {
                M5VisualFoundationDowngradeTrigger::SyntaxOrDiffPaletteCollidedWithDiagnostics
            }
            Self::NonColorCueMissing | Self::ExportMeaningLost => {
                M5VisualFoundationDowngradeTrigger::StatusOrTrustCollapsedToColorOnly
            }
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::MovedConfidenceUnstated
            | Self::HistoricalEmphasisUnstated
            | Self::SurfaceContextUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a chart entry degraded below a clean state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChartEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The chart meaning is encoded by color alone rather than paired with a non-color cue.
    ChartMeaningColorAlone,
    /// No legend or pattern is present where color alone would be insufficient.
    LegendOrPatternMissing,
    /// The chart series does not meet accessible contrast.
    ContrastInsufficient,
    /// A raw color value is inlined instead of tracing to a canonical token.
    RawColorValueInlined,
    /// The meaning is lost under at least one required export / high-contrast channel.
    ExportMeaningLost,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ChartEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::ChartMeaningColorAlone,
        Self::LegendOrPatternMissing,
        Self::ContrastInsufficient,
        Self::RawColorValueInlined,
        Self::ExportMeaningLost,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ChartMeaningColorAlone => "chart_meaning_color_alone",
            Self::LegendOrPatternMissing => "legend_or_pattern_missing",
            Self::ContrastInsufficient => "contrast_insufficient",
            Self::RawColorValueInlined => "raw_color_value_inlined",
            Self::ExportMeaningLost => "export_meaning_lost",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5CodeDataNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5CodeDataNextAction::TraceCanonicalToken
            }
            Self::ChartMeaningColorAlone
            | Self::LegendOrPatternMissing
            | Self::ContrastInsufficient => M5CodeDataNextAction::AddLegendOrPattern,
            Self::ExportMeaningLost => M5CodeDataNextAction::VerifyExportSurvival,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5CodeDataNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::ChartMeaningColorAlone
            | Self::LegendOrPatternMissing
            | Self::ContrastInsufficient
            | Self::ExportMeaningLost => {
                M5VisualFoundationDowngradeTrigger::ChartMeaningDependedOnColorAlone
            }
            Self::TokenNameUnstated | Self::RawColorValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_syntax_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SyntaxEntryResolutionInput {
    /// Stable identity of the syntax-registry entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `syntax.keyword`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The syntax-token role (from the frozen matrix vocabulary).
    pub syntax_role: M5SyntaxTokenRole,
    /// The diagnostics-precedence posture the entry declares.
    pub diagnostics_posture: M5SyntaxDiagnosticsPosture,
    /// The render / surface context.
    pub surface_context: M5CodeDataSurfaceContext,
    /// The export channels this entry keeps its meaning across.
    pub export_channels: Vec<M5MeaningExportChannel>,
    /// True when the entry traces to a canonical token (never an inlined raw color value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe syntax-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSyntaxEntry {
    /// Stable identity of the syntax-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands a non-color cue (status / syntax / diff / chart).
    pub semantic_role_demands_non_color_cue: bool,
    /// The syntax-role token named by the entry.
    pub syntax_role: String,
    /// Whether the syntax role names the disallowed diagnostics-collision token.
    pub syntax_role_collides_with_diagnostics: bool,
    /// The diagnostics-precedence-posture token named by the entry.
    pub diagnostics_posture: String,
    /// Whether diagnostics precedence is honored (diagnostics outrank syntax or never overlap).
    pub diagnostics_precedence_honored: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The export-channel tokens the entry survives.
    pub export_channels: Vec<String>,
    /// Whether the entry survives every required export / high-contrast channel.
    pub survives_required_export_channels: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5SyntaxEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CodeDataNextAction,
    /// Whether diagnostics outrank syntax color for a clean entry naming every fact.
    pub diagnostics_outrank_syntax: bool,
}

impl M5ResolvedSyntaxEntry {
    /// Whether this syntax entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_diff_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiffEntryResolutionInput {
    /// Stable identity of the diff-registry entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The diff-token role (from the frozen matrix vocabulary).
    pub diff_role: M5DiffTokenRole,
    /// The moved-block confidence the entry declares.
    pub moved_confidence: M5DiffMovedConfidence,
    /// The historical-vs-current emphasis the entry declares.
    pub historical_emphasis: M5DiffEmphasis,
    /// The non-color cue paired with the hue.
    pub non_color_cue: M5CodeDataNonColorCue,
    /// The render / surface context.
    pub surface_context: M5CodeDataSurfaceContext,
    /// The export channels this entry keeps its meaning across.
    pub export_channels: Vec<M5MeaningExportChannel>,
    /// True when the diff palette stays distinct from the diagnostics palette.
    pub distinct_from_diagnostics: bool,
    /// True when the entry traces to a canonical token (never an inlined raw color value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe diff-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDiffEntry {
    /// Stable identity of the diff-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands a non-color cue.
    pub semantic_role_demands_non_color_cue: bool,
    /// The diff-role token named by the entry.
    pub diff_role: String,
    /// Whether the diff role names the disallowed diagnostics-collision token.
    pub diff_role_collides_with_diagnostics: bool,
    /// The moved-block-confidence token named by the entry.
    pub moved_confidence: String,
    /// Whether the moved-block confidence is stated.
    pub moved_confidence_stated: bool,
    /// The historical-vs-current-emphasis token named by the entry.
    pub historical_emphasis: String,
    /// Whether the historical-vs-current emphasis is stated.
    pub historical_emphasis_stated: bool,
    /// The non-color-cue token named by the entry.
    pub non_color_cue: String,
    /// Whether a non-color cue is present.
    pub non_color_cue_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The export-channel tokens the entry survives.
    pub export_channels: Vec<String>,
    /// Whether the entry survives every required export / high-contrast channel.
    pub survives_required_export_channels: bool,
    /// Whether the diff palette stays distinct from the diagnostics palette.
    pub distinct_from_diagnostics: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5DiffEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CodeDataNextAction,
    /// Whether the diff meaning survives export / high-contrast for a clean entry naming every fact.
    pub meaning_survives_export: bool,
}

impl M5ResolvedDiffEntry {
    /// Whether this diff entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_chart_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ChartEntryResolutionInput {
    /// Stable identity of the chart-registry entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The chart-token role (from the frozen matrix vocabulary).
    pub chart_role: M5ChartTokenRole,
    /// The non-color cue paired with the hue.
    pub non_color_cue: M5CodeDataNonColorCue,
    /// The render / surface context.
    pub surface_context: M5CodeDataSurfaceContext,
    /// The export channels this entry keeps its meaning across.
    pub export_channels: Vec<M5MeaningExportChannel>,
    /// True when a legend or pattern is present where color alone would be insufficient.
    pub legend_or_pattern_present: bool,
    /// True when the chart series meets accessible contrast.
    pub accessible_contrast: bool,
    /// True when the entry traces to a canonical token (never an inlined raw color value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe chart-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedChartEntry {
    /// Stable identity of the chart-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands a non-color cue.
    pub semantic_role_demands_non_color_cue: bool,
    /// The chart-role token named by the entry.
    pub chart_role: String,
    /// Whether the chart role names the disallowed color-alone token.
    pub chart_role_is_color_alone: bool,
    /// The non-color-cue token named by the entry.
    pub non_color_cue: String,
    /// Whether a non-color cue is present.
    pub non_color_cue_present: bool,
    /// Whether a legend or pattern is present where color alone would be insufficient.
    pub legend_or_pattern_present: bool,
    /// Whether the chart series meets accessible contrast.
    pub accessible_contrast: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The export-channel tokens the entry survives.
    pub export_channels: Vec<String>,
    /// Whether the entry survives every required export / high-contrast channel.
    pub survives_required_export_channels: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5ChartEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5CodeDataNextAction,
    /// Whether the chart meaning survives without relying on color for a clean entry naming every fact.
    pub meaning_survives_without_color: bool,
}

impl M5ResolvedChartEntry {
    /// Whether this chart entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5SyntaxDiffChartResolutionError {
    /// The syntax-entry id was empty.
    EmptySyntaxEntryId,
    /// The diff-entry id was empty.
    EmptyDiffEntryId,
    /// The chart-entry id was empty.
    EmptyChartEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5SyntaxDiffChartResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySyntaxEntryId => "empty_syntax_entry_id",
            Self::EmptyDiffEntryId => "empty_diff_entry_id",
            Self::EmptyChartEntryId => "empty_chart_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5SyntaxDiffChartResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 syntax / diff / chart registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SyntaxDiffChartResolutionError {}

fn export_channel_tokens(channels: &[M5MeaningExportChannel]) -> Vec<String> {
    channels.iter().map(|c| c.as_str().to_owned()).collect()
}

fn survives_required_export_channels(channels: &[M5MeaningExportChannel]) -> bool {
    let present: BTreeSet<M5MeaningExportChannel> = channels.iter().copied().collect();
    M5MeaningExportChannel::REQUIRED
        .iter()
        .all(|channel| present.contains(channel))
}

/// Resolves a syntax-registry entry so its scope stays distinct from the diagnostics palette and lets
/// diagnostics outrank syntax color where they overlap: the entry names its canonical token, keeps its
/// scope distinct from diagnostics, honors diagnostics precedence, and survives every required export
/// channel.
pub fn resolve_syntax_entry(
    input: M5SyntaxEntryResolutionInput,
) -> Result<M5ResolvedSyntaxEntry, M5SyntaxDiffChartResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SyntaxDiffChartResolutionError::EmptySyntaxEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5SyntaxDiffChartResolutionError::ForbiddenMaterial);
    }

    let syntax_role_collides = matches!(
        input.syntax_role,
        M5SyntaxTokenRole::SyntaxDiagnosticCollisionDisallowed
    );
    let precedence_honored = input.diagnostics_posture.honors_diagnostics_precedence();
    let survives_export = survives_required_export_channels(&input.export_channels);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SyntaxEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SyntaxEntryDegradeReason::SurfaceContextUnresolved)
    } else if syntax_role_collides {
        Some(M5SyntaxEntryDegradeReason::SyntaxCollidesWithDiagnostics)
    } else if !precedence_honored {
        Some(M5SyntaxEntryDegradeReason::DiagnosticsPrecedenceMissing)
    } else if !input.references_canonical_token {
        Some(M5SyntaxEntryDegradeReason::RawColorValueInlined)
    } else if !survives_export {
        Some(M5SyntaxEntryDegradeReason::ExportMeaningLost)
    } else if !input.proof_fresh {
        Some(M5SyntaxEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CodeDataNextAction::InspectDiagnosticsPrecedence,
    };

    Ok(M5ResolvedSyntaxEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_non_color_cue: input.semantic_role.demands_non_color_cue(),
        syntax_role: input.syntax_role.as_str().to_owned(),
        syntax_role_collides_with_diagnostics: syntax_role_collides,
        diagnostics_posture: input.diagnostics_posture.as_str().to_owned(),
        diagnostics_precedence_honored: precedence_honored,
        surface_context: input.surface_context.as_str().to_owned(),
        export_channels: export_channel_tokens(&input.export_channels),
        survives_required_export_channels: survives_export,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        diagnostics_outrank_syntax: degrade_reason.is_none(),
    })
}

/// Resolves a diff-registry entry so its regions stay distinct from diagnostics and survive high contrast
/// and export: the entry names its canonical token, keeps its palette distinct from diagnostics, pairs a
/// non-color cue, states its moved-block confidence and historical-vs-current emphasis, and survives every
/// required export channel.
pub fn resolve_diff_entry(
    input: M5DiffEntryResolutionInput,
) -> Result<M5ResolvedDiffEntry, M5SyntaxDiffChartResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SyntaxDiffChartResolutionError::EmptyDiffEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5SyntaxDiffChartResolutionError::ForbiddenMaterial);
    }

    let diff_role_collides = matches!(
        input.diff_role,
        M5DiffTokenRole::DiffDiagnosticCollisionDisallowed
    );
    let survives_export = survives_required_export_channels(&input.export_channels);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5DiffEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5DiffEntryDegradeReason::SurfaceContextUnresolved)
    } else if diff_role_collides || !input.distinct_from_diagnostics {
        Some(M5DiffEntryDegradeReason::DiffCollidesWithDiagnostics)
    } else if !input.non_color_cue.is_present() {
        Some(M5DiffEntryDegradeReason::NonColorCueMissing)
    } else if !input.moved_confidence.is_stated() {
        Some(M5DiffEntryDegradeReason::MovedConfidenceUnstated)
    } else if !input.historical_emphasis.is_stated() {
        Some(M5DiffEntryDegradeReason::HistoricalEmphasisUnstated)
    } else if !input.references_canonical_token {
        Some(M5DiffEntryDegradeReason::RawColorValueInlined)
    } else if !survives_export {
        Some(M5DiffEntryDegradeReason::ExportMeaningLost)
    } else if !input.proof_fresh {
        Some(M5DiffEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CodeDataNextAction::AddLegendOrPattern,
    };

    Ok(M5ResolvedDiffEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_non_color_cue: input.semantic_role.demands_non_color_cue(),
        diff_role: input.diff_role.as_str().to_owned(),
        diff_role_collides_with_diagnostics: diff_role_collides,
        moved_confidence: input.moved_confidence.as_str().to_owned(),
        moved_confidence_stated: input.moved_confidence.is_stated(),
        historical_emphasis: input.historical_emphasis.as_str().to_owned(),
        historical_emphasis_stated: input.historical_emphasis.is_stated(),
        non_color_cue: input.non_color_cue.as_str().to_owned(),
        non_color_cue_present: input.non_color_cue.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        export_channels: export_channel_tokens(&input.export_channels),
        survives_required_export_channels: survives_export,
        distinct_from_diagnostics: input.distinct_from_diagnostics,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        meaning_survives_export: degrade_reason.is_none(),
    })
}

/// Resolves a chart-registry entry so its meaning never depends on color alone and survives high contrast
/// and export: the entry names its canonical token, pairs a legend / pattern / marker cue, meets
/// accessible contrast, and survives every required export channel.
pub fn resolve_chart_entry(
    input: M5ChartEntryResolutionInput,
) -> Result<M5ResolvedChartEntry, M5SyntaxDiffChartResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5SyntaxDiffChartResolutionError::EmptyChartEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5SyntaxDiffChartResolutionError::ForbiddenMaterial);
    }

    let chart_role_is_color_alone = matches!(
        input.chart_role,
        M5ChartTokenRole::ChartColorAloneDisallowed
    );
    let survives_export = survives_required_export_channels(&input.export_channels);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ChartEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ChartEntryDegradeReason::SurfaceContextUnresolved)
    } else if chart_role_is_color_alone || !input.non_color_cue.is_present() {
        Some(M5ChartEntryDegradeReason::ChartMeaningColorAlone)
    } else if !input.legend_or_pattern_present {
        Some(M5ChartEntryDegradeReason::LegendOrPatternMissing)
    } else if !input.accessible_contrast {
        Some(M5ChartEntryDegradeReason::ContrastInsufficient)
    } else if !input.references_canonical_token {
        Some(M5ChartEntryDegradeReason::RawColorValueInlined)
    } else if !survives_export {
        Some(M5ChartEntryDegradeReason::ExportMeaningLost)
    } else if !input.proof_fresh {
        Some(M5ChartEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5CodeDataNextAction::AddLegendOrPattern,
    };

    Ok(M5ResolvedChartEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_non_color_cue: input.semantic_role.demands_non_color_cue(),
        chart_role: input.chart_role.as_str().to_owned(),
        chart_role_is_color_alone,
        non_color_cue: input.non_color_cue.as_str().to_owned(),
        non_color_cue_present: input.non_color_cue.is_present(),
        legend_or_pattern_present: input.legend_or_pattern_present,
        accessible_contrast: input.accessible_contrast,
        surface_context: input.surface_context.as_str().to_owned(),
        export_channels: export_channel_tokens(&input.export_channels),
        survives_required_export_channels: survives_export,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        meaning_survives_without_color: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved syntax, diff, and chart entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyntaxDiffChartRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5CodeDataConsumerSurface,
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
    pub anatomy_parts: Vec<M5CodeDataAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5CodeDataExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    /// Resolved syntax-registry examples.
    pub syntax_entries: Vec<M5ResolvedSyntaxEntry>,
    /// Resolved diff-registry examples.
    pub diff_entries: Vec<M5ResolvedDiffEntry>,
    /// Resolved chart-registry examples.
    pub chart_entries: Vec<M5ResolvedChartEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical syntax/diff/chart domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a syntax or diff palette never collides with diagnostics. MUST be `false`.
    pub syntax_or_diff_palette_collides_with_diagnostics: bool,
    /// Hard invariant: chart meaning never relies on color alone. MUST be `false`.
    pub chart_meaning_relies_on_color_alone: bool,
    /// Hard invariant: diff / chart / code meaning is never lost under high contrast or export. MUST be
    /// `false`.
    pub meaning_lost_under_high_contrast_or_export: bool,
    /// Hard invariant: a raw color value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_color_value_inlined_instead_of_token: bool,
}

impl M5SyntaxDiffChartRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CodeDataAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5CodeDataAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CodeDataExportField> = self.export_fields.iter().copied().collect();
        M5CodeDataExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.syntax_or_diff_palette_collides_with_diagnostics
            && !self.chart_meaning_relies_on_color_alone
            && !self.meaning_lost_under_high_contrast_or_export
            && !self.raw_color_value_inlined_instead_of_token
    }

    fn has_any_entry(&self) -> bool {
        !self.syntax_entries.is_empty()
            || !self.diff_entries.is_empty()
            || !self.chart_entries.is_empty()
    }

    /// True when a clean syntax entry preserves distinct meaning: it traces to a canonical token, never
    /// collides with diagnostics, honors diagnostics precedence, and survives every required export
    /// channel.
    fn syntax_is_honest(ex: &M5ResolvedSyntaxEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.syntax_role_collides_with_diagnostics
                && ex.diagnostics_precedence_honored
                && ex.survives_required_export_channels)
    }

    /// True when a clean diff entry preserves distinct meaning: it traces to a canonical token, never
    /// collides with diagnostics, pairs a non-color cue, states its moved-block confidence and emphasis,
    /// and survives every required export channel.
    fn diff_is_honest(ex: &M5ResolvedDiffEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.diff_role_collides_with_diagnostics
                && ex.distinct_from_diagnostics
                && ex.non_color_cue_present
                && ex.moved_confidence_stated
                && ex.historical_emphasis_stated
                && ex.survives_required_export_channels)
    }

    /// True when a clean chart entry preserves distinct meaning: it traces to a canonical token, never
    /// depends on color alone, pairs a legend / pattern cue, meets accessible contrast, and survives every
    /// required export channel.
    fn chart_is_honest(ex: &M5ResolvedChartEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.chart_role_is_color_alone
                && ex.non_color_cue_present
                && ex.legend_or_pattern_present
                && ex.accessible_contrast
                && ex.survives_required_export_channels)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.syntax_entries.iter().all(Self::syntax_is_honest)
            && self.diff_entries.iter().all(Self::diff_is_honest)
            && self.chart_entries.iter().all(Self::chart_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyntaxDiffChartVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Syntax-role tokens (bound from the frozen matrix).
    pub syntax_roles: Vec<String>,
    /// Diff-role tokens (bound from the frozen matrix).
    pub diff_roles: Vec<String>,
    /// Chart-role tokens (bound from the frozen matrix).
    pub chart_roles: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Non-color-cue tokens (minted by this lane).
    pub non_color_cues: Vec<String>,
    /// Export-channel tokens (minted by this lane).
    pub export_channels: Vec<String>,
    /// Diagnostics-precedence-posture tokens (minted by this lane).
    pub diagnostics_postures: Vec<String>,
    /// Moved-block-confidence tokens (minted by this lane).
    pub moved_confidences: Vec<String>,
    /// Historical-vs-current-emphasis tokens (minted by this lane).
    pub historical_emphases: Vec<String>,
    /// Syntax-entry degrade-reason tokens.
    pub syntax_degrade_reasons: Vec<String>,
    /// Diff-entry degrade-reason tokens.
    pub diff_degrade_reasons: Vec<String>,
    /// Chart-entry degrade-reason tokens.
    pub chart_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SyntaxDiffChartVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualSemanticRole::ALL, |v| v.as_str()),
            syntax_roles: tokens(&M5SyntaxTokenRole::ALL, |v| v.as_str()),
            diff_roles: tokens(&M5DiffTokenRole::ALL, |v| v.as_str()),
            chart_roles: tokens(&M5ChartTokenRole::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5CodeDataSurfaceContext::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5CodeDataNonColorCue::ALL, |v| v.as_str()),
            export_channels: tokens(&M5MeaningExportChannel::ALL, |v| v.as_str()),
            diagnostics_postures: tokens(&M5SyntaxDiagnosticsPosture::ALL, |v| v.as_str()),
            moved_confidences: tokens(&M5DiffMovedConfidence::ALL, |v| v.as_str()),
            historical_emphases: tokens(&M5DiffEmphasis::ALL, |v| v.as_str()),
            syntax_degrade_reasons: tokens(&M5SyntaxEntryDegradeReason::ALL, |v| v.as_str()),
            diff_degrade_reasons: tokens(&M5DiffEntryDegradeReason::ALL, |v| v.as_str()),
            chart_degrade_reasons: tokens(&M5ChartEntryDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5CodeDataAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5CodeDataNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CodeDataExportField::ALL, |v| v.as_str()),
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
pub struct M5SyntaxDiffChartGovernanceReview {
    /// Syntax, diff, and chart surfaces share one stable semantic mapping.
    pub syntax_diff_chart_share_one_semantic_mapping: bool,
    /// Diagnostics visually outrank syntax color wherever they overlap.
    pub diagnostics_outrank_syntax_where_they_overlap: bool,
    /// Syntax and diff palettes never collide with the diagnostics palette.
    pub syntax_and_diff_never_collide_with_diagnostics: bool,
    /// Diff meaning survives high contrast and export.
    pub diff_meaning_survives_high_contrast_and_export: bool,
    /// Chart meaning never relies on color alone.
    pub chart_meaning_never_relies_on_color_alone: bool,
    /// A legend or pattern is present wherever color alone would be insufficient.
    pub legend_or_pattern_parity_present_where_color_insufficient: bool,
    /// Moved-block confidence and historical-vs-current emphasis are stated on diff regions.
    pub moved_block_confidence_and_historical_emphasis_stated: bool,
    /// Raw-color drift is caught by fixtures or lint before release evidence turns green.
    pub raw_color_drift_caught_before_release: bool,
    /// The first editor / review / notebook / data / docs consumers use the canonical families.
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
pub struct M5SyntaxDiffChartConsumerProjection {
    /// The editor surface consumes the shared syntax tokens.
    pub editor_consumes_shared_syntax_tokens: bool,
    /// The review surface consumes the shared diff tokens.
    pub review_consumes_shared_diff_tokens: bool,
    /// The data surface consumes the shared chart tokens.
    pub data_consumes_shared_chart_tokens: bool,
    /// The docs and notebook surfaces consume the shared registries.
    pub docs_and_notebook_consume_shared_registries: bool,
    /// Code and data meaning trace back to one canonical syntax/diff/chart domain contract.
    pub code_and_data_meaning_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyntaxDiffChartProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyntaxDiffChartReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-foundation audit for the lane.
    pub foundation_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SyntaxDiffChartRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SyntaxDiffChartRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SyntaxDiffChartRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SyntaxDiffChartVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SyntaxDiffChartGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SyntaxDiffChartConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SyntaxDiffChartProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SyntaxDiffChartReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 syntax-token, diff-token, and chart-token registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SyntaxDiffChartRegistriesPacket {
    /// Record kind; must equal [`M5_SYNTAX_DIFF_CHART_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SyntaxDiffChartRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SyntaxDiffChartVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SyntaxDiffChartGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SyntaxDiffChartConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SyntaxDiffChartProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SyntaxDiffChartReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SyntaxDiffChartRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SyntaxDiffChartRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SYNTAX_DIFF_CHART_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SyntaxDiffChartRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SYNTAX_DIFF_CHART_REGISTRIES_RECORD_KIND {
            violations.push(M5SyntaxDiffChartRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5SyntaxDiffChartRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SyntaxDiffChartRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 syntax / diff / chart registries packet serializes"),
        ) {
            violations.push(M5SyntaxDiffChartRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 syntax / diff / chart registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,syntax_entries,diff_entries,chart_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .syntax_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.diff_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .chain(
                    row.chart_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.syntax_entries.len(),
                row.diff_entries.len(),
                row.chart_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Syntax-, Diff-, and Chart-Token Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Syntax roles: {}\n",
            self.vocabulary_set.syntax_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Diff roles: {}\n",
            self.vocabulary_set.diff_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Chart roles: {}\n",
            self.vocabulary_set.chart_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Export channels: {}\n",
            self.vocabulary_set.export_channels.join(", ")
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
                "  - Syntax: {} / diff: {} / chart: {}\n",
                row.syntax_entries.len(),
                row.diff_entries.len(),
                row.chart_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SyntaxDiffChartRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SyntaxDiffChartRegistriesViolation>),
}

impl fmt::Display for M5SyntaxDiffChartRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 syntax / diff / chart registries export parse failed: {error}"
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
                    "m5 syntax / diff / chart registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SyntaxDiffChartRegistriesArtifactError {}

/// Validation failures emitted by [`M5SyntaxDiffChartRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SyntaxDiffChartRegistriesViolation {
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
    /// A registry row does not point at the canonical syntax/diff/chart domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (diagnostics collision, color-only, export-losing,
    /// or a raw-color inlining that still reads as clean).
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
    /// One shared semantic mapping is not proven: clean syntax / diff / chart entries do not cover the
    /// canonical semantic roles across the first editor / review / notebook / data / docs surfaces, no
    /// raw-color example degrades, or a clean entry inlines a raw color.
    SharedSemanticMappingNotProven,
    /// Diagnostics precedence or export survival is not proven: no clean syntax entry honors diagnostics
    /// precedence, no syntax-diagnostics collision example degrades, no clean diff and chart entries
    /// survive export, or no export-loss example degrades.
    DiagnosticsPrecedenceOrExportSurvivalNotProven,
    /// Legend / pattern parity is not proven: clean chart / diff entries do not pair a non-color cue, no
    /// color-alone chart example degrades, no cue-missing diff example degrades, or a clean chart / diff
    /// entry lacks the cue.
    LegendOrPatternParityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SyntaxDiffChartRegistriesViolation {
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
            Self::SharedSemanticMappingNotProven => "shared_semantic_mapping_not_proven",
            Self::DiagnosticsPrecedenceOrExportSurvivalNotProven => {
                "diagnostics_precedence_or_export_survival_not_proven"
            }
            Self::LegendOrPatternParityNotProven => "legend_or_pattern_parity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_syntax_diff_chart_registries_export(
) -> Result<M5SyntaxDiffChartRegistriesPacket, M5SyntaxDiffChartRegistriesArtifactError> {
    let packet: M5SyntaxDiffChartRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-syntax-diff-and-chart-token-registries-proof/support_export.json"
    )))
    .map_err(M5SyntaxDiffChartRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SyntaxDiffChartRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SYNTAX_DIFF_CHART_REGISTRIES_SCHEMA_REF,
        M5_SYNTAX_DIFF_CHART_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SyntaxDiffChartRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SyntaxDiffChartRegistriesViolation::NoRegistryRows);
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
            violations.push(M5SyntaxDiffChartRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SYNTAX_DIFF_CHART_TOKENS_SCHEMA_REF) {
            violations.push(M5SyntaxDiffChartRegistriesViolation::DomainSchemaRefMissing);
        }
        if !row.has_any_entry() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5SyntaxDiffChartRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.syntax_diff_chart_share_one_semantic_mapping,
        review.diagnostics_outrank_syntax_where_they_overlap,
        review.syntax_and_diff_never_collide_with_diagnostics,
        review.diff_meaning_survives_high_contrast_and_export,
        review.chart_meaning_never_relies_on_color_alone,
        review.legend_or_pattern_parity_present_where_color_insufficient,
        review.moved_block_confidence_and_historical_emphasis_stated,
        review.raw_color_drift_caught_before_release,
        review.first_consumers_use_canonical_families,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5SyntaxDiffChartRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_consumes_shared_syntax_tokens,
        projection.review_consumes_shared_diff_tokens,
        projection.data_consumes_shared_chart_tokens,
        projection.docs_and_notebook_consume_shared_registries,
        projection.code_and_data_meaning_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5SyntaxDiffChartRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SyntaxDiffChartRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.foundation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SyntaxDiffChartRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5SyntaxDiffChartRegistriesPacket,
    violations: &mut Vec<M5SyntaxDiffChartRegistriesViolation>,
) {
    let syntax = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.syntax_entries.iter())
    };
    let diffs = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.diff_entries.iter())
    };
    let charts = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.chart_entries.iter())
    };

    // AC1: syntax, diff, and chart surfaces share one stable semantic mapping. Clean entries of each
    // family name the canonical syntax / diff / chart semantic roles, cover the first editor / review /
    // notebook / data / docs surfaces, a raw-color example degrades, and no clean entry inlines a raw
    // color.
    let clean_syntax_role = syntax().any(|ex| ex.is_clean() && ex.semantic_role == "syntax");
    let clean_diff_role = diffs().any(|ex| ex.is_clean() && ex.semantic_role == "diff");
    let clean_chart_role = charts().any(|ex| ex.is_clean() && ex.semantic_role == "chart");
    let clean_surfaces: BTreeSet<String> = syntax()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .chain(
            diffs()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .chain(
            charts()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .collect();
    let first_surfaces_covered = M5CodeDataSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_color_degrades = syntax()
        .any(|ex| ex.degrade_reason == Some(M5SyntaxEntryDegradeReason::RawColorValueInlined))
        || diffs()
            .any(|ex| ex.degrade_reason == Some(M5DiffEntryDegradeReason::RawColorValueInlined))
        || charts()
            .any(|ex| ex.degrade_reason == Some(M5ChartEntryDegradeReason::RawColorValueInlined));
    let no_clean_raw = !syntax().any(|ex| ex.is_clean() && !ex.references_canonical_token)
        && !diffs().any(|ex| ex.is_clean() && !ex.references_canonical_token)
        && !charts().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(clean_syntax_role
        && clean_diff_role
        && clean_chart_role
        && first_surfaces_covered
        && raw_color_degrades
        && no_clean_raw)
    {
        violations.push(M5SyntaxDiffChartRegistriesViolation::SharedSemanticMappingNotProven);
    }

    // AC2: diagnostics visually outrank syntax where they overlap, and diff / chart meaning survives
    // high-contrast and export paths. A clean syntax entry honors diagnostics precedence, a
    // syntax-diagnostics collision example degrades, clean diff and chart entries survive export, and an
    // export-loss example degrades.
    let clean_syntax_precedence =
        syntax().any(|ex| ex.is_clean() && ex.diagnostics_precedence_honored);
    let syntax_collision_degrades = syntax().any(|ex| {
        ex.degrade_reason == Some(M5SyntaxEntryDegradeReason::SyntaxCollidesWithDiagnostics)
    });
    let clean_diff_survives_export =
        diffs().any(|ex| ex.is_clean() && ex.survives_required_export_channels);
    let clean_chart_survives_export =
        charts().any(|ex| ex.is_clean() && ex.survives_required_export_channels);
    let export_loss_degrades = syntax()
        .any(|ex| ex.degrade_reason == Some(M5SyntaxEntryDegradeReason::ExportMeaningLost))
        || diffs().any(|ex| ex.degrade_reason == Some(M5DiffEntryDegradeReason::ExportMeaningLost))
        || charts()
            .any(|ex| ex.degrade_reason == Some(M5ChartEntryDegradeReason::ExportMeaningLost));
    if !(clean_syntax_precedence
        && syntax_collision_degrades
        && clean_diff_survives_export
        && clean_chart_survives_export
        && export_loss_degrades)
    {
        violations.push(
            M5SyntaxDiffChartRegistriesViolation::DiagnosticsPrecedenceOrExportSurvivalNotProven,
        );
    }

    // AC3: legend / pattern parity exists wherever color alone would be insufficient. Clean chart and diff
    // entries pair a non-color cue, a color-alone chart example degrades, a cue-missing diff example
    // degrades, and no clean chart / diff entry lacks the cue.
    let clean_chart_has_cue = charts()
        .any(|ex| ex.is_clean() && ex.non_color_cue_present && ex.legend_or_pattern_present);
    let clean_diff_has_cue = diffs().any(|ex| ex.is_clean() && ex.non_color_cue_present);
    let chart_color_alone_degrades = charts()
        .any(|ex| ex.degrade_reason == Some(M5ChartEntryDegradeReason::ChartMeaningColorAlone));
    let diff_cue_missing_degrades =
        diffs().any(|ex| ex.degrade_reason == Some(M5DiffEntryDegradeReason::NonColorCueMissing));
    let no_clean_chart_without_cue = !charts()
        .any(|ex| ex.is_clean() && (!ex.non_color_cue_present || !ex.legend_or_pattern_present));
    let no_clean_diff_without_cue = !diffs().any(|ex| ex.is_clean() && !ex.non_color_cue_present);
    if !(clean_chart_has_cue
        && clean_diff_has_cue
        && chart_color_alone_degrades
        && diff_cue_missing_degrades
        && no_clean_chart_without_cue
        && no_clean_diff_without_cue)
    {
        violations.push(M5SyntaxDiffChartRegistriesViolation::LegendOrPatternParityNotProven);
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

/// The three foundation families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5VisualFoundationFamily; 3] = [
    M5VisualFoundationFamily::SyntaxToken,
    M5VisualFoundationFamily::DiffToken,
    M5VisualFoundationFamily::ChartToken,
];
