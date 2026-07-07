//! Two reusable M5 prompt-composer primitives — the composer header and the
//! context-attachment pill — so pre-send composition truth becomes inspectable on
//! first-class AI composition surfaces.
//!
//! Aureline's frozen prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! names the prompt-composer header and the context-attachment pill as two governed
//! component families and freezes their controlled vocabulary — the composer modes,
//! scopes, and route classes, the attachment kinds and trust states, the budget
//! postures, the staleness reasons, the surface families, the deployment lines, the
//! consumer surfaces, the accessibility routes, the qualification classes, and the
//! downgrade triggers. This module *implements* those two contracts as reusable
//! primitives so a user can tell — from the header or the pill alone — what mode, scope,
//! and route a request composes under, what budget band applies, whether a route stays
//! local-only or is blocked, and, for every attached object, its exact identity, its
//! freshness / trust state, and the remove / open behavior available before send.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_prompt_composer_header`] — takes one composer's mode, scope,
//!    route / provider / model, budget posture, and route-blocked / review signals, and
//!    produces one [`M5ResolvedPromptComposerHeader`] carrying the derived header posture
//!    (ready versus local-only versus review-before-send versus budget-constrained versus
//!    route-blocked versus budget-blocked), whether the request is sendable, and whether
//!    the route stays on device. It never masks the mode or route and never presents a
//!    blocked route or a hard budget block as ready to send.
//! 2. [`resolve_context_attachment_pill`] — takes one attachment's stable id, label,
//!    kind, trust state, staleness, scope, and source-removed signals, and produces one
//!    [`M5ResolvedContextAttachmentPill`] carrying the derived pill posture, the bounded
//!    open / remove / refresh / review / reveal actions, whether the attachment is
//!    openable, and whether it needs review before send. It preserves the exact object
//!    identity, never shows a stale, unverified, or tainted attachment as trusted-fresh,
//!    and always offers a remove action before send.
//!
//! A single parity matrix — [`M5PromptComposerHeaderPillPacket`] — binds one row per
//! claimed M5 composition consumer (the inline assistant, the side panel, the patch
//! draft, the handoff surface, and the CLI / support export) to the shared header and
//! pill anatomy, the same composer modes, scopes, route classes, budget postures,
//! attachment kinds, trust states, header postures, pill postures, bounded actions,
//! export fields, and non-visual accessibility routes, so the mode / route / budget /
//! freshness / trust vocabulary stays identical across inline, side-panel, patch-draft,
//! handoff, and CLI / support exports.
//!
//! The composer mode ([`M5ComposerMode`]), scope ([`M5ComposerScope`]), route class
//! ([`M5ComposerRouteClass`]), attachment kind ([`M5AttachmentKind`]), trust state
//! ([`M5AttachmentTrustState`]), budget posture ([`M5BudgetPosture`]), staleness reason
//! ([`M5StalenessReason`]), surface family ([`M5ComposerSurfaceFamily`]), deployment line
//! ([`M5ComposerDeploymentLine`]), consumer surface ([`M5ComposerConsumerSurface`]),
//! accessibility route ([`M5ComposerAccessibilityRoute`]), qualification class
//! ([`M5ComposerQualificationClass`]), and downgrade trigger
//! ([`M5ComposerDowngradeTrigger`]) are reused verbatim from the frozen matrix. This
//! module mints new vocabulary only for what that matrix left implicit about the header
//! and the pill themselves: their composition consumers, their anatomy parts, their
//! derived header posture, their derived pill posture, their bounded pill actions, and
//! their export fields. No M5 composition surface invents a second composer grammar.
//!
//! Raw prompts, pasted bodies, attachment contents, raw URLs, credentials, and private
//! endpoints stay outside the support boundary; every provider / model label, attachment
//! id, and attachment label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-prompt-composer-header-and-context-attachment-pill.schema.json`](../../../../schemas/ai/m5-prompt-composer-header-and-context-attachment-pill.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces.md`](../../../../docs/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_prompt_composer_header_pill_handoff_beta_narrowed,
    seeded_m5_prompt_composer_header_pill_packet,
    seeded_m5_prompt_composer_header_pill_patch_draft_preview_narrowed,
    M5_PROMPT_COMPOSER_HEADER_PILL_PACKET_ID,
};

