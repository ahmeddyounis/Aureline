//! One reusable M5 support-class / evidence-freshness badge primitive: the support
//! class a capability claims (Certified / Supported / Limited / Community /
//! Experimental) and the freshness of the evidence behind that claim (Fresh /
//! Retest-pending / Evidence-stale / Imported-evidence), projected the same way
//! across every claimed M5 onboarding, Help, marketplace, diagnostics, certification,
//! and evaluation surface — as two distinct, composable cues rather than one
//! overloaded badge.
//!
//! Aureline's frozen badge-family matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! names the support-class badge and the evidence-freshness badge as two governed
//! badge families and freezes the shared badge infrastructure — the surface families,
//! the deployment lines, the accessibility routes, the qualification classes, the
//! explanation-drawer fields, the consumer surfaces, and the downgrade triggers. This
//! module *implements* those two families as one render-facing badge pair so a user
//! can tell — from the two badges and their explanation drawers alone — exactly how
//! supported a thing is *and* how fresh the proof behind that support is, without one
//! badge implying the other.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_badge_claim`] — that takes one capability's subject
//!    label, its declared support class, its declared evidence freshness, its evidence
//!    source, and its last-evaluated timestamp, and produces one
//!    [`M5ResolvedBadgeClaim`] carrying both badges as separate typed fields, the
//!    derived effective claim (current / retest-pending / narrowed-by-stale-evidence /
//!    narrowed-by-imported-evidence), and — whenever imported or stale evidence
//!    reduces the claim — a self-contained [`M5ClaimNarrowingNote`] that names the
//!    exact reason, the next action, and the *preserved* support-class context. The
//!    resolver never collapses the two axes into one pill, never derives freshness
//!    from support class (a Certified thing may still carry stale evidence), never
//!    derives support class from freshness, and never drops the support-class context
//!    when it narrows a claim.
//! 2. A parity matrix — [`M5BadgeClaimPrimitivePacket`] — that binds one row per
//!    claimed M5 badge consumer (the onboarding checklist, the Help capability card,
//!    the marketplace listing, the diagnostics report, the certification record, and
//!    the evaluation pack) to the shared badge anatomy, the same support-class values,
//!    freshness values, effective-claim postures, narrowing reasons, next actions,
//!    explanation-drawer fields, export fields, and non-visual accessibility routes,
//!    so the support-class / evidence-freshness vocabulary stays identical across
//!    onboarding, Help, the marketplace, diagnostics, certification, and evaluation.
//!
//! The badge surface family ([`M5BadgeSurfaceFamily`]), deployment line
//! ([`M5DeploymentLine`]), accessibility route ([`M5BadgeAccessibilityRoute`]),
//! qualification class ([`M5BadgeQualificationClass`]), explanation-drawer field
//! ([`M5BadgeExplanationField`]), consumer surface ([`M5BadgeConsumerSurface`]), and
//! downgrade trigger ([`M5BadgeDowngradeTrigger`]) are reused verbatim from the frozen
//! badge-family matrix. This module mints new vocabulary only for what that matrix
//! left implicit about the two rendered badges themselves: their render-facing value
//! sets, their badge consumers, their badge-pair anatomy parts, their effective-claim
//! postures, their claim-narrowing reasons, their next actions, and their export
//! fields. No M5 badge surface invents a second support-class or freshness grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every subject label, evidence
//! source, and timestamp is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json`](../../../../schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json)
//! and the contract doc is
//! [`docs/release/m5_support_class_and_evidence_freshness_badge_contract.md`](../../../../docs/release/m5_support_class_and_evidence_freshness_badge_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-support-class-and-evidence-freshness-badges/`](../../../../fixtures/ui/m5-support-class-and-evidence-freshness-badges/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_badge_claim_primitive_certification_record_preview_narrowed,
    seeded_m5_badge_claim_primitive_marketplace_listing_beta_narrowed,
    seeded_m5_badge_claim_primitive_packet, M5_BADGE_CLAIM_PRIMITIVE_PACKET_ID,
};

