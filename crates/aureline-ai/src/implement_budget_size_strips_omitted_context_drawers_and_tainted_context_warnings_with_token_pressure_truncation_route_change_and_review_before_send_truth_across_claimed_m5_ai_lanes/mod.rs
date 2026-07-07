//! Two reusable M5 prompt-composer primitives — the budget-or-size strip (with its
//! omitted-context drawer) and the tainted-context warning — so pre-send composition is
//! honest about what will be sent, what was left out, whether the route changed, and whether
//! any pasted or promoted untrusted text is being treated as data instead of instruction.
//!
//! Aureline's frozen prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! names the budget-or-size strip and the tainted-context warning as two governed component
//! families and freezes their controlled vocabulary — the budget postures, the
//! omitted-context reasons, the taint sources, and the taint severities, plus the surface
//! families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, and downgrade triggers. This module *implements* those two contracts as reusable
//! primitives so a user can tell — from the budget strip or the warning alone — which context
//! classes are included versus omitted, why context was truncated or withheld, how much token
//! or size pressure the request is under, whether the route changed and what that means, and,
//! for any untrusted input, its source, its severity, whether it is treated as data, and the
//! review path that must run before a side-effectful AI route.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_budget_size_strip`] — takes one strip's included context classes, its
//!    omitted-context drawer entries (each an omitted class with its reason and detail), the
//!    budget-pressure signals, and the before / after route class, and produces one
//!    [`M5ResolvedBudgetSizeStrip`] carrying the derived [`M5BudgetPosture`] (within-budget
//!    versus near-limit versus over-budget versus truncation-pending versus hard-blocked
//!    versus unmetered-local), the derived [`M5BudgetPressureBand`], the preserved
//!    omitted-context drawer, the derived [`M5RouteSwitchConsequence`], the bounded
//!    inspect / adjust / review / reduce / proceed actions, whether the request is sendable,
//!    and whether omissions, truncation, or a route change require review before send. It
//!    never shows an over-budget or hard-blocked request as within budget and never drops
//!    context without naming the reason.
//! 2. [`resolve_tainted_context_warning`] — takes one untrusted input's taint source and
//!    severity, whether it is treated as data, whether the pending route is side-effecting,
//!    whether it has been acknowledged, and its quarantine note, and produces one
//!    [`M5ResolvedTaintedContextWarning`] carrying the derived [`M5TaintWarningPosture`]
//!    (no-taint-trusted versus flagged-as-data versus elevated-review-required versus
//!    quarantine-held versus injection-blocked versus acknowledged-proceedable), the bounded
//!    review / quarantine / remove / acknowledge / proceed actions, whether the warning blocks
//!    send, whether it needs review before send, and whether the review path is preserved. It
//!    never downplays an injection-suspected or quarantine-required taint and never lets an
//!    untrusted input read as trusted instruction.
//!
//! A single parity matrix — [`M5BudgetTaintPacket`] — binds one row per claimed M5 composer
//! consumer that can send an AI request (the inline composer, the side panel, the patch draft,
//! the CLI / headless surface, and the support export) to the shared budget and taint anatomy,
//! the same budget postures, omitted-context reasons, taint sources, taint severities, warning
//! postures, bounded actions, export fields, and non-visual accessibility routes, so the
//! budget and taint grammar stays identical across every send-capable surface rather than
//! drifting into a separate AI-only grammar.
//!
//! The budget posture ([`M5BudgetPosture`]), omitted-context reason
//! ([`M5OmittedContextReason`]), taint source ([`M5TaintSource`]), taint severity
//! ([`M5TaintSeverity`]), route class ([`M5ComposerRouteClass`]), surface family
//! ([`M5ComposerSurfaceFamily`]), deployment line ([`M5ComposerDeploymentLine`]), consumer
//! surface ([`M5ComposerConsumerSurface`]), accessibility route
//! ([`M5ComposerAccessibilityRoute`]), qualification class ([`M5ComposerQualificationClass`]),
//! and downgrade trigger ([`M5ComposerDowngradeTrigger`]) are reused verbatim from the frozen
//! matrix. This module mints new vocabulary only for what that matrix left implicit about the
//! budget strip and the tainted-context warning themselves: their send-capable consumers, the
//! context classes, the pressure bands, the route-switch consequences, the warning postures,
//! their anatomy parts, their bounded actions, and their export fields. No M5 composer surface
//! invents a second budget or taint grammar.
//!
//! Raw prompts, pasted bodies, tool-output bodies, raw paths, raw URLs, credentials, and
//! private endpoints stay outside the support boundary; every strip id, warning id, context
//! label, omitted-context detail, and quarantine note is carried only as an opaque,
//! export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-budget-size-strip-and-tainted-context-warning.schema.json`](../../../../schemas/ai/m5-budget-size-strip-and-tainted-context-warning.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes.md`](../../../../docs/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_budget_taint_cli_headless_beta_narrowed, seeded_m5_budget_taint_packet,
    seeded_m5_budget_taint_patch_draft_preview_narrowed, M5_BUDGET_TAINT_PACKET_ID,
};

