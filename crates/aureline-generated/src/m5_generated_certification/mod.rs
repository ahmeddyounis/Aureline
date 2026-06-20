//! Promotion-grade certification of generated-artifact truth on every
//! claimed M5 scaffolded / notebook / preview / request / framework-codegen
//! profile.
//!
//! The [`m5_generated_governance`](crate::m5_generated_governance) lane
//! freezes the per-class generated-artifact contract — the seven provenance
//! dimensions every generated artifact must prove. This lane sits one level
//! above it: it is the *certification capstone* that decides whether a
//! publishable M5 profile — a scaffolded/archetype project, a notebook
//! output, a preview/runtime derivative, an API/request artifact, or
//! framework codegen — may carry the maturity its own claim-publication
//! object advertises, given the generated-artifact evidence behind it.
//!
//! Each [`CertificationRow`] binds three things together:
//!
//! - the profile's upstream **claim-publication object** (the
//!   archetype/notebook/preview/request/codegen artifact that advertises the
//!   maturity), so the certification stays aligned with what the product
//!   actually publishes;
//! - the backing generated-artifact
//!   [`ArtifactClass`](crate::m5_generated_governance::ArtifactClass) the
//!   governance lane certifies for that profile, so the two lanes cannot
//!   disagree about which class carries the profile; and
//! - the four [`CertificationDomain`]s the exit gate requires —
//!   canonical-source visibility, writable-boundary truth, regeneration
//!   path, and restore/export honesty — each grounded in a checked-in
//!   generated-artifact evidence packet.
//!
//! One [`certify_profile_outcome`] engine folds the per-domain evidence into
//! a single [`RowVerdict`](crate::m5_generated_governance::RowVerdict), an
//! effective [`ClaimMaturity`](crate::m5_generated_governance::ClaimMaturity)
//! the profile may actually publish at, and a [`PromotionDecision`] release
//! and shiproom read directly. The engine only ever narrows: a profile is
//! certified at its published maturity only when every domain is `current`;
//! partial evidence narrows it to `beta`, stale evidence to `preview`, and
//! missing evidence withholds the claim and holds promotion. A profile
//! absent from the packet is uncertified, never implicitly promotable.
//!
//! Three guardrails are frozen here:
//!
//! - **No claim may outrun its evidence.** The certified maturity starts at
//!   the published claim and is floored by every degraded domain; it can
//!   never exceed the published claim. A green happy-path workflow cannot
//!   keep a row certified while its writable-boundary or restore/export
//!   evidence is stale — the [`certify_profile_outcome`] engine narrows it.
//! - **Missing evidence fails promotion.** A profile that cannot prove
//!   canonical-source visibility, writable-boundary truth, a regeneration
//!   path, or restore/export honesty is [`PromotionDecision::Hold`], not a
//!   silently green row.
//! - **One narrowing engine.** [`certify_profile_outcome`] is the single
//!   source of truth shared by the rows, the [`CertificationFreshnessRule`]s,
//!   the [`CertificationDrill`]s, and the [`M5GeneratedCertificationFixture`]
//!   corpus, so release, support, docs, and help read one verdict instead of
//!   re-deriving staleness.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/generated/m5-generated-certification.schema.json`](../../../../schemas/generated/m5-generated-certification.schema.json)
//! - [`/docs/generated/m5-generated-certification.md`](../../../../docs/generated/m5-generated-certification.md)
//! - [`/artifacts/generated/m5-generated-certification-packet.json`](../../../../artifacts/generated/m5-generated-certification-packet.json)
//! - [`/artifacts/generated/m5-generated-certification.md`](../../../../artifacts/generated/m5-generated-certification.md)
//! - [`/fixtures/generated/m5-generated-certification/`](../../../../fixtures/generated/m5-generated-certification/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_generated_governance::{
    ArtifactClass, ClaimMaturity, DrillPhase, EvidenceState, PublicationChannel, RowVerdict,
};

/// Schema version stamped onto packets and fixtures.
pub const M5_GENERATED_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_GENERATED_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "m5_generated_certification_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_GENERATED_CERTIFICATION_FIXTURE_RECORD_KIND: &str =
    "m5_generated_certification_fixture_record";

/// Stable packet id every binding ingests.
pub const M5_GENERATED_CERTIFICATION_PACKET_ID: &str = "generated.m5_generated_certification.v1";

/// Repo-relative schema ref.
pub const M5_GENERATED_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/generated/m5-generated-certification.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_GENERATED_CERTIFICATION_DOC_REF: &str = "docs/generated/m5-generated-certification.md";

/// Repo-relative machine-readable certification proof packet.
pub const M5_GENERATED_CERTIFICATION_PACKET_REF: &str =
    "artifacts/generated/m5-generated-certification-packet.json";

/// Repo-relative reviewer certification summary.
pub const M5_GENERATED_CERTIFICATION_REPORT_REF: &str =
    "artifacts/generated/m5-generated-certification.md";

/// Repo-relative fixture directory.
pub const M5_GENERATED_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/generated/m5-generated-certification";

/// Repo-relative fixture manifest.
pub const M5_GENERATED_CERTIFICATION_FIXTURE_MANIFEST_REF: &str =
    "fixtures/generated/m5-generated-certification/manifest.yaml";

/// The canonical generated-artifact governance proof packet every row binds
/// back to, so the certification cannot float free of the per-class
/// governance lane it sits above.
pub const GOVERNANCE_EVIDENCE_REF: &str = "artifacts/generated/m5-generated-proof-packet.json";

// ---------------------------------------------------------------------------
// Vocabulary.
// ---------------------------------------------------------------------------

/// A claimed publishable M5 generated-artifact profile under certification.
///
/// These are the five profiles the exit gate names — they map onto the
/// matching [`ArtifactClass`](crate::m5_generated_governance::ArtifactClass)
/// the governance lane certifies, but unlike the generic class they each
/// carry a real upstream claim-publication object the product ships.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedProfile {
    /// A project scaffolded from a template / archetype health bundle.
    ScaffoldedProject,
    /// A notebook output captured from a kernel run.
    NotebookOutput,
    /// A preview / runtime derivative built from source.
    PreviewDerivative,
    /// An API / request artifact captured from a request run.
    RequestArtifact,
    /// Code emitted by a framework code generator or codemod.
    FrameworkCodegen,
}

