//! Shared consumers for the reusable M5 prompt-composer components, so the
//! composer header, context-attachment pill, mention resolver, slash-command row,
//! budget / size strip, tainted-context warning, draft-state row, attachment-stale
//! banner, and split-send / review control keep locality, route/provider/model,
//! approval, and taint truth aligned across every claimed M5 surface where a user
//! composes, reviews, hands off, or inspects an AI request before it leaves the
//! shell.
//!
//! Aureline's frozen prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! names the nine governed prompt-composer component families, and four sibling
//! `implement_*` / `ship_*` lanes narrow those families into working primitives,
//! each with its own canonical schema, contract doc, and support-export artifact:
//!
//! * the prompt-composer header / context-attachment pill
//!   ([`crate::implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces`]),
//! * the mention resolver / slash-command row
//!   ([`crate::ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces`]),
//! * the budget / size strip / tainted-context warning
//!   ([`crate::implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes`]),
//!   and
//! * the draft-state row / attachment-stale banner / split-send-review control
//!   ([`crate::ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces`]).
//!
//! This module is the *adoption* lane over those primitives. It proves the nine
//! families are reusable components — not one inline composer plus a few isolated
//! design objects — by binding every claimed M5 composer consumer (the inline /
//! panel composer, patch review, the branch-agent console, the docs/help surface,
//! and the companion composer) to the same canonical component schemas and the same
//! descriptor vocabulary. Each consumer points at the primitive's canonical schema
//! and support-export artifact rather than re-wording locality, route, approval, or
//! taint facts in local prose, and each keeps that vocabulary truthful even when the
//! surrounding workflow becomes review-only, handoff-only, offline / mirrored, or
//! companion-scoped.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_composer_binding`] — that takes one consumer's adoption
//!    of one component family, the descriptor set it surfaces, the parity-health
//!    mode it renders under, and any export caveats, and produces one
//!    [`M5ComposerResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5ComposerAutoNarrowBanner`]
//!    that names the exact reason (review-only workflow, handoff-only workflow,
//!    offline / mirror scope, or companion-scope limit), the descriptors that stay
//!    preserved, and the recovery action, rather than a generic "degraded" note.
//!    The resolver never lets a narrowed context drop a required descriptor and never
//!    invents a second composer grammar.
//! 2. A parity matrix — [`M5ComposerComponentConsumerPacket`] — that binds one row
//!    per claimed M5 composer consumer to the nine canonical component families, the
//!    one shared descriptor vocabulary, the same parity-health modes, export caveats,
//!    parity states, narrowing reasons, recovery actions, export fields, and
//!    non-visual accessibility routes, so locality / route / approval / taint facts
//!    stop diverging between the product UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the nine component families
//! themselves are reused verbatim from the frozen prompt-composer component matrix.
//! This module mints new vocabulary only for what the adoption lane itself needs: its
//! composer consumers, the shared descriptor vocabulary, the parity-health modes, the
//! export caveats, the claim-parity states, the narrowing reasons and recovery
//! actions, the consumer anatomy parts, and the export fields.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary; every label is carried only as an opaque,
//! export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-prompt-composer-component-consumer.schema.json`](../../../../schemas/ai/m5-prompt-composer-component-consumer.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers.md`](../../../../docs/ai/m5/add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/m5-prompt-composer-component-consumers/`](../../../../fixtures/ai/m5/m5-prompt-composer-component-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_prompt_composer_component_consumer_branch_agent_beta_narrowed,
    seeded_m5_prompt_composer_component_consumer_companion_preview_narrowed,
    seeded_m5_prompt_composer_component_consumer_packet,
    M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the nine component families are
// frozen once, in the prompt-composer component matrix. This adoption lane reuses
// them verbatim so it never invents a parallel composer vocabulary.
pub use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5ComposerAccessibilityRoute, M5ComposerConsumerSurface, M5ComposerDeploymentLine,
    M5ComposerDowngradeTrigger, M5ComposerQualificationClass, M5ComposerSurfaceFamily,
    M5PromptComposerComponentFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer
