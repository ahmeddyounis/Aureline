//! Shared target model for semantic navigation, hierarchy, rename preview, and continuity evidence.
//!
//! The model is intentionally metadata-only. Records carry stable ids,
//! machine-readable relation/proof/scope vocabulary, evidence refs, and
//! reviewable summaries; they do not carry source bodies, raw paths, provider
//! payloads, URLs, credentials, or private rank weights.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for navigation target fidelity case fixtures.
pub const NAVIGATION_TARGET_FIDELITY_CASE_RECORD_KIND: &str =
    "navigation_target_fidelity_case_record";

/// Stable record-kind tag for the navigation target fidelity report.
pub const NAVIGATION_TARGET_FIDELITY_REPORT_RECORD_KIND: &str =
    "navigation_target_fidelity_report_record";

/// Integer schema version for the navigation target beta model.
pub const NAVIGATION_TARGET_SCHEMA_VERSION: NavigationTargetModelVersion = 1;

/// Repo-relative path of the machine-readable boundary schema.
pub const NAVIGATION_TARGET_SCHEMA_REF: &str = "schemas/navigation/navigation_target.schema.json";

/// Repo-relative path of the beta contract document.
pub const NAVIGATION_TARGET_BETA_CONTRACT_DOC_REF: &str =
    "docs/navigation/m3/navigation_target_beta_contract.md";

/// Repo-relative path of the checked-in fidelity report.
pub const NAVIGATION_TARGET_FIDELITY_REPORT_REF: &str =
    "artifacts/navigation/m3/navigation_target_fidelity_report.md";

/// Repo-relative directory of the protected target-accuracy corpus.
pub const NAVIGATION_TARGET_FIDELITY_CORPUS_DIR: &str = "fixtures/navigation/m3/target_accuracy";

/// Integer schema version used by navigation target records.
pub type NavigationTargetModelVersion = u32;

/// Closed relation-kind vocabulary shared by navigation, graph, review, AI, CLI, and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    /// Jump resolves to the implementation body or canonical definition site.
    Definition,
    /// Jump resolves to a declaration, interface, trait, or signature surface.
    Declaration,
    /// Jump resolves to one implementation candidate.
    Implementation,
    /// Row represents a reference occurrence or reference set member.
    Reference,
    /// Jump resolves to a type, schema, trait, interface, or class target.
    Type,
    /// Row represents a callable invocation or call-hierarchy relation.
    Call,
    /// Relation is synthesized from routing, framework, or runtime binding metadata.
    #[serde(rename = "route-binding")]
    RouteBinding,
    /// Relation links an owner, CODEOWNERS-like rule, or stewardship record.
    #[serde(rename = "owner-link")]
    OwnerLink,
    /// Relation links documentation, examples, or generated docs anchors.
    #[serde(rename = "doc-link")]
    DocLink,
}

impl RelationKind {
    /// Returns the stable token serialized into fixtures and support exports.
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

/// Relation kinds that must be covered by the protected beta corpus.
pub const REQUIRED_RELATION_KINDS: [RelationKind; 9] = [
    RelationKind::Definition,
    RelationKind::Declaration,
    RelationKind::Implementation,
    RelationKind::Reference,
    RelationKind::Type,
    RelationKind::Call,
    RelationKind::RouteBinding,
    RelationKind::OwnerLink,
    RelationKind::DocLink,
];

/// Machine-readable access kind for references and rename candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessKind {
    /// Symbol value is read.
    Read,
    /// Symbol location is mutated, assigned, or rebound.
    Write,
    /// Callable symbol is invoked.
    Call,
    /// Structural inheritance, implementation, or override relation.
    Inherit,
    /// Symbol crosses an import boundary.
    Import,
    /// Symbol crosses an export boundary.
    Export,
    /// Occurrence is test-only and should not be hidden inside production counts.
    #[serde(rename = "test-only")]
    TestOnly,
    /// Occurrence is generated, mirrored, framework-emitted, or otherwise non-authored.
    Generated,
}

impl AccessKind {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Call => "call",
            Self::Inherit => "inherit",
            Self::Import => "import",
            Self::Export => "export",
            Self::TestOnly => "test-only",
            Self::Generated => "generated",
        }
    }
}

/// Proof class explaining why a navigation target, reference, or hierarchy edge exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofClass {
    /// Direct semantic provider evidence over current source.
    DirectSemantic,
    /// Current index or graph evidence over the declared scope.
    IndexedSemantic,
    /// Lexical or grep fallback evidence.
    LexicalFallback,
    /// Syntax-tree-only fallback evidence.
    SyntaxFallback,
    /// Imported graph, docs pack, provider overlay, or captured snapshot evidence.
    ImportedEvidence,
    /// Framework pack, route metadata, generator metadata, or build-tool evidence.
    FrameworkDerived,
    /// Runtime trace, debugger, profile, or observed-dispatch evidence.
    RuntimeObserved,
    /// AI-derived hypothesis that is not sufficient for authoritative navigation.
    AiInferred,
    /// No admissible proof exists for the requested relation.
    Unavailable,
}

impl ProofClass {
    /// Returns true when this proof class is weaker than direct or indexed semantic proof.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::DirectSemantic | Self::IndexedSemantic)
    }

    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectSemantic => "direct_semantic",
            Self::IndexedSemantic => "indexed_semantic",
            Self::LexicalFallback => "lexical_fallback",
            Self::SyntaxFallback => "syntax_fallback",
            Self::ImportedEvidence => "imported_evidence",
            Self::FrameworkDerived => "framework_derived",
            Self::RuntimeObserved => "runtime_observed",
            Self::AiInferred => "ai_inferred",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Provider family that produced or admitted the navigation record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderClass {
    /// Syntax or parser-only provider.
    Syntax,
    /// Project graph or semantic graph provider.
    ProjectGraph,
    /// Language server protocol provider.
    LanguageServer,
    /// Framework-specific provider pack.
    FrameworkPack,
    /// Notebook or literate-programming adapter.
    NotebookAdapter,
    /// Generated-source lineage bridge.
    GeneratedSourceBridge,
    /// Search index provider.
    SearchIndex,
    /// Remote index or managed workspace provider.
    RemoteIndex,
    /// Imported docs pack, snapshot, or provider overlay.
    ImportedSnapshot,
    /// Runtime observer, debugger, trace, or profiler provider.
    RuntimeObserver,
    /// AI assistance provider.
    AiAssist,
}

