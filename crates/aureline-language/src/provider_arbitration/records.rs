use serde::{Deserialize, Serialize};

/// Integer schema version for [`ProviderHealthStateRecord`] payloads.
pub type ProviderHealthStateSchemaVersion = u32;

/// Integer schema version for [`ArbitrationDecisionRecord`] payloads.
pub type ArbitrationDecisionSchemaVersion = u32;

/// Schema version used by provider-health-state inspector rows.
pub const PROVIDER_HEALTH_STATE_SCHEMA_VERSION: ProviderHealthStateSchemaVersion = 1;

/// Schema version used by arbitration-decision records.
pub const ARBITRATION_DECISION_SCHEMA_VERSION: ArbitrationDecisionSchemaVersion = 1;

/// Stable record-kind tag for provider-health-state rows.
pub const PROVIDER_HEALTH_STATE_RECORD_KIND: &str = "provider_health_state_record";

/// Stable record-kind tag for arbitration-decision records.
pub const ARBITRATION_DECISION_RECORD_KIND: &str = "arbitration_decision_record";

/// Closed language-action lane vocabulary protected by the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanguageActionLaneClass {
    /// Definition or target lookup.
    Definition,
    /// Reference set lookup.
    References,
    /// Rename or rename preview.
    Rename,
    /// Formatting.
    Formatting,
    /// Organize imports.
    OrganizeImports,
    /// Code action or quick fix.
    CodeAction,
}

/// Frozen provider family vocabulary, mirrored from the arbitration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamily {
    /// Syntax or local text provider.
    Syntax,
    /// Project graph provider.
    ProjectGraph,
    /// Language server provider.
    LanguageServer,
    /// Framework pack provider.
    FrameworkPack,
    /// Notebook adapter provider.
    NotebookAdapter,
    /// Generated-source bridge provider.
    GeneratedSourceBridge,
    /// AI assist provider.
    AiAssist,
}

/// Frozen provider role vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRoleClass {
    /// Primary semantic provider.
    PrimarySemantic,
    /// Secondary semantic provider.
    SecondarySemantic,
    /// Framework overlay provider.
    FrameworkOverlay,
    /// Notebook projection provider.
    NotebookProjection,
    /// Generated artifact overlay provider.
    GeneratedOverlay,
    /// Text fallback provider.
    TextFallback,
    /// Assist-only provider.
    AssistOnly,
}

/// Frozen provider-health vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthState {
    /// Provider is ready for fresh requests.
    Ready,
    /// Provider is warming or reconnecting.
    Warming,
    /// Provider is reachable but degraded.
    Degraded,
    /// Provider may only reuse cached results.
    CachedOnly,
    /// Provider is blocked by policy.
    PolicyBlocked,
    /// Provider does not support the requested capability.
    CapabilityMissing,
    /// Provider is quarantined after a crash loop.
    CrashLoopQuarantined,
    /// Provider cannot be reached.
    Unavailable,
}

/// Frozen freshness vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Fresh against the current epoch.
    AuthoritativeLive,
    /// Warm cached result still in grace.
    WarmCached,
    /// Cached result below ideal posture.
    DegradedCached,
    /// Result past freshness floor.
    Stale,
    /// Freshness could not be proven.
    Unverified,
}

/// Scope a provider currently claims to cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClaimClass {
    /// Active file only.
    SingleFile,
    /// Active notebook cell only.
    NotebookCell,
    /// Currently loaded slice.
    LoadedSlice,
    /// Active workset.
    ActiveWorkset,
    /// Entire admitted workspace.
    WholeWorkspace,
    /// No admissible claim.
    Unavailable,
}

/// Completeness for the claimed scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessClass {
    /// Complete for the claimed scope.
    CompleteForClaimedScope,
    /// Partial for the claimed scope.
    PartialForClaimedScope,
    /// Unavailable for the claimed scope.
    UnavailableForClaimedScope,
}

/// Where the provider ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// In-process provider.
    LocalInProcess,
    /// Local sidecar provider.
    LocalSidecar,
    /// Workspace remote agent.
    WorkspaceRemoteAgent,
    /// Managed service.
    ManagedService,
    /// Imported snapshot.
    ImportedSnapshot,
}

/// Per-lane support posture rendered on the health strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneSupportClass {
    /// Provider can serve exact (semantic, live) results on the lane.
    Exact,
    /// Provider can serve heuristic or non-semantic results on the lane.
    Heuristic,
    /// Provider is partial for the lane.
    Partial,
    /// Provider's lane data is stale.
    Stale,
    /// Provider does not currently support the lane.
    Unsupported,
}

