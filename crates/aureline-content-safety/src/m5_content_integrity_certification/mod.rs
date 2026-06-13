//! Certification capstone for M5 content-integrity, safe-preview, and
//! raw-versus-rendered copy/export honesty on every claimed M5 artifact/viewer
//! family.
//!
//! The earlier M5 content-safety lanes each prove one dimension of the track
//! invariant: the suspicious-text detector parity packet proves shared
//! suspicious-content cues, the trust-class ladder packet proves runtime trust
//! classes, the trust-decision identity packet proves strong-decision strict
//! identity rendering, the safe-preview limited-mode packet proves bounded
//! preview fallbacks, the raw-versus-rendered handoff packet proves copy/export
//! representation honesty, and the mutation-path fix-flow packet proves that
//! bidi/invisible/confusable fixes never rewrite bytes silently. This module is
//! the single capstone that *certifies* each claimed M5 family by binding those
//! per-dimension proofs together and auto-narrowing any family that lacks a
//! current proof on a required dimension.
//!
//! [`project_m5_content_integrity_certification`] is the deterministic
//! auto-narrow rule: a family keeps its claimed qualification only when every
//! required proof dimension is `current_pass`; a stale proof narrows it one
//! rung, a missing proof narrows it to experimental, and a failing proof holds
//! it. The frozen [`frozen_m5_content_integrity_certification_packet`] certifies
//! all ten families with current proof, and the checked-in fixtures exercise the
//! narrowing path so shiproom, docs, and support surfaces ingest one
//! certification result instead of publishing their own viewer trust text.
//!
//! The packet reuses the matrix family, qualification, and consumer-surface
//! vocabularies from
//! [`crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix`]
//! so the certification is genuinely about the same claimed families. It carries
//! no raw suspicious bytes, raw rendered trees, raw provider payloads,
//! credentials, or live preview-origin responses.
//!
//! The boundary schema is
//! [`schemas/security/m5-content-integrity-certification.schema.json`](../../../../schemas/security/m5-content-integrity-certification.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_content_integrity_certification.md`](../../../../docs/security/m5/m5_content_integrity_certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix::{
    M5ContentIntegrityArtifactFamily, M5ContentIntegrityConsumerSurface,
    M5ContentIntegrityQualificationClass, M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID,
};
use crate::m5_mutation_path_fix_flow::M5_MUTATION_PATH_FIX_FLOW_PACKET_ID;
use crate::m5_raw_rendered_handoff::M5_RAW_RENDERED_HANDOFF_PACKET_ID;
use crate::m5_safe_preview_limited_mode::M5_SAFE_PREVIEW_LIMITED_PACKET_ID;
use crate::m5_suspicious_text_detector_parity::M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID;
use crate::m5_trust_class_ladder::M5_TRUST_CLASS_LADDER_PACKET_ID;
use crate::m5_trust_decision_identity::M5_TRUST_DECISION_IDENTITY_PACKET_ID;

/// Stable record-kind tag carried by [`M5ContentIntegrityCertificationPacket`].
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_RECORD_KIND: &str =
    "certify_m5_content_integrity_safe_preview_and_representation_honesty";

/// Schema version for the M5 content-integrity certification records.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/security/m5-content-integrity-certification.schema.json";

/// Repo-relative path of the certification contract doc.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_DOC_REF: &str =
    "docs/security/m5/m5_content_integrity_certification.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_content_integrity_certification";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_content_integrity_certification/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/security/m5/m5_content_integrity_certification.md";

/// Stable packet id minted by [`frozen_m5_content_integrity_certification_packet`].
pub const M5_CONTENT_INTEGRITY_CERTIFICATION_PACKET_ID: &str =
    "m5-content-integrity-certification:stable:0001";

/// A proof dimension that a claimed M5 family must satisfy to keep its claim.
///
/// Each dimension is backed by exactly one upstream content-safety proof lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationProofDimension {
    /// Shared suspicious-text cues (bidi/invisible/confusable) are detected and
    /// surfaced rather than normalized away.
    SuspiciousContentCues,
    /// Safe-preview trust classes and limited-mode fallbacks are resolved.
    SafePreviewTrustClass,
    /// Strong-decision surfaces render owner/origin identity strictly.
    StrongDecisionDisplay,
    /// Raw and rendered copy/export forms stay distinct and both reachable.
    RawRenderedCopyExport,
    /// Active rich content never executes outside its declared trust class.
    ActiveContentContainment,
    /// Bidi/invisible/confusable fixes never rewrite bytes silently.
    SilentRewriteGuard,
}

