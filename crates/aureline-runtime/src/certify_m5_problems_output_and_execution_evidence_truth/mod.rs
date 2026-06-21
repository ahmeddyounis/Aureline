//! Release-bearing qualification of M5 Problems, output-channel, and
//! execution-evidence truth, with automatic claim-narrowing when a profile's
//! causal-link, confidence, freshness, or reopen evidence is stale or failing.
//!
//! The earlier M5 lanes each freeze one slice of the Problems / output /
//! execution-evidence causal system:
//!
//! - [`crate::m5_execution_evidence_causality_matrix`] froze the per-surface
//!   causality lane matrix;
//! - [`crate::m5_problem_records_source_task_correlation_and_rerun_jump_parity`]
//!   froze the individual Problems row;
//! - [`crate::m5_execution_evidence_projection_overlays`] froze the projected
//!   overlay;
//! - [`crate::m5_task_problem_output_chronology_reuse`] froze the chronology
//!   entry;
//! - [`crate::m5_output_channel_virtualization_trust_and_freshness`] froze the
//!   output channel;
//! - [`crate::m5_structured_versus_heuristic_fallback_drills`] froze the
//!   parse-evidence drill corpus.
//!
//! This module is the **capstone qualification gate** that binds those lanes into
//! the M5 promotion model: it publishes one [`ProfileQualification`] per claimed
//! M5 [`ToolingProfile`] (the Problems panel, output channels, terminal/debug
//! runners, notebook output, pipeline overlays, AI-tool evidence, and support
//! export), each graded across the [`CertificationDimension`]s the source set
//! treats as one causal chain — Problems correlation, output-channel identity,
//! evidence-projection lineage, causal-link integrity, confidence honesty,
//! stale/superseded handling, and reopen-to-origin parity.
//!
//! A profile keeps its claim only while every dimension's invariant holds and its
//! proof is current and reopenable. When the causal link breaks, confidence
//! overclaims, identity is flattened, or the reopen path is lost, the profile
//! **auto-narrows to [`ProfileQualificationGrade::Blocked`]**; when honestly
//! labeled proof has aged out it narrows to
//! [`ProfileQualificationGrade::RetestPending`]; a read-only overlay profile holds
//! at [`ProfileQualificationGrade::Limited`]. A narrowed profile carries a strictly
//! lower effective grade, a recorded [`QualificationNarrowTrigger`], and a precise
//! narrowed label — never a generic non-answer — so About/help/service-health,
//! compatibility, release, support, and AI surfaces ingest one qualification state
//! instead of restating tooling claims by hand.
//!
//! The packet also derives explicit [`ReleaseEvidenceRow`]s for the four
//! release-bearing integrity axes — causal-link integrity, confidence honesty,
//! stale/superseded handling, and reopenable-evidence parity — so a release
//! evidence packet carries those rows directly rather than re-deriving them.
//!
//! [`ProblemsOutputEvidenceCertificationPacket::validate`] refuses a packet that
//! lets a Problems or output pane flatten away provider/run/channel identity,
//! heuristic confidence, or stale/superseded state; that lets an imported overlay
//! read as live local truth; that fails to narrow a profile whose proof cannot back
//! its claim; or that leaks raw boundary material across the export.
//!
//! Raw stdout/stderr bytes, command lines, provider payloads, env bodies, absolute
//! paths, URLs, and secrets never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, fingerprint digests, and redaction-aware
//! reviewable labels.
//!
//! The boundary schema is
//! [`schemas/tooling/m5-problems-output-evidence-certification.schema.json`](../../../../schemas/tooling/m5-problems-output-evidence-certification.schema.json).
//! The contract doc is
//! [`docs/tooling/m5-problems-output-evidence-certification.md`](../../../../docs/tooling/m5-problems-output-evidence-certification.md).
//! The canonical support export is
//! [`artifacts/tooling/m5-problems-output-evidence-certification/support_export.json`](../../../../artifacts/tooling/m5-problems-output-evidence-certification/support_export.json)
//! and the perturbation corpus is
//! [`fixtures/tooling/m5-problems-output-evidence-certification/`](../../../../fixtures/tooling/m5-problems-output-evidence-certification/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_execution_evidence_causality_matrix::{
    ConfidenceTier, FreshnessState, OriginClass, ProofCurrency,
};
use crate::m5_structured_versus_heuristic_fallback_drills::ToolingProfile;

/// Stable record-kind tag carried by [`ProblemsOutputEvidenceCertificationPacket`].
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_RECORD_KIND: &str =
    "m5_problems_output_evidence_certification_packet";

/// Schema version for the qualification packet.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_VERSION: u32 = 1;

/// Taxonomy version for the frozen grade / dimension / trigger vocabularies.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_TAXONOMY_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_REF: &str =
    "schemas/tooling/m5-problems-output-evidence-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_DOC_REF: &str =
    "docs/tooling/m5-problems-output-evidence-certification.md";

/// Repo-relative path of the canonical support export (the source of truth).
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF: &str =
    "artifacts/tooling/m5-problems-output-evidence-certification/support_export.json";

/// Repo-relative path of the generated qualification report.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_REPORT_REF: &str =
    "artifacts/tooling/m5-problems-output-evidence-certification/report.md";

/// Repo-relative path of the generated waiver-and-downgrade log.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_WAIVER_LOG_REF: &str =
    "artifacts/tooling/m5-problems-output-evidence-certification/waiver-and-downgrade-log.md";

/// Repo-relative path of the protected perturbation-corpus directory.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_FIXTURE_DIR: &str =
    "fixtures/tooling/m5-problems-output-evidence-certification";

/// Stable packet id minted by the seed builder.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_PACKET_ID: &str =
    "m5-problems-output-evidence-certification:stable:0001";

/// Canonical upstream lane support exports this capstone certifies over. Every
/// claimed profile draws its proof from these checked-in lane artifacts rather than
/// from a private causal model.
pub const PROBLEMS_OUTPUT_EVIDENCE_CERT_UPSTREAM_LANE_REFS: [&str; 6] = [
    "artifacts/tooling/m5-execution-evidence/support_export.json",
    "artifacts/tooling/m5-problem-records/support_export.json",
    "artifacts/tooling/m5-execution-evidence-projections/support_export.json",
    "artifacts/tooling/m5-chronology-reuse/support_export.json",
    "artifacts/tooling/m5-output-channels/support_export.json",
    "artifacts/tooling/m5-fallback-evidence-drills/support_export.json",
];

/// Allowed packet redaction-class tokens (mirrors the upstream lane vocabulary).
const REDACTION_CLASS_TOKENS: [&str; 4] = [
    "metadata_safe_default",
    "structured_fields_with_path_redaction",
    "support_bundle_scoped",
    "broadened_capture",
];

// --------------------------------------------------------------------------- //
// Frozen qualification vocabularies (mirror the boundary schema).
// --------------------------------------------------------------------------- //

/// One evidence dimension every claimed profile is graded on. The source set treats
/// these as one causal chain, not seven independent checks: a Problems row that
/// cannot correlate to its task, an output channel that flattens its provider
/// identity, an overlay that drops its lineage, a heuristic that reads as
/// structured, a stale state shown as fresh, or a finding that cannot be reopened
/// each breaks the same chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationDimension {
    /// Problems rows correlate to their source task / run / owning output channel.
    ProblemsCorrelation,
    /// Output channels preserve provider / run / channel identity and trust class.
    OutputChannelIdentity,
    /// Projected overlays preserve run / step / provider / artifact lineage.
    EvidenceProjectionLineage,
    /// The structured-versus-heuristic causal chain stays unbroken and single-id.
    CausalLinkIntegrity,
    /// Confidence labels do not overclaim; heuristic origins stay labeled.
    ConfidenceHonesty,
    /// Stale and superseded state stay visible rather than implied current.
    StaleSupersededHandling,
    /// Reopen-to-origin resolves to the canonical evidence across every surface.
    ReopenToOriginParity,
}

impl CertificationDimension {
    /// Every graded dimension, in declaration order. All are required core.
    pub const ALL: [Self; 7] = [
        Self::ProblemsCorrelation,
        Self::OutputChannelIdentity,
        Self::EvidenceProjectionLineage,
        Self::CausalLinkIntegrity,
        Self::ConfidenceHonesty,
        Self::StaleSupersededHandling,
        Self::ReopenToOriginParity,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsCorrelation => "problems_correlation",
            Self::OutputChannelIdentity => "output_channel_identity",
            Self::EvidenceProjectionLineage => "evidence_projection_lineage",
            Self::CausalLinkIntegrity => "causal_link_integrity",
            Self::ConfidenceHonesty => "confidence_honesty",
            Self::StaleSupersededHandling => "stale_superseded_handling",
            Self::ReopenToOriginParity => "reopen_to_origin_parity",
        }
    }

    /// The narrow trigger raised when this dimension's invariant fails.
    pub const fn invariant_trigger(self) -> QualificationNarrowTrigger {
        match self {
            Self::ProblemsCorrelation => QualificationNarrowTrigger::ProblemsCorrelationLost,
            Self::OutputChannelIdentity => {
                QualificationNarrowTrigger::OutputChannelIdentityFlattened
            }
            Self::EvidenceProjectionLineage => {
                QualificationNarrowTrigger::ProjectionLineageFlattened
            }
            Self::CausalLinkIntegrity => QualificationNarrowTrigger::CausalLinkBroken,
            Self::ConfidenceHonesty => QualificationNarrowTrigger::ConfidenceOverclaimed,
            Self::StaleSupersededHandling => QualificationNarrowTrigger::SupersededStateHidden,
            Self::ReopenToOriginParity => QualificationNarrowTrigger::ReopenPathLost,
        }
    }
}