/// Retry actions exposed by the health strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetryActionClass {
    /// No retry path is admissible.
    NotAvailable,
    /// Soft retry of the in-flight request.
    SoftRetry,
    /// Restart the provider session.
    RestartSession,
    /// Warm from the last cached snapshot.
    WarmFromSnapshot,
    /// Manual recovery is required from the operator.
    ManualRecoveryRequired,
}

/// Isolate actions exposed by the health strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IsolateActionClass {
    /// No isolate action is admissible.
    NotAvailable,
    /// Isolate the provider for the current session.
    IsolateForSession,
    /// Isolate the provider until explicit recovery.
    IsolateUntilRecovery,
    /// Quarantine the provider for the entire workspace.
    QuarantineForWorkspace,
}

/// Recovery hint rendered next to retry/isolate controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryHintClass {
    /// No recovery hint applies.
    None,
    /// Wait for the provider to warm up.
    WaitForWarmUp,
    /// Wait for the index to finish.
    WaitForIndex,
    /// Reconnect a remote provider.
    ReconnectRemote,
    /// Reload provider configuration.
    ReloadConfig,
    /// Reinstall the language pack.
    ReinstallPack,
    /// Review policy / trust settings.
    ReviewPolicy,
    /// Report the issue to support.
    ReportToSupport,
}

/// Downgraded-promise reason rendered on the health strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradedPromiseReasonClass {
    /// No downgrade applies.
    None,
    /// Narrowed from claimed scope to single file.
    NarrowedToFileLocal,
    /// Narrowed from claimed scope to active workset.
    NarrowedToActiveWorkset,
    /// Narrowed from claimed scope to notebook cell.
    NarrowedToNotebookCell,
    /// Narrowed from claimed scope to loaded slice.
    NarrowedToLoadedSlice,
    /// Semantic provider lost; text fallback won.
    FallbackToText,
    /// Semantic provider lost; heuristic fallback won.
    FallbackToHeuristic,
    /// Stale cache reused with explicit disclosure.
    StaleCacheReuse,
    /// Provider excluded after a crash loop.
    CrashLoopExcluded,
    /// Provider excluded by policy.
    PolicyExcluded,
    /// Remote provider unreachable.
    RemoteUnreachable,
    /// AI assist is admissible only as advisory truth.
    AdvisoryOnlyAiAssist,
}

/// Fault domain that owns restart accounting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainClass {
    /// Shell interaction core.
    ShellInteractionCore,
    /// Workspace knowledge workers.
    WorkspaceKnowledgeWorkers,
    /// Session-scoped execution hosts.
    SessionScopedExecutionHosts,
    /// Extension and tool hosts.
    ExtensionAndToolHosts,
    /// AI and external tool brokers.
    AiAndExternalToolBrokers,
    /// Remote connectors.
    RemoteConnectors,
    /// Policy, entitlement, and verifier helpers.
    PolicyEntitlementAndVerifierHelpers,
}

/// Closed epoch role vocabulary reused from the arbitration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochRoleClass {
    /// Workspace-scope epoch.
    WorkspaceScope,
    /// Graph snapshot epoch.
    GraphSnapshot,
    /// Provider session epoch.
    ProviderSession,
    /// Language project model epoch.
    LanguageProjectModel,
    /// Framework pack snapshot epoch.
    FrameworkPackSnapshot,
    /// Notebook projection epoch.
    NotebookProjection,
    /// Generated lineage epoch.
    GeneratedLineage,
    /// AI context epoch.
    AiContext,
}

/// Trust state shared with the arbitration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    /// Workspace trust admits the inspector decision.
    Trusted,
    /// Trust policy narrows the inspector decision.
    Restricted,
}

/// Redaction posture for exportable inspector records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-only rows are safe by default.
    MetadataSafeDefault,
    /// Operator-only restricted row.
    OperatorOnlyRestricted,
    /// Internal support restricted row.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
}

/// Requested authority floor for an arbitration decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedAuthorityFloorClass {
    /// Authoritative-only.
    AuthoritativeRequired,
    /// Authoritative preferred with labeled fallback allowed.
    AuthoritativePreferred,
    /// Advisory or fallback truth is acceptable.
    AdvisoryAllowed,
}

/// Confidence outcome rendered by the inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceOutcomeClass {
    /// Exact (semantic, complete, live) outcome.
    Exact,
    /// Heuristic outcome (text or structural fallback).
    Heuristic,
    /// Partial outcome (scope narrower than claimed).
    Partial,
    /// Stale outcome (freshness below floor).
    Stale,
    /// No admissible outcome.
    Unavailable,
}

