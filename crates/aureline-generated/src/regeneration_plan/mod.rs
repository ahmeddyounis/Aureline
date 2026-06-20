//! Typed, reviewable regeneration plans with visible partial, blocked,
//! stale-input, and policy-limited outcomes for claimed M5 generated-artifact
//! classes.
//!
//! The sibling [`crate::m5_generated_governance`] matrix certifies
//! generated-artifact truth one row per *class*, the [`crate::descriptor`]
//! lane models the per-*artifact* identity object the surfaces render, and the
//! [`crate::write_boundary`] lane models what happens when a user attempts a
//! direct edit. This module models the *other* half of the writable
//! boundary — what happens when a user asks to **regenerate** a derived
//! artifact instead of editing it.
//!
//! A regenerate action is never a blind command. Each
//! [`RegenerationRequest`] — a set of [`RegenerationTarget`]s plus a
//! [`RollbackBoundary`] — is folded by the single [`plan_regeneration`] engine
//! into a typed [`RegenerationPlan`] the surfaces render *before* execution.
//! The plan is the one object the product regenerate sheet, the help guide,
//! the support export, the release-evidence review, and the AI context all
//! consume, so no surface can present a regenerate button that implies safety
//! or completeness the evidence does not support.
//!
//! Five guardrails are frozen here:
//!
//! - **A plan and a side-effect boundary, always, before execution.** Every
//!   regenerate action resolves to a [`RegenerationPlan`] carrying its target
//!   artifacts, canonical-source refs, generator/runtime requirements, the
//!   [`SideEffectBoundary`] it would cross, and the [`RollbackBoundary`] that
//!   bounds it — never a bare command.
//! - **Blocked, partial, stale, and policy-limited are labeled precisely.**
//!   [`PlanReadiness`] names the headline outcome and never lets a degraded
//!   plan masquerade as success: a plan that can only run for some targets is
//!   [`PlanReadiness::Partial`], a plan whose inputs are stale is
//!   [`PlanReadiness::ReadyStaleInputs`], and a plan gated by policy or an
//!   undeclared side effect is [`PlanReadiness::PolicyLimited`].
//! - **No silent side effects.** A regeneration may not hide a networked
//!   install, a tool download, secret use, or a broad filesystem write: any
//!   sensitive [`SideEffectClass`] that is not declared and reviewed
//!   ([`SideEffectDisclosure::Undeclared`]) holds the target for disclosure
//!   instead of running silently.
//! - **The rollback boundary is honest.** [`RollbackCoverage`] is computed
//!   from the side effects, so a regeneration that escapes the workspace
//!   checkpoint (a global install, a tool download, a broad write) is reported
//!   as [`RollbackCoverage::PartiallyReversible`] rather than implying a clean
//!   undo.
//! - **Every block carries its reason and a recovery path.** A plan that is
//!   not fully ready carries [`RegenerationPlan::why_blocked_tokens`] and a
//!   [`RecoveryStep`] path — restore the source, install the generator,
//!   provision the runtime, refresh stale inputs, declare the side effect, or
//!   resolve the policy — so a block is never reduced to a generic failure.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/regeneration-plan.schema.json`](../../../../schemas/generated/regeneration-plan.schema.json)
//! - [`/docs/generated/regeneration-plan.md`](../../../../docs/generated/regeneration-plan.md)
//! - [`/artifacts/generated/regeneration-plan-packet.json`](../../../../artifacts/generated/regeneration-plan-packet.json)
//! - [`/artifacts/generated/regeneration-plan.md`](../../../../artifacts/generated/regeneration-plan.md)
//! - [`/fixtures/generated/regeneration-plan/`](../../../../fixtures/generated/regeneration-plan/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::descriptor::{GeneratorIdentity, GeneratorKind};
pub use crate::m5_generated_governance::{ArtifactClass, AuthorityClass};

/// Schema version stamped onto the packet and fixtures.
pub const REGENERATION_PLAN_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const REGENERATION_PLAN_PACKET_RECORD_KIND: &str = "regeneration_plan_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const REGENERATION_PLAN_FIXTURE_RECORD_KIND: &str = "regeneration_plan_fixture_record";

/// Stable packet id every surface binding ingests.
pub const REGENERATION_PLAN_PACKET_ID: &str = "generated.regeneration_plan.v1";

/// Repo-relative schema ref.
pub const REGENERATION_PLAN_SCHEMA_REF: &str = "schemas/generated/regeneration-plan.schema.json";

/// Repo-relative reviewer doc ref.
pub const REGENERATION_PLAN_DOC_REF: &str = "docs/generated/regeneration-plan.md";

/// Repo-relative machine-readable proof packet.
pub const REGENERATION_PLAN_PACKET_REF: &str = "artifacts/generated/regeneration-plan-packet.json";

/// Repo-relative reviewer certification summary.
pub const REGENERATION_PLAN_REPORT_REF: &str = "artifacts/generated/regeneration-plan.md";

/// Repo-relative fixture directory.
pub const REGENERATION_PLAN_FIXTURE_DIR: &str = "fixtures/generated/regeneration-plan";

/// Repo-relative fixture manifest.
pub const REGENERATION_PLAN_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/regeneration-plan/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The headline readiness of a regeneration plan. This is the one label every
/// surface renders, and it never lets a degraded plan look like a clean
/// success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanReadiness {
    /// Every target will regenerate in full: all preconditions are satisfied,
    /// no inputs are stale, and every side effect is declared and reviewed.
    Ready,
    /// Every target can regenerate, but at least one input is stale, so the
    /// regenerated bytes may not reflect the latest source. Surfaced, never
    /// silent.
    ReadyStaleInputs,
    /// Some targets will regenerate and at least one cannot, so the plan
    /// applies only partially.
    Partial,
    /// No target can regenerate, and the sole obstruction is a policy block or
    /// an undeclared side effect awaiting review — the plan does not run as-is.
    PolicyLimited,
    /// No target can regenerate because required source, generator, or runtime
    /// is missing.
    Blocked,
}

impl PlanReadiness {
    /// Every readiness state in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Ready,
        Self::ReadyStaleInputs,
        Self::Partial,
        Self::PolicyLimited,
        Self::Blocked,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReadyStaleInputs => "ready_stale_inputs",
            Self::Partial => "partial",
            Self::PolicyLimited => "policy_limited",
            Self::Blocked => "blocked",
        }
    }

    /// A short surface-agnostic label for the state.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::ReadyStaleInputs => "Ready (stale inputs)",
            Self::Partial => "Partial",
            Self::PolicyLimited => "Policy-limited",
            Self::Blocked => "Blocked",
        }
    }

    /// Whether at least one target will regenerate under this plan.
    pub const fn runs_any(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyStaleInputs | Self::Partial)
    }

    /// Whether every target will regenerate under this plan.
    pub const fn runs_in_full(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyStaleInputs)
    }

    /// Whether this readiness names a clean, fully ready plan with nothing to
    /// surface.
    pub const fn is_clean(self) -> bool {
        matches!(self, Self::Ready)
    }
}

/// The outcome the engine reaches for one regeneration target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetOutcome {
    /// The target will regenerate cleanly.
    Ready,
    /// The target will regenerate, but at least one of its inputs is stale.
    ReadyStaleInputs,
    /// The target cannot regenerate: required source, generator, or runtime is
    /// missing.
    Blocked,
    /// The target cannot regenerate because a policy forbids it.
    BlockedByPolicy,
    /// The target is held because it would perform an undeclared sensitive
    /// side effect; it must be declared and reviewed before it can run.
    HeldForDisclosure,
}

impl TargetOutcome {
    /// Every target outcome in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Ready,
        Self::ReadyStaleInputs,
        Self::Blocked,
        Self::BlockedByPolicy,
        Self::HeldForDisclosure,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::ReadyStaleInputs => "ready_stale_inputs",
            Self::Blocked => "blocked",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::HeldForDisclosure => "held_for_disclosure",
        }
    }

    /// Whether this target will regenerate.
    pub const fn will_run(self) -> bool {
        matches!(self, Self::Ready | Self::ReadyStaleInputs)
    }

    /// Whether this outcome is a hard block on missing source, generator, or
    /// runtime, rather than a soft policy or disclosure hold.
    pub const fn is_hard_block(self) -> bool {
        matches!(self, Self::Blocked)
    }
}

/// One precondition a regeneration target must satisfy before it can run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreconditionKind {
    /// The canonical source the artifact regenerates from.
    CanonicalSource,
    /// The generator that rebuilds the artifact.
    Generator,
    /// The runtime or toolchain the generator needs to run.
    Runtime,
    /// The freshness of the inputs the regeneration reads.
    InputFreshness,
    /// The policy that governs whether the artifact may be regenerated.
    Policy,
}

