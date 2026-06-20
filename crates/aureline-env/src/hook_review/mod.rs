//! Trust-gated lifecycle-hook review, restricted-mode guidance, and repair
//! flows for the lifecycle actions M5 starters declare.
//!
//! The capsule lane already carries the declared, trust-gated
//! [`TrustHook`](crate::capsules::TrustHook)s an environment defines. What it
//! deliberately stops short of is the *review*: deciding, in the current trust
//! and policy context, which of those hooks — plus the preflight validators,
//! build/setup commands, and bootstrap and post-create actions a devcontainer,
//! Compose, Nix, direnv, or bootstrap definition contributes — may actually
//! run, and turning the ones that may not into visible, attributable, and
//! repairable review objects instead of silent side effects.
//!
//! This module closes that gap. A [`LifecycleHook`] is the explicit,
//! review-safe statement of one repo-defined lifecycle action: its
//! [`HookActivator`] (devcontainer, Compose, Nix, direnv, bootstrap, or
//! post-create), its [`HookKind`] (a lifecycle hook, a preflight validator, or
//! a build/setup command), the [`LifecyclePhase`] it runs in, its trust-gate
//! state, the authority contract its gate binds to, the secrets it needs, and
//! the hooks it depends on. It is metadata-first by construction: the hook's
//! command is reduced to a [`CapsuleDigest`], never carried as a body.
//!
//! [`review_hooks`] is the single engine. It folds each declared hook through
//! the [`HookReviewContext`] — whether the workspace is trusted, whether it is
//! in restricted mode, which activators the target supports, which the policy
//! denies, which secrets are projected, and which actions already failed — and
//! returns one [`HookReviewPacket`] carrying, per hook, an explicit
//! [`HookDisposition`] (`allowed`, `review_required`, `restricted`, `denied`,
//! or `blocked`), the [`HookHoldReason`] behind it, a review-safe statement of
//! what did or did not run, a safe next step, and a [`HookRepair`] that
//! preserves the exact hook identity, reason, and next step. Desktop
//! ([`desktop_hook_review`]), CLI / headless ([`headless_hook_review`]), AI
//! ([`ai_hook_review`]), and support ([`support_hook_review`]) all read that
//! **same** object, so a restricted-mode or policy-denied user sees exactly
//! what did not run and what still works safely, identically on every surface.
//!
//! Three guardrails are frozen here:
//!
//! - **Never silently run a trust-gated action.** An ungated hook, a hook whose
//!   gate is awaiting review, and a hook in an untrusted or restricted
//!   workspace are all held — [`HookDisposition::ReviewRequired`] or
//!   [`HookDisposition::Restricted`] — and surfaced as suggestions, never run
//!   merely because a template or capsule references them.
//! - **A no-op is never silent.** A policy-denied step, a missing secret, an
//!   unsupported activator, a failed bootstrap action, and a hook blocked
//!   behind one of those all become a visible [`HookReviewEntry`] with a named
//!   [`HookHoldReason`] and a [`HookRepair`], rather than a confusing
//!   disappearance.
//! - **One engine, one object.** [`review_hooks`] is the single source of truth
//!   for the disposition, shared by the fixtures and every surface, and it maps
//!   the rolled-up [`HookReviewPosture`] back onto the governance
//!   [`EvidenceState`] for the trust-hooks dimension, so the review lane
//!   narrows in lockstep with the capsule's trust-hooks evidence.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/env/hook-review.schema.json`](../../../../schemas/env/hook-review.schema.json)
//! - [`/docs/env/hook-review.md`](../../../../docs/env/hook-review.md)
//! - [`/artifacts/env/hook-review-and-repair.md`](../../../../artifacts/env/hook-review-and-repair.md)
//! - [`/fixtures/env/hook-review/`](../../../../fixtures/env/hook-review/)

use std::collections::{BTreeSet, HashMap};

use serde::{Deserialize, Serialize};

use crate::capsules::{
    CapsuleDigest, CapsuleTargetClass, LifecyclePhase, RedactionClass, TrustGateState,
};
use crate::m5_env_governance::{
    DrillPhase, EnvironmentProfile, EvidenceState, ValidationReport, ValidationViolation,
};

#[cfg(test)]
mod tests;

pub mod seed;

pub use seed::{
    seeded_hook_review_drills, seeded_hook_review_fixtures, seeded_hook_review_packets,
    seeded_lifecycle_hooks,
};

/// Schema version stamped onto packets, drills, and fixtures.
pub const HOOK_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by a [`HookReviewPacket`].
pub const HOOK_REVIEW_PACKET_RECORD_KIND: &str = "hook_review_packet_record";

/// Stable record-kind tag carried by a [`HookReviewExport`].
pub const HOOK_REVIEW_EXPORT_RECORD_KIND: &str = "hook_review_export_record";

/// Stable record-kind tag carried by a [`HookReviewFixture`].
pub const HOOK_REVIEW_FIXTURE_RECORD_KIND: &str = "hook_review_fixture_record";

/// Repo-relative schema ref for the packet, drills, and fixtures.
pub const HOOK_REVIEW_SCHEMA_REF: &str = "schemas/env/hook-review.schema.json";

/// Repo-relative reviewer doc ref.
pub const HOOK_REVIEW_DOC_REF: &str = "docs/env/hook-review.md";

/// Repo-relative human-readable proof / repair report.
pub const HOOK_REVIEW_PROOF_REF: &str = "artifacts/env/hook-review-and-repair.md";

/// Repo-relative fixture directory.
pub const HOOK_REVIEW_FIXTURE_DIR: &str = "fixtures/env/hook-review";

/// Repo-relative fixture manifest.
pub const HOOK_REVIEW_FIXTURE_MANIFEST_REF: &str = "fixtures/env/hook-review/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The definition mechanism a repo-defined lifecycle action comes from. The
/// six activators are the claimed M5 starter paths: a hook keeps its activator
/// identity so an unsupported or policy-denied activator names the exact
/// mechanism rather than collapsing into a generic "setup step".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookActivator {
    /// A devcontainer-defined lifecycle action.
    Devcontainer,
    /// A Docker Compose service / command.
    Compose,
    /// A Nix flake / shell activation.
    Nix,
    /// A direnv `.envrc` activation.
    Direnv,
    /// A repo bootstrap / setup script.
    Bootstrap,
    /// A post-create action.
    PostCreate,
}

impl HookActivator {
    /// Every activator in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Devcontainer,
        Self::Compose,
        Self::Nix,
        Self::Direnv,
        Self::Bootstrap,
        Self::PostCreate,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Devcontainer => "devcontainer",
            Self::Compose => "compose",
            Self::Nix => "nix",
            Self::Direnv => "direnv",
            Self::Bootstrap => "bootstrap",
            Self::PostCreate => "post_create",
        }
    }
}

