use serde::{Deserialize, Serialize};

use crate::lsp_router::{
    DegradedStateClass, EpochBinding, FreshnessClass, HealthState, LocalityClass, ProviderFamily,
    ProviderPolicyContext, RedactionClass, RouterTrustState, ScopeClaimClass, ScopeLimitClass,
};

/// Integer schema version for Python launch-wedge assistance records.
pub type PythonSemanticResultSchemaVersion = u32;

/// Integer schema version for Python rename-preview records.
pub type PythonRenamePreviewSchemaVersion = u32;

/// Schema version used by the Python launch-wedge alpha record family.
pub const PYTHON_NAV_ALPHA_SCHEMA_VERSION: PythonSemanticResultSchemaVersion = 1;

/// Python symbol kind used by protected fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonSymbolKindClass {
    /// Function binding.
    Function,
    /// Class binding.
    Class,
    /// Method binding.
    Method,
    /// Constant or value binding.
    Constant,
    /// Variable or attribute binding.
    Variable,
    /// Module-level export surface.
    Module,
}

impl PythonSymbolKindClass {
    /// Returns the stable schema token for this symbol kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Class => "class",
            Self::Method => "method",
            Self::Constant => "constant",
            Self::Variable => "variable",
            Self::Module => "module",
        }
    }
}

/// Reference access kind shared by references and rename previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonAccessKindClass {
    /// Symbol value is read.
    Read,
    /// Symbol location is written or assigned.
    Write,
    /// Callable symbol is invoked.
    Call,
    /// Symbol crosses an import boundary.
    Import,
    /// Symbol crosses an export boundary.
    Export,
    /// Occurrence belongs to generated or framework-derived source.
    Generated,
}

impl PythonAccessKindClass {
    /// Returns the semantic relation class represented by this access kind.
    pub const fn relation_class(self) -> PythonRelationClass {
        match self {
            Self::Read => PythonRelationClass::ReadReference,
            Self::Write => PythonRelationClass::WriteReference,
            Self::Call => PythonRelationClass::CallReference,
            Self::Import | Self::Export => PythonRelationClass::ImportOrExportReference,
            Self::Generated => PythonRelationClass::GeneratedOrFrameworkReference,
        }
    }
}

/// Authored, generated, or external posture for one occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonGeneratedOrExternalStateClass {
    /// Occurrence is in authored workspace source.
    AuthoredSource,
    /// Occurrence is in generated or paired source.
    GeneratedSource,
    /// Occurrence is in an external dependency or imported declaration.
    ExternalDependency,
    /// Occurrence is read-only for this workspace or policy posture.
    ReadOnlySource,
}

impl PythonGeneratedOrExternalStateClass {
    /// Returns true when a rename should not mutate this occurrence.
    pub const fn blocks_rename_candidate(self) -> bool {
        !matches!(self, Self::AuthoredSource)
    }
}

/// Answering layer for a Python language-assistance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonAnswerLayerClass {
    /// Syntax and structural parsing answered the request.
    #[serde(rename = "layer_1_syntax_structure")]
    Layer1SyntaxStructure,
    /// LSP compatibility provider answered the request.
    #[serde(rename = "layer_2_compatibility_breadth")]
    Layer2CompatibilityBreadth,
    /// Aureline-owned graph or framework semantic depth answered the request.
    #[serde(rename = "layer_3_aureline_semantic_depth")]
    Layer3AurelineSemanticDepth,
}

impl PythonAnswerLayerClass {
    /// Returns true when the answer is lower than compatibility semantics.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::Layer1SyntaxStructure)
    }
}

/// Semantic-result identity class emitted by Python navigation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonSemanticResultIdentityClass {
    /// Definition target for the selected symbol.
    Definition,
    /// Reference occurrence for the selected symbol.
    Reference,
    /// Generated or imported reference occurrence.
    ImportedOrGeneratedReference,
}