impl ProviderClass {
    /// Returns the proof class normally implied by this provider family.
    pub const fn default_proof_class(self) -> ProofClass {
        match self {
            Self::Syntax => ProofClass::SyntaxFallback,
            Self::ProjectGraph | Self::SearchIndex | Self::RemoteIndex => {
                ProofClass::IndexedSemantic
            }
            Self::LanguageServer => ProofClass::DirectSemantic,
            Self::FrameworkPack | Self::GeneratedSourceBridge => ProofClass::FrameworkDerived,
            Self::NotebookAdapter | Self::ImportedSnapshot => ProofClass::ImportedEvidence,
            Self::RuntimeObserver => ProofClass::RuntimeObserved,
            Self::AiAssist => ProofClass::AiInferred,
        }
    }
}

/// Confidence class for one navigation result or relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationConfidence {
    /// Proven exactly against current source for the declared scope.
    Exact,
    /// Proven from a current index for the declared scope.
    Indexed,
    /// Imported from a snapshot, docs pack, provider overlay, or generated-source lineage.
    Imported,
    /// Useful but incomplete for the declared scope.
    Partial,
    /// Evidence is stale for the declared scope.
    Stale,
    /// No admissible result exists.
    Unavailable,
    /// Heuristic or fallback mapping.
    Heuristic,
    /// Truthful for the current workset or sparse slice, not the requested workspace.
    WorkspaceSliceLimited,
}

impl NavigationConfidence {
    /// Returns true when the confidence requires a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact | Self::Indexed)
    }
}

/// Freshness class shared by target, reference, hierarchy, and rename rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Revalidated against current source, graph, provider, or runtime epoch.
    AuthoritativeLive,
    /// Warm cache known to be current enough for read-only navigation.
    WarmCached,
    /// Cache is usable only with downgrade disclosure.
    DegradedCached,
    /// Evidence is past its freshness floor.
    Stale,
    /// Evidence has not been verified against the current workspace.
    Unverified,
}

impl FreshnessClass {
    /// Returns true when the freshness requires a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::DegradedCached | Self::Stale | Self::Unverified)
    }
}

/// Ambiguity class for targets and disambiguation sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmbiguityClass {
    /// Exactly one target is selected with no visible ambiguity.
    Unambiguous,
    /// Multiple candidates require explicit user or caller selection.
    AmbiguousNeedsSelection,
    /// Multiple candidates are ranked but still inspectable.
    MultipleCandidatesRanked,
    /// A previous target drifted and cannot auto-open safely.
    DriftedNeedsReview,
    /// Target is missing from the current scope.
    MissingTarget,
    /// Workset, branch, policy, docs pack, or remote shard hides the target.
    ScopeUnavailable,
}

impl AmbiguityClass {
    /// Returns true when a disambiguation or drift disclosure is required.
    pub const fn requires_disambiguation(self) -> bool {
        !matches!(self, Self::Unambiguous)
    }
}

/// Scope completeness class for a target or relation set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeCompleteness {
    /// Result set is complete for the declared materialized scope.
    CompleteForDeclaredScope,
    /// Result set is partial for the declared scope.
    PartialForDeclaredScope,
    /// Result set is stale for the declared scope.
    StaleForDeclaredScope,
    /// Result set is unavailable for the declared scope.
    UnavailableForDeclaredScope,
}

impl ScopeCompleteness {
    /// Returns true when the scope completeness requires a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::CompleteForDeclaredScope)
    }
}

/// Authorship and mutation posture for an anchor or occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedOrExternalState {
    /// Current authored workspace source.
    AuthoredSource,
    /// Generated or paired source.
    GeneratedSource,
    /// External dependency, vendored snapshot, or imported package source.
    ExternalDependency,
    /// Read-only or protected source.
    ReadOnlySource,
    /// Imported snapshot or docs-pack-only source.
    ImportedSnapshot,
}

/// User-visible or export-visible reason a row must be downgraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeReason {
    /// Current workset or sparse scope omits requested rows.
    SparseWorkset,
    /// Policy, trust, or protected source narrowed the result.
    PolicyLimited,
    /// Provider or capability needed for the requested relation is missing.
    MissingProvider,
    /// Provider or index was unavailable.
    ProviderUnavailable,
    /// Graph, index, or provider shard is stale.
    StaleShard,
    /// Result came from lexical fallback.
    LexicalFallbackOnly,
    /// Result came from syntax fallback.
    SyntaxFallbackOnly,
    /// Generated or imported boundary prevented direct source truth.
    GeneratedBoundary,
    /// Runtime or framework proof exists without direct semantic replacement.
    RuntimeOrFrameworkOnly,
    /// Ambiguous candidates require explicit selection.
    AmbiguousCandidates,
    /// Bookmark, history, or breadcrumb target drifted.
    BookmarkOrHistoryDrift,
    /// Support or review export redacted row bodies and kept metadata only.
    MetadataOnlyExport,
}

/// Redaction class applied to navigation evidence exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRedactionClass {
    /// Metadata-safe projection without source bodies.
    MetadataSafeDefault,
    /// Operator-only restricted projection.
    OperatorOnlyRestricted,
    /// Internal support restricted projection.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
}

/// Consumer surface that must preserve target-model vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Product UI surface such as editor, search result pane, or hierarchy tree.
    EditorUi,
    /// CLI, SDK, headless, or automated inspection surface.
    CliHeadless,
    /// AI context picker, composer, or tool-call evidence surface.
    AiContext,
    /// Review workspace, review pack, or hosted review evidence surface.
    ReviewWorkspace,
    /// Support bundle or export packet.
    SupportExport,
    /// Graph overlay or topology surface.
    GraphOverlay,
    /// Shell breadcrumbs, outline, bookmarks, history, or peek continuity surface.
    ShellContinuity,
}