// at, rather than re-wording their facts in local prose.
use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5_PROMPT_COMPOSER_COMPONENT_DOC_REF, M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_budget_size_strips_omitted_context_drawers_and_tainted_context_warnings_with_token_pressure_truncation_route_change_and_review_before_send_truth_across_claimed_m5_ai_lanes::{
    M5_BUDGET_TAINT_ARTIFACT_REF, M5_BUDGET_TAINT_DOC_REF, M5_BUDGET_TAINT_SCHEMA_REF,
};
use crate::implement_prompt_composer_headers_and_context_attachment_pills_with_mode_scope_route_budget_freshness_trust_truth_across_claimed_m5_ai_composition_surfaces::{
    M5_PROMPT_COMPOSER_HEADER_PILL_ARTIFACT_REF, M5_PROMPT_COMPOSER_HEADER_PILL_DOC_REF,
    M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF,
};
use crate::ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces::{
    M5_DRAFT_SEND_ARTIFACT_REF, M5_DRAFT_SEND_DOC_REF, M5_DRAFT_SEND_SCHEMA_REF,
};
use crate::ship_mention_resolvers_and_slash_command_rows_with_exact_target_previews_ambiguity_review_stable_command_ids_and_disabled_state_explanations_across_claimed_m5_composer_surfaces::{
    M5_MENTION_SLASH_COMMAND_ARTIFACT_REF, M5_MENTION_SLASH_COMMAND_DOC_REF,
    M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ComposerComponentConsumerPacket`].
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers";

/// Schema version for M5 prompt-composer-component-consumer records.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the prompt-composer-component-consumer boundary schema.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ai/m5-prompt-composer-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/ai/m5/add_shared_inline_panel_patch_review_branch_agent_docs_help_and_companion_prompt_composer_component_consumers.md";

/// Repo-relative path of the frozen prompt-composer component matrix this lane
/// adopts from.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_PROMPT_COMPOSER_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_PROMPT_COMPOSER_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ai/m5/m5-prompt-composer-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family.
/// A consumer that adopts a family must point at this schema, not a local
/// re-description.
pub const fn family_canonical_schema_ref(family: M5PromptComposerComponentFamily) -> &'static str {
    use M5PromptComposerComponentFamily as Family;
    match family {
        Family::PromptComposerHeader | Family::ContextAttachmentPill => {
            M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF
        }
        Family::MentionResolver | Family::SlashCommandRow => M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
        Family::BudgetSizeStrip | Family::TaintedContextWarning => M5_BUDGET_TAINT_SCHEMA_REF,
        Family::DraftStateRow | Family::AttachmentStaleBanner | Family::SendReviewControl => {
            M5_DRAFT_SEND_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5PromptComposerComponentFamily) -> &'static str {
    use M5PromptComposerComponentFamily as Family;
    match family {
        Family::PromptComposerHeader | Family::ContextAttachmentPill => {
            M5_PROMPT_COMPOSER_HEADER_PILL_DOC_REF
        }
        Family::MentionResolver | Family::SlashCommandRow => M5_MENTION_SLASH_COMMAND_DOC_REF,
        Family::BudgetSizeStrip | Family::TaintedContextWarning => M5_BUDGET_TAINT_DOC_REF,
        Family::DraftStateRow | Family::AttachmentStaleBanner | Family::SendReviewControl => {
            M5_DRAFT_SEND_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a
/// family.
pub const fn family_canonical_artifact_ref(
    family: M5PromptComposerComponentFamily,
) -> &'static str {
    use M5PromptComposerComponentFamily as Family;
    match family {
        Family::PromptComposerHeader | Family::ContextAttachmentPill => {
            M5_PROMPT_COMPOSER_HEADER_PILL_ARTIFACT_REF
        }
        Family::MentionResolver | Family::SlashCommandRow => M5_MENTION_SLASH_COMMAND_ARTIFACT_REF,
        Family::BudgetSizeStrip | Family::TaintedContextWarning => M5_BUDGET_TAINT_ARTIFACT_REF,
        Family::DraftStateRow | Family::AttachmentStaleBanner | Family::SendReviewControl => {
            M5_DRAFT_SEND_ARTIFACT_REF
        }
    }
}

/// One claimed M5 prompt-composer-component consumer that adopts the shared
/// components. These are the consumers the spec names — the inline / panel composer,
/// patch review, the branch-agent console, the docs/help surface, and the companion
/// composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerComponentConsumer {
    /// The inline / panel composer surface.
    InlinePanel,
    /// The patch-review surface.
    PatchReview,
    /// The branch-agent console surface.
    BranchAgent,
    /// The docs/help surface.
    DocsHelp,
    /// The companion composer surface.
    Companion,
}

impl M5ComposerComponentConsumer {
    /// Every claimed composer-component consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::InlinePanel,
        Self::PatchReview,
        Self::BranchAgent,
        Self::DocsHelp,
        Self::Companion,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlinePanel => "inline_panel",
            Self::PatchReview => "patch_review",
            Self::BranchAgent => "branch_agent",
            Self::DocsHelp => "docs_help",
            Self::Companion => "companion",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::InlinePanel => "Inline / Panel Composer",
            Self::PatchReview => "Patch Review",
            Self::BranchAgent => "Branch-Agent Console",
            Self::DocsHelp => "Docs / Help",
            Self::Companion => "Companion Composer",
        }
    }

    /// True when this consumer is a docs/help surface — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_docs_or_help(self) -> bool {
        matches!(self, Self::DocsHelp)
    }
}

/// The one shared descriptor vocabulary every prompt-composer component keeps aligned
/// across surfaces, so no consumer invents a new grammar or stale wording. The
/// descriptors in [`M5ComposerParityDescriptor::REQUIRED`] must be present on every
/// binding — the track invariant that locality, route, approval, and taint stay
/// explicit everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerParityDescriptor {
    /// The draft locality / retention descriptor.
    Locality,
    /// The route / provider / model descriptor.
    Route,
    /// The approval / send-gate descriptor.
    Approval,
    /// The trust / taint descriptor.
    Taint,
}

impl M5ComposerParityDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [Self::Locality, Self::Route, Self::Approval, Self::Taint];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Locality => "locality",
            Self::Route => "route",
            Self::Approval => "approval",
            Self::Taint => "taint",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still
/// keeps the descriptor vocabulary — it only discloses that parity is narrowed
/// relative to the authoritative live composer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerParityHealth {
    /// Full parity: the authoritative live-composer rendering.
    FullParity,
    /// A review-only workflow weakens parity.
    ReviewOnlyNarrowed,
    /// A handoff-only workflow weakens parity.
    HandoffOnlyNarrowed,
    /// An offline / mirrored scope weakens parity.
    OfflineMirrorNarrowed,
    /// A companion-scope limit weakens parity.
    CompanionScopeNarrowed,
}

impl M5ComposerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ReviewOnlyNarrowed,
        Self::HandoffOnlyNarrowed,
        Self::OfflineMirrorNarrowed,
        Self::CompanionScopeNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ReviewOnlyNarrowed => "review_only_narrowed",
            Self::HandoffOnlyNarrowed => "handoff_only_narrowed",
            Self::OfflineMirrorNarrowed => "offline_mirror_narrowed",
            Self::CompanionScopeNarrowed => "companion_scope_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must
    /// disclose a self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5ComposerParityNarrowingReason> {
        Some(match self {
            Self::ReviewOnlyNarrowed => M5ComposerParityNarrowingReason::ReviewOnlyWorkflow,
            Self::HandoffOnlyNarrowed => M5ComposerParityNarrowingReason::HandoffOnlyWorkflow,
            Self::OfflineMirrorNarrowed => M5ComposerParityNarrowingReason::OfflineOrMirrorScope,
            Self::CompanionScopeNarrowed => M5ComposerParityNarrowingReason::CompanionScopeLimited,
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an
/// auto-narrow banner never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerParityNarrowingReason {
    /// The surrounding workflow is review-only, so the live send path is disabled.
    ReviewOnlyWorkflow,
    /// The surrounding workflow is handoff-only, so the draft continues elsewhere.
    HandoffOnlyWorkflow,
    /// The scope is offline or mirrored, so route / provider is shown from a mirror.
    OfflineOrMirrorScope,
    /// The companion scope cannot yet preserve every component truth in full.
    CompanionScopeLimited,
}

impl M5ComposerParityNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewOnlyWorkflow,
        Self::HandoffOnlyWorkflow,
        Self::OfflineOrMirrorScope,
        Self::CompanionScopeLimited,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewOnlyWorkflow => "review_only_workflow",
            Self::HandoffOnlyWorkflow => "handoff_only_workflow",
            Self::OfflineOrMirrorScope => "offline_or_mirror_scope",
            Self::CompanionScopeLimited => "companion_scope_limited",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ReviewOnlyWorkflow => {
                "the surrounding workflow is review-only, so the live send path is disabled here"
            }
            Self::HandoffOnlyWorkflow => {
                "the surrounding workflow is handoff-only, so the draft continues in its originating composer"
            }
            Self::OfflineOrMirrorScope => {
                "the scope is offline or mirrored, so the route and provider are shown from a mirror, not the live endpoint"
            }
            Self::CompanionScopeLimited => {
                "the companion scope cannot yet preserve every component truth in full"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5ComposerParityRecoveryAction {
        match self {
            Self::ReviewOnlyWorkflow => M5ComposerParityRecoveryAction::ReturnToLiveComposerToSend,
            Self::HandoffOnlyWorkflow => {
                M5ComposerParityRecoveryAction::ResumeInOriginatingComposer
            }
            Self::OfflineOrMirrorScope => M5ComposerParityRecoveryAction::ReconnectToLiveRoute,
            Self::CompanionScopeLimited => M5ComposerParityRecoveryAction::OpenInFullComposer,
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is
/// actionable from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerParityRecoveryAction {
    /// Return to the live composer to send.
    ReturnToLiveComposerToSend,
    /// Resume the draft in its originating composer.
    ResumeInOriginatingComposer,
    /// Reconnect to the live route before trusting route / provider truth.
    ReconnectToLiveRoute,
    /// Open the component in the full composer.
    OpenInFullComposer,
}

impl M5ComposerParityRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReturnToLiveComposerToSend,
        Self::ResumeInOriginatingComposer,
        Self::ReconnectToLiveRoute,
        Self::OpenInFullComposer,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturnToLiveComposerToSend => "return_to_live_composer_to_send",
            Self::ResumeInOriginatingComposer => "resume_in_originating_composer",
            Self::ReconnectToLiveRoute => "reconnect_to_live_route",
            Self::OpenInFullComposer => "open_in_full_composer",
        }
    }
}

/// An export caveat a consumer preserves when a component renders outside the live
/// composer (a review-only send lock, a handoff-only draft, a mirrored route, or a
/// reduced companion scope).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerConsumerExportCaveat {
    /// The send path is disabled because the workflow is review-only.
    SendPathDisabledReviewOnly,
    /// The draft is handoff-only and continues in its originating composer.
    DraftHandoffOnly,
    /// The route is shown from a mirror / cache, not the live provider.
    RouteMirroredNotLive,
    /// The companion scope is reduced relative to the full composer.
    CompanionScopeReduced,
}

impl M5ComposerConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SendPathDisabledReviewOnly,
        Self::DraftHandoffOnly,
        Self::RouteMirroredNotLive,
        Self::CompanionScopeReduced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SendPathDisabledReviewOnly => "send_path_disabled_review_only",
            Self::DraftHandoffOnly => "draft_handoff_only",
            Self::RouteMirroredNotLive => "route_mirrored_not_live",
            Self::CompanionScopeReduced => "companion_scope_reduced",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor
/// vocabulary is preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5ComposerClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5ComposerConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5ComposerConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable
/// from the shared model. The fields in [`M5ComposerConsumerExportField::MANDATORY`]
/// are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ComposerConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5ComposerConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay
/// preserved, the export caveats, and the recovery action, so a narrowed rendering is
/// understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5ComposerParityNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5ComposerParityRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5ComposerComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5PromptComposerComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5ComposerParityDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5ComposerConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// descriptors, and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the composer-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5ComposerComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5PromptComposerComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor
    /// so locality, route, approval, and taint stay explicit.
    pub descriptor_families: Vec<M5ComposerParityDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5ComposerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5ComposerConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerResolvedBinding {
    /// The consumer.
    pub consumer: M5ComposerComponentConsumer,
    /// The component family.
    pub component_family: M5PromptComposerComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5ComposerParityDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5ComposerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5ComposerConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5ComposerClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5ComposerAutoNarrowBanner>,
}

/// Errors returned by [`resolve_composer_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ComposerBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5ComposerBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5ComposerBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "composer binding error: {}", self.as_str())
    }
}