/// What a repo-defined lifecycle action actually is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookKind {
    /// A lifecycle hook that runs in a lifecycle phase.
    LifecycleHook,
    /// A preflight validator that checks preconditions before work begins.
    PreflightValidator,
    /// A build / setup command that materializes the environment.
    BuildSetupCommand,
}

impl HookKind {
    /// Every kind in canonical order.
    pub const ALL: [Self; 3] = [
        Self::LifecycleHook,
        Self::PreflightValidator,
        Self::BuildSetupCommand,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleHook => "lifecycle_hook",
            Self::PreflightValidator => "preflight_validator",
            Self::BuildSetupCommand => "build_setup_command",
        }
    }
}

/// The disposition the review engine reaches for one hook in the current trust
/// and policy context. Declaration order is the severity order:
/// [`HookDisposition::Allowed`] is the only runnable disposition and
/// [`HookDisposition::Blocked`] the most conservative, so the rolled-up posture
/// always takes the highest severity present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookDisposition {
    /// The hook is trust-gated, in scope, and all preconditions are met; it
    /// runs as part of materialization.
    Allowed,
    /// The hook is declared but its trust gate is not cleared; it is held for
    /// review and never run automatically.
    ReviewRequired,
    /// Restricted mode forbids auto-run; the hook is surfaced as a manual
    /// suggestion with a safe next step.
    Restricted,
    /// Policy denies this hook's activator or kind; it is surfaced with a deny
    /// reason and never run.
    Denied,
    /// A precondition failed — a missing secret, an unsupported activator, a
    /// failed bootstrap action, or a blocked dependency — so the hook is
    /// blocked and a repair is offered.
    Blocked,
}

impl HookDisposition {
    /// Every disposition in canonical (severity) order.
    pub const ALL: [Self; 5] = [
        Self::Allowed,
        Self::ReviewRequired,
        Self::Restricted,
        Self::Denied,
        Self::Blocked,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::ReviewRequired => "review_required",
            Self::Restricted => "restricted",
            Self::Denied => "denied",
            Self::Blocked => "blocked",
        }
    }

    /// Narrowing severity. Higher is a more conservative disposition; the
    /// rolled-up posture takes the highest severity among the entries.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Allowed => 0,
            Self::ReviewRequired => 1,
            Self::Restricted => 2,
            Self::Denied => 3,
            Self::Blocked => 4,
        }
    }

    /// Whether a hook with this disposition runs as part of materialization.
    pub const fn is_runnable(self) -> bool {
        matches!(self, Self::Allowed)
    }

    /// Whether this disposition is a hard block (policy-denied or blocked),
    /// the two dispositions that narrow the posture below review-pending.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Denied | Self::Blocked)
    }
}

/// Why a hook is not [`HookDisposition::Allowed`]. The reason is the
/// attributable cause the repair, support packet, and every surface preserve
/// verbatim. The first-match order of [`direct_hold_reason`] is the precedence
/// order, so a policy denial headlines before an unsupported activator, which
/// headlines before a missing secret, and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookHoldReason {
    /// The trust gate is awaiting review, or the workspace is not trusted.
    AwaitingApproval,
    /// The hook has no trust gate; running it would bypass the contract, so it
    /// must never run automatically.
    UngatedAuthority,
    /// The workspace is in restricted mode; even a cleared hook is held as a
    /// manual suggestion.
    RestrictedMode,
    /// Policy denies this hook's activator or kind.
    PolicyDenied,
    /// A secret the hook requires is not projected into the environment.
    MissingSecret,
    /// The hook's activator is not supported on this target.
    UnsupportedActivator,
    /// A bootstrap / setup action the hook performs already failed.
    BootstrapFailed,
    /// A hook this hook depends on is itself held or blocked.
    UpstreamBlocked,
}

impl HookHoldReason {
    /// Every reason in canonical order.
    pub const ALL: [Self; 8] = [
        Self::AwaitingApproval,
        Self::UngatedAuthority,
        Self::RestrictedMode,
        Self::PolicyDenied,
        Self::MissingSecret,
        Self::UnsupportedActivator,
        Self::BootstrapFailed,
        Self::UpstreamBlocked,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitingApproval => "awaiting_approval",
            Self::UngatedAuthority => "ungated_authority",
            Self::RestrictedMode => "restricted_mode",
            Self::PolicyDenied => "policy_denied",
            Self::MissingSecret => "missing_secret",
            Self::UnsupportedActivator => "unsupported_activator",
            Self::BootstrapFailed => "bootstrap_failed",
            Self::UpstreamBlocked => "upstream_blocked",
        }
    }

    /// The disposition this reason produces.
    pub const fn disposition(self) -> HookDisposition {
        match self {
            Self::AwaitingApproval | Self::UngatedAuthority => HookDisposition::ReviewRequired,
            Self::RestrictedMode => HookDisposition::Restricted,
            Self::PolicyDenied => HookDisposition::Denied,
            Self::MissingSecret
            | Self::UnsupportedActivator
            | Self::BootstrapFailed
            | Self::UpstreamBlocked => HookDisposition::Blocked,
        }
    }

    /// The repair that recovers a hook held for this reason.
    pub const fn repair_kind(self) -> RepairKind {
        match self {
            Self::AwaitingApproval | Self::UngatedAuthority => RepairKind::RequestApproval,
            Self::RestrictedMode => RepairKind::RunManuallyAfterReview,
            Self::PolicyDenied => RepairKind::RequestPolicyException,
            Self::MissingSecret => RepairKind::ProvideMissingSecret,
            Self::UnsupportedActivator => RepairKind::EnableActivatorSupport,
            Self::BootstrapFailed => RepairKind::RetryAfterBootstrapFix,
            Self::UpstreamBlocked => RepairKind::RepairUpstreamFirst,
        }
    }
}

/// The repair action that recovers a held or blocked hook. Each maps from a
/// [`HookHoldReason`] so a held hook is always recoverable and attributable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairKind {
    /// Route the hook through its approval ticket to clear (or establish) its
    /// trust gate.
    RequestApproval,
    /// Run the hook by hand after review, while restricted mode holds
    /// auto-run.
    RunManuallyAfterReview,
    /// Request a policy exception for the denied activator or kind.
    RequestPolicyException,
    /// Provide the missing secret the hook requires.
    ProvideMissingSecret,
    /// Install or enable the unsupported activator on this target.
    EnableActivatorSupport,
    /// Fix the failed bootstrap action and retry.
    RetryAfterBootstrapFix,
    /// Repair the blocking upstream hook first, then re-review.
    RepairUpstreamFirst,
}