impl PreconditionKind {
    /// Every precondition kind in canonical order.
    pub const ALL: [Self; 5] = [
        Self::CanonicalSource,
        Self::Generator,
        Self::Runtime,
        Self::InputFreshness,
        Self::Policy,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSource => "canonical_source",
            Self::Generator => "generator",
            Self::Runtime => "runtime",
            Self::InputFreshness => "input_freshness",
            Self::Policy => "policy",
        }
    }

    /// Whether a given state is meaningful for this precondition. A runtime is
    /// never "stale", a policy is only satisfied or blocked, and so on; the
    /// validator uses this to keep cases honest.
    pub const fn allows(self, state: PreconditionState) -> bool {
        match self {
            Self::CanonicalSource => matches!(
                state,
                PreconditionState::Satisfied
                    | PreconditionState::Stale
                    | PreconditionState::Missing
            ),
            Self::Generator | Self::Runtime => {
                matches!(
                    state,
                    PreconditionState::Satisfied | PreconditionState::Missing
                )
            }
            Self::InputFreshness => {
                matches!(
                    state,
                    PreconditionState::Satisfied | PreconditionState::Stale
                )
            }
            Self::Policy => matches!(
                state,
                PreconditionState::Satisfied | PreconditionState::BlockedByPolicy
            ),
        }
    }
}

/// The observed state of one precondition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreconditionState {
    /// The precondition holds.
    Satisfied,
    /// The precondition is present but past its freshness window.
    Stale,
    /// The precondition is absent, so the regeneration cannot run.
    Missing,
    /// A policy forbids the regeneration.
    BlockedByPolicy,
}

impl PreconditionState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }
}

/// The class of side effect a regeneration would perform. The four sensitive
/// classes are the no-silent-side-effects guardrail: a regeneration may not
/// hide a networked install, a tool download, secret use, or a broad
/// filesystem write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectClass {
    /// Purely local, in-scope computation that writes only the target
    /// artifact. The benign default.
    LocalCompute,
    /// A networked package or dependency install.
    NetworkInstall,
    /// Downloading a tool or binary.
    ToolDownload,
    /// Reading or using a secret or credential.
    SecretAccess,
    /// Writing beyond the target artifact's own path.
    BroadFilesystemWrite,
}

impl SideEffectClass {
    /// Every side-effect class in canonical order.
    pub const ALL: [Self; 5] = [
        Self::LocalCompute,
        Self::NetworkInstall,
        Self::ToolDownload,
        Self::SecretAccess,
        Self::BroadFilesystemWrite,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCompute => "local_compute",
            Self::NetworkInstall => "network_install",
            Self::ToolDownload => "tool_download",
            Self::SecretAccess => "secret_access",
            Self::BroadFilesystemWrite => "broad_filesystem_write",
        }
    }

    /// Whether this side effect is sensitive and so must be declared and
    /// reviewed before a regeneration may perform it.
    pub const fn is_sensitive(self) -> bool {
        !matches!(self, Self::LocalCompute)
    }

    /// Whether this side effect escapes the workspace rollback checkpoint, so
    /// a plan that performs it cannot promise a fully reversible undo.
    pub const fn escapes_checkpoint(self) -> bool {
        matches!(
            self,
            Self::NetworkInstall | Self::ToolDownload | Self::BroadFilesystemWrite
        )
    }

    /// A review-safe description of the side effect.
    pub const fn describe(self) -> &'static str {
        match self {
            Self::LocalCompute => "local, in-scope computation writing only the target artifact",
            Self::NetworkInstall => "a networked package or dependency install",
            Self::ToolDownload => "downloading a tool or binary",
            Self::SecretAccess => "reading or using a secret or credential",
            Self::BroadFilesystemWrite => "writing beyond the target artifact's own path",
        }
    }
}

/// Whether a side effect was declared in the plan and reviewed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SideEffectDisclosure {
    /// The side effect was declared in the plan and reviewed, so it may run.
    DeclaredReviewed,
    /// The side effect would happen but was not declared or reviewed, so it
    /// must not run silently.
    Undeclared,
}

impl SideEffectDisclosure {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclaredReviewed => "declared_reviewed",
            Self::Undeclared => "undeclared",
        }
    }
}

/// How fully the rollback checkpoint can undo a regeneration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackCoverage {
    /// Every write the regeneration performs is captured by the checkpoint, so
    /// the regeneration is fully reversible.
    FullyReversible,
    /// The regeneration performs a write that escapes the workspace
    /// checkpoint — a global install, a tool download, or a broad write — so
    /// the undo is only partial.
    PartiallyReversible,
}

impl RollbackCoverage {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversible => "fully_reversible",
            Self::PartiallyReversible => "partially_reversible",
        }
    }

    /// The coverage a set of side-effect classes implies.
    pub fn for_classes(classes: impl IntoIterator<Item = SideEffectClass>) -> Self {
        if classes.into_iter().any(SideEffectClass::escapes_checkpoint) {
            Self::PartiallyReversible
        } else {
            Self::FullyReversible
        }
    }
}

/// The class of a recovery step a degraded plan offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryClass {
    /// Regenerate the targets that are ready now, leaving the blocked ones.
    RegenerateReadyTargets,
    /// Refresh the stale inputs before regenerating, so the result reflects
    /// the latest source.
    RefreshInputs,
    /// Restore the missing canonical source before regenerating.
    RestoreCanonicalSource,
    /// Restore the unavailable generator before regenerating.
    RestoreGenerator,
    /// Provision the required runtime before regenerating.
    ProvisionRuntime,
    /// Declare and review the side effect before the regeneration may perform
    /// it.
    DeclareAndReviewSideEffect,
    /// Resolve the policy that blocks regeneration.
    ResolveRegenerationPolicy,
}

impl RecoveryClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegenerateReadyTargets => "regenerate_ready_targets",
            Self::RefreshInputs => "refresh_inputs",
            Self::RestoreCanonicalSource => "restore_canonical_source",
            Self::RestoreGenerator => "restore_generator",
            Self::ProvisionRuntime => "provision_runtime",
            Self::DeclareAndReviewSideEffect => "declare_and_review_side_effect",
            Self::ResolveRegenerationPolicy => "resolve_regeneration_policy",
        }
    }

    /// A short reviewer summary for the step.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::RegenerateReadyTargets => {
                "Regenerate the targets that are ready now, leaving the blocked ones untouched."
            }
            Self::RefreshInputs => {
                "Refresh the stale inputs before regenerating so the result reflects the latest source."
            }
            Self::RestoreCanonicalSource => {
                "Restore the missing canonical source, then regenerate from it."
            }
            Self::RestoreGenerator => {
                "Restore the unavailable generator, then regenerate from the canonical source."
            }
            Self::ProvisionRuntime => {
                "Provision the required runtime, then regenerate from the canonical source."
            }
            Self::DeclareAndReviewSideEffect => {
                "Declare and review the side effect before the regeneration may perform it."
            }
            Self::ResolveRegenerationPolicy => {
                "Resolve the policy that blocks regeneration, then regenerate from the canonical source."
            }
        }
    }
}

/// A surface that renders the regeneration plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegenerationPlanSurface {
    /// The product regenerate sheet that shows the plan and side-effect
    /// boundary before execution.
    RegeneratePlanSheet,
    /// The help guide that explains the readiness states.
    HelpRegenerationGuide,
    /// The metadata-first support export.
    SupportExport,
    /// The release-evidence review.
    ReleaseEvidence,
    /// The AI prompt-context attachment line.
    AiContext,
}

impl RegenerationPlanSurface {
    /// Every rendered surface in canonical order.
    pub const ALL: [Self; 5] = [
        Self::RegeneratePlanSheet,
        Self::HelpRegenerationGuide,
        Self::SupportExport,
        Self::ReleaseEvidence,
        Self::AiContext,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegeneratePlanSheet => "regenerate_plan_sheet",
            Self::HelpRegenerationGuide => "help_regeneration_guide",
            Self::SupportExport => "support_export",
            Self::ReleaseEvidence => "release_evidence",
            Self::AiContext => "ai_context",
        }
    }
}

// ---------------------------------------------------------------------------
// Request (the engine's input).
// ---------------------------------------------------------------------------

/// One declared side effect a regeneration target would perform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffect {
    /// The class of side effect.
    pub class: SideEffectClass,
    /// Whether it was declared in the plan and reviewed.
    pub disclosure: SideEffectDisclosure,
    /// Review-safe description; never a secret body, credential, or raw path.
    pub detail: String,
}

/// One artifact a regeneration plan targets, with the observed state of every
/// precondition and the side effects regenerating it would perform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationTarget {
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe display label for the artifact path.
    pub artifact_path_label: String,
    /// Provenance/authority class of the bytes relative to the source.
    pub authority_class: AuthorityClass,
    /// Generator that rebuilds the artifact, with version.
    pub generator: GeneratorIdentity,
    /// Review-safe description of the runtime the generator requires.
    pub runtime_requirement: String,
    /// Review-safe canonical-source reference. Present unless the source state
    /// is [`PreconditionState::Missing`].
    pub canonical_source_ref: Option<String>,
    /// Review-safe regeneration route that rebuilds the artifact.
    pub regeneration_route: String,
    /// State of the canonical source.
    pub source_state: PreconditionState,
    /// State of the generator.
    pub generator_state: PreconditionState,
    /// State of the runtime.
    pub runtime_state: PreconditionState,
    /// Freshness of the inputs.
    pub input_freshness: PreconditionState,
    /// State of the regeneration policy.
    pub policy_state: PreconditionState,
    /// Side effects the regeneration would perform.
    pub side_effects: Vec<SideEffect>,
}

/// The reversible-checkpoint boundary that bounds a regeneration plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackBoundary {
    /// Reference to the reversible checkpoint that captured the pre-regenerate
    /// state.
    pub checkpoint_ref: String,
    /// Review-safe description of what the checkpoint captures.
    pub scope: String,
}

