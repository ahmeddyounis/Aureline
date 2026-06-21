//! Cross-client follow-state truth for presentation / walkthrough sessions.
//!
//! A presentation can be observed from more than one claimed M5 client at once —
//! the desktop shell, the browser surface, and the companion app. This module is
//! the layer the spec calls for: it makes each client's follow posture an
//! **explicit, attributable state** drawn from one shared vocabulary, rather than
//! something the client infers from viewport drift, connection timing, or a
//! transient toast.
//!
//! The vocabulary is [`FollowMode`]: follow live, break away, request follow,
//! request take over, and — crucially — *cached snapshot*. Every mode resolves to
//! a [`LivenessClass`] (live / independent / cached snapshot) so a viewer can
//! always tell whether they are watching the presenter live, browsing on their
//! own, or looking at a stale picture. A cached snapshot carries a
//! [`SnapshotIdentity`] that labels itself as a snapshot and never claims to be a
//! live shared route — that is the guardrail this row exists to hold.
//!
//! [`project_follow_state_truth`] turns a [`PresentationSession`] and a set of
//! [`ClientFollowInput`]s into a [`FollowStateTruth`] packet: one
//! [`ClientFollowView`] per client, each carrying the same recovery-action
//! vocabulary, a durable breakaway banner when broken away, and a snapshot
//! identity when cached. [`FollowStateTruth::validate`] re-derives the
//! invariants so a hand-edited fixture cannot quietly claim live state for a
//! cached view or diverge the vocabulary between clients.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::presentation_mode::{
    LeaderFollowState, PresentationSession, PRESENTATION_MODE_BETA_SCHEMA_VERSION,
    PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF,
};

/// Stable record kind for [`FollowStateTruth`] payloads.
pub const FOLLOW_STATE_TRUTH_RECORD_KIND: &str = "presentation_follow_state_truth_record";

/// Stable record kind for [`ClientFollowView`] payloads.
pub const CLIENT_FOLLOW_VIEW_RECORD_KIND: &str = "presentation_client_follow_view_record";

/// Stable record kind for [`FollowStateSupportExport`] payloads.
pub const FOLLOW_STATE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "presentation_follow_state_support_export_record";

/// Stable record kind for [`FollowStateSupportExportRow`] payloads.
pub const FOLLOW_STATE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "presentation_follow_state_support_export_row_record";

/// The human-readable follow / breakaway contract this module implements.
pub const PRESENTATION_FOLLOW_AND_BREAKAWAY_DOC_REF: &str =
    "docs/ux/presentation-follow-and-breakaway.md";

/// The cross-client follow matrix this module's corpus backs.
pub const CROSS_CLIENT_FOLLOW_MATRIX_REF: &str =
    "artifacts/presentation/cross-client-follow-matrix.md";

/// Directory holding the checked-in browser/companion follow fixtures.
pub const PRESENTATION_FOLLOW_FIXTURE_DIR: &str =
    "fixtures/presentation/browser-and-companion-follow";

/// A claimed M5 client surface that can observe a presentation.
///
/// The same follow-state vocabulary and recovery actions apply on every one of
/// these; the only thing that differs is which device the viewer is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientSurface {
    /// The desktop shell.
    Desktop,
    /// The browser surface.
    Browser,
    /// The companion app.
    Companion,
}

impl ClientSurface {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Browser => "browser",
            Self::Companion => "companion",
        }
    }
}

/// The explicit, cross-client follow-state vocabulary.
///
/// These are distinct, attributable states — never inferred from cursor motion,
/// connection timing, or a vanished toast. `RequestingTakeOver` and
/// `CachedSnapshot` are runtime client postures that the persisted
/// [`PresentationSession`] does not carry; the rest map one-to-one from
/// [`LeaderFollowState`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowMode {
    /// This client is the presenter driving the walkthrough.
    Presenting,
    /// Following the presenter's live anchor; navigation tracks the leader.
    FollowingLive,
    /// Browsing independently while the presenter's anchor stays reachable.
    BrokenAway,
    /// Asked to (re)join the presenter; not yet synced back to the live route.
    RequestingFollow,
    /// Asked to take over as presenter; still seeing the live route meanwhile.
    RequestingTakeOver,
    /// Showing a cached / stale picture, not the presenter's live route.
    CachedSnapshot,
}

