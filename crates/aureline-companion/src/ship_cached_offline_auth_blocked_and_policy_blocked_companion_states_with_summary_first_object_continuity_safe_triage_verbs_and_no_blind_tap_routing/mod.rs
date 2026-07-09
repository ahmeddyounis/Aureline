//! Cached, offline, auth-blocked, policy-blocked, loading, and deleted-object companion states
//! with summary-first object continuity, safe triage verbs, and no-blind-tap routing across
//! claimed M5 notification and handoff surfaces.
//!
//! This module governs the *degraded* states of every reusable companion component frozen in
//! [`crate::freeze_the_m5_companion_component_matrix`]. The prior implement lanes narrowed the
//! six components (the notification row, the mobile review card, the CI-status card, the
//! session-follow tile, the incident-snapshot card, and the desktop-handoff sheet) for the live
//! path. This lane closes the acceptance-criteria gap that remains once the network, the auth
//! posture, a publish policy, or the object itself is no longer there: a user must be able to
//! tell, *before* invoking an action, whether they are looking at live, cached, offline, or
//! blocked companion data, and no row or card may route blindly into a broken or over-privileged
//! path without an explanatory state and a desktop fallback.
//!
//! Every [`CompanionDegradedSurfaceRow`] binds one governed component family, the object it
//! references, a preserved object *summary*, a stable object identity, its client scope, and one
//! controlled availability state. Its data-trust class is *derived* from that availability state
//! rather than asserted, so a cached, offline, or stale surface can never read as live, and its
//! next-safe-action is *derived* too, so the copy that tells the user what to do next is never
//! invented per surface. A surface whose primary path is broken (offline, loading) or
//! over-privileged (auth-blocked, policy-blocked) always carries an explicit desktop fallback and
//! offers a resolvable desktop handoff, so a tap never silently fails or over-reaches. A surface
//! whose object was deleted preserves its last-known summary and stops routing instead of opening
//! a target that no longer exists.
//!
//! The object kinds ([`M5CompanionObjectKind`]), client scopes ([`M5CompanionClientScope`]),
//! freshness classes ([`M5CompanionFreshness`]), handoff targets ([`M5CompanionHandoffTarget`]),
//! component families ([`M5CompanionComponentFamily`]), degraded reasons
//! ([`M5CompanionDegradedReason`]), required labels ([`M5CompanionRequiredLabel`]), surface
//! families ([`M5CompanionSurfaceFamily`]), deployment lines ([`M5CompanionDeploymentLine`]),
//! consumer surfaces ([`M5CompanionConsumerSurface`]), accessibility routes
//! ([`M5CompanionAccessibilityRoute`]), and downgrade triggers ([`M5CompanionDowngradeTrigger`])
//! are reused directly from the frozen matrix, so this lane never invents a parallel companion
//! vocabulary. It mints new vocabulary only for what the matrix left implicit about degraded
//! states: the controlled availability state, the derived data-trust class, the derived
//! next-safe-action, and the keyboard-complete safe triage verbs that survive a degraded state.
//!
//! Raw object payloads, log bodies, secret values, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-companion-degraded-state-continuity-controls.schema.json`](../../../../schemas/ui/m5-companion-degraded-state-continuity-controls.schema.json).
//! The contract doc is
//! [`docs/companion/ship_cached_offline_auth_blocked_and_policy_blocked_companion_states.md`](../../../../docs/companion/ship_cached_offline_auth_blocked_and_policy_blocked_companion_states.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_companion_degraded_state_continuity_controls,
    seeded_companion_degraded_state_continuity_controls_handoff_surface_deleted_object,
    seeded_companion_degraded_state_continuity_controls_notification_surface_blocked,
    COMPANION_DEGRADED_STATE_CONTINUITY_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The object kind, client scope, freshness, handoff target, component family, degraded reason,
// required labels, surface family, deployment line, consumer surface, accessibility route, and
// downgrade triggers are frozen once, in the companion component matrix. This lane reuses them
// verbatim so it never invents a parallel companion vocabulary.
use crate::freeze_the_m5_companion_component_matrix::{
    M5CompanionAccessibilityRoute, M5CompanionClientScope, M5CompanionComponentFamily,
    M5CompanionConsumerSurface, M5CompanionDegradedReason, M5CompanionDeploymentLine,
    M5CompanionDowngradeTrigger, M5CompanionFreshness, M5CompanionHandoffTarget,
    M5CompanionObjectKind, M5CompanionRequiredLabel, M5CompanionSurfaceFamily,
    M5_COMPANION_COMPONENT_DOC_REF, M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF,
    M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
    M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF, M5_COMPANION_COMPONENT_SCHEMA_REF,
    M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF, M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`CompanionDegradedStateContinuityPacket`].
