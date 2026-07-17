//! M05-1283 closing B152 surface certification over the frozen M5 review-pack evaluator matrix — the
//! reusable review-pack record, ownership signal, required-evidence / required-check row, local-CI parity
//! strip, AI review policy hook, and review-template packet that a review, merge-readiness, AI review,
//! provider-handoff, or support / export consumer must treat as first-class, durable, publish-safe review
//! objects rather than ad hoc review chrome.
//!
//! Where the freeze matrix ([`crate::m5_review_pack_evaluator_matrix`]) defines the six governed review-pack
//! object classes, the M05-1275..1280 implement lanes resolve each review-pack record / result, ownership
//! signal / owner-conflict, required-evidence-check / local-CI-parity, AI-policy-hook / AI-policy-result,
//! review-template-packet / publish-attribution, and invalidation / rerun-compare registry; this closing
//! capstone *certifies* that the shared review-pack truth holds on every claimed M5 review, AI, provider,
//! browser-handoff, and support / export surface — pack labels, pack version / digest, owner provenance,
//! evaluator result class, local-versus-provider parity, pack freshness, and template attribution — and
//! auto-narrows any profile that cannot sustain it.
//!
//! It is keyed on the claimed **profile** a review / diff owner, an AI / automation flow, a provider handoff
//! consumer, or a support / export consumer reads a review pack through (a fully-certified review-pack lane; a
//! reviewable review-pack record structure; a stale-pack-version-digest profile; an unverified-owner-provenance
//! profile; an unevaluated-required-check profile; a local-only-parity profile; an undisclosed-AI-pack-binding
//! profile; and a stale-template-attribution profile), not on the underlying object class or implement lane.
//! Each [`ReviewPackProfileCertificationRow`] certifies one profile across nine truth axes — visual, keyboard,
//! screen-reader, high-zoom-reflow, high-contrast, localization, CLI/export, degraded-state, and
//! review-pack-truth behavior — and either passes (green), auto-narrows its review-pack claim to the weakest
//! supported ceiling (yellow), or is blocked (red) when a degraded axis is hidden behind a fresh certified
//! claim inherited from a healthier profile.
//!
//! The invariant is: **a degraded axis must produce a visible claim narrowing**. A profile that keeps a
//! `CertifiedReviewPackTruth` / `ReviewableReviewPackRecord` claim while one of its truth axes is not current is
//! over-claiming and blocks; a profile that discloses the reduction by narrowing its claim (with a bound reason
//! and a frozen downgrade trigger) is honestly yellow. Only a fully-certified review-pack lane — one whose pack
//! version / digest, owner provenance, evaluator result class, local-versus-provider parity, pack freshness,
//! and template attribution all converge on one export-safe, provider-current, internally consistent review-pack
//! record — may certify a `CertifiedReviewPackTruth` claim; a reviewable, stale-pack, unverified-owner,
//! unevaluated-check, local-only-parity, undisclosed-AI-binding, or stale-template profile that keeps a
//! certified claim is over-reaching and blocks. The always-on CLI/export axis must always stay certified so
//! support and automation can reconstruct the pack label, pack version / digest, owner provenance, evaluator
//! result class, local-versus-provider parity, pack freshness, and template attribution from the same
//! review-pack proof the operator saw.
//!
//! The B152 hard invariants are enforced per row: no profile may let a local parity estimate masquerade as
//! provider-authoritative mergeability or approval truth; hide a ci-only, not-evaluated-here, or
//! provider-unavailable state behind a green summary; flatten advisory-owner and enforced-owner into one owner
//! pill; let AI review run under a different pack version without disclosure; or lose the review-pack version /
//! digest or template attribution when exporting, publishing, or reopening review evidence. A profile that
//! breaches any invariant blocks (red).
//!
//! Every row cites exactly one canonical review-pack evaluator matrix proof bundle
//! ([`REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF`]) — the frozen review-pack evaluator matrix proof — rather than
//! cloning per-profile evidence. The packet is metadata-only: raw credentials, plaintext secrets, bearer
//! tokens, endpoint URLs, and private-key material never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/review/m5-review-pack-surface-certification.schema.json`](../../../../schemas/review/m5-review-pack-surface-certification.schema.json).
//! The contract doc is
//! [`docs/review/m5-review-pack-surface-certification.md`](../../../../docs/review/m5-review-pack-surface-certification.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_review_pack_evaluator_matrix as matrix;
use matrix::{M5ReviewPackDowngradeTrigger, M5ReviewPackObject};