/// Conflict class reused from the arbitration contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictClass {
    /// No conflict.
    None,
    /// Providers disagreed about target sets.
    TargetSetDisagreement,
    /// Providers disagreed about scope coverage.
    ScopeCoverageDisagreement,
    /// Framework and language server disagreed.
    FrameworkLanguageDisagreement,
    /// Semantic and text providers disagreed.
    SemanticTextDisagreement,
    /// Providers disagreed about edit safety.
    EditSafetyDisagreement,
    /// Providers disagreed about freshness or epoch.
    FreshnessOrEpochDisagreement,
    /// Providers disagreed about generated boundary.
    GeneratedBoundaryDisagreement,
    /// Providers disagreed about notebook boundary.
    NotebookBoundaryDisagreement,
    /// Providers disagreed about narrative facts.
    NarrativeFactDisagreement,
}

/// How disagreement must surface to the user.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisagreementVisibilityClass {
    /// No disagreement to render.
    None,
    /// Inline conflict panel attached to the answer.
    InlineConflictPanel,
    /// Side-panel disagreement inspector.
    SidePanelDisagreementInspector,
    /// Preview blocked until explicit review.
    PreviewBlockedUntilReview,
}

/// Apply gate the surface must enforce.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyGateClass {
    /// User may apply after inspecting the result.
    ReadyToApply,
    /// Preview is required before apply.
    PreviewRequired,
    /// Side-branch review is required before apply.
    SideBranchRequired,
    /// Apply blocked because providers disagreed.
    BlockedForDisagreement,
    /// Apply blocked because completeness is partial.
    BlockedForPartialScope,
    /// Apply blocked because provider health is unavailable.
    BlockedForHealth,
    /// Inspect-only path; no apply lane.
    InspectOnly,
}

/// Fallback label rendered next to the visible answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackLabelClass {
    /// No fallback label.
    None,
    /// Text fallback label.
    TextFallback,
    /// Heuristic fallback label.
    HeuristicFallback,
    /// Syntax / file-local fallback label.
    SyntaxFileLocalFallback,
    /// Cached semantic reuse label.
    CachedSemanticReuse,
    /// Advisory AI-assist-only label.
    AdvisoryAiAssistOnly,
    /// Unsupported operation label.
    UnsupportedOperation,
}

/// Consumer surface that reads the arbitration record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Editor chrome (status bar, badges, hover badges, etc.).
    EditorChrome,
    /// Quick-fix preview.
    QuickFixPreview,
    /// Diagnostics detail panel.
    DiagnosticsDetail,
    /// Command result panel.
    CommandResult,
    /// CLI / headless inspector.
    CliHeadlessInspect,
    /// Support export bundle.
    SupportExport,
}

/// Opaque epoch binding carried by inspector rows.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EpochBinding {
    /// Epoch role.
    pub epoch_role_class: EpochRoleClass,
    /// Epoch reference.
    pub epoch_ref: String,
}

/// Minimal policy/trust context exported with inspector rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    /// Policy epoch.
    pub policy_epoch: String,
    /// Trust state.
    pub trust_state: TrustState,
    /// Execution context anchoring toolchain identity.
    pub execution_context_id: String,
}

/// Per-lane support row rendered on the health strip.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct LaneSupportRow {
    /// Lane covered by the row.
    pub language_action_lane_class: LanguageActionLaneClass,
    /// Per-lane support posture.
    pub lane_support_class: LaneSupportClass,
    /// Export-safe support summary.
    pub summary: String,
}

/// Retry/isolate controls exposed on the health strip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetryIsolateControls {
    /// Retry action posture.
    pub retry_action_class: RetryActionClass,
    /// Isolate action posture.
    pub isolate_action_class: IsolateActionClass,
    /// Recovery hint rendered next to the controls.
    pub recovery_hint_class: RecoveryHintClass,
    /// Export-safe control summary.
    pub summary: String,
}

/// Downgraded-promise block carried by inspector rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradedPromiseBlock {
    /// Downgraded-promise reason.
    pub downgraded_promise_reason_class: DowngradedPromiseReasonClass,
    /// Export-safe downgraded-promise summary.
    pub summary: String,
}