impl ConsumerSurface {
    /// Returns the stable token serialized into fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorUi => "editor_ui",
            Self::CliHeadless => "cli_headless",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::SupportExport => "support_export",
            Self::GraphOverlay => "graph_overlay",
            Self::ShellContinuity => "shell_continuity",
        }
    }
}

/// Consumer surfaces that must be covered by the protected beta corpus.
pub const REQUIRED_CONSUMER_SURFACES: [ConsumerSurface; 7] = [
    ConsumerSurface::EditorUi,
    ConsumerSurface::CliHeadless,
    ConsumerSurface::AiContext,
    ConsumerSurface::ReviewWorkspace,
    ConsumerSurface::SupportExport,
    ConsumerSurface::GraphOverlay,
    ConsumerSurface::ShellContinuity,
];

/// Kind of hierarchy edge represented by [`HierarchyEdge`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HierarchyEdgeKind {
    /// Static caller-to-callee call relation.
    Calls,
    /// Runtime-observed call relation.
    RuntimeCalls,
    /// Type inheritance relation.
    Inherits,
    /// Implementation relation.
    Implements,
    /// Override relation.
    Overrides,
    /// Framework or route-binding relation.
    FrameworkBinding,
    /// Ownership or stewardship relation.
    Owner,
    /// Documentation link relation.
    DocumentedBy,
}

/// Rename apply posture derived from preview completeness and blocked candidates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameApplyPosture {
    /// Preview may be applied after explicit review.
    ReadyForApplyAfterPreview,
    /// Scope must be widened or reviewed first.
    BlockedPendingScopeReview,
    /// Provider or index refresh is required first.
    BlockedPendingRefresh,
    /// Policy, protected, generated, or read-only rows require review.
    BlockedPendingPolicyOrProtectedReview,
    /// Preview is inspect-only and cannot be applied.
    InspectOnlyUnavailable,
}

impl RenameApplyPosture {
    /// Returns true when direct apply is blocked.
    pub const fn blocks_apply(self) -> bool {
        !matches!(self, Self::ReadyForApplyAfterPreview)
    }
}

/// Durable target reference used by breadcrumbs, bookmarks, and downstream projections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetRef {
    /// Stable target id.
    pub target_id: String,
    /// Relation kind the target represents.
    pub relation_kind: RelationKind,
    /// Stable object ref resolved by the owning subsystem.
    pub object_ref: String,
    /// Stable anchor ref resolved by the owning subsystem.
    pub anchor_ref: String,
}

/// Stable target selected by go-to, outline, breadcrumb, hierarchy, or search navigation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTarget {
    /// Stable target id.
    pub target_id: String,
    /// Relation kind represented by the target.
    pub relation_kind: RelationKind,
    /// Stable object ref resolved by the owning subsystem.
    pub object_ref: String,
    /// Stable anchor ref resolved by the owning subsystem.
    pub anchor_ref: String,
    /// Provider family that admitted the target.
    pub provider_class: ProviderClass,
    /// Proof class for the target relation.
    pub proof_class: ProofClass,
    /// Confidence class for the target.
    pub confidence: NavigationConfidence,
    /// Freshness class for the target.
    pub freshness: FreshnessClass,
    /// Ambiguity class for the selected target.
    pub ambiguity_class: AmbiguityClass,
    /// Completeness of the materialized scope.
    pub scope_completeness: ScopeCompleteness,
    /// Scope ref covered by this target.
    pub scope_ref: String,
    /// Authorship, generated, imported, or read-only posture.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Downgrade reasons that must be visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl NavigationTarget {
    /// Returns a compact target ref suitable for continuity and disambiguation records.
    pub fn target_ref(&self) -> NavigationTargetRef {
        NavigationTargetRef {
            target_id: self.target_id.clone(),
            relation_kind: self.relation_kind,
            object_ref: self.object_ref.clone(),
            anchor_ref: self.anchor_ref.clone(),
        }
    }

    /// Returns true when the target cannot be rendered as an unquestioned success.
    pub fn requires_downgrade_disclosure(&self) -> bool {
        self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.ambiguity_class.requires_disambiguation()
            || self.scope_completeness.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
    }
}

/// One reference occurrence in a find-references, rename-preview, or evidence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceOccurrence {
    /// Stable occurrence id.
    pub occurrence_id: String,
    /// Target this occurrence references.
    pub target_ref: String,
    /// Stable source anchor ref for the occurrence.
    pub anchor_ref: String,
    /// Access kind for the occurrence.
    pub access_kind: AccessKind,
    /// Scope containing the occurrence.
    pub scope_ref: String,
    /// Authorship, generated, imported, or read-only posture.
    pub generated_or_external_state: GeneratedOrExternalState,
    /// Proof class for this occurrence.
    pub proof_class: ProofClass,
    /// Confidence class for this occurrence.
    pub confidence: NavigationConfidence,
    /// Freshness class for this occurrence.
    pub freshness: FreshnessClass,
    /// Completeness of the materialized reference scope.
    pub scope_completeness: ScopeCompleteness,
    /// Downgrade reasons that must be visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl ReferenceOccurrence {
    /// Returns true when the occurrence must render with caveats.
    pub fn requires_downgrade_disclosure(&self) -> bool {
        self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
    }
}

/// One call, type, ownership, route, or documentation hierarchy edge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyEdge {
    /// Stable edge id.
    pub edge_id: String,
    /// Source target ref.
    pub source_ref: String,
    /// Target target ref.
    pub target_ref: String,
    /// Hierarchy edge kind.
    pub edge_kind: HierarchyEdgeKind,
    /// Proof class for the edge.
    pub proof_class: ProofClass,
    /// Zero-based or one-based depth as emitted by the owning hierarchy provider.
    pub depth: u32,
    /// Completeness of the hierarchy scope.
    pub scope_completeness: ScopeCompleteness,
    /// Freshness class for the edge.
    pub freshness: FreshnessClass,
    /// Confidence class for the edge.
    pub confidence: NavigationConfidence,
    /// Runtime or framework evidence refs; empty for direct semantic edges.
    pub runtime_or_framework_evidence_refs: Vec<String>,
    /// Downgrade reasons that must be visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Export-safe summary.
    pub summary: String,
}

impl HierarchyEdge {
    /// Returns true when the edge must render with caveats.
    pub fn requires_downgrade_disclosure(&self) -> bool {
        self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || !self.downgrade_reasons.is_empty()
    }
}

