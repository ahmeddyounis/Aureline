//! CI-status cards and session-follow tiles carrying provider/source class, run or
//! commit or session identity, freshness, permitted quick actions, an explicit
//! companion-versus-desktop capability boundary, and an exact desktop-handoff target.
//!
//! This module narrows two components frozen in
//! [`crate::freeze_the_m5_companion_component_matrix`] — the `ci_status_card` and the
//! `session_follow_tile` — into one implemented, export-safe packet with two co-equal
//! control vectors. Together they keep the companion honest about live versus stale
//! context: a user never has to infer which run or commit a CI card refers to, whether
//! a rerun is mobile-safe, whether a followed session is live enough to join, or what
//! exactly will open on desktop.
//!
//! A [`CiStatusCard`] always names its provider/source class, its stable run and commit
//! identity, the object it references, its client scope, its failure count, and its
//! freshness. Its result class is *derived* from the frozen CI status rather than
//! asserted: a stale CI status can never read as a live pass or fail. It always offers a
//! keyboard-complete `Open` verb that lands on one stable object, and any rerun or
//! widening verb names one exact desktop-handoff target — never a generic activity page,
//! and never implies a desktop-only rerun is companion-safe.
//!
//! A [`SessionFollowTile`] always names its presenter and session identity, the object it
//! references, its scope, and its freshness. Its joinability class is *derived* from the
//! frozen session-follow state rather than asserted: a diverged, stale, host-inactive, or
//! ended session can never read as live and joinable, so a tile degrades to an explicit
//! stale, read-only, or not-joinable state instead of an ambiguous empty card, and never
//! offers an ambiguous join into an expired or narrowed session.
//!
//! The object kinds ([`M5CompanionObjectKind`]), client scopes
//! ([`M5CompanionClientScope`]), freshness classes ([`M5CompanionFreshness`]), CI statuses
//! ([`M5CompanionCiStatus`]), session-follow states ([`M5CompanionSessionFollowState`]),
//! handoff targets ([`M5CompanionHandoffTarget`]), degraded reasons
//! ([`M5CompanionDegradedReason`]), required labels ([`M5CompanionRequiredLabel`]), surface
//! families ([`M5CompanionSurfaceFamily`]), deployment lines
//! ([`M5CompanionDeploymentLine`]), consumer surfaces ([`M5CompanionConsumerSurface`]),
//! accessibility routes ([`M5CompanionAccessibilityRoute`]), and downgrade triggers
//! ([`M5CompanionDowngradeTrigger`]) are reused directly from the frozen matrix, so this
//! lane never invents a parallel companion vocabulary. It mints new vocabulary only for
//! what that matrix left implicit about these two controls: the CI provider/source class,
//! the derived CI result class, the keyboard-complete CI status verbs, the derived session
//! joinability class, and the keyboard-complete session-follow verbs.
//!
//! Raw log bodies, build output, secret values, and private endpoints stay outside the
//! support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json`](../../../../schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json).
//! The contract doc is
//! [`docs/companion/implement_ci_status_cards_and_session_follow_tiles.md`](../../../../docs/companion/implement_ci_status_cards_and_session_follow_tiles.md).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_ci_status_card_session_follow_tile_controls,
    seeded_ci_status_card_session_follow_tile_controls_ci_status_card_stale,
    seeded_ci_status_card_session_follow_tile_controls_session_follow_tile_not_joinable,
    CI_STATUS_CARD_SESSION_FOLLOW_TILE_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The object kind, client scope, freshness, CI status, session-follow state, handoff
// target, degraded reason, required labels, surface family, deployment line, consumer
// surface, accessibility route, and downgrade triggers are frozen once, in the companion
// component matrix. This lane reuses them verbatim so it never invents a parallel
// companion vocabulary.
use crate::freeze_the_m5_companion_component_matrix::{
    M5CompanionAccessibilityRoute, M5CompanionCiStatus, M5CompanionClientScope,
    M5CompanionComponentFamily, M5CompanionConsumerSurface, M5CompanionDegradedReason,
    M5CompanionDeploymentLine, M5CompanionDowngradeTrigger, M5CompanionFreshness,
    M5CompanionHandoffTarget, M5CompanionObjectKind, M5CompanionRequiredLabel,
    M5CompanionSessionFollowState, M5CompanionSurfaceFamily, M5_CI_STATUS_CARD_SCHEMA_REF,
    M5_COMPANION_COMPONENT_DOC_REF, M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF,
    M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF, M5_COMPANION_COMPONENT_SCHEMA_REF,
    M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`CiStatusCardSessionFollowTileControlsPacket`].
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_RECORD_KIND: &str =
    "ci_status_card_session_follow_tile_controls";

/// Schema version for CI-status-card / session-follow-tile control records.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF: &str =
    "schemas/ui/m5-ci-status-card-session-follow-tile-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_DOC_REF: &str =
    "docs/companion/implement_ci_status_cards_and_session_follow_tiles.md";

/// Repo-relative path of the protected fixture directory.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_FIXTURE_DIR: &str =
    "fixtures/ui/m5-ci-status-card-session-follow-tile-controls";

/// Repo-relative path of the checked support-export artifact.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_ARTIFACT_REF: &str =
    "artifacts/release/m5-ci-status-card-session-follow-tile-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_SUMMARY_REF: &str =
    "artifacts/release/m5-ci-status-card-session-follow-tile-proof/summary.md";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const CI_STATUS_CARD_SESSION_FOLLOW_TILE_CSV_REF: &str =
    "artifacts/release/m5-ci-status-card-session-follow-tile-proof/matrix.csv";

