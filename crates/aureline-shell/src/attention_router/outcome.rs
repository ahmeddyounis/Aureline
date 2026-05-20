//! The governed attention router and its single route-outcome object.
//!
//! [`AttentionRouter`] is the one governed routing system the shell uses to
//! turn a [`NotificationEnvelope`] into a [`NotificationRouteOutcome`] — a
//! single typed object that reasons about the same alert consistently across
//! in-app toasts, banners, status overflow, the activity center, native OS
//! notifications, and companion fanout.
//!
//! It composes, rather than replaces, the existing notification primitives:
//!
//! 1. The shell's live [`ChannelContext`] is folded into an effective
//!    quiet-hours posture and applied to a *clone* of the envelope, so the
//!    router never widens authority by mutating the caller's envelope.
//! 2. The dedupe + suppression core
//!    ([`crate::notifications::router::NotificationRouter`]) routes that
//!    envelope and decides delivered / held / suppressed / deduped per surface.
//! 3. The live channel state then *narrows* each surface: a focused foreground
//!    window drops the redundant OS toast, a non-locked device drops the
//!    lock-screen summary, and an unreachable or policy-blocked companion
//!    drops the push — always recording a visible receipt, never upgrading a
//!    held / suppressed / deduped surface back to delivered.
//!
//! Every resolved surface keeps the envelope's single `reopen_target`, so a
//! toast, a status row, an OS notification, and a companion push all reopen the
//! same canonical object — never a generic home view.

use serde::{Deserialize, Serialize};

use crate::notifications::envelope::{
    DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope, PrivacyClass,
    PrivacyPayloadClass, RedactionClass, ReopenTarget, ReopenTargetKind, SeverityClass,
    SourceSubsystem, StableAction, StaleOrUndeliveredReason, StaleOrUndeliveredReasonClass,
    SuppressionReason,
};
use crate::notifications::external::ForbiddenShortcutActionClass;
use crate::notifications::router::{NotificationRouter, NotificationRoutingError, SurfaceRoute};

use super::context::{ChannelContext, ChannelContextSnapshot, CompanionAvailability};
use super::lifecycle::{governed_user_actions, AvailableLifecycleAction};

/// Schema version for [`NotificationRouteOutcome`] payloads. Mirrors
/// `schemas/ux/notification_route_outcome.schema.json`.
pub const NOTIFICATION_ROUTE_OUTCOME_SCHEMA_VERSION: u32 = 1;

/// Stable record kind for [`NotificationRouteOutcome`] payloads.
pub const NOTIFICATION_ROUTE_OUTCOME_RECORD_KIND: &str = "notification_route_outcome_record";

/// How the live channel context resolved one surface after the dedupe core
/// made its decision. This is the reviewable "why" recorded on every route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChannelResolutionClass {
    /// Delivered to an in-product surface (toast, banner, status, durable row,
    /// activity center).
    DeliveredInApp,
    /// Delivered to an external summary surface (OS notification, lock-screen,
    /// companion push).
    DeliveredExternalSummary,
    /// An OS notification was dropped because the target window is foreground
    /// and focused; the in-app surface carries the interruption.
    SuppressedForegroundRedundant,
    /// A lock-screen summary was dropped because the device is not locked.
    LockScreenNotApplicable,
    /// A companion push was not attempted because no reachable companion
    /// endpoint is available.
    CompanionUnavailable,
    /// A companion push was suppressed because managed policy blocks it.
    CompanionPolicyBlocked,
    /// The dedupe core held this surface for quiet hours / focus / presentation.
    HeldByQuietHoursOrFocus,
    /// The dedupe core suppressed this surface by policy (admin, privacy mode,
    /// or lock-screen payload denial).
    SuppressedByPolicy,
    /// The dedupe core coalesced this surface into an existing event/burst.
    DedupedRepeat,
    /// The dedupe core had no route for this surface on this client.
    NoRouteFromCore,
}

