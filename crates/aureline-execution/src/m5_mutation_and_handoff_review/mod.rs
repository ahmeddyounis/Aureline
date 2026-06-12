//! Canonical M5 mutation-and-handoff review matrix with a non-inheriting commit gate
//! that keeps every M5 mutation path explicit about *who* is acting, *whether* it was
//! approved, *how* it can be recovered, and *what* it does before it crosses a request,
//! browser, preview, remote, or live-resource boundary.
//!
//! Each [`MutationPathReviewRow`] names one M5 mutation path that can widen authority
//! or side effects — request-workspace mutations, browser-runtime actions, preview-route
//! actions, live-resource operations, remote mutations, and the browser/companion and
//! vendor-console handoffs that carry them — and answers, for that path, who is acting
//! ([`ActorAuthorityState`]), whether approval is satisfied ([`ApprovalState`]), what
//! recovery path applies ([`RollbackClass`]), what time-bound route or tunnel effect the
//! action opens ([`RouteEffect`]), and whether a browser/companion handoff preserves the
//! same authority and rollback semantics ([`HandoffContinuity`]). The row then publishes
//! a [`CommitReadiness`] no input can exceed.
//!
//! The [`CommitReadiness`] a path may publish is the weakest ceiling implied by its
//! observed states, so an unverified or merely inherited actor, a bypassed or pending
//! approval, an unknown or provider-managed rollback, an unbounded route, or a severed
//! handoff all narrow, flag, or withhold the commit automatically. The guardrail this
//! enforces: a request-workspace or browser-runtime path can never inherit hidden
//! authority simply because it originated from a trusted local shell, and a browser or
//! companion handoff can never be used as an approval bypass or an excuse to drop
//! rollback or fallback semantics. The [`ReviewDecision`] records the gate's action —
//! reviewed apply, require approval, preview only, flag a renegotiated handoff for
//! review, or withhold — and the recomputed [`MutationNarrowingReason`]s explain it; all
//! are validated against the gate.
//!
//! The sheet model is reviewed and reconstructable. Every path resolves through one
//! reviewed preview/apply/handoff sheet instead of a hidden per-feature prompt: the row
//! surfaces actor, approval, target context, expected duration, time-bound route effect,
//! rollback class, and a fallback or open-in-provider path before commit, and it exports
//! a machine-readable mutation receipt so support, audit, and release evidence can
//! reconstruct which reviewed action class actually ran without replaying the action.
//!
//! The mutation-path and handoff vocabulary is closed and shared. [`MutationPath`] is the
//! single controlled vocabulary every M5 mutation surface reuses instead of inventing
//! feature-local review prompts, and a path that crosses a browser/companion or
//! vendor-console handoff boundary must declare a [`HandoffContinuity`] so the handoff is
//! reviewed rather than treated as a bypass.
//!
//! The packet is checked in at
//! `artifacts/execution/m5/m5-mutation-and-handoff-review.json` and embedded here. It is
//! metadata-only: every field is a typed state or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, host tokens, or control-plane secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 mutation-and-handoff review matrix schema version.
pub const M5_MUTATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_MUTATION_REVIEW_RECORD_KIND: &str = "m5_mutation_and_handoff_review_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_MUTATION_REVIEW_PATH: &str =
    "artifacts/execution/m5/m5-mutation-and-handoff-review.json";

/// Embedded checked-in packet JSON.
pub const M5_MUTATION_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-mutation-and-handoff-review.json"
));

/// An M5 mutation path that can widen authority or side effects and so must cross the
/// reviewed sheet before it commits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationPath {
    /// Request-workspace mutation lane (a request runtime mutating workspace state).
    RequestWorkspaceMutation,
    /// Browser-runtime action lane.
    BrowserRuntimeAction,
    /// Preview-route action lane.
    PreviewRouteAction,
    /// Live-resource operation lane.
    LiveResourceOperation,
    /// Remote mutation lane.
    RemoteMutation,
    /// Browser/companion handoff lane.
    CompanionHandoff,
    /// Time-bound route or tunnel action lane.
    TunnelRouteAction,
    /// Explicit vendor-console handoff lane.
    ProviderConsoleHandoff,
}

impl MutationPath {
    /// Every mutation path, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RequestWorkspaceMutation,
        Self::BrowserRuntimeAction,
        Self::PreviewRouteAction,
        Self::LiveResourceOperation,
        Self::RemoteMutation,
        Self::CompanionHandoff,
        Self::TunnelRouteAction,
        Self::ProviderConsoleHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestWorkspaceMutation => "request_workspace_mutation",
            Self::BrowserRuntimeAction => "browser_runtime_action",
            Self::PreviewRouteAction => "preview_route_action",
            Self::LiveResourceOperation => "live_resource_operation",
            Self::RemoteMutation => "remote_mutation",
            Self::CompanionHandoff => "companion_handoff",
            Self::TunnelRouteAction => "tunnel_route_action",
            Self::ProviderConsoleHandoff => "provider_console_handoff",
        }
    }

    /// Whether this path inherently crosses a browser/companion or vendor-console handoff
    /// boundary and therefore must declare a non-`NotHandoff` [`HandoffContinuity`].
    ///
    /// This is the pinned relationship the gate validates a handoff against, so a browser
    /// or companion handoff can never silently bypass the canonical review model.
    pub const fn requires_handoff(self) -> bool {
        matches!(
            self,
            Self::BrowserRuntimeAction | Self::CompanionHandoff | Self::ProviderConsoleHandoff
        )
    }
}