impl FollowMode {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Presenting => "presenting",
            Self::FollowingLive => "following_live",
            Self::BrokenAway => "broken_away",
            Self::RequestingFollow => "requesting_follow",
            Self::RequestingTakeOver => "requesting_take_over",
            Self::CachedSnapshot => "cached_snapshot",
        }
    }

    /// The liveness class a viewer reads off this mode.
    pub const fn liveness(self) -> LivenessClass {
        match self {
            Self::Presenting | Self::FollowingLive | Self::RequestingTakeOver => {
                LivenessClass::Live
            }
            Self::BrokenAway | Self::RequestingFollow => LivenessClass::Independent,
            Self::CachedSnapshot => LivenessClass::CachedSnapshot,
        }
    }

    /// True when a durable breakaway banner must accompany this mode.
    pub const fn shows_breakaway_banner(self) -> bool {
        matches!(self, Self::BrokenAway)
    }

    /// True when this mode shows a cached snapshot rather than a live route.
    pub const fn is_cached_snapshot(self) -> bool {
        matches!(self, Self::CachedSnapshot)
    }

    /// The cross-client follow vocabulary's basic states, projected from the
    /// persisted leader/follow posture. Take-over and cached-snapshot are
    /// runtime-only and have no session-level source.
    pub const fn from_leader_follow_state(state: LeaderFollowState) -> Self {
        match state {
            LeaderFollowState::Presenting => Self::Presenting,
            LeaderFollowState::FollowingPresenter => Self::FollowingLive,
            LeaderFollowState::BrokenAway => Self::BrokenAway,
            LeaderFollowState::RequestingFollow => Self::RequestingFollow,
        }
    }

    /// The recovery-action kinds offered from this mode, in display order. Every
    /// client offers exactly this set for a given mode, which is what keeps the
    /// recovery vocabulary identical across desktop, browser, and companion.
    pub fn recovery_action_kinds(self) -> Vec<RecoveryKind> {
        match self {
            // The presenter and a live follower are already on the live route.
            Self::Presenting | Self::FollowingLive => Vec::new(),
            // Independent or pending-rejoin views can jump back to the presenter.
            Self::BrokenAway | Self::RequestingFollow | Self::RequestingTakeOver => {
                vec![RecoveryKind::ReturnToPresenter]
            }
            // A cached snapshot can refresh to live or jump to the presenter.
            Self::CachedSnapshot => {
                vec![
                    RecoveryKind::RefreshLiveRoute,
                    RecoveryKind::ReturnToPresenter,
                ]
            }
        }
    }
}

/// How live the picture a client is showing actually is.
///
/// This is the single honesty axis the spec turns on: a viewer must always be
/// able to tell which of these three they are looking at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LivenessClass {
    /// Watching the presenter's live route in real time.
    Live,
    /// Browsing independently; the live route may have moved on.
    Independent,
    /// A cached / stale picture that is not the live route.
    CachedSnapshot,
}

impl LivenessClass {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Independent => "independent",
            Self::CachedSnapshot => "cached_snapshot",
        }
    }
}

/// Why a client is showing a cached snapshot instead of the live route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotStalenessReason {
    /// The connection to the presenter dropped.
    ConnectionLost,
    /// The presentation provider went offline.
    ProviderOffline,
    /// A reconnect is in flight but has not resynced yet.
    ReconnectPending,
    /// The viewer paused live updates deliberately.
    PausedByViewer,
}

impl SnapshotStalenessReason {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectionLost => "connection_lost",
            Self::ProviderOffline => "provider_offline",
            Self::ReconnectPending => "reconnect_pending",
            Self::PausedByViewer => "paused_by_viewer",
        }
    }
}

/// A cached snapshot's self-identification.
///
/// A snapshot must say it is a snapshot and must never claim to be a live shared
/// route. Both guardrail flags are fixed to their safe values by [`Self::new`]
/// and re-checked by [`FollowStateTruth::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotIdentity {
    /// Stable ref to when the cached picture was captured.
    pub snapshot_captured_at_ref: String,
    /// Why the client fell back to a snapshot.
    pub staleness_reason: SnapshotStalenessReason,
    /// True when the presenter's live route has moved past the cached anchor.
    pub presenter_route_diverged: bool,
    /// Always `true`: the view is labeled as a snapshot.
    pub labeled_as_snapshot: bool,
    /// Always `false`: a snapshot never claims to be a live shared route.
    pub claims_live_shared_route: bool,
}