// The budget posture, omitted-context reason, taint source, taint severity, route class,
// surface family, deployment line, consumer surface, accessibility route, qualification
// class, and downgrade triggers are frozen once, in the prompt-composer component matrix.
// These primitives reuse them verbatim so they never invent a parallel budget / taint
// vocabulary.
pub use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5BudgetPosture, M5ComposerAccessibilityRoute, M5ComposerConsumerSurface,
    M5ComposerDeploymentLine, M5ComposerDowngradeTrigger, M5ComposerQualificationClass,
    M5ComposerRouteClass, M5ComposerSurfaceFamily, M5OmittedContextReason, M5TaintSeverity,
    M5TaintSource,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BudgetTaintPacket`].
pub const M5_BUDGET_TAINT_RECORD_KIND: &str =
    "implement_m5_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes";

/// Schema version for M5 budget-strip / tainted-context-warning records.
pub const M5_BUDGET_TAINT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the budget-strip / tainted-context-warning boundary schema.
pub const M5_BUDGET_TAINT_SCHEMA_REF: &str =
    "schemas/ai/m5-budget-size-strip-and-tainted-context-warning.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BUDGET_TAINT_DOC_REF: &str =
    "docs/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes.md";

/// Repo-relative path of the frozen prompt-composer component matrix these primitives
/// narrow from.
pub const M5_BUDGET_TAINT_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the context-assembly record contract the budget strip binds its
/// included / omitted context truth against.
pub const M5_BUDGET_TAINT_CONTEXT_ASSEMBLY_REF: &str = "schemas/ai/context_assembly.schema.json";

/// Repo-relative path of the tainted-context record contract the warning binds its taint
/// source, severity, and data-treatment truth against.
pub const M5_BUDGET_TAINT_TAINTED_CONTEXT_REF: &str = "schemas/ai/tainted_context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BUDGET_TAINT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BUDGET_TAINT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BUDGET_TAINT_CSV_REF: &str =
    "artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BUDGET_TAINT_REPORT_REF: &str =
    "artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes.md";

/// One claimed M5 composer consumer where the user can send an AI request and therefore must
/// see the shared budget strip and the tainted-context warning. These are the consumers the
/// acceptance criteria name — the inline composer, the side panel, the patch draft, the
/// CLI / headless surface, and the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetTaintConsumerSurface {
    /// The inline / AI composer.
    InlineComposer,
    /// The side-panel assistant.
    SidePanel,
    /// The patch-draft composer.
    PatchDraft,
    /// The CLI / headless surface.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl M5BudgetTaintConsumerSurface {
    /// Every claimed send-capable consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlineComposer,
        Self::SidePanel,
        Self::PatchDraft,
        Self::CliHeadless,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineComposer => "inline_composer",
            Self::SidePanel => "side_panel",
            Self::PatchDraft => "patch_draft",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InlineComposer => "Inline Composer",
            Self::SidePanel => "Side Panel",
            Self::PatchDraft => "Patch Draft",
            Self::CliHeadless => "CLI / Headless",
            Self::SupportExport => "Support Export",
        }
    }
}

/// The class of context a budget strip accounts for — the included-versus-omitted context
/// classes named on the strip and in the omitted-context drawer, so a strip never omits a
/// context class without naming which class was left out.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextClass {
    /// System / repo instructions.
    Instructions,
    /// Attached objects (files, symbols, evidence packets).
    AttachedObjects,
    /// The active editor selection.
    ActiveSelection,
    /// Retrieved / indexed snippets.
    RetrievedSnippets,
    /// Prior conversation history.
    ConversationHistory,
    /// Promoted tool output.
    ToolOutput,
}

impl M5ContextClass {
    /// Every context class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Instructions,
        Self::AttachedObjects,
        Self::ActiveSelection,
        Self::RetrievedSnippets,
        Self::ConversationHistory,
        Self::ToolOutput,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Instructions => "instructions",
            Self::AttachedObjects => "attached_objects",
            Self::ActiveSelection => "active_selection",
            Self::RetrievedSnippets => "retrieved_snippets",
            Self::ConversationHistory => "conversation_history",
            Self::ToolOutput => "tool_output",
        }
    }
}

/// The token / size pressure band a budget strip is in — a coarse, non-tokenizer-trivia band
/// derived from the budget posture, so a strip communicates pressure without exposing raw
/// tokenizer internals alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetPressureBand {
    /// Unmetered local execution.
    Unmetered,
    /// Comfortably within budget.
    Nominal,
    /// Approaching the limit.
    Watch,
    /// At or over the limit, truncating to fit.
    Critical,
    /// Hard-blocked by the ceiling.
    Exhausted,
}

impl M5BudgetPressureBand {
    /// Every pressure band, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Unmetered,
        Self::Nominal,
        Self::Watch,
        Self::Critical,
        Self::Exhausted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unmetered => "unmetered",
            Self::Nominal => "nominal",
            Self::Watch => "watch",
            Self::Critical => "critical",
            Self::Exhausted => "exhausted",
        }
    }
}

/// The consequence of a route switch a budget strip must make explicit — so a route change
/// never silently alters behavior between the composed request and what actually leaves the
/// shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RouteSwitchConsequence {
    /// The route is unchanged.
    Unchanged,
    /// The request crossed the on-device boundary (moved on- or off-device).
    LocalityChanged,
    /// The request now travels further / to a wider-reach route.
    ReachWidened,
    /// The request now travels less far / to a narrower-reach route.
    ReachNarrowed,
    /// The route stayed the same reach but changed provider class.
    ProviderChanged,
}

impl M5RouteSwitchConsequence {
    /// Every route-switch consequence, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Unchanged,
        Self::LocalityChanged,
        Self::ReachWidened,
        Self::ReachNarrowed,
        Self::ProviderChanged,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::LocalityChanged => "locality_changed",
            Self::ReachWidened => "reach_widened",
            Self::ReachNarrowed => "reach_narrowed",
            Self::ProviderChanged => "provider_changed",
        }
    }

    /// True when the route actually changed.
    pub const fn is_changed(self) -> bool {
        !matches!(self, Self::Unchanged)
    }
}

/// One bounded action a budget strip offers, so a strip never hides its inspect-omitted /
/// adjust affordances or its route-review / reduce / proceed follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetStripAction {
    /// Open the omitted-context drawer to inspect what was left out.
    InspectOmittedContext,
    /// Review the route change before send.
    ReviewRouteChange,
    /// Adjust the budget or narrow the scope.
    AdjustBudgetOrScope,
    /// Reduce the context to fit.
    ReduceContext,
    /// Proceed to send.
    ProceedToSend,
}

impl M5BudgetStripAction {
    /// Every budget-strip action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InspectOmittedContext,
        Self::ReviewRouteChange,
        Self::AdjustBudgetOrScope,
        Self::ReduceContext,
        Self::ProceedToSend,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectOmittedContext => "inspect_omitted_context",
            Self::ReviewRouteChange => "review_route_change",
            Self::AdjustBudgetOrScope => "adjust_budget_or_scope",
            Self::ReduceContext => "reduce_context",
            Self::ProceedToSend => "proceed_to_send",
        }
    }
}

/// Controlled budget-strip anatomy part the shared strip surfaces. The parts in
/// [`M5BudgetStripAnatomyPart::MANDATORY`] are required on every strip so the included
/// context, omitted-context drawer, budget posture, route switch, and action row are never
/// hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetStripAnatomyPart {
    /// The included context classes.
    IncludedContextCue,
    /// The omitted-context drawer.
    OmittedContextDrawerCue,
    /// The derived budget posture.
    BudgetPostureCue,
    /// The token / size pressure band.
    PressureBandCue,
    /// The truncation reason.
    TruncationReasonCue,
    /// The route-switch consequence.
    RouteSwitchCue,
    /// The bounded action row (inspect / adjust / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5BudgetStripAnatomyPart {
    /// Every budget-strip anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::IncludedContextCue,
        Self::OmittedContextDrawerCue,
        Self::BudgetPostureCue,
        Self::PressureBandCue,
        Self::TruncationReasonCue,
        Self::RouteSwitchCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The budget-strip anatomy parts every strip must render.
    pub const MANDATORY: [Self; 5] = [
        Self::IncludedContextCue,
        Self::OmittedContextDrawerCue,
        Self::BudgetPostureCue,
        Self::RouteSwitchCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludedContextCue => "included_context_cue",
            Self::OmittedContextDrawerCue => "omitted_context_drawer_cue",
            Self::BudgetPostureCue => "budget_posture_cue",
            Self::PressureBandCue => "pressure_band_cue",
            Self::TruncationReasonCue => "truncation_reason_cue",
            Self::RouteSwitchCue => "route_switch_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// The derived posture of a tainted-context warning — the resolver's verdict about whether an
/// untrusted input is untainted, merely flagged as data, elevated and awaiting review, held
/// for quarantine, blocked as suspected injection, or acknowledged and proceedable. Computed
/// in a fixed severity-first order so an injection-suspected or quarantine-required taint never
/// reads as trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintWarningPosture {
    /// No taint; the content is trusted.
    NoTaintTrusted,
    /// Tainted and flagged as data, not instruction.
    FlaggedAsData,
    /// Elevated taint awaiting review before send.
    ElevatedReviewRequired,
    /// Held for quarantine; cannot send until resolved.
    QuarantineHeld,
    /// Suspected prompt injection; blocked.
    InjectionBlocked,
    /// Reviewed and acknowledged as data; proceedable.
    AcknowledgedProceedable,
}

impl M5TaintWarningPosture {
    /// Every warning posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoTaintTrusted,
        Self::FlaggedAsData,
        Self::ElevatedReviewRequired,
        Self::QuarantineHeld,
        Self::InjectionBlocked,
        Self::AcknowledgedProceedable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoTaintTrusted => "no_taint_trusted",
            Self::FlaggedAsData => "flagged_as_data",
            Self::ElevatedReviewRequired => "elevated_review_required",
            Self::QuarantineHeld => "quarantine_held",
            Self::InjectionBlocked => "injection_blocked",
            Self::AcknowledgedProceedable => "acknowledged_proceedable",
        }
    }

    /// True when the warning must carry a quarantine note.
    pub const fn requires_quarantine_note(self) -> bool {
        matches!(self, Self::QuarantineHeld | Self::InjectionBlocked)
    }
}

/// One bounded action a tainted-context warning offers, so a warning never hides its review /
/// quarantine affordances or its remove / acknowledge / proceed follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintWarningAction {
    /// Review the tainted content before send.
    ReviewTaintedContent,
    /// Quarantine the tainted content.
    QuarantineContent,
    /// Remove the tainted content from context.
    RemoveFromContext,
    /// Acknowledge the content as data.
    AcknowledgeAsData,
    /// Proceed with send.
    ProceedWithSend,
}

impl M5TaintWarningAction {
    /// Every warning action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ReviewTaintedContent,
        Self::QuarantineContent,
        Self::RemoveFromContext,
        Self::AcknowledgeAsData,
        Self::ProceedWithSend,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewTaintedContent => "review_tainted_content",
            Self::QuarantineContent => "quarantine_content",
            Self::RemoveFromContext => "remove_from_context",
            Self::AcknowledgeAsData => "acknowledge_as_data",
            Self::ProceedWithSend => "proceed_with_send",
        }
    }
}

/// Controlled tainted-context-warning anatomy part the shared warning surfaces. The parts in
/// [`M5TaintWarningAnatomyPart::MANDATORY`] are required on every warning so the taint source,
/// severity, data-treatment, posture, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintWarningAnatomyPart {
    /// The taint source.
    TaintSourceCue,
    /// The taint severity.
    TaintSeverityCue,
    /// The treated-as-data disclosure.
    TreatedAsDataCue,
    /// The derived warning posture.
    WarningPostureCue,
    /// The quarantine note.
    QuarantineNoteCue,
    /// The review-path cue.
    ReviewPathCue,
    /// The bounded action row (review / quarantine / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5TaintWarningAnatomyPart {
    /// Every warning anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TaintSourceCue,
        Self::TaintSeverityCue,
        Self::TreatedAsDataCue,
        Self::WarningPostureCue,
        Self::QuarantineNoteCue,
        Self::ReviewPathCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The warning anatomy parts every warning must render.
    pub const MANDATORY: [Self; 5] = [
        Self::TaintSourceCue,
        Self::TaintSeverityCue,
        Self::TreatedAsDataCue,
        Self::WarningPostureCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TaintSourceCue => "taint_source_cue",
            Self::TaintSeverityCue => "taint_severity_cue",
            Self::TreatedAsDataCue => "treated_as_data_cue",
            Self::WarningPostureCue => "warning_posture_cue",
            Self::QuarantineNoteCue => "quarantine_note_cue",
            Self::ReviewPathCue => "review_path_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the budget-strip export carries so strip truth is reconstructable. The fields in
/// [`M5BudgetStripExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BudgetStripExportField {
    /// The stable strip id.
    StripId,
    /// The derived budget posture.
    BudgetPosture,
    /// The derived pressure band.
    PressureBand,
    /// The included context classes.
    IncludedContextClasses,
    /// The omitted-context drawer entries.
    OmittedEntries,
    /// The route-switch consequence.
    RouteSwitch,
    /// Whether review is required before send.
    RequiresReviewBeforeSend,
    /// The bounded available actions.
    AvailableActions,
}