impl ChannelResolutionClass {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeliveredInApp => "delivered_in_app",
            Self::DeliveredExternalSummary => "delivered_external_summary",
            Self::SuppressedForegroundRedundant => "suppressed_foreground_redundant",
            Self::LockScreenNotApplicable => "lock_screen_not_applicable",
            Self::CompanionUnavailable => "companion_unavailable",
            Self::CompanionPolicyBlocked => "companion_policy_blocked",
            Self::HeldByQuietHoursOrFocus => "held_by_quiet_hours_or_focus",
            Self::SuppressedByPolicy => "suppressed_by_policy",
            Self::DedupedRepeat => "deduped_repeat",
            Self::NoRouteFromCore => "no_route_from_core",
        }
    }
}

/// One surface decision after live-channel resolution. Carries both the core
/// dedupe decision and the resolved decision so support can see what the
/// channel context changed and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResolvedSurfaceRoute {
    pub fanout_surface_class: FanoutSurfaceClass,
    pub core_receipt_state: FanoutReceiptState,
    pub resolved_receipt_state: FanoutReceiptState,
    pub channel_resolution_class: ChannelResolutionClass,
    pub stale_or_undelivered_reason: StaleOrUndeliveredReason,
    #[serde(default)]
    pub suppression_reasons: Vec<SuppressionReason>,
    pub reopen_target_ref: String,
    pub redaction_class: RedactionClass,
    pub is_external_summary_surface: bool,
    pub visible: bool,
}

impl ResolvedSurfaceRoute {
    /// True when the resolved surface actually renders.
    pub fn is_visible(&self) -> bool {
        matches!(
            self.resolved_receipt_state,
            FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
        )
    }
}

/// How companion fanout resolved for this outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionHandoffClass {
    /// No companion surface was recommended for this envelope.
    NotApplicable,
    /// A summary-first companion push delivered.
    SummaryFanoutDelivered,
    /// A companion push was held by quiet hours / focus / presentation.
    SummaryFanoutHeld,
    /// No reachable companion endpoint was available.
    SummaryFanoutUnavailable,
    /// Managed policy blocked the companion push.
    SummaryFanoutPolicyBlocked,
}

impl CompanionHandoffClass {
    /// Stable token recorded in outcomes and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::SummaryFanoutDelivered => "summary_fanout_delivered",
            Self::SummaryFanoutHeld => "summary_fanout_held",
            Self::SummaryFanoutUnavailable => "summary_fanout_unavailable",
            Self::SummaryFanoutPolicyBlocked => "summary_fanout_policy_blocked",
        }
    }
}

/// Companion / OS handoff posture. Summary-first by default; privileged detail
/// and stale-target repair always reopen into a durable in-product object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionHandoffPosture {
    pub handoff_class: CompanionHandoffClass,
    pub companion_availability: CompanionAvailability,
    /// Companion / OS surfaces are summary-first; never raw detail.
    pub summary_only: bool,
    /// Activation reopens into a durable object or truthful placeholder.
    pub reopen_into_durable_object_required: bool,
    /// Privileged detail must be inspected in-product, never on the summary.
    pub privileged_detail_requires_in_product: bool,
    /// Shortcut classes the handoff refuses to complete off the summary.
    pub forbidden_shortcut_action_classes: Vec<ForbiddenShortcutActionClass>,
}