/// Qualification grade a profile claims or effectively holds. A higher [`Self::rank`]
/// is a stronger claim, so a narrowed profile must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileQualificationGrade {
    /// Fully qualified and release-bearing across every dimension.
    Qualified,
    /// Held at a read-only / overlay ceiling: an honest, narrower claim.
    Limited,
    /// Certified, but proof has aged out and must be re-verified.
    RetestPending,
    /// A causal-link, confidence, identity, or reopen invariant is failing.
    Blocked,
    /// Labs / unadvertised; makes no public claim and is never widened.
    LabsNotClaimed,
}

impl ProfileQualificationGrade {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Qualified,
        Self::Limited,
        Self::RetestPending,
        Self::Blocked,
        Self::LabsNotClaimed,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Limited => "limited",
            Self::RetestPending => "retest_pending",
            Self::Blocked => "blocked",
            Self::LabsNotClaimed => "labs_not_claimed",
        }
    }

    /// Ordinal rank; higher is a stronger claim, so a narrow must move strictly
    /// lower. Labs carries no claim and sorts at the floor.
    pub const fn rank(self) -> u8 {
        match self {
            Self::LabsNotClaimed => 0,
            Self::Blocked => 1,
            Self::RetestPending => 2,
            Self::Limited => 3,
            Self::Qualified => 4,
        }
    }

    /// Whether this grade is a valid *claimed* grade (the others are derived only).
    pub const fn is_claimable_grade(self) -> bool {
        matches!(self, Self::Qualified | Self::Limited | Self::LabsNotClaimed)
    }
}

/// Reason a claimed profile auto-narrowed below its claim. The chrome quotes the
/// trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationNarrowTrigger {
    /// A Problems row lost correlation to its source task / run / channel.
    ProblemsCorrelationLost,
    /// An output channel flattened its provider / run / channel identity.
    OutputChannelIdentityFlattened,
    /// A projected overlay flattened its run / step / provider / artifact lineage.
    ProjectionLineageFlattened,
    /// The structured-versus-heuristic causal chain broke or diverged ids.
    CausalLinkBroken,
    /// A confidence label overclaimed its source (heuristic read as structured).
    ConfidenceOverclaimed,
    /// Honestly labeled evidence aged past its freshness window.
    StaleEvidence,
    /// A superseded or stale state was hidden behind a fresh-looking claim.
    SupersededStateHidden,
    /// The reopen-to-origin path to the canonical evidence was lost.
    ReopenPathLost,
    /// A required dimension carries no proof, or proof that fails closed.
    MissingDimensionProof,
    /// A non-overlay profile leaned on imported / provider proof for a live claim.
    ImportedOverlayClaimsLive,
    /// An upstream lane narrowed and dragged this profile down with it.
    UpstreamLaneNarrowed,
}

impl QualificationNarrowTrigger {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProblemsCorrelationLost => "problems_correlation_lost",
            Self::OutputChannelIdentityFlattened => "output_channel_identity_flattened",
            Self::ProjectionLineageFlattened => "projection_lineage_flattened",
            Self::CausalLinkBroken => "causal_link_broken",
            Self::ConfidenceOverclaimed => "confidence_overclaimed",
            Self::StaleEvidence => "stale_evidence",
            Self::SupersededStateHidden => "superseded_state_hidden",
            Self::ReopenPathLost => "reopen_path_lost",
            Self::MissingDimensionProof => "missing_dimension_proof",
            Self::ImportedOverlayClaimsLive => "imported_overlay_claims_live",
            Self::UpstreamLaneNarrowed => "upstream_lane_narrowed",
        }
    }
}

/// The four release-bearing integrity axes a release evidence packet must carry as
/// explicit rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseEvidenceAxis {
    /// The causal chain from finding back to run stays unbroken.
    CausalLinkIntegrity,
    /// Confidence labels stay honest about their source.
    ConfidenceHonesty,
    /// Stale and superseded state stay visible.
    StaleSupersededHandling,
    /// The canonical evidence stays reopenable across every surface.
    ReopenableEvidenceParity,
}

impl ReleaseEvidenceAxis {
    /// Every release-evidence axis, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CausalLinkIntegrity,
        Self::ConfidenceHonesty,
        Self::StaleSupersededHandling,
        Self::ReopenableEvidenceParity,
    ];

    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CausalLinkIntegrity => "causal_link_integrity",
            Self::ConfidenceHonesty => "confidence_honesty",
            Self::StaleSupersededHandling => "stale_superseded_handling",
            Self::ReopenableEvidenceParity => "reopenable_evidence_parity",
        }
    }

    /// The certification dimension this axis rolls up across profiles.
    pub const fn dimension(self) -> CertificationDimension {
        match self {
            Self::CausalLinkIntegrity => CertificationDimension::CausalLinkIntegrity,
            Self::ConfidenceHonesty => CertificationDimension::ConfidenceHonesty,
            Self::StaleSupersededHandling => CertificationDimension::StaleSupersededHandling,
            Self::ReopenableEvidenceParity => CertificationDimension::ReopenToOriginParity,
        }
    }
}

/// Internal per-dimension status, derived from the invariant and the proof currency.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DimensionStatus {
    /// Invariant holds and the proof is current.
    Current,
    /// Invariant holds but honestly labeled proof has aged out.
    Stale,
    /// The invariant is failing, or the proof is missing / fails closed.
    Failing,
}