impl M5BudgetStripExportField {
    /// Every budget-strip export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StripId,
        Self::BudgetPosture,
        Self::PressureBand,
        Self::IncludedContextClasses,
        Self::OmittedEntries,
        Self::RouteSwitch,
        Self::RequiresReviewBeforeSend,
        Self::AvailableActions,
    ];

    /// The budget-strip export fields every strip must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::StripId,
        Self::BudgetPosture,
        Self::IncludedContextClasses,
        Self::OmittedEntries,
        Self::RouteSwitch,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StripId => "strip_id",
            Self::BudgetPosture => "budget_posture",
            Self::PressureBand => "pressure_band",
            Self::IncludedContextClasses => "included_context_classes",
            Self::OmittedEntries => "omitted_entries",
            Self::RouteSwitch => "route_switch",
            Self::RequiresReviewBeforeSend => "requires_review_before_send",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// A field the tainted-context-warning export carries so warning truth is reconstructable. The
/// fields in [`M5TaintWarningExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TaintWarningExportField {
    /// The stable warning id.
    WarningId,
    /// The taint source.
    TaintSource,
    /// The taint severity.
    TaintSeverity,
    /// The derived warning posture.
    WarningPosture,
    /// Whether the content is treated as data.
    TreatedAsData,
    /// Whether the warning blocks send.
    BlocksSend,
    /// Whether review is required before send.
    RequiresReviewBeforeSend,
    /// The bounded available actions.
    AvailableActions,
}

impl M5TaintWarningExportField {
    /// Every warning export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::WarningId,
        Self::TaintSource,
        Self::TaintSeverity,
        Self::WarningPosture,
        Self::TreatedAsData,
        Self::BlocksSend,
        Self::RequiresReviewBeforeSend,
        Self::AvailableActions,
    ];

    /// The warning export fields every warning must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::WarningId,
        Self::TaintSource,
        Self::TaintSeverity,
        Self::WarningPosture,
        Self::TreatedAsData,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarningId => "warning_id",
            Self::TaintSource => "taint_source",
            Self::TaintSeverity => "taint_severity",
            Self::WarningPosture => "warning_posture",
            Self::TreatedAsData => "treated_as_data",
            Self::BlocksSend => "blocks_send",
            Self::RequiresReviewBeforeSend => "requires_review_before_send",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- budget-or-size strip -----------------------------------------------

/// One entry in a budget strip's omitted-context drawer: a context class that was omitted or
/// truncated, the reason, and an opaque detail. Every drawer entry names its reason and detail
/// so context is never silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OmittedContextEntry {
    /// The context class that was omitted or truncated.
    pub context_class: M5ContextClass,
    /// Why it was omitted or truncated.
    pub reason: M5OmittedContextReason,
    /// The opaque, export-safe detail explaining the omission (must be non-empty).
    pub detail: String,
}

/// The full input to the budget-strip resolver for one composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetSizeStripResolutionInput {
    /// The opaque stable strip id (must be non-empty).
    pub strip_id: String,
    /// The opaque display label (must be non-empty).
    pub strip_label: String,
    /// The context classes included in the request.
    pub included_context_classes: Vec<M5ContextClass>,
    /// The omitted-context drawer entries.
    pub omitted_entries: Vec<M5OmittedContextEntry>,
    /// True when the route runs unmetered on-device.
    pub unmetered_local: bool,
    /// True when the request hit a hard ceiling and cannot send.
    pub hard_ceiling_hit: bool,
    /// True when the request is over budget.
    pub over_budget: bool,
    /// True when truncation is pending to fit.
    pub truncation_pending: bool,
    /// True when the request is near the limit.
    pub near_limit: bool,
    /// The route the composition previously targeted, when known.
    pub route_before: Option<M5ComposerRouteClass>,
    /// The route the composition targets now.
    pub route_after: M5ComposerRouteClass,
}

/// The resolved budget-strip truth for one composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBudgetSizeStrip {
    /// The opaque stable strip id, preserved exactly from the input.
    pub strip_id: String,
    /// The opaque display label.
    pub strip_label: String,
    /// The included context classes.
    pub included_context_classes: Vec<M5ContextClass>,
    /// The omitted-context drawer entries, preserved exactly.
    pub omitted_entries: Vec<M5OmittedContextEntry>,
    /// The derived budget posture.
    pub budget_posture: M5BudgetPosture,
    /// The derived token / size pressure band.
    pub pressure_band: M5BudgetPressureBand,
    /// The route the composition previously targeted, when known.
    pub route_before: Option<M5ComposerRouteClass>,
    /// The route the composition targets now.
    pub route_after: M5ComposerRouteClass,
    /// The derived route-switch consequence.
    pub route_switch: M5RouteSwitchConsequence,
    /// The bounded actions this strip offers.
    pub available_actions: Vec<M5BudgetStripAction>,
    /// True when any context class was omitted or truncated.
    pub has_omitted_context: bool,
    /// True when truncation is active.
    pub truncation_active: bool,
    /// True when the route changed.
    pub route_changed: bool,
    /// True when the request can leave the shell.
    pub is_sendable: bool,
    /// True when omissions, truncation, a route change, or an overrun require review first.
    pub requires_review_before_send: bool,
    /// True when every omission in the drawer names its reason and detail.
    pub discloses_every_omission: bool,
}