/// The single governed route outcome for one envelope under one channel
/// context. This is the object every surface reads — the exit-gate "one typed
/// notification object with stable routing, privacy, dedupe, and action-target
/// semantics instead of surface-local alert behavior".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationRouteOutcome {
    pub record_kind: String,
    pub notification_route_outcome_schema_version: u32,
    pub route_outcome_id: String,
    pub source_notification_envelope_id_ref: String,
    pub canonical_event_id: String,
    pub event_lineage_id_ref: String,
    pub source_subsystem: SourceSubsystem,
    pub severity_class: SeverityClass,
    pub privacy_class: PrivacyClass,
    pub privacy_payload_class: PrivacyPayloadClass,
    pub redaction_class: RedactionClass,
    pub dedupe_key_scheme: DedupeKeyScheme,
    pub dedupe_key_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grouped_burst_id_ref: Option<String>,
    pub summary_label: String,
    pub channel_context: ChannelContextSnapshot,
    pub reopen_target: ReopenTarget,
    pub resolved_surface_routes: Vec<ResolvedSurfaceRoute>,
    pub available_lifecycle_actions: Vec<AvailableLifecycleAction>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub safe_action_target: Option<StableAction>,
    pub companion_handoff: CompanionHandoffPosture,
    pub occurrence_count: u32,
    pub is_dedupe_repeat: bool,
    pub durable_truth_preserved: bool,
    pub all_routes_preserve_reopen_target: bool,
    pub screen_reader_announce_required: bool,
    pub screen_reader_navigable_surface_present: bool,
    pub no_generic_home_reopen: bool,
    pub minted_at: String,
}

impl NotificationRouteOutcome {
    /// Resolved routes that actually render.
    pub fn visible_routes(&self) -> impl Iterator<Item = &ResolvedSurfaceRoute> {
        self.resolved_surface_routes
            .iter()
            .filter(|route| route.is_visible())
    }

    /// True when at least one external summary surface delivered.
    pub fn has_external_delivery(&self) -> bool {
        self.resolved_surface_routes
            .iter()
            .any(|route| route.is_external_summary_surface && route.is_visible())
    }
}

/// The governed attention router. Wraps the dedupe core and resolves each
/// envelope against the shell's live channel context.
#[derive(Debug, Clone, Default)]
pub struct AttentionRouter {
    core: NotificationRouter,
}

impl AttentionRouter {
    /// Build a desktop-scoped attention router.
    pub fn new() -> Self {
        Self {
            core: NotificationRouter::new(),
        }
    }

    /// Reset the dedupe memory of the underlying core (test convenience).
    pub fn reset(&mut self) {
        self.core.reset();
    }

    /// Route one envelope under the given live channel context and produce a
    /// single governed [`NotificationRouteOutcome`].
    pub fn route(
        &mut self,
        envelope: &NotificationEnvelope,
        context: &ChannelContext,
    ) -> Result<NotificationRouteOutcome, NotificationRoutingError> {
        // 1. Fold the live context into an effective quiet posture and apply it
        //    to a clone so the caller's envelope authority is never widened.
        let mut effective_envelope = envelope.clone();
        context
            .effective_posture()
            .apply_to_envelope(&mut effective_envelope);

        // 2. Route through the dedupe + suppression core.
        let routed = self.core.route(&effective_envelope)?;

        // 3. Narrow each surface against the live channel state.
        let resolved_surface_routes: Vec<ResolvedSurfaceRoute> = routed
            .surface_routes
            .iter()
            .map(|route| resolve_surface(route, context))
            .collect();

        let companion_handoff = derive_companion_handoff(&resolved_surface_routes, context);
        let safe_action_target = routed
            .actions
            .iter()
            .find(|action| action_is_safe_external_primary(action, &routed.reopen_target))
            .cloned();

        let durable_truth_preserved = resolved_surface_routes
            .iter()
            .any(route_preserves_durable_path);
        let all_routes_preserve_reopen_target = resolved_surface_routes
            .iter()
            .all(|route| route.reopen_target_ref == routed.reopen_target.reopen_target_ref);
        let screen_reader_navigable_surface_present = resolved_surface_routes
            .iter()
            .any(|route| is_navigable_surface(route.fanout_surface_class) && route.is_visible());

        Ok(NotificationRouteOutcome {
            record_kind: NOTIFICATION_ROUTE_OUTCOME_RECORD_KIND.to_owned(),
            notification_route_outcome_schema_version: NOTIFICATION_ROUTE_OUTCOME_SCHEMA_VERSION,
            route_outcome_id: format!("route-outcome:{}", routed.notification_envelope_id),
            source_notification_envelope_id_ref: routed.notification_envelope_id.clone(),
            canonical_event_id: routed.canonical_event_id.clone(),
            event_lineage_id_ref: routed.event_lineage_id_ref.clone(),
            source_subsystem: routed.source_subsystem,
            severity_class: routed.severity_class,
            privacy_class: routed.privacy_class,
            privacy_payload_class: routed.privacy_payload_class,
            redaction_class: routed.redaction_class,
            dedupe_key_scheme: routed.dedupe_key_scheme,
            dedupe_key_ref: routed.dedupe_key_ref.clone(),
            grouped_burst_id_ref: routed.grouped_burst_id_ref.clone(),
            summary_label: routed.summary_label.clone(),
            channel_context: context.snapshot(),
            reopen_target: routed.reopen_target.clone(),
            resolved_surface_routes,
            available_lifecycle_actions: governed_user_actions(),
            safe_action_target,
            companion_handoff,
            occurrence_count: routed.occurrence_count,
            is_dedupe_repeat: routed.is_dedupe_repeat,
            durable_truth_preserved,
            all_routes_preserve_reopen_target,
            screen_reader_announce_required: context.screen_reader_posture().is_active(),
            screen_reader_navigable_surface_present,
            no_generic_home_reopen: reopen_is_truthful(&routed.reopen_target),
            minted_at: routed.minted_at.clone(),
        })
    }
}

