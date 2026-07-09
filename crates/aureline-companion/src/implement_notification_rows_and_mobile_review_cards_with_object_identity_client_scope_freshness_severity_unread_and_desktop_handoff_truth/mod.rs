//! Notification rows and mobile review cards carrying event/object identity,
//! repo/workspace client scope, freshness, severity/importance, unread state,
//! keyboard-complete quick triage verbs, an explicit companion-versus-desktop
//! capability boundary, and an exact desktop-handoff target.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_companion_component_matrix`] — the `notification_row` and the
//! `mobile_review_card` — into one implemented, export-safe packet with two co-equal
//! control vectors. Together they make the *first glance* at a companion event or
//! review item trustworthy: a user never has to infer what object a tap opens,
//! whether the card is fresh enough to trust, how urgent it is, or whether companion
//! execution is sufficient before acting.
//!
//! A [`NotificationRow`] always names the object it references, the repo/workspace
//! client scope behind it, its severity, its notification category, and its unread
//! state. Its delivery class is *derived* from the freshness class rather than
//! asserted: a stale, offline-held, or expired notification can never read as live.
//! It always offers a keyboard-complete `Open` verb that lands on one stable object,
//! and when scope must widen it names one exact desktop-handoff target — never a
//! generic activity page.
//!
//! A [`MobileReviewCard`] always names the review kind, the object it references, its
//! scope, its freshness, and its companion-versus-desktop capability boundary. Its
//! capability class is *derived* from the frozen disposition vocabulary rather than
//! asserted: a desktop-required or policy-blocked review can never read as
//! companion-completable, so a user can tell whether companion execution is
//! sufficient before tapping `Comment` or `Approve`. It always offers a
//! keyboard-complete `Open` verb that lands on one stable object, and every widening
//! verb names one exact desktop-handoff target.
//!
//! The object kinds ([`M5CompanionObjectKind`]), client scopes
//! ([`M5CompanionClientScope`]), freshness classes ([`M5CompanionFreshness`]),
//! dispositions ([`M5CompanionComponentDisposition`]), severities
//! ([`M5CompanionSeverity`]), review kinds ([`M5CompanionReviewKind`]), notification
//! categories ([`M5CompanionNotificationCategory`]), handoff targets
//! ([`M5CompanionHandoffTarget`]), degraded reasons ([`M5CompanionDegradedReason`]),
//! required labels ([`M5CompanionRequiredLabel`]), surface families
//! ([`M5CompanionSurfaceFamily`]), deployment lines ([`M5CompanionDeploymentLine`]),
//! consumer surfaces ([`M5CompanionConsumerSurface`]), accessibility routes
//! ([`M5CompanionAccessibilityRoute`]), and downgrade triggers
//! ([`M5CompanionDowngradeTrigger`]) are reused directly from the frozen matrix, so
//! this lane never invents a parallel companion vocabulary. It mints new vocabulary
//! only for what that matrix left implicit about these two controls: the derived
//! delivery class, the keyboard-complete notification triage verbs, the derived
//! review capability class, and the keyboard-complete review verbs.
//!
//! Raw file bodies, diff hunks, secret values, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json`](../../../../schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json).
//! The contract doc is
//! [`docs/companion/implement_notification_rows_and_mobile_review_cards.md`](../../../../docs/companion/implement_notification_rows_and_mobile_review_cards.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_notification_row_mobile_review_card_controls,
    seeded_notification_row_mobile_review_card_controls_mobile_review_card_desktop_required,
    seeded_notification_row_mobile_review_card_controls_notification_row_stale,
    NOTIFICATION_ROW_MOBILE_REVIEW_CARD_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The object kind, client scope, freshness, disposition, severity, review kind,
// notification category, handoff target, degraded reason, required labels, surface
// family, deployment line, consumer surface, accessibility route, and downgrade
// triggers are frozen once, in the companion component matrix. This lane reuses them
// verbatim so it never invents a parallel companion vocabulary.
use crate::freeze_the_m5_companion_component_matrix::{
    M5CompanionAccessibilityRoute, M5CompanionClientScope, M5CompanionComponentDisposition,
    M5CompanionComponentFamily, M5CompanionConsumerSurface, M5CompanionDegradedReason,
    M5CompanionDeploymentLine, M5CompanionDowngradeTrigger, M5CompanionFreshness,
    M5CompanionHandoffTarget, M5CompanionNotificationCategory, M5CompanionObjectKind,
    M5CompanionRequiredLabel, M5CompanionReviewKind, M5CompanionSeverity, M5CompanionSurfaceFamily,
    M5_COMPANION_COMPONENT_DOC_REF, M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF,
    M5_COMPANION_COMPONENT_SCHEMA_REF, M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
    M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`NotificationRowMobileReviewCardControlsPacket`].
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_RECORD_KIND: &str =
    "notification_row_mobile_review_card_controls";

