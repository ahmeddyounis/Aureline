//! M5 documentation-claim certification across docs/help/onboarding/AI profiles,
//! auto-narrowing claims when documentation evidence goes stale.
//!
//! Where the frozen docs-contracts matrix locks the seven governed
//! documentation objects — docs source descriptor, docs result object, docs-pack
//! manifest, derived-explanation citation set, version-match state, stale-example
//! finding, and browser-handoff object — this module *certifies* every claimed
//! M5 documentation-facing profile against that matrix and against the checked-in
//! evidence corpus those objects produce. Each
//! [`DocsProfileQualificationRow`] binds one claimed profile (docs browser,
//! help/about/service-health, onboarding/learning, AI explanation, support
//! export) to the documentation-evidence classes it depends on, the upstream
//! schemas and support exports that form its evidence, the qualification class it
//! earned, a certification verdict, downgrade triggers, and an explicit
//! "not greener than the matrix" flag.
//!
//! The packet carries five machine-readable companions that release, claim
//! publication, support, onboarding, and About/help/service-health tooling ingest
//! directly instead of cloning status prose:
//!
//! - a [`DocsClaimCompatibilityReport`] proving every certified profile stays
//!   compatible with — and no greener than — the frozen matrix and that every
//!   documentation-evidence class is covered,
//! - a [`DocsClaimDowngradeRule`] set encoding how a stale source-class,
//!   docs-pack-lifecycle, version-match, citation-set, or browser-handoff
//!   evidence class automatically narrows or holds the affected profile before
//!   publication,
//! - a [`DocsClaimTrustReview`] block recording the documentation-truth
//!   invariants (source class visible, project never masquerades as vendor,
//!   derived explanations never outlive their citation sets, browser handoff
//!   never silently shares context),
//! - a [`DocsClaimConsumerProjection`] block naming the surfaces that consume the
//!   packet rather than re-deriving docs/help truth,
//! - a [`DocsClaimProofFreshness`] block driving auto-narrow and retest-pending
//!   behavior on stale proof.
//!
//! The certification packet is canonical for claimed M5 documentation support in
//! this lane: no profile may stay greener than this packet, and no profile may
//! stay greener than the frozen matrix. It references upstream schemas, support
//! exports, and contracts by id rather than embedding their content. Raw document
//! bodies, raw source files, rendered HTML, raw URLs, raw query text, raw provider
//! payloads, credentials, and live vendor-doc snapshots stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/docs/certify-docs-source-pack-version-citation-and-browser-handoff-truth-and-narrow-stale-claims.schema.json`](../../../../schemas/docs/certify-docs-source-pack-version-citation-and-browser-handoff-truth-and-narrow-stale-claims.schema.json).
//! The contract doc is
//! [`docs/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md`](../../../../docs/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/`](../../../../fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF, BROWSER_HANDOFF_OBJECTS_SCHEMA_REF,
    DERIVED_EXPLANATION_CITATION_ARTIFACT_REF, DERIVED_EXPLANATION_CITATION_SCHEMA_REF,
    DOCS_PACK_MANAGER_ARTIFACT_REF, DOCS_PACK_MANAGER_SCHEMA_REF,
    DOCS_PRECEDENCE_RANKING_ARTIFACT_REF, DOCS_PRECEDENCE_RANKING_SCHEMA_REF,
    DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF, DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF,
    DOCS_VERSION_FRESHNESS_ARTIFACT_REF, DOCS_VERSION_FRESHNESS_SCHEMA_REF,
    M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF, M5_DOCS_CONTRACTS_MATRIX_DOC_REF,
    M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF, M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION,
};

/// Stable record-kind tag carried by [`DocsClaimCertificationPacket`].
pub const DOCS_CLAIM_CERTIFICATION_RECORD_KIND: &str =
    "certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims";

/// Schema version for documentation-claim certification records.
pub const DOCS_CLAIM_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_CLAIM_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/docs/certify-docs-source-pack-version-citation-and-browser-handoff-truth-and-narrow-stale-claims.schema.json";

/// Repo-relative path of the certification contract doc.
pub const DOCS_CLAIM_CERTIFICATION_DOC_REF: &str =
    "docs/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_CLAIM_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_CLAIM_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_CLAIM_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims.md";

/// One claimed M5 documentation-facing profile certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedDocsProfile {
    /// Documentation browser, search, and result rows.
    DocsBrowser,
    /// Help / About / service-health surface.
    HelpAbout,
    /// Onboarding, learning, glossary, and guided-tour surface.
    OnboardingLearning,
    /// AI explanation surface with citation chips.
    AiExplanation,
    /// Support / export packet surface.
    SupportExport,
}

impl CertifiedDocsProfile {
    /// Every certified profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsBrowser,
        Self::HelpAbout,
        Self::OnboardingLearning,
        Self::AiExplanation,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::HelpAbout => "help_about",
            Self::OnboardingLearning => "onboarding_learning",
            Self::AiExplanation => "ai_explanation",
            Self::SupportExport => "support_export",
        }
    }

    /// Documentation-evidence classes this profile depends on, in declaration
    /// order.
    pub fn evidence_classes(self) -> &'static [DocsEvidenceClass] {
        use DocsEvidenceClass as E;
        match self {
            Self::DocsBrowser => &[
                E::SourceClass,
                E::DocsPackLifecycle,
                E::VersionMatch,
                E::BrowserHandoff,
            ],
            Self::HelpAbout => &[E::SourceClass, E::VersionMatch, E::BrowserHandoff],
            Self::OnboardingLearning => &[E::SourceClass, E::VersionMatch, E::CitationSet],
            Self::AiExplanation => &[
                E::SourceClass,
                E::VersionMatch,
                E::CitationSet,
                E::BrowserHandoff,
            ],
            Self::SupportExport => &[
                E::SourceClass,
                E::DocsPackLifecycle,
                E::VersionMatch,
                E::CitationSet,
                E::BrowserHandoff,
            ],
        }
    }

    /// Whether this profile renders derived explanations and so MUST keep a
    /// citation basis.
    pub fn requires_citation_basis(self) -> bool {
        self.evidence_classes()
            .contains(&DocsEvidenceClass::CitationSet)
    }

    /// Whether this profile can hand off to an external surface and so MUST keep
    /// browser-handoff context isolation.
    pub fn touches_browser_handoff(self) -> bool {
        self.evidence_classes()
            .contains(&DocsEvidenceClass::BrowserHandoff)
    }
}

