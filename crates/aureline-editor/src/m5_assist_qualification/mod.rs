//! Editor-assist qualification packet: the certification lane that binds the
//! editor-assist micro-surface truth sources into one per-family claim verdict
//! and auto-narrows a claimed editor family when its assist-surface proof is
//! stale or failing.
//!
//! The product claims a set of editor families ([`EditorSurfaceClass`]) support
//! a rich edit loop: completion, hints, hover, peek, snippet sessions, and
//! decorations. Each of those micro-surfaces is governed by its own frozen
//! truth lane in this crate (the editor-assist matrix, the assist descriptor
//! model, completion rows, signature/snippet, hover/peek, constrained-assist,
//! advanced editing, and the assist support packet). This lane does **not**
//! re-prove those contracts; it consumes them as proof sources and projects one
//! qualification packet that release automation, About/help, service-health,
//! compatibility, and support export all render instead of restating
//! assist-quality claims by hand.
//!
//! The packet pins three things:
//!
//! 1. **Proof dimensions** — the closed set of assist-surface claims a family
//!    is certified on ([`ProofDimension`]): assist-source honesty, precedence,
//!    completion, hint, hover, peek, constrained-file narrowing,
//!    IME / multi-cursor safety, and accessibility parity. Each dimension cites
//!    the upstream lane(s) that prove it and carries a freshness budget.
//! 2. **Proof freshness + failure state** — every dimension resolves to one
//!    [`ProofState`] (`fresh`, `stale`, `failing`, or `missing`) derived from
//!    the upstream lane's pass state and its capture stamp against the
//!    evaluation stamp. Stale or failing micro-surface evidence is the trigger
//!    that narrows the affected claim — exactly the silent-aging gap the
//!    guardrail forbids.
//! 3. **Per-family claim support** — for each claimed editor family, the packet
//!    evaluates only the dimensions that family actually claims (a surface that
//!    blocks peek is not penalized for stale peek proof) and resolves a
//!    [`ClaimSupportClass`] (`fully_supported`, `narrowed`, or `blocked`). A
//!    family stays `fully_supported` only when every dimension it claims is
//!    fresh; a critical-dimension failure blocks the claim, everything else
//!    narrows it, and the narrowing/blocking dimensions are named.
//!
//! [`project_assist_qualification`] is the release-automation entry point: it
//! takes the evaluation stamp and a list of [`ProofInput`]s and computes the
//! packet. [`assist_qualification_packet`] is the canonical binding that feeds
//! it the real in-code proof sources, so the checked-in fixture and the replay
//! gate freeze the certified state byte-for-byte. The record carries no file
//! contents, credential bodies, or raw provider payloads, so it is safe for
//! support export.

use serde::{Deserialize, Serialize};

use crate::m5_advanced_editing::{advanced_editing_model, M5_ADVANCED_EDITING_AS_OF};
use crate::m5_assist_descriptors::{assist_descriptor_model, M5_ASSIST_DESCRIPTORS_AS_OF};
use crate::m5_assist_support::assist_support_packet;
use crate::m5_completion_rows::{completion_row_model, M5_COMPLETION_ROWS_AS_OF};
use crate::m5_constrained_assist::{constrained_assist_model, M5_CONSTRAINED_ASSIST_AS_OF};
use crate::m5_editor_assist::{
    editor_assist_matrix, AssistChannelClass, AssistDegradeClass, EditorSurfaceClass,
    SurfaceAssistProfile, M5_EDITOR_ASSIST_AS_OF, M5_EDITOR_ASSIST_SCHEMA_REF,
};
use crate::m5_hover_peek::{hover_peek_model, M5_HOVER_PEEK_AS_OF};
use crate::m5_signature_snippet::{signature_snippet_model, M5_SIGNATURE_SNIPPET_AS_OF};

/// Schema version for the editor-assist qualification packet.
pub const M5_ASSIST_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the editor-assist qualification packet.
pub const M5_ASSIST_QUALIFICATION_SCHEMA_REF: &str =
    "schemas/editor/m5-assist-qualification.schema.json";

/// Stable record-kind tag for the editor-assist qualification packet.
pub const M5_ASSIST_QUALIFICATION_RECORD_KIND: &str = "m5_assist_qualification_packet";

/// Stable id for the canonical editor-assist qualification packet.
pub const M5_ASSIST_QUALIFICATION_PACKET_ID: &str = "m5-assist-qualification:packet:0001";

/// Evaluation stamp for the canonical packet. Held as a constant so the
/// canonical binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_ASSIST_QUALIFICATION_AS_OF: &str = "2026-06-22T00:00:00Z";

/// Default freshness budget, in days, before a passing proof is treated as
/// stale. Release automation may pass a tighter budget per dimension.
pub const DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS: i64 = 30;

// ---------------------------------------------------------------------------
// Proof dimensions.
// ---------------------------------------------------------------------------