/// The inputs the regeneration engine reads for one plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationRequest {
    /// Stable plan id.
    pub plan_id: String,
    /// Review-safe label for what triggered the regeneration.
    pub trigger_label: String,
    /// The artifacts the plan targets. At least one.
    pub targets: Vec<RegenerationTarget>,
    /// The reversible-checkpoint boundary that bounds the plan.
    pub rollback_boundary: RollbackBoundary,
}

// ---------------------------------------------------------------------------
// Plan (the engine's output).
// ---------------------------------------------------------------------------

/// The status of one precondition on a target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreconditionStatus {
    /// Which precondition this is.
    pub kind: PreconditionKind,
    /// Its observed state.
    pub state: PreconditionState,
    /// Review-safe reference or description for the precondition.
    pub detail: String,
}

/// The computed plan for one target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetPlan {
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe artifact path label.
    pub artifact_path_label: String,
    /// Provenance/authority class.
    pub authority_class: AuthorityClass,
    /// Generator identity.
    pub generator: GeneratorIdentity,
    /// Runtime requirement.
    pub runtime_requirement: String,
    /// Canonical-source reference, when linked.
    pub canonical_source_ref: Option<String>,
    /// Regeneration route.
    pub regeneration_route: String,
    /// One status per [`PreconditionKind`], in canonical order.
    pub preconditions: Vec<PreconditionStatus>,
    /// Side effects this regeneration would perform.
    pub side_effects: Vec<SideEffect>,
    /// The outcome the engine reached for this target.
    pub outcome: TargetOutcome,
    /// Stable tokens naming why this target is blocked or held, sorted and
    /// deduplicated. Empty only when the target will run.
    pub why_blocked_tokens: Vec<String>,
    /// Review-safe summary of the target plan.
    pub summary: String,
}

/// The aggregate side-effect boundary across every target in the plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SideEffectBoundary {
    /// Distinct side-effect classes the plan would cross, in canonical order.
    pub classes_present: Vec<SideEffectClass>,
    /// One aggregated entry per distinct class, taking the most conservative
    /// disclosure (undeclared wins) across the targets.
    pub effects: Vec<SideEffect>,
    /// True when no sensitive side effect is undeclared anywhere in the plan.
    pub all_sensitive_declared: bool,
    /// The sensitive classes that are undeclared somewhere, sorted distinct.
    pub undeclared_sensitive_classes: Vec<SideEffectClass>,
    /// Review-safe summary of the side-effect boundary.
    pub summary: String,
}

/// One recovery step a degraded plan offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryStep {
    /// Recovery class.
    pub class: RecoveryClass,
    /// Review-safe route the step takes.
    pub action_ref: String,
    /// Short reviewer summary.
    pub summary: String,
}

/// The computed regeneration plan the surfaces render before execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlan {
    /// Stable plan id, echoed so the plan is inspectable standalone.
    pub plan_id: String,
    /// Headline readiness of the plan.
    pub readiness: PlanReadiness,
    /// Whether every target will regenerate under this plan.
    pub runs_in_full: bool,
    /// Whether the plan applies only partially.
    pub partial: bool,
    /// Whether any input feeding the plan is stale.
    pub stale_inputs: bool,
    /// Number of targets that will regenerate.
    pub runnable_target_count: u32,
    /// Number of targets that cannot regenerate.
    pub blocked_target_count: u32,
    /// The per-target plans, one per requested target in order.
    pub targets: Vec<TargetPlan>,
    /// The aggregate side-effect boundary the plan would cross.
    pub side_effect_boundary: SideEffectBoundary,
    /// How fully the rollback checkpoint can undo the plan.
    pub rollback_coverage: RollbackCoverage,
    /// The reversible-checkpoint boundary that bounds the plan.
    pub rollback_boundary: RollbackBoundary,
    /// Stable tokens naming every input that blocked or held a target, sorted
    /// and deduplicated. Empty only when the plan is fully ready.
    pub why_blocked_tokens: Vec<String>,
    /// The recovery path, empty only when the plan is fully ready and clean.
    pub recovery: Vec<RecoveryStep>,
    /// The user-visible guidance line.
    pub guidance_line: String,
    /// The one stable copy/export form for the plan.
    pub copy_line: String,
}

// ---------------------------------------------------------------------------
// Engine: the single source of truth for the plan.
// ---------------------------------------------------------------------------

/// Plans a regeneration from its request.
///
/// This is the canonical engine the cases, the fixtures, the validators, and
/// the consuming surfaces all share. Each target is folded independently into
/// a [`TargetPlan`]; the per-target outcomes then fold into the plan-level
/// [`PlanReadiness`]:
///
/// - if every target will run, the plan is [`PlanReadiness::Ready`], or
///   [`PlanReadiness::ReadyStaleInputs`] when an input is stale;
/// - if some targets run and some do not, the plan is
///   [`PlanReadiness::Partial`];
/// - if no target runs and the sole obstruction is policy or an undeclared
///   side effect, the plan is [`PlanReadiness::PolicyLimited`];
/// - otherwise the plan is [`PlanReadiness::Blocked`].
///
/// The side-effect boundary and rollback coverage are computed from the
/// targets' side effects, so a regeneration never hides a networked install,
/// tool download, secret use, or broad write, and never implies a clean undo
/// it cannot deliver.
pub fn plan_regeneration(request: &RegenerationRequest) -> RegenerationPlan {
    let targets: Vec<TargetPlan> = request.targets.iter().map(plan_target).collect();

    let runnable_target_count = targets.iter().filter(|t| t.outcome.will_run()).count() as u32;
    let blocked_target_count = targets.len() as u32 - runnable_target_count;

    let any_hard_block = targets.iter().any(|t| t.outcome.is_hard_block());
    let any_stale = targets.iter().any(|t| {
        t.preconditions
            .iter()
            .any(|p| p.state == PreconditionState::Stale)
    });

    let readiness = if blocked_target_count == 0 {
        if any_stale {
            PlanReadiness::ReadyStaleInputs
        } else {
            PlanReadiness::Ready
        }
    } else if runnable_target_count > 0 {
        PlanReadiness::Partial
    } else if any_hard_block {
        PlanReadiness::Blocked
    } else {
        // Nothing runs and the only obstructions are policy / disclosure.
        PlanReadiness::PolicyLimited
    };

    // Plan-level block tokens are the union of the tokens from targets that did
    // not run, so the plan explains exactly what stopped each one.
    let mut why_blocked_tokens: Vec<String> = targets
        .iter()
        .filter(|t| !t.outcome.will_run())
        .flat_map(|t| t.why_blocked_tokens.iter().cloned())
        .collect();
    why_blocked_tokens.sort();
    why_blocked_tokens.dedup();

    let side_effect_boundary = build_side_effect_boundary(&targets);
    let rollback_coverage =
        RollbackCoverage::for_classes(side_effect_boundary.classes_present.iter().copied());

    let recovery = recovery_for(readiness, &why_blocked_tokens, any_stale, request);
    let guidance_line = guidance_for(
        readiness,
        runnable_target_count,
        targets.len() as u32,
        &side_effect_boundary,
    );
    let copy_line = copy_line_for(
        &request.plan_id,
        readiness,
        targets.len() as u32,
        runnable_target_count,
        blocked_target_count,
        &side_effect_boundary,
        rollback_coverage,
        any_stale,
    );

    RegenerationPlan {
        plan_id: request.plan_id.clone(),
        readiness,
        runs_in_full: readiness.runs_in_full(),
        partial: readiness == PlanReadiness::Partial,
        stale_inputs: any_stale,
        runnable_target_count,
        blocked_target_count,
        targets,
        side_effect_boundary,
        rollback_coverage,
        rollback_boundary: request.rollback_boundary.clone(),
        why_blocked_tokens,
        recovery,
        guidance_line,
        copy_line,
    }
}

