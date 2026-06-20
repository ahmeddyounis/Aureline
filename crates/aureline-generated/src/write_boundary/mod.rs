//! Blocked-direct-edit, reviewed-override, and diverged-from-generator
//! write-boundary decisions for claimed M5 generated-artifact classes.
//!
//! The sibling [`crate::m5_generated_governance`] matrix certifies
//! generated-artifact truth one row per *class*, and the
//! [`crate::descriptor`] lane models the per-*artifact* identity object the
//! surfaces render. This module models the *write boundary itself*: what
//! happens when a user attempts a direct edit to a generated artifact.
//!
//! Each [`WriteBoundaryCase`] pairs a [`WriteBoundarySubject`] — the
//! artifact's class, authority, declared writable-boundary posture, current
//! [`BoundaryState`], canonical-source linkage, regeneration route, and any
//! recorded reviewed override — with the [`WriteBoundaryDecision`] the single
//! [`decide_write_boundary`] engine reaches. The decision is the one object
//! the file-tree save gate, the review override sheet, the diverged-state
//! lineage, the AI context, and the support export all render, so no surface
//! can disagree about whether a direct edit is admitted, held for review, or
//! blocked.
//!
//! Five guardrails are frozen here:
//!
//! - **Blocked by default, never silent.** A non-authoritative generated
//!   artifact is never mutated silently: a direct edit is either admitted
//!   (the artifact is its own canonical source), held through a visible
//!   reviewed override, or blocked in favor of regeneration. The reason is
//!   always carried as a [`WriteBoundaryDecision::why_blocked_tokens`] set,
//!   never reduced to a generic save failure or buried in a log.
//! - **Reviewed override, then divergence.** When a direct edit escalates
//!   through a reviewed override, the engine admits it *only* with a recorded
//!   review, and the admitted override leaves a durable
//!   [`DivergedFromGenerator`] state with an explicit recovery path —
//!   regenerate to discard the divergence, or reconcile the change into the
//!   canonical source.
//! - **Three-way compare, no lost provenance.** Every decision carries a
//!   [`ThreeWayCompare`] over the canonical source, the current artifact, and
//!   the regenerated candidate. Each leg keeps its provenance reference even
//!   when the leg cannot be produced right now, so the user can always see
//!   what the artifact derives from and what a regeneration would yield.
//! - **The five states are explicit.** [`BoundaryState`] names the writable
//!   condition — in sync, drift detected, source missing, generator
//!   unavailable, or regeneration blocked by policy — and each state drives
//!   the gate, the compare-leg availability, and the recovery path the
//!   decision surfaces.
//! - **The boundary only narrows.** The effective edit gate starts at the
//!   declared posture and is floored by the boundary state; it never widens,
//!   so a derived artifact never becomes more directly editable than its
//!   boundary proves, and there is no force-write escape beyond the reviewed
//!   override model.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/write-boundary.schema.json`](../../../../schemas/generated/write-boundary.schema.json)
//! - [`/docs/generated/write-boundary-review.md`](../../../../docs/generated/write-boundary-review.md)
//! - [`/artifacts/generated/write-boundary-packet.json`](../../../../artifacts/generated/write-boundary-packet.json)
//! - [`/artifacts/generated/write-boundary.md`](../../../../artifacts/generated/write-boundary.md)
//! - [`/fixtures/generated/write-boundary/`](../../../../fixtures/generated/write-boundary/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use crate::descriptor::{GeneratorIdentity, GeneratorKind};
pub use crate::m5_generated_governance::{ArtifactClass, AuthorityClass, EditPosture};

/// Schema version stamped onto the packet and fixtures.
pub const WRITE_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const WRITE_BOUNDARY_PACKET_RECORD_KIND: &str = "write_boundary_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const WRITE_BOUNDARY_FIXTURE_RECORD_KIND: &str = "write_boundary_fixture_record";

/// Stable packet id every surface binding ingests.
pub const WRITE_BOUNDARY_PACKET_ID: &str = "generated.write_boundary.v1";

/// Repo-relative schema ref.
pub const WRITE_BOUNDARY_SCHEMA_REF: &str = "schemas/generated/write-boundary.schema.json";

/// Repo-relative reviewer doc ref.
pub const WRITE_BOUNDARY_DOC_REF: &str = "docs/generated/write-boundary-review.md";

/// Repo-relative machine-readable proof packet.
pub const WRITE_BOUNDARY_PACKET_REF: &str = "artifacts/generated/write-boundary-packet.json";

/// Repo-relative reviewer certification summary.
pub const WRITE_BOUNDARY_REPORT_REF: &str = "artifacts/generated/write-boundary.md";

/// Repo-relative fixture directory.
pub const WRITE_BOUNDARY_FIXTURE_DIR: &str = "fixtures/generated/write-boundary";

/// Repo-relative fixture manifest.
pub const WRITE_BOUNDARY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/write-boundary/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// The writable condition of a generated artifact's boundary. These are the
/// consistent states every surface names: a generated artifact is *in sync*
/// with its canonical source, has *drift detected* against it, has lost the
/// *source*, has lost the *generator*, or has its *regeneration blocked by
/// policy*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryState {
    /// The derived bytes match the canonical source and the artifact can be
    /// regenerated from it.
    InSync,
    /// The derived bytes have diverged from the canonical source, so a direct
    /// edit risks clobbering an unreconciled change.
    DriftDetected,
    /// The canonical source is absent, so the artifact cannot be compared or
    /// regenerated against it.
    SourceMissing,
    /// The generator that rebuilds the artifact is unavailable, so the
    /// regeneration route cannot run.
    GeneratorUnavailable,
    /// A policy forbids regenerating the artifact, so the regeneration route
    /// is blocked even though the generator and source exist.
    RegenerationBlockedByPolicy,
}

impl BoundaryState {
    /// Every boundary state in canonical order.
    pub const ALL: [Self; 5] = [
        Self::InSync,
        Self::DriftDetected,
        Self::SourceMissing,
        Self::GeneratorUnavailable,
        Self::RegenerationBlockedByPolicy,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::DriftDetected => "drift_detected",
            Self::SourceMissing => "source_missing",
            Self::GeneratorUnavailable => "generator_unavailable",
            Self::RegenerationBlockedByPolicy => "regeneration_blocked_by_policy",
        }
    }

    /// A short surface-agnostic label for the state.
    pub const fn short_label(self) -> &'static str {
        match self {
            Self::InSync => "In sync",
            Self::DriftDetected => "Drift detected",
            Self::SourceMissing => "Source missing",
            Self::GeneratorUnavailable => "Generator unavailable",
            Self::RegenerationBlockedByPolicy => "Regeneration blocked by policy",
        }
    }

    /// The writable-boundary floor this state forces on the edit gate, if
    /// any.
    ///
    /// A drifting artifact can only be edited through a reviewed override
    /// that reconciles the divergence. A missing source, an unavailable
    /// generator, or a policy block all leave the artifact with no safe
    /// in-place edit, so they force a regenerate-only gate; the decision then
    /// surfaces the precondition that must be restored before regeneration
    /// can run.
    pub const fn edit_gate_floor(self) -> Option<EditPosture> {
        match self {
            Self::InSync => None,
            Self::DriftDetected => Some(EditPosture::ReviewedOverrideRequired),
            Self::SourceMissing
            | Self::GeneratorUnavailable
            | Self::RegenerationBlockedByPolicy => Some(EditPosture::RegenerateOnly),
        }
    }

    /// The regeneration availability this state implies.
    pub const fn regeneration_availability(self) -> RegenerationAvailability {
        match self {
            Self::InSync | Self::DriftDetected => RegenerationAvailability::Available,
            Self::SourceMissing => RegenerationAvailability::BlockedSourceMissing,
            Self::GeneratorUnavailable => RegenerationAvailability::BlockedGeneratorUnavailable,
            Self::RegenerationBlockedByPolicy => RegenerationAvailability::BlockedByPolicy,
        }
    }

    /// Whether the canonical source is recorded and linkable in this state.
    pub const fn canonical_source_linked(self) -> bool {
        !matches!(self, Self::SourceMissing)
    }
}