impl CertifiedProfile {
    /// Every claimed profile in canonical order.
    pub const ALL: [Self; 5] = [
        Self::ScaffoldedProject,
        Self::NotebookOutput,
        Self::PreviewDerivative,
        Self::RequestArtifact,
        Self::FrameworkCodegen,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScaffoldedProject => "scaffolded_project",
            Self::NotebookOutput => "notebook_output",
            Self::PreviewDerivative => "preview_derivative",
            Self::RequestArtifact => "request_artifact",
            Self::FrameworkCodegen => "framework_codegen",
        }
    }

    /// Review-safe label for the profile.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ScaffoldedProject => "Scaffolded project",
            Self::NotebookOutput => "Notebook output",
            Self::PreviewDerivative => "Preview derivative",
            Self::RequestArtifact => "Request artifact",
            Self::FrameworkCodegen => "Framework codegen",
        }
    }

    /// The backing generated-artifact class the governance lane certifies for
    /// this profile, so the two lanes stay aligned on which class carries it.
    pub const fn backing_artifact_class(self) -> ArtifactClass {
        match self {
            Self::ScaffoldedProject => ArtifactClass::ScaffoldedProject,
            Self::NotebookOutput => ArtifactClass::NotebookOutput,
            Self::PreviewDerivative => ArtifactClass::PreviewDerivative,
            Self::RequestArtifact => ArtifactClass::RequestArtifact,
            Self::FrameworkCodegen => ArtifactClass::FrameworkCodegen,
        }
    }

    /// The repo-relative upstream claim-publication object that advertises
    /// this profile's maturity. The certification row may never publish wider
    /// than the generated-artifact evidence behind this object.
    pub const fn claim_publication_ref(self) -> &'static str {
        match self {
            Self::ScaffoldedProject => {
                "artifacts/templates/m5/certify_the_template_registry_scaffold_planner_framework_packs_and_archetype_health_bundles_on_every_claimed_m5_profile.md"
            }
            Self::NotebookOutput => {
                "artifacts/notebook/m5/seed_notebook_round_trip_fixtures_heavy_output_corpora_and_the_canonical_notebook_support_packet.json"
            }
            Self::PreviewDerivative => "artifacts/ecosystem/m5/m5-publish-preview.json",
            Self::RequestArtifact => {
                "artifacts/data/m5/certify-api-collections-graphql-freshness-request-origin-truth-and-persisted-operation-continuity-across-request-profiles.json"
            }
            Self::FrameworkCodegen => {
                "artifacts/language/m4/framework_migration_import_truth_packet.json"
            }
        }
    }
}

/// One generated-artifact truth domain a claimed profile must prove before it
/// may promote. The four domains are the exit-gate anchor: a profile may not
/// publish its claimed maturity unless all four are `current`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDomain {
    /// The profile's derived bytes declare and link to the canonical source
    /// they derive from, so a generated artifact is never presented as
    /// authoritative source.
    CanonicalSourceVisibility,
    /// The profile's writable-boundary posture is enforced, so a direct edit
    /// across a canonical-source boundary is blocked or escalated rather than
    /// silently applied.
    WritableBoundaryTruth,
    /// The profile can be regenerated from its canonical source through a
    /// declared, reviewable route.
    RegenerationPath,
    /// The profile's local-history, restore, and support-export packets state
    /// exactly what was captured, omitted, or rederived, so restore never
    /// implies ordinary full-source history for a derived artifact.
    RestoreExportHonesty,
}

impl CertificationDomain {
    /// Every required domain in canonical order.
    pub const ALL: [Self; 4] = [
        Self::CanonicalSourceVisibility,
        Self::WritableBoundaryTruth,
        Self::RegenerationPath,
        Self::RestoreExportHonesty,
    ];

    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSourceVisibility => "canonical_source_visibility",
            Self::WritableBoundaryTruth => "writable_boundary_truth",
            Self::RegenerationPath => "regeneration_path",
            Self::RestoreExportHonesty => "restore_export_honesty",
        }
    }

    /// The checked-in generated-artifact evidence packets that ground this
    /// domain, so each domain is anchored in real artifacts rather than prose.
    pub fn evidence_packet_refs(self) -> Vec<&'static str> {
        match self {
            Self::CanonicalSourceVisibility => vec![
                "artifacts/generated/m5-generated-proof-packet.json",
                "artifacts/generated/generated-artifact-descriptor-packet.json",
            ],
            Self::WritableBoundaryTruth => vec![
                "artifacts/generated/write-boundary-packet.json",
                "artifacts/generated/mutation-guardrails-packet.json",
            ],
            Self::RegenerationPath => vec!["artifacts/generated/regeneration-plan-packet.json"],
            Self::RestoreExportHonesty => {
                vec!["artifacts/generated/generated-timeline-packet.json"]
            }
        }
    }

    /// Review-safe rationale for what the domain proves.
    pub const fn rationale(self) -> &'static str {
        match self {
            Self::CanonicalSourceVisibility => {
                "The profile's generated bytes declare and link to the canonical source they derive from, so search, review, AI, save, and export surfaces never present a derived file as authoritative source."
            }
            Self::WritableBoundaryTruth => {
                "The profile's writable-boundary posture is enforced, so a direct edit across a canonical-source boundary is blocked by default or escalates through a visible reviewed override rather than landing silently."
            }
            Self::RegenerationPath => {
                "The profile can be rebuilt from its canonical source through a declared, reviewable regeneration route with a disclosed side-effect and rollback boundary, so a derived file is rederived rather than hand-patched."
            }
            Self::RestoreExportHonesty => {
                "The profile's local-history, restore, and support-export packets state exactly what was captured, omitted, or rederived, so a restore or export never implies ordinary full-source history for a derived artifact."
            }
        }
    }
}

/// The promotion decision the certification reaches for one profile. Release
/// and shiproom read this directly to fail or pass promotion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionDecision {
    /// Every domain is current; the profile promotes at its published claim.
    Promote,
    /// One or more domains degraded; the profile promotes only at the
    /// narrowed maturity the engine computed, never the published claim.
    PromoteNarrowed,
    /// A required domain cannot be proven; promotion is held.
    Hold,
}

impl PromotionDecision {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promote => "promote",
            Self::PromoteNarrowed => "promote_narrowed",
            Self::Hold => "hold",
        }
    }

    /// True when the decision holds promotion of the published claim, because
    /// the row is withheld.
    pub const fn holds_promotion(self) -> bool {
        matches!(self, Self::Hold)
    }
}

/// The failure class a certification drill injects into one domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DomainFailureClass {
    /// The profile loses its declared link to the canonical source.
    CanonicalSourceInvisible,
    /// The writable-boundary posture is no longer enforced.
    WritableBoundaryUnproven,
    /// The regeneration route that rebuilds the profile is broken.
    RegenerationPathBroken,
    /// The restore / export packets no longer state what was captured or
    /// omitted.
    RestoreExportDishonest,
}

impl DomainFailureClass {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSourceInvisible => "canonical_source_invisible",
            Self::WritableBoundaryUnproven => "writable_boundary_unproven",
            Self::RegenerationPathBroken => "regeneration_path_broken",
            Self::RestoreExportDishonest => "restore_export_dishonest",
        }
    }

    /// The domain this failure degrades.
    pub const fn domain(self) -> CertificationDomain {
        match self {
            Self::CanonicalSourceInvisible => CertificationDomain::CanonicalSourceVisibility,
            Self::WritableBoundaryUnproven => CertificationDomain::WritableBoundaryTruth,
            Self::RegenerationPathBroken => CertificationDomain::RegenerationPath,
            Self::RestoreExportDishonest => CertificationDomain::RestoreExportHonesty,
        }
    }
}

// ---------------------------------------------------------------------------
// Narrowing engine: the single source of truth for the verdict.
// ---------------------------------------------------------------------------