/// Schema version stamped on the M05-1283 certification packet.
pub const REVIEW_PACK_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ReviewPackProfileCertificationPacket`].
pub const REVIEW_PACK_CERT_RECORD_KIND: &str = "m5_review_pack_surface_certification_packet";

/// Stable record-kind tag carried by each [`ReviewPackProfileCertificationRow`].
pub const REVIEW_PACK_CERT_ROW_RECORD_KIND: &str = "m5_review_pack_surface_certification_row";

/// Repo-relative path of the boundary schema.
pub const REVIEW_PACK_CERT_SCHEMA_REF: &str =
    "schemas/review/m5-review-pack-surface-certification.schema.json";

/// Repo-relative path of the contract doc.
pub const REVIEW_PACK_CERT_DOC_REF: &str = "docs/review/m5-review-pack-surface-certification.md";

/// Repo-relative path of the frozen review-pack evaluator matrix schema the certified profiles render.
pub const REVIEW_PACK_CERT_MATRIX_REF: &str = matrix::M5_REVIEW_PACK_MATRIX_SCHEMA_REF;

/// The one canonical review-pack evaluator matrix proof bundle every certified profile cites as its
/// first-resolved review-pack truth. All eight profiles point back to it rather than cloning per-profile
/// evidence.
pub const REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF: &str = matrix::M5_REVIEW_PACK_ARTIFACT_REF;

/// The review-pack-health dashboard the release surfaces consume. Recorded as a supporting evidence ref on
/// every row so the certification's review-pack truth ties back to the same dashboard consumers read.
pub const REVIEW_PACK_CERT_CONSUMERS_BUNDLE_REF: &str = matrix::M5_REVIEW_PACK_DASHBOARD_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const REVIEW_PACK_CERT_ARTIFACT_REF: &str =
    "artifacts/review/m5-review-pack-surface-certification/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const REVIEW_PACK_CERT_CSV_REF: &str =
    "artifacts/review/m5-review-pack-surface-certification/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const REVIEW_PACK_CERT_REPORT_REF: &str =
    "artifacts/review/m5-review-pack-surface-certification.md";

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_PACK_CERT_FIXTURE_DIR: &str =
    "fixtures/review/m5-review-pack-surface-certification";

/// Stable packet id for the checked-in certification bundle.
pub const REVIEW_PACK_CERT_PACKET_ID: &str = "m5-review-pack-surface-certification:stable:0001";

/// The eight claimed M5 review-pack consumer profiles this capstone certifies. Keyed on the profile a review /
/// diff owner, an AI / automation flow, a provider handoff consumer, or a support / export consumer reads a
/// review pack through — a fully-certified review-pack lane, a reviewable review-pack record structure, a
/// stale-pack-version-digest profile, an unverified-owner-provenance profile, an unevaluated-required-check
/// profile, a local-only-parity profile, an undisclosed-AI-pack-binding profile, and a stale-template-attribution
/// profile — not on the reusable object class it renders. Only a fully-certified review-pack lane profile may
/// certify a certified review-pack claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackCertifiedProfile {
    /// A fully-certified review-pack lane — a review pack whose pack label, pack version / digest, owner
    /// provenance, evaluator result class, local-versus-provider parity, pack freshness, and template
    /// attribution all converge on one export-safe, provider-current, internally consistent review-pack record
    /// that stays identical across every review, AI, provider, browser-handoff, and support / export consumer,
    /// certifying the review-pack claim exactly right now.
    CertifiedReviewPackLane,
    /// A reviewable review-pack record structure: a self-sufficient, inspectable review-template / evidence
    /// record (a pack-bound record an operator can review), never itself a fully-certified review-pack lane.
    ReviewableReviewPackRecordStructure,
    /// A review-pack lane whose pack version / digest can no longer be confirmed fresh — the pack the record was
    /// evaluated against has moved on; the claim narrows to a pack-version-digest-unverified projection that
    /// discloses the last-known pack version / digest and marks the pack stale, never a stale pack silently
    /// looking current after a base/head or pack revision.
    StalePackVersionDigestProfile,
    /// An ownership lane whose advisory-versus-enforced owner provenance is missing or unresolved; the claim
    /// narrows to an owner-provenance-unverified projection that keeps the last-known owner source explicit and
    /// never flattens advisory-owner and enforced-owner into one owner pill.
    UnverifiedOwnerProvenanceProfile,
    /// A required-check lane whose evaluator result class is ci-only, not-evaluated-here, or provider-unavailable;
    /// the claim narrows to an evidence-check-unverified projection that keeps the unevaluated state explicit and
    /// never hides it behind a green summary.
    UnevaluatedRequiredCheckProfile,
    /// A local-CI parity lane whose result is a local parity estimate that diverges from provider-authoritative
    /// state; the claim narrows to a local-parity-unverified projection that keeps the estimate labelled and
    /// never lets it masquerade as provider-authoritative mergeability.
    LocalOnlyParityProfile,
    /// An AI-policy-hook lane whose AI review ran under a pack version / digest that is not disclosed against the
    /// active pack; the claim narrows to an AI-pack-binding-unverified projection that discloses the pack version
    /// the AI review ran under, never an AI review running under a different pack version without disclosure.
    UndisclosedAiPackBindingProfile,
    /// A review-template lane whose template attribution can no longer be verified against the pack it came from;
    /// the claim narrows to a template-attribution-unverified projection that keeps the last-known template
    /// version / digest and pack binding explicit, never a template that loses its attribution on export,
    /// publish, or reopen.
    StaleTemplateAttributionProfile,
}

impl M5ReviewPackCertifiedProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [M5ReviewPackCertifiedProfile; 8] = [
        M5ReviewPackCertifiedProfile::CertifiedReviewPackLane,
        M5ReviewPackCertifiedProfile::ReviewableReviewPackRecordStructure,
        M5ReviewPackCertifiedProfile::StalePackVersionDigestProfile,
        M5ReviewPackCertifiedProfile::UnverifiedOwnerProvenanceProfile,
        M5ReviewPackCertifiedProfile::UnevaluatedRequiredCheckProfile,
        M5ReviewPackCertifiedProfile::LocalOnlyParityProfile,
        M5ReviewPackCertifiedProfile::UndisclosedAiPackBindingProfile,
        M5ReviewPackCertifiedProfile::StaleTemplateAttributionProfile,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedReviewPackLane => "certified_review_pack_lane",
            Self::ReviewableReviewPackRecordStructure => "reviewable_review_pack_record_structure",
            Self::StalePackVersionDigestProfile => "stale_pack_version_digest_profile",
            Self::UnverifiedOwnerProvenanceProfile => "unverified_owner_provenance_profile",
            Self::UnevaluatedRequiredCheckProfile => "unevaluated_required_check_profile",
            Self::LocalOnlyParityProfile => "local_only_parity_profile",
            Self::UndisclosedAiPackBindingProfile => "undisclosed_ai_pack_binding_profile",
            Self::StaleTemplateAttributionProfile => "stale_template_attribution_profile",
        }
    }

    /// True only for the fully-certified review-pack lane profile. A certified review-pack claim may be certified
    /// on this profile alone; every other profile is at most a reviewable review-pack record structure or a
    /// narrowed projection.
    pub const fn is_certified_review_pack_lane(self) -> bool {
        matches!(self, Self::CertifiedReviewPackLane)
    }
}

/// The claim ladder a certified review-pack profile asserts and is certified down to. Minted locally for this
/// capstone: the strongest claim is a fully certified review-pack record; each weaker tier is a disclosed
/// projection that keeps the last-known pack-version/digest, owner-provenance, evidence-check, local-parity,
/// AI-pack-binding, or template-attribution posture explicit rather than overstating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackCertClaim {
    /// Certified review-pack truth: a fully-certified review pack whose pack version / digest, owner provenance,
    /// evaluator result class, local-versus-provider parity, pack freshness, and template attribution all join
    /// to one export-safe, provider-current, internally consistent record — the strongest claim, the review-pack
    /// handling Aureline can present as cleanly-evaluated and publish-safe across every consumer.
    CertifiedReviewPackTruth,
    /// Reviewable review-pack record: a self-sufficient, inspectable pack-bound record (a template / evidence
    /// record an operator can inspect) that is not itself a fully-certified review-pack lane.
    ReviewableReviewPackRecord,
    /// Pack-version-digest-unverified projection: a pack's version / digest cannot be confirmed fresh; the lane
    /// stays a pack-version-digest-unverified projection that discloses the last-known pack version / digest and
    /// marks the pack stale, never a stale pack looking current.
    PackVersionDigestUnverifiedProjection,
    /// Owner-provenance-unverified projection: a scope slice's advisory-versus-enforced owner provenance can no
    /// longer be verified; the lane stays an owner-provenance-unverified projection that keeps the last-known
    /// owner source explicit and never flattens advisory-owner and enforced-owner.
    OwnerProvenanceUnverifiedProjection,
    /// Evidence-check-unverified projection: a required check is ci-only, not-evaluated-here, or
    /// provider-unavailable; the lane stays an evidence-check-unverified projection that keeps the unevaluated
    /// state explicit, never hidden behind a green summary.
    EvidenceCheckUnverifiedProjection,
    /// Local-parity-unverified projection: a check's result is a local parity estimate that diverges from
    /// provider-authoritative state; the lane stays a local-parity-unverified projection that keeps the estimate
    /// labelled, never masquerading as provider-authoritative mergeability.
    LocalParityUnverifiedProjection,
    /// AI-pack-binding-unverified projection: an AI review ran under a pack version / digest that is not disclosed
    /// against the active pack; the lane stays an AI-pack-binding-unverified projection that discloses the pack
    /// version the AI review ran under.
    AiPackBindingUnverifiedProjection,
    /// Template-attribution-unverified projection: a template's attribution can no longer be verified against the
    /// pack it came from; the lane stays a template-attribution-unverified projection that keeps the last-known
    /// template version / digest and pack binding explicit.
    TemplateAttributionUnverifiedProjection,
}

impl M5ReviewPackCertClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::CertifiedReviewPackTruth,
        Self::ReviewableReviewPackRecord,
        Self::PackVersionDigestUnverifiedProjection,
        Self::OwnerProvenanceUnverifiedProjection,
        Self::EvidenceCheckUnverifiedProjection,
        Self::LocalParityUnverifiedProjection,
        Self::AiPackBindingUnverifiedProjection,
        Self::TemplateAttributionUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::CertifiedReviewPackTruth => 7,
            Self::ReviewableReviewPackRecord => 6,
            Self::PackVersionDigestUnverifiedProjection => 5,
            Self::OwnerProvenanceUnverifiedProjection => 4,
            Self::EvidenceCheckUnverifiedProjection => 3,
            Self::LocalParityUnverifiedProjection => 2,
            Self::AiPackBindingUnverifiedProjection => 1,
            Self::TemplateAttributionUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully-certified, certified review-pack record.
    pub const fn asserts_certified_review_pack_truth(self) -> bool {
        matches!(self, Self::CertifiedReviewPackTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (certified or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::CertifiedReviewPackTruth | Self::ReviewableReviewPackRecord
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedReviewPackTruth => "certified_review_pack_truth",
            Self::ReviewableReviewPackRecord => "reviewable_review_pack_record",
            Self::PackVersionDigestUnverifiedProjection => {
                "pack_version_digest_unverified_projection"
            }
            Self::OwnerProvenanceUnverifiedProjection => "owner_provenance_unverified_projection",
            Self::EvidenceCheckUnverifiedProjection => "evidence_check_unverified_projection",
            Self::LocalParityUnverifiedProjection => "local_parity_unverified_projection",
            Self::AiPackBindingUnverifiedProjection => "ai_pack_binding_unverified_projection",
            Self::TemplateAttributionUnverifiedProjection => {
                "template_attribution_unverified_projection"
            }
        }
    }
}

/// The nine truth axes a certified profile is scored on. These are exactly the parity dimensions the spec
/// requires verifying — visual, keyboard, screen-reader, high-zoom reflow, high-contrast, localization,
/// CLI/export, degraded-state, and review-pack-truth behavior. The CLI/export axis is always-on and must stay
/// certified for every profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackCertificationAxis {
    /// Visual parity: the pack label, pack version / digest, owner provenance, evaluator result class,
    /// local-versus-provider parity, pack freshness, and template attribution are shown on the primary surface
    /// without relying on a shell-chrome-only affordance or a mislabeled provider-authoritative-looking row
    /// alone, and no local parity estimate still reads as provider-authoritative mergeability.
    Visual,
    /// Keyboard-reach parity: the same review-pack truth and its bound review operations are reachable and
    /// operable without a pointer, never hover-only, with stable operation IDs.
    Keyboard,
    /// Screen-reader parity: the same truth is announced non-visually, never relying on a shell-chrome-only
    /// affordance, a mislabeled provider-authoritative-looking row, or an unlabeled control alone.
    ScreenReader,
    /// High-zoom reflow parity: the same truth reflows legibly at 200-400% zoom rather than clipping the pack
    /// version / digest, owner provenance, evaluator result class, local-versus-provider parity, or template
    /// attribution.
    HighZoomReflow,
    /// High-contrast parity: the same truth stays legible and operable in high-contrast mode, never dropping the
    /// evaluator result class badge, owner provenance, or local-versus-provider parity.
    HighContrast,
    /// Localization parity: the same truth stays host-correct and faithful across locales, never mislabeling an
    /// evaluator result class, owner provenance, parity state, or template attribution when a locale is
    /// incomplete.
    Localization,
    /// CLI / export parity (always-on): the certified profile state is reconstructable as text / JSON / Markdown
    /// for support and automation.
    CliExport,
    /// Degraded-state parity: a stale pack version / digest, a missing owner provenance, an unevaluated required
    /// check, a local-only parity estimate, an undisclosed AI pack binding, or a stale template attribution
    /// honestly downgrades a `CertifiedReviewPackTruth` / `ReviewableReviewPackRecord` claim rather than reading
    /// as a fresh, provider-current review-pack record.
    DegradedState,
    /// Review-pack-truth parity: the pack label, pack version / digest, owner provenance, evaluator result class,
    /// local-versus-provider parity, pack freshness, and template attribution stay explicit and never let a
    /// local parity estimate masquerade as provider-authoritative; hide a ci-only, not-evaluated-here, or
    /// provider-unavailable state behind a green summary; flatten advisory-owner and enforced-owner into one
    /// owner pill; let AI review run under a different pack version without disclosure; or lose the review-pack
    /// version / digest or template attribution when exporting, publishing, or reopening review evidence.
    ReviewPackTruth,
}

impl ReviewPackCertificationAxis {
    /// Every certification axis, in declaration order.
    pub const ALL: [ReviewPackCertificationAxis; 9] = [
        ReviewPackCertificationAxis::Visual,
        ReviewPackCertificationAxis::Keyboard,
        ReviewPackCertificationAxis::ScreenReader,
        ReviewPackCertificationAxis::HighZoomReflow,
        ReviewPackCertificationAxis::HighContrast,
        ReviewPackCertificationAxis::Localization,
        ReviewPackCertificationAxis::CliExport,
        ReviewPackCertificationAxis::DegradedState,
        ReviewPackCertificationAxis::ReviewPackTruth,
    ];

    /// The always-on CLI/export axis that must stay certified on every row.
    pub const fn is_always_on(self) -> bool {
        matches!(self, Self::CliExport)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visual => "visual",
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrast => "high_contrast",
            Self::Localization => "localization",
            Self::CliExport => "cli_export",
            Self::DegradedState => "degraded_state",
            Self::ReviewPackTruth => "review_pack_truth",
        }
    }
}

/// The certification state of one truth axis on one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackAxisCertificationState {
    /// Green: parity is current; the axis fully certifies.
    Certified,
    /// Yellow: parity is not current, but the reduction is disclosed and binds to a visible claim narrowing.
    DisclosedNarrowed,
    /// Red: parity is not current and the profile hides it behind a trusted claim inherited from a healthier
    /// profile.
    UndisclosedDrift,
}

impl ReviewPackAxisCertificationState {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::UndisclosedDrift => "undisclosed_drift",
        }
    }
}

/// The derived certification verdict for a whole profile. Never asserted by the author — always recomputed from
/// the axis outcomes, guardrails, and claim narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackProfileClaimStatus {
    /// Full standing: every axis certified, every invariant held, claimed configuration tier delivered.
    Green,
    /// Disclosed narrowing: an axis is not current and the claim narrows visibly.
    Yellow,
    /// Blocked: a degraded axis hides behind a full claim, a hard invariant breaks, CLI/export parity drops, a
    /// non-lane review-pack profile claims a certified review-pack record, or the narrowing is inconsistent.
    Red,
}

impl ReviewPackProfileClaimStatus {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// True when the profile is publishable as certified (green or disclosed yellow); red profiles block the
    /// release.
    pub const fn is_publishable(self) -> bool {
        !matches!(self, Self::Red)
    }
}

/// The five B152 hard invariants carried on every certified profile. All five must hold — a breach blocks the
/// profile (red). Each field is `true` only when the profile *breaks* the invariant, so a clean profile carries
/// all-false. The field names are the frozen matrix's exact hard-invariant vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackCertGuardrails {
    /// True if the profile lets a local parity estimate masquerade as provider-authoritative mergeability or
    /// approval truth. Must be false.
    pub lets_a_local_parity_estimate_masquerade_as_provider_authoritative: bool,
    /// True if the profile hides a ci-only, not-evaluated-here, or provider-unavailable state behind a green
    /// summary. Must be false.
    pub hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: bool,
    /// True if the profile flattens advisory-owner and enforced-owner into one owner pill. Must be false.
    pub flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: bool,
    /// True if the profile lets AI review run under a different pack version without disclosure. Must be false.
    pub lets_ai_review_run_under_a_different_pack_version_without_disclosure: bool,
    /// True if the profile loses the review-pack version / digest or template attribution when exporting,
    /// publishing, or reopening review evidence. Must be false.
    pub loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening:
        bool,
}

impl ReviewPackCertGuardrails {
    /// A clean profile: every invariant held.
    pub const CLEAN: Self = Self {
        lets_a_local_parity_estimate_masquerade_as_provider_authoritative: false,
        hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary: false,
        flattens_advisory_owner_and_enforced_owner_into_one_owner_pill: false,
        lets_ai_review_run_under_a_different_pack_version_without_disclosure: false,
        loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening:
            false,
    };

    /// True when every invariant holds (no field is set).
    pub const fn all_held(&self) -> bool {
        !self.lets_a_local_parity_estimate_masquerade_as_provider_authoritative
            && !self.hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary
            && !self.flattens_advisory_owner_and_enforced_owner_into_one_owner_pill
            && !self.lets_ai_review_run_under_a_different_pack_version_without_disclosure
            && !self.loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening
    }
}

/// The copy / export parity a certified profile preserves. The CLI/export axis certifies only when this offers
/// text / JSON / Markdown reconstruction and prohibits a raw-payload-only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackCertExportParity {
    /// The copy formats the profile offers (must include text / json / markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The pack-version-digest / owner-provenance / evaluator-result-class / parity-state / pack-freshness /
    /// template-attribution fields the profile preserves in export.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a raw-payload-only export is prohibited.
    pub raw_payload_only_prohibited: bool,
}

impl ReviewPackCertExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a raw-payload-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.raw_payload_only_prohibited
    }
}

/// One axis outcome on one certified profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackAxisOutcome {
    /// The truth axis this outcome scores.
    pub axis: ReviewPackCertificationAxis,
    /// The certification state of the axis.
    pub state: ReviewPackAxisCertificationState,
    /// The parity note recorded for this axis (always present).
    pub parity_note: String,
    /// The narrowing reason; present iff the axis is not certified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_reason: Option<String>,
    /// The frozen downgrade trigger; present iff the axis is disclosed-narrowed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<M5ReviewPackDowngradeTrigger>,
}

impl ReviewPackAxisOutcome {
    /// Whether the outcome's optional fields are consistent with its state.
    ///
    /// - `Certified` carries neither a narrowing reason nor a trigger.
    /// - `DisclosedNarrowed` carries a non-generic reason *and* a frozen trigger.
    /// - `UndisclosedDrift` carries a reason describing the hidden drift but no visible trigger (that is exactly
    ///   what makes it undisclosed).
    pub fn well_formed(&self) -> bool {
        if self.parity_note.trim().is_empty() {
            return false;
        }
        match self.state {
            ReviewPackAxisCertificationState::Certified => {
                self.narrowing_reason.is_none() && self.downgrade_trigger.is_none()
            }
            ReviewPackAxisCertificationState::DisclosedNarrowed => {
                let reason_ok = self
                    .narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty() && !label_is_generic(r));
                reason_ok && self.downgrade_trigger.is_some()
            }
            ReviewPackAxisCertificationState::UndisclosedDrift => {
                self.narrowing_reason
                    .as_deref()
                    .is_some_and(|r| !r.trim().is_empty())
                    && self.downgrade_trigger.is_none()
            }
        }
    }
}

/// The visible claim narrowing a profile applies when a truth axis is not current. Present iff the certified
/// claim is strictly weaker than the claimed one.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackClaimAutoNarrow {
    /// The axis whose degraded parity forced the narrowing.
    pub binding_axis: ReviewPackCertificationAxis,
    /// The claim the profile would deliver at full parity.
    pub from_claim: M5ReviewPackCertClaim,
    /// The weakest supported claim the profile is certified down to.
    pub to_claim: M5ReviewPackCertClaim,
    /// The visible, non-generic disclosure label.
    pub visible_label: String,
}

/// One certified M5 review-pack object-bearing profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackProfileCertificationRow {
    /// Record kind; must equal [`REVIEW_PACK_CERT_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_PACK_CERT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The certified profile.
    pub profile: M5ReviewPackCertifiedProfile,
    /// The configuration claim ceiling the profile asserts.
    pub claimed_claim: M5ReviewPackCertClaim,
    /// The weakest supported claim the profile is certified down to. Must be no stronger than `claimed_claim`.
    pub certified_claim: M5ReviewPackCertClaim,
    /// The frozen review-pack object classes this profile renders (at least one).
    #[serde(default)]
    pub consumed_families: Vec<M5ReviewPackObject>,
    /// One outcome per [`ReviewPackCertificationAxis`], each axis appearing once.
    #[serde(default)]
    pub axis_outcomes: Vec<ReviewPackAxisOutcome>,
    /// The B152 hard invariants; all must hold.
    pub guardrails: ReviewPackCertGuardrails,
    /// The visible claim narrowing; present iff `certified_claim` is weaker than `claimed_claim`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_auto_narrow: Option<ReviewPackClaimAutoNarrow>,
    /// The one canonical review-pack evaluator matrix proof bundle this profile cites. Must equal
    /// [`REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF`].
    pub canonical_bundle_ref: String,
    /// The derived verdict. Recomputed and compared on validation.
    pub derived_status: ReviewPackProfileClaimStatus,
    /// The copy / export parity of the certified profile state.
    pub export_parity: ReviewPackCertExportParity,
    /// The compatibility notes captured for this profile.
    #[serde(default)]
    pub compatibility_notes: Vec<String>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the certification was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ReviewPackProfileCertificationRow {
    /// The outcome for a given axis, if present.
    pub fn axis(&self, axis: ReviewPackCertificationAxis) -> Option<&ReviewPackAxisOutcome> {
        self.axis_outcomes.iter().find(|o| o.axis == axis)
    }

    /// Whether every axis appears exactly once.
    pub fn covers_all_axes(&self) -> bool {
        let seen: BTreeSet<ReviewPackCertificationAxis> =
            self.axis_outcomes.iter().map(|o| o.axis).collect();
        seen.len() == self.axis_outcomes.len()
            && ReviewPackCertificationAxis::ALL
                .iter()
                .all(|a| seen.contains(a))
    }

    /// Whether every axis outcome is internally well-formed.
    pub fn axis_outcomes_well_formed(&self) -> bool {
        self.axis_outcomes
            .iter()
            .all(ReviewPackAxisOutcome::well_formed)
    }

    /// True when the profile narrows its configuration claim below what it asserts.
    pub fn is_claim_narrowed(&self) -> bool {
        self.certified_claim.capability_rank() < self.claimed_claim.capability_rank()
    }

    /// The axes disclosed as narrowed (yellow).
    pub fn narrowed_axes(&self) -> Vec<ReviewPackCertificationAxis> {
        self.axis_outcomes
            .iter()
            .filter(|o| o.state == ReviewPackAxisCertificationState::DisclosedNarrowed)
            .map(|o| o.axis)
            .collect()
    }

    /// Derives the profile verdict from its axes, invariants, and claim narrowing. This is the heart of the
    /// capstone: a degraded axis must produce a visible claim narrowing, only a fully-certified review-pack lane
    /// profile may certify a certified review-pack record, every hard invariant must hold, CLI/export parity must
    /// always certify, and the narrowing must be consistent.
    pub fn derive_status(&self) -> ReviewPackProfileClaimStatus {
        // Structural prerequisites: malformed rows can never certify.
        if !self.covers_all_axes()
            || !self.axis_outcomes_well_formed()
            || self.canonical_bundle_ref != REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF
            || self.consumed_families.is_empty()
            || !self.export_parity.is_complete()
        {
            return ReviewPackProfileClaimStatus::Red;
        }

        // Every B152 hard invariant must hold.
        if !self.guardrails.all_held() {
            return ReviewPackProfileClaimStatus::Red;
        }

        // Certification may only narrow the claim, never strengthen it.
        if self.certified_claim.capability_rank() > self.claimed_claim.capability_rank() {
            return ReviewPackProfileClaimStatus::Red;
        }

        // Only a fully-certified review-pack lane profile may certify a certified review-pack record.
        if self.certified_claim.asserts_certified_review_pack_truth()
            && !self.profile.is_certified_review_pack_lane()
        {
            return ReviewPackProfileClaimStatus::Red;
        }

        // The always-on CLI/export axis must stay certified.
        match self.axis(ReviewPackCertificationAxis::CliExport) {
            Some(o) if o.state == ReviewPackAxisCertificationState::Certified => {}
            _ => return ReviewPackProfileClaimStatus::Red,
        }

        // Any undisclosed drift blocks outright.
        if self
            .axis_outcomes
            .iter()
            .any(|o| o.state == ReviewPackAxisCertificationState::UndisclosedDrift)
        {
            return ReviewPackProfileClaimStatus::Red;
        }

        let narrowed = self.narrowed_axes();
        let claim_narrowed = self.is_claim_narrowed();

        match (&self.claim_auto_narrow, claim_narrowed) {
            // Spurious narrowing structure without a claim reduction.
            (Some(_), false) => return ReviewPackProfileClaimStatus::Red,
            // A claim reduction with no disclosed narrowing structure.
            (None, true) => return ReviewPackProfileClaimStatus::Red,
            (Some(narrow), true) => {
                if narrow.from_claim != self.claimed_claim
                    || narrow.to_claim != self.certified_claim
                    || !narrowed.contains(&narrow.binding_axis)
                    || narrow.binding_axis.is_always_on()
                    || narrow.visible_label.trim().is_empty()
                    || label_is_generic(&narrow.visible_label)
                {
                    return ReviewPackProfileClaimStatus::Red;
                }
            }
            (None, false) => {}
        }

        if claim_narrowed {
            // A disclosed, consistently-bound narrowing.
            return ReviewPackProfileClaimStatus::Yellow;
        }

        // Claim not narrowed: a degraded axis retained behind a full claim is a hidden overclaim inheriting a
        // healthier profile's truth.
        if !narrowed.is_empty() {
            return ReviewPackProfileClaimStatus::Red;
        }

        ReviewPackProfileClaimStatus::Green
    }

    /// Whether the stored `derived_status` matches a fresh recomputation.
    pub fn status_is_fresh(&self) -> bool {
        self.derived_status == self.derive_status()
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == REVIEW_PACK_CERT_ROW_RECORD_KIND
            && self.schema_version == REVIEW_PACK_CERT_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.canonical_bundle_ref.trim().is_empty()
            && !self.consumed_families.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.compatibility_notes.is_empty()
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "profile={profile} claimed={claimed} certified={certified} status={status} \
narrowed_axes={narrowed}",
            profile = self.profile.as_str(),
            claimed = self.claimed_claim.as_str(),
            certified = self.certified_claim.as_str(),
            status = self.derived_status.as_str(),
            narrowed = self.narrowed_axes().len(),
        )
    }
}