/// One dimension's qualification: the invariant verdict plus the proof currency and
/// a reopenable evidence object, so a grade is backed by an object a reviewer can
/// reopen rather than an asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DimensionQualification {
    /// Dimension being qualified.
    pub dimension: CertificationDimension,
    /// Whether the dimension's causal / identity / reopen invariant holds.
    pub invariant_holds: bool,
    /// Currency of the proof backing this dimension.
    pub proof_currency: ProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref` is
    /// present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl DimensionQualification {
    /// Whether the proof object is reopenable: a present ref carries a distinct
    /// non-display fingerprint and a non-empty summary.
    pub fn proof_reopenable(&self) -> bool {
        match (&self.proof_ref, &self.proof_fingerprint_token) {
            (Some(reference), Some(fingerprint)) => {
                let reference = reference.trim();
                let fingerprint = fingerprint.trim();
                !reference.is_empty() && !fingerprint.is_empty() && fingerprint != reference
            }
            _ => false,
        }
    }

    /// Whether this qualification is well-formed: a missing proof carries no ref, any
    /// other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if matches!(self.proof_currency, ProofCurrency::MissingProof) {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether the proof currency is a current, locally verified or cached proof.
    fn currency_is_current_local(&self) -> bool {
        matches!(
            self.proof_currency,
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow
        )
    }

    /// Whether the proof currency is a current imported / provider-backed proof.
    fn currency_is_imported(&self) -> bool {
        matches!(self.proof_currency, ProofCurrency::ImportedCurrent)
    }

    /// Derives this dimension's status given the owning profile's overlay posture.
    fn status(&self, overlay_profile: bool) -> DimensionStatus {
        if !self.invariant_holds {
            return DimensionStatus::Failing;
        }
        match self.proof_currency {
            ProofCurrency::MissingProof | ProofCurrency::RequiresReview => DimensionStatus::Failing,
            ProofCurrency::StaleExpired => DimensionStatus::Stale,
            ProofCurrency::ImportedCurrent => {
                // Imported proof is only honest on a read-only overlay profile; a
                // first-party profile leaning on it is claiming live truth it has
                // not verified locally.
                if overlay_profile {
                    DimensionStatus::Current
                } else {
                    DimensionStatus::Failing
                }
            }
            ProofCurrency::VerifiedCurrent | ProofCurrency::CachedWithinWindow => {
                DimensionStatus::Current
            }
        }
    }

    /// Whether this dimension currently backs the profile's claim (current proof and
    /// a holding invariant), used by the release-evidence roll-up.
    fn backs_claim(&self, overlay_profile: bool) -> bool {
        self.invariant_holds
            && (self.currency_is_current_local()
                || (overlay_profile && self.currency_is_imported()))
    }
}

/// Constructor input for one profile qualification: the claim and its per-dimension
/// evidence, before the effective grade is derived.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileQualificationInput {
    /// Claimed M5 tooling profile.
    pub profile: ToolingProfile,
    /// Export-safe summary of the claim under qualification.
    pub claim_summary: String,
    /// Origin class of the representative evidence the profile presents.
    pub origin_class: OriginClass,
    /// Representative source confidence tier for the profile's evidence.
    pub representative_confidence: ConfidenceTier,
    /// Representative freshness state for the profile's evidence.
    pub representative_freshness: FreshnessState,
    /// Grade the profile publicly claims.
    pub claimed_grade: ProfileQualificationGrade,
    /// Per-dimension qualifications.
    pub dimensions: Vec<DimensionQualification>,
    /// Whether the canonical evidence stays reopenable on this profile.
    pub canonical_evidence_reopenable: bool,
    /// Whether a summarizing primary alert still keeps the evidence reopenable.
    pub primary_alert_keeps_evidence_reopenable: bool,
    /// Precise narrowed label, required when the derived grade narrows the claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Upstream lane support exports this profile draws its proof from.
    pub upstream_lane_refs: Vec<String>,
    /// Evidence packet refs backing this profile.
    pub evidence_refs: Vec<String>,
}

/// One claimed M5 tooling profile qualified against its evidence dimensions, with the
/// effective grade and narrow evidence derived from the dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileQualification {
    /// Claimed M5 tooling profile.
    pub profile: ToolingProfile,
    /// Export-safe summary of the claim under qualification.
    pub claim_summary: String,
    /// Origin class of the representative evidence the profile presents.
    pub origin_class: OriginClass,
    /// Representative source confidence tier for the profile's evidence.
    pub representative_confidence: ConfidenceTier,
    /// Representative freshness state for the profile's evidence.
    pub representative_freshness: FreshnessState,
    /// True when this profile is an inherently read-only overlay origin.
    pub overlay_profile: bool,
    /// Per-dimension qualifications.
    pub dimensions: Vec<DimensionQualification>,
    /// Whether the canonical evidence stays reopenable on this profile.
    pub canonical_evidence_reopenable: bool,
    /// Whether a summarizing primary alert still keeps the evidence reopenable.
    pub primary_alert_keeps_evidence_reopenable: bool,
    /// Grade the profile publicly claims.
    pub claimed_grade: ProfileQualificationGrade,
    /// Effective grade after auto-narrowing; equals the claim when every dimension is
    /// current, and ranks strictly below it otherwise.
    pub effective_grade: ProfileQualificationGrade,
    /// Trigger that fired the narrow, required when the profile is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<QualificationNarrowTrigger>,
    /// Precise narrowed label, required when the profile is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Upstream lane support exports this profile draws its proof from.
    pub upstream_lane_refs: Vec<String>,
    /// Evidence packet refs backing this profile.
    pub evidence_refs: Vec<String>,
}

impl ProfileQualification {
    /// Derives a profile qualification from its input, computing the effective grade
    /// and narrow trigger from the per-dimension evidence.
    pub fn derive(input: ProfileQualificationInput) -> Self {
        let overlay_profile = input.origin_class.is_overlay();
        let (effective_grade, narrow_trigger) = derive_outcome(
            input.claimed_grade,
            overlay_profile,
            &input.dimensions,
            input.representative_freshness,
        );
        let narrowed_label = if effective_grade.rank() < input.claimed_grade.rank() {
            input.narrowed_label
        } else {
            None
        };
        Self {
            profile: input.profile,
            claim_summary: input.claim_summary,
            origin_class: input.origin_class,
            representative_confidence: input.representative_confidence,
            representative_freshness: input.representative_freshness,
            overlay_profile,
            dimensions: input.dimensions,
            canonical_evidence_reopenable: input.canonical_evidence_reopenable,
            primary_alert_keeps_evidence_reopenable: input.primary_alert_keeps_evidence_reopenable,
            claimed_grade: input.claimed_grade,
            effective_grade,
            narrow_trigger,
            narrowed_label,
            upstream_lane_refs: input.upstream_lane_refs,
            evidence_refs: input.evidence_refs,
        }
    }

    /// Dimensions qualified by this profile.
    pub fn qualified_dimensions(&self) -> BTreeSet<CertificationDimension> {
        self.dimensions.iter().map(|d| d.dimension).collect()
    }

    /// Resolves a qualification by dimension.
    pub fn dimension(&self, dimension: CertificationDimension) -> Option<&DimensionQualification> {
        self.dimensions.iter().find(|d| d.dimension == dimension)
    }

    /// Whether every required-core dimension is qualified.
    pub fn has_all_dimensions(&self) -> bool {
        let qualified = self.qualified_dimensions();
        CertificationDimension::ALL
            .iter()
            .all(|dimension| qualified.contains(dimension))
    }

    /// Recomputes the effective grade and trigger from the current evidence.
    pub fn rederive(
        &self,
    ) -> (
        ProfileQualificationGrade,
        Option<QualificationNarrowTrigger>,
    ) {
        derive_outcome(
            self.claimed_grade,
            self.overlay_profile,
            &self.dimensions,
            self.representative_freshness,
        )
    }

    /// Whether the profile narrowed below its claim.
    pub fn needs_narrow(&self) -> bool {
        self.effective_grade.rank() < self.claimed_grade.rank()
    }

    /// Whether the profile carries a public qualification claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade != ProfileQualificationGrade::LabsNotClaimed
    }

    /// Whether the stored effective grade and trigger agree with the derivation.
    pub fn derivation_consistent(&self) -> bool {
        let (grade, trigger) = self.rederive();
        self.effective_grade == grade && self.narrow_trigger == trigger
    }

    /// Whether the effective grade and narrow evidence are structurally consistent.
    ///
    /// When the profile is not narrowed the effective grade equals the claim and no
    /// trigger or label is carried; otherwise the effective grade ranks strictly
    /// below the claim and a recorded trigger and precise label are present.
    pub fn narrow_consistent(&self) -> bool {
        if self.needs_narrow() {
            self.narrow_trigger.is_some()
                && self
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
                && self.narrow_trigger.is_none()
                && self.narrowed_label.is_none()
        }
    }

    /// Whether every dimension required to record this profile is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.claim_summary.trim().is_empty()
            && self.claimed_grade.is_claimable_grade()
            && self.has_all_dimensions()
            && self.dimensions.len() == CertificationDimension::ALL.len()
            && self
                .dimensions
                .iter()
                .all(DimensionQualification::is_well_formed)
            && self.canonical_evidence_reopenable
            && self.primary_alert_keeps_evidence_reopenable
            && self.narrow_consistent()
            && self.derivation_consistent()
            && !self.upstream_lane_refs.is_empty()
            && self.upstream_lane_refs.iter().all(|r| !r.trim().is_empty())
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// One release-evidence row rolling up a single integrity axis across the profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReleaseEvidenceRow {
    /// Release-bearing integrity axis.
    pub axis: ReleaseEvidenceAxis,
    /// Certification dimension this axis rolls up.
    pub dimension: CertificationDimension,
    /// Count of claimed profiles whose axis dimension backs its claim.
    pub profiles_holding: usize,
    /// Count of claimed profiles graded for this axis.
    pub profiles_claimed: usize,
    /// Weakest effective grade among the claimed profiles for this axis.
    pub worst_effective_grade: ProfileQualificationGrade,
    /// Export-safe roll-up summary.
    pub summary: String,
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationGuardrails {
    /// Problems and output panes never silently flatten provider / run / channel
    /// identity, heuristic confidence, or stale / superseded state.
    pub panes_never_flatten_identity: bool,
    /// Structured and heuristic origins stay distinct.
    pub structured_and_heuristic_origins_stay_distinct: bool,
    /// Stale and superseded state stay visible rather than implied current.
    pub stale_and_superseded_state_stays_visible: bool,
    /// The canonical evidence stays reopenable from every surface.
    pub canonical_evidence_stays_reopenable: bool,
    /// Imported / provider overlays never read as live local authority.
    pub imported_overlay_never_claims_live: bool,
    /// Stale or failing causal-link / confidence proof auto-narrows the claim.
    pub stale_or_failing_proof_auto_narrows: bool,
    /// A primary alert may summarize a failure, but the evidence stays reopenable.
    pub primary_alert_summary_keeps_evidence_reopenable: bool,
}

impl CertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.panes_never_flatten_identity
            && self.structured_and_heuristic_origins_stay_distinct
            && self.stale_and_superseded_state_stays_visible
            && self.canonical_evidence_stays_reopenable
            && self.imported_overlay_never_claims_live
            && self.stale_or_failing_proof_auto_narrows
            && self.primary_alert_summary_keeps_evidence_reopenable
    }
}

/// Consumer-surface block: the surfaces that ingest this qualification state instead
/// of restating tooling claims by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationConsumerSurfaces {
    /// The About surface ingests this qualification.
    pub about_surface_ingests: bool,
    /// The help surface ingests this qualification.
    pub help_surface_ingests: bool,
    /// The service-health surface ingests this qualification.
    pub service_health_ingests: bool,
    /// The compatibility surface ingests this qualification.
    pub compatibility_surface_ingests: bool,
    /// Release evidence ingests this qualification.
    pub release_evidence_ingests: bool,
    /// Support / export ingests this qualification.
    pub support_export_ingests: bool,
    /// AI evidence ingests this qualification.
    pub ai_evidence_ingests: bool,
    /// Narrowed profiles are visibly labeled below their claim in every surface.
    pub narrowed_profiles_labeled_below_claim: bool,
}

impl CertificationConsumerSurfaces {
    /// Whether every consumer-surface invariant holds.
    pub fn all_hold(&self) -> bool {
        self.about_surface_ingests
            && self.help_surface_ingests
            && self.service_health_ingests
            && self.compatibility_surface_ingests
            && self.release_evidence_ingests
            && self.support_export_ingests
            && self.ai_evidence_ingests
            && self.narrowed_profiles_labeled_below_claim
    }
}

/// Evidence-freshness block: the freshness window wired into release automation so
/// stale proof auto-narrows the affected claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationEvidenceFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed profiles.
    pub auto_narrow_on_stale: bool,
}

impl CertificationEvidenceFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.evidence_freshness_slo_hours > 0
            && !self.last_evidence_refresh.trim().is_empty()
            && self.auto_narrow_on_stale
    }
}

/// Constructor input for [`ProblemsOutputEvidenceCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblemsOutputEvidenceCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable qualification label.
    pub label: String,
    /// Capture timestamp.
    pub as_of: String,
    /// Per-profile qualifications.
    pub profiles: Vec<ProfileQualification>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer-surface block.
    pub consumer_surfaces: CertificationConsumerSurfaces,
    /// Evidence-freshness block.
    pub evidence_freshness: CertificationEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe Problems / output / execution-evidence qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemsOutputEvidenceCertificationPacket {
    /// Record kind; must equal [`PROBLEMS_OUTPUT_EVIDENCE_CERT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Taxonomy version; must equal [`PROBLEMS_OUTPUT_EVIDENCE_CERT_TAXONOMY_VERSION`].
    pub taxonomy_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable qualification label.
    pub label: String,
    /// Capture timestamp.
    pub as_of: String,
    /// Packet redaction-class token.
    pub redaction_class_token: String,
    /// Per-profile qualifications.
    pub profiles: Vec<ProfileQualification>,
    /// Derived release-evidence rows for the four integrity axes.
    pub release_evidence_rows: Vec<ReleaseEvidenceRow>,
    /// Guardrail invariants block.
    pub guardrails: CertificationGuardrails,
    /// Consumer-surface block.
    pub consumer_surfaces: CertificationConsumerSurfaces,
    /// Evidence-freshness block.
    pub evidence_freshness: CertificationEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Upstream lane support exports this capstone certifies over.
    pub upstream_lane_refs: Vec<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ProblemsOutputEvidenceCertificationPacket {
    /// Builds a qualification packet, deriving the release-evidence rows from the
    /// profiles.
    pub fn new(input: ProblemsOutputEvidenceCertificationPacketInput) -> Self {
        let release_evidence_rows = derive_release_evidence_rows(&input.profiles);
        Self {
            record_kind: PROBLEMS_OUTPUT_EVIDENCE_CERT_RECORD_KIND.to_owned(),
            schema_version: PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_VERSION,
            taxonomy_version: PROBLEMS_OUTPUT_EVIDENCE_CERT_TAXONOMY_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            as_of: input.as_of,
            redaction_class_token: input.redaction_class_token,
            profiles: input.profiles,
            release_evidence_rows,
            guardrails: input.guardrails,
            consumer_surfaces: input.consumer_surfaces,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            upstream_lane_refs: PROBLEMS_OUTPUT_EVIDENCE_CERT_UPSTREAM_LANE_REFS
                .iter()
                .map(|r| (*r).to_owned())
                .collect(),
            minted_at: input.minted_at,
        }
    }

    /// Profiles represented by some qualification in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<ToolingProfile> {
        self.profiles.iter().map(|p| p.profile).collect()
    }

    /// Dimensions qualified by some profile in this packet.
    pub fn represented_dimensions(&self) -> BTreeSet<CertificationDimension> {
        self.profiles
            .iter()
            .flat_map(ProfileQualification::qualified_dimensions)
            .collect()
    }

    /// Release-evidence axes represented in this packet.
    pub fn represented_axes(&self) -> BTreeSet<ReleaseEvidenceAxis> {
        self.release_evidence_rows.iter().map(|r| r.axis).collect()
    }

    /// Count of profiles that auto-narrowed below their claim.
    pub fn narrowed_profile_count(&self) -> usize {
        self.profiles.iter().filter(|p| p.needs_narrow()).count()
    }

    /// Count of profiles holding a public claim.
    pub fn claimed_profile_count(&self) -> usize {
        self.profiles.iter().filter(|p| p.is_claimed()).count()
    }

    /// Count of overlay profiles.
    pub fn overlay_profile_count(&self) -> usize {
        self.profiles.iter().filter(|p| p.overlay_profile).count()
    }

    /// Profiles that auto-narrowed below their claim, in packet order.
    pub fn narrowed_profiles(&self) -> Vec<&ProfileQualification> {
        self.profiles.iter().filter(|p| p.needs_narrow()).collect()
    }

    /// Resolves a profile qualification by its tooling profile.
    pub fn profile(&self, profile: ToolingProfile) -> Option<&ProfileQualification> {
        self.profiles.iter().find(|p| p.profile == profile)
    }

    /// Validates the qualification invariants.
    pub fn validate(&self) -> Vec<CertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != PROBLEMS_OUTPUT_EVIDENCE_CERT_RECORD_KIND {
            violations.push(CertificationViolation::WrongRecordKind);
        }
        if self.schema_version != PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_VERSION {
            violations.push(CertificationViolation::WrongSchemaVersion);
        }
        if self.taxonomy_version != PROBLEMS_OUTPUT_EVIDENCE_CERT_TAXONOMY_VERSION {
            violations.push(CertificationViolation::WrongTaxonomyVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CertificationViolation::MissingIdentity);
        }
        if !REDACTION_CLASS_TOKENS.contains(&self.redaction_class_token.as_str()) {
            violations.push(CertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_upstream_lane_refs(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_profiles(self, &mut violations);
        validate_release_evidence_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(CertificationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_surfaces.all_hold() {
            violations.push(CertificationViolation::ConsumerSurfacesIncomplete);
        }
        if !self.evidence_freshness.is_valid() {
            violations.push(CertificationViolation::EvidenceFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("qualification packet serializes"),
        ) {
            violations.push(CertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Whether the packet validates cleanly.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("qualification packet serializes")
    }

    /// Deterministic Markdown qualification report for support, docs, or review
    /// handoff.
    pub fn render_markdown_report(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Problems / Output / Execution-Evidence Qualification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Profiles: {} ({} claimed, {} overlay, {} narrowed)\n",
            self.profiles.len(),
            self.claimed_profile_count(),
            self.overlay_profile_count(),
            self.narrowed_profile_count()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {}, auto-narrow on stale: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh,
            self.evidence_freshness.auto_narrow_on_stale,
        ));

        out.push_str("\n## Profiles\n\n");
        out.push_str("| Profile | Origin | Claimed | Effective | Confidence | Freshness |\n");
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for profile in &self.profiles {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} |\n",
                profile.profile.as_str(),
                profile.origin_class.as_str(),
                profile.claimed_grade.as_str(),
                profile.effective_grade.as_str(),
                profile.representative_confidence.as_str(),
                profile.representative_freshness.as_str(),
            ));
        }

        out.push_str("\n## Release-evidence rows\n\n");
        out.push_str("| Axis | Dimension | Holding | Worst grade |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for row in &self.release_evidence_rows {
            out.push_str(&format!(
                "| {} | {} | {}/{} | {} |\n",
                row.axis.as_str(),
                row.dimension.as_str(),
                row.profiles_holding,
                row.profiles_claimed,
                row.worst_effective_grade.as_str(),
            ));
        }

        let narrowed = self.narrowed_profiles();
        if !narrowed.is_empty() {
            out.push_str("\n## Narrowed profiles\n\n");
            for profile in narrowed {
                out.push_str(&format!(
                    "- `{}`: claim `{}` -> effective `{}`",
                    profile.profile.as_str(),
                    profile.claimed_grade.as_str(),
                    profile.effective_grade.as_str(),
                ));
                if let Some(label) = &profile.narrowed_label {
                    out.push_str(&format!(" — {label}"));
                }
                out.push('\n');
            }
        }
        out
    }

    /// Deterministic Markdown waiver-and-downgrade log: the release-visible record of
    /// every claimed profile currently held below its claim, with the trigger and
    /// label that narrowed it. There are no manual waivers — auto-narrowing is the
    /// only mechanism by which a profile sits below its claim.
    pub fn render_waiver_and_downgrade_log(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Problems / Output / Execution-Evidence Waiver and Downgrade Log\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Generated from: `{}`\n",
            PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh,
        ));
        out.push_str(
            "\nNo manual waivers are granted: a profile sits below its claim only by automatic \
             narrowing when current, reopenable proof cannot back it.\n",
        );
        let narrowed = self.narrowed_profiles();
        out.push_str(&format!(
            "\n## Auto-downgraded profiles ({})\n\n",
            narrowed.len()
        ));
        if narrowed.is_empty() {
            out.push_str("None — every claimed profile holds current proof for its claim.\n");
            return out;
        }
        for profile in narrowed {
            out.push_str(&format!(
                "- **{}**: claim `{}` -> effective `{}`\n",
                profile.profile.as_str(),
                profile.claimed_grade.as_str(),
                profile.effective_grade.as_str(),
            ));
            if let Some(trigger) = profile.narrow_trigger {
                out.push_str(&format!("  - Trigger: `{}`\n", trigger.as_str()));
            }
            if let Some(label) = &profile.narrowed_label {
                out.push_str(&format!("  - {label}\n"));
            }
            let uncurrent: Vec<&str> = profile
                .dimensions
                .iter()
                .filter(|d| !d.backs_claim(profile.overlay_profile))
                .map(|d| d.dimension.as_str())
                .collect();
            if !uncurrent.is_empty() {
                out.push_str(&format!(
                    "  - Uncurrent dimensions: {}\n",
                    uncurrent.join(", ")
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum ProblemsOutputEvidenceCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CertificationViolation>),
}

impl fmt::Display for ProblemsOutputEvidenceCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "problems/output/evidence qualification export parse failed: {error}"
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
                    "problems/output/evidence qualification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ProblemsOutputEvidenceCertificationArtifactError {}

/// Validation failures emitted by
/// [`ProblemsOutputEvidenceCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Packet taxonomy version is wrong.
    WrongTaxonomyVersion,
    /// Required identity / redaction field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// Required upstream lane support-export refs are incomplete.
    MissingUpstreamLaneRefs,
    /// A required claimed profile is represented by no qualification.
    RequiredProfileMissing,
    /// Two qualifications declare the same profile.
    DuplicateProfile,
    /// Some certification dimension is qualified by no profile.
    DimensionCoverageMissing,
    /// No profile demonstrates auto-narrowing on uncurrent proof.
    NarrowedProfileCaseMissing,
    /// No profile holds a fully current claim.
    CurrentProfileCaseMissing,
    /// No read-only overlay profile is present.
    OverlayProfileCaseMissing,
    /// A release-evidence axis is represented by no row.
    ReleaseEvidenceAxisMissing,
    /// A profile is incomplete.
    ProfileIncomplete,
    /// A profile's claimed grade is not a valid claimable grade.
    ClaimedGradeNotAClaim,
    /// A profile's stored effective grade disagrees with the derivation.
    EffectiveGradeDrift,
    /// A profile's stored narrow trigger disagrees with the derivation.
    NarrowTriggerDrift,
    /// A profile narrowed below its claim without a precise label or trigger.
    NarrowedProfileMissingLabelOrTrigger,
    /// A profile failed to narrow below its claim despite uncurrent proof.
    ProfileNotNarrowedOnUncurrentProof,
    /// A dimension proof is not reopenable (missing ref or fingerprint substitutes).
    DimensionProofNotReopenable,
    /// A profile flattened identity / confidence / stale state but stayed green.
    IdentityFlattenedButNotNarrowed,
    /// A non-overlay profile let imported proof back a live local claim.
    ImportedOverlayReadsAsLive,
    /// A profile lacks upstream lane or evidence refs.
    ProfileEvidenceMissing,
    /// A stored release-evidence row disagrees with the derivation.
    ReleaseEvidenceRowDrift,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer-surface block does not satisfy required invariants.
    ConsumerSurfacesIncomplete,
    /// Evidence-freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::WrongTaxonomyVersion => "wrong_taxonomy_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingUpstreamLaneRefs => "missing_upstream_lane_refs",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::DuplicateProfile => "duplicate_profile",
            Self::DimensionCoverageMissing => "dimension_coverage_missing",
            Self::NarrowedProfileCaseMissing => "narrowed_profile_case_missing",
            Self::CurrentProfileCaseMissing => "current_profile_case_missing",
            Self::OverlayProfileCaseMissing => "overlay_profile_case_missing",
            Self::ReleaseEvidenceAxisMissing => "release_evidence_axis_missing",
            Self::ProfileIncomplete => "profile_incomplete",
            Self::ClaimedGradeNotAClaim => "claimed_grade_not_a_claim",
            Self::EffectiveGradeDrift => "effective_grade_drift",
            Self::NarrowTriggerDrift => "narrow_trigger_drift",
            Self::NarrowedProfileMissingLabelOrTrigger => {
                "narrowed_profile_missing_label_or_trigger"
            }
            Self::ProfileNotNarrowedOnUncurrentProof => "profile_not_narrowed_on_uncurrent_proof",
            Self::DimensionProofNotReopenable => "dimension_proof_not_reopenable",
            Self::IdentityFlattenedButNotNarrowed => "identity_flattened_but_not_narrowed",
            Self::ImportedOverlayReadsAsLive => "imported_overlay_reads_as_live",
            Self::ProfileEvidenceMissing => "profile_evidence_missing",
            Self::ReleaseEvidenceRowDrift => "release_evidence_row_drift",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerSurfacesIncomplete => "consumer_surfaces_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_problems_output_evidence_certification_export() -> Result<
    ProblemsOutputEvidenceCertificationPacket,
    ProblemsOutputEvidenceCertificationArtifactError,
> {
    let packet: ProblemsOutputEvidenceCertificationPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/tooling/m5-problems-output-evidence-certification/support_export.json"
        )))
        .map_err(ProblemsOutputEvidenceCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ProblemsOutputEvidenceCertificationArtifactError::Validation(violations))
    }
}