/// Schema version for notification-row / mobile-review-card control records.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-notification-row-mobile-review-card-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_DOC_REF: &str =
    "docs/companion/implement_notification_rows_and_mobile_review_cards.md";

/// Repo-relative path of the protected fixture directory.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_FIXTURE_DIR: &str =
    "fixtures/ui/m5-notification-row-mobile-review-card-controls";

/// Repo-relative path of the checked support-export artifact.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_ARTIFACT_REF: &str =
    "artifacts/release/m5-notification-row-mobile-review-card-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SUMMARY_REF: &str =
    "artifacts/release/m5-notification-row-mobile-review-card-proof/summary.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NOTIFICATION_ROW_MOBILE_REVIEW_CARD_CSV_REF: &str =
    "artifacts/release/m5-notification-row-mobile-review-card-proof/matrix.csv";

// ---- notification-row vocabulary ----------------------------------------

/// Derived delivery class a notification row may present.
///
/// This is the notification honesty axis: the class is derived from the freshness
/// class, never asserted, so a stale, offline-held, or expired notification can never
/// present as live in a triage list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationDeliveryClass {
    /// Live from the local core via the relay.
    Live,
    /// A last-known cached value, not live.
    Cached,
    /// Stale, offline-held, or expired — never live.
    Stale,
    /// Freshness could not be determined.
    Unknown,
}

impl NotificationDeliveryClass {
    /// Every delivery class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Live, Self::Cached, Self::Stale, Self::Unknown];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Unknown => "unknown",
        }
    }
}

/// One keyboard-complete default triage verb a notification row offers, so a row
/// never hides its triage affordance behind a pointer-only gesture and every verb is
/// traceable to one stable object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationTriageVerb {
    /// Open the exact object this notification references.
    Open,
    /// Acknowledge the notification.
    Acknowledge,
    /// Mark the notification read.
    MarkRead,
    /// Mute future notifications for this object.
    Mute,
    /// Snooze the notification.
    Snooze,
    /// Hand off to the exact desktop target when scope must widen.
    HandoffToDesktop,
}

impl NotificationTriageVerb {
    /// Every triage verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Acknowledge,
        Self::MarkRead,
        Self::Mute,
        Self::Snooze,
        Self::HandoffToDesktop,
    ];

    /// The default verbs every keyboard-complete row must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Acknowledge => "acknowledge",
            Self::MarkRead => "mark_read",
            Self::Mute => "mute",
            Self::Snooze => "snooze",
            Self::HandoffToDesktop => "handoff_to_desktop",
        }
    }
}

/// Disclosures a notification row must carry, derived from the freshness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NotificationRowDisclosure {
    /// The derived delivery class this row may present.
    pub delivery_class: NotificationDeliveryClass,
    /// Whether the notification is live (streaming from the local core).
    pub is_live: bool,
    /// Whether the row must carry an explicit cached note.
    pub needs_cached_note: bool,
    /// Whether the row must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// Whether the row must carry an explicit unknown-freshness note.
    pub needs_unknown_note: bool,
}

/// Resolves the delivery truth a notification row may present.
///
/// A live notification is live. A cached notification is cached. A stale,
/// offline-held, or expired-snapshot notification is stale — never live. An
/// unknown-freshness notification is unknown, so a card whose freshness cannot be
/// determined never reads as live.
pub fn resolve_notification_delivery(freshness: M5CompanionFreshness) -> NotificationRowDisclosure {
    use M5CompanionFreshness as Fresh;
    use NotificationDeliveryClass as Delivery;

    let delivery_class = match freshness {
        Fresh::Live => Delivery::Live,
        Fresh::Cached => Delivery::Cached,
        Fresh::Stale | Fresh::OfflineHeld | Fresh::ExpiredSnapshot => Delivery::Stale,
        Fresh::UnknownFreshness => Delivery::Unknown,
    };

    NotificationRowDisclosure {
        delivery_class,
        is_live: matches!(delivery_class, Delivery::Live),
        needs_cached_note: matches!(delivery_class, Delivery::Cached),
        needs_stale_note: matches!(delivery_class, Delivery::Stale),
        needs_unknown_note: matches!(delivery_class, Delivery::Unknown),
    }
}

