use serde::{Deserialize, Serialize};

/// Integer schema version for [`RouterDecisionRecord`] payloads.
pub type RouterDecisionSchemaVersion = u32;

/// Schema version used by router decisions and provider status rows.
pub const ROUTER_DECISION_SCHEMA_VERSION: RouterDecisionSchemaVersion = 1;

/// Protected surface asking for language-derived truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Go-to-definition or similar target lookup.
    Definition,
    /// Find-reference lookup.
    Reference,
    /// Hover documentation or symbol facts.
    Hover,
    /// Rename request or rename preview.
    Rename,
    /// Completion list or inline completion surface.
    Completion,
    /// Formatting request.
    Formatting,
    /// Code-action or quick-fix request.
    CodeAction,
    /// Diagnostics refresh or projection.
    Diagnostic,
    /// Signature-help request.
    SignatureHelp,
    /// Inline-hint projection.
    InlineHint,
    /// Test discovery request.
    TestDiscovery,
    /// Test run request.
    TestRun,
    /// Debug launch request.
    DebugLaunch,
    /// Debug attach request.
    DebugAttach,
    /// Debug session control request.
    DebugSessionControl,
    /// Build-target discovery request.
    BuildTargetDiscovery,
    /// Build diagnostics request.
    BuildDiagnostics,
    /// Framework-aware navigation request.
    FrameworkNavigation,
    /// Framework run-scaffold request.
    FrameworkRunScaffold,
    /// Notebook context request.
    NotebookContext,
    /// AI-assist context request.
    AiAssistContext,
}

impl SurfaceClass {
    /// Returns the stable schema token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Reference => "reference",
            Self::Hover => "hover",
            Self::Rename => "rename",
            Self::Completion => "completion",
            Self::Formatting => "formatting",
            Self::CodeAction => "code_action",
            Self::Diagnostic => "diagnostic",
            Self::SignatureHelp => "signature_help",
            Self::InlineHint => "inline_hint",
            Self::TestDiscovery => "test_discovery",
            Self::TestRun => "test_run",
            Self::DebugLaunch => "debug_launch",
            Self::DebugAttach => "debug_attach",
            Self::DebugSessionControl => "debug_session_control",
            Self::BuildTargetDiscovery => "build_target_discovery",
            Self::BuildDiagnostics => "build_diagnostics",
            Self::FrameworkNavigation => "framework_navigation",
            Self::FrameworkRunScaffold => "framework_run_scaffold",
            Self::NotebookContext => "notebook_context",
            Self::AiAssistContext => "ai_assist_context",
        }
    }
}

/// Capability being routed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityClass {
    /// Definition lookup capability.
    Definition,
    /// Reference lookup capability.
    Reference,
    /// Hover capability.
    Hover,
    /// Rename capability.
    Rename,
    /// Completion capability.
    Completion,
    /// Formatting capability.
    Formatting,
    /// Code-action capability.
    CodeAction,
    /// Diagnostics capability.
    Diagnostics,
    /// Signature-help capability.
    SignatureHelp,
    /// Inline-hint capability.
    InlineHint,
    /// Test discovery capability.
    TestDiscovery,
    /// Test run capability.
    TestRun,
    /// Debug launch capability.
    DebugLaunch,
    /// Debug attach capability.
    DebugAttach,
    /// Debug session control capability.
    DebugSessionControl,
    /// Build target discovery capability.
    BuildTargetDiscovery,
    /// Build diagnostics capability.
    BuildDiagnostics,
    /// Framework navigation capability.
    FrameworkNavigation,
    /// Framework run-scaffold capability.
    FrameworkRunScaffold,
    /// Coordinate translation capability.
    CoordinateTranslation,
    /// Provenance explanation capability.
    ProvenanceExplanation,
}