impl SnapshotIdentity {
    /// Build a snapshot identity with the honesty guardrails fixed safe.
    pub fn new(
        snapshot_captured_at_ref: impl Into<String>,
        staleness_reason: SnapshotStalenessReason,
        presenter_route_diverged: bool,
    ) -> Self {
        Self {
            snapshot_captured_at_ref: snapshot_captured_at_ref.into(),
            staleness_reason,
            presenter_route_diverged,
            labeled_as_snapshot: true,
            claims_live_shared_route: false,
        }
    }

    /// True when the snapshot identifies itself honestly.
    pub const fn is_honest(&self) -> bool {
        self.labeled_as_snapshot && !self.claims_live_shared_route
    }
}

/// One recovery / return action a client offers from its current mode.
///
/// The action's stable command id, key-binding ref, and labels come from one
/// canonical table keyed by [`RecoveryKind`], so the recovery vocabulary cannot
/// drift between clients.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryAction {
    /// Which canonical recovery this action is.
    pub kind: RecoveryKind,
    /// Short visible label.
    pub label: String,
    /// Stable command id this action invokes.
    pub command_id: String,
    /// Stable key-binding id so the action is reachable without a pointer.
    pub key_binding_ref: String,
    /// Accessible name announced to assistive technology.
    pub accessible_label: String,
}

impl RecoveryAction {
    /// The canonical recovery action for a kind. Identical on every client.
    pub fn canonical(kind: RecoveryKind) -> Self {
        let (label, command_id, key_binding_ref, accessible_label) = kind.canonical_fields();
        Self {
            kind,
            label: label.to_owned(),
            command_id: command_id.to_owned(),
            key_binding_ref: key_binding_ref.to_owned(),
            accessible_label: accessible_label.to_owned(),
        }
    }

    /// True when this action carries exactly the canonical fields for its kind.
    pub fn is_canonical(&self) -> bool {
        *self == Self::canonical(self.kind)
    }
}

/// The canonical recovery / return actions, shared across every client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryKind {
    /// Jump back to the presenter's current live anchor.
    ReturnToPresenter,
    /// Leave a cached snapshot and rejoin the live route.
    RefreshLiveRoute,
    /// Ask the presenter to bring everyone to this client's anchor.
    RequestFollow,
    /// Ask to take over as presenter.
    RequestTakeOver,
}

impl RecoveryKind {
    /// Stable token recorded in records and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturnToPresenter => "return_to_presenter",
            Self::RefreshLiveRoute => "refresh_live_route",
            Self::RequestFollow => "request_follow",
            Self::RequestTakeOver => "request_take_over",
        }
    }

    /// The canonical `(label, command_id, key_binding_ref, accessible_label)`.
    pub const fn canonical_fields(
        self,
    ) -> (&'static str, &'static str, &'static str, &'static str) {
        match self {
            Self::ReturnToPresenter => (
                "Return to presenter",
                "cmd:presentation.return_to_presenter",
                "key:presentation.return_to_presenter",
                "Return to the presenter's current anchor",
            ),
            Self::RefreshLiveRoute => (
                "Refresh live",
                "cmd:presentation.refresh_live_route",
                "key:presentation.refresh_live_route",
                "Leave the cached snapshot and rejoin the presenter's live route",
            ),
            Self::RequestFollow => (
                "Request follow",
                "cmd:presentation.request_follow",
                "key:presentation.request_follow",
                "Ask the presenter to bring everyone to your anchor",
            ),
            Self::RequestTakeOver => (
                "Take over",
                "cmd:presentation.request_take_over",
                "key:presentation.request_take_over",
                "Request to take over as presenter",
            ),
        }
    }
}

/// A durable breakaway banner with an always-present return-to-presenter path.
///
/// Unlike a toast, the banner persists for as long as the client is broken away,
/// so the return path is never a moment the viewer can miss.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableBreakawayBanner {
    /// The explicit "you are browsing independently" state label.
    pub state_label: String,
    /// The presenter's current anchor, so the return path always resolves.
    pub presenter_anchor_ref: String,
    /// The keyboard-reachable way back to the presenter.
    pub return_to_presenter: RecoveryAction,
    /// Always `true`: the banner persists until the viewer returns; not a toast.
    pub durable: bool,
}

