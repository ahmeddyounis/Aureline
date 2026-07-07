//! Three reusable M5 prompt-composer primitives — the draft-state row, the
//! offline-local-only / attachment-stale banner, and the split-send-or-review control — so
//! pre-send composition is honest about where a draft lives and how long it is retained, keeps a
//! draft intact when an attachment goes stale or the route drops offline, and never collapses a
//! high-authority send into one unqualified affordance.
//!
//! Aureline's frozen prompt-composer component matrix
//! ([`crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix`])
//! names the draft-state row, the attachment-stale banner, and the send-review control as three
//! governed component families and freezes their controlled vocabulary — the draft localities,
//! the staleness reasons, the send postures, and the review requirements, plus the surface
//! families, deployment lines, consumer surfaces, accessibility routes, qualification classes,
//! and downgrade triggers. This module *implements* those three contracts as reusable primitives
//! so a user can tell — from the row, the banner, or the control alone — whether a draft is
//! local-only or synced or shared and how long it is retained, whether a stale attachment or an
//! offline route still preserves the current draft and what refresh or local-safe path resolves
//! it, and, for any route that widens authority, which explain-only / review / mutating send path
//! they are choosing instead of one ambiguous send.
//!
//! The module has three resolvers:
//!
//! 1. [`resolve_draft_state_row`] — takes one draft's locality, saved state, shared-or-retained
//!    exception, sync / policy note, and clear / delete availability, and produces one
//!    [`M5ResolvedDraftStateRow`] carrying the derived [`M5DraftRetentionPosture`], whether the
//!    draft leaves the device, whether every non-local draft discloses its sharing, the bounded
//!    view / save / clear / delete / stop-sharing actions, and whether the row makes a
//!    hidden-sharing assumption. It never shows a synced or shared draft as local-only and never
//!    lets a draft leave the device without disclosing it.
//! 2. [`resolve_attachment_stale_banner`] — takes one attachment's offline-local-only state, its
//!    staleness reason (if any), whether a refresh or a local-safe alternative exists, and its
//!    recovery note, and produces one [`M5ResolvedAttachmentStaleBanner`] carrying the derived
//!    [`M5StaleBannerPosture`] (fresh / offline-local-only / stale-refreshable /
//!    stale-superseded-review / stale-source-gone / stale-access-revoked), the always-preserved
//!    draft, the bounded refresh / review / local-safe-alternative / detach / keep-local actions,
//!    and whether a resolution path is offered instead of a silent retry loop. It never drops the
//!    draft and never leaves a stale or offline state with no way forward.
//! 3. [`resolve_send_review_control`] — takes one send control's route, whether the route widens
//!    authority, whether it is mutating, its pending review requirements, and its policy / budget
//!    / taint blockers, and produces one [`M5ResolvedSendReviewControl`] carrying the derived
//!    [`M5SendPosture`], the bounded explain-only / review-then-send / direct-send paths, whether
//!    the control is split (never one unqualified send on widened authority), and whether review
//!    is required before send. It never collapses a high-authority send into a single ambiguous
//!    affordance.
//!
//! A single parity matrix — [`M5DraftSendPacket`] — binds one row per claimed M5 composer
//! consumer that can hold a draft and send an AI request (the inline composer, the side panel,
//! the patch draft, the CLI / headless surface, and the support export) to the shared draft,
//! stale-banner, and send-control anatomy, the same draft localities, retention postures,
//! staleness reasons, banner postures, send postures, send paths, review requirements, bounded
//! actions, export fields, and non-visual accessibility routes, so the draft, stale, and send
//! grammar stays identical across every send-capable surface rather than drifting into a
//! separate AI-only grammar.
//!
//! The draft locality ([`M5DraftLocality`]), staleness reason ([`M5StalenessReason`]), send
//! posture ([`M5SendPosture`]), review requirement ([`M5ReviewRequirement`]), route class
//! ([`M5ComposerRouteClass`]), surface family ([`M5ComposerSurfaceFamily`]), deployment line
//! ([`M5ComposerDeploymentLine`]), consumer surface ([`M5ComposerConsumerSurface`]),
//! accessibility route ([`M5ComposerAccessibilityRoute`]), qualification class
//! ([`M5ComposerQualificationClass`]), and downgrade trigger ([`M5ComposerDowngradeTrigger`]) are
//! reused verbatim from the frozen matrix. This module mints new vocabulary only for what that
//! matrix left implicit about the three components themselves: their send-capable consumers, the
//! retention postures, the stale-banner postures, the send paths, their anatomy parts, their
//! bounded actions, and their export fields. No M5 composer surface invents a second draft,
//! stale, or send grammar.
//!
//! Raw prompts, draft bodies, attachment bodies, raw paths, raw URLs, credentials, and private
//! endpoints stay outside the support boundary; every draft id, banner id, control id, draft
//! label, attachment label, and note is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ai/m5-draft-state-row-attachment-stale-banner-and-send-review-control.schema.json`](../../../../schemas/ai/m5-draft-state-row-attachment-stale-banner-and-send-review-control.schema.json)
//! and the contract doc is
//! [`docs/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces.md`](../../../../docs/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_draft_send_cli_headless_beta_narrowed, seeded_m5_draft_send_packet,
    seeded_m5_draft_send_patch_draft_preview_narrowed, M5_DRAFT_SEND_PACKET_ID,
};