fn plan_target(target: &RegenerationTarget) -> TargetPlan {
    let preconditions = vec![
        precondition_status(
            PreconditionKind::CanonicalSource,
            target.source_state,
            target
                .canonical_source_ref
                .clone()
                .unwrap_or_else(|| "canonical source not recorded".to_owned()),
        ),
        precondition_status(
            PreconditionKind::Generator,
            target.generator_state,
            target.generator.copy_form(),
        ),
        precondition_status(
            PreconditionKind::Runtime,
            target.runtime_state,
            target.runtime_requirement.clone(),
        ),
        precondition_status(
            PreconditionKind::InputFreshness,
            target.input_freshness,
            target.regeneration_route.clone(),
        ),
        precondition_status(
            PreconditionKind::Policy,
            target.policy_state,
            "regeneration policy".to_owned(),
        ),
    ];

    let mut tokens = Vec::new();
    let mut hard_blocked = false;
    let mut policy_blocked = false;
    let mut held_disclosure = false;
    let mut stale = false;

    match target.source_state {
        PreconditionState::Missing => {
            tokens.push("source_missing".to_owned());
            hard_blocked = true;
        }
        PreconditionState::Stale => stale = true,
        _ => {}
    }
    if target.generator_state == PreconditionState::Missing {
        tokens.push("generator_unavailable".to_owned());
        hard_blocked = true;
    }
    if target.runtime_state == PreconditionState::Missing {
        tokens.push("runtime_unavailable".to_owned());
        hard_blocked = true;
    }
    if target.input_freshness == PreconditionState::Stale {
        stale = true;
    }
    if target.policy_state == PreconditionState::BlockedByPolicy {
        tokens.push("regeneration_blocked_by_policy".to_owned());
        policy_blocked = true;
    }
    for effect in &target.side_effects {
        if effect.class.is_sensitive() && effect.disclosure == SideEffectDisclosure::Undeclared {
            tokens.push(format!("undeclared_side_effect_{}", effect.class.as_str()));
            held_disclosure = true;
        }
    }
    tokens.sort();
    tokens.dedup();

    // Precedence: a hard block (missing material) outranks a policy block,
    // which outranks a disclosure hold; staleness is only a flag on an
    // otherwise-runnable target.
    let outcome = if hard_blocked {
        TargetOutcome::Blocked
    } else if policy_blocked {
        TargetOutcome::BlockedByPolicy
    } else if held_disclosure {
        TargetOutcome::HeldForDisclosure
    } else if stale {
        TargetOutcome::ReadyStaleInputs
    } else {
        TargetOutcome::Ready
    };

    let summary = target_summary(target, outcome);

    TargetPlan {
        artifact_class: target.artifact_class,
        artifact_path_label: target.artifact_path_label.clone(),
        authority_class: target.authority_class,
        generator: target.generator.clone(),
        runtime_requirement: target.runtime_requirement.clone(),
        canonical_source_ref: target.canonical_source_ref.clone(),
        regeneration_route: target.regeneration_route.clone(),
        preconditions,
        side_effects: target.side_effects.clone(),
        outcome,
        why_blocked_tokens: tokens,
        summary,
    }
}

fn precondition_status(
    kind: PreconditionKind,
    state: PreconditionState,
    detail: String,
) -> PreconditionStatus {
    PreconditionStatus {
        kind,
        state,
        detail,
    }
}

fn target_summary(target: &RegenerationTarget, outcome: TargetOutcome) -> String {
    match outcome {
        TargetOutcome::Ready => format!(
            "{} will regenerate via {}.",
            target.artifact_path_label, target.regeneration_route
        ),
        TargetOutcome::ReadyStaleInputs => format!(
            "{} will regenerate via {}, but its inputs are stale.",
            target.artifact_path_label, target.regeneration_route
        ),
        TargetOutcome::Blocked => format!(
            "{} cannot regenerate: required source, generator, or runtime is missing.",
            target.artifact_path_label
        ),
        TargetOutcome::BlockedByPolicy => format!(
            "{} cannot regenerate: a policy forbids it.",
            target.artifact_path_label
        ),
        TargetOutcome::HeldForDisclosure => format!(
            "{} is held: it would perform an undeclared side effect that must be reviewed first.",
            target.artifact_path_label
        ),
    }
}

fn build_side_effect_boundary(targets: &[TargetPlan]) -> SideEffectBoundary {
    let mut classes_present: Vec<SideEffectClass> = Vec::new();
    let mut undeclared_sensitive: Vec<SideEffectClass> = Vec::new();
    let mut effects: Vec<SideEffect> = Vec::new();

    // Aggregate one entry per distinct class in canonical order, taking the
    // most conservative disclosure: undeclared wins.
    for class in SideEffectClass::ALL {
        let occurrences: Vec<&SideEffect> = targets
            .iter()
            .flat_map(|t| t.side_effects.iter())
            .filter(|e| e.class == class)
            .collect();
        if occurrences.is_empty() {
            continue;
        }
        classes_present.push(class);
        let undeclared = occurrences
            .iter()
            .any(|e| e.disclosure == SideEffectDisclosure::Undeclared);
        if class.is_sensitive() && undeclared {
            undeclared_sensitive.push(class);
        }
        effects.push(SideEffect {
            class,
            disclosure: if undeclared {
                SideEffectDisclosure::Undeclared
            } else {
                SideEffectDisclosure::DeclaredReviewed
            },
            detail: class.describe().to_owned(),
        });
    }

    let all_sensitive_declared = undeclared_sensitive.is_empty();
    let summary = if classes_present.is_empty() {
        "No side effects: regeneration writes only the target artifacts.".to_owned()
    } else if all_sensitive_declared {
        format!(
            "Side-effect boundary: {}; every sensitive side effect is declared and reviewed.",
            join_classes(&classes_present)
        )
    } else {
        format!(
            "Side-effect boundary: {}; undeclared sensitive side effects ({}) must be declared and reviewed before regeneration.",
            join_classes(&classes_present),
            join_classes(&undeclared_sensitive)
        )
    };

    SideEffectBoundary {
        classes_present,
        effects,
        all_sensitive_declared,
        undeclared_sensitive_classes: undeclared_sensitive,
        summary,
    }
}

fn join_classes(classes: &[SideEffectClass]) -> String {
    if classes.is_empty() {
        return "none".to_owned();
    }
    classes
        .iter()
        .map(|c| c.as_str())
        .collect::<Vec<_>>()
        .join("+")
}

fn recovery_step(class: RecoveryClass, action_ref: String) -> RecoveryStep {
    RecoveryStep {
        class,
        action_ref,
        summary: class.summary().to_owned(),
    }
}

fn recovery_for(
    readiness: PlanReadiness,
    why_blocked_tokens: &[String],
    any_stale: bool,
    request: &RegenerationRequest,
) -> Vec<RecoveryStep> {
    if readiness == PlanReadiness::Ready {
        return Vec::new();
    }
    if readiness == PlanReadiness::ReadyStaleInputs {
        return vec![recovery_step(
            RecoveryClass::RefreshInputs,
            request.trigger_label.clone(),
        )];
    }

    let mut steps = Vec::new();
    if readiness == PlanReadiness::Partial {
        steps.push(recovery_step(
            RecoveryClass::RegenerateReadyTargets,
            request.trigger_label.clone(),
        ));
    }
    if any_stale {
        steps.push(recovery_step(
            RecoveryClass::RefreshInputs,
            request.trigger_label.clone(),
        ));
    }
    // Map the distinct block tokens to their unblocking step, in a stable
    // order driven by the recovery-class vocabulary.
    if why_blocked_tokens.iter().any(|t| t == "source_missing") {
        steps.push(recovery_step(
            RecoveryClass::RestoreCanonicalSource,
            "restore the canonical source".to_owned(),
        ));
    }
    if why_blocked_tokens
        .iter()
        .any(|t| t == "generator_unavailable")
    {
        steps.push(recovery_step(
            RecoveryClass::RestoreGenerator,
            "restore the generator".to_owned(),
        ));
    }
    if why_blocked_tokens
        .iter()
        .any(|t| t == "runtime_unavailable")
    {
        steps.push(recovery_step(
            RecoveryClass::ProvisionRuntime,
            "provision the required runtime".to_owned(),
        ));
    }
    if why_blocked_tokens
        .iter()
        .any(|t| t.starts_with("undeclared_side_effect_"))
    {
        steps.push(recovery_step(
            RecoveryClass::DeclareAndReviewSideEffect,
            "declare and review the side effect".to_owned(),
        ));
    }
    if why_blocked_tokens
        .iter()
        .any(|t| t == "regeneration_blocked_by_policy")
    {
        steps.push(recovery_step(
            RecoveryClass::ResolveRegenerationPolicy,
            "regeneration policy".to_owned(),
        ));
    }
    steps
}

fn guidance_for(
    readiness: PlanReadiness,
    runnable: u32,
    total: u32,
    boundary: &SideEffectBoundary,
) -> String {
    match readiness {
        PlanReadiness::Ready => format!(
            "Regeneration ready: all {total} target(s) will regenerate. {}",
            boundary.summary
        ),
        PlanReadiness::ReadyStaleInputs => format!(
            "Regeneration ready with stale inputs: all {total} target(s) will regenerate, but the result may not reflect the latest source. Refresh inputs first to be current."
        ),
        PlanReadiness::Partial => format!(
            "Partial regeneration: {runnable} of {total} target(s) will regenerate; the rest are blocked and are listed with the reason. This is not a complete regeneration."
        ),
        PlanReadiness::PolicyLimited => {
            if boundary.all_sensitive_declared {
                "Regeneration policy-limited: a policy blocks every target. Resolve the policy before regenerating; this is not a regeneration.".to_owned()
            } else {
                "Regeneration policy-limited: an undeclared side effect must be declared and reviewed before any target may regenerate. The regenerate action will not run silently.".to_owned()
            }
        }
        PlanReadiness::Blocked => {
            "Regeneration blocked: required source, generator, or runtime is missing for every target. Restore the missing material before regenerating; this is not a regeneration.".to_owned()
        }
    }
}

/// Computes the stable copy/export form for a plan.
pub fn regeneration_plan_copy_line(plan: &RegenerationPlan) -> String {
    copy_line_for(
        &plan.plan_id,
        plan.readiness,
        plan.targets.len() as u32,
        plan.runnable_target_count,
        plan.blocked_target_count,
        &plan.side_effect_boundary,
        plan.rollback_coverage,
        plan.stale_inputs,
    )
}