/// Count summary for a navigation-target rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetCountSummary {
    /// Occurrences the preview can change in the materialized scope.
    pub changed_count: usize,
    /// Candidate occurrences that could not be anchored.
    pub unresolved_count: usize,
    /// Generated or paired-artifact occurrences involved.
    pub generated_count: usize,
    /// Read-only or protected occurrences involved.
    pub protected_count: usize,
    /// Occurrences intentionally omitted from the proposed change.
    pub skipped_count: usize,
}

/// Rename preview expressed in terms of stable reference occurrence ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewSet {
    /// Stable rename preview id.
    pub rename_preview_id: String,
    /// Root target being renamed.
    pub root_target_ref: String,
    /// Candidate occurrence refs considered by the preview.
    pub candidate_occurrence_refs: Vec<String>,
    /// Candidate refs blocked by policy, generation, read-only state, or missing anchors.
    pub blocked_refs: Vec<String>,
    /// Conflict notes such as shadowing or alias ambiguity.
    pub conflict_notes: Vec<String>,
    /// Sparse, partial, stale, or provider-limit reasons.
    pub sparse_or_partial_reasons: Vec<String>,
    /// Generated-scope notes that must remain visible.
    pub generated_scope_notes: Vec<String>,
    /// Count summary for candidate coverage.
    pub count_summary: NavigationTargetCountSummary,
    /// Proof class for the preview set.
    pub proof_class: ProofClass,
    /// Confidence class for the preview set.
    pub confidence: NavigationConfidence,
    /// Freshness class for the preview set.
    pub freshness: FreshnessClass,
    /// Completeness of the materialized rename scope.
    pub scope_completeness: ScopeCompleteness,
    /// Apply posture for the preview.
    pub apply_posture: RenameApplyPosture,
    /// Redaction class for review/support/AI export.
    pub redaction_class: ExportRedactionClass,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl RenamePreviewSet {
    /// Returns true when the preview must render with caveats or cannot be directly applied.
    pub fn requires_downgrade_disclosure(&self) -> bool {
        self.apply_posture.blocks_apply()
            || self.proof_class.requires_disclosure()
            || self.confidence.requires_disclosure()
            || self.freshness.requires_disclosure()
            || self.scope_completeness.requires_disclosure()
            || !self.blocked_refs.is_empty()
            || !self.conflict_notes.is_empty()
            || !self.sparse_or_partial_reasons.is_empty()
            || !self.generated_scope_notes.is_empty()
            || self.count_summary.unresolved_count > 0
            || self.count_summary.generated_count > 0
            || self.count_summary.protected_count > 0
            || self.count_summary.skipped_count > 0
    }
}

/// Disambiguation set emitted when a navigation request has multiple truthful candidates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationDisambiguationSet {
    /// Stable disambiguation set id.
    pub set_id: String,
    /// Relation requested by the caller.
    pub requested_relation: RelationKind,
    /// Candidate target refs available for selection.
    pub candidate_target_refs: Vec<String>,
    /// Selection policy used by UI, CLI, AI, review, or support consumers.
    pub selection_policy: String,
    /// ISO 8601 UTC creation timestamp.
    pub created_at: String,
    /// Ambiguity class represented by this set.
    pub ambiguity_class: AmbiguityClass,
    /// Confidence class for this set.
    pub confidence: NavigationConfidence,
    /// Freshness class for this set.
    pub freshness: FreshnessClass,
    /// Completeness of the materialized candidate scope.
    pub scope_completeness: ScopeCompleteness,
    /// Downgrade reasons that must be visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Evidence refs safe for support, review, AI, and CLI consumers.
    pub evidence_refs: Vec<String>,
    /// Export-safe summary.
    pub summary: String,
}

/// Navigation continuity artifact family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityArtifactKind {
    /// Breadcrumb trail segment or trail target.
    Breadcrumb,
    /// Outline node or outline snapshot target.
    Outline,
    /// Bookmark or mark target.
    Bookmark,
    /// Back/forward or recent-location history target.
    History,
    /// Peek context target.
    Peek,
}

/// Continuity state for breadcrumbs, outline, bookmarks, history, and peek targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityState {
    /// Target still resolves exactly.
    Bound,
    /// Target moved and an authoritative remap succeeded.
    Remapped,
    /// Target drifted and requires inspection before open.
    Drifted,
    /// Target is missing from the current scope.
    MissingTarget,
    /// Target exists outside the active workset, branch, policy, pack, or remote shard.
    ScopeUnavailable,
    /// Target is retained only as an archive/tombstone.
    Archived,
}

impl ContinuityState {
    /// Returns true when a continuity artifact cannot auto-open without disclosure.
    pub const fn requires_user_review(self) -> bool {
        !matches!(self, Self::Bound | Self::Remapped)
    }
}

/// Breadcrumb, outline, bookmark, history, or peek target projection bound to the same target model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TargetContinuityRef {
    /// Stable continuity record id.
    pub continuity_ref_id: String,
    /// Continuity artifact family.
    pub artifact_kind: ContinuityArtifactKind,
    /// Original target ref captured by the artifact.
    pub target_ref: String,
    /// Current continuity state.
    pub continuity_state: ContinuityState,
    /// New target ref when an exact remap succeeded.
    pub remapped_target_ref: Option<String>,
    /// Disambiguation set ref when the user must choose a successor.
    pub disambiguation_set_ref: Option<String>,
    /// Downgrade reasons that must be visible on consumers.
    pub downgrade_reasons: Vec<DowngradeReason>,
    /// Export-safe summary.
    pub summary: String,
}

