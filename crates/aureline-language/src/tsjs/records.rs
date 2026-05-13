use serde::{Deserialize, Serialize};

use crate::lsp_router::{
    DegradedStateClass, EpochBinding, FreshnessClass, HealthState, LocalityClass, ProviderFamily,
    ProviderPolicyContext, RedactionClass, RouterTrustState, ScopeClaimClass, ScopeLimitClass,
};

/// Integer schema version for TS/JS launch-wedge assistance records.
pub type TsJsSemanticResultSchemaVersion = u32;

/// Integer schema version for TS/JS rename-preview records.
pub type TsJsRenamePreviewSchemaVersion = u32;

/// Schema version used by the TS/JS launch-wedge alpha record family.
pub const TSJS_NAV_ALPHA_SCHEMA_VERSION: TsJsSemanticResultSchemaVersion = 1;

/// TypeScript or JavaScript symbol kind used by protected fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsSymbolKindClass {
    /// Function or function-valued binding.
    Function,
    /// React or framework component.
    Component,
    /// Hook-like callable.
    Hook,
    /// Constant or value binding.
    Constant,
    /// Type, interface, or class symbol.
    Type,
    /// Module-level export surface.
    Module,
}

impl TsJsSymbolKindClass {
    /// Returns the stable schema token for this symbol kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Function => "function",
            Self::Component => "component",
            Self::Hook => "hook",
            Self::Constant => "constant",
            Self::Type => "type",
            Self::Module => "module",
        }
    }
}

/// Reference access kind shared by references and rename previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsAccessKindClass {
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

impl TsJsAccessKindClass {
    /// Returns the semantic relation class represented by this access kind.
    pub const fn relation_class(self) -> TsJsRelationClass {
        match self {
            Self::Read => TsJsRelationClass::ReadReference,
            Self::Write => TsJsRelationClass::WriteReference,
            Self::Call => TsJsRelationClass::CallReference,
            Self::Import | Self::Export => TsJsRelationClass::ImportOrExportReference,
            Self::Generated => TsJsRelationClass::GeneratedOrFrameworkReference,
        }
    }
}

/// Authored, generated, or external posture for one occurrence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsGeneratedOrExternalStateClass {
    /// Occurrence is in authored workspace source.
    AuthoredSource,
    /// Occurrence is in generated or paired source.
    GeneratedSource,
    /// Occurrence is in an external dependency or imported declaration.
    ExternalDependency,
    /// Occurrence is read-only for this workspace or policy posture.
    ReadOnlySource,
}

impl TsJsGeneratedOrExternalStateClass {
    /// Returns true when a rename should not mutate this occurrence.
    pub const fn blocks_rename_candidate(self) -> bool {
        !matches!(self, Self::AuthoredSource)
    }
}

/// Answering layer for a TS/JS language-assistance record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsAnswerLayerClass {
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

impl TsJsAnswerLayerClass {
    /// Returns true when the answer is lower than compatibility semantics.
    pub const fn is_fallback(self) -> bool {
        matches!(self, Self::Layer1SyntaxStructure)
    }
}

/// Semantic-result identity class emitted by TS/JS navigation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsSemanticResultIdentityClass {
    /// Definition target for the selected symbol.
    Definition,
    /// Reference occurrence for the selected symbol.
    Reference,
    /// Generated or imported reference occurrence.
    ImportedOrGeneratedReference,
}

/// Semantic relation class emitted by TS/JS navigation records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsRelationClass {
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
pub enum TsJsResultConfidenceClass {
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

impl TsJsResultConfidenceClass {
    /// Returns true when the state requires a visible caveat.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact | Self::Indexed)
    }
}

/// Completeness class for a semantic navigation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsCompletenessClass {
    /// Complete for the declared scope.
    CompleteForDeclaredScope,
    /// Partial for the declared scope.
    PartialForDeclaredScope,
    /// Stale for the declared scope.
    StaleForDeclaredScope,
    /// Unavailable for the declared scope.
    UnavailableForDeclaredScope,
}

impl TsJsCompletenessClass {
    /// Returns true when consumers must disclose incomplete truth.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::CompleteForDeclaredScope)
    }
}

/// Inline visibility policy for a semantic result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsInlineVisibilityClass {
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

/// Source-anchor family for a TS/JS result.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsSourceAnchorKindClass {
    /// Anchor points into current workspace source.
    WorkspaceSourceAnchor,
    /// Anchor points through generated lineage.
    GeneratedLineageAnchor,
    /// Anchor came from a provider overlay.
    ProviderOverlayAnchor,
    /// Anchor could not be resolved.
    UnresolvedAnchor,
}