/// One client's follow posture in a session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientFollowView {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Which claimed client this view is for.
    pub client_surface: ClientSurface,
    /// The explicit follow mode this client is in.
    pub follow_mode: FollowMode,
    /// The liveness class read off the mode.
    pub liveness: LivenessClass,
    /// The presenter's current live anchor, so a return path always exists.
    pub presenter_anchor_ref: Option<String>,
    /// The anchor this client is currently showing — equal to the presenter's
    /// when following live, the viewer's own when broken away, and the stale
    /// anchor when on a cached snapshot.
    pub viewing_anchor_ref: Option<String>,
    /// Present only while broken away.
    pub breakaway_banner: Option<DurableBreakawayBanner>,
    /// Present only while showing a cached snapshot.
    pub snapshot_identity: Option<SnapshotIdentity>,
    /// Recovery actions offered on this client, drawn from the canonical table.
    pub recovery_actions: Vec<RecoveryAction>,
    /// Always `false`: state comes from explicit signals, not viewport drift.
    pub inferred_from_viewport_drift: bool,
    /// Always `false`: state comes from explicit signals, not connection timing.
    pub inferred_from_connection_timing: bool,
    /// Always `false`: this view is durable, not a transient toast.
    pub relies_on_transient_toast_only: bool,
}

impl ClientFollowView {
    /// The recovery-action kinds this view exposes, in display order.
    pub fn recovery_kinds(&self) -> Vec<RecoveryKind> {
        self.recovery_actions.iter().map(|a| a.kind).collect()
    }

    /// True when the view's surfaces line up with its declared mode and the
    /// honesty / non-inference guardrails all hold.
    pub fn is_consistent(&self) -> bool {
        self.consistency_violation().is_none()
    }

    /// The first consistency violation for this view, if any.
    fn consistency_violation(&self) -> Option<FollowStateViolation> {
        let surface = self.client_surface;
        if self.liveness != self.follow_mode.liveness() {
            return Some(FollowStateViolation::LivenessMismatch { surface });
        }
        if self.recovery_kinds() != self.follow_mode.recovery_action_kinds() {
            return Some(FollowStateViolation::RecoveryVocabularyMismatch { surface });
        }
        for action in &self.recovery_actions {
            if !action.is_canonical() {
                return Some(FollowStateViolation::RecoveryActionNotCanonical {
                    surface,
                    kind: action.kind,
                });
            }
        }
        // Breakaway banner must be present exactly when broken away, durable,
        // and carry the return path and the presenter anchor.
        match (
            &self.breakaway_banner,
            self.follow_mode.shows_breakaway_banner(),
        ) {
            (Some(banner), true) => {
                if !banner.durable
                    || banner.return_to_presenter.kind != RecoveryKind::ReturnToPresenter
                    || !banner.return_to_presenter.is_canonical()
                    || banner.presenter_anchor_ref.is_empty()
                {
                    return Some(FollowStateViolation::BreakawayBannerMalformed { surface });
                }
            }
            (None, false) => {}
            _ => return Some(FollowStateViolation::BreakawayBannerMismatch { surface }),
        }
        // Snapshot identity must be present exactly when on a cached snapshot,
        // and must identify itself honestly.
        match (
            &self.snapshot_identity,
            self.follow_mode.is_cached_snapshot(),
        ) {
            (Some(identity), true) => {
                if !identity.is_honest() {
                    return Some(FollowStateViolation::SnapshotImpliesLive { surface });
                }
            }
            (None, false) => {}
            _ => return Some(FollowStateViolation::SnapshotIdentityMismatch { surface }),
        }
        if self.inferred_from_viewport_drift
            || self.inferred_from_connection_timing
            || self.relies_on_transient_toast_only
        {
            return Some(FollowStateViolation::InferredState { surface });
        }
        None
    }
}