#[allow(clippy::too_many_arguments)]
fn copy_line_for(
    plan_id: &str,
    readiness: PlanReadiness,
    total: u32,
    runnable: u32,
    blocked: u32,
    boundary: &SideEffectBoundary,
    rollback_coverage: RollbackCoverage,
    stale_inputs: bool,
) -> String {
    format!(
        "regeneration-plan id={plan_id} readiness={} targets={total} runnable={runnable} blocked={blocked} side_effects={} undeclared={} rollback={} stale_inputs={stale_inputs}",
        readiness.as_str(),
        join_classes(&boundary.classes_present),
        !boundary.all_sensitive_declared,
        rollback_coverage.as_str(),
    )
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One regeneration-plan case: a request and the plan the engine stamps onto
/// it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlanCase {
    /// Stable case id.
    pub case_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The regeneration request the engine reads.
    pub request: RegenerationRequest,
    /// The plan the engine reached.
    pub plan: RegenerationPlan,
    /// Upstream generated-artifact packets backing this case.
    pub evidence_refs: Vec<String>,
    /// One real consumer that renders this case.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a surface ingests this packet rather than re-deriving
/// regeneration-plan truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlanSurfaceBinding {
    /// Surface that ingests the packet.
    pub surface: RegenerationPlanSurface,
    /// Checked consumer ref that renders the plan.
    pub consumer_ref: String,
    /// Packet id the surface ingests.
    pub ingested_packet_id: String,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlanSourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Proof packet ref.
    pub packet_ref: String,
    /// Certification summary ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet modeling regeneration plans across the claimed M5
/// generated-artifact classes and readiness states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlanPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: RegenerationPlanSourceContractRefs,
    /// The readiness states the packet models.
    pub readiness_states: Vec<PlanReadiness>,
    /// The side-effect classes the packet enumerates.
    pub side_effect_classes: Vec<SideEffectClass>,
    /// Upstream generated-artifact packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Regeneration-plan cases.
    pub cases: Vec<RegenerationPlanCase>,
    /// Surface bindings, one per rendered surface.
    pub surface_bindings: Vec<RegenerationPlanSurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a case to its expected plan, proving the canonical
/// planning behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegenerationPlanFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The case under test.
    pub case: RegenerationPlanCase,
    /// Expected plan readiness.
    pub expected_readiness: PlanReadiness,
    /// Expected runnable target count.
    pub expected_runnable_target_count: u32,
    /// Expected blocked target count.
    pub expected_blocked_target_count: u32,
    /// Expected why-blocked tokens.
    pub expected_why_blocked_tokens: Vec<String>,
    /// Expected rollback coverage.
    pub expected_rollback_coverage: RollbackCoverage,
    /// Whether the plan's sensitive side effects are all declared.
    pub expected_all_sensitive_declared: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "regeneration-plan validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Evidence-packet vocabulary used by the seed.
// ---------------------------------------------------------------------------

const GOVERNANCE_PACKET_REF: &str = "artifacts/generated/m5-generated-proof-packet.json";
const DESCRIPTOR_PACKET_REF: &str = "artifacts/generated/generated-artifact-descriptor-packet.json";
const WRITE_BOUNDARY_PACKET_REF: &str = "artifacts/generated/write-boundary-packet.json";
const DIVERGENCE_CONTRACT_REF: &str = "docs/generated/diverged_from_generator_contract.md";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";
const MUTATION_CLASSES_REF: &str = "artifacts/change/mutation_classes.yaml";
const RESTORE_PROVENANCE_REF: &str = "artifacts/migration/m3/restore_provenance_packet.md";

fn evidence_packet_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        WRITE_BOUNDARY_PACKET_REF,
        DIVERGENCE_CONTRACT_REF,
        ROLLBACK_CHECKPOINT_REF,
        MUTATION_CLASSES_REF,
        RESTORE_PROVENANCE_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn case_evidence_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        WRITE_BOUNDARY_PACKET_REF,
        ROLLBACK_CHECKPOINT_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn class_generator(artifact_class: ArtifactClass) -> GeneratorIdentity {
    let (kind, name, version) = match artifact_class {
        ArtifactClass::ScaffoldedProject => (GeneratorKind::Template, "rust-cli-starter", "1.4.0"),
        ArtifactClass::NotebookOutput => (GeneratorKind::Kernel, "python-kernel", "3.11.6"),
        ArtifactClass::PreviewDerivative => (GeneratorKind::Builder, "preview-bundler", "0.9.2"),
        ArtifactClass::RequestArtifact => (GeneratorKind::Runner, "request-runner", "2.3.1"),
        ArtifactClass::FrameworkCodegen => (GeneratorKind::Framework, "openapi-codegen", "5.0.0"),
        ArtifactClass::AiAssistedEdit => (GeneratorKind::Composer, "scoped-composer", "1.0.0"),
        ArtifactClass::SupportPacket => (GeneratorKind::Exporter, "support-exporter", "4.2.0"),
    };
    GeneratorIdentity {
        kind,
        name: name.to_owned(),
        version: version.to_owned(),
    }
}

fn class_authority(artifact_class: ArtifactClass) -> AuthorityClass {
    match artifact_class {
        ArtifactClass::ScaffoldedProject | ArtifactClass::AiAssistedEdit => {
            AuthorityClass::CanonicalAuthoritative
        }
        ArtifactClass::RequestArtifact | ArtifactClass::FrameworkCodegen => {
            AuthorityClass::DerivedEditable
        }
        ArtifactClass::NotebookOutput
        | ArtifactClass::PreviewDerivative
        | ArtifactClass::SupportPacket => AuthorityClass::DerivedReadonly,
    }
}

fn class_path_label(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "src/main.rs",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7-output",
        ArtifactClass::PreviewDerivative => ".preview/bundle.js",
        ArtifactClass::RequestArtifact => "requests/users.list.response.json",
        ArtifactClass::FrameworkCodegen => "generated/api_client.rs",
        ArtifactClass::AiAssistedEdit => "src/parser.rs",
        ArtifactClass::SupportPacket => "support/diagnostic-bundle.json",
    }
}

fn class_source_ref(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "templates/rust-cli-starter",
        ArtifactClass::NotebookOutput => "analysis.ipynb#cell-7",
        ArtifactClass::PreviewDerivative => "src/index.ts",
        ArtifactClass::RequestArtifact => "requests/users.list.request.json",
        ArtifactClass::FrameworkCodegen => "openapi/users.yaml",
        ArtifactClass::AiAssistedEdit => "src/parser.rs@checkpoint",
        ArtifactClass::SupportPacket => "workspace diagnostics snapshot",
    }
}

fn class_regeneration_route(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "re-run the project scaffold from its template",
        ArtifactClass::NotebookOutput => "re-run the notebook cell from its kernel",
        ArtifactClass::PreviewDerivative => "rebuild the preview bundle from source",
        ArtifactClass::RequestArtifact => "replay the saved request",
        ArtifactClass::FrameworkCodegen => "re-run the framework code generator",
        ArtifactClass::AiAssistedEdit => "re-run the scoped AI apply from its checkpoint",
        ArtifactClass::SupportPacket => "re-export the support packet",
    }
}

fn class_runtime_requirement(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "the scaffolding toolchain",
        ArtifactClass::NotebookOutput => "the Python 3.11 kernel runtime",
        ArtifactClass::PreviewDerivative => "the preview bundler runtime",
        ArtifactClass::RequestArtifact => "the request-runner runtime",
        ArtifactClass::FrameworkCodegen => "the code-generator runtime",
        ArtifactClass::AiAssistedEdit => "the AI composer runtime",
        ArtifactClass::SupportPacket => "the support-exporter runtime",
    }
}

fn class_consumer_ref(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => "crates/aureline-vfs/src/save_conflict_suite/mod.rs",
        ArtifactClass::NotebookOutput => "crates/aureline-vfs/src/save_conflict_suite/mod.rs",
        ArtifactClass::PreviewDerivative => "crates/aureline-review/src/change_inspector/mod.rs",
        ArtifactClass::RequestArtifact => "crates/aureline-review/src/change_inspector/mod.rs",
        ArtifactClass::FrameworkCodegen => {
            "crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs"
        }
        ArtifactClass::AiAssistedEdit => "crates/aureline-ai/src/context_inspector/mod.rs",
        ArtifactClass::SupportPacket => "crates/aureline-support/src/generated_lineage/mod.rs",
    }
}

/// A small builder for one regeneration target, used by the seed. The state
/// fields default to satisfied; the seed overrides exactly the ones a scenario
/// degrades.
struct TargetSpec {
    artifact_class: ArtifactClass,
    source_state: PreconditionState,
    generator_state: PreconditionState,
    runtime_state: PreconditionState,
    input_freshness: PreconditionState,
    policy_state: PreconditionState,
    side_effects: Vec<SideEffect>,
}

impl TargetSpec {
    fn healthy(artifact_class: ArtifactClass) -> Self {
        Self {
            artifact_class,
            source_state: PreconditionState::Satisfied,
            generator_state: PreconditionState::Satisfied,
            runtime_state: PreconditionState::Satisfied,
            input_freshness: PreconditionState::Satisfied,
            policy_state: PreconditionState::Satisfied,
            side_effects: vec![local_compute()],
        }
    }