// The composer mode, scope, route class, attachment kind, trust state, budget posture,
// staleness reason, surface family, deployment line, consumer surface, accessibility
// route, qualification class, and downgrade triggers are frozen once, in the
// prompt-composer component matrix. These primitives reuse them verbatim so they never
// invent a parallel composer vocabulary.
pub use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5AttachmentKind, M5AttachmentTrustState, M5BudgetPosture, M5ComposerAccessibilityRoute,
    M5ComposerConsumerSurface, M5ComposerDeploymentLine, M5ComposerDowngradeTrigger, M5ComposerMode,
    M5ComposerQualificationClass, M5ComposerRouteClass, M5ComposerScope, M5ComposerSurfaceFamily,
    M5StalenessReason,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5PromptComposerHeaderPillPacket`].
pub const M5_PROMPT_COMPOSER_HEADER_PILL_RECORD_KIND: &str =
    "implement_m5_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces";

/// Schema version for M5 prompt-composer-header / context-attachment-pill records.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the header / pill boundary schema.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF: &str =
    "schemas/ai/m5-prompt-composer-header-and-context-attachment-pill.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_DOC_REF: &str =
    "docs/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces.md";

/// Repo-relative path of the frozen prompt-composer component matrix these primitives
/// narrow from.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the richer-prompt-composer contract this primitive binds its
/// mode / attachment / omitted-context truth against.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_RICHER_COMPOSER_REF: &str =
    "schemas/ai/implement-a-richer-prompt-composer-with-intent-modes-typed-attachments-context-pinning-and-omitted-context-tru.schema.json";

/// Repo-relative path of the prompt-composer-draft / attachment-provenance record
/// contract this primitive binds its attachment-identity truth against.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_ATTACHMENT_PROVENANCE_REF: &str =
    "schemas/ai/ship-prompt-composer-draft-and-session-artifact-records-attachment-and-mention-provenance-context-add-or-remove-receipts.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_CSV_REF: &str =
    "artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PROMPT_COMPOSER_HEADER_PILL_REPORT_REF: &str =
    "artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces.md";

/// One claimed M5 composition consumer that renders the shared composer header and the
/// context-attachment pill. These are the consumers the acceptance criteria name — the
/// inline assistant, the side panel, the patch draft, the handoff surface, and the CLI /
/// support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PromptComposerHeaderPillConsumerSurface {
    /// The inline assistant composer.
    InlineAssistant,
    /// The side-panel composer.
    SidePanel,
    /// The patch-draft composer.
    PatchDraft,
    /// The branch-agent / handoff composer.
    HandoffSurface,
    /// The CLI inspect / support export.
    CliSupportExport,
}

impl M5PromptComposerHeaderPillConsumerSurface {
    /// Every claimed composition consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlineAssistant,
        Self::SidePanel,
        Self::PatchDraft,
        Self::HandoffSurface,
        Self::CliSupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineAssistant => "inline_assistant",
            Self::SidePanel => "side_panel",
            Self::PatchDraft => "patch_draft",
            Self::HandoffSurface => "handoff_surface",
            Self::CliSupportExport => "cli_support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InlineAssistant => "Inline Assistant",
            Self::SidePanel => "Side Panel",
            Self::PatchDraft => "Patch Draft",
            Self::HandoffSurface => "Handoff Surface",
            Self::CliSupportExport => "CLI / Support Export",
        }
    }
}

/// The derived posture of a composer header — the resolver's verdict about whether a
/// request is ready, stays local-only, needs review before send, is budget-constrained,
/// is route-blocked, or is budget-blocked. Computed in a fixed blocking-first order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerHeaderPosture {
    /// Ready to compose and send on a route that leaves the shell.
    ReadyComposing,
    /// Composing on a route that stays on the local device.
    LocalOnlyComposing,
    /// Composing in a review-first mode; a review is required before send.
    ReviewBeforeSend,
    /// Sendable but near or over the budget band.
    BudgetConstrained,
    /// The route is blocked by policy; the request cannot leave the shell.
    RouteBlocked,
    /// A hard budget ceiling blocks the request.
    BudgetBlocked,
}

impl M5ComposerHeaderPosture {
    /// Every header posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReadyComposing,
        Self::LocalOnlyComposing,
        Self::ReviewBeforeSend,
        Self::BudgetConstrained,
        Self::RouteBlocked,
        Self::BudgetBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyComposing => "ready_composing",
            Self::LocalOnlyComposing => "local_only_composing",
            Self::ReviewBeforeSend => "review_before_send",
            Self::BudgetConstrained => "budget_constrained",
            Self::RouteBlocked => "route_blocked",
            Self::BudgetBlocked => "budget_blocked",
        }
    }

    /// True when the request can leave the shell (not route- or budget-blocked).
    pub const fn is_sendable(self) -> bool {
        !matches!(self, Self::RouteBlocked | Self::BudgetBlocked)
    }

    /// True when the header needs operator attention before send.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::RouteBlocked | Self::BudgetBlocked | Self::BudgetConstrained
        )
    }
}

/// Controlled composer-header anatomy part the shared header surfaces. The parts in
/// [`M5ComposerHeaderAnatomyPart::MANDATORY`] are required on every header so mode, scope,
/// route / provider / model, and budget band are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerHeaderAnatomyPart {
    /// The composer mode badge.
    ModeBadge,
    /// The composer scope cue.
    ScopeCue,
    /// The route / provider / model cue.
    RouteProviderModelCue,
    /// The budget band cue.
    BudgetBandCue,
    /// The review-context entry point.
    ReviewContextEntry,
    /// The local-only / blocked-route cue.
    LocalOrBlockedCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5ComposerHeaderAnatomyPart {
    /// Every header anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ModeBadge,
        Self::ScopeCue,
        Self::RouteProviderModelCue,
        Self::BudgetBandCue,
        Self::ReviewContextEntry,
        Self::LocalOrBlockedCue,
        Self::KeyboardRouteCue,
    ];

    /// The header anatomy parts every header must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ModeBadge,
        Self::ScopeCue,
        Self::RouteProviderModelCue,
        Self::BudgetBandCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModeBadge => "mode_badge",
            Self::ScopeCue => "scope_cue",
            Self::RouteProviderModelCue => "route_provider_model_cue",
            Self::BudgetBandCue => "budget_band_cue",
            Self::ReviewContextEntry => "review_context_entry",
            Self::LocalOrBlockedCue => "local_or_blocked_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// The derived posture of a context-attachment pill — the resolver's verdict about how
/// trustworthy and fresh an attached object is. Computed in a fixed honesty-first order,
/// so a stale, unverified, or tainted attachment never reads as trusted-fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentPillPosture {
    /// Trusted and fresh.
    FreshTrusted,
    /// Attached but stale.
    Stale,
    /// From an unverified source.
    Unverified,
    /// Tainted external content.
    Tainted,
    /// Redacted to a narrower scope.
    Redacted,
    /// Out of the current scope.
    OutOfScope,
}

impl M5AttachmentPillPosture {
    /// Every pill posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FreshTrusted,
        Self::Stale,
        Self::Unverified,
        Self::Tainted,
        Self::Redacted,
        Self::OutOfScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshTrusted => "fresh_trusted",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::Tainted => "tainted",
            Self::Redacted => "redacted",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// True when the attachment needs review before it can be sent.
    pub const fn needs_review_before_send(self) -> bool {
        matches!(self, Self::Tainted | Self::Unverified)
    }

    /// True when the attachment needs attention before send.
    pub const fn needs_attention(self) -> bool {
        matches!(
            self,
            Self::Tainted | Self::Unverified | Self::Stale | Self::OutOfScope
        )
    }
}

/// One bounded action a context-attachment pill offers, so a pill never hides its
/// open / remove affordances or its trust / scope follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentPillAction {
    /// Open the attached object.
    Open,
    /// Remove the attachment from the composition.
    Remove,
    /// Refresh a stale attachment.
    Refresh,
    /// Review the trust / taint of the attachment.
    ReviewTrust,
    /// Reveal the scope narrowing on the attachment.
    RevealScope,
}

impl M5AttachmentPillAction {
    /// Every pill action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Open,
        Self::Remove,
        Self::Refresh,
        Self::ReviewTrust,
        Self::RevealScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Remove => "remove",
            Self::Refresh => "refresh",
            Self::ReviewTrust => "review_trust",
            Self::RevealScope => "reveal_scope",
        }
    }
}

/// Controlled attachment-pill anatomy part the shared pill surfaces. The parts in
/// [`M5AttachmentPillAnatomyPart::MANDATORY`] are required on every pill so identity,
/// kind, trust state, freshness, and the action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentPillAnatomyPart {
    /// The stable object identity.
    IdentityCue,
    /// The attachment kind.
    KindCue,
    /// The display label.
    LabelCue,
    /// The trust state.
    TrustStateCue,
    /// The freshness / staleness cue.
    FreshnessCue,
    /// The scope cue.
    ScopeCue,
    /// The bounded action row (open / remove / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5AttachmentPillAnatomyPart {
    /// Every pill anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::IdentityCue,
        Self::KindCue,
        Self::LabelCue,
        Self::TrustStateCue,
        Self::FreshnessCue,
        Self::ScopeCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The pill anatomy parts every pill must render.
    pub const MANDATORY: [Self; 5] = [
        Self::IdentityCue,
        Self::KindCue,
        Self::TrustStateCue,
        Self::FreshnessCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityCue => "identity_cue",
            Self::KindCue => "kind_cue",
            Self::LabelCue => "label_cue",
            Self::TrustStateCue => "trust_state_cue",
            Self::FreshnessCue => "freshness_cue",
            Self::ScopeCue => "scope_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the header export carries so header truth is reconstructable. The fields in
/// [`M5ComposerHeaderExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerHeaderExportField {
    /// The composer mode.
    ComposerMode,
    /// The composer scope.
    ComposerScope,
    /// The route class.
    RouteClass,
    /// The provider / model label.
    ProviderModelLabel,
    /// The budget posture.
    BudgetPosture,
    /// The derived header posture.
    HeaderPosture,
    /// Whether the route stays on the local device.
    RouteStaysLocal,
    /// Whether a review is required before send.
    RequiresReviewBeforeSend,
}