/// Semantic relation class emitted by Python navigation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonRelationClass {
    /// Definition target relation.
    DefinitionTarget,
    /// Read reference relation.
    ReadReference,
    /// Write reference relation.
    WriteReference,
    /// Call reference relation.
    CallReference,
    /// Import or export relation.
    ImportOrExportReference,
    /// Generated or framework-derived relation.
    GeneratedOrFrameworkReference,
    /// No relation applies.
    NotApplicable,
}

/// Confidence class for one semantic navigation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonResultConfidenceClass {
    /// Result is exact for the declared scope.
    Exact,
    /// Result is index-backed for the declared scope.
    Indexed,
    /// Result is partial for the declared scope.
    Partial,
    /// No admissible result exists.
    Unavailable,
    /// Result is a labeled heuristic or syntax fallback.
    HeuristicallyMapped,
    /// Result is limited to the current workset or sparse slice.
    WorkspaceSliceLimited,
}

impl PythonResultConfidenceClass {
    /// Returns true when the state requires a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact | Self::Indexed)
    }
}

/// Completeness class for a semantic navigation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonCompletenessClass {
    /// Complete for the declared scope.
    CompleteForDeclaredScope,
    /// Partial for the declared scope.
    PartialForDeclaredScope,
    /// Stale for the declared scope.
    StaleForDeclaredScope,
    /// Unavailable for the declared scope.
    UnavailableForDeclaredScope,
}

impl PythonCompletenessClass {
    /// Returns true when consumers must disclose incomplete truth.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::CompleteForDeclaredScope)
    }
}

/// Inline visibility policy for a semantic result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonInlineVisibilityClass {
    /// Result may render inline as authoritative.
    InlineAuthoritativeAllowed,
    /// Result may render inline only with visible caveats.
    InlineCaveatedAllowed,
    /// Result may be inspected but not used as mutation authority.
    InlineInspectOnly,
    /// Result should stay hidden until scope widens or refresh completes.
    InlineHiddenRequiresScopeOrRefresh,
    /// Result is unavailable.
    InlineUnavailable,
}

/// Source-anchor family for a Python result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonSourceAnchorKindClass {
    /// Anchor points into current workspace source.
    WorkspaceSourceAnchor,
    /// Anchor points through generated lineage.
    GeneratedLineageAnchor,
    /// Anchor came from a provider overlay.
    ProviderOverlayAnchor,
    /// Anchor could not be resolved.
    UnresolvedAnchor,
}

/// Rename-preview completeness class for Python previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonRenamePreviewCompletenessClass {
    /// Preview covers the whole admitted workspace.
    FullWorkspaceComplete,
    /// Preview is complete for the requested scope.
    CompleteForRequestedScope,
    /// Preview is partial because the current workset or sparse slice narrows scope.
    PartialDueToWorkspaceSlice,
    /// Preview is partial because an index or provider is unavailable or incomplete.
    PartialDueToIndexOrProvider,
    /// Preview is partial because generated or imported boundaries are involved.
    PartialDueToImportedOrGeneratedBoundaries,
    /// Preview requires provider or index refresh before apply.
    StaleRequiresRefresh,
    /// Preview is unavailable and inspect-only.
    UnavailableBlocked,
}

impl PythonRenamePreviewCompletenessClass {
    /// Returns true when the preview must not be applied directly.
    pub const fn blocks_direct_apply(self) -> bool {
        !matches!(
            self,
            Self::FullWorkspaceComplete | Self::CompleteForRequestedScope
        )
    }
}

/// Apply posture for a Python rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonApplyPostureClass {
    /// Preview may be applied after user review.
    ReadyForApplyAfterPreview,
    /// Scope must be reviewed or widened first.
    BlockedPendingScopeReview,
    /// Provider or index refresh is required first.
    BlockedPendingRefresh,
    /// Policy, protected source, or read-only source needs review first.
    BlockedPendingPolicyOrProtectedReview,
    /// Preview is inspect-only.
    InspectOnlyUnavailable,
}

