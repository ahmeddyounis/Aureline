//! Two reusable M5 prompt-composer primitives — the `@`-mention resolver and the
//! slash-command row — so pre-send composition uses the same stable object and command
//! language as the rest of Aureline.
//!
//! Aureline's frozen prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! names the mention resolver and the slash-command row as two governed component families
//! and freezes their controlled vocabulary — the mention-resolution states and the
//! slash-command availability states, the surface families, the deployment lines, the
//! consumer surfaces, the accessibility routes, the qualification classes, and the
//! downgrade triggers. This module *implements* those two contracts as reusable primitives
//! so a user can tell — from the mention row or the command row alone — which stable object
//! an `@`-mention actually binds to, whether that binding is unique, pinned, ambiguous,
//! unresolved, out of scope, or deferred, what the exact target preview is, and, for every
//! slash command, its stable command id, capability class, help path, availability posture,
//! disabled-state explanation, and approval semantics — the same truth the command graph
//! projects to the palette, automation, and CLI surfaces.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_mention_resolver`] — takes one mention's typed token, scope note, candidate
//!    count, exact-stable / pinned signals, target identity / preview, in-scope, and
//!    deferred signals, and produces one [`M5ResolvedMentionResolver`] carrying the derived
//!    [`M5MentionResolution`] (resolved-unique versus resolved-pinned versus
//!    ambiguous-candidates versus unresolved-missing versus out-of-scope-denied versus
//!    deferred-pending), the bounded open / choose / edit / remove / reveal actions, whether
//!    the mention is bound to a single target, whether an ambiguous or unresolved binding
//!    blocks or narrows send with explicit review, and whether the exact-target preview is
//!    preserved. It prefers exact stable objects, never silently binds an ambiguous or
//!    unresolved mention to the wrong target, and always preserves the scope note.
//! 2. [`resolve_slash_command_row`] — takes one command's stable id, label, capability
//!    class, help path, declared command-graph availability state, approval flag, disabled
//!    reason, and alias target, and produces one [`M5ResolvedSlashCommandRow`] carrying the
//!    derived [`M5SlashCommandRowPosture`] (ready-invocable versus approval-gated versus
//!    disabled-explained versus deprecated-redirect versus policy-hidden versus
//!    unknown-rejected), the bounded invoke / request-approval / open-help / view-canonical /
//!    explain-disabled actions, whether the command is invocable, blocked, or approval-gated,
//!    and whether a disabled state carries its explanation. It reuses stable command ids and
//!    the command-graph availability, never presents a disabled or approval-gated command as
//!    a plain ready action, and never hides a disabled-state reason.
//!
//! A single parity matrix — [`M5MentionSlashCommandPacket`] — binds one row per claimed M5
//! composer consumer (the inline composer, the command palette, the automation recipe, the
//! CLI / headless surface, and the support export) to the shared mention and command
//! anatomy, the same mention resolutions, slash-command states, capability classes, row
//! postures, bounded actions, export fields, and non-visual accessibility routes, so the
//! mention and slash-command grammar stays identical across AI composition, palette,
//! automation, CLI / headless, and support exports rather than drifting into a separate
//! AI-only grammar.
//!
//! The mention-resolution state ([`M5MentionResolution`]), slash-command availability state
//! ([`M5SlashCommandState`]), surface family ([`M5ComposerSurfaceFamily`]), deployment line
//! ([`M5ComposerDeploymentLine`]), consumer surface ([`M5ComposerConsumerSurface`]),
//! accessibility route ([`M5ComposerAccessibilityRoute`]), qualification class
//! ([`M5ComposerQualificationClass`]), and downgrade trigger
//! ([`M5ComposerDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the mention resolver
//! and the slash-command row themselves: their composer consumers, their anatomy parts,
//! their bounded actions, the slash-command capability class, the derived slash-command row
//! posture, and their export fields. No M5 composer surface invents a second command
//! grammar.
//!
//! Raw prompts, mention query bodies, command bodies, raw argument values, raw paths, raw
//! URLs, credentials, and private endpoints stay outside the support boundary; every
//! command id, mention token, scope note, help path, and target label is carried only as an
//! opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-mention-resolver-and-slash-command-row.schema.json`](../../../../schemas/ai/m5-mention-resolver-and-slash-command-row.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces.md`](../../../../docs/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_mention_slash_command_automation_recipe_preview_narrowed,
    seeded_m5_mention_slash_command_cli_headless_beta_narrowed,
    seeded_m5_mention_slash_command_packet, M5_MENTION_SLASH_COMMAND_PACKET_ID,
};