impl M5ComposerHeaderExportField {
    /// Every header export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ComposerMode,
        Self::ComposerScope,
        Self::RouteClass,
        Self::ProviderModelLabel,
        Self::BudgetPosture,
        Self::HeaderPosture,
        Self::RouteStaysLocal,
        Self::RequiresReviewBeforeSend,
    ];

    /// The header export fields every header must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ComposerMode,
        Self::ComposerScope,
        Self::RouteClass,
        Self::BudgetPosture,
        Self::HeaderPosture,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerMode => "composer_mode",
            Self::ComposerScope => "composer_scope",
            Self::RouteClass => "route_class",
            Self::ProviderModelLabel => "provider_model_label",
            Self::BudgetPosture => "budget_posture",
            Self::HeaderPosture => "header_posture",
            Self::RouteStaysLocal => "route_stays_local",
            Self::RequiresReviewBeforeSend => "requires_review_before_send",
        }
    }
}

/// A field the pill export carries so attachment-pill truth is reconstructable. The
/// fields in [`M5AttachmentPillExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AttachmentPillExportField {
    /// The stable attachment id.
    AttachmentId,
    /// The attachment kind.
    AttachmentKind,
    /// The trust state.
    TrustState,
    /// The derived pill posture.
    PillPosture,
    /// Whether the attachment is stale.
    IsStale,
    /// Whether the attachment is openable.
    IsOpenable,
    /// The bounded available actions.
    AvailableActions,
    /// Whether the attachment needs review before send.
    NeedsReviewBeforeSend,
}

impl M5AttachmentPillExportField {
    /// Every pill export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::AttachmentId,
        Self::AttachmentKind,
        Self::TrustState,
        Self::PillPosture,
        Self::IsStale,
        Self::IsOpenable,
        Self::AvailableActions,
        Self::NeedsReviewBeforeSend,
    ];

    /// The pill export fields every pill must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::AttachmentId,
        Self::AttachmentKind,
        Self::TrustState,
        Self::PillPosture,
        Self::IsStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttachmentId => "attachment_id",
            Self::AttachmentKind => "attachment_kind",
            Self::TrustState => "trust_state",
            Self::PillPosture => "pill_posture",
            Self::IsStale => "is_stale",
            Self::IsOpenable => "is_openable",
            Self::AvailableActions => "available_actions",
            Self::NeedsReviewBeforeSend => "needs_review_before_send",
        }
    }
}

/// True when a route class keeps the request on the local device.
pub const fn route_stays_on_device(route: M5ComposerRouteClass) -> bool {
    matches!(route, M5ComposerRouteClass::LocalModel)
}

// ---- header resolver ----------------------------------------------------

/// The full input to the composer-header resolver for one composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderResolutionInput {
    /// The composer mode.
    pub composer_mode: M5ComposerMode,
    /// The composer scope.
    pub composer_scope: M5ComposerScope,
    /// The route class the request will take.
    pub route_class: M5ComposerRouteClass,
    /// The opaque provider / model label (must be non-empty).
    pub provider_model_label: String,
    /// The budget posture of the composition.
    pub budget_posture: M5BudgetPosture,
    /// True when policy blocks the route.
    pub route_blocked: bool,
    /// True when a review-context entry point is available on the header.
    pub review_context_available: bool,
}

/// The resolved composer-header truth for one composition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPromptComposerHeader {
    /// The composer mode.
    pub composer_mode: M5ComposerMode,
    /// The composer scope.
    pub composer_scope: M5ComposerScope,
    /// The route class.
    pub route_class: M5ComposerRouteClass,
    /// The opaque provider / model label.
    pub provider_model_label: String,
    /// The budget posture.
    pub budget_posture: M5BudgetPosture,
    /// True when a review-context entry point is available.
    pub review_context_available: bool,
    /// The derived header posture.
    pub header_posture: M5ComposerHeaderPosture,
    /// True when the request can leave the shell.
    pub is_sendable: bool,
    /// True when the header needs operator attention.
    pub needs_attention: bool,
    /// True when the route stays on the local device.
    pub route_stays_local: bool,
    /// True when the request leaves the shell (crosses off the local device).
    pub route_leaves_shell: bool,
    /// True when a review is required before send.
    pub requires_review_before_send: bool,
}

/// Errors returned by [`resolve_prompt_composer_header`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PromptComposerHeaderResolutionError {
    /// The provider / model label was empty.
    EmptyProviderModelLabel,
    /// A header descriptor carried forbidden material.
    ForbiddenHeaderMaterial,
}

impl M5PromptComposerHeaderResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyProviderModelLabel => "empty_provider_model_label",
            Self::ForbiddenHeaderMaterial => "forbidden_header_material",
        }
    }
}

impl fmt::Display for M5PromptComposerHeaderResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "prompt composer header resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PromptComposerHeaderResolutionError {}

