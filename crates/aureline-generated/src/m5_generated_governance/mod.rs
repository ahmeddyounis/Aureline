//! Governance-grade certification of generated-artifact truth on claimed
//! M5 scaffolded-project / notebook-output / preview-derivative /
//! request-artifact / framework-codegen / AI-assisted-edit /
//! support-packet classes.
//!
//! M5 scaffolds, notebooks, preview/runtime derivatives, API/request
//! artifacts, framework codegen, AI-assisted edits, and exportable
//! support packets all need one typed *generated-artifact contract*: the
//! canonical source the derived bytes point back to, the generator that
//! produced them, the provenance class they carry, the writable-boundary
//! policy that decides whether a direct edit is safe, the regeneration
//! route that rebuilds them, the drift state that says whether the bytes
//! still match their source, and the reversible-checkpoint lineage that
//! captured the change across desktop, search, review, AI, save, support,
//! and export surfaces. This module freezes one canonical matrix over
//! those objects so later M5 surfaces stop inferring generated-artifact
//! truth from how a file happens to look on disk.
//!
//! The module models one [`ArtifactRow`] per claimed [`ArtifactClass`],
//! each carrying the seven required [`ProvenanceDimension`]s and the
//! evidence backing each. A single [`certify_artifact_outcome`] engine
//! folds the per-dimension evidence into one [`RowVerdict`] (`certified` /
//! `narrowed` / `withheld`), an effective [`ClaimMaturity`] floor, **and**
//! a narrowed [`EditPosture`], so a `stable` or `beta` claim — and any
//! `direct_edit_allowed` promise — can never outrun the provenance
//! evidence that backs it. The same engine drives the failure / recovery
//! [`ArtifactDrill`]s and the [`M5GeneratedGovernanceFixture`] corpus, so
//! the certification, the drills, and the fixtures cannot disagree about
//! when a claim must narrow.
//!
//! Four guardrails are frozen here:
//!
//! - **No file-on-disk authority.** A generated artifact is certified at
//!   its claimed maturity only when every required dimension is `current`.
//!   Stale or partial evidence narrows the claim; missing evidence
//!   withholds it. A profile absent from the packet is uncertified, not
//!   implicitly authoritative.
//! - **Derived bytes are not the source.** When the canonical-source or
//!   writable-boundary evidence goes partial or stale, the engine narrows
//!   the writable-boundary posture: a `direct_edit_allowed` claim drops to
//!   a reviewed override or a regenerate-only boundary instead of letting
//!   a direct edit silently cross a canonical-source boundary.
//! - **One narrowing engine.** [`certify_artifact_outcome`] is the single
//!   source of truth for downgrade, shared by the rows, the drills, the
//!   fixtures, the [`EvidenceFreshnessRule`]s, and the
//!   [`EditBoundaryRule`]s. Release, support, docs, and help all read the
//!   resulting verdict and posture rather than re-deriving staleness.
//! - **No silent widening.** The certification only ever narrows; it never
//!   promotes an artifact above its claimed maturity or writable-boundary
//!   posture, so a derived artifact never becomes more editable than its
//!   evidence proves.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/m5-generated-governance.schema.json`](../../../../schemas/generated/m5-generated-governance.schema.json)
//! - [`/docs/generated/m5-generated-governance.md`](../../../../docs/generated/m5-generated-governance.md)
//! - [`/artifacts/generated/m5-generated-proof-packet.json`](../../../../artifacts/generated/m5-generated-proof-packet.json)
//! - [`/artifacts/generated/m5-generated-governance.md`](../../../../artifacts/generated/m5-generated-governance.md)
//! - [`/fixtures/generated/m5-generated-governance/`](../../../../fixtures/generated/m5-generated-governance/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const M5_GENERATED_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_GENERATED_GOVERNANCE_PACKET_RECORD_KIND: &str =
    "m5_generated_governance_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_GENERATED_GOVERNANCE_FIXTURE_RECORD_KIND: &str =
    "m5_generated_governance_fixture_record";

/// Repo-relative schema ref.
pub const M5_GENERATED_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/generated/m5-generated-governance.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_GENERATED_GOVERNANCE_DOC_REF: &str = "docs/generated/m5-generated-governance.md";

/// Repo-relative machine-readable proof packet.
pub const M5_GENERATED_GOVERNANCE_PACKET_REF: &str =
    "artifacts/generated/m5-generated-proof-packet.json";

/// Repo-relative reviewer certification summary.
pub const M5_GENERATED_GOVERNANCE_REPORT_REF: &str =
    "artifacts/generated/m5-generated-governance.md";

/// Repo-relative fixture directory.
pub const M5_GENERATED_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/generated/m5-generated-governance";

/// Repo-relative fixture manifest.
pub const M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/m5-generated-governance/manifest.yaml";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// A claimed M5 generated-artifact class under certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// A project scaffolded from a template or starter.
    ScaffoldedProject,
    /// A notebook cell output captured from a kernel run.
    NotebookOutput,
    /// A preview or runtime derivative built from source.
    PreviewDerivative,
    /// An API/request artifact captured from a request run.
    RequestArtifact,
    /// Code emitted by a framework code generator.
    FrameworkCodegen,
    /// An edit produced by an AI-assisted composer.
    AiAssistedEdit,
    /// An exportable support packet projected for sharing.
    SupportPacket,
}

impl ArtifactClass {
    /// Every claimed class in canonical order.
    pub const ALL: [Self; 7] = [
        Self::ScaffoldedProject,
        Self::NotebookOutput,
        Self::PreviewDerivative,
        Self::RequestArtifact,
        Self::FrameworkCodegen,
        Self::AiAssistedEdit,
        Self::SupportPacket,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScaffoldedProject => "scaffolded_project",
            Self::NotebookOutput => "notebook_output",
            Self::PreviewDerivative => "preview_derivative",
            Self::RequestArtifact => "request_artifact",
            Self::FrameworkCodegen => "framework_codegen",
            Self::AiAssistedEdit => "ai_assisted_edit",
            Self::SupportPacket => "support_packet",
        }
    }
}

/// One generated-artifact dimension a claimed class must prove. The seven
/// dimensions are the exit-gate anchor: a surface may not present a
/// generated artifact as authoritative unless all seven are canonical and
/// testable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvenanceDimension {
    /// The artifact declares the canonical source it derives from, so a
    /// derived file is never mistaken for its own source of truth.
    CanonicalSource,
    /// The artifact declares the generator identity that produced it.
    GeneratorIdentity,
    /// The artifact declares a typed provenance class instead of leaving
    /// its authority implicit.
    ProvenanceClass,
    /// The artifact declares the writable-boundary policy that decides
    /// whether a direct edit is allowed, reviewed, or blocked.
    WritableBoundary,
    /// The artifact declares the regeneration route that rebuilds it from
    /// its canonical source.
    RegenerationRoute,
    /// The artifact declares its drift state — whether the derived bytes
    /// still match their canonical source.
    DriftState,
    /// The artifact declares the reversible-checkpoint lineage that
    /// captured the change, including what was captured, omitted, or
    /// rederived.
    CheckpointLineage,
}

impl ProvenanceDimension {
    /// Every required dimension in canonical order.
    pub const ALL: [Self; 7] = [
        Self::CanonicalSource,
        Self::GeneratorIdentity,
        Self::ProvenanceClass,
        Self::WritableBoundary,
        Self::RegenerationRoute,
        Self::DriftState,
        Self::CheckpointLineage,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSource => "canonical_source",
            Self::GeneratorIdentity => "generator_identity",
            Self::ProvenanceClass => "provenance_class",
            Self::WritableBoundary => "writable_boundary",
            Self::RegenerationRoute => "regeneration_route",
            Self::DriftState => "drift_state",
            Self::CheckpointLineage => "checkpoint_lineage",
        }
    }

    /// Whether degraded evidence on this dimension narrows the
    /// writable-boundary posture. A direct edit is only safe when the
    /// artifact's canonical-source linkage
    /// ([`ProvenanceDimension::CanonicalSource`]) and its boundary policy
    /// ([`ProvenanceDimension::WritableBoundary`]) are current, so those
    /// two dimensions — and only those — govern the edit posture.
    pub const fn governs_edit_posture(self) -> bool {
        matches!(self, Self::CanonicalSource | Self::WritableBoundary)
    }
}

/// The maturity a generated-artifact claim can hold. Declaration order is
/// the narrowing order: [`ClaimMaturity::Stable`] is the strongest claim
/// and [`ClaimMaturity::Withdrawn`] the weakest, so narrowing always moves
/// toward a later variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimMaturity {
    /// Every required dimension is current; the claim holds in full.
    Stable,
    /// One or more dimensions are partial; the claim narrows.
    Beta,
    /// Evidence is stale enough that only a preview claim holds.
    Preview,
    /// A required dimension cannot be proven; the claim is withdrawn.
    Withdrawn,
}

impl ClaimMaturity {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Narrowing severity. Higher is a narrower, more honest claim; the
    /// engine always takes the highest severity among the claimed maturity
    /// and every triggered floor.
    pub const fn severity(self) -> u8 {
        match self {
            Self::Stable => 0,
            Self::Beta => 1,
            Self::Preview => 2,
            Self::Withdrawn => 3,
        }
    }
}

/// How a generated artifact may be written. Declaration order is the
/// narrowing order: [`EditPosture::DirectEditAllowed`] is the strongest
/// claim and [`EditPosture::RegenerateOnly`] the most conservative, so
/// narrowing always moves toward a stricter boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditPosture {
    /// The artifact is its own canonical source; direct edits are allowed.
    DirectEditAllowed,
    /// Direct edits cross a canonical-source boundary and must escalate
    /// through a visible reviewed override.
    ReviewedOverrideRequired,
    /// Direct edits are blocked; the artifact must be regenerated from its
    /// canonical source.
    RegenerateOnly,
}