/// A notification row naming object identity, client scope, severity, category,
/// unread state, derived delivery, quick triage verbs, and an exact handoff target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRow {
    /// Frozen component this control implements; must be `notification_row`.
    pub component: M5CompanionComponentFamily,
    /// Stable row id.
    pub row_id: String,
    /// Human-readable event / notification label; required and non-empty.
    pub event_label: String,
    /// Object kind this notification references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Client scope this notification is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Severity of the referenced object, reused from the frozen matrix.
    pub severity: M5CompanionSeverity,
    /// Human-readable severity label; required and non-empty.
    pub severity_label: String,
    /// Notification category, reused from the frozen matrix.
    pub notification_category: M5CompanionNotificationCategory,
    /// Whether the notification is unread.
    pub is_unread: bool,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Derived delivery class (must equal the resolved class).
    pub delivery_class: NotificationDeliveryClass,
    /// Whether the row claims the notification is live (must equal the derived truth).
    pub claims_live: bool,
    /// Cached note; required when the notification is cached.
    pub cached_note: String,
    /// Stale note; required when the notification is stale.
    pub stale_note: String,
    /// Unknown-freshness note; required when freshness is unknown.
    pub unknown_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default triage verbs (must include the mandatory `Open`).
    pub triage_verbs: Vec<NotificationTriageVerb>,
    /// Degraded reasons this row can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this row can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this row.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this row offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this row's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks client scope or freshness. MUST be `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: never hides its companion-versus-desktop capability boundary.
    /// MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a desktop-required action is companion-safe. MUST
    /// be `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
}

impl NotificationRow {
    /// Delivery disclosures this row must carry, derived from the freshness class.
    pub fn delivery_disclosure(&self) -> NotificationRowDisclosure {
        resolve_notification_delivery(self.freshness)
    }

    /// Whether the row offers every mandatory keyboard-complete triage verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<NotificationTriageVerb> = self.triage_verbs.iter().copied().collect();
        NotificationTriageVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the row offers a desktop-handoff verb.
    fn offers_handoff(&self) -> bool {
        self.triage_verbs
            .contains(&NotificationTriageVerb::HandoffToDesktop)
    }
}

// ---- mobile-review-card vocabulary --------------------------------------

/// Derived companion-versus-desktop capability class a mobile review card may present.
///
/// This is the review honesty axis: the class is derived from the frozen disposition
/// vocabulary, never asserted, so a desktop-required or policy-blocked review can
/// never present as companion-completable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewCapabilityClass {
    /// The companion can view the review only.
    ReviewOnly,
    /// The companion can post a bounded comment.
    CommentCapable,
    /// The action must complete on desktop.
    DesktopRequired,
    /// The action is blocked by policy on the companion.
    PolicyBlocked,
}

impl ReviewCapabilityClass {
    /// Every capability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewOnly,
        Self::CommentCapable,
        Self::DesktopRequired,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewOnly => "review_only",
            Self::CommentCapable => "comment_capable",
            Self::DesktopRequired => "desktop_required",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One keyboard-complete default review verb a mobile review card offers, so a card
/// never hides its review affordance behind a pointer-only gesture and every widening
/// verb is traceable to one exact desktop target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MobileReviewVerb {
    /// Open the exact object this card references.
    Open,
    /// Post a bounded comment.
    Comment,
    /// Approve the change.
    Approve,
    /// Request changes.
    RequestChanges,
    /// Hand off to the exact desktop target.
    HandoffToDesktop,
    /// Dismiss the card.
    Dismiss,
}

impl MobileReviewVerb {
    /// Every review verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Comment,
        Self::Approve,
        Self::RequestChanges,
        Self::HandoffToDesktop,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete card must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Comment => "comment",
            Self::Approve => "approve",
            Self::RequestChanges => "request_changes",
            Self::HandoffToDesktop => "handoff_to_desktop",
            Self::Dismiss => "dismiss",
        }
    }
}

/// Disclosures a mobile review card must carry, derived from the disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReviewCardDisclosure {
    /// The derived capability class this card may present.
    pub capability_class: ReviewCapabilityClass,
    /// Whether companion execution is sufficient to complete the card's action.
    pub companion_execution_sufficient: bool,
    /// Whether the card must carry an explicit desktop-required note.
    pub needs_desktop_required_note: bool,
    /// Whether the card must carry an explicit policy-blocked note.
    pub needs_policy_blocked_note: bool,
}

/// Resolves the capability truth a mobile review card may present.
///
/// A comment-capable disposition is comment-capable and companion-sufficient. A
/// review-only, cached, or stale disposition floors to review-only and is
/// companion-sufficient for viewing. A desktop-required or handoff-ready disposition
/// is desktop-required and never companion-sufficient. A policy-blocked disposition
/// is policy-blocked and never companion-sufficient, so a desktop-required or
/// policy-blocked review can never read as companion-completable.
pub fn resolve_review_capability(
    disposition: M5CompanionComponentDisposition,
) -> ReviewCardDisclosure {
    use M5CompanionComponentDisposition as Disp;
    use ReviewCapabilityClass as Capability;

    let capability_class = match disposition {
        Disp::CommentCapable => Capability::CommentCapable,
        Disp::ReviewOnly | Disp::Cached | Disp::Stale => Capability::ReviewOnly,
        Disp::DesktopRequired | Disp::HandoffReady => Capability::DesktopRequired,
        Disp::PolicyBlocked => Capability::PolicyBlocked,
    };

    ReviewCardDisclosure {
        capability_class,
        companion_execution_sufficient: matches!(
            capability_class,
            Capability::CommentCapable | Capability::ReviewOnly
        ),
        needs_desktop_required_note: matches!(capability_class, Capability::DesktopRequired),
        needs_policy_blocked_note: matches!(capability_class, Capability::PolicyBlocked),
    }
}