/// Resolves one prompt-composer header from its declared state.
///
/// The derived header posture is computed in a fixed blocking-first order: a policy route
/// block wins first (a blocked route never reads as ready), then a hard budget ceiling
/// blocks, then a review-first mode requires review before send, then a near / over
/// budget band reads as budget-constrained, then a route that stays on the local device
/// reads as local-only, and otherwise the header reads as ready to compose. The mode,
/// scope, route, provider / model label, and budget band are carried explicitly, never
/// inferred away, and the header always records whether the route stays local and whether
/// a review is required before send.
pub fn resolve_prompt_composer_header(
    input: &M5PromptComposerHeaderResolutionInput,
) -> Result<M5ResolvedPromptComposerHeader, M5PromptComposerHeaderResolutionError> {
    if input.provider_model_label.trim().is_empty() {
        return Err(M5PromptComposerHeaderResolutionError::EmptyProviderModelLabel);
    }
    if value_repr_is_forbidden(&input.provider_model_label) {
        return Err(M5PromptComposerHeaderResolutionError::ForbiddenHeaderMaterial);
    }

    let header_posture = derive_header_posture(
        input.composer_mode,
        input.route_class,
        input.budget_posture,
        input.route_blocked,
    );
    let route_stays_local = route_stays_on_device(input.route_class);

    Ok(M5ResolvedPromptComposerHeader {
        composer_mode: input.composer_mode,
        composer_scope: input.composer_scope,
        route_class: input.route_class,
        provider_model_label: input.provider_model_label.clone(),
        budget_posture: input.budget_posture,
        review_context_available: input.review_context_available,
        header_posture,
        is_sendable: header_posture.is_sendable(),
        needs_attention: header_posture.needs_attention(),
        route_stays_local,
        route_leaves_shell: !route_stays_local,
        requires_review_before_send: matches!(input.composer_mode, M5ComposerMode::ReviewFirst),
    })
}

/// The fixed blocking-first header-posture ladder.
fn derive_header_posture(
    mode: M5ComposerMode,
    route_class: M5ComposerRouteClass,
    budget_posture: M5BudgetPosture,
    route_blocked: bool,
) -> M5ComposerHeaderPosture {
    if route_blocked {
        M5ComposerHeaderPosture::RouteBlocked
    } else if matches!(budget_posture, M5BudgetPosture::HardBlocked) {
        M5ComposerHeaderPosture::BudgetBlocked
    } else if matches!(mode, M5ComposerMode::ReviewFirst) {
        M5ComposerHeaderPosture::ReviewBeforeSend
    } else if matches!(
        budget_posture,
        M5BudgetPosture::NearLimit
            | M5BudgetPosture::OverBudget
            | M5BudgetPosture::TruncationPending
    ) {
        M5ComposerHeaderPosture::BudgetConstrained
    } else if route_stays_on_device(route_class) {
        M5ComposerHeaderPosture::LocalOnlyComposing
    } else {
        M5ComposerHeaderPosture::ReadyComposing
    }
}

// ---- attachment-pill resolver -------------------------------------------

/// The full input to the context-attachment-pill resolver for one attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextAttachmentPillResolutionInput {
    /// The opaque stable attachment id (must be non-empty).
    pub attachment_id: String,
    /// The opaque display label (must be non-empty).
    pub attachment_label: String,
    /// The attachment kind.
    pub attachment_kind: M5AttachmentKind,
    /// The attachment trust state.
    pub trust_state: M5AttachmentTrustState,
    /// True when the attachment is stale.
    pub is_stale: bool,
    /// The reason the attachment is stale, when it is stale.
    pub staleness_reason: Option<M5StalenessReason>,
    /// True when the source object has been removed (and so cannot be opened).
    pub source_removed: bool,
    /// True when the attachment is in the current scope.
    pub in_scope: bool,
}

/// The resolved context-attachment-pill truth for one attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedContextAttachmentPill {
    /// The opaque stable attachment id, preserved exactly from the input.
    pub attachment_id: String,
    /// The opaque display label.
    pub attachment_label: String,
    /// The attachment kind.
    pub attachment_kind: M5AttachmentKind,
    /// The attachment trust state.
    pub trust_state: M5AttachmentTrustState,
    /// True when the attachment is stale.
    pub is_stale: bool,
    /// The reason the attachment is stale, when it is stale.
    pub staleness_reason: Option<M5StalenessReason>,
    /// The derived pill posture.
    pub pill_posture: M5AttachmentPillPosture,
    /// The bounded actions this pill offers.
    pub available_actions: Vec<M5AttachmentPillAction>,
    /// True when the attachment is openable.
    pub is_openable: bool,
    /// True when the attachment is tainted external content.
    pub is_tainted: bool,
    /// True when the attachment needs review before send.
    pub needs_review_before_send: bool,
    /// True when the attachment needs attention before send.
    pub needs_attention: bool,
}

/// Errors returned by [`resolve_context_attachment_pill`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ContextAttachmentPillResolutionError {
    /// The attachment id was empty.
    EmptyAttachmentId,
    /// The attachment label was empty.
    EmptyAttachmentLabel,
    /// The attachment is stale but no staleness reason was given.
    StaleAttachmentWithoutReason,
    /// An attachment descriptor carried forbidden material.
    ForbiddenAttachmentMaterial,
}

impl M5ContextAttachmentPillResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyAttachmentId => "empty_attachment_id",
            Self::EmptyAttachmentLabel => "empty_attachment_label",
            Self::StaleAttachmentWithoutReason => "stale_attachment_without_reason",
            Self::ForbiddenAttachmentMaterial => "forbidden_attachment_material",
        }
    }
}

impl fmt::Display for M5ContextAttachmentPillResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "context attachment pill resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ContextAttachmentPillResolutionError {}