/// Coverage limit attached to a rename preview affected scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonRenameCoverageLimitClass {
    /// Active workset or sparse slice narrowed coverage.
    WorkspaceSliceLimited,
    /// Semantic index only covered part of the requested scope.
    SemanticIndexPartial,
    /// Provider was unavailable.
    ProviderUnavailable,
    /// Provider epoch was stale.
    ProviderStale,
    /// Remote shard was unreachable.
    RemoteShardUnreachable,
    /// Imported snapshot was not locally verified.
    ImportedSnapshotUnverified,
    /// Generated lineage could not be resolved.
    GeneratedLineageUnresolved,
    /// Protected source blocked a candidate.
    ProtectedScopeBlocked,
    /// Policy narrowed coverage.
    PolicyNarrowed,
    /// User-selected scope excluded candidates.
    UserExcludedScope,
    /// Interpreter or environment selection was incomplete.
    InterpreterSelectionIncomplete,
}

/// Warning class attached to a rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonRenameWarningClass {
    /// Rename may collide with or shadow another symbol.
    ShadowedSymbol,
    /// Alias target is ambiguous.
    AliasTargetAmbiguous,
    /// Alias chain could not be resolved.
    AliasChainUnresolved,
    /// Generated reference would be involved.
    GeneratedReferenceWouldChange,
    /// Read-only or protected target was encountered.
    ProtectedOrReadOnlyTarget,
    /// Provider epoch is stale.
    StaleProviderEpoch,
    /// Imported anchor was not verified locally.
    ImportedAnchorUnverified,
    /// Workspace slice narrowed the preview.
    WorkspaceSliceLimited,
    /// Remote scope could not be reached.
    RemoteScopeUnreachable,
    /// Interpreter or environment selection is incomplete or mismatched.
    InterpreterSelectionIncomplete,
}

/// Checkpoint posture attached to a rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonCheckpointClass {
    /// A local checkpoint was captured for a ready preview.
    CheckpointCaptured,
    /// A checkpoint is required but not captured because apply is blocked.
    CheckpointRequiredNotCaptured,
    /// No checkpoint is required because the preview is inspect-only.
    CheckpointNotRequiredInspectOnly,
}

/// Rollback path attached to a rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonRollbackPathClass {
    /// Exact undo is available through local history.
    ExactUndoViaLocalHistoryCheckpoint,
    /// Revert would be compensating through a workspace diff.
    CompensatingRevertViaWorkspaceDiff,
    /// Regeneration must happen before replay.
    RegenerateFirstThenReplay,
    /// Manual review is required before any automatic rollback path can be claimed.
    ManualReviewRequiredNoAutomaticPath,
    /// No safe rollback path is available.
    NoSafeRollbackAvailable,
}

/// Python environment manager represented by an interpreter context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonEnvironmentManagerClass {
    /// A local virtual environment or `venv`-style interpreter.
    Venv,
    /// A `uv` managed environment.
    Uv,
    /// A Poetry managed environment.
    Poetry,
    /// A Conda managed environment.
    Conda,
    /// A system interpreter with no project environment manager.
    System,
    /// The environment manager is not known.
    Unknown,
}

/// Python interpreter selection state for language assistance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonInterpreterSelectionStateClass {
    /// A single interpreter was selected for the current subject.
    Selected,
    /// Multiple interpreters could apply and user or policy selection is required.
    Ambiguous,
    /// No interpreter has been selected or resolved.
    Missing,
    /// The selected interpreter no longer matches the current workspace state.
    Drifted,
}

impl PythonInterpreterSelectionStateClass {
    /// Returns true when interpreter truth needs a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Selected)
    }
}

/// Readiness of the selected Python interpreter for language assistance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonInterpreterReadinessClass {
    /// Interpreter and package-index context are ready enough for semantic answers.
    ReadyForAnalysis,
    /// Interpreter exists but package or environment truth is partial.
    PartialEnvironment,
    /// Interpreter truth is unavailable, so only lower-authority fallback may answer.
    Unavailable,
}

impl PythonInterpreterReadinessClass {
    /// Returns true when interpreter readiness needs a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::ReadyForAnalysis)
    }
}