/// Errors returned by [`resolve_budget_size_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BudgetSizeStripResolutionError {
    /// The strip id was empty.
    EmptyStripId,
    /// The strip label was empty.
    EmptyStripLabel,
    /// A drawer entry claimed an omission but named no reason (`none_omitted`).
    OmittedEntryWithoutReason,
    /// A drawer entry did not carry its explanatory detail.
    OmittedEntryWithoutDetail,
    /// A strip descriptor carried forbidden material.
    ForbiddenBudgetMaterial,
}

impl M5BudgetSizeStripResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyStripId => "empty_strip_id",
            Self::EmptyStripLabel => "empty_strip_label",
            Self::OmittedEntryWithoutReason => "omitted_entry_without_reason",
            Self::OmittedEntryWithoutDetail => "omitted_entry_without_detail",
            Self::ForbiddenBudgetMaterial => "forbidden_budget_material",
        }
    }
}

impl fmt::Display for M5BudgetSizeStripResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "budget size strip resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BudgetSizeStripResolutionError {}

/// Resolves one budget-or-size strip from its declared context and budget signals.
///
/// The derived budget posture is computed in a fixed blocking-first order: a hard ceiling
/// blocks send first, then an over-budget request, then a truncation-pending request, then a
/// near-limit request, then unmetered local execution, and otherwise the request reads as
/// within budget. Every omitted-context drawer entry must name a real reason and carry its
/// detail — context is never silently dropped — the route-switch consequence is always
/// derived, and the strip offers an inspect-omitted action whenever context was left out so the
/// inspect path is preserved before send.
pub fn resolve_budget_size_strip(
    input: &M5BudgetSizeStripResolutionInput,
) -> Result<M5ResolvedBudgetSizeStrip, M5BudgetSizeStripResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5BudgetSizeStripResolutionError::EmptyStripId);
    }
    if input.strip_label.trim().is_empty() {
        return Err(M5BudgetSizeStripResolutionError::EmptyStripLabel);
    }
    if value_repr_is_forbidden(&input.strip_id) || value_repr_is_forbidden(&input.strip_label) {
        return Err(M5BudgetSizeStripResolutionError::ForbiddenBudgetMaterial);
    }
    for entry in &input.omitted_entries {
        if matches!(entry.reason, M5OmittedContextReason::NoneOmitted) {
            return Err(M5BudgetSizeStripResolutionError::OmittedEntryWithoutReason);
        }
        if entry.detail.trim().is_empty() {
            return Err(M5BudgetSizeStripResolutionError::OmittedEntryWithoutDetail);
        }
        if value_repr_is_forbidden(&entry.detail) {
            return Err(M5BudgetSizeStripResolutionError::ForbiddenBudgetMaterial);
        }
    }

    let budget_posture = derive_budget_posture(
        input.unmetered_local,
        input.hard_ceiling_hit,
        input.over_budget,
        input.truncation_pending,
        input.near_limit,
    );
    let pressure_band = derive_pressure_band(budget_posture);
    let route_switch = derive_route_switch(input.route_before, input.route_after);
    let route_changed = route_switch.is_changed();
    let has_omitted_context = !input.omitted_entries.is_empty();
    let truncation_active = input.truncation_pending
        || input.omitted_entries.iter().any(|entry| {
            matches!(
                entry.reason,
                M5OmittedContextReason::SizeTruncated | M5OmittedContextReason::BudgetCapped
            )
        });
    let is_sendable = !matches!(budget_posture, M5BudgetPosture::HardBlocked);
    let requires_review_before_send = has_omitted_context
        || truncation_active
        || route_changed
        || matches!(
            budget_posture,
            M5BudgetPosture::OverBudget | M5BudgetPosture::HardBlocked
        );
    let available_actions = derive_budget_actions(
        budget_posture,
        has_omitted_context,
        truncation_active,
        route_changed,
        is_sendable,
    );

    Ok(M5ResolvedBudgetSizeStrip {
        strip_id: input.strip_id.clone(),
        strip_label: input.strip_label.clone(),
        included_context_classes: input.included_context_classes.clone(),
        omitted_entries: input.omitted_entries.clone(),
        budget_posture,
        pressure_band,
        route_before: input.route_before,
        route_after: input.route_after,
        route_switch,
        available_actions,
        has_omitted_context,
        truncation_active,
        route_changed,
        is_sendable,
        requires_review_before_send,
        discloses_every_omission: input
            .omitted_entries
            .iter()
            .all(|entry| !entry.detail.trim().is_empty()),
    })
}

/// The fixed blocking-first budget-posture ladder.
fn derive_budget_posture(
    unmetered_local: bool,
    hard_ceiling_hit: bool,
    over_budget: bool,
    truncation_pending: bool,
    near_limit: bool,
) -> M5BudgetPosture {
    if hard_ceiling_hit {
        M5BudgetPosture::HardBlocked
    } else if over_budget {
        M5BudgetPosture::OverBudget
    } else if truncation_pending {
        M5BudgetPosture::TruncationPending
    } else if near_limit {
        M5BudgetPosture::NearLimit
    } else if unmetered_local {
        M5BudgetPosture::UnmeteredLocal
    } else {
        M5BudgetPosture::WithinBudget
    }
}

/// Maps a budget posture to its coarse token / size pressure band.
fn derive_pressure_band(posture: M5BudgetPosture) -> M5BudgetPressureBand {
    match posture {
        M5BudgetPosture::UnmeteredLocal => M5BudgetPressureBand::Unmetered,
        M5BudgetPosture::WithinBudget => M5BudgetPressureBand::Nominal,
        M5BudgetPosture::NearLimit => M5BudgetPressureBand::Watch,
        M5BudgetPosture::TruncationPending | M5BudgetPosture::OverBudget => {
            M5BudgetPressureBand::Critical
        }
        M5BudgetPosture::HardBlocked => M5BudgetPressureBand::Exhausted,
    }
}

/// The reach a route class exposes the request to — 0 stays on-device, higher travels further.
fn route_reach(route: M5ComposerRouteClass) -> u8 {
    match route {
        M5ComposerRouteClass::LocalModel => 0,
        M5ComposerRouteClass::SelfHostedRoute | M5ComposerRouteClass::MirroredRoute => 1,
        M5ComposerRouteClass::PolicyPinnedRoute => 2,
        M5ComposerRouteClass::ByokDirect | M5ComposerRouteClass::ManagedRoute => 3,
    }
}

/// True when a route class keeps the request on-device.
fn route_is_on_device(route: M5ComposerRouteClass) -> bool {
    matches!(route, M5ComposerRouteClass::LocalModel)
}

/// Derives the route-switch consequence from the before / after route classes.
fn derive_route_switch(
    before: Option<M5ComposerRouteClass>,
    after: M5ComposerRouteClass,
) -> M5RouteSwitchConsequence {
    match before {
        None => M5RouteSwitchConsequence::Unchanged,
        Some(before) if before == after => M5RouteSwitchConsequence::Unchanged,
        Some(before) => {
            if route_is_on_device(before) != route_is_on_device(after) {
                M5RouteSwitchConsequence::LocalityChanged
            } else {
                match route_reach(after).cmp(&route_reach(before)) {
                    std::cmp::Ordering::Greater => M5RouteSwitchConsequence::ReachWidened,
                    std::cmp::Ordering::Less => M5RouteSwitchConsequence::ReachNarrowed,
                    std::cmp::Ordering::Equal => M5RouteSwitchConsequence::ProviderChanged,
                }
            }
        }
    }
}