/// Whether the artifact can be regenerated right now, and why not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegenerationAvailability {
    /// The generator and canonical source are present and policy permits
    /// regeneration.
    Available,
    /// Regeneration cannot run because the canonical source is missing.
    BlockedSourceMissing,
    /// Regeneration cannot run because the generator is unavailable.
    BlockedGeneratorUnavailable,
    /// Regeneration is blocked by policy.
    BlockedByPolicy,
}

impl RegenerationAvailability {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::BlockedSourceMissing => "blocked_source_missing",
            Self::BlockedGeneratorUnavailable => "blocked_generator_unavailable",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }

    /// Whether a regenerated-candidate leg can be produced right now.
    pub const fn regenerated_leg_available(self) -> bool {
        matches!(self, Self::Available)
    }

    /// The block-reason token an unavailable regeneration contributes, if
    /// any.
    pub const fn block_token(self) -> Option<&'static str> {
        match self {
            Self::Available => None,
            Self::BlockedSourceMissing => Some("source_missing"),
            Self::BlockedGeneratorUnavailable => Some("generator_unavailable"),
            Self::BlockedByPolicy => Some("regeneration_blocked_by_policy"),
        }
    }
}

/// The outcome of attempting a direct edit against a generated artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttemptOutcome {
    /// The direct edit is admitted: the artifact is its own canonical source
    /// and in sync, so editing it directly is safe.
    DirectEditAdmitted,
    /// The direct edit is held: it crosses a canonical-source boundary and
    /// must escalate through a visible reviewed override before it can be
    /// applied.
    BlockedPendingReview,
    /// A reviewed override was recorded, so the edit is admitted as an
    /// override and leaves a durable diverged-from-generator state.
    OverrideAdmittedWithDivergence,
    /// The direct edit is blocked: the artifact must be regenerated from its
    /// canonical source rather than edited in place.
    BlockedRegenerateFirst,
}

impl AttemptOutcome {
    /// Every outcome in canonical order.
    pub const ALL: [Self; 4] = [
        Self::DirectEditAdmitted,
        Self::BlockedPendingReview,
        Self::OverrideAdmittedWithDivergence,
        Self::BlockedRegenerateFirst,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectEditAdmitted => "direct_edit_admitted",
            Self::BlockedPendingReview => "blocked_pending_review",
            Self::OverrideAdmittedWithDivergence => "override_admitted_with_divergence",
            Self::BlockedRegenerateFirst => "blocked_regenerate_first",
        }
    }

    /// Whether this outcome admits the write to disk at all (directly or as a
    /// reviewed override).
    pub const fn admits_write(self) -> bool {
        matches!(
            self,
            Self::DirectEditAdmitted | Self::OverrideAdmittedWithDivergence
        )
    }

    /// Whether this outcome is a plain, unreviewed direct edit.
    pub const fn is_direct_edit(self) -> bool {
        matches!(self, Self::DirectEditAdmitted)
    }

    /// Whether this outcome leaves a durable diverged-from-generator state.
    pub const fn leaves_divergence(self) -> bool {
        matches!(self, Self::OverrideAdmittedWithDivergence)
    }
}

/// One leg of a three-way compare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareLegKind {
    /// The canonical source the artifact derives from.
    CanonicalSource,
    /// The current generated bytes on disk.
    CurrentArtifact,
    /// The candidate bytes a regeneration would produce.
    RegeneratedCandidate,
}

impl CompareLegKind {
    /// Every compare leg in canonical order.
    pub const ALL: [Self; 3] = [
        Self::CanonicalSource,
        Self::CurrentArtifact,
        Self::RegeneratedCandidate,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSource => "canonical_source",
            Self::CurrentArtifact => "current_artifact",
            Self::RegeneratedCandidate => "regenerated_candidate",
        }
    }
}

/// Whether a compare leg can be produced right now.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LegAvailability {
    /// The leg can be rendered.
    Available,
    /// The leg cannot be rendered right now; its provenance reference is
    /// still preserved.
    Unavailable,
}

impl LegAvailability {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Unavailable => "unavailable",
        }
    }
}

/// The class of a recovery step a blocked or diverged decision offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryClass {
    /// Regenerate the artifact from its canonical source, discarding the
    /// derived bytes.
    RegenerateFromSource,
    /// Promote the local change into the canonical source so the next
    /// regeneration preserves it.
    ReconcileIntoSource,
    /// Escalate the direct edit through the reviewed-override sheet.
    ReviewedOverride,
    /// Restore the missing canonical source before regenerating.
    RestoreCanonicalSource,
    /// Restore the unavailable generator before regenerating.
    RestoreGenerator,
    /// Resolve the policy that blocks regeneration.
    ResolveRegenerationPolicy,
}

impl RecoveryClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::ReconcileIntoSource => "reconcile_into_source",
            Self::ReviewedOverride => "reviewed_override",
            Self::RestoreCanonicalSource => "restore_canonical_source",
            Self::RestoreGenerator => "restore_generator",
            Self::ResolveRegenerationPolicy => "resolve_regeneration_policy",
        }
    }

    /// A short reviewer summary for the step.
    pub const fn summary(self) -> &'static str {
        match self {
            Self::RegenerateFromSource => {
                "Regenerate the artifact from its canonical source, discarding the local bytes."
            }
            Self::ReconcileIntoSource => {
                "Reconcile the change into the canonical source so the next regeneration keeps it."
            }
            Self::ReviewedOverride => {
                "Escalate the direct edit through the reviewed-override sheet before it is applied."
            }
            Self::RestoreCanonicalSource => {
                "Restore the missing canonical source, then regenerate from it."
            }
            Self::RestoreGenerator => {
                "Restore the unavailable generator, then regenerate from the canonical source."
            }
            Self::ResolveRegenerationPolicy => {
                "Resolve the policy that blocks regeneration, then regenerate from the canonical source."
            }
        }
    }
}