impl EditPosture {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectEditAllowed => "direct_edit_allowed",
            Self::ReviewedOverrideRequired => "reviewed_override_required",
            Self::RegenerateOnly => "regenerate_only",
        }
    }

    /// Narrowing severity. Higher is a stricter, more conservative
    /// boundary; the engine always takes the highest severity among the
    /// claimed posture and every triggered floor.
    pub const fn severity(self) -> u8 {
        match self {
            Self::DirectEditAllowed => 0,
            Self::ReviewedOverrideRequired => 1,
            Self::RegenerateOnly => 2,
        }
    }
}

/// The state of the evidence backing one dimension on one class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceState {
    /// Evidence is present, complete, and within its freshness window.
    Current,
    /// Evidence covers only part of the claimed scope.
    Partial,
    /// Evidence exists but is past its freshness window.
    Stale,
    /// No evidence backs this dimension.
    Missing,
    /// The dimension does not apply to this class.
    NotApplicable,
}

impl EvidenceState {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// The maturity floor this evidence state forces on a claim, if any.
    ///
    /// This is the heart of the maturity-narrowing engine: current and
    /// not-applicable evidence impose no floor; partial evidence caps the
    /// claim at beta; stale evidence caps it at preview; missing evidence
    /// withdraws the claim.
    pub const fn qualification_floor(self) -> Option<ClaimMaturity> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Partial => Some(ClaimMaturity::Beta),
            Self::Stale => Some(ClaimMaturity::Preview),
            Self::Missing => Some(ClaimMaturity::Withdrawn),
        }
    }

    /// The writable-boundary posture floor this evidence state forces, if
    /// any, when it lands on an edit-posture-governing dimension.
    ///
    /// Partial canonical-source or writable-boundary evidence caps the
    /// posture at a reviewed override; stale or missing evidence forces a
    /// regenerate-only boundary, because a stale boundary can no longer
    /// prove a direct edit will survive the next regeneration.
    pub const fn edit_posture_floor(self) -> Option<EditPosture> {
        match self {
            Self::Current | Self::NotApplicable => None,
            Self::Partial => Some(EditPosture::ReviewedOverrideRequired),
            Self::Stale | Self::Missing => Some(EditPosture::RegenerateOnly),
        }
    }

    /// Returns true when the state names stale or missing evidence, the two
    /// states the guardrail treats as a freshness defect.
    pub const fn is_stale_or_missing(self) -> bool {
        matches!(self, Self::Stale | Self::Missing)
    }
}

/// The verdict the certification engine reaches for one artifact row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowVerdict {
    /// Every required dimension is current; the claim holds at its claimed
    /// maturity.
    Certified,
    /// The claim narrowed below its claimed maturity but still holds.
    Narrowed,
    /// A required dimension cannot be proven; the claim is withheld.
    Withheld,
}

impl RowVerdict {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Narrowed => "narrowed",
            Self::Withheld => "withheld",
        }
    }
}

/// How authoritative a generated artifact's bytes are relative to its
/// canonical source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    /// After generation the bytes become the canonical source the user
    /// owns and edits directly.
    CanonicalAuthoritative,
    /// Derived bytes with reviewed editable regions; direct edits escalate
    /// through a visible override.
    DerivedEditable,
    /// Purely derived bytes that are never hand-edited; they are
    /// regenerated from their canonical source.
    DerivedReadonly,
}

impl AuthorityClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalAuthoritative => "canonical_authoritative",
            Self::DerivedEditable => "derived_editable",
            Self::DerivedReadonly => "derived_readonly",
        }
    }
}

/// A publication channel that ingests the governance packet. The packet as
/// a whole must bind all four so release, support, docs, and help tell one
/// consistent generated-artifact story.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationChannel {
    /// Release / shiproom promotion surfaces.
    ReleaseShiproom,
    /// Metadata-first support export surfaces.
    SupportExport,
    /// Reviewer / product documentation surfaces.
    Docs,
    /// In-product help and why-this-artifact inspectors.
    Help,
}

impl PublicationChannel {
    /// Every channel in canonical order.
    pub const ALL: [Self; 4] = [
        Self::ReleaseShiproom,
        Self::SupportExport,
        Self::Docs,
        Self::Help,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseShiproom => "release_shiproom",
            Self::SupportExport => "support_export",
            Self::Docs => "docs",
            Self::Help => "help",
        }
    }
}

/// The failure class a generated-artifact drill injects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillFailureClass {
    /// The artifact loses the link to its canonical source.
    CanonicalSourceUnlinked,
    /// The generator identity that produced the artifact is unknown.
    GeneratorIdentityUnknown,
    /// The artifact carries no typed provenance class.
    ProvenanceClassUnclassified,
    /// The writable-boundary policy is not enforced, so a direct edit
    /// could cross a canonical-source boundary.
    WritableBoundaryUnenforced,
    /// The regeneration route that rebuilds the artifact is missing.
    RegenerationRouteMissing,
    /// Drift between the derived bytes and the canonical source goes
    /// undetected.
    DriftUndetected,
    /// The reversible-checkpoint lineage that captured the change is
    /// broken.
    CheckpointLineageBroken,
}

impl DrillFailureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSourceUnlinked => "canonical_source_unlinked",
            Self::GeneratorIdentityUnknown => "generator_identity_unknown",
            Self::ProvenanceClassUnclassified => "provenance_class_unclassified",
            Self::WritableBoundaryUnenforced => "writable_boundary_unenforced",
            Self::RegenerationRouteMissing => "regeneration_route_missing",
            Self::DriftUndetected => "drift_undetected",
            Self::CheckpointLineageBroken => "checkpoint_lineage_broken",
        }
    }
}

/// One ordered phase of a generated-artifact drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// A failure is injected into a backing dimension.
    Inject,
    /// The certification observes the degraded evidence state.
    Observe,
    /// The claim and/or writable-boundary posture narrows under the
    /// failure.
    Narrow,
    /// The evidence is refreshed.
    Refresh,
    /// The claim recovers as the evidence returns to current.
    Recover,
    /// The recovered posture is verified against the engine.
    Verify,
}

impl DrillPhase {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inject => "inject",
            Self::Observe => "observe",
            Self::Narrow => "narrow",
            Self::Refresh => "refresh",
            Self::Recover => "recover",
            Self::Verify => "verify",
        }
    }
}

// ---------------------------------------------------------------------------
// Narrowing engine: the single source of truth for the verdict.
// ---------------------------------------------------------------------------

/// One dimension's evidence on one class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionEvidence {
    /// Provenance dimension being evidenced.
    pub dimension: ProvenanceDimension,
    /// State of the evidence backing this dimension.
    pub evidence_state: EvidenceState,
    /// Upstream generated-artifact packets that prove this dimension.
    pub evidence_refs: Vec<String>,
    /// Review-safe rationale for the evidence.
    pub rationale: String,
}

/// The computed outcome of certifying one row against its evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactOutcome {
    /// The narrowest maturity the claim may hold.
    pub effective_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every dimension that forced maturity
    /// narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing, in stable order.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// The strictest writable-boundary posture the claim may hold.
    pub effective_edit_posture: EditPosture,
    /// True when the writable-boundary posture narrowed below the claimed
    /// one.
    pub edit_posture_downgraded: bool,
    /// Stable tokens naming every edit-posture-governing dimension that
    /// forced a stricter boundary.
    pub edit_posture_downgrade_tokens: Vec<String>,
}