/// Derives the bounded budget-strip-action set.
///
/// Inspect-omitted-context is offered whenever context was left out so the inspect path is
/// preserved before send; review-route-change when the route changed; adjust and reduce follow
/// the pressure; proceed-to-send is offered only when the request is sendable.
fn derive_budget_actions(
    posture: M5BudgetPosture,
    has_omitted_context: bool,
    truncation_active: bool,
    route_changed: bool,
    is_sendable: bool,
) -> Vec<M5BudgetStripAction> {
    use M5BudgetStripAction as Action;
    let mut actions = Vec::new();
    if has_omitted_context {
        actions.push(Action::InspectOmittedContext);
    }
    if route_changed {
        actions.push(Action::ReviewRouteChange);
    }
    if matches!(
        posture,
        M5BudgetPosture::NearLimit
            | M5BudgetPosture::OverBudget
            | M5BudgetPosture::TruncationPending
            | M5BudgetPosture::HardBlocked
    ) {
        actions.push(Action::AdjustBudgetOrScope);
    }
    if truncation_active
        || matches!(
            posture,
            M5BudgetPosture::OverBudget | M5BudgetPosture::HardBlocked
        )
    {
        actions.push(Action::ReduceContext);
    }
    if is_sendable {
        actions.push(Action::ProceedToSend);
    }
    actions
}

// ---- tainted-context warning --------------------------------------------

/// The full input to the tainted-context-warning resolver for one untrusted input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TaintedContextWarningResolutionInput {
    /// The opaque stable warning id (must be non-empty).
    pub warning_id: String,
    /// The opaque label naming the untrusted context (must be non-empty).
    pub context_label: String,
    /// Where the untrusted input came from.
    pub taint_source: M5TaintSource,
    /// How dangerous the taint is.
    pub taint_severity: M5TaintSeverity,
    /// True when the untrusted input is treated as data, not instruction.
    pub treated_as_data: bool,
    /// True when the pending AI route runs side effects.
    pub side_effecting_route: bool,
    /// True when the user has reviewed / acknowledged the taint.
    pub acknowledged: bool,
    /// The opaque quarantine note, when the warning holds or blocks the content.
    pub quarantine_note: Option<String>,
}

/// The resolved tainted-context-warning truth for one untrusted input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTaintedContextWarning {
    /// The opaque stable warning id, preserved exactly from the input.
    pub warning_id: String,
    /// The opaque context label, preserved exactly.
    pub context_label: String,
    /// Where the untrusted input came from.
    pub taint_source: M5TaintSource,
    /// How dangerous the taint is.
    pub taint_severity: M5TaintSeverity,
    /// True when the untrusted input is treated as data, not instruction.
    pub treated_as_data: bool,
    /// True when the pending AI route runs side effects.
    pub side_effecting_route: bool,
    /// True when the user has reviewed / acknowledged the taint.
    pub acknowledged: bool,
    /// The derived warning posture.
    pub warning_posture: M5TaintWarningPosture,
    /// The opaque quarantine note, when the warning holds or blocks the content.
    pub quarantine_note: Option<String>,
    /// The bounded actions this warning offers.
    pub available_actions: Vec<M5TaintWarningAction>,
    /// True when the warning blocks send until it is resolved.
    pub blocks_send: bool,
    /// True when the warning needs review before send.
    pub requires_review_before_send: bool,
    /// True when a review path is preserved before a side-effecting route runs.
    pub preserves_review_path: bool,
    /// True when any tainted content is treated as data, not trusted instruction.
    pub treats_untrusted_as_data: bool,
}

/// Errors returned by [`resolve_tainted_context_warning`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TaintedContextWarningResolutionError {
    /// The warning id was empty.
    EmptyWarningId,
    /// The context label was empty.
    EmptyContextLabel,
    /// A tainted input was not treated as data.
    TaintNotTreatedAsData,
    /// A quarantine-held or injection-blocked warning did not carry its quarantine note.
    QuarantineWithoutNote,
    /// A warning descriptor carried forbidden material.
    ForbiddenTaintMaterial,
}

impl M5TaintedContextWarningResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyWarningId => "empty_warning_id",
            Self::EmptyContextLabel => "empty_context_label",
            Self::TaintNotTreatedAsData => "taint_not_treated_as_data",
            Self::QuarantineWithoutNote => "quarantine_without_note",
            Self::ForbiddenTaintMaterial => "forbidden_taint_material",
        }
    }
}

impl fmt::Display for M5TaintedContextWarningResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "tainted context warning resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TaintedContextWarningResolutionError {}

/// Resolves one tainted-context warning from its declared taint signals.
///
/// The derived posture is computed in a fixed severity-first order: no taint reads as trusted,
/// then suspected injection is blocked, then a quarantine-required taint is held, then an
/// elevated taint reads as elevated-review-required (or acknowledged-proceedable once
/// acknowledged), and an informational taint reads as flagged-as-data (or acknowledged). Any
/// tainted input must be treated as data rather than instruction, a held or blocked warning
/// must carry its quarantine note, the review path is always preserved for tainted content, and
/// a side-effecting route never runs past an unresolved elevated, quarantine, or injection
/// warning.
pub fn resolve_tainted_context_warning(
    input: &M5TaintedContextWarningResolutionInput,
) -> Result<M5ResolvedTaintedContextWarning, M5TaintedContextWarningResolutionError> {
    if input.warning_id.trim().is_empty() {
        return Err(M5TaintedContextWarningResolutionError::EmptyWarningId);
    }
    if input.context_label.trim().is_empty() {
        return Err(M5TaintedContextWarningResolutionError::EmptyContextLabel);
    }
    if value_repr_is_forbidden(&input.warning_id)
        || value_repr_is_forbidden(&input.context_label)
        || input
            .quarantine_note
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
    {
        return Err(M5TaintedContextWarningResolutionError::ForbiddenTaintMaterial);
    }

    let is_tainted = !matches!(input.taint_severity, M5TaintSeverity::None);
    if is_tainted && !input.treated_as_data {
        return Err(M5TaintedContextWarningResolutionError::TaintNotTreatedAsData);
    }

    let warning_posture = derive_taint_posture(input.taint_severity, input.acknowledged);
    if warning_posture.requires_quarantine_note() && input.quarantine_note.is_none() {
        return Err(M5TaintedContextWarningResolutionError::QuarantineWithoutNote);
    }

    let blocks_send = matches!(
        warning_posture,
        M5TaintWarningPosture::QuarantineHeld | M5TaintWarningPosture::InjectionBlocked
    ) || (input.side_effecting_route
        && matches!(
            warning_posture,
            M5TaintWarningPosture::ElevatedReviewRequired
        ));
    let requires_review_before_send = matches!(
        warning_posture,
        M5TaintWarningPosture::FlaggedAsData
            | M5TaintWarningPosture::ElevatedReviewRequired
            | M5TaintWarningPosture::QuarantineHeld
            | M5TaintWarningPosture::InjectionBlocked
    );
    let available_actions = derive_taint_actions(warning_posture, is_tainted, blocks_send);
    let preserves_review_path =
        !is_tainted || available_actions.contains(&M5TaintWarningAction::ReviewTaintedContent);

    Ok(M5ResolvedTaintedContextWarning {
        warning_id: input.warning_id.clone(),
        context_label: input.context_label.clone(),
        taint_source: input.taint_source,
        taint_severity: input.taint_severity,
        treated_as_data: input.treated_as_data,
        side_effecting_route: input.side_effecting_route,
        acknowledged: input.acknowledged,
        warning_posture,
        quarantine_note: input.quarantine_note.clone(),
        available_actions,
        blocks_send,
        requires_review_before_send,
        preserves_review_path,
        treats_untrusted_as_data: !is_tainted || input.treated_as_data,
    })
}

/// The fixed severity-first taint-warning-posture ladder.
fn derive_taint_posture(severity: M5TaintSeverity, acknowledged: bool) -> M5TaintWarningPosture {
    match severity {
        M5TaintSeverity::None => M5TaintWarningPosture::NoTaintTrusted,
        M5TaintSeverity::InjectionSuspected => M5TaintWarningPosture::InjectionBlocked,
        M5TaintSeverity::QuarantineRequired => M5TaintWarningPosture::QuarantineHeld,
        M5TaintSeverity::Elevated => {
            if acknowledged {
                M5TaintWarningPosture::AcknowledgedProceedable
            } else {
                M5TaintWarningPosture::ElevatedReviewRequired
            }
        }
        M5TaintSeverity::Informational => {
            if acknowledged {
                M5TaintWarningPosture::AcknowledgedProceedable
            } else {
                M5TaintWarningPosture::FlaggedAsData
            }
        }
    }
}