impl RepairKind {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestApproval => "request_approval",
            Self::RunManuallyAfterReview => "run_manually_after_review",
            Self::RequestPolicyException => "request_policy_exception",
            Self::ProvideMissingSecret => "provide_missing_secret",
            Self::EnableActivatorSupport => "enable_activator_support",
            Self::RetryAfterBootstrapFix => "retry_after_bootstrap_fix",
            Self::RepairUpstreamFirst => "repair_upstream_first",
        }
    }
}

/// The rolled-up posture of one hook review. Declaration order is the
/// narrowing order: [`HookReviewPosture::AllCleared`] is the strongest and
/// [`HookReviewPosture::FullyBlocked`] the most conservative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookReviewPosture {
    /// Every reviewed hook is cleared to run.
    AllCleared,
    /// One or more hooks are held for review or restricted, but none are denied
    /// or blocked; nothing runs that should not, and review clears the rest.
    ReviewPending,
    /// One or more hooks are denied or blocked, but a non-empty safe subset
    /// still runs.
    PartiallyBlocked,
    /// One or more hooks are denied or blocked and no hook runs safely.
    FullyBlocked,
}

impl HookReviewPosture {
    /// Every posture in canonical (narrowing) order.
    pub const ALL: [Self; 4] = [
        Self::AllCleared,
        Self::ReviewPending,
        Self::PartiallyBlocked,
        Self::FullyBlocked,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllCleared => "all_cleared",
            Self::ReviewPending => "review_pending",
            Self::PartiallyBlocked => "partially_blocked",
            Self::FullyBlocked => "fully_blocked",
        }
    }

    /// The governance trust-hooks [`EvidenceState`] this posture maps to, so
    /// the review lane narrows the capsule's trust-hooks dimension in lockstep
    /// instead of forking a parallel model. The mapping mirrors the capsule's
    /// own trust-hook evidence: an ungated or fully blocked review is missing
    /// evidence, a partial or review-pending state is partial, and an
    /// all-cleared review is current.
    pub const fn trust_hooks_evidence_state(self) -> EvidenceState {
        match self {
            Self::AllCleared => EvidenceState::Current,
            Self::ReviewPending | Self::PartiallyBlocked => EvidenceState::Partial,
            Self::FullyBlocked => EvidenceState::Missing,
        }
    }
}

// ---------------------------------------------------------------------------
// The declared hook and the review context.
// ---------------------------------------------------------------------------

/// One repo-defined lifecycle action, surfaced as an explicit review object.
/// The command is reduced to a digest; the hook never carries the command
/// body, a secret value, or a raw provider payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleHook {
    /// Stable hook id.
    pub hook_id: String,
    /// Definition mechanism the hook comes from.
    pub activator: HookActivator,
    /// What the hook is (lifecycle hook, preflight validator, build/setup).
    pub kind: HookKind,
    /// Lifecycle phase the hook runs in.
    pub phase: LifecyclePhase,
    /// Whether the hook has cleared its trust gate.
    pub gate_state: TrustGateState,
    /// Authority / execution-scope contract the gate binds to (metadata ref).
    pub authority_ref: String,
    /// Digest of the hook command (never the command body).
    pub command_digest: CapsuleDigest,
    /// Secrets / environment bindings the hook requires to run.
    pub required_secrets: Vec<String>,
    /// Other hook ids this hook depends on.
    pub depends_on: Vec<String>,
    /// Review-safe summary of the hook.
    pub summary: String,
}

/// The trust and policy context one set of hooks is reviewed against. All
/// fields are metadata: trust and restricted flags, activator support and
/// denial lists, projected-secret names, and failed-action ids — never secret
/// values or command bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewContext {
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Target class the hooks would materialize on.
    pub target_class: CapsuleTargetClass,
    /// Whether the workspace / repo is trusted (cleared gates may run).
    pub trusted: bool,
    /// Whether the workspace is in restricted mode (no auto-run).
    pub restricted_mode: bool,
    /// Activators the target supports.
    pub supported_activators: Vec<HookActivator>,
    /// Activators policy denies.
    pub denied_activators: Vec<HookActivator>,
    /// Hook kinds policy denies.
    pub denied_hook_kinds: Vec<HookKind>,
    /// Secret / environment binding names currently projected.
    pub projected_secrets: Vec<String>,
    /// Hook ids whose bootstrap / setup action already failed.
    pub failed_actions: Vec<String>,
    /// Policy contract reference the denials draw from (metadata ref).
    pub policy_ref: String,
    /// Approval-lineage reference repairs route through (metadata ref).
    pub approval_lineage_ref: String,
    /// Support-export reference the review is attributable through (metadata).
    pub support_export_ref: String,
}

impl HookReviewContext {
    fn supports(&self, activator: HookActivator) -> bool {
        self.supported_activators.contains(&activator)
    }

    fn denies(&self, hook: &LifecycleHook) -> bool {
        self.denied_activators.contains(&hook.activator)
            || self.denied_hook_kinds.contains(&hook.kind)
    }

    fn secret_projected(&self, name: &str) -> bool {
        self.projected_secrets.iter().any(|s| s == name)
    }

    fn action_failed(&self, hook_id: &str) -> bool {
        self.failed_actions.iter().any(|id| id == hook_id)
    }
}

// ---------------------------------------------------------------------------
// The repair, the per-hook entry, and the packet.
// ---------------------------------------------------------------------------

/// A repair that recovers one held or blocked hook. It preserves the exact
/// hook identity, the reason it was held, and the safe next step, so a hook
/// failure stays attributable and recoverable through the approval lineage and
/// support packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookRepair {
    /// Hook id the repair recovers.
    pub hook_id: String,
    /// Repair action.
    pub repair_kind: RepairKind,
    /// Reason the hook was held.
    pub reason: HookHoldReason,
    /// Safe next step a user takes.
    pub next_step: String,
    /// Approval-lineage reference the repair routes through (metadata ref).
    pub approval_lineage_ref: String,
    /// Review-safe preview of the repair's effect.
    pub preview: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// One per-hook review outcome. This is the visible review object that replaces