impl CapabilityClass {
    /// Returns the stable schema token for this capability.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Definition => "definition",
            Self::Reference => "reference",
            Self::Hover => "hover",
            Self::Rename => "rename",
            Self::Completion => "completion",
            Self::Formatting => "formatting",
            Self::CodeAction => "code_action",
            Self::Diagnostics => "diagnostics",
            Self::SignatureHelp => "signature_help",
            Self::InlineHint => "inline_hint",
            Self::TestDiscovery => "test_discovery",
            Self::TestRun => "test_run",
            Self::DebugLaunch => "debug_launch",
            Self::DebugAttach => "debug_attach",
            Self::DebugSessionControl => "debug_session_control",
            Self::BuildTargetDiscovery => "build_target_discovery",
            Self::BuildDiagnostics => "build_diagnostics",
            Self::FrameworkNavigation => "framework_navigation",
            Self::FrameworkRunScaffold => "framework_run_scaffold",
            Self::CoordinateTranslation => "coordinate_translation",
            Self::ProvenanceExplanation => "provenance_explanation",
        }
    }

    /// Returns true when a syntax fallback can honestly serve a narrow result.
    pub const fn syntax_fallback_allowed(self) -> bool {
        matches!(
            self,
            Self::Definition
                | Self::Reference
                | Self::Hover
                | Self::Rename
                | Self::Completion
                | Self::Diagnostics
                | Self::CodeAction
                | Self::ProvenanceExplanation
        )
    }
}

/// Authority floor requested by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedAuthorityFloorClass {
    /// Only an authoritative provider may satisfy the request.
    AuthoritativeRequired,
    /// An authoritative provider is preferred, with labeled fallback allowed.
    AuthoritativePreferred,
    /// Advisory providers may satisfy the request.
    AdvisoryAllowed,
    /// Inspect-only explanations may satisfy the request.
    InspectOnlyAllowed,
}

impl RequestedAuthorityFloorClass {
    /// Returns true when lower-authority fallback may be selected.
    pub const fn allows_fallback(self) -> bool {
        !matches!(self, Self::AuthoritativeRequired)
    }
}

/// Scope the provider or result claims to cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClaimClass {
    /// Only the active file or document.
    SingleFile,
    /// Only the addressed notebook cell.
    NotebookCell,
    /// Only the currently loaded slice.
    LoadedSlice,
    /// The active workset.
    ActiveWorkset,
    /// The entire admitted workspace.
    WholeWorkspace,
    /// A target graph.
    TargetGraph,
    /// A test tree.
    TestTree,
    /// A debug session.
    DebugSession,
    /// No scope is honestly available.
    Unavailable,
}

/// Locality where provider truth ran or originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    /// Provider ran in the shell process.
    LocalInProcess,
    /// Provider ran in a local helper process.
    LocalSidecar,
    /// Provider ran in a workspace remote agent.
    WorkspaceRemoteAgent,
    /// Provider truth came from a managed service.
    ManagedService,
    /// Provider truth came from an imported snapshot.
    ImportedSnapshot,
}

/// High-level posture for local and remote provider mixing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneClass {
    /// All selected providers are local.
    LocalOnly,
    /// All selected providers are remote.
    RemoteOnly,
    /// Selected and fallback providers mix local and remote lanes.
    Hybrid,
}

/// Placement preference requested by a caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementPreferenceClass {
    /// Prefer local providers.
    PreferLocal,
    /// Prefer workspace-remote providers.
    PreferWorkspaceRemote,
    /// Prefer managed-service providers.
    PreferManagedService,
    /// Match the subject's location.
    MatchSubjectLocation,
    /// No placement preference.
    NoPreference,
}

/// Coordinate-translation requirement for a routed request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoordinateTranslationRequirementClass {
    /// No coordinate projection is required.
    NotRequired,
    /// Translation is required before the request leaves the editor.
    RequiredBeforeRequest,
    /// Translation is required before results render.
    RequiredBeforeResult,
    /// Translation is required before mutation.
    RequiredForMutation,
    /// Routing is blocked until a mapping exists.
    BlockedUntilMappingAvailable,
}

/// Provider implementation family used by router stack rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderKind {
    /// Syntax parser or local text provider.
    SyntaxParser,
    /// Language server provider.
    LanguageServer,
    /// Debug adapter provider.
    DebugAdapter,
    /// Formatter adapter provider.
    FormatterAdapter,
    /// Linter adapter provider.
    LinterAdapter,
    /// Test adapter provider.
    TestAdapter,
    /// Build adapter provider.
    BuildAdapter,
    /// Framework pack provider.
    FrameworkPack,
    /// Native analyzer provider.
    NativeAnalyzer,
    /// Project graph provider.
    ProjectGraph,
    /// Generated source bridge provider.
    GeneratedSourceBridge,
    /// AI assist provider.
    AiAssist,
}