/// A mobile review card naming review kind, object identity, scope, freshness,
/// capability boundary, derived companion sufficiency, and an exact handoff target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobileReviewCard {
    /// Frozen component this control implements; must be `mobile_review_card`.
    pub component: M5CompanionComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable review label; required and non-empty.
    pub review_label: String,
    /// Object kind this card references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Client scope this card is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Review kind, reused from the frozen matrix.
    pub review_kind: M5CompanionReviewKind,
    /// Human-readable review-kind label; required and non-empty.
    pub review_kind_label: String,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Whether the card is unread.
    pub is_unread: bool,
    /// Disposition (capability boundary), reused from the frozen matrix.
    pub disposition: M5CompanionComponentDisposition,
    /// Derived capability class (must equal the resolved class).
    pub capability_class: ReviewCapabilityClass,
    /// Whether the card claims companion execution is sufficient (must equal the
    /// derived truth).
    pub claims_companion_sufficient: bool,
    /// Capability note; always required so the capability boundary stays explicit.
    pub capability_note: String,
    /// Desktop-required note; required when the action must complete on desktop.
    pub desktop_required_note: String,
    /// Policy-blocked note; required when the action is policy-blocked.
    pub policy_blocked_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default review verbs (must include the mandatory `Open`).
    pub review_verbs: Vec<MobileReviewVerb>,
    /// Degraded reasons this card can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this card can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this card.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this card keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this card offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this card's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this card.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks client scope or freshness. MUST be `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: never hides its companion-versus-desktop capability boundary.
    /// MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be
    /// `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a desktop-required action is companion-safe. MUST
    /// be `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
}

impl MobileReviewCard {
    /// Capability disclosures this card must carry, derived from the disposition.
    pub fn capability_disclosure(&self) -> ReviewCardDisclosure {
        resolve_review_capability(self.disposition)
    }