/// Interpreter and environment selection attached to Python assistance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonInterpreterContext {
    /// Stable interpreter reference.
    pub interpreter_ref: String,
    /// Plain-language interpreter label.
    pub interpreter_display_label: String,
    /// Environment or environment-capsule reference.
    pub environment_ref: String,
    /// Environment manager that produced or owns the interpreter.
    pub environment_manager_class: PythonEnvironmentManagerClass,
    /// Interpreter selection state.
    pub selection_state_class: PythonInterpreterSelectionStateClass,
    /// Interpreter readiness for language analysis.
    pub readiness_class: PythonInterpreterReadinessClass,
    /// Python version reference, when known.
    pub python_version_ref: Option<String>,
    /// Package index or environment epoch reference, when known.
    pub package_index_epoch_ref: Option<String>,
    /// Export-safe interpreter summary.
    pub summary: String,
}

impl PythonInterpreterContext {
    /// Returns true when interpreter or environment truth must be disclosed.
    pub const fn requires_disclosure(&self) -> bool {
        self.selection_state_class.requires_disclosure()
            || self.readiness_class.requires_disclosure()
    }

    /// Returns true when the language service may be trusted for semantic answers.
    pub const fn admits_semantic_provider(&self) -> bool {
        !self.requires_disclosure()
    }
}

/// Workspace, workset, and policy context for Python assistance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonWorkspaceContext {
    /// Workspace id.
    pub workspace_id: String,
    /// Active workset id.
    pub workset_id: String,
    /// Top-level workspace root reference.
    pub workspace_root_ref: String,
    /// Root reference containing the requested subject.
    pub subject_root_ref: String,
    /// Package root governing the subject.
    pub package_root_ref: String,
    /// Python config root governing the subject.
    pub config_root_ref: String,
    /// Execution context anchoring Python interpreter identity.
    pub execution_context_id: String,
    /// Selected Python interpreter and package environment context.
    pub interpreter_context: PythonInterpreterContext,
    /// Policy epoch applied to assistance records.
    pub policy_epoch: String,
    /// Trust state applied to assistance records.
    pub trust_state: RouterTrustState,
    /// Scope requested by protected surfaces.
    pub requested_scope_class: ScopeClaimClass,
    /// Scope that is currently materialized.
    pub materialized_scope_class: ScopeClaimClass,
    /// Scope ref covered by the materialized result.
    pub covered_scope_ref: String,
    /// Omitted scope ref, when known.
    pub omitted_scope_ref: Option<String>,
    /// User-visible scope label.
    pub scope_label: String,
    /// Concrete scope limits for the current workset.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Opaque refs for omitted roots or generated scopes.
    #[serde(default)]
    pub omitted_scope_refs: Vec<String>,
}

impl PythonWorkspaceContext {
    /// Builds a policy context used by exportable records.
    pub fn policy_context(&self) -> ProviderPolicyContext {
        ProviderPolicyContext {
            policy_epoch: self.policy_epoch.clone(),
            trust_state: self.trust_state,
            execution_context_id: self.execution_context_id.clone(),
        }
    }
}

/// Opaque source anchor seed for a fixture symbol or occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonAnchorRef {
    /// Stable source anchor ref.
    pub source_anchor_ref: String,
    /// Anchor source family.
    pub source_anchor_kind_class: PythonSourceAnchorKindClass,
    /// Canonical file ref.
    pub canonical_file_ref: String,
    /// Workspace-relative path used only for export-safe display.
    pub workspace_relative_path: String,
    /// Opaque range ref.
    pub range_ref: String,
    /// Export-safe summary for the anchor.
    pub summary: String,
}

/// Protected Python fixture symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonSymbolSeed {
    /// Stable symbol ref.
    pub symbol_ref: String,
    /// Human-readable symbol label.
    pub display_name: String,
    /// Symbol kind.
    pub kind_class: PythonSymbolKindClass,
    /// Export-safe hover label.
    pub hover_label: String,
    /// Export-safe hover summary.
    pub hover_summary: String,
    /// Export-safe hover detail.
    pub hover_detail: String,
    /// Definition anchor for the symbol.
    pub definition_anchor: PythonAnchorRef,
    /// Reference occurrence seeds for the symbol.
    pub occurrences: Vec<PythonOccurrenceSeed>,
}