/// How much authority a provider has for a capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Provider can serve authoritative truth.
    Authoritative,
    /// Provider can provide advisory information.
    Advisory,
    /// Provider can only serve an explicit fallback.
    FallbackOnly,
    /// Provider can explain but not own the result.
    InspectOnly,
    /// Provider cannot support the capability.
    Unsupported,
}

/// Product-level precedence band.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrecedenceBand {
    /// First-party native provider.
    FirstPartyNative,
    /// Framework overlay provider.
    FrameworkOverlay,
    /// Project graph authority provider.
    ProjectGraphAuthority,
    /// Protocol compatibility provider such as LSP.
    ProtocolCompatibility,
    /// Structured tool adapter provider.
    StructuredToolAdapter,
    /// Imported snapshot provider.
    ImportedSnapshot,
    /// Heuristic fallback provider.
    HeuristicFallback,
    /// Assist-only provider.
    AssistOnly,
}

/// Provider health state surfaced by router and status rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthState {
    /// Provider can serve fresh requests.
    Ready,
    /// Provider is starting or reconnecting.
    Warming,
    /// Provider is reachable with narrowed capability.
    Degraded,
    /// Provider can only reuse cache.
    CachedOnly,
    /// Policy prevents provider use.
    PolicyBlocked,
    /// Provider lacks the requested capability.
    CapabilityMissing,
    /// Provider is quarantined after repeated failure.
    CrashLoopQuarantined,
    /// Provider cannot be reached or used.
    Unavailable,
}

impl HealthState {
    /// Returns true when this state may be selected for fresh primary truth.
    pub const fn is_selectable_primary(self) -> bool {
        matches!(self, Self::Ready)
    }

    /// Returns true when this state must surface as degraded or unavailable.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Ready)
    }
}

/// Freshness label attached to a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Fresh against the current admitted epoch.
    AuthoritativeLive,
    /// Warm cached result inside grace.
    WarmCached,
    /// Cached result below ideal posture.
    DegradedCached,
    /// Stale result.
    Stale,
    /// Freshness could not be proven.
    Unverified,
}

/// Fallback path used or available for a route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackClass {
    /// No fallback applied.
    NoFallback,
    /// Semantic provider fell back to a protocol provider.
    SemanticToProtocol,
    /// Protocol provider fell back to native provider.
    ProtocolToNative,
    /// Protocol provider fell back to text or syntax.
    ProtocolToText,
    /// Structured provider fell back to heuristic.
    StructuredToHeuristic,
    /// Live provider fell back to cache.
    LiveToCached,
    /// Local provider fell back to remote.
    LocalToRemote,
    /// Remote provider fell back to local cache.
    RemoteToLocalCached,
    /// No fallback can serve the capability.
    UnsupportedNoFallback,
}

/// Degraded-state label emitted by route decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedStateClass {
    /// No degraded state.
    None,
    /// Preferred provider was unavailable or reconnecting.
    DegradedProviderUnavailable,
    /// Preferred provider was quarantined after a crash loop.
    DegradedCrashLoopQuarantine,
    /// Result fell back to cache.
    DegradedCachedFallback,
    /// Result fell back to a heuristic.
    DegradedHeuristicFallback,
    /// Scope was narrowed.
    DegradedScopeNarrowed,
    /// Remote lane was unreachable.
    DegradedRemoteUnreachable,
    /// Coordinate mapping was missing.
    DegradedCoordinateMappingMissing,
    /// Policy narrowed the route.
    DegradedPolicyNarrowed,
}

/// Routing resolution mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionMode {
    /// One provider won.
    SingleWinner,
    /// A lower-authority fallback answered.
    OrderedFallback,
    /// Multiple providers coexisted.
    MergedCoexistence,
    /// One provider overlaid another.
    Overlay,
    /// No provider won, but explanation is available.
    InspectOnlyNoWinner,
    /// No provider supports the request.
    Unsupported,
}

/// Fault-domain identifier reused from runtime supervision vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultDomainId {
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

/// Router trust state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouterTrustState {
    /// Workspace trust admits the route.
    Trusted,
    /// Trust policy narrows provider use.
    Restricted,
}

/// Redaction posture for exportable router records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// Provider family for status rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderFamily {
    /// Syntax provider family.
    Syntax,
    /// Project graph provider family.
    ProjectGraph,
    /// Language server provider family.
    LanguageServer,
    /// Framework pack provider family.
    FrameworkPack,
    /// Notebook adapter provider family.
    NotebookAdapter,
    /// Generated-source bridge family.
    GeneratedSourceBridge,
    /// AI assist provider family.
    AiAssist,
}

