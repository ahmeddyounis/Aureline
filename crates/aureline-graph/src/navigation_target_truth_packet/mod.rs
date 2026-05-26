//! Stable navigation-target truth packet covering find-references,
//! rename-preview, and impact-surfacing rows across launch languages.
//!
//! This module is the search-and-graph-owned contract for the M4 stable
//! lane that pins how navigation rows (`Go to Definition`,
//! `Go to Declaration`, `Go to Implementation`, `Find References`,
//! call/type hierarchy edges, related-object navigation, and
//! rename-preview sets) report relation kind, provider class, freshness,
//! ambiguity, scope completeness, downgrade state, and access kind to
//! every consumer surface. The packet binds the editor navigation pane,
//! graph topology canvas, AI context evidence, review workspace, support
//! export, CLI/headless inspector, and release proof index to one
//! shared vocabulary so a shallow provider can never silently alias
//! `Go to Definition` to a declaration jump without exporting an
//! explicit reason code.
//!
//! The packet is intentionally metadata-only — it never admits raw
//! query text, raw source bodies, secrets, ambient credentials, or
//! provider payloads. Reference rows carry an [`AccessKind`] (`read`,
//! `write`, `call`, `inherit`, `import`, `export`, `route-binding`,
//! `test-only`, `generated`, `runtime-observed`). Rename-preview rows
//! preserve blocked/generated/readonly/partially-loaded candidate
//! counts and sparse-scope/conflict notes even when code bodies are
//! redacted or hidden by policy. Hierarchy-edge rows preserve their
//! source and target object refs and edge kind. Any row whose
//! provider was too shallow to back the requested relation reports an
//! `aliased_due_to_shallow_provider` downgrade and a paired aliasing
//! context with the relation it was forced to fall back to and the
//! reviewable evidence ref.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`NavigationTargetTruthPacket`].
pub const NAVIGATION_TARGET_TRUTH_PACKET_RECORD_KIND: &str =
    "navigation_target_truth_stable_packet";

/// Stable record-kind tag for [`NavigationTargetTruthSupportExport`].
pub const NAVIGATION_TARGET_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str =
    "navigation_target_truth_support_export";

/// Integer schema version for stable navigation-target truth packets.
pub const NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const NAVIGATION_TARGET_TRUTH_SCHEMA_REF: &str =
    "schemas/search/navigation_target_truth_packet.schema.json";

/// Repo-relative path of the reviewer doc.
pub const NAVIGATION_TARGET_TRUTH_DOC_REF: &str =
    "docs/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md";

/// Repo-relative path of the human-readable reviewer artifact.
pub const NAVIGATION_TARGET_TRUTH_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/harden-find-references-rename-preview-and-impact-surfacing.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const NAVIGATION_TARGET_TRUTH_FIXTURE_DIR: &str =
    "fixtures/search/m4/navigation_target_truth_packet";

/// Repo-relative path of the checked-in stable navigation-target truth packet.
pub const NAVIGATION_TARGET_TRUTH_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/navigation_target_truth_packet.json";

/// Closed row-class vocabulary the packet certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowClass {
    /// `Go to Definition` row.
    Definition,
    /// `Go to Declaration` row.
    Declaration,
    /// `Go to Implementation` row.
    Implementation,
    /// `Find References` occurrence row.
    Reference,
    /// Call hierarchy edge row (caller -> callee).
    CallHierarchyEdge,
    /// Type hierarchy edge row (supertype/subtype/implements).
    TypeHierarchyEdge,
    /// Related-object navigation row (paired tests, docs, ownership).
    RelatedObject,
    /// Rename-preview row (proposed identifier change set).
    RenamePreview,
}

impl RowClass {
    /// Every required row class, in declaration order.
    pub const REQUIRED: [Self; 8] = [
        Self::Definition,
        Self::Declaration,
        Self::Implementation,
        Self::Reference,
        Self::CallHierarchyEdge,
        Self::TypeHierarchyEdge,
        Self::RelatedObject,
        Self::RenamePreview,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Declaration => "declaration",
            Self::Implementation => "implementation",
            Self::Reference => "reference",
            Self::CallHierarchyEdge => "call_hierarchy_edge",
            Self::TypeHierarchyEdge => "type_hierarchy_edge",
            Self::RelatedObject => "related_object",
            Self::RenamePreview => "rename_preview",
        }
    }
}

/// Closed relation-kind vocabulary shared across launch languages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RelationKind {
    /// Resolves to the implementation body or canonical definition site.
    Definition,
    /// Resolves to a declaration, interface, trait, or signature surface.
    Declaration,
    /// Resolves to one implementation candidate.
    Implementation,
    /// Resolves to a reference occurrence or reference set member.
    Reference,
    /// Resolves to a type, schema, trait, interface, or class target.
    Type,
    /// Represents a callable invocation or call-hierarchy relation.
    Call,
    /// Synthesized from routing, framework, or runtime binding metadata.
    RouteBinding,
    /// Links an owner, CODEOWNERS-like rule, or stewardship record.
    OwnerLink,
    /// Links documentation, examples, or generated docs anchors.
    DocLink,
    /// Links a paired test, fixture, or specification record.
    TestLink,
}

impl RelationKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Declaration => "declaration",
            Self::Implementation => "implementation",
            Self::Reference => "reference",
            Self::Type => "type",
            Self::Call => "call",
            Self::RouteBinding => "route_binding",
            Self::OwnerLink => "owner_link",
            Self::DocLink => "doc_link",
            Self::TestLink => "test_link",
        }
    }

    /// Relation kind expected to match a row class without aliasing.
    fn canonical_for_row_class(row_class: RowClass) -> Option<Self> {
        match row_class {
            RowClass::Definition => Some(Self::Definition),
            RowClass::Declaration => Some(Self::Declaration),
            RowClass::Implementation => Some(Self::Implementation),
            RowClass::Reference => Some(Self::Reference),
            _ => None,
        }
    }
}

/// Closed access-kind vocabulary preserved on reference and rename-candidate rows.
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
    /// Route, framework, or binding-level reference (not a direct call).
    RouteBinding,
    /// Occurrence is test-only and must not collapse into production counts.
    TestOnly,
    /// Occurrence is generated, mirrored, or framework-emitted.
    Generated,
    /// Occurrence was observed only by runtime trace, debugger, or profile.
    RuntimeObserved,
}