    /// Whether the card offers every mandatory keyboard-complete review verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<MobileReviewVerb> = self.review_verbs.iter().copied().collect();
        MobileReviewVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the card declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the card offers a desktop-handoff verb.
    fn offers_handoff(&self) -> bool {
        self.review_verbs
            .contains(&MobileReviewVerb::HandoffToDesktop)
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance trust review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRowMobileReviewCardGlanceReview {
    /// The notification row names its object identity and severity.
    pub notification_row_shows_object_and_severity: bool,
    /// The notification row names its client scope and freshness.
    pub notification_row_shows_scope_and_freshness: bool,
    /// The notification row names its unread state.
    pub notification_row_shows_unread_state: bool,
    /// The mobile review card names its companion-versus-desktop capability boundary.
    pub review_card_shows_capability_boundary: bool,
    /// The mobile review card states whether companion execution is sufficient.
    pub review_card_states_companion_sufficiency: bool,
    /// The mobile review card names its review kind.
    pub review_card_shows_review_kind: bool,
    /// The object identity is always explicit.
    pub object_identity_always_explicit: bool,
    /// The client scope is always explicit.
    pub client_scope_always_explicit: bool,
    /// The freshness is always explicit.
    pub freshness_always_explicit: bool,
    /// Delivery / capability is derived from freshness / disposition, never asserted.
    pub delivery_and_capability_derived_never_asserted: bool,
    /// A stale card is never shown as live.
    pub stale_never_shown_as_live: bool,
    /// Every triage verb traces to one stable object.
    pub every_verb_traces_to_one_object: bool,
    /// Every widening verb names one exact desktop-handoff target.
    pub every_handoff_names_exact_target: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl NotificationRowMobileReviewCardGlanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.notification_row_shows_object_and_severity
            && self.notification_row_shows_scope_and_freshness
            && self.notification_row_shows_unread_state
            && self.review_card_shows_capability_boundary
            && self.review_card_states_companion_sufficiency
            && self.review_card_shows_review_kind
            && self.object_identity_always_explicit
            && self.client_scope_always_explicit
            && self.freshness_always_explicit
            && self.delivery_and_capability_derived_never_asserted
            && self.stale_never_shown_as_live
            && self.every_verb_traces_to_one_object
            && self.every_handoff_names_exact_target
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRowMobileReviewCardConsumerProjection {
    /// The notification-triage UI reads a single canonical source.
    pub notification_triage_ui_reads_single_source: bool,
    /// The review-queue UI reads a single canonical source.
    pub review_queue_ui_reads_single_source: bool,
    /// The first glance names object, scope, and freshness without drilling in.
    pub first_glance_names_object_scope_and_freshness: bool,
    /// The companion capability boundary is visible before a tap.
    pub capability_boundary_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl NotificationRowMobileReviewCardConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.notification_triage_ui_reads_single_source
            && self.review_queue_ui_reads_single_source
            && self.first_glance_names_object_scope_and_freshness
            && self.capability_boundary_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRowMobileReviewCardProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`NotificationRowMobileReviewCardControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotificationRowMobileReviewCardControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Notification rows.
    pub notification_rows: Vec<NotificationRow>,
    /// Mobile review cards.
    pub review_cards: Vec<MobileReviewCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: NotificationRowMobileReviewCardGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: NotificationRowMobileReviewCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: NotificationRowMobileReviewCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe notification-row / mobile-review-card controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRowMobileReviewCardControlsPacket {
    /// Record kind; must equal [`NOTIFICATION_ROW_MOBILE_REVIEW_CARD_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Notification rows.
    pub notification_rows: Vec<NotificationRow>,
    /// Mobile review cards.
    pub review_cards: Vec<MobileReviewCard>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: NotificationRowMobileReviewCardGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: NotificationRowMobileReviewCardConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: NotificationRowMobileReviewCardProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl NotificationRowMobileReviewCardControlsPacket {
    /// Builds a notification-row / mobile-review-card controls packet from stable-lane input.
    pub fn new(input: NotificationRowMobileReviewCardControlsPacketInput) -> Self {
        Self {
            record_kind: NOTIFICATION_ROW_MOBILE_REVIEW_CARD_RECORD_KIND.to_owned(),
            schema_version: NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            notification_rows: input.notification_rows,
            review_cards: input.review_cards,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            glance_review: input.glance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the notification-row / mobile-review-card control invariants.
    pub fn validate(&self) -> Vec<NotificationRowMobileReviewCardViolation> {
        let mut violations = Vec::new();

        if self.record_kind != NOTIFICATION_ROW_MOBILE_REVIEW_CARD_RECORD_KIND {
            violations.push(NotificationRowMobileReviewCardViolation::WrongRecordKind);
        }
        if self.schema_version != NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_VERSION {
            violations.push(NotificationRowMobileReviewCardViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(NotificationRowMobileReviewCardViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_notification_rows(self, &mut violations);
        validate_review_cards(self, &mut violations);

        if !self.glance_review.all_hold() {
            violations.push(NotificationRowMobileReviewCardViolation::GlanceReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(NotificationRowMobileReviewCardViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(NotificationRowMobileReviewCardViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("notification row mobile review card packet serializes"),
        ) {
            violations.push(NotificationRowMobileReviewCardViolation::RawBoundaryMaterialInExport);
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
            .expect("notification row mobile review card packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,object_kind,client_scope,freshness,state_or_kind,derived,live_or_sufficient\n",
        );
        for row in &self.notification_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "notification_row",
                csv_field(&row.row_id),
                row.object_kind.as_str(),
                row.client_scope.as_str(),
                row.freshness.as_str(),
                row.severity.as_str(),
                row.delivery_disclosure().delivery_class.as_str(),
                row.delivery_disclosure().is_live,
            ));
        }
        for card in &self.review_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "mobile_review_card",
                csv_field(&card.card_id),
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.freshness.as_str(),
                card.review_kind.as_str(),
                card.capability_disclosure().capability_class.as_str(),
                card.capability_disclosure().companion_execution_sufficient,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .notification_rows
            .iter()
            .filter(|row| !row.delivery_disclosure().is_live)
            .count();
        let desktop_bound = self
            .review_cards
            .iter()
            .filter(|card| !card.capability_disclosure().companion_execution_sufficient)
            .count();

        let mut out = String::new();
        out.push_str("# Notification rows and mobile review cards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Notification rows: {} ({} not live)\n",
            self.notification_rows.len(),
            not_live
        ));
        out.push_str(&format!(
            "- Mobile review cards: {} ({} not companion-sufficient)\n",
            self.review_cards.len(),
            desktop_bound
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Notification rows\n\n");
        for row in &self.notification_rows {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, severity `{}`, freshness `{}` → `{}`, handoff `{}`\n",
                row.event_label,
                row.object_kind.as_str(),
                row.client_scope.as_str(),
                row.severity.as_str(),
                row.freshness.as_str(),
                row.delivery_disclosure().delivery_class.as_str(),
                row.handoff_target.as_str(),
            ));
        }

        out.push_str("\n## Mobile review cards\n\n");
        for card in &self.review_cards {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, kind `{}`, disposition `{}` → `{}`, handoff `{}`\n",
                card.review_label,
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.review_kind.as_str(),
                card.disposition.as_str(),
                card.capability_disclosure().capability_class.as_str(),
                card.handoff_target.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in notification-row / review-card export.
#[derive(Debug)]
pub enum NotificationRowMobileReviewCardArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NotificationRowMobileReviewCardViolation>),
}

impl fmt::Display for NotificationRowMobileReviewCardArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "notification row mobile review card export parse failed: {error}"
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
                    "notification row mobile review card export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for NotificationRowMobileReviewCardArtifactError {}

/// Validation failures emitted by [`NotificationRowMobileReviewCardControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NotificationRowMobileReviewCardViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No notification rows are present.
    NotificationRowsMissing,
    /// A notification row is incomplete.
    NotificationRowIncomplete,
    /// A notification row carries the wrong frozen component class.
    NotificationRowWrongComponentClass,
    /// A notification row does not name its exact object landing reference.
    ObjectLandingRefMissing,
    /// A notification row misrepresents its derived delivery state.
    DeliveryStateMisrepresented,
    /// A control does not name its scope / freshness.
    ScopeAndFreshnessNoteMissing,
    /// A cached notification does not name its cached state.
    CachedNoteMissing,
    /// A stale notification does not name its stale state.
    StaleNoteMissing,
    /// An unknown-freshness notification does not name its unknown state.
    UnknownNoteMissing,
    /// A notification row does not name its severity.
    SeverityLabelMissing,
    /// A notification row does not name its scope label.
    ScopeLabelMissing,
    /// A notification row omits the mandatory `Open` verb.
    NotificationVerbsIncomplete,
    /// A control offers a handoff verb but its handoff target does not resolve exactly.
    HandoffTargetUnresolved,
    /// A control does not name its handoff label.
    HandoffLabelMissing,
    /// The notification rows do not cover every derived delivery class.
    DeliveryClassCoverageMissing,
    /// The notification rows do not cover every severity.
    SeverityCoverageMissing,
    /// No mobile review cards are present.
    ReviewCardsMissing,
    /// A mobile review card is incomplete.
    ReviewCardIncomplete,
    /// A mobile review card carries the wrong frozen component class.
    ReviewCardWrongComponentClass,
    /// A mobile review card misrepresents its derived capability state.
    CapabilityMisrepresented,
    /// A mobile review card does not name its capability note.
    CapabilityNoteMissing,
    /// A desktop-required card does not name its desktop-required state.
    DesktopRequiredNoteMissing,
    /// A policy-blocked card does not name its policy-blocked state.
    PolicyBlockedNoteMissing,
    /// A mobile review card does not name its review kind.
    ReviewKindLabelMissing,
    /// A mobile review card omits the mandatory `Open` verb.
    ReviewVerbsIncomplete,
    /// The mobile review cards do not cover every capability class.
    CapabilityClassCoverageMissing,
    /// The mobile review cards do not cover every review kind.
    ReviewKindCoverageMissing,
    /// A control does not declare its degraded reasons.
    DegradedReasonsMissing,
    /// A control does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A control does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A control masks its client scope or freshness.
    ScopeOrFreshnessMasked,
    /// A control hides its companion-versus-desktop capability boundary.
    CapabilityBoundaryHidden,
    /// A control invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A control implies a desktop-required action is companion-safe.
    DesktopActionImpliedCompanionSafe,
    /// A control routes to a generic activity page instead of one stable object.
    RoutesToGenericActivityPage,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Glance review does not satisfy required invariants.
    GlanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl NotificationRowMobileReviewCardViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NotificationRowsMissing => "notification_rows_missing",
            Self::NotificationRowIncomplete => "notification_row_incomplete",
            Self::NotificationRowWrongComponentClass => "notification_row_wrong_component_class",
            Self::ObjectLandingRefMissing => "object_landing_ref_missing",
            Self::DeliveryStateMisrepresented => "delivery_state_misrepresented",
            Self::ScopeAndFreshnessNoteMissing => "scope_and_freshness_note_missing",
            Self::CachedNoteMissing => "cached_note_missing",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::UnknownNoteMissing => "unknown_note_missing",
            Self::SeverityLabelMissing => "severity_label_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::NotificationVerbsIncomplete => "notification_verbs_incomplete",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::HandoffLabelMissing => "handoff_label_missing",
            Self::DeliveryClassCoverageMissing => "delivery_class_coverage_missing",
            Self::SeverityCoverageMissing => "severity_coverage_missing",
            Self::ReviewCardsMissing => "review_cards_missing",
            Self::ReviewCardIncomplete => "review_card_incomplete",
            Self::ReviewCardWrongComponentClass => "review_card_wrong_component_class",
            Self::CapabilityMisrepresented => "capability_misrepresented",
            Self::CapabilityNoteMissing => "capability_note_missing",
            Self::DesktopRequiredNoteMissing => "desktop_required_note_missing",
            Self::PolicyBlockedNoteMissing => "policy_blocked_note_missing",
            Self::ReviewKindLabelMissing => "review_kind_label_missing",
            Self::ReviewVerbsIncomplete => "review_verbs_incomplete",
            Self::CapabilityClassCoverageMissing => "capability_class_coverage_missing",
            Self::ReviewKindCoverageMissing => "review_kind_coverage_missing",
            Self::DegradedReasonsMissing => "degraded_reasons_missing",
            Self::RequiredLabelsIncomplete => "required_labels_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ScopeOrFreshnessMasked => "scope_or_freshness_masked",
            Self::CapabilityBoundaryHidden => "capability_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::DesktopActionImpliedCompanionSafe => "desktop_action_implied_companion_safe",
            Self::RoutesToGenericActivityPage => "routes_to_generic_activity_page",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GlanceReviewIncomplete => "glance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable notification-row / review-card export.
pub fn current_notification_row_mobile_review_card_export() -> Result<
    NotificationRowMobileReviewCardControlsPacket,
    NotificationRowMobileReviewCardArtifactError,
> {
    let packet: NotificationRowMobileReviewCardControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-notification-row-mobile-review-card-proof/support_export.json"
        )))
        .map_err(NotificationRowMobileReviewCardArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NotificationRowMobileReviewCardArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &NotificationRowMobileReviewCardControlsPacket,
    violations: &mut Vec<NotificationRowMobileReviewCardViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_SCHEMA_REF,
        NOTIFICATION_ROW_MOBILE_REVIEW_CARD_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(NotificationRowMobileReviewCardViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_notification_rows(
    packet: &NotificationRowMobileReviewCardControlsPacket,
    violations: &mut Vec<NotificationRowMobileReviewCardViolation>,
) {
    if packet.notification_rows.is_empty() {
        violations.push(NotificationRowMobileReviewCardViolation::NotificationRowsMissing);
        return;
    }

    let mut delivery_classes: BTreeSet<NotificationDeliveryClass> = BTreeSet::new();
    let mut severities: BTreeSet<M5CompanionSeverity> = BTreeSet::new();

    for row in &packet.notification_rows {
        let disclosure = row.delivery_disclosure();
        delivery_classes.insert(disclosure.delivery_class);
        severities.insert(row.severity);

        if row.row_id.trim().is_empty()
            || row.event_label.trim().is_empty()
            || row.object_label.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.consumer_surfaces.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(NotificationRowMobileReviewCardViolation::NotificationRowIncomplete);
        }
        if row.component != M5CompanionComponentFamily::NotificationRow {
            violations
                .push(NotificationRowMobileReviewCardViolation::NotificationRowWrongComponentClass);
        }
        if row.object_landing_ref.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ObjectLandingRefMissing);
        }
        if row.delivery_class != disclosure.delivery_class || row.claims_live != disclosure.is_live
        {
            violations.push(NotificationRowMobileReviewCardViolation::DeliveryStateMisrepresented);
        }
        if row.scope_and_freshness_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ScopeAndFreshnessNoteMissing);
        }
        if disclosure.needs_cached_note && row.cached_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::CachedNoteMissing);
        }
        if disclosure.needs_stale_note && row.stale_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::StaleNoteMissing);
        }
        if disclosure.needs_unknown_note && row.unknown_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::UnknownNoteMissing);
        }
        if row.severity_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::SeverityLabelMissing);
        }
        if row.scope_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ScopeLabelMissing);
        }
        if !row.declares_mandatory_verbs() {
            violations.push(NotificationRowMobileReviewCardViolation::NotificationVerbsIncomplete);
        }
        if row.offers_handoff() && row.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations.push(NotificationRowMobileReviewCardViolation::HandoffTargetUnresolved);
        }
        if row.handoff_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &row.degraded_reasons,
            row.declares_mandatory_labels(),
            &row.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: row.masks_scope_or_freshness,
                hides_capability_boundary: row.hides_capability_boundary,
                invents_alternate_state_label: row.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: row
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: row.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in NotificationDeliveryClass::ALL {
        if !delivery_classes.contains(&required) {
            violations.push(NotificationRowMobileReviewCardViolation::DeliveryClassCoverageMissing);
            break;
        }
    }
    for required in M5CompanionSeverity::ALL {
        if !severities.contains(&required) {
            violations.push(NotificationRowMobileReviewCardViolation::SeverityCoverageMissing);
            break;
        }
    }
}

