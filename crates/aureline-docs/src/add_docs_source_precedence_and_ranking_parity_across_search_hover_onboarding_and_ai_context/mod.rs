//! Docs-source precedence and ranking parity across docs search, hover/peek,
//! onboarding, and AI context surfaces.
//!
//! When several documentation sources answer the same subject, Aureline must let
//! a reader tell *why* one answer outranked another and *what authority* the
//! winning answer actually carries. This module materializes the ranking-parity
//! truth packet that makes precedence explicit: each candidate keeps its
//! distinguishable [`DocsSourceLane`], its [`SourcePrecedenceClass`], a closed
//! [`PrecedenceReason`] with a human-readable note, its version-match and
//! freshness state, and the project-specific / override cues a surface shows —
//! so project docs, generated docs, mirrored official docs, curated knowledge
//! packs, extension-contributed docs, live external docs, and derived
//! explanations stay seven distinguishable lanes instead of one flattened list.
//!
//! [`DocsPrecedenceRankingPacket::materialize`] computes the validation findings
//! and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so a ranking that flattens a source lane,
//! ranks a less-authoritative source above a more-authoritative one without an
//! explicit reason, lets project docs outrank vendor docs without keeping the
//! vendor answer visible, lets a derived explanation claim primary authority,
//! drops the ranking explanation on a consumer surface, or hides a second
//! ranking model that ignores source-class / version-match / freshness truth,
//! narrows or blocks before it reaches a consumer surface.
//!
//! The lane enforces the precedence-honesty invariants the docs object model
//! requires:
//!
//! * **No flattening.** The seven source lanes — project, generated, mirrored
//!   official, curated knowledge pack, extension-contributed, live external, and
//!   derived — must each be representable and stay distinguishable; a candidate's
//!   declared lane must agree with its source class and trust class.
//! * **Explained ranking.** A candidate ranked above a more-authoritative source
//!   lane must carry a precedence reason that justifies the inversion, and a
//!   project-outranks-vendor candidate must keep at least one vendor / mirrored
//!   alternative visible in the same set and reference it.
//! * **One shared vocabulary.** Docs search, hover/peek, onboarding, AI context,
//!   and support-export surfaces project the same ranking explanation using one
//!   stable vocabulary; no surface may mint a hidden ranking model that ignores
//!   source class, version match, or freshness.
//! * **Honest offline truth.** An offline / air-gapped profile keeps a candidate
//!   inspectable with an explicit unavailable reason rather than silently
//!   dropping it or substituting generic web search.
//!
//! The packet is an inspectable, serde-serializable truth packet: it carries no
//! raw document bodies, raw source files, raw URLs, raw provider payloads, or
//! credentials — only metadata, opaque refs, the controlled precedence / lane /
//! reason vocabulary, and contract refs. It reuses the canonical source-class,
//! trust-class, version-match, freshness, mirror/offline, and source-precedence
//! vocabularies already owned by this crate rather than minting parallel tokens.
//!
//! The boundary schema is
//! [`schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json`](../../../../schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json).
//! The contract doc is
//! [`docs/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md`](../../../../docs/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/`](../../../../fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    CitationSourceClass, DocsFreshnessClass, DocsMirrorOfflinePosture, DocsObjectTrustClass,
    SourcePrecedenceClass, VersionMatchState,
};

/// Stable record-kind tag carried by [`DocsPrecedenceRankingPacket`].
pub const DOCS_PRECEDENCE_RANKING_RECORD_KIND: &str =
    "docs_source_precedence_and_ranking_parity_packet";

/// Stable record-kind tag carried by [`DocsPrecedenceRankingSupportExport`].
pub const DOCS_PRECEDENCE_RANKING_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_source_precedence_and_ranking_parity_support_export";

/// Schema version for docs-source precedence/ranking records.
pub const DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_PRECEDENCE_RANKING_SCHEMA_REF: &str =
    "schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json";

/// Repo-relative path of the contract doc.
pub const DOCS_PRECEDENCE_RANKING_DOC_REF: &str =
    "docs/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_PRECEDENCE_RANKING_ARTIFACT_REF: &str =
    "artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_PRECEDENCE_RANKING_SUMMARY_REF: &str =
    "artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_PRECEDENCE_RANKING_FIXTURE_DIR: &str =
    "fixtures/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context";

/// Repo-relative path of the docs-source/result reuse contract this lane ranks over.
pub const DOCS_PRECEDENCE_RANKING_SOURCE_RESULT_CONTRACT_REF: &str =
    "schemas/docs/stable-docs-source-and-result-object-reuse-across-consumer-surfaces.schema.json";

/// Repo-relative path of the frozen docs-contracts matrix the lane stays aligned with.
pub const DOCS_PRECEDENCE_RANKING_MATRIX_CONTRACT_REF: &str =
    "schemas/docs/freeze-the-m5-docs-source-result-pack-version-match-citation-set-and-browser-handoff-matrix.schema.json";

/// One of the seven distinguishable documentation source lanes.
///
/// Lanes are derived from a source's [`CitationSourceClass`] and
/// [`DocsObjectTrustClass`] so precedence ranking never flattens the source
/// classes the docs object model keeps distinct. The curated-knowledge-pack
/// source class splits into the [`Self::CuratedKnowledgePack`] and
/// [`Self::ExtensionContributedDocs`] lanes by trust class, which is the only
/// way extension-contributed docs stay distinguishable from a first-party
/// curated pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsSourceLane {
    /// Workspace-owned project documentation.
    ProjectDocs,
    /// Generated reference bound to the running build.
    GeneratedDocs,
    /// Signed mirror of official vendor / framework / language docs.
    MirroredOfficialDocs,
    /// First-party curated knowledge pack.
    CuratedKnowledgePack,
    /// Extension-contributed docs signed by a verified publisher.
    ExtensionContributedDocs,
    /// Live external docs resolved through an explicit browser handoff.
    LiveExternalDocs,
    /// Derived explanation; never primary authority.
    DerivedExplanation,
}