/// Rolled-up summary of an M05-1283 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackProfileCertificationSummary {
    pub row_count: usize,
    pub profile_count: usize,
    pub green_row_count: usize,
    pub yellow_row_count: usize,
    pub red_row_count: usize,
    pub all_profiles_present: bool,
    pub all_families_covered: bool,
    pub all_rows_publishable: bool,
    pub all_status_fresh: bool,
    pub all_rows_cite_canonical_bundle: bool,
    pub all_rows_export_parity_certified: bool,
    pub all_guardrails_held: bool,
    pub every_axis_covered_on_every_row: bool,
    pub narrowed_profile_count: usize,
    pub report_clean: bool,
}

/// Constructor input for [`ReviewPackProfileCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackProfileCertificationPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    pub rows: Vec<ReviewPackProfileCertificationRow>,
}

/// Checked-in M05-1283 certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackProfileCertificationPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub canonical_bundle_ref: String,
    #[serde(default)]
    pub rows: Vec<ReviewPackProfileCertificationRow>,
    pub summary: ReviewPackProfileCertificationSummary,
}

impl ReviewPackProfileCertificationPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ReviewPackProfileCertificationPacketInput) -> Self {
        let mut packet = Self {
            schema_version: REVIEW_PACK_CERT_SCHEMA_VERSION,
            record_kind: REVIEW_PACK_CERT_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            canonical_bundle_ref: input.canonical_bundle_ref,
            rows: input.rows,
            summary: ReviewPackProfileCertificationSummary {
                row_count: 0,
                profile_count: 0,
                green_row_count: 0,
                yellow_row_count: 0,
                red_row_count: 0,
                all_profiles_present: false,
                all_families_covered: false,
                all_rows_publishable: false,
                all_status_fresh: false,
                all_rows_cite_canonical_bundle: false,
                all_rows_export_parity_certified: false,
                all_guardrails_held: false,
                every_axis_covered_on_every_row: false,
                narrowed_profile_count: 0,
                report_clean: false,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5ReviewPackCertifiedProfile> {
        self.rows.iter().map(|r| r.profile).collect()
    }

    /// Review-pack object classes rendered by some certified profile in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ReviewPackObject> {
        self.rows
            .iter()
            .flat_map(|r| r.consumed_families.iter().copied())
            .collect()
    }

    /// Whether every certified profile appears exactly once.
    pub fn all_profiles_present(&self) -> bool {
        let profiles = self.represented_profiles();
        profiles.len() == self.rows.len()
            && M5ReviewPackCertifiedProfile::ALL
                .iter()
                .all(|s| profiles.contains(s))
    }

    /// Whether every frozen review-pack object class is certified on at least one profile — proof the full
    /// matrix runs across the claimed consumers.
    pub fn all_families_covered(&self) -> bool {
        let families = self.represented_families();
        M5ReviewPackObject::ALL.iter().all(|f| families.contains(f))
    }

    /// Whether a CLI/export axis is certified on every row.
    pub fn all_rows_export_parity_certified(&self) -> bool {
        self.rows.iter().all(|r| {
            r.axis(ReviewPackCertificationAxis::CliExport)
                .is_some_and(|o| o.state == ReviewPackAxisCertificationState::Certified)
                && r.export_parity.is_complete()
        })
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ReviewPackProfileCertificationSummary {
        let profiles = self.represented_profiles();
        let green = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ReviewPackProfileClaimStatus::Green)
            .count();
        let yellow = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ReviewPackProfileClaimStatus::Yellow)
            .count();
        let red = self
            .rows
            .iter()
            .filter(|r| r.derived_status == ReviewPackProfileClaimStatus::Red)
            .count();
        let all_publishable = self.rows.iter().all(|r| r.derived_status.is_publishable());
        let all_fresh = self
            .rows
            .iter()
            .all(ReviewPackProfileCertificationRow::status_is_fresh);
        let all_profiles = self.all_profiles_present();
        let all_families = self.all_families_covered();

        ReviewPackProfileCertificationSummary {
            row_count: self.rows.len(),
            profile_count: profiles.len(),
            green_row_count: green,
            yellow_row_count: yellow,
            red_row_count: red,
            all_profiles_present: all_profiles,
            all_families_covered: all_families,
            all_rows_publishable: all_publishable,
            all_status_fresh: all_fresh,
            all_rows_cite_canonical_bundle: self
                .rows
                .iter()
                .all(|r| r.canonical_bundle_ref == REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF),
            all_rows_export_parity_certified: self.all_rows_export_parity_certified(),
            all_guardrails_held: self.rows.iter().all(|r| r.guardrails.all_held()),
            every_axis_covered_on_every_row: self
                .rows
                .iter()
                .all(ReviewPackProfileCertificationRow::covers_all_axes),
            narrowed_profile_count: self.rows.iter().filter(|r| r.is_claim_narrowed()).count(),
            report_clean: all_publishable && all_fresh && all_profiles && all_families,
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ReviewPackCertificationViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REVIEW_PACK_CERT_SCHEMA_VERSION {
            violations.push(ReviewPackCertificationViolation::SchemaVersion {
                expected: REVIEW_PACK_CERT_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REVIEW_PACK_CERT_RECORD_KIND {
            violations.push(ReviewPackCertificationViolation::RecordKind {
                expected: REVIEW_PACK_CERT_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ReviewPackCertificationViolation::MissingIdentity);
        }
        if self.canonical_bundle_ref != REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF {
            violations.push(ReviewPackCertificationViolation::WrongCanonicalBundle);
        }

        let mut row_ids = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ReviewPackCertificationViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }

            if !row.is_complete() {
                violations.push(ReviewPackCertificationViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            if !row.covers_all_axes() {
                violations.push(ReviewPackCertificationViolation::AxisCoverageIncomplete {
                    id: row.row_id.clone(),
                });
            }

            if !row.axis_outcomes_well_formed() {
                violations.push(ReviewPackCertificationViolation::MalformedAxisOutcome {
                    id: row.row_id.clone(),
                });
            }

            if row.canonical_bundle_ref != REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF {
                violations.push(
                    ReviewPackCertificationViolation::RowMissingCanonicalBundle {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Every B152 hard invariant must hold.
            if !row.guardrails.all_held() {
                violations.push(ReviewPackCertificationViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                });
            }

            // Only a fully-certified review-pack lane profile may certify a certified review-pack record.
            if row.certified_claim.asserts_certified_review_pack_truth()
                && !row.profile.is_certified_review_pack_lane()
            {
                violations.push(
                    ReviewPackCertificationViolation::NonLaneProfileClaimsCertifiedTruth {
                        id: row.row_id.clone(),
                    },
                );
            }

            // CLI/export parity is always-on.
            if !row.export_parity.is_complete()
                || row
                    .axis(ReviewPackCertificationAxis::CliExport)
                    .is_none_or_state_not_certified()
            {
                violations.push(ReviewPackCertificationViolation::ExportParityNotCertified {
                    id: row.row_id.clone(),
                });
            }

            // Certification may never strengthen a claim.
            if row.certified_claim.capability_rank() > row.claimed_claim.capability_rank() {
                violations.push(
                    ReviewPackCertificationViolation::CertifiedClaimExceedsClaim {
                        id: row.row_id.clone(),
                    },
                );
            }

            // The stored verdict must match a fresh recomputation.
            if !row.status_is_fresh() {
                violations.push(ReviewPackCertificationViolation::StatusDerivationStale {
                    id: row.row_id.clone(),
                });
            }

            // A blocked (red) profile must not ship in a clean packet.
            if row.derived_status == ReviewPackProfileClaimStatus::Red {
                violations.push(ReviewPackCertificationViolation::ProfileBlocked {
                    id: row.row_id.clone(),
                });
            }
        }

        // Every claimed profile must be certified exactly once.
        if !self.all_profiles_present() {
            violations.push(ReviewPackCertificationViolation::ProfileCoverageIncomplete);
        }

        // Every frozen review-pack object class must be certified on some profile.
        if !self.all_families_covered() {
            violations.push(ReviewPackCertificationViolation::FamilyCoverageIncomplete);
        }

        if self.summary != self.computed_summary() {
            violations.push(ReviewPackCertificationViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(ReviewPackCertificationViolation::RawReviewPackMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("certification packet serializes")
    }

    /// Deterministic CSV of the certification rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,profile,claimed_claim,certified_claim,status,narrowed_axes,binding_axis\n",
        );
        for row in &self.rows {
            let binding = row
                .claim_auto_narrow
                .as_ref()
                .map(|n| n.binding_axis.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{id},{profile},{claimed},{certified},{status},{narrowed},{binding}\n",
                id = row.row_id,
                profile = row.profile.as_str(),
                claimed = row.claimed_claim.as_str(),
                certified = row.certified_claim.as_str(),
                status = row.derived_status.as_str(),
                narrowed = row.narrowed_axes().len(),
                binding = binding,
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Review-Pack Surface Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Canonical bundle: `{}`\n",
            self.canonical_bundle_ref
        ));
        out.push_str(&format!(
            "- Profiles: {} / {} certified ({} green, {} yellow, {} red)\n",
            self.summary.profile_count,
            M5ReviewPackCertifiedProfile::ALL.len(),
            self.summary.green_row_count,
            self.summary.yellow_row_count,
            self.summary.red_row_count,
        ));
        out.push_str(&format!(
            "- Families covered: {}\n",
            self.summary.all_families_covered
        ));
        out.push_str(&format!(
            "- Invariants held: {}\n",
            self.summary.all_guardrails_held
        ));
        out.push_str(&format!(
            "- Auto-narrowed profiles: {}\n",
            self.summary.narrowed_profile_count,
        ));
        out.push_str(&format!("- Report clean: {}\n", self.summary.report_clean));
        out.push_str("\n## Profiles\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in certification export.
pub fn current_m5_review_pack_surface_certification_export(
) -> Result<ReviewPackProfileCertificationPacket, ReviewPackCertificationArtifactError> {
    let packet: ReviewPackProfileCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-surface-certification/support_export.json"
    )))
    .map_err(ReviewPackCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewPackCertificationArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum ReviewPackCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewPackCertificationViolation>),
}

impl fmt::Display for ReviewPackCertificationArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "certification export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ReviewPackCertificationArtifactError {}

/// Validation failure for M05-1283 certification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackCertificationViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    WrongCanonicalBundle,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    AxisCoverageIncomplete { id: String },
    MalformedAxisOutcome { id: String },
    RowMissingCanonicalBundle { id: String },
    GuardrailViolated { id: String },
    NonLaneProfileClaimsCertifiedTruth { id: String },
    ExportParityNotCertified { id: String },
    CertifiedClaimExceedsClaim { id: String },
    StatusDerivationStale { id: String },
    ProfileBlocked { id: String },
    ProfileCoverageIncomplete,
    FamilyCoverageIncomplete,
    SummaryMismatch,
    RawReviewPackMaterialInExport,
}

impl fmt::Display for ReviewPackCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::WrongCanonicalBundle => {
                write!(
                    f,
                    "packet does not cite the canonical review-pack evaluator matrix proof bundle"
                )
            }
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete certification row: {id}"),
            Self::AxisCoverageIncomplete { id } => {
                write!(
                    f,
                    "row {id} does not score every certification axis exactly once"
                )
            }
            Self::MalformedAxisOutcome { id } => {
                write!(
                    f,
                    "row {id} has an axis outcome whose disclosure fields disagree with its state"
                )
            }
            Self::RowMissingCanonicalBundle { id } => {
                write!(
                    f,
                    "row {id} does not cite the one canonical review-pack evaluator matrix proof bundle"
                )
            }
            Self::GuardrailViolated { id } => {
                write!(
                    f,
                    "row {id} breaks a B152 hard invariant: letting a local parity estimate masquerade as \
provider-authoritative; hiding a ci-only, not-evaluated-here, or provider-unavailable state behind a green \
summary; flattening advisory-owner and enforced-owner into one owner pill; letting AI review run under a \
different pack version without disclosure; or losing the review-pack version / digest or template attribution \
when exporting, publishing, or reopening review evidence"
                )
            }
            Self::NonLaneProfileClaimsCertifiedTruth { id } => {
                write!(
                    f,
                    "row {id} certifies a certified review-pack record on a non-lane profile"
                )
            }
            Self::ExportParityNotCertified { id } => {
                write!(
                    f,
                    "row {id} drops always-on CLI/export parity (text / JSON / Markdown reconstruction)"
                )
            }
            Self::CertifiedClaimExceedsClaim { id } => {
                write!(
                    f,
                    "row {id} certifies a claim stronger than the claimed one"
                )
            }
            Self::StatusDerivationStale { id } => {
                write!(
                    f,
                    "row {id} stored status disagrees with a fresh derivation"
                )
            }
            Self::ProfileBlocked { id } => {
                write!(
                    f,
                    "row {id} is blocked (red): a degraded axis is hidden behind a fresh certified claim, a hard \
invariant broke, CLI/export parity dropped, a non-lane profile claimed a certified review-pack record, or the \
narrowing is inconsistent"
                )
            }
            Self::ProfileCoverageIncomplete => {
                write!(
                    f,
                    "not every claimed M5 review-pack profile is certified exactly once"
                )
            }
            Self::FamilyCoverageIncomplete => {
                write!(
                    f,
                    "not every frozen review-pack object class is certified on some profile"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawReviewPackMaterialInExport => {
                write!(
                    f,
                    "export contains a raw credential, plaintext secret, bearer token, endpoint URL, or private-key material"
                )
            }
        }
    }
}

impl Error for ReviewPackCertificationViolation {}

/// Small extension so the export-parity check reads cleanly.
trait AxisOutcomeOptionExt {
    fn is_none_or_state_not_certified(&self) -> bool;
}

impl AxisOutcomeOptionExt for Option<&ReviewPackAxisOutcome> {
    fn is_none_or_state_not_certified(&self) -> bool {
        match self {
            None => true,
            Some(o) => o.state != ReviewPackAxisCertificationState::Certified,
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise disclosure. Includes the review-pack generics
/// the spec forbids collapsing distinct pack-version/digest, owner-provenance, evaluator-result-class, parity,
/// and template-attribution truth into (whole-label matches so a full sentence naming a concrete pack version,
/// owner source, or parity state is not flagged).
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "something went wrong"
            | "degraded"
            | "narrowed"
            | "reduced"
            | "stale"
            | "unverified"
            | "offline"
            | "warning"
            | "blocked"
            | "pending"
            | "loading"
            | "partial"
            | "certified"
            | "reviewable"
            | "review pack"
            | "review-pack"
            | "pack"
            | "review"
            | "review finding"
            | "owner"
            | "ownership"
            | "advisory owner"
            | "enforced owner"
            | "owner pill"
            | "provider"
            | "provider authoritative"
            | "local"
            | "local estimate"
            | "local parity"
            | "parity"
            | "parity state"
            | "ci only"
            | "not evaluated here"
            | "provider unavailable"
            | "evaluator result"
            | "result class"
            | "pack version"
            | "pack digest"
            | "version"
            | "digest"
            | "freshness"
            | "pack freshness"
            | "template"
            | "template attribution"
            | "attribution"
            | "ai review"
            | "ai policy"
            | "policy hook"
            | "scope"
            | "check"
            | "required check"
            | "evidence"
            | "export"
            | "export fallback"
            | "copy"
            | "fallback"
            | "drift"
            | "mismatch"
            | "more"
            | "…"
            | "..."
            | "overflow"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON. Mirrors the review-pack evaluator
/// matrix heuristic so the reused [`M5ReviewPackDowngradeTrigger`] narrowings serialize cleanly — the
/// review-pack proof grammar carries only typed class tokens and opaque refs, never raw secret values or
/// endpoints.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

// --------------------------------------------------------------------------
// Seed builder — the one source of truth shared by the tests and the on-disk
// support export so both stay byte-aligned.
// --------------------------------------------------------------------------

/// Builds the canonical, checked-in M05-1283 certification packet. Certifies all eight claimed M5 review-pack
/// profiles: two deliver their claim (green) and six auto-narrow a not-current truth axis to a weaker
/// configuration ceiling (yellow). No profile hides drift or breaks a hard invariant (red).
pub fn seeded_m5_review_pack_surface_certification_packet() -> ReviewPackProfileCertificationPacket
{
    ReviewPackProfileCertificationPacket::new(ReviewPackProfileCertificationPacketInput {
        packet_id: REVIEW_PACK_CERT_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: REVIEW_PACK_CERT_MATRIX_REF.to_owned(),
        canonical_bundle_ref: REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn seed_evidence(id: &str) -> Vec<String> {
    vec![
        format!("evidence:review-pack-surface-certification:{id}"),
        REVIEW_PACK_CERT_CONSUMERS_BUNDLE_REF.to_owned(),
    ]
}

fn seed_export_parity(fields: &[&str]) -> ReviewPackCertExportParity {
    ReviewPackCertExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn seed_certified_note(axis: ReviewPackCertificationAxis) -> &'static str {
    match axis {
        ReviewPackCertificationAxis::Visual => {
            "pack label, pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, pack freshness, and template attribution shown on-surface without a shell-chrome-only affordance or a mislabeled provider-authoritative-looking row alone, and no local parity estimate still reads as provider-authoritative mergeability"
        }
        ReviewPackCertificationAxis::Keyboard => {
            "the same pack version / digest, owner provenance, evaluator result class, and bound review operations are keyboard-reachable with stable operation IDs, never hover-only"
        }
        ReviewPackCertificationAxis::ScreenReader => {
            "the same review-pack truth is announced non-visually, never a shell-chrome-only / mislabeled-provider-authoritative-row / unlabeled-control-only cue"
        }
        ReviewPackCertificationAxis::HighZoomReflow => {
            "the same truth reflows legibly at 200-400% zoom without clipping the pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, or template attribution"
        }
        ReviewPackCertificationAxis::HighContrast => {
            "the same truth stays legible and operable in high-contrast mode without dropping the evaluator result class badge, owner provenance, or local-versus-provider parity"
        }
        ReviewPackCertificationAxis::Localization => {
            "the same truth stays host-correct and faithful across locales without mislabeling an evaluator result class, owner provenance, parity state, or template attribution"
        }
        ReviewPackCertificationAxis::CliExport => {
            "profile state exports as text / JSON / Markdown for support replay"
        }
        ReviewPackCertificationAxis::DegradedState => {
            "a stale pack version / digest, a missing owner provenance, an unevaluated required check, a local-only parity estimate, an undisclosed AI pack binding, or a stale template attribution honestly downgrades the CertifiedReviewPackTruth/ReviewableReviewPackRecord claim rather than reading as a fresh, provider-current review-pack record"
        }
        ReviewPackCertificationAxis::ReviewPackTruth => {
            "pack label, pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, pack freshness, and template attribution stay explicit and never let a local parity estimate masquerade as provider-authoritative, hide a ci-only / not-evaluated-here / provider-unavailable state behind a green summary, flatten advisory-owner and enforced-owner into one owner pill, let AI review run under a different pack version without disclosure, or lose the review-pack version / digest or template attribution when exporting, publishing, or reopening review evidence"
        }
    }
}

fn seed_certified(axis: ReviewPackCertificationAxis) -> ReviewPackAxisOutcome {
    ReviewPackAxisOutcome {
        axis,
        state: ReviewPackAxisCertificationState::Certified,
        parity_note: seed_certified_note(axis).to_owned(),
        narrowing_reason: None,
        downgrade_trigger: None,
    }
}

fn seed_narrowed(
    axis: ReviewPackCertificationAxis,
    note: &str,
    reason: &str,
    trigger: M5ReviewPackDowngradeTrigger,
) -> ReviewPackAxisOutcome {
    ReviewPackAxisOutcome {
        axis,
        state: ReviewPackAxisCertificationState::DisclosedNarrowed,
        parity_note: note.to_owned(),
        narrowing_reason: Some(reason.to_owned()),
        downgrade_trigger: Some(trigger),
    }
}

fn seed_all_certified() -> Vec<ReviewPackAxisOutcome> {
    ReviewPackCertificationAxis::ALL
        .iter()
        .copied()
        .map(seed_certified)
        .collect()
}

fn seed_certified_except(
    axis: ReviewPackCertificationAxis,
    outcome: ReviewPackAxisOutcome,
) -> Vec<ReviewPackAxisOutcome> {
    ReviewPackCertificationAxis::ALL
        .iter()
        .copied()
        .map(|a| {
            if a == axis {
                outcome.clone()
            } else {
                seed_certified(a)
            }
        })
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn seed_row(
    row_id: &str,
    profile: M5ReviewPackCertifiedProfile,
    claimed_claim: M5ReviewPackCertClaim,
    certified_claim: M5ReviewPackCertClaim,
    consumed_families: &[M5ReviewPackObject],
    axis_outcomes: Vec<ReviewPackAxisOutcome>,
    claim_auto_narrow: Option<ReviewPackClaimAutoNarrow>,
    export_fields: &[&str],
    compatibility_notes: &[&str],
) -> ReviewPackProfileCertificationRow {
    let mut row = ReviewPackProfileCertificationRow {
        record_kind: REVIEW_PACK_CERT_ROW_RECORD_KIND.to_owned(),
        schema_version: REVIEW_PACK_CERT_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        profile,
        claimed_claim,
        certified_claim,
        consumed_families: consumed_families.to_vec(),
        axis_outcomes,
        guardrails: ReviewPackCertGuardrails::CLEAN,
        claim_auto_narrow,
        canonical_bundle_ref: REVIEW_PACK_CERT_CANONICAL_BUNDLE_REF.to_owned(),
        derived_status: ReviewPackProfileClaimStatus::Green,
        export_parity: seed_export_parity(export_fields),
        compatibility_notes: compatibility_notes
            .iter()
            .map(|n| (*n).to_owned())
            .collect(),
        source_refs: vec![
            REVIEW_PACK_CERT_MATRIX_REF.to_owned(),
            REVIEW_PACK_CERT_SCHEMA_REF.to_owned(),
        ],
        observed_at: "2026-07-16T00:00:00Z".to_owned(),
        evidence_refs: seed_evidence(row_id),
    };
    row.derived_status = row.derive_status();
    row
}

fn seed_narrow(
    binding_axis: ReviewPackCertificationAxis,
    from_claim: M5ReviewPackCertClaim,
    to_claim: M5ReviewPackCertClaim,
    label: &str,
) -> ReviewPackClaimAutoNarrow {
    ReviewPackClaimAutoNarrow {
        binding_axis,
        from_claim,
        to_claim,
        visible_label: label.to_owned(),
    }
}

fn seeded_rows() -> Vec<ReviewPackProfileCertificationRow> {
    use M5ReviewPackCertClaim::*;
    use M5ReviewPackCertifiedProfile as P;
    use M5ReviewPackDowngradeTrigger as Trig;
    use M5ReviewPackObject::*;
    use ReviewPackCertificationAxis as Ax;

    vec![
        // --- Green: full parity, claim delivered ---------------------------
        seed_row(
            "cert:certified-review-pack-lane",
            P::CertifiedReviewPackLane,
            CertifiedReviewPackTruth,
            CertifiedReviewPackTruth,
            &[ReviewPackRecord],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "pack_version_and_digest",
            ],
            &[
                "certified review-pack lane: the pack label, pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, pack freshness, and template attribution all join to one export-safe review-pack record, never a local parity estimate that reads as provider-authoritative mergeability",
                "the certified review pack keeps stable operation IDs while the pack version / digest, owner provenance, evaluator result class, local-versus-provider parity, and template attribution bind to the one review-pack evaluator matrix across review-detail / merge-readiness / ai-review-panel / provider-handoff / review-pack-summary / ownership-overlay / local-ci-parity-strip / support-export / help-docs surfaces, and no pack reads as provider-authoritative in one surface and local-estimate in another",
                "keyboard / screen-reader / high-zoom / high-contrast / localization reach preserved for the rendered review-pack record",
                "review-pack-truth: a fully-certified review-pack lane with export-safe, provider-current, internally consistent state is the only profile that certifies a certified review-pack record",
            ],
        ),
        seed_row(
            "cert:reviewable-review-pack-record-structure",
            P::ReviewableReviewPackRecordStructure,
            ReviewableReviewPackRecord,
            ReviewableReviewPackRecord,
            &[ReviewTemplatePacket],
            seed_all_certified(),
            None,
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "template_attribution",
            ],
            &[
                "record-structure class: an export-safe review-template / evidence record bound to one pack and inspectable rather than a per-surface description copied by hand, with the template attribution kept bound to the pack version / digest it came from",
                "the reviewable review-pack record keeps its pack version / digest, owner provenance, evaluator result class, and template attribution inspectable rather than a shell-chrome-only or mislabeled-provider-authoritative-row cue",
                "text / JSON / Markdown reconstruction certified so support can replay the reviewable review-pack record structure",
                "review-pack-truth: a reviewable review-pack record never certifies a fully-certified-lane claim and never stays green on a stale pack version / digest or a missing owner provenance",
            ],
        ),
        // --- Yellow: an axis is not current; the claim narrows visibly ------
        seed_row(
            "cert:stale-pack-version-digest-profile",
            P::StalePackVersionDigestProfile,
            ReviewableReviewPackRecord,
            PackVersionDigestUnverifiedProjection,
            &[ReviewPackRecord],
            seed_certified_except(
                Ax::DegradedState,
                seed_narrowed(
                    Ax::DegradedState,
                    "the pack's version / digest can no longer be confirmed fresh for this profile so a provider-current review-pack record cannot be certified and the pack stays inspect-only",
                    "The pack's version / digest can no longer be confirmed fresh — the base/head or pack revision the record was evaluated against has moved on — so the ReviewableReviewPackRecord claim narrows to a pack-version-digest-unverified projection and the lane discloses the last-known pack version / digest and marks the pack stale rather than letting a stale pack look current",
                    Trig::PackVersionOrDigestDropped,
                ),
            ),
            Some(seed_narrow(
                Ax::DegradedState,
                ReviewableReviewPackRecord,
                PackVersionDigestUnverifiedProjection,
                "The pack version / digest is stale for this record, so its last-known pack version / digest is disclosed and it never reads as a provider-current, freshly-evaluated pack",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "stale-pack class: the record names its scope selector, evaluator lineage, and last-known pack version / digest and marks the pack stale rather than letting a stale pack read as freshly evaluated when its version / digest is unconfirmed",
                "the stale-pack surface keeps its scope and last-known pack version / digest legible while the pack is disclosed as stale",
                "degraded-state: ReviewableReviewPackRecord narrows to a pack-version-digest-unverified projection (auto-narrowed)",
                "review-pack-truth: a stale pack never looks current — its pack version / digest is preserved and it never reads as provider-authoritative mergeability",
            ],
        ),
        seed_row(
            "cert:unverified-owner-provenance-profile",
            P::UnverifiedOwnerProvenanceProfile,
            ReviewableReviewPackRecord,
            OwnerProvenanceUnverifiedProjection,
            &[OwnershipSignal],
            seed_certified_except(
                Ax::ReviewPackTruth,
                seed_narrowed(
                    Ax::ReviewPackTruth,
                    "the scope slice's advisory-versus-enforced owner provenance can no longer be verified so a provider-current review-pack record cannot be certified and the owner stays inspect-only",
                    "The scope slice's owner provenance can no longer be verified — the advisory-versus-enforced owner source (CODEOWNERS repo rule, graph overlay, or provider metadata) is unresolved — so the ReviewableReviewPackRecord claim narrows to an owner-provenance-unverified projection and the lane keeps the last-known owner source explicit rather than flattening advisory-owner and enforced-owner into one owner pill",
                    Trig::OwnerProvenanceUnstated,
                ),
            ),
            Some(seed_narrow(
                Ax::ReviewPackTruth,
                ReviewableReviewPackRecord,
                OwnerProvenanceUnverifiedProjection,
                "The owner provenance is unverified, so the last-known advisory-versus-enforced owner source stays explicit and advisory-owner and enforced-owner never flatten into one owner pill",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "owner-provenance class: the ownership signal keeps its advisory-versus-enforced owner source and rationale explicit and marks the provenance unverified rather than presenting one owner pill when the source is unresolved",
                "the owner-provenance surface keeps its advisory-versus-enforced owner source legible while provenance is disclosed as unverified",
                "review-pack-truth: ReviewableReviewPackRecord narrows to an owner-provenance-unverified projection (auto-narrowed)",
                "review-pack-truth: advisory-owner and enforced-owner stay distinct — no owner pill flattens the provenance and the last-known owner source is preserved",
            ],
        ),
        seed_row(
            "cert:unevaluated-required-check-profile",
            P::UnevaluatedRequiredCheckProfile,
            ReviewableReviewPackRecord,
            EvidenceCheckUnverifiedProjection,
            &[RequiredEvidenceCheckRow],
            seed_certified_except(
                Ax::Visual,
                seed_narrowed(
                    Ax::Visual,
                    "the required check is ci-only, not-evaluated-here, or provider-unavailable for this profile so a provider-current review-pack record cannot be certified and the check stays inspect-only",
                    "The required check is ci-only, not-evaluated-here, or provider-unavailable — it could not be evaluated in this lane — so the ReviewableReviewPackRecord claim narrows to an evidence-check-unverified projection and the lane keeps the unevaluated evaluator result class explicit rather than hiding a ci-only / not-evaluated-here / provider-unavailable state behind a green summary",
                    Trig::UnevaluatedCheckHiddenBehindGreenSummary,
                ),
            ),
            Some(seed_narrow(
                Ax::Visual,
                ReviewableReviewPackRecord,
                EvidenceCheckUnverifiedProjection,
                "The required check is unevaluated in this lane, so its ci-only / not-evaluated-here / provider-unavailable state stays visible and is never hidden behind a green summary",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "required-check class: the required-evidence / required-check row keeps its evaluator result class and last-known state explicit and marks the check unevaluated rather than reading green when it is ci-only, not-evaluated-here, or provider-unavailable",
                "the required-check surface keeps its evaluator result class legible while the check is disclosed as unevaluated",
                "visual: ReviewableReviewPackRecord narrows to an evidence-check-unverified projection (auto-narrowed)",
                "review-pack-truth: a ci-only / not-evaluated-here / provider-unavailable check never hides behind a green summary — its unevaluated state stays explicit",
            ],
        ),
        seed_row(
            "cert:local-only-parity-profile",
            P::LocalOnlyParityProfile,
            ReviewableReviewPackRecord,
            LocalParityUnverifiedProjection,
            &[LocalCiParityStrip],
            seed_certified_except(
                Ax::HighZoomReflow,
                seed_narrowed(
                    Ax::HighZoomReflow,
                    "the check's result is a local parity estimate that diverges from provider-authoritative state so a provider-current review-pack record cannot be certified and the result stays a labelled estimate",
                    "The check's result is a local parity estimate — a capability difference (environment, secrets, runner class, service deps, branch protections, or a provider-only merge simulation) means it diverges from provider-authoritative state — so the ReviewableReviewPackRecord claim narrows to a local-parity-unverified projection and the lane keeps the estimate labelled rather than letting a local parity estimate masquerade as provider-authoritative mergeability",
                    Trig::LocalEstimateShownAsProviderAuthoritative,
                ),
            ),
            Some(seed_narrow(
                Ax::HighZoomReflow,
                ReviewableReviewPackRecord,
                LocalParityUnverifiedProjection,
                "The result is a local parity estimate, so it stays labelled as a local estimate with its capability difference explicit and never reads as provider-authoritative mergeability",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "local-parity class: the local-CI parity strip keeps its local-parity-estimate-versus-provider-authoritative state and capability difference explicit and marks the result a local estimate rather than provider-authoritative when the two diverge",
                "the local-parity surface keeps its estimate label and capability difference legible while the result is disclosed as a local estimate",
                "high-zoom-reflow: ReviewableReviewPackRecord narrows to a local-parity-unverified projection (auto-narrowed)",
                "review-pack-truth: a local parity estimate never masquerades as provider-authoritative — the estimate stays labelled and its capability difference is preserved",
            ],
        ),
        seed_row(
            "cert:undisclosed-ai-pack-binding-profile",
            P::UndisclosedAiPackBindingProfile,
            ReviewableReviewPackRecord,
            AiPackBindingUnverifiedProjection,
            &[AiPolicyHook],
            seed_certified_except(
                Ax::Localization,
                seed_narrowed(
                    Ax::Localization,
                    "the AI review ran under a pack version / digest that is not disclosed against the active pack so a provider-current review-pack record cannot be certified and the binding stays inspect-only",
                    "The AI review ran under a pack version / digest that is not disclosed against the active pack — the AI-policy-hook binding cannot confirm it ran under the declared pack revision — so the ReviewableReviewPackRecord claim narrows to an AI-pack-binding-unverified projection and the lane discloses the pack version the AI review ran under rather than letting AI review run under a different pack version without disclosure",
                    Trig::AiReviewRanUnderUndisclosedPackVersion,
                ),
            ),
            Some(seed_narrow(
                Ax::Localization,
                ReviewableReviewPackRecord,
                AiPackBindingUnverifiedProjection,
                "The AI review's pack binding is undisclosed, so the pack version / digest the AI review ran under stays explicit and it never runs under a different pack version without disclosure",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "ai-pack-binding class: the AI review policy hook keeps its disclosed pack version / digest and pack-driven policy explicit and marks the binding unverified rather than running an AI review under a different pack version without disclosure",
                "the ai-pack-binding surface keeps its disclosed pack version / digest legible while the binding is disclosed as unverified",
                "localization: ReviewableReviewPackRecord narrows to an AI-pack-binding-unverified projection (auto-narrowed)",
                "review-pack-truth: an AI review never runs under a different pack version without disclosure — the pack version / digest it ran under stays explicit",
            ],
        ),
        seed_row(
            "cert:stale-template-attribution-profile",
            P::StaleTemplateAttributionProfile,
            ReviewableReviewPackRecord,
            TemplateAttributionUnverifiedProjection,
            &[ReviewTemplatePacket],
            seed_certified_except(
                Ax::ScreenReader,
                seed_narrowed(
                    Ax::ScreenReader,
                    "the template's attribution can no longer be verified against the pack it came from so a provider-current review-pack record cannot be certified and the template stays inspect-only",
                    "The template's attribution can no longer be verified against the pack it came from — the comment / summary template's version / digest or pack binding is unresolved after export, publish, or reopen — so the ReviewableReviewPackRecord claim narrows to a template-attribution-unverified projection and the lane keeps the last-known template version / digest and pack binding explicit rather than losing the review-pack version / digest or template attribution",
                    Trig::TemplateAttributionDropped,
                ),
            ),
            Some(seed_narrow(
                Ax::ScreenReader,
                ReviewableReviewPackRecord,
                TemplateAttributionUnverifiedProjection,
                "The template attribution is unverified, so the last-known template version / digest and pack binding stay explicit and the template never loses its attribution on export, publish, or reopen",
            )),
            &[
                "profile",
                "claimed_claim",
                "certified_claim",
                "status",
                "binding_axis",
            ],
            &[
                "template-attribution class: the review-template packet keeps its template version / digest and pack binding explicit and marks the attribution unverified rather than presenting a template without the pack it came from",
                "the template-attribution surface keeps its last-known template version / digest and pack binding legible non-visually while attribution is disclosed as unverified",
                "screen-reader: ReviewableReviewPackRecord narrows to a template-attribution-unverified projection (auto-narrowed)",
                "review-pack-truth: a template never loses its attribution — its template version / digest and pack binding stay explicit and the review-pack version / digest survives export, publish, and reopen",
            ],
        ),
    ]
}
