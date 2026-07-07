//! Keyboard / screen-reader / CLI / export parity and honest automatic narrowing for the
//! M5 prompt-composer components.
//!
//! This module is the M05-890 accessibility-and-auto-narrowing capstone over the frozen
//! M5 prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`]).
//! Where the freeze matrix defines the reusable prompt-composer header, context-attachment
//! pill, mention resolver, slash-command row, budget / size strip, tainted-context
//! warning, draft-state row, attachment-stale banner, and split-send / review control
//! primitives, and the 885-888 implementation lanes resolve their per-surface truth, this
//! lane certifies — per component family — that pre-send composition claims stay
//! **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and self-narrowing**
//! rather than presenting an unresolved mention, a stale attachment, an over-budget
//! composition, an offline-local-only draft, or a policy-blocked route as a still
//! send-ready composition:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same composer mode,
//!   scope, route / provider / model, attached object identity, mention resolution,
//!   slash-command availability, budget headroom / omitted-context class, taint source /
//!   severity, draft locality / retention, attachment freshness, and send / review gate
//!   the rich composer shows — never a hover-only pill or chip that strands assistive-tech
//!   or headless users. Hierarchy-heavy families (the budget / size strip's omitted-context
//!   drawer with its nested included / omitted context sub-rows) additionally bind their
//!   tree to a flat list / textual path.
//! - **Export parity.** The support / release / evaluation export reconstructs each
//!   component's meaning from typed tokens and opaque refs without a screenshot, preserving
//!   the same mode, route, attachment identity, taint, budget, and send-gate truth shown
//!   in-product so pre-send composition can be reconstructed without private team memory.
//! - **Honest auto-narrowing.** When a mention is unresolved, an attachment goes stale,
//!   composition overflows the budget, a draft is offline-local-only, or a route is
//!   policy-blocked, the component's composer-support claim auto-narrows from `ReadyToSend`
//!   / `ReviewableComposition` to a narrowed / local-only / unresolved / policy-blocked
//!   composition, discloses the narrowing with a precise trigger and binding dimension, and
//!   preserves the canonical composer / attachment / mention / route / draft identity —
//!   the draft is never dropped opaquely. A component with every dimension intact must NOT
//!   carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the inline
//!   composer, composer panel, patch review, branch-agent console, help composer, companion
//!   composer, headless CLI, and support / release exports so product, docs, and release
//!   publication stay aligned on composer downgrade behavior rather than drifting in copy —
//!   a send-ready-looking composition can never outrun the resolution / freshness / budget /
//!   route proof it is being viewed away from.
//!
//! Each [`ComposerComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::M5PromptComposerComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5ComposerRequiredLabel`] and
//! [`M5ComposerDowngradeTrigger`] and the shared [`M5ComposerConsumerSurface`] consumer
//! surfaces rather than minting parallel synonyms, so the certified labels stay
//! byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw prompt bodies, attachment contents, provider
//! credentials, and pasted external text never cross this boundary; the packet carries only
//! typed class tokens, opaque summary / evidence refs, booleans, and redacted labels so
//! support, release, and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking composer material.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's
// families, required labels, downgrade triggers, and consumer surfaces rather than mint
// parallel ones.
use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5ComposerConsumerSurface, M5ComposerDowngradeTrigger, M5ComposerRequiredLabel,
    M5PromptComposerComponentFamily,
};