// The draft locality, staleness reason, send posture, review requirement, route class, surface
// family, deployment line, consumer surface, accessibility route, qualification class, and
// downgrade triggers are frozen once, in the prompt-composer component matrix. These primitives
// reuse them verbatim so they never invent a parallel draft / stale / send vocabulary.
pub use crate::freeze_the_m5_prompt_composer_header_context_attachment_pill_mention_resolver_slash_command_row_budget_strip_tainted_context_warning_and_draft_state_component_matrix::{
    M5ComposerAccessibilityRoute, M5ComposerConsumerSurface, M5ComposerDeploymentLine,
    M5ComposerDowngradeTrigger, M5ComposerQualificationClass, M5ComposerRouteClass,
    M5ComposerSurfaceFamily, M5DraftLocality, M5ReviewRequirement, M5SendPosture, M5StalenessReason,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DraftSendPacket`].
pub const M5_DRAFT_SEND_RECORD_KIND: &str =
    "ship_m5_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces";

/// Schema version for M5 draft-state-row / stale-banner / send-review-control records.
pub const M5_DRAFT_SEND_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the draft-state-row / stale-banner / send-review-control boundary schema.
pub const M5_DRAFT_SEND_SCHEMA_REF: &str =
    "schemas/ai/m5-draft-state-row-attachment-stale-banner-and-send-review-control.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DRAFT_SEND_DOC_REF: &str =
    "docs/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces.md";

/// Repo-relative path of the frozen prompt-composer component matrix these primitives narrow
/// from.
pub const M5_DRAFT_SEND_COMPONENT_MATRIX_REF: &str =
    "schemas/ai/freeze-the-m5-prompt-composer-header-context-attachment-pill-mention-resolver-slash-command-row-budget-strip-tainted-context-warning-and-draft-state-component-matrix.schema.json";

/// Repo-relative path of the prompt-composer-draft record contract the draft-state row binds its
/// locality and retention truth against.
pub const M5_DRAFT_SEND_PROMPT_COMPOSER_DRAFT_REF: &str =
    "schemas/ai/prompt_composer_draft.schema.json";

/// Repo-relative path of the prompt-context-attachment record contract the stale banner binds its
/// staleness truth against.
pub const M5_DRAFT_SEND_CONTEXT_ATTACHMENT_REF: &str =
    "schemas/ai/prompt_context_attachment.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DRAFT_SEND_FIXTURE_DIR: &str =
    "fixtures/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DRAFT_SEND_ARTIFACT_REF: &str =
    "artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DRAFT_SEND_CSV_REF: &str =
    "artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DRAFT_SEND_REPORT_REF: &str =
    "artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces.md";

/// One claimed M5 composer consumer where the user can hold a draft and send an AI request and
/// therefore must see the shared draft-state row, the attachment-stale banner, and the
/// send-review control. These are the consumers the acceptance criteria name — the inline
/// composer, the side panel, the patch draft, the CLI / headless surface, and the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftSendConsumerSurface {
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

impl M5DraftSendConsumerSurface {
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

/// The derived retention posture of a draft-state row — a coarse posture derived one-to-one from
/// the draft locality, so a row never leaves the retention posture implicit and never shows a
/// synced or shared draft as if it were local-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftRetentionPosture {
    /// Local-only and unsaved / ephemeral.
    LocalOnlyEphemeral,
    /// Local-only and persisted on this device.
    LocalOnlyPersisted,
    /// Retained on the workspace sync line.
    WorkspaceRetained,
    /// Retained on the account sync line.
    AccountRetained,
    /// Shared into a thread.
    SharedToThread,
    /// Retained pending purge.
    PurgePending,
}

impl M5DraftRetentionPosture {
    /// Every retention posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnlyEphemeral,
        Self::LocalOnlyPersisted,
        Self::WorkspaceRetained,
        Self::AccountRetained,
        Self::SharedToThread,
        Self::PurgePending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyEphemeral => "local_only_ephemeral",
            Self::LocalOnlyPersisted => "local_only_persisted",
            Self::WorkspaceRetained => "workspace_retained",
            Self::AccountRetained => "account_retained",
            Self::SharedToThread => "shared_to_thread",
            Self::PurgePending => "purge_pending",
        }
    }

    /// True when the posture keeps the draft on this device only.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnlyEphemeral | Self::LocalOnlyPersisted)
    }

    /// True when the posture means the draft leaves this device (synced, shared, or retained).
    pub const fn leaves_device(self) -> bool {
        !self.is_local_only()
    }
}

/// One bounded action a draft-state row offers, so a row never hides its retention-detail /
/// save affordances or its clear / delete / stop-sharing follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftStateAction {
    /// Open the retention / locality detail.
    ViewRetentionDetail,
    /// Save the draft locally.
    SaveLocally,
    /// Clear the draft.
    ClearDraft,
    /// Delete the draft (including any synced / shared copy).
    DeleteDraft,
    /// Stop sharing / syncing the draft.
    StopSharing,
}

impl M5DraftStateAction {
    /// Every draft-state action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ViewRetentionDetail,
        Self::SaveLocally,
        Self::ClearDraft,
        Self::DeleteDraft,
        Self::StopSharing,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewRetentionDetail => "view_retention_detail",
            Self::SaveLocally => "save_locally",
            Self::ClearDraft => "clear_draft",
            Self::DeleteDraft => "delete_draft",
            Self::StopSharing => "stop_sharing",
        }
    }
}

/// Controlled draft-state-row anatomy part the shared row surfaces. The parts in
/// [`M5DraftStateRowAnatomyPart::MANDATORY`] are required on every row so the locality, retention
/// posture, sharing exception, clear / delete affordance, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftStateRowAnatomyPart {
    /// The draft locality.
    LocalityCue,
    /// The derived retention posture.
    RetentionPostureCue,
    /// The shared-or-retained exception.
    SharingExceptionCue,
    /// The sync / policy note.
    SyncOrPolicyNoteCue,
    /// The clear / delete affordance.
    ClearDeleteCue,
    /// The saved-state cue.
    SavedStateCue,
    /// The bounded action row (view / save / clear / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5DraftStateRowAnatomyPart {
    /// Every draft-state-row anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::LocalityCue,
        Self::RetentionPostureCue,
        Self::SharingExceptionCue,
        Self::SyncOrPolicyNoteCue,
        Self::ClearDeleteCue,
        Self::SavedStateCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The draft-state-row anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::LocalityCue,
        Self::RetentionPostureCue,
        Self::SharingExceptionCue,
        Self::ClearDeleteCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalityCue => "locality_cue",
            Self::RetentionPostureCue => "retention_posture_cue",
            Self::SharingExceptionCue => "sharing_exception_cue",
            Self::SyncOrPolicyNoteCue => "sync_or_policy_note_cue",
            Self::ClearDeleteCue => "clear_delete_cue",
            Self::SavedStateCue => "saved_state_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// The derived posture of an attachment-stale / offline-local-only banner — the resolver's
/// verdict about whether the attachment is fresh, the route is offline-local-only, or the
/// attachment is stale (and whether the staleness is refreshable, superseded and reviewable,
/// gone, or access-revoked). Computed in a fixed specific-first order so a deleted or revoked
/// attachment never reads as merely refreshable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleBannerPosture {
    /// Fresh and online; nothing to resolve.
    Fresh,
    /// The route is offline / local-only; the draft is preserved with local-safe alternatives.
    OfflineLocalOnly,
    /// The attachment is stale but a refresh recovers it.
    StaleRefreshable,
    /// A newer revision superseded the attachment; review the newer revision.
    StaleSupersededReview,
    /// The attachment's source was deleted; it cannot be refreshed.
    StaleSourceGone,
    /// Access to the attachment was revoked; it cannot be refreshed.
    StaleAccessRevoked,
}

impl M5StaleBannerPosture {
    /// Every banner posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Fresh,
        Self::OfflineLocalOnly,
        Self::StaleRefreshable,
        Self::StaleSupersededReview,
        Self::StaleSourceGone,
        Self::StaleAccessRevoked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::OfflineLocalOnly => "offline_local_only",
            Self::StaleRefreshable => "stale_refreshable",
            Self::StaleSupersededReview => "stale_superseded_review",
            Self::StaleSourceGone => "stale_source_gone",
            Self::StaleAccessRevoked => "stale_access_revoked",
        }
    }

    /// True when the posture represents a stale attachment.
    pub const fn is_stale(self) -> bool {
        matches!(
            self,
            Self::StaleRefreshable
                | Self::StaleSupersededReview
                | Self::StaleSourceGone
                | Self::StaleAccessRevoked
        )
    }

    /// True when the posture's source can no longer be refreshed in place.
    pub const fn source_unrecoverable(self) -> bool {
        matches!(self, Self::StaleSourceGone | Self::StaleAccessRevoked)
    }
}

/// One bounded action an attachment-stale / offline-local-only banner offers, so a banner never
/// leaves a stale or offline state with no refresh or local-safe way forward.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleBannerAction {
    /// Refresh the attachment from its source.
    RefreshAttachment,
    /// Review the attachment / its newer revision.
    ReviewAttachment,
    /// Use a local-safe alternative instead of the live attachment.
    UseLocalSafeAlternative,
    /// Detach the stale attachment from the draft.
    DetachAttachment,
    /// Keep the current draft working locally.
    KeepDraftLocal,
}

impl M5StaleBannerAction {
    /// Every banner action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RefreshAttachment,
        Self::ReviewAttachment,
        Self::UseLocalSafeAlternative,
        Self::DetachAttachment,
        Self::KeepDraftLocal,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshAttachment => "refresh_attachment",
            Self::ReviewAttachment => "review_attachment",
            Self::UseLocalSafeAlternative => "use_local_safe_alternative",
            Self::DetachAttachment => "detach_attachment",
            Self::KeepDraftLocal => "keep_draft_local",
        }
    }
}

/// Controlled attachment-stale-banner anatomy part the shared banner surfaces. The parts in
/// [`M5StaleBannerAnatomyPart::MANDATORY`] are required on every banner so the condition, the
/// staleness reason, the preserved-draft cue, the posture, and the action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleBannerAnatomyPart {
    /// The condition (offline-local-only vs stale).
    ConditionCue,
    /// The staleness reason.
    StalenessReasonCue,
    /// The preserved-draft cue.
    DraftPreservedCue,
    /// The refresh-path cue.
    RefreshPathCue,
    /// The local-safe-alternative cue.
    LocalSafeAlternativeCue,
    /// The derived banner posture.
    PostureCue,
    /// The bounded action row (refresh / review / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5StaleBannerAnatomyPart {
    /// Every banner anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ConditionCue,
        Self::StalenessReasonCue,
        Self::DraftPreservedCue,
        Self::RefreshPathCue,
        Self::LocalSafeAlternativeCue,
        Self::PostureCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The banner anatomy parts every banner must render.
    pub const MANDATORY: [Self; 5] = [
        Self::ConditionCue,
        Self::StalenessReasonCue,
        Self::DraftPreservedCue,
        Self::PostureCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConditionCue => "condition_cue",
            Self::StalenessReasonCue => "staleness_reason_cue",
            Self::DraftPreservedCue => "draft_preserved_cue",
            Self::RefreshPathCue => "refresh_path_cue",
            Self::LocalSafeAlternativeCue => "local_safe_alternative_cue",
            Self::PostureCue => "posture_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// One bounded qualified send path a send-review control offers when a route widens authority, so
/// a high-authority send never collapses into one unqualified affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SendPath {
    /// Explain-only: run the request without applying any change.
    ExplainOnly,
    /// Review then send: open the review, then send.
    ReviewThenSend,
    /// Direct send: send now (applies the mutating route when the route is mutating).
    DirectSend,
}

impl M5SendPath {
    /// Every send path, in declaration order.
    pub const ALL: [Self; 3] = [Self::ExplainOnly, Self::ReviewThenSend, Self::DirectSend];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplainOnly => "explain_only",
            Self::ReviewThenSend => "review_then_send",
            Self::DirectSend => "direct_send",
        }
    }
}

/// One bounded action a send-review control offers, so a control never hides its resolve-blocker /
/// review affordances or its explain-only / confirm / adjust follow-ups.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SendControlAction {
    /// Resolve the policy / budget / taint blocker first.
    ResolveBlocker,
    /// Choose the explain-only path.
    ChooseExplainOnly,
    /// Open the send review.
    OpenSendReview,
    /// Confirm the direct send.
    ConfirmSend,
    /// Adjust the route / scope before send.
    AdjustBeforeSend,
}

impl M5SendControlAction {
    /// Every send-control action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResolveBlocker,
        Self::ChooseExplainOnly,
        Self::OpenSendReview,
        Self::ConfirmSend,
        Self::AdjustBeforeSend,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveBlocker => "resolve_blocker",
            Self::ChooseExplainOnly => "choose_explain_only",
            Self::OpenSendReview => "open_send_review",
            Self::ConfirmSend => "confirm_send",
            Self::AdjustBeforeSend => "adjust_before_send",
        }
    }
}

/// Controlled send-review-control anatomy part the shared control surfaces. The parts in
/// [`M5SendControlAnatomyPart::MANDATORY`] are required on every control so the route authority,
/// send posture, send paths, review requirement, and action row are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SendControlAnatomyPart {
    /// The route authority (route class + widened-authority cue).
    RouteAuthorityCue,
    /// The derived send posture.
    SendPostureCue,
    /// The bounded qualified send paths.
    SendPathsCue,
    /// The pending review requirement.
    ReviewRequirementCue,
    /// The blocker reason (policy / budget / taint).
    BlockerReasonCue,
    /// The authority-widened cue.
    AuthorityWidenedCue,
    /// The bounded action row (resolve / explain-only / …).
    ActionRowCue,
    /// The non-visual keyboard route.
    KeyboardRouteCue,
}

impl M5SendControlAnatomyPart {
    /// Every send-control anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RouteAuthorityCue,
        Self::SendPostureCue,
        Self::SendPathsCue,
        Self::ReviewRequirementCue,
        Self::BlockerReasonCue,
        Self::AuthorityWidenedCue,
        Self::ActionRowCue,
        Self::KeyboardRouteCue,
    ];

    /// The send-control anatomy parts every control must render.
    pub const MANDATORY: [Self; 5] = [
        Self::RouteAuthorityCue,
        Self::SendPostureCue,
        Self::SendPathsCue,
        Self::ReviewRequirementCue,
        Self::ActionRowCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteAuthorityCue => "route_authority_cue",
            Self::SendPostureCue => "send_posture_cue",
            Self::SendPathsCue => "send_paths_cue",
            Self::ReviewRequirementCue => "review_requirement_cue",
            Self::BlockerReasonCue => "blocker_reason_cue",
            Self::AuthorityWidenedCue => "authority_widened_cue",
            Self::ActionRowCue => "action_row_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the draft-state-row export carries so row truth is reconstructable. The fields in
/// [`M5DraftStateRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DraftStateRowExportField {
    /// The stable draft id.
    DraftId,
    /// The draft locality.
    Locality,
    /// The derived retention posture.
    RetentionPosture,
    /// Whether the row discloses sharing.
    DisclosesSharing,
    /// The sync / policy note.
    SyncOrPolicyNote,
    /// Whether the draft is clearable.
    Clearable,
    /// Whether the draft is deletable.
    Deletable,
    /// The bounded available actions.
    AvailableActions,
}

impl M5DraftStateRowExportField {
    /// Every draft-state-row export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DraftId,
        Self::Locality,
        Self::RetentionPosture,
        Self::DisclosesSharing,
        Self::SyncOrPolicyNote,
        Self::Clearable,
        Self::Deletable,
        Self::AvailableActions,
    ];

    /// The draft-state-row export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::DraftId,
        Self::Locality,
        Self::RetentionPosture,
        Self::DisclosesSharing,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DraftId => "draft_id",
            Self::Locality => "locality",
            Self::RetentionPosture => "retention_posture",
            Self::DisclosesSharing => "discloses_sharing",
            Self::SyncOrPolicyNote => "sync_or_policy_note",
            Self::Clearable => "clearable",
            Self::Deletable => "deletable",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// A field the attachment-stale-banner export carries so banner truth is reconstructable. The
/// fields in [`M5StaleBannerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleBannerExportField {
    /// The stable banner id.
    BannerId,
    /// The derived banner posture.
    BannerPosture,
    /// The staleness reason.
    StalenessReason,
    /// Whether the draft is preserved.
    DraftPreserved,
    /// Whether a resolution path is offered.
    OffersResolutionPath,
    /// Whether the route is offline-local-only.
    IsOfflineLocalOnly,
    /// Whether the attachment is stale.
    IsStale,
    /// The bounded available actions.
    AvailableActions,
}