/// A surface that renders the write-boundary decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteBoundarySurface {
    /// The file-tree / save boundary that intercepts a write to a generated
    /// file.
    FileTreeSaveGate,
    /// The diff/review reviewed-override sheet.
    ReviewOverrideSheet,
    /// The durable diverged-from-generator lineage record.
    DivergedStateLineage,
    /// The AI prompt-context attachment line.
    AiContext,
    /// The metadata-first support export.
    SupportExport,
}

impl WriteBoundarySurface {
    /// Every rendered surface in canonical order.
    pub const ALL: [Self; 5] = [
        Self::FileTreeSaveGate,
        Self::ReviewOverrideSheet,
        Self::DivergedStateLineage,
        Self::AiContext,
        Self::SupportExport,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileTreeSaveGate => "file_tree_save_gate",
            Self::ReviewOverrideSheet => "review_override_sheet",
            Self::DivergedStateLineage => "diverged_state_lineage",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }
}

// ---------------------------------------------------------------------------
// Subject and decision.
// ---------------------------------------------------------------------------

/// The inputs the write-boundary engine reads for one artifact: everything
/// needed to decide whether a direct edit is admitted, held, or blocked.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundarySubject {
    /// Generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe display label for the artifact path.
    pub artifact_path_label: String,
    /// Provenance/authority class of the bytes relative to the source.
    pub authority_class: AuthorityClass,
    /// Generator that produced the artifact, with version.
    pub generator: GeneratorIdentity,
    /// Writable-boundary posture declared for the artifact before narrowing.
    pub declared_edit_posture: EditPosture,
    /// Current writable condition of the artifact's boundary.
    pub boundary_state: BoundaryState,
    /// Review-safe canonical-source reference. Present unless the boundary
    /// state is [`BoundaryState::SourceMissing`].
    pub canonical_source_ref: Option<String>,
    /// Review-safe regeneration route that rebuilds the artifact.
    pub regeneration_route: String,
    /// Reference to the reversible-checkpoint lineage that captured the
    /// change.
    pub checkpoint_lineage_ref: String,
    /// Reference to a recorded reviewed override, when one was supplied for
    /// this attempt.
    pub override_review_ref: Option<String>,
}

/// One leg of the [`ThreeWayCompare`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareLeg {
    /// Which leg this is.
    pub kind: CompareLegKind,
    /// Whether the leg can be produced right now.
    pub availability: LegAvailability,
    /// Review-safe provenance reference for the leg, preserved even when the
    /// leg is unavailable.
    pub provenance_ref: String,
    /// Stable token naming why the leg is unavailable, if it is.
    pub unavailable_reason: Option<String>,
}

/// The three-way compare a decision offers: canonical source, current bytes,
/// and regenerated candidate. Every leg keeps its provenance reference even
/// when it cannot be produced, so a compare never loses provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ThreeWayCompare {
    /// One entry per [`CompareLegKind`], in canonical order.
    pub legs: Vec<CompareLeg>,
    /// True when every leg preserved its provenance reference.
    pub provenance_preserved: bool,
    /// Review-safe summary of which legs are available.
    pub summary: String,
}

/// One recovery step a blocked or diverged decision offers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryStep {
    /// Recovery class.
    pub class: RecoveryClass,
    /// Review-safe route the step takes.
    pub action_ref: String,
    /// Short reviewer summary.
    pub summary: String,
}

/// A jump action to the canonical source, offered whenever the source is
/// linkable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceJump {
    /// Review-safe canonical-source reference.
    pub source_ref: String,
    /// Surface-agnostic action label.
    pub label: String,
}

/// The durable diverged-from-generator state left by an admitted reviewed
/// override.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DivergedFromGenerator {
    /// Stable divergence id.
    pub divergence_id: String,
    /// Always true: the record exists precisely because the artifact diverged
    /// from its generator.
    pub diverged: bool,
    /// Reference to the recorded reviewed override that admitted the
    /// divergence.
    pub override_review_ref: String,
    /// Recovery path out of the divergence — regenerate to discard, or
    /// reconcile into the canonical source.
    pub recovery: Vec<RecoveryStep>,
    /// Review-safe summary of the divergence and its recovery.
    pub summary: String,
}

/// The computed write-boundary decision the surfaces render.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundaryDecision {
    /// Current writable condition of the boundary.
    pub boundary_state: BoundaryState,
    /// Writable-boundary posture after the boundary state floors the declared
    /// one.
    pub effective_edit_gate: EditPosture,
    /// True when the effective gate narrowed below the declared posture.
    pub edit_gate_downgraded: bool,
    /// The outcome of the direct-edit attempt.
    pub attempt_outcome: AttemptOutcome,
    /// Convenience: whether a plain direct edit was admitted.
    pub direct_edit_admitted: bool,
    /// Stable tokens naming every input that blocked or escalated the direct
    /// edit, sorted and deduplicated. Empty only when the edit is directly
    /// admitted.
    pub why_blocked_tokens: Vec<String>,
    /// Jump action to the canonical source, when it is linkable.
    pub canonical_source_jump: Option<CanonicalSourceJump>,
    /// Whether the artifact can be regenerated right now, and why not.
    pub regeneration_availability: RegenerationAvailability,
    /// The three-way compare over source, current bytes, and regenerated
    /// candidate.
    pub three_way_compare: ThreeWayCompare,
    /// The durable diverged-from-generator state, present only for an admitted
    /// reviewed override.
    pub diverged_from_generator: Option<DivergedFromGenerator>,
    /// The recovery path the decision offers, empty only when the edit is
    /// directly admitted.
    pub recovery: Vec<RecoveryStep>,
    /// The user-visible regenerate-first / override guidance line.
    pub guidance_line: String,
    /// The one stable copy/export form for the decision.
    pub copy_line: String,
}

// ---------------------------------------------------------------------------
// Engine: the single source of truth for the decision.
// ---------------------------------------------------------------------------