impl DocsSourceLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ProjectDocs,
        Self::GeneratedDocs,
        Self::MirroredOfficialDocs,
        Self::CuratedKnowledgePack,
        Self::ExtensionContributedDocs,
        Self::LiveExternalDocs,
        Self::DerivedExplanation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::GeneratedDocs => "generated_docs",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::CuratedKnowledgePack => "curated_knowledge_pack",
            Self::ExtensionContributedDocs => "extension_contributed_docs",
            Self::LiveExternalDocs => "live_external_docs",
            Self::DerivedExplanation => "derived_explanation",
        }
    }

    /// Derives the lane from a source class and trust class.
    ///
    /// Returns [`None`] when the trust class is inadmissible for the source class
    /// (for example a project source labelled with a live-provider trust class),
    /// which is exactly how the validator catches project docs trying to
    /// masquerade as vendor docs.
    pub fn from_source_and_trust(
        source_class: CitationSourceClass,
        trust_class: DocsObjectTrustClass,
    ) -> Option<Self> {
        use CitationSourceClass as Sc;
        use DocsObjectTrustClass as Tc;
        match (source_class, trust_class) {
            (Sc::ProjectDocs, Tc::FirstPartyAuthoritative) => Some(Self::ProjectDocs),
            (Sc::GeneratedReference, Tc::FirstPartyAuthoritative) => Some(Self::GeneratedDocs),
            (Sc::MirroredOfficialDocs, Tc::SignedMirrorVerified) => {
                Some(Self::MirroredOfficialDocs)
            }
            (Sc::CuratedKnowledgePack, Tc::CuratedSupported) => Some(Self::CuratedKnowledgePack),
            (Sc::CuratedKnowledgePack, Tc::ExtensionPackSigned) => {
                Some(Self::ExtensionContributedDocs)
            }
            (Sc::VendorProviderDocs, Tc::LiveProviderHandoff) => Some(Self::LiveExternalDocs),
            (Sc::DerivedExplanation, Tc::DerivedInferenceOnly) => Some(Self::DerivedExplanation),
            _ => None,
        }
    }

    /// Default authority rank; lower means more inherently authoritative.
    ///
    /// This is the baseline a ranking is read against: ranking a higher number
    /// above a lower one is an inversion that needs an explicit, justifying
    /// precedence reason.
    pub const fn base_authority_rank(self) -> u8 {
        match self {
            Self::MirroredOfficialDocs => 0,
            Self::ProjectDocs => 1,
            Self::GeneratedDocs => 2,
            Self::CuratedKnowledgePack => 3,
            Self::ExtensionContributedDocs => 4,
            Self::LiveExternalDocs => 5,
            Self::DerivedExplanation => 6,
        }
    }

    /// Whether this lane is a vendor / upstream lane that project docs may
    /// outrank for a repo-specific question.
    pub const fn is_vendor_alternative(self) -> bool {
        matches!(self, Self::MirroredOfficialDocs | Self::LiveExternalDocs)
    }
}

/// Closed vocabulary for *why* a candidate carries the rank it does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceReason {
    /// Project docs answer a repo-specific question and outrank upstream docs.
    ProjectScopeMatch,
    /// The source exactly matches the active build / workspace revision.
    ExactVersionMatch,
    /// A fresher source is preferred over a staler one.
    FreshnessPreferred,
    /// Official upstream docs carry inherent reference authority.
    OfficialUpstreamAuthority,
    /// A curated knowledge pack is topically relevant.
    CuratedPackRelevance,
    /// Extension-contributed docs are scoped to an installed extension.
    ExtensionContributedScope,
    /// Live external docs are offered only as a last-resort handoff fallback.
    LiveExternalFallback,
    /// Vendor docs override project docs under an explicit policy or source rule.
    VendorOverridePolicy,
    /// Project and vendor docs disagree, so both are kept visible.
    DisagreementBothShown,
    /// Derived inference only; never primary authority.
    DerivedInferenceOnly,
}

impl PrecedenceReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProjectScopeMatch,
        Self::ExactVersionMatch,
        Self::FreshnessPreferred,
        Self::OfficialUpstreamAuthority,
        Self::CuratedPackRelevance,
        Self::ExtensionContributedScope,
        Self::LiveExternalFallback,
        Self::VendorOverridePolicy,
        Self::DisagreementBothShown,
        Self::DerivedInferenceOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectScopeMatch => "project_scope_match",
            Self::ExactVersionMatch => "exact_version_match",
            Self::FreshnessPreferred => "freshness_preferred",
            Self::OfficialUpstreamAuthority => "official_upstream_authority",
            Self::CuratedPackRelevance => "curated_pack_relevance",
            Self::ExtensionContributedScope => "extension_contributed_scope",
            Self::LiveExternalFallback => "live_external_fallback",
            Self::VendorOverridePolicy => "vendor_override_policy",
            Self::DisagreementBothShown => "disagreement_both_shown",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }

    /// Whether this reason can justify ranking a less-authoritative source above
    /// a more-authoritative one.
    pub const fn justifies_inversion(self) -> bool {
        matches!(
            self,
            Self::ProjectScopeMatch
                | Self::ExactVersionMatch
                | Self::FreshnessPreferred
                | Self::VendorOverridePolicy
                | Self::DisagreementBothShown
        )
    }

    /// Whether this reason is admissible for the declared precedence class.
    ///
    /// This keeps the closed reason vocabulary consistent with the
    /// project/vendor [`SourcePrecedenceClass`] so a candidate cannot claim, for
    /// example, a vendor-override reason while declaring a project-authoritative
    /// precedence class.
    pub fn is_admissible_for(self, precedence_class: SourcePrecedenceClass) -> bool {
        use SourcePrecedenceClass as Pc;
        match self {
            Self::ProjectScopeMatch => {
                matches!(
                    precedence_class,
                    Pc::ProjectAuthoritativeOnly | Pc::ProjectOutranksVendorDefault
                )
            }
            Self::ExactVersionMatch | Self::FreshnessPreferred => matches!(
                precedence_class,
                Pc::ProjectAuthoritativeOnly | Pc::ProjectOutranksVendorDefault | Pc::NotApplicable
            ),
            Self::OfficialUpstreamAuthority => matches!(
                precedence_class,
                Pc::NotApplicable | Pc::ProjectVendorDisagreementInspectable
            ),
            Self::CuratedPackRelevance | Self::ExtensionContributedScope => {
                matches!(
                    precedence_class,
                    Pc::NotApplicable | Pc::ProjectOutranksVendorDefault
                )
            }
            Self::LiveExternalFallback => {
                matches!(
                    precedence_class,
                    Pc::NotApplicable | Pc::VendorOverrideDisclosed
                )
            }
            Self::VendorOverridePolicy => matches!(precedence_class, Pc::VendorOverrideDisclosed),
            Self::DisagreementBothShown => {
                matches!(precedence_class, Pc::ProjectVendorDisagreementInspectable)
            }
            Self::DerivedInferenceOnly => matches!(precedence_class, Pc::NotApplicable),
        }
    }
}

/// What a ranking set answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankSubjectKind {
    /// A free-text docs/code search query.
    SearchQuery,
    /// A symbol the reader hovered or peeked.
    Symbol,
    /// A natural-language question asked of the assistant.
    Question,
    /// An onboarding / first-run topic.
    OnboardingTopic,
    /// An AI-context-assembly retrieval subject.
    AiContextSubject,
}

impl RankSubjectKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchQuery => "search_query",
            Self::Symbol => "symbol",
            Self::Question => "question",
            Self::OnboardingTopic => "onboarding_topic",
            Self::AiContextSubject => "ai_context_subject",
        }
    }
}