fn validate_review_cards(
    packet: &NotificationRowMobileReviewCardControlsPacket,
    violations: &mut Vec<NotificationRowMobileReviewCardViolation>,
) {
    if packet.review_cards.is_empty() {
        violations.push(NotificationRowMobileReviewCardViolation::ReviewCardsMissing);
        return;
    }

    let mut capability_classes: BTreeSet<ReviewCapabilityClass> = BTreeSet::new();
    let mut review_kinds: BTreeSet<M5CompanionReviewKind> = BTreeSet::new();

    for card in &packet.review_cards {
        let disclosure = card.capability_disclosure();
        capability_classes.insert(disclosure.capability_class);
        review_kinds.insert(card.review_kind);

        if card.card_id.trim().is_empty()
            || card.review_label.trim().is_empty()
            || card.object_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(NotificationRowMobileReviewCardViolation::ReviewCardIncomplete);
        }
        if card.component != M5CompanionComponentFamily::MobileReviewCard {
            violations
                .push(NotificationRowMobileReviewCardViolation::ReviewCardWrongComponentClass);
        }
        if card.object_landing_ref.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ObjectLandingRefMissing);
        }
        if card.capability_class != disclosure.capability_class
            || card.claims_companion_sufficient != disclosure.companion_execution_sufficient
        {
            violations.push(NotificationRowMobileReviewCardViolation::CapabilityMisrepresented);
        }
        if card.capability_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::CapabilityNoteMissing);
        }
        if disclosure.needs_desktop_required_note && card.desktop_required_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::DesktopRequiredNoteMissing);
        }
        if disclosure.needs_policy_blocked_note && card.policy_blocked_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::PolicyBlockedNoteMissing);
        }
        if card.scope_and_freshness_note.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ScopeAndFreshnessNoteMissing);
        }
        if card.review_kind_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ReviewKindLabelMissing);
        }
        if card.scope_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::ScopeLabelMissing);
        }
        if !card.declares_mandatory_verbs() {
            violations.push(NotificationRowMobileReviewCardViolation::ReviewVerbsIncomplete);
        }
        if card.offers_handoff() && card.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations.push(NotificationRowMobileReviewCardViolation::HandoffTargetUnresolved);
        }
        if card.handoff_label.trim().is_empty() {
            violations.push(NotificationRowMobileReviewCardViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &card.degraded_reasons,
            card.declares_mandatory_labels(),
            &card.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: card.masks_scope_or_freshness,
                hides_capability_boundary: card.hides_capability_boundary,
                invents_alternate_state_label: card.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: card
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: card.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in ReviewCapabilityClass::ALL {
        if !capability_classes.contains(&required) {
            violations
                .push(NotificationRowMobileReviewCardViolation::CapabilityClassCoverageMissing);
            break;
        }
    }
    for required in M5CompanionReviewKind::ALL {
        if !review_kinds.contains(&required) {
            violations.push(NotificationRowMobileReviewCardViolation::ReviewKindCoverageMissing);
            break;
        }
    }
}

/// The five hard-invariant bools every control must keep `false`.
struct ControlInvariants {
    masks_scope_or_freshness: bool,
    hides_capability_boundary: bool,
    invents_alternate_state_label: bool,
    implies_desktop_action_is_companion_safe: bool,
    routes_to_generic_activity_page: bool,
}

/// Validates the axes shared by both control vectors.
fn validate_common_control(
    degraded_reasons: &[M5CompanionDegradedReason],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5CompanionAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<NotificationRowMobileReviewCardViolation>,
) {
    if degraded_reasons.is_empty() {
        violations.push(NotificationRowMobileReviewCardViolation::DegradedReasonsMissing);
    }
    if !declares_mandatory_labels {
        violations.push(NotificationRowMobileReviewCardViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5CompanionAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(NotificationRowMobileReviewCardViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_scope_or_freshness {
        violations.push(NotificationRowMobileReviewCardViolation::ScopeOrFreshnessMasked);
    }
    if invariants.hides_capability_boundary {
        violations.push(NotificationRowMobileReviewCardViolation::CapabilityBoundaryHidden);
    }
    if invariants.invents_alternate_state_label {
        violations.push(NotificationRowMobileReviewCardViolation::AlternateStateLabelInvented);
    }
    if invariants.implies_desktop_action_is_companion_safe {
        violations
            .push(NotificationRowMobileReviewCardViolation::DesktopActionImpliedCompanionSafe);
    }
    if invariants.routes_to_generic_activity_page {
        violations.push(NotificationRowMobileReviewCardViolation::RoutesToGenericActivityPage);
    }
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
///
/// The companion vocabulary carries no secret-value words, so this check flags only
/// raw-*value* shapes that must never cross the boundary: a password / passphrase
/// literal, a bearer literal, a URL scheme, or a PEM header.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