/// Decides the write boundary for an artifact from its subject.
///
/// This is the canonical engine the cases, the fixtures, the validators, and
/// the consuming surfaces all share. The effective edit gate starts at the
/// declared posture and is floored by the boundary state — the gate only
/// narrows, never widens. The attempt outcome then follows from the gate and
/// any recorded reviewed override:
///
/// - a `direct_edit_allowed` gate admits the edit directly;
/// - a `reviewed_override_required` gate holds the edit pending review, and
///   admits it with a recorded override — leaving a durable
///   [`DivergedFromGenerator`] state;
/// - a `regenerate_only` gate blocks the edit in favor of regeneration.
///
/// A reviewed override is honored only on a `reviewed_override_required`
/// gate: it is never a force-write escape past a `regenerate_only` block.
pub fn decide_write_boundary(subject: &WriteBoundarySubject) -> WriteBoundaryDecision {
    let boundary_state = subject.boundary_state;
    let declared = subject.declared_edit_posture;

    // The gate only narrows: floor the declared posture by the boundary
    // state, taking the stricter of the two.
    let mut effective_edit_gate = declared;
    if let Some(floor) = boundary_state.edit_gate_floor() {
        if floor.severity() > effective_edit_gate.severity() {
            effective_edit_gate = floor;
        }
    }
    let edit_gate_downgraded = effective_edit_gate.severity() > declared.severity();

    let regeneration_availability = boundary_state.regeneration_availability();

    let attempt_outcome = match effective_edit_gate {
        EditPosture::DirectEditAllowed => AttemptOutcome::DirectEditAdmitted,
        EditPosture::ReviewedOverrideRequired => {
            if subject.override_review_ref.is_some() {
                AttemptOutcome::OverrideAdmittedWithDivergence
            } else {
                AttemptOutcome::BlockedPendingReview
            }
        }
        EditPosture::RegenerateOnly => AttemptOutcome::BlockedRegenerateFirst,
    };

    // Why-blocked tokens name every input that made the gate stricter than a
    // plain direct edit. They are always carried, never reduced to a generic
    // save failure.
    let mut why_blocked_tokens = Vec::new();
    if effective_edit_gate != EditPosture::DirectEditAllowed {
        if declared != EditPosture::DirectEditAllowed {
            why_blocked_tokens.push(format!("declared_{}", declared.as_str()));
        }
        if boundary_state.edit_gate_floor().is_some() {
            why_blocked_tokens.push(format!("boundary_{}", boundary_state.as_str()));
        }
    }
    why_blocked_tokens.sort();
    why_blocked_tokens.dedup();

    let canonical_source_jump = subject
        .canonical_source_ref
        .as_ref()
        .filter(|_| boundary_state.canonical_source_linked())
        .map(|source_ref| CanonicalSourceJump {
            source_ref: source_ref.clone(),
            label: format!("Open canonical source {source_ref}"),
        });

    let three_way_compare = build_compare(subject, regeneration_availability);

    let diverged_from_generator = if attempt_outcome.leaves_divergence() {
        let override_review_ref = subject
            .override_review_ref
            .clone()
            .unwrap_or_else(|| "recorded reviewed override".to_owned());
        let recovery = divergence_recovery(subject);
        Some(DivergedFromGenerator {
            divergence_id: format!("write-boundary.divergence.{}", subject.artifact_class.as_str()),
            diverged: true,
            override_review_ref,
            recovery,
            summary: format!(
                "{} diverges from {} after a reviewed override; regenerate to discard or reconcile into the canonical source.",
                subject.artifact_path_label,
                subject.generator.copy_form()
            ),
        })
    } else {
        None
    };

    let recovery = match attempt_outcome {
        AttemptOutcome::OverrideAdmittedWithDivergence => divergence_recovery(subject),
        _ => recovery_for(subject, attempt_outcome, regeneration_availability),
    };

    let guidance_line = guidance_for(subject, attempt_outcome, regeneration_availability);

    let copy_line = copy_line_for(
        subject.artifact_class,
        subject.authority_class,
        boundary_state,
        effective_edit_gate,
        attempt_outcome,
        regeneration_availability,
    );

    WriteBoundaryDecision {
        boundary_state,
        effective_edit_gate,
        edit_gate_downgraded,
        attempt_outcome,
        direct_edit_admitted: attempt_outcome.is_direct_edit(),
        why_blocked_tokens,
        canonical_source_jump,
        regeneration_availability,
        three_way_compare,
        diverged_from_generator,
        recovery,
        guidance_line,
        copy_line,
    }
}

fn build_compare(
    subject: &WriteBoundarySubject,
    regeneration_availability: RegenerationAvailability,
) -> ThreeWayCompare {
    let canonical_ref = subject
        .canonical_source_ref
        .clone()
        .unwrap_or_else(|| "canonical source not recorded".to_owned());
    let source_available =
        subject.boundary_state.canonical_source_linked() && subject.canonical_source_ref.is_some();

    let legs = vec![
        CompareLeg {
            kind: CompareLegKind::CanonicalSource,
            availability: if source_available {
                LegAvailability::Available
            } else {
                LegAvailability::Unavailable
            },
            provenance_ref: canonical_ref,
            unavailable_reason: if source_available {
                None
            } else {
                Some("source_missing".to_owned())
            },
        },
        CompareLeg {
            kind: CompareLegKind::CurrentArtifact,
            availability: LegAvailability::Available,
            provenance_ref: subject.artifact_path_label.clone(),
            unavailable_reason: None,
        },
        CompareLeg {
            kind: CompareLegKind::RegeneratedCandidate,
            availability: if regeneration_availability.regenerated_leg_available() {
                LegAvailability::Available
            } else {
                LegAvailability::Unavailable
            },
            // The route is always preserved, so the compare names what a
            // regeneration would produce even when it cannot run right now.
            provenance_ref: subject.regeneration_route.clone(),
            unavailable_reason: regeneration_availability.block_token().map(str::to_owned),
        },
    ];

    let provenance_preserved = legs.iter().all(|leg| !leg.provenance_ref.trim().is_empty());
    let available = legs
        .iter()
        .filter(|leg| leg.availability == LegAvailability::Available)
        .count();
    let summary = format!(
        "Three-way compare over canonical source, current artifact, and regenerated candidate; {available} of {} legs available, provenance preserved on all.",
        legs.len()
    );

    ThreeWayCompare {
        legs,
        provenance_preserved,
        summary,
    }
}

fn recovery_step(class: RecoveryClass, action_ref: String) -> RecoveryStep {
    RecoveryStep {
        class,
        action_ref,
        summary: class.summary().to_owned(),
    }
}

fn regenerate_step(subject: &WriteBoundarySubject) -> RecoveryStep {
    recovery_step(
        RecoveryClass::RegenerateFromSource,
        subject.regeneration_route.clone(),
    )
}

fn divergence_recovery(subject: &WriteBoundarySubject) -> Vec<RecoveryStep> {
    let reconcile_ref = subject
        .canonical_source_ref
        .clone()
        .unwrap_or_else(|| "canonical source".to_owned());
    vec![
        regenerate_step(subject),
        recovery_step(RecoveryClass::ReconcileIntoSource, reconcile_ref),
    ]
}

fn recovery_for(
    subject: &WriteBoundarySubject,
    outcome: AttemptOutcome,
    regeneration_availability: RegenerationAvailability,
) -> Vec<RecoveryStep> {
    match outcome {
        AttemptOutcome::DirectEditAdmitted => Vec::new(),
        AttemptOutcome::BlockedPendingReview => vec![
            recovery_step(
                RecoveryClass::ReviewedOverride,
                "reviewed-override sheet".to_owned(),
            ),
            regenerate_step(subject),
        ],
        AttemptOutcome::OverrideAdmittedWithDivergence => divergence_recovery(subject),
        AttemptOutcome::BlockedRegenerateFirst => match regeneration_availability {
            RegenerationAvailability::Available => vec![regenerate_step(subject)],
            RegenerationAvailability::BlockedSourceMissing => vec![
                recovery_step(
                    RecoveryClass::RestoreCanonicalSource,
                    "restore the canonical source".to_owned(),
                ),
                regenerate_step(subject),
            ],
            RegenerationAvailability::BlockedGeneratorUnavailable => vec![
                recovery_step(
                    RecoveryClass::RestoreGenerator,
                    subject.generator.copy_form(),
                ),
                regenerate_step(subject),
            ],
            RegenerationAvailability::BlockedByPolicy => vec![
                recovery_step(
                    RecoveryClass::ResolveRegenerationPolicy,
                    "regeneration policy".to_owned(),
                ),
                regenerate_step(subject),
            ],
        },
    }
}