/// Rename-preview completeness class for TS/JS previews.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsRenamePreviewCompletenessClass {
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

impl TsJsRenamePreviewCompletenessClass {
    /// Returns true when the preview must not be applied directly.
    pub const fn blocks_direct_apply(self) -> bool {
        !matches!(
            self,
            Self::FullWorkspaceComplete | Self::CompleteForRequestedScope
        )
    }
}

/// Apply posture for a TS/JS rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsApplyPostureClass {
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
pub enum TsJsRenameCoverageLimitClass {
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
}

/// Warning class attached to a rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsRenameWarningClass {
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
}

/// Checkpoint posture attached to a rename preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TsJsCheckpointClass {
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
pub enum TsJsRollbackPathClass {
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

/// Workspace, workset, and policy context for TS/JS assistance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsWorkspaceContext {
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
    /// TypeScript or JavaScript config root governing the subject.
    pub config_root_ref: String,
    /// Execution context anchoring Node/toolchain identity.
    pub execution_context_id: String,
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

impl TsJsWorkspaceContext {
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
pub struct TsJsAnchorRef {
    /// Stable source anchor ref.
    pub source_anchor_ref: String,
    /// Anchor source family.
    pub source_anchor_kind_class: TsJsSourceAnchorKindClass,
    /// Canonical file ref.
    pub canonical_file_ref: String,
    /// Workspace-relative path used only for export-safe display.
    pub workspace_relative_path: String,
    /// Opaque range ref.
    pub range_ref: String,
    /// Export-safe summary for the anchor.
    pub summary: String,
}

/// Protected TS/JS fixture symbol.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsSymbolSeed {
    /// Stable symbol ref.
    pub symbol_ref: String,
    /// Human-readable symbol label.
    pub display_name: String,
    /// Symbol kind.
    pub kind_class: TsJsSymbolKindClass,
    /// Export-safe hover label.
    pub hover_label: String,
    /// Export-safe hover summary.
    pub hover_summary: String,
    /// Export-safe hover detail.
    pub hover_detail: String,
    /// Definition anchor for the symbol.
    pub definition_anchor: TsJsAnchorRef,
    /// Reference occurrence seeds for the symbol.
    pub occurrences: Vec<TsJsOccurrenceSeed>,
}

impl TsJsSymbolSeed {
    /// Returns candidate occurrences that are references rather than the definition anchor.
    pub fn reference_occurrences(&self) -> impl Iterator<Item = &TsJsOccurrenceSeed> {
        self.occurrences.iter()
    }
}

/// Protected TS/JS fixture occurrence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsOccurrenceSeed {
    /// Stable occurrence ref.
    pub occurrence_ref: String,
    /// Symbol this occurrence targets.
    pub symbol_ref: String,
    /// Occurrence anchor.
    pub anchor: TsJsAnchorRef,
    /// Access kind for references and previews.
    pub access_kind_class: TsJsAccessKindClass,
    /// Generated, external, or authored posture.
    pub generated_or_external_state_class: TsJsGeneratedOrExternalStateClass,
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

impl TsJsOccurrenceSeed {
    /// Returns true when this occurrence may be changed by a semantic rename preview.
    pub const fn rename_writable_authored_candidate(&self) -> bool {
        self.rename_candidate
            && !self.readonly
            && !self
                .generated_or_external_state_class
                .blocks_rename_candidate()
    }
}

/// Fixture-backed TS/JS launch-wedge snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsLaunchWedgeSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: TsJsSemanticResultSchemaVersion,
    /// Requested language id.
    pub language_id: String,
    /// Workspace, scope, and policy context.
    pub workspace_context: TsJsWorkspaceContext,
    /// Epoch bindings current for the fixture.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Protected TS/JS symbols in the fixture.
    pub symbols: Vec<TsJsSymbolSeed>,
    /// Capture timestamp used by deterministic tests.
    pub captured_at: String,
    /// Export-safe summary for support and review.
    pub export_safe_summary: String,
}

impl TsJsLaunchWedgeSnapshot {
    /// Stable record-kind tag for TS/JS launch-wedge snapshots.
    pub const RECORD_KIND: &'static str = "tsjs_nav_alpha_snapshot";