impl PythonSymbolSeed {
    /// Returns candidate occurrences that are references rather than the definition anchor.
    pub fn reference_occurrences(&self) -> impl Iterator<Item = &PythonOccurrenceSeed> {
        self.occurrences.iter()
    }
}

/// Protected Python fixture occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonOccurrenceSeed {
    /// Stable occurrence ref.
    pub occurrence_ref: String,
    /// Symbol this occurrence targets.
    pub symbol_ref: String,
    /// Occurrence anchor.
    pub anchor: PythonAnchorRef,
    /// Access kind for references and previews.
    pub access_kind_class: PythonAccessKindClass,
    /// Generated, external, or authored posture.
    pub generated_or_external_state_class: PythonGeneratedOrExternalStateClass,
    /// Scope containing this occurrence.
    pub scope_ref: String,
    /// Whether the occurrence is inside the current workset.
    pub in_current_workset: bool,
    /// Whether this occurrence is a candidate for rename.
    pub rename_candidate: bool,
    /// Whether this occurrence is read-only for rename.
    pub readonly: bool,
    /// Export-safe occurrence summary.
    pub summary: String,
}

impl PythonOccurrenceSeed {
    /// Returns true when this occurrence may be changed by a semantic rename preview.
    pub const fn rename_writable_authored_candidate(&self) -> bool {
        self.rename_candidate
            && !self.readonly
            && !self
                .generated_or_external_state_class
                .blocks_rename_candidate()
    }
}

/// Fixture-backed Python launch-wedge snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonLaunchWedgeSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonSemanticResultSchemaVersion,
    /// Requested language id.
    pub language_id: String,
    /// Workspace, scope, and policy context.
    pub workspace_context: PythonWorkspaceContext,
    /// Epoch bindings current for the fixture.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Protected Python symbols in the fixture.
    pub symbols: Vec<PythonSymbolSeed>,
    /// Capture timestamp used by deterministic tests.
    pub captured_at: String,
    /// Export-safe summary for support and review.
    pub export_safe_summary: String,
}

impl PythonLaunchWedgeSnapshot {
    /// Stable record-kind tag for Python launch-wedge snapshots.
    pub const RECORD_KIND: &'static str = "python_nav_alpha_snapshot";

    /// Returns the protected symbol with the requested ref.
    pub fn symbol(&self, symbol_ref: &str) -> Option<&PythonSymbolSeed> {
        self.symbols
            .iter()
            .find(|symbol| symbol.symbol_ref == symbol_ref)
    }
}

/// Source anchor exported on semantic result rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonSourceAnchor {
    /// Stable source anchor ref.
    pub source_anchor_ref: String,
    /// Anchor source family.
    pub source_anchor_kind_class: PythonSourceAnchorKindClass,
    /// Canonical file ref, when known.
    pub canonical_file_ref: Option<String>,
    /// Symbol ref, when known.
    pub symbol_ref: Option<String>,
    /// Opaque range ref, when known.
    pub range_ref: Option<String>,
    /// Generated lineage ref, when known.
    pub generated_lineage_ref: Option<String>,
    /// Imported snapshot ref, when known.
    pub imported_snapshot_ref: Option<String>,
    /// Export-safe summary.
    pub summary: String,
}

impl PythonSourceAnchor {
    /// Builds a result source anchor from a fixture anchor and symbol ref.
    pub fn from_anchor(anchor: &PythonAnchorRef, symbol_ref: &str) -> Self {
        Self {
            source_anchor_ref: anchor.source_anchor_ref.clone(),
            source_anchor_kind_class: anchor.source_anchor_kind_class,
            canonical_file_ref: Some(anchor.canonical_file_ref.clone()),
            symbol_ref: Some(symbol_ref.to_owned()),
            range_ref: Some(anchor.range_ref.clone()),
            generated_lineage_ref: None,
            imported_snapshot_ref: None,
            summary: anchor.summary.clone(),
        }
    }
}