/// One class of documentation evidence a profile is certified against.
///
/// Each class maps to one upstream B-batch documentation contract and its
/// checked-in support export; a class going stale narrows or holds every profile
/// that depends on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsEvidenceClass {
    /// Docs source class, trust class, and source/result-object reuse, including
    /// source-class precedence and ranking parity.
    SourceClass,
    /// Docs-pack manifest, lifecycle, and import/export continuity.
    DocsPackLifecycle,
    /// Version-match and freshness state, including stale-example findings.
    VersionMatch,
    /// Derived-explanation citation set binding generated prose to its evidence.
    CitationSet,
    /// Browser / provider-console handoff object disclosing destination, reason,
    /// privacy consequence, and return anchor.
    BrowserHandoff,
}

impl DocsEvidenceClass {
    /// Every evidence class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SourceClass,
        Self::DocsPackLifecycle,
        Self::VersionMatch,
        Self::CitationSet,
        Self::BrowserHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClass => "source_class",
            Self::DocsPackLifecycle => "docs_pack_lifecycle",
            Self::VersionMatch => "version_match",
            Self::CitationSet => "citation_set",
            Self::BrowserHandoff => "browser_handoff",
        }
    }

    /// Upstream schema refs that form the evidence for this class.
    pub fn evidence_schema_refs(self) -> &'static [&'static str] {
        match self {
            Self::SourceClass => &[
                DOCS_SOURCE_RESULT_REUSE_SCHEMA_REF,
                DOCS_PRECEDENCE_RANKING_SCHEMA_REF,
            ],
            Self::DocsPackLifecycle => &[DOCS_PACK_MANAGER_SCHEMA_REF],
            Self::VersionMatch => &[DOCS_VERSION_FRESHNESS_SCHEMA_REF],
            Self::CitationSet => &[DERIVED_EXPLANATION_CITATION_SCHEMA_REF],
            Self::BrowserHandoff => &[BROWSER_HANDOFF_OBJECTS_SCHEMA_REF],
        }
    }

    /// Upstream support-export refs that form the evidence corpus for this class.
    pub fn evidence_artifact_refs(self) -> &'static [&'static str] {
        match self {
            Self::SourceClass => &[
                DOCS_SOURCE_RESULT_REUSE_ARTIFACT_REF,
                DOCS_PRECEDENCE_RANKING_ARTIFACT_REF,
            ],
            Self::DocsPackLifecycle => &[DOCS_PACK_MANAGER_ARTIFACT_REF],
            Self::VersionMatch => &[DOCS_VERSION_FRESHNESS_ARTIFACT_REF],
            Self::CitationSet => &[DERIVED_EXPLANATION_CITATION_ARTIFACT_REF],
            Self::BrowserHandoff => &[BROWSER_HANDOFF_OBJECTS_ARTIFACT_REF],
        }
    }

    /// Downgrade trigger fired when this class's evidence falls out of the
    /// freshness SLO.
    pub const fn stale_trigger(self) -> DocsClaimDowngradeTrigger {
        match self {
            Self::SourceClass => DocsClaimDowngradeTrigger::SourceClassEvidenceStale,
            Self::DocsPackLifecycle => DocsClaimDowngradeTrigger::DocsPackLifecycleEvidenceStale,
            Self::VersionMatch => DocsClaimDowngradeTrigger::VersionMatchEvidenceStale,
            Self::CitationSet => DocsClaimDowngradeTrigger::CitationSetEvidenceStale,
            Self::BrowserHandoff => DocsClaimDowngradeTrigger::BrowserHandoffEvidenceStale,
        }
    }
}

/// Qualification class a profile is certified at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsClaimQualificationClass {
    /// Certified for the Stable claim.
    Stable,
    /// Certified at Beta.
    Beta,
    /// Certified at Preview.
    Preview,
    /// Experimental; not claimed.
    Experimental,
    /// Unavailable on this build.
    Unavailable,
    /// Held pending upstream resolution.
    Held,
}

impl DocsClaimQualificationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Greenness rank; a higher rank is a stronger public claim.
    ///
    /// Used to enforce that no certified profile is greener than the frozen
    /// matrix.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Stable => 5,
            Self::Beta => 4,
            Self::Preview => 3,
            Self::Experimental => 2,
            Self::Held => 1,
            Self::Unavailable => 0,
        }
    }

    /// Whether the class carries a publicly claimable promotion (Stable or Beta).
    pub const fn is_promoted(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Certification verdict recorded for a profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsClaimVerdict {
    /// Profile is certified at its claimed qualification with current evidence.
    Certified,
    /// Profile was narrowed to a lower, still-promoted qualification to match its
    /// evidence.
    NarrowedToQualified,
    /// Profile's evidence went stale; the claim is narrowed and awaiting a
    /// retest before it may be re-promoted.
    RetestPending,
    /// Profile is held pending evidence or upstream graduation.
    HeldPendingEvidence,
    /// Profile is blocked from publication because it is underqualified.
    BlockedUnderqualified,
}

impl DocsClaimVerdict {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedToQualified => "narrowed_to_qualified",
            Self::RetestPending => "retest_pending",
            Self::HeldPendingEvidence => "held_pending_evidence",
            Self::BlockedUnderqualified => "blocked_underqualified",
        }
    }

    /// Whether the verdict allows the profile to keep a promoted public claim.
    pub const fn permits_publication(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedToQualified)
    }

    /// Whether the verdict is a hard block that must fail claim publication.
    pub const fn blocks_publication(self) -> bool {
        matches!(
            self,
            Self::HeldPendingEvidence | Self::BlockedUnderqualified
        )
    }
}