impl M5CertificationProofDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SuspiciousContentCues,
        Self::SafePreviewTrustClass,
        Self::StrongDecisionDisplay,
        Self::RawRenderedCopyExport,
        Self::ActiveContentContainment,
        Self::SilentRewriteGuard,
    ];

    /// Dimensions every claimed family must certify regardless of surface kind.
    pub const UNIVERSAL_REQUIRED: [Self; 4] = [
        Self::SuspiciousContentCues,
        Self::SafePreviewTrustClass,
        Self::RawRenderedCopyExport,
        Self::ActiveContentContainment,
    ];

    /// Stable token recorded in the certification packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuspiciousContentCues => "suspicious_content_cues",
            Self::SafePreviewTrustClass => "safe_preview_trust_class",
            Self::StrongDecisionDisplay => "strong_decision_display",
            Self::RawRenderedCopyExport => "raw_rendered_copy_export",
            Self::ActiveContentContainment => "active_content_containment",
            Self::SilentRewriteGuard => "silent_rewrite_guard",
        }
    }
}

/// The upstream proof lane that backs a certification dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationProofLane {
    /// Shared suspicious-text detector parity packet.
    SuspiciousTextParity,
    /// Safe-preview limited-mode packet.
    SafePreviewLimitedMode,
    /// Runtime trust-class ladder packet.
    TrustClassLadder,
    /// Strong-decision trust-decision identity packet.
    TrustDecisionIdentity,
    /// Raw-versus-rendered copy/export handoff packet.
    RawRenderedHandoff,
    /// Mutation-path silent-rewrite fix-flow packet.
    MutationPathFixFlow,
    /// Frozen M5 content-integrity maturity matrix packet.
    ContentIntegrityMatrix,
}

impl M5CertificationProofLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SuspiciousTextParity,
        Self::SafePreviewLimitedMode,
        Self::TrustClassLadder,
        Self::TrustDecisionIdentity,
        Self::RawRenderedHandoff,
        Self::MutationPathFixFlow,
        Self::ContentIntegrityMatrix,
    ];

    /// Stable token recorded in the certification packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuspiciousTextParity => "suspicious_text_parity",
            Self::SafePreviewLimitedMode => "safe_preview_limited_mode",
            Self::TrustClassLadder => "trust_class_ladder",
            Self::TrustDecisionIdentity => "trust_decision_identity",
            Self::RawRenderedHandoff => "raw_rendered_handoff",
            Self::MutationPathFixFlow => "mutation_path_fix_flow",
            Self::ContentIntegrityMatrix => "content_integrity_matrix",
        }
    }

    /// Canonical upstream packet id this lane certifies against.
    pub fn packet_ref(self) -> &'static str {
        match self {
            Self::SuspiciousTextParity => M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID,
            Self::SafePreviewLimitedMode => M5_SAFE_PREVIEW_LIMITED_PACKET_ID,
            Self::TrustClassLadder => M5_TRUST_CLASS_LADDER_PACKET_ID,
            Self::TrustDecisionIdentity => M5_TRUST_DECISION_IDENTITY_PACKET_ID,
            Self::RawRenderedHandoff => M5_RAW_RENDERED_HANDOFF_PACKET_ID,
            Self::MutationPathFixFlow => M5_MUTATION_PATH_FIX_FLOW_PACKET_ID,
            Self::ContentIntegrityMatrix => M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID,
        }
    }

    /// The proof lane that conventionally backs a dimension.
    pub const fn for_dimension(dimension: M5CertificationProofDimension) -> Self {
        match dimension {
            M5CertificationProofDimension::SuspiciousContentCues => Self::SuspiciousTextParity,
            M5CertificationProofDimension::SafePreviewTrustClass => Self::SafePreviewLimitedMode,
            M5CertificationProofDimension::StrongDecisionDisplay => Self::TrustDecisionIdentity,
            M5CertificationProofDimension::RawRenderedCopyExport => Self::RawRenderedHandoff,
            M5CertificationProofDimension::ActiveContentContainment => Self::TrustClassLadder,
            M5CertificationProofDimension::SilentRewriteGuard => Self::MutationPathFixFlow,
        }
    }
}