// --------------------------------------------------------------------------- //
// Derivation helpers.
// --------------------------------------------------------------------------- //

/// Derives the effective grade and narrow trigger for one profile from its claim,
/// overlay posture, and per-dimension evidence.
fn derive_outcome(
    claimed_grade: ProfileQualificationGrade,
    overlay_profile: bool,
    dimensions: &[DimensionQualification],
    representative_freshness: FreshnessState,
) -> (
    ProfileQualificationGrade,
    Option<QualificationNarrowTrigger>,
) {
    // A Labs profile makes no public claim and is never widened or narrowed.
    if claimed_grade == ProfileQualificationGrade::LabsNotClaimed {
        return (ProfileQualificationGrade::LabsNotClaimed, None);
    }

    let (ceiling, trigger) = status_ceiling(overlay_profile, dimensions, representative_freshness);
    let effective = if ceiling.rank() <= claimed_grade.rank() {
        ceiling
    } else {
        claimed_grade
    };
    if effective.rank() < claimed_grade.rank() {
        (effective, Some(trigger))
    } else {
        (effective, None)
    }
}

/// Computes the highest grade the evidence can support and the trigger for any
/// shortfall, in a fixed priority order so the most severe cause wins.
fn status_ceiling(
    overlay_profile: bool,
    dimensions: &[DimensionQualification],
    representative_freshness: FreshnessState,
) -> (ProfileQualificationGrade, QualificationNarrowTrigger) {
    // Missing a required dimension fails closed.
    let present: BTreeSet<CertificationDimension> =
        dimensions.iter().map(|d| d.dimension).collect();
    for required in CertificationDimension::ALL {
        if !present.contains(&required) {
            return (
                ProfileQualificationGrade::Blocked,
                QualificationNarrowTrigger::MissingDimensionProof,
            );
        }
    }

    // A broken invariant fails closed, in dimension priority order.
    for dimension in CertificationDimension::ALL {
        if let Some(qualification) = dimensions.iter().find(|d| d.dimension == dimension) {
            if !qualification.invariant_holds {
                return (
                    ProfileQualificationGrade::Blocked,
                    dimension.invariant_trigger(),
                );
            }
        }
    }

    // Missing or review-pending proof fails closed.
    if dimensions.iter().any(|d| {
        matches!(
            d.proof_currency,
            ProofCurrency::MissingProof | ProofCurrency::RequiresReview
        )
    }) {
        return (
            ProfileQualificationGrade::Blocked,
            QualificationNarrowTrigger::MissingDimensionProof,
        );
    }

    // A first-party profile leaning on imported proof is claiming live truth.
    if !overlay_profile
        && dimensions
            .iter()
            .any(|d| matches!(d.proof_currency, ProofCurrency::ImportedCurrent))
    {
        return (
            ProfileQualificationGrade::Blocked,
            QualificationNarrowTrigger::ImportedOverlayClaimsLive,
        );
    }

    // Honestly labeled stale proof narrows to retest-pending.
    let stale = dimensions
        .iter()
        .any(|d| d.status(overlay_profile) == DimensionStatus::Stale)
        || matches!(
            representative_freshness,
            FreshnessState::StaleExpired | FreshnessState::Missing
        );
    if stale {
        return (
            ProfileQualificationGrade::RetestPending,
            QualificationNarrowTrigger::StaleEvidence,
        );
    }

    // Everything current: the evidence can support a full claim. The trigger is a
    // placeholder never surfaced because the effective grade will equal the claim.
    (
        ProfileQualificationGrade::Qualified,
        QualificationNarrowTrigger::StaleEvidence,
    )
}