/// Surfaces that are durable in-product truth: a held / suppressed attention
/// surface still leaves one of these as a path back.
fn is_durable_truth_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::DurableJobRow
            | FanoutSurfaceClass::StatusItem
            | FanoutSurfaceClass::StatusStrip
            | FanoutSurfaceClass::ActivityCenterDigestCard
            | FanoutSurfaceClass::DigestGroupRow
    )
}

/// True when a resolved route keeps a durable path back to the canonical
/// object: a durable surface that either delivered now or coalesced into the
/// durable truth a prior emission already delivered.
fn route_preserves_durable_path(route: &ResolvedSurfaceRoute) -> bool {
    is_durable_truth_surface(route.fanout_surface_class)
        && matches!(
            route.resolved_receipt_state,
            FanoutReceiptState::Delivered
                | FanoutReceiptState::ReleasedFromHold
                | FanoutReceiptState::DedupedCanonicalEvent
                | FanoutReceiptState::DedupedGroupedBurst
        )
}

/// Surfaces a screen reader can navigate to and read in full.
fn is_navigable_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::DurableJobRow
            | FanoutSurfaceClass::StatusItem
            | FanoutSurfaceClass::ActivityCenterDigestCard
            | FanoutSurfaceClass::DigestGroupRow
    )
}

/// External summary surfaces: OS notification, lock-screen summary, companion.
fn is_external_summary_surface(surface: FanoutSurfaceClass) -> bool {
    matches!(
        surface,
        FanoutSurfaceClass::OsNotification
            | FanoutSurfaceClass::LockScreenSummary
            | FanoutSurfaceClass::CompanionPush
    )
}