/// Certifies one row's claim against its per-dimension evidence.
///
/// This is the canonical narrowing engine the whole packet, every drill,
/// every fixture, and release / support tooling share. The effective
/// maturity starts at the claimed maturity and is floored by every
/// degraded dimension; the writable-boundary posture starts at the claimed
/// posture and is floored by every degraded edit-posture-governing
/// dimension; the narrowest (highest-severity) result wins on each axis. A
/// withdrawn maturity is [`RowVerdict::Withheld`]; any other maturity below
/// the claimed one is [`RowVerdict::Narrowed`]; otherwise the row is
/// [`RowVerdict::Certified`].
pub fn certify_artifact_outcome(
    claimed_maturity: ClaimMaturity,
    claimed_edit_posture: EditPosture,
    dimensions: &[DimensionEvidence],
) -> ArtifactOutcome {
    let mut effective_maturity = claimed_maturity;
    let mut effective_edit_posture = claimed_edit_posture;
    let mut narrow_reason_tokens = Vec::new();
    let mut edit_posture_downgrade_tokens = Vec::new();
    let mut stale_or_missing = Vec::new();

    for evidence in dimensions {
        if let Some(floor) = evidence.evidence_state.qualification_floor() {
            if floor.severity() > effective_maturity.severity() {
                effective_maturity = floor;
            }
            narrow_reason_tokens.push(format!(
                "{}_{}",
                evidence.dimension.as_str(),
                evidence.evidence_state.as_str()
            ));
        }
        if evidence.dimension.governs_edit_posture() {
            if let Some(floor) = evidence.evidence_state.edit_posture_floor() {
                if floor.severity() > effective_edit_posture.severity() {
                    effective_edit_posture = floor;
                }
                edit_posture_downgrade_tokens.push(format!(
                    "{}_{}",
                    evidence.dimension.as_str(),
                    evidence.evidence_state.as_str()
                ));
            }
        }
        if evidence.evidence_state.is_stale_or_missing() {
            stale_or_missing.push(evidence.dimension.as_str().to_owned());
        }
    }

    narrow_reason_tokens.sort();
    narrow_reason_tokens.dedup();
    edit_posture_downgrade_tokens.sort();
    edit_posture_downgrade_tokens.dedup();
    stale_or_missing.sort();
    stale_or_missing.dedup();

    let verdict = if effective_maturity == ClaimMaturity::Withdrawn {
        RowVerdict::Withheld
    } else if effective_maturity.severity() > claimed_maturity.severity() {
        RowVerdict::Narrowed
    } else {
        RowVerdict::Certified
    };

    ArtifactOutcome {
        effective_maturity,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        narrow_reason_tokens,
        stale_or_missing_dimension_tokens: stale_or_missing,
        effective_edit_posture,
        edit_posture_downgraded: effective_edit_posture.severity()
            > claimed_edit_posture.severity(),
        edit_posture_downgrade_tokens,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One artifact row: a claimed generated-artifact class, its evidence, and
/// the engine outcome stamped onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed generated-artifact class.
    pub artifact_class: ArtifactClass,
    /// Review-safe label for the class.
    pub class_label: String,
    /// How authoritative this class's bytes are relative to source.
    pub authority_class: AuthorityClass,
    /// Maturity claimed for the class.
    pub claimed_maturity: ClaimMaturity,
    /// Writable-boundary posture claimed for the class.
    pub claimed_edit_posture: EditPosture,
    /// Governance surface classes this class spans.
    pub backing_surface_classes: Vec<String>,
    /// Per-dimension evidence, one entry per required dimension.
    pub dimensions: Vec<DimensionEvidence>,
    /// Effective maturity after narrowing.
    pub effective_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its claimed maturity.
    pub narrowed: bool,
    /// Stable tokens naming every dimension that forced maturity
    /// narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Dimensions whose evidence is stale or missing.
    pub stale_or_missing_dimension_tokens: Vec<String>,
    /// Effective writable-boundary posture after narrowing.
    pub effective_edit_posture: EditPosture,
    /// True when the writable-boundary posture narrowed below the claimed
    /// one.
    pub edit_posture_downgraded: bool,
    /// Stable tokens naming every edit-posture-governing dimension that
    /// forced a stricter boundary.
    pub edit_posture_downgrade_tokens: Vec<String>,
    /// Review-safe "why this artifact" inspector line.
    pub why_this_artifact: String,
    /// Upstream generated-artifact packets this row composes.
    pub supporting_evidence_refs: Vec<String>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One automatic maturity-narrowing rule keyed by evidence state. The
/// floor is computed from [`EvidenceState::qualification_floor`], so the
/// rule set can never drift from the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceFreshnessRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Evidence state that triggers the rule.
    pub trigger_evidence_state: EvidenceState,
    /// Maturity floor the rule imposes.
    pub maturity_floor: ClaimMaturity,
    /// User-visible effect on the claim.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One automatic writable-boundary-narrowing rule keyed by evidence state
/// on an edit-posture-governing dimension. The floor is computed from
/// [`EvidenceState::edit_posture_floor`], so the rule set can never drift
/// from the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditBoundaryRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Evidence state that triggers the rule.
    pub trigger_evidence_state: EvidenceState,
    /// Writable-boundary posture floor the rule imposes.
    pub edit_posture_floor: EditPosture,
    /// User-visible effect on the writable boundary.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One ordered step inside a generated-artifact drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Maturity observed at this step.
    pub observed_maturity: ClaimMaturity,
    /// Writable-boundary posture observed at this step.
    pub observed_edit_posture: EditPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a class from an injected failure
/// through narrowing and back to recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Generated-artifact class exercised by the drill.
    pub artifact_class: ArtifactClass,
    /// Dimension whose evidence the drill degrades.
    pub exercised_dimension: ProvenanceDimension,
    /// Failure class the drill injects.
    pub failure_class: DrillFailureClass,
    /// Evidence state the dimension degrades to.
    pub degraded_evidence_state: EvidenceState,
    /// Maturity claimed before the failure.
    pub claimed_maturity: ClaimMaturity,
    /// Writable-boundary posture claimed before the failure.
    pub claimed_edit_posture: EditPosture,
    /// Verdict expected while the failure is active.
    pub expected_degraded_verdict: RowVerdict,
    /// Maturity expected while the failure is active.
    pub expected_degraded_maturity: ClaimMaturity,
    /// Writable-boundary posture expected while the failure is active.
    pub expected_degraded_edit_posture: EditPosture,
    /// Verdict expected once the evidence is refreshed.
    pub recovers_to_verdict: RowVerdict,
    /// Ordered drill steps.
    pub steps: Vec<ArtifactDrillStep>,
    /// True when the drill proves the claim narrows under the failure.
    pub asserts_claim_narrows_under_failure: bool,
    /// True when the drill proves the claim recovers after refresh.
    pub asserts_recovers_after_refresh: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a publication channel ingests this packet rather
/// than re-deriving generated-artifact truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceBinding {
    /// Channel that ingests the packet.
    pub channel: PublicationChannel,
    /// Checked consumer or contract ref.
    pub consumer_ref: String,
    /// Packet id the channel ingests.
    pub ingested_packet_id: String,
    /// Fields the channel preserves verbatim.
    pub required_verbatim_fields: Vec<String>,
    /// True when the channel narrows in lockstep with the packet.
    pub narrows_with_packet: bool,
    /// Review-safe summary of the binding.
    pub summary: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
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

/// Top-level packet governing generated-artifact truth on claimed M5
/// artifact classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeneratedGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Required provenance dimensions.
    pub certified_dimensions: Vec<ProvenanceDimension>,
    /// Upstream generated-artifact packets this matrix composes.
    pub evidence_packet_refs: Vec<String>,
    /// Artifact rows, one per claimed class.
    pub rows: Vec<ArtifactRow>,
    /// Automatic maturity-narrowing rules over evidence states.
    pub freshness_rules: Vec<EvidenceFreshnessRule>,
    /// Automatic writable-boundary-narrowing rules over evidence states.
    pub edit_boundary_rules: Vec<EditBoundaryRule>,
    /// Failure / recovery drills.
    pub drills: Vec<ArtifactDrill>,
    /// Publication-channel bindings.
    pub surface_bindings: Vec<SurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a class and an observed evidence configuration to
/// the expected verdict and writable-boundary posture, proving the
/// canonical narrowing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeneratedGovernanceFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Generated-artifact class under test.
    pub artifact_class: ArtifactClass,
    /// Maturity claimed before narrowing.
    pub claimed_maturity: ClaimMaturity,
    /// Writable-boundary posture claimed before narrowing.
    pub claimed_edit_posture: EditPosture,
    /// Observed per-dimension evidence.
    pub observed_dimensions: Vec<DimensionEvidence>,
    /// Expected verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_effective_maturity: ClaimMaturity,
    /// Expected effective writable-boundary posture.
    pub expected_edit_posture: EditPosture,
    /// Expected maturity-narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// Expected writable-boundary-downgrade tokens.
    pub expected_edit_posture_downgrade_tokens: Vec<String>,
    /// One consumer that quotes this class.
    pub consumer_ref: String,
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
        writeln!(f, "m5 generated governance validation failed")?;
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

const SCAFFOLD_LINEAGE_REF: &str =
    "artifacts/scaffold/stabilize-template-manifest-scaffold-lineage.md";
const RESTORE_PROVENANCE_REF: &str = "artifacts/migration/m3/restore_provenance_packet.md";
const TEMPLATE_HEALTH_REF: &str = "artifacts/scaffolding/template_health_states.yaml";
const EXPERIMENT_PROVENANCE_REF: &str =
    "artifacts/data/qualify-experiment-provenance-and-result-comparison.json";
const NOTEBOOK_LINEAGE_REF: &str =
    "artifacts/perf/m5/ship-coverage-profile-test-debug-and-notebook-evidence-handoff-bars-with-artifact-lineage.json";
const SAVE_REVIEW_REF: &str = "artifacts/fs/save_review_choice_matrix.yaml";
const MUTATION_CLASSES_REF: &str = "artifacts/change/mutation_classes.yaml";
const ROLLBACK_CHECKPOINT_REF: &str =
    "artifacts/migration/rollback_checkpoint_examples/checkpoint_created_pre_apply.yaml";

fn evidence_packet_refs() -> Vec<String> {
    [
        SCAFFOLD_LINEAGE_REF,
        TEMPLATE_HEALTH_REF,
        EXPERIMENT_PROVENANCE_REF,
        NOTEBOOK_LINEAGE_REF,
        SAVE_REVIEW_REF,
        MUTATION_CLASSES_REF,
        ROLLBACK_CHECKPOINT_REF,
        RESTORE_PROVENANCE_REF,
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

/// The canonical evidence refs for one dimension when it is fully current.
/// Each dimension cites the generated-artifact packets that prove it, so
/// the matrix is anchored in the checked-in artifacts.
fn dimension_evidence_refs(dimension: ProvenanceDimension) -> Vec<&'static str> {
    match dimension {
        ProvenanceDimension::CanonicalSource => vec![SCAFFOLD_LINEAGE_REF, RESTORE_PROVENANCE_REF],
        ProvenanceDimension::GeneratorIdentity => vec![TEMPLATE_HEALTH_REF, SCAFFOLD_LINEAGE_REF],
        ProvenanceDimension::ProvenanceClass => {
            vec![EXPERIMENT_PROVENANCE_REF, NOTEBOOK_LINEAGE_REF]
        }
        ProvenanceDimension::WritableBoundary => vec![SAVE_REVIEW_REF, MUTATION_CLASSES_REF],
        ProvenanceDimension::RegenerationRoute => vec![SCAFFOLD_LINEAGE_REF, MUTATION_CLASSES_REF],
        ProvenanceDimension::DriftState => vec![NOTEBOOK_LINEAGE_REF, EXPERIMENT_PROVENANCE_REF],
        ProvenanceDimension::CheckpointLineage => {
            vec![ROLLBACK_CHECKPOINT_REF, RESTORE_PROVENANCE_REF]
        }
    }
}

fn dimension_rationale(dimension: ProvenanceDimension) -> &'static str {
    match dimension {
        ProvenanceDimension::CanonicalSource => {
            "The artifact declares the canonical source it derives from, so a derived file is inspectable and diffable against its source rather than mistaken for its own source of truth."
        }
        ProvenanceDimension::GeneratorIdentity => {
            "The artifact declares the generator identity that produced it — template, kernel, builder, runner, framework, composer, or exporter — instead of leaving the producer implicit."
        }
        ProvenanceDimension::ProvenanceClass => {
            "The artifact carries a typed provenance class so search, review, AI, save, and export surfaces label its authority instead of inferring it from how the file looks on disk."
        }
        ProvenanceDimension::WritableBoundary => {
            "The artifact declares the writable-boundary policy that decides whether a direct edit is allowed, escalates through a visible reviewed override, or is blocked in favor of regeneration."
        }
        ProvenanceDimension::RegenerationRoute => {
            "The artifact declares the regeneration route that rebuilds it from its canonical source, so a derived file can be rederived rather than hand-patched across a boundary."
        }
        ProvenanceDimension::DriftState => {
            "The artifact declares its drift state — whether the derived bytes still match their canonical source — so a stale derivative is labeled stale rather than presented as current truth."
        }
        ProvenanceDimension::CheckpointLineage => {
            "The artifact declares the reversible-checkpoint lineage that captured the change, explaining exactly what local history captured, omitted, or rederived for rollback."
        }
    }
}

/// Builds the seven fully-current dimensions for a healthy row.
fn current_dimensions() -> Vec<DimensionEvidence> {
    ProvenanceDimension::ALL
        .into_iter()
        .map(|dimension| DimensionEvidence {
            dimension,
            evidence_state: EvidenceState::Current,
            evidence_refs: dimension_evidence_refs(dimension)
                .into_iter()
                .map(str::to_owned)
                .collect(),
            rationale: dimension_rationale(dimension).to_owned(),
        })
        .collect()
}

fn supporting_evidence_refs(dimensions: &[DimensionEvidence]) -> Vec<String> {
    let mut refs: BTreeSet<String> = BTreeSet::new();
    for dimension in dimensions {
        for reference in &dimension.evidence_refs {
            refs.insert(reference.clone());
        }
    }
    refs.into_iter().collect()
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    artifact_class: ArtifactClass,
    class_label: &str,
    authority_class: AuthorityClass,
    claimed_maturity: ClaimMaturity,
    claimed_edit_posture: EditPosture,
    backing_surface_classes: &[&str],
    why_this_artifact: &str,
    consumer_refs: &[&str],
    notes: &str,
) -> ArtifactRow {
    let dimensions = current_dimensions();
    let outcome = certify_artifact_outcome(claimed_maturity, claimed_edit_posture, &dimensions);
    let supporting_evidence_refs = supporting_evidence_refs(&dimensions);
    ArtifactRow {
        row_id: row_id.to_owned(),
        artifact_class,
        class_label: class_label.to_owned(),
        authority_class,
        claimed_maturity,
        claimed_edit_posture,
        backing_surface_classes: backing_surface_classes
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        dimensions,
        effective_maturity: outcome.effective_maturity,
        verdict: outcome.verdict,
        narrowed: outcome.narrowed,
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        stale_or_missing_dimension_tokens: outcome.stale_or_missing_dimension_tokens,
        effective_edit_posture: outcome.effective_edit_posture,
        edit_posture_downgraded: outcome.edit_posture_downgraded,
        edit_posture_downgrade_tokens: outcome.edit_posture_downgrade_tokens,
        why_this_artifact: why_this_artifact.to_owned(),
        supporting_evidence_refs,
        consumer_refs: consumer_refs.iter().map(|s| (*s).to_owned()).collect(),
        notes: notes.to_owned(),
    }
}

fn freshness_rule(
    rule_id: &str,
    trigger: EvidenceState,
    effect: &str,
    rationale: &str,
) -> EvidenceFreshnessRule {
    EvidenceFreshnessRule {
        rule_id: rule_id.to_owned(),
        trigger_evidence_state: trigger,
        maturity_floor: trigger
            .qualification_floor()
            .expect("freshness rules only encode triggers that impose a maturity floor"),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn edit_boundary_rule(
    rule_id: &str,
    trigger: EvidenceState,
    effect: &str,
    rationale: &str,
) -> EditBoundaryRule {
    EditBoundaryRule {
        rule_id: rule_id.to_owned(),
        trigger_evidence_state: trigger,
        edit_posture_floor: trigger
            .edit_posture_floor()
            .expect("edit-boundary rules only encode triggers that impose a posture floor"),
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    observed_maturity: ClaimMaturity,
    observed_edit_posture: EditPosture,
    narration: &str,
) -> ArtifactDrillStep {
    ArtifactDrillStep {
        phase,
        observed_maturity,
        observed_edit_posture,
        narration: narration.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn drill(
    drill_id: &str,
    title: &str,
    artifact_class: ArtifactClass,
    exercised_dimension: ProvenanceDimension,
    failure_class: DrillFailureClass,
    degraded_evidence_state: EvidenceState,
    claimed_maturity: ClaimMaturity,
    claimed_edit_posture: EditPosture,
    steps: Vec<ArtifactDrillStep>,
    notes: &str,
) -> ArtifactDrill {
    // The degraded posture is computed from the same engine the rows use,
    // so a drill can never disagree with the certification.
    let mut degraded = current_dimensions();
    for evidence in &mut degraded {
        if evidence.dimension == exercised_dimension {
            evidence.evidence_state = degraded_evidence_state;
        }
    }
    let degraded_outcome =
        certify_artifact_outcome(claimed_maturity, claimed_edit_posture, &degraded);
    ArtifactDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        artifact_class,
        exercised_dimension,
        failure_class,
        degraded_evidence_state,
        claimed_maturity,
        claimed_edit_posture,
        expected_degraded_verdict: degraded_outcome.verdict,
        expected_degraded_maturity: degraded_outcome.effective_maturity,
        expected_degraded_edit_posture: degraded_outcome.effective_edit_posture,
        recovers_to_verdict: RowVerdict::Certified,
        steps,
        asserts_claim_narrows_under_failure: true,
        asserts_recovers_after_refresh: true,
        notes: notes.to_owned(),
    }
}

fn binding(channel: PublicationChannel, consumer_ref: &str, summary: &str) -> SurfaceBinding {
    SurfaceBinding {
        channel,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: PACKET_ID.to_owned(),
        required_verbatim_fields: REQUIRED_VERBATIM_FIELDS
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrows_with_packet: true,
        summary: summary.to_owned(),
    }
}

const PACKET_ID: &str = "generated.m5_generated_governance.v1";

const REQUIRED_VERBATIM_FIELDS: [&str; 7] = [
    "row_id",
    "artifact_class",
    "claimed_maturity",
    "effective_maturity",
    "verdict",
    "effective_edit_posture",
    "narrow_reason_tokens",
];

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in generated-artifact-governance packet this lane
/// freezes.
pub fn seeded_m5_generated_governance_packet() -> M5GeneratedGovernancePacket {
    let rows = vec![
        row(
            "generated.artifact.scaffolded_project",
            ArtifactClass::ScaffoldedProject,
            "Scaffolded project",
            AuthorityClass::CanonicalAuthoritative,
            ClaimMaturity::Stable,
            EditPosture::DirectEditAllowed,
            &["scaffold_planner", "file_tree"],
            "This artifact is a project scaffolded from a template: once written, its files are the canonical source the user edits directly, with the template recorded as origin and a regeneration route back to the scaffold.",
            &[
                "crates/aureline-scaffold/src/stabilize_template_manifest_scaffold_lineage/mod.rs",
                "crates/aureline-vfs/src/save_conflict_suite/mod.rs",
            ],
            "A scaffolded project is authoritative source the user owns; its canonical-source linkage and writable boundary must stay current for direct edits to remain safe.",
        ),
        row(
            "generated.artifact.notebook_output",
            ArtifactClass::NotebookOutput,
            "Notebook cell output",
            AuthorityClass::DerivedReadonly,
            ClaimMaturity::Beta,
            EditPosture::RegenerateOnly,
            &["notebook_kernel", "output_inspector"],
            "This artifact is a notebook cell output derived from a kernel run: it points back to the cell source and execution as its canonical source, declares its drift state, and is regenerated rather than hand-edited.",
            &[
                "crates/aureline-notebook/src/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback/mod.rs",
                "crates/aureline-notebook/src/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet/mod.rs",
            ],
            "A notebook output is purely derived; it is regenerated from its cell source and never presented as editable authoritative source.",
        ),
        row(
            "generated.artifact.preview_derivative",
            ArtifactClass::PreviewDerivative,
            "Preview/runtime derivative",
            AuthorityClass::DerivedReadonly,
            ClaimMaturity::Beta,
            EditPosture::RegenerateOnly,
            &["preview_builder", "source_map"],
            "This artifact is a preview or runtime derivative built from source: its source map links every derived byte back to canonical source, and a drift signal invalidates it rather than serving stale output.",
            &[
                "crates/aureline-preview/src/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/mod.rs",
                "crates/aureline-preview/src/preview_drift_recovery/mod.rs",
            ],
            "A preview derivative is regenerated from source; edits belong in the source the source map points at, never in the derived output.",
        ),
        row(
            "generated.artifact.request_artifact",
            ArtifactClass::RequestArtifact,
            "API/request artifact",
            AuthorityClass::DerivedEditable,
            ClaimMaturity::Beta,
            EditPosture::ReviewedOverrideRequired,
            &["request_runner", "replay_lane"],
            "This artifact is an API/request artifact captured from a request run: it records the request and contract as canonical source, and a direct edit to the captured response escalates through a visible reviewed override.",
            &[
                "crates/aureline-api/src/implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export/mod.rs",
                "crates/aureline-api/src/freeze_the_api_collection_contract_source_request_origin_and_persisted_operation_matrix/mod.rs",
            ],
            "A request artifact is derived from its request and contract; editing the captured bytes crosses a boundary and must go through a reviewed override.",
        ),
        row(
            "generated.artifact.framework_codegen",
            ArtifactClass::FrameworkCodegen,
            "Framework codegen",
            AuthorityClass::DerivedEditable,
            ClaimMaturity::Beta,
            EditPosture::ReviewedOverrideRequired,
            &["framework_generator", "review_inspector"],
            "This artifact is code emitted by a framework generator: it declares the generator identity and the regeneration route, and direct edits to generated regions escalate through a reviewed override rather than being silently overwritten on the next regeneration.",
            &[
                "crates/aureline-review/src/change_inspector/mod.rs",
                "crates/aureline-scaffold/src/freeze_the_m5_template_registry_framework_pack_and_support_class_matrix/mod.rs",
            ],
            "Framework codegen is derived editable; a direct edit to a generated region must escalate through a reviewed override so it survives the next regeneration.",
        ),
        row(
            "generated.artifact.ai_assisted_edit",
            ArtifactClass::AiAssistedEdit,
            "AI-assisted edit",
            AuthorityClass::CanonicalAuthoritative,
            ClaimMaturity::Stable,
            EditPosture::DirectEditAllowed,
            &["ai_composer", "mutation_journal"],
            "This artifact is an edit produced by an AI-assisted composer: the resulting file is the canonical source the user owns and edits directly, with the composer recorded as generator and the change captured in the reversible-checkpoint lineage.",
            &[
                "crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs",
                "crates/aureline-history/src/mutation_journal/mod.rs",
            ],
            "An AI-assisted edit becomes authoritative source; its canonical-source linkage and writable boundary must stay current for the direct-edit claim to hold.",
        ),
        row(
            "generated.artifact.support_packet",
            ArtifactClass::SupportPacket,
            "Exportable support packet",
            AuthorityClass::DerivedReadonly,
            ClaimMaturity::Stable,
            EditPosture::RegenerateOnly,
            &["support_exporter", "redaction_boundary"],
            "This artifact is an exportable support packet projected from product truth: it is a redacted, regenerated projection that records what was captured, omitted, or rederived, and is never hand-edited.",
            &[
                "crates/aureline-support/src/field_readiness/mod.rs",
                "crates/aureline-workspace/src/local_history_export_replay_lineage/mod.rs",
            ],
            "A support packet is a regenerated redacted projection; it is rebuilt from product truth rather than edited, so its captured/omitted/rederived lineage stays honest.",
        ),
    ];

    let freshness_rules = vec![
        freshness_rule(
            "freshness.partial_narrows_to_beta",
            EvidenceState::Partial,
            "A claimed class with partial evidence on any required dimension narrows to at most a beta claim.",
            "Partial provenance evidence proves only part of the claimed generated-artifact contract, so the class may not present a stable generated-artifact guarantee.",
        ),
        freshness_rule(
            "freshness.stale_narrows_to_preview",
            EvidenceState::Stale,
            "A claimed class with stale evidence on any required dimension narrows to at most a preview claim.",
            "Stale provenance evidence may no longer reflect the current source, generator, or drift truth, so the class drops below beta until the evidence is refreshed.",
        ),
        freshness_rule(
            "freshness.missing_withholds_claim",
            EvidenceState::Missing,
            "A claimed class missing evidence on any required dimension is withheld; promotion fails until the dimension is proven.",
            "A required generated-artifact dimension with no backing evidence cannot be proven, so the class may not be promoted at its claimed maturity.",
        ),
    ];

    let edit_boundary_rules = vec![
        edit_boundary_rule(
            "edit_boundary.partial_narrows_to_reviewed_override",
            EvidenceState::Partial,
            "Partial canonical-source or writable-boundary evidence narrows the writable-boundary posture to at most a reviewed override.",
            "A partially proven boundary cannot prove a direct edit will survive regeneration, so the edit must escalate through a visible reviewed override.",
        ),
        edit_boundary_rule(
            "edit_boundary.stale_forces_regenerate_only",
            EvidenceState::Stale,
            "Stale canonical-source or writable-boundary evidence forces a regenerate-only boundary.",
            "A stale boundary can no longer prove a direct edit is safe against the canonical source, so the artifact must be regenerated rather than hand-patched.",
        ),
        edit_boundary_rule(
            "edit_boundary.missing_forces_regenerate_only",
            EvidenceState::Missing,
            "Missing canonical-source or writable-boundary evidence forces a regenerate-only boundary.",
            "Without a canonical source or boundary policy the artifact cannot be safely edited in place, so direct edits are blocked in favor of regeneration.",
        ),
    ];

    let drills = vec![
        drill(
            "drill.generated_governance.scaffolded_project_canonical_source_partial",
            "Scaffolded project narrows to beta and a reviewed override on partial canonical-source coverage",
            ArtifactClass::ScaffoldedProject,
            ProvenanceDimension::CanonicalSource,
            DrillFailureClass::CanonicalSourceUnlinked,
            EvidenceState::Partial,
            ClaimMaturity::Stable,
            EditPosture::DirectEditAllowed,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "The scaffolded project's canonical-source linkage covers only part of its files after the template manifest changes under it.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "Canonical-source evidence is observed partial for the scaffolded-project class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The certified claim narrows to beta and the writable boundary drops to a reviewed override; a direct edit to a partially-linked file now escalates instead of silently crossing the boundary.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The canonical-source linkage is recomputed across the remaining files.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "Canonical-source evidence returns current; the claim recovers to stable with direct edits allowed.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "The recovered posture matches the certification engine for a fully current scaffolded-project row.",
                ),
            ],
            "Partial canonical-source coverage narrows the scaffolded-project claim to beta and downgrades direct edits to a reviewed override without withholding the class.",
        ),
        drill(
            "drill.generated_governance.ai_assisted_edit_writable_boundary_stale",
            "AI-assisted edit narrows to preview and regenerate-only when its writable boundary goes stale",
            ArtifactClass::AiAssistedEdit,
            ProvenanceDimension::WritableBoundary,
            DrillFailureClass::WritableBoundaryUnenforced,
            EvidenceState::Stale,
            ClaimMaturity::Stable,
            EditPosture::DirectEditAllowed,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "The AI-assisted edit's writable-boundary policy ages past its freshness window after the scoped-apply boundary contract rolls.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "Writable-boundary evidence is observed stale for the AI-assisted-edit class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The certified claim narrows to preview and the writable boundary drops to regenerate-only; direct edits are blocked until the boundary is re-proven.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The writable-boundary policy is re-pinned against the current scoped-apply contract.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "Writable-boundary evidence returns current; the claim recovers to stable with direct edits allowed.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    EditPosture::DirectEditAllowed,
                    "The recovered posture matches the certification engine for a fully current AI-assisted-edit row.",
                ),
            ],
            "A stale writable boundary narrows the AI-assisted-edit claim to preview and forces a regenerate-only boundary, never letting a direct edit cross an unproven canonical-source boundary.",
        ),
        drill(
            "drill.generated_governance.framework_codegen_regeneration_route_missing",
            "Framework codegen is withheld when its regeneration route is missing",
            ArtifactClass::FrameworkCodegen,
            ProvenanceDimension::RegenerationRoute,
            DrillFailureClass::RegenerationRouteMissing,
            EvidenceState::Missing,
            ClaimMaturity::Beta,
            EditPosture::ReviewedOverrideRequired,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The framework-codegen artifact declares no regeneration route, so it cannot be rebuilt from its canonical source.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "Regeneration-route evidence is observed missing for the framework-codegen class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Withdrawn,
                    EditPosture::ReviewedOverrideRequired,
                    "The certification withholds the framework-codegen claim; the generated code cannot be presented as governed until a regeneration route is declared.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Withdrawn,
                    EditPosture::ReviewedOverrideRequired,
                    "The regeneration route is declared and bound to the framework generator.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "Regeneration-route evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The recovered posture matches the certification engine for a fully current framework-codegen row.",
                ),
            ],
            "Missing regeneration-route evidence withholds the framework-codegen claim; the writable boundary is unaffected because the regeneration route does not govern the edit posture.",
        ),
        drill(
            "drill.generated_governance.request_artifact_provenance_class_stale",
            "Request artifact narrows to preview when its provenance class goes stale",
            ArtifactClass::RequestArtifact,
            ProvenanceDimension::ProvenanceClass,
            DrillFailureClass::ProvenanceClassUnclassified,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            EditPosture::ReviewedOverrideRequired,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The request artifact's provenance class ages past its freshness window after the contract source it was classified against changes.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "Provenance-class evidence is observed stale for the request-artifact class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    EditPosture::ReviewedOverrideRequired,
                    "The certified claim narrows to preview; the captured response is labeled with a stale provenance class rather than presented as a current authoritative record.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    EditPosture::ReviewedOverrideRequired,
                    "The provenance class is recomputed against the current contract source.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "Provenance-class evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    EditPosture::ReviewedOverrideRequired,
                    "The recovered posture matches the certification engine for a fully current request-artifact row.",
                ),
            ],
            "A stale provenance class narrows the request-artifact claim to preview until it is reclassified against the current contract source.",
        ),
        drill(
            "drill.generated_governance.notebook_output_drift_undetected",
            "Notebook output narrows to preview when drift goes undetected",
            ArtifactClass::NotebookOutput,
            ProvenanceDimension::DriftState,
            DrillFailureClass::DriftUndetected,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            EditPosture::RegenerateOnly,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "The notebook output's drift signal ages past its freshness window after the cell source changes without a re-run.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "Drift-state evidence is observed stale for the notebook-output class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The certified claim narrows to preview; the output is labeled possibly-stale rather than presented as matching its current cell source.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The cell is re-run and the drift signal is recaptured against the current source.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "Drift-state evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "The recovered posture matches the certification engine for a fully current notebook-output row.",
                ),
            ],
            "Stale drift evidence narrows the notebook-output claim below beta even though the output rendered once on a happy-path run.",
        ),
        drill(
            "drill.generated_governance.preview_derivative_generator_identity_stale",
            "Preview derivative narrows to preview when its generator identity goes stale",
            ArtifactClass::PreviewDerivative,
            ProvenanceDimension::GeneratorIdentity,
            DrillFailureClass::GeneratorIdentityUnknown,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            EditPosture::RegenerateOnly,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "The preview derivative's recorded generator identity ages past its freshness window after a builder stage is swapped under it.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "Generator-identity evidence is observed stale for the preview-derivative class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The certified claim narrows to preview; the derivative labels its generator stale rather than implying a single current trusted builder.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The generator identity is recaptured against the current build stages.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "Generator-identity evidence returns current; the claim recovers to its beta maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Beta,
                    EditPosture::RegenerateOnly,
                    "The recovered posture matches the certification engine for a fully current preview-derivative row.",
                ),
            ],
            "A stale generator identity narrows the preview-derivative claim to preview; the regenerate-only boundary is unchanged because generator identity does not govern the edit posture.",
        ),
        drill(
            "drill.generated_governance.support_packet_checkpoint_lineage_broken",
            "Support packet narrows to preview when its checkpoint lineage breaks",
            ArtifactClass::SupportPacket,
            ProvenanceDimension::CheckpointLineage,
            DrillFailureClass::CheckpointLineageBroken,
            EvidenceState::Stale,
            ClaimMaturity::Stable,
            EditPosture::RegenerateOnly,
            vec![
                step(
                    DrillPhase::Inject,
                    ClaimMaturity::Stable,
                    EditPosture::RegenerateOnly,
                    "The support packet's reversible-checkpoint lineage ages past its freshness window after the captured local-history window rolls.",
                ),
                step(
                    DrillPhase::Observe,
                    ClaimMaturity::Stable,
                    EditPosture::RegenerateOnly,
                    "Checkpoint-lineage evidence is observed stale for the support-packet class.",
                ),
                step(
                    DrillPhase::Narrow,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The certified claim narrows to preview; the packet labels its captured/omitted/rederived lineage stale rather than presenting it as a complete rollback record.",
                ),
                step(
                    DrillPhase::Refresh,
                    ClaimMaturity::Preview,
                    EditPosture::RegenerateOnly,
                    "The checkpoint lineage is recaptured against the current local-history window.",
                ),
                step(
                    DrillPhase::Recover,
                    ClaimMaturity::Stable,
                    EditPosture::RegenerateOnly,
                    "Checkpoint-lineage evidence returns current; the claim recovers to its stable maturity.",
                ),
                step(
                    DrillPhase::Verify,
                    ClaimMaturity::Stable,
                    EditPosture::RegenerateOnly,
                    "The recovered posture matches the certification engine for a fully current support-packet row.",
                ),
            ],
            "A broken checkpoint lineage narrows the support-packet claim to preview so the export never overstates what local history captured.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            PublicationChannel::ReleaseShiproom,
            "artifacts/release/shiproom_dashboard.json",
            "The shiproom dashboard reads the per-row verdict, effective maturity, and writable-boundary posture and holds promotion for any narrowed or withheld release-scope generated-artifact class.",
        ),
        binding(
            PublicationChannel::SupportExport,
            "crates/aureline-support/src/field_readiness/mod.rs",
            "The metadata-first support bundle re-exports the per-row verdict, narrowing tokens, writable-boundary posture, and stale-or-missing dimensions without raw paths, credentials, or generator payloads.",
        ),
        binding(
            PublicationChannel::Docs,
            "docs/generated/m5-generated-governance.md",
            "The reviewer documentation quotes the certified dimensions, freshness and edit-boundary rules, and per-row verdicts directly from the packet.",
        ),
        binding(
            PublicationChannel::Help,
            "crates/aureline-review/src/change_inspector/mod.rs",
            "The in-product why-this-artifact inspector reuses the same verdict and writable-boundary vocabulary so help never presents a derived artifact as more authoritative than the packet.",
        ),
    ];

    M5GeneratedGovernancePacket {
        record_kind: M5_GENERATED_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_GENERATED_GOVERNANCE_SCHEMA_VERSION,
        packet_id: PACKET_ID.to_owned(),
        title: "Generated-artifact provenance, regeneration, writable-boundary, and reversible-checkpoint governance for claimed M5 artifact classes"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: M5_GENERATED_GOVERNANCE_DOC_REF.to_owned(),
            schema_ref: M5_GENERATED_GOVERNANCE_SCHEMA_REF.to_owned(),
            packet_ref: M5_GENERATED_GOVERNANCE_PACKET_REF.to_owned(),
            report_ref: M5_GENERATED_GOVERNANCE_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF.to_owned(),
        },
        certified_dimensions: ProvenanceDimension::ALL.to_vec(),
        evidence_packet_refs: evidence_packet_refs(),
        rows,
        freshness_rules,
        edit_boundary_rules,
        drills,
        surface_bindings,
        invariants: vec![
            "Each claimed M5 generated-artifact class is certified only when every required provenance dimension — canonical source, generator identity, provenance class, writable boundary, regeneration route, drift state, and checkpoint lineage — is proven current.".to_owned(),
            "One narrowing engine folds per-dimension evidence into a verdict and a writable-boundary posture: partial evidence narrows to beta, stale evidence narrows to preview, missing evidence withholds the claim, and stale or partial canonical-source/writable-boundary evidence narrows the edit posture.".to_owned(),
            "Derived bytes are not their own source: a direct-edit claim drops to a reviewed override or a regenerate-only boundary whenever the canonical-source linkage or writable-boundary policy outruns current truth.".to_owned(),
            "No generated artifact is presented as ordinary authoritative source merely because it looks like a file on disk; a class absent from the packet is uncertified rather than implicitly authoritative, and the certification only narrows, never widens.".to_owned(),
            "Release, support, docs, and help all read the same per-row verdict, writable-boundary posture, and narrowing tokens instead of re-deriving generated-artifact staleness.".to_owned(),
            "Failure and recovery drills exercise each class through narrowing and back, computed from the same engine so the certification, drills, and fixtures cannot disagree.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture corpus this lane freezes.
pub fn seeded_m5_generated_governance_fixtures() -> Vec<M5GeneratedGovernanceFixture> {
    let mut fixtures = Vec::new();

    // One healthy fixture per class, pinning the certified verdict.
    for artifact_class in ArtifactClass::ALL {
        let (claimed_maturity, claimed_edit_posture) = claimed_posture_for(artifact_class);
        fixtures.push(fixture(
            &format!(
                "fixture.m5_generated_governance.{}_certified",
                artifact_class.as_str()
            ),
            artifact_class,
            claimed_maturity,
            claimed_edit_posture,
            current_dimensions(),
            consumer_ref_for(artifact_class),
            "A fully current class certifies at its claimed maturity and writable-boundary posture with no narrowing tokens.",
        ));
    }

    // Degraded fixtures exercising every floor and verdict.
    fixtures.push(fixture(
        "fixture.m5_generated_governance.scaffolded_project_canonical_source_partial",
        ArtifactClass::ScaffoldedProject,
        ClaimMaturity::Stable,
        EditPosture::DirectEditAllowed,
        degraded_dimensions(ProvenanceDimension::CanonicalSource, EvidenceState::Partial),
        consumer_ref_for(ArtifactClass::ScaffoldedProject),
        "Partial canonical-source evidence narrows the stable scaffolded-project claim to a beta verdict and downgrades direct edits to a reviewed override.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_governance.ai_assisted_edit_writable_boundary_stale",
        ArtifactClass::AiAssistedEdit,
        ClaimMaturity::Stable,
        EditPosture::DirectEditAllowed,
        degraded_dimensions(ProvenanceDimension::WritableBoundary, EvidenceState::Stale),
        consumer_ref_for(ArtifactClass::AiAssistedEdit),
        "A stale writable boundary narrows the stable AI-assisted-edit claim to a preview verdict and forces a regenerate-only boundary instead of allowing a direct edit.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_governance.framework_codegen_regeneration_route_missing",
        ArtifactClass::FrameworkCodegen,
        ClaimMaturity::Beta,
        EditPosture::ReviewedOverrideRequired,
        degraded_dimensions(
            ProvenanceDimension::RegenerationRoute,
            EvidenceState::Missing,
        ),
        consumer_ref_for(ArtifactClass::FrameworkCodegen),
        "Missing regeneration-route evidence withholds the framework-codegen claim entirely.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_governance.notebook_output_drift_stale",
        ArtifactClass::NotebookOutput,
        ClaimMaturity::Beta,
        EditPosture::RegenerateOnly,
        degraded_dimensions(ProvenanceDimension::DriftState, EvidenceState::Stale),
        consumer_ref_for(ArtifactClass::NotebookOutput),
        "Stale drift evidence narrows the notebook-output claim to preview while the regenerate-only boundary stays unchanged because drift state does not govern the edit posture.",
    ));

    fixtures
}

fn claimed_posture_for(artifact_class: ArtifactClass) -> (ClaimMaturity, EditPosture) {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => (ClaimMaturity::Stable, EditPosture::DirectEditAllowed),
        ArtifactClass::NotebookOutput => (ClaimMaturity::Beta, EditPosture::RegenerateOnly),
        ArtifactClass::PreviewDerivative => (ClaimMaturity::Beta, EditPosture::RegenerateOnly),
        ArtifactClass::RequestArtifact => {
            (ClaimMaturity::Beta, EditPosture::ReviewedOverrideRequired)
        }
        ArtifactClass::FrameworkCodegen => {
            (ClaimMaturity::Beta, EditPosture::ReviewedOverrideRequired)
        }
        ArtifactClass::AiAssistedEdit => (ClaimMaturity::Stable, EditPosture::DirectEditAllowed),
        ArtifactClass::SupportPacket => (ClaimMaturity::Stable, EditPosture::RegenerateOnly),
    }
}