pub const COMPANION_DEGRADED_STATE_CONTINUITY_RECORD_KIND: &str =
    "companion_degraded_state_continuity_controls";

/// Schema version for companion degraded-state continuity control records.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_REF: &str =
    "schemas/ui/m5-companion-degraded-state-continuity-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_DOC_REF: &str =
    "docs/companion/ship_cached_offline_auth_blocked_and_policy_blocked_companion_states.md";

/// Repo-relative path of the protected fixture directory.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-companion-degraded-state-continuity-controls";

/// Repo-relative path of the checked support-export artifact.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-companion-degraded-state-continuity-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/release/m5-companion-degraded-state-continuity-proof/summary.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const COMPANION_DEGRADED_STATE_CONTINUITY_CSV_REF: &str =
    "artifacts/release/m5-companion-degraded-state-continuity-proof/matrix.csv";

// ---- degraded-state vocabulary ------------------------------------------

/// Controlled availability state a companion surface can be in. These are the exact
/// acceptance-criteria states this lane governs: `live`, `cached`, `offline`, `auth-blocked`,
/// `policy-blocked`, `loading`, and `deleted-object`. No companion surface invents a parallel
/// word for any of these states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionAvailabilityState {
    /// Streaming live from the local core; full companion capability.
    Live,
    /// Showing a last-known cached value, not a live one.
    Cached,
    /// The relay is unreachable; the surface is held offline pending reconnection.
    Offline,
    /// The desktop session must re-authenticate before this action can complete.
    AuthBlocked,
    /// A publish or write path is no longer allowed by policy from the companion.
    PolicyBlocked,
    /// The surface is still resolving; detail is not available yet.
    Loading,
    /// The referenced object was deleted; only its last-known summary remains.
    DeletedObject,
}

impl CompanionAvailabilityState {
    /// Every availability state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Live,
        Self::Cached,
        Self::Offline,
        Self::AuthBlocked,
        Self::PolicyBlocked,
        Self::Loading,
        Self::DeletedObject,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Offline => "offline",
            Self::AuthBlocked => "auth_blocked",
            Self::PolicyBlocked => "policy_blocked",
            Self::Loading => "loading",
            Self::DeletedObject => "deleted_object",
        }
    }
}

/// Derived data-trust class a companion surface may present.
///
/// This is the degraded-state honesty axis: the class is derived from the availability state,
/// never asserted, so a cached, offline, or stale surface can never read as live. The two
/// blocked states collapse into one `Blocked` class because both route the same way — to a
/// desktop fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionDataTrustClass {
    /// Live and trustable as current.
    LiveTrusted,
    /// A cached value with reduced trust; refresh for the latest.
    CachedReduced,
    /// A last-known value held offline; stale until reconnection.
    OfflineStale,
    /// Blocked by auth or policy; the primary path is over-privileged from the companion.
    Blocked,
    /// Still loading; detail is not available yet.
    Loading,
    /// The object is gone; only its last-known summary remains.
    Gone,
}

impl CompanionDataTrustClass {
    /// Every data-trust class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveTrusted,
        Self::CachedReduced,
        Self::OfflineStale,
        Self::Blocked,
        Self::Loading,
        Self::Gone,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrusted => "live_trusted",
            Self::CachedReduced => "cached_reduced",
            Self::OfflineStale => "offline_stale",
            Self::Blocked => "blocked",
            Self::Loading => "loading",
            Self::Gone => "gone",
        }
    }
}

/// Derived next-safe-action a companion surface names before an action, so the copy that tells
/// the user what to do next is never invented per surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionNextSafeAction {
    /// Proceed in the companion — the surface is live.
    ProceedInCompanion,
    /// Refresh for the latest — the surface is cached.
    RefreshForLatest,
    /// Retry when back online — the surface is offline.
    RetryWhenOnline,
    /// Re-authenticate on desktop first — the surface is auth-blocked.
    ReauthOnDesktop,
    /// Open read-only on desktop — the publish path is policy-blocked.
    OpenOnDesktopReadOnly,
    /// Wait for the surface to finish loading.
    WaitForLoad,
    /// View the last-known summary only — the object was deleted.
    ViewCachedSummaryOnly,
}