/// Schema version stamped on the M05-890 prompt-composer-component accessibility fallback
/// packet.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ComposerComponentAccessibilityPacket`].
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_prompt_composer_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ComposerComponentAccessibilityRow`].
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_prompt_composer_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ai/m5-prompt-composer-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/ai/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_mentions_are_unresolved_attachments_are_stale_budget_overflow_changes_composition_or_policy_blocks_routes_across_claimed_m5_composer_components.md";

/// Repo-relative path of the frozen prompt-composer component matrix this lane certifies.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ai/m5/m5-prompt-composer-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const COMPOSER_COMPONENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback.md";

/// The reusable component families that render a non-linear hierarchy (the budget / size
/// strip's omitted-context drawer with its nested included / omitted context sub-rows) and
/// therefore MUST bind their tree to an equivalent flat list / textual path so the
/// hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5PromptComposerComponentFamily) -> bool {
    matches!(family, M5PromptComposerComponentFamily::BudgetSizeStrip)
}

/// The composer dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5PromptComposerComponentFamily,
) -> M5ComposerClaimDimension {
    match family {
        M5PromptComposerComponentFamily::PromptComposerHeader => {
            M5ComposerClaimDimension::RouteReadiness
        }
        M5PromptComposerComponentFamily::ContextAttachmentPill => {
            M5ComposerClaimDimension::AttachmentTrust
        }
        M5PromptComposerComponentFamily::MentionResolver => {
            M5ComposerClaimDimension::MentionResolution
        }
        M5PromptComposerComponentFamily::SlashCommandRow => {
            M5ComposerClaimDimension::CommandAvailability
        }
        M5PromptComposerComponentFamily::BudgetSizeStrip => {
            M5ComposerClaimDimension::BudgetHeadroom
        }
        M5PromptComposerComponentFamily::TaintedContextWarning => {
            M5ComposerClaimDimension::ContextTaint
        }
        M5PromptComposerComponentFamily::DraftStateRow => M5ComposerClaimDimension::DraftLocality,
        M5PromptComposerComponentFamily::AttachmentStaleBanner => {
            M5ComposerClaimDimension::AttachmentFreshness
        }
        M5PromptComposerComponentFamily::SendReviewControl => M5ComposerClaimDimension::SendGate,
    }
}

/// A rendered fallback modality for a prompt-composer component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerFallbackModality {
    /// A rich, structured (omitted-context drawer / grouped strip) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5ComposerFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured
    /// surface (i.e. a keyboard / screen-reader / headless path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the
/// same component may render at desktop-full capability or narrow to a companion,
/// read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerRenderingSurface {
    /// The full-capability desktop composer surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ComposerRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless
    /// users (red).
    ViewOnlyTrap,
}

impl ComposerNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless
    /// users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl ComposerExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ComposerNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The composer-support claim ceiling a component asserts: how strong a pre-send
/// composition posture it lets a surface present. Auto-narrowing lowers this ceiling when a
/// composer dimension weakens so an unresolved mention, stale attachment, over-budget
/// composition, offline draft, or policy-blocked route can never keep an old `ReadyToSend`
/// or `ReviewableComposition` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerSupportClaim {
    /// Ready to send: a fully composed, resolved, route-clear, untainted, budget-fitting
    /// composition — the strongest claim.
    ReadyToSend,
    /// Reviewable composition: a resolved, self-sufficient composition (a slash-command row
    /// or reviewed control) that is reviewable but is not itself a certified send-ready
    /// claim.
    ReviewableComposition,
    /// Narrowed composition: usable, but drawn from a narrowed attachment / mention scope
    /// rather than the exact one requested.
    NarrowedComposition,
    /// Local-only composition: the draft is offline / local-only and cannot leave the shell
    /// live until connectivity or route access returns.
    LocalOnlyComposition,
    /// Unresolved composition: a mention / attachment could not be resolved or verified; the
    /// composition is reconstructed from unresolved or tainted material.
    UnresolvedComposition,
    /// Policy-blocked composition: the route is policy-blocked or the composition overflows
    /// a hard budget ceiling.
    PolicyBlockedComposition,
}

impl M5ComposerSupportClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::ReadyToSend,
        Self::ReviewableComposition,
        Self::NarrowedComposition,
        Self::LocalOnlyComposition,
        Self::UnresolvedComposition,
        Self::PolicyBlockedComposition,
    ];

    /// Capability rank; a higher rank asserts a stronger composition posture. Narrowing
    /// lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::ReadyToSend => 5,
            Self::ReviewableComposition => 4,
            Self::NarrowedComposition => 3,
            Self::LocalOnlyComposition => 2,
            Self::UnresolvedComposition => 1,
            Self::PolicyBlockedComposition => 0,
        }
    }

    /// Returns true when this claim asserts a fully send-ready composition.
    pub const fn asserts_ready_to_send(self) -> bool {
        matches!(self, Self::ReadyToSend)
    }

    /// Returns true when this claim asserts a fully self-sufficient (send-ready or
    /// resolved / reviewable) composition.
    pub const fn asserts_full_composition(self) -> bool {
        matches!(self, Self::ReadyToSend | Self::ReviewableComposition)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyToSend => "ready_to_send",
            Self::ReviewableComposition => "reviewable_composition",
            Self::NarrowedComposition => "narrowed_composition",
            Self::LocalOnlyComposition => "local_only_composition",
            Self::UnresolvedComposition => "unresolved_composition",
            Self::PolicyBlockedComposition => "policy_blocked_composition",
        }
    }
}

/// The composer dimension whose state governs how far a component may claim to be a
/// send-ready composition. The five spec axes the lane must auto-narrow on — unresolved
/// mentions, stale attachments, over-budget composition, offline-local-only fallbacks, and
/// policy-blocked routes — are [`Self::MentionResolution`], [`Self::AttachmentFreshness`],
/// [`Self::BudgetHeadroom`], [`Self::DraftLocality`], and [`Self::RouteReadiness`]; the
/// remaining dimensions cover the attachment-pill, slash-command-row, tainted-warning, and
/// send-control families' primary weakening axes so every frozen family carries an honest
/// narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerClaimDimension {
    /// Route readiness: is the composer header's route / provider / model clear and
    /// policy-allowed, or masked / policy-blocked?
    RouteReadiness,
    /// Attachment trust: is the context-attachment pill's object identity verified and
    /// in-scope, or narrowed / out-of-scope?
    AttachmentTrust,
    /// Mention resolution: did the mention resolver bind the `@`-mention to an exact object,
    /// or leave it unresolved?
    MentionResolution,
    /// Command availability: is the slash-command row available and approved, or
    /// disabled / gated / mode-unstated?
    CommandAvailability,
    /// Budget headroom: does the composition fit the budget, or overflow it and change what
    /// is sent?
    BudgetHeadroom,
    /// Context taint: is the pasted / external context untainted, or tainted and unverified?
    ContextTaint,
    /// Draft locality: is the draft persisted and shareable, or offline / local-only?
    DraftLocality,
    /// Attachment freshness: is the attachment current against its source, or stale / a
    /// last-known snapshot?
    AttachmentFreshness,
    /// Send gate: is the send / review control's gate satisfied, or blocked / awaiting
    /// review?
    SendGate,
}

impl M5ComposerClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RouteReadiness,
        Self::AttachmentTrust,
        Self::MentionResolution,
        Self::CommandAvailability,
        Self::BudgetHeadroom,
        Self::ContextTaint,
        Self::DraftLocality,
        Self::AttachmentFreshness,
        Self::SendGate,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a
    /// narrowing. Each dimension maps to the on-topic frozen trigger the freeze matrix
    /// already governs, so the certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ComposerDowngradeTrigger {
        match self {
            Self::RouteReadiness => M5ComposerDowngradeTrigger::RouteOrProviderMasked,
            Self::AttachmentTrust => M5ComposerDowngradeTrigger::AttachmentIdentityUnstated,
            Self::MentionResolution => M5ComposerDowngradeTrigger::MentionLeftUnresolved,
            Self::CommandAvailability => M5ComposerDowngradeTrigger::ComposerModeUnstated,
            Self::BudgetHeadroom => M5ComposerDowngradeTrigger::BudgetOverrunHidden,
            Self::ContextTaint => M5ComposerDowngradeTrigger::TaintStateHidden,
            Self::DraftLocality => M5ComposerDowngradeTrigger::DraftLocalityMasked,
            Self::AttachmentFreshness => M5ComposerDowngradeTrigger::AttachmentStalenessUndisclosed,
            Self::SendGate => M5ComposerDowngradeTrigger::SendReviewGateBypassed,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteReadiness => "route_readiness",
            Self::AttachmentTrust => "attachment_trust",
            Self::MentionResolution => "mention_resolution",
            Self::CommandAvailability => "command_availability",
            Self::BudgetHeadroom => "budget_headroom",
            Self::ContextTaint => "context_taint",
            Self::DraftLocality => "draft_locality",
            Self::AttachmentFreshness => "attachment_freshness",
            Self::SendGate => "send_gate",
        }
    }
}

/// The observed condition of one composer dimension. Anything weaker than [`Self::Composed`]
/// imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerConditionState {
    /// Fully composed / resolved / verified — imposes no ceiling.
    Composed,
    /// Narrowed-in-scope — a narrowed attachment / mention scope; support drops to
    /// narrowed.
    NarrowedInScope,
    /// Local-only — the draft is offline / local-only, not a live send; support drops to
    /// local-only.
    LocalOnly,
    /// Unresolved — the mention / attachment / context could not be proven; support drops
    /// to unresolved.
    Unresolved,
    /// Blocked — the route is policy-blocked or the composition overflows a hard budget
    /// ceiling; support drops to policy-blocked.
    Blocked,
}

impl M5ComposerConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Composed,
        Self::NarrowedInScope,
        Self::LocalOnly,
        Self::Unresolved,
        Self::Blocked,
    ];

    /// Returns true when the dimension is weaker than composed and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Composed)
    }

    /// The strongest composer-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ComposerSupportClaim {
        match self {
            Self::Composed => M5ComposerSupportClaim::ReadyToSend,
            Self::NarrowedInScope => M5ComposerSupportClaim::NarrowedComposition,
            Self::LocalOnly => M5ComposerSupportClaim::LocalOnlyComposition,
            Self::Unresolved => M5ComposerSupportClaim::UnresolvedComposition,
            Self::Blocked => M5ComposerSupportClaim::PolicyBlockedComposition,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Composed => "composed",
            Self::NarrowedInScope => "narrowed_in_scope",
            Self::LocalOnly => "local_only",
            Self::Unresolved => "unresolved",
            Self::Blocked => "blocked",
        }
    }
}

/// One composer dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ComposerClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ComposerConditionState,
}

/// An honest composer-support-claim auto-narrow block. When a composer dimension weakens,
/// the component's support claim lowers to the permitted ceiling, names the binding
/// dimension and frozen trigger, and preserves the canonical composer / attachment /
/// mention / route / draft identity rather than silently dropping it — the draft is never
/// lost opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5ComposerSupportClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5ComposerClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ComposerDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical composer mode, route, attachment / mention identity, budget, and draft
    /// are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The draft is preserved (never discarded) across the narrowing; must hold so blocked,
    /// stale, unresolved, and over-budget states never fail opaquely.
    pub preserves_draft_integrity: bool,
}

impl ComposerClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and draft
    /// integrity and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_draft_integrity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl ComposerCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as the
    /// sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ComposerRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ComposerNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a composer accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims composition, or drops state
    /// silently (red).
    Stranded,
}

impl ComposerComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one prompt-composer component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerComponentAccessibilityRow {
    /// Record kind; must equal [`COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5PromptComposerComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the composer / draft / attachment / mention context this component
    /// acts on; stays visible on every surface, so this is never empty.
    pub composer_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual
    /// (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ComposerFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical mode, route, attachment,
    /// mention, budget, taint, draft, and send-gate truth as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ComposerNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ComposerNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ComposerNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ComposerExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ComposerCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5ComposerSupportClaim,
    /// The observed condition of each modeled composer dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ComposerClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ComposerClaimAutoNarrow>,
    /// Whether the draft is preserved on this component regardless of narrowing; must hold
    /// so blocked, stale, unresolved, and over-budget states never fail opaquely.
    pub draft_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ComposerRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ComposerRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ComposerRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ComposerConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ComposerComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality
    /// is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Composed` when the row does not
    /// model that dimension.
    pub fn condition_for(&self, dimension: M5ComposerClaimDimension) -> M5ComposerConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ComposerConditionState::Composed)
    }

    /// Whether any modeled dimension is weaker than composed.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's
    /// ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5ComposerSupportClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5ComposerClaimDimension> {
        let mut binding: Option<(M5ComposerClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ComposerSupportClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: an unresolved mention, stale attachment, over-budget
    /// composition, offline draft, or policy-blocked route can no longer keep an old
    /// `ReadyToSend` / `ReviewableComposition` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow
    /// block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical identity
    /// and draft integrity. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical
    /// truth — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.composer_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / draft integrity: blocked, stale, unresolved, and over-budget states preserve the
    /// draft. The row must assert `draft_preserved`, and any narrow block must preserve
    /// draft integrity too.
    pub fn preserves_draft_integrity(&self) -> bool {
        self.draft_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_draft_integrity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries
    /// an honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay
    /// aligned on the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ComposerRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ComposerComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_draft_integrity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ComposerComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ComposerComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ComposerComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.composer_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-890 prompt-composer-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerComponentAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_drafts_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ComposerComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ComposerComponentAccessibilityRow>,
}

/// Checked-in M05-890 prompt-composer-component accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ComposerComponentAccessibilityRow>,
    pub summary: ComposerComponentAccessibilitySummary,
}