/// Derives the four release-evidence rows by rolling each integrity axis up across
/// the claimed profiles.
fn derive_release_evidence_rows(profiles: &[ProfileQualification]) -> Vec<ReleaseEvidenceRow> {
    ReleaseEvidenceAxis::ALL
        .into_iter()
        .map(|axis| {
            let dimension = axis.dimension();
            let claimed: Vec<&ProfileQualification> =
                profiles.iter().filter(|p| p.is_claimed()).collect();
            let profiles_claimed = claimed.len();
            let profiles_holding = claimed
                .iter()
                .filter(|p| {
                    p.dimension(dimension)
                        .is_some_and(|d| d.backs_claim(p.overlay_profile))
                })
                .count();
            let worst_effective_grade = claimed
                .iter()
                .map(|p| p.effective_grade)
                .min_by_key(|grade| grade.rank())
                .unwrap_or(ProfileQualificationGrade::Qualified);
            let summary = format!(
                "{} axis: {}/{} claimed profiles hold current proof; weakest effective grade {}",
                axis.as_str(),
                profiles_holding,
                profiles_claimed,
                worst_effective_grade.as_str(),
            );
            ReleaseEvidenceRow {
                axis,
                dimension,
                profiles_holding,
                profiles_claimed,
                worst_effective_grade,
                summary,
            }
        })
        .collect()
}