/// How ready a mutation path is to commit through the reviewed sheet.
///
/// Ordered low-to-high by [`CommitReadiness::rank`]: a [`CommitReadiness::Blocked`] path
/// cannot proceed at all, and a [`CommitReadiness::ReviewedApply`] path may commit a
/// reviewed apply with a verified actor, satisfied approval, a known rollback, a bounded
/// route, and a preserved handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommitReadiness {
    /// A reviewed apply may commit.
    ReviewedApply,
    /// A commit may proceed only after an explicit approval ticket.
    ApprovalRequired,
    /// Only a preview or dry-run is allowed; no commit.
    PreviewOnly,
    /// The mutation cannot proceed at all.
    Blocked,
}

impl CommitReadiness {
    /// Every commit readiness, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewedApply,
        Self::ApprovalRequired,
        Self::PreviewOnly,
        Self::Blocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedApply => "reviewed_apply",
            Self::ApprovalRequired => "approval_required",
            Self::PreviewOnly => "preview_only",
            Self::Blocked => "blocked",
        }
    }

    /// Monotonic rank; higher means more commit authority.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Blocked => 0,
            Self::PreviewOnly => 1,
            Self::ApprovalRequired => 2,
            Self::ReviewedApply => 3,
        }
    }

    /// The weaker (lower-rank) of two readinesses.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// Who is acting on the mutation path and whether that authority is established.
///
/// These states are the guardrail against inherited authority: a path that only
/// *inherits* authority from a trusted local shell — rather than re-establishing it for
/// the actor at the boundary it crosses — can never publish a reviewed apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorAuthorityState {
    /// The actor's authority was verified for this path and boundary.
    Verified,
    /// The actor holds explicitly delegated authority; caps at approval-required.
    Delegated,
    /// The authority is only inherited from the local shell; caps at preview-only.
    Inherited,
    /// No actor authority could be established; caps at blocked.
    Unestablished,
}

impl ActorAuthorityState {
    /// Every actor-authority state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Delegated,
        Self::Inherited,
        Self::Unestablished,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Delegated => "delegated",
            Self::Inherited => "inherited",
            Self::Unestablished => "unestablished",
        }
    }

    /// Highest readiness this actor-authority state permits a path to publish.
    pub const fn readiness_ceiling(self) -> CommitReadiness {
        match self {
            Self::Verified => CommitReadiness::ReviewedApply,
            Self::Delegated => CommitReadiness::ApprovalRequired,
            Self::Inherited => CommitReadiness::PreviewOnly,
            Self::Unestablished => CommitReadiness::Blocked,
        }
    }

    /// Whether this state raises the [`MutationNarrowingReason::InheritedAuthority`]
    /// trigger.
    pub const fn is_inherited_trigger(self) -> bool {
        matches!(self, Self::Inherited)
    }

    /// Whether this state raises the [`MutationNarrowingReason::UnverifiedActor`] trigger.
    pub const fn is_unestablished_trigger(self) -> bool {
        matches!(self, Self::Unestablished)
    }
}

/// Whether the mutation path's approval requirement is satisfied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalState {
    /// Approval was granted and a ticket is bound.
    Approved,
    /// No approval is required for this path.
    NotRequired,
    /// Approval is required and not yet granted; caps at approval-required.
    Required,
    /// Approval was bypassed; caps at blocked.
    Bypassed,
}

impl ApprovalState {
    /// Every approval state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Approved,
        Self::NotRequired,
        Self::Required,
        Self::Bypassed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::NotRequired => "not_required",
            Self::Required => "required",
            Self::Bypassed => "bypassed",
        }
    }

    /// Highest readiness this approval state permits a path to publish.
    pub const fn readiness_ceiling(self) -> CommitReadiness {
        match self {
            Self::Approved | Self::NotRequired => CommitReadiness::ReviewedApply,
            Self::Required => CommitReadiness::ApprovalRequired,
            Self::Bypassed => CommitReadiness::Blocked,
        }
    }

    /// Whether this state requires an approval-ticket ref to be carried.
    pub const fn requires_ticket(self) -> bool {
        matches!(self, Self::Approved | Self::Required)
    }

    /// Whether this state raises the [`MutationNarrowingReason::ApprovalBypassed`]
    /// trigger.
    pub const fn is_bypassed_trigger(self) -> bool {
        matches!(self, Self::Bypassed)
    }
}

/// The recovery path that applies if the mutation must be undone.
///
/// These states keep the reviewed sheet honest about rollback: a mutation whose rollback
/// class is unknown can never commit, and a provider-managed rollback (recoverable only
/// through a vendor console) can only be previewed natively.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackClass {
    /// The mutation is directly reversible.
    Reversible,
    /// The mutation is recoverable via a compensating action; caps at approval-required.
    Compensable,
    /// The mutation is recoverable only via a provider/vendor console; caps at
    /// preview-only.
    ProviderManaged,
    /// The rollback class is unknown; caps at blocked.
    Unknown,
}

impl RollbackClass {
    /// Every rollback class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Reversible,
        Self::Compensable,
        Self::ProviderManaged,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Reversible => "reversible",
            Self::Compensable => "compensable",
            Self::ProviderManaged => "provider_managed",
            Self::Unknown => "unknown",
        }
    }

    /// Highest readiness this rollback class permits a path to publish.
    pub const fn readiness_ceiling(self) -> CommitReadiness {
        match self {
            Self::Reversible => CommitReadiness::ReviewedApply,
            Self::Compensable => CommitReadiness::ApprovalRequired,
            Self::ProviderManaged => CommitReadiness::PreviewOnly,
            Self::Unknown => CommitReadiness::Blocked,
        }
    }

    /// Whether this rollback class requires a rollback-plan ref to be carried.
    pub const fn requires_plan(self) -> bool {
        matches!(self, Self::Compensable | Self::ProviderManaged)
    }

    /// Whether this state raises the [`MutationNarrowingReason::RollbackUnknown`] trigger.
    pub const fn is_unknown_trigger(self) -> bool {
        matches!(self, Self::Unknown)
    }
}

