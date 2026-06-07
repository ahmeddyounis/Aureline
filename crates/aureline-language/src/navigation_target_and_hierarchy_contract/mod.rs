//! Stable navigation-target, hierarchy, and rename-preview contracts.
//!
//! This module owns the language-platform boundary objects that search,
//! graph, review, AI, support, and CLI surfaces consume when they render
//! definition, declaration, implementation, reference, hierarchy, related
//! object, and rename-preview results. The records are metadata-only: they
//! carry canonical ids, relation/access vocabulary, proof and fallback labels,
//! ambiguity sets, scope completeness, and export-safe summaries without raw
//! source bodies or provider payloads.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record kind for [`NavigationTargetAndHierarchyContractPacket`].
pub const NAVIGATION_TARGET_CONTRACT_PACKET_RECORD_KIND: &str =
    "navigation_target_and_hierarchy_contract_packet";

/// Stable record kind for checked-in contract fixtures.
pub const NAVIGATION_TARGET_CONTRACT_CASE_RECORD_KIND: &str =
    "navigation_target_and_hierarchy_contract_case";

/// Integer schema version for the stable navigation-target contract.
pub const NAVIGATION_TARGET_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative boundary schema ref.
pub const NAVIGATION_TARGET_CONTRACT_SCHEMA_REF: &str =
    "schemas/search/navigation-targets.schema.json";

/// Repo-relative reviewer documentation ref.
pub const NAVIGATION_TARGET_CONTRACT_DOC_REF: &str =
    "docs/search/m4/navigation-target-and-hierarchy-contract.md";

/// Repo-relative release artifact ref.
pub const NAVIGATION_TARGET_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/search/m4/navigation-target-and-hierarchy-contract.md";

/// Repo-relative fixture corpus directory.
pub const NAVIGATION_TARGET_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/search/m4/navigation-target-and-hierarchy-contract";

/// Repo-relative stable fixture ref.
pub const NAVIGATION_TARGET_CONTRACT_BASELINE_FIXTURE_REF: &str =
    "fixtures/search/m4/navigation-target-and-hierarchy-contract/baseline_stable.json";

/// Relation kind shown by navigation and related-object surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    /// Target resolves to the canonical definition body.
    Definition,
    /// Target resolves to a declaration, signature, trait, or interface.
    Declaration,
    /// Target resolves to an implementation candidate.
    Implementation,
    /// Target resolves to a reference occurrence.
    Reference,
    /// Target resolves to a type, schema, class, trait, or interface object.
    Type,
    /// Target resolves to a callable invocation relation.
    Call,
    /// Target resolves to a route, framework, or generated binding.
    #[serde(rename = "route-binding")]
    RouteBinding,
    /// Target resolves to an ownership or stewardship relation.
    #[serde(rename = "owner-link")]
    OwnerLink,
    /// Target resolves to documentation, examples, or docs-generated anchors.
    #[serde(rename = "doc-link")]
    DocLink,
}

impl RelationKind {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Declaration => "declaration",
            Self::Implementation => "implementation",
            Self::Reference => "reference",
            Self::Type => "type",
            Self::Call => "call",
            Self::RouteBinding => "route-binding",
            Self::OwnerLink => "owner-link",
            Self::DocLink => "doc-link",
        }
    }
}

/// Access kind shown by references panes, rename previews, review, AI, and support exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessKind {
    /// Symbol value is consumed without mutation.
    Read,
    /// Symbol location is assigned, rebound, or otherwise mutated.
    Write,
    /// Callable symbol is invoked.
    Call,
    /// Structural inheritance, implementation, or override relation.
    Inherit,
    /// Symbol crosses an import boundary.
    Import,
    /// Symbol crosses an export boundary.
    Export,
    /// Occurrence comes from route or framework binding metadata.
    #[serde(rename = "route-binding")]
    RouteBinding,
    /// Occurrence is test-only.
    #[serde(rename = "test-only")]
    TestOnly,
    /// Occurrence is generated, mirrored, or framework-emitted.
    Generated,
    /// Occurrence is corroborated only by runtime evidence.
    #[serde(rename = "runtime-observed")]
    RuntimeObserved,
}

