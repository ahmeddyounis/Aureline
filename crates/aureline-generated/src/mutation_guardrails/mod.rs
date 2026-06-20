//! AI-apply, refactor, quick-fix, and automation mutation guardrails for
//! generated artifacts that cross a canonical-source boundary.
//!
//! The sibling [`crate::write_boundary`] lane decides what happens when a
//! *user* attempts a direct edit to a generated file. This lane governs the
//! *automated* mutation routes — AI apply, refactor, quick fix, and
//! automation — that can reach the same generated artifacts. Without one
//! typed decision, an AI apply or an automation pass could mutate a derived
//! file as if it were ordinary user-authored source: silently, with no
//! preview, no side-effect summary, no regeneration awareness, and no
//! rollback class.
//!
//! Each [`MutationAttempt`] pairs the automated route, its actor lineage
//! (source class, actor reference, mutation class, reversal class), the target
//! artifact's [`WriteBoundarySubject`], whether the artifact carries
//! canonical-source boundary data at all, and the
//! [`MutationSafetyEnvelope`] the route brought — with the
//! [`MutationGuardrailDecision`] the single [`decide_mutation_guardrail`]
//! engine reaches. The decision is the one object the AI apply gate, the
//! refactor transaction, the automation runner, the mutation journal, and the
//! support export all render, so no surface can disagree about whether an
//! automated mutation is admitted, held, or blocked.
//!
//! Four guardrails are frozen here:
//!
//! - **No silent mutation of a generated artifact.** An automated route never
//!   mutates a non-authoritative generated artifact as if it were ordinary
//!   source. The mutation is admitted directly only when the artifact is its
//!   own canonical source and in sync; otherwise it is held for a reviewed
//!   override, blocked in favor of regeneration, or blocked outright when the
//!   canonical-source boundary data is missing.
//! - **Boundary data or no crossing.** When the target carries no
//!   canonical-source boundary data — no proven generator, source linkage, or
//!   edit posture — the route cannot classify it, so it is blocked
//!   ([`GuardrailOutcome::BlockedMissingBoundaryData`]) instead of being
//!   treated as ordinary source or admitted through an override.
//! - **A complete safety envelope for any allowed crossing.** A cross-boundary
//!   mutation is admitted only with all four [`SafetyRequirement`]s satisfied —
//!   a preview, a reviewed side-effect summary, regeneration awareness, and a
//!   rollback class — *and* a recorded reviewed override. Any unmet
//!   requirement holds the mutation for review and names what is missing.
//! - **Reuse, not a hidden mutation path.** The decision composes the existing
//!   [`crate::write_boundary`] decision and the
//!   [`crate::regeneration_plan`] side-effect / rollback vocabulary, and
//!   records actor lineage and mutation class against the shared
//!   mutation-journal contract, so support and audit packets can explain
//!   exactly which route crossed the canonical boundary and under what posture.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/mutation-guardrails.schema.json`](../../../../schemas/generated/mutation-guardrails.schema.json)
//! - [`/docs/generated/mutation-guardrails.md`](../../../../docs/generated/mutation-guardrails.md)
//! - [`/artifacts/generated/mutation-guardrails-packet.json`](../../../../artifacts/generated/mutation-guardrails-packet.json)
//! - [`/artifacts/generated/mutation-guardrails.md`](../../../../artifacts/generated/mutation-guardrails.md)
//! - [`/fixtures/generated/mutation-guardrails/`](../../../../fixtures/generated/mutation-guardrails/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::descriptor::{GeneratorIdentity, GeneratorKind};
use crate::m5_generated_governance::{ArtifactClass, AuthorityClass, EditPosture};
use crate::regeneration_plan::{RollbackCoverage, SideEffectClass, SideEffectDisclosure};
use crate::write_boundary::{
    decide_write_boundary, AttemptOutcome, BoundaryState, WriteBoundaryDecision,
    WriteBoundarySubject,
};

/// Schema version stamped onto the packet and fixtures.
pub const MUTATION_GUARDRAILS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const MUTATION_GUARDRAILS_PACKET_RECORD_KIND: &str = "mutation_guardrails_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const MUTATION_GUARDRAILS_FIXTURE_RECORD_KIND: &str = "mutation_guardrails_fixture_record";

/// Stable packet id every surface binding ingests.
pub const MUTATION_GUARDRAILS_PACKET_ID: &str = "generated.mutation_guardrails.v1";

/// Repo-relative schema ref.
pub const MUTATION_GUARDRAILS_SCHEMA_REF: &str =
    "schemas/generated/mutation-guardrails.schema.json";

/// Repo-relative reviewer doc ref.
pub const MUTATION_GUARDRAILS_DOC_REF: &str = "docs/generated/mutation-guardrails.md";

/// Repo-relative machine-readable proof packet.
pub const MUTATION_GUARDRAILS_PACKET_REF: &str =
    "artifacts/generated/mutation-guardrails-packet.json";

/// Repo-relative reviewer certification summary.
pub const MUTATION_GUARDRAILS_REPORT_REF: &str = "artifacts/generated/mutation-guardrails.md";

/// Repo-relative fixture directory.
pub const MUTATION_GUARDRAILS_FIXTURE_DIR: &str = "fixtures/generated/mutation-guardrails";

/// Repo-relative fixture manifest.
pub const MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/mutation-guardrails/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// An automated mutation route that can reach a generated artifact. These are
/// the four paths the guardrail governs: an AI apply, a refactor transaction,
/// a quick fix / code action, and an automation pass. A direct human edit is
/// covered by the sibling [`crate::write_boundary`] lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationRoute {
    /// An AI-assisted patch apply.
    AiApply,
    /// A refactor transaction (rename symbol, organize imports, extract).
    Refactor,
    /// A quick fix / code action.
    QuickFix,
    /// An automation pass (a task runner, agent, or codegen runner).
    Automation,
}

impl MutationRoute {
    /// Every route in canonical order.
    pub const ALL: [Self; 4] = [
        Self::AiApply,
        Self::Refactor,
        Self::QuickFix,
        Self::Automation,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiApply => "ai_apply",
            Self::Refactor => "refactor",
            Self::QuickFix => "quick_fix",
            Self::Automation => "automation",
        }
    }

    /// A short surface-agnostic label.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::AiApply => "AI apply",
            Self::Refactor => "Refactor",
            Self::QuickFix => "Quick fix",
            Self::Automation => "Automation",
        }
    }

    /// The mutation-journal `actor_class` token this route records, so a
    /// guardrail decision attributes the mutation against the shared journal
    /// contract instead of inventing a route-specific actor vocabulary.
    pub const fn actor_class_token(self) -> &'static str {
        match self {
            Self::AiApply => "ai_apply",
            Self::Refactor => "refactor_engine",
            Self::QuickFix => "code_action",
            Self::Automation => "build_runner",
        }
    }
}

/// The source class of the actor performing a mutation. A review-safe subset
/// of the shared mutation-journal `source_class` vocabulary, naming where the
/// automated route ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationSourceClass {
    /// A hosted AI provider produced the change.
    AiHostedProvider,
    /// A local AI model produced the change.
    AiLocalModel,
    /// A local machine process produced the change.
    MachineLocal,
    /// A remote machine agent produced the change.
    MachineRemoteAgent,
    /// A policy-driven automation produced the change.
    PolicyDriven,
}

impl MutationSourceClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiHostedProvider => "ai_hosted_provider",
            Self::AiLocalModel => "ai_local_model",
            Self::MachineLocal => "machine_local",
            Self::MachineRemoteAgent => "machine_remote_agent",
            Self::PolicyDriven => "policy_driven",
        }
    }
}

/// The family of authority a mutation touched. The closed vocabulary mirrors
/// the shared mutation-class matrix so preview, journaling, reversal, and
/// support-export behavior stays coherent across routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationClass {
    /// In-memory text mutation inside a buffer authority.
    BufferText,
    /// A durable filesystem mutation.
    Filesystem,
    /// An interpreted plan applied to code or structured state (refactor, code
    /// action, AI patch apply, scaffolding step).
    SemanticTooling,
    /// A mutation of generated/derived state (regeneration, recompute).
    GeneratedState,
    /// A mutation with an effect beyond the workspace (install, network, tool
    /// download).
    ExternalEffect,
}