/// Surface-level projection proving the target model survives UI, CLI, AI, review, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerProjection {
    /// Surface consuming the target model.
    pub consumer_surface: ConsumerSurface,
    /// Target refs projected to this surface.
    pub projected_target_refs: Vec<String>,
    /// Reference occurrence refs projected to this surface.
    pub projected_reference_refs: Vec<String>,
    /// Hierarchy edge refs projected to this surface.
    pub projected_edge_refs: Vec<String>,
    /// Rename preview refs projected to this surface.
    pub projected_rename_preview_refs: Vec<String>,
    /// Disambiguation set refs projected to this surface.
    pub projected_disambiguation_refs: Vec<String>,
    /// Continuity refs projected to this surface.
    pub projected_continuity_refs: Vec<String>,
    /// Redaction class for this projection.
    pub redaction_class: ExportRedactionClass,
    /// True when relation-kind labels are preserved.
    pub includes_relation_kind_labels: bool,
    /// True when access-kind labels are preserved for references.
    pub includes_access_kind_labels: bool,
    /// True when proof-class labels are preserved.
    pub includes_proof_class_labels: bool,
    /// True when scope-completeness labels are preserved.
    pub includes_scope_completeness_labels: bool,
    /// True when the projection exported raw code bodies.
    pub exports_code_bodies: bool,
    /// Export-safe summary.
    pub summary: String,
}

/// Path refs that every fidelity fixture pins to avoid doc/schema/report drift.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFidelityReferences {
    /// Repo-relative schema ref.
    pub schema_ref: String,
    /// Repo-relative reviewer doc ref.
    pub doc_ref: String,
    /// Repo-relative report ref.
    pub report_ref: String,
}

/// One protected regression case for navigation target fidelity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFidelityCase {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: NavigationTargetModelVersion,
    /// Stable case id.
    pub case_id: String,
    /// Plain-language title.
    pub title: String,
    /// Launch language lane or cross-surface lane exercised by the case.
    pub language_lane: String,
    /// Target records exercised by the case.
    pub navigation_targets: Vec<NavigationTarget>,
    /// Reference occurrence records exercised by the case.
    pub reference_occurrences: Vec<ReferenceOccurrence>,
    /// Hierarchy edge records exercised by the case.
    pub hierarchy_edges: Vec<HierarchyEdge>,
    /// Rename preview sets exercised by the case.
    pub rename_preview_sets: Vec<RenamePreviewSet>,
    /// Disambiguation sets exercised by the case.
    pub disambiguation_sets: Vec<NavigationDisambiguationSet>,
    /// Breadcrumb/bookmark/history continuity refs exercised by the case.
    pub continuity_refs: Vec<TargetContinuityRef>,
    /// Consumer projections proving model parity across surfaces.
    pub consumer_projections: Vec<ConsumerProjection>,
    /// Case-level downgrade reasons that must remain visible.
    pub expected_downgrade_reasons: Vec<DowngradeReason>,
    /// Cross-file references for schema, docs, and release evidence.
    pub references: NavigationTargetFidelityReferences,
    /// Export-safe case summary.
    pub export_safe_summary: String,
}

/// One entry in the checked-in target fidelity corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTargetFidelityCorpusEntry {
    /// Repo-relative fixture ref.
    pub fixture_ref: String,
    /// Parsed case.
    pub case: NavigationTargetFidelityCase,
}

/// Loaded target fidelity corpus.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationTargetFidelityCorpus {
    /// Parsed corpus entries.
    pub entries: Vec<NavigationTargetFidelityCorpusEntry>,
}

/// Promotion state derived from validating the fidelity corpus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationPromotionState {
    /// Fidelity cases are promotable for the claimed beta rows.
    Promotable,
    /// Fidelity cases pass with caveats that must remain visible.
    NeedsReview,
    /// Fidelity cases block promotion.
    Blocked,
}

impl NavigationPromotionState {
    /// Returns the stable token serialized into reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Promotable => "promotable",
            Self::NeedsReview => "needs_review",
            Self::Blocked => "blocked",
        }
    }
}

/// One validation violation emitted by the fidelity evaluator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFidelityViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject ref that failed the check.
    pub subject_ref: String,
    /// Plain-language violation message.
    pub message: String,
}

/// One report row for a target fidelity case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFidelityReportRow {
    /// Case id.
    pub case_id: String,
    /// Fixture ref.
    pub fixture_ref: String,
    /// Language lane covered by the case.
    pub language_lane: String,
    /// Target count.
    pub target_count: usize,
    /// Reference occurrence count.
    pub reference_occurrence_count: usize,
    /// Hierarchy edge count.
    pub hierarchy_edge_count: usize,
    /// Rename preview count.
    pub rename_preview_count: usize,
    /// Disambiguation set count.
    pub disambiguation_set_count: usize,
    /// Continuity ref count.
    pub continuity_ref_count: usize,
    /// Consumer surfaces covered by this case.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Promotion state for this case.
    pub promotion_state: NavigationPromotionState,
    /// Export-safe summary.
    pub summary: String,
}

/// Aggregate validation report for the target fidelity corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetFidelityReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: NavigationTargetModelVersion,
    /// Repo-relative schema ref.
    pub schema_ref: String,
    /// Repo-relative reviewer doc ref.
    pub doc_ref: String,
    /// Repo-relative protected corpus directory.
    pub corpus_ref: String,
    /// Repo-relative report ref.
    pub report_ref: String,
    /// Per-case report rows.
    pub rows: Vec<NavigationTargetFidelityReportRow>,
    /// Validation violations.
    pub violations: Vec<NavigationTargetFidelityViolation>,
    /// Aggregate promotion state.
    pub promotion_state: NavigationPromotionState,
    /// Export-safe summary.
    pub summary: String,
}

/// Validation and report-building error for the target model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NavigationTargetModelError {
    /// Corpus did not cover a required consumer surface.
    MissingConsumerSurface {
        /// Missing surface.
        surface: ConsumerSurface,
    },
    /// Corpus did not cover a required relation kind.
    MissingRelationKind {
        /// Missing relation kind.
        relation_kind: RelationKind,
    },
}

impl fmt::Display for NavigationTargetModelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingConsumerSurface { surface } => {
                write!(
                    formatter,
                    "navigation target fidelity corpus is missing consumer surface {}",
                    surface.as_str()
                )
            }
            Self::MissingRelationKind { relation_kind } => {
                write!(
                    formatter,
                    "navigation target fidelity corpus is missing relation kind {}",
                    relation_kind.as_str()
                )
            }
        }
    }
}

impl Error for NavigationTargetModelError {}