fn validate_source_contracts(
    packet: &ProblemsOutputEvidenceCertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_REF,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_DOC_REF,
        PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_upstream_lane_refs(
    packet: &ProblemsOutputEvidenceCertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .upstream_lane_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in PROBLEMS_OUTPUT_EVIDENCE_CERT_UPSTREAM_LANE_REFS {
        if !refs.contains(required) {
            violations.push(CertificationViolation::MissingUpstreamLaneRefs);
            break;
        }
    }
}

fn validate_coverage(
    packet: &ProblemsOutputEvidenceCertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let profiles = packet.represented_profiles();
    for required in ToolingProfile::ALL {
        if !profiles.contains(&required) {
            violations.push(CertificationViolation::RequiredProfileMissing);
            break;
        }
    }

    let mut seen: BTreeMap<ToolingProfile, usize> = BTreeMap::new();
    for profile in &packet.profiles {
        *seen.entry(profile.profile).or_insert(0) += 1;
    }
    if seen.values().any(|count| *count > 1) {
        violations.push(CertificationViolation::DuplicateProfile);
    }

    let dimensions = packet.represented_dimensions();
    for required in CertificationDimension::ALL {
        if !dimensions.contains(&required) {
            violations.push(CertificationViolation::DimensionCoverageMissing);
            break;
        }
    }

    let axes = packet.represented_axes();
    for required in ReleaseEvidenceAxis::ALL {
        if !axes.contains(&required) {
            violations.push(CertificationViolation::ReleaseEvidenceAxisMissing);
            break;
        }
    }

    if !packet
        .profiles
        .iter()
        .any(|p| p.needs_narrow() && p.narrow_consistent())
    {
        violations.push(CertificationViolation::NarrowedProfileCaseMissing);
    }

    if !packet.profiles.iter().any(|p| {
        p.claimed_grade == ProfileQualificationGrade::Qualified
            && p.effective_grade == ProfileQualificationGrade::Qualified
    }) {
        violations.push(CertificationViolation::CurrentProfileCaseMissing);
    }

    if packet.overlay_profile_count() == 0 {
        violations.push(CertificationViolation::OverlayProfileCaseMissing);
    }
}

fn validate_profiles(
    packet: &ProblemsOutputEvidenceCertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    for profile in &packet.profiles {
        if !profile.is_complete() {
            violations.push(CertificationViolation::ProfileIncomplete);
        }
        if !profile.claimed_grade.is_claimable_grade() {
            violations.push(CertificationViolation::ClaimedGradeNotAClaim);
        }
        let (grade, trigger) = profile.rederive();
        if profile.effective_grade != grade {
            violations.push(CertificationViolation::EffectiveGradeDrift);
        }
        if profile.narrow_trigger != trigger {
            violations.push(CertificationViolation::NarrowTriggerDrift);
        }
        if profile.needs_narrow()
            && (profile.narrow_trigger.is_none()
                || !profile
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(CertificationViolation::NarrowedProfileMissingLabelOrTrigger);
        }
        // A profile whose evidence cannot back its claim must move strictly below it.
        if grade.rank() < profile.claimed_grade.rank()
            && profile.effective_grade.rank() >= profile.claimed_grade.rank()
        {
            violations.push(CertificationViolation::ProfileNotNarrowedOnUncurrentProof);
        }
        // A broken invariant or imported-on-local proof that did not narrow the claim.
        let has_broken_invariant = profile.dimensions.iter().any(|d| !d.invariant_holds);
        let imported_on_local = !profile.overlay_profile
            && profile
                .dimensions
                .iter()
                .any(|d| matches!(d.proof_currency, ProofCurrency::ImportedCurrent));
        if (has_broken_invariant || imported_on_local) && !profile.needs_narrow() {
            violations.push(CertificationViolation::IdentityFlattenedButNotNarrowed);
        }
        if imported_on_local
            && profile.effective_grade != ProfileQualificationGrade::Blocked
            && profile.is_claimed()
        {
            violations.push(CertificationViolation::ImportedOverlayReadsAsLive);
        }
        if profile.dimensions.iter().any(|d| !d.is_well_formed()) {
            violations.push(CertificationViolation::DimensionProofNotReopenable);
        }
        if profile.upstream_lane_refs.is_empty()
            || profile
                .upstream_lane_refs
                .iter()
                .any(|r| r.trim().is_empty())
            || profile.evidence_refs.is_empty()
            || profile.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(CertificationViolation::ProfileEvidenceMissing);
        }
    }
}

fn validate_release_evidence_rows(
    packet: &ProblemsOutputEvidenceCertificationPacket,
    violations: &mut Vec<CertificationViolation>,
) {
    let expected = derive_release_evidence_rows(&packet.profiles);
    if packet.release_evidence_rows != expected {
        violations.push(CertificationViolation::ReleaseEvidenceRowDrift);
    }
}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise narrow truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "blocked"
            | "limited"
    )
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

// --------------------------------------------------------------------------- //
// Canonical seed builder and perturbation corpus.
// --------------------------------------------------------------------------- //

/// One perturbation corpus case: a derived profile qualification plus the effective
/// outcome it must re-derive to. The Rust tests and the release tool each re-derive
/// from the input and assert the expected outcome, so the checked-in corpus can never
/// imply a wider claim than the current evidence backs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationCorpusCase {
    /// Stable case id.
    pub case_id: String,
    /// Support-safe description of what the case proves.
    pub description: String,
    /// The profile input under test.
    pub input: ProfileQualificationInput,
    /// The outcome the input must re-derive to.
    pub expected: ExpectedQualificationOutcome,
}

impl QualificationCorpusCase {
    /// Re-derives the profile and returns whether it matches the expected outcome,
    /// with a human-readable mismatch description on failure.
    pub fn check(&self) -> Result<(), String> {
        let derived = ProfileQualification::derive(self.input.clone());
        let narrowed = derived.needs_narrow();
        if derived.effective_grade != self.expected.effective_grade {
            return Err(format!(
                "{}: effective grade {} != expected {}",
                self.case_id,
                derived.effective_grade.as_str(),
                self.expected.effective_grade.as_str(),
            ));
        }
        if narrowed != self.expected.narrowed {
            return Err(format!(
                "{}: narrowed {} != expected {}",
                self.case_id, narrowed, self.expected.narrowed
            ));
        }
        if derived.narrow_trigger != self.expected.narrow_trigger {
            return Err(format!(
                "{}: trigger {:?} != expected {:?}",
                self.case_id,
                derived
                    .narrow_trigger
                    .map(QualificationNarrowTrigger::as_str),
                self.expected
                    .narrow_trigger
                    .map(QualificationNarrowTrigger::as_str),
            ));
        }
        Ok(())
    }
}