impl M5StaleBannerExportField {
    /// Every banner export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BannerId,
        Self::BannerPosture,
        Self::StalenessReason,
        Self::DraftPreserved,
        Self::OffersResolutionPath,
        Self::IsOfflineLocalOnly,
        Self::IsStale,
        Self::AvailableActions,
    ];

    /// The banner export fields every banner must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::BannerId,
        Self::BannerPosture,
        Self::DraftPreserved,
        Self::OffersResolutionPath,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BannerId => "banner_id",
            Self::BannerPosture => "banner_posture",
            Self::StalenessReason => "staleness_reason",
            Self::DraftPreserved => "draft_preserved",
            Self::OffersResolutionPath => "offers_resolution_path",
            Self::IsOfflineLocalOnly => "is_offline_local_only",
            Self::IsStale => "is_stale",
            Self::AvailableActions => "available_actions",
        }
    }
}

/// A field the send-review-control export carries so control truth is reconstructable. The fields
/// in [`M5SendControlExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SendControlExportField {
    /// The stable control id.
    ControlId,
    /// The derived send posture.
    SendPosture,
    /// The bounded qualified send paths.
    SendPaths,
    /// Whether the control is split (never one unqualified send on widened authority).
    IsSplit,
    /// Whether the control never collapses a widened-authority send.
    NoAmbiguousSend,
    /// Whether review is required before send.
    RequiresReviewBeforeSend,
    /// Whether the request can leave the shell.
    IsSendable,
    /// The bounded available actions.
    AvailableActions,
}

impl M5SendControlExportField {
    /// Every send-control export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ControlId,
        Self::SendPosture,
        Self::SendPaths,
        Self::IsSplit,
        Self::NoAmbiguousSend,
        Self::RequiresReviewBeforeSend,
        Self::IsSendable,
        Self::AvailableActions,
    ];

    /// The send-control export fields every control must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ControlId,
        Self::SendPosture,
        Self::SendPaths,
        Self::NoAmbiguousSend,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ControlId => "control_id",
            Self::SendPosture => "send_posture",
            Self::SendPaths => "send_paths",
            Self::IsSplit => "is_split",
            Self::NoAmbiguousSend => "no_ambiguous_send",
            Self::RequiresReviewBeforeSend => "requires_review_before_send",
            Self::IsSendable => "is_sendable",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- draft-state row ----------------------------------------------------

/// The full input to the draft-state-row resolver for one draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftStateRowResolutionInput {
    /// The opaque stable draft id (must be non-empty).
    pub draft_id: String,
    /// The opaque display label (must be non-empty).
    pub draft_label: String,
    /// Where the draft lives.
    pub locality: M5DraftLocality,
    /// True when the draft is persisted.
    pub saved: bool,
    /// True when the draft carries a shared-or-retained exception beyond local.
    pub shared_or_retained: bool,
    /// The opaque note disclosing the shared-or-retained exception, when the draft leaves the
    /// device.
    pub sharing_exception_note: Option<String>,
    /// The opaque sync / policy note, when the draft carries one.
    pub sync_or_policy_note: Option<String>,
    /// True when the draft can be cleared.
    pub clearable: bool,
    /// True when the draft can be deleted (including any synced / shared copy).
    pub deletable: bool,
}

/// The resolved draft-state-row truth for one draft.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDraftStateRow {
    /// The opaque stable draft id, preserved exactly from the input.
    pub draft_id: String,
    /// The opaque display label.
    pub draft_label: String,
    /// Where the draft lives.
    pub locality: M5DraftLocality,
    /// True when the draft is persisted.
    pub saved: bool,
    /// The derived retention posture.
    pub retention_posture: M5DraftRetentionPosture,
    /// True when the draft stays on this device only.
    pub is_local_only: bool,
    /// True when the draft leaves this device.
    pub leaves_device: bool,
    /// True when the draft is shared into a thread.
    pub is_shared: bool,
    /// The opaque note disclosing the shared-or-retained exception.
    pub sharing_exception_note: Option<String>,
    /// The opaque sync / policy note.
    pub sync_or_policy_note: Option<String>,
    /// True when a non-local draft discloses its sharing (or the draft is local-only).
    pub discloses_sharing: bool,
    /// True when the draft can be cleared.
    pub clearable: bool,
    /// True when the draft can be deleted.
    pub deletable: bool,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5DraftStateAction>,
    /// True when the row makes no hidden-sharing assumption.
    pub no_hidden_sharing: bool,
}

/// Errors returned by [`resolve_draft_state_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DraftStateRowResolutionError {
    /// The draft id was empty.
    EmptyDraftId,
    /// The draft label was empty.
    EmptyDraftLabel,
    /// A draft that leaves the device did not disclose its sharing / retention exception.
    SharedDraftWithoutDisclosure,
    /// A purge-pending draft did not carry its retention note.
    PurgePendingWithoutNote,
    /// A draft descriptor carried forbidden material.
    ForbiddenDraftMaterial,
}

impl M5DraftStateRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDraftId => "empty_draft_id",
            Self::EmptyDraftLabel => "empty_draft_label",
            Self::SharedDraftWithoutDisclosure => "shared_draft_without_disclosure",
            Self::PurgePendingWithoutNote => "purge_pending_without_note",
            Self::ForbiddenDraftMaterial => "forbidden_draft_material",
        }
    }
}

impl fmt::Display for M5DraftStateRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "draft state row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DraftStateRowResolutionError {}