/// One domain's evidence on one profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DomainEvidence {
    /// Certification domain being evidenced.
    pub domain: CertificationDomain,
    /// State of the evidence backing this domain.
    pub evidence_state: EvidenceState,
    /// Generated-artifact packets that prove this domain.
    pub evidence_refs: Vec<String>,
    /// Review-safe rationale for the evidence.
    pub rationale: String,
}

/// The computed outcome of certifying one profile against its evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileOutcome {
    /// The narrowest maturity the profile may publish at.
    pub certified_maturity: ClaimMaturity,
    /// The verdict the engine reaches.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its published maturity.
    pub narrowed: bool,
    /// The promotion decision release and shiproom read.
    pub promotion_decision: PromotionDecision,
    /// Stable tokens naming every domain that forced narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Domains whose evidence is stale or missing, in stable order.
    pub stale_or_missing_domain_tokens: Vec<String>,
}

/// Certifies one profile's published claim against its per-domain evidence.
///
/// This is the canonical narrowing engine the whole packet, every freshness
/// rule, every drill, and every fixture share. The certified maturity starts
/// at the published claim and is floored by every degraded domain; the
/// narrowest (highest-severity) result wins. A withdrawn maturity is
/// [`RowVerdict::Withheld`] and [`PromotionDecision::Hold`]; any other
/// maturity below the published claim is [`RowVerdict::Narrowed`] and
/// [`PromotionDecision::PromoteNarrowed`]; otherwise the profile is
/// [`RowVerdict::Certified`] and [`PromotionDecision::Promote`]. The engine
/// only ever narrows — it never widens a profile above its published claim.
pub fn certify_profile_outcome(
    published_claim_maturity: ClaimMaturity,
    domains: &[DomainEvidence],
) -> ProfileOutcome {
    let mut certified_maturity = published_claim_maturity;
    let mut narrow_reason_tokens = Vec::new();
    let mut stale_or_missing = Vec::new();

    for evidence in domains {
        if let Some(floor) = evidence.evidence_state.qualification_floor() {
            if floor.severity() > certified_maturity.severity() {
                certified_maturity = floor;
            }
            narrow_reason_tokens.push(format!(
                "{}_{}",
                evidence.domain.as_str(),
                evidence.evidence_state.as_str()
            ));
        }
        if evidence.evidence_state.is_stale_or_missing() {
            stale_or_missing.push(evidence.domain.as_str().to_owned());
        }
    }

    narrow_reason_tokens.sort();
    narrow_reason_tokens.dedup();
    stale_or_missing.sort();
    stale_or_missing.dedup();

    let verdict = if certified_maturity == ClaimMaturity::Withdrawn {
        RowVerdict::Withheld
    } else if certified_maturity.severity() > published_claim_maturity.severity() {
        RowVerdict::Narrowed
    } else {
        RowVerdict::Certified
    };

    let promotion_decision = match verdict {
        RowVerdict::Certified => PromotionDecision::Promote,
        RowVerdict::Narrowed => PromotionDecision::PromoteNarrowed,
        RowVerdict::Withheld => PromotionDecision::Hold,
    };

    ProfileOutcome {
        certified_maturity,
        verdict,
        narrowed: verdict == RowVerdict::Narrowed,
        promotion_decision,
        narrow_reason_tokens,
        stale_or_missing_domain_tokens: stale_or_missing,
    }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One certification row: a claimed profile, its claim-publication object,
/// its backing class, its per-domain evidence, and the engine outcome stamped
/// onto it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed publishable profile.
    pub profile: CertifiedProfile,
    /// Review-safe label for the profile.
    pub profile_label: String,
    /// The upstream claim-publication object this row certifies.
    pub claim_publication_ref: String,
    /// The backing generated-artifact class the governance lane certifies.
    pub backing_artifact_class: ArtifactClass,
    /// The governance proof packet this row binds back to.
    pub governance_evidence_ref: String,
    /// Maturity the claim-publication object advertises.
    pub published_claim_maturity: ClaimMaturity,
    /// Per-domain evidence, one entry per required domain.
    pub domains: Vec<DomainEvidence>,
    /// Effective maturity after narrowing.
    pub certified_maturity: ClaimMaturity,
    /// Engine verdict.
    pub verdict: RowVerdict,
    /// True when the claim narrowed below its published maturity.
    pub narrowed: bool,
    /// Promotion decision release and shiproom read.
    pub promotion_decision: PromotionDecision,
    /// Stable tokens naming every domain that forced narrowing.
    pub narrow_reason_tokens: Vec<String>,
    /// Domains whose evidence is stale or missing.
    pub stale_or_missing_domain_tokens: Vec<String>,
    /// Review-safe "why this certified" inspector line.
    pub why_certified: String,
    /// Union of the per-domain evidence refs this row composes.
    pub supporting_evidence_refs: Vec<String>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One automatic maturity-narrowing rule keyed by evidence state. The floor is
/// computed from [`EvidenceState::qualification_floor`], so the rule set can
/// never drift from the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationFreshnessRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Evidence state that triggers the rule.
    pub trigger_evidence_state: EvidenceState,
    /// Maturity floor the rule imposes.
    pub maturity_floor: ClaimMaturity,
    /// The promotion decision this floor implies.
    pub promotion_decision: PromotionDecision,
    /// User-visible effect on the claim.
    pub effect: String,
    /// Review-safe rationale.
    pub rationale: String,
}

/// One ordered step inside a certification drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Maturity observed at this step.
    pub observed_maturity: ClaimMaturity,
    /// Promotion decision observed at this step.
    pub observed_promotion_decision: PromotionDecision,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One failure / recovery drill walking a profile from an injected domain
/// failure through narrowing and back to recovery.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Profile exercised by the drill.
    pub profile: CertifiedProfile,
    /// Domain whose evidence the drill degrades.
    pub exercised_domain: CertificationDomain,
    /// Failure class the drill injects.
    pub failure_class: DomainFailureClass,
    /// Evidence state the domain degrades to.
    pub degraded_evidence_state: EvidenceState,
    /// Maturity published before the failure.
    pub published_claim_maturity: ClaimMaturity,
    /// Verdict expected while the failure is active.
    pub expected_degraded_verdict: RowVerdict,
    /// Maturity expected while the failure is active.
    pub expected_degraded_maturity: ClaimMaturity,
    /// Promotion decision expected while the failure is active.
    pub expected_degraded_promotion_decision: PromotionDecision,
    /// Verdict expected once the evidence is refreshed.
    pub recovers_to_verdict: RowVerdict,
    /// Ordered drill steps.
    pub steps: Vec<CertificationDrillStep>,
    /// True when the drill proves the claim narrows or holds under the failure.
    pub asserts_claim_narrows_under_failure: bool,
    /// True when the drill proves the claim recovers after refresh.
    pub asserts_recovers_after_refresh: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// One binding proving a publication channel ingests this packet rather than