/// The closed set of assist-surface claims an editor family is certified on.
///
/// Each dimension names one micro-surface claim and cites the upstream lane(s)
/// whose freshness and pass state decide whether the claim holds. The set is
/// the union of the certification requirements (assist-source honesty,
/// precedence, constrained-file narrowing, IME / multi-cursor safety,
/// hover/peek provenance, accessibility parity) and the explicit
/// completion / hint / hover / peek release-evidence rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofDimension {
    /// Deterministic, cached/lexical, snippet-only, and AI-backed assist sources
    /// stay distinct and labeled. A mislabeled source is a trust violation.
    AssistSourceHonesty,
    /// Editing truth (diagnostics, current frame, conflicts, review, search,
    /// selection) outranks convenience assist chrome everywhere.
    Precedence,
    /// Completion rows are honest about source, trust weight, and apply effect.
    Completion,
    /// Inlay-hint and code-lens descriptors are honest about source, placement,
    /// and AI provenance.
    Hint,
    /// Hover cards preserve symbol anchor, source/provider/freshness provenance,
    /// and raw-versus-rendered truth.
    Hover,
    /// Peek surfaces preserve provenance and mapping quality and disclose
    /// non-live state.
    Peek,
    /// Read-only, generated, protected, projection, partial-index, and
    /// large-file states narrow or block unsafe assist classes visibly.
    ConstrainedFileNarrowing,
    /// IME composition and multi-cursor editing stay coherent in the typing
    /// loop, or degrade explicitly to one disclosed primary caret.
    ImeMultiCursorSafety,
    /// Every offered assist surface stays keyboard-complete, non-color-only, and
    /// density / zoom / reduced-motion aware.
    AccessibilityParity,
}

impl ProofDimension {
    /// All proof dimensions, in packet order.
    pub const ALL: [Self; 9] = [
        Self::AssistSourceHonesty,
        Self::Precedence,
        Self::Completion,
        Self::Hint,
        Self::Hover,
        Self::Peek,
        Self::ConstrainedFileNarrowing,
        Self::ImeMultiCursorSafety,
        Self::AccessibilityParity,
    ];

    /// Returns the stable schema token for this dimension.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AssistSourceHonesty => "assist_source_honesty",
            Self::Precedence => "precedence",
            Self::Completion => "completion",
            Self::Hint => "hint",
            Self::Hover => "hover",
            Self::Peek => "peek",
            Self::ConstrainedFileNarrowing => "constrained_file_narrowing",
            Self::ImeMultiCursorSafety => "ime_multi_cursor_safety",
            Self::AccessibilityParity => "accessibility_parity",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AssistSourceHonesty => "Assist-source honesty",
            Self::Precedence => "Editing-truth precedence",
            Self::Completion => "Completion",
            Self::Hint => "Inlay hints / code lenses",
            Self::Hover => "Hover provenance",
            Self::Peek => "Peek provenance",
            Self::ConstrainedFileNarrowing => "Constrained-file narrowing",
            Self::ImeMultiCursorSafety => "IME / multi-cursor safety",
            Self::AccessibilityParity => "Accessibility parity",
        }
    }

    /// Whether failing or missing proof on this dimension blocks the claim
    /// rather than merely narrowing it.
    ///
    /// Only the two safety dimensions are critical: a mislabeled assist source
    /// (honesty) and convenience chrome outranking editing truth (precedence)
    /// are correctness violations the edit loop cannot ship around. Every other
    /// dimension degrades honestly — it narrows the claim and discloses the
    /// limit rather than blocking the family outright.
    pub const fn is_critical(self) -> bool {
        matches!(self, Self::AssistSourceHonesty | Self::Precedence)
    }

    /// The assist channels this dimension governs, or an empty slice when the
    /// dimension applies to every surface that offers any assist at all.
    ///
    /// A dimension applies to a family only when the family claims at least one
    /// governing channel with real fidelity, so a surface that blocks peek is
    /// not penalized when peek proof ages out.
    pub const fn governing_channels(self) -> &'static [AssistChannelClass] {
        match self {
            // Global dimensions: every surface draws decorations and is held to
            // honest sourcing, precedence, and accessibility parity.
            Self::AssistSourceHonesty | Self::Precedence | Self::AccessibilityParity => &[],
            Self::Completion => &[AssistChannelClass::Completion],
            Self::Hint => &[AssistChannelClass::CodeLens, AssistChannelClass::InlayHint],
            Self::Hover => &[AssistChannelClass::Hover],
            Self::Peek => &[AssistChannelClass::Peek],
            // Constrained-file narrowing is gated on surface constraint, not a
            // channel; the empty slice here is unused (see `applies_to`).
            Self::ConstrainedFileNarrowing => &[],
            Self::ImeMultiCursorSafety => &[
                AssistChannelClass::SnippetSession,
                AssistChannelClass::Completion,
            ],
        }
    }

    /// Returns true when this dimension's proof governs the given family.
    pub fn applies_to(self, profile: &SurfaceAssistProfile) -> bool {
        match self {
            // Constrained-file narrowing only matters where the family actually
            // constrains assist; unconstrained code/config files have nothing to
            // narrow, so the dimension does not apply.
            Self::ConstrainedFileNarrowing => profile.is_constrained,
            // Global dimensions apply to every surface.
            Self::AssistSourceHonesty | Self::Precedence | Self::AccessibilityParity => true,
            // Channel-governed dimensions apply when the family claims at least
            // one governing channel with real fidelity.
            _ => self
                .governing_channels()
                .iter()
                .any(|channel| match profile.cell(*channel) {
                    Some(cell) => channel_claims_assist(cell.degrade_state),
                    None => false,
                }),
        }
    }
}