// ---- ci-status-card vocabulary ------------------------------------------

/// Controlled provider / source class a CI-status card binds, so a user always knows where
/// a pipeline result came from and whether it is a live provider read or a mirrored
/// snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiProviderClass {
    /// CI executed by the local core.
    LocalCore,
    /// CI executed by a hosted provider (e.g. a hosted pipeline service).
    HostedProvider,
    /// CI executed by a self-hosted runner.
    SelfHostedRunner,
    /// A mirrored / offline snapshot of a provider result.
    MirroredSnapshot,
    /// An aggregate of results from more than one source.
    AggregatedSource,
    /// The provider / source could not be determined.
    UnknownSource,
}

impl CiProviderClass {
    /// Every provider class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalCore,
        Self::HostedProvider,
        Self::SelfHostedRunner,
        Self::MirroredSnapshot,
        Self::AggregatedSource,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCore => "local_core",
            Self::HostedProvider => "hosted_provider",
            Self::SelfHostedRunner => "self_hosted_runner",
            Self::MirroredSnapshot => "mirrored_snapshot",
            Self::AggregatedSource => "aggregated_source",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Derived result class a CI-status card may present.
///
/// This is the CI honesty axis: the class is derived from the frozen CI status, never
/// asserted, so a stale CI status can never present as a live pass or fail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiResultClass {
    /// The pipeline passed.
    Green,
    /// The pipeline failed.
    Red,
    /// The pipeline is running or queued.
    InFlight,
    /// The pipeline was canceled.
    Canceled,
    /// The status is stale and cannot be read as a live result.
    StaleUnknown,
}

impl CiResultClass {
    /// Every result class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Green,
        Self::Red,
        Self::InFlight,
        Self::Canceled,
        Self::StaleUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Red => "red",
            Self::InFlight => "in_flight",
            Self::Canceled => "canceled",
            Self::StaleUnknown => "stale_unknown",
        }
    }
}

/// One keyboard-complete default quick action a CI-status card offers, so a card never
/// hides its action affordance behind a pointer-only gesture and every widening or
/// mutating action is traceable to one exact desktop target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CiStatusCardVerb {
    /// Open the exact CI run this card references.
    Open,
    /// Follow the pipeline for further updates.
    Follow,
    /// Open the run logs (read-only from the companion).
    OpenLogs,
    /// Rerun the pipeline — a desktop-required action, never completed inline.
    Rerun,
    /// Hand off to the exact desktop target.
    HandoffToDesktop,
    /// Dismiss the card.
    Dismiss,
}

impl CiStatusCardVerb {
    /// Every CI status verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Follow,
        Self::OpenLogs,
        Self::Rerun,
        Self::HandoffToDesktop,
        Self::Dismiss,
    ];

    /// The default verbs every keyboard-complete card must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Follow => "follow",
            Self::OpenLogs => "open_logs",
            Self::Rerun => "rerun",
            Self::HandoffToDesktop => "handoff_to_desktop",
            Self::Dismiss => "dismiss",
        }
    }
}

/// Disclosures a CI-status card must carry, derived from the frozen CI status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CiCardDisclosure {
    /// The derived result class this card may present.
    pub result_class: CiResultClass,
    /// Whether the card may present a live, current result.
    pub is_live_result: bool,
    /// Whether the card must carry an explicit stale note.
    pub needs_stale_note: bool,
    /// Whether the card must carry an explicit in-flight note.
    pub needs_in_flight_note: bool,
    /// Whether the result class expects a non-zero failure count.
    pub expects_failures: bool,
    /// Whether the result class expects a zero failure count.
    pub expects_no_failures: bool,
}

/// Resolves the result truth a CI-status card may present.
///
/// A passed pipeline is green. A failed pipeline is red. A running or queued pipeline is
/// in-flight. A canceled pipeline is canceled. A stale status is stale-unknown — never a
/// live pass or fail — so a card whose status could not be refreshed never reads as a
/// current result.
pub fn resolve_ci_result(status: M5CompanionCiStatus) -> CiCardDisclosure {
    use CiResultClass as Result;
    use M5CompanionCiStatus as Ci;

    let result_class = match status {
        Ci::Passed => Result::Green,
        Ci::Failed => Result::Red,
        Ci::Running | Ci::Queued => Result::InFlight,
        Ci::Canceled => Result::Canceled,
        Ci::Stale => Result::StaleUnknown,
    };

    CiCardDisclosure {
        result_class,
        is_live_result: !matches!(result_class, Result::StaleUnknown),
        needs_stale_note: matches!(result_class, Result::StaleUnknown),
        needs_in_flight_note: matches!(result_class, Result::InFlight),
        expects_failures: matches!(result_class, Result::Red),
        expects_no_failures: matches!(result_class, Result::Green),
    }
}