/// Resolves one draft-state row from its declared locality and retention signals.
///
/// The retention posture is derived one-to-one from the locality so a row never leaves the
/// retention posture implicit. A draft that leaves the device — workspace-synced, account-synced,
/// shared to a thread, or retained pending purge — must disclose its sharing / retention
/// exception, so the row never makes a hidden-sharing assumption. A purge-pending draft must
/// carry its retention note, and the row always offers a view-retention-detail action and the
/// clear / delete / stop-sharing follow-ups its state allows.
pub fn resolve_draft_state_row(
    input: &M5DraftStateRowResolutionInput,
) -> Result<M5ResolvedDraftStateRow, M5DraftStateRowResolutionError> {
    if input.draft_id.trim().is_empty() {
        return Err(M5DraftStateRowResolutionError::EmptyDraftId);
    }
    if input.draft_label.trim().is_empty() {
        return Err(M5DraftStateRowResolutionError::EmptyDraftLabel);
    }
    if value_repr_is_forbidden(&input.draft_id)
        || value_repr_is_forbidden(&input.draft_label)
        || input
            .sharing_exception_note
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
        || input
            .sync_or_policy_note
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
    {
        return Err(M5DraftStateRowResolutionError::ForbiddenDraftMaterial);
    }

    let retention_posture = derive_retention_posture(input.locality);
    let is_local_only = retention_posture.is_local_only();
    let leaves_device = retention_posture.leaves_device();
    let is_shared = matches!(input.locality, M5DraftLocality::SharedThread);

    if leaves_device
        && (!input.shared_or_retained
            || input
                .sharing_exception_note
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty())
    {
        return Err(M5DraftStateRowResolutionError::SharedDraftWithoutDisclosure);
    }
    if matches!(input.locality, M5DraftLocality::RetentionPendingPurge)
        && input
            .sync_or_policy_note
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
    {
        return Err(M5DraftStateRowResolutionError::PurgePendingWithoutNote);
    }

    let discloses_sharing =
        !leaves_device || (input.shared_or_retained && input.sharing_exception_note.is_some());
    let available_actions = derive_draft_actions(
        is_local_only,
        input.saved,
        is_shared,
        input.clearable,
        input.deletable,
    );

    Ok(M5ResolvedDraftStateRow {
        draft_id: input.draft_id.clone(),
        draft_label: input.draft_label.clone(),
        locality: input.locality,
        saved: input.saved,
        retention_posture,
        is_local_only,
        leaves_device,
        is_shared,
        sharing_exception_note: input.sharing_exception_note.clone(),
        sync_or_policy_note: input.sync_or_policy_note.clone(),
        discloses_sharing,
        clearable: input.clearable,
        deletable: input.deletable,
        available_actions,
        no_hidden_sharing: discloses_sharing,
    })
}

/// Maps a draft locality to its coarse retention posture.
fn derive_retention_posture(locality: M5DraftLocality) -> M5DraftRetentionPosture {
    match locality {
        M5DraftLocality::EphemeralUnsaved => M5DraftRetentionPosture::LocalOnlyEphemeral,
        M5DraftLocality::LocalOnly => M5DraftRetentionPosture::LocalOnlyPersisted,
        M5DraftLocality::WorkspaceSynced => M5DraftRetentionPosture::WorkspaceRetained,
        M5DraftLocality::AccountSynced => M5DraftRetentionPosture::AccountRetained,
        M5DraftLocality::SharedThread => M5DraftRetentionPosture::SharedToThread,
        M5DraftLocality::RetentionPendingPurge => M5DraftRetentionPosture::PurgePending,
    }
}

/// Derives the bounded draft-state-action set.
///
/// View-retention-detail is always offered so the retention path is preserved; save-locally is
/// offered for an unsaved local draft; clear / delete follow their availability; stop-sharing is
/// offered for a shared draft.
fn derive_draft_actions(
    is_local_only: bool,
    saved: bool,
    is_shared: bool,
    clearable: bool,
    deletable: bool,
) -> Vec<M5DraftStateAction> {
    use M5DraftStateAction as Action;
    let mut actions = vec![Action::ViewRetentionDetail];
    if is_local_only && !saved {
        actions.push(Action::SaveLocally);
    }
    if clearable {
        actions.push(Action::ClearDraft);
    }
    if deletable {
        actions.push(Action::DeleteDraft);
    }
    if is_shared {
        actions.push(Action::StopSharing);
    }
    actions
}

// ---- attachment-stale / offline-local-only banner -----------------------

/// The full input to the attachment-stale-banner resolver for one attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AttachmentStaleBannerResolutionInput {
    /// The opaque stable banner id (must be non-empty).
    pub banner_id: String,
    /// The opaque label naming the attachment (must be non-empty).
    pub attachment_label: String,
    /// True when the route / attachment source is offline / only available locally.
    pub offline_local_only: bool,
    /// Why the attachment is stale, when it is.
    pub staleness_reason: Option<M5StalenessReason>,
    /// True when a refresh path exists for the attachment.
    pub refresh_available: bool,
    /// True when a local-safe alternative exists.
    pub local_safe_alternative_available: bool,
    /// The opaque recovery note, when the attachment is gone or access-revoked.
    pub recovery_note: Option<String>,
}

/// The resolved attachment-stale-banner truth for one attachment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedAttachmentStaleBanner {
    /// The opaque stable banner id, preserved exactly from the input.
    pub banner_id: String,
    /// The opaque attachment label, preserved exactly.
    pub attachment_label: String,
    /// True when the route / attachment source is offline / only available locally.
    pub offline_local_only: bool,
    /// Why the attachment is stale, when it is.
    pub staleness_reason: Option<M5StalenessReason>,
    /// The derived banner posture.
    pub banner_posture: M5StaleBannerPosture,
    /// True when the current draft is preserved (always true).
    pub draft_preserved: bool,
    /// True when the attachment is stale.
    pub is_stale: bool,
    /// True when the route is offline-local-only.
    pub is_offline_local_only: bool,
    /// True when a refresh recovers the attachment.
    pub refreshable: bool,
    /// The opaque recovery note, when the attachment is gone or access-revoked.
    pub recovery_note: Option<String>,
    /// The bounded actions this banner offers.
    pub available_actions: Vec<M5StaleBannerAction>,
    /// True when the banner offers a refresh, review, alternative, or detach path instead of a
    /// silent retry loop.
    pub offers_resolution_path: bool,
}

/// Errors returned by [`resolve_attachment_stale_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AttachmentStaleBannerResolutionError {
    /// The banner id was empty.
    EmptyBannerId,
    /// The attachment label was empty.
    EmptyAttachmentLabel,
    /// A gone or access-revoked attachment did not carry its recovery note.
    GoneAttachmentWithoutRecoveryNote,
    /// An offline-local-only banner offered neither a refresh nor a local-safe alternative.
    OfflineWithoutRefreshOrAlternative,
    /// A banner descriptor carried forbidden material.
    ForbiddenStaleMaterial,
}

impl M5AttachmentStaleBannerResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBannerId => "empty_banner_id",
            Self::EmptyAttachmentLabel => "empty_attachment_label",
            Self::GoneAttachmentWithoutRecoveryNote => "gone_attachment_without_recovery_note",
            Self::OfflineWithoutRefreshOrAlternative => "offline_without_refresh_or_alternative",
            Self::ForbiddenStaleMaterial => "forbidden_stale_material",
        }
    }
}

impl fmt::Display for M5AttachmentStaleBannerResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "attachment stale banner resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5AttachmentStaleBannerResolutionError {}

/// Resolves one attachment-stale / offline-local-only banner from its declared signals.
///
/// The banner posture is computed in a fixed specific-first order: a deleted source reads as
/// source-gone and an access-revoked source reads as access-revoked before either can read as
/// merely refreshable; a superseded revision reads as superseded-review; an edited, moved, or
/// reindexed source reads as refreshable; an offline route with no staleness reads as
/// offline-local-only; and otherwise the attachment reads as fresh. The current draft is always
/// preserved, a gone or revoked attachment must carry its recovery note, an offline-local-only
/// banner must offer a refresh or a local-safe alternative rather than a silent retry, and the
/// banner always keeps a keep-draft-local action so the draft is never dropped.
pub fn resolve_attachment_stale_banner(
    input: &M5AttachmentStaleBannerResolutionInput,
) -> Result<M5ResolvedAttachmentStaleBanner, M5AttachmentStaleBannerResolutionError> {
    if input.banner_id.trim().is_empty() {
        return Err(M5AttachmentStaleBannerResolutionError::EmptyBannerId);
    }
    if input.attachment_label.trim().is_empty() {
        return Err(M5AttachmentStaleBannerResolutionError::EmptyAttachmentLabel);
    }
    if value_repr_is_forbidden(&input.banner_id)
        || value_repr_is_forbidden(&input.attachment_label)
        || input
            .recovery_note
            .as_deref()
            .is_some_and(value_repr_is_forbidden)
    {
        return Err(M5AttachmentStaleBannerResolutionError::ForbiddenStaleMaterial);
    }

    let banner_posture = derive_banner_posture(input.staleness_reason, input.offline_local_only);

    if banner_posture.source_unrecoverable()
        && input
            .recovery_note
            .as_deref()
            .map(str::trim)
            .unwrap_or("")
            .is_empty()
    {
        return Err(M5AttachmentStaleBannerResolutionError::GoneAttachmentWithoutRecoveryNote);
    }
    if matches!(banner_posture, M5StaleBannerPosture::OfflineLocalOnly)
        && !input.refresh_available
        && !input.local_safe_alternative_available
    {
        return Err(M5AttachmentStaleBannerResolutionError::OfflineWithoutRefreshOrAlternative);
    }

    let is_stale = banner_posture.is_stale();
    let is_offline_local_only = matches!(banner_posture, M5StaleBannerPosture::OfflineLocalOnly);
    let refreshable = input.refresh_available
        && matches!(
            banner_posture,
            M5StaleBannerPosture::StaleRefreshable
                | M5StaleBannerPosture::StaleSupersededReview
                | M5StaleBannerPosture::OfflineLocalOnly
        );
    let available_actions = derive_banner_actions(
        banner_posture,
        input.refresh_available,
        input.local_safe_alternative_available,
    );
    let offers_resolution_path = available_actions.iter().any(|action| {
        matches!(
            action,
            M5StaleBannerAction::RefreshAttachment
                | M5StaleBannerAction::ReviewAttachment
                | M5StaleBannerAction::UseLocalSafeAlternative
                | M5StaleBannerAction::DetachAttachment
        )
    });

    Ok(M5ResolvedAttachmentStaleBanner {
        banner_id: input.banner_id.clone(),
        attachment_label: input.attachment_label.clone(),
        offline_local_only: input.offline_local_only,
        staleness_reason: input.staleness_reason,
        banner_posture,
        draft_preserved: true,
        is_stale,
        is_offline_local_only,
        refreshable,
        recovery_note: input.recovery_note.clone(),
        available_actions,
        offers_resolution_path,
    })
}

