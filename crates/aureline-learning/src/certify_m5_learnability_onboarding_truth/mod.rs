//! Release-bearing certification of learning-mode, guided-exercise,
//! progress-snapshot, educational-AI, and offline/mirror docs-pack truth on
//! every claimed M5 feature-family onboarding row.
//!
//! Where the per-lane learnability modules in this crate (tour/glossary packs,
//! [`crate::guided_exercise_rails`], [`crate::progress_snapshots`],
//! [`crate::educational_ai_and_contextual_cards`], and
//! [`crate::learning_mode_profiles`]) define and validate *each* governed
//! learnability artifact, this module certifies whether each claimed M5
//! **feature-family onboarding row** — notebook, request/API, database, profiler,
//! docs/browser, preview, framework/template, companion, or sync/offboarding —
//! actually carries *current* evidence for every learnability dimension it
//! claims. It is the capstone gate: a marketed onboarding row may keep its
//! certification grade only when its command-backed tour, guided-exercise,
//! user-owned progress, cited educational-AI, and offline/mirror continuity proof
//! is present, reopenable, and inside its freshness window. A row that loses
//! current proof auto-narrows below its claim instead of coasting on an adjacent
//! green row.
//!
//! * a [`CertifiedLearnabilityRow`] ties a durable [`CertifiedLearnabilitySubject`]
//!   (keyed by an [`M5LearningSurfaceFamily`], a mirror-served continuity flag, and
//!   a non-display fingerprint distinct from its id) to a list of
//!   [`LearnabilityDimensionCertification`] rows over the
//!   [`LearnabilityEvidenceDimension`] vocabulary, a claimed
//!   [`LearnabilityCertificationGrade`], an effective grade, and — when narrowed — a
//!   [`LearnabilityCertificationNarrowTrigger`] plus a precise narrowed label;
//! * each [`LearnabilityDimensionCertification`] is **evidence-bound, not
//!   asserted**: it names a [`LearnabilityProofCurrency`] and, unless the proof is
//!   missing, a reopenable `proof_ref` keyed by a non-display fingerprint, so
//!   certification review can reopen the same tour / exercise / progress /
//!   educational-AI / offline-mirror evidence object that backs the grade;
//! * the row **auto-narrows**: [`CertifiedLearnabilityRow::needs_narrow`] is true
//!   whenever a required-core dimension is uncertified or any certified dimension
//!   lacks current proof (stale, missing, requires-review, or mirror proof standing
//!   in for a live local claim). A narrowed row must carry an effective grade
//!   strictly below its claim, a recorded trigger, and a precise label — never a
//!   generic non-answer.
//!
//! [`LearnabilityCertificationPacket::validate`] also refuses a packet that lets a
//! tour or hint route bypass the command graph, lets onboarding progress widen
//! into repo/collaborator-visible telemetry, lets a mirrored/offline pack read as
//! live authoritative content, lets educational AI mutate live state outside the
//! standard preview/approval model, or traps experts inside a tutorial.
//!
//! Raw progress bodies, speaker notes, repository contents, provider payloads,
//! credentials, and raw docs-pack bytes never cross this boundary; the packet
//! carries only typed class tokens, booleans, opaque ids, fingerprint digests, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/help/m5-learnability-cert-report.schema.json`](../../../../schemas/help/m5-learnability-cert-report.schema.json).
//! The contract doc is
//! [`docs/m5/learnability-certification.md`](../../../../docs/m5/learnability-certification.md).
//! The protected fixture directory is
//! [`fixtures/help/m5/certification-corpus/`](../../../../fixtures/help/m5/certification-corpus/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_feature_family_learning_rails::M5LearningSurfaceFamily;

/// Stable record-kind tag carried by [`LearnabilityCertificationPacket`].
pub const LEARNABILITY_CERT_RECORD_KIND: &str = "certify_m5_learnability_onboarding_truth_packet";

/// Schema version for the learnability certification packet.
pub const LEARNABILITY_CERT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const LEARNABILITY_CERT_SCHEMA_REF: &str =
    "schemas/help/m5-learnability-cert-report.schema.json";

/// Repo-relative path of the contract doc.
pub const LEARNABILITY_CERT_DOC_REF: &str = "docs/m5/learnability-certification.md";

/// Repo-relative path of the checked support-export artifact.
pub const LEARNABILITY_CERT_ARTIFACT_REF: &str =
    "artifacts/m5/learnability/certification-report/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const LEARNABILITY_CERT_SUMMARY_REF: &str =
    "artifacts/m5/learnability/certification-report/support_export.md";

/// Repo-relative path of the checked waiver-and-downgrade log.
pub const LEARNABILITY_CERT_WAIVER_LOG_REF: &str =
    "artifacts/m5/learnability/waiver-and-downgrade-log/support_export.md";

/// Repo-relative path of the protected fixture directory.
pub const LEARNABILITY_CERT_FIXTURE_DIR: &str = "fixtures/help/m5/certification-corpus";