/// A CI-status card naming provider/source class, run/commit identity, failure count,
/// derived result, permitted quick actions, and an exact handoff target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiStatusCard {
    /// Frozen component this control implements; must be `ci_status_card`.
    pub component: M5CompanionComponentFamily,
    /// Stable card id.
    pub card_id: String,
    /// Human-readable pipeline label; required and non-empty.
    pub pipeline_label: String,
    /// Object kind this card references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Stable run identity (e.g. a run number/id); required and non-empty.
    pub run_ref: String,
    /// Stable commit identity (e.g. a commit ref); required and non-empty.
    pub commit_ref: String,
    /// Client scope this card is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Provider / source class behind this card.
    pub provider_class: CiProviderClass,
    /// Human-readable provider / source label; required and non-empty.
    pub provider_label: String,
    /// CI status, reused from the frozen matrix.
    pub ci_status: M5CompanionCiStatus,
    /// Derived result class (must equal the resolved class).
    pub result_class: CiResultClass,
    /// Whether the card claims a live, current result (must equal the derived truth).
    pub claims_live_result: bool,
    /// Failure count surfaced on the card.
    pub failure_count: u32,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Stale note; required when the CI result is stale-unknown.
    pub stale_note: String,
    /// In-flight note; required when the pipeline is running or queued.
    pub in_flight_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default quick actions (must include the mandatory `Open`).
    pub status_verbs: Vec<CiStatusCardVerb>,
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
    /// Hard invariant: never implies a desktop-required action (e.g. rerun) is
    /// companion-safe. MUST be `false`.
    pub implies_desktop_action_is_companion_safe: bool,
    /// Hard invariant: `Open` never routes to a generic activity page. MUST be `false`.
    pub routes_to_generic_activity_page: bool,
}

impl CiStatusCard {
    /// Result disclosures this card must carry, derived from the frozen CI status.
    pub fn result_disclosure(&self) -> CiCardDisclosure {
        resolve_ci_result(self.ci_status)
    }

    /// Whether the card offers every mandatory keyboard-complete quick action.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<CiStatusCardVerb> = self.status_verbs.iter().copied().collect();
        CiStatusCardVerb::MANDATORY
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
        self.status_verbs
            .contains(&CiStatusCardVerb::HandoffToDesktop)
    }

    /// Whether the card offers a rerun quick action.
    fn offers_rerun(&self) -> bool {
        self.status_verbs.contains(&CiStatusCardVerb::Rerun)
    }
}

// ---- session-follow-tile vocabulary -------------------------------------

/// Derived joinability class a session-follow tile may present.
///
/// This is the session honesty axis: the class is derived from the frozen session-follow
/// state, never asserted, so a diverged, stale, host-inactive, or ended session can never
/// present as live and joinable — a tile degrades to an explicit read-only or not-joinable
/// state instead of an ambiguous empty card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionJoinability {
    /// Live and joinable now.
    LiveJoinable,
    /// Paused, resumable back into a live follow.
    PausedResumable,
    /// A read-only, stale mirror — followable for context, not live.
    StaleReadOnly,
    /// Not joinable — the host session is inactive or following has ended.
    NotJoinable,
}

impl SessionJoinability {
    /// Every joinability class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveJoinable,
        Self::PausedResumable,
        Self::StaleReadOnly,
        Self::NotJoinable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveJoinable => "live_joinable",
            Self::PausedResumable => "paused_resumable",
            Self::StaleReadOnly => "stale_read_only",
            Self::NotJoinable => "not_joinable",
        }
    }
}

/// One keyboard-complete default follow verb a session-follow tile offers, so a tile never
/// hides its follow affordance behind a pointer-only gesture and never offers an ambiguous
/// join into an expired or narrowed session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionFollowTileVerb {
    /// Open the exact followed session this tile references.
    Open,
    /// Follow the host session live.
    Follow,
    /// Pause the live follow.
    PauseFollow,
    /// Resume a paused follow.
    ResumeFollow,
    /// Hand off to the exact desktop target.
    HandoffToDesktop,
    /// Leave the follow.
    LeaveFollow,
}

impl SessionFollowTileVerb {
    /// Every session-follow verb, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Follow,
        Self::PauseFollow,
        Self::ResumeFollow,
        Self::HandoffToDesktop,
        Self::LeaveFollow,
    ];

    /// The default verbs every keyboard-complete tile must offer.
    pub const MANDATORY: [Self; 1] = [Self::Open];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Follow => "follow",
            Self::PauseFollow => "pause_follow",
            Self::ResumeFollow => "resume_follow",
            Self::HandoffToDesktop => "handoff_to_desktop",
            Self::LeaveFollow => "leave_follow",
        }
    }

    /// Whether this verb tries to join or resume a live follow.
    fn is_join_verb(self) -> bool {
        matches!(self, Self::Follow | Self::ResumeFollow)
    }
}

/// Disclosures a session-follow tile must carry, derived from the follow state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionTileDisclosure {
    /// The derived joinability class this tile may present.
    pub joinability: SessionJoinability,
    /// Whether the tile may present a live session.
    pub is_live_session: bool,
    /// Whether the tile may present the session as joinable.
    pub is_joinable: bool,
    /// Whether the tile must carry an explicit stale / read-only note.
    pub needs_stale_note: bool,
    /// Whether the tile must carry an explicit not-joinable note.
    pub needs_not_joinable_note: bool,
}

/// Resolves the joinability truth a session-follow tile may present.
///
/// A live-following session is live and joinable. A paused follow is paused and resumable.
/// A diverged or read-only-mirror session floors to a stale read-only state that is never
/// joinable. A host-inactive or follow-ended session is not joinable, so a followed
/// session's state is never shown greener than reality and no tile offers an ambiguous join
/// into an expired or narrowed session.
pub fn resolve_session_joinability(state: M5CompanionSessionFollowState) -> SessionTileDisclosure {
    use M5CompanionSessionFollowState as Follow;
    use SessionJoinability as Join;

    let joinability = match state {
        Follow::LiveFollowing => Join::LiveJoinable,
        Follow::PausedFollow => Join::PausedResumable,
        Follow::DivergedFromHost | Follow::ReadOnlyMirror => Join::StaleReadOnly,
        Follow::HostInactive | Follow::FollowEnded => Join::NotJoinable,
    };

    SessionTileDisclosure {
        joinability,
        is_live_session: matches!(joinability, Join::LiveJoinable),
        is_joinable: matches!(joinability, Join::LiveJoinable | Join::PausedResumable),
        needs_stale_note: matches!(joinability, Join::StaleReadOnly),
        needs_not_joinable_note: matches!(joinability, Join::NotJoinable),
    }
}