/// One ranked documentation candidate answering a subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankedDocsCandidate {
    /// Stable candidate id, unique within its ranking set.
    pub candidate_id: String,
    /// Ref to the docs-source descriptor that answered this candidate.
    pub docs_source_ref: String,
    /// Ref to the docs-result object this candidate ranks.
    pub result_ref: String,
    /// Canonical source class.
    pub source_class: CitationSourceClass,
    /// Trust class; must stay admissible for the source class.
    pub trust_class: DocsObjectTrustClass,
    /// Distinguishable lane; must equal the lane derived from class and trust.
    pub lane: DocsSourceLane,
    /// Project/vendor precedence class.
    pub precedence_class: SourcePrecedenceClass,
    /// Closed reason for this candidate's rank.
    pub precedence_reason: PrecedenceReason,
    /// Human-readable note explaining why this candidate ranks where it does.
    pub precedence_reason_note: String,
    /// Version-match state.
    pub version_match_state: VersionMatchState,
    /// Freshness state.
    pub freshness_state: DocsFreshnessClass,
    /// Mirror / offline posture.
    pub mirror_offline_posture: DocsMirrorOfflinePosture,
    /// 1-based rank position within the ranking set.
    pub rank_position: u32,
    /// True when the surface shows a project-specific cue for this candidate.
    pub project_specific_cue: bool,
    /// True when the surface shows an override cue for this candidate.
    pub override_cue: bool,
    /// Candidate ids this candidate outranks (for project-outranks-vendor visibility).
    #[serde(default)]
    pub outranks_refs: Vec<String>,
    /// True when this candidate stays inspectable in an offline / air-gapped profile.
    pub available_in_offline_profile: bool,
    /// Explicit reason the candidate is unavailable offline (required when not available).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unavailable_reason: Option<String>,
    /// Disclosure note for a derived, drifted, stale, override, or unavailable candidate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
    /// True when raw boundary material is excluded.
    pub raw_boundary_material_excluded: bool,
}

impl RankedDocsCandidate {
    fn is_well_formed(&self) -> bool {
        !self.candidate_id.trim().is_empty()
            && !self.docs_source_ref.trim().is_empty()
            && !self.result_ref.trim().is_empty()
            && self.rank_position >= 1
    }

    /// Whether the declared lane agrees with the source class and trust class.
    fn lane_resolves(&self) -> Option<DocsSourceLane> {
        DocsSourceLane::from_source_and_trust(self.source_class, self.trust_class)
    }

    fn is_derived(&self) -> bool {
        self.lane == DocsSourceLane::DerivedExplanation
    }

    fn has_disclosure(&self) -> bool {
        self.disclosure_note
            .as_deref()
            .map(|note| !note.trim().is_empty())
            .unwrap_or(false)
    }

    /// Whether this candidate must carry a disclosure note.
    fn requires_disclosure(&self) -> bool {
        self.is_derived()
            || self.version_match_state != VersionMatchState::ExactBuildMatch
            || self.freshness_state.lowers_certainty()
            || matches!(
                self.precedence_class,
                SourcePrecedenceClass::VendorOverrideDisclosed
                    | SourcePrecedenceClass::ProjectVendorDisagreementInspectable
                    | SourcePrecedenceClass::ProjectOutranksVendorDefault
            )
            || !self.available_in_offline_profile
    }

    /// Whether a derived candidate keeps its inference-only, never-primary posture.
    fn derived_posture_ok(&self) -> bool {
        self.rank_position != 1
            && self.precedence_class == SourcePrecedenceClass::NotApplicable
            && self.precedence_reason == PrecedenceReason::DerivedInferenceOnly
    }

    fn outranks_vendor_default(&self) -> bool {
        self.precedence_class == SourcePrecedenceClass::ProjectOutranksVendorDefault
    }
}

/// A ranked answer set for one subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsRankingSet {
    /// Stable subject id, unique within the packet.
    pub subject_id: String,
    /// Human-readable subject label.
    pub subject_label: String,
    /// What the subject is.
    pub subject_kind: RankSubjectKind,
    /// True when the subject is a repo-specific question (where project docs may
    /// outrank upstream docs).
    pub project_specific_subject: bool,
    /// Candidates, ordered by rank position.
    pub candidates: Vec<RankedDocsCandidate>,
    /// Disclosure note for the ranking set as a whole.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosure_note: Option<String>,
}

impl DocsRankingSet {
    fn is_well_formed(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && !self.subject_label.trim().is_empty()
            && !self.candidates.is_empty()
    }

    fn candidate(&self, candidate_id: &str) -> Option<&RankedDocsCandidate> {
        self.candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)
    }
}

/// A consumer surface that must project the ranking explanation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RankExplanationSurface {
    /// Docs / code search result list.
    DocsSearch,
    /// Hover / peek documentation popover.
    HoverPeek,
    /// Onboarding / first-run teaching surface.
    Onboarding,
    /// AI context-assembly surface.
    AiContext,
    /// Support bundle export.
    SupportExport,
}

impl RankExplanationSurface {
    /// Every required surface, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::DocsSearch,
        Self::HoverPeek,
        Self::Onboarding,
        Self::AiContext,
        Self::SupportExport,
    ];

    /// Surfaces whose projections must cover every ranking set so an export or a
    /// reopened history never silently drops a ranking.
    pub const FULL_COVERAGE: [Self; 1] = [Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSearch => "docs_search",
            Self::HoverPeek => "hover_peek",
            Self::Onboarding => "onboarding",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this surface must project every ranking set.
    pub fn requires_full_coverage(self) -> bool {
        Self::FULL_COVERAGE.contains(&self)
    }
}

/// One per-surface projection asserting the surface shows the shared ranking
/// explanation without minting a hidden ranking model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RankExplanationProjection {
    /// Consumer surface.
    pub surface: RankExplanationSurface,
    /// Stable projection id.
    pub projection_id: String,
    /// Ranking set (subject id) this projection explains.
    pub ranking_set_ref: String,
    /// True when the surface shows the source class / lane.
    pub shows_source_class: bool,
    /// True when the surface shows the precedence reason.
    pub shows_precedence_reason: bool,
    /// True when the surface shows the version-match state.
    pub shows_version_match: bool,
    /// True when the surface shows the freshness state.
    pub shows_freshness: bool,
    /// True when the surface shows the project-specific / override cue.
    pub shows_project_or_override_cue: bool,
    /// True when the surface reuses the shared ranking vocabulary instead of a
    /// hidden, parallel ranking model. Must stay true.
    pub uses_shared_ranking_vocabulary: bool,
    /// True when the ranking explanation is inspectable on demand.
    pub explanation_inspectable_on_demand: bool,
    /// Candidate ids this surface projects.
    pub candidate_id_refs: Vec<String>,
}

impl RankExplanationProjection {
    /// True when the projection preserves every shared-explanation flag.
    fn shows_required_truth(&self) -> bool {
        self.shows_source_class
            && self.shows_precedence_reason
            && self.shows_version_match
            && self.shows_freshness
            && self.shows_project_or_override_cue
            && self.explanation_inspectable_on_demand
    }
}

/// Derived promotion state of a ranking packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPrecedenceRankingPromotionState {
    /// All invariants hold; the packet certifies a clean stable claim.
    Stable,
    /// A non-fatal narrowing applies (an air-gapped candidate or a disclosed
    /// project/vendor disagreement); the claim is narrowed below stable.
    NarrowedBelowStable,
    /// A blocking invariant failed; the packet may not claim stable.
    BlocksStable,
}

impl DocsPrecedenceRankingPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPrecedenceRankingFindingSeverity {
    /// Informational finding.
    Info,
    /// Narrows the claim below stable.
    Warning,
    /// Blocks the stable claim.
    Blocker,
}

impl DocsPrecedenceRankingFindingSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Blocker => "blocker",
        }
    }
}