/// Provider health-state inspector row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHealthStateRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub provider_health_state_schema_version: ProviderHealthStateSchemaVersion,
    /// Stable health-state id.
    pub provider_health_state_id: String,
    /// Provider id.
    pub provider_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Provider role.
    pub provider_role_class: ProviderRoleClass,
    /// Optional connected pack / server ref.
    #[serde(default)]
    pub connected_pack_ref: Option<String>,
    /// Provider health state.
    pub health_state: HealthState,
    /// Optional health-reason summary.
    #[serde(default)]
    pub health_reason_summary: Option<String>,
    /// Freshness class.
    pub freshness_class: FreshnessClass,
    /// Last time the provider answered with ready+authoritative-live.
    #[serde(default)]
    pub last_good_at: Option<String>,
    /// Scope claim class.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness class.
    pub completeness_class: CompletenessClass,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Optional host identity ref.
    #[serde(default)]
    pub host_identity_ref: Option<String>,
    /// Fault domain class.
    pub fault_domain_class: FaultDomainClass,
    /// Restart strikes in the active accounting window.
    pub restart_strike_count: u32,
    /// Optional quarantine ref.
    #[serde(default)]
    pub quarantine_ref: Option<String>,
    /// Per-lane support rows.
    pub lane_support_rows: Vec<LaneSupportRow>,
    /// Retry/isolate controls.
    pub retry_isolate_controls: RetryIsolateControls,
    /// Downgraded-promise block.
    pub downgraded_promise_block: DowngradedPromiseBlock,
    /// Current epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Policy context.
    pub policy_context: PolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl ProviderHealthStateRecord {
    /// Returns true when the row must surface a non-`ready` posture to users.
    pub fn requires_disclosure(&self) -> bool {
        self.health_state != HealthState::Ready
            || self.freshness_class != FreshnessClass::AuthoritativeLive
            || self
                .downgraded_promise_block
                .downgraded_promise_reason_class
                != DowngradedPromiseReasonClass::None
    }
}

/// One provider order row in the arbitration decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderOrderRow {
    /// 1-based rank.
    pub rank: u32,
    /// Provider id.
    pub provider_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Provider role.
    pub provider_role_class: ProviderRoleClass,
    /// Reference into the matching provider-health-state row.
    pub provider_health_state_ref: String,
    /// Export-safe row summary.
    pub summary: String,
}

/// Consumer routing row for the arbitration decision.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConsumerRoutingRow {
    /// Consumer surface.
    pub consumer_surface_class: ConsumerSurfaceClass,
    /// Export-safe routing summary.
    pub summary: String,
}

/// Disagreement block for the arbitration decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisagreementBlock {
    /// Conflict class.
    pub conflict_class: ConflictClass,
    /// Disagreement visibility class.
    pub disagreement_visibility_class: DisagreementVisibilityClass,
    /// Export-safe disagreement summary.
    pub summary: String,
}

/// Back-references into existing record kinds the decision projects from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedRecordRefs {
    /// Capability negotiation packet ref.
    #[serde(default)]
    pub capability_negotiation_packet_ref: Option<String>,
    /// Result provenance record ref.
    #[serde(default)]
    pub result_provenance_ref: Option<String>,
    /// Router decision record ref.
    #[serde(default)]
    pub router_decision_ref: Option<String>,
}

/// Arbitration decision record consumed by the inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrationDecisionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub arbitration_decision_schema_version: ArbitrationDecisionSchemaVersion,
    /// Stable arbitration-decision id.
    pub arbitration_decision_id: String,
    /// Lane covered by the decision.
    pub language_action_lane_class: LanguageActionLaneClass,
    /// Requested authority floor.
    pub requested_authority_floor_class: RequestedAuthorityFloorClass,
    /// Requested scope claim.
    pub requested_scope_claim_class: ScopeClaimClass,
    /// Requested subject ref.
    pub requested_subject_ref: String,
    /// Provider order rows.
    pub provider_order_rows: Vec<ProviderOrderRow>,
    /// Chosen provider id. `None` when the outcome is unavailable.
    #[serde(default)]
    pub chosen_provider_id: Option<String>,
    /// Confidence outcome class.
    pub confidence_outcome_class: ConfidenceOutcomeClass,
    /// Negotiated scope claim.
    pub negotiated_scope_claim_class: ScopeClaimClass,
    /// Negotiated completeness.
    pub negotiated_completeness_class: CompletenessClass,
    /// Negotiated freshness.
    pub negotiated_freshness_class: FreshnessClass,
    /// Disagreement block.
    pub disagreement_block: DisagreementBlock,
    /// Downgraded promise block.
    pub downgraded_promise_block: DowngradedPromiseBlock,
    /// Fallback label class.
    pub fallback_label_class: FallbackLabelClass,
    /// Apply gate class.
    pub apply_gate_class: ApplyGateClass,
    /// Consumer routing rows.
    pub consumer_routing_rows: Vec<ConsumerRoutingRow>,
    /// Linked record refs.
    pub linked_record_refs: LinkedRecordRefs,
    /// Policy context.
    pub policy_context: PolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl ArbitrationDecisionRecord {
    /// Returns true when the decision must enforce a non-ready apply gate.
    pub fn requires_apply_gate(&self) -> bool {
        self.apply_gate_class != ApplyGateClass::ReadyToApply
    }
}