impl AccessKind {
    /// Every access-kind token that stable surfaces must preserve.
    pub const REQUIRED: [Self; 10] = [
        Self::Read,
        Self::Write,
        Self::Call,
        Self::Inherit,
        Self::Import,
        Self::Export,
        Self::RouteBinding,
        Self::TestOnly,
        Self::Generated,
        Self::RuntimeObserved,
    ];

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Call => "call",
            Self::Inherit => "inherit",
            Self::Import => "import",
            Self::Export => "export",
            Self::RouteBinding => "route-binding",
            Self::TestOnly => "test-only",
            Self::Generated => "generated",
            Self::RuntimeObserved => "runtime-observed",
        }
    }
}

/// Provider or source family that admitted a contract object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderSourceClass {
    /// Language-server protocol provider.
    LanguageServer,
    /// Parser or syntax-tree-only provider.
    Parser,
    /// Search index or lexical provider.
    SearchIndex,
    /// Semantic graph provider.
    SemanticGraph,
    /// Framework-specific analyzer.
    FrameworkAnalyzer,
    /// Generated-source lineage bridge.
    GeneratedSource,
    /// Notebook or literate-programming adapter.
    NotebookAdapter,
    /// Diff or review snapshot adapter.
    DiffAdapter,
    /// Runtime observer, debugger, trace, or profile source.
    RuntimeObserver,
    /// Imported provider snapshot or docs pack.
    ImportedSnapshot,
}

/// Confidence class for a navigation object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Proven exactly for the declared scope.
    Exact,
    /// Proven from a current index or graph.
    Indexed,
    /// Useful but incomplete for the declared scope.
    Partial,
    /// Imported from a snapshot, docs pack, or overlay.
    Imported,
    /// Evidence is stale for the declared scope.
    Stale,
    /// Heuristic or fallback evidence.
    Heuristic,
    /// No admissible result exists.
    Unavailable,
}

impl ConfidenceClass {
    /// Returns true when a result needs a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact | Self::Indexed)
    }
}

/// Freshness class for target, occurrence, hierarchy, and rename rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Revalidated against current source, graph, provider, or runtime epoch.
    AuthoritativeLive,
    /// Warm cache is current enough for read-only navigation.
    WarmCached,
    /// Cache is usable only with a visible downgrade label.
    DegradedCached,
    /// Evidence is past its freshness floor.
    Stale,
    /// Evidence has not been verified against the current workspace.
    Unverified,
}

impl FreshnessClass {
    /// Returns true when a result needs a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Ambiguity state for targets and disambiguation sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmbiguityClass {
    /// Exactly one target is available.
    Unambiguous,
    /// Multiple candidates require user or caller selection.
    AmbiguousNeedsSelection,
    /// Multiple candidates are ranked but still inspectable.
    MultipleCandidatesRanked,
    /// Prior target drifted and cannot be opened automatically.
    DriftedNeedsReview,
    /// Target is missing from the current scope.
    MissingTarget,
}

impl AmbiguityClass {
    /// Returns true when a disambiguation set or visible drift state is required.
    pub const fn requires_disambiguation(self) -> bool {
        !matches!(self, Self::Unambiguous)
    }
}

/// Completeness of the scope used to produce a result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCompleteness {
    /// Result set is complete for the declared materialized scope.
    Complete,
    /// Result set is partial for the declared scope.
    Partial,
    /// Result set is stale for the declared scope.
    Stale,
    /// Result set is unavailable for the declared scope.
    Unavailable,
}

impl ScopeCompleteness {
    /// Returns true when a result needs a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Complete)
    }
}

/// Fallback mode for unsupported, downgraded, partial, or redacted rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackMode {
    /// No fallback or downgrade applies.
    None,
    /// Requested relation is unsupported by the provider.
    UnsupportedRelation,
    /// Provider downgraded to a shallower relation.
    DowngradedProvider,
    /// Lexical or grep fallback produced the object.
    LexicalFallback,
    /// Syntax-only fallback produced the object.
    SyntaxFallback,
    /// Stale cache produced the object.
    StaleCache,
    /// Sparse scope narrowed the object set.
    SparseScope,
    /// Export includes metadata only because bodies are redacted.
    RedactedMetadataOnly,
    /// Generated-source boundary limited the object.
    GeneratedBoundary,
    /// Runtime evidence exists without static semantic replacement.
    RuntimeObservedOnly,
    /// Framework evidence exists without static semantic replacement.
    FrameworkDerivedOnly,
    /// Protected row was blocked or omitted from mutation.
    ProtectedRowOmitted,
}

impl Default for FallbackMode {
    fn default() -> Self {
        Self::None
    }
}