/// Downgrade trigger that can narrow a profile below its certified claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsClaimDowngradeTrigger {
    /// Docs source-class / source-result evidence went stale.
    SourceClassEvidenceStale,
    /// Docs-pack lifecycle / manifest evidence went stale.
    DocsPackLifecycleEvidenceStale,
    /// Version-match / freshness evidence went stale.
    VersionMatchEvidenceStale,
    /// Derived-explanation citation-set evidence went stale.
    CitationSetEvidenceStale,
    /// Browser-handoff evidence went stale.
    BrowserHandoffEvidenceStale,
    /// Profile parity against the frozen matrix degraded.
    ParityDegradedVsMatrix,
    /// An upstream governed object in the frozen matrix narrowed.
    UpstreamMatrixNarrowed,
    /// Proof packet exceeded the freshness SLO.
    ProofFreshnessExpired,
    /// Certified profile drifted greener than the frozen matrix.
    GreenerThanMatrix,
}

impl DocsClaimDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SourceClassEvidenceStale,
        Self::DocsPackLifecycleEvidenceStale,
        Self::VersionMatchEvidenceStale,
        Self::CitationSetEvidenceStale,
        Self::BrowserHandoffEvidenceStale,
        Self::ParityDegradedVsMatrix,
        Self::UpstreamMatrixNarrowed,
        Self::ProofFreshnessExpired,
        Self::GreenerThanMatrix,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClassEvidenceStale => "source_class_evidence_stale",
            Self::DocsPackLifecycleEvidenceStale => "docs_pack_lifecycle_evidence_stale",
            Self::VersionMatchEvidenceStale => "version_match_evidence_stale",
            Self::CitationSetEvidenceStale => "citation_set_evidence_stale",
            Self::BrowserHandoffEvidenceStale => "browser_handoff_evidence_stale",
            Self::ParityDegradedVsMatrix => "parity_degraded_vs_matrix",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::ProofFreshnessExpired => "proof_freshness_expired",
            Self::GreenerThanMatrix => "greener_than_matrix",
        }
    }
}

/// Automatic narrowing action a downgrade rule applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsClaimDowngradeAction {
    /// Narrow the profile to Beta.
    NarrowToBeta,
    /// Narrow the profile to Preview.
    NarrowToPreview,
    /// Mark the profile retest-pending until its evidence is re-proven.
    MarkRetestPending,
    /// Hold the profile pending evidence.
    Hold,
    /// Block publication of the profile.
    BlockPublication,
}

impl DocsClaimDowngradeAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowToBeta => "narrow_to_beta",
            Self::NarrowToPreview => "narrow_to_preview",
            Self::MarkRetestPending => "mark_retest_pending",
            Self::Hold => "hold",
            Self::BlockPublication => "block_publication",
        }
    }
}

/// Consumer surface that must project this certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsClaimConsumerSurface {
    /// Release / promotion gate tooling.
    ReleaseGate,
    /// Claim-publication pipeline.
    ClaimPublication,
    /// About / help / service-health surface.
    AboutHelpServiceHealth,
    /// Support / export packet.
    SupportExport,
    /// Onboarding / learning surface.
    Onboarding,
    /// AI context / explanation surface.
    AiContext,
    /// Docs M5 evidence index.
    EvidenceIndex,
}

impl DocsClaimConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReleaseGate,
        Self::ClaimPublication,
        Self::AboutHelpServiceHealth,
        Self::SupportExport,
        Self::Onboarding,
        Self::AiContext,
        Self::EvidenceIndex,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseGate => "release_gate",
            Self::ClaimPublication => "claim_publication",
            Self::AboutHelpServiceHealth => "about_help_service_health",
            Self::SupportExport => "support_export",
            Self::Onboarding => "onboarding",
            Self::AiContext => "ai_context",
            Self::EvidenceIndex => "evidence_index",
        }
    }
}

/// One certified documentation-profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsProfileQualificationRow {
    /// Claimed documentation-facing profile.
    pub profile: CertifiedDocsProfile,
    /// Qualification class the profile is certified at.
    pub qualification: DocsClaimQualificationClass,
    /// Certification verdict.
    pub verdict: DocsClaimVerdict,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Documentation-evidence classes this profile is certified against.
    pub evidence_classes: Vec<DocsEvidenceClass>,
    /// Upstream schema refs that form this profile's evidence.
    pub evidence_schema_refs: Vec<String>,
    /// Upstream support-export refs that form this profile's evidence corpus.
    pub evidence_artifact_refs: Vec<String>,
    /// Proof packet refs backing this certification.
    pub evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this profile.
    pub downgrade_triggers: Vec<DocsClaimDowngradeTrigger>,
    /// True when the certified claim is not greener than the frozen matrix.
    pub not_greener_than_matrix: bool,
    /// Source class is disclosed on every result so project docs never
    /// masquerade as vendor docs.
    pub source_class_disclosed: bool,
    /// Derived explanations on this profile keep a citation basis (required when
    /// the profile depends on the citation-set evidence class).
    pub citation_basis_required: bool,
    /// Browser handoff from this profile keeps context isolated (required when
    /// the profile depends on the browser-handoff evidence class).
    pub browser_handoff_context_isolated: bool,
}

impl DocsProfileQualificationRow {
    /// Whether this row carries a promoted, publication-permitting certification.
    pub fn is_promoted_and_certified(&self) -> bool {
        self.qualification.is_promoted() && self.verdict.permits_publication()
    }
}

/// Compatibility report binding the certification to the frozen matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimCompatibilityReport {
    /// Ref of the frozen matrix support export this packet certifies against.
    pub matrix_artifact_ref: String,
    /// Ref of the frozen matrix schema.
    pub matrix_schema_ref: String,
    /// Matrix schema version this certification is compatible with.
    pub matrix_schema_version: u32,
    /// Every claimed profile is present in the packet.
    pub all_profiles_present: bool,
    /// Every documentation-evidence class is covered by at least one profile.
    pub all_evidence_classes_covered: bool,
    /// No certified profile is greener than the frozen matrix.
    pub no_profile_greener_than_matrix: bool,
    /// Every certified profile references its upstream evidence.
    pub every_profile_has_evidence: bool,
    /// Downgrade rules are auto-enforced in release/claim/support tooling.
    pub downgrade_rules_auto_enforced: bool,
}