/// The fixed specific-first banner-posture ladder.
fn derive_banner_posture(
    staleness_reason: Option<M5StalenessReason>,
    offline_local_only: bool,
) -> M5StaleBannerPosture {
    match staleness_reason {
        Some(M5StalenessReason::SourceDeleted) => M5StaleBannerPosture::StaleSourceGone,
        Some(M5StalenessReason::PermissionRevoked) => M5StaleBannerPosture::StaleAccessRevoked,
        Some(M5StalenessReason::RevisionSuperseded) => M5StaleBannerPosture::StaleSupersededReview,
        Some(
            M5StalenessReason::SourceEdited
            | M5StalenessReason::SourceMoved
            | M5StalenessReason::IndexReindexed,
        ) => M5StaleBannerPosture::StaleRefreshable,
        None => {
            if offline_local_only {
                M5StaleBannerPosture::OfflineLocalOnly
            } else {
                M5StaleBannerPosture::Fresh
            }
        }
    }
}

/// Derives the bounded banner-action set.
///
/// Refresh is offered when a refresh recovers the attachment (refreshable / superseded / offline);
/// review is offered whenever the attachment is stale; use-local-safe-alternative is offered for
/// an offline or unrecoverable source when an alternative exists; detach is offered for an
/// unrecoverable source; keep-draft-local is always offered so the draft is never dropped.
fn derive_banner_actions(
    posture: M5StaleBannerPosture,
    refresh_available: bool,
    local_safe_alternative_available: bool,
) -> Vec<M5StaleBannerAction> {
    use M5StaleBannerAction as Action;
    let mut actions = Vec::new();
    if refresh_available
        && matches!(
            posture,
            M5StaleBannerPosture::StaleRefreshable
                | M5StaleBannerPosture::StaleSupersededReview
                | M5StaleBannerPosture::OfflineLocalOnly
        )
    {
        actions.push(Action::RefreshAttachment);
    }
    if posture.is_stale() {
        actions.push(Action::ReviewAttachment);
    }
    if local_safe_alternative_available
        && matches!(
            posture,
            M5StaleBannerPosture::OfflineLocalOnly
                | M5StaleBannerPosture::StaleSourceGone
                | M5StaleBannerPosture::StaleAccessRevoked
        )
    {
        actions.push(Action::UseLocalSafeAlternative);
    }
    if posture.source_unrecoverable() {
        actions.push(Action::DetachAttachment);
    }
    actions.push(Action::KeepDraftLocal);
    actions
}

// ---- split-send / review control ----------------------------------------

/// The full input to the send-review-control resolver for one send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SendReviewControlResolutionInput {
    /// The opaque stable control id (must be non-empty).
    pub control_id: String,
    /// The opaque display label (must be non-empty).
    pub control_label: String,
    /// The route the send previously targeted, when known.
    pub route_before: Option<M5ComposerRouteClass>,
    /// The route the send targets now.
    pub route_after: M5ComposerRouteClass,
    /// True when the route widens the request's authority.
    pub widens_authority: bool,
    /// True when the route can mutate.
    pub is_mutating_route: bool,
    /// The pending review requirements (each a non-`none` requirement).
    pub pending_reviews: Vec<M5ReviewRequirement>,
    /// True when the send is blocked by policy.
    pub policy_blocked: bool,
    /// True when the send is blocked because it is over budget.
    pub over_budget: bool,
    /// True when the send is blocked because context is tainted.
    pub taint_blocked: bool,
}

/// The resolved send-review-control truth for one send.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSendReviewControl {
    /// The opaque stable control id, preserved exactly from the input.
    pub control_id: String,
    /// The opaque display label.
    pub control_label: String,
    /// The route the send previously targeted, when known.
    pub route_before: Option<M5ComposerRouteClass>,
    /// The route the send targets now.
    pub route_after: M5ComposerRouteClass,
    /// True when the route widens the request's authority.
    pub widens_authority: bool,
    /// True when the route can mutate.
    pub is_mutating_route: bool,
    /// The pending review requirements.
    pub pending_reviews: Vec<M5ReviewRequirement>,
    /// The derived send posture.
    pub send_posture: M5SendPosture,
    /// The bounded qualified send paths.
    pub send_paths: Vec<M5SendPath>,
    /// The bounded actions this control offers.
    pub available_actions: Vec<M5SendControlAction>,
    /// True when the send is blocked.
    pub is_blocked: bool,
    /// True when the request can leave the shell.
    pub is_sendable: bool,
    /// True when the control offers more than one qualified path.
    pub is_split: bool,
    /// True when review is required before send.
    pub requires_review_before_send: bool,
    /// True when a widened-authority send never collapses into one unqualified affordance.
    pub no_ambiguous_send: bool,
}

/// Errors returned by [`resolve_send_review_control`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SendReviewControlResolutionError {
    /// The control id was empty.
    EmptyControlId,
    /// The control label was empty.
    EmptyControlLabel,
    /// A pending review requirement was listed as `none`, which is not actionable.
    ReviewRequirementNotActionable,
    /// A widened-authority send collapsed into a single unqualified affordance.
    AmbiguousWideningSend,
    /// A control descriptor carried forbidden material.
    ForbiddenSendMaterial,
}

impl M5SendReviewControlResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyControlId => "empty_control_id",
            Self::EmptyControlLabel => "empty_control_label",
            Self::ReviewRequirementNotActionable => "review_requirement_not_actionable",
            Self::AmbiguousWideningSend => "ambiguous_widening_send",
            Self::ForbiddenSendMaterial => "forbidden_send_material",
        }
    }
}

impl fmt::Display for M5SendReviewControlResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "send review control resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SendReviewControlResolutionError {}

/// Resolves one send-review control from its declared route and blocker signals.
///
/// The send posture is computed in a fixed blocking-first order: a policy block blocks send
/// first, then a taint block, then an over-budget block; then a route that widens authority and
/// can mutate reads as split-send-review; then any pending review reads as review-before-send; and
/// otherwise the send reads as ready. A blocked send offers no qualified path until the blocker is
/// resolved. A send that widens authority always offers more than one qualified path — explain-only,
/// review-then-send, and direct-send — so it never collapses into one unqualified affordance.
pub fn resolve_send_review_control(
    input: &M5SendReviewControlResolutionInput,
) -> Result<M5ResolvedSendReviewControl, M5SendReviewControlResolutionError> {
    if input.control_id.trim().is_empty() {
        return Err(M5SendReviewControlResolutionError::EmptyControlId);
    }
    if input.control_label.trim().is_empty() {
        return Err(M5SendReviewControlResolutionError::EmptyControlLabel);
    }
    if value_repr_is_forbidden(&input.control_id) || value_repr_is_forbidden(&input.control_label) {
        return Err(M5SendReviewControlResolutionError::ForbiddenSendMaterial);
    }
    if input
        .pending_reviews
        .iter()
        .any(|requirement| matches!(requirement, M5ReviewRequirement::None))
    {
        return Err(M5SendReviewControlResolutionError::ReviewRequirementNotActionable);
    }

    let is_blocked = input.policy_blocked || input.over_budget || input.taint_blocked;
    let has_pending_review = !input.pending_reviews.is_empty();
    let send_posture = derive_send_posture(
        input.policy_blocked,
        input.taint_blocked,
        input.over_budget,
        input.widens_authority,
        input.is_mutating_route,
        has_pending_review,
    );
    let requires_review_before_send = matches!(
        send_posture,
        M5SendPosture::SplitSendReview | M5SendPosture::ReviewBeforeSend
    );
    let send_paths = derive_send_paths(send_posture, input.is_mutating_route);
    let is_split = send_paths.len() >= 2;
    let no_ambiguous_send = !input.widens_authority || is_split;

    if input.widens_authority && !is_blocked && !is_split {
        return Err(M5SendReviewControlResolutionError::AmbiguousWideningSend);
    }

    let available_actions = derive_send_actions(is_blocked, &send_paths);

    Ok(M5ResolvedSendReviewControl {
        control_id: input.control_id.clone(),
        control_label: input.control_label.clone(),
        route_before: input.route_before,
        route_after: input.route_after,
        widens_authority: input.widens_authority,
        is_mutating_route: input.is_mutating_route,
        pending_reviews: input.pending_reviews.clone(),
        send_posture,
        send_paths,
        available_actions,
        is_blocked,
        is_sendable: !is_blocked,
        is_split,
        requires_review_before_send,
        no_ambiguous_send,
    })
}