/// re-deriving certification truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSurfaceBinding {
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
pub struct CertificationSourceContractRefs {
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

/// Top-level packet certifying generated-artifact truth on every claimed M5
/// publishable profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeneratedCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: CertificationSourceContractRefs,
    /// Required certification domains.
    pub certified_domains: Vec<CertificationDomain>,
    /// Generated-artifact evidence packets this certification composes.
    pub evidence_packet_refs: Vec<String>,
    /// Certification rows, one per claimed profile.
    pub rows: Vec<CertificationRow>,
    /// Automatic maturity-narrowing rules over evidence states.
    pub freshness_rules: Vec<CertificationFreshnessRule>,
    /// Failure / recovery drills.
    pub drills: Vec<CertificationDrill>,
    /// Publication-channel bindings.
    pub surface_bindings: Vec<CertificationSurfaceBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a profile and an observed evidence configuration to the
/// expected verdict and promotion decision, proving the canonical narrowing
/// behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5GeneratedCertificationFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Profile under test.
    pub profile: CertifiedProfile,
    /// Maturity published before narrowing.
    pub published_claim_maturity: ClaimMaturity,
    /// Observed per-domain evidence.
    pub observed_domains: Vec<DomainEvidence>,
    /// Expected verdict.
    pub expected_verdict: RowVerdict,
    /// Expected effective maturity.
    pub expected_certified_maturity: ClaimMaturity,
    /// Expected promotion decision.
    pub expected_promotion_decision: PromotionDecision,
    /// Expected narrowing tokens.
    pub expected_narrow_reason_tokens: Vec<String>,
    /// One consumer that quotes this profile.
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
        writeln!(f, "m5 generated certification validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Copy line.
// ---------------------------------------------------------------------------

/// Renders a redaction-safe one-line certification summary for a row, reused
/// by docs and the help "why this certified" inspector.
pub fn certification_copy_line(row: &CertificationRow) -> String {
    let posture = match row.promotion_decision {
        PromotionDecision::Promote => format!(
            "promotes at {} — all four generated-artifact domains current",
            row.certified_maturity.as_str()
        ),
        PromotionDecision::PromoteNarrowed => format!(
            "narrowed from {} to {} — {}",
            row.published_claim_maturity.as_str(),
            row.certified_maturity.as_str(),
            row.narrow_reason_tokens.join(", ")
        ),
        PromotionDecision::Hold => format!(
            "promotion held — {} cannot be proven",
            row.stale_or_missing_domain_tokens.join(", ")
        ),
    };
    format!("{} {}", row.profile_label, posture)
}

// ---------------------------------------------------------------------------
// Seed helpers.
// ---------------------------------------------------------------------------

const REQUIRED_VERBATIM_FIELDS: [&str; 7] = [
    "row_id",
    "profile",
    "published_claim_maturity",
    "certified_maturity",
    "verdict",
    "promotion_decision",
    "narrow_reason_tokens",
];

/// Builds the four fully-current domains for a healthy row.
fn current_domains() -> Vec<DomainEvidence> {
    CertificationDomain::ALL
        .into_iter()
        .map(|domain| DomainEvidence {
            domain,
            evidence_state: EvidenceState::Current,
            evidence_refs: domain
                .evidence_packet_refs()
                .into_iter()
                .map(str::to_owned)
                .collect(),
            rationale: domain.rationale().to_owned(),
        })
        .collect()
}

/// Builds the four domains with one degraded to a given evidence state.
fn degraded_domains(domain: CertificationDomain, state: EvidenceState) -> Vec<DomainEvidence> {
    let mut domains = current_domains();
    for evidence in &mut domains {
        if evidence.domain == domain {
            evidence.evidence_state = state;
        }
    }
    domains
}

fn supporting_evidence_refs(domains: &[DomainEvidence]) -> Vec<String> {
    let mut refs: BTreeSet<String> = BTreeSet::new();
    for domain in domains {
        for reference in &domain.evidence_refs {
            refs.insert(reference.clone());
        }
    }
    refs.into_iter().collect()
}

fn row(
    profile: CertifiedProfile,
    published_claim_maturity: ClaimMaturity,
    why_certified: &str,
    consumer_refs: &[&str],
    notes: &str,
) -> CertificationRow {
    let domains = current_domains();
    let outcome = certify_profile_outcome(published_claim_maturity, &domains);
    let supporting_evidence_refs = supporting_evidence_refs(&domains);
    CertificationRow {
        row_id: format!("generated.certification.{}", profile.as_str()),
        profile,
        profile_label: profile.label().to_owned(),
        claim_publication_ref: profile.claim_publication_ref().to_owned(),
        backing_artifact_class: profile.backing_artifact_class(),
        governance_evidence_ref: GOVERNANCE_EVIDENCE_REF.to_owned(),
        published_claim_maturity,
        domains,
        certified_maturity: outcome.certified_maturity,
        verdict: outcome.verdict,
        narrowed: outcome.narrowed,
        promotion_decision: outcome.promotion_decision,
        narrow_reason_tokens: outcome.narrow_reason_tokens,
        stale_or_missing_domain_tokens: outcome.stale_or_missing_domain_tokens,
        why_certified: why_certified.to_owned(),
        supporting_evidence_refs,
        consumer_refs: consumer_refs.iter().map(|s| (*s).to_owned()).collect(),
        notes: notes.to_owned(),
    }
}

fn freshness_rule(
    rule_id: &str,
    trigger: EvidenceState,
    promotion_decision: PromotionDecision,
    effect: &str,
    rationale: &str,
) -> CertificationFreshnessRule {
    CertificationFreshnessRule {
        rule_id: rule_id.to_owned(),
        trigger_evidence_state: trigger,
        maturity_floor: trigger
            .qualification_floor()
            .expect("freshness rules only encode triggers that impose a maturity floor"),
        promotion_decision,
        effect: effect.to_owned(),
        rationale: rationale.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    observed_maturity: ClaimMaturity,
    observed_promotion_decision: PromotionDecision,
    narration: &str,
) -> CertificationDrillStep {
    CertificationDrillStep {
        phase,
        observed_maturity,
        observed_promotion_decision,
        narration: narration.to_owned(),
    }
}

fn drill(
    drill_id: &str,
    title: &str,
    failure_class: DomainFailureClass,
    degraded_evidence_state: EvidenceState,
    published_claim_maturity: ClaimMaturity,
    profile: CertifiedProfile,
    notes: &str,
) -> CertificationDrill {
    let exercised_domain = failure_class.domain();
    // The degraded outcome is computed from the same engine the rows use, so a
    // drill can never disagree with the certification.
    let degraded = degraded_domains(exercised_domain, degraded_evidence_state);
    let degraded_outcome = certify_profile_outcome(published_claim_maturity, &degraded);
    let healthy = certify_profile_outcome(published_claim_maturity, &current_domains());
    let steps = vec![
        step(
            DrillPhase::Inject,
            published_claim_maturity,
            healthy.promotion_decision,
            "Inject the domain failure into the backing generated-artifact evidence.",
        ),
        step(
            DrillPhase::Observe,
            published_claim_maturity,
            healthy.promotion_decision,
            "The certification observes the degraded domain evidence state.",
        ),
        step(
            DrillPhase::Narrow,
            degraded_outcome.certified_maturity,
            degraded_outcome.promotion_decision,
            "The certified maturity and promotion decision narrow under the failure.",
        ),
        step(
            DrillPhase::Refresh,
            degraded_outcome.certified_maturity,
            degraded_outcome.promotion_decision,
            "The backing generated-artifact evidence is refreshed to current.",
        ),
        step(
            DrillPhase::Recover,
            published_claim_maturity,
            healthy.promotion_decision,
            "The certified maturity recovers to the published claim.",
        ),
        step(
            DrillPhase::Verify,
            published_claim_maturity,
            healthy.promotion_decision,
            "The recovered outcome is verified against the engine.",
        ),
    ];
    CertificationDrill {
        drill_id: drill_id.to_owned(),
        title: title.to_owned(),
        profile,
        exercised_domain,
        failure_class,
        degraded_evidence_state,
        published_claim_maturity,
        expected_degraded_verdict: degraded_outcome.verdict,
        expected_degraded_maturity: degraded_outcome.certified_maturity,
        expected_degraded_promotion_decision: degraded_outcome.promotion_decision,
        recovers_to_verdict: RowVerdict::Certified,
        steps,
        asserts_claim_narrows_under_failure: true,
        asserts_recovers_after_refresh: true,
        notes: notes.to_owned(),
    }
}

fn binding(
    channel: PublicationChannel,
    consumer_ref: &str,
    summary: &str,
) -> CertificationSurfaceBinding {
    CertificationSurfaceBinding {
        channel,
        consumer_ref: consumer_ref.to_owned(),
        ingested_packet_id: M5_GENERATED_CERTIFICATION_PACKET_ID.to_owned(),
        required_verbatim_fields: REQUIRED_VERBATIM_FIELDS
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrows_with_packet: true,
        summary: summary.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in generated-artifact certification packet this lane
/// freezes.
pub fn seeded_m5_generated_certification_packet() -> M5GeneratedCertificationPacket {
    let rows = vec![
        row(
            CertifiedProfile::ScaffoldedProject,
            ClaimMaturity::Stable,
            "A scaffolded project promotes at stable because its canonical source is visible, its writable boundary is enforced, it regenerates from its template, and its restore/export packets are honest.",
            &[
                "crates/aureline-scaffold/src/lib.rs",
                "crates/aureline-release/src/seed_the_m5_certified_archetype_health_bundle_matrix_and_regression_guardrails/mod.rs",
            ],
            "The scaffolded-project profile owns its bytes after generation, so a direct edit is allowed only while canonical-source and writable-boundary evidence stay current.",
        ),
        row(
            CertifiedProfile::NotebookOutput,
            ClaimMaturity::Beta,
            "A notebook output promotes at beta because its kernel-derived bytes link to the canonical notebook source, its outputs are regenerate-first, and its support packet states what was captured or omitted.",
            &[
                "crates/aureline-notebook/src/lib.rs",
                "crates/aureline-release/src/notebook_and_data_rich_surface_qualification/mod.rs",
            ],
            "Notebook outputs are derived bytes; restore/export honesty is the load-bearing domain, because a captured output must never imply full-source history.",
        ),
        row(
            CertifiedProfile::PreviewDerivative,
            ClaimMaturity::Beta,
            "A preview derivative promotes at beta because it is rebuilt from source on demand, its writable boundary blocks direct edits, and its history packets disclose that the bytes are regenerated.",
            &[
                "crates/aureline-preview/src/lib.rs",
                "crates/aureline-release/src/preview_designer_publish_surface_qualification/mod.rs",
            ],
            "Preview derivatives are regenerate-only; the regeneration-path domain is load-bearing, because a stale preview must be rebuilt rather than hand-edited.",
        ),
        row(
            CertifiedProfile::RequestArtifact,
            ClaimMaturity::Beta,
            "A request artifact promotes at beta because it links to its persisted operation source, its writable boundary escalates direct edits, and its history is export-safe.",
            &[
                "crates/aureline-requests/src/lib.rs",
                "crates/aureline-release/src/implement_qualification_matrix_and_claim_scope_export_packets_for_support_shiproom_docs_and_partner_review_with_row_level_stale_retest_needed_truth/mod.rs",
            ],
            "Request artifacts carry redaction-sensitive captures; writable-boundary truth gates whether a captured request can be hand-edited across its persisted-operation boundary.",
        ),
        row(
            CertifiedProfile::FrameworkCodegen,
            ClaimMaturity::Beta,
            "Framework codegen promotes at beta because each emitted file declares its generator, links to its source sibling, regenerates through the framework pack, and discloses divergence in history.",
            &[
                "crates/aureline-generated/src/write_boundary/mod.rs",
                "crates/aureline-release/src/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs/mod.rs",
            ],
            "Framework codegen is the canonical generated-then-edited case; canonical-source visibility decides whether an edit to an emitted file survives the next regeneration.",
        ),
    ];

    let freshness_rules = vec![
        freshness_rule(
            "certification.freshness.partial",
            EvidenceState::Partial,
            PromotionDecision::PromoteNarrowed,
            "Partial domain evidence narrows the certified maturity to beta and promotes only at beta.",
            "A domain proven for only part of its scope cannot back the full published claim, so the profile promotes at the narrowed maturity instead of the published one.",
        ),
        freshness_rule(
            "certification.freshness.stale",
            EvidenceState::Stale,
            PromotionDecision::PromoteNarrowed,
            "Stale domain evidence narrows the certified maturity to preview and promotes only at preview.",
            "Evidence past its freshness window can no longer prove the profile still holds, so the profile promotes at preview rather than its published claim.",
        ),
        freshness_rule(
            "certification.freshness.missing",
            EvidenceState::Missing,
            PromotionDecision::Hold,
            "Missing domain evidence withholds the claim and holds promotion.",
            "A profile that cannot prove one of its four generated-artifact domains has no certified claim, so promotion is held rather than allowed on a happy path.",
        ),
    ];

    let drills = vec![
        drill(
            "certification.drill.scaffolded_project_canonical_source_partial",
            "Scaffolded project loses part of its canonical-source linkage",
            DomainFailureClass::CanonicalSourceInvisible,
            EvidenceState::Partial,
            ClaimMaturity::Stable,
            CertifiedProfile::ScaffoldedProject,
            "Partial canonical-source visibility narrows the stable scaffolded claim to beta and promotes only at beta until the linkage is refreshed.",
        ),
        drill(
            "certification.drill.notebook_output_restore_export_stale",
            "Notebook output restore/export evidence ages out",
            DomainFailureClass::RestoreExportDishonest,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            CertifiedProfile::NotebookOutput,
            "Stale restore/export honesty narrows the notebook claim to preview, because a notebook capture can no longer prove what its history omitted.",
        ),
        drill(
            "certification.drill.preview_derivative_regeneration_missing",
            "Preview derivative loses its regeneration route",
            DomainFailureClass::RegenerationPathBroken,
            EvidenceState::Missing,
            ClaimMaturity::Beta,
            CertifiedProfile::PreviewDerivative,
            "A preview derivative with no regeneration route is withheld and held from promotion, because a regenerate-only artifact that cannot be rebuilt has no claim.",
        ),
        drill(
            "certification.drill.request_artifact_writable_boundary_stale",
            "Request artifact writable-boundary evidence ages out",
            DomainFailureClass::WritableBoundaryUnproven,
            EvidenceState::Stale,
            ClaimMaturity::Beta,
            CertifiedProfile::RequestArtifact,
            "Stale writable-boundary truth narrows the request claim to preview, because an unenforced boundary can no longer prove a captured request stays inside its persisted-operation boundary.",
        ),
        drill(
            "certification.drill.framework_codegen_canonical_source_missing",
            "Framework codegen loses its source-sibling linkage",
            DomainFailureClass::CanonicalSourceInvisible,
            EvidenceState::Missing,
            ClaimMaturity::Beta,
            CertifiedProfile::FrameworkCodegen,
            "Framework codegen with no canonical-source linkage is withheld and held from promotion, because an emitted file with no source cannot prove an edit survives regeneration.",
        ),
    ];

    let surface_bindings = vec![
        binding(
            PublicationChannel::ReleaseShiproom,
            "crates/aureline-release/src/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix/mod.rs",
            "Release / shiproom holds promotion for any profile whose promotion decision is hold and publishes the narrowed maturity for any narrowed profile.",
        ),
        binding(
            PublicationChannel::SupportExport,
            "crates/aureline-support/src/lib.rs",
            "Support export re-exports the verdict, certified maturity, promotion decision, and narrowing tokens with no raw generated bytes, credentials, or generator payloads.",
        ),
        binding(
            PublicationChannel::Docs,
            M5_GENERATED_CERTIFICATION_DOC_REF,
            "Docs quote the certified domains, freshness rules, and per-profile verdicts verbatim from this packet.",
        ),
        binding(
            PublicationChannel::Help,
            "crates/aureline-generated/src/m5_generated_certification/mod.rs",
            "The help why-this-certified inspector reuses the same vocabulary through certification_copy_line.",
        ),
    ];

    let evidence_packet_refs = {
        let mut refs: BTreeSet<String> = BTreeSet::new();
        for domain in CertificationDomain::ALL {
            for reference in domain.evidence_packet_refs() {
                refs.insert(reference.to_owned());
            }
        }
        refs.insert(GOVERNANCE_EVIDENCE_REF.to_owned());
        refs.into_iter().collect()
    };

    M5GeneratedCertificationPacket {
        record_kind: M5_GENERATED_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_GENERATED_CERTIFICATION_SCHEMA_VERSION,
        packet_id: M5_GENERATED_CERTIFICATION_PACKET_ID.to_owned(),
        title: "M5 generated-artifact certification".to_owned(),
        source_contract_refs: CertificationSourceContractRefs {
            doc_ref: M5_GENERATED_CERTIFICATION_DOC_REF.to_owned(),
            schema_ref: M5_GENERATED_CERTIFICATION_SCHEMA_REF.to_owned(),
            packet_ref: M5_GENERATED_CERTIFICATION_PACKET_REF.to_owned(),
            report_ref: M5_GENERATED_CERTIFICATION_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_GENERATED_CERTIFICATION_FIXTURE_MANIFEST_REF.to_owned(),
        },
        certified_domains: CertificationDomain::ALL.to_vec(),
        evidence_packet_refs,
        rows,
        freshness_rules,
        drills,
        surface_bindings,
        invariants: vec![
            "A profile is certified at its published maturity only when all four generated-artifact domains are current.".to_owned(),
            "The certified maturity never exceeds the published claim; the engine only narrows.".to_owned(),
            "Missing domain evidence withholds the claim and holds promotion rather than leaving a green row.".to_owned(),
            "Every certification row binds an upstream claim-publication object, a backing generated-artifact class, and the governance proof packet.".to_owned(),
            "Release, support, docs, and help read one verdict and promotion decision instead of re-deriving staleness.".to_owned(),
        ],
    }
}

/// Returns the checked-in certification fixture corpus.
pub fn seeded_m5_generated_certification_fixtures() -> Vec<M5GeneratedCertificationFixture> {
    let mut fixtures = Vec::new();

    // One certified fixture per profile at its published claim.
    for (profile, published, consumer) in [
        (
            CertifiedProfile::ScaffoldedProject,
            ClaimMaturity::Stable,
            "crates/aureline-scaffold/src/lib.rs",
        ),
        (
            CertifiedProfile::NotebookOutput,
            ClaimMaturity::Beta,
            "crates/aureline-notebook/src/lib.rs",
        ),
        (
            CertifiedProfile::PreviewDerivative,
            ClaimMaturity::Beta,
            "crates/aureline-preview/src/lib.rs",
        ),
        (
            CertifiedProfile::RequestArtifact,
            ClaimMaturity::Beta,
            "crates/aureline-requests/src/lib.rs",
        ),
        (
            CertifiedProfile::FrameworkCodegen,
            ClaimMaturity::Beta,
            "crates/aureline-generated/src/write_boundary/mod.rs",
        ),
    ] {
        fixtures.push(fixture(
            &format!("fixture.m5_generated_certification.{}_certified", profile.as_str()),
            profile,
            published,
            current_domains(),
            consumer,
            "A fully current profile certifies at its published maturity and promotes with no narrowing tokens.",
        ));
    }

    // Degraded fixtures covering narrowed, withheld, and held promotion.
    fixtures.push(fixture(
        "fixture.m5_generated_certification.scaffolded_project_canonical_source_partial",
        CertifiedProfile::ScaffoldedProject,
        ClaimMaturity::Stable,
        degraded_domains(
            CertificationDomain::CanonicalSourceVisibility,
            EvidenceState::Partial,
        ),
        "crates/aureline-scaffold/src/lib.rs",
        "Partial canonical-source visibility narrows the stable scaffolded claim to beta and promotes only at beta.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_certification.request_artifact_writable_boundary_stale",
        CertifiedProfile::RequestArtifact,
        ClaimMaturity::Beta,
        degraded_domains(
            CertificationDomain::WritableBoundaryTruth,
            EvidenceState::Stale,
        ),
        "crates/aureline-requests/src/lib.rs",
        "Stale writable-boundary truth narrows the request claim to preview.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_certification.notebook_output_restore_export_stale",
        CertifiedProfile::NotebookOutput,
        ClaimMaturity::Beta,
        degraded_domains(
            CertificationDomain::RestoreExportHonesty,
            EvidenceState::Stale,
        ),
        "crates/aureline-notebook/src/lib.rs",
        "Stale restore/export honesty narrows the notebook claim to preview.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_certification.preview_derivative_regeneration_missing",
        CertifiedProfile::PreviewDerivative,
        ClaimMaturity::Beta,
        degraded_domains(
            CertificationDomain::RegenerationPath,
            EvidenceState::Missing,
        ),
        "crates/aureline-preview/src/lib.rs",
        "A preview derivative with no regeneration route is withheld and held from promotion.",
    ));
    fixtures.push(fixture(
        "fixture.m5_generated_certification.framework_codegen_canonical_source_missing",
        CertifiedProfile::FrameworkCodegen,
        ClaimMaturity::Beta,
        degraded_domains(
            CertificationDomain::CanonicalSourceVisibility,
            EvidenceState::Missing,
        ),
        "crates/aureline-generated/src/write_boundary/mod.rs",
        "Framework codegen with no canonical-source linkage is withheld and held from promotion.",
    ));

    fixtures
}

fn fixture(
    fixture_id: &str,
    profile: CertifiedProfile,
    published_claim_maturity: ClaimMaturity,
    observed_domains: Vec<DomainEvidence>,
    consumer_ref: &str,
    notes: &str,
) -> M5GeneratedCertificationFixture {
    let outcome = certify_profile_outcome(published_claim_maturity, &observed_domains);
    M5GeneratedCertificationFixture {
        record_kind: M5_GENERATED_CERTIFICATION_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: M5_GENERATED_CERTIFICATION_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        profile,
        published_claim_maturity,
        observed_domains,
        expected_verdict: outcome.verdict,
        expected_certified_maturity: outcome.certified_maturity,
        expected_promotion_decision: outcome.promotion_decision,
        expected_narrow_reason_tokens: outcome.narrow_reason_tokens,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the checked-in certification packet against the frozen contract.
pub fn validate_m5_generated_certification_packet(
    packet: &M5GeneratedCertificationPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_GENERATED_CERTIFICATION_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != M5_GENERATED_CERTIFICATION_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.packet_id != M5_GENERATED_CERTIFICATION_PACKET_ID {
        report.push("packet.packet_id", "packet_id drifted from the frozen id");
    }
    if packet.title.trim().is_empty() {
        report.push("packet.title", "packet must carry a title");
    }
    if packet.source_contract_refs.doc_ref != M5_GENERATED_CERTIFICATION_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != M5_GENERATED_CERTIFICATION_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen schema",
        );
    }
    if packet.source_contract_refs.packet_ref != M5_GENERATED_CERTIFICATION_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != M5_GENERATED_CERTIFICATION_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != M5_GENERATED_CERTIFICATION_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.certified_domains != CertificationDomain::ALL.to_vec() {
        report.push(
            "packet.certified_domains",
            "packet must certify every required domain in canonical order",
        );
    }
    if packet.evidence_packet_refs.is_empty() {
        report.push(
            "packet.evidence_packet_refs",
            "packet must cite the generated-artifact evidence packets",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut covered_profiles = BTreeSet::new();
    for certification_row in &packet.rows {
        if !covered_profiles.insert(certification_row.profile) {
            report.push(
                "row.profile_unique",
                format!("duplicate profile {}", certification_row.profile.as_str()),
            );
        }
        validate_row(&mut report, certification_row);
    }
    for required in CertifiedProfile::ALL {
        if !covered_profiles.contains(&required) {
            report.push(
                "packet.covered_profile",
                format!("packet must certify profile {}", required.as_str()),
            );
        }
    }

    validate_freshness_rules(&mut report, packet);
    validate_drills(&mut report, packet);
    validate_surface_bindings(&mut report, packet);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_domains(report: &mut ValidationReport, owner: &str, domains: &[DomainEvidence]) {
    let mut seen = BTreeSet::new();
    for evidence in domains {
        if !seen.insert(evidence.domain) {
            report.push(
                "domain.unique",
                format!("{owner} repeats domain {}", evidence.domain.as_str()),
            );
        }
        if evidence.evidence_state != EvidenceState::Missing && evidence.evidence_refs.is_empty() {
            report.push(
                "domain.evidence_refs",
                format!(
                    "{owner} domain {} must cite evidence unless it is missing",
                    evidence.domain.as_str()
                ),
            );
        }
        if evidence.rationale.trim().is_empty() {
            report.push(
                "domain.rationale",
                format!(
                    "{owner} domain {} must carry a rationale",
                    evidence.domain.as_str()
                ),
            );
        }
    }
    for required in CertificationDomain::ALL {
        if !seen.contains(&required) {
            report.push(
                "domain.coverage",
                format!("{owner} must evidence domain {}", required.as_str()),
            );
        }
    }
}

fn validate_row(report: &mut ValidationReport, certification_row: &CertificationRow) {
    if certification_row.row_id.trim().is_empty() {
        report.push("row.id", "row must carry a stable id");
    }
    if certification_row.profile_label.trim().is_empty() {
        report.push(
            "row.profile_label",
            format!(
                "row {} must carry a profile label",
                certification_row.row_id
            ),
        );
    }
    if certification_row.claim_publication_ref != certification_row.profile.claim_publication_ref()
    {
        report.push(
            "row.claim_publication_ref",
            format!(
                "row {} claim_publication_ref must match its profile",
                certification_row.row_id
            ),
        );
    }
    if certification_row.backing_artifact_class
        != certification_row.profile.backing_artifact_class()
    {
        report.push(
            "row.backing_artifact_class",
            format!(
                "row {} backing_artifact_class must match its profile",
                certification_row.row_id
            ),
        );
    }
    if certification_row.governance_evidence_ref != GOVERNANCE_EVIDENCE_REF {
        report.push(
            "row.governance_evidence_ref",
            format!(
                "row {} must bind back to the governance proof packet",
                certification_row.row_id
            ),
        );
    }
    if certification_row.why_certified.trim().is_empty() {
        report.push(
            "row.why_certified",
            format!(
                "row {} must carry a why-this-certified inspector line",
                certification_row.row_id
            ),
        );
    }
    if certification_row.consumer_refs.is_empty() {
        report.push(
            "row.consumer_refs",
            format!(
                "row {} must cite at least one consumer ref",
                certification_row.row_id
            ),
        );
    }
    if certification_row.notes.trim().is_empty() {
        report.push(
            "row.notes",
            format!(
                "row {} must carry a reviewer note",
                certification_row.row_id
            ),
        );
    }

    validate_domains(
        report,
        &format!("row {}", certification_row.row_id),
        &certification_row.domains,
    );

    // The stamped outcome must equal what the engine computes.
    let outcome = certify_profile_outcome(
        certification_row.published_claim_maturity,
        &certification_row.domains,
    );
    if certification_row.certified_maturity != outcome.certified_maturity {
        report.push(
            "row.certified_maturity",
            format!(
                "row {} certified_maturity {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.certified_maturity.as_str(),
                outcome.certified_maturity.as_str()
            ),
        );
    }
    if certification_row.verdict != outcome.verdict {
        report.push(
            "row.verdict",
            format!(
                "row {} verdict {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.verdict.as_str(),
                outcome.verdict.as_str()
            ),
        );
    }
    if certification_row.narrowed != outcome.narrowed {
        report.push(
            "row.narrowed",
            format!(
                "row {} narrowed flag disagrees with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.promotion_decision != outcome.promotion_decision {
        report.push(
            "row.promotion_decision",
            format!(
                "row {} promotion_decision {} disagrees with the engine ({})",
                certification_row.row_id,
                certification_row.promotion_decision.as_str(),
                outcome.promotion_decision.as_str()
            ),
        );
    }
    if certification_row.narrow_reason_tokens != outcome.narrow_reason_tokens {
        report.push(
            "row.narrow_reason_tokens",
            format!(
                "row {} narrow_reason_tokens disagree with the engine",
                certification_row.row_id
            ),
        );
    }
    if certification_row.stale_or_missing_domain_tokens != outcome.stale_or_missing_domain_tokens {
        report.push(
            "row.stale_or_missing_domain_tokens",
            format!(
                "row {} stale_or_missing_domain_tokens disagree with the engine",
                certification_row.row_id
            ),
        );
    }
    // A certified maturity may never exceed the published claim.
    if certification_row.certified_maturity.severity()
        < certification_row.published_claim_maturity.severity()
    {
        report.push(
            "row.never_widens",
            format!(
                "row {} certified_maturity must never exceed the published claim",
                certification_row.row_id
            ),
        );
    }

    let expected_support = supporting_evidence_refs(&certification_row.domains);
    if certification_row.supporting_evidence_refs != expected_support {
        report.push(
            "row.supporting_evidence_refs",
            format!(
                "row {} supporting_evidence_refs must equal the union of its domain evidence refs",
                certification_row.row_id
            ),
        );
    }
}

fn validate_freshness_rules(
    report: &mut ValidationReport,
    packet: &M5GeneratedCertificationPacket,
) {
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
        let expected_decision = if rule.maturity_floor == ClaimMaturity::Withdrawn {
            PromotionDecision::Hold
        } else {
            PromotionDecision::PromoteNarrowed
        };
        if rule.promotion_decision != expected_decision {
            report.push(
                "freshness_rule.promotion_decision",
                format!(
                    "rule {} promotion_decision disagrees with its maturity floor",
                    rule.rule_id
                ),
            );
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

fn validate_drills(report: &mut ValidationReport, packet: &M5GeneratedCertificationPacket) {
    if packet.drills.is_empty() {
        report.push(
            "packet.drills",
            "packet must declare failure/recovery drills",
        );
    }
    let mut drill_ids = BTreeSet::new();
    let mut drilled_profiles = BTreeSet::new();
    let mut drilled_domains = BTreeSet::new();
    let mut has_narrowed = false;
    let mut has_withheld = false;
    for certification_drill in &packet.drills {
        if !drill_ids.insert(certification_drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", certification_drill.drill_id),
            );
        }
        drilled_profiles.insert(certification_drill.profile);
        drilled_domains.insert(certification_drill.exercised_domain);

        if certification_drill.failure_class.domain() != certification_drill.exercised_domain {
            report.push(
                "drill.failure_domain",
                format!(
                    "drill {} failure class does not match the exercised domain",
                    certification_drill.drill_id
                ),
            );
        }

        // Recompute the degraded outcome from the engine.
        let degraded = degraded_domains(
            certification_drill.exercised_domain,
            certification_drill.degraded_evidence_state,
        );
        let degraded_outcome =
            certify_profile_outcome(certification_drill.published_claim_maturity, &degraded);
        if certification_drill.expected_degraded_verdict != degraded_outcome.verdict {
            report.push(
                "drill.degraded_verdict",
                format!(
                    "drill {} degraded verdict disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if certification_drill.expected_degraded_maturity != degraded_outcome.certified_maturity {
            report.push(
                "drill.degraded_maturity",
                format!(
                    "drill {} degraded maturity disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if certification_drill.expected_degraded_promotion_decision
            != degraded_outcome.promotion_decision
        {
            report.push(
                "drill.degraded_promotion_decision",
                format!(
                    "drill {} degraded promotion decision disagrees with the engine",
                    certification_drill.drill_id
                ),
            );
        }
        if degraded_outcome.verdict == RowVerdict::Certified {
            report.push(
                "drill.must_degrade",
                format!(
                    "drill {} must inject a failure that actually narrows or withholds",
                    certification_drill.drill_id
                ),
            );
        }
        match degraded_outcome.verdict {
            RowVerdict::Narrowed => has_narrowed = true,
            RowVerdict::Withheld => has_withheld = true,
            RowVerdict::Certified => {}
        }
        if certification_drill.recovers_to_verdict != RowVerdict::Certified {
            report.push(
                "drill.recovers",
                format!(
                    "drill {} must recover to certified",
                    certification_drill.drill_id
                ),
            );
        }
        if !certification_drill.asserts_claim_narrows_under_failure
            || !certification_drill.asserts_recovers_after_refresh
        {
            report.push(
                "drill.assertions",
                format!(
                    "drill {} must assert it narrows under failure and recovers after refresh",
                    certification_drill.drill_id
                ),
            );
        }
        validate_drill_steps(report, certification_drill);
    }
    for required in CertifiedProfile::ALL {
        if !drilled_profiles.contains(&required) {
            report.push(
                "packet.drilled_profile",
                format!("packet must drill profile {}", required.as_str()),
            );
        }
    }
    for required in CertificationDomain::ALL {
        if !drilled_domains.contains(&required) {
            report.push(
                "packet.drilled_domain",
                format!("packet must drill domain {}", required.as_str()),
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
}

fn validate_drill_steps(report: &mut ValidationReport, certification_drill: &CertificationDrill) {
    if certification_drill.steps.is_empty() {
        report.push(
            "drill.steps",
            format!("drill {} must declare steps", certification_drill.drill_id),
        );
        return;
    }
    if certification_drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Inject) {
        report.push(
            "drill.first_phase",
            format!(
                "drill {} must begin with an inject step",
                certification_drill.drill_id
            ),
        );
    }
    if certification_drill.steps.last().map(|s| s.phase) != Some(DrillPhase::Verify) {
        report.push(
            "drill.last_phase",
            format!(
                "drill {} must end with a verify step",
                certification_drill.drill_id
            ),
        );
    }
    let has_narrow = certification_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Narrow);
    let has_recover = certification_drill
        .steps
        .iter()
        .any(|s| s.phase == DrillPhase::Recover);
    if !has_narrow || !has_recover {
        report.push(
            "drill.phases",
            format!(
                "drill {} must include a narrow step and a recover step",
                certification_drill.drill_id
            ),
        );
    }
    for (index, drill_step) in certification_drill.steps.iter().enumerate() {
        if drill_step.narration.trim().is_empty() {
            report.push(
                "drill.step_narration",
                format!(
                    "drill {} step {index} must narrate",
                    certification_drill.drill_id
                ),
            );
        }
    }
}

fn validate_surface_bindings(
    report: &mut ValidationReport,
    packet: &M5GeneratedCertificationPacket,
) {
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
pub fn validate_m5_generated_certification_fixture(
    fixture: &M5GeneratedCertificationFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != M5_GENERATED_CERTIFICATION_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != M5_GENERATED_CERTIFICATION_SCHEMA_VERSION {
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

    validate_domains(
        &mut report,
        &format!("fixture {}", fixture.fixture_id),
        &fixture.observed_domains,
    );

    let outcome =
        certify_profile_outcome(fixture.published_claim_maturity, &fixture.observed_domains);
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
    if fixture.expected_certified_maturity != outcome.certified_maturity {
        report.push(
            "fixture.expected_certified_maturity",
            format!(
                "fixture {} expected maturity {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_certified_maturity.as_str(),
                outcome.certified_maturity.as_str()
            ),
        );
    }
    if fixture.expected_promotion_decision != outcome.promotion_decision {
        report.push(
            "fixture.expected_promotion_decision",
            format!(
                "fixture {} expected promotion decision {} disagrees with the engine ({})",
                fixture.fixture_id,
                fixture.expected_promotion_decision.as_str(),
                outcome.promotion_decision.as_str()
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

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