/// The time-bound route or tunnel effect the mutation opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteEffect {
    /// The mutation opens no route or tunnel.
    #[serde(rename = "none")]
    NoneEffect,
    /// The mutation opens a time-bound route that auto-expires; requires an expiry ref.
    TimeBound,
    /// The mutation opens a persistent route; caps at approval-required.
    Persistent,
    /// The mutation opens an unbounded route with no expiry; caps at preview-only.
    Unbounded,
}

impl RouteEffect {
    /// Every route effect, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoneEffect,
        Self::TimeBound,
        Self::Persistent,
        Self::Unbounded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneEffect => "none",
            Self::TimeBound => "time_bound",
            Self::Persistent => "persistent",
            Self::Unbounded => "unbounded",
        }
    }

    /// Highest readiness this route effect permits a path to publish.
    pub const fn readiness_ceiling(self) -> CommitReadiness {
        match self {
            Self::NoneEffect | Self::TimeBound => CommitReadiness::ReviewedApply,
            Self::Persistent => CommitReadiness::ApprovalRequired,
            Self::Unbounded => CommitReadiness::PreviewOnly,
        }
    }

    /// Whether this route effect requires a route-expiry ref to be carried.
    pub const fn requires_expiry(self) -> bool {
        matches!(self, Self::TimeBound)
    }

    /// Whether this state raises the [`MutationNarrowingReason::UnboundedRoute`] trigger.
    pub const fn is_unbounded_trigger(self) -> bool {
        matches!(self, Self::Unbounded)
    }
}

/// Whether a browser/companion handoff preserves the path's authority and rollback
/// semantics.
///
/// These states enforce that a handoff is reviewed rather than used as a bypass: a
/// severed handoff that would drop authority or rollback semantics can never commit, and
/// a renegotiated handoff is always flagged for review before it is adopted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffContinuity {
    /// No handoff is involved in this path.
    NotHandoff,
    /// The handoff preserves the same authority and rollback semantics.
    Preserved,
    /// The handoff re-established authority and must be reviewed; caps at
    /// approval-required.
    Renegotiated,
    /// The handoff would drop authority or rollback semantics; caps at blocked.
    Severed,
}

impl HandoffContinuity {
    /// Every handoff-continuity state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotHandoff,
        Self::Preserved,
        Self::Renegotiated,
        Self::Severed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotHandoff => "not_handoff",
            Self::Preserved => "preserved",
            Self::Renegotiated => "renegotiated",
            Self::Severed => "severed",
        }
    }

    /// Highest readiness this handoff-continuity state permits a path to publish.
    pub const fn readiness_ceiling(self) -> CommitReadiness {
        match self {
            Self::NotHandoff | Self::Preserved => CommitReadiness::ReviewedApply,
            Self::Renegotiated => CommitReadiness::ApprovalRequired,
            Self::Severed => CommitReadiness::Blocked,
        }
    }

    /// Whether the gate should flag the handoff for review rather than silently adopt it.
    ///
    /// A renegotiated handoff must be surfaced explicitly so a browser or companion
    /// handoff is never treated as an approval bypass.
    pub const fn is_flaggable(self) -> bool {
        matches!(self, Self::Renegotiated)
    }

    /// Whether this state raises the [`MutationNarrowingReason::HandoffSevered`] trigger.
    pub const fn is_severed_trigger(self) -> bool {
        matches!(self, Self::Severed)
    }
}

/// The expected duration of the mutation, surfaced on the reviewed sheet.
///
/// This is a descriptive label the user sees before commit; it does not cap the
/// readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurationClass {
    /// The mutation completes effectively instantly.
    Instant,
    /// The mutation completes in a short, bounded window.
    Short,
    /// The mutation runs for an extended but bounded window.
    Extended,
    /// The mutation runs open-ended until explicitly stopped.
    OpenEnded,
}

impl DurationClass {
    /// Every duration class, in declaration order.
    pub const ALL: [Self; 4] = [Self::Instant, Self::Short, Self::Extended, Self::OpenEnded];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Short => "short",
            Self::Extended => "extended",
            Self::OpenEnded => "open_ended",
        }
    }
}

/// The fallback or open-in-provider path surfaced when a mutation cannot cleanly apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackPath {
    /// Retry the reviewed action after the narrowing condition is resolved.
    ReviewedRetry,
    /// Open the action in the provider's surface.
    OpenInProvider,
    /// Hand the action off to the vendor's console.
    VendorConsole,
    /// No fallback is offered; only valid when the path cleanly applies.
    NoFallback,
}

impl FallbackPath {
    /// Every fallback path, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewedRetry,
        Self::OpenInProvider,
        Self::VendorConsole,
        Self::NoFallback,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewedRetry => "reviewed_retry",
            Self::OpenInProvider => "open_in_provider",
            Self::VendorConsole => "vendor_console",
            Self::NoFallback => "no_fallback",
        }
    }

    /// Whether this is a real fallback the user can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoFallback)
    }
}