// The mention-resolution state, slash-command availability state, surface family,
// deployment line, consumer surface, accessibility route, qualification class, and
// downgrade triggers are frozen once, in the prompt-composer component matrix. These
// primitives reuse them verbatim so they never invent a parallel command vocabulary.
pub use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5ComposerAccessibilityRoute, M5ComposerConsumerSurface, M5ComposerDeploymentLine,
    M5ComposerDowngradeTrigger, M5ComposerQualificationClass, M5ComposerSurfaceFamily,
    M5MentionResolution, M5SlashCommandState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5MentionSlashCommandPacket`].
pub const M5_MENTION_SLASH_COMMAND_RECORD_KIND: &str =
    "ship_m5_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces";

/// Schema version for M5 mention-resolver / slash-command-row records.
pub const M5_MENTION_SLASH_COMMAND_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the mention-resolver / slash-command-row boundary schema.
pub const M5_MENTION_SLASH_COMMAND_SCHEMA_REF: &str =
    "schemas/ai/m5-mention-resolver-and-slash-command-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MENTION_SLASH_COMMAND_DOC_REF: &str =
    "docs/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces.md";

/// Repo-relative path of the frozen prompt-composer component matrix these primitives
/// narrow from.
pub const M5_MENTION_SLASH_COMMAND_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the stable command-descriptor contract these rows reuse their
/// command id, capability class, help path, availability, and approval semantics against.
pub const M5_MENTION_SLASH_COMMAND_COMMAND_GRAPH_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Repo-relative path of the prompt-composer-draft / mention-provenance record contract
/// this primitive binds its mention-identity truth against.
pub const M5_MENTION_SLASH_COMMAND_MENTION_PROVENANCE_REF: &str =
    "schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MENTION_SLASH_COMMAND_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MENTION_SLASH_COMMAND_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_MENTION_SLASH_COMMAND_CSV_REF: &str =
    "artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_MENTION_SLASH_COMMAND_REPORT_REF: &str =
    "artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces.md";

/// One claimed M5 composer consumer that renders the shared mention resolver and the
/// slash-command row. These are the consumers the acceptance criteria name — the inline
/// composer, the command palette, the automation recipe, the CLI / headless surface, and
/// the support export — spanning AI composition and the non-AI surfaces the same commands
/// are reached from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MentionSlashCommandConsumerSurface {
    /// The inline / AI composer.
    InlineComposer,
    /// The command palette.
    CommandPalette,
    /// The automation recipe editor.
    AutomationRecipe,
    /// The CLI / headless surface.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl M5MentionSlashCommandConsumerSurface {
    /// Every claimed composer consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlineComposer,
        Self::CommandPalette,
        Self::AutomationRecipe,
        Self::CliHeadless,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineComposer => "inline_composer",
            Self::CommandPalette => "command_palette",
            Self::AutomationRecipe => "automation_recipe",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InlineComposer => "Inline Composer",
            Self::CommandPalette => "Command Palette",
            Self::AutomationRecipe => "Automation Recipe",
            Self::CliHeadless => "CLI / Headless",
            Self::SupportExport => "Support Export",
        }
    }
}

/// One bounded action a mention row offers, so a mention never hides its open / choose /
/// edit affordances or its remove / reveal-scope follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MentionResolverAction {
    /// Open the bound target object.
    OpenTarget,
    /// Choose among ambiguous candidates before send.
    ChooseCandidate,
    /// Edit the mention query to resolve or narrow it.
    EditMention,
    /// Remove the mention from the composition.
    RemoveMention,
    /// Reveal the scope that denied the mention.
    RevealScope,
}

impl M5MentionResolverAction {
    /// Every mention action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenTarget,
        Self::ChooseCandidate,
        Self::EditMention,
        Self::RemoveMention,
        Self::RevealScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenTarget => "open_target",
            Self::ChooseCandidate => "choose_candidate",
            Self::EditMention => "edit_mention",
            Self::RemoveMention => "remove_mention",
            Self::RevealScope => "reveal_scope",
        }
    }
}

/// Controlled mention-row anatomy part the shared mention resolver surfaces. The parts in
/// [`M5MentionResolverAnatomyPart::MANDATORY`] are required on every mention row so the
/// typed token, resolution state, target identity, exact-target preview, and action row are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MentionResolverAnatomyPart {
    /// The typed `@`-mention token.
    MentionTokenCue,
    /// The derived resolution state.
    ResolutionStateCue,
    /// The stable target object identity.
    TargetIdentityCue,
    /// The exact-target preview.
    TargetPreviewCue,
    /// The preserved scope note.
    ScopeNoteCue,
    /// The candidate count.
    CandidateCountCue,
    /// The bounded action row (open / choose / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5MentionResolverAnatomyPart {
    /// Every mention anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MentionTokenCue,
        Self::ResolutionStateCue,
        Self::TargetIdentityCue,
        Self::TargetPreviewCue,
        Self::ScopeNoteCue,
        Self::CandidateCountCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The mention anatomy parts every mention row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::MentionTokenCue,
        Self::ResolutionStateCue,
        Self::TargetIdentityCue,
        Self::TargetPreviewCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MentionTokenCue => "mention_token_cue",
            Self::ResolutionStateCue => "resolution_state_cue",
            Self::TargetIdentityCue => "target_identity_cue",
            Self::TargetPreviewCue => "target_preview_cue",
            Self::ScopeNoteCue => "scope_note_cue",
            Self::CandidateCountCue => "candidate_count_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// The capability / authority class a slash command effects when it succeeds, reused from
/// the command graph so an AI-surfaced command advertises the same authority it does on the
/// palette, automation, and CLI surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandCapabilityClass {
    /// A read-only query with no side effects.
    ReadOnlyQuery,
    /// A scoped, reversible-local mutation.
    ScopedMutation,
    /// A repository-wide mutation.
    RepositoryMutation,
    /// A command with an externally visible side effect.
    ExternalSideEffect,
    /// A privileged administrative command.
    PrivilegedAdmin,
    /// A meta / help / navigation command.
    MetaHelp,
}

impl M5SlashCommandCapabilityClass {
    /// Every capability class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadOnlyQuery,
        Self::ScopedMutation,
        Self::RepositoryMutation,
        Self::ExternalSideEffect,
        Self::PrivilegedAdmin,
        Self::MetaHelp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyQuery => "read_only_query",
            Self::ScopedMutation => "scoped_mutation",
            Self::RepositoryMutation => "repository_mutation",
            Self::ExternalSideEffect => "external_side_effect",
            Self::PrivilegedAdmin => "privileged_admin",
            Self::MetaHelp => "meta_help",
        }
    }
}

/// The derived posture of a slash-command row — the resolver's verdict about whether a
/// command is plainly invocable, approval-gated, disabled with an explanation, deprecated
/// and redirecting, hidden by policy, or an unknown command. Computed in a fixed
/// blocking-first order so a disabled, gated, or hidden command never reads as a plain ready
/// action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandRowPosture {
    /// Ready and directly invocable.
    ReadyInvocable,
    /// Invocable but gated behind an approval.
    ApprovalGated,
    /// Disabled by an unmet precondition, with an explanation.
    DisabledExplained,
    /// Deprecated / aliased; invocable but redirects to the canonical command.
    DeprecatedRedirect,
    /// Hidden by policy; not invocable here.
    PolicyHidden,
    /// An unknown command; rejected.
    UnknownRejected,
}

impl M5SlashCommandRowPosture {
    /// Every row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadyInvocable,
        Self::ApprovalGated,
        Self::DisabledExplained,
        Self::DeprecatedRedirect,
        Self::PolicyHidden,
        Self::UnknownRejected,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyInvocable => "ready_invocable",
            Self::ApprovalGated => "approval_gated",
            Self::DisabledExplained => "disabled_explained",
            Self::DeprecatedRedirect => "deprecated_redirect",
            Self::PolicyHidden => "policy_hidden",
            Self::UnknownRejected => "unknown_rejected",
        }
    }

    /// True when the command can be selected / run (directly or after redirect).
    pub const fn is_invocable(self) -> bool {
        matches!(self, Self::ReadyInvocable | Self::DeprecatedRedirect)
    }

    /// True when the command cannot run at all from this surface.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::PolicyHidden | Self::UnknownRejected)
    }

    /// True when the row must carry a disabled-state explanation.
    pub const fn requires_disabled_reason(self) -> bool {
        matches!(self, Self::DisabledExplained | Self::PolicyHidden)
    }
}

/// One bounded action a slash-command row offers, so a row never hides its invoke /
/// request-approval affordances or its help / canonical / explain follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandRowAction {
    /// Invoke the command.
    Invoke,
    /// Request the approval that gates the command.
    RequestApproval,
    /// Open the command's help path.
    OpenHelp,
    /// View the canonical command a deprecated alias redirects to.
    ViewCanonical,
    /// Explain why the command is disabled or hidden.
    ExplainDisabled,
}

impl M5SlashCommandRowAction {
    /// Every slash-command action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Invoke,
        Self::RequestApproval,
        Self::OpenHelp,
        Self::ViewCanonical,
        Self::ExplainDisabled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Invoke => "invoke",
            Self::RequestApproval => "request_approval",
            Self::OpenHelp => "open_help",
            Self::ViewCanonical => "view_canonical",
            Self::ExplainDisabled => "explain_disabled",
        }
    }
}

/// Controlled slash-command-row anatomy part the shared row surfaces. The parts in
/// [`M5SlashCommandRowAnatomyPart::MANDATORY`] are required on every row so the command id,
/// capability class, state posture, help path, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandRowAnatomyPart {
    /// The stable command id.
    CommandIdCue,
    /// The display label.
    CommandLabelCue,
    /// The capability / authority class.
    CapabilityClassCue,
    /// The derived state posture.
    StatePostureCue,
    /// The disabled-state explanation.
    DisabledReasonCue,
    /// The help path.
    HelpPathCue,
    /// The bounded action row (invoke / request-approval / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5SlashCommandRowAnatomyPart {
    /// Every slash-command anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CommandIdCue,
        Self::CommandLabelCue,
        Self::CapabilityClassCue,
        Self::StatePostureCue,
        Self::DisabledReasonCue,
        Self::HelpPathCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The slash-command anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::CommandIdCue,
        Self::CapabilityClassCue,
        Self::StatePostureCue,
        Self::HelpPathCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandIdCue => "command_id_cue",
            Self::CommandLabelCue => "command_label_cue",
            Self::CapabilityClassCue => "capability_class_cue",
            Self::StatePostureCue => "state_posture_cue",
            Self::DisabledReasonCue => "disabled_reason_cue",
            Self::HelpPathCue => "help_path_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the mention export carries so mention-row truth is reconstructable. The fields
/// in [`M5MentionResolverExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MentionResolverExportField {
    /// The typed mention token.
    MentionToken,
    /// The derived resolution state.
    Resolution,
    /// The stable target object id.
    TargetObjectId,
    /// The exact-target preview label.
    TargetPreviewLabel,
    /// The preserved scope note.
    ScopeNote,
    /// Whether the mention is bound to a single target.
    IsBound,
    /// Whether the mention blocks or narrows send.
    BlocksSend,
    /// The bounded available actions.
    AvailableActions,
}