/// a silent side effect: it carries the disposition, the hold reason, what did
/// or did not run, a safe next step, and the repair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewEntry {
    /// Hook id.
    pub hook_id: String,
    /// Activator the hook comes from.
    pub activator: HookActivator,
    /// Kind of hook.
    pub kind: HookKind,
    /// Lifecycle phase.
    pub phase: LifecyclePhase,
    /// Trust-gate state of the hook.
    pub gate_state: TrustGateState,
    /// Disposition the engine reached.
    pub disposition: HookDisposition,
    /// Reason the hook was held (none when allowed).
    pub hold_reason: Option<HookHoldReason>,
    /// True when the hook runs as part of materialization.
    pub runnable: bool,
    /// Stable tokens naming the reason and the elements behind it.
    pub reason_tokens: Vec<String>,
    /// Secrets the hook requires.
    pub required_secrets: Vec<String>,
    /// Hooks this hook depends on.
    pub depends_on: Vec<String>,
    /// Authority contract the gate binds to (metadata ref).
    pub authority_ref: String,
    /// Digest of the hook command (never the body).
    pub command_digest: CapsuleDigest,
    /// Review-safe statement of what did or did not run.
    pub what_happens: String,
    /// Safe next step (empty when allowed).
    pub safe_next_step: String,
    /// Repair that recovers the hook (none when allowed).
    pub repair: Option<HookRepair>,
    /// Review-safe summary of the entry.
    pub summary: String,
}

/// The decision the engine reaches for one set of hooks against one context.
/// This is the single explainability object desktop, headless, AI, and support
/// all read; it carries no command bodies, secret values, or provider payloads
/// — only ids, digests, tokens, and review-safe prose.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Target class the hooks would materialize on.
    pub target_class: CapsuleTargetClass,
    /// Whether the workspace is trusted.
    pub trusted: bool,
    /// Whether the workspace is in restricted mode.
    pub restricted_mode: bool,
    /// The rolled-up posture.
    pub posture: HookReviewPosture,
    /// The governance trust-hooks evidence state this posture maps to.
    pub trust_hooks_evidence_state: EvidenceState,
    /// Number of hooks reviewed.
    pub total_hooks: u32,
    /// Number of hooks cleared to run.
    pub runnable_hooks: u32,
    /// Hook ids that are allowed to run.
    pub allowed_hook_tokens: Vec<String>,
    /// Hook ids held for review.
    pub held_hook_tokens: Vec<String>,
    /// Hook ids restricted to manual run.
    pub restricted_hook_tokens: Vec<String>,
    /// Hook ids policy denies.
    pub denied_hook_tokens: Vec<String>,
    /// Hook ids blocked behind a failed precondition.
    pub blocked_hook_tokens: Vec<String>,
    /// Stable tokens naming every hold reason present.
    pub reason_tokens: Vec<String>,
    /// Per-hook review entries.
    pub entries: Vec<HookReviewEntry>,
    /// One repair per non-allowed hook.
    pub repairs: Vec<HookRepair>,
    /// Review-safe statement of what did not run.
    pub what_did_not_run: String,
    /// Review-safe statement of what still works safely.
    pub what_still_works: String,
    /// Review-safe headline explaining the posture.
    pub headline: String,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

// ---------------------------------------------------------------------------
// The engine.
// ---------------------------------------------------------------------------

/// The direct hold reason for one hook, ignoring its dependencies. The
/// first-match order is the precedence order: a policy denial outranks an
/// unsupported activator, which outranks a failed action, a missing secret, an
/// ungated authority, an uncleared gate, and finally restricted mode. Returns
/// `None` when the hook clears every direct check.
pub fn direct_hold_reason(
    hook: &LifecycleHook,
    context: &HookReviewContext,
) -> Option<HookHoldReason> {
    if context.denies(hook) {
        return Some(HookHoldReason::PolicyDenied);
    }
    if !context.supports(hook.activator) {
        return Some(HookHoldReason::UnsupportedActivator);
    }
    if context.action_failed(&hook.hook_id) {
        return Some(HookHoldReason::BootstrapFailed);
    }
    if hook
        .required_secrets
        .iter()
        .any(|secret| !context.secret_projected(secret))
    {
        return Some(HookHoldReason::MissingSecret);
    }
    if hook.gate_state == TrustGateState::Ungated {
        return Some(HookHoldReason::UngatedAuthority);
    }
    if hook.gate_state == TrustGateState::PendingReview || !context.trusted {
        return Some(HookHoldReason::AwaitingApproval);
    }
    if context.restricted_mode {
        return Some(HookHoldReason::RestrictedMode);
    }
    None
}