impl MutationClass {
    /// Every mutation class in canonical order.
    pub const ALL: [Self; 5] = [
        Self::BufferText,
        Self::Filesystem,
        Self::SemanticTooling,
        Self::GeneratedState,
        Self::ExternalEffect,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BufferText => "buffer_text",
            Self::Filesystem => "filesystem",
            Self::SemanticTooling => "semantic_tooling",
            Self::GeneratedState => "generated_state",
            Self::ExternalEffect => "external_effect",
        }
    }
}

/// The reversal class a mutation declares. The closed vocabulary mirrors the
/// shared mutation-journal `reversal_class` so the recorded rollback posture is
/// the same one the journal and undo surfaces understand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversalClass {
    /// The mutation can be reverted exactly.
    ExactUndo,
    /// The mutation can be reverted by a compensating inverse.
    CompensatingUndo,
    /// The mutation is reverted by regenerating or recomputing the artifact.
    RegenerateOrRecompute,
    /// The mutation is reverted by restoring a reversible checkpoint.
    RestoreFromCheckpoint,
    /// The mutation cannot be reverted; only an audit record remains.
    AuditOnly,
}

impl ReversalClass {
    /// Every reversal class in canonical order.
    pub const ALL: [Self; 5] = [
        Self::ExactUndo,
        Self::CompensatingUndo,
        Self::RegenerateOrRecompute,
        Self::RestoreFromCheckpoint,
        Self::AuditOnly,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::CompensatingUndo => "compensating_undo",
            Self::RegenerateOrRecompute => "regenerate_or_recompute",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::AuditOnly => "audit_only",
        }
    }

    /// Whether this reversal class actually provides a rollback. Audit-only is
    /// not rollback-safe, so it never satisfies the rollback-class
    /// requirement on a cross-boundary mutation.
    pub const fn provides_rollback(self) -> bool {
        !matches!(self, Self::AuditOnly)
    }
}

/// Whether the target artifact carries canonical-source boundary data at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryDataState {
    /// The artifact carries a proven canonical-source boundary descriptor —
    /// generator identity, source linkage, and edit posture — so the
    /// write-boundary engine can classify it.
    Present,
    /// The artifact carries no canonical-source boundary data, so it cannot be
    /// classified and an automated route must not mutate it.
    Missing,
}

impl BoundaryDataState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Missing => "missing",
        }
    }
}

/// The outcome the guardrail engine reaches for one automated mutation
/// attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardrailOutcome {
    /// The mutation is admitted directly: the target is its own canonical
    /// source and in sync, so the route edits it as ordinary source. The
    /// mutation is still recorded with actor lineage and mutation class.
    AdmittedDirect,
    /// The cross-boundary mutation is admitted: the safety envelope is complete
    /// and a reviewed override is recorded, so it proceeds and leaves a durable
    /// diverged-from-generator state.
    AdmittedWithPreviewAndOverride,
    /// The cross-boundary mutation is held: it lacks a complete safety envelope
    /// or a reviewed override, so it cannot proceed until both are supplied.
    BlockedPendingReview,
    /// The mutation is blocked in favor of regeneration: the artifact is
    /// regenerated from its canonical source rather than mutated in place.
    BlockedRegenerateFirst,
    /// The mutation is blocked because the artifact carries no canonical-source
    /// boundary data, so the route cannot classify it or offer a safe override.
    BlockedMissingBoundaryData,
}

impl GuardrailOutcome {
    /// Every outcome in canonical order.
    pub const ALL: [Self; 5] = [
        Self::AdmittedDirect,
        Self::AdmittedWithPreviewAndOverride,
        Self::BlockedPendingReview,
        Self::BlockedRegenerateFirst,
        Self::BlockedMissingBoundaryData,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdmittedDirect => "admitted_direct",
            Self::AdmittedWithPreviewAndOverride => "admitted_with_preview_and_override",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::BlockedRegenerateFirst => "blocked_regenerate_first",
            Self::BlockedMissingBoundaryData => "blocked_missing_boundary_data",
        }
    }

    /// Whether this outcome lets the mutation reach the artifact (directly or
    /// as a reviewed override).
    pub const fn admits_mutation(self) -> bool {
        matches!(
            self,
            Self::AdmittedDirect | Self::AdmittedWithPreviewAndOverride
        )
    }

    /// Whether this outcome is a plain, unreviewed direct edit.
    pub const fn is_direct(self) -> bool {
        matches!(self, Self::AdmittedDirect)
    }

    /// Whether this outcome leaves a durable diverged-from-generator state.
    pub const fn leaves_divergence(self) -> bool {
        matches!(self, Self::AdmittedWithPreviewAndOverride)
    }
}

/// One requirement an allowed cross-boundary mutation must satisfy. All four
/// are required together: a preview, a reviewed side-effect summary,
/// regeneration awareness, and a rollback class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyRequirement {
    /// A preview of the change the route would apply.
    Preview,
    /// A reviewed summary of the side effects the mutation would perform.
    SideEffectSummary,
    /// An acknowledgement that the artifact is regenerated, so the change may
    /// be clobbered by the next regeneration.
    RegenerationAwareness,
    /// A declared rollback class that can actually reverse the mutation.
    RollbackClass,
}

impl SafetyRequirement {
    /// Every requirement in canonical order.
    pub const ALL: [Self; 4] = [
        Self::Preview,
        Self::SideEffectSummary,
        Self::RegenerationAwareness,
        Self::RollbackClass,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preview => "preview",
            Self::SideEffectSummary => "side_effect_summary",
            Self::RegenerationAwareness => "regeneration_awareness",
            Self::RollbackClass => "rollback_class",
        }
    }

    /// The stable why-blocked token an unmet requirement contributes.
    pub const fn unmet_token(self) -> &'static str {
        match self {
            Self::Preview => "missing_preview",
            Self::SideEffectSummary => "undeclared_side_effects",
            Self::RegenerationAwareness => "regeneration_not_acknowledged",
            Self::RollbackClass => "no_rollback_class",
        }
    }

    /// A short reviewer summary of the requirement.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::Preview => "a preview of the change the route would apply",
            Self::SideEffectSummary => "a reviewed summary of the mutation's side effects",
            Self::RegenerationAwareness => {
                "an acknowledgement that the artifact is regenerated and may be clobbered"
            }
            Self::RollbackClass => "a declared rollback class that can reverse the mutation",
        }
    }
}

/// A surface that renders the mutation-guardrail decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationGuardrailSurface {
    /// The AI scoped-apply gate.
    AiApplyGate,
    /// The refactor / change-object transaction.
    RefactorTransaction,
    /// The automation runner that drives codegen and tasks.
    AutomationRunner,
    /// The mutation journal that records the change with actor lineage.
    MutationJournal,
    /// The metadata-first support export.
    SupportExport,
}

impl MutationGuardrailSurface {
    /// Every rendered surface in canonical order.
    pub const ALL: [Self; 5] = [
        Self::AiApplyGate,
        Self::RefactorTransaction,
        Self::AutomationRunner,
        Self::MutationJournal,
        Self::SupportExport,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiApplyGate => "ai_apply_gate",
            Self::RefactorTransaction => "refactor_transaction",
            Self::AutomationRunner => "automation_runner",
            Self::MutationJournal => "mutation_journal",
            Self::SupportExport => "support_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Attempt, envelope, and decision.
// ---------------------------------------------------------------------------

/// The safety envelope an automated route brings to a mutation: the preview,
/// declared side effects, regeneration awareness, rollback class, and any
/// recorded reviewed override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationSafetyEnvelope {
    /// Review-safe reference to the preview of the change, when one is
    /// supplied.
    pub preview_ref: Option<String>,
    /// The side-effect classes the mutation would perform.
    pub side_effects: Vec<SideEffectClass>,
    /// Whether the side effects were declared and reviewed.
    pub side_effect_disclosure: SideEffectDisclosure,
    /// Whether the route acknowledged that the artifact is regenerated and the
    /// change may be clobbered.
    pub regeneration_acknowledged: bool,
    /// The reversal class the route declares for the mutation.
    pub reversal_class: ReversalClass,
    /// Reference to a recorded reviewed override, when one was supplied.
    pub override_review_ref: Option<String>,
}

/// The inputs the guardrail engine reads for one automated mutation attempt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationAttempt {
    /// Stable attempt id.
    pub attempt_id: String,
    /// The automated route performing the mutation.
    pub route: MutationRoute,
    /// Source class of the actor.
    pub source_class: MutationSourceClass,
    /// Review-safe actor reference (e.g. `ai/scoped-composer@1.0.0`).
    pub actor_ref: String,
    /// The family of authority the mutation touches.
    pub mutation_class: MutationClass,
    /// Whether the target carries canonical-source boundary data at all.
    pub boundary_data_state: BoundaryDataState,
    /// The target artifact's write-boundary subject.
    pub artifact: WriteBoundarySubject,
    /// The safety envelope the route brought.
    pub envelope: MutationSafetyEnvelope,
}