/// Stateless evaluator for the protected navigation target fidelity corpus.
#[derive(Debug, Default, Clone)]
pub struct NavigationTargetFidelityEvaluator;

impl NavigationTargetFidelityEvaluator {
    /// Validates one fidelity case and returns all findings.
    pub fn validate_case(
        &self,
        case: &NavigationTargetFidelityCase,
    ) -> Vec<NavigationTargetFidelityViolation> {
        let mut violations = Vec::new();
        self.validate_case_identity(case, &mut violations);
        self.validate_targets(case, &mut violations);
        self.validate_references(case, &mut violations);
        self.validate_hierarchy_edges(case, &mut violations);
        self.validate_rename_sets(case, &mut violations);
        self.validate_disambiguation_sets(case, &mut violations);
        self.validate_continuity_refs(case, &mut violations);
        self.validate_consumer_projections(case, &mut violations);
        violations
    }

    /// Builds an aggregate fidelity report for a corpus.
    pub fn build_report(
        &self,
        corpus: &NavigationTargetFidelityCorpus,
    ) -> NavigationTargetFidelityReport {
        let mut violations = Vec::new();
        let mut rows = Vec::new();
        let mut covered_surfaces = BTreeSet::new();
        let mut covered_relation_kinds = BTreeSet::new();

        for entry in &corpus.entries {
            let mut case_violations = self.validate_case(&entry.case);
            let case_state = if case_violations.is_empty() {
                case_promotion_state(&entry.case)
            } else {
                NavigationPromotionState::Blocked
            };
            violations.append(&mut case_violations);
            for projection in &entry.case.consumer_projections {
                covered_surfaces.insert(projection.consumer_surface);
            }
            for target in &entry.case.navigation_targets {
                covered_relation_kinds.insert(target.relation_kind);
            }
            for occurrence in &entry.case.reference_occurrences {
                relation_kind_for_access(occurrence.access_kind, &mut covered_relation_kinds);
            }
            rows.push(NavigationTargetFidelityReportRow {
                case_id: entry.case.case_id.clone(),
                fixture_ref: entry.fixture_ref.clone(),
                language_lane: entry.case.language_lane.clone(),
                target_count: entry.case.navigation_targets.len(),
                reference_occurrence_count: entry.case.reference_occurrences.len(),
                hierarchy_edge_count: entry.case.hierarchy_edges.len(),
                rename_preview_count: entry.case.rename_preview_sets.len(),
                disambiguation_set_count: entry.case.disambiguation_sets.len(),
                continuity_ref_count: entry.case.continuity_refs.len(),
                consumer_surfaces: entry
                    .case
                    .consumer_projections
                    .iter()
                    .map(|projection| projection.consumer_surface)
                    .collect(),
                promotion_state: case_state,
                summary: entry.case.export_safe_summary.clone(),
            });
        }

        for surface in REQUIRED_CONSUMER_SURFACES {
            if !covered_surfaces.contains(&surface) {
                push_violation(
                    &mut violations,
                    "corpus.required_consumer_surface_missing",
                    surface.as_str(),
                    format!("corpus must cover consumer surface {}", surface.as_str()),
                );
            }
        }
        for relation_kind in REQUIRED_RELATION_KINDS {
            if !covered_relation_kinds.contains(&relation_kind) {
                push_violation(
                    &mut violations,
                    "corpus.required_relation_kind_missing",
                    relation_kind.as_str(),
                    format!("corpus must cover relation kind {}", relation_kind.as_str()),
                );
            }
        }

        let promotion_state = if violations
            .iter()
            .any(|violation| violation.check_id.contains("blocked"))
            || !violations.is_empty()
        {
            NavigationPromotionState::Blocked
        } else if rows
            .iter()
            .any(|row| row.promotion_state == NavigationPromotionState::NeedsReview)
        {
            NavigationPromotionState::NeedsReview
        } else {
            NavigationPromotionState::Promotable
        };

        NavigationTargetFidelityReport {
            record_kind: NAVIGATION_TARGET_FIDELITY_REPORT_RECORD_KIND.to_owned(),
            schema_version: NAVIGATION_TARGET_SCHEMA_VERSION,
            schema_ref: NAVIGATION_TARGET_SCHEMA_REF.to_owned(),
            doc_ref: NAVIGATION_TARGET_BETA_CONTRACT_DOC_REF.to_owned(),
            corpus_ref: NAVIGATION_TARGET_FIDELITY_CORPUS_DIR.to_owned(),
            report_ref: NAVIGATION_TARGET_FIDELITY_REPORT_REF.to_owned(),
            rows,
            violations,
            promotion_state,
            summary: "Navigation target fidelity corpus preserves typed relation, access, proof, freshness, ambiguity, and scope-completeness labels across beta surfaces.".to_owned(),
        }
    }

    fn validate_case_identity(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        if case.record_kind != NAVIGATION_TARGET_FIDELITY_CASE_RECORD_KIND {
            push_violation(
                violations,
                "case.record_kind",
                &case.case_id,
                format!("record_kind must be {NAVIGATION_TARGET_FIDELITY_CASE_RECORD_KIND}"),
            );
        }
        if case.schema_version != NAVIGATION_TARGET_SCHEMA_VERSION {
            push_violation(
                violations,
                "case.schema_version",
                &case.case_id,
                format!("schema_version must be {NAVIGATION_TARGET_SCHEMA_VERSION}"),
            );
        }
        if case.references.schema_ref != NAVIGATION_TARGET_SCHEMA_REF {
            push_violation(
                violations,
                "case.references.schema_ref",
                &case.case_id,
                format!("references.schema_ref must pin {NAVIGATION_TARGET_SCHEMA_REF}"),
            );
        }
        if case.references.doc_ref != NAVIGATION_TARGET_BETA_CONTRACT_DOC_REF {
            push_violation(
                violations,
                "case.references.doc_ref",
                &case.case_id,
                format!("references.doc_ref must pin {NAVIGATION_TARGET_BETA_CONTRACT_DOC_REF}"),
            );
        }
        if case.references.report_ref != NAVIGATION_TARGET_FIDELITY_REPORT_REF {
            push_violation(
                violations,
                "case.references.report_ref",
                &case.case_id,
                format!("references.report_ref must pin {NAVIGATION_TARGET_FIDELITY_REPORT_REF}"),
            );
        }
    }