/// Freshness/pass state of a single dimension proof for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationProofState {
    /// Proof is present, fresh, and passing.
    CurrentPass,
    /// Proof is present and passing but past its freshness SLO.
    StalePass,
    /// Proof is present but failing.
    Failing,
    /// No proof lane covers this dimension for this family.
    Missing,
    /// The dimension does not apply to this family.
    NotApplicable,
}

impl M5CertificationProofState {
    /// Stable token recorded in the certification packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentPass => "current_pass",
            Self::StalePass => "stale_pass",
            Self::Failing => "failing",
            Self::Missing => "missing",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether a dimension is required for a family or does not apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationDimensionApplicability {
    /// The dimension must be certified with a current proof.
    Required,
    /// The dimension does not apply to this family.
    NotApplicable,
}

impl M5CertificationDimensionApplicability {
    /// Stable token recorded in the certification packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Why a family's certified qualification was narrowed below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CertificationNarrowingCause {
    /// A required dimension has no current proof lane.
    ProofMissing,
    /// A required dimension's proof is past its freshness SLO.
    ProofStale,
    /// A required dimension's proof is failing.
    ProofFailing,
}

impl M5CertificationNarrowingCause {
    /// Stable token recorded in the certification packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofMissing => "proof_missing",
            Self::ProofStale => "proof_stale",
            Self::ProofFailing => "proof_failing",
        }
    }
}

/// One reason a family's certified qualification narrowed below its claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationNarrowingReason {
    /// The proof dimension that triggered the narrowing.
    pub dimension: M5CertificationProofDimension,
    /// The narrowing cause.
    pub cause: M5CertificationNarrowingCause,
    /// Human-readable detail recorded in support/shiproom surfaces.
    pub detail: String,
}

/// One certified dimension proof for a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertifiedDimensionProof {
    /// The proof dimension.
    pub dimension: M5CertificationProofDimension,
    /// The upstream lane that backs this dimension.
    pub backing_lane: M5CertificationProofLane,
    /// The upstream packet id this proof certifies against.
    pub backing_packet_ref: String,
    /// Whether the dimension is required or does not apply.
    pub applicability: M5CertificationDimensionApplicability,
    /// Freshness/pass state of this proof.
    pub state: M5CertificationProofState,
    /// Human-readable note for support/shiproom surfaces.
    pub note: String,
}

/// One certified M5 artifact/viewer family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertifiedFamilyRow {
    /// Artifact/viewer family.
    pub family: M5ContentIntegrityArtifactFamily,
    /// Qualification class the matrix claims for this family.
    pub claimed_qualification: M5ContentIntegrityQualificationClass,
    /// Qualification class certified after the auto-narrow rule.
    pub certified_qualification: M5ContentIntegrityQualificationClass,
    /// Per-dimension proofs backing the certification.
    pub dimension_proofs: Vec<M5CertifiedDimensionProof>,
    /// Reasons the certified qualification narrowed below the claim.
    pub narrowing_reasons: Vec<M5CertificationNarrowingReason>,
    /// Distinct upstream packet refs the certification ingested.
    pub proof_refs: Vec<String>,
    /// Consumer surfaces that must project this certified family.
    pub consumer_surfaces: Vec<M5ContentIntegrityConsumerSurface>,
}

impl M5CertifiedFamilyRow {
    /// Whether this family was narrowed below its claimed qualification.
    pub fn was_narrowed(&self) -> bool {
        rank(self.certified_qualification) < rank(self.claimed_qualification)
    }
}

/// One dimension input to [`project_m5_content_integrity_certification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CertificationDimensionInput {
    /// The proof dimension.
    pub dimension: M5CertificationProofDimension,
    /// The proof state observed for this family.
    pub state: M5CertificationProofState,
    /// Human-readable note carried into the certified proof.
    pub note: String,
}

/// Seed for certifying a single family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CertificationFamilySeed {
    /// Artifact/viewer family.
    pub family: M5ContentIntegrityArtifactFamily,
    /// Qualification class claimed by the matrix.
    pub claimed_qualification: M5ContentIntegrityQualificationClass,
    /// Per-dimension inputs.
    pub dimensions: Vec<M5CertificationDimensionInput>,
    /// Consumer surfaces that must project this family.
    pub consumer_surfaces: Vec<M5ContentIntegrityConsumerSurface>,
}