/// Provider role for status rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderRoleClass {
    /// Current primary semantic lane.
    PrimarySemantic,
    /// Secondary semantic contributor.
    SecondarySemantic,
    /// Framework overlay lane.
    FrameworkOverlay,
    /// Notebook projection lane.
    NotebookProjection,
    /// Generated artifact overlay lane.
    GeneratedOverlay,
    /// Text fallback lane.
    TextFallback,
    /// Assist-only lane.
    AssistOnly,
}

/// Completeness for the provider's claimed scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompletenessClass {
    /// Complete for the claimed scope.
    CompleteForClaimedScope,
    /// Partial for the claimed scope.
    PartialForClaimedScope,
    /// Unavailable for the claimed scope.
    UnavailableForClaimedScope,
}

/// Concrete reason a provider is narrower than the possible scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeLimitClass {
    /// Provider is limited to one file.
    SingleFileOnly,
    /// Provider is limited to the active workset.
    ActiveWorksetOnly,
    /// Unloaded roots were omitted.
    UnloadedRootsOmitted,
    /// Generated overlay only.
    GeneratedOverlayOnly,
    /// Generated candidates were omitted.
    GeneratedCandidatesOmitted,
    /// Notebook cell projection only.
    NotebookCellProjectionOnly,
    /// Cross-cell context is unavailable.
    CrossCellContextUnavailable,
    /// Diff or review slice only.
    DiffOrReviewSliceOnly,
    /// Policy narrowed scope.
    PolicyNarrowed,
    /// Remote shard is unreachable.
    RemoteShardUnreachable,
}

/// Support posture for one surface in a provider status row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceSupportClass {
    /// Provider is authoritative on the surface.
    Authoritative,
    /// Provider is advisory on the surface.
    Advisory,
    /// Provider is fallback-only on the surface.
    FallbackOnly,
    /// Provider is unsupported on the surface.
    Unsupported,
}

/// Epoch role carried by provider status rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochRoleClass {
    /// Workspace-scope epoch.
    WorkspaceScope,
    /// Graph snapshot epoch.
    GraphSnapshot,
    /// Provider-session epoch.
    ProviderSession,
    /// Language project model epoch.
    LanguageProjectModel,
    /// Framework pack snapshot epoch.
    FrameworkPackSnapshot,
    /// Notebook projection epoch.
    NotebookProjection,
    /// Generated-lineage epoch.
    GeneratedLineage,
    /// AI context epoch.
    AiContext,
}

/// Request context block in a router decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouterRequestContext {
    /// Surface requesting language-derived truth.
    pub requested_surface_class: SurfaceClass,
    /// Capability being routed.
    pub requested_capability_class: CapabilityClass,
    /// Authority floor requested by the surface.
    pub requested_authority_floor_class: RequestedAuthorityFloorClass,
    /// Scope requested by the surface.
    pub requested_scope_claim_class: ScopeClaimClass,
    /// Opaque subject reference.
    pub requested_subject_ref: String,
    /// Placement preference for provider selection.
    pub placement_preference_class: PlacementPreferenceClass,
    /// Coordinate translation requirement.
    pub coordinate_translation_requirement_class: CoordinateTranslationRequirementClass,
    /// Policy epoch applied to the decision.
    pub policy_epoch: String,
    /// Trust state applied to the decision.
    pub trust_state: RouterTrustState,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
}

/// Root, scope, target, and toolchain context for a router decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoutingContext {
    /// Workspace id.
    pub workspace_id: String,
    /// Workset id.
    pub workset_id: String,
    /// Workspace root reference.
    pub workspace_root_ref: String,
    /// Subject root reference.
    pub subject_root_ref: String,
    /// Package root reference, when known.
    pub package_root_ref: Option<String>,
    /// Config root reference, when known.
    pub config_root_ref: Option<String>,
    /// Local, remote, or hybrid lane posture.
    pub lane_class: LaneClass,
    /// Export-safe target summary.
    pub target_summary: String,
    /// Export-safe toolchain summary.
    pub toolchain_summary: String,
}