impl M5MentionResolverExportField {
    /// Every mention export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MentionToken,
        Self::Resolution,
        Self::TargetObjectId,
        Self::TargetPreviewLabel,
        Self::ScopeNote,
        Self::IsBound,
        Self::BlocksSend,
        Self::AvailableActions,
    ];

    /// The mention export fields every mention row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::MentionToken,
        Self::Resolution,
        Self::TargetObjectId,
        Self::ScopeNote,
        Self::IsBound,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MentionToken => "mention_token",
            Self::Resolution => "resolution",
            Self::TargetObjectId => "target_object_id",
            Self::TargetPreviewLabel => "target_preview_label",
            Self::ScopeNote => "scope_note",
            Self::IsBound => "is_bound",
            Self::BlocksSend => "blocks_send",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// A field the slash-command export carries so row truth is reconstructable. The fields in
/// [`M5SlashCommandRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SlashCommandRowExportField {
    /// The stable command id.
    CommandId,
    /// The capability class.
    CapabilityClass,
    /// The command-graph availability state.
    State,
    /// The derived row posture.
    RowPosture,
    /// Whether the command requires approval.
    RequiresApproval,
    /// Whether a disabled-state reason is present.
    DisabledReasonPresent,
    /// The help path.
    HelpPath,
    /// The bounded available actions.
    AvailableActions,
}

impl M5SlashCommandRowExportField {
    /// Every slash-command export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CommandId,
        Self::CapabilityClass,
        Self::State,
        Self::RowPosture,
        Self::RequiresApproval,
        Self::DisabledReasonPresent,
        Self::HelpPath,
        Self::AvailableActions,
    ];

    /// The slash-command export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CommandId,
        Self::CapabilityClass,
        Self::State,
        Self::RowPosture,
        Self::HelpPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::CapabilityClass => "capability_class",
            Self::State => "state",
            Self::RowPosture => "row_posture",
            Self::RequiresApproval => "requires_approval",
            Self::DisabledReasonPresent => "disabled_reason_present",
            Self::HelpPath => "help_path",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// True when a mention resolution binds to a single stable target.
pub const fn resolution_is_bound(resolution: M5MentionResolution) -> bool {
    matches!(
        resolution,
        M5MentionResolution::ResolvedUnique | M5MentionResolution::ResolvedPinned
    )
}

/// True when a mention resolution needs explicit review before it can proceed — ambiguous
/// or unresolved bindings never silently bind to the wrong target.
pub const fn resolution_needs_explicit_review(resolution: M5MentionResolution) -> bool {
    matches!(
        resolution,
        M5MentionResolution::AmbiguousCandidates | M5MentionResolution::UnresolvedMissing
    )
}

// ---- mention resolver ---------------------------------------------------

/// The full input to the mention resolver for one `@`-mention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionResolverResolutionInput {
    /// The opaque typed mention token (must be non-empty).
    pub mention_token: String,
    /// The opaque preserved scope note (must be non-empty).
    pub scope_note: String,
    /// How many candidates matched the mention.
    pub candidate_count: usize,
    /// True when an exact stable object is available and preferred.
    pub has_exact_stable_target: bool,
    /// True when the preferred exact target is a pinned object.
    pub target_is_pinned: bool,
    /// The opaque stable id of the bound target, when the mention binds.
    pub target_object_id: Option<String>,
    /// The opaque exact-target preview label, when the mention binds.
    pub target_preview_label: Option<String>,
    /// True when the mention is within the composer scope.
    pub in_scope: bool,
    /// True when resolution is deferred pending.
    pub deferred: bool,
}

/// The resolved mention-row truth for one `@`-mention.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMentionResolver {
    /// The opaque typed mention token.
    pub mention_token: String,
    /// The opaque preserved scope note, carried through exactly.
    pub scope_note: String,
    /// How many candidates matched the mention.
    pub candidate_count: usize,
    /// The derived resolution state.
    pub resolution: M5MentionResolution,
    /// The opaque stable id of the bound target, when the mention binds.
    pub target_object_id: Option<String>,
    /// The opaque exact-target preview label, when the mention binds.
    pub target_preview_label: Option<String>,
    /// The bounded actions this mention row offers.
    pub available_actions: Vec<M5MentionResolverAction>,
    /// True when the mention binds to a single target.
    pub is_bound: bool,
    /// True when the mention blocks or narrows send.
    pub blocks_send: bool,
    /// True when the mention needs explicit review before send.
    pub needs_explicit_review: bool,
    /// True when a bound mention preserves its exact-target preview.
    pub preserves_exact_target_preview: bool,
}

