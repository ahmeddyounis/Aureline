//! Keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity, and honest automatic claim
//! narrowing for the M5 review-pack record / ownership signal / required-evidence-check row / local-CI parity
//! strip / AI policy hook / review-template packet objects.
//!
//! This module is the M05-1282 accessibility-and-auto-narrowing capstone over the frozen M5 review-pack
//! evaluator matrix ([`crate::m5_review_pack_evaluator_matrix`]). Where the freeze matrix defines the
//! reusable review-pack record, ownership signal, required-evidence-check row, local-CI parity strip, AI
//! policy hook, and review-template packet objects, and the 1275-1280 implementation lanes resolve their
//! per-surface truth, this lane certifies — per object class — that review-pack claims stay
//! **keyboard-complete, assistive-tech-reachable, high-zoom / high-contrast-safe, CLI/export-safe, and
//! self-narrowing** rather than presenting a stale pack version / digest, a missing owner provenance, an
//! unevaluated required check, a local-versus-provider parity capability difference, an AI review under an
//! undisclosed pack version, or a stale template attribution as still a fully provider-aware, publish-safe
//! review-pack surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / CLI reach.** Every object exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, high-contrast-safe, and
//!   CLI/headless-reachable path into the same object identity, pack version / digest, owner provenance,
//!   evaluator result class, local-versus-provider parity state, pack freshness, and template attribution the
//!   rich object shows — never a color-only parity badge, a hover-only owner pill, or a pointer-only pack
//!   affordance that strands assistive-tech or headless-CLI users. Structure-heavy objects (the required
//!   evidence / check set, the review-template packet's rationale blocks / checklist / bundle manifest)
//!   additionally bind their structured layout to a flat list / textual path.
//! - **Export parity.** The support / CLI / release export reconstructs each object's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same pack version / digest, owner
//!   provenance, parity state, and template attribution labels visible in-product so support, help, and
//!   release proof can reconstruct exactly what the user was shown without leaking a raw diff hunk, message
//!   payload, secret, endpoint, or provider token.
//! - **Honest auto-narrowing.** When a pack version / digest is stale, owner provenance is missing, a
//!   required check is unevaluated (ci-only / not-evaluated-here / provider-unavailable), a local parity
//!   estimate diverges from provider-authoritative state, an AI review runs under an undisclosed pack
//!   version, or a template attribution is stale, the object's claim auto-narrows from
//!   `trusted_review_surface` / `reviewable_review_surface` to a pack-version-unverified /
//!   owner-provenance-unverified / evidence-check-unverified / local-parity-unverified /
//!   ai-pack-version-unverified / template-attribution-unverified projection, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical object identity / last-known state.
//!   The underlying pack, ownership, evidence, parity, AI-policy, and template truth is never dropped
//!   opaquely. An object with every dimension intact must NOT carry a spurious narrowing, and a stale-pack /
//!   missing-owner / unevaluated-check / parity-diverged / undisclosed-AI-pack / stale-template state can
//!   never keep a fully provider-aware, publish-safe claim — a local parity estimate never masquerades as
//!   provider-authoritative, and advisory-owner and enforced-owner are never flattened into one pill.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the review detail, merge-readiness,
//!   AI review panel, provider handoff, review-pack summary, ownership overlay, local-CI parity strip,
//!   support / export packet, and help / docs so product, help, and release publication stay aligned on
//!   downgrade behavior rather than drifting in copy — a trusted-looking object can never outrun the pack
//!   version / digest, owner provenance, parity, or template attribution evidence it is being viewed away
//!   from.
//!
//! Each [`ReviewPackAccessibilityRow`] keys on one
//! [`crate::m5_review_pack_evaluator_matrix::M5ReviewPackObject`] and reuses that frozen object vocabulary
//! plus the frozen [`M5ReviewPackRequiredLabel`], [`M5ReviewPackDowngradeTrigger`], and shared
//! [`M5ReviewPackConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling review-pack packets.
//!
//! The packet is metadata-only: raw diff hunks, message payloads, credentials, secrets, and endpoint refs
//! never cross this boundary; the packet carries only typed class tokens, opaque object refs, booleans, and
//! controlled labels so support, release, and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen review-pack vocabulary — the capstone certifies the freeze matrix's objects, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_review_pack_evaluator_matrix::{
    M5ReviewPackConsumerSurface, M5ReviewPackDowngradeTrigger, M5ReviewPackObject,
    M5ReviewPackRequiredLabel, M5_REVIEW_PACK_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1282 review-pack accessibility parity packet.