/// One provider row in the ordered router stack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStackRow {
    /// Provider id.
    pub provider_id: String,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Provider kind.
    pub provider_kind: ProviderKind,
    /// Capability represented by the row.
    pub capability_class: CapabilityClass,
    /// Authority support for this capability.
    pub support_class: SupportClass,
    /// Precedence band used by the router.
    pub precedence_band: PrecedenceBand,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Provider health.
    pub health_state: HealthState,
    /// Provider freshness.
    pub freshness_class: FreshnessClass,
    /// Fault domain that owns restart accounting.
    pub fault_domain_id: FaultDomainId,
    /// Restart strikes counted by the supervisor.
    pub restart_strike_count: u32,
    /// Restart budget reference.
    pub restart_budget_ref: String,
    /// Quarantine reference, when active.
    pub quarantine_ref: Option<String>,
    /// Fallback path available or used.
    pub fallback_class: FallbackClass,
    /// Export-safe provider row summary.
    pub summary: String,
}

/// Decision outcome block for a router decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecisionOutcome {
    /// Resolution mode selected by the router.
    pub resolution_mode: ResolutionMode,
    /// Degraded state for the decision.
    pub degraded_state_class: DegradedStateClass,
    /// Fallback path used by the decision.
    pub fallback_class: FallbackClass,
    /// Selected provider id.
    pub selected_provider_id: String,
    /// Export-safe routing reason.
    pub routing_reason: String,
    /// Export-safe fallback summary.
    pub fallback_summary: String,
}

/// Compact report consumed by shell, CLI, and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceReport {
    /// Provider or lane label shown as origin.
    pub origin_label: String,
    /// Degraded state projected to the surface.
    pub degraded_state_class: DegradedStateClass,
    /// User-visible export-safe summary.
    pub user_visible_summary: String,
    /// Export-safe explanation for support packets.
    pub export_safe_explanation: String,
}

/// Router decision record matching the existing language schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouterDecisionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for router decisions.
    pub router_decision_schema_version: RouterDecisionSchemaVersion,
    /// Stable decision id.
    pub router_decision_id: String,
    /// Request context.
    pub request_context: RouterRequestContext,
    /// Root and execution routing context.
    pub routing_context: RoutingContext,
    /// Ordered provider stack.
    pub provider_stack_rows: Vec<ProviderStackRow>,
    /// Decision outcome.
    pub decision_outcome: DecisionOutcome,
    /// Surface report.
    pub surface_report: SurfaceReport,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl RouterDecisionRecord {
    /// Stable record-kind tag for router decision records.
    pub const RECORD_KIND: &'static str = "router_decision_record";

    /// Returns true when the decision must render degraded or fallback state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.decision_outcome.degraded_state_class != DegradedStateClass::None
            || self
                .provider_stack_rows
                .iter()
                .any(|row| row.health_state.requires_disclosure())
    }
}

/// Minimal policy/trust context exported with a provider status row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderPolicyContext {
    /// Policy epoch applied to the row.
    pub policy_epoch: String,
    /// Trust state applied to the row.
    pub trust_state: RouterTrustState,
    /// Execution context id anchoring toolchain identity.
    pub execution_context_id: String,
}

/// Opaque epoch binding relevant to a provider status row.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EpochBinding {
    /// Epoch role.
    pub epoch_role_class: EpochRoleClass,
    /// Epoch reference.
    pub epoch_ref: String,
}

/// Per-surface support posture for a provider.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceSupportClassRow {
    /// Surface supported by the provider.
    pub surface_class: SurfaceClass,
    /// Support posture on that surface.
    pub surface_support_class: SurfaceSupportClass,
    /// Export-safe support summary.
    pub summary: String,
}

/// Provider status row matching the existing language schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderStatusRowRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for provider status rows.
    pub provider_status_row_schema_version: u32,
    /// Stable status-row id.
    pub provider_status_row_id: String,
    /// Provider id.
    pub provider_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Provider role.
    pub provider_role_class: ProviderRoleClass,
    /// Connected pack or server ref.
    pub connected_pack_ref: Option<String>,
    /// Provider health state.
    pub health_state: HealthState,
    /// Health reason summary.
    pub health_reason_summary: Option<String>,
    /// Freshness class.
    pub freshness_class: FreshnessClass,
    /// Scope claim class.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness class.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Host identity reference.
    pub host_identity_ref: Option<String>,
    /// Generated-source notes.
    pub generated_note_classes: Vec<String>,
    /// Notebook notes.
    pub notebook_note_classes: Vec<String>,
    /// Epoch bindings.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Supported surfaces.
    pub supported_surface_rows: Vec<SurfaceSupportClassRow>,
    /// Policy context.
    pub policy_context: ProviderPolicyContext,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl ProviderStatusRowRecord {
    /// Stable record-kind tag for provider status rows.
    pub const RECORD_KIND: &'static str = "provider_status_row_record";

    /// Integer schema version matching `provider_status_row.schema.json`.
    pub const SCHEMA_VERSION: u32 = 1;
}