/// The cross-client follow-state truth packet for one session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateTruth {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// The presenter's current live anchor across every client.
    pub presenter_anchor_ref: Option<String>,
    /// One view per participating client.
    pub client_views: Vec<ClientFollowView>,
    // ---- invariant / guardrail flags (derived; re-checked by validate) ----
    /// Every client draws its follow mode from the same vocabulary.
    pub vocabulary_parity_across_clients: bool,
    /// Every client offers the same canonical recovery actions per mode.
    pub recovery_actions_parity_across_clients: bool,
    /// Every breakaway banner persists rather than flashing as a toast.
    pub breakaway_banner_durable: bool,
    /// No view infers its state from viewport drift.
    pub no_state_from_viewport_drift: bool,
    /// No view infers its state from connection timing.
    pub no_state_from_connection_timing: bool,
    /// No view depends on a transient toast alone to carry its state.
    pub no_transient_toast_only_state: bool,
    /// No cached snapshot claims to be a live shared route.
    pub no_snapshot_implies_live: bool,
    /// Always `false`: following is not a mutation shortcut.
    pub grants_mutation_authority: bool,
    /// Always `false`: following is not shared editing / debug control.
    pub grants_control_authority: bool,
}

impl FollowStateTruth {
    /// The view for `surface`, if present.
    pub fn client_view(&self, surface: ClientSurface) -> Option<&ClientFollowView> {
        self.client_views
            .iter()
            .find(|v| v.client_surface == surface)
    }