// The surface families, deployment lines, accessibility routes, qualification classes,
// explanation-drawer fields, consumer surfaces, and downgrade triggers are frozen once,
// in the badge-family matrix. This primitive reuses them verbatim so it never invents a
// parallel badge grammar for the shared badge infrastructure.
pub use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeAccessibilityRoute, M5BadgeConsumerSurface, M5BadgeDowngradeTrigger,
    M5BadgeExplanationField, M5BadgeQualificationClass, M5BadgeSurfaceFamily, M5DeploymentLine,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BadgeClaimPrimitivePacket`].
pub const M5_BADGE_CLAIM_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces";

/// Schema version for M5 support-class / evidence-freshness badge records.
pub const M5_BADGE_CLAIM_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the support-class / evidence-freshness badge boundary schema.
pub const M5_BADGE_CLAIM_SCHEMA_REF: &str =
    "schemas/ui/m5-support-class-and-evidence-freshness-badge.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BADGE_CLAIM_DOC_REF: &str =
    "docs/release/m5_support_class_and_evidence_freshness_badge_contract.md";

/// Repo-relative path of the frozen badge-family matrix this primitive narrows from.
pub const M5_BADGE_CLAIM_FAMILY_MATRIX_REF: &str = "schemas/ui/m5-badge-family-matrix.schema.json";

/// Repo-relative path of the support-class ledger this primitive projects support
/// posture from.
pub const M5_BADGE_CLAIM_SUPPORT_CLASS_REF: &str =
    "schemas/release/support_class_ledger.schema.json";

/// Repo-relative path of the evidence-freshness descriptor this primitive projects
/// freshness from.
pub const M5_BADGE_CLAIM_FRESHNESS_REF: &str =
    "schemas/provenance/m5-freshness-descriptor.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BADGE_CLAIM_FIXTURE_DIR: &str =
    "fixtures/ui/m5-support-class-and-evidence-freshness-badges";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BADGE_CLAIM_ARTIFACT_REF: &str =
    "artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BADGE_CLAIM_CSV_REF: &str =
    "artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BADGE_CLAIM_REPORT_REF: &str =
    "artifacts/components/m5-support-class-and-evidence-freshness-badges.md";

/// One claimed M5 badge consumer that renders the shared support-class and
/// evidence-freshness badge pair. These are the surfaces the acceptance criteria name
/// — onboarding, Help, the marketplace, diagnostics, certification, and evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeClaimConsumerSurface {
    /// The onboarding checklist / first-run capability list.
    OnboardingChecklist,
    /// The Help / capability card.
    HelpCapabilityCard,
    /// The marketplace listing.
    MarketplaceListing,
    /// The diagnostics report.
    DiagnosticsReport,
    /// The certification record.
    CertificationRecord,
    /// The evaluation pack.
    EvaluationPack,
}

impl M5BadgeClaimConsumerSurface {
    /// Every claimed badge consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OnboardingChecklist,
        Self::HelpCapabilityCard,
        Self::MarketplaceListing,
        Self::DiagnosticsReport,
        Self::CertificationRecord,
        Self::EvaluationPack,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingChecklist => "onboarding_checklist",
            Self::HelpCapabilityCard => "help_capability_card",
            Self::MarketplaceListing => "marketplace_listing",
            Self::DiagnosticsReport => "diagnostics_report",
            Self::CertificationRecord => "certification_record",
            Self::EvaluationPack => "evaluation_pack",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnboardingChecklist => "Onboarding Checklist",
            Self::HelpCapabilityCard => "Help Capability Card",
            Self::MarketplaceListing => "Marketplace Listing",
            Self::DiagnosticsReport => "Diagnostics Report",
            Self::CertificationRecord => "Certification Record",
            Self::EvaluationPack => "Evaluation Pack",
        }
    }
}

/// Controlled support-class badge value — how supported a capability is. This is the
/// render-facing support-class vocabulary the acceptance criteria name: Certified,
/// Supported, Limited, Community, Experimental. A support-class badge never leaves its
/// posture implicit and never implies anything about evidence freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportClassBadgeValue {
    /// Certified: fully supported and independently certified.
    Certified,
    /// Supported: covered by an active support commitment.
    Supported,
    /// Limited: supported with a stated limitation or scope.
    Limited,
    /// Community: community-supported only.
    Community,
    /// Experimental: no support commitment; may change or be withdrawn.
    Experimental,
}

impl M5SupportClassBadgeValue {
    /// Every support-class value, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Certified,
        Self::Supported,
        Self::Limited,
        Self::Community,
        Self::Experimental,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Community => "community",
            Self::Experimental => "experimental",
        }
    }

    /// Review-safe label for the badge and narrowing note.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Certified => "Certified",
            Self::Supported => "Supported",
            Self::Limited => "Limited",
            Self::Community => "Community",
            Self::Experimental => "Experimental",
        }
    }
}

/// Controlled evidence-freshness badge value — how fresh the proof behind a claim is.
/// This is the render-facing freshness vocabulary the acceptance criteria name: Fresh,
/// Retest-pending, Evidence-stale, Imported-evidence. A freshness badge never presents
/// stale or imported evidence as fresh and never implies a support class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshnessValue {
    /// Fresh: the evidence is current within its freshness window.
    Fresh,
    /// Retest pending: a retest is scheduled but not yet complete.
    RetestPending,
    /// Evidence stale: the evidence is past its freshness window.
    EvidenceStale,
    /// Imported evidence: the evidence was imported from an external source and not
    /// re-verified locally.
    ImportedEvidence,
}

impl M5EvidenceFreshnessValue {
    /// Every evidence-freshness value, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Fresh,
        Self::RetestPending,
        Self::EvidenceStale,
        Self::ImportedEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::ImportedEvidence => "imported_evidence",
        }
    }

    /// Review-safe label for the badge.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Fresh => "Fresh",
            Self::RetestPending => "Retest pending",
            Self::EvidenceStale => "Evidence stale",
            Self::ImportedEvidence => "Imported evidence",
        }
    }
}

/// One anatomy part the shared support-class / evidence-freshness badge pair surfaces.
/// The parts in [`M5BadgeClaimAnatomyPart::MANDATORY`] are required on every consumer
/// so the two cues stay distinct and each opens its own explanation drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeClaimAnatomyPart {
    /// The support-class badge itself.
    SupportClassBadge,
    /// The evidence-freshness badge itself.
    EvidenceFreshnessBadge,
    /// The support-class explanation drawer.
    SupportClassExplanationDrawer,
    /// The evidence-freshness explanation drawer.
    FreshnessExplanationDrawer,
    /// The separately-filterable filter keys for both axes.
    FilterKeys,
    /// The derived effective-claim note.
    EffectiveClaimNote,
    /// The claim-narrowing banner (shown when the claim is narrowed).
    ClaimNarrowingBanner,
}

impl M5BadgeClaimAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::SupportClassBadge,
        Self::EvidenceFreshnessBadge,
        Self::SupportClassExplanationDrawer,
        Self::FreshnessExplanationDrawer,
        Self::FilterKeys,
        Self::EffectiveClaimNote,
        Self::ClaimNarrowingBanner,
    ];

    /// The anatomy parts every badge consumer must render: both badges as distinct
    /// cues, and both explanation drawers.
    pub const MANDATORY: [Self; 4] = [
        Self::SupportClassBadge,
        Self::EvidenceFreshnessBadge,
        Self::SupportClassExplanationDrawer,
        Self::FreshnessExplanationDrawer,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClassBadge => "support_class_badge",
            Self::EvidenceFreshnessBadge => "evidence_freshness_badge",
            Self::SupportClassExplanationDrawer => "support_class_explanation_drawer",
            Self::FreshnessExplanationDrawer => "freshness_explanation_drawer",
            Self::FilterKeys => "filter_keys",
            Self::EffectiveClaimNote => "effective_claim_note",
            Self::ClaimNarrowingBanner => "claim_narrowing_banner",
        }
    }
}

/// The derived effective claim — the resolver's verdict about how the evidence
/// freshness affects the *currency* of the support-class claim, computed from the
/// freshness axis alone so it never implies or overrides the support class itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EffectiveClaimPosture {
    /// The support-class claim is current: evidence is fresh.
    ClaimCurrent,
    /// The support-class claim is shown with a retest pending.
    ClaimRetestPending,
    /// The claim is narrowed because the evidence is stale.
    ClaimNarrowedEvidenceStale,
    /// The claim is narrowed because the evidence was imported and not re-verified.
    ClaimNarrowedImportedEvidence,
}

impl M5EffectiveClaimPosture {
    /// Every effective-claim posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ClaimCurrent,
        Self::ClaimRetestPending,
        Self::ClaimNarrowedEvidenceStale,
        Self::ClaimNarrowedImportedEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimCurrent => "claim_current",
            Self::ClaimRetestPending => "claim_retest_pending",
            Self::ClaimNarrowedEvidenceStale => "claim_narrowed_evidence_stale",
            Self::ClaimNarrowedImportedEvidence => "claim_narrowed_imported_evidence",
        }
    }

    /// True when the claim is current (evidence fresh).
    pub const fn is_current(self) -> bool {
        matches!(self, Self::ClaimCurrent)
    }

    /// True when the claim carries a pending retest but is not narrowed.
    pub const fn is_retest_pending(self) -> bool {
        matches!(self, Self::ClaimRetestPending)
    }

    /// True when imported or stale evidence has narrowed the claim.
    pub const fn is_narrowed(self) -> bool {
        matches!(
            self,
            Self::ClaimNarrowedEvidenceStale | Self::ClaimNarrowedImportedEvidence
        )
    }

    /// The reason the freshness axis reduced the claim, if any. Returns `None` for a
    /// current claim.
    pub const fn reduces_reason(self) -> Option<M5FreshnessReducesClaimReason> {
        Some(match self {
            Self::ClaimRetestPending => M5FreshnessReducesClaimReason::RetestPending,
            Self::ClaimNarrowedEvidenceStale => M5FreshnessReducesClaimReason::EvidenceStale,
            Self::ClaimNarrowedImportedEvidence => M5FreshnessReducesClaimReason::ImportedEvidence,
            Self::ClaimCurrent => return None,
        })
    }
}

/// The exact reason the evidence-freshness axis reduces a support-class claim, so a
/// claim-narrowing note never reads like a generic `claim unavailable`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FreshnessReducesClaimReason {
    /// A retest is pending; the claim is shown but its evidence is being refreshed.
    RetestPending,
    /// The evidence behind the claim is stale.
    EvidenceStale,
    /// The evidence behind the claim was imported and not re-verified locally.
    ImportedEvidence,
}

impl M5FreshnessReducesClaimReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RetestPending,
        Self::EvidenceStale,
        Self::ImportedEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetestPending => "retest_pending",
            Self::EvidenceStale => "evidence_stale",
            Self::ImportedEvidence => "imported_evidence",
        }
    }

    /// Review-safe reason phrase for the narrowing-note headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::RetestPending => "a retest is pending on the supporting evidence",
            Self::EvidenceStale => "the supporting evidence is stale",
            Self::ImportedEvidence => "the supporting evidence was imported and not re-verified",
        }
    }

    /// True when this reason narrows the claim (stale or imported), as opposed to
    /// merely flagging a pending retest.
    pub const fn narrows_claim(self) -> bool {
        matches!(self, Self::EvidenceStale | Self::ImportedEvidence)
    }

    /// The next action a reviewer should take to restore claim currency.
    pub const fn next_action(self) -> M5BadgeNextAction {
        match self {
            Self::RetestPending => M5BadgeNextAction::AwaitRetest,
            Self::EvidenceStale => M5BadgeNextAction::RefreshEvidence,
            Self::ImportedEvidence => M5BadgeNextAction::ReverifyImportedEvidence,
        }
    }
}

/// The next action named on a claim-narrowing note, so a narrowed claim is actionable
/// from the note itself rather than from a secondary report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeNextAction {
    /// Await the pending retest.
    AwaitRetest,
    /// Refresh the stale evidence.
    RefreshEvidence,
    /// Re-verify the imported evidence locally.
    ReverifyImportedEvidence,
}

impl M5BadgeNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::AwaitRetest,
        Self::RefreshEvidence,
        Self::ReverifyImportedEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitRetest => "await_retest",
            Self::RefreshEvidence => "refresh_evidence",
            Self::ReverifyImportedEvidence => "reverify_imported_evidence",
        }
    }
}

/// A field the support / export packet carries so support-class and freshness truth is
/// reconstructable from the shared model. The fields in
/// [`M5BadgeClaimExportField::MANDATORY`] are required, and the support class and
/// freshness are always carried as *separate* fields so exported evidence never loses
/// badge meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeClaimExportField {
    /// The support-class value.
    SupportClass,
    /// The evidence-freshness value.
    Freshness,
    /// The derived effective-claim posture.
    EffectiveClaim,
    /// The support-class explanation.
    SupportClassExplanation,
    /// The freshness explanation.
    FreshnessExplanation,
    /// The opaque evidence source.
    EvidenceSource,
    /// The opaque last-evaluated timestamp.
    LastEvaluated,
    /// The claim-narrowing reason (when narrowed or retest-pending).
    NarrowingReason,
    /// The separately-filterable filter keys.
    FilterKeys,
}

impl M5BadgeClaimExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SupportClass,
        Self::Freshness,
        Self::EffectiveClaim,
        Self::SupportClassExplanation,
        Self::FreshnessExplanation,
        Self::EvidenceSource,
        Self::LastEvaluated,
        Self::NarrowingReason,
        Self::FilterKeys,
    ];

    /// The export fields every badge export must carry: both badge axes as separate
    /// fields, the effective claim, and the evidence source.
    pub const MANDATORY: [Self; 4] = [
        Self::SupportClass,
        Self::Freshness,
        Self::EffectiveClaim,
        Self::EvidenceSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClass => "support_class",
            Self::Freshness => "freshness",
            Self::EffectiveClaim => "effective_claim",
            Self::SupportClassExplanation => "support_class_explanation",
            Self::FreshnessExplanation => "freshness_explanation",
            Self::EvidenceSource => "evidence_source",
            Self::LastEvaluated => "last_evaluated",
            Self::NarrowingReason => "narrowing_reason",
            Self::FilterKeys => "filter_keys",
        }
    }
}

/// A self-contained claim-narrowing note: the exact reason, the next action, and — the
/// acceptance-criterion invariant — the *preserved* support-class context, so a
/// narrowed claim is understood from the note alone and the support class the claim
/// was making is never dropped when the evidence weakens.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ClaimNarrowingNote {
    /// The exact reason the freshness axis reduced the claim.
    pub reason: M5FreshnessReducesClaimReason,
    /// The next action a reviewer should take.
    pub next_action: M5BadgeNextAction,
    /// The support class the claim was making, preserved as context even though the
    /// evidence has weakened. Always equals the resolved support class.
    pub preserved_support_class: M5SupportClassBadgeValue,
    /// True when this reason narrows the claim (stale or imported evidence).
    pub narrows_claim: bool,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// support class, and the next action — never a generic `claim unavailable` and
    /// never implying freshness from the support class.
    pub headline: String,
}

/// The full input to the badge-claim resolver for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimInput {
    /// The opaque, export-safe subject label.
    pub subject_label: String,
    /// The declared support class.
    pub support_class: M5SupportClassBadgeValue,
    /// The declared evidence freshness.
    pub freshness: M5EvidenceFreshnessValue,
    /// The opaque, export-safe evidence source.
    pub evidence_source_repr: String,
    /// The opaque, export-safe last-evaluated representation.
    pub last_evaluated_repr: String,
}

/// The resolved support-class / evidence-freshness truth for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBadgeClaim {
    /// The opaque subject label.
    pub subject_label: String,
    /// The support class — carried as its own field, never merged with freshness.
    pub support_class: M5SupportClassBadgeValue,
    /// The evidence freshness — carried as its own field, never merged with support.
    pub freshness: M5EvidenceFreshnessValue,
    /// The derived effective claim, computed from freshness alone.
    pub effective_claim: M5EffectiveClaimPosture,
    /// True when the claim is current.
    pub is_current: bool,
    /// True when a retest is pending (claim shown, not narrowed).
    pub is_retest_pending: bool,
    /// True when imported or stale evidence has narrowed the claim.
    pub is_narrowed: bool,
    /// The opaque evidence source.
    pub evidence_source_repr: String,
    /// The opaque last-evaluated representation.
    pub last_evaluated_repr: String,
    /// The claim-narrowing note, present whenever the claim is not current.
    pub narrowing_note: Option<M5ClaimNarrowingNote>,
}

/// Errors returned by [`resolve_badge_claim`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BadgeClaimError {
    /// The subject label was empty.
    EmptySubjectLabel,
    /// The evidence source was empty.
    EmptyEvidenceSource,
    /// The last-evaluated representation was empty.
    EmptyLastEvaluated,
    /// A subject label, evidence source, or timestamp carried forbidden material.
    ForbiddenBadgeMaterial,
}

impl M5BadgeClaimError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySubjectLabel => "empty_subject_label",
            Self::EmptyEvidenceSource => "empty_evidence_source",
            Self::EmptyLastEvaluated => "empty_last_evaluated",
            Self::ForbiddenBadgeMaterial => "forbidden_badge_material",
        }
    }
}

impl fmt::Display for M5BadgeClaimError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "badge-claim resolution error: {}", self.as_str())
    }
}

impl Error for M5BadgeClaimError {}

/// Resolves one badge claim from its declared support class and evidence freshness.
///
/// The support class and the freshness stay two separate, composable cues. The derived
/// effective claim is computed from the freshness axis alone — a Certified capability
/// with stale evidence narrows exactly the same way a Community one does, because
/// freshness is never derived from support class. When imported or stale evidence
/// reduces the claim, the resolver produces a self-contained narrowing note that
/// *preserves* the underlying support-class context rather than dropping it.
pub fn resolve_badge_claim(
    input: &M5BadgeClaimInput,
) -> Result<M5ResolvedBadgeClaim, M5BadgeClaimError> {
    if input.subject_label.trim().is_empty() {
        return Err(M5BadgeClaimError::EmptySubjectLabel);
    }
    if input.evidence_source_repr.trim().is_empty() {
        return Err(M5BadgeClaimError::EmptyEvidenceSource);
    }
    if input.last_evaluated_repr.trim().is_empty() {
        return Err(M5BadgeClaimError::EmptyLastEvaluated);
    }
    if value_repr_is_forbidden(&input.subject_label)
        || value_repr_is_forbidden(&input.evidence_source_repr)
        || value_repr_is_forbidden(&input.last_evaluated_repr)
    {
        return Err(M5BadgeClaimError::ForbiddenBadgeMaterial);
    }

    let effective_claim = derive_effective_claim(input.freshness);
    let is_current = effective_claim.is_current();
    let is_retest_pending = effective_claim.is_retest_pending();
    let is_narrowed = effective_claim.is_narrowed();

    let narrowing_note = effective_claim.reduces_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Support claim {}: {} — support class '{}' preserved as context; next: {}",
            if reason.narrows_claim() {
                "narrowed"
            } else {
                "flagged"
            },
            reason.phrase(),
            input.support_class.label(),
            next_action.as_str()
        );
        M5ClaimNarrowingNote {
            reason,
            next_action,
            preserved_support_class: input.support_class,
            narrows_claim: reason.narrows_claim(),
            headline,
        }
    });

    Ok(M5ResolvedBadgeClaim {
        subject_label: input.subject_label.clone(),
        support_class: input.support_class,
        freshness: input.freshness,
        effective_claim,
        is_current,
        is_retest_pending,
        is_narrowed,
        evidence_source_repr: input.evidence_source_repr.clone(),
        last_evaluated_repr: input.last_evaluated_repr.clone(),
        narrowing_note,
    })
}

/// Derives the effective claim from evidence freshness alone, so the support class is
/// never derived from freshness and freshness is never derived from support class.
fn derive_effective_claim(freshness: M5EvidenceFreshnessValue) -> M5EffectiveClaimPosture {
    match freshness {
        M5EvidenceFreshnessValue::Fresh => M5EffectiveClaimPosture::ClaimCurrent,
        M5EvidenceFreshnessValue::RetestPending => M5EffectiveClaimPosture::ClaimRetestPending,
        M5EvidenceFreshnessValue::EvidenceStale => {
            M5EffectiveClaimPosture::ClaimNarrowedEvidenceStale
        }
        M5EvidenceFreshnessValue::ImportedEvidence => {
            M5EffectiveClaimPosture::ClaimNarrowedImportedEvidence
        }
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs support-class and freshness truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimResolutionCase {
    /// The resolver input.
    pub input: M5BadgeClaimInput,
    /// The resolved truth. Must equal `resolve_badge_claim(&input)`.
    pub resolved: M5ResolvedBadgeClaim,
}

impl M5BadgeClaimResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BadgeClaimInput) -> Self {
        let resolved = resolve_badge_claim(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_badge_claim(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one badge consumer bound to the shared badge
/// anatomy, support-class values, freshness values, effective-claim postures,
/// narrowing reasons, next actions, explanation-drawer fields, export fields, and
/// accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimRow {
    /// Badge consumer family.
    pub consumer_surface: M5BadgeClaimConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 badge surface families that render / consume this pair.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this pair keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this consumer renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5BadgeClaimAnatomyPart>,
    /// Support-class values this consumer names.
    pub support_class_values: Vec<M5SupportClassBadgeValue>,
    /// Freshness values this consumer distinguishes.
    pub freshness_values: Vec<M5EvidenceFreshnessValue>,
    /// Effective-claim postures this consumer distinguishes.
    pub effective_claim_postures: Vec<M5EffectiveClaimPosture>,
    /// Claim-narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5FreshnessReducesClaimReason>,
    /// Next actions this consumer names.
    pub next_actions: Vec<M5BadgeNextAction>,
    /// Explanation-drawer fields this consumer opens (must include the mandatory
    /// [`M5BadgeExplanationField::MANDATORY`] fields).
    pub explanation_fields: Vec<M5BadgeExplanationField>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5BadgeClaimExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Badge subsystems that consume this pair's projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5BadgeClaimResolutionCase>,
    /// Hard invariant: this consumer never collapses the support and freshness axes
    /// into one overloaded badge. MUST be `false`.
    pub collapses_support_and_freshness_into_one_badge: bool,
    /// Hard invariant: this consumer never implies evidence freshness from the support
    /// class. MUST be `false`.
    pub implies_freshness_from_support_class: bool,
    /// Hard invariant: this consumer never drops the support-class context when it
    /// narrows a claim. MUST be `false`.
    pub drops_support_class_context_on_narrowing: bool,
    /// Hard invariant: this consumer never lets exported evidence lose badge meaning.
    /// MUST be `false`.
    pub drops_badge_meaning_in_export: bool,
}

impl M5BadgeClaimRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BadgeClaimAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BadgeClaimAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeClaimExportField> =
            self.export_fields.iter().copied().collect();
        M5BadgeClaimExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory explanation-drawer field.
    fn declares_mandatory_explanation_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeExplanationField> =
            self.explanation_fields.iter().copied().collect();
        M5BadgeExplanationField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_support_and_freshness_into_one_badge
            && !self.implies_freshness_from_support_class
            && !self.drops_support_class_context_on_narrowing
            && !self.drops_badge_meaning_in_export
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimVocabularySet {
    /// Badge-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Support-class-value tokens.
    pub support_class_values: Vec<String>,
    /// Freshness-value tokens.
    pub freshness_values: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Effective-claim-posture tokens.
    pub effective_claim_postures: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Explanation-field tokens (reused from the frozen matrix).
    pub explanation_fields: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Badge-consumer-subsystem tokens (reused from the frozen matrix).
    pub badge_consumer_surfaces: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5BadgeClaimVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5BadgeClaimConsumerSurface::ALL, |v| v.as_str()),
            support_class_values: tokens(&M5SupportClassBadgeValue::ALL, |v| v.as_str()),
            freshness_values: tokens(&M5EvidenceFreshnessValue::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5BadgeClaimAnatomyPart::ALL, |v| v.as_str()),
            effective_claim_postures: tokens(&M5EffectiveClaimPosture::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5FreshnessReducesClaimReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5BadgeNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BadgeClaimExportField::ALL, |v| v.as_str()),
            explanation_fields: tokens(&M5BadgeExplanationField::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BadgeSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
            badge_consumer_surfaces: tokens(&M5BadgeConsumerSurface::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BadgeDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimGovernanceReview {
    /// Support class and freshness are shown as two distinct, composable cues.
    pub support_and_freshness_shown_as_distinct_cues: bool,
    /// Neither badge is ever collapsed into the other.
    pub neither_badge_collapsed_into_the_other: bool,
    /// The support class never implies evidence freshness.
    pub support_class_never_implies_freshness: bool,
    /// The evidence freshness never implies a support class.
    pub freshness_never_implies_support_class: bool,
    /// Imported or stale evidence automatically narrows the claim.
    pub stale_or_imported_evidence_auto_narrows_claim: bool,
    /// Narrowing preserves the underlying support-class context.
    pub narrowing_preserves_support_class_context: bool,
    /// Every badge can open its explanation drawer.
    pub every_badge_opens_explanation_drawer: bool,
    /// Every badge is separately filterable.
    pub every_badge_is_separately_filterable: bool,
    /// Exported evidence keeps both badges' meaning.
    pub exported_evidence_keeps_badge_meaning: bool,
    /// No surface invents a second support-class or freshness grammar.
    pub no_surface_invents_second_badge_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel badge vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimConsumerProjection {
    /// Onboarding, Help, and marketplace surfaces consume the shared badge pair.
    pub onboarding_help_marketplace_surfaces_consume_shared_badges: bool,
    /// Diagnostics, certification, and evaluation surfaces consume the shared pair.
    pub diagnostics_certification_evaluation_surfaces_consume_shared_badges: bool,
    /// The support-class filter reads a single canonical source.
    pub support_class_filter_reads_single_source: bool,
    /// The freshness filter reads a single canonical source.
    pub freshness_filter_reads_single_source: bool,
    /// Support / export reads a single canonical badge-pair source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the badge-claim primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting badge audit.
    pub badge_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BadgeClaimPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BadgeClaimPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5BadgeClaimRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeClaimVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgeClaimGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeClaimConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeClaimProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeClaimReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 support-class / evidence-freshness badge primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeClaimPrimitivePacket {
    /// Record kind; must equal [`M5_BADGE_CLAIM_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BADGE_CLAIM_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5BadgeClaimRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeClaimVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgeClaimGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeClaimConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeClaimProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeClaimReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BadgeClaimPrimitivePacket {
    /// Builds an M5 badge-claim primitive packet from stable-lane input.
    pub fn new(input: M5BadgeClaimPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_BADGE_CLAIM_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_BADGE_CLAIM_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            badge_rows: input.badge_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 badge-claim primitive invariants.
    pub fn validate(&self) -> Vec<M5BadgeClaimPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BADGE_CLAIM_PRIMITIVE_RECORD_KIND {
            violations.push(M5BadgeClaimPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BADGE_CLAIM_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5BadgeClaimPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BadgeClaimPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_badge_rows(self, &mut violations);
        validate_distinct_cues_coverage(self, &mut violations);
        validate_context_preservation_coverage(self, &mut violations);
        validate_fresh_and_narrowed_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 badge-claim primitive packet serializes"),
        ) {
            violations.push(M5BadgeClaimPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 badge-claim primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per badge consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,support_class_values,freshness_values,effective_claim_postures,narrowing_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.badge_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.support_class_values, |v| v.as_str()),
                join_tokens(&row.freshness_values, |v| v.as_str()),
                join_tokens(&row.effective_claim_postures, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .badge_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Support-Class and Evidence-Freshness Badge Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge consumers: {} ({} stable)\n",
            self.badge_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Support-class values: {}\n",
            self.vocabulary_set.support_class_values.join(", ")
        ));
        out.push_str(&format!(
            "- Freshness values: {}\n",
            self.vocabulary_set.freshness_values.join(", ")
        ));
        out.push_str(&format!(
            "- Effective-claim postures: {}\n",
            self.vocabulary_set.effective_claim_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge consumers\n\n");
        for row in &self.badge_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let note = match &case.resolved.narrowing_note {
                    Some(note) => note.reason.as_str(),
                    None => "current",
                };
                out.push_str(&format!(
                    "    - support `{}` + freshness `{}` → `{}` (note `{}`)\n",
                    case.resolved.support_class.as_str(),
                    case.resolved.freshness.as_str(),
                    case.resolved.effective_claim.as_str(),
                    note
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 badge-claim primitive export.
#[derive(Debug)]
pub enum M5BadgeClaimPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BadgeClaimPrimitiveViolation>),
}

impl fmt::Display for M5BadgeClaimPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 badge-claim primitive export parse failed: {error}"
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
                    "m5 badge-claim primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BadgeClaimPrimitiveArtifactError {}

/// Validation failures emitted by [`M5BadgeClaimPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BadgeClaimPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required badge consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A badge row is incomplete.
    BadgeRowIncomplete,
    /// A badge row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A badge row declares no support-class values.
    SupportClassValueMissing,
    /// A badge row declares no freshness values.
    FreshnessValueMissing,
    /// A badge row declares no effective-claim postures.
    EffectiveClaimPostureMissing,
    /// A badge row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A badge row omits one of the mandatory explanation-drawer fields.
    ExplanationDrawerIncomplete,
    /// A badge row declares no accessibility routes (or misses keyboard focus or
    /// non-color encoding).
    AccessibilityRouteMissing,
    /// A badge row declares no badge-consumer subsystems.
    ConsumerSurfacesMissing,
    /// A badge row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A badge row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A badge claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves support and freshness as distinct cues (a high
    /// support class carried with narrowed evidence).
    DistinctCuesUnproven,
    /// No worked resolution proves a narrowed claim preserving its support-class
    /// context.
    ContextPreservationUnproven,
    /// No worked resolution proves both a current and a narrowed claim.
    FreshAndNarrowedCoverageUnproven,
    /// A badge row violates a hard invariant.
    BadgeInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BadgeClaimPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::BadgeRowIncomplete => "badge_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::SupportClassValueMissing => "support_class_value_missing",
            Self::FreshnessValueMissing => "freshness_value_missing",
            Self::EffectiveClaimPostureMissing => "effective_claim_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ExplanationDrawerIncomplete => "explanation_drawer_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::DistinctCuesUnproven => "distinct_cues_unproven",
            Self::ContextPreservationUnproven => "context_preservation_unproven",
            Self::FreshAndNarrowedCoverageUnproven => "fresh_and_narrowed_coverage_unproven",
            Self::BadgeInvariantViolated => "badge_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 badge-claim primitive export.
pub fn current_stable_m5_badge_claim_primitive_export(
) -> Result<M5BadgeClaimPrimitivePacket, M5BadgeClaimPrimitiveArtifactError> {
    let packet: M5BadgeClaimPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-support-class-and-evidence-freshness-badge-proof/support_export.json"
    )))
    .map_err(M5BadgeClaimPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BadgeClaimPrimitiveArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BADGE_CLAIM_SCHEMA_REF,
        M5_BADGE_CLAIM_DOC_REF,
        M5_BADGE_CLAIM_FAMILY_MATRIX_REF,
        M5_BADGE_CLAIM_SUPPORT_CLASS_REF,
        M5_BADGE_CLAIM_FRESHNESS_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BadgeClaimPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BadgeClaimPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_badge_rows(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let present: BTreeSet<M5BadgeClaimConsumerSurface> = packet
        .badge_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5BadgeClaimConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5BadgeClaimPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.badge_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5BadgeClaimPrimitiveViolation::BadgeRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5BadgeClaimPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.support_class_values.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::SupportClassValueMissing);
        }
        if row.freshness_values.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::FreshnessValueMissing);
        }
        if row.effective_claim_postures.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::EffectiveClaimPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BadgeClaimPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if !row.declares_mandatory_explanation_fields() {
            violations.push(M5BadgeClaimPrimitiveViolation::ExplanationDrawerIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5BadgeClaimPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5BadgeClaimPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BadgeClaimPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BadgeClaimPrimitiveViolation::BadgeInvariantViolated);
        }
    }
}