/// Errors returned by [`resolve_mention_resolver`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MentionResolverResolutionError {
    /// The mention token was empty.
    EmptyMentionToken,
    /// The scope note was empty.
    EmptyScopeNote,
    /// A bound mention did not carry both its target id and exact-target preview.
    BoundMentionWithoutTarget,
    /// A mention descriptor carried forbidden material.
    ForbiddenMentionMaterial,
}

impl M5MentionResolverResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyMentionToken => "empty_mention_token",
            Self::EmptyScopeNote => "empty_scope_note",
            Self::BoundMentionWithoutTarget => "bound_mention_without_target",
            Self::ForbiddenMentionMaterial => "forbidden_mention_material",
        }
    }
}

impl fmt::Display for M5MentionResolverResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "mention resolver resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MentionResolverResolutionError {}

/// Resolves one `@`-mention from its declared candidate signals.
///
/// The derived resolution is computed in a fixed order that prefers exact stable objects and
/// never silently binds an ambiguous or unresolved mention: an out-of-scope mention is denied
/// first, then a mention with no candidates reads as unresolved-missing, then a deferred
/// mention reads as deferred-pending, then an exact stable target binds (pinned or unique),
/// then a single remaining candidate binds as unique, and otherwise the mention reads as
/// ambiguous and blocks send with an explicit choose-candidate review. A bound mention must
/// carry its exact-target preview, the scope note is always preserved, and every mention row
/// always offers a remove action.
pub fn resolve_mention_resolver(
    input: &M5MentionResolverResolutionInput,
) -> Result<M5ResolvedMentionResolver, M5MentionResolverResolutionError> {
    if input.mention_token.trim().is_empty() {
        return Err(M5MentionResolverResolutionError::EmptyMentionToken);
    }
    if input.scope_note.trim().is_empty() {
        return Err(M5MentionResolverResolutionError::EmptyScopeNote);
    }
    if value_repr_is_forbidden(&input.mention_token)
        || value_repr_is_forbidden(&input.scope_note)
        || input
            .target_object_id
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
        || input
            .target_preview_label
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
    {
        return Err(M5MentionResolverResolutionError::ForbiddenMentionMaterial);
    }

    let resolution = derive_mention_resolution(
        input.in_scope,
        input.candidate_count,
        input.has_exact_stable_target,
        input.target_is_pinned,
        input.deferred,
    );
    let is_bound = resolution_is_bound(resolution);
    if is_bound && (input.target_object_id.is_none() || input.target_preview_label.is_none()) {
        return Err(M5MentionResolverResolutionError::BoundMentionWithoutTarget);
    }

    let available_actions = derive_mention_actions(resolution, is_bound);

    Ok(M5ResolvedMentionResolver {
        mention_token: input.mention_token.clone(),
        scope_note: input.scope_note.clone(),
        candidate_count: input.candidate_count,
        resolution,
        target_object_id: input.target_object_id.clone(),
        target_preview_label: input.target_preview_label.clone(),
        available_actions,
        is_bound,
        blocks_send: !is_bound,
        needs_explicit_review: resolution_needs_explicit_review(resolution),
        preserves_exact_target_preview: is_bound && input.target_preview_label.is_some(),
    })
}

/// The fixed exact-stable-first mention-resolution ladder.
fn derive_mention_resolution(
    in_scope: bool,
    candidate_count: usize,
    has_exact_stable_target: bool,
    target_is_pinned: bool,
    deferred: bool,
) -> M5MentionResolution {
    if !in_scope {
        M5MentionResolution::OutOfScopeDenied
    } else if candidate_count == 0 {
        M5MentionResolution::UnresolvedMissing
    } else if deferred {
        M5MentionResolution::DeferredPending
    } else if has_exact_stable_target {
        if target_is_pinned {
            M5MentionResolution::ResolvedPinned
        } else {
            M5MentionResolution::ResolvedUnique
        }
    } else if candidate_count == 1 {
        M5MentionResolution::ResolvedUnique
    } else {
        M5MentionResolution::AmbiguousCandidates
    }
}

/// Derives the bounded mention-action set from the resolution and bound signals.
///
/// Remove is always offered so a mention can be dropped before send; open-target is offered
/// only when the mention binds; choose-candidate, edit-mention, and reveal-scope follow the
/// resolution.
fn derive_mention_actions(
    resolution: M5MentionResolution,
    is_bound: bool,
) -> Vec<M5MentionResolverAction> {
    use M5MentionResolution as Res;
    use M5MentionResolverAction as Action;
    let mut actions = Vec::new();
    if is_bound {
        actions.push(Action::OpenTarget);
    }
    if matches!(resolution, Res::AmbiguousCandidates) {
        actions.push(Action::ChooseCandidate);
    }
    if matches!(
        resolution,
        Res::AmbiguousCandidates | Res::UnresolvedMissing | Res::DeferredPending
    ) {
        actions.push(Action::EditMention);
    }
    actions.push(Action::RemoveMention);
    if matches!(resolution, Res::OutOfScopeDenied) {
        actions.push(Action::RevealScope);
    }
    actions
}

// ---- slash-command-row resolver -----------------------------------------

/// The full input to the slash-command-row resolver for one command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SlashCommandRowResolutionInput {
    /// The opaque stable command id from the shared command graph (must be non-empty).
    pub command_id: String,
    /// The opaque display label (must be non-empty).
    pub command_label: String,
    /// The capability / authority class the command effects.
    pub capability_class: M5SlashCommandCapabilityClass,
    /// The opaque help path (must be non-empty).
    pub help_path: String,
    /// The command-graph availability state.
    pub state: M5SlashCommandState,
    /// True when the command requires approval before it runs.
    pub requires_approval: bool,
    /// The opaque disabled-state explanation, when disabled or hidden.
    pub disabled_reason: Option<String>,
    /// The opaque canonical command id a deprecated alias redirects to.
    pub alias_of: Option<String>,
}

/// The resolved slash-command-row truth for one command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSlashCommandRow {
    /// The opaque stable command id, preserved exactly from the input.
    pub command_id: String,
    /// The opaque display label.
    pub command_label: String,
    /// The capability / authority class.
    pub capability_class: M5SlashCommandCapabilityClass,
    /// The opaque help path.
    pub help_path: String,
    /// The command-graph availability state.
    pub state: M5SlashCommandState,
    /// True when the command requires approval before it runs.
    pub requires_approval: bool,
    /// The derived row posture.
    pub row_posture: M5SlashCommandRowPosture,
    /// The opaque disabled-state explanation, when disabled or hidden.
    pub disabled_reason: Option<String>,
    /// The opaque canonical command id a deprecated alias redirects to.
    pub alias_of: Option<String>,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5SlashCommandRowAction>,
    /// True when the command can be selected / run.
    pub is_invocable: bool,
    /// True when the command cannot run at all from this surface.
    pub is_blocked: bool,
    /// True when the command requires approval before it runs.
    pub requires_approval_before_run: bool,
    /// True when a disabled row carries its explanation.
    pub explains_disabled_state: bool,
}