/// One machine-readable downgrade rule consumed by release/claim/support tooling.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimDowngradeRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Trigger that fires the rule.
    pub trigger: DocsClaimDowngradeTrigger,
    /// Narrowing action the rule applies.
    pub action: DocsClaimDowngradeAction,
    /// Profiles the rule applies to.
    pub applies_to_profiles: Vec<CertifiedDocsProfile>,
    /// Evidence classes whose staleness fires this rule.
    pub applies_to_evidence_classes: Vec<DocsEvidenceClass>,
    /// True when the rule is enforced automatically rather than by hand.
    pub auto_enforced: bool,
    /// Human-readable rationale.
    pub rationale: String,
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimTrustReview {
    /// Source class stays visible and project docs never masquerade as vendor docs.
    pub source_class_visible_no_vendor_masquerade: bool,
    /// Version-match and freshness state stay visible on every documentation answer.
    pub version_match_and_freshness_visible: bool,
    /// Derived explanations keep a citation basis and never outlive their citation sets.
    pub citation_basis_preserved_derived_never_outlives_citations: bool,
    /// Browser handoff never silently shares context or impersonates a docs surface.
    pub browser_handoff_context_not_silently_shared: bool,
    /// Mirror / offline posture stays visible rather than masked.
    pub mirror_offline_state_visible: bool,
    /// No certified profile stays greener than this canonical packet.
    pub no_profile_greener_than_packet: bool,
    /// Downgrade narrows the claim rather than hiding the profile.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale, partial, or failing evidence narrows or blocks claim publication.
    pub stale_or_partial_evidence_narrows_or_blocks_publication: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimConsumerProjection {
    /// Release gate ingests the certification packet rather than cloning text.
    pub release_gate_consumes_packet: bool,
    /// Claim publication narrows public language from this packet.
    pub claim_publication_consumes_packet: bool,
    /// About / help / service-health shows certification truth.
    pub about_help_service_health_consumes_packet: bool,
    /// Support export shows certification truth.
    pub support_export_consumes_packet: bool,
    /// Onboarding / learning shows certification truth.
    pub onboarding_consumes_packet: bool,
    /// AI context shows certification truth.
    pub ai_context_consumes_packet: bool,
    /// Narrowed, retest-pending, held, or blocked profiles are visibly labeled,
    /// not hidden.
    pub narrowed_profiles_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the certification.
    pub auto_narrow_on_stale: bool,
    /// True when stale proof marks affected profiles retest-pending.
    pub retest_pending_on_stale: bool,
}

/// Constructor input for [`DocsClaimCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocsClaimCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profile rows.
    pub profile_rows: Vec<DocsProfileQualificationRow>,
    /// Compatibility report.
    pub compatibility_report: DocsClaimCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<DocsClaimDowngradeRule>,
    /// Trust review block.
    pub trust_review: DocsClaimTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DocsClaimConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DocsClaimProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Upstream support-export refs forming the evidence corpus.
    pub evidence_corpus_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 documentation-claim certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsClaimCertificationPacket {
    /// Record kind; must equal [`DOCS_CLAIM_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_CLAIM_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified profile rows.
    pub profile_rows: Vec<DocsProfileQualificationRow>,
    /// Compatibility report.
    pub compatibility_report: DocsClaimCompatibilityReport,
    /// Downgrade rules.
    pub downgrade_rules: Vec<DocsClaimDowngradeRule>,
    /// Trust review block.
    pub trust_review: DocsClaimTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DocsClaimConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: DocsClaimProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Upstream support-export refs forming the evidence corpus.
    pub evidence_corpus_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DocsClaimCertificationPacket {
    /// Builds a certification packet from stable-lane input.
    pub fn new(input: DocsClaimCertificationPacketInput) -> Self {
        Self {
            record_kind: DOCS_CLAIM_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: DOCS_CLAIM_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            profile_rows: input.profile_rows,
            compatibility_report: input.compatibility_report,
            downgrade_rules: input.downgrade_rules,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            evidence_corpus_refs: input.evidence_corpus_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Profiles whose certification narrows or holds the public claim.
    ///
    /// Release, claim-publication, and support tooling use this to render the
    /// narrowed profiles rather than hiding them.
    pub fn narrowed_profiles(&self) -> Vec<CertifiedDocsProfile> {
        self.profile_rows
            .iter()
            .filter(|row| {
                !matches!(row.verdict, DocsClaimVerdict::Certified)
                    || !row.qualification.is_promoted()
            })
            .map(|row| row.profile)
            .collect()
    }

    /// Profiles whose evidence went stale and that await a retest.
    pub fn retest_pending_profiles(&self) -> Vec<CertifiedDocsProfile> {
        self.profile_rows
            .iter()
            .filter(|row| matches!(row.verdict, DocsClaimVerdict::RetestPending))
            .map(|row| row.profile)
            .collect()
    }

    /// Profiles whose verdict is a hard block on claim publication.
    ///
    /// A non-empty result means publication must fail until the profile is
    /// re-certified or narrowed.
    pub fn publication_blockers(&self) -> Vec<CertifiedDocsProfile> {
        self.profile_rows
            .iter()
            .filter(|row| row.verdict.blocks_publication())
            .map(|row| row.profile)
            .collect()
    }

    /// Returns a narrowed copy of this packet for a set of stale evidence classes.
    ///
    /// Every profile that depends on a stale evidence class is marked
    /// retest-pending and narrowed to Preview, with the corresponding stale
    /// trigger recorded. This is the auto-narrow behavior release and
    /// claim-publication tooling apply when documentation evidence falls out of
    /// the freshness SLO, narrowing the public claim instead of leaving old green
    /// language in product and docs.
    pub fn narrowed_for_stale_evidence(&self, stale: &[DocsEvidenceClass]) -> Self {
        let stale_set: BTreeSet<DocsEvidenceClass> = stale.iter().copied().collect();
        let mut next = self.clone();
        for row in next.profile_rows.iter_mut() {
            let row_stale: Vec<DocsEvidenceClass> = row
                .evidence_classes
                .iter()
                .copied()
                .filter(|class| stale_set.contains(class))
                .collect();
            if row_stale.is_empty() {
                continue;
            }
            row.verdict = DocsClaimVerdict::RetestPending;
            row.qualification = DocsClaimQualificationClass::Preview;
            for class in &row_stale {
                let trigger = class.stale_trigger();
                if !row.downgrade_triggers.contains(&trigger) {
                    row.downgrade_triggers.push(trigger);
                }
            }
            let classes = row_stale
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            row.scope_summary = format!(
                "{} — narrowed to Preview, retest pending: evidence for [{classes}] exceeded the freshness SLO; the public claim is narrowed until re-proven",
                row.scope_summary
            );
        }
        next
    }

    /// Validates the certification invariants.
    pub fn validate(&self) -> Vec<DocsClaimCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != DOCS_CLAIM_CERTIFICATION_RECORD_KIND {
            violations.push(DocsClaimCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != DOCS_CLAIM_CERTIFICATION_SCHEMA_VERSION {
            violations.push(DocsClaimCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DocsClaimCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_evidence_corpus(self, &mut violations);
        validate_profile_rows(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_downgrade_rules(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("certification packet serializes"),
        ) {
            violations.push(DocsClaimCertificationViolation::RawBoundaryMaterialInExport);
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

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let certified = self
            .profile_rows
            .iter()
            .filter(|row| matches!(row.verdict, DocsClaimVerdict::Certified))
            .count();
        let mut out = String::new();
        out.push_str("# M5 Documentation-Claim Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Profiles: {} ({} certified, {} narrowed/retest-pending/held/blocked)\n",
            self.profile_rows.len(),
            certified,
            self.narrowed_profiles().len()
        ));
        out.push_str(&format!(
            "- Evidence classes: {} (each covered by at least one profile)\n",
            DocsEvidenceClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Downgrade rules: {} (auto-enforced)\n",
            self.downgrade_rules.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Profiles\n\n");
        for row in &self.profile_rows {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}`\n",
                row.profile.as_str(),
                row.qualification.as_str(),
                row.verdict.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            let classes = row
                .evidence_classes
                .iter()
                .map(|class| class.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("  - Evidence: {classes}\n"));
        }
        let pending = self.retest_pending_profiles();
        if !pending.is_empty() {
            out.push_str("\n## Retest-pending profiles\n\n");
            for profile in pending {
                out.push_str(&format!("- `{}`\n", profile.as_str()));
            }
        }
        let blockers = self.publication_blockers();
        if !blockers.is_empty() {
            out.push_str("\n## Publication blockers\n\n");
            for profile in blockers {
                out.push_str(&format!("- `{}`\n", profile.as_str()));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in certification export.
#[derive(Debug)]
pub enum DocsClaimCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DocsClaimCertificationViolation>),
}

impl fmt::Display for DocsClaimCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "certification export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsClaimCertificationArtifactError {}

/// Validation failures emitted by [`DocsClaimCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DocsClaimCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// Evidence corpus refs are incomplete.
    MissingEvidenceCorpus,
    /// A required profile is missing from the packet.
    RequiredProfileMissing,
    /// A documentation-evidence class is not covered by any profile.
    EvidenceClassUncovered,
    /// A profile row is incomplete.
    ProfileRowIncomplete,
    /// A profile row's evidence refs do not match its evidence classes.
    EvidenceRefMismatch,
    /// A certified-and-promoted profile is missing proof packet refs.
    CertifiedProfileMissingEvidence,
    /// A profile has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A profile claims to be no greener than the matrix but is not flagged so.
    ProfileGreenerThanMatrix,
    /// A citation-using profile does not preserve its citation basis.
    CitationBasisMissing,
    /// A browser-handoff profile does not isolate handoff context.
    BrowserHandoffContextNotIsolated,
    /// A publication-permitting verdict carries a non-promoted qualification, or
    /// a blocking verdict carries a promoted qualification.
    VerdictQualificationMismatch,
    /// Compatibility report does not satisfy required invariants.
    CompatibilityReportIncomplete,
    /// Downgrade rules are missing, not auto-enforced, or do not cover every
    /// evidence-class staleness trigger.
    DowngradeRulesIncomplete,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl DocsClaimCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingEvidenceCorpus => "missing_evidence_corpus",
            Self::RequiredProfileMissing => "required_profile_missing",
            Self::EvidenceClassUncovered => "evidence_class_uncovered",
            Self::ProfileRowIncomplete => "profile_row_incomplete",
            Self::EvidenceRefMismatch => "evidence_ref_mismatch",
            Self::CertifiedProfileMissingEvidence => "certified_profile_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ProfileGreenerThanMatrix => "profile_greener_than_matrix",
            Self::CitationBasisMissing => "citation_basis_missing",
            Self::BrowserHandoffContextNotIsolated => "browser_handoff_context_not_isolated",
            Self::VerdictQualificationMismatch => "verdict_qualification_mismatch",
            Self::CompatibilityReportIncomplete => "compatibility_report_incomplete",
            Self::DowngradeRulesIncomplete => "downgrade_rules_incomplete",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable certification export.
pub fn current_stable_docs_claim_certification_export(
) -> Result<DocsClaimCertificationPacket, DocsClaimCertificationArtifactError> {
    let packet: DocsClaimCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/certify_docs_source_pack_version_citation_and_browser_handoff_truth_and_narrow_stale_claims/support_export.json"
    )))
    .map_err(DocsClaimCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DocsClaimCertificationArtifactError::Validation(violations))
    }
}

/// Seeded stable certification input for emitters, the artifact, and tests.
pub fn seeded_stable_docs_claim_certification_input() -> DocsClaimCertificationPacketInput {
    DocsClaimCertificationPacketInput {
        packet_id: "m5-docs-claim-certification:stable:0001".to_owned(),
        certification_label: "M5 Documentation-Claim Certification".to_owned(),
        profile_rows: seeded_profile_rows(),
        compatibility_report: seeded_compatibility_report(),
        downgrade_rules: seeded_downgrade_rules(),
        trust_review: seeded_trust_review(),
        consumer_projection: seeded_consumer_projection(),
        proof_freshness: DocsClaimProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-26T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
            retest_pending_on_stale: true,
        },
        source_contract_refs: seeded_source_contract_refs(),
        evidence_corpus_refs: seeded_evidence_corpus_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-26T00:00:00Z".to_owned(),
    }
}

fn seeded_source_contract_refs() -> Vec<String> {
    let mut refs = vec![
        DOCS_CLAIM_CERTIFICATION_SCHEMA_REF.to_owned(),
        DOCS_CLAIM_CERTIFICATION_DOC_REF.to_owned(),
        M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF.to_owned(),
        M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF.to_owned(),
        M5_DOCS_CONTRACTS_MATRIX_DOC_REF.to_owned(),
    ];
    for class in DocsEvidenceClass::ALL {
        for schema_ref in class.evidence_schema_refs() {
            refs.push((*schema_ref).to_owned());
        }
    }
    refs
}

fn seeded_evidence_corpus_refs() -> Vec<String> {
    let mut refs = vec![M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF.to_owned()];
    for class in DocsEvidenceClass::ALL {
        for artifact_ref in class.evidence_artifact_refs() {
            refs.push((*artifact_ref).to_owned());
        }
    }
    refs
}

fn evidence_ref(profile: CertifiedDocsProfile) -> String {
    format!("evidence:{}:m5", profile.as_str().replace('_', "-"))
}

fn certified_row(
    profile: CertifiedDocsProfile,
    qualification: DocsClaimQualificationClass,
    scope_summary: &str,
    triggers: Vec<DocsClaimDowngradeTrigger>,
) -> DocsProfileQualificationRow {
    let evidence_classes = profile.evidence_classes().to_vec();
    let evidence_schema_refs = evidence_classes
        .iter()
        .flat_map(|class| class.evidence_schema_refs().iter().map(|s| (*s).to_owned()))
        .collect();
    let evidence_artifact_refs = evidence_classes
        .iter()
        .flat_map(|class| {
            class
                .evidence_artifact_refs()
                .iter()
                .map(|s| (*s).to_owned())
        })
        .collect();
    DocsProfileQualificationRow {
        profile,
        qualification,
        verdict: DocsClaimVerdict::Certified,
        scope_summary: scope_summary.to_owned(),
        evidence_classes,
        evidence_schema_refs,
        evidence_artifact_refs,
        evidence_packet_refs: vec![evidence_ref(profile)],
        downgrade_triggers: triggers,
        not_greener_than_matrix: true,
        source_class_disclosed: true,
        citation_basis_required: profile.requires_citation_basis(),
        browser_handoff_context_isolated: profile.touches_browser_handoff(),
    }
}

fn seeded_profile_rows() -> Vec<DocsProfileQualificationRow> {
    use CertifiedDocsProfile as P;
    use DocsClaimDowngradeTrigger as T;
    use DocsClaimQualificationClass as Q;
    vec![
        certified_row(
            P::DocsBrowser,
            Q::Stable,
            "Documentation browser, search, and result rows: source class, version match, freshness, and mirror/offline posture stay visible; docs-pack rows carry lifecycle state; external views route through a disclosed browser handoff",
            vec![
                T::SourceClassEvidenceStale,
                T::DocsPackLifecycleEvidenceStale,
                T::VersionMatchEvidenceStale,
                T::BrowserHandoffEvidenceStale,
                T::ParityDegradedVsMatrix,
            ],
        ),
        certified_row(
            P::HelpAbout,
            Q::Stable,
            "Help / About / service-health surface: explains which documentation source and version back each answer and routes external help through a disclosed browser handoff",
            vec![
                T::SourceClassEvidenceStale,
                T::VersionMatchEvidenceStale,
                T::BrowserHandoffEvidenceStale,
                T::UpstreamMatrixNarrowed,
            ],
        ),
        certified_row(
            P::OnboardingLearning,
            Q::Beta,
            "Onboarding, learning, glossary, and guided-tour surface: glossary cards and tour steps carry a citation basis and disclose source class and version match",
            vec![
                T::SourceClassEvidenceStale,
                T::VersionMatchEvidenceStale,
                T::CitationSetEvidenceStale,
                T::UpstreamMatrixNarrowed,
            ],
        ),
        certified_row(
            P::AiExplanation,
            Q::Beta,
            "AI explanation surface: every derived explanation binds to a citation set, discloses source class and version match, and routes provider-console exits through a disclosed browser handoff",
            vec![
                T::SourceClassEvidenceStale,
                T::VersionMatchEvidenceStale,
                T::CitationSetEvidenceStale,
                T::BrowserHandoffEvidenceStale,
            ],
        ),
        certified_row(
            P::SupportExport,
            Q::Stable,
            "Support / export packet surface: carries source-class, docs-pack-lifecycle, version-match, citation-set, and browser-handoff truth from one packet set without raw document bodies or provider payloads",
            vec![
                T::SourceClassEvidenceStale,
                T::DocsPackLifecycleEvidenceStale,
                T::VersionMatchEvidenceStale,
                T::CitationSetEvidenceStale,
                T::BrowserHandoffEvidenceStale,
                T::ProofFreshnessExpired,
            ],
        ),
    ]
}

fn seeded_compatibility_report() -> DocsClaimCompatibilityReport {
    DocsClaimCompatibilityReport {
        matrix_artifact_ref: M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF.to_owned(),
        matrix_schema_ref: M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF.to_owned(),
        matrix_schema_version: M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION,
        all_profiles_present: true,
        all_evidence_classes_covered: true,
        no_profile_greener_than_matrix: true,
        every_profile_has_evidence: true,
        downgrade_rules_auto_enforced: true,
    }
}

fn seeded_downgrade_rules() -> Vec<DocsClaimDowngradeRule> {
    use CertifiedDocsProfile as P;
    use DocsClaimDowngradeAction as A;
    use DocsClaimDowngradeTrigger as T;
    use DocsEvidenceClass as E;
    vec![
        DocsClaimDowngradeRule {
            rule_id: "downgrade:source_class_stale:retest_pending".to_owned(),
            trigger: T::SourceClassEvidenceStale,
            action: A::MarkRetestPending,
            applies_to_profiles: vec![
                P::DocsBrowser,
                P::HelpAbout,
                P::OnboardingLearning,
                P::AiExplanation,
                P::SupportExport,
            ],
            applies_to_evidence_classes: vec![E::SourceClass],
            auto_enforced: true,
            rationale: "When docs source-class / source-result evidence exceeds the freshness SLO, every profile that discloses source class is marked retest-pending and narrowed until re-proven, so project docs never silently masquerade as vendor docs.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:docs_pack_lifecycle_stale:narrow_beta".to_owned(),
            trigger: T::DocsPackLifecycleEvidenceStale,
            action: A::NarrowToBeta,
            applies_to_profiles: vec![P::DocsBrowser, P::SupportExport],
            applies_to_evidence_classes: vec![E::DocsPackLifecycle],
            auto_enforced: true,
            rationale: "Stale docs-pack manifest / lifecycle evidence narrows the docs browser and support export to Beta with explicit pack-state labels rather than overstating pack freshness.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:version_match_stale:retest_pending".to_owned(),
            trigger: T::VersionMatchEvidenceStale,
            action: A::MarkRetestPending,
            applies_to_profiles: vec![
                P::DocsBrowser,
                P::HelpAbout,
                P::OnboardingLearning,
                P::AiExplanation,
                P::SupportExport,
            ],
            applies_to_evidence_classes: vec![E::VersionMatch],
            auto_enforced: true,
            rationale: "When version-match / freshness evidence goes stale, affected profiles are marked retest-pending so no answer keeps exact-current confidence on a possibly-drifted version.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:citation_set_stale:hold".to_owned(),
            trigger: T::CitationSetEvidenceStale,
            action: A::Hold,
            applies_to_profiles: vec![P::OnboardingLearning, P::AiExplanation, P::SupportExport],
            applies_to_evidence_classes: vec![E::CitationSet],
            auto_enforced: true,
            rationale: "A derived-explanation citation set going stale holds the explaining profile: derived explanations never outlive their citation sets, so the explanation is held rather than published past its evidence.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:browser_handoff_stale:block".to_owned(),
            trigger: T::BrowserHandoffEvidenceStale,
            action: A::BlockPublication,
            applies_to_profiles: vec![
                P::DocsBrowser,
                P::HelpAbout,
                P::AiExplanation,
                P::SupportExport,
            ],
            applies_to_evidence_classes: vec![E::BrowserHandoff],
            auto_enforced: true,
            rationale: "Stale browser-handoff evidence blocks publication of any handoff-bearing profile: a handoff must not silently share context or impersonate a governed docs surface, so it is blocked until re-proven.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:proof_freshness_expired:hold".to_owned(),
            trigger: T::ProofFreshnessExpired,
            action: A::Hold,
            applies_to_profiles: vec![
                P::DocsBrowser,
                P::HelpAbout,
                P::OnboardingLearning,
                P::AiExplanation,
                P::SupportExport,
            ],
            applies_to_evidence_classes: DocsEvidenceClass::ALL.to_vec(),
            auto_enforced: true,
            rationale: "When the certification proof exceeds the freshness SLO, every profile is held until re-proven rather than shipping stale documentation maturity language.".to_owned(),
        },
        DocsClaimDowngradeRule {
            rule_id: "downgrade:greener_than_matrix:block".to_owned(),
            trigger: T::GreenerThanMatrix,
            action: A::BlockPublication,
            applies_to_profiles: vec![
                P::DocsBrowser,
                P::HelpAbout,
                P::OnboardingLearning,
                P::AiExplanation,
                P::SupportExport,
            ],
            applies_to_evidence_classes: DocsEvidenceClass::ALL.to_vec(),
            auto_enforced: true,
            rationale: "A profile drifting greener than the frozen docs-contracts matrix blocks publication; this packet is canonical and no profile may stay greener than the matrix.".to_owned(),
        },
    ]
}

fn seeded_trust_review() -> DocsClaimTrustReview {
    DocsClaimTrustReview {
        source_class_visible_no_vendor_masquerade: true,
        version_match_and_freshness_visible: true,
        citation_basis_preserved_derived_never_outlives_citations: true,
        browser_handoff_context_not_silently_shared: true,
        mirror_offline_state_visible: true,
        no_profile_greener_than_packet: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_partial_evidence_narrows_or_blocks_publication: true,
    }
}

fn seeded_consumer_projection() -> DocsClaimConsumerProjection {
    DocsClaimConsumerProjection {
        release_gate_consumes_packet: true,
        claim_publication_consumes_packet: true,
        about_help_service_health_consumes_packet: true,
        support_export_consumes_packet: true,
        onboarding_consumes_packet: true,
        ai_context_consumes_packet: true,
        narrowed_profiles_labeled_not_hidden: true,
    }
}

fn validate_source_contracts(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required = vec![
        DOCS_CLAIM_CERTIFICATION_SCHEMA_REF,
        DOCS_CLAIM_CERTIFICATION_DOC_REF,
        M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF,
        M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF,
    ];
    for class in DocsEvidenceClass::ALL {
        required.extend_from_slice(class.evidence_schema_refs());
    }
    for needed in required {
        if !refs.contains(needed) {
            violations.push(DocsClaimCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_evidence_corpus(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .evidence_corpus_refs
        .iter()
        .map(String::as_str)
        .collect();
    for class in DocsEvidenceClass::ALL {
        for needed in class.evidence_artifact_refs() {
            if !refs.contains(needed) {
                violations.push(DocsClaimCertificationViolation::MissingEvidenceCorpus);
                return;
            }
        }
    }
}

fn validate_profile_rows(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let present: BTreeSet<CertifiedDocsProfile> =
        packet.profile_rows.iter().map(|row| row.profile).collect();
    for required in CertifiedDocsProfile::ALL {
        if !present.contains(&required) {
            violations.push(DocsClaimCertificationViolation::RequiredProfileMissing);
            return;
        }
    }

    let mut covered: BTreeSet<DocsEvidenceClass> = BTreeSet::new();
    for row in &packet.profile_rows {
        covered.extend(row.evidence_classes.iter().copied());
    }
    for class in DocsEvidenceClass::ALL {
        if !covered.contains(&class) {
            violations.push(DocsClaimCertificationViolation::EvidenceClassUncovered);
            break;
        }
    }

    for row in &packet.profile_rows {
        if row.scope_summary.trim().is_empty()
            || row.evidence_classes.is_empty()
            || row.evidence_schema_refs.is_empty()
            || row.evidence_artifact_refs.is_empty()
            || !row.source_class_disclosed
        {
            violations.push(DocsClaimCertificationViolation::ProfileRowIncomplete);
        }

        let expected_schema: BTreeSet<&str> = row
            .evidence_classes
            .iter()
            .flat_map(|class| class.evidence_schema_refs().iter().copied())
            .collect();
        let actual_schema: BTreeSet<&str> = row
            .evidence_schema_refs
            .iter()
            .map(String::as_str)
            .collect();
        let expected_artifact: BTreeSet<&str> = row
            .evidence_classes
            .iter()
            .flat_map(|class| class.evidence_artifact_refs().iter().copied())
            .collect();
        let actual_artifact: BTreeSet<&str> = row
            .evidence_artifact_refs
            .iter()
            .map(String::as_str)
            .collect();
        if expected_schema != actual_schema || expected_artifact != actual_artifact {
            violations.push(DocsClaimCertificationViolation::EvidenceRefMismatch);
        }

        if row.is_promoted_and_certified() && row.evidence_packet_refs.is_empty() {
            violations.push(DocsClaimCertificationViolation::CertifiedProfileMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(DocsClaimCertificationViolation::DowngradeTriggersMissing);
        }
        if !row.not_greener_than_matrix {
            violations.push(DocsClaimCertificationViolation::ProfileGreenerThanMatrix);
        }
        if row
            .evidence_classes
            .contains(&DocsEvidenceClass::CitationSet)
            && !row.citation_basis_required
        {
            violations.push(DocsClaimCertificationViolation::CitationBasisMissing);
        }
        if row
            .evidence_classes
            .contains(&DocsEvidenceClass::BrowserHandoff)
            && !row.browser_handoff_context_isolated
        {
            violations.push(DocsClaimCertificationViolation::BrowserHandoffContextNotIsolated);
        }
        if row.verdict.permits_publication() != row.qualification.is_promoted() {
            violations.push(DocsClaimCertificationViolation::VerdictQualificationMismatch);
        }
    }
}

fn validate_compatibility_report(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let report = &packet.compatibility_report;
    let refs_ok = report.matrix_artifact_ref == M5_DOCS_CONTRACTS_MATRIX_ARTIFACT_REF
        && report.matrix_schema_ref == M5_DOCS_CONTRACTS_MATRIX_SCHEMA_REF
        && report.matrix_schema_version == M5_DOCS_CONTRACTS_MATRIX_SCHEMA_VERSION;
    let flags_ok = report.all_profiles_present
        && report.all_evidence_classes_covered
        && report.no_profile_greener_than_matrix
        && report.every_profile_has_evidence
        && report.downgrade_rules_auto_enforced;
    if !refs_ok || !flags_ok {
        violations.push(DocsClaimCertificationViolation::CompatibilityReportIncomplete);
    }
}

fn validate_downgrade_rules(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    if packet.downgrade_rules.is_empty() {
        violations.push(DocsClaimCertificationViolation::DowngradeRulesIncomplete);
        return;
    }
    for rule in &packet.downgrade_rules {
        if rule.rule_id.trim().is_empty()
            || rule.rationale.trim().is_empty()
            || rule.applies_to_profiles.is_empty()
            || rule.applies_to_evidence_classes.is_empty()
            || !rule.auto_enforced
        {
            violations.push(DocsClaimCertificationViolation::DowngradeRulesIncomplete);
            return;
        }
    }
    // Every evidence-class staleness trigger must have an auto-enforced rule so
    // stale evidence in any class narrows or blocks the affected profiles.
    let triggers: BTreeSet<DocsClaimDowngradeTrigger> = packet
        .downgrade_rules
        .iter()
        .map(|rule| rule.trigger)
        .collect();
    for class in DocsEvidenceClass::ALL {
        if !triggers.contains(&class.stale_trigger()) {
            violations.push(DocsClaimCertificationViolation::DowngradeRulesIncomplete);
            return;
        }
    }
}

fn validate_trust_review(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.source_class_visible_no_vendor_masquerade,
        review.version_match_and_freshness_visible,
        review.citation_basis_preserved_derived_never_outlives_citations,
        review.browser_handoff_context_not_silently_shared,
        review.mirror_offline_state_visible,
        review.no_profile_greener_than_packet,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_partial_evidence_narrows_or_blocks_publication,
    ] {
        if !ok {
            violations.push(DocsClaimCertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.release_gate_consumes_packet,
        projection.claim_publication_consumes_packet,
        projection.about_help_service_health_consumes_packet,
        projection.support_export_consumes_packet,
        projection.onboarding_consumes_packet,
        projection.ai_context_consumes_packet,
        projection.narrowed_profiles_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(DocsClaimCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &DocsClaimCertificationPacket,
    violations: &mut Vec<DocsClaimCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(DocsClaimCertificationViolation::ProofFreshnessIncomplete);
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