    /// Returns the protected symbol with the requested ref.
    pub fn symbol(&self, symbol_ref: &str) -> Option<&TsJsSymbolSeed> {
        self.symbols
            .iter()
            .find(|symbol| symbol.symbol_ref == symbol_ref)
    }
}

/// Source anchor exported on semantic result rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsSourceAnchor {
    /// Stable source anchor ref.
    pub source_anchor_ref: String,
    /// Anchor source family.
    pub source_anchor_kind_class: TsJsSourceAnchorKindClass,
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

impl TsJsSourceAnchor {
    /// Builds a result source anchor from a fixture anchor and symbol ref.
    pub fn from_anchor(anchor: &TsJsAnchorRef, symbol_ref: &str) -> Self {
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

/// Provider snapshot embedded into TS/JS assistance records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsProviderSnapshot {
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
    /// Current provider epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Export-safe provider summary.
    pub summary: String,
}

impl TsJsProviderSnapshot {
    /// Returns true when a surface must disclose provider degradation.
    pub const fn requires_disclosure(&self) -> bool {
        self.provider_health_class.requires_disclosure()
            || !matches!(self.freshness_class, FreshnessClass::AuthoritativeLive)
    }
}

/// Scope descriptor shared by hover, definition, references, and rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsScopeDescriptor {
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

impl TsJsScopeDescriptor {
    /// Returns true when requested and materialized scope differ or limits exist.
    pub fn requires_scope_disclosure(&self) -> bool {
        self.requested_scope_class != self.materialized_scope_class
            || !self.scope_limit_classes.is_empty()
            || self.omitted_scope_ref.is_some()
    }
}

/// Ambiguity descriptor for one semantic result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsAmbiguityDescriptor {
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
pub struct TsJsSemanticEvidenceBinding {
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

/// Exportable TS/JS semantic result record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsSemanticResultRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub semantic_result_ref_schema_version: TsJsSemanticResultSchemaVersion,
    /// Stable semantic result id.
    pub semantic_result_id: String,
    /// Result identity class.
    pub semantic_result_identity_class: TsJsSemanticResultIdentityClass,
    /// Relation class.
    pub relation_class: TsJsRelationClass,
    /// Source anchor.
    pub source_anchor: TsJsSourceAnchor,
    /// Provider snapshot.
    pub provider_snapshot: TsJsProviderSnapshot,
    /// Result confidence class.
    pub result_confidence_class: TsJsResultConfidenceClass,
    /// Result completeness class.
    pub completeness_class: TsJsCompletenessClass,
    /// Inline visibility class.
    pub inline_visibility_class: TsJsInlineVisibilityClass,
    /// Scope descriptor.
    pub scope_descriptor: TsJsScopeDescriptor,
    /// Ambiguity descriptor.
    pub ambiguity_descriptor: TsJsAmbiguityDescriptor,
    /// Evidence binding.
    pub evidence_binding: TsJsSemanticEvidenceBinding,
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

impl TsJsSemanticResultRecord {
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

/// Exportable TS/JS hover record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsHoverRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: TsJsSemanticResultSchemaVersion,
    /// Stable hover record id.
    pub hover_id: String,
    /// Target symbol ref.
    pub target_symbol_ref: String,
    /// Display label for the hover target.
    pub display_label: String,
    /// Layer that answered the hover.
    pub answer_layer_class: TsJsAnswerLayerClass,
    /// Provider snapshot.
    pub provider_snapshot: TsJsProviderSnapshot,
    /// Scope descriptor.
    pub scope_descriptor: TsJsScopeDescriptor,
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

impl TsJsHoverRecord {
    /// Stable record-kind tag for hover records.
    pub const RECORD_KIND: &'static str = "tsjs_hover_record";