impl FallbackMode {
    /// Returns true when the row requires an explanatory note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Proof class for a target, occurrence, edge, or rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofClass {
    /// Direct static semantic proof.
    Direct,
    /// Transitive proof through a static graph or hierarchy.
    Transitive,
    /// Inferred proof from partial semantic evidence.
    Inferred,
    /// Framework or generator proof.
    FrameworkGenerated,
    /// Runtime trace, debugger, profile, or observed dispatch proof.
    RuntimeObserved,
    /// Lexical or grep proof.
    LexicalFallback,
    /// No proof was available.
    Unavailable,
}

impl ProofClass {
    /// Returns true when proof cannot replace direct static semantic proof.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Direct)
    }
}

/// Hierarchy edge class preserved in call, type, framework, and runtime views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyEdgeKind {
    /// Direct static relation.
    Direct,
    /// Transitive relation.
    Transitive,
    /// Inferred relation.
    Inferred,
    /// Framework-generated relation.
    FrameworkGenerated,
    /// Runtime-observed relation.
    RuntimeObserved,
}

impl HierarchyEdgeKind {
    /// Every edge class stable hierarchy views must preserve.
    pub const REQUIRED: [Self; 5] = [
        Self::Direct,
        Self::Transitive,
        Self::Inferred,
        Self::FrameworkGenerated,
        Self::RuntimeObserved,
    ];

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::Inferred => "inferred",
            Self::FrameworkGenerated => "framework_generated",
            Self::RuntimeObserved => "runtime_observed",
        }
    }
}

/// Stable target selected by definition, declaration, implementation, hierarchy, or related-object navigation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTarget {
    /// Stable target id.
    pub target_id: String,
    /// Relation represented by the target.
    pub relation_kind: RelationKind,
    /// Stable object ref resolved by the owning subsystem.
    pub object_ref: String,
    /// Stable anchor ref resolved by the owning subsystem.
    pub anchor_ref: String,
    /// Provider or source family that admitted the target.
    pub provider_class: ProviderSourceClass,
    /// Confidence class for the target.
    pub confidence: ConfidenceClass,
    /// Freshness class for the target.
    pub freshness: FreshnessClass,
    /// Ambiguity class for the target.
    pub ambiguity_class: AmbiguityClass,
    /// Completeness of the declared scope.
    pub scope_completeness: ScopeCompleteness,
    /// Fallback mode, if any.
    #[serde(default)]
    pub fallback_mode: FallbackMode,
    /// Support-safe explanation for fallback, ambiguity, or partiality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
    /// Disambiguation set ref used when the target is ambiguous.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disambiguation_set_ref: Option<String>,
    /// Support-safe evidence refs.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NavigationTarget {
    /// Returns true when the target needs a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.ambiguity_class.requires_disambiguation()
            || self.scope_completeness.requires_disclosure()
            || self.fallback_mode.requires_note()
    }
}

/// One occurrence returned by references, rename preview, review evidence, AI context, or support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceOccurrence {
    /// Stable occurrence id.
    pub occurrence_id: String,
    /// Target this occurrence references.
    pub target_ref: String,
    /// Stable source anchor ref.
    pub anchor_ref: String,
    /// Access kind for the occurrence.
    pub access_kind: AccessKind,
    /// Scope containing the occurrence.
    pub scope_ref: String,
    /// Provider or source family that admitted the occurrence.
    pub provider_class: ProviderSourceClass,
    /// Confidence class for the occurrence.
    pub confidence: ConfidenceClass,
    /// Freshness class for the occurrence.
    pub freshness: FreshnessClass,
    /// Completeness of the occurrence scope.
    pub scope_completeness: ScopeCompleteness,
    /// Fallback mode, if any.
    #[serde(default)]
    pub fallback_mode: FallbackMode,
    /// Support-safe explanation for fallback or partiality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
    /// True when surrounding code bodies were intentionally omitted.
    pub code_body_redacted: bool,
}

impl ReferenceOccurrence {
    /// Returns true when the occurrence needs a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || self.fallback_mode.requires_note()
            || self.code_body_redacted
    }
}

/// Candidate set emitted when navigation has more than one truthful target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationDisambiguationSet {
    /// Stable set id.
    pub set_id: String,
    /// Relation requested by the caller.
    pub requested_relation: RelationKind,
    /// Candidate target refs available for explicit selection.
    pub candidate_target_refs: Vec<String>,
    /// Selection policy shown to UI, CLI, AI, review, and support consumers.
    pub selection_policy: String,
    /// ISO 8601 UTC creation timestamp.
    pub created_at: String,
    /// Ambiguity represented by this set.
    pub ambiguity_class: AmbiguityClass,
    /// Fallback mode, if any.
    #[serde(default)]
    pub fallback_mode: FallbackMode,
    /// Support-safe explanation for fallback or ambiguity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
}