/// A headline reason the mutation gate narrows a path.
///
/// These are the canonical mutation release-control triggers: an unverified actor, an
/// inherited authority, a bypassed approval, an unknown rollback, an unbounded route, and
/// a severed handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationNarrowingReason {
    /// No actor authority could be established.
    UnverifiedActor,
    /// The actor authority is only inherited from the local shell.
    InheritedAuthority,
    /// The approval requirement was bypassed.
    ApprovalBypassed,
    /// The rollback class is unknown.
    RollbackUnknown,
    /// The mutation opens an unbounded route.
    UnboundedRoute,
    /// A browser/companion handoff would drop authority or rollback semantics.
    HandoffSevered,
}

impl MutationNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UnverifiedActor,
        Self::InheritedAuthority,
        Self::ApprovalBypassed,
        Self::RollbackUnknown,
        Self::UnboundedRoute,
        Self::HandoffSevered,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnverifiedActor => "unverified_actor",
            Self::InheritedAuthority => "inherited_authority",
            Self::ApprovalBypassed => "approval_bypassed",
            Self::RollbackUnknown => "rollback_unknown",
            Self::UnboundedRoute => "unbounded_route",
            Self::HandoffSevered => "handoff_severed",
        }
    }
}

/// The action the mutation gate takes on a path relative to a clean reviewed apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewDecision {
    /// No narrowing; the path commits a reviewed apply.
    Apply,
    /// The path may commit only after an explicit approval ticket.
    RequireApproval,
    /// Only a preview or dry-run is allowed; no commit.
    PreviewOnly,
    /// Surface the renegotiated handoff for review before it is adopted.
    FlagForReview,
    /// Withhold the mutation entirely; it cannot proceed.
    Withhold,
}

impl ReviewDecision {
    /// Every review decision, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Apply,
        Self::RequireApproval,
        Self::PreviewOnly,
        Self::FlagForReview,
        Self::Withhold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Apply => "apply",
            Self::RequireApproval => "require_approval",
            Self::PreviewOnly => "preview_only",
            Self::FlagForReview => "flag_for_review",
            Self::Withhold => "withhold",
        }
    }

    /// Whether the gate narrowed, flagged, or withheld the path.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Apply)
    }
}