impl Error for M5ComposerBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the track invariant that locality,
/// route, approval, and taint stay explicit on every surface. The claim-parity state
/// is preserved at full parity and auto-narrowed under any weakened parity-health
/// mode, and a weakened mode always produces a self-contained banner naming the exact
/// reason and recovery action while keeping the descriptor vocabulary intact.
pub fn resolve_composer_binding(
    input: &M5ComposerBindingInput,
) -> Result<M5ComposerResolvedBinding, M5ComposerBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5ComposerBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5ComposerParityDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5ComposerParityDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5ComposerBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5ComposerBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future
        // free-text extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5ComposerBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let claim_parity_state = if is_narrowed {
        M5ComposerClaimParityState::ClaimsAutoNarrowed
    } else {
        M5ComposerClaimParityState::ClaimsPreserved
    };

    let auto_narrow_banner = input.parity_health.narrowing_reason().map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5ComposerAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5ComposerResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerBindingCase {
    /// The resolver input.
    pub input: M5ComposerBindingInput,
    /// The resolved truth. Must equal `resolve_composer_binding(&input)`.
    pub resolved: M5ComposerResolvedBinding,
}

impl M5ComposerBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5ComposerBindingInput) -> Self {
        let resolved = resolve_composer_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_composer_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5PromptComposerComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal
    /// the family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5ComposerBindingCase>,
}