/// Returns true when a degraded-state class still claims real assist fidelity,
/// so its proof freshness matters to the family's qualification.
///
/// A channel that is suppressed in large-file mode or blocked outright is not
/// promising the user anything, so aging proof for it does not narrow the claim.
fn channel_claims_assist(state: AssistDegradeClass) -> bool {
    matches!(
        state,
        AssistDegradeClass::FullFidelity
            | AssistDegradeClass::SourceLabeledFallback
            | AssistDegradeClass::ReadOnlyNoApply
            | AssistDegradeClass::PendingPartialIndex
    )
}

// ---------------------------------------------------------------------------
// Proof state + claim support.
// ---------------------------------------------------------------------------

/// The resolved freshness / failure state of one proof dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofState {
    /// Proof is present, passing, and captured within its freshness budget.
    Fresh,
    /// Proof is present and was passing, but captured outside its freshness
    /// budget — it has silently aged out.
    Stale,
    /// Proof is present but the upstream contract did not hold.
    Failing,
    /// No proof was supplied for this dimension.
    Missing,
}

impl ProofState {
    /// Returns the stable schema token for this proof state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Failing => "failing",
            Self::Missing => "missing",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::Stale => "Stale (aged out)",
            Self::Failing => "Failing",
            Self::Missing => "Missing",
        }
    }

    /// Whether the proof is fresh and passing (the only state that keeps a claim
    /// fully supported).
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Fresh)
    }
}

/// The certified support level of one editor family's assist claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimSupportClass {
    /// Every dimension the family claims is fresh and passing.
    FullySupported,
    /// At least one claimed dimension is stale or failing, but no critical
    /// safety dimension failed; the claim is degraded and discloses its limits.
    Narrowed,
    /// A critical safety dimension (assist-source honesty or precedence) failed
    /// or is missing; the family's assist claim is withdrawn.
    Blocked,
}

impl ClaimSupportClass {
    /// Returns the stable schema token for this support level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySupported => "fully_supported",
            Self::Narrowed => "narrowed",
            Self::Blocked => "blocked",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::FullySupported => "Fully supported",
            Self::Narrowed => "Narrowed",
            Self::Blocked => "Blocked",
        }
    }

    /// Severity rank; higher is worse. Used to fold per-dimension effects into a
    /// family verdict.
    const fn severity(self) -> u8 {
        match self {
            Self::FullySupported => 0,
            Self::Narrowed => 1,
            Self::Blocked => 2,
        }
    }

    /// Returns the worse of two support levels.
    fn worst(self, other: Self) -> Self {
        if other.severity() > self.severity() {
            other
        } else {
            self
        }
    }
}

/// The support effect a proof state has on a claim, given whether the dimension
/// is critical.
fn dimension_effect(state: ProofState, critical: bool) -> ClaimSupportClass {
    match state {
        ProofState::Fresh => ClaimSupportClass::FullySupported,
        ProofState::Stale => ClaimSupportClass::Narrowed,
        ProofState::Failing | ProofState::Missing => {
            if critical {
                ClaimSupportClass::Blocked
            } else {
                ClaimSupportClass::Narrowed
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Release-automation input.
// ---------------------------------------------------------------------------

/// One raw proof observation fed to [`project_assist_qualification`] by release
/// automation.
///
/// Release automation knows where each micro-surface proof was last captured
/// and whether it passed; this struct carries that verbatim so the projection
/// derives the freshness / failure state deterministically rather than the
/// caller pre-deciding the verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProofInput {
    /// Dimension this observation proves.
    pub dimension: ProofDimension,
    /// Primary upstream lane schema ref this proof is drawn from.
    pub proof_source_ref: String,
    /// All upstream lane refs that must hold for the proof to pass, including
    /// the primary.
    pub contributing_proof_refs: Vec<String>,
    /// Capture stamp of the proof, or `None` when no proof exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_as_of: Option<String>,
    /// Whether every contributing upstream contract held when captured.
    pub passing: bool,
    /// Freshness budget, in days, before the proof is treated as stale.
    pub freshness_budget_days: i64,
    /// Human-readable note about the proof.
    pub detail: String,
}

impl ProofInput {
    /// Resolves this input to a [`ProofState`] against the evaluation stamp.
    pub fn resolve_state(&self, evaluated_as_of: &str) -> ProofState {
        derive_proof_state(
            self.captured_as_of.as_deref(),
            self.passing,
            self.freshness_budget_days,
            evaluated_as_of,
        )
    }
}

// ---------------------------------------------------------------------------
// Projected records.
// ---------------------------------------------------------------------------

/// The resolved global proof for one dimension, shared across every family that
/// claims it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionProof {
    /// Dimension this proof covers.
    pub dimension: ProofDimension,
    /// Human-readable dimension label.
    pub label: String,
    /// Whether failing / missing proof on this dimension blocks rather than
    /// narrows a claim.
    pub critical: bool,
    /// Primary upstream lane schema ref.
    pub proof_source_ref: String,
    /// All upstream lane refs that contribute to this proof.
    pub contributing_proof_refs: Vec<String>,
    /// Capture stamp, when proof exists.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_as_of: Option<String>,
    /// Freshness budget, in days.
    pub freshness_budget_days: i64,
    /// Resolved freshness / failure state.
    pub state: ProofState,
    /// Human-readable note.
    pub detail: String,
}

/// One family's verdict on a single proof dimension.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionVerdict {
    /// Dimension this verdict covers.
    pub dimension: ProofDimension,
    /// Whether the family claims this dimension at all.
    pub applicable: bool,
    /// Resolved proof state for the dimension.
    pub state: ProofState,
    /// The support effect this dimension contributes to the family claim
    /// (`fully_supported` when not applicable or fresh).
    pub effect: ClaimSupportClass,
}

/// One claimed editor family's certified assist qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FamilyQualificationRow {
    /// Editor family this row certifies.
    pub surface: EditorSurfaceClass,
    /// Human-readable family label.
    pub label: String,
    /// Whether the family constrains assist relative to a code file.
    pub is_constrained: bool,
    /// Resolved support level for the family's assist claim.
    pub support: ClaimSupportClass,
    /// Per-dimension verdicts, one per [`ProofDimension`] in packet order.
    pub dimension_verdicts: Vec<DimensionVerdict>,
    /// Applicable dimensions that narrowed the claim, in dimension order.
    pub narrowed_by: Vec<ProofDimension>,
    /// Applicable dimensions that blocked the claim, in dimension order.
    pub blocked_by: Vec<ProofDimension>,
    /// Human-readable summary of the family verdict.
    pub summary: String,
}