/// Resolve one core surface route against the live channel state. Only ever
/// narrows: a delivered surface may be dropped, but a held / suppressed /
/// deduped surface is never promoted back to delivered.
fn resolve_surface(route: &SurfaceRoute, context: &ChannelContext) -> ResolvedSurfaceRoute {
    use crate::attention_router::context::ActiveWindowState;

    let core_state = route.receipt_state;
    let external = is_external_summary_surface(route.fanout_surface_class);

    // Decisions the core already made stand: dedupe, hold, suppress, no-route.
    let (resolved_state, class, stale, suppression_reasons) = match core_state {
        FanoutReceiptState::DedupedCanonicalEvent | FanoutReceiptState::DedupedGroupedBurst => (
            core_state,
            ChannelResolutionClass::DedupedRepeat,
            route.stale_or_undelivered_reason.clone(),
            route.suppression_reasons.clone(),
        ),
        FanoutReceiptState::HeldQuietHours => (
            core_state,
            ChannelResolutionClass::HeldByQuietHoursOrFocus,
            route.stale_or_undelivered_reason.clone(),
            route.suppression_reasons.clone(),
        ),
        FanoutReceiptState::SuppressedPolicy => (
            core_state,
            ChannelResolutionClass::SuppressedByPolicy,
            route.stale_or_undelivered_reason.clone(),
            route.suppression_reasons.clone(),
        ),
        FanoutReceiptState::NotAttemptedNoRoute => (
            core_state,
            ChannelResolutionClass::NoRouteFromCore,
            route.stale_or_undelivered_reason.clone(),
            route.suppression_reasons.clone(),
        ),
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold => {
            // The core delivered; now apply live-channel narrowing.
            match route.fanout_surface_class {
                FanoutSurfaceClass::OsNotification => {
                    if matches!(context.active_window_state(), ActiveWindowState::ForegroundFocused)
                    {
                        (
                            FanoutReceiptState::NotAttemptedNoRoute,
                            ChannelResolutionClass::SuppressedForegroundRedundant,
                            no_route_reason("Foreground window focused; in-product surface used."),
                            Vec::new(),
                        )
                    } else {
                        (
                            core_state,
                            ChannelResolutionClass::DeliveredExternalSummary,
                            route.stale_or_undelivered_reason.clone(),
                            route.suppression_reasons.clone(),
                        )
                    }
                }
                FanoutSurfaceClass::LockScreenSummary => {
                    if matches!(context.active_window_state(), ActiveWindowState::LockedOrAway) {
                        (
                            core_state,
                            ChannelResolutionClass::DeliveredExternalSummary,
                            route.stale_or_undelivered_reason.clone(),
                            route.suppression_reasons.clone(),
                        )
                    } else {
                        (
                            FanoutReceiptState::NotAttemptedNoRoute,
                            ChannelResolutionClass::LockScreenNotApplicable,
                            no_route_reason("Device not locked; lock-screen summary not applicable."),
                            Vec::new(),
                        )
                    }
                }
                FanoutSurfaceClass::CompanionPush => match context.companion_availability() {
                    CompanionAvailability::PairedAvailable => (
                        core_state,
                        ChannelResolutionClass::DeliveredExternalSummary,
                        route.stale_or_undelivered_reason.clone(),
                        route.suppression_reasons.clone(),
                    ),
                    CompanionAvailability::PolicyBlocked => (
                        FanoutReceiptState::SuppressedPolicy,
                        ChannelResolutionClass::CompanionPolicyBlocked,
                        suppressed_reason("Companion fanout blocked by managed policy."),
                        vec![SuppressionReason::AdminPolicySuppression],
                    ),
                    CompanionAvailability::Unpaired | CompanionAvailability::PairedUnavailable => (
                        FanoutReceiptState::NotAttemptedNoRoute,
                        ChannelResolutionClass::CompanionUnavailable,
                        no_route_reason("No reachable companion endpoint."),
                        Vec::new(),
                    ),
                },
                _ => (
                    core_state,
                    ChannelResolutionClass::DeliveredInApp,
                    route.stale_or_undelivered_reason.clone(),
                    route.suppression_reasons.clone(),
                ),
            }
        }
    };

    let visible = matches!(
        resolved_state,
        FanoutReceiptState::Delivered | FanoutReceiptState::ReleasedFromHold
    );

    ResolvedSurfaceRoute {
        fanout_surface_class: route.fanout_surface_class,
        core_receipt_state: core_state,
        resolved_receipt_state: resolved_state,
        channel_resolution_class: class,
        stale_or_undelivered_reason: stale,
        suppression_reasons,
        reopen_target_ref: route.reopen_target_ref.clone(),
        redaction_class: route.redaction_class,
        is_external_summary_surface: external,
        visible,
    }
}