    fn validate_targets(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        if case.navigation_targets.is_empty() {
            push_violation(
                violations,
                "case.navigation_targets.empty",
                &case.case_id,
                "case must include at least one navigation target",
            );
        }
        for target in &case.navigation_targets {
            validate_nonempty(
                violations,
                "target.target_id.empty",
                &target.target_id,
                &target.target_id,
                "target_id must be non-empty",
            );
            validate_nonempty(
                violations,
                "target.object_ref.empty",
                &target.target_id,
                &target.object_ref,
                "object_ref must be non-empty",
            );
            validate_nonempty(
                violations,
                "target.anchor_ref.empty",
                &target.target_id,
                &target.anchor_ref,
                "anchor_ref must be non-empty",
            );
            if target.requires_downgrade_disclosure() && target.downgrade_reasons.is_empty() {
                push_violation(
                    violations,
                    "target.downgrade_reason_missing",
                    &target.target_id,
                    "degraded target must carry at least one downgrade reason",
                );
            }
            if target.ambiguity_class.requires_disambiguation()
                && !case
                    .disambiguation_sets
                    .iter()
                    .any(|set| set.candidate_target_refs.contains(&target.target_id))
            {
                push_violation(
                    violations,
                    "target.disambiguation_set_missing",
                    &target.target_id,
                    "ambiguous or drifted target must be present in a disambiguation set",
                );
            }
        }
    }

    fn validate_references(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        for occurrence in &case.reference_occurrences {
            validate_nonempty(
                violations,
                "reference.occurrence_id.empty",
                &occurrence.occurrence_id,
                &occurrence.occurrence_id,
                "occurrence_id must be non-empty",
            );
            validate_nonempty(
                violations,
                "reference.anchor_ref.empty",
                &occurrence.occurrence_id,
                &occurrence.anchor_ref,
                "anchor_ref must be non-empty",
            );
            if occurrence.requires_downgrade_disclosure() && occurrence.downgrade_reasons.is_empty()
            {
                push_violation(
                    violations,
                    "reference.downgrade_reason_missing",
                    &occurrence.occurrence_id,
                    "degraded reference occurrence must carry at least one downgrade reason",
                );
            }
        }
    }

    fn validate_hierarchy_edges(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        for edge in &case.hierarchy_edges {
            validate_nonempty(
                violations,
                "hierarchy.edge_id.empty",
                &edge.edge_id,
                &edge.edge_id,
                "edge_id must be non-empty",
            );
            if matches!(
                edge.proof_class,
                ProofClass::FrameworkDerived | ProofClass::RuntimeObserved
            ) && edge.runtime_or_framework_evidence_refs.is_empty()
            {
                push_violation(
                    violations,
                    "hierarchy.runtime_or_framework_evidence_missing",
                    &edge.edge_id,
                    "runtime/framework-derived edge must preserve evidence refs",
                );
            }
            if edge.requires_downgrade_disclosure() && edge.downgrade_reasons.is_empty() {
                push_violation(
                    violations,
                    "hierarchy.downgrade_reason_missing",
                    &edge.edge_id,
                    "degraded hierarchy edge must carry at least one downgrade reason",
                );
            }
        }
    }

    fn validate_rename_sets(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        for preview in &case.rename_preview_sets {
            validate_nonempty(
                violations,
                "rename.rename_preview_id.empty",
                &preview.rename_preview_id,
                &preview.rename_preview_id,
                "rename_preview_id must be non-empty",
            );
            if preview.count_summary.generated_count > 0 && preview.generated_scope_notes.is_empty()
            {
                push_violation(
                    violations,
                    "rename.generated_scope_notes_missing",
                    &preview.rename_preview_id,
                    "rename preview with generated candidates must preserve generated scope notes",
                );
            }
            if (preview.count_summary.protected_count > 0
                || preview.count_summary.skipped_count > 0
                || preview.count_summary.unresolved_count > 0)
                && preview.blocked_refs.is_empty()
            {
                push_violation(
                    violations,
                    "rename.blocked_refs_missing",
                    &preview.rename_preview_id,
                    "rename preview with blocked or omitted candidates must preserve blocked_refs",
                );
            }
            if preview.scope_completeness.requires_disclosure()
                && preview.sparse_or_partial_reasons.is_empty()
            {
                push_violation(
                    violations,
                    "rename.partial_reasons_missing",
                    &preview.rename_preview_id,
                    "partial rename preview must preserve sparse_or_partial_reasons",
                );
            }
        }
    }

    fn validate_disambiguation_sets(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        for set in &case.disambiguation_sets {
            if set.candidate_target_refs.len() < 2 {
                push_violation(
                    violations,
                    "disambiguation.candidate_count_too_low",
                    &set.set_id,
                    "disambiguation set must carry at least two candidates",
                );
            }
            if set.selection_policy.trim().is_empty() {
                push_violation(
                    violations,
                    "disambiguation.selection_policy_empty",
                    &set.set_id,
                    "selection_policy must explain how callers choose a target",
                );
            }
        }
    }

    fn validate_continuity_refs(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        for continuity in &case.continuity_refs {
            if continuity.continuity_state.requires_user_review()
                && continuity.disambiguation_set_ref.is_none()
                && continuity.remapped_target_ref.is_none()
                && continuity.downgrade_reasons.is_empty()
            {
                push_violation(
                    violations,
                    "continuity.review_path_missing",
                    &continuity.continuity_ref_id,
                    "drifted or missing continuity ref must preserve a review, remap, or downgrade path",
                );
            }
        }
    }