/// Closed set of validation finding kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsPrecedenceRankingFindingKind {
    /// Record kind does not match the contract.
    WrongRecordKind,
    /// Schema version does not match the contract.
    WrongSchemaVersion,
    /// Packet identity is incomplete.
    MissingPacketIdentity,
    /// Source contract refs omit the schema or contract doc.
    MissingSourceContracts,
    /// Packet declares no ranking sets.
    MissingRankingSets,
    /// A ranking set drops a required identity field or has no candidates.
    RankingSetIncomplete,
    /// A candidate drops a required identity field.
    CandidateIncomplete,
    /// A candidate's declared lane disagrees with its source class and trust class.
    CandidateLaneMismatch,
    /// A candidate's source class / trust class does not resolve to a lane
    /// (for example project docs labelled with a vendor trust class).
    CandidateLaneUnresolved,
    /// The seven distinguishable source lanes are not all represented.
    SourceClassDistinguishabilityMissing,
    /// Two candidates in a set share a rank position.
    DuplicateRankPosition,
    /// A candidate drops the note explaining its rank.
    PrecedenceReasonNoteMissing,
    /// A candidate's precedence reason is inadmissible for its precedence class.
    PrecedenceReasonClassMismatch,
    /// A less-authoritative source is ranked above a more-authoritative one
    /// without a justifying precedence reason.
    UnexplainedRankInversion,
    /// Project docs outrank vendor docs without keeping the vendor answer visible.
    OutrankWithoutVisibleAlternative,
    /// A derived explanation is ranked as primary authority.
    DerivedExplanationRankedAsPrimary,
    /// A required disclosure note is missing.
    BoundaryDisclosureMissing,
    /// A candidate is unavailable offline without an explicit reason.
    OfflineUnavailableReasonMissing,
    /// A required ranking-explanation surface has no projection.
    MissingRankExplanationSurface,
    /// A projection drops part of the shared ranking explanation.
    RankExplanationDropsTruth,
    /// A surface mints a hidden ranking model that ignores source-class truth.
    HiddenRankingModel,
    /// A projection references an unknown ranking set or candidate.
    ProjectionRefUnresolved,
    /// A full-coverage surface (support export) drops a ranking set.
    SupportExportDropsRankingSet,
    /// Raw boundary material is present in the export.
    RawBoundaryMaterialPresent,
    /// A candidate is honestly disclosed as unavailable offline and narrows.
    AirGappedCandidateNarrowed,
    /// Project and vendor docs disagree and both are shown, which narrows.
    ProjectVendorDisagreementNarrowed,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl DocsPrecedenceRankingFindingKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingRankingSets => "missing_ranking_sets",
            Self::RankingSetIncomplete => "ranking_set_incomplete",
            Self::CandidateIncomplete => "candidate_incomplete",
            Self::CandidateLaneMismatch => "candidate_lane_mismatch",
            Self::CandidateLaneUnresolved => "candidate_lane_unresolved",
            Self::SourceClassDistinguishabilityMissing => "source_class_distinguishability_missing",
            Self::DuplicateRankPosition => "duplicate_rank_position",
            Self::PrecedenceReasonNoteMissing => "precedence_reason_note_missing",
            Self::PrecedenceReasonClassMismatch => "precedence_reason_class_mismatch",
            Self::UnexplainedRankInversion => "unexplained_rank_inversion",
            Self::OutrankWithoutVisibleAlternative => "outrank_without_visible_alternative",
            Self::DerivedExplanationRankedAsPrimary => "derived_explanation_ranked_as_primary",
            Self::BoundaryDisclosureMissing => "boundary_disclosure_missing",
            Self::OfflineUnavailableReasonMissing => "offline_unavailable_reason_missing",
            Self::MissingRankExplanationSurface => "missing_rank_explanation_surface",
            Self::RankExplanationDropsTruth => "rank_explanation_drops_truth",
            Self::HiddenRankingModel => "hidden_ranking_model",
            Self::ProjectionRefUnresolved => "projection_ref_unresolved",
            Self::SupportExportDropsRankingSet => "support_export_drops_ranking_set",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::AirGappedCandidateNarrowed => "air_gapped_candidate_narrowed",
            Self::ProjectVendorDisagreementNarrowed => "project_vendor_disagreement_narrowed",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the ranking validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPrecedenceRankingValidationFinding {
    /// Closed finding kind.
    pub finding_kind: DocsPrecedenceRankingFindingKind,
    /// Finding severity.
    pub severity: DocsPrecedenceRankingFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl DocsPrecedenceRankingValidationFinding {
    fn blocker(finding_kind: DocsPrecedenceRankingFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: DocsPrecedenceRankingFindingSeverity::Blocker,
            summary: summary.into(),
        }
    }

    fn warning(finding_kind: DocsPrecedenceRankingFindingKind, summary: impl Into<String>) -> Self {
        Self {
            finding_kind,
            severity: DocsPrecedenceRankingFindingSeverity::Warning,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`DocsPrecedenceRankingPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPrecedenceRankingPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Ranked answer sets.
    pub ranking_sets: Vec<DocsRankingSet>,
    /// Per-surface projections.
    pub surface_projections: Vec<RankExplanationProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
}

/// Export-safe docs-source precedence/ranking parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPrecedenceRankingPacket {
    /// Record kind; must equal [`DOCS_PRECEDENCE_RANKING_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface or workflow label.
    pub surface_label: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Ranked answer sets.
    pub ranking_sets: Vec<DocsRankingSet>,
    /// Per-surface projections.
    pub surface_projections: Vec<RankExplanationProjection>,
    /// Source contract refs.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Derived promotion state.
    pub promotion_state: DocsPrecedenceRankingPromotionState,
    /// Validation findings.
    #[serde(default)]
    pub validation_findings: Vec<DocsPrecedenceRankingValidationFinding>,
}

impl DocsPrecedenceRankingPacket {
    /// Materializes the packet and records its derived findings and promotion state.
    pub fn materialize(input: DocsPrecedenceRankingPacketInput) -> Self {
        let mut packet = Self {
            record_kind: DOCS_PRECEDENCE_RANKING_RECORD_KIND.to_owned(),
            schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            generated_at: input.generated_at,
            ranking_sets: input.ranking_sets,
            surface_projections: input.surface_projections,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            promotion_state: DocsPrecedenceRankingPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet's invariants, including the stored promotion state.
    pub fn validate(&self) -> Vec<DocsPrecedenceRankingValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when no blocker validation findings exist.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == DocsPrecedenceRankingFindingSeverity::Blocker)
    }

    /// Returns true when the packet certifies the clean stable claim.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DocsPrecedenceRankingPromotionState::Stable
            && self.validate().is_empty()
    }

    /// Returns the distinguishable lanes present across the packet.
    pub fn lanes_present(&self) -> Vec<DocsSourceLane> {
        let mut set = BTreeSet::new();
        for ranking_set in &self.ranking_sets {
            for candidate in &ranking_set.candidates {
                set.insert(candidate.lane);
            }
        }
        set.into_iter().collect()
    }

    /// Returns the consumer surfaces with at least one projection.
    pub fn covered_surfaces(&self) -> Vec<RankExplanationSurface> {
        let mut set = BTreeSet::new();
        for projection in &self.surface_projections {
            set.insert(projection.surface);
        }
        set.into_iter().collect()
    }

    /// Returns the ranking set with the given subject id, if present.
    pub fn ranking_set(&self, subject_id: &str) -> Option<&DocsRankingSet> {
        self.ranking_sets
            .iter()
            .find(|set| set.subject_id == subject_id)
    }

    /// Wraps the packet in an export-safe support export.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> DocsPrecedenceRankingSupportExport {
        DocsPrecedenceRankingSupportExport {
            record_kind: DOCS_PRECEDENCE_RANKING_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ranking_explanation_preserved: true,
            export_packet: self.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs precedence ranking packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs-Source Precedence and Ranking Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} validation findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Ranking sets: {} / Projections: {}\n",
            self.ranking_sets.len(),
            self.surface_projections.len()
        ));
        out.push_str("\n## Ranking sets\n\n");
        for ranking_set in &self.ranking_sets {
            out.push_str(&format!(
                "- **{}** (`{}`, {})\n",
                ranking_set.subject_label,
                ranking_set.subject_id,
                ranking_set.subject_kind.as_str(),
            ));
            let mut ordered: Vec<&RankedDocsCandidate> = ranking_set.candidates.iter().collect();
            ordered.sort_by_key(|candidate| candidate.rank_position);
            for candidate in ordered {
                out.push_str(&format!(
                    "   {}. `{}` — reason `{}`, precedence `{}` ({}, {})\n",
                    candidate.rank_position,
                    candidate.lane.as_str(),
                    candidate.precedence_reason.as_str(),
                    candidate.precedence_class.as_str(),
                    candidate.version_match_state.as_str(),
                    candidate.freshness_state.as_str(),
                ));
            }
        }
        out.push_str("\n## Surfaces\n\n");
        for surface in RankExplanationSurface::REQUIRED {
            let count = self
                .surface_projections
                .iter()
                .filter(|projection| projection.surface == surface)
                .count();
            out.push_str(&format!(
                "- `{}`: {} projection(s)\n",
                surface.as_str(),
                count
            ));
        }
        out
    }

    fn derived_findings(
        &self,
        check_promotion: bool,
    ) -> Vec<DocsPrecedenceRankingValidationFinding> {
        let mut findings = Vec::new();

        if self.record_kind != DOCS_PRECEDENCE_RANKING_RECORD_KIND {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::WrongRecordKind,
                "record kind does not match the docs precedence/ranking contract",
            ));
        }
        if self.schema_version != DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::WrongSchemaVersion,
                "schema version does not match the docs precedence/ranking contract",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::MissingPacketIdentity,
                "packet identity is incomplete",
            ));
        }

        self.validate_source_contracts(&mut findings);
        self.validate_ranking_sets(&mut findings);
        self.validate_distinguishability(&mut findings);
        self.validate_projections(&mut findings);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("docs precedence ranking packet serializes"),
        ) {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::RawBoundaryMaterialPresent,
                "export contains forbidden raw boundary material",
            ));
        }

        if check_promotion {
            let derived = promotion_state_for(&findings);
            if self.promotion_state != derived {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::PromotionStateMismatch,
                    "stored promotion state disagrees with derived findings",
                ));
            }
        }

        findings
    }

    fn validate_source_contracts(
        &self,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        let refs: BTreeSet<&str> = self
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(DOCS_PRECEDENCE_RANKING_SCHEMA_REF)
            || !refs.contains(DOCS_PRECEDENCE_RANKING_DOC_REF)
        {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::MissingSourceContracts,
                "source contract refs omit the schema or contract doc",
            ));
        }
    }

    fn validate_ranking_sets(&self, findings: &mut Vec<DocsPrecedenceRankingValidationFinding>) {
        if self.ranking_sets.is_empty() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::MissingRankingSets,
                "packet must declare at least one ranking set",
            ));
        }
        for ranking_set in &self.ranking_sets {
            self.validate_one_ranking_set(ranking_set, findings);
        }
    }

    fn validate_one_ranking_set(
        &self,
        ranking_set: &DocsRankingSet,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        if !ranking_set.is_well_formed() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::RankingSetIncomplete,
                format!("ranking set {} is incomplete", ranking_set.subject_id),
            ));
        }

        let mut seen_ranks: BTreeSet<u32> = BTreeSet::new();
        for candidate in &ranking_set.candidates {
            if candidate.rank_position >= 1 && !seen_ranks.insert(candidate.rank_position) {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::DuplicateRankPosition,
                    format!(
                        "ranking set {} has two candidates at rank {}",
                        ranking_set.subject_id, candidate.rank_position
                    ),
                ));
            }
            self.validate_candidate(ranking_set, candidate, findings);
        }

        self.validate_rank_order(ranking_set, findings);
    }

    fn validate_candidate(
        &self,
        ranking_set: &DocsRankingSet,
        candidate: &RankedDocsCandidate,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        if !candidate.is_well_formed() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::CandidateIncomplete,
                format!("candidate {} is incomplete", candidate.candidate_id),
            ));
        }
        if !candidate.raw_boundary_material_excluded {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::RawBoundaryMaterialPresent,
                format!(
                    "candidate {} retains raw boundary material",
                    candidate.candidate_id
                ),
            ));
        }

        match candidate.lane_resolves() {
            None => findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::CandidateLaneUnresolved,
                format!(
                    "candidate {} labels {} docs with trust class {} so it has no distinguishable lane",
                    candidate.candidate_id,
                    candidate.source_class.as_str(),
                    candidate.trust_class.as_str()
                ),
            )),
            Some(lane) if lane != candidate.lane => {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::CandidateLaneMismatch,
                    format!(
                        "candidate {} declares lane {} but its source/trust resolves to {}",
                        candidate.candidate_id,
                        candidate.lane.as_str(),
                        lane.as_str()
                    ),
                ));
            }
            Some(_) => {}
        }

        if candidate.precedence_reason_note.trim().is_empty() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::PrecedenceReasonNoteMissing,
                format!(
                    "candidate {} must explain why it carries its rank",
                    candidate.candidate_id
                ),
            ));
        }

        if !candidate
            .precedence_reason
            .is_admissible_for(candidate.precedence_class)
        {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::PrecedenceReasonClassMismatch,
                format!(
                    "candidate {} reason {} is inadmissible for precedence class {}",
                    candidate.candidate_id,
                    candidate.precedence_reason.as_str(),
                    candidate.precedence_class.as_str()
                ),
            ));
        }

        if candidate.is_derived() && !candidate.derived_posture_ok() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::DerivedExplanationRankedAsPrimary,
                format!(
                    "candidate {} is a derived explanation ranked as primary authority",
                    candidate.candidate_id
                ),
            ));
        }

        if candidate.outranks_vendor_default() {
            self.validate_outrank_visibility(ranking_set, candidate, findings);
        }

        if candidate.requires_disclosure() && !candidate.has_disclosure() {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::BoundaryDisclosureMissing,
                format!(
                    "candidate {} omits a required disclosure note",
                    candidate.candidate_id
                ),
            ));
        }

        if !candidate.available_in_offline_profile {
            match candidate.unavailable_reason.as_deref() {
                Some(reason) if !reason.trim().is_empty() => {
                    findings.push(DocsPrecedenceRankingValidationFinding::warning(
                        DocsPrecedenceRankingFindingKind::AirGappedCandidateNarrowed,
                        format!(
                            "candidate {} is unavailable in an offline profile but discloses why; narrows below stable",
                            candidate.candidate_id
                        ),
                    ));
                }
                _ => findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::OfflineUnavailableReasonMissing,
                    format!(
                        "candidate {} is unavailable offline without an explicit reason",
                        candidate.candidate_id
                    ),
                )),
            }
        }

        if candidate.precedence_class == SourcePrecedenceClass::ProjectVendorDisagreementInspectable
        {
            findings.push(DocsPrecedenceRankingValidationFinding::warning(
                DocsPrecedenceRankingFindingKind::ProjectVendorDisagreementNarrowed,
                format!(
                    "candidate {} surfaces a project/vendor disagreement; both shown, narrows below stable",
                    candidate.candidate_id
                ),
            ));
        }
    }

    fn validate_outrank_visibility(
        &self,
        ranking_set: &DocsRankingSet,
        candidate: &RankedDocsCandidate,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        let vendor_present = ranking_set
            .candidates
            .iter()
            .any(|other| other.lane.is_vendor_alternative());
        let references_visible_vendor = candidate.outranks_refs.iter().any(|outranked_ref| {
            ranking_set
                .candidate(outranked_ref)
                .map(|other| other.lane.is_vendor_alternative())
                .unwrap_or(false)
        });
        if !vendor_present || !references_visible_vendor {
            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                DocsPrecedenceRankingFindingKind::OutrankWithoutVisibleAlternative,
                format!(
                    "candidate {} outranks vendor docs but keeps no visible vendor/mirrored alternative",
                    candidate.candidate_id
                ),
            ));
        }
    }

    fn validate_rank_order(
        &self,
        ranking_set: &DocsRankingSet,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        for higher in &ranking_set.candidates {
            for lower in &ranking_set.candidates {
                if higher.candidate_id == lower.candidate_id {
                    continue;
                }
                // `higher` is ranked above `lower` (smaller rank position) yet its
                // lane is less authoritative than `lower`'s lane: an inversion.
                let inverted = higher.rank_position < lower.rank_position
                    && higher.lane.base_authority_rank() > lower.lane.base_authority_rank();
                if inverted && !higher.precedence_reason.justifies_inversion() {
                    findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                        DocsPrecedenceRankingFindingKind::UnexplainedRankInversion,
                        format!(
                            "candidate {} ({}) outranks more-authoritative {} ({}) without a justifying reason",
                            higher.candidate_id,
                            higher.lane.as_str(),
                            lower.candidate_id,
                            lower.lane.as_str()
                        ),
                    ));
                    return;
                }
            }
        }
    }

    fn validate_distinguishability(
        &self,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        let present: HashSet<DocsSourceLane> = self
            .ranking_sets
            .iter()
            .flat_map(|set| set.candidates.iter())
            .map(|candidate| candidate.lane)
            .collect();
        for required in DocsSourceLane::ALL {
            if !present.contains(&required) {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::SourceClassDistinguishabilityMissing,
                    format!(
                        "no candidate represents the {} lane so it cannot stay distinguishable",
                        required.as_str()
                    ),
                ));
                return;
            }
        }
    }

    fn validate_projections(&self, findings: &mut Vec<DocsPrecedenceRankingValidationFinding>) {
        let present: BTreeSet<RankExplanationSurface> = self
            .surface_projections
            .iter()
            .map(|projection| projection.surface)
            .collect();
        for required in RankExplanationSurface::REQUIRED {
            if !present.contains(&required) {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::MissingRankExplanationSurface,
                    format!(
                        "no projection explains the ranking on the {} surface",
                        required.as_str()
                    ),
                ));
                break;
            }
        }

        let set_by_id: BTreeMap<&str, &DocsRankingSet> = self
            .ranking_sets
            .iter()
            .map(|set| (set.subject_id.as_str(), set))
            .collect();

        for projection in &self.surface_projections {
            if !projection.uses_shared_ranking_vocabulary {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::HiddenRankingModel,
                    format!(
                        "surface {} mints a hidden ranking model instead of the shared vocabulary",
                        projection.surface.as_str()
                    ),
                ));
            }
            if !projection.shows_required_truth() {
                findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::RankExplanationDropsTruth,
                    format!(
                        "surface {} drops part of the shared ranking explanation",
                        projection.surface.as_str()
                    ),
                ));
            }
            match set_by_id.get(projection.ranking_set_ref.as_str()) {
                None => findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                    DocsPrecedenceRankingFindingKind::ProjectionRefUnresolved,
                    format!(
                        "surface {} references unknown ranking set {}",
                        projection.surface.as_str(),
                        projection.ranking_set_ref
                    ),
                )),
                Some(ranking_set) => {
                    if projection.candidate_id_refs.is_empty() {
                        findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                            DocsPrecedenceRankingFindingKind::RankExplanationDropsTruth,
                            format!(
                                "surface {} projects no candidates for ranking set {}",
                                projection.surface.as_str(),
                                projection.ranking_set_ref
                            ),
                        ));
                    }
                    for candidate_ref in &projection.candidate_id_refs {
                        if ranking_set.candidate(candidate_ref).is_none() {
                            findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                                DocsPrecedenceRankingFindingKind::ProjectionRefUnresolved,
                                format!(
                                    "surface {} references unknown candidate {}",
                                    projection.surface.as_str(),
                                    candidate_ref
                                ),
                            ));
                        }
                    }
                }
            }
        }

        self.validate_full_coverage(&set_by_id, findings);
    }

    fn validate_full_coverage(
        &self,
        set_by_id: &BTreeMap<&str, &DocsRankingSet>,
        findings: &mut Vec<DocsPrecedenceRankingValidationFinding>,
    ) {
        for surface in RankExplanationSurface::FULL_COVERAGE {
            let covered: BTreeSet<&str> = self
                .surface_projections
                .iter()
                .filter(|projection| projection.surface == surface)
                .map(|projection| projection.ranking_set_ref.as_str())
                .collect();
            for subject_id in set_by_id.keys() {
                if !covered.contains(subject_id) {
                    findings.push(DocsPrecedenceRankingValidationFinding::blocker(
                        DocsPrecedenceRankingFindingKind::SupportExportDropsRankingSet,
                        format!(
                            "surface {} drops ranking set {} from its full-coverage projection",
                            surface.as_str(),
                            subject_id
                        ),
                    ));
                    break;
                }
            }
        }
    }
}