    fn build(self) -> RegenerationTarget {
        let canonical_source_ref = if self.source_state == PreconditionState::Missing {
            None
        } else {
            Some(class_source_ref(self.artifact_class).to_owned())
        };
        RegenerationTarget {
            artifact_class: self.artifact_class,
            artifact_path_label: class_path_label(self.artifact_class).to_owned(),
            authority_class: class_authority(self.artifact_class),
            generator: class_generator(self.artifact_class),
            runtime_requirement: class_runtime_requirement(self.artifact_class).to_owned(),
            canonical_source_ref,
            regeneration_route: class_regeneration_route(self.artifact_class).to_owned(),
            source_state: self.source_state,
            generator_state: self.generator_state,
            runtime_state: self.runtime_state,
            input_freshness: self.input_freshness,
            policy_state: self.policy_state,
            side_effects: self.side_effects,
        }
    }
}

fn local_compute() -> SideEffect {
    SideEffect {
        class: SideEffectClass::LocalCompute,
        disclosure: SideEffectDisclosure::DeclaredReviewed,
        detail: SideEffectClass::LocalCompute.describe().to_owned(),
    }
}

fn side_effect(class: SideEffectClass, disclosure: SideEffectDisclosure) -> SideEffect {
    SideEffect {
        class,
        disclosure,
        detail: class.describe().to_owned(),
    }
}

fn rollback_boundary() -> RollbackBoundary {
    RollbackBoundary {
        checkpoint_ref: ROLLBACK_CHECKPOINT_REF.to_owned(),
        scope: "a reversible checkpoint of the target artifacts and their workspace neighborhood"
            .to_owned(),
    }
}

fn request(plan_id: &str, trigger_label: &str, specs: Vec<TargetSpec>) -> RegenerationRequest {
    RegenerationRequest {
        plan_id: plan_id.to_owned(),
        trigger_label: trigger_label.to_owned(),
        targets: specs.into_iter().map(TargetSpec::build).collect(),
        rollback_boundary: rollback_boundary(),
    }
}

fn case(
    case_id: &str,
    scenario: &str,
    consumer_class: ArtifactClass,
    request: RegenerationRequest,
    notes: &str,
) -> RegenerationPlanCase {
    let plan = plan_regeneration(&request);
    RegenerationPlanCase {
        case_id: case_id.to_owned(),
        scenario: scenario.to_owned(),
        request,
        plan,
        evidence_refs: case_evidence_refs(),
        consumer_ref: class_consumer_ref(consumer_class).to_owned(),
        notes: notes.to_owned(),
    }
}