impl CompanionNextSafeAction {
    /// Every next-safe-action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProceedInCompanion,
        Self::RefreshForLatest,
        Self::RetryWhenOnline,
        Self::ReauthOnDesktop,
        Self::OpenOnDesktopReadOnly,
        Self::WaitForLoad,
        Self::ViewCachedSummaryOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedInCompanion => "proceed_in_companion",
            Self::RefreshForLatest => "refresh_for_latest",
            Self::RetryWhenOnline => "retry_when_online",
            Self::ReauthOnDesktop => "reauth_on_desktop",
            Self::OpenOnDesktopReadOnly => "open_on_desktop_read_only",
            Self::WaitForLoad => "wait_for_load",
            Self::ViewCachedSummaryOnly => "view_cached_summary_only",
        }
    }
}

/// One keyboard-complete safe triage verb a companion surface preserves even in a degraded
/// state, so a degraded surface never hides its action affordance and never routes blindly into
/// a broken or over-privileged path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompanionSafeVerb {
    /// Open the exact object this surface references (lands on its stable object landing ref).
    Open,
    /// View the last-known summary without fetching full detail.
    ViewSummary,
    /// Refresh the surface to fetch the latest.
    Refresh,
    /// Hand off to the exact desktop target — the degraded-path fallback.
    HandoffToDesktop,
    /// Copy the stable object reference.
    CopyReference,
    /// Dismiss the surface.
    Dismiss,
}

impl CompanionSafeVerb {
    /// Every safe verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::ViewSummary,
        Self::Refresh,
        Self::HandoffToDesktop,
        Self::CopyReference,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete surface must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::ViewSummary => "view_summary",
            Self::Refresh => "refresh",
            Self::HandoffToDesktop => "handoff_to_desktop",
            Self::CopyReference => "copy_reference",
            Self::Dismiss => "dismiss",
        }
    }

    /// Whether this verb hands off to the desktop.
    fn is_handoff_verb(self) -> bool {
        matches!(self, Self::HandoffToDesktop)
    }
}

/// Disclosures a companion surface must carry, derived from its availability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AvailabilityDisclosure {
    /// The derived data-trust class this surface may present.
    pub trust_class: CompanionDataTrustClass,
    /// The derived next-safe-action this surface names.
    pub next_safe_action: CompanionNextSafeAction,
    /// Whether the surface may claim live, current data.
    pub is_live: bool,
    /// Whether the surface must carry an explicit degraded-state explanation.
    pub needs_state_explanation: bool,
    /// Whether the primary path is broken or over-privileged and needs a desktop fallback.
    pub needs_desktop_fallback: bool,
    /// Whether the referenced object is gone (deleted) and must stop routing.
    pub is_gone: bool,
}

/// Resolves the degraded-state truth a companion surface may present.
///
/// A live surface is trusted and proceeds in the companion. A cached surface has reduced trust
/// and refreshes for the latest. An offline surface is stale and retries when online. An
/// auth-blocked or policy-blocked surface is blocked — the primary path is over-privileged from
/// the companion, so it routes to a desktop fallback. A loading surface waits. A deleted-object
/// surface is gone: it preserves its last-known summary and stops routing rather than opening a
/// target that no longer exists.
pub fn resolve_availability(state: CompanionAvailabilityState) -> AvailabilityDisclosure {
    use CompanionAvailabilityState as State;
    use CompanionDataTrustClass as Trust;
    use CompanionNextSafeAction as Next;

    let trust_class = match state {
        State::Live => Trust::LiveTrusted,
        State::Cached => Trust::CachedReduced,
        State::Offline => Trust::OfflineStale,
        State::AuthBlocked | State::PolicyBlocked => Trust::Blocked,
        State::Loading => Trust::Loading,
        State::DeletedObject => Trust::Gone,
    };

    let next_safe_action = match state {
        State::Live => Next::ProceedInCompanion,
        State::Cached => Next::RefreshForLatest,
        State::Offline => Next::RetryWhenOnline,
        State::AuthBlocked => Next::ReauthOnDesktop,
        State::PolicyBlocked => Next::OpenOnDesktopReadOnly,
        State::Loading => Next::WaitForLoad,
        State::DeletedObject => Next::ViewCachedSummaryOnly,
    };

    let is_live = matches!(trust_class, Trust::LiveTrusted);
    let is_gone = matches!(trust_class, Trust::Gone);
    // A broken (offline, loading) or over-privileged (blocked) primary path must carry a desktop
    // fallback. A live or cached surface can proceed in the companion; a gone object has nothing
    // to open, so it stops routing instead.
    let needs_desktop_fallback = matches!(
        trust_class,
        Trust::OfflineStale | Trust::Blocked | Trust::Loading
    );

    AvailabilityDisclosure {
        trust_class,
        next_safe_action,
        is_live,
        needs_state_explanation: !is_live,
        needs_desktop_fallback,
        is_gone,
    }
}