impl AccessKind {
    /// Every stable access kind launch surfaces must preserve.
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

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Write => "write",
            Self::Call => "call",
            Self::Inherit => "inherit",
            Self::Import => "import",
            Self::Export => "export",
            Self::RouteBinding => "route_binding",
            Self::TestOnly => "test_only",
            Self::Generated => "generated",
            Self::RuntimeObserved => "runtime_observed",
        }
    }
}

/// Closed provider-class vocabulary attached to a navigation truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderClass {
    /// Language-server protocol provider (deep semantic provider).
    LanguageServer,
    /// Project graph or semantic graph provider.
    ProjectGraph,
    /// Search index provider (file/symbol/text/path).
    SearchIndex,
    /// Framework-specific provider pack (route, schema, generator).
    FrameworkPack,
    /// Notebook or literate-programming adapter.
    NotebookAdapter,
    /// Generated-source lineage bridge.
    GeneratedSourceBridge,
    /// Runtime observer, debugger, trace, or profile provider.
    RuntimeObserver,
    /// Imported docs pack, snapshot, or provider overlay.
    ImportedSnapshot,
    /// Syntax tree / parser-only provider.
    SyntaxFallback,
    /// Remote index or managed workspace provider.
    RemoteIndex,
}

impl ProviderClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LanguageServer => "language_server",
            Self::ProjectGraph => "project_graph",
            Self::SearchIndex => "search_index",
            Self::FrameworkPack => "framework_pack",
            Self::NotebookAdapter => "notebook_adapter",
            Self::GeneratedSourceBridge => "generated_source_bridge",
            Self::RuntimeObserver => "runtime_observer",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::SyntaxFallback => "syntax_fallback",
            Self::RemoteIndex => "remote_index",
        }
    }

    /// True when the provider class is shallow enough to risk silent aliasing
    /// unless the row carries an explicit downgrade and aliasing context.
    pub const fn is_shallow(self) -> bool {
        matches!(
            self,
            Self::SyntaxFallback
                | Self::SearchIndex
                | Self::ImportedSnapshot
                | Self::RuntimeObserver
        )
    }
}

/// Closed freshness-class vocabulary attached to a navigation truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Revalidated against current source or runtime epoch.
    AuthoritativeLive,
    /// Warm cache known to be current enough for read-only navigation.
    WarmCached,
    /// Cache is usable only with an explicit downgrade disclosure.
    DegradedCached,
    /// Evidence is past its freshness floor.
    Stale,
    /// Evidence has not been verified against the current workspace.
    Unverified,
    /// Evidence captured from an imported-provider snapshot.
    ImportedSnapshot,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }
}

/// Closed ambiguity-class vocabulary attached to a navigation truth row.
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
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unambiguous => "unambiguous",
            Self::AmbiguousNeedsSelection => "ambiguous_needs_selection",
            Self::MultipleCandidatesRanked => "multiple_candidates_ranked",
            Self::DriftedNeedsReview => "drifted_needs_review",
            Self::MissingTarget => "missing_target",
            Self::ScopeUnavailable => "scope_unavailable",
        }
    }
}

/// Closed scope-completeness vocabulary attached to a navigation truth row.
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
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteForDeclaredScope => "complete_for_declared_scope",
            Self::PartialForDeclaredScope => "partial_for_declared_scope",
            Self::StaleForDeclaredScope => "stale_for_declared_scope",
            Self::UnavailableForDeclaredScope => "unavailable_for_declared_scope",
        }
    }
}

/// Closed downgrade-state vocabulary on a navigation truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeState {
    /// Row is canonical, no downgrade applied.
    Canonical,
    /// Row was forced to alias to a different relation because the provider was shallow.
    AliasedDueToShallowProvider,
    /// Row is partial because the index slice is still warming up.
    PartialIndexDisclosed,
    /// Row is stale; a disclosure shows when it was last verified.
    StaleDisclosed,
    /// Row crosses a generated-source boundary; direct semantic proof is unavailable.
    GeneratedBoundaryDisclosed,
    /// Row's only proof came from runtime or framework metadata.
    RuntimeOrFrameworkOnlyDisclosed,
    /// Row's only proof came from lexical fallback.
    LexicalFallbackDisclosed,
    /// Row's only proof came from syntax fallback.
    SyntaxFallbackDisclosed,
    /// Workset, branch, policy, or remote shard makes the target scope unavailable.
    ScopeUnavailableDisclosed,
    /// Row was contributed by an imported provider snapshot.
    ImportedSnapshotDisclosed,
}

impl DowngradeState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canonical => "canonical",
            Self::AliasedDueToShallowProvider => "aliased_due_to_shallow_provider",
            Self::PartialIndexDisclosed => "partial_index_disclosed",
            Self::StaleDisclosed => "stale_disclosed",
            Self::GeneratedBoundaryDisclosed => "generated_boundary_disclosed",
            Self::RuntimeOrFrameworkOnlyDisclosed => "runtime_or_framework_only_disclosed",
            Self::LexicalFallbackDisclosed => "lexical_fallback_disclosed",
            Self::SyntaxFallbackDisclosed => "syntax_fallback_disclosed",
            Self::ScopeUnavailableDisclosed => "scope_unavailable_disclosed",
            Self::ImportedSnapshotDisclosed => "imported_snapshot_disclosed",
        }
    }
}

/// Closed confidence-class vocabulary attached to a navigation truth row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Proven exactly against current source for the declared scope.
    Exact,
    /// Proven from a current index for the declared scope.
    Indexed,
    /// Useful but incomplete for the declared scope.
    Partial,
    /// Imported from a snapshot, docs pack, or provider overlay.
    Imported,
    /// Evidence is stale for the declared scope.
    Stale,
    /// Heuristic or fallback mapping.
    Heuristic,
    /// No admissible result exists.
    Unavailable,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Indexed => "indexed",
            Self::Partial => "partial",
            Self::Imported => "imported",
            Self::Stale => "stale",
            Self::Heuristic => "heuristic",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Closed hierarchy-edge kind vocabulary on call/type hierarchy rows.
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