    fn validate_consumer_projections(
        &self,
        case: &NavigationTargetFidelityCase,
        violations: &mut Vec<NavigationTargetFidelityViolation>,
    ) {
        if case.consumer_projections.is_empty() {
            push_violation(
                violations,
                "case.consumer_projections.empty",
                &case.case_id,
                "case must include at least one consumer projection",
            );
        }
        for projection in &case.consumer_projections {
            if !projection.includes_relation_kind_labels {
                push_violation(
                    violations,
                    "projection.relation_labels_missing",
                    projection.consumer_surface.as_str(),
                    "consumer projection must preserve relation_kind labels",
                );
            }
            if !projection.includes_access_kind_labels {
                push_violation(
                    violations,
                    "projection.access_labels_missing",
                    projection.consumer_surface.as_str(),
                    "consumer projection must preserve access_kind labels",
                );
            }
            if !projection.includes_proof_class_labels {
                push_violation(
                    violations,
                    "projection.proof_labels_missing",
                    projection.consumer_surface.as_str(),
                    "consumer projection must preserve proof_class labels",
                );
            }
            if !projection.includes_scope_completeness_labels {
                push_violation(
                    violations,
                    "projection.scope_labels_missing",
                    projection.consumer_surface.as_str(),
                    "consumer projection must preserve scope_completeness labels",
                );
            }
            if projection.exports_code_bodies {
                push_violation(
                    violations,
                    "projection.raw_code_body_export_blocked",
                    projection.consumer_surface.as_str(),
                    "consumer projection must not export raw code bodies",
                );
            }
        }
    }
}

/// Loads a YAML-encoded [`NavigationTargetFidelityCase`].
pub fn load_navigation_target_fidelity_case(
    yaml: &str,
) -> Result<NavigationTargetFidelityCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in navigation target fidelity corpus.
pub fn current_navigation_target_fidelity_corpus(
) -> Result<NavigationTargetFidelityCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<NavigationTargetFidelityCase>(yaml).map(|case| {
                NavigationTargetFidelityCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(NavigationTargetFidelityCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration order.
pub fn current_navigation_target_fidelity_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

fn case_promotion_state(case: &NavigationTargetFidelityCase) -> NavigationPromotionState {
    if case
        .expected_downgrade_reasons
        .iter()
        .any(|reason| matches!(reason, DowngradeReason::MissingProvider))
    {
        NavigationPromotionState::Blocked
    } else if !case.expected_downgrade_reasons.is_empty()
        || case
            .navigation_targets
            .iter()
            .any(NavigationTarget::requires_downgrade_disclosure)
        || case
            .reference_occurrences
            .iter()
            .any(ReferenceOccurrence::requires_downgrade_disclosure)
        || case
            .hierarchy_edges
            .iter()
            .any(HierarchyEdge::requires_downgrade_disclosure)
        || case
            .rename_preview_sets
            .iter()
            .any(RenamePreviewSet::requires_downgrade_disclosure)
        || case
            .continuity_refs
            .iter()
            .any(|continuity| continuity.continuity_state.requires_user_review())
    {
        NavigationPromotionState::NeedsReview
    } else {
        NavigationPromotionState::Promotable
    }
}

fn relation_kind_for_access(access_kind: AccessKind, relation_kinds: &mut BTreeSet<RelationKind>) {
    match access_kind {
        AccessKind::Call => {
            relation_kinds.insert(RelationKind::Call);
            relation_kinds.insert(RelationKind::Reference);
        }
        _ => {
            relation_kinds.insert(RelationKind::Reference);
        }
    }
}

fn validate_nonempty(
    violations: &mut Vec<NavigationTargetFidelityViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    value: &str,
    message: impl Into<String>,
) {
    if value.trim().is_empty() {
        push_violation(violations, check_id, subject_ref, message);
    }
}

fn push_violation(
    violations: &mut Vec<NavigationTargetFidelityViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(NavigationTargetFidelityViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/navigation/m3/target_accuracy/cross_language_definition_reference.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/cross_language_definition_reference.yaml"
        )),
    ),
    (
        "fixtures/navigation/m3/target_accuracy/hierarchy_framework_runtime_edges.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/hierarchy_framework_runtime_edges.yaml"
        )),
    ),
    (
        "fixtures/navigation/m3/target_accuracy/generated_boundary_disambiguation.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/generated_boundary_disambiguation.yaml"
        )),
    ),
    (
        "fixtures/navigation/m3/target_accuracy/drifted_bookmark_breadcrumb.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/drifted_bookmark_breadcrumb.yaml"
        )),
    ),
    (
        "fixtures/navigation/m3/target_accuracy/rename_conflicts_partial_scope.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/rename_conflicts_partial_scope.yaml"
        )),
    ),
    (
        "fixtures/navigation/m3/target_accuracy/export_evidence_parity.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/navigation/m3/target_accuracy/export_evidence_parity.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_corpus_loads_and_validates() {
        let corpus = current_navigation_target_fidelity_corpus().expect("corpus loads");
        assert_eq!(corpus.entries.len(), 6);
        let report = NavigationTargetFidelityEvaluator.build_report(&corpus);
        assert_eq!(report.violations, Vec::new());
        assert_eq!(
            report.promotion_state,
            NavigationPromotionState::NeedsReview
        );
        assert_eq!(
            report.rows.len(),
            current_navigation_target_fidelity_fixture_refs().count()
        );
    }

    #[test]
    fn degraded_target_requires_downgrade_reason() {
        let mut case = current_navigation_target_fidelity_corpus()
            .expect("corpus loads")
            .entries
            .remove(2)
            .case;
        case.navigation_targets[0].downgrade_reasons.clear();
        let violations = NavigationTargetFidelityEvaluator.validate_case(&case);
        assert!(violations
            .iter()
            .any(|violation| violation.check_id == "target.downgrade_reason_missing"));
    }

    #[test]
    fn consumer_projection_blocks_raw_code_body_exports() {
        let mut case = current_navigation_target_fidelity_corpus()
            .expect("corpus loads")
            .entries
            .remove(0)
            .case;
        case.consumer_projections[0].exports_code_bodies = true;
        let violations = NavigationTargetFidelityEvaluator.validate_case(&case);
        assert!(violations
            .iter()
            .any(|violation| violation.check_id == "projection.raw_code_body_export_blocked"));
    }

    #[test]
    fn report_round_trips_through_json() {
        let corpus = current_navigation_target_fidelity_corpus().expect("corpus loads");
        let report = NavigationTargetFidelityEvaluator.build_report(&corpus);
        let json = serde_json::to_string(&report).expect("report serializes");
        let round_trip: NavigationTargetFidelityReport =
            serde_json::from_str(&json).expect("report deserializes");
        assert_eq!(round_trip, report);
    }
}