/// A companion surface in one governed availability state, preserving its object summary, stable
/// identity, freshness, next-safe-action, safe triage verbs, and — where its path is broken or
/// over-privileged — an explicit desktop fallback.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDegradedSurfaceRow {
    /// Governed component family this surface belongs to, reused from the frozen matrix.
    pub component: M5CompanionComponentFamily,
    /// Stable surface id.
    pub surface_id: String,
    /// Human-readable surface title; required and non-empty.
    pub surface_title: String,
    /// Object kind this surface references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Preserved object summary — the last-known summary shown even when full detail cannot be
    /// fetched, so object continuity survives a degraded state. Required and non-empty.
    pub object_summary_note: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a generic
    /// activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Stable object identity, preserved across degraded states; required and non-empty.
    pub stable_object_ref: String,
    /// Client scope this surface is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Governed availability state.
    pub availability_state: CompanionAvailabilityState,
    /// Derived data-trust class (must equal the resolved class).
    pub trust_class: CompanionDataTrustClass,
    /// Whether the surface claims live, current data (must equal the derived truth).
    pub claims_live_data: bool,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Degraded-state explanation; required whenever the surface is not live, so a degraded state
    /// is always explicit before an action.
    pub state_explanation_note: String,
    /// Derived next-safe-action (must equal the resolved action).
    pub next_safe_action: CompanionNextSafeAction,
    /// Next-safe-action copy; always required so the user always knows what to do next.
    pub next_safe_action_note: String,
    /// Desktop-fallback note; required whenever the primary path is broken or over-privileged.
    pub desktop_fallback_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete safe triage verbs (must include the mandatory `Open`).
    pub safe_verbs: Vec<CompanionSafeVerb>,
    /// Degraded reasons this surface can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this surface can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this surface.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this surface keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this surface offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this surface's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this surface.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never masks client scope or freshness. MUST be `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: never hides its companion-versus-desktop capability boundary.
    /// MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: never invents an alternate label for a governed state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: never implies a desktop-required action is companion-safe. MUST be
    /// `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
    /// Hard invariant: never routes blindly into a broken or over-privileged path without an
    /// explanatory state and desktop fallback. MUST be `false`.
    pub routes_blindly_into_broken_or_overprivileged_path: bool,
}

impl CompanionDegradedSurfaceRow {
    /// Degraded-state disclosures this surface must carry, derived from its availability state.
    pub fn availability_disclosure(&self) -> AvailabilityDisclosure {
        resolve_availability(self.availability_state)
    }

    /// Whether the surface offers every mandatory keyboard-complete verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<CompanionSafeVerb> = self.safe_verbs.iter().copied().collect();
        CompanionSafeVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the surface declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the surface offers a desktop-handoff verb.
    fn offers_handoff(&self) -> bool {
        self.safe_verbs.iter().any(|verb| verb.is_handoff_verb())
    }