/// Support-export wrapper preserving the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsPrecedenceRankingSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Exported packet id.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when the ranking explanation is preserved across the export boundary.
    pub ranking_explanation_preserved: bool,
    /// Exact packet preserved by the export.
    pub export_packet: DocsPrecedenceRankingPacket,
}

impl DocsPrecedenceRankingSupportExport {
    /// Returns true when the export preserves the same packet safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == DOCS_PRECEDENCE_RANKING_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == DOCS_PRECEDENCE_RANKING_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ranking_explanation_preserved
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted while reading the checked-in ranking export.
#[derive(Debug)]
pub enum DocsPrecedenceRankingArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export's packet failed validation.
    Validation(Vec<DocsPrecedenceRankingValidationFinding>),
    /// Support export wrapper is not export-safe.
    NotExportSafe,
}

impl fmt::Display for DocsPrecedenceRankingArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs precedence ranking export parse failed: {error}"
                )
            }
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs precedence ranking export failed validation: {tokens}"
                )
            }
            Self::NotExportSafe => {
                write!(
                    formatter,
                    "docs precedence ranking export wrapper is not export-safe"
                )
            }
        }
    }
}

impl Error for DocsPrecedenceRankingArtifactError {}

/// Returns the seeded stable ranking packet input.
pub fn seeded_stable_docs_precedence_ranking_input() -> DocsPrecedenceRankingPacketInput {
    seed::seeded_input()
}