/// Resolves one context-attachment pill from its declared state.
///
/// The derived pill posture is computed in a fixed honesty-first order: tainted external
/// content wins first, then an out-of-scope attachment, then an unverified source, then a
/// stale attachment, then a redacted-scope attachment, and otherwise a trusted-fresh
/// attachment. The pill always preserves the exact object identity, always offers a
/// remove action, offers an open action only when the source has not been removed, and
/// offers refresh, review-trust, and reveal-scope follow-ups matched to the posture — so
/// a stale, unverified, or tainted attachment never reads as trusted-fresh, and a moved
/// or deleted attachment never appears silently openable.
pub fn resolve_context_attachment_pill(
    input: &M5ContextAttachmentPillResolutionInput,
) -> Result<M5ResolvedContextAttachmentPill, M5ContextAttachmentPillResolutionError> {
    if input.attachment_id.trim().is_empty() {
        return Err(M5ContextAttachmentPillResolutionError::EmptyAttachmentId);
    }
    if input.attachment_label.trim().is_empty() {
        return Err(M5ContextAttachmentPillResolutionError::EmptyAttachmentLabel);
    }
    if input.is_stale && input.staleness_reason.is_none() {
        return Err(M5ContextAttachmentPillResolutionError::StaleAttachmentWithoutReason);
    }
    if value_repr_is_forbidden(&input.attachment_id)
        || value_repr_is_forbidden(&input.attachment_label)
    {
        return Err(M5ContextAttachmentPillResolutionError::ForbiddenAttachmentMaterial);
    }

    let pill_posture = derive_pill_posture(input.trust_state, input.is_stale, input.in_scope);
    let is_openable = !input.source_removed;
    let available_actions = derive_pill_actions(pill_posture, is_openable, input.is_stale);

    Ok(M5ResolvedContextAttachmentPill {
        attachment_id: input.attachment_id.clone(),
        attachment_label: input.attachment_label.clone(),
        attachment_kind: input.attachment_kind,
        trust_state: input.trust_state,
        is_stale: input.is_stale,
        staleness_reason: input.staleness_reason,
        pill_posture,
        available_actions,
        is_openable,
        is_tainted: matches!(input.trust_state, M5AttachmentTrustState::TaintedExternal),
        needs_review_before_send: pill_posture.needs_review_before_send(),
        needs_attention: pill_posture.needs_attention(),
    })
}

/// The fixed honesty-first pill-posture ladder.
fn derive_pill_posture(
    trust_state: M5AttachmentTrustState,
    is_stale: bool,
    in_scope: bool,
) -> M5AttachmentPillPosture {
    if matches!(trust_state, M5AttachmentTrustState::TaintedExternal) {
        M5AttachmentPillPosture::Tainted
    } else if !in_scope || matches!(trust_state, M5AttachmentTrustState::OutOfScope) {
        M5AttachmentPillPosture::OutOfScope
    } else if matches!(trust_state, M5AttachmentTrustState::UnverifiedSource) {
        M5AttachmentPillPosture::Unverified
    } else if is_stale || matches!(trust_state, M5AttachmentTrustState::TrustedStale) {
        M5AttachmentPillPosture::Stale
    } else if matches!(trust_state, M5AttachmentTrustState::RedactedScope) {
        M5AttachmentPillPosture::Redacted
    } else {
        M5AttachmentPillPosture::FreshTrusted
    }
}

/// Derives the bounded action set from the pill posture and openable / stale signals.
///
/// Remove is always offered so an attachment can be dropped before send; open is offered
/// only when the source has not been removed; refresh, review-trust, and reveal-scope
/// follow the posture.
fn derive_pill_actions(
    posture: M5AttachmentPillPosture,
    is_openable: bool,
    is_stale: bool,
) -> Vec<M5AttachmentPillAction> {
    use M5AttachmentPillAction as Action;
    let mut actions = Vec::new();
    if is_openable {
        actions.push(Action::Open);
    }
    actions.push(Action::Remove);
    if is_stale || matches!(posture, M5AttachmentPillPosture::Stale) {
        actions.push(Action::Refresh);
    }
    if matches!(
        posture,
        M5AttachmentPillPosture::Tainted | M5AttachmentPillPosture::Unverified
    ) {
        actions.push(Action::ReviewTrust);
    }
    if matches!(
        posture,
        M5AttachmentPillPosture::OutOfScope | M5AttachmentPillPosture::Redacted
    ) {
        actions.push(Action::RevealScope);
    }
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked header resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderResolutionCase {
    /// The resolver input.
    pub input: M5PromptComposerHeaderResolutionInput,
    /// The resolved truth. Must equal `resolve_prompt_composer_header(&input)`.
    pub resolved: M5ResolvedPromptComposerHeader,
}

impl M5PromptComposerHeaderResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PromptComposerHeaderResolutionInput) -> Self {
        let resolved = resolve_prompt_composer_header(&input).expect("seed header case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_prompt_composer_header(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked attachment-pill resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextAttachmentPillResolutionCase {
    /// The resolver input.
    pub input: M5ContextAttachmentPillResolutionInput,
    /// The resolved truth. Must equal `resolve_context_attachment_pill(&input)`.
    pub resolved: M5ResolvedContextAttachmentPill,
}

impl M5ContextAttachmentPillResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ContextAttachmentPillResolutionInput) -> Self {
        let resolved =
            resolve_context_attachment_pill(&input).expect("seed attachment pill case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_context_attachment_pill(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved attachment id preserves the input id exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.attachment_id == self.input.attachment_id
    }
}

/// One row in the primitive matrix: one composition consumer bound to the shared header
/// and pill anatomy, composer modes, scopes, route classes, budget postures, attachment
/// kinds, trust states, header postures, pill postures, bounded actions, export fields,
/// and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillRow {
    /// Composition consumer family.
    pub consumer_surface: M5PromptComposerHeaderPillConsumerSurface,
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
    /// Header anatomy parts this row renders (must include the mandatory parts).
    pub header_anatomy_parts: Vec<M5ComposerHeaderAnatomyPart>,
    /// Pill anatomy parts this row renders (must include the mandatory parts).
    pub pill_anatomy_parts: Vec<M5AttachmentPillAnatomyPart>,
    /// Composer modes this consumer distinguishes.
    pub composer_modes: Vec<M5ComposerMode>,
    /// Composer scopes this consumer distinguishes.
    pub composer_scopes: Vec<M5ComposerScope>,
    /// Route classes this consumer distinguishes.
    pub route_classes: Vec<M5ComposerRouteClass>,
    /// Budget postures this consumer distinguishes.
    pub budget_postures: Vec<M5BudgetPosture>,
    /// Header postures this consumer distinguishes.
    pub header_postures: Vec<M5ComposerHeaderPosture>,
    /// Attachment kinds this consumer distinguishes.
    pub attachment_kinds: Vec<M5AttachmentKind>,
    /// Attachment trust states this consumer distinguishes.
    pub attachment_trust_states: Vec<M5AttachmentTrustState>,
    /// Pill postures this consumer distinguishes.
    pub pill_postures: Vec<M5AttachmentPillPosture>,
    /// Bounded pill actions this consumer offers.
    pub pill_actions: Vec<M5AttachmentPillAction>,
    /// Header export fields this row carries (must include the mandatory fields).
    pub header_export_fields: Vec<M5ComposerHeaderExportField>,
    /// Pill export fields this row carries (must include the mandatory fields).
    pub pill_export_fields: Vec<M5AttachmentPillExportField>,
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
    /// Worked header resolutions proving the header resolver on this consumer.
    pub header_examples: Vec<M5PromptComposerHeaderResolutionCase>,
    /// Worked attachment-pill resolutions proving the pill resolver on this consumer.
    pub pill_examples: Vec<M5ContextAttachmentPillResolutionCase>,
    /// Hard invariant: this consumer never masks its composer mode or route. MUST be
    /// `false`.
    pub masks_mode_or_route: bool,
    /// Hard invariant: this consumer never hides an attachment's freshness or trust
    /// state. MUST be `false`.
    pub hides_attachment_freshness_or_trust: bool,
    /// Hard invariant: this consumer never invents a private composer grammar. MUST be
    /// `false`.
    pub invents_private_composer_grammar: bool,
    /// Hard invariant: this consumer never bypasses the review-before-send gate. MUST be
    /// `false`.
    pub bypasses_review_before_send: bool,
}