/// One learnability evidence dimension a feature-family onboarding row is
/// certified against. The first five are the **required core** every claimed row
/// must certify; the learning-mode profile is a dimension a row certifies only
/// when it ships a learning-mode profile of its own.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityEvidenceDimension {
    /// Command-backed guided-tour truth (stable target refs, prerequisites,
    /// citations, no hidden mutating shortcut).
    GuidedTour,
    /// Guided-exercise rail truth (per-step success criteria, hint/reveal/reset/
    /// skip, sandbox/reversible preference).
    GuidedExercise,
    /// User-owned, restart-safe progress-snapshot truth (local-first, redacted
    /// export, explicit reset disclosure).
    ProgressSnapshot,
    /// Cited educational-AI truth (repository-truth citations, explain separate
    /// from do behind preview/approval).
    EducationalAi,
    /// Offline / mirrored docs-pack continuity truth (explicit freshness, no dead
    /// links, never reads as live authoritative content).
    OfflineMirror,
    /// Opt-in learning-mode profile truth (tip intensity, jargon level, explain-
    /// versus-do posture, reversible controls).
    LearningModeProfile,
}

impl LearnabilityEvidenceDimension {
    /// Every evidence dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::GuidedTour,
        Self::GuidedExercise,
        Self::ProgressSnapshot,
        Self::EducationalAi,
        Self::OfflineMirror,
        Self::LearningModeProfile,
    ];

    /// The required-core dimensions every claimed onboarding row must certify.
    pub const REQUIRED_CORE: [Self; 5] = [
        Self::GuidedTour,
        Self::GuidedExercise,
        Self::ProgressSnapshot,
        Self::EducationalAi,
        Self::OfflineMirror,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GuidedTour => "guided_tour",
            Self::GuidedExercise => "guided_exercise",
            Self::ProgressSnapshot => "progress_snapshot",
            Self::EducationalAi => "educational_ai",
            Self::OfflineMirror => "offline_mirror",
            Self::LearningModeProfile => "learning_mode_profile",
        }
    }

    /// Whether this dimension is part of the required core.
    pub const fn is_core(self) -> bool {
        matches!(
            self,
            Self::GuidedTour
                | Self::GuidedExercise
                | Self::ProgressSnapshot
                | Self::EducationalAi
                | Self::OfflineMirror
        )
    }
}

/// Currency of the proof backing one dimension certification. Only a current,
/// reopenable proof backs a claim; a stale, missing, mirror-on-live, or
/// review-pending proof auto-narrows the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityProofCurrency {
    /// A fresh local proof verified inside its freshness window.
    VerifiedCurrent,
    /// A cached local pack still inside its freshness window.
    CachedWithinWindow,
    /// A current proof served from a disclosed offline / mirrored pack, read-only.
    MirrorCurrent,
    /// A proof that exists but has aged outside its freshness window.
    StaleExpired,
    /// No proof object exists for this dimension.
    MissingProof,
    /// An educational-AI or provider verdict that still requires review and fails
    /// closed.
    RequiresReview,
}

impl LearnabilityProofCurrency {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VerifiedCurrent => "verified_current",
            Self::CachedWithinWindow => "cached_within_window",
            Self::MirrorCurrent => "mirror_current",
            Self::StaleExpired => "stale_expired",
            Self::MissingProof => "missing_proof",
            Self::RequiresReview => "requires_review",
        }
    }

    /// Whether this is a current, locally verified or cached proof.
    pub const fn is_current_local(self) -> bool {
        matches!(self, Self::VerifiedCurrent | Self::CachedWithinWindow)
    }

    /// Whether this is a current mirror / offline-pack proof.
    pub const fn is_mirror_current(self) -> bool {
        matches!(self, Self::MirrorCurrent)
    }

    /// Whether this currency carries no proof object (only [`Self::MissingProof`]).
    pub const fn is_absent(self) -> bool {
        matches!(self, Self::MissingProof)
    }
}

/// Certification grade a row claims or effectively holds. Higher [`Self::rank`] is
/// a stronger claim, so a narrowed row must move strictly lower.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityCertificationGrade {
    /// Fully certified and release-bearing.
    ReleaseCertified,
    /// Certified, publicly claimed.
    Certified,
    /// Provisionally certified (e.g. mirror-current evidence only).
    ProvisionallyCertified,
    /// Not certified; held below a public claim.
    Uncertified,
    /// Certification does not apply on this row.
    NotApplicable,
}

impl LearnabilityCertificationGrade {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCertified => "release_certified",
            Self::Certified => "certified",
            Self::ProvisionallyCertified => "provisionally_certified",
            Self::Uncertified => "uncertified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this grade carries a public certification claim.
    pub const fn is_certified(self) -> bool {
        matches!(
            self,
            Self::ReleaseCertified | Self::Certified | Self::ProvisionallyCertified
        )
    }

    /// Ordinal rank; higher is a stronger claim, so a narrow must move strictly
    /// lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NotApplicable => 0,
            Self::Uncertified => 1,
            Self::ProvisionallyCertified => 2,
            Self::Certified => 3,
            Self::ReleaseCertified => 4,
        }
    }
}

/// Reason a claimed row auto-narrowed below its claim. The chrome quotes the
/// trigger verbatim instead of a generic error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LearnabilityCertificationNarrowTrigger {
    /// A required-core dimension carries no certification at all.
    MissingDimensionProof,
    /// A certified dimension's proof aged outside its freshness window.
    StaleDimensionProof,
    /// A live row leaned on mirror / offline-pack proof to back a live claim.
    MirrorProofOnLiveRow,
    /// An educational-AI verdict still requires review and fails closed.
    VerdictRequiresReview,
    /// A guided-tour step could not be proven command-backed.
    TourStepsNotCommandBacked,
    /// Onboarding progress could not be certified user-owned and local-first.
    ProgressNotUserOwned,
    /// Educational-AI repository-truth citations could not be certified current.
    EducationalAiCitationsMissing,
    /// Offline / mirror docs-pack continuity could not be certified current.
    OfflineMirrorContinuityLost,
    /// Explain and do separation was lost (a teaching path mutated live state
    /// outside the standard preview/approval model).
    ExplainDoSeparationLost,
    /// An upstream learnability dependency narrowed and dragged this row down.
    UpstreamDependencyNarrowed,
}

impl LearnabilityCertificationNarrowTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingDimensionProof => "missing_dimension_proof",
            Self::StaleDimensionProof => "stale_dimension_proof",
            Self::MirrorProofOnLiveRow => "mirror_proof_on_live_row",
            Self::VerdictRequiresReview => "verdict_requires_review",
            Self::TourStepsNotCommandBacked => "tour_steps_not_command_backed",
            Self::ProgressNotUserOwned => "progress_not_user_owned",
            Self::EducationalAiCitationsMissing => "educational_ai_citations_missing",
            Self::OfflineMirrorContinuityLost => "offline_mirror_continuity_lost",
            Self::ExplainDoSeparationLost => "explain_do_separation_lost",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Durable subject of a certified learnability row, keyed by an M5 feature family,
/// a mirror-served continuity flag, and a non-display fingerprint distinct from
/// its id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedLearnabilitySubject {
    /// Durable onboarding-row / feature-family lane id of the certified subject.
    pub subject_id: String,
    /// Feature family, reusing the canonical learning-surface vocabulary so unlike
    /// families are never flattened into one synthetic onboarding claim.
    pub family: M5LearningSurfaceFamily,
    /// Whether this onboarding row's learnability continuity is currently served
    /// from a disclosed offline / mirrored pack rather than a live authoritative
    /// one. A mirror-served row never reads as a live local result.
    pub mirror_served: bool,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](CertifiedLearnabilitySubject::subject_id).
    pub subject_fingerprint_token: String,
}

impl CertifiedLearnabilitySubject {
    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty() && self.fingerprint_independent_of_id()
    }
}

/// One dimension's certification: the proof currency plus a reopenable evidence
/// object, so a grade is backed by an object a reviewer can reopen rather than an
/// asserted claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityDimensionCertification {
    /// Dimension being certified.
    pub dimension: LearnabilityEvidenceDimension,
    /// Currency of the proof backing this dimension.
    pub proof_currency: LearnabilityProofCurrency,
    /// Reopenable ref of the proof object. Present unless the proof is missing.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_ref: Option<String>,
    /// Non-display fingerprint token of the proof object. Present iff `proof_ref`
    /// is present, and must differ from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_fingerprint_token: Option<String>,
    /// Export-safe reviewable summary of the proof.
    pub summary: String,
}

impl LearnabilityDimensionCertification {
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

    /// Whether this certification is well-formed: a missing proof carries no ref,
    /// any other currency carries a reopenable proof, and the summary is present.
    pub fn is_well_formed(&self) -> bool {
        if self.summary.trim().is_empty() {
            return false;
        }
        if self.proof_currency.is_absent() {
            self.proof_ref.is_none() && self.proof_fingerprint_token.is_none()
        } else {
            self.proof_reopenable()
        }
    }

    /// Whether this certification backs a current claim for the given row's
    /// mirror posture. A live row needs locally verified or cached proof; a
    /// mirror-served row needs current mirror proof. Either way the proof must be
    /// reopenable.
    pub fn backs_claim(&self, mirror_row: bool) -> bool {
        if !self.proof_reopenable() {
            return false;
        }
        if mirror_row {
            self.proof_currency.is_mirror_current()
        } else {
            self.proof_currency.is_current_local()
        }
    }
}