/// A session-follow tile naming presenter/session identity, scope, freshness, derived
/// joinability, permitted follow verbs, and an exact handoff target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionFollowTile {
    /// Frozen component this control implements; must be `session_follow_tile`.
    pub component: M5CompanionComponentFamily,
    /// Stable tile id.
    pub tile_id: String,
    /// Human-readable session label; required and non-empty.
    pub session_label: String,
    /// Object kind this tile references, reused from the frozen matrix.
    pub object_kind: M5CompanionObjectKind,
    /// Human-readable object label; required and non-empty.
    pub object_label: String,
    /// Exact object landing reference — the one stable object `Open` lands on, never a
    /// generic activity page. Required and non-empty.
    pub object_landing_ref: String,
    /// Presenter identity — who is presenting the session; required and non-empty.
    pub presenter_ref: String,
    /// Session identity — the stable session id; required and non-empty.
    pub session_ref: String,
    /// Client scope this tile is scoped to, reused from the frozen matrix.
    pub client_scope: M5CompanionClientScope,
    /// Human-readable client-scope label; required and non-empty.
    pub scope_label: String,
    /// Session-follow state, reused from the frozen matrix.
    pub follow_state: M5CompanionSessionFollowState,
    /// Derived joinability class (must equal the resolved class).
    pub joinability: SessionJoinability,
    /// Whether the tile claims the session is live (must equal the derived truth).
    pub claims_live_session: bool,
    /// Whether the tile claims the session is joinable (must equal the derived truth).
    pub claims_joinable: bool,
    /// Freshness class, reused from the frozen matrix.
    pub freshness: M5CompanionFreshness,
    /// Joinability note; always required so the joinability boundary stays explicit.
    pub joinability_note: String,
    /// Stale / read-only note; required when the session is a stale read-only mirror.
    pub stale_note: String,
    /// Not-joinable note; required when the session is not joinable.
    pub not_joinable_note: String,
    /// Scope / freshness note; always required so scope and freshness stay explicit.
    pub scope_and_freshness_note: String,
    /// Exact desktop-handoff target, reused from the frozen matrix.
    pub handoff_target: M5CompanionHandoffTarget,
    /// Human-readable handoff label; always required so the handoff target is explicit.
    pub handoff_label: String,
    /// Keyboard-complete default follow verbs (must include the mandatory `Open`).
    pub follow_verbs: Vec<SessionFollowTileVerb>,
    /// Degraded reasons this tile can name (required, matching the frozen matrix).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Mandatory labels this tile can show (must include the mandatory labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Claimed M5 surface families that render this tile.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this tile keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Non-visual accessibility routes this tile offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Companion subsystems that consume this tile's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this tile.
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

impl SessionFollowTile {
    /// Joinability disclosures this tile must carry, derived from the follow state.
    pub fn joinability_disclosure(&self) -> SessionTileDisclosure {
        resolve_session_joinability(self.follow_state)
    }

    /// Whether the tile offers every mandatory keyboard-complete follow verb.
    fn declares_mandatory_verbs(&self) -> bool {
        let present: BTreeSet<SessionFollowTileVerb> = self.follow_verbs.iter().copied().collect();
        SessionFollowTileVerb::MANDATORY
            .iter()
            .all(|verb| present.contains(verb))
    }

    /// Whether the tile declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// Whether the tile offers a desktop-handoff verb.
    fn offers_handoff(&self) -> bool {
        self.follow_verbs
            .contains(&SessionFollowTileVerb::HandoffToDesktop)
    }

    /// Whether the tile offers a join or resume verb.
    fn offers_join(&self) -> bool {
        self.follow_verbs.iter().any(|verb| verb.is_join_verb())
    }
}

// ---- review blocks ------------------------------------------------------

/// First-glance trust review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiStatusCardSessionFollowTileGlanceReview {
    /// The CI card names its stable run and commit identity.
    pub ci_card_shows_run_and_commit_identity: bool,
    /// The CI card names its provider / source class.
    pub ci_card_shows_provider_source: bool,
    /// The CI card names its failure count.
    pub ci_card_shows_failure_count: bool,
    /// The session tile names its presenter and session identity.
    pub session_tile_shows_presenter_and_session_identity: bool,
    /// The session tile states its joinability.
    pub session_tile_states_joinability: bool,
    /// The session tile degrades to an explicit stale / offline / not-joinable state.
    pub session_tile_degrades_to_explicit_state: bool,
    /// The object identity is always explicit.
    pub object_identity_always_explicit: bool,
    /// The client scope is always explicit.
    pub client_scope_always_explicit: bool,
    /// The freshness is always explicit.
    pub freshness_always_explicit: bool,
    /// Result / joinability is derived from CI status / follow state, never asserted.
    pub result_and_joinability_derived_never_asserted: bool,
    /// A stale card is never shown as live.
    pub stale_never_shown_as_live: bool,
    /// Every verb traces to one stable object.
    pub every_verb_traces_to_one_object: bool,
    /// Every widening verb names one exact desktop-handoff target.
    pub every_handoff_names_exact_target: bool,
    /// A desktop-only action (e.g. rerun) is never implied companion-safe.
    pub desktop_only_action_never_implied_companion_safe: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The controls keep the same truth across every deployment line.
    pub controls_stable_across_deployment_lines: bool,
    /// The controls stay copy and export safe.
    pub copy_and_export_safe: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
}