    /// The distinct client surfaces covered by this packet.
    pub fn surfaces_covered(&self) -> Vec<ClientSurface> {
        self.client_views
            .iter()
            .map(|v| v.client_surface)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    /// Validate every invariant the packet claims. An empty result means the
    /// packet is internally honest and the cross-client vocabulary is consistent.
    pub fn validate(&self) -> Vec<FollowStateViolation> {
        let mut violations = Vec::new();

        if self.record_kind != FOLLOW_STATE_TRUTH_RECORD_KIND
            || self.schema_version != PRESENTATION_MODE_BETA_SCHEMA_VERSION
            || self.shared_contract_ref != PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF
        {
            violations.push(FollowStateViolation::MalformedPacket);
        }

        if self.grants_mutation_authority || self.grants_control_authority {
            violations.push(FollowStateViolation::AuthorityWidened);
        }

        // Per-view consistency.
        for view in &self.client_views {
            if let Some(v) = view.consistency_violation() {
                violations.push(v);
            }
            // A non-presenter view must be able to reach the presenter's anchor.
            if view.follow_mode != FollowMode::Presenting
                && view.presenter_anchor_ref.is_none()
                && self.presenter_anchor_ref.is_none()
            {
                violations.push(FollowStateViolation::ReturnPathUnreachable {
                    surface: view.client_surface,
                });
            }
        }

        // Cross-client recovery vocabulary parity: any RecoveryKind that appears
        // anywhere must resolve to identical canonical fields everywhere.
        for view in &self.client_views {
            for action in &view.recovery_actions {
                if *action != RecoveryAction::canonical(action.kind) {
                    violations.push(FollowStateViolation::CrossClientVocabularyDivergence {
                        kind: action.kind,
                    });
                }
            }
        }

        // The derived flags must match what the views actually say.
        let expected = derive_flags(&self.client_views);
        let claimed = DerivedFlags {
            vocabulary_parity_across_clients: self.vocabulary_parity_across_clients,
            recovery_actions_parity_across_clients: self.recovery_actions_parity_across_clients,
            breakaway_banner_durable: self.breakaway_banner_durable,
            no_state_from_viewport_drift: self.no_state_from_viewport_drift,
            no_state_from_connection_timing: self.no_state_from_connection_timing,
            no_transient_toast_only_state: self.no_transient_toast_only_state,
            no_snapshot_implies_live: self.no_snapshot_implies_live,
        };
        if expected != claimed {
            violations.push(FollowStateViolation::DerivedFlagsMismatch);
        }

        violations
    }
}

/// A reason a [`FollowStateTruth`] failed validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FollowStateViolation {
    /// The packet carried the wrong record kind, version, or contract ref.
    MalformedPacket,
    /// The packet claimed to widen mutation or control authority.
    AuthorityWidened,
    /// A view's liveness did not match its follow mode.
    LivenessMismatch {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A view's recovery-action kinds did not match its follow mode.
    RecoveryVocabularyMismatch {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A recovery action carried non-canonical fields.
    RecoveryActionNotCanonical {
        /// The offending client surface.
        surface: ClientSurface,
        /// The offending recovery kind.
        kind: RecoveryKind,
    },
    /// A breakaway banner was present/absent against its mode, or malformed.
    BreakawayBannerMismatch {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A breakaway banner was not durable, or lacked its return path / anchor.
    BreakawayBannerMalformed {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A snapshot identity was present/absent against its mode.
    SnapshotIdentityMismatch {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A cached snapshot claimed to be a live shared route.
    SnapshotImpliesLive {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A view inferred its state from drift, timing, or a toast alone.
    InferredState {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A non-presenter view had no reachable presenter anchor.
    ReturnPathUnreachable {
        /// The offending client surface.
        surface: ClientSurface,
    },
    /// A recovery action diverged from the canonical vocabulary across clients.
    CrossClientVocabularyDivergence {
        /// The diverging recovery kind.
        kind: RecoveryKind,
    },
    /// The packet's derived invariant flags did not match its views.
    DerivedFlagsMismatch,
}

/// One support-safe row per client view. Carries enums, kinds, and booleans —
/// never anchor refs, accessible labels, or other potentially sensitive bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateSupportExportRow {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Session id.
    pub session_id: String,
    /// Which claimed client this row is for.
    pub client_surface: ClientSurface,
    /// The explicit follow mode.
    pub follow_mode: FollowMode,
    /// The liveness class.
    pub liveness: LivenessClass,
    /// Whether a durable breakaway banner is present.
    pub has_breakaway_banner: bool,
    /// Whether the view is on a cached snapshot.
    pub is_cached_snapshot: bool,
    /// Why the snapshot is stale, when on one.
    pub snapshot_staleness_reason: Option<SnapshotStalenessReason>,
    /// The recovery-action kinds offered, in display order.
    pub recovery_action_kinds: Vec<RecoveryKind>,
    /// Whether the view inferred its state from viewport drift.
    pub inferred_from_viewport_drift: bool,
    /// Whether the view inferred its state from connection timing.
    pub inferred_from_connection_timing: bool,
    /// Whether the view relies on a transient toast alone.
    pub relies_on_transient_toast_only: bool,
}

/// Support-export wrapper over a set of [`FollowStateTruth`] packets. Privacy-safe
/// by construction: no anchor refs or accessible labels are carried.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FollowStateSupportExport {
    /// Record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Export id.
    pub export_id: String,
    /// Mint timestamp.
    pub generated_at: String,
    /// Support-safe rows.
    pub rows: Vec<FollowStateSupportExportRow>,
    /// Always `true`: anchor refs and accessible labels are excluded.
    pub raw_private_material_excluded: bool,
}

impl FollowStateSupportExport {
    /// Project a set of follow-state packets into a support-safe export.
    pub fn from_packets<'a>(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        packets: impl IntoIterator<Item = &'a FollowStateTruth>,
    ) -> Self {
        let rows = packets
            .into_iter()
            .flat_map(|packet| {
                packet
                    .client_views
                    .iter()
                    .map(move |view| FollowStateSupportExportRow {
                        record_kind: FOLLOW_STATE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
                        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
                        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
                        session_id: packet.session_id.clone(),
                        client_surface: view.client_surface,
                        follow_mode: view.follow_mode,
                        liveness: view.liveness,
                        has_breakaway_banner: view.breakaway_banner.is_some(),
                        is_cached_snapshot: view.snapshot_identity.is_some(),
                        snapshot_staleness_reason: view
                            .snapshot_identity
                            .as_ref()
                            .map(|s| s.staleness_reason),
                        recovery_action_kinds: view.recovery_kinds(),
                        inferred_from_viewport_drift: view.inferred_from_viewport_drift,
                        inferred_from_connection_timing: view.inferred_from_connection_timing,
                        relies_on_transient_toast_only: view.relies_on_transient_toast_only,
                    })
            })
            .collect();
        Self {
            record_kind: FOLLOW_STATE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
            shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            rows,
            raw_private_material_excluded: true,
        }
    }
}

/// One client's intended follow posture, the input to
/// [`project_follow_state_truth`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClientFollowInput {
    /// Which claimed client this input is for.
    pub client_surface: ClientSurface,
    /// The explicit follow mode the client is in.
    pub follow_mode: FollowMode,
    /// The anchor this client is currently showing, if it differs from the
    /// presenter's. `None` falls back to the presenter's anchor for live modes.
    pub viewing_anchor_ref: Option<String>,
    /// The snapshot identity, required when `follow_mode` is a cached snapshot.
    pub snapshot_identity: Option<SnapshotIdentity>,
}

impl ClientFollowInput {
    /// A live-following client on `surface`.
    pub fn following(surface: ClientSurface) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::FollowingLive,
            viewing_anchor_ref: None,
            snapshot_identity: None,
        }
    }

    /// A presenting client on `surface`.
    pub fn presenting(surface: ClientSurface) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::Presenting,
            viewing_anchor_ref: None,
            snapshot_identity: None,
        }
    }

    /// A broken-away client browsing `viewing_anchor_ref`.
    pub fn broken_away(surface: ClientSurface, viewing_anchor_ref: impl Into<String>) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::BrokenAway,
            viewing_anchor_ref: Some(viewing_anchor_ref.into()),
            snapshot_identity: None,
        }
    }

    /// A cached-snapshot client showing `viewing_anchor_ref` under `identity`.
    pub fn cached_snapshot(
        surface: ClientSurface,
        viewing_anchor_ref: impl Into<String>,
        identity: SnapshotIdentity,
    ) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::CachedSnapshot,
            viewing_anchor_ref: Some(viewing_anchor_ref.into()),
            snapshot_identity: Some(identity),
        }
    }

    /// A client requesting follow, browsing `viewing_anchor_ref` meanwhile.
    pub fn requesting_follow(
        surface: ClientSurface,
        viewing_anchor_ref: impl Into<String>,
    ) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::RequestingFollow,
            viewing_anchor_ref: Some(viewing_anchor_ref.into()),
            snapshot_identity: None,
        }
    }

    /// A client requesting take-over while still seeing the live route.
    pub fn requesting_take_over(surface: ClientSurface) -> Self {
        Self {
            client_surface: surface,
            follow_mode: FollowMode::RequestingTakeOver,
            viewing_anchor_ref: None,
            snapshot_identity: None,
        }
    }
}