/// The actor lineage a guardrail decision records, so support and audit packets
/// can explain how a generated artifact was changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActorLineage {
    /// The automated route.
    pub route: MutationRoute,
    /// The mutation-journal `actor_class` token the route records.
    pub actor_class: String,
    /// Source class of the actor.
    pub source_class: MutationSourceClass,
    /// Review-safe actor reference.
    pub actor_ref: String,
    /// The mutation class recorded for the change.
    pub mutation_class: MutationClass,
    /// The reversal class recorded for the change.
    pub reversal_class: ReversalClass,
    /// Review-safe summary of the lineage.
    pub summary: String,
}

/// The computed mutation-guardrail decision the surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailDecision {
    /// The automated route.
    pub route: MutationRoute,
    /// The mutation class the change touches.
    pub mutation_class: MutationClass,
    /// Whether the target carries canonical-source boundary data.
    pub boundary_data_state: BoundaryDataState,
    /// The guardrail outcome.
    pub guardrail_outcome: GuardrailOutcome,
    /// Convenience: whether the mutation reaches the artifact.
    pub mutation_admitted: bool,
    /// Whether the mutation crosses a canonical-source boundary.
    pub crosses_canonical_boundary: bool,
    /// The effective edit gate after the boundary state floors the declared
    /// posture; a missing-boundary-data target floors to regenerate-only.
    pub effective_edit_gate: EditPosture,
    /// The safety requirements an allowed crossing must satisfy. Empty when the
    /// mutation does not cross a canonical boundary.
    pub required_safety: Vec<SafetyRequirement>,
    /// The safety requirements the envelope failed to satisfy, in canonical
    /// order.
    pub unmet_safety_requirements: Vec<SafetyRequirement>,
    /// Whether every required-and-checked safety requirement is satisfied.
    pub safety_envelope_complete: bool,
    /// How fully the rollback checkpoint can undo the mutation, given its
    /// declared side effects.
    pub rollback_coverage: RollbackCoverage,
    /// The reused write-boundary decision over the target artifact.
    pub boundary_decision: WriteBoundaryDecision,
    /// The actor lineage recorded for the change.
    pub actor_lineage: ActorLineage,
    /// Stable tokens naming every input that blocked the mutation, sorted and
    /// deduplicated. Empty only when the mutation is admitted.
    pub why_blocked_tokens: Vec<String>,
    /// The user-visible guidance line.
    pub guidance_line: String,
    /// The support/audit explanation of which route crossed the boundary and
    /// under what posture.
    pub support_summary: String,
    /// The one stable copy/export form for the decision.
    pub copy_line: String,
}

// ---------------------------------------------------------------------------
// Engine: the single source of truth for the decision.
// ---------------------------------------------------------------------------

/// Decides the mutation guardrail for an automated route from its attempt.
///
/// This is the canonical engine the cases, the fixtures, the validators, and
/// the consuming surfaces all share. It reuses [`decide_write_boundary`] for
/// the underlying boundary classification and layers the automated-route
/// requirements on top:
///
/// - a missing canonical-source boundary descriptor blocks the route outright;
/// - a direct edit the boundary admits (the artifact is its own canonical
///   source and in sync) is admitted directly and recorded;
/// - a regenerate-only boundary blocks the route in favor of regeneration;
/// - a reviewed-override boundary admits the cross-boundary mutation only when
///   the safety envelope is complete *and* the boundary admitted the recorded
///   override, otherwise it is held for review.
pub fn decide_mutation_guardrail(attempt: &MutationAttempt) -> MutationGuardrailDecision {
    let boundary_decision = decide_write_boundary(&attempt.artifact);
    let rollback_coverage =
        RollbackCoverage::for_classes(attempt.envelope.side_effects.iter().copied());

    let unmet_safety_requirements = unmet_requirements(&attempt.envelope);
    let safety_envelope_complete = unmet_safety_requirements.is_empty();

    let crosses_canonical_boundary = attempt.boundary_data_state == BoundaryDataState::Missing
        || boundary_decision.attempt_outcome != AttemptOutcome::DirectEditAdmitted;

    let guardrail_outcome = if attempt.boundary_data_state == BoundaryDataState::Missing {
        GuardrailOutcome::BlockedMissingBoundaryData
    } else {
        match boundary_decision.attempt_outcome {
            AttemptOutcome::DirectEditAdmitted => GuardrailOutcome::AdmittedDirect,
            AttemptOutcome::BlockedRegenerateFirst => GuardrailOutcome::BlockedRegenerateFirst,
            AttemptOutcome::BlockedPendingReview => GuardrailOutcome::BlockedPendingReview,
            AttemptOutcome::OverrideAdmittedWithDivergence => {
                if safety_envelope_complete {
                    GuardrailOutcome::AdmittedWithPreviewAndOverride
                } else {
                    GuardrailOutcome::BlockedPendingReview
                }
            }
        }
    };

    let effective_edit_gate = if attempt.boundary_data_state == BoundaryDataState::Missing {
        EditPosture::RegenerateOnly
    } else {
        boundary_decision.effective_edit_gate
    };

    let required_safety = if crosses_canonical_boundary {
        SafetyRequirement::ALL.to_vec()
    } else {
        Vec::new()
    };

    let why_blocked_tokens = compose_why_blocked(
        guardrail_outcome,
        &boundary_decision,
        &unmet_safety_requirements,
    );

    let actor_lineage = build_lineage(attempt);

    let guidance_line = guidance_for(
        attempt,
        guardrail_outcome,
        &boundary_decision,
        &unmet_safety_requirements,
    );

    let support_summary = support_summary_for(
        attempt,
        guardrail_outcome,
        crosses_canonical_boundary,
        effective_edit_gate,
        rollback_coverage,
    );

    let copy_line = copy_line_for(
        attempt.route,
        attempt.mutation_class,
        attempt.boundary_data_state,
        guardrail_outcome,
        crosses_canonical_boundary,
        effective_edit_gate,
        rollback_coverage,
        attempt.envelope.reversal_class,
    );

    MutationGuardrailDecision {
        route: attempt.route,
        mutation_class: attempt.mutation_class,
        boundary_data_state: attempt.boundary_data_state,
        guardrail_outcome,
        mutation_admitted: guardrail_outcome.admits_mutation(),
        crosses_canonical_boundary,
        effective_edit_gate,
        required_safety,
        unmet_safety_requirements,
        safety_envelope_complete,
        rollback_coverage,
        boundary_decision,
        actor_lineage,
        why_blocked_tokens,
        guidance_line,
        support_summary,
        copy_line,
    }
}

/// Computes the safety requirements the envelope fails to satisfy, in
/// [`SafetyRequirement::ALL`] order.
fn unmet_requirements(envelope: &MutationSafetyEnvelope) -> Vec<SafetyRequirement> {
    let mut unmet = Vec::new();
    let has_preview = envelope
        .preview_ref
        .as_ref()
        .map(|s| !s.trim().is_empty())
        .unwrap_or(false);
    if !has_preview {
        unmet.push(SafetyRequirement::Preview);
    }
    if envelope.side_effect_disclosure != SideEffectDisclosure::DeclaredReviewed {
        unmet.push(SafetyRequirement::SideEffectSummary);
    }
    if !envelope.regeneration_acknowledged {
        unmet.push(SafetyRequirement::RegenerationAwareness);
    }
    if !envelope.reversal_class.provides_rollback() {
        unmet.push(SafetyRequirement::RollbackClass);
    }
    unmet
}