/// One claimed M5 feature-family onboarding row certified against its learnability
/// evidence dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertifiedLearnabilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Durable subject the row certifies.
    pub subject: CertifiedLearnabilitySubject,
    /// Human-readable row label.
    pub label_summary: String,
    /// True when the row's learnability continuity is served from a disclosed
    /// offline / mirrored pack and must never read as a live local result.
    pub mirror_served: bool,
    /// Per-dimension certifications.
    pub certifications: Vec<LearnabilityDimensionCertification>,
    /// Whether every guided-tour and hint step is command-backed rather than a
    /// hidden mutating shortcut.
    pub tour_steps_command_backed: bool,
    /// Whether onboarding progress is user-owned and local-first rather than
    /// repo/collaborator-visible telemetry.
    pub progress_user_owned_local_first: bool,
    /// Whether educational-AI claims cite repository truth (files, symbols, docs,
    /// examples, commands) rather than answering omnisciently.
    pub educational_ai_cites_repository_truth: bool,
    /// Whether offline / mirror continuity is disclosed with explicit freshness
    /// rather than reading as live authoritative content.
    pub offline_mirror_continuity_disclosed: bool,
    /// Whether explain stays separate from do: every teaching mutation routes
    /// through the same preview/approval model as ordinary work.
    pub explain_separate_from_do: bool,
    /// Whether experts can dismiss, skip, and exit without being trapped in a
    /// tutorial.
    pub experts_not_trapped_in_tutorials: bool,
    /// Whether onboarding progress stays private to the user (never shared
    /// implicitly with repos or collaborators).
    pub progress_private_to_user: bool,
    /// Headline certification grade publicly claimed for this row.
    pub claimed_grade: LearnabilityCertificationGrade,
    /// Effective grade after auto-narrowing; equals the claim when every dimension
    /// is current, and ranks strictly below it otherwise.
    pub effective_grade: LearnabilityCertificationGrade,
    /// Trigger that fired the narrow, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrow_trigger: Option<LearnabilityCertificationNarrowTrigger>,
    /// Precise narrowed label, required when the row is narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CertifiedLearnabilityRow {
    /// Dimensions certified by this row.
    pub fn certified_dimensions(&self) -> BTreeSet<LearnabilityEvidenceDimension> {
        self.certifications.iter().map(|c| c.dimension).collect()
    }

    /// Resolves a certification by dimension.
    pub fn certification(
        &self,
        dimension: LearnabilityEvidenceDimension,
    ) -> Option<&LearnabilityDimensionCertification> {
        self.certifications
            .iter()
            .find(|c| c.dimension == dimension)
    }

    /// Whether every required-core dimension is certified.
    pub fn has_all_required_core(&self) -> bool {
        let certified = self.certified_dimensions();
        LearnabilityEvidenceDimension::REQUIRED_CORE
            .iter()
            .all(|dimension| certified.contains(dimension))
    }

    /// Whether the row carries a public certification claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_grade.is_certified()
    }

    /// Whether every certified dimension backs a current claim for this row's
    /// mirror posture.
    pub fn all_dimensions_current(&self) -> bool {
        self.certifications
            .iter()
            .all(|c| c.backs_claim(self.mirror_served))
    }

    /// Whether the row must narrow below its claim because a required-core
    /// dimension is uncertified or any certified dimension lacks current proof.
    pub fn needs_narrow(&self) -> bool {
        !self.has_all_required_core() || !self.all_dimensions_current()
    }

    /// Whether the effective grade and narrow evidence are consistent.
    ///
    /// When every dimension is current the effective grade equals the claim;
    /// otherwise it must rank strictly below the claim and carry both a recorded
    /// trigger and a precise narrowed label.
    pub fn narrow_consistent(&self) -> bool {
        if self.needs_narrow() {
            self.effective_grade.rank() < self.claimed_grade.rank()
                && self.narrow_trigger.is_some()
                && self
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_grade == self.claimed_grade
        }
    }

    /// Whether the mirror posture is consistent: the row flag and its subject
    /// agree, so a mirror-served row never reads as a live local result.
    pub fn mirror_posture_consistent(&self) -> bool {
        self.mirror_served == self.subject.mirror_served
    }

    /// Whether every dimension required to record this row is present and its
    /// invariants hold.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.subject.is_valid()
            && !self.certifications.is_empty()
            && self
                .certifications
                .iter()
                .all(LearnabilityDimensionCertification::is_well_formed)
            && self.narrow_consistent()
            && self.mirror_posture_consistent()
            && self.tour_steps_command_backed
            && self.progress_user_owned_local_first
            && self.educational_ai_cites_repository_truth
            && self.offline_mirror_continuity_disclosed
            && self.explain_separate_from_do
            && self.experts_not_trapped_in_tutorials
            && self.progress_private_to_user
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityCertificationGuardrails {
    /// Every tutorial, hint, and exercise path stays command-backed and opt-in.
    pub tutorials_stay_command_backed_and_opt_in: bool,
    /// Explain and do stay separate; every teaching mutation uses the standard
    /// preview/approval model.
    pub explain_and_do_stay_separate: bool,
    /// Progress stays user-owned and local-first by default.
    pub progress_user_owned_and_local_first: bool,
    /// Cached and offline/mirror packs stay explicit with disclosed freshness.
    pub cached_offline_packs_stay_explicit: bool,
    /// Educational AI routes any action through the same preview/approval model as
    /// ordinary work.
    pub educational_ai_routes_through_preview_approval: bool,
    /// Experts are never trapped in tutorials.
    pub experts_never_trapped_in_tutorials: bool,
    /// Any claimed row lacking current proof auto-narrows below its claim.
    pub rows_auto_narrow_without_current_proof: bool,
}

impl LearnabilityCertificationGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.tutorials_stay_command_backed_and_opt_in
            && self.explain_and_do_stay_separate
            && self.progress_user_owned_and_local_first
            && self.cached_offline_packs_stay_explicit
            && self.educational_ai_routes_through_preview_approval
            && self.experts_never_trapped_in_tutorials
            && self.rows_auto_narrow_without_current_proof
    }
}

/// Consumer projection block: the surfaces that read this certification without
/// re-deriving learnability maturity by hand.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityCertificationConsumerProjection {
    /// Help/About ingests this certification.
    pub help_about_ingests_certification: bool,
    /// Docs / migration packets ingest the same certification.
    pub docs_migration_ingests_certification: bool,
    /// Support / export ingests the same certification.
    pub support_export_ingests_certification: bool,
    /// The release center ingests the same certification.
    pub release_center_ingests_certification: bool,
    /// The Start Center / onboarding surface ingests the same certification.
    pub start_center_ingests_certification: bool,
    /// AI evidence ingests the same certification.
    pub ai_evidence_ingests_certification: bool,
    /// Narrowed rows are visibly labeled below their claim in every surface.
    pub narrowed_rows_labeled_below_claim: bool,
}

impl LearnabilityCertificationConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.help_about_ingests_certification
            && self.docs_migration_ingests_certification
            && self.support_export_ingests_certification
            && self.release_center_ingests_certification
            && self.start_center_ingests_certification
            && self.ai_evidence_ingests_certification
            && self.narrowed_rows_labeled_below_claim
    }
}

/// Evidence freshness block for the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityCertificationFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically narrows claimed rows.
    pub auto_narrow_on_stale: bool,
}