/// One checked-in corpus entry: an arbitration decision plus the health rows it references.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderArbitrationCorpusEntry {
    /// Repository-relative fixture ref.
    #[serde(default)]
    pub fixture_ref: String,
    /// Arbitration decision under test.
    pub arbitration_decision: ArbitrationDecisionRecord,
    /// Provider-health rows referenced by the decision.
    pub provider_health_states: Vec<ProviderHealthStateRecord>,
}

/// Checked-in inspector corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderArbitrationCorpus {
    /// Corpus entries.
    pub entries: Vec<ProviderArbitrationCorpusEntry>,
}

/// Aggregate counts for an inspector corpus report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrationDecisionAggregateCounts {
    /// Total row count.
    pub total_rows: u32,
    /// Exact outcome rows.
    pub exact_rows: u32,
    /// Heuristic outcome rows.
    pub heuristic_rows: u32,
    /// Partial outcome rows.
    pub partial_rows: u32,
    /// Stale outcome rows.
    pub stale_rows: u32,
    /// Unavailable outcome rows.
    pub unavailable_rows: u32,
    /// Rows that surfaced a non-empty conflict.
    pub conflict_rows: u32,
    /// Rows whose bundled health states include a quarantined provider.
    pub quarantined_provider_rows: u32,
    /// Rows gated by preview review.
    pub preview_gated_rows: u32,
    /// Rows gated by side-branch review.
    pub side_branch_gated_rows: u32,
}

/// One report row projected from a corpus entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrationDecisionReportRow {
    /// Repository-relative fixture ref.
    pub fixture_ref: String,
    /// Arbitration decision id.
    pub arbitration_decision_id: String,
    /// Lane covered by the decision.
    pub language_action_lane_class: LanguageActionLaneClass,
    /// Confidence outcome class.
    pub confidence_outcome_class: ConfidenceOutcomeClass,
    /// Conflict class.
    pub conflict_class: ConflictClass,
    /// Apply gate class.
    pub apply_gate_class: ApplyGateClass,
    /// Fallback label class.
    pub fallback_label_class: FallbackLabelClass,
    /// Number of provider order rows.
    pub provider_order_count: u32,
    /// Number of bundled health-state rows.
    pub bundled_health_state_count: u32,
    /// Number of consumer routing rows.
    pub consumer_routing_count: u32,
}

impl ArbitrationDecisionReportRow {
    pub(crate) fn from_entry(entry: &ProviderArbitrationCorpusEntry) -> Self {
        let decision = &entry.arbitration_decision;
        Self {
            fixture_ref: entry.fixture_ref.clone(),
            arbitration_decision_id: decision.arbitration_decision_id.clone(),
            language_action_lane_class: decision.language_action_lane_class,
            confidence_outcome_class: decision.confidence_outcome_class,
            conflict_class: decision.disagreement_block.conflict_class,
            apply_gate_class: decision.apply_gate_class,
            fallback_label_class: decision.fallback_label_class,
            provider_order_count: decision.provider_order_rows.len() as u32,
            bundled_health_state_count: entry.provider_health_states.len() as u32,
            consumer_routing_count: decision.consumer_routing_rows.len() as u32,
        }
    }
}

/// Corpus report consumed by release, support, and shiproom evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArbitrationInspectorBetaReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Report id.
    pub report_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Documentation ref.
    pub doc_ref: String,
    /// Provider-health-state schema ref.
    pub provider_health_state_schema_ref: String,
    /// Arbitration-decision schema ref.
    pub arbitration_decision_schema_ref: String,
    /// True when raw provider payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private source material is excluded.
    pub raw_private_material_excluded: bool,
    /// Aggregate counts.
    pub aggregate_counts: ArbitrationDecisionAggregateCounts,
    /// Report rows.
    pub rows: Vec<ArbitrationDecisionReportRow>,
}

impl ArbitrationInspectorBetaReport {
    /// Returns true when the report is safe to include in support evidence.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && !self.rows.is_empty()
            && self.aggregate_counts.total_rows == self.rows.len() as u32
    }
}