fn consumer_ref_for(artifact_class: ArtifactClass) -> &'static str {
    match artifact_class {
        ArtifactClass::ScaffoldedProject => {
            "crates/aureline-scaffold/src/stabilize_template_manifest_scaffold_lineage/mod.rs"
        }
        ArtifactClass::NotebookOutput => {
            "crates/aureline-notebook/src/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback/mod.rs"
        }
        ArtifactClass::PreviewDerivative => {
            "crates/aureline-preview/src/freeze_the_m5_source_first_preview_runtime_source_map_and_browser_runtime_inspection_matrix/mod.rs"
        }
        ArtifactClass::RequestArtifact => {
            "crates/aureline-api/src/implement_the_request_composer_mutation_review_sheets_and_replay_or_history_lanes_with_redaction_safe_export/mod.rs"
        }
        ArtifactClass::FrameworkCodegen => "crates/aureline-review/src/change_inspector/mod.rs",
        ArtifactClass::AiAssistedEdit => "crates/aureline-ai/src/harden_ai_scoped_apply/mod.rs",
        ArtifactClass::SupportPacket => "crates/aureline-support/src/field_readiness/mod.rs",
    }
}

fn degraded_dimensions(
    dimension: ProvenanceDimension,
    state: EvidenceState,
) -> Vec<DimensionEvidence> {
    let mut dimensions = current_dimensions();
    for evidence in &mut dimensions {
        if evidence.dimension == dimension {
            evidence.evidence_state = state;
        }
    }
    dimensions
}