impl FamilyQualificationRow {
    /// Returns true when every dimension the family claims is fresh.
    pub fn is_fully_supported(&self) -> bool {
        self.support == ClaimSupportClass::FullySupported
    }

    /// Returns the verdict for a given dimension, when present.
    pub fn verdict(&self, dimension: ProofDimension) -> Option<&DimensionVerdict> {
        self.dimension_verdicts
            .iter()
            .find(|verdict| verdict.dimension == dimension)
    }
}

/// Cross-family rollup of the qualification packet, for service-health and
/// About surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationRollup {
    /// Number of families fully supported.
    pub fully_supported: usize,
    /// Number of families narrowed.
    pub narrowed: usize,
    /// Number of families blocked.
    pub blocked: usize,
    /// Number of dimensions whose proof is stale.
    pub stale_dimensions: usize,
    /// Number of dimensions whose proof is failing.
    pub failing_dimensions: usize,
    /// Number of dimensions whose proof is missing.
    pub missing_dimensions: usize,
}

/// One frozen invariant the packet must satisfy, with the result of evaluating
/// it over the packet's own data.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// Human-readable statement of the invariant.
    pub statement: String,
    /// Whether the invariant holds on the built packet.
    pub holds: bool,
}

/// The editor-assist qualification packet: per-family claim verdicts derived
/// from the editor-assist micro-surface proof sources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistQualificationPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_assist_qualification_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable packet id.
    pub packet_id: String,
    /// Evaluation stamp the proof freshness is measured against.
    pub as_of: String,
    /// Resolved global proof per dimension, in dimension order.
    pub dimensions: Vec<DimensionProof>,
    /// Per-family qualification rows, one per [`EditorSurfaceClass`].
    pub families: Vec<FamilyQualificationRow>,
    /// Cross-family rollup.
    pub rollup: QualificationRollup,
    /// Frozen invariants and whether each holds on this packet.
    pub invariants: Vec<QualificationInvariant>,
    /// Whether the packet is metadata-safe for support export.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl AssistQualificationPacket {
    /// Returns true when every frozen invariant holds on this packet.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|invariant| invariant.holds)
    }

    /// Returns true when the packet is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == M5_ASSIST_QUALIFICATION_SCHEMA_REF
            && self.record_kind == M5_ASSIST_QUALIFICATION_RECORD_KIND
    }

    /// Returns the qualification row for a family, when present.
    pub fn family(&self, surface: EditorSurfaceClass) -> Option<&FamilyQualificationRow> {
        self.families.iter().find(|row| row.surface == surface)
    }

    /// Returns the resolved proof for a dimension, when present.
    pub fn dimension(&self, dimension: ProofDimension) -> Option<&DimensionProof> {
        self.dimensions
            .iter()
            .find(|proof| proof.dimension == dimension)
    }

    /// Returns true when the family's assist claim is fully supported.
    pub fn is_family_fully_supported(&self, surface: EditorSurfaceClass) -> bool {
        self.family(surface)
            .is_some_and(FamilyQualificationRow::is_fully_supported)
    }
}