pub const REVIEW_PACK_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ReviewPackAccessibilityPacket`].
pub const REVIEW_PACK_A11Y_RECORD_KIND: &str = "m5_review_pack_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`ReviewPackAccessibilityRow`].
pub const REVIEW_PACK_A11Y_ROW_RECORD_KIND: &str = "m5_review_pack_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const REVIEW_PACK_A11Y_SCHEMA_REF: &str =
    "schemas/review/m5-review-pack-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const REVIEW_PACK_A11Y_DOC_REF: &str = "docs/review/m5_review_pack_accessibility_parity.md";

/// Repo-relative path of the frozen review-pack evaluator matrix this lane certifies.
pub const REVIEW_PACK_A11Y_MATRIX_REF: &str = M5_REVIEW_PACK_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const REVIEW_PACK_A11Y_FIXTURE_DIR: &str =
    "fixtures/review/m5-review-pack-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const REVIEW_PACK_A11Y_ARTIFACT_REF: &str =
    "artifacts/review/m5-review-pack-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const REVIEW_PACK_A11Y_CSV_REF: &str =
    "artifacts/review/m5-review-pack-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const REVIEW_PACK_A11Y_REPORT_REF: &str =
    "artifacts/review/m5-review-pack-accessibility-parity.md";

/// The reusable objects that render a dense, structured surface (the required evidence / check set, the
/// review-template packet's rationale blocks / checklist / bundle manifest) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable non-visually.
const fn object_is_structure_heavy(object: M5ReviewPackObject) -> bool {
    matches!(
        object,
        M5ReviewPackObject::RequiredEvidenceCheckRow | M5ReviewPackObject::ReviewTemplatePacket
    )
}

/// The review-pack-truth dimension whose weakening an object primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn object_primary_dimension(object: M5ReviewPackObject) -> M5ReviewPackClaimDimension {
    match object {
        M5ReviewPackObject::ReviewPackRecord => {
            M5ReviewPackClaimDimension::PackVersionDigestClarity
        }
        M5ReviewPackObject::OwnershipSignal => M5ReviewPackClaimDimension::OwnerProvenanceClarity,
        M5ReviewPackObject::RequiredEvidenceCheckRow => {
            M5ReviewPackClaimDimension::EvidenceCheckStateClarity
        }
        M5ReviewPackObject::LocalCiParityStrip => {
            M5ReviewPackClaimDimension::LocalProviderParityClarity
        }
        M5ReviewPackObject::AiPolicyHook => M5ReviewPackClaimDimension::AiPackBindingClarity,
        M5ReviewPackObject::ReviewTemplatePacket => {
            M5ReviewPackClaimDimension::TemplateAttributionClarity
        }
    }
}

/// A rendered fallback modality for an AI-review-assist object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackFallbackModality {
    /// A rich, structured (outbound action set / lifecycle history) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ReviewPackFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same object may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ReviewPackRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / CLI reach for an object's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl ReviewPackNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the object meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackExportSummaryState {
    /// The object meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl ReviewPackExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl ReviewPackNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The review-pack claim ceiling an object asserts: how strong a provider-aware, publish-safe posture it lets
/// a surface present. Auto-narrowing lowers this ceiling when a pack-version / owner-provenance /
/// evidence-check / local-parity / ai-pack-binding / template-attribution dimension weakens so a stale pack
/// version / digest, a missing owner provenance, an unevaluated check, a divergent parity estimate, an
/// undisclosed AI pack version, or a stale template attribution can never keep an old `TrustedReviewSurface`
/// or `ReviewableReviewSurface` label — a local parity estimate never masquerades as provider-authoritative
/// from a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackA11yClaim {
    /// Trusted review surface: a fully pack-versioned, owner-provenanced, evidence-evaluated,
    /// parity-disclosed, pack-bound, template-attributed object — the strongest claim, a review-pack surface
    /// Aureline can present as exactly provider-aware and publish-safe to inspect, rerun, compare, export, or
    /// reopen right now.
    TrustedReviewSurface,
    /// Reviewable review surface: a self-sufficient, reviewable read-only object (a review-template packet a
    /// user can inspect) that is not itself an authoritative, mergeability-driving surface.
    ReviewableReviewSurface,
    /// Pack-version-unverified projection: the pack version / digest is stale; the object stays a
    /// pack-version-unverified projection with its last-known pack identity preserved, never a stale pack
    /// version / digest shown as current, provider-authoritative truth.
    PackVersionUnverifiedProjection,
    /// Owner-provenance-unverified projection: the advisory-versus-enforced owner provenance is missing; the
    /// object stays an owner-provenance-unverified projection that keeps advisory-owner and enforced-owner
    /// distinct, never flattening them into one owner pill.
    OwnerProvenanceUnverifiedProjection,
    /// Evidence-check-unverified projection: a required check is unevaluated here (ci-only /
    /// not-evaluated-here / provider-unavailable); the object stays an evidence-check-unverified projection
    /// that keeps the evaluation state explicit, never folding an unevaluated check into a green summary.
    EvidenceCheckUnverifiedProjection,
    /// Local-parity-unverified projection: a local parity estimate diverges from provider-authoritative
    /// state; the object stays a local-parity-unverified projection that names the capability difference,
    /// never widening a local estimate into provider-authoritative mergeability.
    LocalParityUnverifiedProjection,
    /// AI-pack-version-unverified projection: the AI review ran under an undisclosed or different pack
    /// version; the object stays an ai-pack-version-unverified projection that discloses the pack binding,
    /// never presenting an AI review under a different pack version as pack-compliant.
    AiPackVersionUnverifiedProjection,
    /// Template-attribution-unverified projection: the comment / summary template attribution is stale; the
    /// object stays a template-attribution-unverified projection that keeps the template source / version
    /// visible, never dropping template attribution on export or reopen.
    TemplateAttributionUnverifiedProjection,
}

impl M5ReviewPackA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedReviewSurface,
        Self::ReviewableReviewSurface,
        Self::PackVersionUnverifiedProjection,
        Self::OwnerProvenanceUnverifiedProjection,
        Self::EvidenceCheckUnverifiedProjection,
        Self::LocalParityUnverifiedProjection,
        Self::AiPackVersionUnverifiedProjection,
        Self::TemplateAttributionUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedReviewSurface => 7,
            Self::ReviewableReviewSurface => 6,
            Self::PackVersionUnverifiedProjection => 5,
            Self::OwnerProvenanceUnverifiedProjection => 4,
            Self::EvidenceCheckUnverifiedProjection => 3,
            Self::LocalParityUnverifiedProjection => 2,
            Self::AiPackVersionUnverifiedProjection => 1,
            Self::TemplateAttributionUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully provider-aware, publish-safe review surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedReviewSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedReviewSurface | Self::ReviewableReviewSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedReviewSurface => "trusted_review_surface",
            Self::ReviewableReviewSurface => "reviewable_review_surface",
            Self::PackVersionUnverifiedProjection => "pack_version_unverified_projection",
            Self::OwnerProvenanceUnverifiedProjection => "owner_provenance_unverified_projection",
            Self::EvidenceCheckUnverifiedProjection => "evidence_check_unverified_projection",
            Self::LocalParityUnverifiedProjection => "local_parity_unverified_projection",
            Self::AiPackVersionUnverifiedProjection => "ai_pack_version_unverified_projection",
            Self::TemplateAttributionUnverifiedProjection => {
                "template_attribution_unverified_projection"
            }
        }
    }
}

/// The pack-version / owner-provenance / evidence-check / local-parity / ai-pack-binding /
/// template-attribution dimension whose state governs how far an object may claim to be a fully
/// provider-aware, publish-safe review surface. The dimensions map to the six frozen review-pack objects so
/// every object carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackClaimDimension {
    /// Pack-version / digest clarity: does the review-pack record keep its version / digest current so a
    /// stale pack never reads as current, provider-authoritative truth (review-pack-record)?
    PackVersionDigestClarity,
    /// Owner-provenance clarity: does the ownership signal keep its advisory-versus-enforced owner provenance
    /// explicit rather than flattening it into one owner pill (ownership-signal)?
    OwnerProvenanceClarity,
    /// Evidence-check-state clarity: does the required-evidence-check row keep its evaluation state (ci-only /
    /// not-evaluated-here / provider-unavailable) explicit rather than folding it into a green summary
    /// (required-evidence-check-row)?
    EvidenceCheckStateClarity,
    /// Local-versus-provider parity clarity: does the local-CI parity strip keep its local estimate distinct
    /// from provider-authoritative state and name the capability difference rather than widening the estimate
    /// (local-ci-parity-strip)?
    LocalProviderParityClarity,
    /// AI-pack-binding clarity: does the AI policy hook keep its AI review bound to a disclosed pack version /
    /// digest rather than running under a different or undisclosed pack version (ai-policy-hook)?
    AiPackBindingClarity,
    /// Template-attribution clarity: does the review-template packet keep its comment / summary template
    /// attribution bound to the pack rather than dropping it on export or reopen (review-template-packet)?
    TemplateAttributionClarity,
}

impl M5ReviewPackClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PackVersionDigestClarity,
        Self::OwnerProvenanceClarity,
        Self::EvidenceCheckStateClarity,
        Self::LocalProviderParityClarity,
        Self::AiPackBindingClarity,
        Self::TemplateAttributionClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackVersionDigestClarity => "pack_version_digest_clarity",
            Self::OwnerProvenanceClarity => "owner_provenance_clarity",
            Self::EvidenceCheckStateClarity => "evidence_check_state_clarity",
            Self::LocalProviderParityClarity => "local_provider_parity_clarity",
            Self::AiPackBindingClarity => "ai_pack_binding_clarity",
            Self::TemplateAttributionClarity => "template_attribution_clarity",
        }
    }
}

/// The observed condition of one review-pack-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the object's claim. The stale / missing / unevaluated / diverged states
/// the lane must auto-narrow on — a stale pack version / digest, a missing owner provenance, an unevaluated
/// required check, a local-versus-provider parity capability difference, an AI review under an undisclosed
/// pack version, and a stale template attribution — are the states that [`Self::cannot_be_shown_trusted`]
/// flags: each is a genuine truth degradation that can never be shown as a fully provider-aware, publish-safe
/// review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReviewPackConditionState {
    /// Fully pack-versioned, owner-provenanced, evidence-evaluated, parity-disclosed, pack-bound,
    /// template-attributed — imposes no ceiling.
    FullyQualified,
    /// The pack version / digest is stale — claim drops to a pack-version-unverified projection.
    PackVersionDigestStale,
    /// The advisory-versus-enforced owner provenance is missing — claim drops to an
    /// owner-provenance-unverified projection.
    OwnerProvenanceMissing,
    /// A required check is unevaluated here (ci-only / not-evaluated-here / provider-unavailable) — claim
    /// drops to an evidence-check-unverified projection.
    EvidenceCheckUnevaluated,
    /// A local parity estimate diverges from provider-authoritative state — claim drops to a
    /// local-parity-unverified projection.
    LocalParityCapabilityDifference,
    /// The AI review ran under an undisclosed or different pack version — claim drops to an
    /// ai-pack-version-unverified projection.
    AiPackVersionUndisclosed,
    /// The comment / summary template attribution is stale — claim drops to a
    /// template-attribution-unverified projection.
    TemplateAttributionStale,
}

impl M5ReviewPackConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::PackVersionDigestStale,
        Self::OwnerProvenanceMissing,
        Self::EvidenceCheckUnevaluated,
        Self::LocalParityCapabilityDifference,
        Self::AiPackVersionUndisclosed,
        Self::TemplateAttributionStale,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects a weakened state that cannot be shown as a fully
    /// provider-aware, publish-safe review surface and must never be shown as such. Every weak review-pack
    /// condition is a genuine truth degradation, so all six flag here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::PackVersionDigestStale
                | Self::OwnerProvenanceMissing
                | Self::EvidenceCheckUnevaluated
                | Self::LocalParityCapabilityDifference
                | Self::AiPackVersionUndisclosed
                | Self::TemplateAttributionStale
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ReviewPackA11yClaim {
        match self {
            Self::FullyQualified => M5ReviewPackA11yClaim::TrustedReviewSurface,
            Self::PackVersionDigestStale => M5ReviewPackA11yClaim::PackVersionUnverifiedProjection,
            Self::OwnerProvenanceMissing => {
                M5ReviewPackA11yClaim::OwnerProvenanceUnverifiedProjection
            }
            Self::EvidenceCheckUnevaluated => {
                M5ReviewPackA11yClaim::EvidenceCheckUnverifiedProjection
            }
            Self::LocalParityCapabilityDifference => {
                M5ReviewPackA11yClaim::LocalParityUnverifiedProjection
            }
            Self::AiPackVersionUndisclosed => {
                M5ReviewPackA11yClaim::AiPackVersionUnverifiedProjection
            }
            Self::TemplateAttributionStale => {
                M5ReviewPackA11yClaim::TemplateAttributionUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ReviewPackDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5ReviewPackDowngradeTrigger::ReviewPackMatrixStale,
            Self::PackVersionDigestStale => {
                M5ReviewPackDowngradeTrigger::PackVersionOrDigestDropped
            }
            Self::OwnerProvenanceMissing => M5ReviewPackDowngradeTrigger::OwnerProvenanceUnstated,
            Self::EvidenceCheckUnevaluated => {
                M5ReviewPackDowngradeTrigger::UnevaluatedCheckHiddenBehindGreenSummary
            }
            Self::LocalParityCapabilityDifference => {
                M5ReviewPackDowngradeTrigger::LocalEstimateShownAsProviderAuthoritative
            }
            Self::AiPackVersionUndisclosed => {
                M5ReviewPackDowngradeTrigger::AiReviewRanUnderUndisclosedPackVersion
            }
            Self::TemplateAttributionStale => {
                M5ReviewPackDowngradeTrigger::TemplateAttributionDropped
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::PackVersionDigestStale => "pack_version_digest_stale",
            Self::OwnerProvenanceMissing => "owner_provenance_missing",
            Self::EvidenceCheckUnevaluated => "evidence_check_unevaluated",
            Self::LocalParityCapabilityDifference => "local_parity_capability_difference",
            Self::AiPackVersionUndisclosed => "ai_pack_version_undisclosed",
            Self::TemplateAttributionStale => "template_attribution_stale",
        }
    }
}

/// One review-pack-truth dimension's observed condition on an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ReviewPackClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ReviewPackConditionState,
}

/// An honest claim auto-narrow block. When an AI-review-truth dimension weakens, the object's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// object identity / last-known state rather than silently dropping it — the underlying finding, scope,
/// publish, and lifecycle truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackClaimAutoNarrow {
    /// The claim the object is narrowed to.
    pub narrowed_to: M5ReviewPackA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5ReviewPackClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ReviewPackDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying finding / scope / publish / lifecycle truth is preserved (never dropped) across the
    /// narrowing; must hold so provider-freshness-unverified, diff-scope-unverified,
    /// publish-target-unverified, and finding-lifecycle-unverified states never fail opaquely, and no local
    /// draft or evidence is lost.
    pub preserves_truth_continuity: bool,
}

impl ReviewPackClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and finding / scope /
    /// publish / lifecycle truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an object's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl ReviewPackCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ReviewPackRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: ReviewPackNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an AI-review-assist accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewPackAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl ReviewPackAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one AI-review-assist object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackAccessibilityRow {
    /// Record kind; must equal [`REVIEW_PACK_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REVIEW_PACK_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen object this row certifies.
    pub object: M5ReviewPackObject,
    /// Ref to the frozen per-object domain schema this row certifies.
    pub source_object_schema_ref: String,
    /// Opaque ref to the object this row represents; stays visible on every surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a structure-heavy object must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ReviewPackFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, finding class / severity,
    /// analyzed scope, publish mode / provider destination, local-versus-provider state, and finding
    /// lifecycle state as the rich object; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ReviewPackNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ReviewPackNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: ReviewPackNonVisualReachState,
    /// High-contrast / forced-colors behavior of the non-visual path.
    pub high_contrast_reach: ReviewPackNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ReviewPackNonVisualReachState,
    /// Whether the export-safe summary preserves object meaning.
    pub export_summary: ReviewPackExportSummaryState,
    /// Ref to the export-safe summary object for this object.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ReviewPackCopyExportParity,
    /// The full claim this object asserts when every dimension is intact.
    pub full_ready_claim: M5ReviewPackA11yClaim,
    /// The observed condition of each modeled AI-review-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ReviewPackClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the object's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ReviewPackClaimAutoNarrow>,
    /// Whether the underlying finding / scope / publish / lifecycle truth is preserved on this object
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this object is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ReviewPackRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ReviewPackRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ReviewPackRequiredLabel>,
    /// Semantic consumer surfaces this object is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ReviewPackConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ReviewPackAccessibilityRow {
    /// Returns true when this object renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        object_is_structure_heavy(self.object)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5ReviewPackClaimDimension,
    ) -> M5ReviewPackConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ReviewPackConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// object's full claim.
    pub fn permitted_claim(&self) -> M5ReviewPackA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the object's full claim.
    pub fn binding_condition(&self) -> Option<&ReviewPackClaimConditionEntry> {
        let mut binding: Option<(&ReviewPackClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5ReviewPackClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this object effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ReviewPackA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-provider finding, a diff-drifted scope, an unavailable publish
    /// target, or an outdated / suppressed lifecycle state can no longer keep an old `TrustedReviewSurface` /
    /// `ReviewableReviewSurface` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and truth. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: a stale-pack / missing-owner / unevaluated-check / parity-diverged /
    /// undisclosed-AI-pack / stale-template state never keeps a trusted claim — a local parity estimate never
    /// masquerades as provider-authoritative from a narrowed object. When such a state is modeled, the
    /// effective claim must not assert `TrustedReviewSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / CLI trap, a structure-heavy object offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.object_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the object meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying finding / scope / publish /
    /// lifecycle truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the object carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its object's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = object_primary_dimension(self.object);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ReviewPackRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ReviewPackAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ReviewPackAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ReviewPackAccessibilityStatus::NarrowedDisclosed
        } else {
            ReviewPackAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == REVIEW_PACK_A11Y_ROW_RECORD_KIND
            && self.schema_version == REVIEW_PACK_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_object_schema_ref.trim().is_empty()
            && !self.object_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "object={object} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            object = self.object.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1272 AI-review-assist accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackAccessibilitySummary {
    pub row_count: usize,
    pub object_count: usize,
    pub structure_heavy_object_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ReviewPackAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ReviewPackAccessibilityRow>,
}

/// Checked-in M05-1272 AI-review-assist accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ReviewPackAccessibilityRow>,
    pub summary: ReviewPackAccessibilitySummary,
}