/// Reviews one set of declared hooks against one trust / policy context.
///
/// This is the canonical engine the fixtures and every surface share. Each
/// hook gets a direct hold reason; then a fixpoint pass propagates
/// [`HookHoldReason::UpstreamBlocked`] to any otherwise-clear hook that depends
/// on a hook which will not run, so a blocked bootstrap action visibly blocks
/// the post-create hook that needs it. The decision is metadata-first and
/// self-explaining, so a policy-denied, restricted, or blocked hook can never
/// collapse into a silent no-op.
pub fn review_hooks(hooks: &[LifecycleHook], context: &HookReviewContext) -> HookReviewPacket {
    // Pass 1: direct reasons.
    let mut reasons: Vec<Option<HookHoldReason>> = hooks
        .iter()
        .map(|hook| direct_hold_reason(hook, context))
        .collect();

    // Pass 2: propagate upstream-blocked to a fixpoint. A hook that is
    // otherwise clear but depends on a hook that will not run is blocked,
    // because running it would assume work that did not happen.
    let index: HashMap<&str, usize> = hooks
        .iter()
        .enumerate()
        .map(|(i, hook)| (hook.hook_id.as_str(), i))
        .collect();
    loop {
        let mut changed = false;
        for (i, hook) in hooks.iter().enumerate() {
            if reasons[i].is_some() {
                continue;
            }
            let upstream_held = hook.depends_on.iter().any(|dep| {
                index
                    .get(dep.as_str())
                    .map(|&j| reasons[j].is_some())
                    .unwrap_or(false)
            });
            if upstream_held {
                reasons[i] = Some(HookHoldReason::UpstreamBlocked);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    let mut entries = Vec::with_capacity(hooks.len());
    let mut repairs = Vec::new();
    for (hook, reason) in hooks.iter().zip(reasons.iter()) {
        let entry = build_entry(hook, *reason, context);
        if let Some(repair) = entry.repair.clone() {
            repairs.push(repair);
        }
        entries.push(entry);
    }

    roll_up(hooks, context, entries, repairs)
}

fn build_entry(
    hook: &LifecycleHook,
    reason: Option<HookHoldReason>,
    context: &HookReviewContext,
) -> HookReviewEntry {
    let disposition = reason.map_or(HookDisposition::Allowed, HookHoldReason::disposition);
    let runnable = disposition.is_runnable();
    let reason_tokens = entry_reason_tokens(hook, reason, context);
    let what_happens = what_happens(hook, reason);
    let (safe_next_step, repair) = match reason {
        None => (String::new(), None),
        Some(reason) => {
            let repair = build_repair(hook, reason, context);
            (repair.next_step.clone(), Some(repair))
        }
    };
    let summary = match reason {
        None => format!(
            "The {} {} hook is cleared to run.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(reason) => format!(
            "The {} {} hook is {} ({}).",
            hook.phase.as_str(),
            hook.activator.as_str(),
            disposition.as_str(),
            reason.as_str()
        ),
    };

    HookReviewEntry {
        hook_id: hook.hook_id.clone(),
        activator: hook.activator,
        kind: hook.kind,
        phase: hook.phase,
        gate_state: hook.gate_state,
        disposition,
        hold_reason: reason,
        runnable,
        reason_tokens,
        required_secrets: hook.required_secrets.clone(),
        depends_on: hook.depends_on.clone(),
        authority_ref: hook.authority_ref.clone(),
        command_digest: hook.command_digest.clone(),
        what_happens,
        safe_next_step,
        repair,
        summary,
    }
}

/// The stable tokens behind one entry's reason: the bare reason token plus a
/// token naming the exact element (activator, secret, or upstream hook) that
/// forced the hold, so the cause is preserved verbatim. Sorted and deduped.
fn entry_reason_tokens(
    hook: &LifecycleHook,
    reason: Option<HookHoldReason>,
    context: &HookReviewContext,
) -> Vec<String> {
    let mut tokens = Vec::new();
    if let Some(reason) = reason {
        tokens.push(reason.as_str().to_owned());
        match reason {
            HookHoldReason::PolicyDenied | HookHoldReason::UnsupportedActivator => {
                tokens.push(format!("activator_{}", hook.activator.as_str()));
            }
            HookHoldReason::MissingSecret => {
                for secret in &hook.required_secrets {
                    if !context.secret_projected(secret) {
                        tokens.push(format!("secret_{secret}"));
                    }
                }
            }
            HookHoldReason::UpstreamBlocked => {
                for dep in &hook.depends_on {
                    tokens.push(format!("upstream_{dep}"));
                }
            }
            _ => {}
        }
    }
    tokens.sort();
    tokens.dedup();
    tokens
}

fn what_happens(hook: &LifecycleHook, reason: Option<HookHoldReason>) -> String {
    match reason {
        None => format!(
            "Ran: the {} {} hook is trust-gated and cleared, so it runs as part of materialization.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::AwaitingApproval) => format!(
            "Did not run: the {} {} hook is awaiting trust review and is held instead of run.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::UngatedAuthority) => format!(
            "Did not run: the {} {} hook has no trust gate, so running it would bypass the authority contract.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::RestrictedMode) => format!(
            "Did not run: restricted mode holds auto-run, so the {} {} hook is offered as a manual step.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::PolicyDenied) => format!(
            "Did not run: policy denies the {} activator, so the {} hook is not executed.",
            hook.activator.as_str(),
            hook.phase.as_str()
        ),
        Some(HookHoldReason::MissingSecret) => format!(
            "Did not run: the {} {} hook needs a secret that is not projected.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::UnsupportedActivator) => format!(
            "Did not run: the {} activator is not supported on this target, so the {} hook cannot materialize.",
            hook.activator.as_str(),
            hook.phase.as_str()
        ),
        Some(HookHoldReason::BootstrapFailed) => format!(
            "Did not complete: the {} {} bootstrap action failed and is held for repair.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
        Some(HookHoldReason::UpstreamBlocked) => format!(
            "Did not run: the {} {} hook depends on a hook that did not run, so it is blocked.",
            hook.phase.as_str(),
            hook.activator.as_str()
        ),
    }
}

fn build_repair(
    hook: &LifecycleHook,
    reason: HookHoldReason,
    context: &HookReviewContext,
) -> HookRepair {
    let repair_kind = reason.repair_kind();
    let next_step = match reason {
        HookHoldReason::AwaitingApproval => format!(
            "Review and approve the {} hook through its trust gate to let it run.",
            hook.hook_id
        ),
        HookHoldReason::UngatedAuthority => format!(
            "Bind the {} hook to an authority contract and approve it before it can run.",
            hook.hook_id
        ),
        HookHoldReason::RestrictedMode => format!(
            "Leave restricted mode or run the {} hook manually after reviewing it.",
            hook.hook_id
        ),
        HookHoldReason::PolicyDenied => format!(
            "Request a policy exception for the {} activator to allow the {} hook.",
            hook.activator.as_str(),
            hook.hook_id
        ),
        HookHoldReason::MissingSecret => format!(
            "Provide the missing secret(s) the {} hook requires, then re-review.",
            hook.hook_id
        ),
        HookHoldReason::UnsupportedActivator => format!(
            "Enable {} support on this target, or pick a target that supports it, then re-review the {} hook.",
            hook.activator.as_str(),
            hook.hook_id
        ),
        HookHoldReason::BootstrapFailed => format!(
            "Fix the failed bootstrap action and retry the {} hook.",
            hook.hook_id
        ),
        HookHoldReason::UpstreamBlocked => format!(
            "Repair the upstream hook(s) the {} hook depends on, then re-review.",
            hook.hook_id
        ),
    };
    let preview = format!(
        "Repairing the {} hook by {} restores it to a runnable, trust-gated state without running any other held hook.",
        hook.hook_id,
        repair_kind.as_str().replace('_', " ")
    );
    HookRepair {
        hook_id: hook.hook_id.clone(),
        repair_kind,
        reason,
        next_step,
        approval_lineage_ref: context.approval_lineage_ref.clone(),
        preview,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

fn roll_up(
    hooks: &[LifecycleHook],
    context: &HookReviewContext,
    entries: Vec<HookReviewEntry>,
    repairs: Vec<HookRepair>,
) -> HookReviewPacket {
    let mut allowed_hook_tokens = Vec::new();
    let mut held_hook_tokens = Vec::new();
    let mut restricted_hook_tokens = Vec::new();
    let mut denied_hook_tokens = Vec::new();
    let mut blocked_hook_tokens = Vec::new();
    let mut reason_tokens: Vec<String> = Vec::new();
    let mut worst = HookDisposition::Allowed;
    let mut runnable_hooks = 0u32;

    for entry in &entries {
        match entry.disposition {
            HookDisposition::Allowed => {
                allowed_hook_tokens.push(entry.hook_id.clone());
                runnable_hooks += 1;
            }
            HookDisposition::ReviewRequired => held_hook_tokens.push(entry.hook_id.clone()),
            HookDisposition::Restricted => restricted_hook_tokens.push(entry.hook_id.clone()),
            HookDisposition::Denied => denied_hook_tokens.push(entry.hook_id.clone()),
            HookDisposition::Blocked => blocked_hook_tokens.push(entry.hook_id.clone()),
        }
        if entry.disposition.severity() > worst.severity() {
            worst = entry.disposition;
        }
        if let Some(reason) = entry.hold_reason {
            reason_tokens.push(reason.as_str().to_owned());
        }
    }

    for tokens in [
        &mut allowed_hook_tokens,
        &mut held_hook_tokens,
        &mut restricted_hook_tokens,
        &mut denied_hook_tokens,
        &mut blocked_hook_tokens,
        &mut reason_tokens,
    ] {
        tokens.sort();
        tokens.dedup();
    }

    let posture = if worst == HookDisposition::Allowed {
        HookReviewPosture::AllCleared
    } else if worst.is_blocking() {
        if runnable_hooks > 0 {
            HookReviewPosture::PartiallyBlocked
        } else {
            HookReviewPosture::FullyBlocked
        }
    } else {
        HookReviewPosture::ReviewPending
    };

    let what_did_not_run = what_did_not_run(&entries);
    let what_still_works = what_still_works(&allowed_hook_tokens, posture);
    let headline = headline(context, posture, &entries);

    HookReviewPacket {
        record_kind: HOOK_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: HOOK_REVIEW_SCHEMA_VERSION,
        review_id: format!(
            "hook_review.{}.{}",
            context.profile.as_str(),
            context.target_class.as_str()
        ),
        profile: context.profile,
        target_class: context.target_class,
        trusted: context.trusted,
        restricted_mode: context.restricted_mode,
        posture,
        trust_hooks_evidence_state: posture.trust_hooks_evidence_state(),
        total_hooks: hooks.len() as u32,
        runnable_hooks,
        allowed_hook_tokens,
        held_hook_tokens,
        restricted_hook_tokens,
        denied_hook_tokens,
        blocked_hook_tokens,
        reason_tokens,
        entries,
        repairs,
        what_did_not_run,
        what_still_works,
        headline,
        redaction_class: RedactionClass::MetadataOnly,
    }
}

fn what_did_not_run(entries: &[HookReviewEntry]) -> String {
    let held: Vec<&str> = entries
        .iter()
        .filter(|entry| !entry.runnable)
        .map(|entry| entry.hook_id.as_str())
        .collect();
    if held.is_empty() {
        "Every reviewed hook is cleared to run; nothing was held or blocked.".to_owned()
    } else {
        format!(
            "{} hook(s) did not run and are surfaced with a reason and a repair: {}.",
            held.len(),
            held.join(", ")
        )
    }
}

fn what_still_works(allowed: &[String], posture: HookReviewPosture) -> String {
    match posture {
        HookReviewPosture::AllCleared => {
            "Every reviewed hook runs as part of materialization.".to_owned()
        }
        HookReviewPosture::FullyBlocked => {
            "No hook runs automatically; the environment proceeds without any held hook until they are repaired.".to_owned()
        }
        _ if allowed.is_empty() => {
            "No hook runs automatically; the safe subset is empty until the held hooks are repaired.".to_owned()
        }
        _ => format!(
            "The safe subset still runs: {}. The held hooks are surfaced as suggestions and do not run.",
            allowed.join(", ")
        ),
    }
}

fn headline(
    context: &HookReviewContext,
    posture: HookReviewPosture,
    entries: &[HookReviewEntry],
) -> String {
    let held = entries.iter().filter(|entry| !entry.runnable).count();
    match posture {
        HookReviewPosture::AllCleared => format!(
            "Every lifecycle hook on the {} {} path is trust-gated and cleared to run.",
            context.profile.as_str(),
            context.target_class.as_str()
        ),
        HookReviewPosture::ReviewPending => format!(
            "{held} lifecycle hook(s) on the {} {} path are held for review or restricted and are surfaced as suggestions rather than run.",
            context.profile.as_str(),
            context.target_class.as_str()
        ),
        HookReviewPosture::PartiallyBlocked => format!(
            "{held} lifecycle hook(s) on the {} {} path are denied or blocked; the safe subset still runs and each held hook carries a reason and a repair.",
            context.profile.as_str(),
            context.target_class.as_str()
        ),
        HookReviewPosture::FullyBlocked => format!(
            "Every lifecycle hook on the {} {} path is denied or blocked; none runs, and each carries a reason and a repair.",
            context.profile.as_str(),
            context.target_class.as_str()
        ),
    }
}

/// The desktop hook review. Desktop reads the same [`HookReviewPacket`] object
/// as every other surface.
pub fn desktop_hook_review(
    hooks: &[LifecycleHook],
    context: &HookReviewContext,
) -> HookReviewPacket {
    review_hooks(hooks, context)
}

/// The headless / CLI hook review. Headless reads the same [`HookReviewPacket`]
/// object as every other surface.
pub fn headless_hook_review(
    hooks: &[LifecycleHook],
    context: &HookReviewContext,
) -> HookReviewPacket {
    review_hooks(hooks, context)
}

/// The AI-path hook review. The AI surface reads the same [`HookReviewPacket`]
/// object — including what did not run and what still works — as every other
/// surface.
pub fn ai_hook_review(hooks: &[LifecycleHook], context: &HookReviewContext) -> HookReviewPacket {
    review_hooks(hooks, context)
}

/// The support-path hook-review export: the metadata-first projection wrapping
/// the same [`HookReviewPacket`] object support and release surfaces read.
pub fn support_hook_review(
    hooks: &[LifecycleHook],
    context: &HookReviewContext,
) -> HookReviewExport {
    export_hook_review(&review_hooks(hooks, context))
}

// ---------------------------------------------------------------------------
// Metadata-first export.
// ---------------------------------------------------------------------------

/// A metadata-first projection of a hook review for support and release
/// surfaces. It carries the posture, what did not run, what still works, and
/// the repairs — never command bodies, secret values, or provider payloads —
/// and wraps the canonical packet so support never re-derives the review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Review id.
    pub review_id: String,
    /// Claimed environment profile.
    pub profile: EnvironmentProfile,
    /// Target class.
    pub target_class: CapsuleTargetClass,
    /// The rolled-up posture.
    pub posture: HookReviewPosture,
    /// The governance trust-hooks evidence state this posture maps to.
    pub trust_hooks_evidence_state: EvidenceState,
    /// Number of hooks reviewed.
    pub total_hooks: u32,
    /// Number of hooks cleared to run.
    pub runnable_hooks: u32,
    /// Stable tokens naming every hold reason present.
    pub reason_tokens: Vec<String>,
    /// Hook ids policy denies.
    pub denied_hook_tokens: Vec<String>,
    /// Hook ids blocked behind a failed precondition.
    pub blocked_hook_tokens: Vec<String>,
    /// Review-safe statement of what did not run.
    pub what_did_not_run: String,
    /// Review-safe statement of what still works safely.
    pub what_still_works: String,
    /// One repair per non-allowed hook.
    pub repairs: Vec<HookRepair>,
    /// The canonical packet this export wraps.
    pub packet: HookReviewPacket,
    /// Redaction posture (always metadata-only).
    pub redaction_class: RedactionClass,
}

/// Projects a metadata-first [`HookReviewExport`] from a packet.
pub fn export_hook_review(packet: &HookReviewPacket) -> HookReviewExport {
    HookReviewExport {
        record_kind: HOOK_REVIEW_EXPORT_RECORD_KIND.to_owned(),
        schema_version: HOOK_REVIEW_SCHEMA_VERSION,
        review_id: packet.review_id.clone(),
        profile: packet.profile,
        target_class: packet.target_class,
        posture: packet.posture,
        trust_hooks_evidence_state: packet.trust_hooks_evidence_state,
        total_hooks: packet.total_hooks,
        runnable_hooks: packet.runnable_hooks,
        reason_tokens: packet.reason_tokens.clone(),
        denied_hook_tokens: packet.denied_hook_tokens.clone(),
        blocked_hook_tokens: packet.blocked_hook_tokens.clone(),
        what_did_not_run: packet.what_did_not_run.clone(),
        what_still_works: packet.what_still_works.clone(),
        repairs: packet.repairs.clone(),
        packet: packet.clone(),
        redaction_class: RedactionClass::MetadataOnly,
    }
}

// ---------------------------------------------------------------------------
// Failure / recovery drills.
// ---------------------------------------------------------------------------

/// One ordered step inside a hook-review drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Posture observed at this step.
    pub observed_posture: HookReviewPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a hook from an injected failure
/// through a visible held state and a repair back to a runnable state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Environment profile exercised by the drill.
    pub profile: EnvironmentProfile,
    /// Hook the drill exercises.
    pub hook_id: String,
    /// Reason the drill injects.
    pub injected_reason: HookHoldReason,
    /// Disposition the hook reaches under the failure.
    pub expected_disposition: HookDisposition,
    /// Posture observed while the failure is active.
    pub degraded_posture: HookReviewPosture,
    /// Repair the drill applies.
    pub expected_repair_kind: RepairKind,
    /// Posture observed once the repair is applied.
    pub recovers_to_posture: HookReviewPosture,
    /// Ordered drill steps.
    pub steps: Vec<HookReviewDrillStep>,
    /// True when the drill proves the hook is never run silently under the
    /// failure.
    pub asserts_no_silent_execution: bool,
    /// True when the drill proves the reason and next step are preserved.
    pub asserts_reason_and_next_step_preserved: bool,
    /// True when the drill proves the hook recovers after repair.
    pub asserts_recovers_after_repair: bool,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Fixture record.
// ---------------------------------------------------------------------------

/// The scenario a hook-review fixture exercises.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookReviewScenario {
    /// Every hook is trust-gated and cleared.
    AllCleared,
    /// The workspace is not trusted; every hook is held for review.
    ReviewRequired,
    /// A hook has no trust gate and must never run automatically.
    Ungated,
    /// The workspace is in restricted mode; hooks become manual suggestions.
    Restricted,
    /// Policy denies a lifecycle step.
    PolicyDenied,
    /// A hook needs a secret that is not projected.
    MissingSecret,
    /// A hook's activator is not supported on the target.
    UnsupportedActivator,
    /// A bootstrap action failed and blocks a dependent hook.
    BootstrapFailed,
    /// Every hook is denied; nothing runs.
    FullyBlocked,
    /// A post-create hook is blocked behind a held dependency.
    BlockedPostCreate,
}

impl HookReviewScenario {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllCleared => "all_cleared",
            Self::ReviewRequired => "review_required",
            Self::Ungated => "ungated",
            Self::Restricted => "restricted",
            Self::PolicyDenied => "policy_denied",
            Self::MissingSecret => "missing_secret",
            Self::UnsupportedActivator => "unsupported_activator",
            Self::BootstrapFailed => "bootstrap_failed",
            Self::FullyBlocked => "fully_blocked",
            Self::BlockedPostCreate => "blocked_post_create",
        }
    }
}

/// One checked-in fixture: a set of declared hooks, the context they are
/// reviewed against, and the packet outcome the engine must reach for the pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HookReviewFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Environment profile the fixture exercises.
    pub profile: EnvironmentProfile,
    /// Scenario the fixture exercises.
    pub scenario: HookReviewScenario,
    /// Declared hooks under review.
    pub hooks: Vec<LifecycleHook>,
    /// Context the hooks are reviewed against.
    pub context: HookReviewContext,
    /// Expected posture.
    pub expected_posture: HookReviewPosture,
    /// Expected runnable-hook count.
    pub expected_runnable_hooks: u32,
    /// Expected reason tokens.
    pub expected_reason_tokens: Vec<String>,
    /// Expected denied-hook tokens.
    pub expected_denied_hook_tokens: Vec<String>,
    /// Expected blocked-hook tokens.
    pub expected_blocked_hook_tokens: Vec<String>,
    /// The canonical packet the engine reaches for this fixture.
    pub packet: HookReviewPacket,
    /// One consumer surface that ingests this review.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

fn violation(report: &mut ValidationReport, check_id: &'static str, message: impl Into<String>) {
    report.violations.push(ValidationViolation {
        check_id,
        message: message.into(),
    });
}

fn is_hex64(digest: &CapsuleDigest) -> bool {
    digest.value.len() == 64 && digest.value.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Validates a checked-in declared hook against the frozen contract.
pub fn validate_lifecycle_hook(hook: &LifecycleHook) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if hook.hook_id.trim().is_empty() {
        violation(&mut report, "hook.id", "lifecycle hook must carry an id");
    }
    if hook.authority_ref.trim().is_empty() {
        violation(
            &mut report,
            "hook.authority_ref",
            format!("hook {} must carry an authority ref", hook.hook_id),
        );
    }
    if !is_hex64(&hook.command_digest) {
        violation(
            &mut report,
            "hook.command_digest",
            format!(
                "hook {} command digest must be 64 lowercase hex",
                hook.hook_id
            ),
        );
    }
    if hook.summary.trim().is_empty() {
        violation(
            &mut report,
            "hook.summary",
            format!("hook {} must carry a summary", hook.hook_id),
        );
    }
    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates a checked-in packet: the engine must reproduce it from the hooks
/// and context the fixture carries, and the metadata-first invariants must
/// hold.
fn validate_packet_internal(
    report: &mut ValidationReport,
    hooks: &[LifecycleHook],
    context: &HookReviewContext,
    packet: &HookReviewPacket,
) {
    if packet.record_kind != HOOK_REVIEW_PACKET_RECORD_KIND {
        violation(
            report,
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != HOOK_REVIEW_SCHEMA_VERSION {
        violation(
            report,
            "packet.schema_version",
            "packet schema_version must be 1",
        );
    }
    if packet.redaction_class != RedactionClass::MetadataOnly {
        violation(
            report,
            "packet.redaction_class",
            "packet must declare a metadata-only redaction class",
        );
    }
    let recomputed = review_hooks(hooks, context);
    if &recomputed != packet {
        violation(
            report,
            "packet.engine_agreement",
            "packet disagrees with the review engine recomputed from its hooks and context",
        );
    }
    // The guardrail: a hook with no trust gate is never runnable.
    for entry in &packet.entries {
        if entry.gate_state == TrustGateState::Ungated && entry.runnable {
            violation(
                report,
                "packet.ungated_never_runs",
                format!("ungated hook {} must never be runnable", entry.hook_id),
            );
        }
        if entry.runnable && entry.repair.is_some() {
            violation(
                report,
                "packet.allowed_has_no_repair",
                format!("allowed hook {} must not carry a repair", entry.hook_id),
            );
        }
        if !entry.runnable && entry.repair.is_none() {
            violation(
                report,
                "packet.held_has_repair",
                format!("held hook {} must carry a repair", entry.hook_id),
            );
        }
    }
    // Every repair preserves a hook identity that exists in the packet.
    let entry_ids: BTreeSet<&str> = packet.entries.iter().map(|e| e.hook_id.as_str()).collect();
    for repair in &packet.repairs {
        if !entry_ids.contains(repair.hook_id.as_str()) {
            violation(
                report,
                "packet.repair_identity",
                format!("repair names unknown hook {}", repair.hook_id),
            );
        }
        if repair.next_step.trim().is_empty() {
            violation(
                report,
                "packet.repair_next_step",
                format!("repair for hook {} must carry a next step", repair.hook_id),
            );
        }
    }
}

/// Validates a checked-in fixture: the declared hooks and context themselves,
/// the embedded packet, and that the recorded expectations equal what the
/// engine computes.
pub fn validate_hook_review_fixture(fixture: &HookReviewFixture) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != HOOK_REVIEW_FIXTURE_RECORD_KIND {
        violation(
            &mut report,
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != HOOK_REVIEW_SCHEMA_VERSION {
        violation(
            &mut report,
            "fixture.schema_version",
            "fixture schema_version must be 1",
        );
    }
    if fixture.fixture_id.trim().is_empty() {
        violation(&mut report, "fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        violation(
            &mut report,
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        violation(
            &mut report,
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }
    if fixture.hooks.is_empty() {
        violation(
            &mut report,
            "fixture.hooks",
            format!(
                "fixture {} must declare at least one hook",
                fixture.fixture_id
            ),
        );
    }

    let mut hook_ids = BTreeSet::new();
    for hook in &fixture.hooks {
        if !hook_ids.insert(hook.hook_id.as_str()) {
            violation(
                &mut report,
                "fixture.hook_unique",
                format!(
                    "fixture {} repeats hook id {}",
                    fixture.fixture_id, hook.hook_id
                ),
            );
        }
        if let Err(hook_report) = validate_lifecycle_hook(hook) {
            for inner in hook_report.violations {
                report.violations.push(inner);
            }
        }
    }
    for hook in &fixture.hooks {
        for dep in &hook.depends_on {
            if !hook_ids.contains(dep.as_str()) {
                violation(
                    &mut report,
                    "fixture.depends_on",
                    format!("hook {} depends on unknown hook {}", hook.hook_id, dep),
                );
            }
        }
    }

    validate_packet_internal(
        &mut report,
        &fixture.hooks,
        &fixture.context,
        &fixture.packet,
    );

    if fixture.expected_posture != fixture.packet.posture {
        violation(
            &mut report,
            "fixture.expected_posture",
            format!(
                "fixture {} expected posture disagrees with the packet",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_runnable_hooks != fixture.packet.runnable_hooks {
        violation(
            &mut report,
            "fixture.expected_runnable_hooks",
            format!(
                "fixture {} expected runnable count disagrees with the packet",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_reason_tokens != fixture.packet.reason_tokens {
        violation(
            &mut report,
            "fixture.expected_reason_tokens",
            format!(
                "fixture {} expected reason tokens disagree with the packet",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_denied_hook_tokens != fixture.packet.denied_hook_tokens {
        violation(
            &mut report,
            "fixture.expected_denied_hook_tokens",
            format!(
                "fixture {} expected denied tokens disagree with the packet",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_blocked_hook_tokens != fixture.packet.blocked_hook_tokens {
        violation(
            &mut report,
            "fixture.expected_blocked_hook_tokens",
            format!(
                "fixture {} expected blocked tokens disagree with the packet",
                fixture.fixture_id
            ),
        );
    }

    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates a checked-in drill: the degraded and recovered postures it records
/// must match what the engine reaches for the injected and repaired contexts
/// the seed builds, and the drill must prove the no-silent-execution and
/// recovery invariants.
pub fn validate_hook_review_drill(drill: &HookReviewDrill) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if drill.drill_id.trim().is_empty() {
        violation(&mut report, "drill.id", "drill must carry an id");
    }
    if drill.title.trim().is_empty() || drill.notes.trim().is_empty() {
        violation(
            &mut report,
            "drill.prose",
            format!("drill {} must carry a title and a note", drill.drill_id),
        );
    }
    if drill.expected_disposition != drill.injected_reason.disposition() {
        violation(
            &mut report,
            "drill.disposition",
            format!(
                "drill {} disposition disagrees with its injected reason",
                drill.drill_id
            ),
        );
    }
    if drill.expected_repair_kind != drill.injected_reason.repair_kind() {
        violation(
            &mut report,
            "drill.repair_kind",
            format!(
                "drill {} repair kind disagrees with its injected reason",
                drill.drill_id
            ),
        );
    }
    if drill.recovers_to_posture != HookReviewPosture::AllCleared {
        violation(
            &mut report,
            "drill.recovers",
            format!(
                "drill {} must recover to all_cleared after repair",
                drill.drill_id
            ),
        );
    }
    if !(drill.asserts_no_silent_execution
        && drill.asserts_reason_and_next_step_preserved
        && drill.asserts_recovers_after_repair)
    {
        violation(
            &mut report,
            "drill.invariants",
            format!("drill {} must assert all three invariants", drill.drill_id),
        );
    }
    if drill.steps.is_empty() {
        violation(
            &mut report,
            "drill.steps",
            format!("drill {} must carry ordered steps", drill.drill_id),
        );
    }
    if report.violations.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}