impl M5ComposerComponentBinding {
    /// True when the binding points at the family's canonical refs and references the
    /// canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one composer-component consumer bound to the
/// canonical component families, the shared descriptor vocabulary, the parity-health
/// modes, export caveats, parity states, narrowing reasons, recovery actions, export
/// fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerRow {
    /// Composer-component consumer.
    pub consumer: M5ComposerComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ComposerQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 composer surface families that render / consume this projection.
    pub surface_families: Vec<M5ComposerSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5ComposerDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5ComposerConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5ComposerParityDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5ComposerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5ComposerConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5ComposerClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5ComposerParityNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5ComposerParityRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5ComposerConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ComposerAccessibilityRoute>,
    /// Composer subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComposerConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ComposerDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5ComposerComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be
    /// `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new composer grammar. MUST be
    /// `false`.
    pub invents_new_composer_grammar: bool,
    /// Hard invariant: this consumer never drops locality, route, approval, or taint
    /// truth when narrowed. MUST be `false`.
    pub drops_locality_route_approval_or_taint_when_narrowed: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier
    /// surface instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_surface: bool,
}

impl M5ComposerComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ComposerConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ComposerConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ComposerConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5ComposerConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5ComposerParityDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5ComposerParityDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5ComposerComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5PromptComposerComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_composer_grammar
            && !self.drops_locality_route_approval_or_taint_when_narrowed
            && !self.inherits_stronger_label_from_healthier_surface
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerVocabularySet {
    /// Composer-component-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5ComposerComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5ComposerComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5PromptComposerComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5ComposerParityDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5ComposerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5ComposerConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5ComposerParityNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5ComposerParityRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5ComposerClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5ComposerConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ComposerConsumerExportField::ALL, |v| v.as_str()),
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
pub struct M5ComposerComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new composer grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Locality, route, approval, and taint stay explicit everywhere.
    pub locality_route_approval_taint_explicit_on_every_surface: bool,
    /// Review-only, handoff-only, offline / mirror, and companion scopes auto-narrow
    /// the claim.
    pub degraded_workflow_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// Help and companion consumers present the same locality and route truth shown
    /// in-product.
    pub help_and_companion_present_same_locality_and_route_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerProjection {
    /// The inline / panel composer, patch review, branch-agent console, docs/help, and
    /// companion composer all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The locality descriptor reads a single canonical source.
    pub locality_reads_single_source: bool,
    /// The route descriptor reads a single canonical source.
    pub route_reads_single_source: bool,
    /// The approval descriptor reads a single canonical source.
    pub approval_reads_single_source: bool,
    /// The taint descriptor reads a single canonical source.
    pub taint_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI consumer audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ComposerComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ComposerComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ComposerComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ComposerComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ComposerComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ComposerComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ComposerComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ComposerComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 prompt-composer-component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ComposerComponentConsumerPacket {
    /// Record kind; must equal [`M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5ComposerComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ComposerComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ComposerComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ComposerComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ComposerComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ComposerComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ComposerComponentConsumerPacket {
    /// Builds an M5 prompt-composer-component-consumer packet from stable-lane input.
    pub fn new(input: M5ComposerComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
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

    /// Validates the M5 prompt-composer-component-consumer invariants.
    pub fn validate(&self) -> Vec<M5ComposerComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5ComposerComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5ComposerComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ComposerComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_docs_help_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 prompt-composer component consumer packet serializes"),
        ) {
            violations.push(M5ComposerComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 prompt-composer component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Prompt-Composer-Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Composer-component consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Composer-component consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 prompt-composer-component-consumer
/// export.
#[derive(Debug)]
pub enum M5ComposerComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ComposerComponentConsumerViolation>),
}

impl fmt::Display for M5ComposerComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 prompt-composer component consumer export parse failed: {error}"
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
                    "m5 prompt-composer component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ComposerComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5ComposerComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ComposerComponentConsumerViolation {
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
    /// A required composer-component consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one
    /// consumer (reuse across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// A docs/help consumer does not reference the canonical component schema.
    DocsHelpReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
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

impl M5ComposerComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::DocsHelpReferenceMissing => "docs_help_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 prompt-composer-component-consumer
/// export.
pub fn current_stable_m5_prompt_composer_component_consumer_export(
) -> Result<M5ComposerComponentConsumerPacket, M5ComposerComponentConsumerArtifactError> {
    let packet: M5ComposerComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/m5-prompt-composer-component-consumer-proof/support_export.json"
    )))
    .map_err(M5ComposerComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ComposerComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_DOC_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_PROMPT_COMPOSER_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_PROMPT_COMPOSER_HEADER_PILL_SCHEMA_REF,
        M5_MENTION_SLASH_COMMAND_SCHEMA_REF,
        M5_BUDGET_TAINT_SCHEMA_REF,
        M5_DRAFT_SEND_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ComposerComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ComposerComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let present: BTreeSet<M5ComposerComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5ComposerComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5ComposerComponentConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5ComposerComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ComposerComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5ComposerComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ComposerComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ComposerComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ComposerComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ComposerComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5ComposerComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5ComposerComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5ComposerComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5ComposerComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ComposerComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ComposerComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers
/// — the acceptance-criterion proof that the families are reusable components rather
/// than one inline composer plus a few isolated design objects.
fn validate_family_reuse(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    for family in M5PromptComposerComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5ComposerComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose
/// banner carries a specific reason, a recovery action, and a non-empty set of
/// preserved descriptors — the acceptance-criterion example that composer components
/// stay truthful when the surrounding workflow becomes review-only, handoff-only,
/// offline / mirrored, or companion-scoped.
fn validate_narrowing_disclosure(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5ComposerComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering
/// with preserved parity and no banner — the acceptance-criterion example that
/// full-parity consumers keep the descriptor vocabulary without a spurious narrowing
/// note.
fn validate_scope_preserved(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5ComposerClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5ComposerComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every docs/help consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that docs/help prose can never drift
/// from the product truth.
fn validate_docs_help_reference(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_docs_or_help() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5ComposerComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5ComposerComponentConsumerViolation::DocsHelpReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.locality_route_approval_taint_explicit_on_every_surface,
        review.degraded_workflow_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.help_and_companion_present_same_locality_and_route_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5ComposerComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.locality_reads_single_source,
        projection.route_reads_single_source,
        projection.approval_reads_single_source,
        projection.taint_reads_single_source,
    ] {
        if !ok {
            violations.push(M5ComposerComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ComposerComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ComposerComponentConsumerPacket,
    violations: &mut Vec<M5ComposerComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ComposerComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5ComposerComponentConsumerPacket,
) -> impl Iterator<Item = &M5ComposerBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces
/// a stray comma.
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