/// The outcome a corpus case must re-derive to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedQualificationOutcome {
    /// Effective grade the input must derive.
    pub effective_grade: ProfileQualificationGrade,
    /// Whether the input narrows below its claim.
    pub narrowed: bool,
    /// Trigger the input must derive, when narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<QualificationNarrowTrigger>,
}

/// The manifest header written alongside the perturbation corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QualificationCorpusIndex {
    /// Stable corpus id.
    pub corpus_id: String,
    /// Support-safe description of the corpus.
    pub description: String,
    /// Ref of the canonical support export the corpus perturbs from.
    pub source_set_ref: String,
    /// Ordered list of case file names.
    pub cases: Vec<String>,
}

/// Builds a well-formed all-current dimension set for a profile, given a per-dimension
/// proof currency to apply uniformly.
fn current_dimensions(
    profile: ToolingProfile,
    currency: ProofCurrency,
) -> Vec<DimensionQualification> {
    let token = profile.as_str();
    CertificationDimension::ALL
        .into_iter()
        .map(|dimension| DimensionQualification {
            dimension,
            invariant_holds: true,
            proof_currency: currency,
            proof_ref: Some(format!("proof:{token}:{}", dimension.as_str())),
            proof_fingerprint_token: Some(format!("fp-{token}-{}", dimension.as_str())),
            summary: format!(
                "{} proof for {} on the {} profile",
                dimension.as_str(),
                currency.as_str(),
                token
            ),
        })
        .collect()
}

/// Builds the canonical, healthy profile input for one tooling profile.
fn canonical_profile_input(profile: ToolingProfile) -> ProfileQualificationInput {
    let token = profile.as_str();
    let evidence_refs = vec![
        format!("evidence:{token}:problems"),
        format!("evidence:{token}:output"),
        format!("evidence:{token}:projection"),
    ];
    let upstream_lane_refs = PROBLEMS_OUTPUT_EVIDENCE_CERT_UPSTREAM_LANE_REFS
        .iter()
        .map(|r| (*r).to_owned())
        .collect();

    // Each profile certifies the hardest path it actually presents.
    match profile {
        ToolingProfile::PipelineOverlay => {
            // A read-only overlay over an imported provider run: honest, narrower claim.
            let mut dimensions = current_dimensions(profile, ProofCurrency::ImportedCurrent);
            // The overlay's own structured/heuristic distinction stays cached-local proof.
            if let Some(d) = dimensions
                .iter_mut()
                .find(|d| d.dimension == CertificationDimension::ConfidenceHonesty)
            {
                d.proof_currency = ProofCurrency::CachedWithinWindow;
                d.proof_ref = Some(format!("proof:{token}:confidence"));
                d.proof_fingerprint_token = Some(format!("fp-{token}-confidence"));
            }
            ProfileQualificationInput {
                profile,
                claim_summary: "pipeline overlays render imported provider runs as a read-only \
                                overlay that never claims live local authority"
                    .to_owned(),
                origin_class: OriginClass::PipelineProviderRun,
                representative_confidence: ConfidenceTier::ProviderMapped,
                representative_freshness: FreshnessState::CachedWithinWindow,
                claimed_grade: ProfileQualificationGrade::Limited,
                dimensions,
                canonical_evidence_reopenable: true,
                primary_alert_keeps_evidence_reopenable: true,
                narrowed_label: None,
                upstream_lane_refs,
                evidence_refs,
            }
        }
        ToolingProfile::NotebookOutput => {
            // A claimed-stable profile whose freshness proof has honestly aged out:
            // it auto-narrows to retest-pending while staying reopenable.
            let mut dimensions = current_dimensions(profile, ProofCurrency::VerifiedCurrent);
            if let Some(d) = dimensions
                .iter_mut()
                .find(|d| d.dimension == CertificationDimension::StaleSupersededHandling)
            {
                d.proof_currency = ProofCurrency::StaleExpired;
                d.summary = "stale/superseded handling proof aged past the freshness window; \
                             lineage stays reopenable until re-verified"
                    .to_owned();
            }
            ProfileQualificationInput {
                profile,
                claim_summary: "notebook cell output reuses the canonical run/channel evidence \
                                with structured truth"
                    .to_owned(),
                origin_class: OriginClass::NotebookRun,
                representative_confidence: ConfidenceTier::StructuredFull,
                representative_freshness: FreshnessState::StaleExpired,
                claimed_grade: ProfileQualificationGrade::Qualified,
                dimensions,
                canonical_evidence_reopenable: true,
                primary_alert_keeps_evidence_reopenable: true,
                narrowed_label: Some(
                    "Held at retest_pending below the qualified claim: stale/superseded handling \
                     proof aged out; reopen-to-origin stays available until re-verified"
                        .to_owned(),
                ),
                upstream_lane_refs,
                evidence_refs,
            }
        }
        _ => {
            let (origin_class, confidence, summary) = match profile {
                ToolingProfile::ProblemsPanel => (
                    OriginClass::LocalTask,
                    ConfidenceTier::StructuredFull,
                    "Problems rows correlate each finding to its source task, run, and owning \
                     output channel with structured truth",
                ),
                ToolingProfile::OutputChannel => (
                    OriginClass::LocalTask,
                    ConfidenceTier::StructuredFull,
                    "output channels preserve provider/run/channel identity and the content \
                     trust class with stream-first virtualization",
                ),
                ToolingProfile::TerminalRunner => (
                    OriginClass::LocalTask,
                    ConfidenceTier::HeuristicHigh,
                    "the terminal task runner keeps the heuristic parse visibly distinct from \
                     structured truth and keeps a raw-output backlink",
                ),
                ToolingProfile::DebugConsole => (
                    OriginClass::LocalDebugSession,
                    ConfidenceTier::StructuredFull,
                    "the debug console reuses the canonical session/run evidence with structured \
                     truth",
                ),
                ToolingProfile::AiToolEvidence => (
                    OriginClass::AiTriggeredRun,
                    ConfidenceTier::StructuredFull,
                    "AI-tool evidence packets reuse the canonical run/channel/problem objects \
                     rather than restating a parallel causal model",
                ),
                ToolingProfile::SupportExport => (
                    OriginClass::HeadlessAutomation,
                    ConfidenceTier::StructuredFull,
                    "support exports stay self-contained and reopenable without the originating \
                     UI state",
                ),
                ToolingProfile::PipelineOverlay | ToolingProfile::NotebookOutput => unreachable!(),
            };
            ProfileQualificationInput {
                profile,
                claim_summary: summary.to_owned(),
                origin_class,
                representative_confidence: confidence,
                representative_freshness: FreshnessState::Live,
                claimed_grade: ProfileQualificationGrade::Qualified,
                dimensions: current_dimensions(profile, ProofCurrency::VerifiedCurrent),
                canonical_evidence_reopenable: true,
                primary_alert_keeps_evidence_reopenable: true,
                narrowed_label: None,
                upstream_lane_refs,
                evidence_refs,
            }
        }
    }
}