/// Provider snapshot embedded into Python assistance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonProviderSnapshot {
    /// Provider id.
    pub provider_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Provider health.
    pub provider_health_class: HealthState,
    /// Provider freshness.
    pub freshness_class: FreshnessClass,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Host identity ref, when known.
    pub host_identity_ref: Option<String>,
    /// Interpreter and environment context used by this provider result.
    pub interpreter_context: PythonInterpreterContext,
    /// Current provider epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Export-safe provider summary.
    pub summary: String,
}

impl PythonProviderSnapshot {
    /// Returns true when a surface must disclose provider degradation.
    pub const fn requires_disclosure(&self) -> bool {
        self.provider_health_class.requires_disclosure()
            || !matches!(self.freshness_class, FreshnessClass::AuthoritativeLive)
            || self.interpreter_context.requires_disclosure()
    }
}

/// Scope descriptor shared by hover, definition, references, and rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonScopeDescriptor {
    /// Scope requested by the caller.
    pub requested_scope_class: ScopeClaimClass,
    /// Scope actually materialized by the provider.
    pub materialized_scope_class: ScopeClaimClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Covered scope ref.
    pub covered_scope_ref: String,
    /// Omitted scope ref, when known.
    pub omitted_scope_ref: Option<String>,
    /// Export-safe caveat summary.
    pub caveat_summary: String,
}

impl PythonScopeDescriptor {
    /// Returns true when requested and materialized scope differ or limits exist.
    pub fn requires_scope_disclosure(&self) -> bool {
        self.requested_scope_class != self.materialized_scope_class
            || !self.scope_limit_classes.is_empty()
            || self.omitted_scope_ref.is_some()
    }
}

/// Ambiguity descriptor for one semantic result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonAmbiguityDescriptor {
    /// Number of ambiguous candidates.
    pub ambiguous_candidate_count: u32,
    /// Number of selected candidates.
    pub selected_candidate_count: u32,
    /// Whether a disambiguation surface is required.
    pub disambiguation_required: bool,
    /// Export-safe ambiguity summary.
    pub summary: String,
}

/// Evidence refs carried by one semantic result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonSemanticEvidenceBinding {
    /// Durable semantic result id.
    pub durable_result_id: String,
    /// Result provenance ref, when known.
    pub result_provenance_ref: Option<String>,
    /// Navigation artifact ref, when known.
    pub navigation_artifact_ref: Option<String>,
    /// Review packet ref, when known.
    pub review_packet_ref: Option<String>,
    /// AI citation anchor ref, when known.
    pub ai_citation_anchor_ref: Option<String>,
    /// Support export ref, when known.
    pub support_export_ref: Option<String>,
    /// Source evidence refs.
    pub source_evidence_refs: Vec<String>,
    /// Scope caveat refs.
    pub scope_caveat_refs: Vec<String>,
}

/// Exportable Python semantic result record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonSemanticResultRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub semantic_result_ref_schema_version: PythonSemanticResultSchemaVersion,
    /// Stable semantic result id.
    pub semantic_result_id: String,
    /// Result identity class.
    pub semantic_result_identity_class: PythonSemanticResultIdentityClass,
    /// Relation class.
    pub relation_class: PythonRelationClass,
    /// Source anchor.
    pub source_anchor: PythonSourceAnchor,
    /// Provider snapshot.
    pub provider_snapshot: PythonProviderSnapshot,
    /// Result confidence class.
    pub result_confidence_class: PythonResultConfidenceClass,
    /// Result completeness class.
    pub completeness_class: PythonCompletenessClass,
    /// Inline visibility class.
    pub inline_visibility_class: PythonInlineVisibilityClass,
    /// Scope descriptor.
    pub scope_descriptor: PythonScopeDescriptor,
    /// Ambiguity descriptor.
    pub ambiguity_descriptor: PythonAmbiguityDescriptor,
    /// Evidence binding.
    pub evidence_binding: PythonSemanticEvidenceBinding,
    /// Current epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Policy context.
    pub policy_context: ProviderPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Router decision id that admitted the result.
    pub router_decision_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl PythonSemanticResultRecord {
    /// Stable record-kind tag for semantic results.
    pub const RECORD_KIND: &'static str = "semantic_result_ref_record";

    /// Returns true when a consumer must show degraded, fallback, or scope state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.result_confidence_class.requires_disclosure()
            || self.completeness_class.requires_disclosure()
            || self.provider_snapshot.requires_disclosure()
            || self.scope_descriptor.requires_scope_disclosure()
    }
}