/// One edge in a call, type, implementation, framework, or runtime hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyEdge {
    /// Stable edge id.
    pub edge_id: String,
    /// Source target ref.
    pub source_ref: String,
    /// Target target ref.
    pub target_ref: String,
    /// Edge class preserved by hierarchy views.
    pub edge_kind: HierarchyEdgeKind,
    /// Proof class for the edge.
    pub proof_class: ProofClass,
    /// Edge depth in the rendered hierarchy.
    pub depth: u32,
    /// Completeness of the hierarchy scope.
    pub scope_completeness: ScopeCompleteness,
    /// Freshness class for the edge.
    pub freshness: FreshnessClass,
    /// Confidence class for the edge.
    pub confidence: ConfidenceClass,
    /// Fallback mode, if any.
    #[serde(default)]
    pub fallback_mode: FallbackMode,
    /// Support-safe explanation for fallback or partiality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
    /// Support-safe proof refs for non-direct edges.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl HierarchyEdge {
    /// Returns true when the edge needs a visible caveat.
    pub fn requires_disclosure(&self) -> bool {
        self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || self.fallback_mode.requires_note()
    }
}

/// Reason a rename candidate is blocked, omitted, or inspect-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameCandidateDisposition {
    /// Candidate can be changed.
    Changeable,
    /// Candidate is blocked by policy or protected source.
    Blocked,
    /// Candidate is generated and cannot be directly authored.
    Generated,
    /// Candidate is read-only.
    Readonly,
    /// Candidate is omitted because the scope is sparse.
    SparseScopeOmitted,
    /// Candidate is only partially loaded.
    PartiallyLoaded,
    /// Candidate is retained as metadata only after redaction.
    Redacted,
}

/// Rename candidate ref retained even when the underlying body is hidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenameCandidateRef {
    /// Occurrence or target ref for the candidate.
    pub candidate_ref: String,
    /// Candidate access kind.
    pub access_kind: AccessKind,
    /// Candidate disposition.
    pub disposition: RenameCandidateDisposition,
    /// Support-safe reason for blocked, omitted, partial, or redacted candidates.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Governed rename preview expressed in stable occurrence identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewSet {
    /// Stable rename-preview id.
    pub rename_preview_id: String,
    /// Root target being renamed.
    pub root_target_ref: String,
    /// Candidate occurrences the preview can change.
    pub candidate_occurrence_refs: Vec<String>,
    /// Candidate refs blocked from mutation or retained as metadata-only truth.
    #[serde(default)]
    pub blocked_refs: Vec<RenameCandidateRef>,
    /// Conflict notes such as shadowing, alias collision, or downstream rename risk.
    #[serde(default)]
    pub conflict_notes: Vec<String>,
    /// Sparse, partial, stale, or provider-limit reasons.
    #[serde(default)]
    pub sparse_or_partial_reasons: Vec<String>,
    /// Generated-scope notes that must remain visible.
    #[serde(default)]
    pub generated_scope_notes: Vec<String>,
    /// Count of candidates hidden from body-level preview by redaction.
    pub redacted_candidate_count: u32,
    /// Fallback mode, if any.
    #[serde(default)]
    pub fallback_mode: FallbackMode,
    /// Support-safe explanation for fallback or partiality.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_note: Option<String>,
}

impl RenamePreviewSet {
    /// Returns true when the preview needs a visible caveat or cannot apply directly.
    pub fn requires_disclosure(&self) -> bool {
        !self.blocked_refs.is_empty()
            || !self.conflict_notes.is_empty()
            || !self.sparse_or_partial_reasons.is_empty()
            || !self.generated_scope_notes.is_empty()
            || self.redacted_candidate_count > 0
            || self.fallback_mode.requires_note()
    }
}

/// Consumer surface that must preserve the stable vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor navigation UI.
    EditorUi,
    /// Search results and references panes.
    Search,
    /// Graph topology or hierarchy surface.
    Graph,
    /// Review evidence surface.
    ReviewEvidence,
    /// AI context or tool evidence surface.
    AiContext,
    /// Support export.
    SupportExport,
    /// CLI or headless inspector.
    CliHeadless,
}