fn compose_why_blocked(
    outcome: GuardrailOutcome,
    boundary_decision: &WriteBoundaryDecision,
    unmet: &[SafetyRequirement],
) -> Vec<String> {
    let mut tokens = Vec::new();
    match outcome {
        GuardrailOutcome::AdmittedDirect | GuardrailOutcome::AdmittedWithPreviewAndOverride => {}
        GuardrailOutcome::BlockedMissingBoundaryData => {
            tokens.push("missing_canonical_source_boundary_data".to_owned());
        }
        GuardrailOutcome::BlockedPendingReview => {
            tokens.extend(boundary_decision.why_blocked_tokens.iter().cloned());
            tokens.extend(unmet.iter().map(|req| req.unmet_token().to_owned()));
        }
        GuardrailOutcome::BlockedRegenerateFirst => {
            tokens.extend(boundary_decision.why_blocked_tokens.iter().cloned());
        }
    }
    tokens.sort();
    tokens.dedup();
    tokens
}

fn build_lineage(attempt: &MutationAttempt) -> ActorLineage {
    let summary = format!(
        "{} by {} ({}) recorded as mutation class {} with reversal class {}.",
        attempt.route.short_label(),
        attempt.actor_ref,
        attempt.source_class.as_str(),
        attempt.mutation_class.as_str(),
        attempt.envelope.reversal_class.as_str(),
    );
    ActorLineage {
        route: attempt.route,
        actor_class: attempt.route.actor_class_token().to_owned(),
        source_class: attempt.source_class,
        actor_ref: attempt.actor_ref.clone(),
        mutation_class: attempt.mutation_class,
        reversal_class: attempt.envelope.reversal_class,
        summary,
    }
}

fn unmet_summary(unmet: &[SafetyRequirement]) -> String {
    unmet
        .iter()
        .map(|req| req.summary())
        .collect::<Vec<_>>()
        .join(", ")
}

fn guidance_for(
    attempt: &MutationAttempt,
    outcome: GuardrailOutcome,
    boundary_decision: &WriteBoundaryDecision,
    unmet: &[SafetyRequirement],
) -> String {
    let path = &attempt.artifact.artifact_path_label;
    let route = attempt.route.short_label();
    match outcome {
        GuardrailOutcome::AdmittedDirect => format!(
            "Mutation admitted: {path} is its own canonical source and in sync, so {route} edits it as ordinary source — recorded with actor lineage and mutation class."
        ),
        GuardrailOutcome::AdmittedWithPreviewAndOverride => format!(
            "Cross-boundary mutation admitted: {route} carried a preview, a reviewed side-effect summary, regeneration awareness, and a rollback class, and a reviewed override was recorded. {path} now diverges from its generator — regenerate to discard or reconcile into the canonical source."
        ),
        GuardrailOutcome::BlockedPendingReview => {
            let mut needs = Vec::new();
            if !boundary_decision.attempt_outcome.leaves_divergence() {
                needs.push("a recorded reviewed override".to_owned());
            }
            if !unmet.is_empty() {
                needs.push(unmet_summary(unmet));
            }
            let needs = if needs.is_empty() {
                "a complete safety envelope".to_owned()
            } else {
                needs.join(", and ")
            };
            format!(
                "Cross-boundary mutation held: {route} cannot mutate {path} as ordinary source. Provide {needs} before it can proceed."
            )
        }
        GuardrailOutcome::BlockedRegenerateFirst => format!(
            "Cross-boundary mutation blocked: {path} is regenerated from its canonical source. Regenerate rather than letting {route} edit the derived bytes."
        ),
        GuardrailOutcome::BlockedMissingBoundaryData => format!(
            "Mutation blocked: {path} has no canonical-source boundary data, so {route} must not mutate it as ordinary source. Establish the canonical-source boundary — generator identity, source linkage, and edit posture — before any cross-boundary mutation."
        ),
    }
}

fn support_summary_for(
    attempt: &MutationAttempt,
    outcome: GuardrailOutcome,
    crosses_canonical_boundary: bool,
    effective_edit_gate: EditPosture,
    rollback_coverage: RollbackCoverage,
) -> String {
    let crossing = if crosses_canonical_boundary {
        "crossed"
    } else {
        "did not cross"
    };
    format!(
        "{} by {} ({}) of {} [{}] {} the canonical-source boundary; outcome={}, edit_gate={}, rollback={}/{}.",
        attempt.route.short_label(),
        attempt.actor_ref,
        attempt.source_class.as_str(),
        attempt.artifact.artifact_path_label,
        attempt.mutation_class.as_str(),
        crossing,
        outcome.as_str(),
        effective_edit_gate.as_str(),
        rollback_coverage.as_str(),
        attempt.envelope.reversal_class.as_str(),
    )
}

/// Computes the stable copy/export form for a decision.
pub fn mutation_guardrails_copy_line(decision: &MutationGuardrailDecision) -> String {
    copy_line_for(
        decision.route,
        decision.mutation_class,
        decision.boundary_data_state,
        decision.guardrail_outcome,
        decision.crosses_canonical_boundary,
        decision.effective_edit_gate,
        decision.rollback_coverage,
        decision.actor_lineage.reversal_class,
    )
}