/// Derives the bounded taint-warning-action set.
///
/// Review-tainted-content and remove-from-context are offered whenever content is tainted so
/// the review path is preserved; quarantine follows a held / blocked posture; acknowledge
/// follows a flagged / elevated posture; proceed-with-send is offered only when the warning does
/// not block send.
fn derive_taint_actions(
    posture: M5TaintWarningPosture,
    is_tainted: bool,
    blocks_send: bool,
) -> Vec<M5TaintWarningAction> {
    use M5TaintWarningAction as Action;
    let mut actions = Vec::new();
    if is_tainted {
        actions.push(Action::ReviewTaintedContent);
    }
    if matches!(
        posture,
        M5TaintWarningPosture::QuarantineHeld | M5TaintWarningPosture::InjectionBlocked
    ) {
        actions.push(Action::QuarantineContent);
    }
    if is_tainted {
        actions.push(Action::RemoveFromContext);
    }
    if matches!(
        posture,
        M5TaintWarningPosture::FlaggedAsData | M5TaintWarningPosture::ElevatedReviewRequired
    ) {
        actions.push(Action::AcknowledgeAsData);
    }
    if !blocks_send {
        actions.push(Action::ProceedWithSend);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked budget-strip resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetSizeStripResolutionCase {
    /// The resolver input.
    pub input: M5BudgetSizeStripResolutionInput,
    /// The resolved truth. Must equal `resolve_budget_size_strip(&input)`.
    pub resolved: M5ResolvedBudgetSizeStrip,
}

impl M5BudgetSizeStripResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BudgetSizeStripResolutionInput) -> Self {
        let resolved = resolve_budget_size_strip(&input).expect("seed budget case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_budget_size_strip(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved strip id preserves the input id exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.strip_id == self.input.strip_id
    }
}

/// One worked tainted-context-warning resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TaintedContextWarningResolutionCase {
    /// The resolver input.
    pub input: M5TaintedContextWarningResolutionInput,
    /// The resolved truth. Must equal `resolve_tainted_context_warning(&input)`.
    pub resolved: M5ResolvedTaintedContextWarning,
}

impl M5TaintedContextWarningResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5TaintedContextWarningResolutionInput) -> Self {
        let resolved = resolve_tainted_context_warning(&input).expect("seed taint case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_tainted_context_warning(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved warning id and context label preserve the input exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.warning_id == self.input.warning_id
            && self.resolved.context_label == self.input.context_label
    }
}

/// One row in the primitive matrix: one send-capable consumer bound to the shared budget and
/// taint anatomy, budget postures, pressure bands, omitted-context reasons, context classes,
/// route-switch consequences, taint sources, taint severities, warning postures, bounded
/// actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintRow {
    /// Send-capable consumer family.
    pub consumer_surface: M5BudgetTaintConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ComposerQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 composer surface families that render / consume these components.
    pub surface_families: Vec<M5ComposerSurfaceFamily>,
    /// Deployment lines these components keep the same truth across.
    pub deployment_lines: Vec<M5ComposerDeploymentLine>,
    /// Budget-strip anatomy parts this row renders (must include the mandatory parts).
    pub budget_anatomy_parts: Vec<M5BudgetStripAnatomyPart>,
    /// Warning anatomy parts this row renders (must include the mandatory parts).
    pub taint_anatomy_parts: Vec<M5TaintWarningAnatomyPart>,
    /// Budget postures this consumer distinguishes.
    pub budget_postures: Vec<M5BudgetPosture>,
    /// Pressure bands this consumer distinguishes.
    pub pressure_bands: Vec<M5BudgetPressureBand>,
    /// Omitted-context reasons this consumer distinguishes.
    pub omitted_reasons: Vec<M5OmittedContextReason>,
    /// Context classes this consumer accounts for.
    pub context_classes: Vec<M5ContextClass>,
    /// Route-switch consequences this consumer distinguishes.
    pub route_switch_consequences: Vec<M5RouteSwitchConsequence>,
    /// Bounded budget-strip actions this consumer offers.
    pub budget_actions: Vec<M5BudgetStripAction>,
    /// Taint sources this consumer distinguishes.
    pub taint_sources: Vec<M5TaintSource>,
    /// Taint severities this consumer distinguishes.
    pub taint_severities: Vec<M5TaintSeverity>,
    /// Warning postures this consumer distinguishes.
    pub taint_warning_postures: Vec<M5TaintWarningPosture>,
    /// Bounded warning actions this consumer offers.
    pub taint_actions: Vec<M5TaintWarningAction>,
    /// Budget export fields this row carries (must include the mandatory fields).
    pub budget_export_fields: Vec<M5BudgetStripExportField>,
    /// Warning export fields this row carries (must include the mandatory fields).
    pub taint_export_fields: Vec<M5TaintWarningExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ComposerAccessibilityRoute>,
    /// Composer subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComposerConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ComposerDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked budget-strip resolutions proving the budget resolver on this consumer.
    pub budget_examples: Vec<M5BudgetSizeStripResolutionCase>,
    /// Worked warning resolutions proving the taint resolver on this consumer.
    pub taint_examples: Vec<M5TaintedContextWarningResolutionCase>,
    /// Hard invariant: this consumer never masks budget or omission truth. MUST be `false`.
    pub masks_budget_or_omission_truth: bool,
    /// Hard invariant: this consumer never downplays a taint source or severity. MUST be
    /// `false`.
    pub downplays_taint_source_or_severity: bool,
    /// Hard invariant: this consumer never invents a private context grammar. MUST be `false`.
    pub invents_private_context_grammar: bool,
    /// Hard invariant: this consumer never bypasses review before a side-effecting send. MUST
    /// be `false`.
    pub bypasses_review_before_side_effecting_send: bool,
}