impl LearnabilityCertificationFreshness {
    /// Whether the freshness block is well-formed.
    pub fn is_valid(&self) -> bool {
        self.evidence_freshness_slo_hours > 0 && !self.last_evidence_refresh.trim().is_empty()
    }
}

/// Constructor input for [`LearnabilityCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearnabilityCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedLearnabilityRow>,
    /// Guardrail invariants block.
    pub guardrails: LearnabilityCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: LearnabilityCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: LearnabilityCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe learnability certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LearnabilityCertificationPacket {
    /// Record kind; must equal [`LEARNABILITY_CERT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LEARNABILITY_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub label: String,
    /// Per-row certifications.
    pub rows: Vec<CertifiedLearnabilityRow>,
    /// Guardrail invariants block.
    pub guardrails: LearnabilityCertificationGuardrails,
    /// Consumer projection block.
    pub consumer_projection: LearnabilityCertificationConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: LearnabilityCertificationFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl LearnabilityCertificationPacket {
    /// Builds a learnability certification packet.
    pub fn new(input: LearnabilityCertificationPacketInput) -> Self {
        Self {
            record_kind: LEARNABILITY_CERT_RECORD_KIND.to_owned(),
            schema_version: LEARNABILITY_CERT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Feature families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5LearningSurfaceFamily> {
        self.rows.iter().map(|row| row.subject.family).collect()
    }

    /// Evidence dimensions certified by some row in this packet.
    pub fn represented_dimensions(&self) -> BTreeSet<LearnabilityEvidenceDimension> {
        self.rows
            .iter()
            .flat_map(|row| row.certified_dimensions())
            .collect()
    }

    /// Proof currencies represented across certifications.
    pub fn represented_currencies(&self) -> BTreeSet<LearnabilityProofCurrency> {
        self.rows
            .iter()
            .flat_map(|row| row.certifications.iter().map(|c| c.proof_currency))
            .collect()
    }

    /// Count of rows that auto-narrowed below their claim.
    pub fn narrowed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_narrow()).count()
    }

    /// Count of rows holding a public certification claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Count of mirror-served rows.
    pub fn mirror_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.mirror_served).count()
    }

    /// Rows that auto-narrowed below their claim, in packet order.
    pub fn narrowed_rows(&self) -> Vec<&CertifiedLearnabilityRow> {
        self.rows.iter().filter(|row| row.needs_narrow()).collect()
    }

    /// Resolves a row by its id.
    pub fn row(&self, row_id: &str) -> Option<&CertifiedLearnabilityRow> {
        self.rows.iter().find(|row| row.row_id == row_id)
    }

    /// Validates the learnability certification invariants.
    pub fn validate(&self) -> Vec<LearnabilityCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != LEARNABILITY_CERT_RECORD_KIND {
            violations.push(LearnabilityCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != LEARNABILITY_CERT_SCHEMA_VERSION {
            violations.push(LearnabilityCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(LearnabilityCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(LearnabilityCertificationViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(LearnabilityCertificationViolation::ConsumerProjectionIncomplete);
        }
        if !self.evidence_freshness.is_valid() {
            violations.push(LearnabilityCertificationViolation::EvidenceFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("learnability certification packet serializes"),
        ) {
            violations.push(LearnabilityCertificationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("learnability certification packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Learnability Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} mirror-served, {} narrowed)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.mirror_row_count(),
            self.narrowed_row_count()
        ));
        out.push_str(&format!(
            "- Families: {} / {}\n",
            self.represented_families().len(),
            M5LearningSurfaceFamily::ALL.len()
        ));
        out.push_str(&format!(
            "- Dimensions certified: {} / {}\n",
            self.represented_dimensions().len(),
            LearnabilityEvidenceDimension::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.subject.family.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - subject `{}` (family `{}`), mirror_served={}\n",
                row.subject.subject_id,
                row.subject.family.as_str(),
                row.mirror_served
            ));
            for cert in &row.certifications {
                out.push_str(&format!(
                    "  - {} = `{}`\n",
                    cert.dimension.as_str(),
                    cert.proof_currency.as_str()
                ));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - Narrowed: {label}\n"));
            }
        }
        out
    }

    /// Deterministic Markdown waiver-and-downgrade log: the release-visible record
    /// of every claimed row currently held below its claim, with the trigger and
    /// label that narrowed it. There are no manual waivers — auto-narrowing is the
    /// only mechanism by which a row sits below its claim.
    pub fn render_waiver_and_downgrade_log(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Learnability Waiver and Downgrade Log\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Generated from: `{}`\n",
            LEARNABILITY_CERT_ARTIFACT_REF
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str(
            "\nNo manual waivers are granted: a learnability row sits below its claim only by \
             automatic narrowing when current, reopenable proof cannot back it.\n",
        );
        let narrowed = self.narrowed_rows();
        out.push_str(&format!(
            "\n## Auto-downgraded rows ({})\n\n",
            narrowed.len()
        ));
        if narrowed.is_empty() {
            out.push_str("None — every claimed row holds current proof for its claim.\n");
            return out;
        }
        for row in narrowed {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.subject.family.as_str(),
                row.claimed_grade.as_str(),
                row.effective_grade.as_str()
            ));
            if let Some(trigger) = row.narrow_trigger {
                out.push_str(&format!("  - Trigger: `{}`\n", trigger.as_str()));
            }
            if let Some(label) = &row.narrowed_label {
                out.push_str(&format!("  - {label}\n"));
            }
            let uncurrent: Vec<&str> = row
                .certifications
                .iter()
                .filter(|c| !c.backs_claim(row.mirror_served))
                .map(|c| c.dimension.as_str())
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
pub enum LearnabilityCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<LearnabilityCertificationViolation>),
}

impl fmt::Display for LearnabilityCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "learnability certification export parse failed: {error}"
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
                    "learnability certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for LearnabilityCertificationArtifactError {}

/// Validation failures emitted by [`LearnabilityCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LearnabilityCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed feature family is represented by no row.
    RequiredFamilyMissing,
    /// Some evidence dimension is certified by no row.
    DimensionCoverageMissing,
    /// No row demonstrates auto-narrowing on uncurrent proof.
    NarrowedRowCaseMissing,
    /// No row certifies current proof.
    CurrentProofCaseMissing,
    /// No mirror-served row is present.
    MirrorRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not narrowed below its claim despite uncurrent proof.
    RowNotNarrowedOnUncurrentProof,
    /// A narrowed row lacks a precise narrowed label or trigger.
    NarrowedRowMissingLabelOrTrigger,
    /// A row's subject fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// A guided-tour or hint step bypassed the command graph.
    TourStepNotCommandBacked,
    /// Onboarding progress widened into repo/collaborator-visible telemetry.
    ProgressNotUserOwned,
    /// Educational AI answered omnisciently without citing repository truth.
    EducationalAiUncited,
    /// A mirrored/offline pack read as live authoritative content.
    OfflineMirrorReadsAsLive,
    /// A mirror-served row read as a live local result.
    MirrorReadsAsLive,
    /// A teaching mutation bypassed the standard preview/approval model.
    ExplainDoConflated,
    /// An expert was trapped in a tutorial with no escape.
    ExpertTrappedInTutorial,
    /// A dimension proof is not reopenable (missing ref or fingerprint substitutes).
    DimensionProofNotReopenable,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl LearnabilityCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::DimensionCoverageMissing => "dimension_coverage_missing",
            Self::NarrowedRowCaseMissing => "narrowed_row_case_missing",
            Self::CurrentProofCaseMissing => "current_proof_case_missing",
            Self::MirrorRowCaseMissing => "mirror_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotNarrowedOnUncurrentProof => "row_not_narrowed_on_uncurrent_proof",
            Self::NarrowedRowMissingLabelOrTrigger => "narrowed_row_missing_label_or_trigger",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::TourStepNotCommandBacked => "tour_step_not_command_backed",
            Self::ProgressNotUserOwned => "progress_not_user_owned",
            Self::EducationalAiUncited => "educational_ai_uncited",
            Self::OfflineMirrorReadsAsLive => "offline_mirror_reads_as_live",
            Self::MirrorReadsAsLive => "mirror_reads_as_live",
            Self::ExplainDoConflated => "explain_do_conflated",
            Self::ExpertTrappedInTutorial => "expert_trapped_in_tutorial",
            Self::DimensionProofNotReopenable => "dimension_proof_not_reopenable",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
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
pub fn current_m5_learnability_certification_export(
) -> Result<LearnabilityCertificationPacket, LearnabilityCertificationArtifactError> {
    let packet: LearnabilityCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/learnability/certification-report/support_export.json"
    )))
    .map_err(LearnabilityCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(LearnabilityCertificationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &LearnabilityCertificationPacket,
    violations: &mut Vec<LearnabilityCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        LEARNABILITY_CERT_SCHEMA_REF,
        LEARNABILITY_CERT_DOC_REF,
        LEARNABILITY_CERT_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(LearnabilityCertificationViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &LearnabilityCertificationPacket,
    violations: &mut Vec<LearnabilityCertificationViolation>,
) {
    let families = packet.represented_families();
    for required in M5LearningSurfaceFamily::ALL {
        if !families.contains(&required) {
            violations.push(LearnabilityCertificationViolation::RequiredFamilyMissing);
            break;
        }
    }

    let dimensions = packet.represented_dimensions();
    for required in LearnabilityEvidenceDimension::ALL {
        if !dimensions.contains(&required) {
            violations.push(LearnabilityCertificationViolation::DimensionCoverageMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_narrow() && row.narrow_consistent())
    {
        violations.push(LearnabilityCertificationViolation::NarrowedRowCaseMissing);
    }

    let currencies = packet.represented_currencies();
    if !currencies
        .iter()
        .any(|currency| currency.is_current_local() || currency.is_mirror_current())
    {
        violations.push(LearnabilityCertificationViolation::CurrentProofCaseMissing);
    }

    if packet.mirror_row_count() == 0 {
        violations.push(LearnabilityCertificationViolation::MirrorRowCaseMissing);
    }
}

fn validate_rows(
    packet: &LearnabilityCertificationPacket,
    violations: &mut Vec<LearnabilityCertificationViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(LearnabilityCertificationViolation::RowIncomplete);
        }
        if row.needs_narrow() && row.effective_grade.rank() >= row.claimed_grade.rank() {
            violations.push(LearnabilityCertificationViolation::RowNotNarrowedOnUncurrentProof);
        }
        if row.needs_narrow()
            && (row.narrow_trigger.is_none()
                || !row
                    .narrowed_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations.push(LearnabilityCertificationViolation::NarrowedRowMissingLabelOrTrigger);
        }
        if !row.subject.fingerprint_independent_of_id() {
            violations.push(LearnabilityCertificationViolation::FingerprintSubstitutesIdentity);
        }
        if !row.tour_steps_command_backed {
            violations.push(LearnabilityCertificationViolation::TourStepNotCommandBacked);
        }
        if !row.progress_user_owned_local_first || !row.progress_private_to_user {
            violations.push(LearnabilityCertificationViolation::ProgressNotUserOwned);
        }
        if !row.educational_ai_cites_repository_truth {
            violations.push(LearnabilityCertificationViolation::EducationalAiUncited);
        }
        if !row.offline_mirror_continuity_disclosed {
            violations.push(LearnabilityCertificationViolation::OfflineMirrorReadsAsLive);
        }
        if !row.mirror_posture_consistent() {
            violations.push(LearnabilityCertificationViolation::MirrorReadsAsLive);
        }
        if !row.explain_separate_from_do {
            violations.push(LearnabilityCertificationViolation::ExplainDoConflated);
        }
        if !row.experts_not_trapped_in_tutorials {
            violations.push(LearnabilityCertificationViolation::ExpertTrappedInTutorial);
        }
        if row.certifications.iter().any(|cert| !cert.is_well_formed()) {
            violations.push(LearnabilityCertificationViolation::DimensionProofNotReopenable);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(LearnabilityCertificationViolation::RowEvidenceMissing);
        }
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
            | "uncertified"
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

/// Builds the canonical, seeded M5 learnability certification packet.
///
/// The corpus certifies every claimed M5 feature-family onboarding row — notebook,
/// request/API, database, profiler, docs/browser, preview, framework/template,
/// companion, and sync/offboarding — against the command-backed tour,
/// guided-exercise, user-owned progress, cited educational-AI, and offline/mirror
/// continuity model, plus the learning-mode-profile dimension on the families that
/// ship one.
///
/// The companion row is the mirror-served drill: it is held read-only on a
/// disclosed offline/mirror pack (`mirror_current` proof) and never reads as a live
/// local result. The extra profiler row is the auto-downgrade drill: its
/// offline-mirror proof aged outside its freshness window, so it auto-narrows from
/// `certified` to `uncertified` with an `offline_mirror_continuity_lost` trigger and
/// a precise narrowed label, while every other row's effective grade equals its
/// claim.
pub fn seeded_m5_learnability_certification() -> LearnabilityCertificationPacket {
    LearnabilityCertificationPacket::new(seeded_packet_input())
}

const PACKET_ID: &str = "m5-learnability-certification:stable:0001";
const MINTED_AT: &str = "2026-06-19T00:00:00Z";

fn seeded_refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn seeded_cert(
    dimension: LearnabilityEvidenceDimension,
    proof_currency: LearnabilityProofCurrency,
) -> LearnabilityDimensionCertification {
    let (proof_ref, proof_fingerprint_token) = if proof_currency.is_absent() {
        (None, None)
    } else {
        (
            Some(format!("evidence:{}", dimension.as_str())),
            Some(format!("fp:{}", dimension.as_str())),
        )
    };
    LearnabilityDimensionCertification {
        dimension,
        proof_currency,
        proof_ref,
        proof_fingerprint_token,
        summary: format!(
            "{} certified with {} proof",
            dimension.as_str(),
            proof_currency.as_str()
        ),
    }
}

fn seeded_core(
    proof_currency: LearnabilityProofCurrency,
) -> Vec<LearnabilityDimensionCertification> {
    LearnabilityEvidenceDimension::REQUIRED_CORE
        .iter()
        .map(|dimension| seeded_cert(*dimension, proof_currency))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seeded_row(
    row_id: &str,
    family: M5LearningSurfaceFamily,
    label: &str,
    mirror_served: bool,
    certifications: Vec<LearnabilityDimensionCertification>,
    claimed: LearnabilityCertificationGrade,
) -> CertifiedLearnabilityRow {
    CertifiedLearnabilityRow {
        row_id: row_id.to_owned(),
        subject: CertifiedLearnabilitySubject {
            subject_id: format!("subject:{row_id}"),
            family,
            mirror_served,
            subject_fingerprint_token: format!("fp:{row_id}"),
        },
        label_summary: label.to_owned(),
        mirror_served,
        certifications,
        tour_steps_command_backed: true,
        progress_user_owned_local_first: true,
        educational_ai_cites_repository_truth: true,
        offline_mirror_continuity_disclosed: true,
        explain_separate_from_do: true,
        experts_not_trapped_in_tutorials: true,
        progress_private_to_user: true,
        claimed_grade: claimed,
        effective_grade: claimed,
        narrow_trigger: None,
        narrowed_label: None,
        evidence_refs: seeded_refs(&[&format!("evidence:row:{row_id}")]),
        source_contract_refs: seeded_refs(&[LEARNABILITY_CERT_DOC_REF]),
    }
}

fn seeded_core_with_profile(
    proof_currency: LearnabilityProofCurrency,
) -> Vec<LearnabilityDimensionCertification> {
    let mut certifications = seeded_core(proof_currency);
    certifications.push(seeded_cert(
        LearnabilityEvidenceDimension::LearningModeProfile,
        proof_currency,
    ));
    certifications
}

fn seeded_stale_offline_mirror_row() -> CertifiedLearnabilityRow {
    let mut certifications = seeded_core(LearnabilityProofCurrency::VerifiedCurrent);
    for certification in &mut certifications {
        if certification.dimension == LearnabilityEvidenceDimension::OfflineMirror {
            certification.proof_currency = LearnabilityProofCurrency::StaleExpired;
        }
    }
    let mut narrowed = seeded_row(
        "learn-cert:profiler_trace:stale-offline-mirror:0001",
        M5LearningSurfaceFamily::ProfilerTrace,
        "Profiler onboarding row whose offline/mirror docs-pack aged outside its freshness window",
        false,
        certifications,
        LearnabilityCertificationGrade::Certified,
    );
    narrowed.effective_grade = LearnabilityCertificationGrade::Uncertified;
    narrowed.narrow_trigger =
        Some(LearnabilityCertificationNarrowTrigger::OfflineMirrorContinuityLost);
    narrowed.narrowed_label = Some(
        "Offline/mirror docs-pack aged outside its freshness window; held uncertified until a fresh mirror re-backs continuity"
            .to_owned(),
    );
    narrowed
}

fn seeded_rows() -> Vec<CertifiedLearnabilityRow> {
    vec![
        seeded_row(
            "learn-cert:notebook:0001",
            M5LearningSurfaceFamily::Notebook,
            "Notebook onboarding row with current command-backed tour, exercise, progress, cited educational-AI, offline-mirror, and learning-mode-profile proof",
            false,
            seeded_core_with_profile(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:request_workspace:0001",
            M5LearningSurfaceFamily::RequestWorkspace,
            "HTTP/API request workspace onboarding row with current command-backed core learnability proof",
            false,
            seeded_core(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:database_workspace:0001",
            M5LearningSurfaceFamily::DatabaseWorkspace,
            "Database/SQL workspace onboarding row with current command-backed core learnability proof",
            false,
            seeded_core(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:profiler_trace:0001",
            M5LearningSurfaceFamily::ProfilerTrace,
            "Profiler/trace onboarding row with current command-backed core learnability proof",
            false,
            seeded_core(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:docs_browser:0001",
            M5LearningSurfaceFamily::DocsBrowser,
            "Docs/knowledge and embedded-browser onboarding row with current core proof plus a learning-mode profile, held release-certified",
            false,
            seeded_core_with_profile(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::ReleaseCertified,
        ),
        seeded_row(
            "learn-cert:preview:0001",
            M5LearningSurfaceFamily::Preview,
            "Preview/runtime onboarding row with current command-backed core learnability proof",
            false,
            seeded_core(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:template_scaffold:0001",
            M5LearningSurfaceFamily::TemplateScaffold,
            "Template/scaffold (framework-pack) onboarding row with current core proof plus a learning-mode profile",
            false,
            seeded_core_with_profile(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::Certified,
        ),
        seeded_row(
            "learn-cert:companion:0001",
            M5LearningSurfaceFamily::Companion,
            "Companion onboarding row held read-only on a disclosed offline/mirror pack with current mirror proof that never reads as a live local result",
            true,
            seeded_core(LearnabilityProofCurrency::MirrorCurrent),
            LearnabilityCertificationGrade::ProvisionallyCertified,
        ),
        seeded_row(
            "learn-cert:sync_offboarding:0001",
            M5LearningSurfaceFamily::SyncOffboarding,
            "Sync/retention/offboarding onboarding row with current command-backed core learnability proof, held release-certified",
            false,
            seeded_core(LearnabilityProofCurrency::VerifiedCurrent),
            LearnabilityCertificationGrade::ReleaseCertified,
        ),
        seeded_stale_offline_mirror_row(),
    ]
}

fn seeded_guardrails() -> LearnabilityCertificationGuardrails {
    LearnabilityCertificationGuardrails {
        tutorials_stay_command_backed_and_opt_in: true,
        explain_and_do_stay_separate: true,
        progress_user_owned_and_local_first: true,
        cached_offline_packs_stay_explicit: true,
        educational_ai_routes_through_preview_approval: true,
        experts_never_trapped_in_tutorials: true,
        rows_auto_narrow_without_current_proof: true,
    }
}

fn seeded_consumer_projection() -> LearnabilityCertificationConsumerProjection {
    LearnabilityCertificationConsumerProjection {
        help_about_ingests_certification: true,
        docs_migration_ingests_certification: true,
        support_export_ingests_certification: true,
        release_center_ingests_certification: true,
        start_center_ingests_certification: true,
        ai_evidence_ingests_certification: true,
        narrowed_rows_labeled_below_claim: true,
    }
}

fn seeded_evidence_freshness() -> LearnabilityCertificationFreshness {
    LearnabilityCertificationFreshness {
        evidence_freshness_slo_hours: 168,
        last_evidence_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    seeded_refs(&[
        LEARNABILITY_CERT_SCHEMA_REF,
        LEARNABILITY_CERT_DOC_REF,
        LEARNABILITY_CERT_ARTIFACT_REF,
        "schemas/help/m5-tour-and-glossary-packages.schema.json",
        "schemas/help/m5-guided-exercise-rails.schema.json",
        "schemas/help/m5-learning-progress-snapshots.schema.json",
        "schemas/help/m5-educational-ai-and-practice.schema.json",
        "schemas/help/m5-learning-mode-profiles.schema.json",
        "schemas/help/learning-session-export.schema.json",
    ])
}

fn seeded_packet_input() -> LearnabilityCertificationPacketInput {
    LearnabilityCertificationPacketInput {
        packet_id: PACKET_ID.to_owned(),
        label: "M5 Learnability Certification".to_owned(),
        rows: seeded_rows(),
        guardrails: seeded_guardrails(),
        consumer_projection: seeded_consumer_projection(),
        evidence_freshness: seeded_evidence_freshness(),
        source_contract_refs: seeded_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    }
}