/// Errors returned by [`resolve_slash_command_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SlashCommandRowResolutionError {
    /// The command id was empty.
    EmptyCommandId,
    /// The command label was empty.
    EmptyCommandLabel,
    /// The help path was empty.
    EmptyHelpPath,
    /// A disabled or policy-hidden row did not carry a disabled-state explanation.
    DisabledWithoutExplanation,
    /// A deprecated / aliased command did not name its canonical target.
    DeprecatedWithoutCanonicalTarget,
    /// A command descriptor carried forbidden material.
    ForbiddenCommandMaterial,
}

impl M5SlashCommandRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyCommandId => "empty_command_id",
            Self::EmptyCommandLabel => "empty_command_label",
            Self::EmptyHelpPath => "empty_help_path",
            Self::DisabledWithoutExplanation => "disabled_without_explanation",
            Self::DeprecatedWithoutCanonicalTarget => "deprecated_without_canonical_target",
            Self::ForbiddenCommandMaterial => "forbidden_command_material",
        }
    }
}

impl fmt::Display for M5SlashCommandRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "slash command row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SlashCommandRowResolutionError {}

/// Resolves one slash-command row from its declared command-graph state.
///
/// The derived row posture is computed in a fixed blocking-first order: an unknown command is
/// rejected first, then a policy-hidden command, then a disabled-with-unmet-precondition
/// command reads as disabled-explained, then an approval requirement (declared or raised by
/// the `requires_approval` flag) gates the command, then a deprecated alias redirects, and
/// otherwise the command reads as ready-invocable. A disabled or policy-hidden row must carry
/// its explanation, a deprecated alias must name its canonical target, the command id,
/// capability class, and help path are carried explicitly, and the row always offers an
/// open-help action — so a disabled, gated, or hidden command never reads as a plain ready
/// action.
pub fn resolve_slash_command_row(
    input: &M5SlashCommandRowResolutionInput,
) -> Result<M5ResolvedSlashCommandRow, M5SlashCommandRowResolutionError> {
    if input.command_id.trim().is_empty() {
        return Err(M5SlashCommandRowResolutionError::EmptyCommandId);
    }
    if input.command_label.trim().is_empty() {
        return Err(M5SlashCommandRowResolutionError::EmptyCommandLabel);
    }
    if input.help_path.trim().is_empty() {
        return Err(M5SlashCommandRowResolutionError::EmptyHelpPath);
    }
    if value_repr_is_forbidden(&input.command_id)
        || value_repr_is_forbidden(&input.command_label)
        || value_repr_is_forbidden(&input.help_path)
        || input
            .disabled_reason
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
        || input
            .alias_of
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
    {
        return Err(M5SlashCommandRowResolutionError::ForbiddenCommandMaterial);
    }

    let row_posture = derive_slash_command_posture(input.state, input.requires_approval);
    if row_posture.requires_disabled_reason() && input.disabled_reason.is_none() {
        return Err(M5SlashCommandRowResolutionError::DisabledWithoutExplanation);
    }
    if matches!(input.state, M5SlashCommandState::DeprecatedAliased) && input.alias_of.is_none() {
        return Err(M5SlashCommandRowResolutionError::DeprecatedWithoutCanonicalTarget);
    }

    let available_actions = derive_slash_actions(row_posture);

    Ok(M5ResolvedSlashCommandRow {
        command_id: input.command_id.clone(),
        command_label: input.command_label.clone(),
        capability_class: input.capability_class,
        help_path: input.help_path.clone(),
        state: input.state,
        requires_approval: input.requires_approval,
        row_posture,
        disabled_reason: input.disabled_reason.clone(),
        alias_of: input.alias_of.clone(),
        available_actions,
        is_invocable: row_posture.is_invocable(),
        is_blocked: row_posture.is_blocked(),
        requires_approval_before_run: matches!(
            row_posture,
            M5SlashCommandRowPosture::ApprovalGated
        ),
        explains_disabled_state: row_posture.requires_disabled_reason()
            && input.disabled_reason.is_some(),
    })
}

/// The fixed blocking-first slash-command-row-posture ladder. An approval requirement raises
/// an otherwise-available (or deprecated) command to the approval-gated posture.
fn derive_slash_command_posture(
    state: M5SlashCommandState,
    requires_approval: bool,
) -> M5SlashCommandRowPosture {
    match state {
        M5SlashCommandState::UnknownCommand => M5SlashCommandRowPosture::UnknownRejected,
        M5SlashCommandState::PolicyHidden => M5SlashCommandRowPosture::PolicyHidden,
        M5SlashCommandState::DisabledUnmetPrecondition => {
            M5SlashCommandRowPosture::DisabledExplained
        }
        M5SlashCommandState::RequiresApproval => M5SlashCommandRowPosture::ApprovalGated,
        M5SlashCommandState::DeprecatedAliased => {
            if requires_approval {
                M5SlashCommandRowPosture::ApprovalGated
            } else {
                M5SlashCommandRowPosture::DeprecatedRedirect
            }
        }
        M5SlashCommandState::Available => {
            if requires_approval {
                M5SlashCommandRowPosture::ApprovalGated
            } else {
                M5SlashCommandRowPosture::ReadyInvocable
            }
        }
    }
}

/// Derives the bounded slash-command-action set from the row posture.
///
/// Open-help is always offered so a command's help path is never hidden; invoke is offered
/// only when the command is invocable; request-approval, view-canonical, and explain-disabled
/// follow the posture.
fn derive_slash_actions(posture: M5SlashCommandRowPosture) -> Vec<M5SlashCommandRowAction> {
    use M5SlashCommandRowAction as Action;
    let mut actions = Vec::new();
    if posture.is_invocable() {
        actions.push(Action::Invoke);
    }
    if matches!(posture, M5SlashCommandRowPosture::ApprovalGated) {
        actions.push(Action::RequestApproval);
    }
    actions.push(Action::OpenHelp);
    if matches!(posture, M5SlashCommandRowPosture::DeprecatedRedirect) {
        actions.push(Action::ViewCanonical);
    }
    if matches!(
        posture,
        M5SlashCommandRowPosture::DisabledExplained
            | M5SlashCommandRowPosture::PolicyHidden
            | M5SlashCommandRowPosture::UnknownRejected
    ) {
        actions.push(Action::ExplainDisabled);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked mention resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionResolverResolutionCase {
    /// The resolver input.
    pub input: M5MentionResolverResolutionInput,
    /// The resolved truth. Must equal `resolve_mention_resolver(&input)`.
    pub resolved: M5ResolvedMentionResolver,
}

impl M5MentionResolverResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5MentionResolverResolutionInput) -> Self {
        let resolved = resolve_mention_resolver(&input).expect("seed mention case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_mention_resolver(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved mention token preserves the input token exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.mention_token == self.input.mention_token
            && self.resolved.scope_note == self.input.scope_note
    }
}

/// One worked slash-command-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SlashCommandRowResolutionCase {
    /// The resolver input.
    pub input: M5SlashCommandRowResolutionInput,
    /// The resolved truth. Must equal `resolve_slash_command_row(&input)`.
    pub resolved: M5ResolvedSlashCommandRow,
}