/// Certifies one family by applying the deterministic auto-narrow rule.
///
/// A family keeps its claimed qualification only when every required dimension
/// is [`M5CertificationProofState::CurrentPass`]. A stale proof narrows it one
/// rung, a missing proof narrows it to experimental, and a failing proof holds
/// it; the certified qualification is the lowest rung any narrowing implies.
pub fn project_m5_content_integrity_certification(
    seed: M5CertificationFamilySeed,
) -> M5CertifiedFamilyRow {
    let dimension_proofs: Vec<M5CertifiedDimensionProof> = seed
        .dimensions
        .iter()
        .map(|input| {
            let lane = M5CertificationProofLane::for_dimension(input.dimension);
            let applicability = match input.state {
                M5CertificationProofState::NotApplicable => {
                    M5CertificationDimensionApplicability::NotApplicable
                }
                _ => M5CertificationDimensionApplicability::Required,
            };
            M5CertifiedDimensionProof {
                dimension: input.dimension,
                backing_lane: lane,
                backing_packet_ref: lane.packet_ref().to_owned(),
                applicability,
                state: input.state,
                note: input.note.clone(),
            }
        })
        .collect();

    let (certified_qualification, narrowing_reasons) =
        certify_from_proofs(seed.claimed_qualification, &dimension_proofs);

    let mut proof_refs: Vec<String> = dimension_proofs
        .iter()
        .filter(|proof| proof.applicability == M5CertificationDimensionApplicability::Required)
        .map(|proof| proof.backing_packet_ref.clone())
        .collect();
    proof_refs.sort();
    proof_refs.dedup();

    M5CertifiedFamilyRow {
        family: seed.family,
        claimed_qualification: seed.claimed_qualification,
        certified_qualification,
        dimension_proofs,
        narrowing_reasons,
        proof_refs,
        consumer_surfaces: seed.consumer_surfaces,
    }
}

/// Recomputes the certified qualification and narrowing reasons from proofs.
///
/// This is the single source of the auto-narrow rule; both
/// [`project_m5_content_integrity_certification`] and packet validation call it
/// so a checked-in row cannot record a certified qualification that disagrees
/// with its own proofs.
fn certify_from_proofs(
    claimed: M5ContentIntegrityQualificationClass,
    proofs: &[M5CertifiedDimensionProof],
) -> (
    M5ContentIntegrityQualificationClass,
    Vec<M5CertificationNarrowingReason>,
) {
    let mut certified = claimed;
    let mut reasons = Vec::new();
    for proof in proofs {
        if proof.applicability != M5CertificationDimensionApplicability::Required {
            continue;
        }
        let (cause, target) = match proof.state {
            M5CertificationProofState::CurrentPass | M5CertificationProofState::NotApplicable => {
                continue
            }
            M5CertificationProofState::StalePass => (
                M5CertificationNarrowingCause::ProofStale,
                narrow_one_step(claimed),
            ),
            M5CertificationProofState::Missing => (
                M5CertificationNarrowingCause::ProofMissing,
                M5ContentIntegrityQualificationClass::Experimental,
            ),
            M5CertificationProofState::Failing => (
                M5CertificationNarrowingCause::ProofFailing,
                M5ContentIntegrityQualificationClass::Held,
            ),
        };
        certified = lower_qualification(certified, target);
        reasons.push(M5CertificationNarrowingReason {
            dimension: proof.dimension,
            cause,
            detail: format!(
                "{} proof for {} narrows the claim",
                cause.as_str(),
                proof.dimension.as_str()
            ),
        });
    }
    (certified, reasons)
}

/// Total ordering rank for a qualification class (higher = stronger claim).
const fn rank(qualification: M5ContentIntegrityQualificationClass) -> u8 {
    match qualification {
        M5ContentIntegrityQualificationClass::Stable => 5,
        M5ContentIntegrityQualificationClass::Beta => 4,
        M5ContentIntegrityQualificationClass::Preview => 3,
        M5ContentIntegrityQualificationClass::Experimental => 2,
        M5ContentIntegrityQualificationClass::Held => 1,
        M5ContentIntegrityQualificationClass::Unavailable => 0,
    }
}

/// Returns the weaker of two qualification classes.
fn lower_qualification(
    a: M5ContentIntegrityQualificationClass,
    b: M5ContentIntegrityQualificationClass,
) -> M5ContentIntegrityQualificationClass {
    if rank(a) <= rank(b) {
        a
    } else {
        b
    }
}