// ---------------------------------------------------------------------------
// Projection (release-automation entry point).
// ---------------------------------------------------------------------------

/// Projects the editor-assist qualification packet from a set of proof
/// observations.
///
/// This is the release-automation entry point. It resolves each
/// [`ProofDimension`] to a [`ProofState`] against `evaluated_as_of`, then for
/// every claimed editor family evaluates only the dimensions that family
/// actually claims and folds them into one [`ClaimSupportClass`]. A family stays
/// fully supported only when every dimension it claims is fresh; a stale or
/// failing dimension narrows the claim and a critical-dimension failure blocks
/// it, so micro-surface proof that has aged out can never leave a family green.
///
/// Dimensions with no supplied input resolve to [`ProofState::Missing`].
pub fn project_assist_qualification(
    evaluated_as_of: impl Into<String>,
    proofs: &[ProofInput],
) -> AssistQualificationPacket {
    let evaluated_as_of = evaluated_as_of.into();
    let matrix = editor_assist_matrix();

    let dimensions: Vec<DimensionProof> = ProofDimension::ALL
        .iter()
        .map(|dimension| project_dimension(*dimension, proofs, &evaluated_as_of))
        .collect();

    let families: Vec<FamilyQualificationRow> = matrix
        .surface_profiles
        .iter()
        .map(|profile| project_family(profile, &dimensions))
        .collect();

    let rollup = build_rollup(&dimensions, &families);
    let invariants = build_invariants(&dimensions, &families);
    let summary = build_summary(&rollup);

    AssistQualificationPacket {
        record_kind: M5_ASSIST_QUALIFICATION_RECORD_KIND.to_owned(),
        m5_assist_qualification_schema_version: M5_ASSIST_QUALIFICATION_SCHEMA_VERSION,
        schema_ref: M5_ASSIST_QUALIFICATION_SCHEMA_REF.to_owned(),
        packet_id: M5_ASSIST_QUALIFICATION_PACKET_ID.to_owned(),
        as_of: evaluated_as_of,
        dimensions,
        families,
        rollup,
        invariants,
        raw_payload_excluded: true,
        summary,
    }
}

fn project_dimension(
    dimension: ProofDimension,
    proofs: &[ProofInput],
    evaluated_as_of: &str,
) -> DimensionProof {
    match proofs.iter().find(|input| input.dimension == dimension) {
        Some(input) => DimensionProof {
            dimension,
            label: dimension.label().to_owned(),
            critical: dimension.is_critical(),
            proof_source_ref: input.proof_source_ref.clone(),
            contributing_proof_refs: input.contributing_proof_refs.clone(),
            captured_as_of: input.captured_as_of.clone(),
            freshness_budget_days: input.freshness_budget_days,
            state: input.resolve_state(evaluated_as_of),
            detail: input.detail.clone(),
        },
        None => DimensionProof {
            dimension,
            label: dimension.label().to_owned(),
            critical: dimension.is_critical(),
            proof_source_ref: String::new(),
            contributing_proof_refs: Vec::new(),
            captured_as_of: None,
            freshness_budget_days: 0,
            state: ProofState::Missing,
            detail: "no proof supplied for this dimension".to_owned(),
        },
    }
}

fn project_family(
    profile: &SurfaceAssistProfile,
    dimensions: &[DimensionProof],
) -> FamilyQualificationRow {
    let mut verdicts = Vec::with_capacity(dimensions.len());
    let mut support = ClaimSupportClass::FullySupported;
    let mut narrowed_by = Vec::new();
    let mut blocked_by = Vec::new();

    for proof in dimensions {
        let applicable = proof.dimension.applies_to(profile);
        let effect = if applicable {
            dimension_effect(proof.state, proof.critical)
        } else {
            ClaimSupportClass::FullySupported
        };

        if applicable {
            match effect {
                ClaimSupportClass::Narrowed => narrowed_by.push(proof.dimension),
                ClaimSupportClass::Blocked => blocked_by.push(proof.dimension),
                ClaimSupportClass::FullySupported => {}
            }
            support = support.worst(effect);
        }

        verdicts.push(DimensionVerdict {
            dimension: proof.dimension,
            applicable,
            state: proof.state,
            effect,
        });
    }

    let summary = build_family_summary(profile.surface, support, &narrowed_by, &blocked_by);

    FamilyQualificationRow {
        surface: profile.surface,
        label: profile.label.clone(),
        is_constrained: profile.is_constrained,
        support,
        dimension_verdicts: verdicts,
        narrowed_by,
        blocked_by,
        summary,
    }
}

fn build_rollup(
    dimensions: &[DimensionProof],
    families: &[FamilyQualificationRow],
) -> QualificationRollup {
    let count_state = |state: ProofState| dimensions.iter().filter(|d| d.state == state).count();
    QualificationRollup {
        fully_supported: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::FullySupported)
            .count(),
        narrowed: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::Narrowed)
            .count(),
        blocked: families
            .iter()
            .filter(|f| f.support == ClaimSupportClass::Blocked)
            .count(),
        stale_dimensions: count_state(ProofState::Stale),
        failing_dimensions: count_state(ProofState::Failing),
        missing_dimensions: count_state(ProofState::Missing),
    }
}