#[allow(clippy::too_many_arguments)]
fn fixture(
    fixture_id: &str,
    artifact_class: ArtifactClass,
    claimed_maturity: ClaimMaturity,
    claimed_edit_posture: EditPosture,
    observed_dimensions: Vec<DimensionEvidence>,
    consumer_ref: &str,
    notes: &str,
) -> M5GeneratedGovernanceFixture {
    let outcome =
        certify_artifact_outcome(claimed_maturity, claimed_edit_posture, &observed_dimensions);
    M5GeneratedGovernanceFixture {
        record_kind: M5_GENERATED_GOVERNANCE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: M5_GENERATED_GOVERNANCE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        artifact_class,
        claimed_maturity,
        claimed_edit_posture,
        observed_dimensions,
        expected_verdict: outcome.verdict,
        expected_effective_maturity: outcome.effective_maturity,
        expected_edit_posture: outcome.effective_edit_posture,
        expected_narrow_reason_tokens: outcome.narrow_reason_tokens,
        expected_edit_posture_downgrade_tokens: outcome.edit_posture_downgrade_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in packet contract.
pub fn validate_m5_generated_governance_packet(
    packet: &M5GeneratedGovernancePacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_GENERATED_GOVERNANCE_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != M5_GENERATED_GOVERNANCE_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.source_contract_refs.doc_ref != M5_GENERATED_GOVERNANCE_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != M5_GENERATED_GOVERNANCE_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != M5_GENERATED_GOVERNANCE_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != M5_GENERATED_GOVERNANCE_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != M5_GENERATED_GOVERNANCE_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.certified_dimensions != ProvenanceDimension::ALL.to_vec() {
        report.push(
            "packet.certified_dimensions",
            "packet must certify every required dimension in canonical order",
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

    let mut covered_classes = BTreeSet::new();
    for artifact_row in &packet.rows {
        if !covered_classes.insert(artifact_row.artifact_class) {
            report.push(
                "row.class_unique",
                format!("duplicate class {}", artifact_row.artifact_class.as_str()),
            );
        }
        validate_row(&mut report, artifact_row);
    }
    for required in ArtifactClass::ALL {
        if !covered_classes.contains(&required) {
            report.push(
                "packet.covered_class",
                format!("packet must certify class {}", required.as_str()),
            );
        }
    }

    validate_freshness_rules(&mut report, packet);
    validate_edit_boundary_rules(&mut report, packet);
    validate_drills(&mut report, packet);
    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_dimensions(
    report: &mut ValidationReport,
    owner: &str,
    dimensions: &[DimensionEvidence],
) {
    let mut seen = BTreeSet::new();
    for evidence in dimensions {
        if !seen.insert(evidence.dimension) {
            report.push(
                "dimension.unique",
                format!("{owner} repeats dimension {}", evidence.dimension.as_str()),
            );
        }
        if evidence.evidence_state != EvidenceState::Missing && evidence.evidence_refs.is_empty() {
            report.push(
                "dimension.evidence_refs",
                format!(
                    "{owner} dimension {} must cite evidence unless it is missing",
                    evidence.dimension.as_str()
                ),
            );
        }
        if evidence.rationale.trim().is_empty() {
            report.push(
                "dimension.rationale",
                format!(
                    "{owner} dimension {} must carry a rationale",
                    evidence.dimension.as_str()
                ),
            );
        }
    }
    for required in ProvenanceDimension::ALL {
        if !seen.contains(&required) {
            report.push(
                "dimension.coverage",
                format!("{owner} must evidence dimension {}", required.as_str()),
            );
        }
    }
}

fn validate_row(report: &mut ValidationReport, artifact_row: &ArtifactRow) {
    if artifact_row.row_id.trim().is_empty() {
        report.push("row.id", "row must carry a stable id");
    }
    if artifact_row.class_label.trim().is_empty() {
        report.push(
            "row.class_label",
            format!("row {} must carry a class label", artifact_row.row_id),
        );
    }
    if artifact_row.backing_surface_classes.is_empty() {
        report.push(
            "row.backing_surface_classes",
            format!(
                "row {} must name its backing surface classes",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.why_this_artifact.trim().is_empty() {
        report.push(
            "row.why_this_artifact",
            format!(
                "row {} must carry a why-this-artifact inspector line",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.consumer_refs.is_empty() {
        report.push(
            "row.consumer_refs",
            format!(
                "row {} must cite at least one consumer ref",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.notes.trim().is_empty() {
        report.push(
            "row.notes",
            format!("row {} must carry a reviewer note", artifact_row.row_id),
        );
    }

    validate_dimensions(
        report,
        &format!("row {}", artifact_row.row_id),
        &artifact_row.dimensions,
    );

    // The stamped outcome must equal what the engine computes.
    let outcome = certify_artifact_outcome(
        artifact_row.claimed_maturity,
        artifact_row.claimed_edit_posture,
        &artifact_row.dimensions,
    );
    if artifact_row.effective_maturity != outcome.effective_maturity {
        report.push(
            "row.effective_maturity",
            format!(
                "row {} effective_maturity {} disagrees with the engine ({})",
                artifact_row.row_id,
                artifact_row.effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if artifact_row.verdict != outcome.verdict {
        report.push(
            "row.verdict",
            format!(
                "row {} verdict {} disagrees with the engine ({})",
                artifact_row.row_id,
                artifact_row.verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if artifact_row.narrowed != outcome.narrowed {
        report.push(
            "row.narrowed",
            format!(
                "row {} narrowed flag disagrees with the engine",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "row.narrow_reason_tokens",
            format!(
                "row {} narrow_reason_tokens disagree with the engine",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.stale_or_missing_dimension_tokens != outcome.stale_or_missing_dimension_tokens {
        report.push(
            "row.stale_or_missing_dimension_tokens",
            format!(
                "row {} stale_or_missing_dimension_tokens disagree with the engine",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.effective_edit_posture != outcome.effective_edit_posture {
        report.push(
            "row.effective_edit_posture",
            format!(
                "row {} effective_edit_posture {} disagrees with the engine ({})",
                artifact_row.row_id,
                artifact_row.effective_edit_posture.as_str(),
                outcome.effective_edit_posture.as_str()
            ),
        );
    }
    if artifact_row.edit_posture_downgraded != outcome.edit_posture_downgraded {
        report.push(
            "row.edit_posture_downgraded",
            format!(
                "row {} edit_posture_downgraded flag disagrees with the engine",
                artifact_row.row_id
            ),
        );
    }
    if artifact_row.edit_posture_downgrade_tokens != outcome.edit_posture_downgrade_tokens {
        report.push(
            "row.edit_posture_downgrade_tokens",
            format!(
                "row {} edit_posture_downgrade_tokens disagree with the engine",
                artifact_row.row_id
            ),
        );
    }

    let expected_support = supporting_evidence_refs(&artifact_row.dimensions);
    if artifact_row.supporting_evidence_refs != expected_support {
        report.push(
            "row.supporting_evidence_refs",
            format!(
                "row {} supporting_evidence_refs must equal the union of its dimension evidence refs",
                artifact_row.row_id
            ),
        );
    }
}

fn validate_freshness_rules(report: &mut ValidationReport, packet: &M5GeneratedGovernancePacket) {
    if packet.freshness_rules.is_empty() {
        report.push(
            "packet.freshness_rules",
            "packet must declare freshness rules",
        );
    }
    let mut covered = BTreeSet::new();
    for rule in &packet.freshness_rules {
        covered.insert(rule.trigger_evidence_state);
        match rule.trigger_evidence_state.qualification_floor() {
            Some(expected) if expected == rule.maturity_floor => {}
            Some(expected) => report.push(
                "freshness_rule.floor",
                format!(
                    "rule {} floor {} disagrees with the engine ({})",
                    rule.rule_id,
                    rule.maturity_floor.as_str(),
                    expected.as_str()
                ),
            ),
            None => report.push(
                "freshness_rule.trigger",
                format!(
                    "rule {} trigger {} imposes no maturity floor and must not be a rule",
                    rule.rule_id,
                    rule.trigger_evidence_state.as_str()
                ),
            ),
        }
        if rule.effect.trim().is_empty() || rule.rationale.trim().is_empty() {
            report.push(
                "freshness_rule.prose",
                format!("rule {} must carry an effect and rationale", rule.rule_id),
            );
        }
    }
    for required in [
        EvidenceState::Partial,
        EvidenceState::Stale,
        EvidenceState::Missing,
    ] {
        if !covered.contains(&required) {
            report.push(
                "packet.freshness_rule_coverage",
                format!(
                    "packet must encode a freshness rule for {} evidence",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_edit_boundary_rules(
    report: &mut ValidationReport,
    packet: &M5GeneratedGovernancePacket,
) {
    if packet.edit_boundary_rules.is_empty() {
        report.push(
            "packet.edit_boundary_rules",
            "packet must declare edit-boundary rules",
        );
    }
    let mut covered = BTreeSet::new();
    for rule in &packet.edit_boundary_rules {
        covered.insert(rule.trigger_evidence_state);
        match rule.trigger_evidence_state.edit_posture_floor() {
            Some(expected) if expected == rule.edit_posture_floor => {}
            Some(expected) => report.push(
                "edit_boundary_rule.floor",
                format!(
                    "rule {} edit-posture floor {} disagrees with the engine ({})",
                    rule.rule_id,
                    rule.edit_posture_floor.as_str(),
                    expected.as_str()
                ),
            ),
            None => report.push(
                "edit_boundary_rule.trigger",
                format!(
                    "rule {} trigger {} imposes no edit-posture floor and must not be a rule",
                    rule.rule_id,
                    rule.trigger_evidence_state.as_str()
                ),
            ),
        }
        if rule.effect.trim().is_empty() || rule.rationale.trim().is_empty() {
            report.push(
                "edit_boundary_rule.prose",
                format!("rule {} must carry an effect and rationale", rule.rule_id),
            );
        }
    }
    for required in [
        EvidenceState::Partial,
        EvidenceState::Stale,
        EvidenceState::Missing,
    ] {
        if !covered.contains(&required) {
            report.push(
                "packet.edit_boundary_rule_coverage",
                format!(
                    "packet must encode an edit-boundary rule for {} evidence",
                    required.as_str()
                ),
            );
        }
    }
}

fn validate_drills(report: &mut ValidationReport, packet: &M5GeneratedGovernancePacket) {
    if packet.drills.is_empty() {
        report.push(
            "packet.drills",
            "packet must declare failure/recovery drills",
        );
    }
    let mut drill_ids = BTreeSet::new();
    let mut drilled_classes = BTreeSet::new();
    let mut has_narrowed = false;
    let mut has_withheld = false;
    let mut has_edit_posture_downgrade = false;
    for artifact_drill in &packet.drills {
        if !drill_ids.insert(artifact_drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", artifact_drill.drill_id),
            );
        }
        drilled_classes.insert(artifact_drill.artifact_class);

        // Recompute the degraded outcome from the engine.
        let mut degraded = current_dimensions();
        for evidence in &mut degraded {
            if evidence.dimension == artifact_drill.exercised_dimension {
                evidence.evidence_state = artifact_drill.degraded_evidence_state;
            }
        }
        let degraded_outcome = certify_artifact_outcome(
            artifact_drill.claimed_maturity,
            artifact_drill.claimed_edit_posture,
            &degraded,
        );
        if artifact_drill.expected_degraded_verdict != degraded_outcome.verdict {
            report.push(
                "drill.degraded_verdict",
                format!(
                    "drill {} degraded verdict disagrees with the engine",
                    artifact_drill.drill_id
                ),
            );
        }
        if artifact_drill.expected_degraded_maturity != degraded_outcome.effective_maturity {
            report.push(
                "drill.degraded_maturity",
                format!(
                    "drill {} degraded maturity disagrees with the engine",
                    artifact_drill.drill_id
                ),
            );
        }
        if artifact_drill.expected_degraded_edit_posture != degraded_outcome.effective_edit_posture
        {
            report.push(
                "drill.degraded_edit_posture",
                format!(
                    "drill {} degraded edit posture disagrees with the engine",
                    artifact_drill.drill_id
                ),
            );
        }
        if degraded_outcome.verdict == RowVerdict::Certified {
            report.push(
                "drill.must_degrade",
                format!(
                    "drill {} must inject a failure that actually narrows or withholds",
                    artifact_drill.drill_id
                ),
            );
        }
        match degraded_outcome.verdict {
            RowVerdict::Narrowed => has_narrowed = true,
            RowVerdict::Withheld => has_withheld = true,
            RowVerdict::Certified => {}
        }
        if degraded_outcome.edit_posture_downgraded {
            has_edit_posture_downgrade = true;
        }
        if artifact_drill.recovers_to_verdict != RowVerdict::Certified {
            report.push(
                "drill.recovers",
                format!(
                    "drill {} must recover to certified",
                    artifact_drill.drill_id
                ),
            );
        }
        if !artifact_drill.asserts_claim_narrows_under_failure
            || !artifact_drill.asserts_recovers_after_refresh
        {
            report.push(
                "drill.assertions",
                format!(
                    "drill {} must assert it narrows under failure and recovers after refresh",
                    artifact_drill.drill_id
                ),
            );
        }
        validate_drill_steps(report, artifact_drill);
    }
    for required in ArtifactClass::ALL {
        if !drilled_classes.contains(&required) {
            report.push(
                "packet.drilled_class",
                format!("packet must drill class {}", required.as_str()),
            );
        }
    }
    if !has_narrowed {
        report.push(
            "packet.narrowed_drill",
            "packet must drill at least one narrowed verdict",
        );
    }
    if !has_withheld {
        report.push(
            "packet.withheld_drill",
            "packet must drill at least one withheld verdict",
        );
    }
    if !has_edit_posture_downgrade {
        report.push(
            "packet.edit_posture_downgrade_drill",
            "packet must drill at least one writable-boundary downgrade",
        );
    }
}

fn validate_drill_steps(report: &mut ValidationReport, artifact_drill: &ArtifactDrill) {
    if artifact_drill.steps.is_empty() {
        report.push(
            "drill.steps",
            format!("drill {} must declare steps", artifact_drill.drill_id),
        );
        return;
    }
    if artifact_drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Inject) {
        report.push(
            "drill.first_phase",
            format!(
                "drill {} must begin with an inject step",
                artifact_drill.drill_id
            ),
        );
    }
    if artifact_drill.steps.last().map(|s| s.phase) != Some(DrillPhase::Verify) {
        report.push(
            "drill.last_phase",
            format!(
                "drill {} must end with a verify step",
                artifact_drill.drill_id
            ),
        );
    }
    let has_narrow = artifact_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Narrow);
    let has_recover = artifact_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Recover);
    if !has_narrow || !has_recover {
        report.push(
            "drill.phases",
            format!(
                "drill {} must include a narrow step and a recover step",
                artifact_drill.drill_id
            ),
        );
    }
    for (index, drill_step) in artifact_drill.steps.iter().enumerate() {
        if drill_step.narration.trim().is_empty() {
            report.push(
                "drill.step_narration",
                format!(
                    "drill {} step {index} must narrate",
                    artifact_drill.drill_id
                ),
            );
        }
    }
}

fn validate_surface_bindings(report: &mut ValidationReport, packet: &M5GeneratedGovernancePacket) {
    let mut channels = BTreeSet::new();
    for surface_binding in &packet.surface_bindings {
        channels.insert(surface_binding.channel);
        if surface_binding.ingested_packet_id != packet.packet_id {
            report.push(
                "binding.packet_id",
                format!(
                    "binding for {} must ingest the packet id",
                    surface_binding.channel.as_str()
                ),
            );
        }
        if surface_binding.required_verbatim_fields.is_empty() {
            report.push(
                "binding.required_verbatim_fields",
                format!(
                    "binding for {} must name the fields it preserves verbatim",
                    surface_binding.channel.as_str()
                ),
            );
        }
        if !surface_binding.narrows_with_packet {
            report.push(
                "binding.narrows_with_packet",
                format!(
                    "binding for {} must narrow in lockstep with the packet",
                    surface_binding.channel.as_str()
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
                    surface_binding.channel.as_str()
                ),
            );
        }
    }
    for required in PublicationChannel::ALL {
        if !channels.contains(&required) {
            report.push(
                "packet.binding_coverage",
                format!("packet must bind channel {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in fixture against the frozen contract.
pub fn validate_m5_generated_governance_fixture(
    fixture: &M5GeneratedGovernanceFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != M5_GENERATED_GOVERNANCE_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != M5_GENERATED_GOVERNANCE_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }
    if fixture.fixture_id.trim().is_empty() {
        report.push("fixture.id", "fixture must carry a stable id");
    }
    if fixture.consumer_ref.trim().is_empty() {
        report.push(
            "fixture.consumer_ref",
            format!("fixture {} must cite a consumer ref", fixture.fixture_id),
        );
    }
    if fixture.notes.trim().is_empty() {
        report.push(
            "fixture.notes",
            format!("fixture {} must carry a reviewer note", fixture.fixture_id),
        );
    }

    validate_dimensions(
        &mut report,
        &format!("fixture {}", fixture.fixture_id),
        &fixture.observed_dimensions,
    );

    let outcome = certify_artifact_outcome(
        fixture.claimed_maturity,
        fixture.claimed_edit_posture,
        &fixture.observed_dimensions,
    );
    if fixture.expected_verdict != outcome.verdict {
        report.push(
            "fixture.expected_verdict",
            format!(
                "fixture {} expected verdict {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if fixture.expected_effective_maturity != outcome.effective_maturity {
        report.push(
            "fixture.expected_effective_maturity",
            format!(
                "fixture {} expected maturity {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_effective_maturity.as_str(),
                outcome.effective_maturity.as_str()
            ),
        );
    }
    if fixture.expected_edit_posture != outcome.effective_edit_posture {
        report.push(
            "fixture.expected_edit_posture",
            format!(
                "fixture {} expected edit posture {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_edit_posture.as_str(),
                outcome.effective_edit_posture.as_str()
            ),
        );
    }
    if fixture.expected_narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "fixture.expected_narrow_reason_tokens",
            format!(
                "fixture {} expected narrowing tokens disagree with the engine",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_edit_posture_downgrade_tokens != outcome.edit_posture_downgrade_tokens {
        report.push(
            "fixture.expected_edit_posture_downgrade_tokens",
            format!(
                "fixture {} expected edit-posture downgrade tokens disagree with the engine",
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