/// Exportable Python hover record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonHoverRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonSemanticResultSchemaVersion,
    /// Stable hover record id.
    pub hover_id: String,
    /// Target symbol ref.
    pub target_symbol_ref: String,
    /// Display label for the hover target.
    pub display_label: String,
    /// Layer that answered the hover.
    pub answer_layer_class: PythonAnswerLayerClass,
    /// Provider snapshot.
    pub provider_snapshot: PythonProviderSnapshot,
    /// Scope descriptor.
    pub scope_descriptor: PythonScopeDescriptor,
    /// Router decision id that admitted the hover.
    pub router_decision_id: String,
    /// Export-safe hover label.
    pub hover_label: String,
    /// Export-safe hover summary.
    pub hover_summary: String,
    /// Export-safe hover detail.
    pub hover_detail: String,
    /// Fallback summary, when a lower-authority path answered.
    pub fallback_summary: String,
    /// Degraded state projected from routing.
    pub degraded_state_class: DegradedStateClass,
    /// Policy context.
    pub policy_context: ProviderPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl PythonHoverRecord {
    /// Stable record-kind tag for hover records.
    pub const RECORD_KIND: &'static str = "python_hover_record";

    /// Returns true when hover output must render a caveat.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.answer_layer_class.is_fallback()
            || self.degraded_state_class != DegradedStateClass::None
            || self.provider_snapshot.requires_disclosure()
            || self.scope_descriptor.requires_scope_disclosure()
    }
}

/// Count summary for a Python references result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonReferenceCountSummary {
    /// Total known reference occurrences in the protected fixture.
    pub total_count: usize,
    /// Occurrences materialized in this result set.
    pub materialized_count: usize,
    /// Occurrences omitted by scope, provider, or fallback limits.
    pub omitted_count: usize,
    /// Materialized generated occurrences.
    pub generated_count: usize,
    /// Materialized read-only occurrences.
    pub readonly_count: usize,
}

/// Exportable Python references result set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonReferenceSetRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: PythonSemanticResultSchemaVersion,
    /// Stable reference set id.
    pub reference_set_id: String,
    /// Target symbol ref.
    pub target_symbol_ref: String,
    /// Materialized reference occurrence records.
    pub occurrence_results: Vec<PythonSemanticResultRecord>,
    /// Scope descriptor.
    pub scope_descriptor: PythonScopeDescriptor,
    /// Provider snapshot.
    pub provider_snapshot: PythonProviderSnapshot,
    /// Router decision id that admitted the reference set.
    pub router_decision_id: String,
    /// Count summary.
    pub count_summary: PythonReferenceCountSummary,
    /// Degraded state projected from routing.
    pub degraded_state_class: DegradedStateClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl PythonReferenceSetRecord {
    /// Stable record-kind tag for reference sets.
    pub const RECORD_KIND: &'static str = "python_reference_set_record";

    /// Returns true when references must render a caveat.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.degraded_state_class != DegradedStateClass::None
            || self.provider_snapshot.requires_disclosure()
            || self.scope_descriptor.requires_scope_disclosure()
            || self.count_summary.omitted_count > 0
            || self
                .occurrence_results
                .iter()
                .any(PythonSemanticResultRecord::requires_degraded_disclosure)
    }
}

/// Count summary for a Python rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenameCountSummary {
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
    /// Files or documents with proposed changes.
    pub changed_file_count: usize,
    /// Distinct symbol identities affected.
    pub changed_symbol_count: usize,
}

/// Affected scope row in a Python rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenameAffectedScopeRow {
    /// Requested scope class.
    pub requested_scope_class: ScopeClaimClass,
    /// Materialized scope class.
    pub materialized_scope_class: ScopeClaimClass,
    /// Coverage limits for this scope row.
    pub coverage_limit_classes: Vec<PythonRenameCoverageLimitClass>,
    /// Affected result refs in this scope.
    pub affected_result_refs: Vec<String>,
    /// Omitted result count for this scope.
    pub omitted_result_count: usize,
    /// Export-safe caveat summary.
    pub caveat_summary: String,
}

