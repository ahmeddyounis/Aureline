//! Implemented M5 typography-scale, font-stack, and text-overflow registries.
//!
//! The frozen [visual-foundation matrix][matrix] names Aureline's eight visual-foundation families and
//! locks their controlled vocabulary. The [color / theme registries lane][color] turned the two color
//! families into resolvers, and the [syntax / diff / chart registries lane][code] turned the three
//! code-and-data families into resolvers. This module is the next implement lane over that matrix: it
//! turns the **typography** family — the type scale, the code / UI font stacks, the line-height guards,
//! the tabular-numeral rule, and the sentence-case / default-text rule — plus the text-layout guard that
//! governs **overflow, truncation, and wrap behavior** into registry resolvers that produce export-safe,
//! honest projections, so titles, body, labels, and code read as one hierarchy across the shell, editor,
//! review, docs, and dense data consumers, so counts / timings / diagnostics use tabular numerals, and so
//! overflow never silently destroys meaning under zoom or density changes.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement the canonical type scale for titles, body, labels, and code, with stable UI and
//!   monospace stack selection and line-height guards.** [`resolve_type_scale_entry`] refuses to read as a
//!   clean type-scale entry unless it names a canonical token, names a resolved [text role][M5TextRole],
//!   selects a stable [font stack][M5FontStackSelection] that matches its role (code uses the monospace
//!   stack, UI text uses the sans stack), and guards its line-height against drift.
//! * **Require tabular numerals for counts / timings / diagnostics and consistent sentence-case /
//!   default-text rules where the source docs call for them.** A type-scale entry whose role is numeric
//!   data degrades to [`M5TypeScaleDegradeReason::TabularNumeralsMissing`] unless tabular numerals are
//!   enabled, and every entry states a [case rule][M5TextCaseRule] or degrades to
//!   [`M5TypeScaleDegradeReason::CaseRuleUnstated`].
//! * **Add overflow, truncation, and wrap behavior for tabs, rows, inspectors, banners, and code-adjacent
//!   metadata under zoom and density changes.** [`resolve_overflow_entry`] refuses to read as clean unless
//!   its [treatment][M5OverflowTreatment] preserves meaning (never a silent clip), the full meaning stays
//!   reachable off the truncation, and the entry survives both a zoom change and a density change.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualSemanticRole`] role
//! vocabulary and the [`M5TypographyRole`] typography-role vocabulary — so the shell, editor, review,
//! docs, data, and support surfaces can never fork their own type scale, font policy, or overflow
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_visual_foundation_matrix
//! [color]: crate::m5_color_system_and_semantic_theme_token_registries
//! [code]: crate::m5_syntax_diff_and_chart_token_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_typography_overflow_registries,
    seeded_m5_typography_overflow_registries_data_ui_preview_narrowed,
    seeded_m5_typography_overflow_registries_editor_ui_beta_narrowed,
    M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_visual_foundation_matrix::{
    M5TypographyRole, M5VisualFoundationAccessibilityRoute, M5VisualFoundationConsumerSurface,
    M5VisualFoundationDeploymentLine, M5VisualFoundationDowngradeTrigger, M5VisualFoundationFamily,
    M5VisualFoundationQualificationClass, M5VisualFoundationRequiredLabel, M5VisualSemanticRole,
    M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF, M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
    M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TypographyOverflowRegistriesPacket`].
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_typography_scale_font_stack_and_overflow_registries";

/// Schema version for M5 typography / overflow registry records.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-typography-scale-font-stack-and-overflow-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_typography_scale_font_stack_and_overflow_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-typography-scale-font-stack-and-overflow-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-typography-scale-font-stack-and-overflow-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-typography-scale-font-stack-and-overflow-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-typography-scale-font-stack-and-overflow-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5TypographyConsumerSurface = M5VisualFoundationConsumerSurface;

/// Controlled render context — which claimed M5 surface renders the registry entry, so the type scale and
/// overflow behavior stay stable whether they appear in the shell, editor, review, docs, or dense data
/// surface. Minted by this lane, tracking the first-consumer surfaces the goal names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextSurfaceContext {
    /// The shell surface (chrome, status, tabs).
    Shell,
    /// The editor surface (code and gutter text).
    Editor,
    /// The review surface (diff and annotation text).
    Review,
    /// The docs surface (rendered prose and code).
    Docs,
    /// The dense data surface (tables, counts, timings).
    Data,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5TextSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Docs,
        Self::Data,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the goal names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Shell,
        Self::Editor,
        Self::Review,
        Self::Docs,
        Self::Data,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Editor => "editor",
            Self::Review => "review",
            Self::Docs => "docs",
            Self::Data => "data",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled type-hierarchy role a type-scale entry names, so titles, body, labels, code, and numeric
/// data each map to a stable step of the canonical scale. Minted by this lane, tracking the type roles the
/// implementation requirement names by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextRole {
    /// A title / heading step.
    Title,
    /// A body / paragraph step.
    Body,
    /// A label / caption step.
    Label,
    /// A code / monospace step.
    Code,
    /// Numeric data (counts / timings / diagnostics) that demands tabular numerals.
    NumericData,
    /// The type role is unstated.
    RoleUnknown,
}

impl M5TextRole {
    /// Every text role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Title,
        Self::Body,
        Self::Label,
        Self::Code,
        Self::NumericData,
        Self::RoleUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Body => "body",
            Self::Label => "label",
            Self::Code => "code",
            Self::NumericData => "numeric_data",
            Self::RoleUnknown => "role_unknown",
        }
    }

    /// Whether the type role is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::RoleUnknown)
    }

    /// Whether the role demands tabular numerals (numeric data — counts / timings / diagnostics).
    pub const fn demands_tabular_numerals(self) -> bool {
        matches!(self, Self::NumericData)
    }

    /// Whether the role must select the monospace / code font stack.
    pub const fn requires_mono_stack(self) -> bool {
        matches!(self, Self::Code)
    }
}

/// Controlled font-stack selection a type-scale entry names, so the UI sans stack and the code monospace
/// stack stay stable and no surface forks a local font. Minted by this lane, tracking the code / UI font
/// policy the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FontStackSelection {
    /// The canonical UI / sans font stack.
    UiSansStack,
    /// The canonical code / monospace font stack.
    CodeMonoStack,
    /// A feature-local font stack, which is disallowed.
    LocalFontStackDisallowed,
    /// The font stack is unstated.
    StackUnknown,
}

impl M5FontStackSelection {
    /// Every font-stack selection, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UiSansStack,
        Self::CodeMonoStack,
        Self::LocalFontStackDisallowed,
        Self::StackUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiSansStack => "ui_sans_stack",
            Self::CodeMonoStack => "code_mono_stack",
            Self::LocalFontStackDisallowed => "local_font_stack_disallowed",
            Self::StackUnknown => "stack_unknown",
        }
    }

    /// Whether the selection names one of the two canonical stacks.
    pub const fn is_canonical(self) -> bool {
        matches!(self, Self::UiSansStack | Self::CodeMonoStack)
    }
}

/// Controlled text-case rule a type-scale entry declares, so sentence-case and default-text rules stay
/// consistent where the source docs call for them. Minted by this lane, tracking the case rules the
/// implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextCaseRule {
    /// Sentence case (the default for most UI text).
    SentenceCase,
    /// Default / source-preserving text (e.g. code, identifiers, user content).
    DefaultText,
    /// Title case (reserved for the small set of surfaces that call for it).
    TitleCase,
    /// The case rule is unstated.
    CaseRuleUnknown,
}

impl M5TextCaseRule {
    /// Every case rule, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SentenceCase,
        Self::DefaultText,
        Self::TitleCase,
        Self::CaseRuleUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SentenceCase => "sentence_case",
            Self::DefaultText => "default_text",
            Self::TitleCase => "title_case",
            Self::CaseRuleUnknown => "case_rule_unknown",
        }
    }

    /// Whether the case rule is stated (never the unknown sentinel).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::CaseRuleUnknown)
    }
}

/// Controlled surface element an overflow entry governs, so tabs, rows, inspectors, banners, and
/// code-adjacent metadata each declare their overflow behavior. Minted by this lane, tracking the surface
/// elements the implementation requirement names by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextSurfaceElement {
    /// A tab label.
    Tab,
    /// A table / list row.
    Row,
    /// An inspector / detail field.
    Inspector,
    /// A banner / inline notice.
    Banner,
    /// Code-adjacent metadata (path, ref, timing).
    CodeAdjacentMetadata,
    /// The surface element cannot currently be resolved.
    ElementUnknown,
}

impl M5TextSurfaceElement {
    /// Every surface element, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Tab,
        Self::Row,
        Self::Inspector,
        Self::Banner,
        Self::CodeAdjacentMetadata,
        Self::ElementUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tab => "tab",
            Self::Row => "row",
            Self::Inspector => "inspector",
            Self::Banner => "banner",
            Self::CodeAdjacentMetadata => "code_adjacent_metadata",
            Self::ElementUnknown => "element_unknown",
        }
    }

    /// Whether the surface element is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ElementUnknown)
    }
}

/// Controlled overflow treatment an overflow entry declares, so meaning is never silently clipped: text
/// truncates with a tooltip, wraps to the next line, ellipsizes with a reveal, or scrolls. Minted by this
/// lane, tracking the truncation / wrap behavior the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverflowTreatment {
    /// Truncate with a tooltip that carries the full text.
    TruncateWithTooltip,
    /// Wrap to the next line.
    WrapToNextLine,
    /// Ellipsize with a reveal affordance (expand / inspector).
    EllipsisWithReveal,
    /// Horizontal scroll that keeps the full text reachable.
    HorizontalScroll,
    /// A silent clip that destroys meaning, which is disallowed.
    SilentClipDisallowed,
    /// The overflow treatment is unstated.
    TreatmentUnknown,
}

impl M5OverflowTreatment {
    /// Every overflow treatment, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TruncateWithTooltip,
        Self::WrapToNextLine,
        Self::EllipsisWithReveal,
        Self::HorizontalScroll,
        Self::SilentClipDisallowed,
        Self::TreatmentUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TruncateWithTooltip => "truncate_with_tooltip",
            Self::WrapToNextLine => "wrap_to_next_line",
            Self::EllipsisWithReveal => "ellipsis_with_reveal",
            Self::HorizontalScroll => "horizontal_scroll",
            Self::SilentClipDisallowed => "silent_clip_disallowed",
            Self::TreatmentUnknown => "treatment_unknown",
        }
    }

    /// Whether the treatment preserves meaning (never a silent clip or the unstated sentinel).
    pub const fn preserves_meaning(self) -> bool {
        matches!(
            self,
            Self::TruncateWithTooltip
                | Self::WrapToNextLine
                | Self::EllipsisWithReveal
                | Self::HorizontalScroll
        )
    }

    /// Whether the treatment is the disallowed silent-clip token.
    pub const fn is_silent_clip(self) -> bool {
        matches!(self, Self::SilentClipDisallowed)
    }
}

/// Controlled density context an overflow entry is evaluated under, so overflow behavior stays honest as
/// the surface moves between comfortable, compact, and coarse-pointer densities. Minted by this lane,
/// tracking the density changes the implementation requirement calls for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DensityContext {
    /// Comfortable density.
    Comfortable,
    /// Compact density.
    Compact,
    /// Coarse-pointer density.
    CoarsePointer,
    /// The density context is unstated.
    DensityUnknown,
}

impl M5DensityContext {
    /// Every density context, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Comfortable,
        Self::Compact,
        Self::CoarsePointer,
        Self::DensityUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Comfortable => "comfortable",
            Self::Compact => "compact",
            Self::CoarsePointer => "coarse_pointer",
            Self::DensityUnknown => "density_unknown",
        }
    }

    /// Whether the density context is resolved (never the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::DensityUnknown)
    }
}

/// One mandatory rendered part a type-scale or overflow entry must be able to show, so no role, stack,
/// guard, numeral, case, element, or treatment fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The type-hierarchy role (type-scale entry).
    TextRole,
    /// The font-stack selection (type-scale entry).
    FontStack,
    /// The line-height guard (type-scale entry).
    LineHeightGuard,
    /// The tabular-numeral state (type-scale entry).
    TabularNumerals,
    /// The case rule (type-scale entry).
    CaseRule,
    /// The surface element (overflow entry).
    SurfaceElement,
    /// The overflow treatment (overflow entry).
    OverflowTreatment,
    /// The render / surface context.
    SurfaceContext,
}

impl M5TextAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::TextRole,
        Self::FontStack,
        Self::LineHeightGuard,
        Self::TabularNumerals,
        Self::CaseRule,
        Self::SurfaceElement,
        Self::OverflowTreatment,
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
            Self::TextRole => "text_role",
            Self::FontStack => "font_stack",
            Self::LineHeightGuard => "line_height_guard",
            Self::TabularNumerals => "tabular_numerals",
            Self::CaseRule => "case_rule",
            Self::SurfaceElement => "surface_element",
            Self::OverflowTreatment => "overflow_treatment",
            Self::SurfaceContext => "surface_context",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect the type
/// scale, adjust overflow behavior, trace a token, verify zoom / density, or review a degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextNextAction {
    /// Inspect the canonical type scale, font stack, and line-height guard.
    InspectTypeScale,
    /// Adjust the overflow / truncation / wrap behavior so meaning survives.
    AdjustOverflowBehavior,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Verify the entry survives zoom and density changes.
    VerifyZoomDensity,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5TextNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectTypeScale,
        Self::AdjustOverflowBehavior,
        Self::TraceCanonicalToken,
        Self::VerifyZoomDensity,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectTypeScale => "inspect_type_scale",
            Self::AdjustOverflowBehavior => "adjust_overflow_behavior",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::VerifyZoomDensity => "verify_zoom_density",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TextExportField {
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
    /// The type-hierarchy roles named.
    TextRoles,
    /// The font stacks selected.
    FontStacks,
    /// The case rules stated.
    CaseRules,
    /// The surface elements governed.
    SurfaceElements,
    /// The overflow treatments declared.
    OverflowTreatments,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TextExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::FoundationFamilies,
        Self::SemanticRoles,
        Self::DegradeReasons,
        Self::Qualification,
        Self::TextRoles,
        Self::FontStacks,
        Self::CaseRules,
        Self::SurfaceElements,
        Self::OverflowTreatments,
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
            Self::TextRoles => "text_roles",
            Self::FontStacks => "font_stacks",
            Self::CaseRules => "case_rules",
            Self::SurfaceElements => "surface_elements",
            Self::OverflowTreatments => "overflow_treatments",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a type-scale entry degraded below a clean state. The degrade-first ladder returns one of these
/// instead of ever letting a font-forked, drifting, numeral-missing, or raw-value entry read as clean.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TypeScaleDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The type-hierarchy role is unstated.
    TypeRoleUnstated,
    /// The font stack is unstable or does not match the role (code must use the monospace stack).
    FontStackUnstable,
    /// The line-height is not guarded and may drift.
    LineHeightDrifted,
    /// Tabular numerals are missing for numeric data (counts / timings / diagnostics).
    TabularNumeralsMissing,
    /// The sentence-case / default-text rule is unstated.
    CaseRuleUnstated,
    /// A raw type value is inlined instead of tracing to a canonical token.
    RawTypeValueInlined,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TypeScaleDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::TypeRoleUnstated,
        Self::FontStackUnstable,
        Self::LineHeightDrifted,
        Self::TabularNumeralsMissing,
        Self::CaseRuleUnstated,
        Self::RawTypeValueInlined,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::TypeRoleUnstated => "type_role_unstated",
            Self::FontStackUnstable => "font_stack_unstable",
            Self::LineHeightDrifted => "line_height_drifted",
            Self::TabularNumeralsMissing => "tabular_numerals_missing",
            Self::CaseRuleUnstated => "case_rule_unstated",
            Self::RawTypeValueInlined => "raw_type_value_inlined",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TextNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawTypeValueInlined => {
                M5TextNextAction::TraceCanonicalToken
            }
            Self::TypeRoleUnstated
            | Self::FontStackUnstable
            | Self::LineHeightDrifted
            | Self::TabularNumeralsMissing
            | Self::CaseRuleUnstated => M5TextNextAction::InspectTypeScale,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5TextNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::TokenNameUnstated | Self::RawTypeValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::FontStackUnstable => M5VisualFoundationDowngradeTrigger::FontStackUnstable,
            Self::LineHeightDrifted => M5VisualFoundationDowngradeTrigger::TypographyScaleDrifted,
            Self::TabularNumeralsMissing => {
                M5VisualFoundationDowngradeTrigger::TabularNumeralsMissing
            }
            Self::TypeRoleUnstated | Self::CaseRuleUnstated | Self::SurfaceContextUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an overflow entry degraded below a clean state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverflowDegradeReason {
    /// The canonical token / identity is unstated.
    IdentityUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The surface element cannot currently be resolved.
    SurfaceElementUnresolved,
    /// The overflow treatment silently clips and destroys meaning.
    MeaningSilentlyDestroyed,
    /// The full meaning is not reachable off the truncation.
    FullMeaningUnreachable,
    /// The behavior regresses under a zoom change (does not reflow legibly).
    ZoomRegression,
    /// The behavior regresses under a density change.
    DensityRegression,
    /// A raw layout value is inlined instead of tracing to a canonical token.
    RawLayoutValueInlined,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OverflowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::IdentityUnstated,
        Self::SurfaceContextUnresolved,
        Self::SurfaceElementUnresolved,
        Self::MeaningSilentlyDestroyed,
        Self::FullMeaningUnreachable,
        Self::ZoomRegression,
        Self::DensityRegression,
        Self::RawLayoutValueInlined,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityUnstated => "identity_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SurfaceElementUnresolved => "surface_element_unresolved",
            Self::MeaningSilentlyDestroyed => "meaning_silently_destroyed",
            Self::FullMeaningUnreachable => "full_meaning_unreachable",
            Self::ZoomRegression => "zoom_regression",
            Self::DensityRegression => "density_regression",
            Self::RawLayoutValueInlined => "raw_layout_value_inlined",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TextNextAction {
        match self {
            Self::IdentityUnstated | Self::RawLayoutValueInlined => {
                M5TextNextAction::TraceCanonicalToken
            }
            Self::MeaningSilentlyDestroyed | Self::FullMeaningUnreachable => {
                M5TextNextAction::AdjustOverflowBehavior
            }
            Self::ZoomRegression | Self::DensityRegression => M5TextNextAction::VerifyZoomDensity,
            Self::SurfaceContextUnresolved | Self::SurfaceElementUnresolved | Self::ProofStale => {
                M5TextNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualFoundationDowngradeTrigger {
        match self {
            Self::IdentityUnstated | Self::RawLayoutValueInlined => {
                M5VisualFoundationDowngradeTrigger::TokenReferenceUnstated
            }
            Self::MeaningSilentlyDestroyed
            | Self::FullMeaningUnreachable
            | Self::ZoomRegression => M5VisualFoundationDowngradeTrigger::TypographyScaleDrifted,
            Self::DensityRegression => {
                M5VisualFoundationDowngradeTrigger::LocalGeometryForkedFromFoundation
            }
            Self::SurfaceContextUnresolved | Self::SurfaceElementUnresolved => {
                M5VisualFoundationDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualFoundationDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_type_scale_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TypeScaleEntryResolutionInput {
    /// Stable identity of the type-scale entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `type.title`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The typography role (from the frozen matrix vocabulary).
    pub typography_role: M5TypographyRole,
    /// The type-hierarchy role the entry names.
    pub text_role: M5TextRole,
    /// The font-stack selection the entry names.
    pub font_stack: M5FontStackSelection,
    /// The case rule the entry declares.
    pub case_rule: M5TextCaseRule,
    /// The render / surface context.
    pub surface_context: M5TextSurfaceContext,
    /// True when the line-height is guarded against drift.
    pub line_height_guarded: bool,
    /// True when tabular numerals are enabled for numeric data.
    pub tabular_numerals_enabled: bool,
    /// True when the entry traces to a canonical token (never an inlined raw value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe type-scale projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTypeScaleEntry {
    /// Stable identity of the type-scale entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The typography-role token named by the entry.
    pub typography_role: String,
    /// The type-hierarchy-role token named by the entry.
    pub text_role: String,
    /// Whether the type role demands tabular numerals (numeric data).
    pub text_role_demands_tabular_numerals: bool,
    /// The font-stack token named by the entry.
    pub font_stack: String,
    /// Whether the font stack is stable and matches the role.
    pub font_stack_stable: bool,
    /// The case-rule token named by the entry.
    pub case_rule: String,
    /// Whether the case rule is stated.
    pub case_rule_stated: bool,
    /// Whether the line-height is guarded against drift.
    pub line_height_guarded: bool,
    /// Whether tabular numerals are enabled.
    pub tabular_numerals_enabled: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5TypeScaleDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TextNextAction,
    /// Whether the type hierarchy reads as readable for a clean entry naming every fact.
    pub type_hierarchy_is_readable: bool,
}

impl M5ResolvedTypeScaleEntry {
    /// Whether this type-scale entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_overflow_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OverflowEntryResolutionInput {
    /// Stable identity of the overflow entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualSemanticRole,
    /// The surface element the entry governs.
    pub surface_element: M5TextSurfaceElement,
    /// The overflow treatment the entry declares.
    pub overflow_treatment: M5OverflowTreatment,
    /// The density context the entry is evaluated under.
    pub density_context: M5DensityContext,
    /// The render / surface context.
    pub surface_context: M5TextSurfaceContext,
    /// True when the full meaning stays reachable off the truncation (tooltip / reveal / wrap / export).
    pub full_meaning_reachable: bool,
    /// True when the behavior survives a zoom change (reflows legibly).
    pub survives_zoom: bool,
    /// True when the behavior survives a density change.
    pub survives_density: bool,
    /// True when the entry traces to a canonical token (never an inlined raw value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe overflow projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOverflowEntry {
    /// Stable identity of the overflow entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The surface-element token named by the entry.
    pub surface_element: String,
    /// Whether the surface element is resolved.
    pub surface_element_resolved: bool,
    /// The overflow-treatment token named by the entry.
    pub overflow_treatment: String,
    /// Whether the treatment preserves meaning (never a silent clip).
    pub overflow_preserves_meaning: bool,
    /// Whether the full meaning stays reachable off the truncation.
    pub full_meaning_reachable: bool,
    /// The density-context token named by the entry.
    pub density_context: String,
    /// Whether the behavior survives a zoom change.
    pub survives_zoom: bool,
    /// Whether the behavior survives a density change.
    pub survives_density: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean state.
    pub degrade_reason: Option<M5OverflowDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TextNextAction,
    /// Whether the meaning survives zoom and density for a clean entry naming every fact.
    pub meaning_survives_zoom_and_density: bool,
}

impl M5ResolvedOverflowEntry {
    /// Whether this overflow entry reads as a clean state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TypographyOverflowResolutionError {
    /// The type-scale-entry id was empty.
    EmptyTypeScaleEntryId,
    /// The overflow-entry id was empty.
    EmptyOverflowEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TypographyOverflowResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyTypeScaleEntryId => "empty_type_scale_entry_id",
            Self::EmptyOverflowEntryId => "empty_overflow_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TypographyOverflowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 typography / overflow registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TypographyOverflowResolutionError {}

fn font_stack_matches_role(role: M5TextRole, stack: M5FontStackSelection) -> bool {
    if role.requires_mono_stack() {
        matches!(stack, M5FontStackSelection::CodeMonoStack)
    } else {
        matches!(stack, M5FontStackSelection::UiSansStack)
    }
}

/// Resolves a type-scale entry so titles, body, labels, and code read as one hierarchy: the entry names
/// its canonical token, names a resolved role, selects a stable font stack matching its role, guards its
/// line-height, pairs tabular numerals with numeric data, and states its case rule.
pub fn resolve_type_scale_entry(
    input: M5TypeScaleEntryResolutionInput,
) -> Result<M5ResolvedTypeScaleEntry, M5TypographyOverflowResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5TypographyOverflowResolutionError::EmptyTypeScaleEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5TypographyOverflowResolutionError::ForbiddenMaterial);
    }

    let font_stack_stable = input.font_stack.is_canonical()
        && font_stack_matches_role(input.text_role, input.font_stack);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5TypeScaleDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5TypeScaleDegradeReason::SurfaceContextUnresolved)
    } else if !input.text_role.is_resolved() {
        Some(M5TypeScaleDegradeReason::TypeRoleUnstated)
    } else if !font_stack_stable {
        Some(M5TypeScaleDegradeReason::FontStackUnstable)
    } else if !input.line_height_guarded {
        Some(M5TypeScaleDegradeReason::LineHeightDrifted)
    } else if input.text_role.demands_tabular_numerals() && !input.tabular_numerals_enabled {
        Some(M5TypeScaleDegradeReason::TabularNumeralsMissing)
    } else if !input.case_rule.is_stated() {
        Some(M5TypeScaleDegradeReason::CaseRuleUnstated)
    } else if !input.references_canonical_token {
        Some(M5TypeScaleDegradeReason::RawTypeValueInlined)
    } else if !input.proof_fresh {
        Some(M5TypeScaleDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TextNextAction::InspectTypeScale,
    };

    Ok(M5ResolvedTypeScaleEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        typography_role: input.typography_role.as_str().to_owned(),
        text_role: input.text_role.as_str().to_owned(),
        text_role_demands_tabular_numerals: input.text_role.demands_tabular_numerals(),
        font_stack: input.font_stack.as_str().to_owned(),
        font_stack_stable,
        case_rule: input.case_rule.as_str().to_owned(),
        case_rule_stated: input.case_rule.is_stated(),
        line_height_guarded: input.line_height_guarded,
        tabular_numerals_enabled: input.tabular_numerals_enabled,
        surface_context: input.surface_context.as_str().to_owned(),
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        type_hierarchy_is_readable: degrade_reason.is_none(),
    })
}

/// Resolves an overflow entry so meaning is never silently destroyed under zoom or density: the entry
/// names its canonical token and surface element, declares a treatment that preserves meaning, keeps the
/// full meaning reachable off the truncation, and survives both a zoom change and a density change.
pub fn resolve_overflow_entry(
    input: M5OverflowEntryResolutionInput,
) -> Result<M5ResolvedOverflowEntry, M5TypographyOverflowResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5TypographyOverflowResolutionError::EmptyOverflowEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5TypographyOverflowResolutionError::ForbiddenMaterial);
    }

    let preserves_meaning = input.overflow_treatment.preserves_meaning();

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5OverflowDegradeReason::IdentityUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5OverflowDegradeReason::SurfaceContextUnresolved)
    } else if !input.surface_element.is_resolved() {
        Some(M5OverflowDegradeReason::SurfaceElementUnresolved)
    } else if input.overflow_treatment.is_silent_clip() || !preserves_meaning {
        Some(M5OverflowDegradeReason::MeaningSilentlyDestroyed)
    } else if !input.full_meaning_reachable {
        Some(M5OverflowDegradeReason::FullMeaningUnreachable)
    } else if !input.survives_zoom {
        Some(M5OverflowDegradeReason::ZoomRegression)
    } else if !input.survives_density {
        Some(M5OverflowDegradeReason::DensityRegression)
    } else if !input.references_canonical_token {
        Some(M5OverflowDegradeReason::RawLayoutValueInlined)
    } else if !input.proof_fresh {
        Some(M5OverflowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TextNextAction::AdjustOverflowBehavior,
    };

    Ok(M5ResolvedOverflowEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        surface_element: input.surface_element.as_str().to_owned(),
        surface_element_resolved: input.surface_element.is_resolved(),
        overflow_treatment: input.overflow_treatment.as_str().to_owned(),
        overflow_preserves_meaning: preserves_meaning,
        full_meaning_reachable: input.full_meaning_reachable,
        density_context: input.density_context.as_str().to_owned(),
        survives_zoom: input.survives_zoom,
        survives_density: input.survives_density,
        surface_context: input.surface_context.as_str().to_owned(),
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        meaning_survives_zoom_and_density: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved type-scale and overflow entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5TypographyConsumerSurface,
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
    pub anatomy_parts: Vec<M5TextAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TextExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualFoundationDowngradeTrigger>,
    /// Resolved type-scale examples.
    pub type_scale_entries: Vec<M5ResolvedTypeScaleEntry>,
    /// Resolved overflow examples.
    pub overflow_entries: Vec<M5ResolvedOverflowEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical typography / geometry domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: the type scale or a font stack never drifts from the shared foundation. MUST be
    /// `false`.
    pub typography_scale_or_font_stack_drifted: bool,
    /// Hard invariant: overflow never silently destroys meaning. MUST be `false`.
    pub overflow_silently_destroyed_meaning: bool,
    /// Hard invariant: a zoom / density regression never ships unnoticed. MUST be `false`.
    pub zoom_or_density_regression_uncaught: bool,
    /// Hard invariant: a raw type value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_type_value_inlined_instead_of_token: bool,
}

impl M5TypographyOverflowRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TextAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5TextAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TextExportField> = self.export_fields.iter().copied().collect();
        M5TextExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.typography_scale_or_font_stack_drifted
            && !self.overflow_silently_destroyed_meaning
            && !self.zoom_or_density_regression_uncaught
            && !self.raw_type_value_inlined_instead_of_token
    }

    fn has_any_entry(&self) -> bool {
        !self.type_scale_entries.is_empty() || !self.overflow_entries.is_empty()
    }

    /// True when a clean type-scale entry preserves a readable hierarchy: it traces to a canonical token,
    /// selects a stable font stack, guards line-height, pairs tabular numerals with numeric data, and
    /// states its case rule.
    fn type_scale_is_honest(ex: &M5ResolvedTypeScaleEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.font_stack_stable
                && ex.line_height_guarded
                && (!ex.text_role_demands_tabular_numerals || ex.tabular_numerals_enabled)
                && ex.case_rule_stated)
    }

    /// True when a clean overflow entry preserves meaning: it traces to a canonical token, declares a
    /// meaning-preserving treatment, keeps the full meaning reachable, and survives zoom and density.
    fn overflow_is_honest(ex: &M5ResolvedOverflowEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.overflow_preserves_meaning
                && ex.full_meaning_reachable
                && ex.survives_zoom
                && ex.survives_density)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.type_scale_entries
            .iter()
            .all(Self::type_scale_is_honest)
            && self.overflow_entries.iter().all(Self::overflow_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Typography-role tokens (bound from the frozen matrix).
    pub typography_roles: Vec<String>,
    /// Type-hierarchy-role tokens (minted by this lane).
    pub text_roles: Vec<String>,
    /// Font-stack-selection tokens (minted by this lane).
    pub font_stacks: Vec<String>,
    /// Case-rule tokens (minted by this lane).
    pub case_rules: Vec<String>,
    /// Surface-element tokens (minted by this lane).
    pub surface_elements: Vec<String>,
    /// Overflow-treatment tokens (minted by this lane).
    pub overflow_treatments: Vec<String>,
    /// Density-context tokens (minted by this lane).
    pub density_contexts: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Type-scale-entry degrade-reason tokens.
    pub type_scale_degrade_reasons: Vec<String>,
    /// Overflow-entry degrade-reason tokens.
    pub overflow_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TypographyOverflowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualSemanticRole::ALL, |v| v.as_str()),
            typography_roles: tokens(&M5TypographyRole::ALL, |v| v.as_str()),
            text_roles: tokens(&M5TextRole::ALL, |v| v.as_str()),
            font_stacks: tokens(&M5FontStackSelection::ALL, |v| v.as_str()),
            case_rules: tokens(&M5TextCaseRule::ALL, |v| v.as_str()),
            surface_elements: tokens(&M5TextSurfaceElement::ALL, |v| v.as_str()),
            overflow_treatments: tokens(&M5OverflowTreatment::ALL, |v| v.as_str()),
            density_contexts: tokens(&M5DensityContext::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5TextSurfaceContext::ALL, |v| v.as_str()),
            type_scale_degrade_reasons: tokens(&M5TypeScaleDegradeReason::ALL, |v| v.as_str()),
            overflow_degrade_reasons: tokens(&M5OverflowDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TextAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TextNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TextExportField::ALL, |v| v.as_str()),
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
pub struct M5TypographyOverflowGovernanceReview {
    /// The shell, editor, review, docs, and data surfaces share one readable type hierarchy.
    pub one_readable_type_hierarchy_across_surfaces: bool,
    /// The code / UI font policy is stable (sans for UI text, monospace for code).
    pub code_and_ui_font_policy_is_stable: bool,
    /// Line-height guards hold and never drift.
    pub line_height_guards_hold: bool,
    /// Tabular numerals are present for counts / timings / diagnostics.
    pub tabular_numerals_present_for_numeric_data: bool,
    /// Overflow never silently destroys meaning.
    pub overflow_never_silently_destroys_meaning: bool,
    /// The full meaning stays reachable off truncation.
    pub full_meaning_reachable_off_truncation: bool,
    /// Zoom and density regressions are caught by fixtures before release evidence turns green.
    pub zoom_and_density_regressions_caught_before_release: bool,
    /// Raw-type-value drift is caught before release evidence turns green.
    pub raw_type_value_drift_caught_before_release: bool,
    /// The first shell / editor / review / docs / data consumers use the canonical type scale.
    pub first_consumers_use_canonical_type_scale: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowConsumerProjection {
    /// The shell and editor surfaces consume the shared type scale.
    pub shell_and_editor_consume_shared_type_scale: bool,
    /// The review surface consumes the shared type scale.
    pub review_consumes_shared_type_scale: bool,
    /// The data surface consumes the tabular-numeral policy.
    pub data_consumes_tabular_numeral_policy: bool,
    /// The docs surface consumes the shared type scale.
    pub docs_consumes_shared_type_scale: bool,
    /// Type and layout meaning trace back to one canonical typography / geometry domain contract.
    pub type_and_layout_meaning_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical typography source.
    pub support_export_reads_single_typography_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-foundation audit for the lane.
    pub foundation_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TypographyOverflowRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TypographyOverflowRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5TypographyOverflowRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TypographyOverflowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TypographyOverflowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TypographyOverflowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TypographyOverflowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TypographyOverflowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 typography-scale, font-stack, and text-overflow registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TypographyOverflowRegistriesPacket {
    /// Record kind; must equal [`M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5TypographyOverflowRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TypographyOverflowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TypographyOverflowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TypographyOverflowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TypographyOverflowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TypographyOverflowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TypographyOverflowRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5TypographyOverflowRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5TypographyOverflowRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_RECORD_KIND {
            violations.push(M5TypographyOverflowRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5TypographyOverflowRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TypographyOverflowRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TypographyOverflowRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 typography / overflow registries packet serializes"),
        ) {
            violations.push(M5TypographyOverflowRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 typography / overflow registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,type_scale_entries,overflow_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .type_scale_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.overflow_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.type_scale_entries.len(),
                row.overflow_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Typography-Scale, Font-Stack, and Text-Overflow Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Text roles: {}\n",
            self.vocabulary_set.text_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Font stacks: {}\n",
            self.vocabulary_set.font_stacks.join(", ")
        ));
        out.push_str(&format!(
            "- Overflow treatments: {}\n",
            self.vocabulary_set.overflow_treatments.join(", ")
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
                "  - Type scale: {} / overflow: {}\n",
                row.type_scale_entries.len(),
                row.overflow_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5TypographyOverflowRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TypographyOverflowRegistriesViolation>),
}

impl fmt::Display for M5TypographyOverflowRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 typography / overflow registries export parse failed: {error}"
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
                    "m5 typography / overflow registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TypographyOverflowRegistriesArtifactError {}

/// Validation failures emitted by [`M5TypographyOverflowRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TypographyOverflowRegistriesViolation {
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
    /// A registry row carries a dishonest clean example (font drift, silent clip, zoom / density
    /// regression, or a raw-value inlining that still reads as clean).
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
    /// One readable type hierarchy is not proven: clean type-scale entries do not cover the title / body /
    /// label / code roles and the UI-sans / code-mono font policy across the first shell / editor / review
    /// / docs / data surfaces, no raw-type example degrades, or a clean entry inlines a raw value.
    SharedTypeHierarchyNotProven,
    /// Tabular numerals or overflow safety is not proven: no clean numeric entry enables tabular numerals,
    /// no tabular-missing example degrades, no clean overflow entry preserves meaning, or no
    /// meaning-destroyed example degrades.
    TabularNumeralsOrOverflowSafetyNotProven,
    /// Zoom / density regressions are not caught: no clean overflow entry survives zoom and density, no
    /// zoom regression degrades, no density regression degrades, or no line-height guard is exercised.
    ZoomDensityRegressionNotCaught,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TypographyOverflowRegistriesViolation {
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
            Self::SharedTypeHierarchyNotProven => "shared_type_hierarchy_not_proven",
            Self::TabularNumeralsOrOverflowSafetyNotProven => {
                "tabular_numerals_or_overflow_safety_not_proven"
            }
            Self::ZoomDensityRegressionNotCaught => "zoom_density_regression_not_caught",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_typography_overflow_registries_export(
) -> Result<M5TypographyOverflowRegistriesPacket, M5TypographyOverflowRegistriesArtifactError> {
    let packet: M5TypographyOverflowRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-typography-scale-font-stack-and-overflow-registries-proof/support_export.json"
    )))
    .map_err(M5TypographyOverflowRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TypographyOverflowRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_SCHEMA_REF,
        M5_TYPOGRAPHY_OVERFLOW_REGISTRIES_DOC_REF,
        M5_VISUAL_FOUNDATION_MATRIX_SCHEMA_REF,
        M5_VISUAL_FOUNDATION_MATRIX_DOC_REF,
        M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TypographyOverflowRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5TypographyOverflowRegistriesViolation::NoRegistryRows);
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
            violations.push(M5TypographyOverflowRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TypographyOverflowRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TypographyOverflowRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TYPOGRAPHY_AND_GEOMETRY_SCHEMA_REF) {
            violations.push(M5TypographyOverflowRegistriesViolation::DomainSchemaRefMissing);
        }
        if !row.has_any_entry() {
            violations.push(M5TypographyOverflowRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TypographyOverflowRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TypographyOverflowRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_readable_type_hierarchy_across_surfaces,
        review.code_and_ui_font_policy_is_stable,
        review.line_height_guards_hold,
        review.tabular_numerals_present_for_numeric_data,
        review.overflow_never_silently_destroys_meaning,
        review.full_meaning_reachable_off_truncation,
        review.zoom_and_density_regressions_caught_before_release,
        review.raw_type_value_drift_caught_before_release,
        review.first_consumers_use_canonical_type_scale,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TypographyOverflowRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_editor_consume_shared_type_scale,
        projection.review_consumes_shared_type_scale,
        projection.data_consumes_tabular_numeral_policy,
        projection.docs_consumes_shared_type_scale,
        projection.type_and_layout_meaning_traces_to_single_domain_contract,
        projection.support_export_reads_single_typography_source,
    ] {
        if !ok {
            violations.push(M5TypographyOverflowRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TypographyOverflowRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.foundation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TypographyOverflowRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TypographyOverflowRegistriesPacket,
    violations: &mut Vec<M5TypographyOverflowRegistriesViolation>,
) {
    let types = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.type_scale_entries.iter())
    };
    let overflows = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.overflow_entries.iter())
    };

    // AC1: the first claimed M5 consumers share one readable type hierarchy and code / UI font policy.
    // Clean type-scale entries cover the title / body / label / code roles and both canonical font stacks
    // across the first shell / editor / review / docs / data surfaces, a raw-type example degrades, and no
    // clean entry inlines a raw value.
    let clean_roles: BTreeSet<String> = types()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.text_role.clone())
        .collect();
    let hierarchy_covered = ["title", "body", "label", "code"]
        .iter()
        .all(|r| clean_roles.contains(*r));
    let clean_stacks: BTreeSet<String> = types()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.font_stack.clone())
        .collect();
    let font_policy_covered =
        clean_stacks.contains("ui_sans_stack") && clean_stacks.contains("code_mono_stack");
    let clean_surfaces: BTreeSet<String> = types()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .chain(
            overflows()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .collect();
    let first_surfaces_covered = M5TextSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_type_degrades =
        types().any(|ex| ex.degrade_reason == Some(M5TypeScaleDegradeReason::RawTypeValueInlined));
    let no_clean_raw = !types().any(|ex| ex.is_clean() && !ex.references_canonical_token)
        && !overflows().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(hierarchy_covered
        && font_policy_covered
        && first_surfaces_covered
        && raw_type_degrades
        && no_clean_raw)
    {
        violations.push(M5TypographyOverflowRegistriesViolation::SharedTypeHierarchyNotProven);
    }

    // AC2: counts / timings / dense tables use tabular numerals where required, and overflow does not
    // silently destroy meaning. A clean numeric entry enables tabular numerals, a tabular-missing example
    // degrades, a clean overflow entry preserves meaning, a meaning-destroyed example degrades, and no
    // clean overflow entry silently destroys meaning.
    let clean_numeric_has_tabular = types().any(|ex| {
        ex.is_clean() && ex.text_role_demands_tabular_numerals && ex.tabular_numerals_enabled
    });
    let tabular_missing_degrades = types()
        .any(|ex| ex.degrade_reason == Some(M5TypeScaleDegradeReason::TabularNumeralsMissing));
    let clean_overflow_preserves =
        overflows().any(|ex| ex.is_clean() && ex.overflow_preserves_meaning);
    let meaning_destroyed_degrades = overflows()
        .any(|ex| ex.degrade_reason == Some(M5OverflowDegradeReason::MeaningSilentlyDestroyed));
    let no_clean_overflow_destroys =
        !overflows().any(|ex| ex.is_clean() && !ex.overflow_preserves_meaning);
    if !(clean_numeric_has_tabular
        && tabular_missing_degrades
        && clean_overflow_preserves
        && meaning_destroyed_degrades
        && no_clean_overflow_destroys)
    {
        violations.push(
            M5TypographyOverflowRegistriesViolation::TabularNumeralsOrOverflowSafetyNotProven,
        );
    }

    // AC3: zoom / density regressions in typography are caught before release evidence turns green. A
    // clean overflow entry survives zoom and density, a zoom regression degrades, a density regression
    // degrades, a clean type-scale entry guards line-height while a line-height-drift example degrades,
    // and no clean overflow entry fails zoom / density.
    let clean_overflow_survives =
        overflows().any(|ex| ex.is_clean() && ex.survives_zoom && ex.survives_density);
    let zoom_regression_degrades =
        overflows().any(|ex| ex.degrade_reason == Some(M5OverflowDegradeReason::ZoomRegression));
    let density_regression_degrades =
        overflows().any(|ex| ex.degrade_reason == Some(M5OverflowDegradeReason::DensityRegression));
    let clean_line_height_guarded = types().any(|ex| ex.is_clean() && ex.line_height_guarded);
    let line_height_drift_degrades =
        types().any(|ex| ex.degrade_reason == Some(M5TypeScaleDegradeReason::LineHeightDrifted));
    let no_clean_overflow_fails_zoom_density =
        !overflows().any(|ex| ex.is_clean() && (!ex.survives_zoom || !ex.survives_density));
    if !(clean_overflow_survives
        && zoom_regression_degrades
        && density_regression_degrades
        && clean_line_height_guarded
        && line_height_drift_degrades
        && no_clean_overflow_fails_zoom_density)
    {
        violations.push(M5TypographyOverflowRegistriesViolation::ZoomDensityRegressionNotCaught);
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

/// The one foundation family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5VisualFoundationFamily; 1] =
    [M5VisualFoundationFamily::Typography];