fn derive_companion_handoff(
    routes: &[ResolvedSurfaceRoute],
    context: &ChannelContext,
) -> CompanionHandoffPosture {
    let companion_route = routes
        .iter()
        .find(|route| matches!(route.fanout_surface_class, FanoutSurfaceClass::CompanionPush));

    let handoff_class = match companion_route {
        None => CompanionHandoffClass::NotApplicable,
        Some(route) => match route.channel_resolution_class {
            ChannelResolutionClass::DeliveredExternalSummary => {
                CompanionHandoffClass::SummaryFanoutDelivered
            }
            ChannelResolutionClass::HeldByQuietHoursOrFocus => {
                CompanionHandoffClass::SummaryFanoutHeld
            }
            ChannelResolutionClass::CompanionPolicyBlocked
            | ChannelResolutionClass::SuppressedByPolicy => {
                CompanionHandoffClass::SummaryFanoutPolicyBlocked
            }
            _ => CompanionHandoffClass::SummaryFanoutUnavailable,
        },
    };

    CompanionHandoffPosture {
        handoff_class,
        companion_availability: context.companion_availability(),
        summary_only: true,
        reopen_into_durable_object_required: true,
        privileged_detail_requires_in_product: true,
        forbidden_shortcut_action_classes: forbidden_shortcut_action_classes(),
    }
}

fn no_route_reason(label: &str) -> StaleOrUndeliveredReason {
    StaleOrUndeliveredReason {
        reason_class: StaleOrUndeliveredReasonClass::NoRouteForSurfaceOrTier,
        reason_label: Some(label.to_owned()),
    }
}

fn suppressed_reason(label: &str) -> StaleOrUndeliveredReason {
    StaleOrUndeliveredReason {
        reason_class: StaleOrUndeliveredReasonClass::SuppressedByPolicy,
        reason_label: Some(label.to_owned()),
    }
}

/// A reopen target is truthful when it resolves to an exact identity or
/// honestly announces a placeholder / revalidation requirement. A generic
/// home-screen reopen is not representable in the vocabulary, so this is the
/// proof that the outcome never falls back to one.
fn reopen_is_truthful(reopen_target: &ReopenTarget) -> bool {
    reopen_target.resolves_to_exact_target()
        || matches!(
            reopen_target.reopen_target_kind,
            ReopenTargetKind::PlaceholderAnnounced | ReopenTargetKind::DeniedRequiresRevalidation
        )
}

/// Mirror of the external-payload safe-primary rule: a summary-surface action
/// is safe only when it is non-destructive, open-only, and points at the
/// exact reopen identity.
fn action_is_safe_external_primary(action: &StableAction, reopen_target: &ReopenTarget) -> bool {
    if action.is_destructive {
        return false;
    }
    let matches_exact_target = reopen_target
        .exact_target_identity_ref
        .as_deref()
        .map(|exact| exact == action.target_identity_ref)
        .unwrap_or(false);
    matches_exact_target && command_is_open_only(&action.command_id)
}

fn command_is_open_only(command_id: &str) -> bool {
    command_id.contains(".open")
        || command_id.contains("_open")
        || command_id.contains(".focus")
        || command_id.ends_with(".show")
}

/// The shortcut classes the OS / companion handoff refuses to complete off a
/// summary surface, mirroring the external-payload contract.
fn forbidden_shortcut_action_classes() -> Vec<ForbiddenShortcutActionClass> {
    use ForbiddenShortcutActionClass as Class;
    vec![
        Class::DestructivePublishOrApply,
        Class::SecretOrCredentialReveal,
        Class::IrreversibleHighBlast,
        Class::BypassReviewSheet,
        Class::BypassApprovalWorkflow,
        Class::CrossWorkspaceMutation,
        Class::DirectMutationFromLockScreen,
        Class::DirectMutationFromCompanionPush,
        Class::DirectMutationFromDockOrTaskbar,
        Class::DirectMutationFromSystemTray,
        Class::PolicyOverrideFromOsShortcut,
        Class::TrustStateChangeFromOsShortcut,
        Class::ProviderGrantChangeFromOsShortcut,
    ]
}