impl CiStatusCardSessionFollowTileGlanceReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.ci_card_shows_run_and_commit_identity
            && self.ci_card_shows_provider_source
            && self.ci_card_shows_failure_count
            && self.session_tile_shows_presenter_and_session_identity
            && self.session_tile_states_joinability
            && self.session_tile_degrades_to_explicit_state
            && self.object_identity_always_explicit
            && self.client_scope_always_explicit
            && self.freshness_always_explicit
            && self.result_and_joinability_derived_never_asserted
            && self.stale_never_shown_as_live
            && self.every_verb_traces_to_one_object
            && self.every_handoff_names_exact_target
            && self.desktop_only_action_never_implied_companion_safe
            && self.no_surface_invents_alternate_state_label
            && self.controls_stable_across_deployment_lines
            && self.copy_and_export_safe
            && self.downgrade_narrows_instead_of_hides
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiStatusCardSessionFollowTileConsumerProjection {
    /// The CI-status UI reads a single canonical source.
    pub ci_status_ui_reads_single_source: bool,
    /// The session-follow UI reads a single canonical source.
    pub session_follow_ui_reads_single_source: bool,
    /// The first glance names object, scope, and freshness without drilling in.
    pub first_glance_names_object_scope_and_freshness: bool,
    /// The rerun / join posture is visible before a tap.
    pub rerun_and_join_posture_visible_before_tap: bool,
    /// Support export shows control truth.
    pub support_export_shows_control_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_control_truth: bool,
}

impl CiStatusCardSessionFollowTileConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.ci_status_ui_reads_single_source
            && self.session_follow_ui_reads_single_source
            && self.first_glance_names_object_scope_and_freshness
            && self.rerun_and_join_posture_visible_before_tap
            && self.support_export_shows_control_truth
            && self.help_about_shows_control_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiStatusCardSessionFollowTileProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`CiStatusCardSessionFollowTileControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CiStatusCardSessionFollowTileControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// CI-status cards.
    pub ci_status_cards: Vec<CiStatusCard>,
    /// Session-follow tiles.
    pub session_follow_tiles: Vec<SessionFollowTile>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: CiStatusCardSessionFollowTileGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: CiStatusCardSessionFollowTileConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CiStatusCardSessionFollowTileProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe CI-status-card / session-follow-tile controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CiStatusCardSessionFollowTileControlsPacket {
    /// Record kind; must equal [`CI_STATUS_CARD_SESSION_FOLLOW_TILE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// CI-status cards.
    pub ci_status_cards: Vec<CiStatusCard>,
    /// Session-follow tiles.
    pub session_follow_tiles: Vec<SessionFollowTile>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Glance review block.
    pub glance_review: CiStatusCardSessionFollowTileGlanceReview,
    /// Consumer projection block.
    pub consumer_projection: CiStatusCardSessionFollowTileConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: CiStatusCardSessionFollowTileProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CiStatusCardSessionFollowTileControlsPacket {
    /// Builds a CI-status-card / session-follow-tile controls packet from stable-lane input.
    pub fn new(input: CiStatusCardSessionFollowTileControlsPacketInput) -> Self {
        Self {
            record_kind: CI_STATUS_CARD_SESSION_FOLLOW_TILE_RECORD_KIND.to_owned(),
            schema_version: CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            ci_status_cards: input.ci_status_cards,
            session_follow_tiles: input.session_follow_tiles,
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

    /// Validates the CI-status-card / session-follow-tile control invariants.
    pub fn validate(&self) -> Vec<CiStatusCardSessionFollowTileViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CI_STATUS_CARD_SESSION_FOLLOW_TILE_RECORD_KIND {
            violations.push(CiStatusCardSessionFollowTileViolation::WrongRecordKind);
        }
        if self.schema_version != CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_VERSION {
            violations.push(CiStatusCardSessionFollowTileViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CiStatusCardSessionFollowTileViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_ci_status_cards(self, &mut violations);
        validate_session_follow_tiles(self, &mut violations);

        if !self.glance_review.all_hold() {
            violations.push(CiStatusCardSessionFollowTileViolation::GlanceReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(CiStatusCardSessionFollowTileViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(CiStatusCardSessionFollowTileViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("ci status card session follow tile packet serializes"),
        ) {
            violations.push(CiStatusCardSessionFollowTileViolation::RawBoundaryMaterialInExport);
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
            .expect("ci status card session follow tile packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one line per control.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "control,id,object_kind,client_scope,freshness,state_or_source,derived,live_or_joinable\n",
        );
        for card in &self.ci_status_cards {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "ci_status_card",
                csv_field(&card.card_id),
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.freshness.as_str(),
                card.ci_status.as_str(),
                card.result_disclosure().result_class.as_str(),
                card.result_disclosure().is_live_result,
            ));
        }
        for tile in &self.session_follow_tiles {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                "session_follow_tile",
                csv_field(&tile.tile_id),
                tile.object_kind.as_str(),
                tile.client_scope.as_str(),
                tile.freshness.as_str(),
                tile.follow_state.as_str(),
                tile.joinability_disclosure().joinability.as_str(),
                tile.joinability_disclosure().is_joinable,
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let not_live = self
            .ci_status_cards
            .iter()
            .filter(|card| !card.result_disclosure().is_live_result)
            .count();
        let not_joinable = self
            .session_follow_tiles
            .iter()
            .filter(|tile| !tile.joinability_disclosure().is_joinable)
            .count();

        let mut out = String::new();
        out.push_str("# CI-status cards and session-follow tiles\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- CI-status cards: {} ({} not a live result)\n",
            self.ci_status_cards.len(),
            not_live
        ));
        out.push_str(&format!(
            "- Session-follow tiles: {} ({} not joinable)\n",
            self.session_follow_tiles.len(),
            not_joinable
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## CI-status cards\n\n");
        for card in &self.ci_status_cards {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, source `{}`, status `{}`, freshness `{}` → `{}`, handoff `{}`\n",
                card.pipeline_label,
                card.object_kind.as_str(),
                card.client_scope.as_str(),
                card.provider_class.as_str(),
                card.ci_status.as_str(),
                card.freshness.as_str(),
                card.result_disclosure().result_class.as_str(),
                card.handoff_target.as_str(),
            ));
        }

        out.push_str("\n## Session-follow tiles\n\n");
        for tile in &self.session_follow_tiles {
            out.push_str(&format!(
                "- **{}** ({}) — scope `{}`, state `{}`, freshness `{}` → `{}`, handoff `{}`\n",
                tile.session_label,
                tile.object_kind.as_str(),
                tile.client_scope.as_str(),
                tile.follow_state.as_str(),
                tile.freshness.as_str(),
                tile.joinability_disclosure().joinability.as_str(),
                tile.handoff_target.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in CI-status-card / session-follow-tile export.
#[derive(Debug)]
pub enum CiStatusCardSessionFollowTileArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CiStatusCardSessionFollowTileViolation>),
}

impl fmt::Display for CiStatusCardSessionFollowTileArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "ci status card session follow tile export parse failed: {error}"
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
                    "ci status card session follow tile export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CiStatusCardSessionFollowTileArtifactError {}

/// Validation failures emitted by [`CiStatusCardSessionFollowTileControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CiStatusCardSessionFollowTileViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No CI-status cards are present.
    CiStatusCardsMissing,
    /// A CI-status card is incomplete.
    CiStatusCardIncomplete,
    /// A CI-status card carries the wrong frozen component class.
    CiStatusCardWrongComponentClass,
    /// A CI-status card does not name its exact object landing reference.
    ObjectLandingRefMissing,
    /// A CI-status card does not name its stable run / commit identity.
    RunOrCommitIdentityMissing,
    /// A CI-status card does not name its provider / source label.
    ProviderLabelMissing,
    /// A CI-status card misrepresents its derived result state.
    ResultStateMisrepresented,
    /// A CI-status card misrepresents its failure count for the derived result.
    FailureCountMisrepresented,
    /// A stale CI-status card does not name its stale state.
    StaleNoteMissing,
    /// An in-flight CI-status card does not name its in-flight state.
    InFlightNoteMissing,
    /// A CI-status card omits the mandatory `Open` verb.
    CiStatusVerbsIncomplete,
    /// A CI-status card offers a rerun but has no exact desktop handoff target.
    RerunTargetUnresolved,
    /// The CI-status cards do not cover every derived result class.
    ResultClassCoverageMissing,
    /// The CI-status cards do not cover every CI status.
    CiStatusCoverageMissing,
    /// No session-follow tiles are present.
    SessionFollowTilesMissing,
    /// A session-follow tile is incomplete.
    SessionFollowTileIncomplete,
    /// A session-follow tile carries the wrong frozen component class.
    SessionFollowTileWrongComponentClass,
    /// A session-follow tile does not name its presenter / session identity.
    PresenterOrSessionIdentityMissing,
    /// A session-follow tile misrepresents its derived joinability state.
    JoinabilityMisrepresented,
    /// A session-follow tile does not name its joinability note.
    JoinabilityNoteMissing,
    /// A stale read-only session-follow tile does not name its stale state.
    SessionStaleNoteMissing,
    /// A not-joinable session-follow tile does not name its not-joinable state.
    NotJoinableNoteMissing,
    /// A session-follow tile omits the mandatory `Open` verb.
    SessionFollowVerbsIncomplete,
    /// A session-follow tile offers an ambiguous join into a non-joinable session.
    AmbiguousJoinOffered,
    /// The session-follow tiles do not cover every joinability class.
    JoinabilityCoverageMissing,
    /// The session-follow tiles do not cover every session-follow state.
    SessionFollowStateCoverageMissing,
    /// A control does not name its scope / freshness.
    ScopeAndFreshnessNoteMissing,
    /// A control does not name its scope label.
    ScopeLabelMissing,
    /// A control offers a handoff verb but its handoff target does not resolve exactly.
    HandoffTargetUnresolved,
    /// A control does not name its handoff label.
    HandoffLabelMissing,
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

impl CiStatusCardSessionFollowTileViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::CiStatusCardsMissing => "ci_status_cards_missing",
            Self::CiStatusCardIncomplete => "ci_status_card_incomplete",
            Self::CiStatusCardWrongComponentClass => "ci_status_card_wrong_component_class",
            Self::ObjectLandingRefMissing => "object_landing_ref_missing",
            Self::RunOrCommitIdentityMissing => "run_or_commit_identity_missing",
            Self::ProviderLabelMissing => "provider_label_missing",
            Self::ResultStateMisrepresented => "result_state_misrepresented",
            Self::FailureCountMisrepresented => "failure_count_misrepresented",
            Self::StaleNoteMissing => "stale_note_missing",
            Self::InFlightNoteMissing => "in_flight_note_missing",
            Self::CiStatusVerbsIncomplete => "ci_status_verbs_incomplete",
            Self::RerunTargetUnresolved => "rerun_target_unresolved",
            Self::ResultClassCoverageMissing => "result_class_coverage_missing",
            Self::CiStatusCoverageMissing => "ci_status_coverage_missing",
            Self::SessionFollowTilesMissing => "session_follow_tiles_missing",
            Self::SessionFollowTileIncomplete => "session_follow_tile_incomplete",
            Self::SessionFollowTileWrongComponentClass => {
                "session_follow_tile_wrong_component_class"
            }
            Self::PresenterOrSessionIdentityMissing => "presenter_or_session_identity_missing",
            Self::JoinabilityMisrepresented => "joinability_misrepresented",
            Self::JoinabilityNoteMissing => "joinability_note_missing",
            Self::SessionStaleNoteMissing => "session_stale_note_missing",
            Self::NotJoinableNoteMissing => "not_joinable_note_missing",
            Self::SessionFollowVerbsIncomplete => "session_follow_verbs_incomplete",
            Self::AmbiguousJoinOffered => "ambiguous_join_offered",
            Self::JoinabilityCoverageMissing => "joinability_coverage_missing",
            Self::SessionFollowStateCoverageMissing => "session_follow_state_coverage_missing",
            Self::ScopeAndFreshnessNoteMissing => "scope_and_freshness_note_missing",
            Self::ScopeLabelMissing => "scope_label_missing",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
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

/// Reads and validates the checked-in stable CI-status-card / session-follow-tile export.
pub fn current_ci_status_card_session_follow_tile_export(
) -> Result<CiStatusCardSessionFollowTileControlsPacket, CiStatusCardSessionFollowTileArtifactError>
{
    let packet: CiStatusCardSessionFollowTileControlsPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-ci-status-card-session-follow-tile-proof/support_export.json"
        )))
        .map_err(CiStatusCardSessionFollowTileArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CiStatusCardSessionFollowTileArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CiStatusCardSessionFollowTileControlsPacket,
    violations: &mut Vec<CiStatusCardSessionFollowTileViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_SCHEMA_REF,
        CI_STATUS_CARD_SESSION_FOLLOW_TILE_DOC_REF,
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_CI_STATUS_CARD_SCHEMA_REF,
        M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CiStatusCardSessionFollowTileViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_ci_status_cards(
    packet: &CiStatusCardSessionFollowTileControlsPacket,
    violations: &mut Vec<CiStatusCardSessionFollowTileViolation>,
) {
    if packet.ci_status_cards.is_empty() {
        violations.push(CiStatusCardSessionFollowTileViolation::CiStatusCardsMissing);
        return;
    }

    let mut result_classes: BTreeSet<CiResultClass> = BTreeSet::new();
    let mut ci_statuses: BTreeSet<M5CompanionCiStatus> = BTreeSet::new();

    for card in &packet.ci_status_cards {
        let disclosure = card.result_disclosure();
        result_classes.insert(disclosure.result_class);
        ci_statuses.insert(card.ci_status);

        if card.card_id.trim().is_empty()
            || card.pipeline_label.trim().is_empty()
            || card.object_label.trim().is_empty()
            || card.fields_shown.is_empty()
            || card.surface_families.is_empty()
            || card.deployment_lines.is_empty()
            || card.consumer_surfaces.is_empty()
            || card.source_contract_refs.is_empty()
        {
            violations.push(CiStatusCardSessionFollowTileViolation::CiStatusCardIncomplete);
        }
        if card.component != M5CompanionComponentFamily::CiStatusCard {
            violations
                .push(CiStatusCardSessionFollowTileViolation::CiStatusCardWrongComponentClass);
        }
        if card.object_landing_ref.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ObjectLandingRefMissing);
        }
        if card.run_ref.trim().is_empty() || card.commit_ref.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::RunOrCommitIdentityMissing);
        }
        if card.provider_label.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ProviderLabelMissing);
        }
        if card.result_class != disclosure.result_class
            || card.claims_live_result != disclosure.is_live_result
        {
            violations.push(CiStatusCardSessionFollowTileViolation::ResultStateMisrepresented);
        }
        if (disclosure.expects_failures && card.failure_count == 0)
            || (disclosure.expects_no_failures && card.failure_count != 0)
        {
            violations.push(CiStatusCardSessionFollowTileViolation::FailureCountMisrepresented);
        }
        if disclosure.needs_stale_note && card.stale_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::StaleNoteMissing);
        }
        if disclosure.needs_in_flight_note && card.in_flight_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::InFlightNoteMissing);
        }
        if card.scope_and_freshness_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ScopeAndFreshnessNoteMissing);
        }
        if card.scope_label.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ScopeLabelMissing);
        }
        if !card.declares_mandatory_verbs() {
            violations.push(CiStatusCardSessionFollowTileViolation::CiStatusVerbsIncomplete);
        }
        if card.offers_handoff() && card.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations.push(CiStatusCardSessionFollowTileViolation::HandoffTargetUnresolved);
        }
        if card.offers_rerun() && card.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations.push(CiStatusCardSessionFollowTileViolation::RerunTargetUnresolved);
        }
        if card.handoff_label.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::HandoffLabelMissing);
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

    for required in CiResultClass::ALL {
        if !result_classes.contains(&required) {
            violations.push(CiStatusCardSessionFollowTileViolation::ResultClassCoverageMissing);
            break;
        }
    }
    for required in M5CompanionCiStatus::ALL {
        if !ci_statuses.contains(&required) {
            violations.push(CiStatusCardSessionFollowTileViolation::CiStatusCoverageMissing);
            break;
        }
    }
}