impl HierarchyEdgeKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Calls => "calls",
            Self::RuntimeCalls => "runtime_calls",
            Self::Inherits => "inherits",
            Self::Implements => "implements",
            Self::Overrides => "overrides",
            Self::FrameworkBinding => "framework_binding",
            Self::Owner => "owner",
            Self::DocumentedBy => "documented_by",
        }
    }

    fn matches_row_class(self, row_class: RowClass) -> bool {
        match row_class {
            RowClass::CallHierarchyEdge => matches!(
                self,
                Self::Calls | Self::RuntimeCalls | Self::FrameworkBinding
            ),
            RowClass::TypeHierarchyEdge => {
                matches!(self, Self::Inherits | Self::Implements | Self::Overrides)
            }
            RowClass::RelatedObject => {
                matches!(
                    self,
                    Self::Owner | Self::DocumentedBy | Self::FrameworkBinding
                )
            }
            _ => true,
        }
    }
}

/// Closed rename-blocked reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RenameBlockedReason {
    /// Candidate is hidden by policy or protected boundary.
    PolicyHidden,
    /// Candidate lives in generated source and cannot be authored directly.
    Generated,
    /// Candidate lives in a read-only artifact.
    Readonly,
    /// Candidate's pack or index slice is only partially loaded.
    PartiallyLoaded,
    /// Candidate is omitted because the current scope is sparse.
    SparseScopeOmission,
    /// Candidate conflicts with an alias, shadow, or another rename target.
    ConflictAmbiguous,
}

impl RenameBlockedReason {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyHidden => "policy_hidden",
            Self::Generated => "generated",
            Self::Readonly => "readonly",
            Self::PartiallyLoaded => "partially_loaded",
            Self::SparseScopeOmission => "sparse_scope_omission",
            Self::ConflictAmbiguous => "conflict_ambiguous",
        }
    }
}

/// Per-row reference context (required for `reference` rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReferenceContext {
    /// Closed access kind for the occurrence.
    pub access_kind: AccessKind,
    /// Repo-relative occurrence anchor ref.
    pub occurrence_anchor_ref: String,
}

/// Per-row hierarchy-edge context (required for hierarchy / related-object rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HierarchyEdgeContext {
    /// Source object ref for the edge.
    pub source_object_ref: String,
    /// Target object ref for the edge.
    pub target_object_ref: String,
    /// Closed hierarchy edge kind.
    pub edge_kind: HierarchyEdgeKind,
    /// Edge depth as reported by the owning provider.
    pub depth: u32,
}

/// Per-row rename-preview context (required for `rename_preview` rows).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenamePreviewContext {
    /// Stable rename-preview set id.
    pub rename_preview_id: String,
    /// Root target ref being renamed.
    pub root_target_ref: String,
    /// Candidate occurrences the preview can change.
    pub changed_candidate_count: u32,
    /// Candidate occurrences blocked from change.
    pub blocked_candidate_count: u32,
    /// Generated-source candidates included in the preview.
    pub generated_candidate_count: u32,
    /// Read-only candidates included in the preview.
    pub readonly_candidate_count: u32,
    /// Candidates whose pack or slice is only partially loaded.
    pub partial_loaded_candidate_count: u32,
    /// Candidates omitted because the current scope is sparse.
    pub sparse_scope_omission_count: u32,
    /// Conflict notes (shadowing, alias collision, downstream rename).
    pub conflict_note_count: u32,
    /// True when code bodies were redacted from the preview projection.
    pub code_bodies_redacted: bool,
    /// True when policy hides part of the preview from the active viewer.
    pub policy_hides_candidates: bool,
    /// Closed blocked-reason vocabulary present in the preview.
    #[serde(default)]
    pub blocked_reasons: Vec<RenameBlockedReason>,
}

/// Aliasing context (required when downgrade is `aliased_due_to_shallow_provider`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AliasingContext {
    /// Relation the row was forced to fall back to.
    pub aliased_to_relation: RelationKind,
    /// Closed token naming why the alias was needed (e.g. `provider_shallow`).
    pub aliased_reason_token: String,
    /// Repo-relative evidence ref the alias was recorded against.
    pub alias_evidence_ref: String,
}