/// Materializes the checked-in stable ranking packet.
///
/// # Errors
///
/// Returns an error when the seeded packet fails its own stable invariants.
pub fn current_stable_docs_precedence_ranking_packet(
) -> Result<DocsPrecedenceRankingPacket, DocsPrecedenceRankingArtifactError> {
    let packet =
        DocsPrecedenceRankingPacket::materialize(seeded_stable_docs_precedence_ranking_input());
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(DocsPrecedenceRankingArtifactError::Validation(findings))
    }
}

/// Reads and validates the checked-in stable support export.
///
/// # Errors
///
/// Returns an error when the checked artifact fails to parse, is not
/// export-safe, or its packet fails validation.
pub fn current_stable_docs_precedence_ranking_export(
) -> Result<DocsPrecedenceRankingSupportExport, DocsPrecedenceRankingArtifactError> {
    let export: DocsPrecedenceRankingSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/add_docs_source_precedence_and_ranking_parity_across_search_hover_onboarding_and_ai_context/support_export.json"
    )))
    .map_err(DocsPrecedenceRankingArtifactError::SupportExport)?;
    let findings = export.export_packet.validate();
    if !findings.is_empty() {
        return Err(DocsPrecedenceRankingArtifactError::Validation(findings));
    }
    if !export.is_export_safe() {
        return Err(DocsPrecedenceRankingArtifactError::NotExportSafe);
    }
    Ok(export)
}

