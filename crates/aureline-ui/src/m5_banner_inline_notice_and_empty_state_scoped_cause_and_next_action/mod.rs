//! Implemented M5 banner / inline-notice and empty-state primitives.
//!
//! The frozen [decision / feedback component matrix][matrix] names Aureline's ubiquitous decision and
//! feedback primitives and locks their controlled vocabulary. This module is the third implement lane
//! over that matrix: it turns the two scoped-state explanation primitives — the **banner / inline
//! notice** and the **empty state** — into resolvers that produce export-safe, honest projections, so a
//! user can trust that a scoped degraded, blocked, or empty state is always calm, actionable, and
//! honest: a banner names its scope, cause, what still works, primary next action, and support / help
//! back-link rather than generic "something went wrong" copy, and an empty state names what the area is
//! for, why it is empty now, and the best next action rather than decorative marketing filler — whether
//! it appears in the start center, a review workspace, a settings area, an update area, or a support
//! area.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement banners and inline notices with explicit scope, cause, what-still-works, primary next
//!   action, and support / help back-links where the failure or limitation is material.**
//!   [`resolve_banner`] refuses to read as a clean, trustworthy notice when the label is unstated, the
//!   surface context is unresolved, the notice scope is the disallowed unscoped / color-only token, the
//!   degraded-state variant is unresolved, the cause is unstated, what-still-works is unstated, the
//!   primary next action is missing, the support / help back-link is missing, generic failure language is
//!   used, the explanation cannot be reconstructed from the export, or the proof packet is stale; it
//!   degrades instead.
//! * **Implement reusable empty-state cards that state what the area is for, why it is empty now, and the
//!   best next action instead of decorative marketing filler.** [`resolve_empty_state`] degrades when the
//!   label is unstated, the surface context is unresolved, the empty-state purpose is the disallowed
//!   blank-no-explanation token, the degraded-state variant is unresolved, the purpose is unstated, the
//!   emptiness reason is unresolved or unexplained, the best next action is missing, decorative filler is
//!   used, generic failure language is used, the explanation cannot be reconstructed from the export, or
//!   the proof packet is stale.
//! * **Carry blocked-by-policy, partial, stale, offline, and restricted variants into the first reusable
//!   banner / empty-state consumers without inventing feature-local synonyms.** Both resolvers carry the
//!   single [`M5DegradedStateVariant`] vocabulary so the scope and degraded-state wording stays stable
//!   across local, remote, managed, and export-sensitive panes.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5DecisionFeedbackDisposition`] state vocabulary, the [`M5NoticeScope`] notice-scope vocabulary, and
//! the [`M5EmptyStatePurpose`] empty-state-purpose vocabulary — so shell, review, settings, updates, and
//! support surfaces can never fork their own state, scope, or empty-state wording. Raw secret values and
//! private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_decision_feedback_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_banner_empty_state_controls,
    seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed,
    seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed,
    M5_BANNER_EMPTY_STATE_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_decision_feedback_component_matrix::{
    M5DecisionFeedbackAccessibilityRoute, M5DecisionFeedbackConsumerSurface,
    M5DecisionFeedbackDeploymentLine, M5DecisionFeedbackDisposition,
    M5DecisionFeedbackDowngradeTrigger, M5DecisionFeedbackFamily,
    M5DecisionFeedbackQualificationClass, M5DecisionFeedbackRequiredLabel, M5EmptyStatePurpose,
    M5NoticeScope, M5_BANNER_INLINE_NOTICE_SCHEMA_REF, M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_EMPTY_STATE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5BannerEmptyStateControlsPacket`].
pub const M5_BANNER_EMPTY_STATE_CONTROLS_RECORD_KIND: &str =
    "implement_m5_banner_inline_notice_and_empty_state_controls";

/// Schema version for M5 banner / empty-state controls records.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-banner-inline-notice-and-empty-state-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_DOC_REF: &str =
    "docs/components/m5_banner_inline_notice_and_empty_state_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BANNER_EMPTY_STATE_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-banner-inline-notice-and-empty-state-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5BannerEmptyStateConsumerSurface = M5DecisionFeedbackConsumerSurface;

/// Controlled render context — which claimed M5 pane renders the primitive, so a banner or empty state's
/// scope, cause, and next-action truth stay stable whether it appears in the start center, a review
/// workspace, a settings area, an update area, or a support area. Minted by this lane, tracking the
/// first claimed M5 panes directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionStateSurfaceContext {
    /// The start center / entry pane.
    EntryStartCenter,
    /// A review workspace pane.
    ReviewWorkspace,
    /// A settings area pane.
    SettingsArea,
    /// An update / install area pane.
    UpdatesArea,
    /// A support area pane.
    SupportArea,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5DecisionStateSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EntryStartCenter,
        Self::ReviewWorkspace,
        Self::SettingsArea,
        Self::UpdatesArea,
        Self::SupportArea,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EntryStartCenter => "entry_start_center",
            Self::ReviewWorkspace => "review_workspace",
            Self::SettingsArea => "settings_area",
            Self::UpdatesArea => "updates_area",
            Self::SupportArea => "support_area",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// Controlled degraded-state variant a banner or empty state names, so a scoped limitation is always
/// classified with one shared vocabulary — blocked by policy, partial capability, stale data, offline,
/// or restricted access — rather than a feature-local synonym. Minted by this lane so scope and
/// degraded-state vocabulary stay consistent across local, remote, managed, and export-sensitive panes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DegradedStateVariant {
    /// Blocked by an administrative or policy decision.
    BlockedByPolicy,
    /// Only partial capability is currently available.
    PartialCapability,
    /// The shown data is stale / not fully refreshed.
    StaleData,
    /// The surface is offline / disconnected.
    Offline,
    /// Access is restricted for this actor / deployment.
    RestrictedAccess,
    /// The degraded-state variant cannot currently be resolved.
    VariantUnknown,
}

impl M5DegradedStateVariant {
    /// Every degraded-state variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BlockedByPolicy,
        Self::PartialCapability,
        Self::StaleData,
        Self::Offline,
        Self::RestrictedAccess,
        Self::VariantUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::PartialCapability => "partial_capability",
            Self::StaleData => "stale_data",
            Self::Offline => "offline",
            Self::RestrictedAccess => "restricted_access",
            Self::VariantUnknown => "variant_unknown",
        }
    }

    /// Whether the degraded-state variant is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::VariantUnknown)
    }
}

/// Controlled recovery posture a banner routes the user through, so a scoped notice always points at a
/// primary next action, a support back-link, a help reference, an inline retry, or a safe dismissal.
/// Minted by this lane because the frozen matrix carries the notice *scope* but not the recovery posture
/// the banner acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BannerActionPosture {
    /// Routes to a primary next action.
    PrimaryNextAction,
    /// Routes to a support back-link.
    SupportBackLink,
    /// Routes to a help reference.
    HelpReference,
    /// Offers an inline retry.
    RetryInline,
    /// Offers a safe dismissal that continues the flow.
    DismissAndContinue,
    /// The action posture cannot currently be resolved.
    PostureUnknown,
}

impl M5BannerActionPosture {
    /// Every action posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PrimaryNextAction,
        Self::SupportBackLink,
        Self::HelpReference,
        Self::RetryInline,
        Self::DismissAndContinue,
        Self::PostureUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryNextAction => "primary_next_action",
            Self::SupportBackLink => "support_back_link",
            Self::HelpReference => "help_reference",
            Self::RetryInline => "retry_inline",
            Self::DismissAndContinue => "dismiss_and_continue",
            Self::PostureUnknown => "posture_unknown",
        }
    }

    /// Whether the action posture is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::PostureUnknown)
    }
}

/// Controlled reason an empty state gives for being empty now, so a blank pane always explains whether it
/// was never populated, was cleared, is filtered to no results, is awaiting a first run, or is blocked
/// upstream. Minted by this lane because the frozen matrix carries the empty-state *purpose* but not the
/// emptiness reason the empty-state acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmptyStateReason {
    /// The area has never been populated.
    NeverPopulated,
    /// Every item was cleared.
    AllItemsCleared,
    /// The active filter excludes all items.
    FilterExcludesAll,
    /// The area is awaiting a first run.
    AwaitingFirstRun,
    /// The area is blocked upstream.
    BlockedUpstream,
    /// The emptiness reason cannot currently be resolved.
    ReasonUnknown,
}

impl M5EmptyStateReason {
    /// Every emptiness reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NeverPopulated,
        Self::AllItemsCleared,
        Self::FilterExcludesAll,
        Self::AwaitingFirstRun,
        Self::BlockedUpstream,
        Self::ReasonUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NeverPopulated => "never_populated",
            Self::AllItemsCleared => "all_items_cleared",
            Self::FilterExcludesAll => "filter_excludes_all",
            Self::AwaitingFirstRun => "awaiting_first_run",
            Self::BlockedUpstream => "blocked_upstream",
            Self::ReasonUnknown => "reason_unknown",
        }
    }

    /// Whether the emptiness reason is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ReasonUnknown)
    }
}

/// One mandatory rendered part a banner or empty state must be able to show, so no scope, cause,
/// what-still-works, next-action, purpose, or emptiness-reason fact is left implicit behind generic
/// chrome or a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BannerEmptyStateAnatomyPart {
    /// The primitive's stable identity / permanent title.
    Identity,
    /// The primitive's current typed state disposition.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The named scope the notice applies to (banner).
    Scope,
    /// The named cause of the limitation (banner).
    Cause,
    /// What still works despite the limitation (banner).
    WhatStillWorks,
    /// The primary next action (banner).
    PrimaryNextAction,
    /// The support / help back-link (banner).
    SupportBackLink,
    /// What the area is for (empty state).
    Purpose,
    /// Why the area is empty now (empty state).
    EmptinessReason,
    /// The best next action (empty state).
    NextBestAction,
}

impl M5BannerEmptyStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::Scope,
        Self::Cause,
        Self::WhatStillWorks,
        Self::PrimaryNextAction,
        Self::SupportBackLink,
        Self::Purpose,
        Self::EmptinessReason,
        Self::NextBestAction,
    ];

    /// The three parts every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::Scope => "scope",
            Self::Cause => "cause",
            Self::WhatStillWorks => "what_still_works",
            Self::PrimaryNextAction => "primary_next_action",
            Self::SupportBackLink => "support_back_link",
            Self::Purpose => "purpose",
            Self::EmptinessReason => "emptiness_reason",
            Self::NextBestAction => "next_best_action",
        }
    }
}

/// Next safe action a primitive surfaces so a user is never left without a route to read the scope and
/// cause, follow the primary next action, open support / help, read the empty-state purpose, or start the
/// first action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BannerEmptyStateNextAction {
    /// Read the banner's scope and named cause.
    ReviewScopeAndCause,
    /// Follow the banner's primary next action.
    FollowPrimaryNextAction,
    /// Open the support / help back-link.
    OpenSupportOrHelp,
    /// Read the empty-state purpose and emptiness reason.
    ReadEmptyStatePurpose,
    /// Start the best first action from the empty state.
    StartFirstAction,
    /// No action is needed; the primitive is clean.
    NoActionNeeded,
}

impl M5BannerEmptyStateNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReviewScopeAndCause,
        Self::FollowPrimaryNextAction,
        Self::OpenSupportOrHelp,
        Self::ReadEmptyStatePurpose,
        Self::StartFirstAction,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewScopeAndCause => "review_scope_and_cause",
            Self::FollowPrimaryNextAction => "follow_primary_next_action",
            Self::OpenSupportOrHelp => "open_support_or_help",
            Self::ReadEmptyStatePurpose => "read_empty_state_purpose",
            Self::StartFirstAction => "start_first_action",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BannerEmptyStateExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The state dispositions carried.
    Dispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The notice scope named by the banner.
    NoticeScope,
    /// The empty-state purpose named by the empty state.
    EmptyStatePurpose,
    /// The render / surface context named by both primitives.
    SurfaceContext,
    /// The degraded-state variant named by both primitives.
    DegradedVariant,
    /// The action posture named by the banner.
    ActionPosture,
    /// The accountable owner role.
    OwnerRole,
}

impl M5BannerEmptyStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::NoticeScope,
        Self::EmptyStatePurpose,
        Self::SurfaceContext,
        Self::DegradedVariant,
        Self::ActionPosture,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::Dispositions => "dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::NoticeScope => "notice_scope",
            Self::EmptyStatePurpose => "empty_state_purpose",
            Self::SurfaceContext => "surface_context",
            Self::DegradedVariant => "degraded_variant",
            Self::ActionPosture => "action_posture",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a banner degraded below a clean, scoped, actionable notice. The degrade-first ladder returns
/// one of these instead of ever letting an unscoped, generic-failure, or next-action-less banner read as
/// a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BannerDegradeReason {
    /// The banner label / identity is unstated.
    BannerLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The notice scope is the disallowed unscoped / color-only token.
    ScopeUnscopedOrColorOnly,
    /// The degraded-state variant cannot currently be resolved.
    DegradedVariantUnresolved,
    /// The cause of the limitation is unstated.
    CauseUnstated,
    /// What still works is unstated.
    WhatStillWorksUnstated,
    /// The primary next action is missing.
    PrimaryNextActionMissing,
    /// The support / help back-link is missing.
    SupportBacklinkMissing,
    /// Generic failure language was used ("something went wrong").
    GenericFailureLanguageUsed,
    /// The explanation cannot be reconstructed from the support export.
    NotReconstructableFromExport,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BannerDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::BannerLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::ScopeUnscopedOrColorOnly,
        Self::DegradedVariantUnresolved,
        Self::CauseUnstated,
        Self::WhatStillWorksUnstated,
        Self::PrimaryNextActionMissing,
        Self::SupportBacklinkMissing,
        Self::GenericFailureLanguageUsed,
        Self::NotReconstructableFromExport,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BannerLabelUnstated => "banner_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ScopeUnscopedOrColorOnly => "scope_unscoped_or_color_only",
            Self::DegradedVariantUnresolved => "degraded_variant_unresolved",
            Self::CauseUnstated => "cause_unstated",
            Self::WhatStillWorksUnstated => "what_still_works_unstated",
            Self::PrimaryNextActionMissing => "primary_next_action_missing",
            Self::SupportBacklinkMissing => "support_backlink_missing",
            Self::GenericFailureLanguageUsed => "generic_failure_language_used",
            Self::NotReconstructableFromExport => "not_reconstructable_from_export",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BannerEmptyStateNextAction {
        match self {
            Self::BannerLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::ScopeUnscopedOrColorOnly
            | Self::DegradedVariantUnresolved
            | Self::CauseUnstated
            | Self::WhatStillWorksUnstated => M5BannerEmptyStateNextAction::ReviewScopeAndCause,
            Self::PrimaryNextActionMissing => M5BannerEmptyStateNextAction::FollowPrimaryNextAction,
            Self::SupportBacklinkMissing
            | Self::GenericFailureLanguageUsed
            | Self::NotReconstructableFromExport
            | Self::ProofStale => M5BannerEmptyStateNextAction::OpenSupportOrHelp,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::ScopeUnscopedOrColorOnly => {
                M5DecisionFeedbackDowngradeTrigger::ColorAloneUsedForMeaning
            }
            Self::DegradedVariantUnresolved | Self::WhatStillWorksUnstated => {
                M5DecisionFeedbackDowngradeTrigger::ScopeUnstated
            }
            Self::CauseUnstated => M5DecisionFeedbackDowngradeTrigger::RationaleUnstated,
            Self::PrimaryNextActionMissing | Self::SupportBacklinkMissing => {
                M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated
            }
            Self::BannerLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::GenericFailureLanguageUsed
            | Self::NotReconstructableFromExport => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an empty state degraded below a clean, purpose-named, next-action-honest card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EmptyStateDegradeReason {
    /// The empty-state label / identity is unstated.
    EmptyLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The empty-state purpose is the disallowed blank-no-explanation token.
    PurposeAsBlankDisallowed,
    /// The degraded-state variant cannot currently be resolved.
    DegradedVariantUnresolved,
    /// What the area is for is unstated.
    PurposeUnstated,
    /// Why the area is empty now is unresolved or unexplained.
    EmptinessReasonUnresolved,
    /// The best next action is missing.
    BestNextActionMissing,
    /// Decorative marketing filler was used instead of a next action.
    DecorativeFillerUsed,
    /// Generic failure language was used ("something went wrong").
    GenericFailureLanguageUsed,
    /// The explanation cannot be reconstructed from the support export.
    NotReconstructableFromExport,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5EmptyStateDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::EmptyLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::PurposeAsBlankDisallowed,
        Self::DegradedVariantUnresolved,
        Self::PurposeUnstated,
        Self::EmptinessReasonUnresolved,
        Self::BestNextActionMissing,
        Self::DecorativeFillerUsed,
        Self::GenericFailureLanguageUsed,
        Self::NotReconstructableFromExport,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyLabelUnstated => "empty_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::PurposeAsBlankDisallowed => "purpose_as_blank_disallowed",
            Self::DegradedVariantUnresolved => "degraded_variant_unresolved",
            Self::PurposeUnstated => "purpose_unstated",
            Self::EmptinessReasonUnresolved => "emptiness_reason_unresolved",
            Self::BestNextActionMissing => "best_next_action_missing",
            Self::DecorativeFillerUsed => "decorative_filler_used",
            Self::GenericFailureLanguageUsed => "generic_failure_language_used",
            Self::NotReconstructableFromExport => "not_reconstructable_from_export",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BannerEmptyStateNextAction {
        match self {
            Self::EmptyLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::PurposeAsBlankDisallowed
            | Self::DegradedVariantUnresolved
            | Self::PurposeUnstated
            | Self::EmptinessReasonUnresolved => {
                M5BannerEmptyStateNextAction::ReadEmptyStatePurpose
            }
            Self::BestNextActionMissing => M5BannerEmptyStateNextAction::StartFirstAction,
            Self::DecorativeFillerUsed
            | Self::GenericFailureLanguageUsed
            | Self::NotReconstructableFromExport
            | Self::ProofStale => M5BannerEmptyStateNextAction::OpenSupportOrHelp,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::PurposeAsBlankDisallowed => {
                M5DecisionFeedbackDowngradeTrigger::UsefulPaneBlankedDuringLoading
            }
            Self::DegradedVariantUnresolved => M5DecisionFeedbackDowngradeTrigger::ScopeUnstated,
            Self::PurposeUnstated | Self::EmptinessReasonUnresolved => {
                M5DecisionFeedbackDowngradeTrigger::RationaleUnstated
            }
            Self::BestNextActionMissing => M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated,
            Self::EmptyLabelUnstated
            | Self::SurfaceContextUnresolved
            | Self::DecorativeFillerUsed
            | Self::GenericFailureLanguageUsed
            | Self::NotReconstructableFromExport => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BannerResolutionInput {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The banner label / heading shown; empty means unstated.
    pub banner_label: String,
    /// The notice scope (from the frozen matrix vocabulary).
    pub notice_scope: M5NoticeScope,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5DecisionStateSurfaceContext,
    /// The degraded-state variant named by the banner.
    pub degraded_variant: M5DegradedStateVariant,
    /// The recovery action posture.
    pub action_posture: M5BannerActionPosture,
    /// True when the cause of the limitation is named.
    pub cause_named: bool,
    /// True when what still works is stated.
    pub what_still_works_stated: bool,
    /// True when a primary next action is present.
    pub primary_next_action_present: bool,
    /// True when a support / help back-link is present.
    pub support_or_help_backlink_present: bool,
    /// True when the banner avoids generic failure language ("something went wrong").
    pub avoids_generic_failure_language: bool,
    /// True when the banner explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe banner projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBanner {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The banner label named by the banner.
    pub banner_label: String,
    /// The notice-scope token named by the banner.
    pub notice_scope: String,
    /// Whether the notice scope is the disallowed unscoped / color-only token.
    pub scope_is_unscoped_or_color_only: bool,
    /// The state-disposition token named by the banner.
    pub disposition: String,
    /// The render / surface-context token named by the banner.
    pub surface_context: String,
    /// The degraded-state-variant token named by the banner.
    pub degraded_variant: String,
    /// The action-posture token named by the banner.
    pub action_posture: String,
    /// Whether the cause of the limitation is named.
    pub cause_named: bool,
    /// Whether what still works is stated.
    pub what_still_works_stated: bool,
    /// Whether a primary next action is present.
    pub primary_next_action_present: bool,
    /// Whether a support / help back-link is present.
    pub support_or_help_backlink_present: bool,
    /// Whether the banner avoids generic failure language.
    pub avoids_generic_failure_language: bool,
    /// Whether the banner explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// Degrade reason, if the banner could not read as a clean, scoped, actionable notice.
    pub degrade_reason: Option<M5BannerDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BannerEmptyStateNextAction,
    /// Whether the banner names scope, cause, and the next action without generic failure language
    /// (clean banner naming every fact).
    pub states_scope_cause_and_next_action: bool,
}

impl M5ResolvedBanner {
    /// Whether this banner reads as a clean, scoped, actionable notice.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_empty_state`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5EmptyStateResolutionInput {
    /// Stable identity of the empty-state instance.
    pub empty_state_id: String,
    /// The empty-state label / heading shown; empty means unstated.
    pub empty_state_label: String,
    /// The empty-state purpose (from the frozen matrix vocabulary).
    pub empty_purpose: M5EmptyStatePurpose,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5DecisionStateSurfaceContext,
    /// The degraded-state variant named by the empty state.
    pub degraded_variant: M5DegradedStateVariant,
    /// The emptiness reason.
    pub empty_reason: M5EmptyStateReason,
    /// True when what the area is for is stated.
    pub purpose_stated: bool,
    /// True when why the area is empty now is explained.
    pub emptiness_explained: bool,
    /// True when a best next action is present.
    pub best_next_action_present: bool,
    /// True when the card avoids decorative marketing filler.
    pub avoids_decorative_filler: bool,
    /// True when the card avoids generic failure language.
    pub avoids_generic_failure_language: bool,
    /// True when the empty-state explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe empty-state projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedEmptyState {
    /// Stable identity of the empty-state instance.
    pub empty_state_id: String,
    /// The empty-state label named by the card.
    pub empty_state_label: String,
    /// The empty-state-purpose token named by the card.
    pub empty_purpose: String,
    /// Whether the purpose is the disallowed blank-no-explanation token.
    pub purpose_is_blank_disallowed: bool,
    /// The state-disposition token named by the card.
    pub disposition: String,
    /// The render / surface-context token named by the card.
    pub surface_context: String,
    /// The degraded-state-variant token named by the card.
    pub degraded_variant: String,
    /// The emptiness-reason token named by the card.
    pub empty_reason: String,
    /// Whether what the area is for is stated.
    pub purpose_stated: bool,
    /// Whether why the area is empty now is explained.
    pub emptiness_explained: bool,
    /// Whether a best next action is present.
    pub best_next_action_present: bool,
    /// Whether the card avoids decorative marketing filler.
    pub avoids_decorative_filler: bool,
    /// Whether the card avoids generic failure language.
    pub avoids_generic_failure_language: bool,
    /// Whether the empty-state explanation can be reconstructed from the support export.
    pub reconstructable_from_export: bool,
    /// Degrade reason, if the card could not read as a clean, purpose-named, next-action-honest state.
    pub degrade_reason: Option<M5EmptyStateDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BannerEmptyStateNextAction,
    /// Whether the card names purpose, emptiness, and the next action (clean card naming every fact).
    pub states_purpose_emptiness_and_next_action: bool,
}

impl M5ResolvedEmptyState {
    /// Whether this empty state reads as a clean, purpose-named, next-action-honest card.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5BannerEmptyStateResolutionError {
    /// The banner id was empty.
    EmptyBannerId,
    /// The empty-state id was empty.
    EmptyEmptyStateId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5BannerEmptyStateResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBannerId => "empty_banner_id",
            Self::EmptyEmptyStateId => "empty_empty_state_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BannerEmptyStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 banner / empty-state resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BannerEmptyStateResolutionError {}

/// Resolves a banner so it stays a scoped, actionable notice: the banner names its label, notice scope
/// (never unscoped / color-only), state disposition, surface context, degraded-state variant, and action
/// posture, states its cause and what still works, exposes a primary next action and a support / help
/// back-link, avoids generic failure language, and stays reconstructable from the export.
pub fn resolve_banner(
    input: M5BannerResolutionInput,
) -> Result<M5ResolvedBanner, M5BannerEmptyStateResolutionError> {
    if input.banner_id.trim().is_empty() {
        return Err(M5BannerEmptyStateResolutionError::EmptyBannerId);
    }
    if string_is_forbidden(&input.banner_id) || string_is_forbidden(&input.banner_label) {
        return Err(M5BannerEmptyStateResolutionError::ForbiddenMaterial);
    }

    let scope_is_unscoped_or_color_only = matches!(
        input.notice_scope,
        M5NoticeScope::UnscopedColorOnlyDisallowed
    );

    let degrade_reason = if input.banner_label.trim().is_empty() {
        Some(M5BannerDegradeReason::BannerLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5BannerDegradeReason::SurfaceContextUnresolved)
    } else if scope_is_unscoped_or_color_only {
        Some(M5BannerDegradeReason::ScopeUnscopedOrColorOnly)
    } else if !input.degraded_variant.is_resolved() {
        Some(M5BannerDegradeReason::DegradedVariantUnresolved)
    } else if !input.cause_named {
        Some(M5BannerDegradeReason::CauseUnstated)
    } else if !input.what_still_works_stated {
        Some(M5BannerDegradeReason::WhatStillWorksUnstated)
    } else if !input.primary_next_action_present {
        Some(M5BannerDegradeReason::PrimaryNextActionMissing)
    } else if !input.support_or_help_backlink_present {
        Some(M5BannerDegradeReason::SupportBacklinkMissing)
    } else if !input.avoids_generic_failure_language {
        Some(M5BannerDegradeReason::GenericFailureLanguageUsed)
    } else if !input.reconstructable_from_export {
        Some(M5BannerDegradeReason::NotReconstructableFromExport)
    } else if !input.proof_fresh {
        Some(M5BannerDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BannerEmptyStateNextAction::ReviewScopeAndCause,
    };

    Ok(M5ResolvedBanner {
        banner_id: input.banner_id,
        banner_label: input.banner_label,
        notice_scope: input.notice_scope.as_str().to_owned(),
        scope_is_unscoped_or_color_only,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        degraded_variant: input.degraded_variant.as_str().to_owned(),
        action_posture: input.action_posture.as_str().to_owned(),
        cause_named: input.cause_named,
        what_still_works_stated: input.what_still_works_stated,
        primary_next_action_present: input.primary_next_action_present,
        support_or_help_backlink_present: input.support_or_help_backlink_present,
        avoids_generic_failure_language: input.avoids_generic_failure_language,
        reconstructable_from_export: input.reconstructable_from_export,
        degrade_reason,
        next_action,
        states_scope_cause_and_next_action: degrade_reason.is_none(),
    })
}

/// Resolves an empty state so it explains its purpose and offers the best next action: the card names
/// its label, empty-state purpose (never blank-no-explanation), state disposition, surface context,
/// degraded-state variant, and emptiness reason, states what the area is for and why it is empty now,
/// exposes a best next action, avoids decorative filler and generic failure language, and stays
/// reconstructable from the export.
pub fn resolve_empty_state(
    input: M5EmptyStateResolutionInput,
) -> Result<M5ResolvedEmptyState, M5BannerEmptyStateResolutionError> {
    if input.empty_state_id.trim().is_empty() {
        return Err(M5BannerEmptyStateResolutionError::EmptyEmptyStateId);
    }
    if string_is_forbidden(&input.empty_state_id) || string_is_forbidden(&input.empty_state_label) {
        return Err(M5BannerEmptyStateResolutionError::ForbiddenMaterial);
    }

    let purpose_is_blank_disallowed = matches!(
        input.empty_purpose,
        M5EmptyStatePurpose::BlankNoExplanationDisallowed
    );

    let degrade_reason = if input.empty_state_label.trim().is_empty() {
        Some(M5EmptyStateDegradeReason::EmptyLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5EmptyStateDegradeReason::SurfaceContextUnresolved)
    } else if purpose_is_blank_disallowed {
        Some(M5EmptyStateDegradeReason::PurposeAsBlankDisallowed)
    } else if !input.degraded_variant.is_resolved() {
        Some(M5EmptyStateDegradeReason::DegradedVariantUnresolved)
    } else if !input.purpose_stated {
        Some(M5EmptyStateDegradeReason::PurposeUnstated)
    } else if !input.empty_reason.is_resolved() || !input.emptiness_explained {
        Some(M5EmptyStateDegradeReason::EmptinessReasonUnresolved)
    } else if !input.best_next_action_present {
        Some(M5EmptyStateDegradeReason::BestNextActionMissing)
    } else if !input.avoids_decorative_filler {
        Some(M5EmptyStateDegradeReason::DecorativeFillerUsed)
    } else if !input.avoids_generic_failure_language {
        Some(M5EmptyStateDegradeReason::GenericFailureLanguageUsed)
    } else if !input.reconstructable_from_export {
        Some(M5EmptyStateDegradeReason::NotReconstructableFromExport)
    } else if !input.proof_fresh {
        Some(M5EmptyStateDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BannerEmptyStateNextAction::ReadEmptyStatePurpose,
    };

    Ok(M5ResolvedEmptyState {
        empty_state_id: input.empty_state_id,
        empty_state_label: input.empty_state_label,
        empty_purpose: input.empty_purpose.as_str().to_owned(),
        purpose_is_blank_disallowed,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        degraded_variant: input.degraded_variant.as_str().to_owned(),
        empty_reason: input.empty_reason.as_str().to_owned(),
        purpose_stated: input.purpose_stated,
        emptiness_explained: input.emptiness_explained,
        best_next_action_present: input.best_next_action_present,
        avoids_decorative_filler: input.avoids_decorative_filler,
        avoids_generic_failure_language: input.avoids_generic_failure_language,
        reconstructable_from_export: input.reconstructable_from_export,
        degrade_reason,
        next_action,
        states_purpose_emptiness_and_next_action: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved banner and empty-state examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5BannerEmptyStateConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5DecisionFeedbackQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5DecisionFeedbackDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5DecisionFeedbackRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5DecisionFeedbackAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5BannerEmptyStateAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5BannerEmptyStateExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    /// Resolved banner examples.
    pub banner_examples: Vec<M5ResolvedBanner>,
    /// Resolved empty-state examples.
    pub empty_state_examples: Vec<M5ResolvedEmptyState>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a banner never relies on color alone for meaning. MUST be `false`.
    pub banner_relies_on_color_alone_for_meaning: bool,
    /// Hard invariant: a banner never uses generic failure language. MUST be `false`.
    pub banner_uses_generic_failure_language: bool,
    /// Hard invariant: an empty state never blanks a pane without a next action. MUST be `false`.
    pub empty_state_blanks_pane_without_next_action: bool,
    /// Hard invariant: an empty state never uses decorative marketing filler. MUST be `false`.
    pub empty_state_uses_decorative_marketing_filler: bool,
}

impl M5BannerEmptyStateControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BannerEmptyStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BannerEmptyStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BannerEmptyStateExportField> =
            self.export_fields.iter().copied().collect();
        M5BannerEmptyStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.banner_relies_on_color_alone_for_meaning
            && !self.banner_uses_generic_failure_language
            && !self.empty_state_blanks_pane_without_next_action
            && !self.empty_state_uses_decorative_marketing_filler
    }

    /// True when a clean banner preserves scoped-notice truth: it is never unscoped / color-only, names
    /// its cause and what still works, exposes a primary next action and a support / help back-link,
    /// avoids generic failure language, and stays reconstructable from the export.
    fn banner_is_honest(ex: &M5ResolvedBanner) -> bool {
        !ex.is_clean()
            || (!ex.scope_is_unscoped_or_color_only
                && ex.cause_named
                && ex.what_still_works_stated
                && ex.primary_next_action_present
                && ex.support_or_help_backlink_present
                && ex.avoids_generic_failure_language
                && ex.reconstructable_from_export)
    }

    /// True when a clean empty state preserves purpose and next-action truth: it never reads as
    /// blank-no-explanation, states its purpose and emptiness, exposes a best next action, avoids
    /// decorative filler and generic failure language, and stays reconstructable from the export.
    fn empty_state_is_honest(ex: &M5ResolvedEmptyState) -> bool {
        !ex.is_clean()
            || (!ex.purpose_is_blank_disallowed
                && ex.purpose_stated
                && ex.emptiness_explained
                && ex.best_next_action_present
                && ex.avoids_decorative_filler
                && ex.avoids_generic_failure_language
                && ex.reconstructable_from_export)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.banner_examples.iter().all(Self::banner_is_honest)
            && self
                .empty_state_examples
                .iter()
                .all(Self::empty_state_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateVocabularySet {
    /// State-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Notice-scope tokens (bound from the frozen matrix).
    pub notice_scopes: Vec<String>,
    /// Empty-state-purpose tokens (bound from the frozen matrix).
    pub empty_state_purposes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Degraded-state-variant tokens (minted by this lane).
    pub degraded_variants: Vec<String>,
    /// Action-posture tokens (minted by this lane).
    pub action_postures: Vec<String>,
    /// Emptiness-reason tokens (minted by this lane).
    pub empty_reasons: Vec<String>,
    /// Banner degrade-reason tokens.
    pub banner_degrade_reasons: Vec<String>,
    /// Empty-state degrade-reason tokens.
    pub empty_state_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5BannerEmptyStateVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5DecisionFeedbackDisposition::ALL, |v| v.as_str()),
            notice_scopes: tokens(&M5NoticeScope::ALL, |v| v.as_str()),
            empty_state_purposes: tokens(&M5EmptyStatePurpose::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5DecisionStateSurfaceContext::ALL, |v| v.as_str()),
            degraded_variants: tokens(&M5DegradedStateVariant::ALL, |v| v.as_str()),
            action_postures: tokens(&M5BannerActionPosture::ALL, |v| v.as_str()),
            empty_reasons: tokens(&M5EmptyStateReason::ALL, |v| v.as_str()),
            banner_degrade_reasons: tokens(&M5BannerDegradeReason::ALL, |v| v.as_str()),
            empty_state_degrade_reasons: tokens(&M5EmptyStateDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5BannerEmptyStateAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5BannerEmptyStateNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BannerEmptyStateExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5DecisionFeedbackConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5BannerEmptyStateGovernanceReview {
    /// The banner states its scope, cause, and what still works.
    pub banner_states_scope_cause_and_what_still_works: bool,
    /// The banner exposes a primary next action.
    pub banner_exposes_primary_next_action: bool,
    /// The banner offers a support / help back-link.
    pub banner_offers_support_or_help_backlink: bool,
    /// The banner avoids generic failure language.
    pub banner_avoids_generic_failure_language: bool,
    /// The banner meaning is never color-only.
    pub banner_meaning_never_color_only: bool,
    /// The empty state states its purpose and why it is empty now.
    pub empty_state_states_purpose_and_emptiness: bool,
    /// The empty state offers a best next action.
    pub empty_state_offers_best_next_action: bool,
    /// The empty state avoids decorative marketing filler.
    pub empty_state_avoids_decorative_filler: bool,
    /// The empty state is never blank without an explanation.
    pub empty_state_never_blank_without_explanation: bool,
    /// Both primitives are reconstructable from the support export.
    pub both_reconstructable_from_export: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateConsumerProjection {
    /// Review surfaces consume the shared banner and empty-state vocabulary.
    pub review_surfaces_consume_banner_and_empty_state_vocabulary: bool,
    /// Settings surfaces consume the shared banner and empty-state vocabulary.
    pub settings_surfaces_consume_banner_and_empty_state_vocabulary: bool,
    /// Update / install surfaces consume the shared banner vocabulary.
    pub updates_surfaces_consume_banner_vocabulary: bool,
    /// Support surfaces consume the shared empty-state vocabulary.
    pub support_surfaces_consume_empty_state_vocabulary: bool,
    /// Banner and empty-state facts trace back to one canonical component contract.
    pub banner_and_empty_state_trace_to_single_component_contract: bool,
    /// Support / export reads a single canonical banner / empty-state source.
    pub support_export_reads_single_banner_empty_state_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BannerEmptyStateControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BannerEmptyStateControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BannerEmptyStateControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BannerEmptyStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BannerEmptyStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BannerEmptyStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BannerEmptyStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BannerEmptyStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 banner / empty-state controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BannerEmptyStateControlsPacket {
    /// Record kind; must equal [`M5_BANNER_EMPTY_STATE_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BannerEmptyStateControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BannerEmptyStateVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BannerEmptyStateGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BannerEmptyStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BannerEmptyStateProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BannerEmptyStateReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BannerEmptyStateControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5BannerEmptyStateControlsPacketInput) -> Self {
        Self {
            record_kind: M5_BANNER_EMPTY_STATE_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5BannerEmptyStateControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BANNER_EMPTY_STATE_CONTROLS_RECORD_KIND {
            violations.push(M5BannerEmptyStateControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_VERSION {
            violations.push(M5BannerEmptyStateControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BannerEmptyStateControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5BannerEmptyStateControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 banner / empty-state controls packet serializes"),
        ) {
            violations.push(M5BannerEmptyStateControlsViolation::RawMaterialInExport);
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
            .expect("m5 banner / empty-state controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,banner_examples,empty_state_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .banner_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.empty_state_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.banner_examples.len(),
                row.empty_state_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Banner / Inline-Notice and Empty-State Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Notice scopes: {}\n",
            self.vocabulary_set.notice_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Degraded-state variants: {}\n",
            self.vocabulary_set.degraded_variants.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Banner examples: {} / empty-state examples: {}\n",
                row.banner_examples.len(),
                row.empty_state_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5BannerEmptyStateControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BannerEmptyStateControlsViolation>),
}

impl fmt::Display for M5BannerEmptyStateControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 banner / empty-state controls export parse failed: {error}"
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
                    "m5 banner / empty-state controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BannerEmptyStateControlsArtifactError {}

/// Validation failures emitted by [`M5BannerEmptyStateControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BannerEmptyStateControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (unscoped banner, generic-failure banner,
    /// next-action-less banner, blank empty state, decorative-filler empty state, or a
    /// non-reconstructable primitive).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Generic failure language or the next safe action is not proven: clean primitives do not always
    /// avoid generic language and expose a next action, or no generic-language / next-action-missing
    /// example degrades, or a clean primitive uses generic language or omits its next action.
    GenericLanguageAndNextActionNotProven,
    /// Scope and degraded-state vocabulary consistency is not proven: clean banners do not cover the
    /// scope grammar, or the degraded-state variants are not all covered, or no unscoped / color-only
    /// example degrades, or no variant-unresolved example degrades.
    ScopeAndDegradedVocabularyNotProven,
    /// The primitives are not proven reconstructable from the export: no clean banner and clean empty
    /// state stay reachable off-screenshot, or no not-reconstructable banner / empty-state example
    /// degrades.
    ReconstructableFromExportNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BannerEmptyStateControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::GenericLanguageAndNextActionNotProven => {
                "generic_language_and_next_action_not_proven"
            }
            Self::ScopeAndDegradedVocabularyNotProven => "scope_and_degraded_vocabulary_not_proven",
            Self::ReconstructableFromExportNotProven => "reconstructable_from_export_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_banner_empty_state_controls_export(
) -> Result<M5BannerEmptyStateControlsPacket, M5BannerEmptyStateControlsArtifactError> {
    let packet: M5BannerEmptyStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-banner-inline-notice-and-empty-state-controls-proof/support_export.json"
    )))
    .map_err(M5BannerEmptyStateControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BannerEmptyStateControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BANNER_EMPTY_STATE_CONTROLS_SCHEMA_REF,
        M5_BANNER_EMPTY_STATE_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BANNER_INLINE_NOTICE_SCHEMA_REF,
        M5_EMPTY_STATE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BannerEmptyStateControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5BannerEmptyStateControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5BannerEmptyStateControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5BannerEmptyStateControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BannerEmptyStateControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_BANNER_INLINE_NOTICE_SCHEMA_REF)
            || !refs.contains(M5_EMPTY_STATE_SCHEMA_REF)
        {
            violations.push(M5BannerEmptyStateControlsViolation::ComponentSchemaRefMissing);
        }
        if row.banner_examples.is_empty() || row.empty_state_examples.is_empty() {
            violations.push(M5BannerEmptyStateControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5BannerEmptyStateControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5BannerEmptyStateControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.banner_states_scope_cause_and_what_still_works,
        review.banner_exposes_primary_next_action,
        review.banner_offers_support_or_help_backlink,
        review.banner_avoids_generic_failure_language,
        review.banner_meaning_never_color_only,
        review.empty_state_states_purpose_and_emptiness,
        review.empty_state_offers_best_next_action,
        review.empty_state_avoids_decorative_filler,
        review.empty_state_never_blank_without_explanation,
        review.both_reconstructable_from_export,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5BannerEmptyStateControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_surfaces_consume_banner_and_empty_state_vocabulary,
        projection.settings_surfaces_consume_banner_and_empty_state_vocabulary,
        projection.updates_surfaces_consume_banner_vocabulary,
        projection.support_surfaces_consume_empty_state_vocabulary,
        projection.banner_and_empty_state_trace_to_single_component_contract,
        projection.support_export_reads_single_banner_empty_state_source,
    ] {
        if !ok {
            violations.push(M5BannerEmptyStateControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BannerEmptyStateControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BannerEmptyStateControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5BannerEmptyStateControlsPacket,
    violations: &mut Vec<M5BannerEmptyStateControlsViolation>,
) {
    let banners = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.banner_examples.iter())
    };
    let empties = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.empty_state_examples.iter())
    };

    // AC1: the first claimed M5 banner / empty-state consumers avoid generic failure language and always
    // expose the next safe action. Every clean primitive avoids generic language and exposes a next
    // action, a generic-language banner degrades, a generic-language empty state degrades, a
    // next-action-missing banner degrades, a next-action-missing empty state degrades, and no clean
    // primitive uses generic language or omits its next action.
    let clean_banners_avoid_generic_and_expose_next = !banners().any(|ex| {
        ex.is_clean() && (!ex.avoids_generic_failure_language || !ex.primary_next_action_present)
    });
    let clean_empties_avoid_generic_and_expose_next = !empties().any(|ex| {
        ex.is_clean() && (!ex.avoids_generic_failure_language || !ex.best_next_action_present)
    });
    let banner_generic_degrades = banners()
        .any(|ex| ex.degrade_reason == Some(M5BannerDegradeReason::GenericFailureLanguageUsed));
    let empty_generic_degrades = empties()
        .any(|ex| ex.degrade_reason == Some(M5EmptyStateDegradeReason::GenericFailureLanguageUsed));
    let banner_next_action_missing_degrades = banners()
        .any(|ex| ex.degrade_reason == Some(M5BannerDegradeReason::PrimaryNextActionMissing));
    let empty_next_action_missing_degrades = empties()
        .any(|ex| ex.degrade_reason == Some(M5EmptyStateDegradeReason::BestNextActionMissing));
    if !(clean_banners_avoid_generic_and_expose_next
        && clean_empties_avoid_generic_and_expose_next
        && banner_generic_degrades
        && empty_generic_degrades
        && banner_next_action_missing_degrades
        && empty_next_action_missing_degrades)
    {
        violations.push(M5BannerEmptyStateControlsViolation::GenericLanguageAndNextActionNotProven);
    }

    // AC2: scope and degraded-state vocabulary stay consistent across local, remote, managed, and
    // export-sensitive panes. Clean banners cover at least the page-scoped / section-scoped /
    // actionable-with-next-step scope grammar, the blocked-by-policy / partial-capability / stale-data /
    // offline / restricted-access degraded-state variants are all covered by clean examples, an unscoped
    // / color-only banner degrades, and a variant-unresolved example degrades.
    let clean_scopes: BTreeSet<String> = banners()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.notice_scope.clone())
        .collect();
    let scope_grammar_covered = ["page_scoped", "section_scoped", "actionable_with_next_step"]
        .iter()
        .all(|s| clean_scopes.contains(*s));
    let clean_variants: BTreeSet<String> = banners()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.degraded_variant.clone())
        .chain(
            empties()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.degraded_variant.clone()),
        )
        .collect();
    let variant_grammar_covered = [
        "blocked_by_policy",
        "partial_capability",
        "stale_data",
        "offline",
        "restricted_access",
    ]
    .iter()
    .all(|v| clean_variants.contains(*v));
    let unscoped_banner_degrades = banners()
        .any(|ex| ex.degrade_reason == Some(M5BannerDegradeReason::ScopeUnscopedOrColorOnly));
    let variant_unresolved_degrades = banners()
        .any(|ex| ex.degrade_reason == Some(M5BannerDegradeReason::DegradedVariantUnresolved))
        || empties().any(|ex| {
            ex.degrade_reason == Some(M5EmptyStateDegradeReason::DegradedVariantUnresolved)
        });
    if !(scope_grammar_covered
        && variant_grammar_covered
        && unscoped_banner_degrades
        && variant_unresolved_degrades)
    {
        violations.push(M5BannerEmptyStateControlsViolation::ScopeAndDegradedVocabularyNotProven);
    }

    // AC3: help / support / export packets can reconstruct why the pane was empty or bannered at the time
    // of capture. At least one clean banner stays reconstructable off-screenshot with a support / help
    // back-link, at least one clean empty state stays reconstructable off-screenshot, a
    // not-reconstructable banner degrades, and a not-reconstructable empty state degrades.
    let clean_reconstructable_banner = banners().any(|ex| {
        ex.is_clean() && ex.reconstructable_from_export && ex.support_or_help_backlink_present
    });
    let clean_reconstructable_empty =
        empties().any(|ex| ex.is_clean() && ex.reconstructable_from_export);
    let banner_not_reconstructable_degrades = banners()
        .any(|ex| ex.degrade_reason == Some(M5BannerDegradeReason::NotReconstructableFromExport));
    let empty_not_reconstructable_degrades = empties().any(|ex| {
        ex.degrade_reason == Some(M5EmptyStateDegradeReason::NotReconstructableFromExport)
    });
    if !(clean_reconstructable_banner
        && clean_reconstructable_empty
        && banner_not_reconstructable_degrades
        && empty_not_reconstructable_degrades)
    {
        violations.push(M5BannerEmptyStateControlsViolation::ReconstructableFromExportNotProven);
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5DecisionFeedbackFamily; 2] = [
    M5DecisionFeedbackFamily::BannerInlineNotice,
    M5DecisionFeedbackFamily::EmptyState,
];