#[allow(clippy::too_many_arguments)]
fn copy_line_for(
    route: MutationRoute,
    mutation_class: MutationClass,
    boundary_data_state: BoundaryDataState,
    outcome: GuardrailOutcome,
    crosses_canonical_boundary: bool,
    effective_edit_gate: EditPosture,
    rollback_coverage: RollbackCoverage,
    reversal_class: ReversalClass,
) -> String {
    format!(
        "mutation-guardrails route={} mutation_class={} boundary_data={} outcome={} admitted={} crosses_boundary={} gate={} rollback={} reversal={}",
        route.as_str(),
        mutation_class.as_str(),
        boundary_data_state.as_str(),
        outcome.as_str(),
        outcome.admits_mutation(),
        crosses_canonical_boundary,
        effective_edit_gate.as_str(),
        rollback_coverage.as_str(),
        reversal_class.as_str(),
    )
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One mutation-guardrail case: an attempt and the decision the engine stamps
/// onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailCase {
    /// Stable case id.
    pub case_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The mutation attempt the engine reads.
    pub attempt: MutationAttempt,
    /// The decision the engine reached.
    pub decision: MutationGuardrailDecision,
    /// Upstream generated-artifact packets backing this case.
    pub evidence_refs: Vec<String>,
    /// One real consumer that renders this case.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a surface ingests this packet rather than re-deriving
/// mutation-guardrail truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailSurfaceBinding {
    /// Surface that ingests the packet.
    pub surface: MutationGuardrailSurface,
    /// Checked consumer ref that renders the decision.
    pub consumer_ref: String,
    /// Packet id the surface ingests.
    pub ingested_packet_id: String,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailSourceContractRefs {
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

/// Top-level packet modeling mutation-guardrail decisions across the automated
/// mutation routes and guardrail outcomes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: MutationGuardrailSourceContractRefs,
    /// The automated routes the packet models.
    pub routes: Vec<MutationRoute>,
    /// The guardrail outcomes the packet models.
    pub guardrail_outcomes: Vec<GuardrailOutcome>,
    /// The safety requirements an allowed crossing must satisfy.
    pub safety_requirements: Vec<SafetyRequirement>,
    /// Upstream generated-artifact packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Mutation-guardrail cases.
    pub cases: Vec<MutationGuardrailCase>,
    /// Surface bindings, one per rendered surface.
    pub surface_bindings: Vec<MutationGuardrailSurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a case to its expected decision, proving the canonical
/// decision behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationGuardrailFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The case under test.
    pub case: MutationGuardrailCase,
    /// Expected guardrail outcome.
    pub expected_guardrail_outcome: GuardrailOutcome,
    /// Expected admit flag.
    pub expected_mutation_admitted: bool,
    /// Expected crosses-canonical-boundary flag.
    pub expected_crosses_canonical_boundary: bool,
    /// Expected unmet safety requirements.
    pub expected_unmet_safety_requirements: Vec<SafetyRequirement>,
    /// Expected why-blocked tokens.
    pub expected_why_blocked_tokens: Vec<String>,
    /// Whether the case is expected to leave a durable divergence state.
    pub expected_leaves_divergence: bool,
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
        writeln!(f, "mutation-guardrails validation failed")?;
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
const REGENERATION_PLAN_PACKET_REF: &str = "artifacts/generated/regeneration-plan-packet.json";
const MUTATION_CLASSES_REF: &str = "artifacts/change/mutation_classes.yaml";
const MUTATION_JOURNAL_SCHEMA_REF: &str = "schemas/workspace/mutation_journal.schema.json";
const COMMAND_RESULT_SCHEMA_REF: &str = "schemas/commands/command_result.schema.json";
const DIVERGENCE_CONTRACT_REF: &str = "docs/generated/diverged_from_generator_contract.md";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";

/// The reviewed-override review evidence a recorded override cites.
const OVERRIDE_REVIEW_REF: &str = "artifacts/fs/save_review_choice_matrix.yaml";

fn evidence_packet_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        WRITE_BOUNDARY_PACKET_REF,
        REGENERATION_PLAN_PACKET_REF,
        MUTATION_CLASSES_REF,
        MUTATION_JOURNAL_SCHEMA_REF,
        COMMAND_RESULT_SCHEMA_REF,
        DIVERGENCE_CONTRACT_REF,
        ROLLBACK_CHECKPOINT_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn case_evidence_refs() -> Vec<String> {
    [
        WRITE_BOUNDARY_PACKET_REF,
        REGENERATION_PLAN_PACKET_REF,
        MUTATION_CLASSES_REF,
        MUTATION_JOURNAL_SCHEMA_REF,
        COMMAND_RESULT_SCHEMA_REF,
        ROLLBACK_CHECKPOINT_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

// ---------------------------------------------------------------------------
// Artifact-subject helpers, mirroring the write-boundary class mapping.
// ---------------------------------------------------------------------------

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

fn class_authority(artifact_class: ArtifactClass) -> (AuthorityClass, EditPosture) {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => (
            AuthorityClass::CanonicalAuthoritative,
            EditPosture::DirectEditAllowed,
        ),
        ArtifactClass::NotebookOutput => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
        ArtifactClass::PreviewDerivative => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
        ArtifactClass::RequestArtifact => (
            AuthorityClass::DerivedEditable,
            EditPosture::ReviewedOverrideRequired,
        ),
        ArtifactClass::FrameworkCodegen => (
            AuthorityClass::DerivedEditable,
            EditPosture::ReviewedOverrideRequired,
        ),
        ArtifactClass::AiAssistedEdit => (
            AuthorityClass::CanonicalAuthoritative,
            EditPosture::DirectEditAllowed,
        ),
        ArtifactClass::SupportPacket => {
            (AuthorityClass::DerivedReadonly, EditPosture::RegenerateOnly)
        }
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

/// Builds a write-boundary subject for a class in a given boundary state.
fn artifact_subject(
    artifact_class: ArtifactClass,
    boundary_state: BoundaryState,
    override_review_ref: Option<&str>,
) -> WriteBoundarySubject {
    let (authority_class, declared_edit_posture) = class_authority(artifact_class);
    let canonical_source_ref = if boundary_state.canonical_source_linked() {
        Some(class_source_ref(artifact_class).to_owned())
    } else {
        None
    };
    WriteBoundarySubject {
        artifact_class,
        artifact_path_label: class_path_label(artifact_class).to_owned(),
        authority_class,
        generator: class_generator(artifact_class),
        declared_edit_posture,
        boundary_state,
        canonical_source_ref,
        regeneration_route: class_regeneration_route(artifact_class).to_owned(),
        checkpoint_lineage_ref: ROLLBACK_CHECKPOINT_REF.to_owned(),
        override_review_ref: override_review_ref.map(str::to_owned),
    }
}

fn route_consumer_ref(route: MutationRoute) -> &'static str {
    match route {
        MutationRoute::AiApply => "crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs",
        MutationRoute::Refactor | MutationRoute::QuickFix => {
            "crates/aureline-review/src/change_inspector/mod.rs"
        }
        MutationRoute::Automation => {
            "crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs"
        }
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Builders for the safety envelope a route brings.
struct EnvelopeSpec {
    preview_ref: Option<&'static str>,
    side_effects: Vec<SideEffectClass>,
    side_effect_disclosure: SideEffectDisclosure,
    regeneration_acknowledged: bool,
    reversal_class: ReversalClass,
    override_recorded: bool,
}

impl EnvelopeSpec {
    fn build(self) -> MutationSafetyEnvelope {
        MutationSafetyEnvelope {
            preview_ref: self.preview_ref.map(str::to_owned),
            side_effects: self.side_effects,
            side_effect_disclosure: self.side_effect_disclosure,
            regeneration_acknowledged: self.regeneration_acknowledged,
            reversal_class: self.reversal_class,
            override_review_ref: self
                .override_recorded
                .then(|| OVERRIDE_REVIEW_REF.to_owned()),
        }
    }
}

const PREVIEW_REF: &str = "review diff preview packet";

/// A complete cross-boundary envelope: preview, reviewed local side effects,
/// regeneration awareness, a reversible rollback class, and a recorded
/// override.
fn complete_envelope() -> EnvelopeSpec {
    EnvelopeSpec {
        preview_ref: Some(PREVIEW_REF),
        side_effects: vec![SideEffectClass::LocalCompute],
        side_effect_disclosure: SideEffectDisclosure::DeclaredReviewed,
        regeneration_acknowledged: true,
        reversal_class: ReversalClass::RestoreFromCheckpoint,
        override_recorded: true,
    }
}

#[allow(clippy::too_many_arguments)]
fn make_attempt(
    attempt_id: &str,
    route: MutationRoute,
    source_class: MutationSourceClass,
    actor_ref: &str,
    mutation_class: MutationClass,
    boundary_data_state: BoundaryDataState,
    artifact_class: ArtifactClass,
    boundary_state: BoundaryState,
    envelope: MutationSafetyEnvelope,
) -> MutationAttempt {
    let artifact = artifact_subject(
        artifact_class,
        boundary_state,
        envelope.override_review_ref.as_deref(),
    );
    MutationAttempt {
        attempt_id: attempt_id.to_owned(),
        route,
        source_class,
        actor_ref: actor_ref.to_owned(),
        mutation_class,
        boundary_data_state,
        artifact,
        envelope,
    }
}

fn case(
    case_id: &str,
    scenario: &str,
    attempt: MutationAttempt,
    notes: &str,
) -> MutationGuardrailCase {
    let consumer_ref = route_consumer_ref(attempt.route).to_owned();
    let decision = decide_mutation_guardrail(&attempt);
    MutationGuardrailCase {
        case_id: case_id.to_owned(),
        scenario: scenario.to_owned(),
        attempt,
        decision,
        evidence_refs: case_evidence_refs(),
        consumer_ref,
        notes: notes.to_owned(),
    }
}

fn binding(
    surface: MutationGuardrailSurface,
    consumer_ref: &str,
    summary: &str,
) -> MutationGuardrailSurfaceBinding {
    MutationGuardrailSurfaceBinding {
        surface,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: MUTATION_GUARDRAILS_PACKET_ID.to_owned(),
        summary: summary.to_owned(),
    }
}

/// Returns the seeded mutation-guardrail cases this lane freezes: every route,
/// every guardrail outcome, and every unmet-safety-requirement reason.
fn seeded_cases() -> Vec<MutationGuardrailCase> {
    vec![
        // Admitted-direct: the artifact is its own canonical source and in sync.
        case(
            "mutation-guardrails.ai_apply.ai_assisted_edit.admitted_direct",
            "AI apply edits its own accepted edit, in sync — admitted as ordinary source",
            make_attempt(
                "attempt.ai_apply.ai_assisted_edit",
                MutationRoute::AiApply,
                MutationSourceClass::AiHostedProvider,
                "ai/scoped-composer@1.0.0",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::AiAssistedEdit,
                BoundaryState::InSync,
                complete_envelope().build(),
            ),
            "An accepted AI edit is canonical source the user owns; in sync, a further AI apply edits it as ordinary source and is recorded.",
        ),
        case(
            "mutation-guardrails.refactor.scaffolded_project.admitted_direct",
            "Refactor edits a scaffolded-project file, in sync — admitted as ordinary source",
            make_attempt(
                "attempt.refactor.scaffolded_project",
                MutationRoute::Refactor,
                MutationSourceClass::MachineLocal,
                "refactor/rename-symbol",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::ScaffoldedProject,
                BoundaryState::InSync,
                complete_envelope().build(),
            ),
            "A scaffolded project is its own canonical source; in sync, a refactor edits it as ordinary source.",
        ),
        // Admitted cross-boundary: complete envelope plus a recorded override.
        case(
            "mutation-guardrails.refactor.framework_codegen.admitted_with_preview_and_override",
            "Refactor edits framework codegen with a complete envelope and a recorded override — admitted with divergence",
            make_attempt(
                "attempt.refactor.framework_codegen.admitted",
                MutationRoute::Refactor,
                MutationSourceClass::MachineLocal,
                "refactor/extract-function",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::InSync,
                complete_envelope().build(),
            ),
            "A complete safety envelope and a recorded reviewed override admit the cross-boundary refactor and leave a durable diverged-from-generator state.",
        ),
        // Blocked pending review: missing preview.
        case(
            "mutation-guardrails.ai_apply.framework_codegen.blocked_missing_preview",
            "AI apply edits framework codegen with an override but no preview — held for review",
            make_attempt(
                "attempt.ai_apply.framework_codegen.no_preview",
                MutationRoute::AiApply,
                MutationSourceClass::AiHostedProvider,
                "ai/scoped-composer@1.0.0",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::InSync,
                EnvelopeSpec {
                    preview_ref: None,
                    ..complete_envelope()
                }
                .build(),
            ),
            "Even with a recorded override, a cross-boundary AI apply with no preview is held until the preview is supplied.",
        ),
        // Blocked pending review: undeclared side effects.
        case(
            "mutation-guardrails.automation.framework_codegen.blocked_undeclared_side_effects",
            "Automation edits framework codegen with an undeclared networked install — held for review",
            make_attempt(
                "attempt.automation.framework_codegen.undeclared",
                MutationRoute::Automation,
                MutationSourceClass::PolicyDriven,
                "automation/codegen-runner",
                MutationClass::GeneratedState,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::InSync,
                EnvelopeSpec {
                    side_effects: vec![
                        SideEffectClass::LocalCompute,
                        SideEffectClass::NetworkInstall,
                    ],
                    side_effect_disclosure: SideEffectDisclosure::Undeclared,
                    reversal_class: ReversalClass::RegenerateOrRecompute,
                    ..complete_envelope()
                }
                .build(),
            ),
            "An undeclared networked install must not run silently; the mutation is held until the side effect is declared and reviewed.",
        ),
        // Blocked pending review: regeneration not acknowledged.
        case(
            "mutation-guardrails.ai_apply.framework_codegen.blocked_regeneration_not_acknowledged",
            "AI apply edits framework codegen without acknowledging regeneration — held for review",
            make_attempt(
                "attempt.ai_apply.framework_codegen.no_regen_ack",
                MutationRoute::AiApply,
                MutationSourceClass::AiLocalModel,
                "ai/local-composer@0.8.0",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::InSync,
                EnvelopeSpec {
                    regeneration_acknowledged: false,
                    ..complete_envelope()
                }
                .build(),
            ),
            "A cross-boundary edit that does not acknowledge the artifact is regenerated is held — the change would be clobbered by the next regeneration.",
        ),
        // Blocked pending review: rollback class is audit-only.
        case(
            "mutation-guardrails.automation.framework_codegen.blocked_no_rollback_class",
            "Automation edits framework codegen with an audit-only reversal class — held for review",
            make_attempt(
                "attempt.automation.framework_codegen.audit_only",
                MutationRoute::Automation,
                MutationSourceClass::PolicyDriven,
                "automation/codegen-runner",
                MutationClass::GeneratedState,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::InSync,
                EnvelopeSpec {
                    side_effects: vec![
                        SideEffectClass::LocalCompute,
                        SideEffectClass::NetworkInstall,
                    ],
                    reversal_class: ReversalClass::AuditOnly,
                    ..complete_envelope()
                }
                .build(),
            ),
            "An audit-only reversal class is not rollback-safe; the cross-boundary mutation is held until a reversible rollback class is declared.",
        ),
        // Blocked pending review: no recorded override.
        case(
            "mutation-guardrails.quick_fix.request_artifact.blocked_pending_review",
            "Quick fix edits a request artifact with no recorded override — held for review",
            make_attempt(
                "attempt.quick_fix.request_artifact",
                MutationRoute::QuickFix,
                MutationSourceClass::MachineLocal,
                "quick-fix/apply-suggestion",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::RequestArtifact,
                BoundaryState::InSync,
                EnvelopeSpec {
                    override_recorded: false,
                    ..complete_envelope()
                }
                .build(),
            ),
            "A request artifact is derived-editable; a quick fix is held until it escalates through a recorded reviewed override.",
        ),
        // Blocked regenerate-first: derived-readonly.
        case(
            "mutation-guardrails.automation.notebook_output.blocked_regenerate_first",
            "Automation edits a notebook output, in sync — blocked, regenerate first",
            make_attempt(
                "attempt.automation.notebook_output",
                MutationRoute::Automation,
                MutationSourceClass::MachineRemoteAgent,
                "automation/notebook-runner",
                MutationClass::GeneratedState,
                BoundaryDataState::Present,
                ArtifactClass::NotebookOutput,
                BoundaryState::InSync,
                complete_envelope().build(),
            ),
            "A notebook output is purely derived; an automation mutation is blocked in favor of re-running the cell.",
        ),
        // Blocked regenerate-first: generator unavailable.
        case(
            "mutation-guardrails.quick_fix.preview_derivative.blocked_regenerate_first",
            "Quick fix edits a preview derivative with the generator unavailable — blocked, restore generator first",
            make_attempt(
                "attempt.quick_fix.preview_derivative.generator_unavailable",
                MutationRoute::QuickFix,
                MutationSourceClass::MachineLocal,
                "quick-fix/apply-suggestion",
                MutationClass::GeneratedState,
                BoundaryDataState::Present,
                ArtifactClass::PreviewDerivative,
                BoundaryState::GeneratorUnavailable,
                complete_envelope().build(),
            ),
            "A preview derivative is rebuilt from source; with the builder unavailable the quick fix is blocked and the recovery restores the generator first.",
        ),
        // Blocked regenerate-first: regeneration blocked by policy.
        case(
            "mutation-guardrails.refactor.framework_codegen.blocked_regeneration_policy",
            "Refactor edits framework codegen with regeneration blocked by policy — blocked, resolve policy first",
            make_attempt(
                "attempt.refactor.framework_codegen.policy_blocked",
                MutationRoute::Refactor,
                MutationSourceClass::MachineLocal,
                "refactor/organize-imports",
                MutationClass::SemanticTooling,
                BoundaryDataState::Present,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::RegenerationBlockedByPolicy,
                complete_envelope().build(),
            ),
            "Policy forbids regenerating the artifact; the refactor is blocked and the decision surfaces the policy block rather than a generic failure.",
        ),
        // Blocked: no canonical-source boundary data at all.
        case(
            "mutation-guardrails.ai_apply.missing_boundary_data.blocked",
            "AI apply targets a generated-looking artifact with no canonical-source boundary data — blocked",
            make_attempt(
                "attempt.ai_apply.missing_boundary_data",
                MutationRoute::AiApply,
                MutationSourceClass::AiHostedProvider,
                "ai/scoped-composer@1.0.0",
                MutationClass::SemanticTooling,
                BoundaryDataState::Missing,
                ArtifactClass::FrameworkCodegen,
                BoundaryState::SourceMissing,
                complete_envelope().build(),
            ),
            "Without proven canonical-source boundary data the artifact cannot be classified, so an AI apply must not mutate it as ordinary source or through an override.",
        ),
    ]
}

/// Returns the checked-in mutation-guardrails packet this lane freezes.
pub fn seeded_mutation_guardrails_packet() -> MutationGuardrailPacket {
    let surface_bindings = vec![
        binding(
            MutationGuardrailSurface::AiApplyGate,
            "crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs",
            "The AI scoped-apply gate renders the guardrail decision so an AI apply that crosses a canonical boundary is held for a complete safety envelope and a reviewed override — never applied silently.",
        ),
        binding(
            MutationGuardrailSurface::RefactorTransaction,
            "crates/aureline-review/src/stabilize_worktree_patch_stack_and_explicit_change_object/mod.rs",
            "The refactor / change-object transaction renders the why-blocked tokens, the required safety envelope, and the boundary compare so a refactor or quick fix only crosses the generator boundary through a recorded review.",
        ),
        binding(
            MutationGuardrailSurface::AutomationRunner,
            "crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs",
            "The automation runner consults the decision before a codegen or task pass mutates a generated artifact, blocking undeclared side effects and regenerate-only targets instead of writing derived bytes.",
        ),
        binding(
            MutationGuardrailSurface::MutationJournal,
            "crates/aureline-workspace/src/mutation_journal/mod.rs",
            "The mutation journal records the actor lineage and mutation class the decision carries, so a journaled mutation attributes which route crossed the canonical boundary and under what reversal class.",
        ),
        binding(
            MutationGuardrailSurface::SupportExport,
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "The support export re-emits the decision support summary, outcome, and rollback class with no raw bytes, diffs, or credentials, so diagnostics can explain how a generated artifact was changed.",
        ),
    ];

    MutationGuardrailPacket {
        record_kind: MUTATION_GUARDRAILS_PACKET_RECORD_KIND.to_owned(),
        schema_version: MUTATION_GUARDRAILS_SCHEMA_VERSION,
        packet_id: MUTATION_GUARDRAILS_PACKET_ID.to_owned(),
        title: "AI-apply, refactor, quick-fix, and automation mutation guardrails for generated artifacts that cross a canonical-source boundary"
            .to_owned(),
        source_contract_refs: MutationGuardrailSourceContractRefs {
            doc_ref: MUTATION_GUARDRAILS_DOC_REF.to_owned(),
            schema_ref: MUTATION_GUARDRAILS_SCHEMA_REF.to_owned(),
            packet_ref: MUTATION_GUARDRAILS_PACKET_REF.to_owned(),
            report_ref: MUTATION_GUARDRAILS_REPORT_REF.to_owned(),
            fixture_manifest_ref: MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF.to_owned(),
        },
        routes: MutationRoute::ALL.to_vec(),
        guardrail_outcomes: GuardrailOutcome::ALL.to_vec(),
        safety_requirements: SafetyRequirement::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        cases: seeded_cases(),
        surface_bindings,
        invariants: vec![
            "No AI apply, refactor, quick fix, or automation path can silently mutate a non-authoritative generated artifact as if it were ordinary user-authored source: it is admitted directly only when the artifact is its own canonical source and in sync.".to_owned(),
            "An automated route that targets an artifact with no canonical-source boundary data is blocked outright — it cannot be classified, treated as ordinary source, or admitted through an override.".to_owned(),
            "A cross-boundary mutation is admitted only with all four safety requirements — preview, reviewed side-effect summary, regeneration awareness, and a rollback class — and a recorded reviewed override; any unmet requirement holds the mutation and names what is missing.".to_owned(),
            "An undeclared networked install, tool download, secret use, or broad write is never run silently; an audit-only reversal class is never accepted as rollback-safe.".to_owned(),
            "Every decision records actor lineage and mutation class against the shared mutation-journal contract and reuses the write-boundary decision and regeneration side-effect/rollback vocabulary, so no route gets a hidden mutation path and support/export can explain which route crossed the boundary and under what posture.".to_owned(),
        ],
    }
}

/// Returns the checked-in mutation-guardrails fixture corpus this lane freezes:
/// one fixture per seeded case.
pub fn seeded_mutation_guardrails_fixtures() -> Vec<MutationGuardrailFixture> {
    seeded_cases().into_iter().map(fixture).collect()
}

fn fixture(case: MutationGuardrailCase) -> MutationGuardrailFixture {
    let decision = &case.decision;
    MutationGuardrailFixture {
        record_kind: MUTATION_GUARDRAILS_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: MUTATION_GUARDRAILS_SCHEMA_VERSION,
        fixture_id: format!("fixture.{}", case.case_id),
        scenario: case.scenario.clone(),
        expected_guardrail_outcome: decision.guardrail_outcome,
        expected_mutation_admitted: decision.mutation_admitted,
        expected_crosses_canonical_boundary: decision.crosses_canonical_boundary,
        expected_unmet_safety_requirements: decision.unmet_safety_requirements.clone(),
        expected_why_blocked_tokens: decision.why_blocked_tokens.clone(),
        expected_leaves_divergence: decision.guardrail_outcome.leaves_divergence(),
        notes: case.notes.clone(),
        case,
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in mutation-guardrails packet contract.
pub fn validate_mutation_guardrails_packet(
    packet: &MutationGuardrailPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != MUTATION_GUARDRAILS_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != MUTATION_GUARDRAILS_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != MUTATION_GUARDRAILS_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != MUTATION_GUARDRAILS_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != MUTATION_GUARDRAILS_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != MUTATION_GUARDRAILS_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != MUTATION_GUARDRAILS_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != MUTATION_GUARDRAILS_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.routes != MutationRoute::ALL.to_vec() {
        report.push(
            "packet.routes",
            "packet must declare every mutation route in canonical order",
        );
    }
    if packet.guardrail_outcomes != GuardrailOutcome::ALL.to_vec() {
        report.push(
            "packet.guardrail_outcomes",
            "packet must declare every guardrail outcome in canonical order",
        );
    }
    if packet.safety_requirements != SafetyRequirement::ALL.to_vec() {
        report.push(
            "packet.safety_requirements",
            "packet must declare every safety requirement in canonical order",
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
    let mut covered_routes = BTreeSet::new();
    let mut covered_outcomes = BTreeSet::new();
    let mut covered_unmet = BTreeSet::new();
    for guard_case in &packet.cases {
        if !case_ids.insert(guard_case.case_id.as_str()) {
            report.push(
                "case.id_unique",
                format!("duplicate case id {}", guard_case.case_id),
            );
        }
        covered_routes.insert(guard_case.decision.route);
        covered_outcomes.insert(guard_case.decision.guardrail_outcome);
        for unmet in &guard_case.decision.unmet_safety_requirements {
            covered_unmet.insert(*unmet);
        }
        validate_case(&mut report, guard_case);
    }
    for required in MutationRoute::ALL {
        if !covered_routes.contains(&required) {
            report.push(
                "packet.route_coverage",
                format!("packet must cover mutation route {}", required.as_str()),
            );
        }
    }
    for required in GuardrailOutcome::ALL {
        if !covered_outcomes.contains(&required) {
            report.push(
                "packet.outcome_coverage",
                format!("packet must cover guardrail outcome {}", required.as_str()),
            );
        }
    }
    for required in SafetyRequirement::ALL {
        if !covered_unmet.contains(&required) {
            report.push(
                "packet.unmet_requirement_coverage",
                format!(
                    "packet must exercise unmet safety requirement {}",
                    required.as_str()
                ),
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

fn validate_case(report: &mut ValidationReport, guard_case: &MutationGuardrailCase) {
    let owner = format!("case {}", guard_case.case_id);

    if guard_case.case_id.trim().is_empty() {
        report.push("case.id", "case must carry a stable id");
    }
    if guard_case.scenario.trim().is_empty() {
        report.push("case.scenario", format!("{owner} must carry a scenario"));
    }
    if guard_case.consumer_ref.trim().is_empty() {
        report.push(
            "case.consumer_ref",
            format!("{owner} must cite a consumer ref"),
        );
    }
    if guard_case.notes.trim().is_empty() {
        report.push("case.notes", format!("{owner} must carry a reviewer note"));
    }
    if guard_case.evidence_refs.is_empty() {
        report.push(
            "case.evidence_refs",
            format!("{owner} must cite at least one evidence ref"),
        );
    }

    validate_attempt(report, &owner, &guard_case.attempt);

    // The stamped decision must equal what the engine computes.
    let expected = decide_mutation_guardrail(&guard_case.attempt);
    if guard_case.decision != expected {
        report.push(
            "case.decision",
            format!("{owner} stamped decision disagrees with the engine"),
        );
    }

    validate_decision(report, &owner, &guard_case.attempt, &guard_case.decision);
}

fn validate_attempt(report: &mut ValidationReport, owner: &str, attempt: &MutationAttempt) {
    if attempt.attempt_id.trim().is_empty() {
        report.push("attempt.id", format!("{owner} must carry an attempt id"));
    }
    if attempt.actor_ref.trim().is_empty() {
        report.push(
            "attempt.actor_ref",
            format!("{owner} must carry an actor ref"),
        );
    }
    // The subject's recorded override must mirror the envelope's, so the reused
    // write-boundary engine and the guardrail engine read one override.
    if attempt.artifact.override_review_ref != attempt.envelope.override_review_ref {
        report.push(
            "attempt.override_consistency",
            format!("{owner} artifact override ref must mirror the envelope override ref"),
        );
    }
}

fn validate_decision(
    report: &mut ValidationReport,
    owner: &str,
    attempt: &MutationAttempt,
    decision: &MutationGuardrailDecision,
) {
    // The embedded boundary decision must equal what the boundary engine
    // computes for the attempt's artifact — the guardrail reuses it, never
    // re-derives a private boundary verdict.
    if decision.boundary_decision != decide_write_boundary(&attempt.artifact) {
        report.push(
            "decision.boundary_reuse",
            format!("{owner} embedded boundary decision disagrees with the write-boundary engine"),
        );
    }

    // Admit flag and divergence must follow from the outcome.
    if decision.mutation_admitted != decision.guardrail_outcome.admits_mutation() {
        report.push(
            "decision.admit_flag",
            format!("{owner} mutation_admitted disagrees with the outcome"),
        );
    }

    // Rollback coverage must follow from the declared side effects.
    let expected_coverage =
        RollbackCoverage::for_classes(attempt.envelope.side_effects.iter().copied());
    if decision.rollback_coverage != expected_coverage {
        report.push(
            "decision.rollback_coverage",
            format!("{owner} rollback coverage disagrees with the declared side effects"),
        );
    }

    // Admitted outcomes carry no why-blocked tokens; blocked outcomes must name
    // why and carry a guidance line.
    match decision.guardrail_outcome {
        GuardrailOutcome::AdmittedDirect | GuardrailOutcome::AdmittedWithPreviewAndOverride => {
            if !decision.why_blocked_tokens.is_empty() {
                report.push(
                    "decision.admitted_no_block",
                    format!("{owner} an admitted mutation must carry no why-blocked tokens"),
                );
            }
        }
        _ => {
            if decision.why_blocked_tokens.is_empty() {
                report.push(
                    "decision.block_reason",
                    format!("{owner} a blocked mutation must name why it was blocked"),
                );
            }
            if decision.guidance_line.trim().is_empty() {
                report.push(
                    "decision.guidance",
                    format!("{owner} a blocked mutation must carry a guidance line"),
                );
            }
        }
    }

    // The core composition invariant: when the boundary admits the recorded
    // override, the guardrail admits the crossing exactly when the envelope is
    // complete, and holds it for review otherwise.
    if decision.boundary_decision.attempt_outcome == AttemptOutcome::OverrideAdmittedWithDivergence
        && decision.boundary_data_state == BoundaryDataState::Present
    {
        match (
            decision.safety_envelope_complete,
            decision.guardrail_outcome,
        ) {
            (true, GuardrailOutcome::AdmittedWithPreviewAndOverride) => {}
            (false, GuardrailOutcome::BlockedPendingReview) => {
                if decision.unmet_safety_requirements.is_empty() {
                    report.push(
                        "decision.held_needs_unmet",
                        format!("{owner} a held cross-boundary mutation must name an unmet safety requirement"),
                    );
                }
            }
            _ => report.push(
                "decision.envelope_composition",
                format!(
                    "{owner} an admitted-override boundary must admit exactly when the envelope is complete"
                ),
            ),
        }
    }

    // Divergence is left exactly by an admitted cross-boundary mutation.
    match (
        decision.guardrail_outcome.leaves_divergence(),
        &decision.boundary_decision.diverged_from_generator,
    ) {
        (true, Some(_)) => {}
        (true, None) => report.push(
            "decision.divergence_missing",
            format!("{owner} an admitted cross-boundary mutation must leave a divergence record"),
        ),
        (false, _) => {}
    }

    // No-silent-mutation: a missing-boundary-data target is never admitted and
    // names the missing-data reason.
    if decision.boundary_data_state == BoundaryDataState::Missing {
        if decision.guardrail_outcome != GuardrailOutcome::BlockedMissingBoundaryData {
            report.push(
                "decision.missing_data_blocked",
                format!("{owner} a missing-boundary-data target must be blocked"),
            );
        }
        if decision.mutation_admitted {
            report.push(
                "decision.missing_data_not_admitted",
                format!("{owner} a missing-boundary-data target must not admit a mutation"),
            );
        }
        if !decision
            .why_blocked_tokens
            .iter()
            .any(|token| token == "missing_canonical_source_boundary_data")
        {
            report.push(
                "decision.missing_data_token",
                format!("{owner} a missing-boundary-data block must name the missing-data reason"),
            );
        }
    }

    // crosses_canonical_boundary must follow from the boundary outcome and data
    // state.
    let expected_crossing = decision.boundary_data_state == BoundaryDataState::Missing
        || decision.boundary_decision.attempt_outcome != AttemptOutcome::DirectEditAdmitted;
    if decision.crosses_canonical_boundary != expected_crossing {
        report.push(
            "decision.crossing_flag",
            format!("{owner} crosses_canonical_boundary disagrees with the boundary outcome"),
        );
    }

    // required_safety is the full set on a crossing, empty otherwise.
    let expected_required = if expected_crossing {
        SafetyRequirement::ALL.to_vec()
    } else {
        Vec::new()
    };
    if decision.required_safety != expected_required {
        report.push(
            "decision.required_safety",
            format!(
                "{owner} required_safety must be the full set on a crossing and empty otherwise"
            ),
        );
    }

    // Actor lineage records the route's journal actor class and the declared
    // reversal class.
    if decision.actor_lineage.actor_class != attempt.route.actor_class_token() {
        report.push(
            "decision.lineage_actor_class",
            format!("{owner} actor lineage must record the route's journal actor class"),
        );
    }
    if decision.actor_lineage.reversal_class != attempt.envelope.reversal_class {
        report.push(
            "decision.lineage_reversal_class",
            format!("{owner} actor lineage must record the declared reversal class"),
        );
    }
    if decision.support_summary.trim().is_empty() {
        report.push(
            "decision.support_summary",
            format!("{owner} decision must carry a support summary"),
        );
    }

    if decision.copy_line != mutation_guardrails_copy_line(decision) {
        report.push(
            "decision.copy_line",
            format!("{owner} stamped copy line disagrees with the engine"),
        );
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &MutationGuardrailPacket) {
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
    for required in MutationGuardrailSurface::ALL {
        if !surfaces.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind surface {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in mutation-guardrails fixture against the frozen
/// contract.
pub fn validate_mutation_guardrails_fixture(
    fixture: &MutationGuardrailFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != MUTATION_GUARDRAILS_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != MUTATION_GUARDRAILS_SCHEMA_VERSION {
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

    let decision = &fixture.case.decision;
    if fixture.expected_guardrail_outcome != decision.guardrail_outcome {
        report.push(
            "fixture.expected_guardrail_outcome",
            format!(
                "fixture {} expected outcome disagrees with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_mutation_admitted != decision.mutation_admitted {
        report.push(
            "fixture.expected_mutation_admitted",
            format!(
                "fixture {} expected admit flag disagrees with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_crosses_canonical_boundary != decision.crosses_canonical_boundary {
        report.push(
            "fixture.expected_crosses_canonical_boundary",
            format!(
                "fixture {} expected crossing flag disagrees with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_unmet_safety_requirements != decision.unmet_safety_requirements {
        report.push(
            "fixture.expected_unmet_safety_requirements",
            format!(
                "fixture {} expected unmet requirements disagree with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_why_blocked_tokens != decision.why_blocked_tokens {
        report.push(
            "fixture.expected_why_blocked_tokens",
            format!(
                "fixture {} expected why-blocked tokens disagree with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_leaves_divergence != decision.guardrail_outcome.leaves_divergence() {
        report.push(
            "fixture.expected_leaves_divergence",
            format!(
                "fixture {} expected divergence flag disagrees with the decision",
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