/// The fixed blocking-first send-posture ladder.
fn derive_send_posture(
    policy_blocked: bool,
    taint_blocked: bool,
    over_budget: bool,
    widens_authority: bool,
    is_mutating_route: bool,
    has_pending_review: bool,
) -> M5SendPosture {
    if policy_blocked {
        M5SendPosture::PolicyBlocked
    } else if taint_blocked {
        M5SendPosture::TaintBlocked
    } else if over_budget {
        M5SendPosture::OverBudgetBlocked
    } else if widens_authority && is_mutating_route {
        M5SendPosture::SplitSendReview
    } else if widens_authority || has_pending_review {
        M5SendPosture::ReviewBeforeSend
    } else {
        M5SendPosture::ReadyToSend
    }
}

/// Derives the bounded qualified send-path set.
///
/// A blocked send has no path until the blocker is resolved. A split-send-review offers all three
/// qualified paths. A review-before-send offers explain-only and review-then-send. A ready send
/// offers explain-only and direct-send when mutating, or a single direct-send when not.
fn derive_send_paths(posture: M5SendPosture, is_mutating_route: bool) -> Vec<M5SendPath> {
    use M5SendPath as Path;
    match posture {
        M5SendPosture::PolicyBlocked
        | M5SendPosture::TaintBlocked
        | M5SendPosture::OverBudgetBlocked => Vec::new(),
        M5SendPosture::SplitSendReview => {
            vec![Path::ExplainOnly, Path::ReviewThenSend, Path::DirectSend]
        }
        M5SendPosture::ReviewBeforeSend => vec![Path::ExplainOnly, Path::ReviewThenSend],
        M5SendPosture::ReadyToSend => {
            if is_mutating_route {
                vec![Path::ExplainOnly, Path::DirectSend]
            } else {
                vec![Path::DirectSend]
            }
        }
    }
}

/// Derives the bounded send-control-action set.
///
/// Resolve-blocker is offered when the send is blocked; choose-explain-only, open-send-review, and
/// confirm-send follow the available qualified paths; adjust-before-send is always offered so the
/// route or scope can be narrowed before send.
fn derive_send_actions(is_blocked: bool, send_paths: &[M5SendPath]) -> Vec<M5SendControlAction> {
    use M5SendControlAction as Action;
    let mut actions = Vec::new();
    if is_blocked {
        actions.push(Action::ResolveBlocker);
    }
    if send_paths.contains(&M5SendPath::ExplainOnly) {
        actions.push(Action::ChooseExplainOnly);
    }
    if send_paths.contains(&M5SendPath::ReviewThenSend) {
        actions.push(Action::OpenSendReview);
    }
    if send_paths.contains(&M5SendPath::DirectSend) {
        actions.push(Action::ConfirmSend);
    }
    actions.push(Action::AdjustBeforeSend);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked draft-state-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftStateRowResolutionCase {
    /// The resolver input.
    pub input: M5DraftStateRowResolutionInput,
    /// The resolved truth. Must equal `resolve_draft_state_row(&input)`.
    pub resolved: M5ResolvedDraftStateRow,
}

impl M5DraftStateRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DraftStateRowResolutionInput) -> Self {
        let resolved = resolve_draft_state_row(&input).expect("seed draft case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_draft_state_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved draft id preserves the input id exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.draft_id == self.input.draft_id
    }
}

/// One worked attachment-stale-banner resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AttachmentStaleBannerResolutionCase {
    /// The resolver input.
    pub input: M5AttachmentStaleBannerResolutionInput,
    /// The resolved truth. Must equal `resolve_attachment_stale_banner(&input)`.
    pub resolved: M5ResolvedAttachmentStaleBanner,
}

impl M5AttachmentStaleBannerResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5AttachmentStaleBannerResolutionInput) -> Self {
        let resolved = resolve_attachment_stale_banner(&input).expect("seed stale case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_attachment_stale_banner(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved banner id and attachment label preserve the input exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.banner_id == self.input.banner_id
            && self.resolved.attachment_label == self.input.attachment_label
    }
}

/// One worked send-review-control resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SendReviewControlResolutionCase {
    /// The resolver input.
    pub input: M5SendReviewControlResolutionInput,
    /// The resolved truth. Must equal `resolve_send_review_control(&input)`.
    pub resolved: M5ResolvedSendReviewControl,
}

impl M5SendReviewControlResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SendReviewControlResolutionInput) -> Self {
        let resolved = resolve_send_review_control(&input).expect("seed send case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_send_review_control(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved control id preserves the input id exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.control_id == self.input.control_id
    }
}

/// One row in the primitive matrix: one send-capable consumer bound to the shared draft,
/// stale-banner, and send-control anatomy, draft localities, retention postures, staleness
/// reasons, banner postures, send postures, send paths, review requirements, bounded actions,
/// export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendRow {
    /// Send-capable consumer family.
    pub consumer_surface: M5DraftSendConsumerSurface,
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
    /// Draft-state-row anatomy parts this row renders (must include the mandatory parts).
    pub draft_anatomy_parts: Vec<M5DraftStateRowAnatomyPart>,
    /// Banner anatomy parts this row renders (must include the mandatory parts).
    pub stale_anatomy_parts: Vec<M5StaleBannerAnatomyPart>,
    /// Send-control anatomy parts this row renders (must include the mandatory parts).
    pub send_anatomy_parts: Vec<M5SendControlAnatomyPart>,
    /// Draft localities this consumer distinguishes.
    pub draft_localities: Vec<M5DraftLocality>,
    /// Retention postures this consumer distinguishes.
    pub retention_postures: Vec<M5DraftRetentionPosture>,
    /// Draft actions this consumer offers.
    pub draft_actions: Vec<M5DraftStateAction>,
    /// Staleness reasons this consumer distinguishes.
    pub staleness_reasons: Vec<M5StalenessReason>,
    /// Banner postures this consumer distinguishes.
    pub banner_postures: Vec<M5StaleBannerPosture>,
    /// Banner actions this consumer offers.
    pub stale_actions: Vec<M5StaleBannerAction>,
    /// Send postures this consumer distinguishes.
    pub send_postures: Vec<M5SendPosture>,
    /// Send paths this consumer offers.
    pub send_paths: Vec<M5SendPath>,
    /// Review requirements this consumer distinguishes.
    pub review_requirements: Vec<M5ReviewRequirement>,
    /// Route classes this consumer names.
    pub route_classes: Vec<M5ComposerRouteClass>,
    /// Send actions this consumer offers.
    pub send_actions: Vec<M5SendControlAction>,
    /// Draft export fields this row carries (must include the mandatory fields).
    pub draft_export_fields: Vec<M5DraftStateRowExportField>,
    /// Banner export fields this row carries (must include the mandatory fields).
    pub stale_export_fields: Vec<M5StaleBannerExportField>,
    /// Send export fields this row carries (must include the mandatory fields).
    pub send_export_fields: Vec<M5SendControlExportField>,
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
    /// Worked draft-state-row resolutions proving the draft resolver on this consumer.
    pub draft_examples: Vec<M5DraftStateRowResolutionCase>,
    /// Worked banner resolutions proving the stale resolver on this consumer.
    pub stale_examples: Vec<M5AttachmentStaleBannerResolutionCase>,
    /// Worked send-control resolutions proving the send resolver on this consumer.
    pub send_examples: Vec<M5SendReviewControlResolutionCase>,
    /// Hard invariant: this consumer never masks draft locality or retention. MUST be `false`.
    pub masks_draft_locality_or_retention: bool,
    /// Hard invariant: this consumer never assumes hidden sharing. MUST be `false`.
    pub assumes_hidden_sharing: bool,
    /// Hard invariant: this consumer never invents a private send grammar. MUST be `false`.
    pub invents_private_send_grammar: bool,
    /// Hard invariant: this consumer never collapses a high-authority send. MUST be `false`.
    pub collapses_high_authority_send: bool,
}