impl ConsumerSurface {
    /// Required consumer surfaces for stable parity.
    pub const REQUIRED: [Self; 7] = [
        Self::EditorUi,
        Self::Search,
        Self::Graph,
        Self::ReviewEvidence,
        Self::AiContext,
        Self::SupportExport,
        Self::CliHeadless,
    ];
}

/// Projection proving a consumer preserves the same contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// Surface receiving the projection.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the surface.
    pub packet_ref: String,
    /// True when relation-kind labels are preserved.
    pub preserves_relation_kind: bool,
    /// True when access-kind labels are preserved.
    pub preserves_access_kind: bool,
    /// True when provider/source labels are preserved.
    pub preserves_provider_source: bool,
    /// True when confidence and freshness labels are preserved.
    pub preserves_confidence_and_freshness: bool,
    /// True when ambiguity sets are preserved.
    pub preserves_ambiguity: bool,
    /// True when hierarchy edge classes are preserved.
    pub preserves_hierarchy_edges: bool,
    /// True when rename blocked, generated, readonly, sparse, partial, and redacted truth is preserved.
    pub preserves_rename_preview_truth: bool,
    /// True when raw source bodies are excluded from this projection.
    pub raw_bodies_excluded: bool,
}

impl ConsumerProjection {
    fn preserves_contract(&self, packet_id: &str) -> bool {
        self.packet_ref == packet_id
            && self.preserves_relation_kind
            && self.preserves_access_kind
            && self.preserves_provider_source
            && self.preserves_confidence_and_freshness
            && self.preserves_ambiguity
            && self.preserves_hierarchy_edges
            && self.preserves_rename_preview_truth
            && self.raw_bodies_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Validation finding kind emitted by the contract validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContractFindingKind {
    /// Required identity field is empty.
    MissingIdentity,
    /// Fallback mode is present without a stable reason.
    FallbackReasonMissing,
    /// Ambiguous target is missing a disambiguation set.
    AmbiguitySetMissing,
    /// Disambiguation set has fewer than two candidates.
    DisambiguationCandidateCountTooLow,
    /// Required access-kind vocabulary is not represented.
    MissingAccessKindCoverage,
    /// Required hierarchy-edge vocabulary is not represented.
    MissingHierarchyEdgeCoverage,
    /// Non-direct hierarchy edge dropped evidence refs.
    HierarchyEvidenceMissing,
    /// Rename preview dropped blocked, generated, readonly, sparse, partial, or redacted truth.
    RenamePreviewTruthMissing,
    /// Consumer projection is missing or drops contract vocabulary.
    ConsumerProjectionDrift,
}

/// Severity for one validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationContractFindingSeverity {
    /// Informational finding.
    Info,
    /// Finding requires review before a stable claim widens.
    Warning,
    /// Finding blocks the stable claim.
    Blocker,
}

/// One validation finding emitted by the contract validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationContractFinding {
    /// Finding kind.
    pub finding_kind: NavigationContractFindingKind,
    /// Finding severity.
    pub severity: NavigationContractFindingSeverity,
    /// Subject ref that failed validation.
    pub subject_ref: String,
    /// Support-safe summary.
    pub summary: String,
}

impl NavigationContractFinding {
    fn blocker(
        finding_kind: NavigationContractFindingKind,
        subject_ref: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity: NavigationContractFindingSeverity::Blocker,
            subject_ref: subject_ref.into(),
            summary: summary.into(),
        }
    }
}

/// Packet containing stable target, occurrence, hierarchy, disambiguation, and rename objects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetAndHierarchyContractPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Capture timestamp.
    pub generated_at: String,
    /// Stable navigation targets.
    #[serde(default)]
    pub navigation_targets: Vec<NavigationTarget>,
    /// Stable reference occurrences.
    #[serde(default)]
    pub reference_occurrences: Vec<ReferenceOccurrence>,
    /// Navigation disambiguation sets.
    #[serde(default)]
    pub disambiguation_sets: Vec<NavigationDisambiguationSet>,
    /// Hierarchy edges.
    #[serde(default)]
    pub hierarchy_edges: Vec<HierarchyEdge>,
    /// Rename preview sets.
    #[serde(default)]
    pub rename_preview_sets: Vec<RenamePreviewSet>,
    /// Consumer projections that preserve this packet.
    #[serde(default)]
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// True when raw source bodies and provider payloads are excluded.
    pub raw_bodies_excluded: bool,
}