impl M5SlashCommandRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SlashCommandRowResolutionInput) -> Self {
        let resolved = resolve_slash_command_row(&input).expect("seed slash command case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_slash_command_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved command id preserves the input id exactly.
    pub fn preserves_command_id(&self) -> bool {
        self.resolved.command_id == self.input.command_id
    }
}

/// One row in the primitive matrix: one composer consumer bound to the shared mention and
/// command anatomy, mention resolutions, slash-command states, capability classes, row
/// postures, bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandRow {
    /// Composer consumer family.
    pub consumer_surface: M5MentionSlashCommandConsumerSurface,
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
    /// Mention anatomy parts this row renders (must include the mandatory parts).
    pub mention_anatomy_parts: Vec<M5MentionResolverAnatomyPart>,
    /// Slash-command anatomy parts this row renders (must include the mandatory parts).
    pub slash_anatomy_parts: Vec<M5SlashCommandRowAnatomyPart>,
    /// Mention resolutions this consumer distinguishes.
    pub mention_resolutions: Vec<M5MentionResolution>,
    /// Bounded mention actions this consumer offers.
    pub mention_actions: Vec<M5MentionResolverAction>,
    /// Slash-command states this consumer distinguishes.
    pub slash_command_states: Vec<M5SlashCommandState>,
    /// Capability classes this consumer distinguishes.
    pub capability_classes: Vec<M5SlashCommandCapabilityClass>,
    /// Slash-command row postures this consumer distinguishes.
    pub slash_row_postures: Vec<M5SlashCommandRowPosture>,
    /// Bounded slash-command actions this consumer offers.
    pub slash_actions: Vec<M5SlashCommandRowAction>,
    /// Mention export fields this row carries (must include the mandatory fields).
    pub mention_export_fields: Vec<M5MentionResolverExportField>,
    /// Slash-command export fields this row carries (must include the mandatory fields).
    pub slash_export_fields: Vec<M5SlashCommandRowExportField>,
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
    /// Worked mention resolutions proving the mention resolver on this consumer.
    pub mention_examples: Vec<M5MentionResolverResolutionCase>,
    /// Worked slash-command resolutions proving the slash resolver on this consumer.
    pub slash_examples: Vec<M5SlashCommandRowResolutionCase>,
    /// Hard invariant: this consumer never masks a command's identity or capability. MUST be
    /// `false`.
    pub masks_command_identity_or_capability: bool,
    /// Hard invariant: this consumer never hides a mention's resolution or ambiguity. MUST be
    /// `false`.
    pub hides_mention_resolution_or_ambiguity: bool,
    /// Hard invariant: this consumer never invents a private command grammar. MUST be
    /// `false`.
    pub invents_private_command_grammar: bool,
    /// Hard invariant: this consumer never bypasses the ambiguity or approval gate. MUST be
    /// `false`.
    pub bypasses_ambiguity_or_approval_gate: bool,
}