impl M5DraftSendRow {
    /// True when the row declares every mandatory draft anatomy part.
    fn declares_mandatory_draft_anatomy(&self) -> bool {
        let present: BTreeSet<M5DraftStateRowAnatomyPart> =
            self.draft_anatomy_parts.iter().copied().collect();
        M5DraftStateRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory banner anatomy part.
    fn declares_mandatory_stale_anatomy(&self) -> bool {
        let present: BTreeSet<M5StaleBannerAnatomyPart> =
            self.stale_anatomy_parts.iter().copied().collect();
        M5StaleBannerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory send anatomy part.
    fn declares_mandatory_send_anatomy(&self) -> bool {
        let present: BTreeSet<M5SendControlAnatomyPart> =
            self.send_anatomy_parts.iter().copied().collect();
        M5SendControlAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory draft export field.
    fn declares_mandatory_draft_export(&self) -> bool {
        let present: BTreeSet<M5DraftStateRowExportField> =
            self.draft_export_fields.iter().copied().collect();
        M5DraftStateRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory banner export field.
    fn declares_mandatory_stale_export(&self) -> bool {
        let present: BTreeSet<M5StaleBannerExportField> =
            self.stale_export_fields.iter().copied().collect();
        M5StaleBannerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory send export field.
    fn declares_mandatory_send_export(&self) -> bool {
        let present: BTreeSet<M5SendControlExportField> =
            self.send_export_fields.iter().copied().collect();
        M5SendControlExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_draft_locality_or_retention
            && !self.assumes_hidden_sharing
            && !self.invents_private_send_grammar
            && !self.collapses_high_authority_send
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendVocabularySet {
    /// Send-capable-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Draft-anatomy-part tokens.
    pub draft_anatomy_parts: Vec<String>,
    /// Banner-anatomy-part tokens.
    pub stale_anatomy_parts: Vec<String>,
    /// Send-anatomy-part tokens.
    pub send_anatomy_parts: Vec<String>,
    /// Draft-locality tokens (reused from the frozen matrix).
    pub draft_localities: Vec<String>,
    /// Retention-posture tokens.
    pub retention_postures: Vec<String>,
    /// Draft-action tokens.
    pub draft_actions: Vec<String>,
    /// Staleness-reason tokens (reused from the frozen matrix).
    pub staleness_reasons: Vec<String>,
    /// Banner-posture tokens.
    pub banner_postures: Vec<String>,
    /// Banner-action tokens.
    pub stale_actions: Vec<String>,
    /// Send-posture tokens (reused from the frozen matrix).
    pub send_postures: Vec<String>,
    /// Send-path tokens.
    pub send_paths: Vec<String>,
    /// Review-requirement tokens (reused from the frozen matrix).
    pub review_requirements: Vec<String>,
    /// Route-class tokens (reused from the frozen matrix).
    pub route_classes: Vec<String>,
    /// Send-action tokens.
    pub send_actions: Vec<String>,
    /// Draft-export-field tokens.
    pub draft_export_fields: Vec<String>,
    /// Banner-export-field tokens.
    pub stale_export_fields: Vec<String>,
    /// Send-export-field tokens.
    pub send_export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DraftSendVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DraftSendConsumerSurface::ALL, |v| v.as_str()),
            draft_anatomy_parts: tokens(&M5DraftStateRowAnatomyPart::ALL, |v| v.as_str()),
            stale_anatomy_parts: tokens(&M5StaleBannerAnatomyPart::ALL, |v| v.as_str()),
            send_anatomy_parts: tokens(&M5SendControlAnatomyPart::ALL, |v| v.as_str()),
            draft_localities: tokens(&M5DraftLocality::ALL, |v| v.as_str()),
            retention_postures: tokens(&M5DraftRetentionPosture::ALL, |v| v.as_str()),
            draft_actions: tokens(&M5DraftStateAction::ALL, |v| v.as_str()),
            staleness_reasons: tokens(&M5StalenessReason::ALL, |v| v.as_str()),
            banner_postures: tokens(&M5StaleBannerPosture::ALL, |v| v.as_str()),
            stale_actions: tokens(&M5StaleBannerAction::ALL, |v| v.as_str()),
            send_postures: tokens(&M5SendPosture::ALL, |v| v.as_str()),
            send_paths: tokens(&M5SendPath::ALL, |v| v.as_str()),
            review_requirements: tokens(&M5ReviewRequirement::ALL, |v| v.as_str()),
            route_classes: tokens(&M5ComposerRouteClass::ALL, |v| v.as_str()),
            send_actions: tokens(&M5SendControlAction::ALL, |v| v.as_str()),
            draft_export_fields: tokens(&M5DraftStateRowExportField::ALL, |v| v.as_str()),
            stale_export_fields: tokens(&M5StaleBannerExportField::ALL, |v| v.as_str()),
            send_export_fields: tokens(&M5SendControlExportField::ALL, |v| v.as_str()),
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
pub struct M5DraftSendGovernanceReview {
    /// One primitive trio carries draft, stale, and send truth on every consumer.
    pub one_primitive_carries_draft_stale_send_truth: bool,
    /// The draft-state row names the draft locality and retention posture.
    pub draft_row_names_locality_and_retention: bool,
    /// Every shared-or-retained exception is disclosed.
    pub shared_or_retained_exceptions_always_disclosed: bool,
    /// Sync / policy notes are exportable.
    pub sync_or_policy_notes_exportable: bool,
    /// Clear / delete behavior is always available.
    pub clear_delete_behavior_always_available: bool,
    /// The stale banner always preserves the current draft.
    pub stale_banner_preserves_draft: bool,
    /// An offline-local-only banner offers a refresh or local-safe alternative.
    pub offline_local_only_offers_refresh_or_local_alternative: bool,
    /// No stale or offline state leaves the user in a silent retry loop.
    pub no_silent_retry_loops: bool,
    /// The send control splits high-authority paths.
    pub send_control_splits_high_authority_paths: bool,
    /// A widened-authority send never collapses into one unqualified affordance.
    pub no_single_unqualified_send_on_widened_authority: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Descriptors stay stable across UI, export, and support surfaces.
    pub descriptors_stable_across_ui_export_support: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendConsumerProjection {
    /// Every send-capable surface consumes the shared primitive trio.
    pub send_capable_surfaces_consume_shared_primitive: bool,
    /// The draft-state derivation reads a single canonical source.
    pub draft_state_reads_single_source: bool,
    /// The stale-state derivation reads a single canonical source.
    pub stale_state_reads_single_source: bool,
    /// The send-posture derivation reads a single canonical source.
    pub send_posture_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the primitive trio.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting AI audit.
    pub ai_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DraftSendPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DraftSendPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5DraftSendRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DraftSendVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DraftSendGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DraftSendConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DraftSendProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DraftSendReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 draft-state-row / stale-banner / send-review-control primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DraftSendPacket {
    /// Record kind; must equal [`M5_DRAFT_SEND_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DRAFT_SEND_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Composer rows.
    pub rows: Vec<M5DraftSendRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DraftSendVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DraftSendGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DraftSendConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DraftSendProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DraftSendReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DraftSendPacket {
    /// Builds an M5 draft/stale/send-primitive packet from stable-lane input.
    pub fn new(input: M5DraftSendPacketInput) -> Self {
        Self {
            record_kind: M5_DRAFT_SEND_RECORD_KIND.to_owned(),
            schema_version: M5_DRAFT_SEND_SCHEMA_VERSION,
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

    /// Validates the M5 draft/stale/send-primitive invariants.
    pub fn validate(&self) -> Vec<M5DraftSendViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DRAFT_SEND_RECORD_KIND {
            violations.push(M5DraftSendViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DRAFT_SEND_SCHEMA_VERSION {
            violations.push(M5DraftSendViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DraftSendViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_draft_locality_disclosure(self, &mut violations);
        validate_draft_hidden_sharing(self, &mut violations);
        validate_stale_preserves_draft_and_offers_alternative(self, &mut violations);
        validate_stale_condition_coverage(self, &mut violations);
        validate_send_split_no_ambiguous(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 draft/send primitive packet serializes"),
        ) {
            violations.push(M5DraftSendViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 draft/send primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per send-capable consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,draft_anatomy,stale_anatomy,send_anatomy,draft_localities,banner_postures,send_postures,send_paths,draft_examples,stale_examples,send_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.draft_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.stale_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.send_anatomy_parts, |v| v.as_str()),
                join_tokens(&row.draft_localities, |v| v.as_str()),
                join_tokens(&row.banner_postures, |v| v.as_str()),
                join_tokens(&row.send_postures, |v| v.as_str()),
                join_tokens(&row.send_paths, |v| v.as_str()),
                row.draft_examples.len(),
                row.stale_examples.len(),
                row.send_examples.len(),
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
        out.push_str(
            "# M5 Draft-State-Row, Attachment-Stale-Banner, and Send-Review-Control Primitive\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Send-capable consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Retention postures: {}\n",
            self.vocabulary_set.retention_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Banner postures: {}\n",
            self.vocabulary_set.banner_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Send paths: {}\n",
            self.vocabulary_set.send_paths.join(", ")
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
                "  - Worked drafts: {}\n",
                row.draft_examples.len()
            ));
            for case in &row.draft_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (leaves device `{}`, discloses `{}`)\n",
                    case.resolved.draft_id,
                    case.resolved.retention_posture.as_str(),
                    case.resolved.leaves_device,
                    case.resolved.discloses_sharing,
                ));
            }
            out.push_str(&format!(
                "  - Worked banners: {}\n",
                row.stale_examples.len()
            ));
            for case in &row.stale_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (draft preserved `{}`, resolution path `{}`)\n",
                    case.resolved.banner_id,
                    case.resolved.banner_posture.as_str(),
                    case.resolved.draft_preserved,
                    case.resolved.offers_resolution_path,
                ));
            }
            out.push_str(&format!(
                "  - Worked send controls: {}\n",
                row.send_examples.len()
            ));
            for case in &row.send_examples {
                out.push_str(&format!(
                    "    - `{}` → `{}` (split `{}`, no ambiguous send `{}`, review `{}`)\n",
                    case.resolved.control_id,
                    case.resolved.send_posture.as_str(),
                    case.resolved.is_split,
                    case.resolved.no_ambiguous_send,
                    case.resolved.requires_review_before_send,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 draft/stale/send-primitive export.
#[derive(Debug)]
pub enum M5DraftSendArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DraftSendViolation>),
}

impl fmt::Display for M5DraftSendArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 draft/send primitive export parse failed: {error}"
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
                    "m5 draft/send primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DraftSendArtifactError {}

/// Validation failures emitted by [`M5DraftSendPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DraftSendViolation {
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
    /// A row omits one of the mandatory draft anatomy parts.
    MandatoryDraftAnatomyMissing,
    /// A row omits one of the mandatory banner anatomy parts.
    MandatoryStaleAnatomyMissing,
    /// A row omits one of the mandatory send anatomy parts.
    MandatorySendAnatomyMissing,
    /// A row omits one of the mandatory draft export fields.
    MandatoryDraftExportMissing,
    /// A row omits one of the mandatory banner export fields.
    MandatoryStaleExportMissing,
    /// A row omits one of the mandatory send export fields.
    MandatorySendExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked draft resolutions.
    DraftExampleMissing,
    /// A row declares no worked banner resolutions.
    StaleExampleMissing,
    /// A row declares no worked send resolutions.
    SendExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked draft resolution proves a non-local draft that discloses its sharing.
    DraftLocalityDisclosureUnproven,
    /// A worked draft that leaves the device did not disclose its sharing.
    DraftHiddenSharingFound,
    /// No worked banner proves a stale-or-offline state that preserves the draft and offers a
    /// resolution path.
    StalePreservesDraftUnproven,
    /// The offline-local-only and attachment-stale conditions are not both proven by a worked
    /// banner.
    StaleConditionCoverageUnproven,
    /// No worked send control proves a widened-authority send that stays split and unambiguous.
    SendSplitNoAmbiguousUnproven,
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

impl M5DraftSendViolation {
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
            Self::MandatoryDraftAnatomyMissing => "mandatory_draft_anatomy_missing",
            Self::MandatoryStaleAnatomyMissing => "mandatory_stale_anatomy_missing",
            Self::MandatorySendAnatomyMissing => "mandatory_send_anatomy_missing",
            Self::MandatoryDraftExportMissing => "mandatory_draft_export_missing",
            Self::MandatoryStaleExportMissing => "mandatory_stale_export_missing",
            Self::MandatorySendExportMissing => "mandatory_send_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::DraftExampleMissing => "draft_example_missing",
            Self::StaleExampleMissing => "stale_example_missing",
            Self::SendExampleMissing => "send_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::DraftLocalityDisclosureUnproven => "draft_locality_disclosure_unproven",
            Self::DraftHiddenSharingFound => "draft_hidden_sharing_found",
            Self::StalePreservesDraftUnproven => "stale_preserves_draft_unproven",
            Self::StaleConditionCoverageUnproven => "stale_condition_coverage_unproven",
            Self::SendSplitNoAmbiguousUnproven => "send_split_no_ambiguous_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 draft/stale/send-primitive export.
pub fn current_stable_m5_draft_send_export() -> Result<M5DraftSendPacket, M5DraftSendArtifactError>
{
    let packet: M5DraftSendPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/ship_draft_state_rows_offline_local_only_banners_attachment_stale_warnings_and_split_send_or_review_controls_with_no_hidden_sharing_and_no_ambiguous_send_truth_across_claimed_m5_composer_surfaces/support_export.json"
    )))
    .map_err(M5DraftSendArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DraftSendArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DRAFT_SEND_SCHEMA_REF,
        M5_DRAFT_SEND_DOC_REF,
        M5_DRAFT_SEND_COMPONENT_MATRIX_REF,
        M5_DRAFT_SEND_PROMPT_COMPOSER_DRAFT_REF,
        M5_DRAFT_SEND_CONTEXT_ATTACHMENT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DraftSendViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(packet: &M5DraftSendPacket, violations: &mut Vec<M5DraftSendViolation>) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DraftSendViolation::VocabularySetDrift);
    }
}

fn validate_rows(packet: &M5DraftSendPacket, violations: &mut Vec<M5DraftSendViolation>) {
    let present: BTreeSet<M5DraftSendConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5DraftSendConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DraftSendViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.draft_anatomy_parts.is_empty()
            || row.stale_anatomy_parts.is_empty()
            || row.send_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.draft_localities.is_empty()
            || row.retention_postures.is_empty()
            || row.draft_actions.is_empty()
            || row.staleness_reasons.is_empty()
            || row.banner_postures.is_empty()
            || row.stale_actions.is_empty()
            || row.send_postures.is_empty()
            || row.send_paths.is_empty()
            || row.review_requirements.is_empty()
            || row.route_classes.is_empty()
            || row.send_actions.is_empty()
        {
            violations.push(M5DraftSendViolation::RowIncomplete);
        }
        if !row.declares_mandatory_draft_anatomy() {
            violations.push(M5DraftSendViolation::MandatoryDraftAnatomyMissing);
        }
        if !row.declares_mandatory_stale_anatomy() {
            violations.push(M5DraftSendViolation::MandatoryStaleAnatomyMissing);
        }
        if !row.declares_mandatory_send_anatomy() {
            violations.push(M5DraftSendViolation::MandatorySendAnatomyMissing);
        }
        if !row.declares_mandatory_draft_export() {
            violations.push(M5DraftSendViolation::MandatoryDraftExportMissing);
        }
        if !row.declares_mandatory_stale_export() {
            violations.push(M5DraftSendViolation::MandatoryStaleExportMissing);
        }
        if !row.declares_mandatory_send_export() {
            violations.push(M5DraftSendViolation::MandatorySendExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComposerAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DraftSendViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DraftSendViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DraftSendViolation::DowngradeTriggersMissing);
        }
        if row.draft_examples.is_empty() {
            violations.push(M5DraftSendViolation::DraftExampleMissing);
        }
        if row.stale_examples.is_empty() {
            violations.push(M5DraftSendViolation::StaleExampleMissing);
        }
        if row.send_examples.is_empty() {
            violations.push(M5DraftSendViolation::SendExampleMissing);
        }
        if row
            .draft_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .stale_examples
                .iter()
                .any(|case| !case.is_self_consistent())
            || row
                .send_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5DraftSendViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DraftSendViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DraftSendViolation::RowInvariantViolated);
        }
    }
}

/// At least one worked draft resolution across the matrix must prove a draft that leaves the
/// device and discloses its sharing — the acceptance-criterion example that draft locality and
/// retention posture stay visible without a hidden-sharing assumption.
fn validate_draft_locality_disclosure(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.draft_examples
            .iter()
            .any(|case| case.resolved.leaves_device && case.resolved.discloses_sharing)
    });
    if !proven {
        violations.push(M5DraftSendViolation::DraftLocalityDisclosureUnproven);
    }
}

/// Every worked draft that leaves the device must disclose its sharing — the acceptance-criterion
/// example that a synced or shared draft is never left with a hidden-sharing assumption.
fn validate_draft_hidden_sharing(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let hidden = packet.rows.iter().any(|row| {
        row.draft_examples
            .iter()
            .any(|case| case.resolved.leaves_device && !case.resolved.discloses_sharing)
    });
    if hidden {
        violations.push(M5DraftSendViolation::DraftHiddenSharingFound);
    }
}

/// At least one worked banner must prove a stale-or-offline state that preserves the draft and
/// offers a resolution path — the acceptance-criterion example that offline-local-only and
/// attachment-stale states preserve the draft and offer refresh or safe-local alternatives instead
/// of a silent retry loop.
fn validate_stale_preserves_draft_and_offers_alternative(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.stale_examples.iter().any(|case| {
            (case.resolved.is_stale || case.resolved.is_offline_local_only)
                && case.resolved.draft_preserved
                && case.resolved.offers_resolution_path
        })
    });
    if !proven {
        violations.push(M5DraftSendViolation::StalePreservesDraftUnproven);
    }
}

/// Both the offline-local-only and the attachment-stale conditions must each be proven by a worked
/// banner, so the banner appears for both defined conditions.
fn validate_stale_condition_coverage(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let offline = packet.rows.iter().any(|row| {
        row.stale_examples
            .iter()
            .any(|case| case.resolved.is_offline_local_only)
    });
    let stale = packet
        .rows
        .iter()
        .any(|row| row.stale_examples.iter().any(|case| case.resolved.is_stale));
    if !offline || !stale {
        violations.push(M5DraftSendViolation::StaleConditionCoverageUnproven);
    }
}

/// At least one worked send control must prove a route that widens authority yet stays split and
/// unambiguous and requires review before send — the acceptance-criterion example that a
/// high-authority send is reviewable and no longer collapses into a single unqualified send.
fn validate_send_split_no_ambiguous(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let proven = packet.rows.iter().any(|row| {
        row.send_examples.iter().any(|case| {
            case.resolved.widens_authority
                && case.resolved.is_split
                && case.resolved.no_ambiguous_send
                && case.resolved.requires_review_before_send
        })
    });
    if !proven {
        violations.push(M5DraftSendViolation::SendSplitNoAmbiguousUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_draft_stale_send_truth,
        review.draft_row_names_locality_and_retention,
        review.shared_or_retained_exceptions_always_disclosed,
        review.sync_or_policy_notes_exportable,
        review.clear_delete_behavior_always_available,
        review.stale_banner_preserves_draft,
        review.offline_local_only_offers_refresh_or_local_alternative,
        review.no_silent_retry_loops,
        review.send_control_splits_high_authority_paths,
        review.no_single_unqualified_send_on_widened_authority,
        review.every_row_declares_accessibility_route,
        review.descriptors_stable_across_ui_export_support,
    ] {
        if !ok {
            violations.push(M5DraftSendViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.send_capable_surfaces_consume_shared_primitive,
        projection.draft_state_reads_single_source,
        projection.stale_state_reads_single_source,
        projection.send_posture_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DraftSendViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DraftSendViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DraftSendPacket,
    violations: &mut Vec<M5DraftSendViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.ai_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DraftSendViolation::ReleasePostureIncomplete);
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