impl M5BudgetTaintRow {
    /// True when the row declares every mandatory budget anatomy part.
    fn declares_mandatory_budget_anatomy(&self) -> bool {
        let present: BTreeSet<M5BudgetStripAnatomyPart> =
            self.budget_anatomy_parts.iter().copied().collect();
        M5BudgetStripAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory warning anatomy part.
    fn declares_mandatory_taint_anatomy(&self) -> bool {
        let present: BTreeSet<M5TaintWarningAnatomyPart> =
            self.taint_anatomy_parts.iter().copied().collect();
        M5TaintWarningAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory budget export field.
    fn declares_mandatory_budget_export(&self) -> bool {
        let present: BTreeSet<M5BudgetStripExportField> =
            self.budget_export_fields.iter().copied().collect();
        M5BudgetStripExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory warning export field.
    fn declares_mandatory_taint_export(&self) -> bool {
        let present: BTreeSet<M5TaintWarningExportField> =
            self.taint_export_fields.iter().copied().collect();
        M5TaintWarningExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_budget_or_omission_truth
            && !self.downplays_taint_source_or_severity
            && !self.invents_private_context_grammar
            && !self.bypasses_review_before_side_effecting_send
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintVocabularySet {
    /// Send-capable-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Budget-anatomy-part tokens.
    pub budget_anatomy_parts: Vec<String>,
    /// Warning-anatomy-part tokens.
    pub taint_anatomy_parts: Vec<String>,
    /// Budget-posture tokens (reused from the frozen matrix).
    pub budget_postures: Vec<String>,
    /// Pressure-band tokens.
    pub pressure_bands: Vec<String>,
    /// Omitted-context-reason tokens (reused from the frozen matrix).
    pub omitted_reasons: Vec<String>,
    /// Context-class tokens.
    pub context_classes: Vec<String>,
    /// Route-switch-consequence tokens.
    pub route_switch_consequences: Vec<String>,
    /// Budget-action tokens.
    pub budget_actions: Vec<String>,
    /// Taint-source tokens (reused from the frozen matrix).
    pub taint_sources: Vec<String>,
    /// Taint-severity tokens (reused from the frozen matrix).
    pub taint_severities: Vec<String>,
    /// Warning-posture tokens.
    pub taint_warning_postures: Vec<String>,
    /// Warning-action tokens.
    pub taint_actions: Vec<String>,
    /// Budget-export-field tokens.
    pub budget_export_fields: Vec<String>,
    /// Warning-export-field tokens.
    pub taint_export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5BudgetTaintVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5BudgetTaintConsumerSurface::ALL, |v| v.as_str()),
            budget_anatomy_parts: tokens(&M5BudgetStripAnatomyPart::ALL, |v| v.as_str()),
            taint_anatomy_parts: tokens(&M5TaintWarningAnatomyPart::ALL, |v| v.as_str()),
            budget_postures: tokens(&M5BudgetPosture::ALL, |v| v.as_str()),
            pressure_bands: tokens(&M5BudgetPressureBand::ALL, |v| v.as_str()),
            omitted_reasons: tokens(&M5OmittedContextReason::ALL, |v| v.as_str()),
            context_classes: tokens(&M5ContextClass::ALL, |v| v.as_str()),
            route_switch_consequences: tokens(&M5RouteSwitchConsequence::ALL, |v| v.as_str()),
            budget_actions: tokens(&M5BudgetStripAction::ALL, |v| v.as_str()),
            taint_sources: tokens(&M5TaintSource::ALL, |v| v.as_str()),
            taint_severities: tokens(&M5TaintSeverity::ALL, |v| v.as_str()),
            taint_warning_postures: tokens(&M5TaintWarningPosture::ALL, |v| v.as_str()),
            taint_actions: tokens(&M5TaintWarningAction::ALL, |v| v.as_str()),
            budget_export_fields: tokens(&M5BudgetStripExportField::ALL, |v| v.as_str()),
            taint_export_fields: tokens(&M5TaintWarningExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComposerAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5BudgetTaintGovernanceReview {
    /// One primitive pair carries budget and taint truth on every consumer.
    pub one_primitive_carries_budget_and_taint_truth: bool,
    /// The budget strip names included and omitted context classes.
    pub budget_strip_names_included_and_omitted_context: bool,
    /// Every omission names its reason and detail.
    pub omitted_context_always_names_reason_and_detail: bool,
    /// A truncation reason is always disclosed.
    pub truncation_reason_always_disclosed: bool,
    /// A route-switch consequence is always explicit.
    pub route_switch_consequence_always_explicit: bool,
    /// A taint source and severity are always shown.
    pub taint_source_and_severity_always_shown: bool,
    /// Untrusted content is always treated as data, not instruction.
    pub untrusted_content_treated_as_data: bool,
    /// The taint warning preserves a review path before a side-effecting send.
    pub taint_preserves_review_before_side_effecting_send: bool,
    /// Omission and route-change truth is exportable, not inferred later.
    pub omission_and_route_change_exportable: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintConsumerProjection {
    /// Every send-capable surface consumes the shared primitive pair.
    pub send_capable_surfaces_consume_shared_primitive: bool,
    /// The budget-posture derivation reads a single canonical source.
    pub budget_posture_reads_single_source: bool,
    /// The omitted-context derivation reads a single canonical source.
    pub omitted_context_reads_single_source: bool,
    /// The taint-state derivation reads a single canonical source.
    pub taint_state_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BudgetTaintPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BudgetTaintPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5BudgetTaintRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BudgetTaintVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BudgetTaintGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BudgetTaintConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BudgetTaintProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BudgetTaintReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 budget-strip / tainted-context-warning primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BudgetTaintPacket {
    /// Record kind; must equal [`M5_BUDGET_TAINT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BUDGET_TAINT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5BudgetTaintRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BudgetTaintVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BudgetTaintGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BudgetTaintConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BudgetTaintProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BudgetTaintReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BudgetTaintPacket {
    /// Builds an M5 budget/taint-primitive packet from stable-lane input.
    pub fn new(input: M5BudgetTaintPacketInput) -> Self {
        Self {
            record_kind: M5_BUDGET_TAINT_RECORD_KIND.to_owned(),
            schema_version: M5_BUDGET_TAINT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 budget/taint-primitive invariants.
    pub fn validate(&self) -> Vec<M5BudgetTaintViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BUDGET_TAINT_RECORD_KIND {
            violations.push(M5BudgetTaintViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BUDGET_TAINT_SCHEMA_VERSION {
            violations.push(M5BudgetTaintViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BudgetTaintViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_budget_omission_disclosure(self, &mut violations);
        validate_budget_route_change_coverage(self, &mut violations);
        validate_taint_input_class_coverage(self, &mut violations);
        validate_taint_review_path(self, &mut violations);
        validate_taint_treated_as_data(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 budget/taint primitive packet serializes"),
        ) {
            violations.push(M5BudgetTaintViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 budget/taint primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per send-capable consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,budget_anatomy,taint_anatomy,budget_postures,pressure_bands,omitted_reasons,route_switches,taint_sources,taint_severities,warning_postures,budget_examples,taint_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.budget_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.taint_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.budget_postures, |v| v.as_str()),
                join_tokens(&row.pressure_bands, |v| v.as_str()),
                join_tokens(&row.omitted_reasons, |v| v.as_str()),
                join_tokens(&row.route_switch_consequences, |v| v.as_str()),
                join_tokens(&row.taint_sources, |v| v.as_str()),
                join_tokens(&row.taint_severities, |v| v.as_str()),
                join_tokens(&row.taint_warning_postures, |v| v.as_str()),
                row.budget_examples.len(),
                row.taint_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Budget-Size-Strip and Tainted-Context-Warning Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Send-capable consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Budget postures: {}\n",
            self.vocabulary_set.budget_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Route-switch consequences: {}\n",
            self.vocabulary_set.route_switch_consequences.join(", ")
        ));
        out.push_str(&format!(
            "- Warning postures: {}\n",
            self.vocabulary_set.taint_warning_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Send-capable consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked strips: {}\n",
                row.budget_examples.len()
            ));
            for case in &row.budget_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (band `{}`, omitted `{}`, route `{}`, review `{}`)\n",
                    case.resolved.strip_id,
                    case.resolved.budget_posture.as_str(),
                    case.resolved.pressure_band.as_str(),
                    case.resolved.has_omitted_context,
                    case.resolved.route_switch.as_str(),
                    case.resolved.requires_review_before_send,
                ));
            }
            out.push_str(&format!(
                "  - Worked warnings: {}\n",
                row.taint_examples.len()
            ));
            for case in &row.taint_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (blocks send `{}`, review `{}`)\n",
                    case.resolved.warning_id,
                    case.resolved.taint_source.as_str(),
                    case.resolved.warning_posture.as_str(),
                    case.resolved.blocks_send,
                    case.resolved.requires_review_before_send,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 budget/taint-primitive export.
#[derive(Debug)]
pub enum M5BudgetTaintArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BudgetTaintViolation>),
}

impl fmt::Display for M5BudgetTaintArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 budget/taint primitive export parse failed: {error}"
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
                    "m5 budget/taint primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BudgetTaintArtifactError {}

/// Validation failures emitted by [`M5BudgetTaintPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BudgetTaintViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required send-capable consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A composer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory budget anatomy parts.
    MandatoryBudgetAnatomyMissing,
    /// A row omits one of the mandatory warning anatomy parts.
    MandatoryTaintAnatomyMissing,
    /// A row omits one of the mandatory budget export fields.
    MandatoryBudgetExportMissing,
    /// A row omits one of the mandatory warning export fields.
    MandatoryTaintExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked budget resolutions.
    BudgetExampleMissing,
    /// A row declares no worked warning resolutions.
    TaintExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked budget resolution proves an omission disclosed before send.
    BudgetOmissionDisclosureUnproven,
    /// No worked budget resolution proves a route change reviewed before send.
    BudgetRouteChangeCoverageUnproven,
    /// The defined tainted-input classes are not all proven by a worked warning.
    TaintInputClassCoverageUnproven,
    /// No worked warning proves a review path preserved before a side-effecting send.
    TaintReviewPathUnproven,
    /// A tainted worked warning does not treat its untrusted content as data.
    TaintTreatedAsDataUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BudgetTaintViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryBudgetAnatomyMissing => "mandatory_budget_anatomy_missing",
            Self::MandatoryTaintAnatomyMissing => "mandatory_taint_anatomy_missing",
            Self::MandatoryBudgetExportMissing => "mandatory_budget_export_missing",
            Self::MandatoryTaintExportMissing => "mandatory_taint_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::BudgetExampleMissing => "budget_example_missing",
            Self::TaintExampleMissing => "taint_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::BudgetOmissionDisclosureUnproven => "budget_omission_disclosure_unproven",
            Self::BudgetRouteChangeCoverageUnproven => "budget_route_change_coverage_unproven",
            Self::TaintInputClassCoverageUnproven => "taint_input_class_coverage_unproven",
            Self::TaintReviewPathUnproven => "taint_review_path_unproven",
            Self::TaintTreatedAsDataUnproven => "taint_treated_as_data_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 budget/taint-primitive export.
pub fn current_stable_m5_budget_taint_export(
) -> Result<M5BudgetTaintPacket, M5BudgetTaintArtifactError> {
    let packet: M5BudgetTaintPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes/support_export.json"
    )))
    .map_err(M5BudgetTaintArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BudgetTaintArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BUDGET_TAINT_SCHEMA_REF,
        M5_BUDGET_TAINT_DOC_REF,
        M5_BUDGET_TAINT_COMPONENT_MATRIX_REF,
        M5_BUDGET_TAINT_CONTEXT_ASSEMBLY_REF,
        M5_BUDGET_TAINT_TAINTED_CONTEXT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BudgetTaintViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BudgetTaintViolation::VocabularySetDrift);
    }
}

fn validate_rows(packet: &M5BudgetTaintPacket, violations: &mut Vec<M5BudgetTaintViolation>) {
    let present: BTreeSet<M5BudgetTaintConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5BudgetTaintConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5BudgetTaintViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.budget_anatomy_parts.is_empty()
            || row.taint_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.budget_postures.is_empty()
            || row.pressure_bands.is_empty()
            || row.omitted_reasons.is_empty()
            || row.context_classes.is_empty()
            || row.route_switch_consequences.is_empty()
            || row.budget_actions.is_empty()
            || row.taint_sources.is_empty()
            || row.taint_severities.is_empty()
            || row.taint_warning_postures.is_empty()
            || row.taint_actions.is_empty()
        {
            violations.push(M5BudgetTaintViolation::RowIncomplete);
        }
        if !row.declares_mandatory_budget_anatomy() {
            violations.push(M5BudgetTaintViolation::MandatoryBudgetAnatomyMissing);
        }
        if !row.declares_mandatory_taint_anatomy() {
            violations.push(M5BudgetTaintViolation::MandatoryTaintAnatomyMissing);
        }
        if !row.declares_mandatory_budget_export() {
            violations.push(M5BudgetTaintViolation::MandatoryBudgetExportMissing);
        }
        if !row.declares_mandatory_taint_export() {
            violations.push(M5BudgetTaintViolation::MandatoryTaintExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5BudgetTaintViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BudgetTaintViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BudgetTaintViolation::DowngradeTriggersMissing);
        }
        if row.budget_examples.is_empty() {
            violations.push(M5BudgetTaintViolation::BudgetExampleMissing);
        }
        if row.taint_examples.is_empty() {
            violations.push(M5BudgetTaintViolation::TaintExampleMissing);
        }
        if row
            .budget_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .taint_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BudgetTaintViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BudgetTaintViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BudgetTaintViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked budget resolution across the matrix must prove an omission that is
/// disclosed and requires review before send — the acceptance-criterion example that a strip
/// surfaces omitted or truncated context truth everywhere the user can send.
fn validate_budget_omission_disclosure(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.budget_examples.iter().any(|case| {
            case.resolved.has_omitted_context
                && case.resolved.requires_review_before_send
                && case.resolved.discloses_every_omission
        })
    });
    if !proven {
        violations.push(M5BudgetTaintViolation::BudgetOmissionDisclosureUnproven);
    }
}

/// At least one worked budget resolution must prove a route change that offers a review-route
/// action before send — the acceptance-criterion example that route-change states are explicit
/// instead of inferred later.
fn validate_budget_route_change_coverage(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.budget_examples.iter().any(|case| {
            case.resolved.route_changed
                && case
                    .resolved
                    .available_actions
                    .contains(&M5BudgetStripAction::ReviewRouteChange)
        })
    });
    if !proven {
        violations.push(M5BudgetTaintViolation::BudgetRouteChangeCoverageUnproven);
    }
}

/// The defined tainted-input classes — pasted external text, promoted tool output, and prior
/// model output — must each be proven by a worked warning, so the warning appears for the
/// defined input classes.
fn validate_taint_input_class_coverage(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let sources: BTreeSet<M5TaintSource> = packet
        .rows
        .iter()
        .flat_map(|row| row.taint_examples.iter())
        .map(|case| case.resolved.taint_source)
        .collect();
    let required = [
        M5TaintSource::PastedExternalText,
        M5TaintSource::ToolOutput,
        M5TaintSource::PriorModelOutput,
    ];
    if !required.iter().all(|source| sources.contains(source)) {
        violations.push(M5BudgetTaintViolation::TaintInputClassCoverageUnproven);
    }
}

/// At least one worked warning must prove a side-effecting route that blocks send and preserves
/// its review path — the acceptance-criterion example that the warning preserves a review path
/// before any side-effectful AI route runs.
fn validate_taint_review_path(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.taint_examples.iter().any(|case| {
            case.resolved.side_effecting_route
                && case.resolved.blocks_send
                && case.resolved.preserves_review_path
        })
    });
    if !proven {
        violations.push(M5BudgetTaintViolation::TaintReviewPathUnproven);
    }
}