fn guidance_for(
    subject: &WriteBoundarySubject,
    outcome: AttemptOutcome,
    regeneration_availability: RegenerationAvailability,
) -> String {
    match outcome {
        AttemptOutcome::DirectEditAdmitted => {
            "Direct edit allowed: this artifact is its own canonical source and is in sync."
                .to_owned()
        }
        AttemptOutcome::BlockedPendingReview => format!(
            "Direct edit held: editing {} across its generator boundary requires a reviewed override. Compare against the canonical source and the regenerated candidate first.",
            subject.artifact_path_label
        ),
        AttemptOutcome::OverrideAdmittedWithDivergence => {
            "Reviewed override admitted: the artifact now diverges from its generator. Regenerate to discard the divergence, or reconcile the change into the canonical source."
                .to_owned()
        }
        AttemptOutcome::BlockedRegenerateFirst => match regeneration_availability {
            RegenerationAvailability::Available => format!(
                "Direct edit blocked: regenerate {} from its canonical source rather than editing the derived bytes.",
                subject.artifact_path_label
            ),
            RegenerationAvailability::BlockedSourceMissing => {
                "Direct edit blocked and regeneration unavailable: the canonical source is missing. Restore the source before regenerating."
                    .to_owned()
            }
            RegenerationAvailability::BlockedGeneratorUnavailable => {
                "Direct edit blocked and regeneration unavailable: the generator is unavailable. Restore the generator before regenerating."
                    .to_owned()
            }
            RegenerationAvailability::BlockedByPolicy => {
                "Direct edit blocked and regeneration blocked by policy: resolve the regeneration policy before this artifact can be rebuilt."
                    .to_owned()
            }
        },
    }
}

/// Computes the stable copy/export form for a decision.
pub fn write_boundary_copy_line(
    decision: &WriteBoundaryDecision,
    subject: &WriteBoundarySubject,
) -> String {
    copy_line_for(
        subject.artifact_class,
        subject.authority_class,
        decision.boundary_state,
        decision.effective_edit_gate,
        decision.attempt_outcome,
        decision.regeneration_availability,
    )
}