impl NavigationTargetAndHierarchyContractPacket {
    /// Validates stable navigation-target and rename-preview invariants.
    pub fn validate(&self) -> Vec<NavigationContractFinding> {
        let mut findings = Vec::new();

        if self.record_kind != NAVIGATION_TARGET_CONTRACT_PACKET_RECORD_KIND
            || self.schema_version != NAVIGATION_TARGET_CONTRACT_SCHEMA_VERSION
            || self.packet_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
            || !self.raw_bodies_excluded
        {
            findings.push(NavigationContractFinding::blocker(
                NavigationContractFindingKind::MissingIdentity,
                &self.packet_id,
                "packet identity, schema, and metadata-only boundary must be intact",
            ));
        }

        self.validate_targets(&mut findings);
        self.validate_occurrences(&mut findings);
        self.validate_disambiguation_sets(&mut findings);
        self.validate_hierarchy_edges(&mut findings);
        self.validate_rename_previews(&mut findings);
        self.validate_consumer_projections(&mut findings);

        findings
    }

    /// Returns true when validation has no blocker findings.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == NavigationContractFindingSeverity::Blocker)
    }

    /// Returns unique access-kind tokens observed across references and rename candidates.
    pub fn access_kind_tokens(&self) -> Vec<&'static str> {
        let mut tokens = BTreeSet::new();
        for occurrence in &self.reference_occurrences {
            tokens.insert(occurrence.access_kind);
        }
        for preview in &self.rename_preview_sets {
            for candidate in &preview.blocked_refs {
                tokens.insert(candidate.access_kind);
            }
        }
        tokens.into_iter().map(AccessKind::as_str).collect()
    }

    /// Returns unique hierarchy edge tokens observed across hierarchy rows.
    pub fn hierarchy_edge_tokens(&self) -> Vec<&'static str> {
        let mut tokens = BTreeSet::new();
        for edge in &self.hierarchy_edges {
            tokens.insert(edge.edge_kind);
        }
        tokens.into_iter().map(HierarchyEdgeKind::as_str).collect()
    }

    fn validate_targets(&self, findings: &mut Vec<NavigationContractFinding>) {
        for target in &self.navigation_targets {
            if target.target_id.trim().is_empty()
                || target.object_ref.trim().is_empty()
                || target.anchor_ref.trim().is_empty()
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingIdentity,
                    &target.target_id,
                    "navigation target must carry target, object, and anchor refs",
                ));
            }
            if target.fallback_mode.requires_note() && is_empty_note(&target.fallback_note) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::FallbackReasonMissing,
                    &target.target_id,
                    "downgraded navigation target must carry a stable fallback reason",
                ));
            }
            if target.ambiguity_class.requires_disambiguation()
                && target
                    .disambiguation_set_ref
                    .as_ref()
                    .map_or(true, |set_ref| !self.has_disambiguation_set(set_ref))
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::AmbiguitySetMissing,
                    &target.target_id,
                    "ambiguous target must point at an inspectable disambiguation set",
                ));
            }
        }
    }

    fn validate_occurrences(&self, findings: &mut Vec<NavigationContractFinding>) {
        for occurrence in &self.reference_occurrences {
            if occurrence.occurrence_id.trim().is_empty()
                || occurrence.target_ref.trim().is_empty()
                || occurrence.anchor_ref.trim().is_empty()
                || occurrence.scope_ref.trim().is_empty()
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingIdentity,
                    &occurrence.occurrence_id,
                    "reference occurrence must carry occurrence, target, anchor, and scope refs",
                ));
            }
            if occurrence.fallback_mode.requires_note() && is_empty_note(&occurrence.fallback_note)
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::FallbackReasonMissing,
                    &occurrence.occurrence_id,
                    "downgraded reference occurrence must carry a stable fallback reason",
                ));
            }
        }

        let observed = self
            .reference_occurrences
            .iter()
            .map(|occurrence| occurrence.access_kind)
            .chain(
                self.rename_preview_sets
                    .iter()
                    .flat_map(|preview| preview.blocked_refs.iter())
                    .map(|candidate| candidate.access_kind),
            )
            .collect::<BTreeSet<_>>();
        for required in AccessKind::REQUIRED {
            if !observed.contains(&required) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingAccessKindCoverage,
                    required.as_str(),
                    format!(
                        "stable packet must preserve access kind {}",
                        required.as_str()
                    ),
                ));
            }
        }
    }

    fn validate_disambiguation_sets(&self, findings: &mut Vec<NavigationContractFinding>) {
        for set in &self.disambiguation_sets {
            if set.set_id.trim().is_empty()
                || set.selection_policy.trim().is_empty()
                || set.created_at.trim().is_empty()
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingIdentity,
                    &set.set_id,
                    "disambiguation set must carry id, policy, and timestamp",
                ));
            }
            if set.candidate_target_refs.len() < 2 {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::DisambiguationCandidateCountTooLow,
                    &set.set_id,
                    "disambiguation set must expose at least two candidates",
                ));
            }
            if set.fallback_mode.requires_note() && is_empty_note(&set.fallback_note) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::FallbackReasonMissing,
                    &set.set_id,
                    "downgraded disambiguation set must carry a stable fallback reason",
                ));
            }
        }
    }

    fn validate_hierarchy_edges(&self, findings: &mut Vec<NavigationContractFinding>) {
        let observed = self
            .hierarchy_edges
            .iter()
            .map(|edge| edge.edge_kind)
            .collect::<BTreeSet<_>>();
        for required in HierarchyEdgeKind::REQUIRED {
            if !observed.contains(&required) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingHierarchyEdgeCoverage,
                    required.as_str(),
                    format!(
                        "stable packet must preserve hierarchy edge kind {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for edge in &self.hierarchy_edges {
            if edge.edge_id.trim().is_empty()
                || edge.source_ref.trim().is_empty()
                || edge.target_ref.trim().is_empty()
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingIdentity,
                    &edge.edge_id,
                    "hierarchy edge must carry edge, source, and target refs",
                ));
            }
            if edge.requires_disclosure()
                && edge.fallback_mode.requires_note()
                && is_empty_note(&edge.fallback_note)
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::FallbackReasonMissing,
                    &edge.edge_id,
                    "downgraded hierarchy edge must carry a stable fallback reason",
                ));
            }
            if !matches!(edge.proof_class, ProofClass::Direct) && edge.evidence_refs.is_empty() {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::HierarchyEvidenceMissing,
                    &edge.edge_id,
                    "non-direct hierarchy edge must preserve proof refs",
                ));
            }
        }
    }

    fn validate_rename_previews(&self, findings: &mut Vec<NavigationContractFinding>) {
        for preview in &self.rename_preview_sets {
            if preview.rename_preview_id.trim().is_empty()
                || preview.root_target_ref.trim().is_empty()
                || preview.candidate_occurrence_refs.is_empty()
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::MissingIdentity,
                    &preview.rename_preview_id,
                    "rename preview must carry id, root target, and candidate occurrence refs",
                ));
            }
            if preview.fallback_mode.requires_note() && is_empty_note(&preview.fallback_note) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::FallbackReasonMissing,
                    &preview.rename_preview_id,
                    "downgraded rename preview must carry a stable fallback reason",
                ));
            }
            let has_blocked = preview.blocked_refs.iter().any(|candidate| {
                !matches!(
                    candidate.disposition,
                    RenameCandidateDisposition::Changeable
                )
            });
            let has_reasonless_blocked = preview.blocked_refs.iter().any(|candidate| {
                !matches!(
                    candidate.disposition,
                    RenameCandidateDisposition::Changeable
                ) && candidate
                    .reason
                    .as_ref()
                    .map_or(true, |reason| reason.trim().is_empty())
            });
            if preview.requires_disclosure()
                && (!has_blocked
                    || has_reasonless_blocked
                    || (preview.redacted_candidate_count > 0
                        && !preview.blocked_refs.iter().any(|candidate| {
                            candidate.disposition == RenameCandidateDisposition::Redacted
                        })))
            {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::RenamePreviewTruthMissing,
                    &preview.rename_preview_id,
                    "rename preview must preserve blocked, omitted, generated, readonly, partial, and redacted candidates",
                ));
            }
        }
    }

    fn validate_consumer_projections(&self, findings: &mut Vec<NavigationContractFinding>) {
        for surface in ConsumerSurface::REQUIRED {
            if !self.consumer_projections.iter().any(|projection| {
                projection.consumer_surface == surface
                    && projection.preserves_contract(&self.packet_id)
            }) {
                findings.push(NavigationContractFinding::blocker(
                    NavigationContractFindingKind::ConsumerProjectionDrift,
                    format!("{surface:?}"),
                    "consumer projection must preserve target, reference, hierarchy, ambiguity, fallback, and rename truth",
                ));
            }
        }
    }

    fn has_disambiguation_set(&self, set_ref: &str) -> bool {
        self.disambiguation_sets
            .iter()
            .any(|set| set.set_id == set_ref)
    }
}