/// Every tainted worked warning must treat its untrusted content as data — the
/// acceptance-criterion example that untrusted content is treated as data, not trusted
/// instruction.
fn validate_taint_treated_as_data(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let treated = packet
        .rows
        .iter()
        .flat_map(|row| row.taint_examples.iter())
        .filter(|case| !matches!(case.resolved.taint_severity, M5TaintSeverity::None))
        .all(|case| case.resolved.treats_untrusted_as_data);
    if !treated {
        violations.push(M5BudgetTaintViolation::TaintTreatedAsDataUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_budget_and_taint_truth,
        review.budget_strip_names_included_and_omitted_context,
        review.omitted_context_always_names_reason_and_detail,
        review.truncation_reason_always_disclosed,
        review.route_switch_consequence_always_explicit,
        review.taint_source_and_severity_always_shown,
        review.untrusted_content_treated_as_data,
        review.taint_preserves_review_before_side_effecting_send,
        review.omission_and_route_change_exportable,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5BudgetTaintViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.send_capable_surfaces_consume_shared_primitive,
        projection.budget_posture_reads_single_source,
        projection.omitted_context_reads_single_source,
        projection.taint_state_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BudgetTaintViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BudgetTaintViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BudgetTaintPacket,
    violations: &mut Vec<M5BudgetTaintViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BudgetTaintViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