fn build_invariants(
    dimensions: &[DimensionProof],
    families: &[FamilyQualificationRow],
) -> Vec<QualificationInvariant> {
    let dimension_set_complete = ProofDimension::ALL
        .iter()
        .all(|dimension| dimensions.iter().any(|proof| proof.dimension == *dimension));

    let every_family_present = EditorSurfaceClass::ALL
        .iter()
        .all(|surface| families.iter().any(|row| row.surface == *surface));

    // No family stays fully supported while any dimension it claims is not
    // fresh — the core guardrail against silent aging.
    let no_green_with_nonfresh_proof = families.iter().all(|row| {
        row.support != ClaimSupportClass::FullySupported
            || row
                .dimension_verdicts
                .iter()
                .all(|verdict| !verdict.applicable || verdict.state.is_ok())
    });

    // Every non-green family names the dimension(s) responsible — no unattributed
    // downgrade.
    let every_downgrade_is_named = families.iter().all(|row| match row.support {
        ClaimSupportClass::FullySupported => {
            row.narrowed_by.is_empty() && row.blocked_by.is_empty()
        }
        ClaimSupportClass::Narrowed => !row.narrowed_by.is_empty(),
        ClaimSupportClass::Blocked => !row.blocked_by.is_empty(),
    });

    // A failing or missing critical dimension blocks every family that claims it.
    let critical_failure_blocks = families.iter().all(|row| {
        row.dimension_verdicts.iter().all(|verdict| {
            !(verdict.applicable
                && verdict.dimension.is_critical()
                && matches!(verdict.state, ProofState::Failing | ProofState::Missing))
                || row.support == ClaimSupportClass::Blocked
        })
    });

    // The explicit release-evidence dimensions are all present.
    let acceptance_dimensions_present = [
        ProofDimension::Completion,
        ProofDimension::Hint,
        ProofDimension::Hover,
        ProofDimension::Peek,
        ProofDimension::ConstrainedFileNarrowing,
        ProofDimension::ImeMultiCursorSafety,
        ProofDimension::AccessibilityParity,
    ]
    .iter()
    .all(|dimension| dimensions.iter().any(|proof| proof.dimension == *dimension));

    vec![
        QualificationInvariant {
            invariant_id: "dimension_set_complete".to_owned(),
            statement: "Every proof dimension resolves to exactly one global proof.".to_owned(),
            holds: dimension_set_complete,
        },
        QualificationInvariant {
            invariant_id: "every_claimed_family_present".to_owned(),
            statement: "Every claimed editor family has a qualification row.".to_owned(),
            holds: every_family_present,
        },
        QualificationInvariant {
            invariant_id: "no_fully_supported_family_with_nonfresh_proof".to_owned(),
            statement:
                "A family stays fully supported only when every dimension it claims is fresh."
                    .to_owned(),
            holds: no_green_with_nonfresh_proof,
        },
        QualificationInvariant {
            invariant_id: "every_downgrade_is_named".to_owned(),
            statement: "Every narrowed or blocked family names the responsible dimension(s)."
                .to_owned(),
            holds: every_downgrade_is_named,
        },
        QualificationInvariant {
            invariant_id: "critical_failure_blocks_claim".to_owned(),
            statement:
                "A failing or missing critical dimension blocks every family that claims it."
                    .to_owned(),
            holds: critical_failure_blocks,
        },
        QualificationInvariant {
            invariant_id: "release_evidence_dimensions_present".to_owned(),
            statement: "Completion, hint, hover, peek, constrained-file narrowing, IME / \
                        multi-cursor, and accessibility-parity rows are all present."
                .to_owned(),
            holds: acceptance_dimensions_present,
        },
    ]
}

fn build_family_summary(
    surface: EditorSurfaceClass,
    support: ClaimSupportClass,
    narrowed_by: &[ProofDimension],
    blocked_by: &[ProofDimension],
) -> String {
    match support {
        ClaimSupportClass::FullySupported => format!(
            "{label}: assist fully supported; every claimed dimension is fresh.",
            label = surface.label(),
        ),
        ClaimSupportClass::Narrowed => format!(
            "{label}: assist narrowed by {reasons}.",
            label = surface.label(),
            reasons = join_dimensions(narrowed_by),
        ),
        ClaimSupportClass::Blocked => format!(
            "{label}: assist blocked by {reasons}.",
            label = surface.label(),
            reasons = join_dimensions(blocked_by),
        ),
    }
}

fn build_summary(rollup: &QualificationRollup) -> String {
    format!(
        "Editor-assist qualification: {full} fully supported, {narrowed} narrowed, {blocked} \
         blocked across {total} families ({stale} stale, {failing} failing, {missing} missing \
         dimension proof(s)).",
        full = rollup.fully_supported,
        narrowed = rollup.narrowed,
        blocked = rollup.blocked,
        total = rollup.fully_supported + rollup.narrowed + rollup.blocked,
        stale = rollup.stale_dimensions,
        failing = rollup.failing_dimensions,
        missing = rollup.missing_dimensions,
    )
}