impl ComposerComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ComposerComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ComposerComponentAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_drafts_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5PromptComposerComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ComposerClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ComposerSupportClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ComposerConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComposerComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ComposerConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&ComposerComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ComposerComponentAccessibilityStatus::Parity => green += 1,
                ComposerComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ComposerComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ComposerComponentAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ComposerComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ComposerComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ComposerComponentAccessibilityRow::export_preserves_meaning),
            all_drafts_preserved: self
                .rows
                .iter()
                .all(ComposerComponentAccessibilityRow::preserves_draft_integrity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ComposerComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComposerComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ComposerComponentAccessibilityViolation::SchemaVersion {
                expected: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != COMPOSER_COMPONENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ComposerComponentAccessibilityViolation::RecordKind {
                expected: COMPOSER_COMPONENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ComposerComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComposerComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(ComposerComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory composer label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual
            // path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ComposerFallbackModality::Structured)
            {
                violations.push(
                    ComposerComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts a send-ready / reviewable composition for a
            // weakened one.
            if !row.claim_is_honest() {
                violations.push(ComposerComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    ComposerComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    ComposerComponentAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: blocked, stale, unresolved, and over-budget states preserve the draft.
            if !row.preserves_draft_integrity() {
                violations.push(ComposerComponentAccessibilityViolation::DraftDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ComposerComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == ComposerComponentAccessibilityStatus::Stranded {
                violations.push(ComposerComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5PromptComposerComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ComposerClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full
        // narrowing spectrum (ready-to-send → … → policy-blocked) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ComposerSupportClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Cross-surface: the same narrowed state must reach the inline composer, panel,
        // patch review, branch-agent console, help / companion composer, CLI, and support /
        // release exports — so every consumer surface is exercised at least once across the
        // packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ComposerConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ComposerComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ComposerComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("prompt composer accessibility fallback packet serializes"),
        ) {
            violations.push(ComposerComponentAccessibilityViolation::RawComposerMaterialInExport);
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
            .expect("prompt composer accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Prompt-Composer-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5PromptComposerComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_support_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in prompt-composer-component accessibility fallback
/// export.
pub fn current_m5_composer_component_a11y_fallback_export(
) -> Result<ComposerComponentAccessibilityPacket, ComposerComponentAccessibilityArtifactError> {
    let packet: ComposerComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-prompt-composer-component-accessibility-fallback/support_export.json"
    )))
    .map_err(ComposerComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ComposerComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in prompt-composer-component accessibility
/// fallback export.
#[derive(Debug)]
pub enum ComposerComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ComposerComponentAccessibilityViolation>),
}

impl fmt::Display for ComposerComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "prompt composer accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "prompt composer accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ComposerComponentAccessibilityArtifactError {}

/// Validation failure for M05-890 prompt-composer-component accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComposerComponentAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5ComposerClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
        id: String,
    },
    DraftDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5PromptComposerComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ComposerClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5ComposerSupportClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5ComposerConsumerSurface,
    },
    SummaryMismatch,
    RawComposerMaterialInExport,
}

impl fmt::Display for ComposerComponentAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory composer label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a send-ready / reviewable composition for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
                )
            }
            Self::DraftDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve draft integrity across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawComposerMaterialInExport => {
                write!(f, "export contains raw composer material")
            }
        }
    }
}

impl Error for ComposerComponentAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "stale"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in prompt-composer-component accessibility fallback packet.
/// This is the one source of truth shared by the tests and the on-disk support export so
/// both stay byte-aligned.
pub fn seeded_m5_composer_component_a11y_fallback_packet() -> ComposerComponentAccessibilityPacket {
    ComposerComponentAccessibilityPacket::new(ComposerComponentAccessibilityPacketInput {
        packet_id: "m5-prompt-composer-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-07T00:00:00Z".to_owned(),
        matrix_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:prompt-composer-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ComposerRequiredLabel> {
    M5ComposerRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ComposerCopyExportParity {
    ComposerCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ComposerClaimDimension,
    state: M5ComposerConditionState,
) -> ComposerClaimConditionEntry {
    ComposerClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and
/// CLI inspect — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ComposerConsumerSurface]) -> Vec<M5ComposerConsumerSurface> {
    let mut out = vec![
        M5ComposerConsumerSurface::SupportExport,
        M5ComposerConsumerSurface::CliInspect,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row
/// keeps full label and summary parity on the narrower surfaces; a narrowed row discloses
/// the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: ComposerNarrowingDisclosureState,
) -> Vec<ComposerRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ComposerRenderingNarrowingDisclosure {
            rendering_surface: M5ComposerRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ComposerRenderingNarrowingDisclosure {
            rendering_surface: M5ComposerRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_send".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ComposerRenderingNarrowingDisclosure> {
    surface_disclosures(labels, ComposerNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ComposerRenderingNarrowingDisclosure> {
    surface_disclosures(labels, ComposerNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5ComposerRenderingSurface> {
    vec![
        M5ComposerRenderingSurface::DesktopFull,
        M5ComposerRenderingSurface::CliHeadless,
        M5ComposerRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<ComposerComponentAccessibilityRow> {
    vec![
        // Prompt-composer header — the route / provider / model is clear and policy-allowed,
        // so the header carries a fully send-ready composition and is reachable on every
        // surface (green).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:prompt-composer-header".to_owned(),
            component_family: M5PromptComposerComponentFamily::PromptComposerHeader,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:header:0001".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:prompt-composer-header:a11y".to_owned(),
            copy_export: copy_export(&["composer_mode", "composer_scope", "route_class", "provider"]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::RouteReadiness,
                M5ComposerConditionState::Composed,
            )],
            claim_narrow: None,
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["composer_mode", "route_class", "provider"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::InlineComposerUi,
                M5ComposerConsumerSurface::ComposerPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15 Prompt Composer header contract".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("prompt-composer-header"),
        },
        // Slash-command row — the command is available and approved and states its mode, so
        // the row carries a reviewable, self-sufficient composition with no gate to clear
        // (green).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:slash-command-row".to_owned(),
            component_family: M5PromptComposerComponentFamily::SlashCommandRow,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:slash-command:0002".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:slash-command-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "command_id",
                "availability",
                "approval_gate",
                "capability_class",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReviewableComposition,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::CommandAvailability,
                M5ComposerConditionState::Composed,
            )],
            claim_narrow: None,
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["command_id", "availability", "approval_gate"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::ComposerPanelUi,
                M5ComposerConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15 slash-command row rules".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("slash-command-row"),
        },
        // Context-attachment pill — the attached object could only be resolved at a narrowed
        // scope, so the pill auto-narrows to a narrowed composition rather than the exact
        // in-scope object (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:context-attachment-pill".to_owned(),
            component_family: M5PromptComposerComponentFamily::ContextAttachmentPill,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:attachment-pill:0003".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:context-attachment-pill:a11y".to_owned(),
            copy_export: copy_export(&[
                "attachment_kind",
                "object_identity",
                "trust_state",
                "scope",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::AttachmentTrust,
                M5ComposerConditionState::NarrowedInScope,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::NarrowedComposition,
                binding_dimension: M5ComposerClaimDimension::AttachmentTrust,
                trigger: M5ComposerDowngradeTrigger::AttachmentIdentityUnstated,
                narrowed_label:
                    "Attachment resolvable only at a narrowed scope — shown narrowed, not the exact in-scope object"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "attachment_kind",
                "object_identity",
                "trust_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::PatchReviewUi,
                M5ComposerConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix EY attachment pill grammar".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("context-attachment-pill"),
        },
        // Mention resolver — the `@`-mention could not be bound to an exact object, so the
        // resolver auto-narrows to an unresolved composition and never lets an unresolved
        // mention pass as send-ready (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:mention-resolver".to_owned(),
            component_family: M5PromptComposerComponentFamily::MentionResolver,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:mention-resolver:0004".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:mention-resolver:a11y".to_owned(),
            copy_export: copy_export(&[
                "mention_text",
                "resolution_state",
                "candidate_target",
                "review_action",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::MentionResolution,
                M5ComposerConditionState::Unresolved,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::UnresolvedComposition,
                binding_dimension: M5ComposerClaimDimension::MentionResolution,
                trigger: M5ComposerDowngradeTrigger::MentionLeftUnresolved,
                narrowed_label:
                    "Mention could not be bound to an exact object — shown unresolved and held for review before send"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "mention_text",
                "resolution_state",
                "candidate_target",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::InlineComposerUi,
                M5ComposerConsumerSurface::HelpComposerUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15 mention-resolution failure handling".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("mention-resolver"),
        },
        // Budget / size strip — hierarchy-heavy (omitted-context drawer with its nested
        // included / omitted context sub-rows); the composition overflows a hard budget
        // ceiling, so the strip auto-narrows to a policy-blocked composition and binds its
        // drawer to a flat list / textual path (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:budget-size-strip".to_owned(),
            component_family: M5PromptComposerComponentFamily::BudgetSizeStrip,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:budget-strip:0005".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::Structured,
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:budget-size-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "budget_posture",
                "omitted_context_reason",
                "included_classes",
                "omitted_classes",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::BudgetHeadroom,
                M5ComposerConditionState::Blocked,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::PolicyBlockedComposition,
                binding_dimension: M5ComposerClaimDimension::BudgetHeadroom,
                trigger: M5ComposerDowngradeTrigger::BudgetOverrunHidden,
                narrowed_label:
                    "Composition overflows the hard budget ceiling — shown policy-blocked with omitted context disclosed, not silently truncated"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "budget_posture",
                "omitted_context_reason",
                "included_classes",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::BranchAgentConsoleUi,
                M5ComposerConsumerSurface::PatchReviewUi,
            ]),
            source_refs: vec![
                "TAD §18.4 AI context transparency / budget".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("budget-size-strip"),
        },
        // Tainted-context warning — pasted external text is tainted and unverified, so the
        // warning auto-narrows to an unresolved composition rather than treating the tainted
        // context as trusted (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:tainted-context-warning".to_owned(),
            component_family: M5PromptComposerComponentFamily::TaintedContextWarning,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:tainted-warning:0006".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:tainted-context-warning:a11y".to_owned(),
            copy_export: copy_export(&[
                "taint_source",
                "taint_severity",
                "data_treatment",
                "review_path",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::ContextTaint,
                M5ComposerConditionState::Unresolved,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::UnresolvedComposition,
                binding_dimension: M5ComposerClaimDimension::ContextTaint,
                trigger: M5ComposerDowngradeTrigger::TaintStateHidden,
                narrowed_label:
                    "Pasted external context is tainted and unverified — shown unresolved with a review path before it can be trusted"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "taint_source",
                "taint_severity",
                "data_treatment",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::CompanionComposerUi,
                M5ComposerConsumerSurface::InlineComposerUi,
            ]),
            source_refs: vec![
                "TAD §18.6 prompt-injection / tool-output taint".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("tainted-context-warning"),
        },
        // Draft-state row — the draft is offline / local-only and cannot leave the shell
        // live, so the row auto-narrows to a local-only composition rather than presenting a
        // shareable, send-ready draft (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:draft-state-row".to_owned(),
            component_family: M5PromptComposerComponentFamily::DraftStateRow,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:draft-state:0007".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:draft-state-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "draft_locality",
                "retention_posture",
                "sharing_state",
                "sync_state",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::DraftLocality,
                M5ComposerConditionState::LocalOnly,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::LocalOnlyComposition,
                binding_dimension: M5ComposerClaimDimension::DraftLocality,
                trigger: M5ComposerDowngradeTrigger::DraftLocalityMasked,
                narrowed_label:
                    "Draft is offline / local-only — shown local-only and preserved on this device, not sent or shared until connectivity returns"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "draft_locality",
                "retention_posture",
                "sharing_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::BranchAgentConsoleUi,
                M5ComposerConsumerSurface::HelpComposerUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15 draft-state / offline-local-only rules".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("draft-state-row"),
        },
        // Attachment-stale banner — the attachment drifted from its source and is a
        // last-known snapshot, so the banner auto-narrows to a local-only composition rather
        // than presenting the attachment as current (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:attachment-stale-banner".to_owned(),
            component_family: M5PromptComposerComponentFamily::AttachmentStaleBanner,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:attachment-stale:0008".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:attachment-stale-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "staleness_reason",
                "attached_object",
                "anchored_version",
                "refresh_action",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::AttachmentFreshness,
                M5ComposerConditionState::LocalOnly,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::LocalOnlyComposition,
                binding_dimension: M5ComposerClaimDimension::AttachmentFreshness,
                trigger: M5ComposerDowngradeTrigger::AttachmentStalenessUndisclosed,
                narrowed_label:
                    "Attachment drifted from its source — shown from a last-known snapshot anchored to an older revision, not current"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "staleness_reason",
                "attached_object",
                "anchored_version",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::PatchReviewUi,
                M5ComposerConsumerSurface::ComposerPanelUi,
            ]),
            source_refs: vec![
                "UI/UX Spec Appendix EY attachment-stale warning grammar".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("attachment-stale-banner"),
        },
        // Send / review control — the route is policy-blocked so the send gate cannot be
        // cleared, and the control auto-narrows to a policy-blocked composition rather than
        // offering a widened-authority send (yellow).
        ComposerComponentAccessibilityRow {
            record_kind: COMPOSER_COMPONENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_COMPONENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:send-review-control".to_owned(),
            component_family: M5PromptComposerComponentFamily::SendReviewControl,
            source_family_schema_ref: COMPOSER_COMPONENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF
                .to_owned(),
            composer_context_ref: "composer:send-review:0009".to_owned(),
            fallback_modalities: vec![
                M5ComposerFallbackModality::List,
                M5ComposerFallbackModality::Textual,
                M5ComposerFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            cli_reach: ComposerNonVisualReachState::ReachableAndLabeled,
            export_summary: ComposerExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:send-review-control:a11y".to_owned(),
            copy_export: copy_export(&[
                "send_posture",
                "review_requirement",
                "send_path",
                "route_state",
            ]),
            full_support_claim: M5ComposerSupportClaim::ReadyToSend,
            claim_conditions: vec![condition(
                M5ComposerClaimDimension::SendGate,
                M5ComposerConditionState::Blocked,
            )],
            claim_narrow: Some(ComposerClaimAutoNarrow {
                narrowed_to: M5ComposerSupportClaim::PolicyBlockedComposition,
                binding_dimension: M5ComposerClaimDimension::SendGate,
                trigger: M5ComposerDowngradeTrigger::SendReviewGateBypassed,
                narrowed_label:
                    "Route is policy-blocked so the send gate cannot clear — shown policy-blocked with the draft preserved, never a widened-authority send"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_draft_integrity: true,
            }),
            draft_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "send_posture",
                "review_requirement",
                "send_path",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ComposerConsumerSurface::CompanionComposerUi,
                M5ComposerConsumerSurface::BranchAgentConsoleUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §15 split-send / review-before-send controls".to_owned(),
                COMPOSER_COMPONENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-07T00:00:00Z".to_owned(),
            evidence_refs: ev("send-review-control"),
        },
    ]
}