impl M5MentionSlashCommandRow {
    /// True when the row declares every mandatory mention anatomy part.
    fn declares_mandatory_mention_anatomy(&self) -> bool {
        let present: BTreeSet<M5MentionResolverAnatomyPart> =
            self.mention_anatomy_parts.iter().copied().collect();
        M5MentionResolverAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory slash-command anatomy part.
    fn declares_mandatory_slash_anatomy(&self) -> bool {
        let present: BTreeSet<M5SlashCommandRowAnatomyPart> =
            self.slash_anatomy_parts.iter().copied().collect();
        M5SlashCommandRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory mention export field.
    fn declares_mandatory_mention_export(&self) -> bool {
        let present: BTreeSet<M5MentionResolverExportField> =
            self.mention_export_fields.iter().copied().collect();
        M5MentionResolverExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory slash-command export field.
    fn declares_mandatory_slash_export(&self) -> bool {
        let present: BTreeSet<M5SlashCommandRowExportField> =
            self.slash_export_fields.iter().copied().collect();
        M5SlashCommandRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_command_identity_or_capability
            && !self.hides_mention_resolution_or_ambiguity
            && !self.invents_private_command_grammar
            && !self.bypasses_ambiguity_or_approval_gate
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandVocabularySet {
    /// Composer-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Mention-anatomy-part tokens.
    pub mention_anatomy_parts: Vec<String>,
    /// Slash-anatomy-part tokens.
    pub slash_anatomy_parts: Vec<String>,
    /// Mention-resolution tokens (reused from the frozen matrix).
    pub mention_resolutions: Vec<String>,
    /// Mention-action tokens.
    pub mention_actions: Vec<String>,
    /// Slash-command-state tokens (reused from the frozen matrix).
    pub slash_command_states: Vec<String>,
    /// Capability-class tokens.
    pub capability_classes: Vec<String>,
    /// Slash-command row-posture tokens.
    pub slash_row_postures: Vec<String>,
    /// Slash-command-action tokens.
    pub slash_actions: Vec<String>,
    /// Mention-export-field tokens.
    pub mention_export_fields: Vec<String>,
    /// Slash-export-field tokens.
    pub slash_export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5MentionSlashCommandVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5MentionSlashCommandConsumerSurface::ALL, |v| v.as_str()),
            mention_anatomy_parts: tokens(&M5MentionResolverAnatomyPart::ALL, |v| v.as_str()),
            slash_anatomy_parts: tokens(&M5SlashCommandRowAnatomyPart::ALL, |v| v.as_str()),
            mention_resolutions: tokens(&M5MentionResolution::ALL, |v| v.as_str()),
            mention_actions: tokens(&M5MentionResolverAction::ALL, |v| v.as_str()),
            slash_command_states: tokens(&M5SlashCommandState::ALL, |v| v.as_str()),
            capability_classes: tokens(&M5SlashCommandCapabilityClass::ALL, |v| v.as_str()),
            slash_row_postures: tokens(&M5SlashCommandRowPosture::ALL, |v| v.as_str()),
            slash_actions: tokens(&M5SlashCommandRowAction::ALL, |v| v.as_str()),
            mention_export_fields: tokens(&M5MentionResolverExportField::ALL, |v| v.as_str()),
            slash_export_fields: tokens(&M5SlashCommandRowExportField::ALL, |v| v.as_str()),
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
pub struct M5MentionSlashCommandGovernanceReview {
    /// One primitive pair carries mention and command truth on every consumer.
    pub one_primitive_carries_mention_and_command_truth: bool,
    /// The mention resolver prefers exact stable objects.
    pub mention_prefers_exact_stable_objects: bool,
    /// An ambiguous binding blocks or narrows send with explicit review.
    pub ambiguous_binding_blocks_send_with_review: bool,
    /// An unresolved binding never silently binds to the wrong target.
    pub unresolved_binding_never_silently_bound: bool,
    /// A mention's scope note is always preserved.
    pub mention_scope_note_always_preserved: bool,
    /// A bound mention's exact-target preview is always shown.
    pub exact_target_preview_always_shown: bool,
    /// Slash-command rows reuse stable command ids from the command graph.
    pub slash_reuses_stable_command_ids: bool,
    /// A disabled state is always explained.
    pub disabled_state_always_explained: bool,
    /// Approval semantics match the command graph.
    pub approval_semantics_match_command_graph: bool,
    /// Availability, authority, and disabled reasons match the non-AI surfaces.
    pub availability_matches_non_ai_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandConsumerProjection {
    /// Inline, palette, automation, CLI / headless, and support consumers all consume the
    /// shared primitive pair.
    pub composition_and_palette_surfaces_consume_shared_primitive: bool,
    /// The mention-resolution derivation reads a single canonical source.
    pub mention_resolution_reads_single_source: bool,
    /// The slash-command-posture derivation reads a single canonical source.
    pub slash_command_posture_reads_single_source: bool,
    /// The command-graph metadata reads a single canonical source.
    pub command_graph_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MentionSlashCommandPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MentionSlashCommandPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5MentionSlashCommandRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MentionSlashCommandVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MentionSlashCommandGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MentionSlashCommandConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MentionSlashCommandProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MentionSlashCommandReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 mention-resolver / slash-command-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MentionSlashCommandPacket {
    /// Record kind; must equal [`M5_MENTION_SLASH_COMMAND_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MENTION_SLASH_COMMAND_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5MentionSlashCommandRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MentionSlashCommandVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MentionSlashCommandGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MentionSlashCommandConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MentionSlashCommandProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MentionSlashCommandReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MentionSlashCommandPacket {
    /// Builds an M5 mention/command-primitive packet from stable-lane input.
    pub fn new(input: M5MentionSlashCommandPacketInput) -> Self {
        Self {
            record_kind: M5_MENTION_SLASH_COMMAND_RECORD_KIND.to_owned(),
            schema_version: M5_MENTION_SLASH_COMMAND_SCHEMA_VERSION,
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

    /// Validates the M5 mention/command-primitive invariants.
    pub fn validate(&self) -> Vec<M5MentionSlashCommandViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MENTION_SLASH_COMMAND_RECORD_KIND {
            violations.push(M5MentionSlashCommandViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MENTION_SLASH_COMMAND_SCHEMA_VERSION {
            violations.push(M5MentionSlashCommandViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MentionSlashCommandViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_mention_bind_coverage(self, &mut violations);
        validate_mention_ambiguity_review(self, &mut violations);
        validate_mention_target_preview(self, &mut violations);
        validate_slash_disabled_explanation(self, &mut violations);
        validate_slash_approval_availability_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 mention/command primitive packet serializes"),
        ) {
            violations.push(M5MentionSlashCommandViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 mention/command primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per composer consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,mention_anatomy,slash_anatomy,mention_resolutions,slash_states,capability_classes,slash_postures,mention_actions,slash_actions,mention_examples,slash_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.mention_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.slash_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.mention_resolutions, |v| v.as_str()),
                join_tokens(&row.slash_command_states, |v| v.as_str()),
                join_tokens(&row.capability_classes, |v| v.as_str()),
                join_tokens(&row.slash_row_postures, |v| v.as_str()),
                join_tokens(&row.mention_actions, |v| v.as_str()),
                join_tokens(&row.slash_actions, |v| v.as_str()),
                row.mention_examples.len(),
                row.slash_examples.len(),
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
        out.push_str("# M5 Mention-Resolver and Slash-Command-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Composer consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Mention resolutions: {}\n",
            self.vocabulary_set.mention_resolutions.join(", ")
        ));
        out.push_str(&format!(
            "- Slash row postures: {}\n",
            self.vocabulary_set.slash_row_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Capability classes: {}\n",
            self.vocabulary_set.capability_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Composer consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked mentions: {}\n",
                row.mention_examples.len()
            ));
            for case in &row.mention_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (bound `{}`, blocks send `{}`, review `{}`)\n",
                    case.resolved.mention_token,
                    case.resolved.resolution.as_str(),
                    case.resolved.is_bound,
                    case.resolved.blocks_send,
                    case.resolved.needs_explicit_review,
                ));
            }
            out.push_str(&format!(
                "  - Worked commands: {}\n",
                row.slash_examples.len()
            ));
            for case in &row.slash_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (invocable `{}`, approval `{}`)\n",
                    case.resolved.command_id,
                    case.resolved.capability_class.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.is_invocable,
                    case.resolved.requires_approval_before_run,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 mention/command-primitive export.
#[derive(Debug)]
pub enum M5MentionSlashCommandArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MentionSlashCommandViolation>),
}

impl fmt::Display for M5MentionSlashCommandArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 mention/command primitive export parse failed: {error}"
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
                    "m5 mention/command primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MentionSlashCommandArtifactError {}

/// Validation failures emitted by [`M5MentionSlashCommandPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MentionSlashCommandViolation {
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
    /// A required composer consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A composer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory mention anatomy parts.
    MandatoryMentionAnatomyMissing,
    /// A row omits one of the mandatory slash-command anatomy parts.
    MandatorySlashAnatomyMissing,
    /// A row omits one of the mandatory mention export fields.
    MandatoryMentionExportMissing,
    /// A row omits one of the mandatory slash-command export fields.
    MandatorySlashExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked mention resolutions.
    MentionExampleMissing,
    /// A row declares no worked slash-command resolutions.
    SlashExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked mention resolution proves both a bound and a blocked mention.
    MentionBindCoverageUnproven,
    /// No worked mention resolution proves an ambiguous binding that blocks send with review.
    MentionAmbiguityReviewUnproven,
    /// A bound worked mention resolution does not preserve its exact-target preview.
    MentionTargetPreviewUnproven,
    /// No worked slash-command resolution proves a disabled row that carries its explanation.
    SlashDisabledExplanationUnproven,
    /// No worked slash-command resolution proves both an approval-gated and a ready command.
    SlashApprovalAvailabilityCoverageUnproven,
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

impl M5MentionSlashCommandViolation {
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
            Self::MandatoryMentionAnatomyMissing => "mandatory_mention_anatomy_missing",
            Self::MandatorySlashAnatomyMissing => "mandatory_slash_anatomy_missing",
            Self::MandatoryMentionExportMissing => "mandatory_mention_export_missing",
            Self::MandatorySlashExportMissing => "mandatory_slash_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::MentionExampleMissing => "mention_example_missing",
            Self::SlashExampleMissing => "slash_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::MentionBindCoverageUnproven => "mention_bind_coverage_unproven",
            Self::MentionAmbiguityReviewUnproven => "mention_ambiguity_review_unproven",
            Self::MentionTargetPreviewUnproven => "mention_target_preview_unproven",
            Self::SlashDisabledExplanationUnproven => "slash_disabled_explanation_unproven",
            Self::SlashApprovalAvailabilityCoverageUnproven => {
                "slash_approval_availability_coverage_unproven"
            }
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 mention/command-primitive export.
pub fn current_stable_m5_mention_slash_command_export(
) -> Result<M5MentionSlashCommandPacket, M5MentionSlashCommandArtifactError> {
    let packet: M5MentionSlashCommandPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces/support_export.json"
    )))
    .map_err(M5MentionSlashCommandArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MentionSlashCommandArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
        M5_MENTION_SLASH_COMMAND_DOC_REF,
        M5_MENTION_SLASH_COMMAND_COMPONENT_MATRIX_REF,
        M5_MENTION_SLASH_COMMAND_COMMAND_GRAPH_REF,
        M5_MENTION_SLASH_COMMAND_MENTION_PROVENANCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MentionSlashCommandViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MentionSlashCommandViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let present: BTreeSet<M5MentionSlashCommandConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5MentionSlashCommandConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5MentionSlashCommandViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.mention_anatomy_parts.is_empty()
            || row.slash_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.mention_resolutions.is_empty()
            || row.mention_actions.is_empty()
            || row.slash_command_states.is_empty()
            || row.capability_classes.is_empty()
            || row.slash_row_postures.is_empty()
            || row.slash_actions.is_empty()
        {
            violations.push(M5MentionSlashCommandViolation::RowIncomplete);
        }
        if !row.declares_mandatory_mention_anatomy() {
            violations.push(M5MentionSlashCommandViolation::MandatoryMentionAnatomyMissing);
        }
        if !row.declares_mandatory_slash_anatomy() {
            violations.push(M5MentionSlashCommandViolation::MandatorySlashAnatomyMissing);
        }
        if !row.declares_mandatory_mention_export() {
            violations.push(M5MentionSlashCommandViolation::MandatoryMentionExportMissing);
        }
        if !row.declares_mandatory_slash_export() {
            violations.push(M5MentionSlashCommandViolation::MandatorySlashExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5MentionSlashCommandViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MentionSlashCommandViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MentionSlashCommandViolation::DowngradeTriggersMissing);
        }
        if row.mention_examples.is_empty() {
            violations.push(M5MentionSlashCommandViolation::MentionExampleMissing);
        }
        if row.slash_examples.is_empty() {
            violations.push(M5MentionSlashCommandViolation::SlashExampleMissing);
        }
        if row
            .mention_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .slash_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5MentionSlashCommandViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5MentionSlashCommandViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5MentionSlashCommandViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked mention resolution across the matrix must prove a clean bind and at
/// least one must prove a blocked (ambiguous / unresolved / out-of-scope / deferred) mention
/// — the acceptance-criterion example that a mention binds to an exact stable object but
/// never silently binds an unresolved one.
fn validate_mention_bind_coverage(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let has_bound = packet.rows.iter().any(|row| {
        row.mention_examples
            .iter()
            .any(|case| case.resolved.is_bound)
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.mention_examples
            .iter()
            .any(|case| case.resolved.blocks_send)
    });
    if !(has_bound && has_blocked) {
        violations.push(M5MentionSlashCommandViolation::MentionBindCoverageUnproven);
    }
}

/// At least one worked mention resolution must prove an ambiguous binding that blocks send
/// and needs explicit review — the acceptance-criterion example that ambiguity blocks or
/// narrows send with explicit review instead of silently binding to the wrong target.
fn validate_mention_ambiguity_review(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.mention_examples.iter().any(|case| {
            matches!(
                case.resolved.resolution,
                M5MentionResolution::AmbiguousCandidates
            ) && case.resolved.needs_explicit_review
                && case.resolved.blocks_send
        })
    });
    if !proven {
        violations.push(M5MentionSlashCommandViolation::MentionAmbiguityReviewUnproven);
    }
}

/// Every bound worked mention resolution must preserve its exact-target preview — the
/// acceptance-criterion example that a resolved mention shows its exact target before send.
fn validate_mention_target_preview(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.mention_examples.iter())
        .filter(|case| case.resolved.is_bound)
        .all(|case| case.resolved.preserves_exact_target_preview);
    if !preserved {
        violations.push(M5MentionSlashCommandViolation::MentionTargetPreviewUnproven);
    }
}

/// At least one worked slash-command resolution must prove a disabled row that carries its
/// explanation — the acceptance-criterion example that a disabled command names its reason
/// rather than reading as a plain ready action.
fn validate_slash_disabled_explanation(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.slash_examples.iter().any(|case| {
            matches!(
                case.resolved.row_posture,
                M5SlashCommandRowPosture::DisabledExplained
            ) && case.resolved.explains_disabled_state
        })
    });
    if !proven {
        violations.push(M5MentionSlashCommandViolation::SlashDisabledExplanationUnproven);
    }
}

/// At least one worked slash-command resolution must prove an approval-gated command and at
/// least one must prove a ready-invocable command — the acceptance-criterion example that
/// approval semantics and availability match the shared command graph.
fn validate_slash_approval_availability_coverage(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let has_approval = packet.rows.iter().any(|row| {
        row.slash_examples
            .iter()
            .any(|case| case.resolved.requires_approval_before_run)
    });
    let has_ready = packet.rows.iter().any(|row| {
        row.slash_examples.iter().any(|case| {
            matches!(
                case.resolved.row_posture,
                M5SlashCommandRowPosture::ReadyInvocable
            )
        })
    });
    if !(has_approval && has_ready) {
        violations.push(M5MentionSlashCommandViolation::SlashApprovalAvailabilityCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_mention_and_command_truth,
        review.mention_prefers_exact_stable_objects,
        review.ambiguous_binding_blocks_send_with_review,
        review.unresolved_binding_never_silently_bound,
        review.mention_scope_note_always_preserved,
        review.exact_target_preview_always_shown,
        review.slash_reuses_stable_command_ids,
        review.disabled_state_always_explained,
        review.approval_semantics_match_command_graph,
        review.availability_matches_non_ai_surfaces,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5MentionSlashCommandViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.composition_and_palette_surfaces_consume_shared_primitive,
        projection.mention_resolution_reads_single_source,
        projection.slash_command_posture_reads_single_source,
        projection.command_graph_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5MentionSlashCommandViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MentionSlashCommandViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MentionSlashCommandPacket,
    violations: &mut Vec<M5MentionSlashCommandViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MentionSlashCommandViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