/// One mutation-and-handoff review row for an M5 mutation path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MutationPathReviewRow {
    /// Stable mutation-path review id.
    pub path_id: String,
    /// M5 mutation path this row governs.
    pub mutation_path: MutationPath,
    /// Who is acting and whether the authority is established.
    pub actor_authority: ActorAuthorityState,
    /// Whether the approval requirement is satisfied.
    pub approval_state: ApprovalState,
    /// Recovery path that applies if the mutation must be undone.
    pub rollback_class: RollbackClass,
    /// Time-bound route or tunnel effect the mutation opens.
    pub route_effect: RouteEffect,
    /// Whether a browser/companion handoff preserves authority and rollback semantics.
    pub handoff_continuity: HandoffContinuity,
    /// Expected duration surfaced on the reviewed sheet.
    pub expected_duration: DurationClass,
    /// Fallback or open-in-provider path surfaced when the mutation cannot apply cleanly.
    pub fallback_path: FallbackPath,
    /// Readiness the path's own evidence asserts, before the gate.
    pub declared_readiness: CommitReadiness,
    /// Readiness actually published after the gate narrows the path.
    ///
    /// Must equal [`MutationPathReviewRow::effective_readiness`].
    pub published_readiness: CommitReadiness,
    /// Decision the gate takes; must equal the recomputed decision.
    pub review_decision: ReviewDecision,
    /// Headline narrowing reasons; must equal the recomputed set.
    #[serde(default)]
    pub narrowing_reasons: Vec<MutationNarrowingReason>,
    /// Ref to the actor identity the mutation is attributed to.
    pub actor_ref: String,
    /// Ref to the target context the mutation applies to.
    pub target_context_ref: String,
    /// Ref to the reviewed preview/apply/handoff sheet the user saw.
    pub review_sheet_ref: String,
    /// Ref to the machine-readable mutation receipt for support, audit, and release
    /// evidence.
    pub mutation_receipt_ref: String,
    /// Ref to the approval ticket; required when approval is granted or pending.
    #[serde(default)]
    pub approval_ticket_ref: String,
    /// Ref to the rollback plan; required when rollback is compensable or
    /// provider-managed.
    #[serde(default)]
    pub rollback_plan_ref: String,
    /// Ref to the route expiry; required when the route is time-bound.
    #[serde(default)]
    pub route_expiry_ref: String,
    /// Ref to the in-product execution this mutation applies to.
    pub execution_ref: String,
    /// Ref binding this row into desktop, CLI, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl MutationPathReviewRow {
    /// The readiness the path's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> CommitReadiness {
        self.declared_readiness
    }

    /// The readiness the gate permits this path to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the actor authority,
    /// approval, rollback, route, and handoff states, so an inherited authority, a
    /// bypassed approval, an unknown rollback, an unbounded route, or a severed handoff
    /// can never publish a reviewed apply.
    pub fn effective_readiness(&self) -> CommitReadiness {
        self.capability_floor()
            .min(self.actor_authority.readiness_ceiling())
            .min(self.approval_state.readiness_ceiling())
            .min(self.rollback_class.readiness_ceiling())
            .min(self.route_effect.readiness_ceiling())
            .min(self.handoff_continuity.readiness_ceiling())
    }

    /// The headline narrowing reasons recomputed from the path's observed states.
    pub fn computed_narrowing_reasons(&self) -> Vec<MutationNarrowingReason> {
        let mut reasons = Vec::new();
        if self.actor_authority.is_unestablished_trigger() {
            reasons.push(MutationNarrowingReason::UnverifiedActor);
        }
        if self.actor_authority.is_inherited_trigger() {
            reasons.push(MutationNarrowingReason::InheritedAuthority);
        }
        if self.approval_state.is_bypassed_trigger() {
            reasons.push(MutationNarrowingReason::ApprovalBypassed);
        }
        if self.rollback_class.is_unknown_trigger() {
            reasons.push(MutationNarrowingReason::RollbackUnknown);
        }
        if self.route_effect.is_unbounded_trigger() {
            reasons.push(MutationNarrowingReason::UnboundedRoute);
        }
        if self.handoff_continuity.is_severed_trigger() {
            reasons.push(MutationNarrowingReason::HandoffSevered);
        }
        reasons
    }

    /// The decision the gate must record for this path.
    ///
    /// A blocked readiness is withheld; a renegotiated handoff is flagged for review
    /// before adoption; a reviewed apply commits; and anything in between narrows to
    /// require-approval or preview-only.
    pub fn required_decision(&self) -> ReviewDecision {
        let effective = self.effective_readiness();
        if effective == CommitReadiness::Blocked {
            ReviewDecision::Withhold
        } else if self.handoff_continuity.is_flaggable() {
            ReviewDecision::FlagForReview
        } else {
            match effective {
                CommitReadiness::ReviewedApply => ReviewDecision::Apply,
                CommitReadiness::ApprovalRequired => ReviewDecision::RequireApproval,
                CommitReadiness::PreviewOnly => ReviewDecision::PreviewOnly,
                CommitReadiness::Blocked => ReviewDecision::Withhold,
            }
        }
    }

    /// Whether the path may commit a clean reviewed apply.
    pub fn is_committable(&self) -> bool {
        self.effective_readiness() == CommitReadiness::ReviewedApply
    }

    /// Whether the path carries its own non-empty actor, target, sheet, receipt,
    /// execution, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.actor_ref.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && !self.review_sheet_ref.trim().is_empty()
            && !self.mutation_receipt_ref.trim().is_empty()
            && !self.execution_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published readiness, decision, and narrowing reasons all agree
    /// with the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_readiness == self.effective_readiness()
            && self.review_decision == self.required_decision()
            && self.narrowing_reasons == self.computed_narrowing_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MutationReviewSummary {
    /// Total path rows.
    pub total_paths: usize,
    /// Number of claimed paths.
    pub path_count: usize,
    /// Paths published with a reviewed-apply readiness.
    pub committable_paths: usize,
    /// Paths published with an approval-required readiness.
    pub approval_required_paths: usize,
    /// Paths published with a preview-only readiness.
    pub preview_only_paths: usize,
    /// Paths published with a blocked readiness.
    pub blocked_paths: usize,
    /// Paths the gate cleared to commit a reviewed apply.
    pub applied_paths: usize,
    /// Paths the gate narrowed to require approval.
    pub require_approval_paths: usize,
    /// Paths the gate flagged for review.
    pub flagged_paths: usize,
    /// Paths the gate narrowed to preview-only.
    pub preview_decision_paths: usize,
    /// Paths the gate withheld entirely.
    pub withheld_paths: usize,
    /// Paths that cross a browser/companion or vendor-console handoff boundary.
    pub handoff_paths: usize,
    /// Paths carrying a granted approval.
    pub approved_paths: usize,
    /// Paths whose approval was bypassed.
    pub bypassed_approval_paths: usize,
    /// Paths whose rollback class is unknown.
    pub unknown_rollback_paths: usize,
    /// Paths opening a time-bound route.
    pub time_bound_route_paths: usize,
    /// Paths carrying at least one narrowing reason.
    pub paths_with_narrowing_reasons: usize,
}

/// A redaction-safe export row projected from a mutation-path review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationReviewExportRow {
    /// Mutation-path review id.
    pub path_id: String,
    /// Mutation-path token.
    pub mutation_path: String,
    /// Actor-authority-state token.
    pub actor_authority: String,
    /// Approval-state token.
    pub approval_state: String,
    /// Rollback-class token.
    pub rollback_class: String,
    /// Route-effect token.
    pub route_effect: String,
    /// Handoff-continuity token.
    pub handoff_continuity: String,
    /// Expected-duration token.
    pub expected_duration: String,
    /// Fallback-path token.
    pub fallback_path: String,
    /// Declared-readiness token.
    pub declared_readiness: String,
    /// Published-readiness token.
    pub published_readiness: String,
    /// Review-decision token.
    pub review_decision: String,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Actor identity ref.
    pub actor_ref: String,
    /// Mutation-receipt ref.
    pub mutation_receipt_ref: String,
    /// Execution ref the mutation applies to.
    pub execution_ref: String,
    /// Whether the path crosses a handoff boundary.
    pub crosses_handoff: bool,
    /// Whether the path may commit a reviewed apply.
    pub committable: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MutationReviewExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub paths: Vec<M5MutationReviewExportRow>,
    /// Whether every path's published readiness and decision agree with the gate.
    pub all_paths_gate_consistent: bool,
    /// Paths that may commit a reviewed apply.
    pub committable_count: usize,
    /// Paths the gate narrowed, flagged, or withheld.
    pub narrowed_count: usize,
    /// Paths the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 mutation-and-handoff review matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5MutationReviewMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Claimed paths; one row per path.
    pub mutation_paths: Vec<MutationPath>,
    /// Closed commit-readiness vocabulary.
    pub commit_readinesses: Vec<CommitReadiness>,
    /// Closed actor-authority-state vocabulary.
    pub actor_authority_states: Vec<ActorAuthorityState>,
    /// Closed approval-state vocabulary.
    pub approval_states: Vec<ApprovalState>,
    /// Closed rollback-class vocabulary.
    pub rollback_classes: Vec<RollbackClass>,
    /// Closed route-effect vocabulary.
    pub route_effects: Vec<RouteEffect>,
    /// Closed handoff-continuity vocabulary.
    pub handoff_continuities: Vec<HandoffContinuity>,
    /// Closed duration-class vocabulary.
    pub duration_classes: Vec<DurationClass>,
    /// Closed fallback-path vocabulary.
    pub fallback_paths: Vec<FallbackPath>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<MutationNarrowingReason>,
    /// Closed review-decision vocabulary.
    pub review_decisions: Vec<ReviewDecision>,
    /// Mutation-path review rows, one per claimed path.
    #[serde(default)]
    pub paths: Vec<MutationPathReviewRow>,
    /// Summary counts.
    pub summary: M5MutationReviewSummary,
}