fn join_dimensions(dimensions: &[ProofDimension]) -> String {
    if dimensions.is_empty() {
        return "(none)".to_owned();
    }
    dimensions
        .iter()
        .map(|dimension| dimension.as_str())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// Canonical binding to the in-code proof sources.
// ---------------------------------------------------------------------------

/// Builds the canonical editor-assist qualification packet by binding the real
/// in-code proof sources.
///
/// Each dimension's pass state is read from its upstream lane's
/// `all_invariants_hold` and its capture stamp from the lane's `AS_OF`
/// constant, then [`project_assist_qualification`] folds them into the
/// per-family verdicts. The checked-in fixture and the replay gate freeze the
/// result so the certified state cannot drift from the published artifact.
pub fn assist_qualification_packet() -> AssistQualificationPacket {
    project_assist_qualification(M5_ASSIST_QUALIFICATION_AS_OF, &canonical_proof_inputs())
}

/// The canonical proof inputs, read from the in-code micro-surface lanes.
fn canonical_proof_inputs() -> Vec<ProofInput> {
    let descriptors_ok = assist_descriptor_model().all_invariants_hold();
    let matrix_ok = editor_assist_matrix().all_invariants_hold();
    let completion_ok = completion_row_model().all_invariants_hold();
    let hover_peek_ok = hover_peek_model().all_invariants_hold();
    let constrained_ok = constrained_assist_model().all_invariants_hold();
    let signature_ok = signature_snippet_model().all_invariants_hold();
    let advanced_ok = advanced_editing_model().all_invariants_hold();
    let support_ok = assist_support_packet().all_invariants_hold();

    let descriptors = "schemas/editor/m5-assist-descriptors.schema.json".to_owned();
    let matrix = M5_EDITOR_ASSIST_SCHEMA_REF.to_owned();
    let completion = "schemas/editor/m5-completion-rows.schema.json".to_owned();
    let hover_peek = "schemas/editor/m5-hover-peek.schema.json".to_owned();
    let constrained = "schemas/editor/m5-constrained-assist.schema.json".to_owned();
    let signature = "schemas/editor/m5-signature-snippet.schema.json".to_owned();
    let advanced = "schemas/editor/m5-advanced-editing.schema.json".to_owned();
    let support = "schemas/editor/m5-assist-support.schema.json".to_owned();

    vec![
        ProofInput {
            dimension: ProofDimension::AssistSourceHonesty,
            proof_source_ref: descriptors.clone(),
            contributing_proof_refs: vec![descriptors.clone(), matrix.clone()],
            captured_as_of: Some(M5_ASSIST_DESCRIPTORS_AS_OF.to_owned()),
            passing: descriptors_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Assist source classes stay distinct and labeled across descriptors and the \
                     editor-assist matrix."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::Precedence,
            proof_source_ref: matrix.clone(),
            contributing_proof_refs: vec![matrix.clone()],
            captured_as_of: Some(M5_EDITOR_ASSIST_AS_OF.to_owned()),
            passing: matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Editing-truth layers outrank every convenience layer in the editor-assist \
                     precedence ladder."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::Completion,
            proof_source_ref: completion.clone(),
            contributing_proof_refs: vec![completion.clone(), support.clone()],
            captured_as_of: Some(M5_COMPLETION_ROWS_AS_OF.to_owned()),
            passing: completion_ok && support_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Completion rows are honest about source, trust weight, and apply effect."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::Hint,
            proof_source_ref: descriptors.clone(),
            contributing_proof_refs: vec![descriptors.clone(), matrix.clone(), support.clone()],
            captured_as_of: Some(M5_ASSIST_DESCRIPTORS_AS_OF.to_owned()),
            passing: descriptors_ok && matrix_ok && support_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Inlay-hint and code-lens descriptors disclose source, placement, and AI \
                     provenance."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::Hover,
            proof_source_ref: hover_peek.clone(),
            contributing_proof_refs: vec![hover_peek.clone(), support.clone()],
            captured_as_of: Some(M5_HOVER_PEEK_AS_OF.to_owned()),
            passing: hover_peek_ok && support_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Hover cards preserve anchor, provenance, freshness, and raw-versus-rendered \
                     truth."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::Peek,
            proof_source_ref: hover_peek.clone(),
            contributing_proof_refs: vec![hover_peek.clone(), support.clone()],
            captured_as_of: Some(M5_HOVER_PEEK_AS_OF.to_owned()),
            passing: hover_peek_ok && support_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Peek surfaces preserve provenance and mapping quality and disclose non-live \
                     state."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::ConstrainedFileNarrowing,
            proof_source_ref: constrained.clone(),
            contributing_proof_refs: vec![constrained.clone(), matrix.clone()],
            captured_as_of: Some(M5_CONSTRAINED_ASSIST_AS_OF.to_owned()),
            passing: constrained_ok && matrix_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Constrained surfaces narrow or block unsafe assist classes visibly with a \
                     named reason."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::ImeMultiCursorSafety,
            proof_source_ref: signature.clone(),
            contributing_proof_refs: vec![signature.clone(), advanced.clone()],
            captured_as_of: Some(M5_SIGNATURE_SNIPPET_AS_OF.to_owned()),
            passing: signature_ok && advanced_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "IME composition and multi-cursor editing stay coherent or degrade to one \
                     disclosed primary caret."
                .to_owned(),
        },
        ProofInput {
            dimension: ProofDimension::AccessibilityParity,
            proof_source_ref: advanced.clone(),
            contributing_proof_refs: vec![advanced, descriptors],
            captured_as_of: Some(M5_ADVANCED_EDITING_AS_OF.to_owned()),
            passing: advanced_ok && descriptors_ok,
            freshness_budget_days: DEFAULT_PROOF_FRESHNESS_BUDGET_DAYS,
            detail: "Offered assist stays keyboard-complete, non-color-only, and density / zoom / \
                     reduced-motion aware."
                .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Freshness derivation.
// ---------------------------------------------------------------------------

/// Derives the proof state from a capture stamp, pass state, freshness budget,
/// and evaluation stamp.
///
/// Missing capture means no proof; a non-passing proof is failing; otherwise the
/// proof is stale when its age exceeds the budget. A capture stamp that cannot
/// be parsed is treated as stale: the evidence exists but its age cannot be
/// trusted, so the conservative outcome is to narrow.
fn derive_proof_state(
    captured_as_of: Option<&str>,
    passing: bool,
    budget_days: i64,
    evaluated_as_of: &str,
) -> ProofState {
    let Some(captured) = captured_as_of else {
        return ProofState::Missing;
    };
    if !passing {
        return ProofState::Failing;
    }
    match (
        parse_civil_days(captured),
        parse_civil_days(evaluated_as_of),
    ) {
        (Some(captured_days), Some(evaluated_days)) => {
            if evaluated_days - captured_days > budget_days.max(0) {
                ProofState::Stale
            } else {
                ProofState::Fresh
            }
        }
        _ => ProofState::Stale,
    }
}

/// Parses the `YYYY-MM-DD` date prefix of an ISO 8601 stamp into a day count.
fn parse_civil_days(stamp: &str) -> Option<i64> {
    let date = stamp.get(..10)?;
    let mut parts = date.split('-');
    let year: i64 = parts.next()?.parse().ok()?;
    let month: i64 = parts.next()?.parse().ok()?;
    let day: i64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() || !(1..=12).contains(&month) || !(1..=31).contains(&day) {
        return None;
    }
    Some(days_from_civil(year, month, day))
}

/// Returns the number of days since the Unix epoch for a proleptic Gregorian
/// date, using Howard Hinnant's `days_from_civil` algorithm.
fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = if month <= 2 { year - 1 } else { year };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let year_of_era = year - era * 400;
    let day_of_year = (153 * (if month > 2 { month - 3 } else { month + 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146097 + day_of_era - 719468
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the export-safe human-readable lines for a qualification packet.
///
/// This is the shared projection consumed by the editor's About/help surface,
/// the service-health and compatibility surfaces, the headless CLI emitter, and
/// support export, so none of them clone the certified state from each other.
pub fn assist_qualification_lines(packet: &AssistQualificationPacket) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Editor-assist qualification — {} (as of {})",
        packet.packet_id, packet.as_of
    ));
    lines.push(format!(
        "rollup: fully_supported={} narrowed={} blocked={} | stale={} failing={} missing={}",
        packet.rollup.fully_supported,
        packet.rollup.narrowed,
        packet.rollup.blocked,
        packet.rollup.stale_dimensions,
        packet.rollup.failing_dimensions,
        packet.rollup.missing_dimensions,
    ));

    lines.push("Proof dimensions:".to_owned());
    for proof in &packet.dimensions {
        lines.push(format!(
            "  {dim} [{state}] critical={critical} budget={budget}d source={source} captured={captured}",
            dim = proof.dimension.as_str(),
            state = proof.state.as_str(),
            critical = proof.critical,
            budget = proof.freshness_budget_days,
            source = if proof.proof_source_ref.is_empty() {
                "(none)"
            } else {
                proof.proof_source_ref.as_str()
            },
            captured = proof.captured_as_of.as_deref().unwrap_or("(none)"),
        ));
    }

    lines.push("Families:".to_owned());
    for family in &packet.families {
        lines.push(format!(
            "  {surface} [{support}] constrained={constrained}{detail}",
            surface = family.surface.as_str(),
            support = family.support.as_str(),
            constrained = family.is_constrained,
            detail = family_reason_suffix(family),
        ));
    }

    lines.push(packet.summary.clone());
    lines
}

fn family_reason_suffix(family: &FamilyQualificationRow) -> String {
    match family.support {
        ClaimSupportClass::FullySupported => String::new(),
        ClaimSupportClass::Narrowed => {
            format!(" narrowed_by={}", join_dimensions(&family.narrowed_by))
        }
        ClaimSupportClass::Blocked => {
            format!(" blocked_by={}", join_dimensions(&family.blocked_by))
        }
    }
}

#[cfg(test)]
mod tests;