fn copy_line_for(
    artifact_class: ArtifactClass,
    authority_class: AuthorityClass,
    boundary_state: BoundaryState,
    effective_edit_gate: EditPosture,
    attempt_outcome: AttemptOutcome,
    regeneration_availability: RegenerationAvailability,
) -> String {
    format!(
        "write-boundary class={} authority={} boundary={} gate={} outcome={} direct_edit={} regen={}",
        artifact_class.as_str(),
        authority_class.as_str(),
        boundary_state.as_str(),
        effective_edit_gate.as_str(),
        attempt_outcome.as_str(),
        attempt_outcome.is_direct_edit(),
        regeneration_availability.as_str(),
    )
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One write-boundary case: a subject and the decision the engine stamps onto
/// it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundaryCase {
    /// Stable case id.
    pub case_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The artifact subject the engine reads.
    pub subject: WriteBoundarySubject,
    /// The decision the engine reached.
    pub decision: WriteBoundaryDecision,
    /// Upstream generated-artifact packets backing this case.
    pub evidence_refs: Vec<String>,
    /// One real consumer that renders this case.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a surface ingests this packet rather than re-deriving
/// write-boundary truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundarySurfaceBinding {
    /// Surface that ingests the packet.
    pub surface: WriteBoundarySurface,
    /// Checked consumer ref that renders the decision.
    pub consumer_ref: String,
    /// Packet id the surface ingests.
    pub ingested_packet_id: String,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundarySourceContractRefs {
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

/// Top-level packet modeling write-boundary decisions across the claimed M5
/// generated-artifact classes and boundary states.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundaryPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: WriteBoundarySourceContractRefs,
    /// The consistent boundary states the packet models.
    pub boundary_states: Vec<BoundaryState>,
    /// Upstream generated-artifact packets this lane composes.
    pub evidence_packet_refs: Vec<String>,
    /// Write-boundary cases.
    pub cases: Vec<WriteBoundaryCase>,
    /// Surface bindings, one per rendered surface.
    pub surface_bindings: Vec<WriteBoundarySurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a case to its expected decision, proving the canonical
/// decision behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WriteBoundaryFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Reviewer scenario label.
    pub scenario: String,
    /// The case under test.
    pub case: WriteBoundaryCase,
    /// Expected attempt outcome.
    pub expected_attempt_outcome: AttemptOutcome,
    /// Expected effective edit gate.
    pub expected_effective_edit_gate: EditPosture,
    /// Expected boundary state.
    pub expected_boundary_state: BoundaryState,
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
        writeln!(f, "write-boundary validation failed")?;
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
const SAVE_REVIEW_REF: &str = "artifacts/fs/save_review_choice_matrix.yaml";
const MUTATION_CLASSES_REF: &str = "artifacts/change/mutation_classes.yaml";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";
const RESTORE_PROVENANCE_REF: &str = "artifacts/migration/m3/restore_provenance_packet.md";
const DIVERGENCE_CONTRACT_REF: &str = "docs/generated/diverged_from_generator_contract.md";
const DIVERGENCE_SCHEMA_REF: &str = "schemas/generated/divergence_record.schema.json";

/// The reviewed-override review evidence a recorded override cites.
const OVERRIDE_REVIEW_REF: &str = "artifacts/fs/save_review_choice_matrix.yaml";

fn evidence_packet_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        SAVE_REVIEW_REF,
        MUTATION_CLASSES_REF,
        ROLLBACK_CHECKPOINT_REF,
        RESTORE_PROVENANCE_REF,
        DIVERGENCE_CONTRACT_REF,
        DIVERGENCE_SCHEMA_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn case_evidence_refs() -> Vec<String> {
    [
        GOVERNANCE_PACKET_REF,
        DESCRIPTOR_PACKET_REF,
        DIVERGENCE_CONTRACT_REF,
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

/// Builds a subject for a class in a given boundary state, optionally with a
/// recorded reviewed override.
fn subject_for(
    artifact_class: ArtifactClass,
    boundary_state: BoundaryState,
    override_recorded: bool,
) -> WriteBoundarySubject {
    let (authority_class, declared_edit_posture) = class_authority(artifact_class);
    let canonical_source_ref = if boundary_state.canonical_source_linked() {
        Some(class_source_ref(artifact_class).to_owned())
    } else {
        None
    };
    let override_review_ref = if override_recorded {
        Some(OVERRIDE_REVIEW_REF.to_owned())
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
        override_review_ref,
    }
}

fn case(
    case_id: &str,
    scenario: &str,
    subject: WriteBoundarySubject,
    notes: &str,
) -> WriteBoundaryCase {
    let consumer_ref = class_consumer_ref(subject.artifact_class).to_owned();
    let decision = decide_write_boundary(&subject);
    WriteBoundaryCase {
        case_id: case_id.to_owned(),
        scenario: scenario.to_owned(),
        subject,
        decision,
        evidence_refs: case_evidence_refs(),
        consumer_ref,
        notes: notes.to_owned(),
    }
}

fn binding(
    surface: WriteBoundarySurface,
    consumer_ref: &str,
    summary: &str,
) -> WriteBoundarySurfaceBinding {
    WriteBoundarySurfaceBinding {
        surface,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: WRITE_BOUNDARY_PACKET_ID.to_owned(),
        summary: summary.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the seeded write-boundary cases this lane freezes: one healthy
/// case per class plus one case per degraded boundary state and the reviewed
/// override that leaves a divergence.
fn seeded_cases() -> Vec<WriteBoundaryCase> {
    vec![
        // One healthy in-sync case per class, proving the default gate.
        case(
            "write-boundary.scaffolded_project.in_sync",
            "Scaffolded project in sync — direct edit allowed",
            subject_for(ArtifactClass::ScaffoldedProject, BoundaryState::InSync, false),
            "A scaffolded project is its own canonical source; in sync it is directly editable with no block.",
        ),
        case(
            "write-boundary.ai_assisted_edit.in_sync",
            "AI-assisted edit in sync — direct edit allowed",
            subject_for(ArtifactClass::AiAssistedEdit, BoundaryState::InSync, false),
            "An accepted AI-assisted edit is canonical source the user owns; in sync it is directly editable.",
        ),
        case(
            "write-boundary.notebook_output.in_sync",
            "Notebook output in sync — blocked, regenerate first",
            subject_for(ArtifactClass::NotebookOutput, BoundaryState::InSync, false),
            "A notebook output is purely derived; a direct edit is blocked in favor of re-running the cell.",
        ),
        case(
            "write-boundary.preview_derivative.in_sync",
            "Preview derivative in sync — blocked, regenerate first",
            subject_for(ArtifactClass::PreviewDerivative, BoundaryState::InSync, false),
            "A preview derivative is rebuilt from source; a direct edit is blocked in favor of regeneration.",
        ),
        case(
            "write-boundary.support_packet.in_sync",
            "Support packet in sync — blocked, regenerate first",
            subject_for(ArtifactClass::SupportPacket, BoundaryState::InSync, false),
            "A support packet is a regenerated projection; a direct edit is blocked in favor of re-export.",
        ),
        case(
            "write-boundary.request_artifact.in_sync",
            "Request artifact in sync — held for reviewed override",
            subject_for(ArtifactClass::RequestArtifact, BoundaryState::InSync, false),
            "A request artifact is derived-editable; a direct edit is held until it escalates through a reviewed override.",
        ),
        case(
            "write-boundary.framework_codegen.in_sync",
            "Framework codegen in sync — held for reviewed override",
            subject_for(ArtifactClass::FrameworkCodegen, BoundaryState::InSync, false),
            "Framework codegen is derived-editable; a direct edit is held until it escalates through a reviewed override.",
        ),
        // The reviewed-override flow that leaves a divergence.
        case(
            "write-boundary.framework_codegen.override_admitted",
            "Framework codegen with a recorded reviewed override — override admitted with divergence",
            subject_for(ArtifactClass::FrameworkCodegen, BoundaryState::InSync, true),
            "A recorded reviewed override admits the edit and leaves a durable diverged-from-generator state with a recovery path.",
        ),
        // Drift detected forces a reviewed override.
        case(
            "write-boundary.request_artifact.drift_detected",
            "Request artifact with drift detected — held for reviewed override",
            subject_for(
                ArtifactClass::RequestArtifact,
                BoundaryState::DriftDetected,
                false,
            ),
            "Drift against the captured request makes a direct edit unsafe; it is held until reconciled through a reviewed override.",
        ),
        // Source missing: blocked, with restore-source recovery.
        case(
            "write-boundary.notebook_output.source_missing",
            "Notebook output with the canonical source missing — blocked, restore source first",
            subject_for(
                ArtifactClass::NotebookOutput,
                BoundaryState::SourceMissing,
                false,
            ),
            "Without the cell source the output cannot be compared or regenerated; the recovery restores the source first.",
        ),
        // Generator unavailable: blocked, with restore-generator recovery.
        case(
            "write-boundary.preview_derivative.generator_unavailable",
            "Preview derivative with the generator unavailable — blocked, restore generator first",
            subject_for(
                ArtifactClass::PreviewDerivative,
                BoundaryState::GeneratorUnavailable,
                false,
            ),
            "Without the builder the derivative cannot be regenerated; the recovery restores the generator first.",
        ),
        // Regeneration blocked by policy: blocked, with resolve-policy recovery.
        case(
            "write-boundary.framework_codegen.regeneration_blocked_by_policy",
            "Framework codegen with regeneration blocked by policy — blocked, resolve policy first",
            subject_for(
                ArtifactClass::FrameworkCodegen,
                BoundaryState::RegenerationBlockedByPolicy,
                false,
            ),
            "Policy forbids regenerating the artifact; the decision surfaces the policy block rather than a generic save failure.",
        ),
    ]
}

/// Returns the checked-in write-boundary packet this lane freezes.
pub fn seeded_write_boundary_packet() -> WriteBoundaryPacket {
    let surface_bindings = vec![
        binding(
            WriteBoundarySurface::FileTreeSaveGate,
            "crates/aureline-vfs/src/save_conflict_suite/mod.rs",
            "The file-tree save gate intercepts a write to a generated file and renders the decision: a direct edit is admitted, held for a reviewed override, or blocked with regenerate-first guidance — never a generic save failure.",
        ),
        binding(
            WriteBoundarySurface::ReviewOverrideSheet,
            "crates/aureline-review/src/change_inspector/mod.rs",
            "The reviewed-override sheet renders the why-blocked tokens, the canonical-source jump, and the three-way compare so a direct edit only crosses the generator boundary through a recorded review.",
        ),
        binding(
            WriteBoundarySurface::DivergedStateLineage,
            "crates/aureline-workspace/src/mutation_and_generated_artifact_lineage/mod.rs",
            "The diverged-state lineage persists the diverged-from-generator record an admitted override leaves, with its recovery path, so the divergence survives the session.",
        ),
        binding(
            WriteBoundarySurface::AiContext,
            "crates/aureline-ai/src/context_inspector/mod.rs",
            "The AI context attaches the effective edit gate and boundary state so the model is told a generated file is blocked, held, or directly editable instead of editing derived bytes silently.",
        ),
        binding(
            WriteBoundarySurface::SupportExport,
            "crates/aureline-support/src/generated_lineage/mod.rs",
            "The support export re-emits the decision copy line, boundary state, and recovery path with no raw bytes, diffs, or credentials, so diagnostics cite one decision object.",
        ),
    ];

    WriteBoundaryPacket {
        record_kind: WRITE_BOUNDARY_PACKET_RECORD_KIND.to_owned(),
        schema_version: WRITE_BOUNDARY_SCHEMA_VERSION,
        packet_id: WRITE_BOUNDARY_PACKET_ID.to_owned(),
        title: "Blocked-direct-edit, reviewed-override, and diverged-from-generator write-boundary decisions for the M5 generated-artifact classes"
            .to_owned(),
        source_contract_refs: WriteBoundarySourceContractRefs {
            doc_ref: WRITE_BOUNDARY_DOC_REF.to_owned(),
            schema_ref: WRITE_BOUNDARY_SCHEMA_REF.to_owned(),
            packet_ref: WRITE_BOUNDARY_PACKET_REF.to_owned(),
            report_ref: WRITE_BOUNDARY_REPORT_REF.to_owned(),
            fixture_manifest_ref: WRITE_BOUNDARY_FIXTURE_MANIFEST_REF.to_owned(),
        },
        boundary_states: BoundaryState::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        cases: seeded_cases(),
        surface_bindings,
        invariants: vec![
            "A non-authoritative generated artifact is never mutated silently: a direct edit is admitted only when the artifact is its own canonical source and in sync, otherwise it is held for a reviewed override or blocked in favor of regeneration.".to_owned(),
            "Every block carries its reason as why-blocked tokens and a guidance line; a block is never reduced to a generic save failure or buried in a log or toast.".to_owned(),
            "A reviewed override is admitted only with a recorded review and leaves a durable diverged-from-generator state with a recovery path — regenerate to discard, or reconcile into the canonical source.".to_owned(),
            "Every decision carries a three-way compare over the canonical source, the current artifact, and the regenerated candidate; each leg preserves its provenance reference even when the leg cannot be produced right now.".to_owned(),
            "The five boundary states — in sync, drift detected, source missing, generator unavailable, and regeneration blocked by policy — are explicit, and each drives the gate, the compare-leg availability, and the recovery path.".to_owned(),
            "The edit gate only narrows: it starts at the declared posture, is floored by the boundary state, and never widens, so there is no force-write escape beyond the reviewed override model.".to_owned(),
        ],
    }
}

/// Returns the checked-in write-boundary fixture corpus this lane freezes:
/// one fixture per seeded case.
pub fn seeded_write_boundary_fixtures() -> Vec<WriteBoundaryFixture> {
    seeded_cases().into_iter().map(fixture).collect()
}

fn fixture(case: WriteBoundaryCase) -> WriteBoundaryFixture {
    let decision = &case.decision;
    WriteBoundaryFixture {
        record_kind: WRITE_BOUNDARY_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: WRITE_BOUNDARY_SCHEMA_VERSION,
        fixture_id: format!("fixture.{}", case.case_id),
        scenario: case.scenario.clone(),
        expected_attempt_outcome: decision.attempt_outcome,
        expected_effective_edit_gate: decision.effective_edit_gate,
        expected_boundary_state: decision.boundary_state,
        expected_why_blocked_tokens: decision.why_blocked_tokens.clone(),
        expected_leaves_divergence: decision.diverged_from_generator.is_some(),
        notes: case.notes.clone(),
        case,
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in write-boundary packet contract.
pub fn validate_write_boundary_packet(
    packet: &WriteBoundaryPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != WRITE_BOUNDARY_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != WRITE_BOUNDARY_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != WRITE_BOUNDARY_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != WRITE_BOUNDARY_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != WRITE_BOUNDARY_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != WRITE_BOUNDARY_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != WRITE_BOUNDARY_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != WRITE_BOUNDARY_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.boundary_states != BoundaryState::ALL.to_vec() {
        report.push(
            "packet.boundary_states",
            "packet must declare every boundary state in canonical order",
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
    let mut covered_states = BTreeSet::new();
    let mut covered_outcomes = BTreeSet::new();
    for write_case in &packet.cases {
        if !case_ids.insert(write_case.case_id.as_str()) {
            report.push(
                "case.id_unique",
                format!("duplicate case id {}", write_case.case_id),
            );
        }
        covered_states.insert(write_case.decision.boundary_state);
        covered_outcomes.insert(write_case.decision.attempt_outcome);
        validate_case(&mut report, write_case);
    }
    for required in BoundaryState::ALL {
        if !covered_states.contains(&required) {
            report.push(
                "packet.boundary_state_coverage",
                format!("packet must cover boundary state {}", required.as_str()),
            );
        }
    }
    for required in AttemptOutcome::ALL {
        if !covered_outcomes.contains(&required) {
            report.push(
                "packet.outcome_coverage",
                format!("packet must cover attempt outcome {}", required.as_str()),
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

fn validate_case(report: &mut ValidationReport, write_case: &WriteBoundaryCase) {
    let owner = format!("case {}", write_case.case_id);

    if write_case.case_id.trim().is_empty() {
        report.push("case.id", "case must carry a stable id");
    }
    if write_case.scenario.trim().is_empty() {
        report.push("case.scenario", format!("{owner} must carry a scenario"));
    }
    if write_case.consumer_ref.trim().is_empty() {
        report.push(
            "case.consumer_ref",
            format!("{owner} must cite a consumer ref"),
        );
    }
    if write_case.notes.trim().is_empty() {
        report.push("case.notes", format!("{owner} must carry a reviewer note"));
    }
    if write_case.evidence_refs.is_empty() {
        report.push(
            "case.evidence_refs",
            format!("{owner} must cite at least one evidence ref"),
        );
    }

    validate_subject(report, &owner, &write_case.subject);

    // The stamped decision must equal what the engine computes.
    let expected = decide_write_boundary(&write_case.subject);
    if write_case.decision != expected {
        report.push(
            "case.decision",
            format!("{owner} stamped decision disagrees with the engine"),
        );
    }

    validate_decision(report, &owner, &write_case.subject, &write_case.decision);
}

fn validate_subject(report: &mut ValidationReport, owner: &str, subject: &WriteBoundarySubject) {
    if subject.artifact_path_label.trim().is_empty() {
        report.push(
            "subject.path_label",
            format!("{owner} must carry an artifact path label"),
        );
    }
    if subject.generator.name.trim().is_empty() || subject.generator.version.trim().is_empty() {
        report.push(
            "subject.generator",
            format!("{owner} must carry a generator name and version"),
        );
    }
    if subject.regeneration_route.trim().is_empty() {
        report.push(
            "subject.regeneration_route",
            format!("{owner} must carry a regeneration route"),
        );
    }
    if subject.checkpoint_lineage_ref.trim().is_empty() {
        report.push(
            "subject.checkpoint_lineage_ref",
            format!("{owner} must carry a checkpoint lineage ref"),
        );
    }
    // Source linkage must agree with the boundary state.
    match subject.boundary_state {
        BoundaryState::SourceMissing => {
            if subject.canonical_source_ref.is_some() {
                report.push(
                    "subject.source_consistency",
                    format!(
                        "{owner} source-missing boundary must not carry a canonical source ref"
                    ),
                );
            }
        }
        _ => {
            if subject
                .canonical_source_ref
                .as_ref()
                .map(|s| s.trim().is_empty())
                .unwrap_or(true)
            {
                report.push(
                    "subject.source_consistency",
                    format!(
                        "{owner} non-source-missing boundary must carry a canonical source ref"
                    ),
                );
            }
        }
    }
}

fn validate_decision(
    report: &mut ValidationReport,
    owner: &str,
    subject: &WriteBoundarySubject,
    decision: &WriteBoundaryDecision,
) {
    // The gate only narrows.
    if decision.effective_edit_gate.severity() < subject.declared_edit_posture.severity() {
        report.push(
            "decision.gate_widened",
            format!("{owner} effective edit gate widened above the declared posture"),
        );
    }

    // Blocked or escalated outcomes must carry a reason; an admitted direct
    // edit must not.
    match decision.attempt_outcome {
        AttemptOutcome::DirectEditAdmitted => {
            if !decision.why_blocked_tokens.is_empty() {
                report.push(
                    "decision.admitted_no_block",
                    format!("{owner} a directly admitted edit must carry no why-blocked tokens"),
                );
            }
            if !decision.recovery.is_empty() {
                report.push(
                    "decision.admitted_no_recovery",
                    format!("{owner} a directly admitted edit needs no recovery path"),
                );
            }
        }
        _ => {
            if decision.why_blocked_tokens.is_empty() {
                report.push(
                    "decision.block_reason",
                    format!("{owner} a held or blocked edit must name why it was blocked"),
                );
            }
            if decision.recovery.is_empty() {
                report.push(
                    "decision.recovery",
                    format!("{owner} a held or blocked edit must offer a recovery path"),
                );
            }
            if decision.guidance_line.trim().is_empty() {
                report.push(
                    "decision.guidance",
                    format!("{owner} a held or blocked edit must carry a guidance line"),
                );
            }
        }
    }

    // A divergence is present exactly when an override was admitted.
    match (
        decision.attempt_outcome.leaves_divergence(),
        &decision.diverged_from_generator,
    ) {
        (true, Some(divergence)) => {
            if !divergence.diverged {
                report.push(
                    "decision.divergence_flag",
                    format!("{owner} a divergence record must be flagged diverged"),
                );
            }
            if divergence.recovery.is_empty() {
                report.push(
                    "decision.divergence_recovery",
                    format!("{owner} an admitted override must leave a divergence recovery path"),
                );
            }
            if divergence.override_review_ref.trim().is_empty() {
                report.push(
                    "decision.divergence_review",
                    format!("{owner} a divergence record must cite the reviewed override"),
                );
            }
        }
        (true, None) => report.push(
            "decision.divergence_missing",
            format!("{owner} an admitted override must leave a durable divergence record"),
        ),
        (false, Some(_)) => report.push(
            "decision.divergence_unexpected",
            format!("{owner} only an admitted override may leave a divergence record"),
        ),
        (false, None) => {}
    }

    // A recorded override is honored only on a reviewed-override gate; it is
    // never a force-write past a regenerate-only block.
    if subject.override_review_ref.is_some()
        && decision.effective_edit_gate == EditPosture::RegenerateOnly
        && decision.attempt_outcome.admits_write()
    {
        report.push(
            "decision.no_force_write",
            format!(
                "{owner} a recorded override must not force a write past a regenerate-only block"
            ),
        );
    }

    // The canonical-source jump is present exactly when the source is linked.
    match (
        subject.boundary_state.canonical_source_linked(),
        &decision.canonical_source_jump,
    ) {
        (true, None) => report.push(
            "decision.jump_missing",
            format!("{owner} a linked canonical source must offer a jump action"),
        ),
        (false, Some(_)) => report.push(
            "decision.jump_unexpected",
            format!("{owner} a missing canonical source must not offer a jump action"),
        ),
        _ => {}
    }

    // Regeneration availability must follow from the boundary state.
    if decision.regeneration_availability != subject.boundary_state.regeneration_availability() {
        report.push(
            "decision.regen_availability",
            format!("{owner} regeneration availability disagrees with the boundary state"),
        );
    }

    validate_compare(report, owner, decision);

    if decision.copy_line != write_boundary_copy_line(decision, subject) {
        report.push(
            "decision.copy_line",
            format!("{owner} stamped copy line disagrees with the engine"),
        );
    }
}

fn validate_compare(report: &mut ValidationReport, owner: &str, decision: &WriteBoundaryDecision) {
    let compare = &decision.three_way_compare;
    let kinds: Vec<_> = compare.legs.iter().map(|leg| leg.kind).collect();
    if kinds != CompareLegKind::ALL.to_vec() {
        report.push(
            "compare.legs",
            format!("{owner} three-way compare must carry all three legs in canonical order"),
        );
    }
    for leg in &compare.legs {
        if leg.provenance_ref.trim().is_empty() {
            report.push(
                "compare.provenance",
                format!(
                    "{owner} compare leg {} must preserve a provenance ref even when unavailable",
                    leg.kind.as_str()
                ),
            );
        }
        match leg.availability {
            LegAvailability::Available => {
                if leg.unavailable_reason.is_some() {
                    report.push(
                        "compare.available_reason",
                        format!(
                            "{owner} an available compare leg {} must not carry an unavailable reason",
                            leg.kind.as_str()
                        ),
                    );
                }
            }
            LegAvailability::Unavailable => {
                if leg
                    .unavailable_reason
                    .as_ref()
                    .map(|r| r.trim().is_empty())
                    .unwrap_or(true)
                {
                    report.push(
                        "compare.unavailable_reason",
                        format!(
                            "{owner} an unavailable compare leg {} must name why it is unavailable",
                            leg.kind.as_str()
                        ),
                    );
                }
            }
        }
    }
    // The current-artifact leg is always available.
    if let Some(current) = compare
        .legs
        .iter()
        .find(|leg| leg.kind == CompareLegKind::CurrentArtifact)
    {
        if current.availability != LegAvailability::Available {
            report.push(
                "compare.current_available",
                format!("{owner} the current-artifact leg must always be available"),
            );
        }
    }
    if !compare.provenance_preserved {
        report.push(
            "compare.provenance_preserved",
            format!("{owner} the three-way compare must preserve provenance on every leg"),
        );
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &WriteBoundaryPacket) {
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
    for required in WriteBoundarySurface::ALL {
        if !surfaces.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind surface {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in write-boundary fixture against the frozen
/// contract.
pub fn validate_write_boundary_fixture(
    fixture: &WriteBoundaryFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != WRITE_BOUNDARY_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != WRITE_BOUNDARY_SCHEMA_VERSION {
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
    if fixture.expected_attempt_outcome != decision.attempt_outcome {
        report.push(
            "fixture.expected_attempt_outcome",
            format!(
                "fixture {} expected outcome disagrees with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_effective_edit_gate != decision.effective_edit_gate {
        report.push(
            "fixture.expected_effective_edit_gate",
            format!(
                "fixture {} expected edit gate disagrees with the decision",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_boundary_state != decision.boundary_state {
        report.push(
            "fixture.expected_boundary_state",
            format!(
                "fixture {} expected boundary state disagrees with the decision",
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
    if fixture.expected_leaves_divergence != decision.diverged_from_generator.is_some() {
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