/// Narrows a qualification class by exactly one rung.
const fn narrow_one_step(
    qualification: M5ContentIntegrityQualificationClass,
) -> M5ContentIntegrityQualificationClass {
    match qualification {
        M5ContentIntegrityQualificationClass::Stable => M5ContentIntegrityQualificationClass::Beta,
        M5ContentIntegrityQualificationClass::Beta => M5ContentIntegrityQualificationClass::Preview,
        M5ContentIntegrityQualificationClass::Preview => {
            M5ContentIntegrityQualificationClass::Experimental
        }
        other => other,
    }
}

/// Certification review block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationReview {
    /// All four representation-honesty dimensions are certified together.
    pub all_dimensions_certified_together: bool,
    /// Auto-narrow fires when any required dimension lacks a current proof.
    pub auto_narrow_on_missing_or_stale_proof: bool,
    /// Strong-decision families certify the strict-identity display dimension.
    pub strong_decision_families_certify_strict_identity: bool,
    /// Narrowing narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Regression or missing threat-class coverage blocks promotion.
    pub regression_or_missing_coverage_blocks_promotion: bool,
    /// Suspicious bytes are never normalized away to earn a proof.
    pub suspicious_bytes_not_normalized_away: bool,
    /// Active rich content never auto-executes to earn a proof.
    pub no_auto_execute_to_earn_proof: bool,
}

impl M5CertificationReview {
    /// Whether every certification invariant holds.
    pub fn all_hold(&self) -> bool {
        self.all_dimensions_certified_together
            && self.auto_narrow_on_missing_or_stale_proof
            && self.strong_decision_families_certify_strict_identity
            && self.downgrade_narrows_instead_of_hides
            && self.regression_or_missing_coverage_blocks_promotion
            && self.suspicious_bytes_not_normalized_away
            && self.no_auto_execute_to_earn_proof
    }
}

/// Consumer projection block; every field encodes a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationConsumerProjection {
    /// Shiproom ingests the certification result rather than its own text.
    pub shiproom_ingests_certification: bool,
    /// Docs ingest the certification result rather than their own text.
    pub docs_ingest_certification: bool,
    /// Support export ingests the certification result.
    pub support_export_ingests_certification: bool,
    /// CLI / headless shows the certification result.
    pub cli_headless_shows_certification: bool,
    /// Diagnostics shows the certification result.
    pub diagnostics_shows_certification: bool,
    /// Narrowed families are visibly labeled rather than silently dropped.
    pub narrowed_families_visibly_labeled: bool,
}