fn binding(
    surface: RegenerationPlanSurface,
    consumer_ref: &str,
    summary: &str,
) -> RegenerationPlanSurfaceBinding {
    RegenerationPlanSurfaceBinding {
        surface,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: REGENERATION_PLAN_PACKET_ID.to_owned(),
        summary: summary.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the seeded regeneration-plan cases this lane freezes: one per
/// readiness state plus the no-silent-side-effects and declared-side-effect
/// flows.
fn seeded_cases() -> Vec<RegenerationPlanCase> {
    vec![
        // Ready: a single local-compute target with every precondition met.
        case(
            "regeneration-plan.scaffolded_project.ready",
            "Scaffolded project — ready, local compute only",
            ArtifactClass::ScaffoldedProject,
            request(
                "regeneration-plan.scaffolded_project.ready",
                "Regenerate the scaffolded project",
                vec![TargetSpec::healthy(ArtifactClass::ScaffoldedProject)],
            ),
            "Every precondition is satisfied and the only side effect is local compute, so the plan is fully ready and fully reversible.",
        ),
        // Ready with stale inputs: the source/inputs are stale but present.
        case(
            "regeneration-plan.notebook_output.ready_stale_inputs",
            "Notebook output — ready, but inputs are stale",
            ArtifactClass::NotebookOutput,
            request(
                "regeneration-plan.notebook_output.ready_stale_inputs",
                "Re-run the notebook cell",
                vec![TargetSpec {
                    input_freshness: PreconditionState::Stale,
                    ..TargetSpec::healthy(ArtifactClass::NotebookOutput)
                }],
            ),
            "The cell can regenerate, but its inputs are stale, so the plan is labeled ready-stale-inputs and does not masquerade as a clean, current result.",
        ),
        // Partial: one target ready, one blocked on a missing source.
        case(
            "regeneration-plan.framework_codegen.partial",
            "Framework codegen — partial: one target ready, one source missing",
            ArtifactClass::FrameworkCodegen,
            request(
                "regeneration-plan.framework_codegen.partial",
                "Regenerate the API client and its models",
                vec![
                    TargetSpec::healthy(ArtifactClass::FrameworkCodegen),
                    TargetSpec {
                        source_state: PreconditionState::Missing,
                        ..TargetSpec::healthy(ArtifactClass::RequestArtifact)
                    },
                ],
            ),
            "One target regenerates and one is blocked on a missing source, so the plan is partial and names exactly which target could not run.",
        ),
        // Blocked: the runtime the generator needs is unavailable.
        case(
            "regeneration-plan.preview_derivative.blocked_runtime",
            "Preview derivative — blocked: required runtime unavailable",
            ArtifactClass::PreviewDerivative,
            request(
                "regeneration-plan.preview_derivative.blocked_runtime",
                "Rebuild the preview bundle",
                vec![TargetSpec {
                    runtime_state: PreconditionState::Missing,
                    ..TargetSpec::healthy(ArtifactClass::PreviewDerivative)
                }],
            ),
            "The bundler runtime is unavailable, so nothing can regenerate; the plan is blocked and offers a provision-runtime recovery rather than a generic failure.",
        ),
        // Policy-limited: a policy forbids regeneration.
        case(
            "regeneration-plan.request_artifact.policy_limited",
            "Request artifact — policy-limited: regeneration forbidden by policy",
            ArtifactClass::RequestArtifact,
            request(
                "regeneration-plan.request_artifact.policy_limited",
                "Replay the saved request",
                vec![TargetSpec {
                    policy_state: PreconditionState::BlockedByPolicy,
                    ..TargetSpec::healthy(ArtifactClass::RequestArtifact)
                }],
            ),
            "A policy forbids replaying the request, so the plan is policy-limited and surfaces the policy block instead of running silently.",
        ),
        // Policy-limited via undeclared side effect: a hidden network install.
        case(
            "regeneration-plan.scaffolded_project.undeclared_side_effect",
            "Scaffolded project — held: undeclared networked install",
            ArtifactClass::ScaffoldedProject,
            request(
                "regeneration-plan.scaffolded_project.undeclared_side_effect",
                "Regenerate the scaffolded project",
                vec![TargetSpec {
                    side_effects: vec![
                        local_compute(),
                        side_effect(
                            SideEffectClass::NetworkInstall,
                            SideEffectDisclosure::Undeclared,
                        ),
                    ],
                    ..TargetSpec::healthy(ArtifactClass::ScaffoldedProject)
                }],
            ),
            "Regenerating would perform an undeclared networked install; the plan holds for disclosure so the regenerate action cannot hide the side effect.",
        ),
        // Ready with a declared sensitive side effect: honest partial rollback.
        case(
            "regeneration-plan.framework_codegen.ready_declared_install",
            "Framework codegen — ready, with a declared networked install",
            ArtifactClass::FrameworkCodegen,
            request(
                "regeneration-plan.framework_codegen.ready_declared_install",
                "Regenerate the API client",
                vec![TargetSpec {
                    side_effects: vec![
                        local_compute(),
                        side_effect(
                            SideEffectClass::NetworkInstall,
                            SideEffectDisclosure::DeclaredReviewed,
                        ),
                        side_effect(
                            SideEffectClass::ToolDownload,
                            SideEffectDisclosure::DeclaredReviewed,
                        ),
                    ],
                    ..TargetSpec::healthy(ArtifactClass::FrameworkCodegen)
                }],
            ),
            "The networked install and tool download are declared and reviewed, so the plan is ready — but because those writes escape the checkpoint, the rollback boundary is reported as only partially reversible.",
        ),
    ]
}

/// Returns the surface bindings this lane freezes.
fn seeded_surface_bindings() -> Vec<RegenerationPlanSurfaceBinding> {
    vec![
        binding(
            RegenerationPlanSurface::RegeneratePlanSheet,
            "crates/aureline-vfs/src/save_conflict_suite/mod.rs",
            "The product regenerate sheet renders the plan, its side-effect boundary, and its rollback coverage before execution, so a regenerate button never runs a blind command.",
        ),
        binding(
            RegenerationPlanSurface::HelpRegenerationGuide,
            "crates/aureline-shell/src/help/mod.rs",
            "The help guide explains the readiness states — ready, ready-stale-inputs, partial, policy-limited, and blocked — so the user can tell a complete regeneration from a degraded one.",
        ),
        binding(
            RegenerationPlanSurface::SupportExport,
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "The metadata-first support export re-emits the plan copy line, why-blocked tokens, and recovery path with no raw bytes, diffs, or credentials, so a support packet can explain why regeneration was unavailable or only partial after the fact.",
        ),
        binding(
            RegenerationPlanSurface::ReleaseEvidence,
            "crates/aureline-release/src/harden_docs_help_about_and_service_health_truth/mod.rs",
            "The release-evidence review preserves the plan packet so regeneration behavior — its side-effect boundary and rollback coverage — is inspectable in release evidence.",
        ),
        binding(
            RegenerationPlanSurface::AiContext,
            "crates/aureline-ai/src/context_inspector/mod.rs",
            "The AI context attaches the plan readiness and side-effect boundary so the model is told a regeneration is partial, blocked, or policy-limited instead of treating a regenerate as a guaranteed clean rebuild.",
        ),
    ]
}

/// Returns the checked-in regeneration-plan packet this lane freezes.
pub fn seeded_regeneration_plan_packet() -> RegenerationPlanPacket {
    RegenerationPlanPacket {
        record_kind: REGENERATION_PLAN_PACKET_RECORD_KIND.to_owned(),
        schema_version: REGENERATION_PLAN_SCHEMA_VERSION,
        packet_id: REGENERATION_PLAN_PACKET_ID.to_owned(),
        title: "Typed regeneration plans with visible partial, blocked, stale-input, and policy-limited outcomes for the M5 generated-artifact classes"
            .to_owned(),
        source_contract_refs: RegenerationPlanSourceContractRefs {
            doc_ref: REGENERATION_PLAN_DOC_REF.to_owned(),
            schema_ref: REGENERATION_PLAN_SCHEMA_REF.to_owned(),
            packet_ref: REGENERATION_PLAN_PACKET_REF.to_owned(),
            report_ref: REGENERATION_PLAN_REPORT_REF.to_owned(),
            fixture_manifest_ref: REGENERATION_PLAN_FIXTURE_MANIFEST_REF.to_owned(),
        },
        readiness_states: PlanReadiness::ALL.to_vec(),
        side_effect_classes: SideEffectClass::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        cases: seeded_cases(),
        surface_bindings: seeded_surface_bindings(),
        invariants: vec![
            "Every regenerate action resolves to a typed plan carrying its target artifacts, canonical-source refs, generator/runtime requirements, side-effect boundary, and rollback boundary before execution — never a bare command.".to_owned(),
            "Blocked, partial, stale-input, and policy-limited plans are labeled precisely by the readiness state and never masquerade as a clean success.".to_owned(),
            "A regeneration never hides a networked install, tool download, secret use, or broad filesystem write: any undeclared sensitive side effect holds the target for disclosure instead of running silently.".to_owned(),
            "The rollback coverage is computed from the side effects, so a regeneration that escapes the workspace checkpoint is reported as only partially reversible rather than implying a clean undo.".to_owned(),
            "Every plan that is not fully ready carries why-blocked tokens and a recovery path, so a block is never reduced to a generic failure; the plan and result packets are preserved in support exports and release evidence.".to_owned(),
        ],
    }
}

/// Returns the checked-in regeneration-plan fixture corpus this lane freezes:
/// one fixture per seeded case.
pub fn seeded_regeneration_plan_fixtures() -> Vec<RegenerationPlanFixture> {
    seeded_cases().into_iter().map(fixture).collect()
}

fn fixture(case: RegenerationPlanCase) -> RegenerationPlanFixture {
    let plan = &case.plan;
    RegenerationPlanFixture {
        record_kind: REGENERATION_PLAN_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: REGENERATION_PLAN_SCHEMA_VERSION,
        fixture_id: format!("fixture.{}", case.case_id),
        scenario: case.scenario.clone(),
        expected_readiness: plan.readiness,
        expected_runnable_target_count: plan.runnable_target_count,
        expected_blocked_target_count: plan.blocked_target_count,
        expected_why_blocked_tokens: plan.why_blocked_tokens.clone(),
        expected_rollback_coverage: plan.rollback_coverage,
        expected_all_sensitive_declared: plan.side_effect_boundary.all_sensitive_declared,
        notes: case.notes.clone(),
        case,
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in regeneration-plan packet contract.
pub fn validate_regeneration_plan_packet(
    packet: &RegenerationPlanPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != REGENERATION_PLAN_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != REGENERATION_PLAN_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != REGENERATION_PLAN_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != REGENERATION_PLAN_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != REGENERATION_PLAN_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != REGENERATION_PLAN_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != REGENERATION_PLAN_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != REGENERATION_PLAN_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.readiness_states != PlanReadiness::ALL.to_vec() {
        report.push(
            "packet.readiness_states",
            "packet must declare every readiness state in canonical order",
        );
    }
    if packet.side_effect_classes != SideEffectClass::ALL.to_vec() {
        report.push(
            "packet.side_effect_classes",
            "packet must enumerate every side-effect class in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the upstream generated-artifact evidence packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut case_ids = BTreeSet::new();
    let mut covered_readiness = BTreeSet::new();
    let mut covered_outcomes = BTreeSet::new();
    for plan_case in &packet.cases {
        if !case_ids.insert(plan_case.case_id.as_str()) {
            report.push(
                "case.id_unique",
                format!("duplicate case id {}", plan_case.case_id),
            );
        }
        covered_readiness.insert(plan_case.plan.readiness);
        for target in &plan_case.plan.targets {
            covered_outcomes.insert(target.outcome);
        }
        validate_case(&mut report, plan_case);
    }
    for required in PlanReadiness::ALL {
        if !covered_readiness.contains(&required) {
            report.push(
                "packet.readiness_coverage",
                format!("packet must cover readiness state {}", required.as_str()),
            );
        }
    }
    for required in TargetOutcome::ALL {
        if !covered_outcomes.contains(&required) {
            report.push(
                "packet.outcome_coverage",
                format!("packet must cover target outcome {}", required.as_str()),
            );
        }
    }

    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_case(report: &mut ValidationReport, plan_case: &RegenerationPlanCase) {
    let owner = format!("case {}", plan_case.case_id);

    if plan_case.case_id.trim().is_empty() {
        report.push("case.id", "case must carry a stable id");
    }
    if plan_case.scenario.trim().is_empty() {
        report.push("case.scenario", format!("{owner} must carry a scenario"));
    }
    if plan_case.consumer_ref.trim().is_empty() {
        report.push(
            "case.consumer_ref",
            format!("{owner} must cite a consumer ref"),
        );
    }
    if plan_case.notes.trim().is_empty() {
        report.push("case.notes", format!("{owner} must carry a reviewer note"));
    }
    if plan_case.evidence_refs.is_empty() {
        report.push(
            "case.evidence_refs",
            format!("{owner} must cite at least one evidence ref"),
        );
    }

    validate_request(report, &owner, &plan_case.request);

    // The stamped plan must equal what the engine computes.
    let expected = plan_regeneration(&plan_case.request);
    if plan_case.plan != expected {
        report.push(
            "case.plan",
            format!("{owner} stamped plan disagrees with the engine"),
        );
    }

    validate_plan(report, &owner, &plan_case.plan);
}

fn validate_request(report: &mut ValidationReport, owner: &str, request: &RegenerationRequest) {
    if request.plan_id.trim().is_empty() {
        report.push("request.plan_id", format!("{owner} must carry a plan id"));
    }
    if request.trigger_label.trim().is_empty() {
        report.push(
            "request.trigger_label",
            format!("{owner} must carry a trigger label"),
        );
    }
    if request.targets.is_empty() {
        report.push(
            "request.targets",
            format!("{owner} must carry at least one target"),
        );
    }
    if request.rollback_boundary.checkpoint_ref.trim().is_empty() {
        report.push(
            "request.rollback_checkpoint",
            format!("{owner} must carry a rollback checkpoint ref"),
        );
    }
    for target in &request.targets {
        if target.artifact_path_label.trim().is_empty() {
            report.push(
                "target.path_label",
                format!("{owner} target must carry an artifact path label"),
            );
        }
        if target.generator.name.trim().is_empty() || target.generator.version.trim().is_empty() {
            report.push(
                "target.generator",
                format!("{owner} target must carry a generator name and version"),
            );
        }
        if target.runtime_requirement.trim().is_empty() {
            report.push(
                "target.runtime_requirement",
                format!("{owner} target must carry a runtime requirement"),
            );
        }
        if target.regeneration_route.trim().is_empty() {
            report.push(
                "target.regeneration_route",
                format!("{owner} target must carry a regeneration route"),
            );
        }
        // The source ref must agree with the source state.
        match target.source_state {
            PreconditionState::Missing => {
                if target.canonical_source_ref.is_some() {
                    report.push(
                        "target.source_consistency",
                        format!("{owner} a missing-source target must not carry a source ref"),
                    );
                }
            }
            _ => {
                if target
                    .canonical_source_ref
                    .as_ref()
                    .map(|s| s.trim().is_empty())
                    .unwrap_or(true)
                {
                    report.push(
                        "target.source_consistency",
                        format!("{owner} a present-source target must carry a source ref"),
                    );
                }
            }
        }
        // Each precondition state must be meaningful for its precondition.
        let pairs = [
            (PreconditionKind::CanonicalSource, target.source_state),
            (PreconditionKind::Generator, target.generator_state),
            (PreconditionKind::Runtime, target.runtime_state),
            (PreconditionKind::InputFreshness, target.input_freshness),
            (PreconditionKind::Policy, target.policy_state),
        ];
        for (kind, state) in pairs {
            if !kind.allows(state) {
                report.push(
                    "target.precondition_domain",
                    format!(
                        "{owner} precondition {} cannot hold state {}",
                        kind.as_str(),
                        state.as_str()
                    ),
                );
            }
        }
        for effect in &target.side_effects {
            if effect.detail.trim().is_empty() {
                report.push(
                    "target.side_effect_detail",
                    format!("{owner} a side effect must carry a review-safe detail"),
                );
            }
        }
    }
}

fn validate_plan(report: &mut ValidationReport, owner: &str, plan: &RegenerationPlan) {
    // Counts must be consistent.
    if plan.runnable_target_count + plan.blocked_target_count != plan.targets.len() as u32 {
        report.push(
            "plan.counts",
            format!("{owner} runnable + blocked target counts must equal the target count"),
        );
    }
    let runnable = plan.targets.iter().filter(|t| t.outcome.will_run()).count() as u32;
    if plan.runnable_target_count != runnable {
        report.push(
            "plan.runnable_count",
            format!("{owner} runnable target count disagrees with the target outcomes"),
        );
    }

    // Readiness must agree with runs_in_full / partial flags.
    if plan.runs_in_full != plan.readiness.runs_in_full() {
        report.push(
            "plan.runs_in_full",
            format!("{owner} runs_in_full disagrees with the readiness"),
        );
    }
    if plan.partial != (plan.readiness == PlanReadiness::Partial) {
        report.push(
            "plan.partial",
            format!("{owner} partial flag disagrees with the readiness"),
        );
    }

    // Block reasons and recovery: a not-fully-ready plan must explain itself,
    // a clean plan must not.
    match plan.readiness {
        PlanReadiness::Ready => {
            if !plan.why_blocked_tokens.is_empty() {
                report.push(
                    "plan.ready_no_block",
                    format!("{owner} a fully ready plan must carry no why-blocked tokens"),
                );
            }
            if !plan.recovery.is_empty() {
                report.push(
                    "plan.ready_no_recovery",
                    format!("{owner} a fully ready plan needs no recovery path"),
                );
            }
        }
        PlanReadiness::ReadyStaleInputs => {
            // Stale inputs are surfaced via the readiness state and a refresh
            // recovery, but are not a block.
            if plan.recovery.is_empty() {
                report.push(
                    "plan.stale_recovery",
                    format!("{owner} a stale-inputs plan must offer a refresh recovery"),
                );
            }
        }
        PlanReadiness::Partial | PlanReadiness::PolicyLimited | PlanReadiness::Blocked => {
            if plan.why_blocked_tokens.is_empty() {
                report.push(
                    "plan.block_reason",
                    format!("{owner} a partial or blocked plan must name why a target was blocked"),
                );
            }
            if plan.recovery.is_empty() {
                report.push(
                    "plan.recovery",
                    format!("{owner} a partial or blocked plan must offer a recovery path"),
                );
            }
            if plan.guidance_line.trim().is_empty() {
                report.push(
                    "plan.guidance",
                    format!("{owner} a partial or blocked plan must carry a guidance line"),
                );
            }
        }
    }

    // stale_inputs flag must match the preconditions.
    let any_stale = plan.targets.iter().any(|t| {
        t.preconditions
            .iter()
            .any(|p| p.state == PreconditionState::Stale)
    });
    if plan.stale_inputs != any_stale {
        report.push(
            "plan.stale_flag",
            format!("{owner} stale_inputs flag disagrees with the target preconditions"),
        );
    }

    // Rollback coverage must follow from the side effects.
    let expected_coverage =
        RollbackCoverage::for_classes(plan.side_effect_boundary.classes_present.iter().copied());
    if plan.rollback_coverage != expected_coverage {
        report.push(
            "plan.rollback_coverage",
            format!("{owner} rollback coverage disagrees with the side-effect boundary"),
        );
    }

    validate_targets(report, owner, plan);
    validate_side_effect_boundary(report, owner, plan);

    if plan.copy_line != regeneration_plan_copy_line(plan) {
        report.push(
            "plan.copy_line",
            format!("{owner} stamped copy line disagrees with the engine"),
        );
    }
}

fn validate_targets(report: &mut ValidationReport, owner: &str, plan: &RegenerationPlan) {
    for target in &plan.targets {
        let kinds: Vec<_> = target.preconditions.iter().map(|p| p.kind).collect();
        if kinds != PreconditionKind::ALL.to_vec() {
            report.push(
                "target.preconditions",
                format!(
                    "{owner} target {} must carry every precondition in canonical order",
                    target.artifact_path_label
                ),
            );
        }
        // A running target carries no block tokens; a blocked or held one
        // must.
        if target.outcome.will_run() {
            if !target.why_blocked_tokens.is_empty() {
                report.push(
                    "target.run_no_block",
                    format!(
                        "{owner} running target {} must carry no why-blocked tokens",
                        target.artifact_path_label
                    ),
                );
            }
        } else if target.why_blocked_tokens.is_empty() {
            report.push(
                "target.block_reason",
                format!(
                    "{owner} blocked target {} must name why it was blocked",
                    target.artifact_path_label
                ),
            );
        }
        for status in &target.preconditions {
            if status.detail.trim().is_empty() {
                report.push(
                    "target.precondition_detail",
                    format!(
                        "{owner} precondition {} must carry a review-safe detail",
                        status.kind.as_str()
                    ),
                );
            }
        }
    }
}

fn validate_side_effect_boundary(
    report: &mut ValidationReport,
    owner: &str,
    plan: &RegenerationPlan,
) {
    let boundary = &plan.side_effect_boundary;

    // The aggregated classes must be the distinct classes across the targets,
    // in canonical order.
    let mut expected_classes: Vec<SideEffectClass> = Vec::new();
    for class in SideEffectClass::ALL {
        if plan
            .targets
            .iter()
            .any(|t| t.side_effects.iter().any(|e| e.class == class))
        {
            expected_classes.push(class);
        }
    }
    if boundary.classes_present != expected_classes {
        report.push(
            "boundary.classes",
            format!("{owner} side-effect boundary classes disagree with the targets"),
        );
    }

    // all_sensitive_declared must be consistent with the undeclared list.
    if boundary.all_sensitive_declared != boundary.undeclared_sensitive_classes.is_empty() {
        report.push(
            "boundary.declared_flag",
            format!("{owner} all_sensitive_declared disagrees with the undeclared list"),
        );
    }

    // An undeclared sensitive side effect must hold at least one target for
    // disclosure — it can never silently run.
    if !boundary.all_sensitive_declared {
        let held = plan
            .targets
            .iter()
            .any(|t| t.outcome == TargetOutcome::HeldForDisclosure || t.outcome.is_hard_block());
        if !held {
            report.push(
                "boundary.no_silent_side_effect",
                format!(
                    "{owner} an undeclared sensitive side effect must hold a target, never run silently"
                ),
            );
        }
    }

    for class in &boundary.undeclared_sensitive_classes {
        if !class.is_sensitive() {
            report.push(
                "boundary.undeclared_sensitive",
                format!(
                    "{owner} a non-sensitive class {} cannot be undeclared-sensitive",
                    class.as_str()
                ),
            );
        }
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &RegenerationPlanPacket) {
    let mut surfaces = BTreeSet::new();
    for surface_binding in &packet.surface_bindings {
        surfaces.insert(surface_binding.surface);
        if surface_binding.ingested_packet_id != packet.packet_id {
            report.push(
                "binding.packet_id",
                format!(
                    "binding for {} must ingest the packet id",
                    surface_binding.surface.as_str()
                ),
            );
        }
        if surface_binding.consumer_ref.trim().is_empty()
            || surface_binding.summary.trim().is_empty()
        {
            report.push(
                "binding.prose",
                format!(
                    "binding for {} must carry a consumer ref and summary",
                    surface_binding.surface.as_str()
                ),
            );
        }
    }
    for required in RegenerationPlanSurface::ALL {
        if !surfaces.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind surface {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in regeneration-plan fixture against the frozen
/// contract.
pub fn validate_regeneration_plan_fixture(
    fixture: &RegenerationPlanFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != REGENERATION_PLAN_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != REGENERATION_PLAN_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.push("fixture.id", "fixture must carry a stable id");
    }
    if fixture.scenario.trim().is_empty() {
        report.push(
            "fixture.scenario",
            format!("fixture {} must carry a scenario label", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        report.push(
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    validate_case(&mut report, &fixture.case);

    let plan = &fixture.case.plan;
    if fixture.expected_readiness != plan.readiness {
        report.push(
            "fixture.expected_readiness",
            format!(
                "fixture {} expected readiness disagrees with the plan",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_runnable_target_count != plan.runnable_target_count {
        report.push(
            "fixture.expected_runnable_target_count",
            format!(
                "fixture {} expected runnable count disagrees with the plan",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_blocked_target_count != plan.blocked_target_count {
        report.push(
            "fixture.expected_blocked_target_count",
            format!(
                "fixture {} expected blocked count disagrees with the plan",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_why_blocked_tokens != plan.why_blocked_tokens {
        report.push(
            "fixture.expected_why_blocked_tokens",
            format!(
                "fixture {} expected why-blocked tokens disagree with the plan",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_rollback_coverage != plan.rollback_coverage {
        report.push(
            "fixture.expected_rollback_coverage",
            format!(
                "fixture {} expected rollback coverage disagrees with the plan",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_all_sensitive_declared != plan.side_effect_boundary.all_sensitive_declared {
        report.push(
            "fixture.expected_all_sensitive_declared",
            format!(
                "fixture {} expected declared flag disagrees with the plan",
                fixture.fixture_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