/// Stable identity for one language-server host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageServerHostIdentity {
    /// Host instance id.
    pub host_instance_id: String,
    /// Provider id served by this host.
    pub provider_id: String,
    /// Workspace that owns the host.
    pub workspace_id: String,
    /// Root that scoped host startup.
    pub root_ref: String,
    /// Language id served by the host.
    pub language_id: String,
    /// Plain-language server label.
    pub server_label: String,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Host locality.
    pub locality_class: LocalityClass,
    /// Fault domain owning restart accounting.
    pub fault_domain_id: FaultDomainId,
    /// Restart budget reference.
    pub restart_budget_ref: String,
}

/// Router-readable status for one supervised language-server host.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LanguageServerHostStatus {
    /// Stable host identity.
    pub identity: LanguageServerHostIdentity,
    /// Current provider health.
    pub health_state: HealthState,
    /// Current provider freshness.
    pub freshness_class: FreshnessClass,
    /// Scope the provider claims to cover.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Supported capability classes.
    pub supported_capability_classes: Vec<CapabilityClass>,
    /// Restart strikes counted in the active window.
    pub restart_strike_count: u32,
    /// Quarantine reference, when active.
    pub quarantine_ref: Option<String>,
    /// Fallback path used when this host cannot win.
    pub fallback_class: FallbackClass,
    /// Export-safe health summary.
    pub health_summary: String,
}

impl LanguageServerHostStatus {
    /// Returns true when the host can honestly serve the requested capability.
    pub fn supports_capability(&self, capability: CapabilityClass) -> bool {
        self.supported_capability_classes.contains(&capability)
    }

    /// Projects this host into a router provider-stack row.
    pub fn provider_stack_row(&self, capability: CapabilityClass) -> ProviderStackRow {
        let capability_supported = self.supports_capability(capability);
        ProviderStackRow {
            provider_id: self.identity.provider_id.clone(),
            provider_display_label: self.identity.server_label.clone(),
            provider_kind: ProviderKind::LanguageServer,
            capability_class: capability,
            support_class: if capability_supported {
                SupportClass::Authoritative
            } else {
                SupportClass::Unsupported
            },
            precedence_band: PrecedenceBand::ProtocolCompatibility,
            locality_class: self.identity.locality_class,
            health_state: if capability_supported {
                self.health_state
            } else {
                HealthState::CapabilityMissing
            },
            freshness_class: self.freshness_class,
            fault_domain_id: self.identity.fault_domain_id,
            restart_strike_count: self.restart_strike_count,
            restart_budget_ref: self.identity.restart_budget_ref.clone(),
            quarantine_ref: self.quarantine_ref.clone(),
            fallback_class: self.fallback_class,
            summary: if capability_supported {
                self.health_summary.clone()
            } else {
                format!(
                    "{} does not advertise {} for this route.",
                    self.identity.server_label,
                    capability.as_str()
                )
            },
        }
    }