impl M5CertificationConsumerProjection {
    /// Whether every projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.shiproom_ingests_certification
            && self.docs_ingest_certification
            && self.support_export_ingests_certification
            && self.cli_headless_shows_certification
            && self.diagnostics_shows_certification
            && self.narrowed_families_visibly_labeled
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last certification refresh.
    pub last_certified_at: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Counts summarizing the certification result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CertificationSummary {
    /// Total certified families.
    pub total_families: u32,
    /// Families certified at the Stable claim.
    pub certified_stable: u32,
    /// Families narrowed below their claimed qualification.
    pub narrowed_families: u32,
}

/// Constructor input for [`M5ContentIntegrityCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContentIntegrityCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified family rows.
    pub family_rows: Vec<M5CertifiedFamilyRow>,
    /// Certification review block.
    pub review: M5CertificationReview,
    /// Consumer projection block.
    pub consumer_projection: M5CertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 content-integrity certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityCertificationPacket {
    /// Record kind; must equal [`M5_CONTENT_INTEGRITY_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified family rows.
    pub family_rows: Vec<M5CertifiedFamilyRow>,
    /// Certification summary counts.
    pub summary: M5CertificationSummary,
    /// Certification review block.
    pub review: M5CertificationReview,
    /// Consumer projection block.
    pub consumer_projection: M5CertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ContentIntegrityCertificationPacket {
    /// Builds a certification packet, deriving the summary from its rows.
    pub fn new(input: M5ContentIntegrityCertificationPacketInput) -> Self {
        let summary = derive_summary(&input.family_rows);
        Self {
            record_kind: M5_CONTENT_INTEGRITY_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            family_rows: input.family_rows,
            summary,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the certification invariants.
    pub fn validate(&self) -> Vec<M5ContentIntegrityCertificationViolation> {
        use M5ContentIntegrityCertificationViolation as V;
        let mut violations = Vec::new();

        if self.record_kind != M5_CONTENT_INTEGRITY_CERTIFICATION_RECORD_KIND {
            violations.push(V::WrongRecordKind);
        }
        if self.schema_version != M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_VERSION {
            violations.push(V::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(V::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_family_rows(self, &mut violations);
        validate_summary(self, &mut violations);

        if !self.review.all_hold() {
            violations.push(V::ReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(V::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_certified_at.trim().is_empty()
        {
            violations.push(V::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 content-integrity certification serializes"),
        ) {
            violations.push(V::RawBoundaryMaterialInExport);
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
            .expect("m5 content-integrity certification packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Content-Integrity, Safe-Preview, and Representation-Honesty Certification\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Families: {} ({} certified stable, {} narrowed)\n",
            self.summary.total_families,
            self.summary.certified_stable,
            self.summary.narrowed_families
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last certified: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_certified_at
        ));
        out.push_str("\n## Certified families\n\n");
        for row in &self.family_rows {
            out.push_str(&format!(
                "- **{}**: claimed `{}` → certified `{}`\n",
                row.family.as_str(),
                row.claimed_qualification.as_str(),
                row.certified_qualification.as_str()
            ));
            if row.narrowing_reasons.is_empty() {
                out.push_str("  - All required proof dimensions current.\n");
            } else {
                for reason in &row.narrowing_reasons {
                    out.push_str(&format!(
                        "  - Narrowed: {} ({})\n",
                        reason.dimension.as_str(),
                        reason.cause.as_str()
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum M5ContentIntegrityCertificationExportError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContentIntegrityCertificationViolation>),
}

impl fmt::Display for M5ContentIntegrityCertificationExportError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 content-integrity certification export parse failed: {error}"
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
                    "m5 content-integrity certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ContentIntegrityCertificationExportError {}

/// Validation failures emitted by [`M5ContentIntegrityCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContentIntegrityCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required family is missing from the certification.
    RequiredFamilyMissing,
    /// A family row is incomplete.
    FamilyRowIncomplete,
    /// A required proof dimension is absent for a family.
    RequiredDimensionMissing,
    /// A required dimension proof has no backing packet ref.
    ProofRefMissing,
    /// A family has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// The certified qualification disagrees with its own proofs.
    CertifiedQualificationDrift,
    /// The narrowing reasons disagree with the certified qualification.
    NarrowingReasonsInconsistent,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Summary counts disagree with the family rows.
    SummaryCountMismatch,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ContentIntegrityCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::FamilyRowIncomplete => "family_row_incomplete",
            Self::RequiredDimensionMissing => "required_dimension_missing",
            Self::ProofRefMissing => "proof_ref_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::CertifiedQualificationDrift => "certified_qualification_drift",
            Self::NarrowingReasonsInconsistent => "narrowing_reasons_inconsistent",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::SummaryCountMismatch => "summary_count_mismatch",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Required source contract refs for the certification packet.
fn required_source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        M5_CONTENT_INTEGRITY_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_CONTENT_INTEGRITY_CERTIFICATION_DOC_REF.to_owned(),
    ];
    for lane in M5CertificationProofLane::ALL {
        refs.push(lane.packet_ref().to_owned());
    }
    refs
}

/// Builds the canonical frozen stable M5 content-integrity certification packet.
///
/// Every claimed family certifies all required dimensions with current proof, so
/// each certified qualification equals its matrix claim. This is the single
/// in-code source of truth for the checked-in support export at
/// [`M5_CONTENT_INTEGRITY_CERTIFICATION_ARTIFACT_REF`].
pub fn frozen_m5_content_integrity_certification_packet() -> M5ContentIntegrityCertificationPacket {
    use M5ContentIntegrityArtifactFamily as Family;
    use M5ContentIntegrityConsumerSurface as Surface;
    use M5ContentIntegrityQualificationClass as Qual;

    let family_rows = canonical_family_seeds()
        .into_iter()
        .map(project_m5_content_integrity_certification)
        .collect::<Vec<_>>();

    // Defensive: the canonical seeds must certify cleanly (no narrowing).
    debug_assert!(family_rows.iter().all(|row| !row.was_narrowed()));
    debug_assert_eq!(family_rows.len(), Family::ALL.len());
    let _ = (Surface::SupportExport, Qual::Stable);

    M5ContentIntegrityCertificationPacket::new(M5ContentIntegrityCertificationPacketInput {
        packet_id: M5_CONTENT_INTEGRITY_CERTIFICATION_PACKET_ID.to_owned(),
        certification_label:
            "M5 Content-Integrity, Safe-Preview, and Representation-Honesty Certification"
                .to_owned(),
        family_rows,
        review: M5CertificationReview {
            all_dimensions_certified_together: true,
            auto_narrow_on_missing_or_stale_proof: true,
            strong_decision_families_certify_strict_identity: true,
            downgrade_narrows_instead_of_hides: true,
            regression_or_missing_coverage_blocks_promotion: true,
            suspicious_bytes_not_normalized_away: true,
            no_auto_execute_to_earn_proof: true,
        },
        consumer_projection: M5CertificationConsumerProjection {
            shiproom_ingests_certification: true,
            docs_ingest_certification: true,
            support_export_ingests_certification: true,
            cli_headless_shows_certification: true,
            diagnostics_shows_certification: true,
            narrowed_families_visibly_labeled: true,
        },
        proof_freshness: M5CertificationProofFreshness {
            proof_freshness_slo_hours: 168,
            last_certified_at: "2026-06-12T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: required_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable certification export.
pub fn current_m5_content_integrity_certification_export(
) -> Result<M5ContentIntegrityCertificationPacket, M5ContentIntegrityCertificationExportError> {
    let packet: M5ContentIntegrityCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/security/m5/m5_content_integrity_certification/support_export.json"
        )))
        .map_err(M5ContentIntegrityCertificationExportError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ContentIntegrityCertificationExportError::Validation(
            violations,
        ))
    }
}

/// Canonical per-family seeds with current proof on every required dimension.
fn canonical_family_seeds() -> Vec<M5CertificationFamilySeed> {
    use M5ContentIntegrityArtifactFamily as Family;
    use M5ContentIntegrityConsumerSurface as Surface;
    use M5ContentIntegrityQualificationClass as Qual;

    // The claimed qualifications mirror the frozen matrix rows so the
    // certification is genuinely about the same claimed families.
    let claims = [
        (
            Family::NotebookRichOutput,
            Qual::Beta,
            Surface::NotebookViewer,
            false,
        ),
        (
            Family::DocsBrowserPanel,
            Qual::Stable,
            Surface::DocsBrowserPanel,
            false,
        ),
        (
            Family::AiEvidenceViewer,
            Qual::Stable,
            Surface::AiEvidenceViewer,
            false,
        ),
        (
            Family::PipelineArtifactBrowser,
            Qual::Stable,
            Surface::PipelineArtifactBrowser,
            false,
        ),
        (
            Family::ProviderOverlay,
            Qual::Stable,
            Surface::ProviderOverlay,
            true,
        ),
        (
            Family::MarketplaceInstallUpdate,
            Qual::Stable,
            Surface::MarketplaceSurface,
            true,
        ),
        (
            Family::RemotePreviewTarget,
            Qual::Beta,
            Surface::RemotePreviewPanel,
            true,
        ),
        (
            Family::IncidentExportPacket,
            Qual::Stable,
            Surface::IncidentExport,
            false,
        ),
        (
            Family::GeneratedArtifact,
            Qual::Beta,
            Surface::StructuredCompareView,
            false,
        ),
        (
            Family::StructuredCompareView,
            Qual::Stable,
            Surface::StructuredCompareView,
            false,
        ),
    ];

    claims
        .into_iter()
        .map(|(family, claimed, primary_surface, strong_decision)| {
            let mut dimensions = vec![
                current_dim(
                    M5CertificationProofDimension::SuspiciousContentCues,
                    "shared suspicious-text detector parity holds for this surface",
                ),
                current_dim(
                    M5CertificationProofDimension::SafePreviewTrustClass,
                    "safe-preview trust class and limited-mode fallback resolved",
                ),
                current_dim(
                    M5CertificationProofDimension::RawRenderedCopyExport,
                    "raw and rendered copy/export forms stay distinct and reachable",
                ),
                current_dim(
                    M5CertificationProofDimension::ActiveContentContainment,
                    "active content stays inside its declared trust class",
                ),
                current_dim(
                    M5CertificationProofDimension::SilentRewriteGuard,
                    "bidi/invisible/confusable fixes never rewrite bytes silently",
                ),
            ];
            // Strong-decision surfaces additionally certify strict identity.
            dimensions.push(if strong_decision {
                current_dim(
                    M5CertificationProofDimension::StrongDecisionDisplay,
                    "strong-decision strict-identity rendering certified",
                )
            } else {
                M5CertificationDimensionInput {
                    dimension: M5CertificationProofDimension::StrongDecisionDisplay,
                    state: M5CertificationProofState::NotApplicable,
                    note: "ordinary browsing surface; strict-identity display not required"
                        .to_owned(),
                }
            });

            M5CertificationFamilySeed {
                family,
                claimed_qualification: claimed,
                dimensions,
                consumer_surfaces: vec![
                    primary_surface,
                    Surface::CliHeadless,
                    Surface::SupportExport,
                    Surface::Diagnostics,
                ],
            }
        })
        .collect()
}

fn current_dim(
    dimension: M5CertificationProofDimension,
    note: &str,
) -> M5CertificationDimensionInput {
    M5CertificationDimensionInput {
        dimension,
        state: M5CertificationProofState::CurrentPass,
        note: note.to_owned(),
    }
}

fn derive_summary(rows: &[M5CertifiedFamilyRow]) -> M5CertificationSummary {
    let certified_stable = rows
        .iter()
        .filter(|row| row.certified_qualification == M5ContentIntegrityQualificationClass::Stable)
        .count() as u32;
    let narrowed = rows.iter().filter(|row| row.was_narrowed()).count() as u32;
    M5CertificationSummary {
        total_families: rows.len() as u32,
        certified_stable,
        narrowed_families: narrowed,
    }
}

fn validate_source_contracts(
    packet: &M5ContentIntegrityCertificationPacket,
    violations: &mut Vec<M5ContentIntegrityCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in required_source_contract_refs() {
        if !refs.contains(required.as_str()) {
            violations.push(M5ContentIntegrityCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_family_rows(
    packet: &M5ContentIntegrityCertificationPacket,
    violations: &mut Vec<M5ContentIntegrityCertificationViolation>,
) {
    use M5ContentIntegrityCertificationViolation as V;

    let present: BTreeSet<M5ContentIntegrityArtifactFamily> =
        packet.family_rows.iter().map(|row| row.family).collect();
    for required in M5ContentIntegrityArtifactFamily::ALL {
        if !present.contains(&required) {
            violations.push(V::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.family_rows {
        if row.dimension_proofs.is_empty() || row.proof_refs.is_empty() {
            violations.push(V::FamilyRowIncomplete);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(V::ConsumerSurfacesMissing);
        }

        let required_dims: BTreeSet<M5CertificationProofDimension> = row
            .dimension_proofs
            .iter()
            .filter(|proof| proof.applicability == M5CertificationDimensionApplicability::Required)
            .map(|proof| proof.dimension)
            .collect();
        for required in M5CertificationProofDimension::UNIVERSAL_REQUIRED {
            if !required_dims.contains(&required) {
                violations.push(V::RequiredDimensionMissing);
            }
        }
        if row.family.is_strong_decision_surface()
            && !required_dims.contains(&M5CertificationProofDimension::StrongDecisionDisplay)
        {
            violations.push(V::RequiredDimensionMissing);
        }

        for proof in &row.dimension_proofs {
            if proof.applicability == M5CertificationDimensionApplicability::Required
                && proof.backing_packet_ref.trim().is_empty()
            {
                violations.push(V::ProofRefMissing);
            }
            if proof.backing_lane != M5CertificationProofLane::for_dimension(proof.dimension) {
                violations.push(V::FamilyRowIncomplete);
            }
        }

        let (certified, reasons) =
            certify_from_proofs(row.claimed_qualification, &row.dimension_proofs);
        if certified != row.certified_qualification {
            violations.push(V::CertifiedQualificationDrift);
        }
        if reasons != row.narrowing_reasons {
            violations.push(V::NarrowingReasonsInconsistent);
        }
    }
}

fn validate_summary(
    packet: &M5ContentIntegrityCertificationPacket,
    violations: &mut Vec<M5ContentIntegrityCertificationViolation>,
) {
    if packet.summary != derive_summary(&packet.family_rows) {
        violations.push(M5ContentIntegrityCertificationViolation::SummaryCountMismatch);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