/// Builds the canonical stable qualification packet input.
pub fn current_m5_problems_output_evidence_certification_input(
) -> ProblemsOutputEvidenceCertificationPacketInput {
    let profiles = ToolingProfile::ALL
        .into_iter()
        .map(|profile| ProfileQualification::derive(canonical_profile_input(profile)))
        .collect();
    ProblemsOutputEvidenceCertificationPacketInput {
        packet_id: PROBLEMS_OUTPUT_EVIDENCE_CERT_PACKET_ID.to_owned(),
        label: "M5 Problems, output-channel, and execution-evidence qualification with \
                automatic claim-narrowing on stale or failing causal-link and confidence proof"
            .to_owned(),
        as_of: "2026-06-21T00:00:00Z".to_owned(),
        profiles,
        guardrails: CertificationGuardrails {
            panes_never_flatten_identity: true,
            structured_and_heuristic_origins_stay_distinct: true,
            stale_and_superseded_state_stays_visible: true,
            canonical_evidence_stays_reopenable: true,
            imported_overlay_never_claims_live: true,
            stale_or_failing_proof_auto_narrows: true,
            primary_alert_summary_keeps_evidence_reopenable: true,
        },
        consumer_surfaces: CertificationConsumerSurfaces {
            about_surface_ingests: true,
            help_surface_ingests: true,
            service_health_ingests: true,
            compatibility_surface_ingests: true,
            release_evidence_ingests: true,
            support_export_ingests: true,
            ai_evidence_ingests: true,
            narrowed_profiles_labeled_below_claim: true,
        },
        evidence_freshness: CertificationEvidenceFreshness {
            evidence_freshness_slo_hours: 168,
            last_evidence_refresh: "2026-06-21T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            PROBLEMS_OUTPUT_EVIDENCE_CERT_SCHEMA_REF.to_owned(),
            PROBLEMS_OUTPUT_EVIDENCE_CERT_DOC_REF.to_owned(),
            PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-21T00:00:00Z".to_owned(),
    }
}

/// Materializes the canonical stable qualification packet.
pub fn seeded_m5_problems_output_evidence_certification_packet(
) -> ProblemsOutputEvidenceCertificationPacket {
    ProblemsOutputEvidenceCertificationPacket::new(
        current_m5_problems_output_evidence_certification_input(),
    )
}

/// Builds the perturbation corpus: one case per narrowing trigger, each starting from
/// a healthy profile input and breaking exactly one invariant or aging one proof.
pub fn seeded_m5_problems_output_evidence_certification_corpus() -> Vec<QualificationCorpusCase> {
    let mut cases = Vec::new();

    // A clean qualified profile stays qualified.
    cases.push(QualificationCorpusCase {
        case_id: "clean-qualified".to_owned(),
        description: "A clean Problems-panel profile certifies across every dimension.".to_owned(),
        input: canonical_profile_input(ToolingProfile::ProblemsPanel),
        expected: ExpectedQualificationOutcome {
            effective_grade: ProfileQualificationGrade::Qualified,
            narrowed: false,
            narrow_trigger: None,
        },
    });

    // A clean overlay profile holds at its limited ceiling.
    cases.push(QualificationCorpusCase {
        case_id: "overlay-limited-holds".to_owned(),
        description: "A read-only pipeline overlay holds at its limited claim when current."
            .to_owned(),
        input: canonical_profile_input(ToolingProfile::PipelineOverlay),
        expected: ExpectedQualificationOutcome {
            effective_grade: ProfileQualificationGrade::Limited,
            narrowed: false,
            narrow_trigger: None,
        },
    });

    // A stale-but-honest profile narrows to retest-pending.
    cases.push(QualificationCorpusCase {
        case_id: "stale-narrows-to-retest".to_owned(),
        description: "A claimed-stable profile whose freshness proof aged out narrows to \
                      retest-pending while staying reopenable."
            .to_owned(),
        input: canonical_profile_input(ToolingProfile::NotebookOutput),
        expected: ExpectedQualificationOutcome {
            effective_grade: ProfileQualificationGrade::RetestPending,
            narrowed: true,
            narrow_trigger: Some(QualificationNarrowTrigger::StaleEvidence),
        },
    });

    // One blocking case per invariant-bearing dimension.
    let broken_dimension_cases = [
        (
            "problems-correlation-lost",
            CertificationDimension::ProblemsCorrelation,
            QualificationNarrowTrigger::ProblemsCorrelationLost,
            "A Problems row that loses correlation to its source task blocks the claim.",
        ),
        (
            "output-channel-identity-flattened",
            CertificationDimension::OutputChannelIdentity,
            QualificationNarrowTrigger::OutputChannelIdentityFlattened,
            "An output channel that flattens its provider/run/channel identity blocks.",
        ),
        (
            "projection-lineage-flattened",
            CertificationDimension::EvidenceProjectionLineage,
            QualificationNarrowTrigger::ProjectionLineageFlattened,
            "A projected overlay that flattens its run/step/provider lineage blocks.",
        ),
        (
            "causal-link-broken",
            CertificationDimension::CausalLinkIntegrity,
            QualificationNarrowTrigger::CausalLinkBroken,
            "A finding whose structured-versus-heuristic causal chain breaks blocks.",
        ),
        (
            "confidence-overclaimed",
            CertificationDimension::ConfidenceHonesty,
            QualificationNarrowTrigger::ConfidenceOverclaimed,
            "A heuristic parse that reads as structured truth blocks the claim.",
        ),
        (
            "superseded-state-hidden",
            CertificationDimension::StaleSupersededHandling,
            QualificationNarrowTrigger::SupersededStateHidden,
            "A superseded state hidden behind a fresh-looking claim blocks.",
        ),
        (
            "reopen-path-lost",
            CertificationDimension::ReopenToOriginParity,
            QualificationNarrowTrigger::ReopenPathLost,
            "A finding whose reopen-to-origin path is lost blocks the claim.",
        ),
    ];
    for (case_id, dimension, trigger, description) in broken_dimension_cases {
        let mut input = canonical_profile_input(ToolingProfile::ProblemsPanel);
        if let Some(d) = input
            .dimensions
            .iter_mut()
            .find(|d| d.dimension == dimension)
        {
            d.invariant_holds = false;
            d.summary = format!("{} invariant is failing", dimension.as_str());
        }
        input.narrowed_label = Some(format!(
            "Blocked below the qualified claim: {} invariant failing; canonical evidence stays \
             reopenable via the raw-output backlink",
            dimension.as_str()
        ));
        cases.push(QualificationCorpusCase {
            case_id: case_id.to_owned(),
            description: description.to_owned(),
            input,
            expected: ExpectedQualificationOutcome {
                effective_grade: ProfileQualificationGrade::Blocked,
                narrowed: true,
                narrow_trigger: Some(trigger),
            },
        });
    }

    // Missing proof on a required dimension fails closed.
    {
        let mut input = canonical_profile_input(ToolingProfile::OutputChannel);
        if let Some(d) = input
            .dimensions
            .iter_mut()
            .find(|d| d.dimension == CertificationDimension::ReopenToOriginParity)
        {
            d.proof_currency = ProofCurrency::MissingProof;
            d.proof_ref = None;
            d.proof_fingerprint_token = None;
            d.summary = "no reopen-to-origin proof object exists".to_owned();
        }
        input.narrowed_label = Some(
            "Blocked below the qualified claim: a required dimension carries no proof; the \
             primary alert still keeps the evidence reopenable"
                .to_owned(),
        );
        cases.push(QualificationCorpusCase {
            case_id: "missing-dimension-proof".to_owned(),
            description: "A required dimension with no proof object fails closed.".to_owned(),
            input,
            expected: ExpectedQualificationOutcome {
                effective_grade: ProfileQualificationGrade::Blocked,
                narrowed: true,
                narrow_trigger: Some(QualificationNarrowTrigger::MissingDimensionProof),
            },
        });
    }

    // A first-party profile leaning on imported proof claims live truth it has not
    // locally verified.
    {
        let mut input = canonical_profile_input(ToolingProfile::DebugConsole);
        if let Some(d) = input
            .dimensions
            .iter_mut()
            .find(|d| d.dimension == CertificationDimension::CausalLinkIntegrity)
        {
            d.proof_currency = ProofCurrency::ImportedCurrent;
            d.summary =
                "causal-link proof is imported provider evidence, not locally verified".to_owned();
        }
        input.narrowed_label = Some(
            "Blocked below the qualified claim: a first-party profile leaned on imported \
             provider proof; the imported overlay stays read-only and reopenable"
                .to_owned(),
        );
        cases.push(QualificationCorpusCase {
            case_id: "imported-overlay-claims-live".to_owned(),
            description: "A first-party profile leaning on imported proof blocks the live claim."
                .to_owned(),
            input,
            expected: ExpectedQualificationOutcome {
                effective_grade: ProfileQualificationGrade::Blocked,
                narrowed: true,
                narrow_trigger: Some(QualificationNarrowTrigger::ImportedOverlayClaimsLive),
            },
        });
    }

    // A Labs profile makes no public claim and is never widened or narrowed.
    {
        let mut input = canonical_profile_input(ToolingProfile::AiToolEvidence);
        input.claimed_grade = ProfileQualificationGrade::LabsNotClaimed;
        input.claim_summary =
            "an unadvertised AI-tool evidence lane makes no public qualification claim".to_owned();
        cases.push(QualificationCorpusCase {
            case_id: "labs-not-claimed".to_owned(),
            description: "An unadvertised Labs profile makes no claim and is never narrowed."
                .to_owned(),
            input,
            expected: ExpectedQualificationOutcome {
                effective_grade: ProfileQualificationGrade::LabsNotClaimed,
                narrowed: false,
                narrow_trigger: None,
            },
        });
    }

    cases
}

/// Builds the corpus index manifest for the perturbation cases.
pub fn seeded_m5_problems_output_evidence_certification_corpus_index() -> QualificationCorpusIndex {
    QualificationCorpusIndex {
        corpus_id: "m5-problems-output-evidence-certification-corpus:0001".to_owned(),
        description:
            "Perturbation corpus for the Problems/output/execution-evidence qualification \
                      engine. Each case starts from a healthy profile input, breaks exactly one \
                      causal-link/confidence/reopen invariant or ages one proof, and asserts the \
                      re-derived effective grade, narrowed flag, and narrow trigger."
                .to_owned(),
        source_set_ref: PROBLEMS_OUTPUT_EVIDENCE_CERT_SUPPORT_EXPORT_REF.to_owned(),
        cases: seeded_m5_problems_output_evidence_certification_corpus()
            .into_iter()
            .map(|case| format!("{}.json", case.case_id))
            .collect(),
    }
}
