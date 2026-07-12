//! Implemented M5 badge / chip / pill and popover primitives.
//!
//! The frozen [decision / feedback component matrix][matrix] names Aureline's ubiquitous decision and
//! feedback primitives and locks their controlled vocabulary. This module is the first implement lane
//! over that matrix: it turns the two most ubiquitous compact primitives — the **badge / chip / pill**
//! and the **popover** — into resolvers that produce export-safe, honest projections, so a user can
//! trust that a compact label means the same thing, always expands into plain language rather than
//! color-only shorthand, and that a popover stays a lightweight secondary control with anchored focus
//! return whether it appears in help, settings, marketplace, review, repair, or export-sensitive
//! surfaces.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement badge / chip / pill primitives with concise text, icon support where needed, overflow
//!   rules, and expansion paths back to plain-language explanation rather than color-only meaning.**
//!   [`resolve_badge`] refuses to read as a clean, legible badge when the label is unstated, the render
//!   surface, meaning taxonomy, or overflow behavior is unresolved, meaning is encoded by color alone,
//!   the expansion route is unreachable, the plain-language explanation is missing, the explanation is
//!   reachable only on hover, or the lifecycle / support / provider / policy / source / freshness
//!   taxonomy drifts across surfaces; it degrades instead.
//! * **Implement popovers as lightweight secondary controls only, with dismissibility, anchored focus
//!   return, and no requirement that critical workflow steps live solely inside them.**
//!   [`resolve_popover`] degrades when the popover is not dismissible, is not keyboard operable, does
//!   not return focus to its trigger, carries the only critical workflow instruction, traps critical
//!   steps inside itself, stops being a lightweight non-modal secondary surface, or hides its content
//!   behind hover only.
//! * **Preserve the lifecycle / support / provider / policy / source / freshness taxonomy when badges
//!   and popovers appear in help, settings, marketplace, review, repair, and export-sensitive
//!   surfaces.** Both resolvers carry the render surface context and the badge meaning taxonomy so a
//!   drift across surfaces degrades honestly rather than silently.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5DecisionFeedbackDisposition`] state vocabulary, the [`M5BadgeExpression`] badge-expression
//! vocabulary, and the [`M5PopoverDismissal`] popover-dismissal vocabulary — so help, settings,
//! marketplace, review, repair, and support surfaces can never fork their own state, badge-meaning, or
//! popover-behavior wording. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_decision_feedback_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_badge_popover_controls, seeded_m5_badge_popover_controls_help_ui_beta_narrowed,
    seeded_m5_badge_popover_controls_review_ui_preview_narrowed,
    M5_BADGE_POPOVER_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_decision_feedback_component_matrix::{
    M5BadgeExpression, M5DecisionFeedbackAccessibilityRoute, M5DecisionFeedbackConsumerSurface,
    M5DecisionFeedbackDeploymentLine, M5DecisionFeedbackDisposition,
    M5DecisionFeedbackDowngradeTrigger, M5DecisionFeedbackFamily,
    M5DecisionFeedbackQualificationClass, M5DecisionFeedbackRequiredLabel, M5PopoverDismissal,
    M5_BADGE_CHIP_PILL_SCHEMA_REF, M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
    M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF, M5_POPOVER_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5BadgePopoverControlsPacket`].
pub const M5_BADGE_POPOVER_CONTROLS_RECORD_KIND: &str =
    "implement_m5_badge_chip_pill_and_popover_controls";

/// Schema version for M5 badge / popover controls records.
pub const M5_BADGE_POPOVER_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_BADGE_POPOVER_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-badge-chip-pill-and-popover-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_BADGE_POPOVER_CONTROLS_DOC_REF: &str =
    "docs/components/m5_badge_chip_pill_and_popover_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BADGE_POPOVER_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_BADGE_POPOVER_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BADGE_POPOVER_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BADGE_POPOVER_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-badge-chip-pill-and-popover-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface taxonomy
/// so no lane invents a parallel surface set.
pub type M5BadgePopoverConsumerSurface = M5DecisionFeedbackConsumerSurface;

/// Controlled overflow behavior a badge / chip / pill names, so a compact label stays concise without
/// hiding meaning: it truncates, collapses to a count, or wraps into plain language while always
/// keeping an expansion path. Minted by this lane because the frozen matrix carries the badge
/// *expression* but not the overflow posture the badge acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeOverflowBehavior {
    /// The badge fits with no overflow.
    NoOverflow,
    /// The label truncates but keeps a tooltip and an expansion path.
    TruncatesWithExpansion,
    /// A set of chips collapses to a count with an expansion path.
    CollapsesToCountWithExpansion,
    /// The label wraps into plain language rather than clipping meaning.
    WrapsToPlainLanguage,
    /// The chips scroll within their container without losing meaning.
    ScrollsWithinContainer,
    /// The overflow behavior cannot currently be resolved.
    BehaviorUnknown,
}

impl M5BadgeOverflowBehavior {
    /// Every overflow behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoOverflow,
        Self::TruncatesWithExpansion,
        Self::CollapsesToCountWithExpansion,
        Self::WrapsToPlainLanguage,
        Self::ScrollsWithinContainer,
        Self::BehaviorUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOverflow => "no_overflow",
            Self::TruncatesWithExpansion => "truncates_with_expansion",
            Self::CollapsesToCountWithExpansion => "collapses_to_count_with_expansion",
            Self::WrapsToPlainLanguage => "wraps_to_plain_language",
            Self::ScrollsWithinContainer => "scrolls_within_container",
            Self::BehaviorUnknown => "behavior_unknown",
        }
    }

    /// Whether the badge is currently overflowing its concise footprint.
    pub const fn is_overflowing(self) -> bool {
        matches!(
            self,
            Self::TruncatesWithExpansion
                | Self::CollapsesToCountWithExpansion
                | Self::WrapsToPlainLanguage
                | Self::ScrollsWithinContainer
        )
    }

    /// Whether the overflow behavior is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::BehaviorUnknown)
    }
}

/// Controlled meaning taxonomy a badge preserves, so a badge that carries lifecycle, support, provider,
/// policy, source, or freshness meaning keeps that classification stable across help, settings,
/// marketplace, review, repair, and export surfaces instead of drifting. Minted by this lane because
/// the frozen matrix carries no badge-meaning classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeMeaningTaxonomy {
    /// A lifecycle / channel state (e.g. stable, beta, deprecated).
    LifecycleState,
    /// A support class / evidence-freshness posture.
    SupportClass,
    /// A provider / origin identity.
    ProviderOrigin,
    /// A policy / governance source.
    PolicySource,
    /// A source / provenance and freshness signal.
    SourceFreshness,
    /// The meaning is unclassified, which is disallowed.
    TaxonomyUnclassified,
}

impl M5BadgeMeaningTaxonomy {
    /// Every meaning taxonomy, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LifecycleState,
        Self::SupportClass,
        Self::ProviderOrigin,
        Self::PolicySource,
        Self::SourceFreshness,
        Self::TaxonomyUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleState => "lifecycle_state",
            Self::SupportClass => "support_class",
            Self::ProviderOrigin => "provider_origin",
            Self::PolicySource => "policy_source",
            Self::SourceFreshness => "source_freshness",
            Self::TaxonomyUnclassified => "taxonomy_unclassified",
        }
    }

    /// Whether the badge meaning is classified into the preserved taxonomy (never unclassified).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::TaxonomyUnclassified)
    }
}

/// Controlled expansion route back to plain-language explanation, so a badge's meaning is never
/// color-only or hover-only: it reaches a plain-language explanation by an inline expansion, a
/// disclosure drawer, a linked detail popover, a help reference, or a screen-reader description. Minted
/// by this lane because the frozen matrix carries no badge-expansion route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeExpansionRoute {
    /// Expands inline into plain-language text.
    InlineTextExpansion,
    /// Opens a disclosure drawer with the plain-language explanation.
    DisclosureDrawer,
    /// Links to a detail popover carrying the plain-language explanation.
    LinkedDetailPopover,
    /// Points at a help reference for the plain-language explanation.
    HelpReference,
    /// Carries a screen-reader description of the meaning.
    ScreenReaderDescription,
    /// The expansion route cannot currently be resolved.
    RouteUnknown,
}

impl M5BadgeExpansionRoute {
    /// Every expansion route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InlineTextExpansion,
        Self::DisclosureDrawer,
        Self::LinkedDetailPopover,
        Self::HelpReference,
        Self::ScreenReaderDescription,
        Self::RouteUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InlineTextExpansion => "inline_text_expansion",
            Self::DisclosureDrawer => "disclosure_drawer",
            Self::LinkedDetailPopover => "linked_detail_popover",
            Self::HelpReference => "help_reference",
            Self::ScreenReaderDescription => "screen_reader_description",
            Self::RouteUnknown => "route_unknown",
        }
    }

    /// Whether the expansion route is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::RouteUnknown)
    }
}

/// Controlled render context — which claimed M5 surface renders the primitive, so a badge or popover's
/// meaning and behavior stay stable whether it appears in a help panel, settings row, marketplace
/// listing, review sheet, or repair flow. Minted by this lane, tracking the implementation-requirement
/// surfaces directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DecisionSurfaceContext {
    /// A help panel.
    HelpPanel,
    /// A settings row.
    SettingsRow,
    /// A marketplace listing.
    MarketplaceListing,
    /// A review sheet.
    ReviewSheet,
    /// A repair / recovery flow.
    RepairFlow,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5DecisionSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HelpPanel,
        Self::SettingsRow,
        Self::MarketplaceListing,
        Self::ReviewSheet,
        Self::RepairFlow,
        Self::ContextUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HelpPanel => "help_panel",
            Self::SettingsRow => "settings_row",
            Self::MarketplaceListing => "marketplace_listing",
            Self::ReviewSheet => "review_sheet",
            Self::RepairFlow => "repair_flow",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a badge or popover must be able to show, so no meaning, state, or
/// behavior fact is left implicit behind color, hover, or a secondary surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgePopoverAnatomyPart {
    /// The primitive's stable identity / permanent label.
    Identity,
    /// The primitive's current typed state disposition.
    State,
    /// The non-visual keyboard route to the primitive.
    KeyboardRoute,
    /// The badge expression (badge).
    BadgeExpression,
    /// The badge meaning taxonomy preserved across surfaces (badge).
    MeaningTaxonomy,
    /// The expansion route back to plain language (badge).
    ExpansionRoute,
    /// The overflow behavior keeping the badge concise (badge).
    OverflowBehavior,
    /// The render / surface context (both primitives).
    SurfaceContext,
    /// The popover dismissal / focus behavior (popover).
    PopoverDismissal,
    /// The anchored focus return to the trigger (popover).
    FocusReturn,
    /// The plain-language explanation of meaning (both primitives).
    PlainLanguageExplanation,
}

impl M5BadgePopoverAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::BadgeExpression,
        Self::MeaningTaxonomy,
        Self::ExpansionRoute,
        Self::OverflowBehavior,
        Self::SurfaceContext,
        Self::PopoverDismissal,
        Self::FocusReturn,
        Self::PlainLanguageExplanation,
    ];

    /// The three parts every claimed primitive must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::BadgeExpression => "badge_expression",
            Self::MeaningTaxonomy => "meaning_taxonomy",
            Self::ExpansionRoute => "expansion_route",
            Self::OverflowBehavior => "overflow_behavior",
            Self::SurfaceContext => "surface_context",
            Self::PopoverDismissal => "popover_dismissal",
            Self::FocusReturn => "focus_return",
            Self::PlainLanguageExplanation => "plain_language_explanation",
        }
    }
}

/// Next safe action a primitive surfaces so a user is never left without a route to inspect meaning,
/// state, or a degraded badge / popover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgePopoverNextAction {
    /// Expand the badge's compact meaning.
    ExpandBadgeMeaning,
    /// Inspect the badge's meaning taxonomy.
    InspectBadgeTaxonomy,
    /// Open the plain-language explanation of meaning.
    OpenPlainLanguageExplanation,
    /// Return focus to the popover's trigger.
    ReturnFocusToTrigger,
    /// Review a blocked / degraded primitive.
    ReviewBlockedOrDegraded,
    /// No action is needed; the primitive is clean.
    NoActionNeeded,
}

impl M5BadgePopoverNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandBadgeMeaning,
        Self::InspectBadgeTaxonomy,
        Self::OpenPlainLanguageExplanation,
        Self::ReturnFocusToTrigger,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandBadgeMeaning => "expand_badge_meaning",
            Self::InspectBadgeTaxonomy => "inspect_badge_taxonomy",
            Self::OpenPlainLanguageExplanation => "open_plain_language_explanation",
            Self::ReturnFocusToTrigger => "return_focus_to_trigger",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgePopoverExportField {
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
    /// The badge expression named by the badge.
    BadgeExpression,
    /// The badge meaning taxonomy preserved by the badge.
    MeaningTaxonomy,
    /// The expansion route named by the badge.
    ExpansionRoute,
    /// The render / surface context named by both primitives.
    SurfaceContext,
    /// The popover dismissal named by the popover.
    PopoverDismissal,
    /// The accountable owner role.
    OwnerRole,
}

impl M5BadgePopoverExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::Dispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::BadgeExpression,
        Self::MeaningTaxonomy,
        Self::ExpansionRoute,
        Self::SurfaceContext,
        Self::PopoverDismissal,
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
            Self::BadgeExpression => "badge_expression",
            Self::MeaningTaxonomy => "meaning_taxonomy",
            Self::ExpansionRoute => "expansion_route",
            Self::SurfaceContext => "surface_context",
            Self::PopoverDismissal => "popover_dismissal",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a badge degraded below a clean, legible state. The degrade-first ladder returns one of these
/// instead of ever letting an opaque, color-only, or hover-only badge read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeDegradeReason {
    /// The badge label is unstated; a user cannot read what the badge means.
    BadgeLabelUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The badge meaning is unclassified (not in the preserved taxonomy).
    MeaningTaxonomyUnclassified,
    /// The overflow behavior cannot currently be resolved.
    OverflowBehaviorUnresolved,
    /// The badge meaning is encoded by color alone rather than plain language.
    MeaningEncodedByColorAlone,
    /// No expansion route back to plain language is reachable.
    ExpansionRouteUnreachable,
    /// The plain-language explanation is missing.
    PlainLanguageExplanationMissing,
    /// The plain-language explanation is reachable only on hover (not keyboard / screen reader /
    /// export).
    ExpansionOnlyViaHover,
    /// The meaning taxonomy drifted across surfaces.
    TaxonomyDriftedAcrossSurface,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5BadgeDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::BadgeLabelUnstated,
        Self::SurfaceContextUnresolved,
        Self::MeaningTaxonomyUnclassified,
        Self::OverflowBehaviorUnresolved,
        Self::MeaningEncodedByColorAlone,
        Self::ExpansionRouteUnreachable,
        Self::PlainLanguageExplanationMissing,
        Self::ExpansionOnlyViaHover,
        Self::TaxonomyDriftedAcrossSurface,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BadgeLabelUnstated => "badge_label_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::MeaningTaxonomyUnclassified => "meaning_taxonomy_unclassified",
            Self::OverflowBehaviorUnresolved => "overflow_behavior_unresolved",
            Self::MeaningEncodedByColorAlone => "meaning_encoded_by_color_alone",
            Self::ExpansionRouteUnreachable => "expansion_route_unreachable",
            Self::PlainLanguageExplanationMissing => "plain_language_explanation_missing",
            Self::ExpansionOnlyViaHover => "expansion_only_via_hover",
            Self::TaxonomyDriftedAcrossSurface => "taxonomy_drifted_across_surface",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BadgePopoverNextAction {
        match self {
            Self::BadgeLabelUnstated | Self::MeaningEncodedByColorAlone => {
                M5BadgePopoverNextAction::ExpandBadgeMeaning
            }
            Self::SurfaceContextUnresolved
            | Self::MeaningTaxonomyUnclassified
            | Self::OverflowBehaviorUnresolved
            | Self::TaxonomyDriftedAcrossSurface => M5BadgePopoverNextAction::InspectBadgeTaxonomy,
            Self::ExpansionRouteUnreachable
            | Self::PlainLanguageExplanationMissing
            | Self::ExpansionOnlyViaHover
            | Self::ProofStale => M5BadgePopoverNextAction::OpenPlainLanguageExplanation,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::MeaningEncodedByColorAlone | Self::ExpansionOnlyViaHover => {
                M5DecisionFeedbackDowngradeTrigger::ColorAloneUsedForMeaning
            }
            Self::MeaningTaxonomyUnclassified | Self::TaxonomyDriftedAcrossSurface => {
                M5DecisionFeedbackDowngradeTrigger::StateTaxonomyDrifted
            }
            Self::BadgeLabelUnstated | Self::PlainLanguageExplanationMissing => {
                M5DecisionFeedbackDowngradeTrigger::RationaleUnstated
            }
            Self::ExpansionRouteUnreachable => {
                M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated
            }
            Self::SurfaceContextUnresolved | Self::OverflowBehaviorUnresolved => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a popover degraded below a clean, lightweight, safe-focus state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PopoverDegradeReason {
    /// The popover identity / accessible name is unstated.
    PopoverIdentityUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The named dismissal model is the disallowed carries-only-instruction token.
    DismissalModelDisallowed,
    /// The popover is not dismissible (no Escape, outside click, or explicit close).
    NotDismissible,
    /// The popover is not operable by keyboard.
    KeyboardOperationMissing,
    /// The popover does not return focus to its trigger when closed.
    FocusDoesNotReturnToTrigger,
    /// The popover carries the only critical workflow instruction.
    CarriesOnlyCriticalInstruction,
    /// Critical workflow steps are trapped solely inside the popover.
    CriticalStepsTrappedInPopover,
    /// The popover is not a lightweight non-modal secondary surface.
    NotLightweightSecondary,
    /// The popover content is reachable only on hover.
    ContentReachableOnlyOnHover,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PopoverDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::PopoverIdentityUnstated,
        Self::SurfaceContextUnresolved,
        Self::DismissalModelDisallowed,
        Self::NotDismissible,
        Self::KeyboardOperationMissing,
        Self::FocusDoesNotReturnToTrigger,
        Self::CarriesOnlyCriticalInstruction,
        Self::CriticalStepsTrappedInPopover,
        Self::NotLightweightSecondary,
        Self::ContentReachableOnlyOnHover,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PopoverIdentityUnstated => "popover_identity_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DismissalModelDisallowed => "dismissal_model_disallowed",
            Self::NotDismissible => "not_dismissible",
            Self::KeyboardOperationMissing => "keyboard_operation_missing",
            Self::FocusDoesNotReturnToTrigger => "focus_does_not_return_to_trigger",
            Self::CarriesOnlyCriticalInstruction => "carries_only_critical_instruction",
            Self::CriticalStepsTrappedInPopover => "critical_steps_trapped_in_popover",
            Self::NotLightweightSecondary => "not_lightweight_secondary",
            Self::ContentReachableOnlyOnHover => "content_reachable_only_on_hover",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5BadgePopoverNextAction {
        match self {
            Self::PopoverIdentityUnstated
            | Self::SurfaceContextUnresolved
            | Self::NotLightweightSecondary => M5BadgePopoverNextAction::ReviewBlockedOrDegraded,
            Self::DismissalModelDisallowed
            | Self::NotDismissible
            | Self::KeyboardOperationMissing
            | Self::FocusDoesNotReturnToTrigger => M5BadgePopoverNextAction::ReturnFocusToTrigger,
            Self::CarriesOnlyCriticalInstruction
            | Self::CriticalStepsTrappedInPopover
            | Self::ContentReachableOnlyOnHover
            | Self::ProofStale => M5BadgePopoverNextAction::OpenPlainLanguageExplanation,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5DecisionFeedbackDowngradeTrigger {
        match self {
            Self::DismissalModelDisallowed
            | Self::CarriesOnlyCriticalInstruction
            | Self::CriticalStepsTrappedInPopover => {
                M5DecisionFeedbackDowngradeTrigger::PopoverCarriedOnlyCriticalInstruction
            }
            Self::ContentReachableOnlyOnHover => {
                M5DecisionFeedbackDowngradeTrigger::ColorAloneUsedForMeaning
            }
            Self::NotDismissible => M5DecisionFeedbackDowngradeTrigger::RecoveryPathUnstated,
            Self::PopoverIdentityUnstated => M5DecisionFeedbackDowngradeTrigger::RationaleUnstated,
            Self::SurfaceContextUnresolved
            | Self::KeyboardOperationMissing
            | Self::FocusDoesNotReturnToTrigger
            | Self::NotLightweightSecondary => {
                M5DecisionFeedbackDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5DecisionFeedbackDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_badge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BadgeResolutionInput {
    /// Stable identity of the badge instance.
    pub badge_id: String,
    /// The concise badge label shown; empty means unstated.
    pub badge_label: String,
    /// The badge expression (from the frozen matrix vocabulary).
    pub expression: M5BadgeExpression,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The badge meaning taxonomy preserved across surfaces.
    pub meaning_taxonomy: M5BadgeMeaningTaxonomy,
    /// The overflow behavior keeping the badge concise.
    pub overflow_behavior: M5BadgeOverflowBehavior,
    /// The expansion route back to plain language.
    pub expansion_route: M5BadgeExpansionRoute,
    /// The render / surface context.
    pub surface_context: M5DecisionSurfaceContext,
    /// True when the meaning is stated in plain language, never color alone.
    pub meaning_stated_non_color_only: bool,
    /// True when a plain-language explanation of the meaning is present.
    pub plain_language_explanation_present: bool,
    /// True when the plain-language explanation is reachable by keyboard, screen reader, and export
    /// (never hover-only).
    pub explanation_reachable_by_keyboard_sr_export: bool,
    /// True when the meaning taxonomy stays stable across surfaces (no drift).
    pub taxonomy_stable_across_surfaces: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe badge projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBadge {
    /// Stable identity of the badge instance.
    pub badge_id: String,
    /// The concise badge label named by the badge.
    pub badge_label: String,
    /// The badge-expression token named by the badge.
    pub expression: String,
    /// Whether the expression names the disallowed color-only shorthand.
    pub expression_is_color_only: bool,
    /// The state-disposition token named by the badge.
    pub disposition: String,
    /// Whether the disposition demands a plain-language explanation (warning / blocked / degraded).
    pub disposition_demands_explanation: bool,
    /// The meaning-taxonomy token named by the badge.
    pub meaning_taxonomy: String,
    /// Whether the meaning is classified into the preserved taxonomy.
    pub meaning_is_classified: bool,
    /// The overflow-behavior token named by the badge.
    pub overflow_behavior: String,
    /// Whether the badge is currently overflowing its concise footprint.
    pub is_overflowing: bool,
    /// The expansion-route token named by the badge.
    pub expansion_route: String,
    /// The render / surface-context token named by the badge.
    pub surface_context: String,
    /// Whether the meaning is stated in plain language, never color alone.
    pub meaning_stated_non_color_only: bool,
    /// Whether a plain-language explanation of the meaning is present.
    pub plain_language_explanation_present: bool,
    /// Whether the plain-language explanation is reachable by keyboard / screen reader / export.
    pub explanation_reachable_by_keyboard_sr_export: bool,
    /// Whether the meaning taxonomy stays stable across surfaces.
    pub taxonomy_stable_across_surfaces: bool,
    /// Degrade reason, if the badge could not read as a clean, legible state.
    pub degrade_reason: Option<M5BadgeDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BadgePopoverNextAction,
    /// Whether the meaning is legible without hover (clean badge naming every fact).
    pub meaning_legible_without_hover: bool,
}

impl M5ResolvedBadge {
    /// Whether this badge reads as a clean, legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_popover`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PopoverResolutionInput {
    /// Stable identity of the popover instance.
    pub popover_id: String,
    /// The accessible name shown; empty means unstated.
    pub accessible_name: String,
    /// The popover dismissal / focus behavior (from the frozen matrix vocabulary).
    pub dismissal: M5PopoverDismissal,
    /// The current state disposition (from the frozen matrix vocabulary).
    pub disposition: M5DecisionFeedbackDisposition,
    /// The render / surface context.
    pub surface_context: M5DecisionSurfaceContext,
    /// True when the popover is dismissible (Escape / outside click / explicit close).
    pub is_dismissible: bool,
    /// True when the popover is operable by keyboard.
    pub keyboard_operable: bool,
    /// True when the popover returns focus to its trigger when closed.
    pub focus_returns_to_trigger: bool,
    /// True when the popover carries the only critical workflow instruction (guardrail; MUST be
    /// `false` on a clean popover).
    pub carries_only_critical_instruction: bool,
    /// True when critical workflow steps are also available outside the popover.
    pub critical_steps_available_elsewhere: bool,
    /// True when the popover is a lightweight non-modal secondary surface.
    pub is_non_modal_secondary: bool,
    /// True when the popover content is reachable without hover (keyboard / screen reader / export).
    pub content_reachable_without_hover: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe popover projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPopover {
    /// Stable identity of the popover instance.
    pub popover_id: String,
    /// The accessible name named by the popover.
    pub accessible_name: String,
    /// The dismissal-behavior token named by the popover.
    pub dismissal: String,
    /// Whether the dismissal names the disallowed carries-only-instruction token.
    pub dismissal_is_disallowed: bool,
    /// The state-disposition token named by the popover.
    pub disposition: String,
    /// The render / surface-context token named by the popover.
    pub surface_context: String,
    /// Whether the popover is dismissible.
    pub is_dismissible: bool,
    /// Whether the popover is operable by keyboard.
    pub keyboard_operable: bool,
    /// Whether the popover returns focus to its trigger when closed.
    pub focus_returns_to_trigger: bool,
    /// Guardrail (MUST be `false` on a clean popover): the popover carries the only critical
    /// instruction.
    pub carries_only_critical_instruction: bool,
    /// Whether critical workflow steps are also available outside the popover.
    pub critical_steps_available_elsewhere: bool,
    /// Whether the popover is a lightweight non-modal secondary surface.
    pub is_non_modal_secondary: bool,
    /// Whether the popover content is reachable without hover.
    pub content_reachable_without_hover: bool,
    /// Degrade reason, if the popover could not read as a clean, lightweight, safe-focus state.
    pub degrade_reason: Option<M5PopoverDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5BadgePopoverNextAction,
    /// Whether the popover stays a lightweight secondary control with safe focus return (clean popover
    /// naming every fact).
    pub stays_lightweight_secondary_with_safe_focus: bool,
}

impl M5ResolvedPopover {
    /// Whether this popover reads as a clean, lightweight, safe-focus state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5BadgePopoverResolutionError {
    /// The badge id was empty.
    EmptyBadgeId,
    /// The popover id was empty.
    EmptyPopoverId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5BadgePopoverResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBadgeId => "empty_badge_id",
            Self::EmptyPopoverId => "empty_popover_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5BadgePopoverResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 badge / popover resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5BadgePopoverResolutionError {}

/// Resolves a badge so its meaning is legible without hover: the badge names its concise label, its
/// expression (never color-only), its state disposition, its preserved meaning taxonomy, its overflow
/// behavior, its surface context, and an expansion route back to a plain-language explanation reachable
/// by keyboard, screen reader, and export.
pub fn resolve_badge(
    input: M5BadgeResolutionInput,
) -> Result<M5ResolvedBadge, M5BadgePopoverResolutionError> {
    if input.badge_id.trim().is_empty() {
        return Err(M5BadgePopoverResolutionError::EmptyBadgeId);
    }
    if string_is_forbidden(&input.badge_id) || string_is_forbidden(&input.badge_label) {
        return Err(M5BadgePopoverResolutionError::ForbiddenMaterial);
    }

    let expression_is_color_only =
        matches!(input.expression, M5BadgeExpression::ColorOnlyDisallowed);

    let degrade_reason = if input.badge_label.trim().is_empty() {
        Some(M5BadgeDegradeReason::BadgeLabelUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5BadgeDegradeReason::SurfaceContextUnresolved)
    } else if !input.meaning_taxonomy.is_classified() {
        Some(M5BadgeDegradeReason::MeaningTaxonomyUnclassified)
    } else if !input.overflow_behavior.is_resolved() {
        Some(M5BadgeDegradeReason::OverflowBehaviorUnresolved)
    } else if expression_is_color_only || !input.meaning_stated_non_color_only {
        Some(M5BadgeDegradeReason::MeaningEncodedByColorAlone)
    } else if !input.expansion_route.is_resolved() {
        Some(M5BadgeDegradeReason::ExpansionRouteUnreachable)
    } else if !input.plain_language_explanation_present {
        Some(M5BadgeDegradeReason::PlainLanguageExplanationMissing)
    } else if !input.explanation_reachable_by_keyboard_sr_export {
        Some(M5BadgeDegradeReason::ExpansionOnlyViaHover)
    } else if !input.taxonomy_stable_across_surfaces {
        Some(M5BadgeDegradeReason::TaxonomyDriftedAcrossSurface)
    } else if !input.proof_fresh {
        Some(M5BadgeDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BadgePopoverNextAction::ExpandBadgeMeaning,
    };

    Ok(M5ResolvedBadge {
        badge_id: input.badge_id,
        badge_label: input.badge_label,
        expression: input.expression.as_str().to_owned(),
        expression_is_color_only,
        disposition: input.disposition.as_str().to_owned(),
        disposition_demands_explanation: input.disposition.demands_plain_language_explanation(),
        meaning_taxonomy: input.meaning_taxonomy.as_str().to_owned(),
        meaning_is_classified: input.meaning_taxonomy.is_classified(),
        overflow_behavior: input.overflow_behavior.as_str().to_owned(),
        is_overflowing: input.overflow_behavior.is_overflowing(),
        expansion_route: input.expansion_route.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        meaning_stated_non_color_only: input.meaning_stated_non_color_only,
        plain_language_explanation_present: input.plain_language_explanation_present,
        explanation_reachable_by_keyboard_sr_export: input
            .explanation_reachable_by_keyboard_sr_export,
        taxonomy_stable_across_surfaces: input.taxonomy_stable_across_surfaces,
        degrade_reason,
        next_action,
        meaning_legible_without_hover: degrade_reason.is_none(),
    })
}

/// Resolves a popover so it stays a lightweight secondary control with safe focus return: the popover
/// names its accessible name, dismissal behavior, state disposition, and surface context, is
/// dismissible and keyboard operable, returns focus to its trigger, never carries the only critical
/// instruction, never traps critical steps, stays a non-modal secondary surface, and keeps its content
/// reachable without hover.
pub fn resolve_popover(
    input: M5PopoverResolutionInput,
) -> Result<M5ResolvedPopover, M5BadgePopoverResolutionError> {
    if input.popover_id.trim().is_empty() {
        return Err(M5BadgePopoverResolutionError::EmptyPopoverId);
    }
    if string_is_forbidden(&input.popover_id) || string_is_forbidden(&input.accessible_name) {
        return Err(M5BadgePopoverResolutionError::ForbiddenMaterial);
    }

    let dismissal_is_disallowed = matches!(
        input.dismissal,
        M5PopoverDismissal::CarriesOnlyInstructionDisallowed
    );

    let degrade_reason = if input.accessible_name.trim().is_empty() {
        Some(M5PopoverDegradeReason::PopoverIdentityUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PopoverDegradeReason::SurfaceContextUnresolved)
    } else if dismissal_is_disallowed {
        Some(M5PopoverDegradeReason::DismissalModelDisallowed)
    } else if !input.is_dismissible {
        Some(M5PopoverDegradeReason::NotDismissible)
    } else if !input.keyboard_operable {
        Some(M5PopoverDegradeReason::KeyboardOperationMissing)
    } else if !input.focus_returns_to_trigger {
        Some(M5PopoverDegradeReason::FocusDoesNotReturnToTrigger)
    } else if input.carries_only_critical_instruction {
        Some(M5PopoverDegradeReason::CarriesOnlyCriticalInstruction)
    } else if !input.critical_steps_available_elsewhere {
        Some(M5PopoverDegradeReason::CriticalStepsTrappedInPopover)
    } else if !input.is_non_modal_secondary {
        Some(M5PopoverDegradeReason::NotLightweightSecondary)
    } else if !input.content_reachable_without_hover {
        Some(M5PopoverDegradeReason::ContentReachableOnlyOnHover)
    } else if !input.proof_fresh {
        Some(M5PopoverDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5BadgePopoverNextAction::ReturnFocusToTrigger,
    };

    Ok(M5ResolvedPopover {
        popover_id: input.popover_id,
        accessible_name: input.accessible_name,
        dismissal: input.dismissal.as_str().to_owned(),
        dismissal_is_disallowed,
        disposition: input.disposition.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        is_dismissible: input.is_dismissible,
        keyboard_operable: input.keyboard_operable,
        focus_returns_to_trigger: input.focus_returns_to_trigger,
        carries_only_critical_instruction: input.carries_only_critical_instruction,
        critical_steps_available_elsewhere: input.critical_steps_available_elsewhere,
        is_non_modal_secondary: input.is_non_modal_secondary,
        content_reachable_without_hover: input.content_reachable_without_hover,
        degrade_reason,
        next_action,
        stays_lightweight_secondary_with_safe_focus: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved badge and popover examples it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5BadgePopoverConsumerSurface,
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
    pub anatomy_parts: Vec<M5BadgePopoverAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5BadgePopoverExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5DecisionFeedbackDowngradeTrigger>,
    /// Resolved badge examples.
    pub badge_examples: Vec<M5ResolvedBadge>,
    /// Resolved popover examples.
    pub popover_examples: Vec<M5ResolvedPopover>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a badge never relies on color alone to convey meaning. MUST be `false`.
    pub badge_meaning_relies_on_color_alone: bool,
    /// Hard invariant: a badge's meaning is never hidden behind hover only. MUST be `false`.
    pub badge_meaning_hidden_behind_hover_only: bool,
    /// Hard invariant: a popover never carries the only critical workflow instruction. MUST be `false`.
    pub popover_carries_only_critical_instruction: bool,
    /// Hard invariant: a popover never fails to return focus to its trigger. MUST be `false`.
    pub popover_fails_to_return_focus_to_trigger: bool,
}

impl M5BadgePopoverControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BadgePopoverAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BadgePopoverAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BadgePopoverExportField> =
            self.export_fields.iter().copied().collect();
        M5BadgePopoverExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.badge_meaning_relies_on_color_alone
            && !self.badge_meaning_hidden_behind_hover_only
            && !self.popover_carries_only_critical_instruction
            && !self.popover_fails_to_return_focus_to_trigger
    }

    /// True when a clean badge preserves legibility: it is never color-only, states meaning in plain
    /// language, keeps the explanation reachable off-hover, keeps a classified taxonomy stable across
    /// surfaces, and carries a present plain-language explanation.
    fn badge_is_honest(ex: &M5ResolvedBadge) -> bool {
        !ex.is_clean()
            || (!ex.expression_is_color_only
                && ex.meaning_stated_non_color_only
                && ex.explanation_reachable_by_keyboard_sr_export
                && ex.plain_language_explanation_present
                && ex.taxonomy_stable_across_surfaces
                && ex.meaning_is_classified)
    }

    /// True when a clean popover preserves lightweight-secondary safety: it never carries the only
    /// instruction, never names the disallowed dismissal, is dismissible, returns focus to its trigger,
    /// keeps critical steps available elsewhere, stays a non-modal secondary surface, and keeps content
    /// reachable off-hover.
    fn popover_is_honest(ex: &M5ResolvedPopover) -> bool {
        !ex.is_clean()
            || (!ex.carries_only_critical_instruction
                && !ex.dismissal_is_disallowed
                && ex.is_dismissible
                && ex.focus_returns_to_trigger
                && ex.critical_steps_available_elsewhere
                && ex.is_non_modal_secondary
                && ex.content_reachable_without_hover)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.badge_examples.iter().all(Self::badge_is_honest)
            && self.popover_examples.iter().all(Self::popover_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverVocabularySet {
    /// State-disposition tokens (bound from the frozen matrix).
    pub dispositions: Vec<String>,
    /// Badge-expression tokens (bound from the frozen matrix).
    pub badge_expressions: Vec<String>,
    /// Popover-dismissal tokens (bound from the frozen matrix).
    pub popover_dismissals: Vec<String>,
    /// Meaning-taxonomy tokens (minted by this lane).
    pub meaning_taxonomies: Vec<String>,
    /// Overflow-behavior tokens (minted by this lane).
    pub overflow_behaviors: Vec<String>,
    /// Expansion-route tokens (minted by this lane).
    pub expansion_routes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Badge degrade-reason tokens.
    pub badge_degrade_reasons: Vec<String>,
    /// Popover degrade-reason tokens.
    pub popover_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5BadgePopoverVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            dispositions: tokens(&M5DecisionFeedbackDisposition::ALL, |v| v.as_str()),
            badge_expressions: tokens(&M5BadgeExpression::ALL, |v| v.as_str()),
            popover_dismissals: tokens(&M5PopoverDismissal::ALL, |v| v.as_str()),
            meaning_taxonomies: tokens(&M5BadgeMeaningTaxonomy::ALL, |v| v.as_str()),
            overflow_behaviors: tokens(&M5BadgeOverflowBehavior::ALL, |v| v.as_str()),
            expansion_routes: tokens(&M5BadgeExpansionRoute::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5DecisionSurfaceContext::ALL, |v| v.as_str()),
            badge_degrade_reasons: tokens(&M5BadgeDegradeReason::ALL, |v| v.as_str()),
            popover_degrade_reasons: tokens(&M5PopoverDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5BadgePopoverAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5BadgePopoverNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BadgePopoverExportField::ALL, |v| v.as_str()),
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
pub struct M5BadgePopoverGovernanceReview {
    /// The badge names its concise label and its meaning.
    pub badge_names_label_and_meaning: bool,
    /// The badge always expands into a plain-language explanation.
    pub badge_expands_to_plain_language: bool,
    /// The badge never relies on color alone to convey meaning.
    pub badge_never_relies_on_color_alone: bool,
    /// The badge's meaning is reachable by keyboard, screen reader, and export (never hover-only).
    pub badge_meaning_reachable_by_keyboard_sr_export: bool,
    /// The badge preserves the lifecycle / support / provider / policy / source / freshness taxonomy.
    pub badge_preserves_taxonomy_across_surfaces: bool,
    /// The popover stays a lightweight non-modal secondary control.
    pub popover_stays_lightweight_secondary: bool,
    /// The popover is dismissible with anchored focus return to its trigger.
    pub popover_is_dismissible_with_anchored_focus_return: bool,
    /// The popover never carries the only critical workflow instruction.
    pub popover_never_carries_only_critical_instruction: bool,
    /// The popover content is reachable without hover.
    pub popover_content_reachable_without_hover: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverConsumerProjection {
    /// Help surfaces consume the shared badge and popover vocabulary.
    pub help_surfaces_consume_badge_vocabulary: bool,
    /// Settings surfaces consume the shared badge and popover vocabulary.
    pub settings_surfaces_consume_badge_vocabulary: bool,
    /// Review surfaces consume the shared badge and popover vocabulary.
    pub review_surfaces_consume_badge_and_popover_vocabulary: bool,
    /// Marketplace surfaces consume the shared badge vocabulary.
    pub marketplace_surfaces_consume_badge_vocabulary: bool,
    /// Badge and popover facts trace back to one canonical component contract.
    pub badge_meaning_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical badge / popover source.
    pub support_export_reads_single_badge_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BadgePopoverControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BadgePopoverControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BadgePopoverControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgePopoverVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgePopoverGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgePopoverConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgePopoverProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgePopoverReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 badge / popover controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgePopoverControlsPacket {
    /// Record kind; must equal [`M5_BADGE_POPOVER_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BADGE_POPOVER_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5BadgePopoverControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgePopoverVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgePopoverGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgePopoverConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgePopoverProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgePopoverReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BadgePopoverControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5BadgePopoverControlsPacketInput) -> Self {
        Self {
            record_kind: M5_BADGE_POPOVER_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_BADGE_POPOVER_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5BadgePopoverControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BADGE_POPOVER_CONTROLS_RECORD_KIND {
            violations.push(M5BadgePopoverControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BADGE_POPOVER_CONTROLS_SCHEMA_VERSION {
            violations.push(M5BadgePopoverControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BadgePopoverControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5BadgePopoverControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 badge / popover controls packet serializes"),
        ) {
            violations.push(M5BadgePopoverControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 badge / popover controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,badge_examples,popover_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .badge_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.popover_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.badge_examples.len(),
                row.popover_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Badge / Chip / Pill and Popover Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Badge expressions: {}\n",
            self.vocabulary_set.badge_expressions.join(", ")
        ));
        out.push_str(&format!(
            "- Meaning taxonomies: {}\n",
            self.vocabulary_set.meaning_taxonomies.join(", ")
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
                "  - Badge examples: {} / popover examples: {}\n",
                row.badge_examples.len(),
                row.popover_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5BadgePopoverControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BadgePopoverControlsViolation>),
}

impl fmt::Display for M5BadgePopoverControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 badge / popover controls export parse failed: {error}"
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
                    "m5 badge / popover controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BadgePopoverControlsArtifactError {}

/// Validation failures emitted by [`M5BadgePopoverControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BadgePopoverControlsViolation {
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
    /// A controls row carries a dishonest clean example (color-only badge, hover-only meaning, taxonomy
    /// drift, popover that carries the only instruction, or a popover that loses focus return).
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
    /// Badge taxonomy and popover focus behavior are not proven: clean badges do not cover the badge
    /// expression / meaning taxonomy grammar, or no color-only / hover-only / popover-focus example
    /// degrades, or a clean badge is color-only / hover-only, or a clean popover loses focus return.
    BadgeTaxonomyAndPopoverFocusNotProven,
    /// Plain-language reachability is not proven: no clean badge reaches a plain-language explanation by
    /// keyboard / screen reader / export, or no hover-only / plain-language-missing example degrades, or
    /// a clean badge is hover-only.
    PlainLanguageReachabilityNotProven,
    /// Badge / popover drift is not proven: no clean badge and clean popover both trace to a canonical
    /// component contract, or no taxonomy-drift example degrades before release evidence turns green.
    BadgePopoverDriftNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BadgePopoverControlsViolation {
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
            Self::BadgeTaxonomyAndPopoverFocusNotProven => {
                "badge_taxonomy_and_popover_focus_not_proven"
            }
            Self::PlainLanguageReachabilityNotProven => "plain_language_reachability_not_proven",
            Self::BadgePopoverDriftNotProven => "badge_popover_drift_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_badge_popover_controls_export(
) -> Result<M5BadgePopoverControlsPacket, M5BadgePopoverControlsArtifactError> {
    let packet: M5BadgePopoverControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-chip-pill-and-popover-controls-proof/support_export.json"
    )))
    .map_err(M5BadgePopoverControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BadgePopoverControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BADGE_POPOVER_CONTROLS_SCHEMA_REF,
        M5_BADGE_POPOVER_CONTROLS_DOC_REF,
        M5_DECISION_FEEDBACK_COMPONENT_SCHEMA_REF,
        M5_DECISION_FEEDBACK_COMPONENT_DOC_REF,
        M5_BADGE_CHIP_PILL_SCHEMA_REF,
        M5_POPOVER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BadgePopoverControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5BadgePopoverControlsViolation::NoControlsRows);
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
            violations.push(M5BadgePopoverControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5BadgePopoverControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BadgePopoverControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_BADGE_CHIP_PILL_SCHEMA_REF) || !refs.contains(M5_POPOVER_SCHEMA_REF) {
            violations.push(M5BadgePopoverControlsViolation::ComponentSchemaRefMissing);
        }
        if row.badge_examples.is_empty() || row.popover_examples.is_empty() {
            violations.push(M5BadgePopoverControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5BadgePopoverControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5BadgePopoverControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.badge_names_label_and_meaning,
        review.badge_expands_to_plain_language,
        review.badge_never_relies_on_color_alone,
        review.badge_meaning_reachable_by_keyboard_sr_export,
        review.badge_preserves_taxonomy_across_surfaces,
        review.popover_stays_lightweight_secondary,
        review.popover_is_dismissible_with_anchored_focus_return,
        review.popover_never_carries_only_critical_instruction,
        review.popover_content_reachable_without_hover,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5BadgePopoverControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.help_surfaces_consume_badge_vocabulary,
        projection.settings_surfaces_consume_badge_vocabulary,
        projection.review_surfaces_consume_badge_and_popover_vocabulary,
        projection.marketplace_surfaces_consume_badge_vocabulary,
        projection.badge_meaning_traces_to_single_component_contract,
        projection.support_export_reads_single_badge_source,
    ] {
        if !ok {
            violations.push(M5BadgePopoverControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BadgePopoverControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BadgePopoverControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5BadgePopoverControlsPacket,
    violations: &mut Vec<M5BadgePopoverControlsViolation>,
) {
    let badges = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.badge_examples.iter())
    };
    let popovers = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.popover_examples.iter())
    };

    // AC1: the first claimed M5 consumers show consistent badge taxonomy and popover focus behavior
    // without hover-only truth gaps. Clean badges cover at least the text-label / icon-with-text /
    // status-word expression grammar and the lifecycle / support / policy meaning taxonomy, a
    // color-only example degrades, a hover-only example degrades, a popover-focus example degrades, and
    // no clean badge is color-only / hover-only and no clean popover loses focus return.
    let clean_expressions: BTreeSet<String> = badges()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.expression.clone())
        .collect();
    let clean_taxonomies: BTreeSet<String> = badges()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.meaning_taxonomy.clone())
        .collect();
    let expression_grammar_covered = ["text_label", "icon_with_text", "status_word"]
        .iter()
        .all(|e| clean_expressions.contains(*e));
    let taxonomy_grammar_covered = ["lifecycle_state", "support_class", "policy_source"]
        .iter()
        .all(|t| clean_taxonomies.contains(*t));
    let color_only_degrades = badges()
        .any(|ex| ex.degrade_reason == Some(M5BadgeDegradeReason::MeaningEncodedByColorAlone));
    let hover_only_degrades =
        badges().any(|ex| ex.degrade_reason == Some(M5BadgeDegradeReason::ExpansionOnlyViaHover));
    let popover_focus_degrades = popovers()
        .any(|ex| ex.degrade_reason == Some(M5PopoverDegradeReason::FocusDoesNotReturnToTrigger));
    let no_clean_color_or_hover_badge = !badges().any(|ex| {
        ex.is_clean()
            && (ex.expression_is_color_only
                || !ex.meaning_stated_non_color_only
                || !ex.explanation_reachable_by_keyboard_sr_export)
    });
    let no_clean_popover_without_focus =
        !popovers().any(|ex| ex.is_clean() && !ex.focus_returns_to_trigger);
    if !(expression_grammar_covered
        && taxonomy_grammar_covered
        && color_only_degrades
        && hover_only_degrades
        && popover_focus_degrades
        && no_clean_color_or_hover_badge
        && no_clean_popover_without_focus)
    {
        violations.push(M5BadgePopoverControlsViolation::BadgeTaxonomyAndPopoverFocusNotProven);
    }

    // AC2: plain-language explanation for badge meaning is reachable from keyboard, screen reader, and
    // exported / support views. At least one clean badge reaches a plain-language explanation off-hover,
    // a hover-only example degrades, a plain-language-missing example degrades, and no clean badge is
    // hover-only.
    let clean_reachable_badge = badges().any(|ex| {
        ex.is_clean()
            && ex.explanation_reachable_by_keyboard_sr_export
            && ex.plain_language_explanation_present
    });
    let plain_missing_degrades = badges()
        .any(|ex| ex.degrade_reason == Some(M5BadgeDegradeReason::PlainLanguageExplanationMissing));
    let no_clean_hover_only =
        !badges().any(|ex| ex.is_clean() && !ex.explanation_reachable_by_keyboard_sr_export);
    if !(clean_reachable_badge
        && hover_only_degrades
        && plain_missing_degrades
        && no_clean_hover_only)
    {
        violations.push(M5BadgePopoverControlsViolation::PlainLanguageReachabilityNotProven);
    }

    // AC3: badge / popover drift is caught by fixtures or linting before release evidence turns green —
    // a user can trace badge and popover truth back to one canonical component contract, and a taxonomy
    // drift degrades. At least one clean badge and one clean popover both keep a reachable, canonical
    // trace.
    let traceable_badge =
        badges().any(|ex| ex.is_clean() && ex.explanation_reachable_by_keyboard_sr_export);
    let traceable_popover =
        popovers().any(|ex| ex.is_clean() && ex.content_reachable_without_hover);
    let taxonomy_drift_degrades = badges()
        .any(|ex| ex.degrade_reason == Some(M5BadgeDegradeReason::TaxonomyDriftedAcrossSurface));
    if !(traceable_badge && traceable_popover && taxonomy_drift_degrades) {
        violations.push(M5BadgePopoverControlsViolation::BadgePopoverDriftNotProven);
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
    M5DecisionFeedbackFamily::BadgeChipPill,
    M5DecisionFeedbackFamily::Popover,
];