/// One navigation-target truth row covered by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetRow {
    /// Stable row id within the packet.
    pub row_id: String,
    /// Closed row class for this truth row.
    pub row_class: RowClass,
    /// Closed relation kind reported for the row.
    pub relation_kind: RelationKind,
    /// Launch-language lane covered by the row (e.g. `rust`, `typescript`, `python`).
    pub language_lane: String,
    /// Stable workspace identity.
    pub workspace_id: String,
    /// Workset/scope ref the row was captured under.
    pub scope_ref: String,
    /// Object ref the row navigates to or from.
    pub object_ref: String,
    /// Provider class that admitted the row.
    pub provider_class: ProviderClass,
    /// Freshness class for the row.
    pub freshness_class: FreshnessClass,
    /// Ambiguity class for the row.
    pub ambiguity_class: AmbiguityClass,
    /// Scope completeness for the row.
    pub scope_completeness: ScopeCompleteness,
    /// Downgrade state for the row.
    pub downgrade_state: DowngradeState,
    /// Confidence class for the row.
    pub confidence_class: ConfidenceClass,
    /// Repo-relative ref to the disclosure shown on the row.
    pub disclosure_ref: String,
    /// Per-row reference context (required for `reference` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_context: Option<ReferenceContext>,
    /// Per-row hierarchy-edge context (required for hierarchy and related-object rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hierarchy_edge_context: Option<HierarchyEdgeContext>,
    /// Per-row rename-preview context (required for `rename_preview` rows).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rename_preview_context: Option<RenamePreviewContext>,
    /// Aliasing context (required when `downgrade_state == aliased_due_to_shallow_provider`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub aliasing_context: Option<AliasingContext>,
    /// True when raw private material is excluded from this row.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Consumer surface that must inherit this packet's truth verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Editor navigation pane (peek, references view, hierarchy tree, rename preview).
    EditorNavigationPane,
    /// Graph topology canvas or list/table fallback.
    GraphTopology,
    /// AI context evidence pane (composer, tool-call evidence).
    AiContext,
    /// Review workspace and review-pack consumers.
    ReviewWorkspace,
    /// Support export bundle.
    SupportExport,
    /// CLI or headless inspection surface.
    CliHeadless,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl ConsumerSurface {
    /// Every required consumer surface, in declaration order.
    pub const REQUIRED: [Self; 7] = [
        Self::EditorNavigationPane,
        Self::GraphTopology,
        Self::AiContext,
        Self::ReviewWorkspace,
        Self::SupportExport,
        Self::CliHeadless,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorNavigationPane => "editor_navigation_pane",
            Self::GraphTopology => "graph_topology",
            Self::AiContext => "ai_context",
            Self::ReviewWorkspace => "review_workspace",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Consumer projection proving a surface reads this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: ConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub navigation_target_packet_id_ref: String,
    /// Rendered-at timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves row-class identity per row.
    pub preserves_row_class_vocabulary: bool,
    /// True when the surface preserves relation-kind labels.
    pub preserves_relation_kind_vocabulary: bool,
    /// True when the surface preserves access-kind labels on reference rows.
    pub preserves_access_kind_vocabulary: bool,
    /// True when the surface preserves provider-class labels.
    pub preserves_provider_class_vocabulary: bool,
    /// True when the surface preserves freshness-class labels.
    pub preserves_freshness_vocabulary: bool,
    /// True when the surface preserves downgrade-state labels.
    pub preserves_downgrade_vocabulary: bool,
    /// True when the surface preserves ambiguity-class labels.
    pub preserves_ambiguity_vocabulary: bool,
    /// True when the surface preserves rename-preview blocked candidate counts and reasons.
    pub preserves_rename_blocked_candidate_vocabulary: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority/credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl NavigationTargetConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.navigation_target_packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_row_class_vocabulary
            && self.preserves_relation_kind_vocabulary
            && self.preserves_access_kind_vocabulary
            && self.preserves_provider_class_vocabulary
            && self.preserves_freshness_vocabulary
            && self.preserves_downgrade_vocabulary
            && self.preserves_ambiguity_vocabulary
            && self.preserves_rename_blocked_candidate_vocabulary
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Closed promotion state for [`NavigationTargetTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PromotionState {
    /// Packet certifies a stable claim for every declared row class.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl PromotionState {
    /// Stable token used in fixtures, schemas, and support exports.
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
pub enum FindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the row below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`NavigationTargetTruthPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Required identity field is empty.
    MissingIdentity,
    /// Required row-class row is missing from the packet.
    MissingRowClassCoverage,
    /// A row drops its disclosure ref.
    MissingDisclosureRef,
    /// A reference row drops its reference context.
    ReferenceMissingAccessContext,
    /// A reference row's access kind is empty or unknown.
    ReferenceMissingAccessKind,
    /// A rename-preview row drops its rename_preview_context.
    RenamePreviewMissingContext,
    /// A hierarchy-edge or related-object row drops its hierarchy_edge_context endpoints.
    HierarchyEdgeMissingEndpoints,
    /// A row claims `canonical` but its relation kind silently aliases the row class.
    SilentRelationAliasPresent,
    /// A row declares `aliased_due_to_shallow_provider` but drops its aliasing context.
    AliasingContextMissingForDowngrade,
    /// An aliasing context aliases to its row class's own canonical relation kind.
    AliasingContextRelationCollision,
    /// A row claims a freshness/provider posture inconsistent with itself.
    ProviderClassFreshnessMismatch,
    /// A non-reference row carries a reference_context.
    AccessContextInvalidForRowClass,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops part of the closed vocabulary.
    ConsumerProjectionDrift,
    /// A consumer projection collapses the row-class vocabulary.
    RowClassVocabularyCollapsed,
    /// A consumer projection drops the relation-kind vocabulary.
    RelationKindVocabularyDropped,
    /// A consumer projection drops the access-kind vocabulary.
    AccessKindVocabularyDropped,
    /// A consumer projection drops the provider-class vocabulary.
    ProviderClassVocabularyDropped,
    /// A consumer projection drops the freshness vocabulary.
    FreshnessVocabularyDropped,
    /// A consumer projection drops the downgrade vocabulary.
    DowngradeVocabularyDropped,
    /// A consumer projection drops the ambiguity vocabulary.
    AmbiguityVocabularyDropped,
    /// A consumer projection drops rename-preview blocked-candidate counts or reasons.
    RenameBlockedCandidateVocabularyDropped,
    /// Row admits raw query text, source bodies, secrets, or ambient credentials.
    RawBoundaryMaterialPresent,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
    /// Packet does not cover every required access kind on its reference rows.
    MissingAccessKindCoverage,
}

impl FindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingRowClassCoverage => "missing_row_class_coverage",
            Self::MissingDisclosureRef => "missing_disclosure_ref",
            Self::ReferenceMissingAccessContext => "reference_missing_access_context",
            Self::ReferenceMissingAccessKind => "reference_missing_access_kind",
            Self::RenamePreviewMissingContext => "rename_preview_missing_context",
            Self::HierarchyEdgeMissingEndpoints => "hierarchy_edge_missing_endpoints",
            Self::SilentRelationAliasPresent => "silent_relation_alias_present",
            Self::AliasingContextMissingForDowngrade => "aliasing_context_missing_for_downgrade",
            Self::AliasingContextRelationCollision => "aliasing_context_relation_collision",
            Self::ProviderClassFreshnessMismatch => "provider_class_freshness_mismatch",
            Self::AccessContextInvalidForRowClass => "access_context_invalid_for_row_class",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::RowClassVocabularyCollapsed => "row_class_vocabulary_collapsed",
            Self::RelationKindVocabularyDropped => "relation_kind_vocabulary_dropped",
            Self::AccessKindVocabularyDropped => "access_kind_vocabulary_dropped",
            Self::ProviderClassVocabularyDropped => "provider_class_vocabulary_dropped",
            Self::FreshnessVocabularyDropped => "freshness_vocabulary_dropped",
            Self::DowngradeVocabularyDropped => "downgrade_vocabulary_dropped",
            Self::AmbiguityVocabularyDropped => "ambiguity_vocabulary_dropped",
            Self::RenameBlockedCandidateVocabularyDropped => {
                "rename_blocked_candidate_vocabulary_dropped"
            }
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
            Self::MissingAccessKindCoverage => "missing_access_kind_coverage",
        }
    }
}