/// Checked-in contract case used by the fixture corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetAndHierarchyContractCase {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Contract packet exercised by the case.
    pub packet: NavigationTargetAndHierarchyContractPacket,
    /// Expected validation finding count.
    pub expected_finding_count: usize,
}

/// Loads a JSON-encoded contract case.
pub fn load_navigation_target_contract_case(
    json: &str,
) -> Result<NavigationTargetAndHierarchyContractCase, serde_json::Error> {
    serde_json::from_str(json)
}

/// Returns the checked-in stable contract case.
pub fn current_navigation_target_contract_case(
) -> Result<NavigationTargetAndHierarchyContractCase, serde_json::Error> {
    load_navigation_target_contract_case(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/search/m4/navigation-target-and-hierarchy-contract/baseline_stable.json"
    )))
}

/// Error returned when the checked-in contract fixture is invalid.
#[derive(Debug)]
pub enum NavigationTargetContractError {
    /// Fixture JSON failed to parse.
    Parse(serde_json::Error),
    /// Fixture metadata is not the expected contract case.
    WrongCaseMetadata,
    /// Fixture packet failed validation.
    Validation(Vec<NavigationContractFinding>),
}

impl fmt::Display for NavigationTargetContractError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "navigation contract fixture parse: {err}"),
            Self::WrongCaseMetadata => {
                write!(formatter, "navigation contract fixture metadata drifted")
            }
            Self::Validation(findings) => write!(
                formatter,
                "navigation contract fixture has {} validation findings",
                findings.len()
            ),
        }
    }
}