/// Project the presenter's current anchor for a session, mirroring how the
/// overlay picks the focused waypoint (current focus, else the first waypoint).
fn presenter_anchor(session: &PresentationSession) -> Option<String> {
    session
        .current_focus_waypoint_ref
        .as_ref()
        .and_then(|r| session.waypoints.iter().find(|w| &w.waypoint_id == r))
        .or_else(|| session.waypoints.first())
        .map(|w| w.target_object_ref.clone())
}

/// Build one client's follow view from its input and the presenter anchor.
fn build_client_view(
    input: &ClientFollowInput,
    presenter_anchor_ref: &Option<String>,
) -> ClientFollowView {
    let mode = input.follow_mode;
    let recovery_actions = mode
        .recovery_action_kinds()
        .into_iter()
        .map(RecoveryAction::canonical)
        .collect();

    let breakaway_banner = if mode.shows_breakaway_banner() {
        let anchor = presenter_anchor_ref
            .clone()
            .unwrap_or_else(|| "presentation:anchor:unknown".to_owned());
        Some(DurableBreakawayBanner {
            state_label: "You are browsing independently".to_owned(),
            presenter_anchor_ref: anchor,
            return_to_presenter: RecoveryAction::canonical(RecoveryKind::ReturnToPresenter),
            durable: true,
        })
    } else {
        None
    };

    let snapshot_identity = if mode.is_cached_snapshot() {
        input.snapshot_identity.clone()
    } else {
        None
    };

    // The anchor a live view shows is the presenter's; an independent or cached
    // view shows its own, falling back to the presenter's when none was given.
    let viewing_anchor_ref = match mode.liveness() {
        LivenessClass::Live => presenter_anchor_ref.clone(),
        LivenessClass::Independent | LivenessClass::CachedSnapshot => input
            .viewing_anchor_ref
            .clone()
            .or_else(|| presenter_anchor_ref.clone()),
    };

    ClientFollowView {
        record_kind: CLIENT_FOLLOW_VIEW_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        client_surface: input.client_surface,
        follow_mode: mode,
        liveness: mode.liveness(),
        presenter_anchor_ref: presenter_anchor_ref.clone(),
        viewing_anchor_ref,
        breakaway_banner,
        snapshot_identity,
        recovery_actions,
        inferred_from_viewport_drift: false,
        inferred_from_connection_timing: false,
        relies_on_transient_toast_only: false,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DerivedFlags {
    vocabulary_parity_across_clients: bool,
    recovery_actions_parity_across_clients: bool,
    breakaway_banner_durable: bool,
    no_state_from_viewport_drift: bool,
    no_state_from_connection_timing: bool,
    no_transient_toast_only_state: bool,
    no_snapshot_implies_live: bool,
}

fn derive_flags(views: &[ClientFollowView]) -> DerivedFlags {
    let mut vocabulary_parity = true;
    let mut recovery_parity = true;
    let mut banner_durable = true;
    let mut no_drift = true;
    let mut no_timing = true;
    let mut no_toast_only = true;
    let mut no_snapshot_live = true;

    for view in views {
        // Liveness must follow from the mode for the vocabulary to be coherent.
        if view.liveness != view.follow_mode.liveness() {
            vocabulary_parity = false;
        }
        if view.recovery_kinds() != view.follow_mode.recovery_action_kinds() {
            recovery_parity = false;
        }
        for action in &view.recovery_actions {
            if !action.is_canonical() {
                recovery_parity = false;
            }
        }
        if let Some(banner) = &view.breakaway_banner {
            if !banner.durable {
                banner_durable = false;
            }
        }
        if view.inferred_from_viewport_drift {
            no_drift = false;
        }
        if view.inferred_from_connection_timing {
            no_timing = false;
        }
        if view.relies_on_transient_toast_only {
            no_toast_only = false;
        }
        if let Some(identity) = &view.snapshot_identity {
            if !identity.is_honest() {
                no_snapshot_live = false;
            }
        }
    }

    DerivedFlags {
        vocabulary_parity_across_clients: vocabulary_parity,
        recovery_actions_parity_across_clients: recovery_parity,
        breakaway_banner_durable: banner_durable,
        no_state_from_viewport_drift: no_drift,
        no_state_from_connection_timing: no_timing,
        no_transient_toast_only_state: no_toast_only,
        no_snapshot_implies_live: no_snapshot_live,
    }
}

/// Project a [`FollowStateTruth`] packet for `session` across `client_inputs`.
///
/// The presenter's anchor is taken from the session's focused waypoint; every
/// client view is built from the same canonical recovery vocabulary, so the
/// resulting packet validates as long as the inputs are themselves coherent
/// (e.g. a cached-snapshot input carries an honest [`SnapshotIdentity`]).
pub fn project_follow_state_truth(
    session: &PresentationSession,
    client_inputs: &[ClientFollowInput],
) -> FollowStateTruth {
    let presenter_anchor_ref = presenter_anchor(session);
    let client_views: Vec<ClientFollowView> = client_inputs
        .iter()
        .map(|input| build_client_view(input, &presenter_anchor_ref))
        .collect();
    let flags = derive_flags(&client_views);
    FollowStateTruth {
        record_kind: FOLLOW_STATE_TRUTH_RECORD_KIND.to_owned(),
        schema_version: PRESENTATION_MODE_BETA_SCHEMA_VERSION,
        shared_contract_ref: PRESENTATION_MODE_BETA_SHARED_CONTRACT_REF.to_owned(),
        session_id: session.session_id.clone(),
        presenter_anchor_ref,
        client_views,
        vocabulary_parity_across_clients: flags.vocabulary_parity_across_clients,
        recovery_actions_parity_across_clients: flags.recovery_actions_parity_across_clients,
        breakaway_banner_durable: flags.breakaway_banner_durable,
        no_state_from_viewport_drift: flags.no_state_from_viewport_drift,
        no_state_from_connection_timing: flags.no_state_from_connection_timing,
        no_transient_toast_only_state: flags.no_transient_toast_only_state,
        no_snapshot_implies_live: flags.no_snapshot_implies_live,
        grants_mutation_authority: false,
        grants_control_authority: false,
    }
}