/// One validation finding emitted by the validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FindingKind,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl ValidationFinding {
    fn new(
        finding_kind: FindingKind,
        severity: FindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Constructor input for [`NavigationTargetTruthPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetTruthPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Row classes the packet covers.
    #[serde(default)]
    pub covered_row_classes: Vec<RowClass>,
    /// Access kinds the packet covers (across reference rows).
    #[serde(default)]
    pub covered_access_kinds: Vec<AccessKind>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<NavigationTargetRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<NavigationTargetConsumerProjection>,
    /// Source contracts (docs/schema/fixtures) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Search-and-graph-owned packet for stable navigation-target truth across
/// find-references, rename-preview, and impact-surfacing rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetTruthPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Packet capture timestamp.
    pub generated_at: String,
    /// Row classes the packet covers.
    #[serde(default)]
    pub covered_row_classes: Vec<RowClass>,
    /// Access kinds the packet covers across reference rows.
    #[serde(default)]
    pub covered_access_kinds: Vec<AccessKind>,
    /// Truth rows.
    #[serde(default)]
    pub rows: Vec<NavigationTargetRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<NavigationTargetConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: PromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<ValidationFinding>,
}

impl NavigationTargetTruthPacket {
    /// Materializes a packet and records derived validation findings.
    pub fn materialize(input: NavigationTargetTruthPacketInput) -> Self {
        let mut packet = Self {
            record_kind: NAVIGATION_TARGET_TRUTH_PACKET_RECORD_KIND.to_owned(),
            schema_version: NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            covered_row_classes: input.covered_row_classes,
            covered_access_kinds: input.covered_access_kinds,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: PromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validates the packet against stable navigation-target invariants.
    pub fn validate(&self) -> Vec<ValidationFinding> {
        self.derived_findings(true)
    }

    /// Returns true when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet.
    pub fn has_projection_for(&self, surface: ConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique row-class tokens observed across rows.
    pub fn row_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.row_class);
        }
        set.into_iter().map(RowClass::as_str).collect()
    }

    /// Returns the unique relation-kind tokens observed across rows.
    pub fn relation_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.relation_kind);
        }
        set.into_iter().map(RelationKind::as_str).collect()
    }

    /// Returns the unique access-kind tokens observed across reference rows.
    pub fn access_kind_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            if let Some(context) = &row.reference_context {
                set.insert(context.access_kind);
            }
        }
        set.into_iter().map(AccessKind::as_str).collect()
    }

    /// Returns the unique downgrade-state tokens observed across rows.
    pub fn downgrade_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.downgrade_state);
        }
        set.into_iter().map(DowngradeState::as_str).collect()
    }

    /// Returns the unique provider-class tokens observed across rows.
    pub fn provider_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.provider_class);
        }
        set.into_iter().map(ProviderClass::as_str).collect()
    }

    /// Builds a support export wrapping the exact packet shown to product surfaces.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> NavigationTargetTruthSupportExport {
        NavigationTargetTruthSupportExport {
            record_kind: NAVIGATION_TARGET_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION,
            export_id: export_id.into(),
            navigation_target_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            navigation_target_packet: self.clone(),
        }
    }

    fn derived_findings(&self, include_record_fields: bool) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields && self.record_kind != NAVIGATION_TARGET_TRUTH_PACKET_RECORD_KIND {
            findings.push(ValidationFinding::new(
                FindingKind::WrongRecordKind,
                FindingSeverity::Blocker,
                "navigation-target truth packet has the wrong record kind",
            ));
        }
        if include_record_fields && self.schema_version != NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION {
            findings.push(ValidationFinding::new(
                FindingKind::WrongSchemaVersion,
                FindingSeverity::Blocker,
                "navigation-target truth packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(ValidationFinding::new(
                FindingKind::MissingIdentity,
                FindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        for required in RowClass::REQUIRED {
            let in_coverage = self.covered_row_classes.contains(&required);
            let in_rows = self.rows.iter().any(|row| row.row_class == required);
            if !in_coverage || !in_rows {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingRowClassCoverage,
                    FindingSeverity::Blocker,
                    format!("no row covers required row class {}", required.as_str()),
                ));
            }
        }

        for required in AccessKind::REQUIRED {
            if !self.covered_access_kinds.contains(&required) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAccessKindCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "covered_access_kinds drops required access kind {}",
                        required.as_str()
                    ),
                ));
            }
        }
        let mut observed_access_kinds = BTreeSet::new();
        for row in &self.rows {
            if let Some(context) = &row.reference_context {
                observed_access_kinds.insert(context.access_kind);
            }
        }
        for required in AccessKind::REQUIRED {
            if !observed_access_kinds.contains(&required) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingAccessKindCoverage,
                    FindingSeverity::Blocker,
                    format!(
                        "no reference row preserves access kind {}",
                        required.as_str()
                    ),
                ));
            }
        }

        for row in &self.rows {
            if row.row_id.trim().is_empty()
                || row.workspace_id.trim().is_empty()
                || row.scope_ref.trim().is_empty()
                || row.object_ref.trim().is_empty()
                || row.language_lane.trim().is_empty()
                || row.captured_at.trim().is_empty()
            {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingIdentity,
                    FindingSeverity::Blocker,
                    format!("row {} drops a required identity field", row.row_id),
                ));
            }
            if row.disclosure_ref.trim().is_empty() {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingDisclosureRef,
                    FindingSeverity::Blocker,
                    format!("row {} drops its disclosure ref", row.row_id),
                ));
            }
            self.validate_row_class_context(row, &mut findings);
            self.validate_row_alias_invariants(row, &mut findings);
            self.validate_row_provider_freshness(row, &mut findings);
            if !row.raw_boundary_material_excluded {
                findings.push(ValidationFinding::new(
                    FindingKind::RawBoundaryMaterialPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, source bodies, secrets, or credentials",
                        row.row_id
                    ),
                ));
            }
        }

        for required_surface in ConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(ValidationFinding::new(
                    FindingKind::MissingConsumerProjection,
                    FindingSeverity::Blocker,
                    format!(
                        "packet {} is missing a preserved {} projection",
                        self.packet_id,
                        required_surface.as_str()
                    ),
                ));
            }
        }
        for projection in &self.consumer_projections {
            if !projection.preserves_truth_for(&self.packet_id) {
                findings.push(ValidationFinding::new(
                    FindingKind::ConsumerProjectionDrift,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve navigation-target truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_row_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RowClassVocabularyCollapsed,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} collapses the row-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_relation_kind_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RelationKindVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the relation-kind vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_access_kind_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AccessKindVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the access-kind vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_provider_class_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::ProviderClassVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the provider-class vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_freshness_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::FreshnessVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the freshness vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_downgrade_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::DowngradeVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the downgrade-state vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_ambiguity_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::AmbiguityVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the ambiguity vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_rename_blocked_candidate_vocabulary {
                findings.push(ValidationFinding::new(
                    FindingKind::RenameBlockedCandidateVocabularyDropped,
                    FindingSeverity::Blocker,
                    format!(
                        "projection {} drops the rename blocked-candidate vocabulary",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion
                .retain(|finding| finding.finding_kind != FindingKind::PromotionStateMismatch);
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(ValidationFinding::new(
                    FindingKind::PromotionStateMismatch,
                    FindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }

    fn validate_row_class_context(
        &self,
        row: &NavigationTargetRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        match row.row_class {
            RowClass::Reference => match row.reference_context.as_ref() {
                None => findings.push(ValidationFinding::new(
                    FindingKind::ReferenceMissingAccessContext,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is reference but missing reference_context",
                        row.row_id
                    ),
                )),
                Some(context) => {
                    if context.occurrence_anchor_ref.trim().is_empty() {
                        findings.push(ValidationFinding::new(
                            FindingKind::ReferenceMissingAccessKind,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} reference_context drops occurrence anchor ref",
                                row.row_id
                            ),
                        ));
                    }
                }
            },
            RowClass::CallHierarchyEdge | RowClass::TypeHierarchyEdge | RowClass::RelatedObject => {
                let context_ok = row.hierarchy_edge_context.as_ref().is_some_and(|context| {
                    !context.source_object_ref.trim().is_empty()
                        && !context.target_object_ref.trim().is_empty()
                        && context.edge_kind.matches_row_class(row.row_class)
                });
                if !context_ok {
                    findings.push(ValidationFinding::new(
                        FindingKind::HierarchyEdgeMissingEndpoints,
                        FindingSeverity::Blocker,
                        format!(
                            "row {} ({}) drops hierarchy_edge_context endpoints or kind",
                            row.row_id,
                            row.row_class.as_str()
                        ),
                    ));
                }
            }
            RowClass::RenamePreview => match row.rename_preview_context.as_ref() {
                None => findings.push(ValidationFinding::new(
                    FindingKind::RenamePreviewMissingContext,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is rename_preview but missing rename_preview_context",
                        row.row_id
                    ),
                )),
                Some(context) => {
                    if context.rename_preview_id.trim().is_empty()
                        || context.root_target_ref.trim().is_empty()
                    {
                        findings.push(ValidationFinding::new(
                            FindingKind::RenamePreviewMissingContext,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} rename_preview_context drops identity fields",
                                row.row_id
                            ),
                        ));
                    }
                }
            },
            _ => {}
        }
        if row.reference_context.is_some() && row.row_class != RowClass::Reference {
            findings.push(ValidationFinding::new(
                FindingKind::AccessContextInvalidForRowClass,
                FindingSeverity::Blocker,
                format!(
                    "row {} carries reference_context but is not a reference row",
                    row.row_id
                ),
            ));
        }
    }

    fn validate_row_alias_invariants(
        &self,
        row: &NavigationTargetRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let canonical = RelationKind::canonical_for_row_class(row.row_class);
        if let Some(canonical_relation) = canonical {
            if row.downgrade_state == DowngradeState::Canonical
                && row.relation_kind != canonical_relation
            {
                findings.push(ValidationFinding::new(
                    FindingKind::SilentRelationAliasPresent,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} silently aliases {} to {}",
                        row.row_id,
                        row.row_class.as_str(),
                        row.relation_kind.as_str()
                    ),
                ));
            }
        }
        if row.downgrade_state == DowngradeState::AliasedDueToShallowProvider {
            match row.aliasing_context.as_ref() {
                None => findings.push(ValidationFinding::new(
                    FindingKind::AliasingContextMissingForDowngrade,
                    FindingSeverity::Blocker,
                    format!(
                        "row {} is aliased_due_to_shallow_provider but missing aliasing_context",
                        row.row_id
                    ),
                )),
                Some(context) => {
                    if context.aliased_reason_token.trim().is_empty()
                        || context.alias_evidence_ref.trim().is_empty()
                    {
                        findings.push(ValidationFinding::new(
                            FindingKind::AliasingContextMissingForDowngrade,
                            FindingSeverity::Blocker,
                            format!(
                                "row {} aliasing_context drops reason or evidence ref",
                                row.row_id
                            ),
                        ));
                    }
                    if let Some(canonical_relation) = canonical {
                        if context.aliased_to_relation == canonical_relation {
                            findings.push(ValidationFinding::new(
                                FindingKind::AliasingContextRelationCollision,
                                FindingSeverity::Blocker,
                                format!(
                                    "row {} aliasing_context aliases back to canonical relation {}",
                                    row.row_id,
                                    canonical_relation.as_str()
                                ),
                            ));
                        }
                    }
                }
            }
        } else if row.aliasing_context.is_some() {
            findings.push(ValidationFinding::new(
                FindingKind::AliasingContextMissingForDowngrade,
                FindingSeverity::Blocker,
                format!(
                    "row {} carries aliasing_context but downgrade_state is not aliased_due_to_shallow_provider",
                    row.row_id
                ),
            ));
        }
    }

    fn validate_row_provider_freshness(
        &self,
        row: &NavigationTargetRow,
        findings: &mut Vec<ValidationFinding>,
    ) {
        let provider_implies_imported = row.provider_class == ProviderClass::ImportedSnapshot;
        let freshness_implies_imported = row.freshness_class == FreshnessClass::ImportedSnapshot;
        if provider_implies_imported != freshness_implies_imported {
            findings.push(ValidationFinding::new(
                FindingKind::ProviderClassFreshnessMismatch,
                FindingSeverity::Blocker,
                format!(
                    "row {} provider/freshness disagree on imported_snapshot posture",
                    row.row_id
                ),
            ));
        }
        if row.provider_class.is_shallow() && row.downgrade_state == DowngradeState::Canonical {
            findings.push(ValidationFinding::new(
                FindingKind::ProviderClassFreshnessMismatch,
                FindingSeverity::Blocker,
                format!(
                    "row {} shallow provider {} cannot back canonical downgrade",
                    row.row_id,
                    row.provider_class.as_str()
                ),
            ));
        }
    }
}

fn promotion_state_for_findings(findings: &[ValidationFinding]) -> PromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Blocker)
    {
        PromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        PromotionState::NarrowedBelowStable
    } else {
        PromotionState::Stable
    }
}

/// Support-export wrapper that preserves the product packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NavigationTargetTruthSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Packet id preserved by the export.
    pub navigation_target_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials/authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub navigation_target_packet: NavigationTargetTruthPacket,
}

impl NavigationTargetTruthSupportExport {
    /// Returns true when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == NAVIGATION_TARGET_TRUTH_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == NAVIGATION_TARGET_TRUTH_SCHEMA_VERSION
            && self.navigation_target_packet_id_ref == self.navigation_target_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.navigation_target_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable packet.
#[derive(Debug)]
pub enum NavigationTargetTruthArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<ValidationFinding>),
}

impl fmt::Display for NavigationTargetTruthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "navigation-target truth packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "navigation-target truth packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for NavigationTargetTruthArtifactError {}

/// Returns the checked-in stable navigation-target truth packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_navigation_target_truth_packet(
) -> Result<NavigationTargetTruthPacket, NavigationTargetTruthArtifactError> {
    let packet: NavigationTargetTruthPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/navigation_target_truth_packet.json"
    )))
    .map_err(NavigationTargetTruthArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(NavigationTargetTruthArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn projection_for(surface: ConsumerSurface) -> NavigationTargetConsumerProjection {
        NavigationTargetConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            navigation_target_packet_id_ref: "packet:m4:navigation_target:test".to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_row_class_vocabulary: true,
            preserves_relation_kind_vocabulary: true,
            preserves_access_kind_vocabulary: true,
            preserves_provider_class_vocabulary: true,
            preserves_freshness_vocabulary: true,
            preserves_downgrade_vocabulary: true,
            preserves_ambiguity_vocabulary: true,
            preserves_rename_blocked_candidate_vocabulary: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn base_row(
        row_id: &str,
        row_class: RowClass,
        relation_kind: RelationKind,
    ) -> NavigationTargetRow {
        NavigationTargetRow {
            row_id: row_id.to_owned(),
            row_class,
            relation_kind,
            language_lane: "rust".to_owned(),
            workspace_id: "workspace:test".to_owned(),
            scope_ref: "scope:test:default".to_owned(),
            object_ref: format!("object:{row_id}"),
            provider_class: ProviderClass::LanguageServer,
            freshness_class: FreshnessClass::AuthoritativeLive,
            ambiguity_class: AmbiguityClass::Unambiguous,
            scope_completeness: ScopeCompleteness::CompleteForDeclaredScope,
            downgrade_state: DowngradeState::Canonical,
            confidence_class: ConfidenceClass::Exact,
            disclosure_ref: NAVIGATION_TARGET_TRUTH_DOC_REF.to_owned(),
            reference_context: None,
            hierarchy_edge_context: None,
            rename_preview_context: None,
            aliasing_context: None,
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn reference_row(row_id: &str, access_kind: AccessKind) -> NavigationTargetRow {
        let mut row = base_row(row_id, RowClass::Reference, RelationKind::Reference);
        row.reference_context = Some(ReferenceContext {
            access_kind,
            occurrence_anchor_ref: format!("anchor:{row_id}"),
        });
        row
    }

    fn hierarchy_row(
        row_id: &str,
        row_class: RowClass,
        edge_kind: HierarchyEdgeKind,
        relation_kind: RelationKind,
    ) -> NavigationTargetRow {
        let mut row = base_row(row_id, row_class, relation_kind);
        row.hierarchy_edge_context = Some(HierarchyEdgeContext {
            source_object_ref: format!("source:{row_id}"),
            target_object_ref: format!("target:{row_id}"),
            edge_kind,
            depth: 1,
        });
        row
    }

    fn rename_row(row_id: &str) -> NavigationTargetRow {
        let mut row = base_row(row_id, RowClass::RenamePreview, RelationKind::Reference);
        row.rename_preview_context = Some(RenamePreviewContext {
            rename_preview_id: format!("rename:{row_id}"),
            root_target_ref: format!("root:{row_id}"),
            changed_candidate_count: 8,
            blocked_candidate_count: 2,
            generated_candidate_count: 1,
            readonly_candidate_count: 1,
            partial_loaded_candidate_count: 0,
            sparse_scope_omission_count: 0,
            conflict_note_count: 0,
            code_bodies_redacted: true,
            policy_hides_candidates: false,
            blocked_reasons: vec![
                RenameBlockedReason::Generated,
                RenameBlockedReason::Readonly,
            ],
        });
        row
    }

    fn baseline_input() -> NavigationTargetTruthPacketInput {
        let mut rows = vec![
            base_row("row:def", RowClass::Definition, RelationKind::Definition),
            base_row("row:decl", RowClass::Declaration, RelationKind::Declaration),
            base_row(
                "row:impl",
                RowClass::Implementation,
                RelationKind::Implementation,
            ),
            hierarchy_row(
                "row:call",
                RowClass::CallHierarchyEdge,
                HierarchyEdgeKind::Calls,
                RelationKind::Call,
            ),
            hierarchy_row(
                "row:type",
                RowClass::TypeHierarchyEdge,
                HierarchyEdgeKind::Inherits,
                RelationKind::Type,
            ),
            hierarchy_row(
                "row:related",
                RowClass::RelatedObject,
                HierarchyEdgeKind::DocumentedBy,
                RelationKind::DocLink,
            ),
            rename_row("row:rename"),
        ];
        for (idx, access) in AccessKind::REQUIRED.iter().copied().enumerate() {
            rows.push(reference_row(&format!("row:ref:{idx}"), access));
        }
        NavigationTargetTruthPacketInput {
            packet_id: "packet:m4:navigation_target:test".to_owned(),
            workflow_or_surface_id: "workflow.search_graph.navigation_target.test".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            covered_row_classes: RowClass::REQUIRED.to_vec(),
            covered_access_kinds: AccessKind::REQUIRED.to_vec(),
            rows,
            consumer_projections: ConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(projection_for)
                .collect(),
            source_contract_refs: vec![NAVIGATION_TARGET_TRUTH_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(RowClass::RenamePreview.as_str(), "rename_preview");
        assert_eq!(AccessKind::RuntimeObserved.as_str(), "runtime_observed");
        assert_eq!(ProviderClass::LanguageServer.as_str(), "language_server");
        assert_eq!(
            DowngradeState::AliasedDueToShallowProvider.as_str(),
            "aliased_due_to_shallow_provider"
        );
        assert_eq!(
            FindingKind::SilentRelationAliasPresent.as_str(),
            "silent_relation_alias_present"
        );
        assert_eq!(PromotionState::BlocksStable.as_str(), "blocks_stable");
    }

    #[test]
    fn baseline_input_materializes_stable() {
        let packet = NavigationTargetTruthPacket::materialize(baseline_input());
        assert_eq!(
            packet.promotion_state,
            PromotionState::Stable,
            "findings: {:?}",
            packet.validation_findings
        );
        assert!(packet.validation_findings.is_empty());
        assert!(packet.is_stable());
    }

    #[test]
    fn missing_disclosure_blocks_stable() {
        let mut input = baseline_input();
        input.rows[0].disclosure_ref = String::new();
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingDisclosureRef));
    }

    #[test]
    fn reference_without_access_context_blocks_stable() {
        let mut input = baseline_input();
        let ref_idx = input
            .rows
            .iter()
            .position(|row| row.row_class == RowClass::Reference)
            .expect("reference row present");
        input.rows[ref_idx].reference_context = None;
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::ReferenceMissingAccessContext));
    }

    #[test]
    fn silent_relation_alias_blocks_stable() {
        let mut input = baseline_input();
        let def_idx = input
            .rows
            .iter()
            .position(|row| row.row_class == RowClass::Definition)
            .expect("definition row present");
        input.rows[def_idx].relation_kind = RelationKind::Declaration;
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::SilentRelationAliasPresent));
    }

    #[test]
    fn aliased_without_aliasing_context_blocks_stable() {
        let mut input = baseline_input();
        let def_idx = input
            .rows
            .iter()
            .position(|row| row.row_class == RowClass::Definition)
            .expect("definition row present");
        input.rows[def_idx].downgrade_state = DowngradeState::AliasedDueToShallowProvider;
        input.rows[def_idx].provider_class = ProviderClass::SyntaxFallback;
        input.rows[def_idx].aliasing_context = None;
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::AliasingContextMissingForDowngrade
        }));
    }

    #[test]
    fn rename_preview_without_context_blocks_stable() {
        let mut input = baseline_input();
        let rename_idx = input
            .rows
            .iter()
            .position(|row| row.row_class == RowClass::RenamePreview)
            .expect("rename row present");
        input.rows[rename_idx].rename_preview_context = None;
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::RenamePreviewMissingContext));
    }

    #[test]
    fn missing_required_row_class_blocks_stable() {
        let mut input = baseline_input();
        input
            .rows
            .retain(|row| row.row_class != RowClass::RenamePreview);
        input
            .covered_row_classes
            .retain(|class| *class != RowClass::RenamePreview);
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingRowClassCoverage));
    }

    #[test]
    fn missing_access_kind_coverage_blocks_stable() {
        let mut input = baseline_input();
        // Drop runtime_observed reference row and its coverage entry.
        input.rows.retain(|row| {
            row.reference_context
                .as_ref()
                .is_none_or(|context| context.access_kind != AccessKind::RuntimeObserved)
        });
        input
            .covered_access_kinds
            .retain(|kind| *kind != AccessKind::RuntimeObserved);
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingAccessKindCoverage));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = vec![projection_for(ConsumerSurface::EditorNavigationPane)];
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::MissingConsumerProjection));
    }

    #[test]
    fn projection_drops_access_kind_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = projection_for(surface);
                if surface == ConsumerSurface::AiContext {
                    projection.preserves_access_kind_vocabulary = false;
                }
                projection
            })
            .collect();
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind == FindingKind::AccessKindVocabularyDropped));
    }

    #[test]
    fn projection_drops_rename_blocked_candidate_vocabulary_blocks_stable() {
        let mut input = baseline_input();
        input.consumer_projections = ConsumerSurface::REQUIRED
            .iter()
            .copied()
            .map(|surface| {
                let mut projection = projection_for(surface);
                if surface == ConsumerSurface::SupportExport {
                    projection.preserves_rename_blocked_candidate_vocabulary = false;
                }
                projection
            })
            .collect();
        let packet = NavigationTargetTruthPacket::materialize(input);
        assert_eq!(packet.promotion_state, PromotionState::BlocksStable);
        assert!(packet.validation_findings.iter().any(|finding| {
            finding.finding_kind == FindingKind::RenameBlockedCandidateVocabularyDropped
        }));
    }

    #[test]
    fn support_export_is_export_safe_when_packet_is_stable() {
        let packet = NavigationTargetTruthPacket::materialize(baseline_input());
        let export = packet.support_export("export:test", "2026-05-26T12:00:10Z");
        assert!(export.is_export_safe());
    }
}