    /// Projects this host into an exportable provider status row.
    pub fn provider_status_row(
        &self,
        policy_epoch: impl Into<String>,
        trust_state: RouterTrustState,
        captured_at: impl Into<String>,
    ) -> ProviderStatusRowRecord {
        let captured_at = captured_at.into();
        ProviderStatusRowRecord {
            record_kind: ProviderStatusRowRecord::RECORD_KIND.into(),
            provider_status_row_schema_version: ProviderStatusRowRecord::SCHEMA_VERSION,
            provider_status_row_id: format!("status:{}:{}", self.identity.provider_id, captured_at),
            provider_id: self.identity.provider_id.clone(),
            provider_family: ProviderFamily::LanguageServer,
            provider_display_label: self.identity.server_label.clone(),
            provider_role_class: ProviderRoleClass::PrimarySemantic,
            connected_pack_ref: Some(format!("language-server:{}", self.identity.language_id)),
            health_state: self.health_state,
            health_reason_summary: Some(self.health_summary.clone()),
            freshness_class: self.freshness_class,
            scope_claim_class: self.scope_claim_class,
            completeness_class: self.completeness_class,
            scope_limit_classes: self.scope_limit_classes.clone(),
            locality_class: self.identity.locality_class,
            host_identity_ref: Some(self.identity.host_instance_id.clone()),
            generated_note_classes: Vec::new(),
            notebook_note_classes: Vec::new(),
            current_epoch_bindings: vec![
                EpochBinding {
                    epoch_role_class: EpochRoleClass::WorkspaceScope,
                    epoch_ref: format!("epoch:{}", self.identity.workspace_id),
                },
                EpochBinding {
                    epoch_role_class: EpochRoleClass::ProviderSession,
                    epoch_ref: format!("epoch:{}", self.identity.host_instance_id),
                },
                EpochBinding {
                    epoch_role_class: EpochRoleClass::LanguageProjectModel,
                    epoch_ref: format!("epoch:{}", self.identity.root_ref),
                },
            ],
            supported_surface_rows: surface_rows(&self.supported_capability_classes),
            policy_context: ProviderPolicyContext {
                policy_epoch: policy_epoch.into(),
                trust_state,
                execution_context_id: self.identity.execution_context_id.clone(),
            },
            redaction_class: RedactionClass::MetadataSafeDefault,
            captured_at,
            export_safe_summary: format!(
                "{} is {} for {} in {}.",
                self.identity.server_label,
                health_summary_token(self.health_state),
                self.identity.language_id,
                self.identity.root_ref
            ),
        }
    }
}

fn surface_rows(capabilities: &[CapabilityClass]) -> Vec<SurfaceSupportClassRow> {
    capabilities
        .iter()
        .copied()
        .filter_map(capability_surface)
        .map(|surface| SurfaceSupportClassRow {
            surface_class: surface,
            surface_support_class: SurfaceSupportClass::Authoritative,
            summary: format!(
                "Language server advertises authoritative support for {}.",
                surface.as_str()
            ),
        })
        .collect()
}

fn capability_surface(capability: CapabilityClass) -> Option<SurfaceClass> {
    Some(match capability {
        CapabilityClass::Definition => SurfaceClass::Definition,
        CapabilityClass::Reference => SurfaceClass::Reference,
        CapabilityClass::Hover => SurfaceClass::Hover,
        CapabilityClass::Rename => SurfaceClass::Rename,
        CapabilityClass::Completion => SurfaceClass::Completion,
        CapabilityClass::Formatting => SurfaceClass::Formatting,
        CapabilityClass::CodeAction => SurfaceClass::CodeAction,
        CapabilityClass::Diagnostics => SurfaceClass::Diagnostic,
        CapabilityClass::SignatureHelp => SurfaceClass::SignatureHelp,
        CapabilityClass::InlineHint => SurfaceClass::InlineHint,
        CapabilityClass::TestDiscovery => SurfaceClass::TestDiscovery,
        CapabilityClass::TestRun => SurfaceClass::TestRun,
        CapabilityClass::DebugLaunch => SurfaceClass::DebugLaunch,
        CapabilityClass::DebugAttach => SurfaceClass::DebugAttach,
        CapabilityClass::DebugSessionControl => SurfaceClass::DebugSessionControl,
        CapabilityClass::BuildTargetDiscovery => SurfaceClass::BuildTargetDiscovery,
        CapabilityClass::BuildDiagnostics => SurfaceClass::BuildDiagnostics,
        CapabilityClass::FrameworkNavigation => SurfaceClass::FrameworkNavigation,
        CapabilityClass::FrameworkRunScaffold => SurfaceClass::FrameworkRunScaffold,
        CapabilityClass::CoordinateTranslation | CapabilityClass::ProvenanceExplanation => {
            return None
        }
    })
}

fn health_summary_token(health_state: HealthState) -> &'static str {
    match health_state {
        HealthState::Ready => "ready",
        HealthState::Warming => "warming",
        HealthState::Degraded => "degraded",
        HealthState::CachedOnly => "cached-only",
        HealthState::PolicyBlocked => "policy-blocked",
        HealthState::CapabilityMissing => "capability-missing",
        HealthState::CrashLoopQuarantined => "quarantined",
        HealthState::Unavailable => "unavailable",
    }
}