fn promotion_state_for(
    findings: &[DocsPrecedenceRankingValidationFinding],
) -> DocsPrecedenceRankingPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == DocsPrecedenceRankingFindingSeverity::Blocker)
    {
        DocsPrecedenceRankingPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == DocsPrecedenceRankingFindingSeverity::Warning)
    {
        DocsPrecedenceRankingPromotionState::NarrowedBelowStable
    } else {
        DocsPrecedenceRankingPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => {
            let lower = text.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_url:")
                || lower.contains("raw_body:")
        }
        serde_json::Value::Array(items) => {
            items.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

mod seed {
    use super::*;

    pub(super) const PACKET_ID: &str = "packet:docs_source_precedence_and_ranking_parity:001";

    pub(super) const REPO_SUBJECT_ID: &str = "subject:repo-specific:how-do-we-configure-runtime";
    pub(super) const LIBRARY_SUBJECT_ID: &str = "subject:library:vec-push-semantics";

    fn project_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:repo:project-runbook".to_owned(),
            docs_source_ref: "docs-source:project-readme".to_owned(),
            result_ref: "docs-result:project-overview".to_owned(),
            source_class: CitationSourceClass::ProjectDocs,
            trust_class: DocsObjectTrustClass::FirstPartyAuthoritative,
            lane: DocsSourceLane::ProjectDocs,
            precedence_class: SourcePrecedenceClass::ProjectOutranksVendorDefault,
            precedence_reason: PrecedenceReason::ProjectScopeMatch,
            precedence_reason_note:
                "Project docs answer this repo-specific runtime question and outrank upstream docs."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::AuthoritativeLive,
            mirror_offline_posture: DocsMirrorOfflinePosture::LocalProjectPack,
            rank_position: 1,
            project_specific_cue: true,
            override_cue: false,
            outranks_refs: vec![
                "candidate:repo:mirror-std".to_owned(),
                "candidate:repo:vendor-live".to_owned(),
            ],
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: Some(
                "Project docs outrank upstream docs here because the question is repo-specific; upstream answers stay visible below.".to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn mirror_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:repo:mirror-std".to_owned(),
            docs_source_ref: "docs-source:mirror-std".to_owned(),
            result_ref: "docs-result:std-fn".to_owned(),
            source_class: CitationSourceClass::MirroredOfficialDocs,
            trust_class: DocsObjectTrustClass::SignedMirrorVerified,
            lane: DocsSourceLane::MirroredOfficialDocs,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::OfficialUpstreamAuthority,
            precedence_reason_note:
                "Mirrored official docs carry upstream reference authority and stay visible under the project answer."
                    .to_owned(),
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            freshness_state: DocsFreshnessClass::WarmCached,
            mirror_offline_posture: DocsMirrorOfflinePosture::MirroredPack,
            rank_position: 2,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: Some(
                "Signed mirror is a compatible minor drift from the active build.".to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn generated_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:repo:generated-ref".to_owned(),
            docs_source_ref: "docs-source:generated-ref".to_owned(),
            result_ref: "docs-result:generated-symbol".to_owned(),
            source_class: CitationSourceClass::GeneratedReference,
            trust_class: DocsObjectTrustClass::FirstPartyAuthoritative,
            lane: DocsSourceLane::GeneratedDocs,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::ExactVersionMatch,
            precedence_reason_note:
                "Generated reference is bound to the running build and matches it exactly."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::AuthoritativeLive,
            mirror_offline_posture: DocsMirrorOfflinePosture::GeneratedLocal,
            rank_position: 3,
            project_specific_cue: true,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn live_external_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:repo:vendor-live".to_owned(),
            docs_source_ref: "docs-source:vendor-live-api".to_owned(),
            result_ref: "docs-result:vendor-endpoint".to_owned(),
            source_class: CitationSourceClass::VendorProviderDocs,
            trust_class: DocsObjectTrustClass::LiveProviderHandoff,
            lane: DocsSourceLane::LiveExternalDocs,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::LiveExternalFallback,
            precedence_reason_note:
                "Live external docs are offered only as a last-resort handoff for unmirrored detail."
                    .to_owned(),
            version_match_state: VersionMatchState::UnknownTargetBuild,
            freshness_state: DocsFreshnessClass::AuthoritativeLive,
            mirror_offline_posture: DocsMirrorOfflinePosture::LiveOnline,
            rank_position: 4,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: Some(
                "Live external docs open through an explicit, isolated browser handoff.".to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn derived_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:repo:derived-summary".to_owned(),
            docs_source_ref: "docs-source:derived-explanation".to_owned(),
            result_ref: "docs-result:derived-summary".to_owned(),
            source_class: CitationSourceClass::DerivedExplanation,
            trust_class: DocsObjectTrustClass::DerivedInferenceOnly,
            lane: DocsSourceLane::DerivedExplanation,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::DerivedInferenceOnly,
            precedence_reason_note:
                "Derived summary cites the answers above but never claims primary authority."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::AuthoritativeLive,
            mirror_offline_posture: DocsMirrorOfflinePosture::GeneratedLocal,
            rank_position: 5,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: Some(
                "Derived explanation; never primary authority and bound to its citations."
                    .to_owned(),
            ),
            raw_boundary_material_excluded: true,
        }
    }

    fn library_mirror_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:library:mirror-std".to_owned(),
            docs_source_ref: "docs-source:mirror-std".to_owned(),
            result_ref: "docs-result:std-fn".to_owned(),
            source_class: CitationSourceClass::MirroredOfficialDocs,
            trust_class: DocsObjectTrustClass::SignedMirrorVerified,
            lane: DocsSourceLane::MirroredOfficialDocs,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::OfficialUpstreamAuthority,
            precedence_reason_note:
                "For a general library question, mirrored official docs are the upstream authority."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::WarmCached,
            mirror_offline_posture: DocsMirrorOfflinePosture::MirroredPack,
            rank_position: 1,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn curated_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:library:curated-cookbook".to_owned(),
            docs_source_ref: "docs-source:curated-cookbook".to_owned(),
            result_ref: "docs-result:cookbook-recipe".to_owned(),
            source_class: CitationSourceClass::CuratedKnowledgePack,
            trust_class: DocsObjectTrustClass::CuratedSupported,
            lane: DocsSourceLane::CuratedKnowledgePack,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::CuratedPackRelevance,
            precedence_reason_note:
                "Curated knowledge pack is topically relevant but ranks below upstream reference."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::WarmCached,
            mirror_offline_posture: DocsMirrorOfflinePosture::OfflinePinnedPack,
            rank_position: 2,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn extension_candidate() -> RankedDocsCandidate {
        RankedDocsCandidate {
            candidate_id: "candidate:library:extension-pack".to_owned(),
            docs_source_ref: "docs-source:ext-pack-cookbook".to_owned(),
            result_ref: "docs-result:extension-recipe".to_owned(),
            source_class: CitationSourceClass::CuratedKnowledgePack,
            trust_class: DocsObjectTrustClass::ExtensionPackSigned,
            lane: DocsSourceLane::ExtensionContributedDocs,
            precedence_class: SourcePrecedenceClass::NotApplicable,
            precedence_reason: PrecedenceReason::ExtensionContributedScope,
            precedence_reason_note:
                "Extension-contributed docs are scoped to an installed extension and rank last."
                    .to_owned(),
            version_match_state: VersionMatchState::ExactBuildMatch,
            freshness_state: DocsFreshnessClass::WarmCached,
            mirror_offline_posture: DocsMirrorOfflinePosture::OfflinePinnedPack,
            rank_position: 3,
            project_specific_cue: false,
            override_cue: false,
            outranks_refs: Vec::new(),
            available_in_offline_profile: true,
            unavailable_reason: None,
            disclosure_note: None,
            raw_boundary_material_excluded: true,
        }
    }

    fn repo_ranking_set() -> DocsRankingSet {
        DocsRankingSet {
            subject_id: REPO_SUBJECT_ID.to_owned(),
            subject_label: "How do we configure the runtime?".to_owned(),
            subject_kind: RankSubjectKind::Question,
            project_specific_subject: true,
            candidates: vec![
                project_candidate(),
                mirror_candidate(),
                generated_candidate(),
                live_external_candidate(),
                derived_candidate(),
            ],
            disclosure_note: Some(
                "Repo-specific question: project docs lead, but every upstream answer stays visible and labelled.".to_owned(),
            ),
        }
    }

    fn library_ranking_set() -> DocsRankingSet {
        DocsRankingSet {
            subject_id: LIBRARY_SUBJECT_ID.to_owned(),
            subject_label: "What are the semantics of Vec::push?".to_owned(),
            subject_kind: RankSubjectKind::Symbol,
            project_specific_subject: false,
            candidates: vec![
                library_mirror_candidate(),
                curated_candidate(),
                extension_candidate(),
            ],
            disclosure_note: None,
        }
    }

    fn ranking_sets() -> Vec<DocsRankingSet> {
        vec![repo_ranking_set(), library_ranking_set()]
    }

    fn projection(
        surface: RankExplanationSurface,
        ranking_set_ref: &str,
        candidate_id_refs: Vec<String>,
    ) -> RankExplanationProjection {
        RankExplanationProjection {
            surface,
            projection_id: format!("projection:{}:{}", surface.as_str(), ranking_set_ref),
            ranking_set_ref: ranking_set_ref.to_owned(),
            shows_source_class: true,
            shows_precedence_reason: true,
            shows_version_match: true,
            shows_freshness: true,
            shows_project_or_override_cue: true,
            uses_shared_ranking_vocabulary: true,
            explanation_inspectable_on_demand: true,
            candidate_id_refs,
        }
    }

    fn repo_candidate_ids() -> Vec<String> {
        repo_ranking_set()
            .candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect()
    }

    fn library_candidate_ids() -> Vec<String> {
        library_ranking_set()
            .candidates
            .iter()
            .map(|candidate| candidate.candidate_id.clone())
            .collect()
    }

    fn projections() -> Vec<RankExplanationProjection> {
        vec![
            // Docs search shows the full repo ranking with reasons.
            projection(
                RankExplanationSurface::DocsSearch,
                REPO_SUBJECT_ID,
                repo_candidate_ids(),
            ),
            // Hover/peek explains the library symbol ranking.
            projection(
                RankExplanationSurface::HoverPeek,
                LIBRARY_SUBJECT_ID,
                library_candidate_ids(),
            ),
            // Onboarding teaches the repo-specific precedence.
            projection(
                RankExplanationSurface::Onboarding,
                REPO_SUBJECT_ID,
                repo_candidate_ids(),
            ),
            // AI context assembly reuses the same ranking truth.
            projection(
                RankExplanationSurface::AiContext,
                REPO_SUBJECT_ID,
                repo_candidate_ids(),
            ),
            // Support export reconstructs every ranking set (full coverage).
            projection(
                RankExplanationSurface::SupportExport,
                REPO_SUBJECT_ID,
                repo_candidate_ids(),
            ),
            projection(
                RankExplanationSurface::SupportExport,
                LIBRARY_SUBJECT_ID,
                library_candidate_ids(),
            ),
        ]
    }

    pub(super) fn seeded_input() -> DocsPrecedenceRankingPacketInput {
        DocsPrecedenceRankingPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label:
                "workflow:docs_source_precedence_and_ranking_parity:search_hover_onboarding_ai:stable"
                    .to_owned(),
            generated_at: "2026-06-26T00:00:00Z".to_owned(),
            ranking_sets: ranking_sets(),
            surface_projections: projections(),
            source_contract_refs: vec![
                DOCS_PRECEDENCE_RANKING_SCHEMA_REF.to_owned(),
                DOCS_PRECEDENCE_RANKING_DOC_REF.to_owned(),
                DOCS_PRECEDENCE_RANKING_ARTIFACT_REF.to_owned(),
                DOCS_PRECEDENCE_RANKING_SUMMARY_REF.to_owned(),
                DOCS_PRECEDENCE_RANKING_SOURCE_RESULT_CONTRACT_REF.to_owned(),
                DOCS_PRECEDENCE_RANKING_MATRIX_CONTRACT_REF.to_owned(),
            ],
            redaction_class_token: "metadata_safe_default".to_owned(),
        }
    }
}