impl ReviewPackAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ReviewPackAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            record_kind: REVIEW_PACK_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ReviewPackAccessibilitySummary {
                row_count: 0,
                object_count: 0,
                structure_heavy_object_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Objects represented by some row in this packet.
    pub fn represented_objects(&self) -> BTreeSet<M5ReviewPackObject> {
        self.rows.iter().map(|r| r.object).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ReviewPackClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ReviewPackConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ReviewPackA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ReviewPackConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ReviewPackAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ReviewPackConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&ReviewPackAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ReviewPackAccessibilityStatus::Parity => green += 1,
                ReviewPackAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ReviewPackAccessibilityStatus::Stranded => red += 1,
            }
        }

        ReviewPackAccessibilitySummary {
            row_count: self.rows.len(),
            object_count: self.represented_objects().len(),
            structure_heavy_object_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ReviewPackAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ReviewPackAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != REVIEW_PACK_A11Y_SCHEMA_VERSION {
            violations.push(ReviewPackAccessibilityViolation::SchemaVersion {
                expected: REVIEW_PACK_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != REVIEW_PACK_A11Y_RECORD_KIND {
            violations.push(ReviewPackAccessibilityViolation::RecordKind {
                expected: REVIEW_PACK_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ReviewPackAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_objects = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ReviewPackAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_objects.insert(row.object);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(ReviewPackAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its object's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(ReviewPackAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: object_primary_dimension(row.object),
                });
            }

            // Each row must preserve every mandatory object label.
            if !row.preserves_mandatory_labels() {
                violations.push(ReviewPackAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy object must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ReviewPackFallbackModality::Structured)
            {
                violations.push(
                    ReviewPackAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(ReviewPackAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-provider / diff-drifted / publish-target-unavailable /
            // lifecycle-degraded state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(ReviewPackAccessibilityViolation::WeakStateShownAsTrusted {
                    id: row.row_id.clone(),
                });
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(ReviewPackAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(ReviewPackAccessibilityViolation::ExportRequiresRawPayload {
                    id: row.row_id.clone(),
                });
            }

            // AC / no-loss: weakened states preserve finding / scope / publish / lifecycle truth.
            if !row.preserves_truth_continuity() {
                violations.push(ReviewPackAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ReviewPackAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ReviewPackAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ReviewPackAccessibilityStatus::Stranded {
                violations.push(ReviewPackAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen object is certified at least once.
        for object in M5ReviewPackObject::ALL {
            if !seen_objects.contains(&object) {
                violations.push(ReviewPackAccessibilityViolation::MissingObjectCoverage { object });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ReviewPackClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(ReviewPackAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ReviewPackConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ReviewPackAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → finding-lifecycle-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ReviewPackA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(ReviewPackAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one stale-provider / diff-drifted /
        // publish-target-unavailable / lifecycle-degraded row in the packet, so the "cannot-prove never
        // shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(ReviewPackAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the review detail, AI panel, finding row, scope
        // selector, publish sheet, pending-review tray, provider publish review, resolution memory ledger,
        // and support / export packet — so every consumer surface is exercised at least once.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ReviewPackConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ReviewPackAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ReviewPackAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("review-pack accessibility parity packet serializes"),
        ) {
            violations.push(ReviewPackAccessibilityViolation::RawObjectMaterialInExport);
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
            .expect("review-pack accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,object,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{object},{keyboard},{screen_reader},{high_zoom},{high_contrast},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                object = row.object.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Review-Pack Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Objects: {} certified across {} / {} frozen objects\n",
            self.summary.object_count,
            self.represented_objects().len(),
            M5ReviewPackObject::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.object.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in AI-review-assist accessibility parity export.
pub fn current_m5_review_pack_accessibility_parity_export(
) -> Result<ReviewPackAccessibilityPacket, ReviewPackAccessibilityArtifactError> {
    let packet: ReviewPackAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-review-pack-accessibility-parity/support_export.json"
    )))
    .map_err(ReviewPackAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ReviewPackAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in AI-review-assist accessibility parity export.
#[derive(Debug)]
pub enum ReviewPackAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ReviewPackAccessibilityViolation>),
}

impl fmt::Display for ReviewPackAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "review-pack accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "review-pack accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ReviewPackAccessibilityArtifactError {}

/// Validation failure for M05-1272 AI-review-assist accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5ReviewPackClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingObjectCoverage {
        object: M5ReviewPackObject,
    },
    MissingDimensionCoverage {
        dimension: M5ReviewPackClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ReviewPackConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ReviewPackA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ReviewPackConsumerSurface,
    },
    SummaryMismatch,
    RawObjectMaterialInExport,
}

impl ReviewPackAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingObjectCoverage { .. } => "missing_object_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawObjectMaterialInExport => "raw_object_material_in_export",
        }
    }
}

impl fmt::Display for ReviewPackAccessibilityViolation {
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
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its object's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory object label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows a stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded state as a trusted review surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve finding / scope / publish / lifecycle truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingObjectCoverage { object } => {
                write!(f, "object {object:?} is not certified in the packet")
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawObjectMaterialInExport => {
                write!(f, "export contains raw object material")
            }
        }
    }
}

impl Error for ReviewPackAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
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
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const REVIEW_PACK_A11Y_PACKET_ID: &str = "m5-review-pack-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in review-pack accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_review_pack_accessibility_parity_packet() -> ReviewPackAccessibilityPacket {
    ReviewPackAccessibilityPacket::new(ReviewPackAccessibilityPacketInput {
        packet_id: REVIEW_PACK_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: REVIEW_PACK_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:review-pack-accessibility-parity:{id}")]
}

fn all_required_labels() -> Vec<M5ReviewPackRequiredLabel> {
    M5ReviewPackRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ReviewPackCopyExportParity {
    ReviewPackCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ReviewPackClaimDimension,
    state: M5ReviewPackConditionState,
) -> ReviewPackClaimConditionEntry {
    ReviewPackClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the support / export packet and the review
/// detail surface — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ReviewPackConsumerSurface]) -> Vec<M5ReviewPackConsumerSurface> {
    let mut out = vec![
        M5ReviewPackConsumerSurface::SupportExportPacket,
        M5ReviewPackConsumerSurface::ReviewDetail,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: ReviewPackNarrowingDisclosureState,
) -> Vec<ReviewPackRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ReviewPackRenderingNarrowingDisclosure {
            rendering_surface: M5ReviewPackRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        ReviewPackRenderingNarrowingDisclosure {
            rendering_surface: M5ReviewPackRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_publish_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ReviewPackRenderingNarrowingDisclosure> {
    surface_disclosures(labels, ReviewPackNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ReviewPackRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ReviewPackNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ReviewPackRenderingSurface> {
    vec![
        M5ReviewPackRenderingSurface::DesktopFull,
        M5ReviewPackRenderingSurface::CliHeadless,
        M5ReviewPackRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ReviewPackFallbackModality> {
    vec![
        M5ReviewPackFallbackModality::List,
        M5ReviewPackFallbackModality::Textual,
        M5ReviewPackFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5ReviewPackFallbackModality> {
    vec![
        M5ReviewPackFallbackModality::Structured,
        M5ReviewPackFallbackModality::List,
        M5ReviewPackFallbackModality::Textual,
        M5ReviewPackFallbackModality::Cli,
    ]
}

const REACHABLE: ReviewPackNonVisualReachState = ReviewPackNonVisualReachState::ReachableAndLabeled;
const REDUCED: ReviewPackNonVisualReachState =
    ReviewPackNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<ReviewPackAccessibilityRow> {
    vec![
        // Review-pack record (fresh pack version / digest) — the record keeps its pack version / digest,
        // scope selectors, and evaluator identity current, so it is a fully provider-aware, publish-safe
        // review surface reachable on every surface with no narrowing (green). Keyboard-only and
        // screen-reader users can inspect, rerun, compare, export, and reopen it without losing pack identity
        // or parity truth.
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-pack-record-fresh-pack-version".to_owned(),
            object: M5ReviewPackObject::ReviewPackRecord,
            source_object_schema_ref: M5ReviewPackObject::ReviewPackRecord
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:review-pack-record:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-pack-record-fresh-pack-version:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "pack_version_and_digest",
                "evaluator_result_class",
                "local_versus_provider_parity",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::PackVersionDigestClarity,
                M5ReviewPackConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "pack_version_and_digest",
                "evaluator_result_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::ReviewPackSummary,
                M5ReviewPackConsumerSurface::MergeReadiness,
            ]),
            source_refs: vec![
                "TAD v1.25 §16.11.3 — review packs".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("review-pack-record-fresh-pack-version"),
        },
        // Review-template packet (attribution bound) — structure-heavy (rationale blocks / checklist /
        // bundle manifest); it keeps its comment / summary template attribution bound to the pack, so it is a
        // self-sufficient reviewable review surface a user can inspect, with full parity on every surface
        // (green). Its structured template binds to a flat list / textual path.
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-template-packet-attribution-bound".to_owned(),
            object: M5ReviewPackObject::ReviewTemplatePacket,
            source_object_schema_ref: M5ReviewPackObject::ReviewTemplatePacket
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:review-template-packet:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-template-packet-attribution-bound:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "template_attribution",
                "pack_version_and_digest",
                "template_source_and_version",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::ReviewableReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::TemplateAttributionClarity,
                M5ReviewPackConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "template_attribution",
                "pack_version_and_digest",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::ReviewPackSummary,
                M5ReviewPackConsumerSurface::HelpDocs,
            ]),
            source_refs: vec![
                "TAD v1.25 §16.11.3 — comment / summary templates".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("review-template-packet-attribution-bound"),
        },
        // Review-pack record (pack version / digest stale) — the pack version / digest is stale, so it
        // auto-narrows to a pack-version-unverified projection that keeps the last-known pack identity visible
        // without presenting a stale pack version / digest as current, provider-authoritative truth (yellow).
        // Its screen-reader traversal discloses a reduced linear walk.
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-pack-record-pack-version-stale".to_owned(),
            object: M5ReviewPackObject::ReviewPackRecord,
            source_object_schema_ref: M5ReviewPackObject::ReviewPackRecord
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:review-pack-record:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-pack-record-pack-version-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "last_known_pack_version_and_digest",
                "pack_freshness_state",
                "local_versus_provider_parity",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::PackVersionDigestClarity,
                M5ReviewPackConditionState::PackVersionDigestStale,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::PackVersionUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::PackVersionDigestClarity,
                trigger: M5ReviewPackDowngradeTrigger::PackVersionOrDigestDropped,
                narrowed_label:
                    "This review pack's version / digest is stale — shown as a pack-version-unverified projection that keeps the last-known pack version, digest, and scope explicit, never presenting a stale pack version / digest as current, provider-authoritative truth or dropping the digest on export"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "last_known_pack_version_and_digest",
                "pack_freshness_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::ReviewPackSummary,
                M5ReviewPackConsumerSurface::ProviderHandoff,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.6.3 — review-pack version / digest".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("review-pack-record-pack-version-stale"),
        },
        // Ownership signal (owner provenance missing) — the advisory-versus-enforced owner provenance is
        // missing, so it auto-narrows to an owner-provenance-unverified projection that keeps advisory-owner
        // and enforced-owner distinct, never flattening them into one owner pill (yellow).
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ownership-signal-owner-provenance-missing".to_owned(),
            object: M5ReviewPackObject::OwnershipSignal,
            source_object_schema_ref: M5ReviewPackObject::OwnershipSignal
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:ownership-signal:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ownership-signal-owner-provenance-missing:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "advisory_versus_enforced_owner",
                "owner_source_provenance",
                "pack_version_and_digest",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::OwnerProvenanceClarity,
                M5ReviewPackConditionState::OwnerProvenanceMissing,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::OwnerProvenanceUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::OwnerProvenanceClarity,
                trigger: M5ReviewPackDowngradeTrigger::OwnerProvenanceUnstated,
                narrowed_label:
                    "This ownership signal's advisory-versus-enforced owner provenance is missing — shown as an owner-provenance-unverified projection that keeps advisory-owner and enforced-owner mechanically distinct and names the unresolved source, never flattening them into one owner pill"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "advisory_versus_enforced_owner",
                "owner_source_provenance",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::OwnershipOverlay,
                M5ReviewPackConsumerSurface::MergeReadiness,
            ]),
            source_refs: vec![
                "TAD v1.25 §16.11.3 — advisory-owner versus enforced-owner".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("ownership-signal-owner-provenance-missing"),
        },
        // Required-evidence-check row (check unevaluated here) — structure-heavy (a required evidence / check
        // set); a required check is unevaluated here (ci-only / not-evaluated-here / provider-unavailable), so
        // it auto-narrows to an evidence-check-unverified projection that keeps the evaluation state explicit,
        // never folding an unevaluated check into a green summary (yellow). Its dense reflow narrows the
        // high-zoom legibility to a disclosed reduction.
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:required-evidence-check-row-check-unevaluated".to_owned(),
            object: M5ReviewPackObject::RequiredEvidenceCheckRow,
            source_object_schema_ref: M5ReviewPackObject::RequiredEvidenceCheckRow
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:required-evidence-check-row:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:required-evidence-check-row-check-unevaluated:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "evidence_check_state",
                "evaluator_result_class",
                "not_evaluated_here_reason",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::EvidenceCheckStateClarity,
                M5ReviewPackConditionState::EvidenceCheckUnevaluated,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::EvidenceCheckUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::EvidenceCheckStateClarity,
                trigger: M5ReviewPackDowngradeTrigger::UnevaluatedCheckHiddenBehindGreenSummary,
                narrowed_label:
                    "This required check is not evaluated here (ci-only / not-evaluated-here / provider-unavailable) — shown as an evidence-check-unverified projection that keeps the evaluation state explicit, never folding an unevaluated or provider-only check into a green summary"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "evidence_check_state",
                "evaluator_result_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::MergeReadiness,
                M5ReviewPackConsumerSurface::LocalCiParityStrip,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.6.6.3 — required evidence / checks".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("required-evidence-check-row-check-unevaluated"),
        },
        // Local-CI parity strip (local-versus-provider capability difference) — a local parity estimate
        // diverges from provider-authoritative state, so it auto-narrows to a local-parity-unverified
        // projection that names the capability difference (environment / secrets / runner class / service
        // dependencies / branch protections / provider-only merge simulation), never widening a local
        // estimate into provider-authoritative mergeability (yellow).
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:local-ci-parity-strip-capability-difference".to_owned(),
            object: M5ReviewPackObject::LocalCiParityStrip,
            source_object_schema_ref: M5ReviewPackObject::LocalCiParityStrip
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:local-ci-parity-strip:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:local-ci-parity-strip-capability-difference:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "local_versus_provider_parity",
                "capability_difference",
                "compare_action",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::LocalProviderParityClarity,
                M5ReviewPackConditionState::LocalParityCapabilityDifference,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::LocalParityUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::LocalProviderParityClarity,
                trigger: M5ReviewPackDowngradeTrigger::LocalEstimateShownAsProviderAuthoritative,
                narrowed_label:
                    "This parity strip's local estimate diverges from provider-authoritative state — shown as a local-parity-unverified projection that names the capability difference and keeps the compare action explicit, never widening a local estimate into provider-authoritative mergeability"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "local_versus_provider_parity",
                "capability_difference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::LocalCiParityStrip,
                M5ReviewPackConsumerSurface::ProviderHandoff,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §13.6 — provider-authoritative versus local parity estimate".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("local-ci-parity-strip-capability-difference"),
        },
        // AI policy hook (AI review under an undisclosed pack version) — the AI review ran under an
        // undisclosed or different pack version, so it auto-narrows to an ai-pack-version-unverified
        // projection that discloses the pack binding, never presenting an AI review under a different pack
        // version as pack-compliant evidence (yellow).
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ai-policy-hook-undisclosed-pack-version".to_owned(),
            object: M5ReviewPackObject::AiPolicyHook,
            source_object_schema_ref: M5ReviewPackObject::AiPolicyHook
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:ai-policy-hook:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REDUCED,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ai-policy-hook-undisclosed-pack-version:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "ai_pack_version_binding",
                "evaluator_result_class",
                "pack_version_and_digest",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::AiPackBindingClarity,
                M5ReviewPackConditionState::AiPackVersionUndisclosed,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::AiPackVersionUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::AiPackBindingClarity,
                trigger: M5ReviewPackDowngradeTrigger::AiReviewRanUnderUndisclosedPackVersion,
                narrowed_label:
                    "This AI review ran under an undisclosed or different pack version — shown as an ai-pack-version-unverified projection that discloses the pack version / digest the run was bound to, never presenting an AI review under a different pack version as current, pack-compliant evidence"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "ai_pack_version_binding",
                "evaluator_result_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::AiReviewPanel,
                M5ReviewPackConsumerSurface::ProviderHandoff,
            ]),
            source_refs: vec![
                "TAD v1.25 §16.11.3 — AI review policy hooks".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("ai-policy-hook-undisclosed-pack-version"),
        },
        // Review-template packet (template attribution stale) — structure-heavy (rationale blocks /
        // checklist / bundle manifest); the comment / summary template attribution is stale, so it
        // auto-narrows to a template-attribution-unverified projection that keeps the template source /
        // version visible, never dropping template attribution on export or reopen (yellow).
        ReviewPackAccessibilityRow {
            record_kind: REVIEW_PACK_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: REVIEW_PACK_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-template-packet-attribution-stale".to_owned(),
            object: M5ReviewPackObject::ReviewTemplatePacket,
            source_object_schema_ref: M5ReviewPackObject::ReviewTemplatePacket
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:review-template-packet:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ReviewPackExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-template-packet-attribution-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "template_attribution",
                "template_source_and_version",
                "last_known_pack_version_and_digest",
            ]),
            full_ready_claim: M5ReviewPackA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5ReviewPackClaimDimension::TemplateAttributionClarity,
                M5ReviewPackConditionState::TemplateAttributionStale,
            )],
            claim_narrow: Some(ReviewPackClaimAutoNarrow {
                narrowed_to: M5ReviewPackA11yClaim::TemplateAttributionUnverifiedProjection,
                binding_dimension: M5ReviewPackClaimDimension::TemplateAttributionClarity,
                trigger: M5ReviewPackDowngradeTrigger::TemplateAttributionDropped,
                narrowed_label:
                    "This review-template packet's comment / summary template attribution is stale — shown as a template-attribution-unverified projection that keeps the template source, version, and pack digest visible, never dropping template attribution or its pack version / digest on export or reopen"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "template_attribution",
                "template_source_and_version",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ReviewPackConsumerSurface::HelpDocs,
                M5ReviewPackConsumerSurface::AiReviewPanel,
            ]),
            source_refs: vec![
                "TAD v1.25 §16.11.3 — comment / summary template attribution".to_owned(),
                REVIEW_PACK_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("review-template-packet-attribution-stale"),
        },
    ]
}