impl M5PromptComposerHeaderPillRow {
    /// True when the row declares every mandatory header anatomy part.
    fn declares_mandatory_header_anatomy(&self) -> bool {
        let present: BTreeSet<M5ComposerHeaderAnatomyPart> =
            self.header_anatomy_parts.iter().copied().collect();
        M5ComposerHeaderAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory pill anatomy part.
    fn declares_mandatory_pill_anatomy(&self) -> bool {
        let present: BTreeSet<M5AttachmentPillAnatomyPart> =
            self.pill_anatomy_parts.iter().copied().collect();
        M5AttachmentPillAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory header export field.
    fn declares_mandatory_header_export(&self) -> bool {
        let present: BTreeSet<M5ComposerHeaderExportField> =
            self.header_export_fields.iter().copied().collect();
        M5ComposerHeaderExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory pill export field.
    fn declares_mandatory_pill_export(&self) -> bool {
        let present: BTreeSet<M5AttachmentPillExportField> =
            self.pill_export_fields.iter().copied().collect();
        M5AttachmentPillExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_mode_or_route
            && !self.hides_attachment_freshness_or_trust
            && !self.invents_private_composer_grammar
            && !self.bypasses_review_before_send
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillVocabularySet {
    /// Composition-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Header-anatomy-part tokens.
    pub header_anatomy_parts: Vec<String>,
    /// Pill-anatomy-part tokens.
    pub pill_anatomy_parts: Vec<String>,
    /// Header-posture tokens.
    pub header_postures: Vec<String>,
    /// Pill-posture tokens.
    pub pill_postures: Vec<String>,
    /// Pill-action tokens.
    pub pill_actions: Vec<String>,
    /// Header-export-field tokens.
    pub header_export_fields: Vec<String>,
    /// Pill-export-field tokens.
    pub pill_export_fields: Vec<String>,
    /// Composer-mode tokens (reused from the frozen matrix).
    pub composer_modes: Vec<String>,
    /// Composer-scope tokens (reused from the frozen matrix).
    pub composer_scopes: Vec<String>,
    /// Route-class tokens (reused from the frozen matrix).
    pub route_classes: Vec<String>,
    /// Budget-posture tokens (reused from the frozen matrix).
    pub budget_postures: Vec<String>,
    /// Attachment-kind tokens (reused from the frozen matrix).
    pub attachment_kinds: Vec<String>,
    /// Attachment-trust-state tokens (reused from the frozen matrix).
    pub attachment_trust_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5PromptComposerHeaderPillVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5PromptComposerHeaderPillConsumerSurface::ALL, |v| {
                v.as_str()
            }),
            header_anatomy_parts: tokens(&M5ComposerHeaderAnatomyPart::ALL, |v| v.as_str()),
            pill_anatomy_parts: tokens(&M5AttachmentPillAnatomyPart::ALL, |v| v.as_str()),
            header_postures: tokens(&M5ComposerHeaderPosture::ALL, |v| v.as_str()),
            pill_postures: tokens(&M5AttachmentPillPosture::ALL, |v| v.as_str()),
            pill_actions: tokens(&M5AttachmentPillAction::ALL, |v| v.as_str()),
            header_export_fields: tokens(&M5ComposerHeaderExportField::ALL, |v| v.as_str()),
            pill_export_fields: tokens(&M5AttachmentPillExportField::ALL, |v| v.as_str()),
            composer_modes: tokens(&M5ComposerMode::ALL, |v| v.as_str()),
            composer_scopes: tokens(&M5ComposerScope::ALL, |v| v.as_str()),
            route_classes: tokens(&M5ComposerRouteClass::ALL, |v| v.as_str()),
            budget_postures: tokens(&M5BudgetPosture::ALL, |v| v.as_str()),
            attachment_kinds: tokens(&M5AttachmentKind::ALL, |v| v.as_str()),
            attachment_trust_states: tokens(&M5AttachmentTrustState::ALL, |v| v.as_str()),
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
pub struct M5PromptComposerHeaderPillGovernanceReview {
    /// One primitive pair carries header and pill truth on every consumer.
    pub one_primitive_carries_header_and_pill_truth: bool,
    /// The mode, scope, route, and budget band are shown without a secondary inspector.
    pub mode_scope_route_budget_always_shown: bool,
    /// A blocked route or hard budget block never reads as ready to send.
    pub header_posture_never_masks_blocked: bool,
    /// A local-only route is always disclosed rather than implied managed.
    pub local_only_route_always_disclosed: bool,
    /// An attachment's exact object identity is always preserved.
    pub attachment_identity_always_preserved: bool,
    /// A stale, unverified, or tainted attachment never reads as trusted-fresh.
    pub attachment_freshness_and_trust_never_masked: bool,
    /// The remove action is always offered before send.
    pub remove_action_always_offered: bool,
    /// The support / export packet reconstructs header and pill truth.
    pub support_export_reconstructs_header_and_pill_truth: bool,
    /// No consumer invents a second composer grammar.
    pub no_surface_invents_parallel_vocabulary: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillConsumerProjection {
    /// Inline, side-panel, patch-draft, handoff, and CLI / support consumers all consume
    /// the shared primitive pair.
    pub composition_surfaces_consume_shared_primitive: bool,
    /// The header-posture resolver reads a single canonical source.
    pub header_posture_reads_single_source: bool,
    /// The pill-posture resolver reads a single canonical source.
    pub pill_posture_reads_single_source: bool,
    /// The pill-action derivation reads a single canonical source.
    pub pill_actions_read_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PromptComposerHeaderPillPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PromptComposerHeaderPillPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composition rows.
    pub rows: Vec<M5PromptComposerHeaderPillRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PromptComposerHeaderPillVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PromptComposerHeaderPillGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PromptComposerHeaderPillConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PromptComposerHeaderPillProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PromptComposerHeaderPillReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 prompt-composer-header / context-attachment-pill primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PromptComposerHeaderPillPacket {
    /// Record kind; must equal [`M5_PROMPT_COMPOSER_HEADER_PILL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composition rows.
    pub rows: Vec<M5PromptComposerHeaderPillRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PromptComposerHeaderPillVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PromptComposerHeaderPillGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PromptComposerHeaderPillConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PromptComposerHeaderPillProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PromptComposerHeaderPillReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PromptComposerHeaderPillPacket {
    /// Builds an M5 header/pill-primitive packet from stable-lane input.
    pub fn new(input: M5PromptComposerHeaderPillPacketInput) -> Self {
        Self {
            record_kind: M5_PROMPT_COMPOSER_HEADER_PILL_RECORD_KIND.to_owned(),
            schema_version: M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_VERSION,
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

    /// Validates the M5 header/pill-primitive invariants.
    pub fn validate(&self) -> Vec<M5PromptComposerHeaderPillViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROMPT_COMPOSER_HEADER_PILL_RECORD_KIND {
            violations.push(M5PromptComposerHeaderPillViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_VERSION {
            violations.push(M5PromptComposerHeaderPillViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PromptComposerHeaderPillViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_header_sendability_coverage(self, &mut violations);
        validate_header_local_only_disclosure(self, &mut violations);
        validate_attachment_identity_preservation(self, &mut violations);
        validate_attachment_trust_coverage(self, &mut violations);
        validate_attachment_open_remove_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 header/pill primitive packet serializes"),
        ) {
            violations.push(M5PromptComposerHeaderPillViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 header/pill primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per composition consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,header_anatomy,pill_anatomy,composer_modes,route_classes,header_postures,pill_postures,pill_actions,header_examples,pill_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.header_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.pill_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.composer_modes, |v| v.as_str()),
                join_tokens(&row.route_classes, |v| v.as_str()),
                join_tokens(&row.header_postures, |v| v.as_str()),
                join_tokens(&row.pill_postures, |v| v.as_str()),
                join_tokens(&row.pill_actions, |v| v.as_str()),
                row.header_examples.len(),
                row.pill_examples.len(),
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
        out.push_str("# M5 Prompt-Composer-Header and Context-Attachment-Pill Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Composition consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Header postures: {}\n",
            self.vocabulary_set.header_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Pill postures: {}\n",
            self.vocabulary_set.pill_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Pill actions: {}\n",
            self.vocabulary_set.pill_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Composition consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked headers: {}\n",
                row.header_examples.len()
            ));
            for case in &row.header_examples {
                out.push_str(&format!(
                    "    - `{}` / `{}` on `{}` → `{}` (sendable `{}`, local `{}`)\n",
                    case.resolved.composer_mode.as_str(),
                    case.resolved.composer_scope.as_str(),
                    case.resolved.route_class.as_str(),
                    case.resolved.header_posture.as_str(),
                    case.resolved.is_sendable,
                    case.resolved.route_stays_local,
                ));
            }
            out.push_str(&format!("  - Worked pills: {}\n", row.pill_examples.len()));
            for case in &row.pill_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (openable `{}`, review `{}`)\n",
                    case.resolved.attachment_id,
                    case.resolved.attachment_kind.as_str(),
                    case.resolved.pill_posture.as_str(),
                    case.resolved.is_openable,
                    case.resolved.needs_review_before_send,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 header/pill-primitive export.
#[derive(Debug)]
pub enum M5PromptComposerHeaderPillArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PromptComposerHeaderPillViolation>),
}

impl fmt::Display for M5PromptComposerHeaderPillArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 header/pill primitive export parse failed: {error}"
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
                    "m5 header/pill primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PromptComposerHeaderPillArtifactError {}

/// Validation failures emitted by [`M5PromptComposerHeaderPillPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PromptComposerHeaderPillViolation {
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
    /// A required composition consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A composition row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory header anatomy parts.
    MandatoryHeaderAnatomyMissing,
    /// A row omits one of the mandatory pill anatomy parts.
    MandatoryPillAnatomyMissing,
    /// A row omits one of the mandatory header export fields.
    MandatoryHeaderExportMissing,
    /// A row omits one of the mandatory pill export fields.
    MandatoryPillExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked header resolutions.
    HeaderExampleMissing,
    /// A row declares no worked pill resolutions.
    PillExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked header resolution proves both a sendable and a non-sendable header.
    HeaderSendabilityCoverageUnproven,
    /// No worked header resolution proves a local-only route.
    HeaderLocalOnlyDisclosureUnproven,
    /// A worked pill resolution does not preserve its exact object identity.
    AttachmentIdentityPreservationUnproven,
    /// No worked pill resolution proves both a fresh-trusted and a needs-attention
    /// attachment.
    AttachmentTrustCoverageUnproven,
    /// No worked pill resolution proves an openable attachment with an open action and a
    /// removed-source attachment that still offers remove.
    AttachmentOpenRemoveCoverageUnproven,
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

impl M5PromptComposerHeaderPillViolation {
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
            Self::MandatoryHeaderAnatomyMissing => "mandatory_header_anatomy_missing",
            Self::MandatoryPillAnatomyMissing => "mandatory_pill_anatomy_missing",
            Self::MandatoryHeaderExportMissing => "mandatory_header_export_missing",
            Self::MandatoryPillExportMissing => "mandatory_pill_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::HeaderExampleMissing => "header_example_missing",
            Self::PillExampleMissing => "pill_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::HeaderSendabilityCoverageUnproven => "header_sendability_coverage_unproven",
            Self::HeaderLocalOnlyDisclosureUnproven => "header_local_only_disclosure_unproven",
            Self::AttachmentIdentityPreservationUnproven => {
                "attachment_identity_preservation_unproven"
            }
            Self::AttachmentTrustCoverageUnproven => "attachment_trust_coverage_unproven",
            Self::AttachmentOpenRemoveCoverageUnproven => {
                "attachment_open_remove_coverage_unproven"
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

/// Reads and validates the checked-in stable M5 header/pill-primitive export.
pub fn current_stable_m5_prompt_composer_header_pill_export(
) -> Result<M5PromptComposerHeaderPillPacket, M5PromptComposerHeaderPillArtifactError> {
    let packet: M5PromptComposerHeaderPillPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces/support_export.json"
    )))
    .map_err(M5PromptComposerHeaderPillArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PromptComposerHeaderPillArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_DOC_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_COMPONENT_MATRIX_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_RICHER_COMPOSER_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_ATTACHMENT_PROVENANCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5PromptComposerHeaderPillViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5PromptComposerHeaderPillViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let present: BTreeSet<M5PromptComposerHeaderPillConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5PromptComposerHeaderPillConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5PromptComposerHeaderPillViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.header_anatomy_parts.is_empty()
            || row.pill_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.composer_modes.is_empty()
            || row.composer_scopes.is_empty()
            || row.route_classes.is_empty()
            || row.budget_postures.is_empty()
            || row.header_postures.is_empty()
            || row.attachment_kinds.is_empty()
            || row.attachment_trust_states.is_empty()
            || row.pill_postures.is_empty()
            || row.pill_actions.is_empty()
        {
            violations.push(M5PromptComposerHeaderPillViolation::RowIncomplete);
        }
        if !row.declares_mandatory_header_anatomy() {
            violations.push(M5PromptComposerHeaderPillViolation::MandatoryHeaderAnatomyMissing);
        }
        if !row.declares_mandatory_pill_anatomy() {
            violations.push(M5PromptComposerHeaderPillViolation::MandatoryPillAnatomyMissing);
        }
        if !row.declares_mandatory_header_export() {
            violations.push(M5PromptComposerHeaderPillViolation::MandatoryHeaderExportMissing);
        }
        if !row.declares_mandatory_pill_export() {
            violations.push(M5PromptComposerHeaderPillViolation::MandatoryPillExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5PromptComposerHeaderPillViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5PromptComposerHeaderPillViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5PromptComposerHeaderPillViolation::DowngradeTriggersMissing);
        }
        if row.header_examples.is_empty() {
            violations.push(M5PromptComposerHeaderPillViolation::HeaderExampleMissing);
        }
        if row.pill_examples.is_empty() {
            violations.push(M5PromptComposerHeaderPillViolation::PillExampleMissing);
        }
        if row
            .header_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .pill_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5PromptComposerHeaderPillViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5PromptComposerHeaderPillViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5PromptComposerHeaderPillViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked header resolution across the matrix must prove a sendable header
/// and at least one must prove a non-sendable (route- or budget-blocked) header — the
/// acceptance-criterion example that a blocked route never reads as ready to send.
fn validate_header_sendability_coverage(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let has_sendable = packet.rows.iter().any(|row| {
        row.header_examples
            .iter()
            .any(|case| case.resolved.is_sendable)
    });
    let has_blocked = packet.rows.iter().any(|row| {
        row.header_examples
            .iter()
            .any(|case| !case.resolved.is_sendable)
    });
    if !(has_sendable && has_blocked) {
        violations.push(M5PromptComposerHeaderPillViolation::HeaderSendabilityCoverageUnproven);
    }
}

/// At least one worked header resolution must prove a route that stays on the local
/// device — the acceptance-criterion example that a local-only route is never hidden.
fn validate_header_local_only_disclosure(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.header_examples
            .iter()
            .any(|case| case.resolved.route_stays_local)
    });
    if !proven {
        violations.push(M5PromptComposerHeaderPillViolation::HeaderLocalOnlyDisclosureUnproven);
    }
}

/// Every worked pill resolution must preserve its exact object identity — the
/// acceptance-criterion example that an attachment row preserves exact object identity
/// before send.
fn validate_attachment_identity_preservation(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.pill_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations
            .push(M5PromptComposerHeaderPillViolation::AttachmentIdentityPreservationUnproven);
    }
}

/// At least one worked pill resolution must prove a fresh-trusted attachment and at least
/// one must prove a needs-attention (tainted, unverified, stale, or out-of-scope)
/// attachment — the acceptance-criterion example that freshness / trust state is never
/// hidden.
fn validate_attachment_trust_coverage(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let has_fresh = packet.rows.iter().any(|row| {
        row.pill_examples.iter().any(|case| {
            matches!(
                case.resolved.pill_posture,
                M5AttachmentPillPosture::FreshTrusted
            )
        })
    });
    let has_attention = packet.rows.iter().any(|row| {
        row.pill_examples
            .iter()
            .any(|case| case.resolved.needs_attention)
    });
    if !(has_fresh && has_attention) {
        violations.push(M5PromptComposerHeaderPillViolation::AttachmentTrustCoverageUnproven);
    }
}

/// At least one worked pill resolution must prove an openable attachment that offers an
/// open action, and at least one must prove a removed-source attachment that is not
/// openable but still offers remove — the acceptance-criterion example that remove / open
/// behavior is preserved before send.
fn validate_attachment_open_remove_coverage(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let has_openable = packet.rows.iter().any(|row| {
        row.pill_examples.iter().any(|case| {
            case.resolved.is_openable
                && case
                    .resolved
                    .available_actions
                    .contains(&M5AttachmentPillAction::Open)
        })
    });
    let has_removed_still_removable = packet.rows.iter().any(|row| {
        row.pill_examples.iter().any(|case| {
            !case.resolved.is_openable
                && case
                    .resolved
                    .available_actions
                    .contains(&M5AttachmentPillAction::Remove)
        })
    });
    if !(has_openable && has_removed_still_removable) {
        violations.push(M5PromptComposerHeaderPillViolation::AttachmentOpenRemoveCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_header_and_pill_truth,
        review.mode_scope_route_budget_always_shown,
        review.header_posture_never_masks_blocked,
        review.local_only_route_always_disclosed,
        review.attachment_identity_always_preserved,
        review.attachment_freshness_and_trust_never_masked,
        review.remove_action_always_offered,
        review.support_export_reconstructs_header_and_pill_truth,
        review.no_surface_invents_parallel_vocabulary,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5PromptComposerHeaderPillViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.composition_surfaces_consume_shared_primitive,
        projection.header_posture_reads_single_source,
        projection.pill_posture_reads_single_source,
        projection.pill_actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5PromptComposerHeaderPillViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5PromptComposerHeaderPillViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PromptComposerHeaderPillPacket,
    violations: &mut Vec<M5PromptComposerHeaderPillViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5PromptComposerHeaderPillViolation::ReleasePostureIncomplete);
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