    /// Returns true when hover output must render a caveat.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.answer_layer_class.is_fallback()
            || self.degraded_state_class != DegradedStateClass::None
            || self.provider_snapshot.requires_disclosure()
            || self.scope_descriptor.requires_scope_disclosure()
    }
}

/// Count summary for a TS/JS references result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsReferenceCountSummary {
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

/// Exportable TS/JS references result set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsReferenceSetRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: TsJsSemanticResultSchemaVersion,
    /// Stable reference set id.
    pub reference_set_id: String,
    /// Target symbol ref.
    pub target_symbol_ref: String,
    /// Materialized reference occurrence records.
    pub occurrence_results: Vec<TsJsSemanticResultRecord>,
    /// Scope descriptor.
    pub scope_descriptor: TsJsScopeDescriptor,
    /// Provider snapshot.
    pub provider_snapshot: TsJsProviderSnapshot,
    /// Router decision id that admitted the reference set.
    pub router_decision_id: String,
    /// Count summary.
    pub count_summary: TsJsReferenceCountSummary,
    /// Degraded state projected from routing.
    pub degraded_state_class: DegradedStateClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl TsJsReferenceSetRecord {
    /// Stable record-kind tag for reference sets.
    pub const RECORD_KIND: &'static str = "tsjs_reference_set_record";

    /// Returns true when references must render a caveat.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.degraded_state_class != DegradedStateClass::None
            || self.provider_snapshot.requires_disclosure()
            || self.scope_descriptor.requires_scope_disclosure()
            || self.count_summary.omitted_count > 0
            || self
                .occurrence_results
                .iter()
                .any(TsJsSemanticResultRecord::requires_degraded_disclosure)
    }
}

/// Count summary for a TS/JS rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenameCountSummary {
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

/// Affected scope row in a TS/JS rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenameAffectedScopeRow {
    /// Requested scope class.
    pub requested_scope_class: ScopeClaimClass,
    /// Materialized scope class.
    pub materialized_scope_class: ScopeClaimClass,
    /// Coverage limits for this scope row.
    pub coverage_limit_classes: Vec<TsJsRenameCoverageLimitClass>,
    /// Affected result refs in this scope.
    pub affected_result_refs: Vec<String>,
    /// Omitted result count for this scope.
    pub omitted_result_count: usize,
    /// Export-safe caveat summary.
    pub caveat_summary: String,
}

/// Warning row in a TS/JS rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenameWarningRow {
    /// Warning class.
    pub warning_class: TsJsRenameWarningClass,
    /// Count of occurrences behind the warning.
    pub warning_count: usize,
    /// Affected result refs.
    pub affected_result_refs: Vec<String>,
    /// Export-safe warning summary.
    pub summary: String,
}

/// Checkpoint and rollback descriptor for a TS/JS rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenameCheckpointDescriptor {
    /// Checkpoint posture.
    pub checkpoint_class: TsJsCheckpointClass,
    /// Checkpoint ref, when captured.
    pub checkpoint_ref: Option<String>,
    /// Rollback ref, when available.
    pub rollback_ref: Option<String>,
    /// Rollback path class.
    pub rollback_path_class: TsJsRollbackPathClass,
    /// Export-safe summary.
    pub summary: String,
}

/// Evidence refs carried by a TS/JS rename preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenameEvidenceBinding {
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

/// Exportable TS/JS rename preview record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TsJsRenamePreviewRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub rename_preview_schema_version: TsJsRenamePreviewSchemaVersion,
    /// Stable rename preview id.
    pub rename_preview_id: String,
    /// Target semantic result ref.
    pub target_semantic_result_ref: String,
    /// Opaque requested new name ref.
    pub requested_new_name_ref: String,
    /// Preview completeness class.
    pub preview_completeness_class: TsJsRenamePreviewCompletenessClass,
    /// Apply posture class.
    pub apply_posture_class: TsJsApplyPostureClass,
    /// Count summary.
    pub count_summary: TsJsRenameCountSummary,
    /// Affected scope rows.
    pub affected_scope_rows: Vec<TsJsRenameAffectedScopeRow>,
    /// Warning rows.
    pub warning_rows: Vec<TsJsRenameWarningRow>,
    /// Checkpoint descriptor.
    pub checkpoint_descriptor: TsJsRenameCheckpointDescriptor,
    /// Provider snapshot.
    pub provider_snapshot: TsJsProviderSnapshot,
    /// Current epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Evidence binding.
    pub evidence_binding: TsJsRenameEvidenceBinding,
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

impl TsJsRenamePreviewRecord {
    /// Stable record-kind tag for rename previews.
    pub const RECORD_KIND: &'static str = "rename_preview_record";

    /// Returns true when the preview is ready for apply after inspection.
    pub const fn is_ready_for_apply_after_preview(&self) -> bool {
        matches!(
            self.apply_posture_class,
            TsJsApplyPostureClass::ReadyForApplyAfterPreview
        )
    }

    /// Returns true when a consumer must disclose degraded or partial scope.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.preview_completeness_class.blocks_direct_apply()
            || !matches!(
                self.apply_posture_class,
                TsJsApplyPostureClass::ReadyForApplyAfterPreview
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