fn validate_session_follow_tiles(
    packet: &CiStatusCardSessionFollowTileControlsPacket,
    violations: &mut Vec<CiStatusCardSessionFollowTileViolation>,
) {
    if packet.session_follow_tiles.is_empty() {
        violations.push(CiStatusCardSessionFollowTileViolation::SessionFollowTilesMissing);
        return;
    }

    let mut joinabilities: BTreeSet<SessionJoinability> = BTreeSet::new();
    let mut follow_states: BTreeSet<M5CompanionSessionFollowState> = BTreeSet::new();

    for tile in &packet.session_follow_tiles {
        let disclosure = tile.joinability_disclosure();
        joinabilities.insert(disclosure.joinability);
        follow_states.insert(tile.follow_state);

        if tile.tile_id.trim().is_empty()
            || tile.session_label.trim().is_empty()
            || tile.object_label.trim().is_empty()
            || tile.fields_shown.is_empty()
            || tile.surface_families.is_empty()
            || tile.deployment_lines.is_empty()
            || tile.consumer_surfaces.is_empty()
            || tile.source_contract_refs.is_empty()
        {
            violations.push(CiStatusCardSessionFollowTileViolation::SessionFollowTileIncomplete);
        }
        if tile.component != M5CompanionComponentFamily::SessionFollowTile {
            violations
                .push(CiStatusCardSessionFollowTileViolation::SessionFollowTileWrongComponentClass);
        }
        if tile.object_landing_ref.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ObjectLandingRefMissing);
        }
        if tile.presenter_ref.trim().is_empty() || tile.session_ref.trim().is_empty() {
            violations
                .push(CiStatusCardSessionFollowTileViolation::PresenterOrSessionIdentityMissing);
        }
        if tile.joinability != disclosure.joinability
            || tile.claims_live_session != disclosure.is_live_session
            || tile.claims_joinable != disclosure.is_joinable
        {
            violations.push(CiStatusCardSessionFollowTileViolation::JoinabilityMisrepresented);
        }
        if tile.joinability_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::JoinabilityNoteMissing);
        }
        if disclosure.needs_stale_note && tile.stale_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::SessionStaleNoteMissing);
        }
        if disclosure.needs_not_joinable_note && tile.not_joinable_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::NotJoinableNoteMissing);
        }
        if tile.scope_and_freshness_note.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ScopeAndFreshnessNoteMissing);
        }
        if tile.scope_label.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::ScopeLabelMissing);
        }
        if !tile.declares_mandatory_verbs() {
            violations.push(CiStatusCardSessionFollowTileViolation::SessionFollowVerbsIncomplete);
        }
        if tile.offers_join() && !disclosure.is_joinable {
            violations.push(CiStatusCardSessionFollowTileViolation::AmbiguousJoinOffered);
        }
        if tile.offers_handoff() && tile.handoff_target == M5CompanionHandoffTarget::NoHandoff {
            violations.push(CiStatusCardSessionFollowTileViolation::HandoffTargetUnresolved);
        }
        if tile.handoff_label.trim().is_empty() {
            violations.push(CiStatusCardSessionFollowTileViolation::HandoffLabelMissing);
        }
        validate_common_control(
            &tile.degraded_reasons,
            tile.declares_mandatory_labels(),
            &tile.accessibility_routes,
            ControlInvariants {
                masks_scope_or_freshness: tile.masks_scope_or_freshness,
                hides_capability_boundary: tile.hides_capability_boundary,
                invents_alternate_state_label: tile.invents_alternate_state_label,
                implies_desktop_action_is_companion_safe: tile
                    .implies_desktop_action_is_companion_safe,
                routes_to_generic_activity_page: tile.routes_to_generic_activity_page,
            },
            violations,
        );
    }

    for required in SessionJoinability::ALL {
        if !joinabilities.contains(&required) {
            violations.push(CiStatusCardSessionFollowTileViolation::JoinabilityCoverageMissing);
            break;
        }
    }
    for required in M5CompanionSessionFollowState::ALL {
        if !follow_states.contains(&required) {
            violations
                .push(CiStatusCardSessionFollowTileViolation::SessionFollowStateCoverageMissing);
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
    violations: &mut Vec<CiStatusCardSessionFollowTileViolation>,
) {
    if degraded_reasons.is_empty() {
        violations.push(CiStatusCardSessionFollowTileViolation::DegradedReasonsMissing);
    }
    if !declares_mandatory_labels {
        violations.push(CiStatusCardSessionFollowTileViolation::RequiredLabelsIncomplete);
    }
    if accessibility_routes.is_empty()
        || !accessibility_routes.contains(&M5CompanionAccessibilityRoute::KeyboardFocusable)
    {
        violations.push(CiStatusCardSessionFollowTileViolation::AccessibilityRouteMissing);
    }
    if invariants.masks_scope_or_freshness {
        violations.push(CiStatusCardSessionFollowTileViolation::ScopeOrFreshnessMasked);
    }
    if invariants.hides_capability_boundary {
        violations.push(CiStatusCardSessionFollowTileViolation::CapabilityBoundaryHidden);
    }
    if invariants.invents_alternate_state_label {
        violations.push(CiStatusCardSessionFollowTileViolation::AlternateStateLabelInvented);
    }
    if invariants.implies_desktop_action_is_companion_safe {
        violations.push(CiStatusCardSessionFollowTileViolation::DesktopActionImpliedCompanionSafe);
    }
    if invariants.routes_to_generic_activity_page {
        violations.push(CiStatusCardSessionFollowTileViolation::RoutesToGenericActivityPage);
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