    /// Whether the surface names a resolvable desktop handoff (verb plus a non-`no_handoff`
    /// target).
    fn offers_resolvable_handoff(&self) -> bool {
        self.offers_handoff() && self.handoff_target != M5CompanionHandoffTarget::NoHandoff
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance trust review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDegradedStateGlanceReview {
    /// Every surface names its object summary and stable identity.
    pub every_surface_names_object_summary_and_identity: bool,
    /// Every surface states its freshness.
    pub every_surface_states_its_freshness: bool,
    /// Every surface states its next-safe-action before an action.
    pub every_surface_states_next_safe_action: bool,
    /// The degraded state is explicit before an action.
    pub degraded_state_is_explicit_before_action: bool,
    /// Live, cached, offline, and blocked data are distinguishable before a tap.
    pub live_cached_offline_blocked_distinguishable: bool,
    /// A cached or stale surface is never shown as live.
    pub cached_or_stale_never_shown_as_live: bool,
    /// The data-trust class is derived from the availability state, never asserted.
    pub trust_class_derived_never_asserted: bool,
    /// Safe triage verbs are preserved when full detail cannot be fetched.
    pub safe_triage_verbs_preserved_when_detail_unavailable: bool,
    /// A blocked publish path routes to a desktop fallback rather than failing.
    pub blocked_publish_path_routes_to_desktop: bool,
    /// No surface routes blindly into a broken or over-privileged path.
    pub no_surface_routes_blindly_into_broken_or_overprivileged_path: bool,
    /// A deleted object preserves its summary and stops routing.
    pub deleted_object_preserves_summary_and_stops_routing: bool,
    /// Every broken or over-privileged state names a desktop fallback.
    pub every_broken_or_overprivileged_state_names_desktop_fallback: bool,
    /// The object identity is always explicit.
    pub object_identity_always_explicit: bool,
    /// The client scope is always explicit.
    pub client_scope_always_explicit: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl CompanionDegradedStateGlanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.every_surface_names_object_summary_and_identity
            && self.every_surface_states_its_freshness
            && self.every_surface_states_next_safe_action
            && self.degraded_state_is_explicit_before_action
            && self.live_cached_offline_blocked_distinguishable
            && self.cached_or_stale_never_shown_as_live
            && self.trust_class_derived_never_asserted
            && self.safe_triage_verbs_preserved_when_detail_unavailable
            && self.blocked_publish_path_routes_to_desktop
            && self.no_surface_routes_blindly_into_broken_or_overprivileged_path
            && self.deleted_object_preserves_summary_and_stops_routing
            && self.every_broken_or_overprivileged_state_names_desktop_fallback
            && self.object_identity_always_explicit
            && self.client_scope_always_explicit
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDegradedStateConsumerProjection {
    /// The notification / triage UIs read a single canonical source.
    pub notification_surfaces_read_single_source: bool,
    /// The desktop-handoff UI reads a single canonical source.
    pub handoff_surfaces_read_single_source: bool,
    /// The first glance names state, scope, and freshness without drilling in.
    pub first_glance_names_state_scope_and_freshness: bool,
    /// The next-safe-action is visible before a tap.
    pub next_safe_action_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl CompanionDegradedStateConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.notification_surfaces_read_single_source
            && self.handoff_surfaces_read_single_source
            && self.first_glance_names_state_scope_and_freshness
            && self.next_safe_action_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDegradedStateProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`CompanionDegradedStateContinuityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompanionDegradedStateContinuityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Degraded companion surfaces.
    pub surfaces: Vec<CompanionDegradedSurfaceRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: CompanionDegradedStateGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionDegradedStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionDegradedStateProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe companion degraded-state continuity controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompanionDegradedStateContinuityPacket {
    /// Record kind; must equal [`COMPANION_DEGRADED_STATE_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Degraded companion surfaces.
    pub surfaces: Vec<CompanionDegradedSurfaceRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: CompanionDegradedStateGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: CompanionDegradedStateConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CompanionDegradedStateProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CompanionDegradedStateContinuityPacket {
    /// Builds a companion degraded-state continuity controls packet from stable-lane input.
    pub fn new(input: CompanionDegradedStateContinuityPacketInput) -> Self {
        Self {
            record_kind: COMPANION_DEGRADED_STATE_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            surfaces: input.surfaces,
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

    /// Validates the companion degraded-state continuity control invariants.
    pub fn validate(&self) -> Vec<CompanionDegradedStateContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != COMPANION_DEGRADED_STATE_CONTINUITY_RECORD_KIND {
            violations.push(CompanionDegradedStateContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_VERSION {
            violations.push(CompanionDegradedStateContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CompanionDegradedStateContinuityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_surfaces(self, &mut violations);

        if !self.glance_review.all_hold() {
            violations.push(CompanionDegradedStateContinuityViolation::GlanceReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(CompanionDegradedStateContinuityViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(CompanionDegradedStateContinuityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("companion degraded-state continuity packet serializes"),
        ) {
            violations.push(CompanionDegradedStateContinuityViolation::RawBoundaryMaterialInExport);
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
            .expect("companion degraded-state continuity packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface,id,component,object_kind,client_scope,availability_state,trust_class,claims_live,next_safe_action,handoff_target\n",
        );
        for surface in &self.surfaces {
            let disclosure = surface.availability_disclosure();
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                "companion_degraded_surface",
                csv_field(&surface.surface_id),
                surface.component.as_str(),
                surface.object_kind.as_str(),
                surface.client_scope.as_str(),
                surface.availability_state.as_str(),
                disclosure.trust_class.as_str(),
                disclosure.is_live,
                disclosure.next_safe_action.as_str(),
                surface.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .surfaces
            .iter()
            .filter(|surface| !surface.availability_disclosure().is_live)
            .count();
        let needs_fallback = self
            .surfaces
            .iter()
            .filter(|surface| surface.availability_disclosure().needs_desktop_fallback)
            .count();

        let mut out = String::new();
        out.push_str("# Companion degraded-state continuity controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Degraded surfaces: {} ({} not live, {} needing a desktop fallback)\n",
            self.surfaces.len(),
            not_live,
            needs_fallback
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Degraded surfaces\n\n");
        for surface in &self.surfaces {
            let disclosure = surface.availability_disclosure();
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, state `{}` → trust `{}`, freshness `{}`, next `{}`, handoff `{}`\n",
                surface.surface_title,
                surface.component.as_str(),
                surface.client_scope.as_str(),
                surface.availability_state.as_str(),
                disclosure.trust_class.as_str(),
                surface.freshness.as_str(),
                disclosure.next_safe_action.as_str(),
                surface.handoff_target.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in companion degraded-state continuity export.
#[derive(Debug)]
pub enum CompanionDegradedStateContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CompanionDegradedStateContinuityViolation>),
}

impl fmt::Display for CompanionDegradedStateContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "companion degraded-state continuity export parse failed: {error}"
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
                    "companion degraded-state continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CompanionDegradedStateContinuityArtifactError {}

/// Validation failures emitted by [`CompanionDegradedStateContinuityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompanionDegradedStateContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No degraded surfaces are present.
    SurfacesMissing,
    /// A degraded surface is incomplete.
    SurfaceIncomplete,
    /// A surface does not name its exact object landing reference.
    ObjectLandingRefMissing,
    /// A surface does not name its stable object identity.
    StableObjectRefMissing,
    /// A surface does not preserve its object summary.
    ObjectSummaryMissing,
    /// A surface misrepresents its derived availability state.
    AvailabilityStateMisrepresented,
    /// A degraded surface does not name its degraded-state explanation.
    StateExplanationMissing,
    /// A surface misrepresents its derived next-safe-action.
    NextSafeActionMisrepresented,
    /// A surface does not name its next-safe-action copy.
    NextSafeActionNoteMissing,
    /// A broken or over-privileged surface does not name its desktop fallback.
    DesktopFallbackMissing,
    /// A broken or over-privileged surface does not offer a resolvable desktop handoff.
    BlindHandoffRouteMissing,
    /// A deleted-object surface still routes to a live handoff target.
    GoneObjectStillRoutes,
    /// A surface routes blindly into a broken or over-privileged path.
    RoutesBlindlyIntoBrokenOrOverprivilegedPath,
    /// A surface omits the mandatory `Open` verb.
    SafeVerbsIncomplete,
    /// The surfaces do not cover every availability state.
    AvailabilityStateCoverageMissing,
    /// The surfaces do not cover every component family.
    ComponentFamilyCoverageMissing,
    /// A surface does not name its scope / freshness.
    ScopeAndFreshnessNoteMissing,
    /// A surface does not name its scope label.
    ScopeLabelMissing,
    /// A surface does not name its handoff label.
    HandoffLabelMissing,
    /// A surface does not declare its degraded reasons.
    DegradedReasonsMissing,
    /// A surface does not declare its mandatory labels.
    RequiredLabelsIncomplete,
    /// A surface does not declare an accessibility route (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface masks its client scope or freshness.
    ScopeOrFreshnessMasked,
    /// A surface hides its companion-versus-desktop capability boundary.
    CapabilityBoundaryHidden,
    /// A surface invents an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A surface implies a desktop-required action is companion-safe.
    DesktopActionImpliedCompanionSafe,
    /// A surface routes to a generic activity page instead of one stable object.
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

impl CompanionDegradedStateContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::SurfacesMissing => "surfaces_missing",
            Self::SurfaceIncomplete => "surface_incomplete",
            Self::ObjectLandingRefMissing => "object_landing_ref_missing",
            Self::StableObjectRefMissing => "stable_object_ref_missing",
            Self::ObjectSummaryMissing => "object_summary_missing",
            Self::AvailabilityStateMisrepresented => "availability_state_misrepresented",
            Self::StateExplanationMissing => "state_explanation_missing",
            Self::NextSafeActionMisrepresented => "next_safe_action_misrepresented",
            Self::NextSafeActionNoteMissing => "next_safe_action_note_missing",
            Self::DesktopFallbackMissing => "desktop_fallback_missing",
            Self::BlindHandoffRouteMissing => "blind_handoff_route_missing",
            Self::GoneObjectStillRoutes => "gone_object_still_routes",
            Self::RoutesBlindlyIntoBrokenOrOverprivilegedPath => {
                "routes_blindly_into_broken_or_overprivileged_path"
            }
            Self::SafeVerbsIncomplete => "safe_verbs_incomplete",
            Self::AvailabilityStateCoverageMissing => "availability_state_coverage_missing",
            Self::ComponentFamilyCoverageMissing => "component_family_coverage_missing",
            Self::ScopeAndFreshnessNoteMissing => "scope_and_freshness_note_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::HandoffLabelMissing => "handoff_label_missing",
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

/// Reads and validates the checked-in companion degraded-state continuity export.
pub fn current_companion_degraded_state_continuity_export(
) -> Result<CompanionDegradedStateContinuityPacket, CompanionDegradedStateContinuityArtifactError> {
    let packet: CompanionDegradedStateContinuityPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-companion-degraded-state-continuity-proof/support_export.json"
    )))
        .map_err(CompanionDegradedStateContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CompanionDegradedStateContinuityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CompanionDegradedStateContinuityPacket,
    violations: &mut Vec<CompanionDegradedStateContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        COMPANION_DEGRADED_STATE_CONTINUITY_SCHEMA_REF,
        COMPANION_DEGRADED_STATE_CONTINUITY_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CompanionDegradedStateContinuityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_surfaces(
    packet: &CompanionDegradedStateContinuityPacket,
    violations: &mut Vec<CompanionDegradedStateContinuityViolation>,
) {
    if packet.surfaces.is_empty() {
        violations.push(CompanionDegradedStateContinuityViolation::SurfacesMissing);
        return;
    }

    let mut availability_states: BTreeSet<CompanionAvailabilityState> = BTreeSet::new();
    let mut component_families: BTreeSet<M5CompanionComponentFamily> = BTreeSet::new();

    for surface in &packet.surfaces {
        let disclosure = surface.availability_disclosure();
        availability_states.insert(surface.availability_state);
        component_families.insert(surface.component);

        if surface.surface_id.trim().is_empty()
            || surface.surface_title.trim().is_empty()
            || surface.object_label.trim().is_empty()
            || surface.fields_shown.is_empty()
            || surface.surface_families.is_empty()
            || surface.deployment_lines.is_empty()
            || surface.consumer_surfaces.is_empty()
            || surface.source_contract_refs.is_empty()
        {
            violations.push(CompanionDegradedStateContinuityViolation::SurfaceIncomplete);
        }
        if surface.object_landing_ref.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::ObjectLandingRefMissing);
        }
        if surface.stable_object_ref.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::StableObjectRefMissing);
        }
        if surface.object_summary_note.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::ObjectSummaryMissing);
        }
        if surface.trust_class != disclosure.trust_class
            || surface.claims_live_data != disclosure.is_live
        {
            violations
                .push(CompanionDegradedStateContinuityViolation::AvailabilityStateMisrepresented);
        }
        if disclosure.needs_state_explanation && surface.state_explanation_note.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::StateExplanationMissing);
        }
        if surface.next_safe_action != disclosure.next_safe_action {
            violations
                .push(CompanionDegradedStateContinuityViolation::NextSafeActionMisrepresented);
        }
        if surface.next_safe_action_note.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::NextSafeActionNoteMissing);
        }
        if disclosure.needs_desktop_fallback {
            if surface.desktop_fallback_note.trim().is_empty() {
                violations.push(CompanionDegradedStateContinuityViolation::DesktopFallbackMissing);
            }
            if !surface.offers_resolvable_handoff() {
                violations
                    .push(CompanionDegradedStateContinuityViolation::BlindHandoffRouteMissing);
            }
        }
        if disclosure.is_gone && surface.offers_resolvable_handoff() {
            violations.push(CompanionDegradedStateContinuityViolation::GoneObjectStillRoutes);
        }
        if surface.routes_blindly_into_broken_or_overprivileged_path {
            violations.push(
                CompanionDegradedStateContinuityViolation::RoutesBlindlyIntoBrokenOrOverprivilegedPath,
            );
        }
        if surface.scope_and_freshness_note.trim().is_empty() {
            violations
                .push(CompanionDegradedStateContinuityViolation::ScopeAndFreshnessNoteMissing);
        }
        if surface.scope_label.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::ScopeLabelMissing);
        }
        if !surface.declares_mandatory_verbs() {
            violations.push(CompanionDegradedStateContinuityViolation::SafeVerbsIncomplete);
        }
        if surface.handoff_label.trim().is_empty() {
            violations.push(CompanionDegradedStateContinuityViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &surface.degraded_reasons,
            surface.declares_mandatory_labels(),
            &surface.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: surface.masks_scope_or_freshness,
                hides_capability_boundary: surface.hides_capability_boundary,
                invents_alternate_state_label: surface.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: surface
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: surface.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in CompanionAvailabilityState::ALL {
        if !availability_states.contains(&required) {
            violations
                .push(CompanionDegradedStateContinuityViolation::AvailabilityStateCoverageMissing);
            break;
        }
    }
    for required in M5CompanionComponentFamily::ALL {
        if !component_families.contains(&required) {
            violations
                .push(CompanionDegradedStateContinuityViolation::ComponentFamilyCoverageMissing);
            break;
        }
    }
}

/// The five hard-invariant bools every control shares with the frozen matrix.
struct ControlInvariants {
    masks_scope_or_freshness: bool,
    hides_capability_boundary: bool,
    invents_alternate_state_label: bool,
    implies_desktop_action_is_companion_safe: bool,
    routes_to_generic_activity_page: bool,
}

/// Validates the axes shared by every surface.
fn validate_common_control(
    degraded_reasons: &[M5CompanionDegradedReason],
    declares_mandatory_labels: bool,
    accessibility_routes: &[M5CompanionAccessibilityRoute],
    invariants: ControlInvariants,
    violations: &mut Vec<CompanionDegradedStateContinuityViolation>,
) {
    if degraded_reasons.is_empty() {
        violations.push(CompanionDegradedStateContinuityViolation::DegradedReasonsMissing);
    }
    if !declares_mandatory_labels {
        violations.push(CompanionDegradedStateContinuityViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5CompanionAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(CompanionDegradedStateContinuityViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_scope_or_freshness {
        violations.push(CompanionDegradedStateContinuityViolation::ScopeOrFreshnessMasked);
    }
    if invariants.hides_capability_boundary {
        violations.push(CompanionDegradedStateContinuityViolation::CapabilityBoundaryHidden);
    }
    if invariants.invents_alternate_state_label {
        violations.push(CompanionDegradedStateContinuityViolation::AlternateStateLabelInvented);
    }
    if invariants.implies_desktop_action_is_companion_safe {
        violations
            .push(CompanionDegradedStateContinuityViolation::DesktopActionImpliedCompanionSafe);
    }
    if invariants.routes_to_generic_activity_page {
        violations.push(CompanionDegradedStateContinuityViolation::RoutesToGenericActivityPage);
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
/// raw-*value* shapes that must never cross the boundary: a password / passphrase literal, a
/// bearer literal, a URL scheme, or a PEM header.
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