/// Warning row in a Python rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenameWarningRow {
    /// Warning class.
    pub warning_class: PythonRenameWarningClass,
    /// Count of occurrences behind the warning.
    pub warning_count: usize,
    /// Affected result refs.
    pub affected_result_refs: Vec<String>,
    /// Export-safe warning summary.
    pub summary: String,
}

/// Checkpoint and rollback descriptor for a Python rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenameCheckpointDescriptor {
    /// Checkpoint posture.
    pub checkpoint_class: PythonCheckpointClass,
    /// Checkpoint ref, when captured.
    pub checkpoint_ref: Option<String>,
    /// Rollback ref, when available.
    pub rollback_ref: Option<String>,
    /// Rollback path class.
    pub rollback_path_class: PythonRollbackPathClass,
    /// Export-safe summary.
    pub summary: String,
}

/// Evidence refs carried by a Python rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenameEvidenceBinding {
    /// Durable preview id.
    pub durable_preview_id: String,
    /// Result provenance ref, when known.
    pub result_provenance_ref: Option<String>,
    /// Refactor preview ref, when known.
    pub refactor_preview_ref: Option<String>,
    /// Review packet ref, when known.
    pub review_packet_ref: Option<String>,
    /// AI citation anchor ref, when known.
    pub ai_citation_anchor_ref: Option<String>,
    /// Support export ref, when known.
    pub support_export_ref: Option<String>,
    /// Source evidence refs.
    pub source_evidence_refs: Vec<String>,
    /// Scope caveat refs.
    pub scope_caveat_refs: Vec<String>,
}

/// Exportable Python rename preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonRenamePreviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub rename_preview_schema_version: PythonRenamePreviewSchemaVersion,
    /// Stable rename preview id.
    pub rename_preview_id: String,
    /// Target semantic result ref.
    pub target_semantic_result_ref: String,
    /// Opaque requested new name ref.
    pub requested_new_name_ref: String,
    /// Preview completeness class.
    pub preview_completeness_class: PythonRenamePreviewCompletenessClass,
    /// Apply posture class.
    pub apply_posture_class: PythonApplyPostureClass,
    /// Count summary.
    pub count_summary: PythonRenameCountSummary,
    /// Affected scope rows.
    pub affected_scope_rows: Vec<PythonRenameAffectedScopeRow>,
    /// Warning rows.
    pub warning_rows: Vec<PythonRenameWarningRow>,
    /// Checkpoint descriptor.
    pub checkpoint_descriptor: PythonRenameCheckpointDescriptor,
    /// Provider snapshot.
    pub provider_snapshot: PythonProviderSnapshot,
    /// Current epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Evidence binding.
    pub evidence_binding: PythonRenameEvidenceBinding,
    /// Policy context.
    pub policy_context: ProviderPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Router decision id that admitted the preview.
    pub router_decision_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl PythonRenamePreviewRecord {
    /// Stable record-kind tag for rename previews.
    pub const RECORD_KIND: &'static str = "rename_preview_record";

    /// Returns true when the preview is ready for apply after inspection.
    pub const fn is_ready_for_apply_after_preview(&self) -> bool {
        matches!(
            self.apply_posture_class,
            PythonApplyPostureClass::ReadyForApplyAfterPreview
        )
    }

    /// Returns true when a consumer must disclose degraded or partial scope.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.preview_completeness_class.blocks_direct_apply()
            || !matches!(
                self.apply_posture_class,
                PythonApplyPostureClass::ReadyForApplyAfterPreview
            )
            || self.provider_snapshot.requires_disclosure()
            || self.count_summary.unresolved_count > 0
            || self.count_summary.protected_count > 0
            || self.count_summary.skipped_count > 0
            || self
                .affected_scope_rows
                .iter()
                .any(|row| !row.coverage_limit_classes.is_empty())
    }
}