/// AC1: at least one worked resolution must prove the support class and the freshness
/// stay distinct, composable cues — a high support class (Certified or Supported)
/// carried together with narrowed (imported or stale) evidence, proving that neither
/// axis is derived from the other.
fn validate_distinct_cues_coverage(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            matches!(
                case.resolved.support_class,
                M5SupportClassBadgeValue::Certified | M5SupportClassBadgeValue::Supported
            ) && case.resolved.is_narrowed
        })
    });
    if !proven {
        violations.push(M5BadgeClaimPrimitiveViolation::DistinctCuesUnproven);
    }
}

/// AC2: at least one worked resolution must prove a narrowed claim whose narrowing note
/// preserves the underlying support-class context — the support class the claim was
/// making is carried into the note rather than dropped.
fn validate_context_preservation_coverage(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_narrowed
                && case.resolved.narrowing_note.as_ref().is_some_and(|note| {
                    note.preserved_support_class == case.resolved.support_class
                        && note.narrows_claim
                        && !note.headline.trim().is_empty()
                })
        })
    });
    if !proven {
        violations.push(M5BadgeClaimPrimitiveViolation::ContextPreservationUnproven);
    }
}

/// At least one worked resolution must prove a current claim (fresh evidence) and at
/// least one must prove a narrowed claim — the acceptance-criterion example that
/// freshness moves the claim independently of support class.
fn validate_fresh_and_narrowed_coverage(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let has_current = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_current)
    });
    let has_narrowed = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_narrowed)
    });
    if !(has_current && has_narrowed) {
        violations.push(M5BadgeClaimPrimitiveViolation::FreshAndNarrowedCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.support_and_freshness_shown_as_distinct_cues,
        review.neither_badge_collapsed_into_the_other,
        review.support_class_never_implies_freshness,
        review.freshness_never_implies_support_class,
        review.stale_or_imported_evidence_auto_narrows_claim,
        review.narrowing_preserves_support_class_context,
        review.every_badge_opens_explanation_drawer,
        review.every_badge_is_separately_filterable,
        review.exported_evidence_keeps_badge_meaning,
        review.no_surface_invents_second_badge_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BadgeClaimPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.onboarding_help_marketplace_surfaces_consume_shared_badges,
        projection.diagnostics_certification_evaluation_surfaces_consume_shared_badges,
        projection.support_class_filter_reads_single_source,
        projection.freshness_filter_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BadgeClaimPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BadgeClaimPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BadgeClaimPrimitivePacket,
    violations: &mut Vec<M5BadgeClaimPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.badge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BadgeClaimPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