impl M5MutationReviewMatrix {
    /// Returns the row for a claimed path.
    pub fn path(&self, path: MutationPath) -> Option<&MutationPathReviewRow> {
        self.paths.iter().find(|p| p.mutation_path == path)
    }

    /// Paths that may commit a reviewed apply.
    pub fn committable_paths(&self) -> impl Iterator<Item = &MutationPathReviewRow> {
        self.paths.iter().filter(|p| p.is_committable())
    }

    /// Paths the gate narrowed, flagged, or withheld in any way.
    pub fn narrowed_paths(&self) -> impl Iterator<Item = &MutationPathReviewRow> {
        self.paths
            .iter()
            .filter(|p| p.required_decision().is_narrowed())
    }

    /// Paths the gate withheld entirely.
    pub fn withheld_paths(&self) -> impl Iterator<Item = &MutationPathReviewRow> {
        self.paths
            .iter()
            .filter(|p| p.required_decision() == ReviewDecision::Withhold)
    }

    /// Whether every path's stored published readiness, decision, and reasons agree with
    /// the recomputed gate decision.
    pub fn all_paths_gate_consistent(&self) -> bool {
        self.paths.iter().all(|p| p.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5MutationReviewSummary {
        let count_published = |readiness: CommitReadiness| {
            self.paths
                .iter()
                .filter(|p| p.published_readiness == readiness)
                .count()
        };
        let count_decision = |decision: ReviewDecision| {
            self.paths
                .iter()
                .filter(|p| p.review_decision == decision)
                .count()
        };
        M5MutationReviewSummary {
            total_paths: self.paths.len(),
            path_count: self.mutation_paths.len(),
            committable_paths: count_published(CommitReadiness::ReviewedApply),
            approval_required_paths: count_published(CommitReadiness::ApprovalRequired),
            preview_only_paths: count_published(CommitReadiness::PreviewOnly),
            blocked_paths: count_published(CommitReadiness::Blocked),
            applied_paths: count_decision(ReviewDecision::Apply),
            require_approval_paths: count_decision(ReviewDecision::RequireApproval),
            flagged_paths: count_decision(ReviewDecision::FlagForReview),
            preview_decision_paths: count_decision(ReviewDecision::PreviewOnly),
            withheld_paths: count_decision(ReviewDecision::Withhold),
            handoff_paths: self
                .paths
                .iter()
                .filter(|p| p.mutation_path.requires_handoff())
                .count(),
            approved_paths: self
                .paths
                .iter()
                .filter(|p| p.approval_state == ApprovalState::Approved)
                .count(),
            bypassed_approval_paths: self
                .paths
                .iter()
                .filter(|p| p.approval_state.is_bypassed_trigger())
                .count(),
            unknown_rollback_paths: self
                .paths
                .iter()
                .filter(|p| p.rollback_class.is_unknown_trigger())
                .count(),
            time_bound_route_paths: self
                .paths
                .iter()
                .filter(|p| p.route_effect.requires_expiry())
                .count(),
            paths_with_narrowing_reasons: self
                .paths
                .iter()
                .filter(|p| !p.narrowing_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — desktop and CLI review
    /// sheets, request/browser/preview/remote/live-resource mutation lanes, companion and
    /// vendor-console handoff surfaces, support exports, and release/audit evidence —
    /// render instead of restating each mutation's review posture by hand.
    pub fn export_projection(&self) -> M5MutationReviewExportProjection {
        let paths = self
            .paths
            .iter()
            .map(|p| M5MutationReviewExportRow {
                path_id: p.path_id.clone(),
                mutation_path: p.mutation_path.as_str().to_owned(),
                actor_authority: p.actor_authority.as_str().to_owned(),
                approval_state: p.approval_state.as_str().to_owned(),
                rollback_class: p.rollback_class.as_str().to_owned(),
                route_effect: p.route_effect.as_str().to_owned(),
                handoff_continuity: p.handoff_continuity.as_str().to_owned(),
                expected_duration: p.expected_duration.as_str().to_owned(),
                fallback_path: p.fallback_path.as_str().to_owned(),
                declared_readiness: p.declared_readiness.as_str().to_owned(),
                published_readiness: p.published_readiness.as_str().to_owned(),
                review_decision: p.review_decision.as_str().to_owned(),
                narrowing_reasons: p
                    .narrowing_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                actor_ref: p.actor_ref.clone(),
                mutation_receipt_ref: p.mutation_receipt_ref.clone(),
                execution_ref: p.execution_ref.clone(),
                crosses_handoff: p.mutation_path.requires_handoff(),
                committable: p.is_committable(),
                summary: format!(
                    "{}: actor {}, approval {}, rollback {}, route {}, handoff {}, duration {}, fallback {}, declared {}, published {} ({})",
                    p.mutation_path.as_str(),
                    p.actor_authority.as_str(),
                    p.approval_state.as_str(),
                    p.rollback_class.as_str(),
                    p.route_effect.as_str(),
                    p.handoff_continuity.as_str(),
                    p.expected_duration.as_str(),
                    p.fallback_path.as_str(),
                    p.declared_readiness.as_str(),
                    p.published_readiness.as_str(),
                    p.review_decision.as_str()
                ),
            })
            .collect();
        M5MutationReviewExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            paths,
            all_paths_gate_consistent: self.all_paths_gate_consistent(),
            committable_count: self.committable_paths().count(),
            narrowed_count: self.narrowed_paths().count(),
            withheld_count: self.withheld_paths().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5MutationReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<MutationPath> = self.mutation_paths.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_paths = BTreeSet::new();
        for row in &self.paths {
            if !seen_ids.insert(row.path_id.clone()) {
                violations.push(M5MutationReviewViolation::DuplicatePathId {
                    path_id: row.path_id.clone(),
                });
            }
            if !seen_paths.insert(row.mutation_path) {
                violations.push(M5MutationReviewViolation::DuplicatePathRow {
                    path: row.mutation_path.as_str(),
                });
            }
            if !claimed.contains(&row.mutation_path) {
                violations.push(M5MutationReviewViolation::UnclaimedPathRow {
                    path_id: row.path_id.clone(),
                    path: row.mutation_path.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed path must carry its own row, so a path never inherits a reviewed
        // apply from an adjacent one.
        for &path in &self.mutation_paths {
            if !seen_paths.contains(&path) {
                violations.push(M5MutationReviewViolation::MissingPathRow {
                    path: path.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5MutationReviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5MutationReviewViolation>) {
        if self.schema_version != M5_MUTATION_REVIEW_SCHEMA_VERSION {
            violations.push(M5MutationReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_MUTATION_REVIEW_RECORD_KIND {
            violations.push(M5MutationReviewViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MutationReviewViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "mutation_paths",
                self.mutation_paths == MutationPath::ALL.to_vec(),
            ),
            (
                "commit_readinesses",
                self.commit_readinesses == CommitReadiness::ALL.to_vec(),
            ),
            (
                "actor_authority_states",
                self.actor_authority_states == ActorAuthorityState::ALL.to_vec(),
            ),
            (
                "approval_states",
                self.approval_states == ApprovalState::ALL.to_vec(),
            ),
            (
                "rollback_classes",
                self.rollback_classes == RollbackClass::ALL.to_vec(),
            ),
            (
                "route_effects",
                self.route_effects == RouteEffect::ALL.to_vec(),
            ),
            (
                "handoff_continuities",
                self.handoff_continuities == HandoffContinuity::ALL.to_vec(),
            ),
            (
                "duration_classes",
                self.duration_classes == DurationClass::ALL.to_vec(),
            ),
            (
                "fallback_paths",
                self.fallback_paths == FallbackPath::ALL.to_vec(),
            ),
            (
                "narrowing_reasons",
                self.narrowing_reasons == MutationNarrowingReason::ALL.to_vec(),
            ),
            (
                "review_decisions",
                self.review_decisions == ReviewDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5MutationReviewViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(
        &self,
        row: &MutationPathReviewRow,
        violations: &mut Vec<M5MutationReviewViolation>,
    ) {
        for (field, value) in [
            ("path_id", &row.path_id),
            ("actor_ref", &row.actor_ref),
            ("target_context_ref", &row.target_context_ref),
            ("review_sheet_ref", &row.review_sheet_ref),
            ("mutation_receipt_ref", &row.mutation_receipt_ref),
            ("execution_ref", &row.execution_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5MutationReviewViolation::EmptyField {
                    id: row.path_id.clone(),
                    field_name: field,
                });
            }
        }

        // A granted or pending approval must carry its ticket ref so the reviewed sheet
        // can prove the approval the user saw.
        if row.approval_state.requires_ticket() && row.approval_ticket_ref.trim().is_empty() {
            violations.push(M5MutationReviewViolation::EmptyField {
                id: row.path_id.clone(),
                field_name: "approval_ticket_ref",
            });
        }

        // A compensable or provider-managed rollback must carry its plan ref so a recovery
        // path is never omitted.
        if row.rollback_class.requires_plan() && row.rollback_plan_ref.trim().is_empty() {
            violations.push(M5MutationReviewViolation::EmptyField {
                id: row.path_id.clone(),
                field_name: "rollback_plan_ref",
            });
        }

        // A time-bound route must carry its expiry ref so the route effect stays bounded.
        if row.route_effect.requires_expiry() && row.route_expiry_ref.trim().is_empty() {
            violations.push(M5MutationReviewViolation::EmptyField {
                id: row.path_id.clone(),
                field_name: "route_expiry_ref",
            });
        }

        // A path that crosses a browser/companion or vendor-console handoff boundary must
        // declare a handoff continuity, so the handoff is reviewed instead of bypassing
        // the canonical review model.
        if row.mutation_path.requires_handoff()
            && row.handoff_continuity == HandoffContinuity::NotHandoff
        {
            violations.push(M5MutationReviewViolation::HandoffContextMissing {
                path_id: row.path_id.clone(),
                path: row.mutation_path.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.narrowing_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5MutationReviewViolation::DuplicateNarrowingReason {
                    path_id: row.path_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published readiness must equal the gate's recomputed ceiling, so an
        // inherited, unapproved, unrecoverable, or severed mutation can never read as a
        // reviewed apply.
        let effective = row.effective_readiness();
        if row.published_readiness != effective {
            violations.push(M5MutationReviewViolation::OverstatedReadiness {
                path_id: row.path_id.clone(),
                published: row.published_readiness.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.review_decision != required {
            violations.push(M5MutationReviewViolation::DecisionMismatch {
                path_id: row.path_id.clone(),
                declared: row.review_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded narrowing reasons must equal the reasons recomputed from the
        // observed states, so a narrowing can never be asserted or hidden by hand.
        let computed = row.computed_narrowing_reasons();
        if row.narrowing_reasons != computed {
            violations.push(M5MutationReviewViolation::NarrowingReasonsMismatch {
                path_id: row.path_id.clone(),
            });
        }

        // A non-applying decision must offer a real fallback or open-in-provider path, so
        // a narrowed or withheld mutation never drops its fallback semantics.
        if row.review_decision.is_narrowed() && !row.fallback_path.is_offered() {
            violations.push(M5MutationReviewViolation::MissingFallback {
                path_id: row.path_id.clone(),
            });
        }

        // A committable path must be genuinely clean: a reviewed-apply ceiling on every
        // input, a reviewed-apply capability floor, no narrowing reason, and no flaggable
        // handoff. This is the non-inheritance guardrail.
        if row.is_committable()
            && (row.actor_authority.readiness_ceiling() != CommitReadiness::ReviewedApply
                || row.approval_state.readiness_ceiling() != CommitReadiness::ReviewedApply
                || row.rollback_class.readiness_ceiling() != CommitReadiness::ReviewedApply
                || row.route_effect.readiness_ceiling() != CommitReadiness::ReviewedApply
                || row.handoff_continuity.readiness_ceiling() != CommitReadiness::ReviewedApply
                || row.capability_floor() != CommitReadiness::ReviewedApply
                || row.handoff_continuity.is_flaggable()
                || !row.narrowing_reasons.is_empty())
        {
            violations.push(M5MutationReviewViolation::CommittablePathNotClean {
                path_id: row.path_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 mutation-and-handoff review packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5MutationReviewViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A mutation-path review id appears more than once.
    DuplicatePathId {
        /// Duplicate path id.
        path_id: String,
    },
    /// A claimed path carries more than one row.
    DuplicatePathRow {
        /// Path token.
        path: &'static str,
    },
    /// A claimed path has no row.
    MissingPathRow {
        /// Path token.
        path: &'static str,
    },
    /// A row covers a path the packet does not claim.
    UnclaimedPathRow {
        /// Row id.
        path_id: String,
        /// Path token.
        path: &'static str,
    },
    /// A handoff-crossing path does not declare a handoff continuity.
    HandoffContextMissing {
        /// Row id.
        path_id: String,
        /// Path token.
        path: &'static str,
    },
    /// A row lists a narrowing reason more than once.
    DuplicateNarrowingReason {
        /// Row id.
        path_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A path publishes a readiness beyond what its evidence supports.
    OverstatedReadiness {
        /// Row id.
        path_id: String,
        /// Published readiness token.
        published: &'static str,
        /// Computed effective readiness token.
        computed: &'static str,
    },
    /// A path's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        path_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A path's narrowing reasons disagree with the recomputed reasons.
    NarrowingReasonsMismatch {
        /// Row id.
        path_id: String,
    },
    /// A narrowed or withheld path offers no fallback or open-in-provider path.
    MissingFallback {
        /// Row id.
        path_id: String,
    },
    /// A committable path still carries a narrowing reason or a non-clean state.
    CommittablePathNotClean {
        /// Row id.
        path_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5MutationReviewViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicatePathId { path_id } => {
                write!(f, "duplicate path id {path_id}")
            }
            Self::DuplicatePathRow { path } => {
                write!(f, "duplicate row for path {path}")
            }
            Self::MissingPathRow { path } => {
                write!(f, "missing row for claimed path {path}")
            }
            Self::UnclaimedPathRow { path_id, path } => {
                write!(f, "row {path_id} covers unclaimed path {path}")
            }
            Self::HandoffContextMissing { path_id, path } => {
                write!(
                    f,
                    "row {path_id} crosses handoff path {path} but declares no handoff continuity"
                )
            }
            Self::DuplicateNarrowingReason { path_id, reason } => {
                write!(f, "row {path_id} repeats narrowing reason {reason}")
            }
            Self::OverstatedReadiness {
                path_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {path_id} publishes readiness {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                path_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {path_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::NarrowingReasonsMismatch { path_id } => {
                write!(f, "row {path_id} narrowing reasons disagree with the gate")
            }
            Self::MissingFallback { path_id } => {
                write!(
                    f,
                    "row {path_id} is narrowed or withheld but offers no fallback path"
                )
            }
            Self::CommittablePathNotClean { path_id } => {
                write!(
                    f,
                    "row {path_id} is committable but carries a narrowing reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5MutationReviewViolation {}

/// Loads the embedded M5 mutation-and-handoff review matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5MutationReviewMatrix`].
pub fn current_m5_mutation_review_matrix() -> Result<M5MutationReviewMatrix, serde_json::Error> {
    serde_json::from_str(M5_MUTATION_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