impl Error for NavigationTargetContractError {}

impl From<serde_json::Error> for NavigationTargetContractError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

/// Loads and validates the checked-in stable contract packet.
pub fn current_navigation_target_contract_packet(
) -> Result<NavigationTargetAndHierarchyContractPacket, NavigationTargetContractError> {
    let case = current_navigation_target_contract_case()?;
    if case.record_kind != NAVIGATION_TARGET_CONTRACT_CASE_RECORD_KIND
        || case.schema_version != NAVIGATION_TARGET_CONTRACT_SCHEMA_VERSION
    {
        return Err(NavigationTargetContractError::WrongCaseMetadata);
    }
    let findings = case.packet.validate();
    if findings.len() != case.expected_finding_count || !findings.is_empty() {
        return Err(NavigationTargetContractError::Validation(findings));
    }
    Ok(case.packet)
}

fn is_empty_note(note: &Option<String>) -> bool {
    note.as_ref().map_or(true, |note| note.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checked_in_contract_packet_is_stable() {
        let packet = current_navigation_target_contract_packet().expect("packet validates");
        assert!(packet.is_stable());
        assert_eq!(
            packet.access_kind_tokens().len(),
            AccessKind::REQUIRED.len()
        );
        assert_eq!(
            packet.hierarchy_edge_tokens().len(),
            HierarchyEdgeKind::REQUIRED.len()
        );
    }

    #[test]
    fn fallback_without_reason_blocks_stable() {
        let mut packet = current_navigation_target_contract_packet().expect("packet validates");
        packet.navigation_targets[0].fallback_mode = FallbackMode::DowngradedProvider;
        packet.navigation_targets[0].fallback_note = None;
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == NavigationContractFindingKind::FallbackReasonMissing
        }));
    }

    #[test]
    fn ambiguous_target_without_disambiguation_blocks_stable() {
        let mut packet = current_navigation_target_contract_packet().expect("packet validates");
        packet.navigation_targets[2].ambiguity_class = AmbiguityClass::AmbiguousNeedsSelection;
        packet.navigation_targets[2].disambiguation_set_ref = None;
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == NavigationContractFindingKind::AmbiguitySetMissing
        }));
    }

    #[test]
    fn rename_redaction_without_candidate_truth_blocks_stable() {
        let mut packet = current_navigation_target_contract_packet().expect("packet validates");
        packet.rename_preview_sets[0].blocked_refs.clear();
        let findings = packet.validate();
        assert!(findings.iter().any(|finding| {
            finding.finding_kind == NavigationContractFindingKind::RenamePreviewTruthMissing
        }));
    }
}
