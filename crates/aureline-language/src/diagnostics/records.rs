use serde::{Deserialize, Serialize};

use crate::lsp_router::{
    CompletenessClass, FaultDomainId, FreshnessClass as RouterFreshnessClass, HealthState,
    LocalityClass, ProviderKind, ProviderStackRow, RedactionClass, RouterDecisionRecord,
    ScopeClaimClass, ScopeLimitClass, SupportClass,
};

/// Integer schema version for diagnostic bus payloads.
pub type DiagnosticBusSchemaVersion = u32;

/// Schema version used by diagnostic bus records and projections.
pub const DIAGNOSTIC_BUS_SCHEMA_VERSION: DiagnosticBusSchemaVersion = 1;

/// Source family that produced or preserved a diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSourceFamily {
    /// Parser, encoding, Unicode, generated-file, or structural editor guard.
    EditorStructural,
    /// Compiler, build adapter, or structured build-task output.
    CompilerOrBuild,
    /// LSP or supervised language-server diagnostic.
    LanguageServer,
    /// Linter, formatter, style, or organize-imports rule.
    LinterFormatterStyle,
    /// Framework pack, schema analyzer, or convention analyzer.
    FrameworkOrSchemaAnalyzer,
    /// Runtime, test, debug, notebook, or observed execution finding.
    RuntimeTestOrDebug,
    /// Imported scanner, SARIF, review packet, or provider snapshot finding.
    ScannerImport,
    /// Policy, trust, compliance, license, or security finding.
    PolicyTrustOrSecurity,
    /// Project graph or cached semantic graph diagnostic.
    ProjectGraph,
    /// Heuristic parser, problem matcher, or fallback-only finding.
    Heuristic,
}

impl DiagnosticSourceFamily {
    /// Projects a language-router provider kind into the diagnostic source family.
    pub const fn from_provider_kind(provider_kind: ProviderKind) -> Self {
        match provider_kind {
            ProviderKind::SyntaxParser => Self::EditorStructural,
            ProviderKind::LanguageServer => Self::LanguageServer,
            ProviderKind::DebugAdapter | ProviderKind::TestAdapter => Self::RuntimeTestOrDebug,
            ProviderKind::FormatterAdapter | ProviderKind::LinterAdapter => {
                Self::LinterFormatterStyle
            }
            ProviderKind::BuildAdapter => Self::CompilerOrBuild,
            ProviderKind::FrameworkPack
            | ProviderKind::NativeAnalyzer
            | ProviderKind::GeneratedSourceBridge => Self::FrameworkOrSchemaAnalyzer,
            ProviderKind::ProjectGraph => Self::ProjectGraph,
            ProviderKind::AiAssist => Self::Heuristic,
        }
    }
}

/// Plane of evidence behind a diagnostic finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticEvidencePlaneClass {
    /// Derived from source, syntax, schema, or semantic analysis.
    StaticAnalysis,
    /// Derived from compile, build, or structured task execution.
    BuildTimeExecution,
    /// Derived from a run, test, debug session, notebook, or live process.
    RuntimeOrTestExecution,
    /// Derived from policy, trust, security, compliance, or governed review logic.
    PolicyOrTrustEvaluation,
    /// Derived from imported evidence whose producer session is not current-local.
    ImportedSnapshotEvidence,
    /// Derived from heuristic parsing or correlation.
    HeuristicFallback,
}

/// Origin of the diagnostic evidence copy currently held by the bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticOriginClass {
    /// Produced against the current local workspace session.
    LiveLocalSession,
    /// Produced against the current workspace-remote target session.
    LiveRemoteSession,
    /// Produced live by a managed or service-backed provider.
    ManagedProviderLive,
    /// Imported from SARIF-like, release, review, support, or scanner evidence.
    ImportedSnapshot,
    /// Replayed from preserved support evidence rather than rerun live.
    ReplayedSupportBundle,
    /// Restored from a local cache without fresh producer confirmation.
    LocalCache,
}

/// Normalized diagnostic severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverityClass {
    /// Blocking error.
    Error,
    /// Warning that does not necessarily block execution.
    Warning,
    /// Notice-level diagnostic.
    Notice,
    /// Hint-level diagnostic.
    Hint,
}

/// Freshness state for a diagnostic row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFreshnessClass {
    /// Current for the admitted epoch and scope.
    Current,
    /// Recently observed and still inside the accepted freshness window.
    Recent,
    /// Cached result still warm enough to inspect with a cue.
    WarmCached,
    /// Cached result below ideal posture and requiring downgrade disclosure.
    DegradedCached,
    /// Belongs to an older epoch or target and must render as stale.
    Stale,
    /// Superseded by newer evidence but preserved for lineage.
    Superseded,
    /// Imported snapshot evidence, not a current live local run.
    ImportedSnapshot,
    /// Freshness could not be proven.
    Unverified,
}

impl DiagnosticFreshnessClass {
    /// Projects router freshness into diagnostic freshness.
    pub const fn from_router_freshness(freshness_class: RouterFreshnessClass) -> Self {
        match freshness_class {
            RouterFreshnessClass::AuthoritativeLive => Self::Current,
            RouterFreshnessClass::WarmCached => Self::WarmCached,
            RouterFreshnessClass::DegradedCached => Self::DegradedCached,
            RouterFreshnessClass::Stale => Self::Stale,
            RouterFreshnessClass::Unverified => Self::Unverified,
        }
    }

    /// Returns true when the row is cache-backed.
    pub const fn is_cached(self) -> bool {
        matches!(self, Self::WarmCached | Self::DegradedCached)
    }

    /// Returns true when the row is imported rather than live local truth.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedSnapshot)
    }

    /// Returns true when the row requires stale or freshness disclosure.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Current)
    }

    const fn rank(self) -> u8 {
        match self {
            Self::Current => 0,
            Self::Recent => 1,
            Self::WarmCached => 2,
            Self::ImportedSnapshot => 3,
            Self::DegradedCached => 4,
            Self::Stale => 5,
            Self::Superseded => 6,
            Self::Unverified => 7,
        }
    }
}

/// Remap state for the current diagnostic anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticAnchorRemapStateClass {
    /// Current range is exact.
    Exact,
    /// Current range was remapped from surrounding context.
    Contextual,
    /// Current range belongs to a stale epoch.
    Stale,
    /// No current range can be shown.
    Unmapped,
    /// Imported static location has not been locally revalidated.
    ImportedStatic,
    /// No editor anchor applies.
    NotApplicable,
}

impl DiagnosticAnchorRemapStateClass {
    /// Returns true when an inline editor marker may cite this anchor.
    pub const fn allows_inline_projection(self) -> bool {
        matches!(
            self,
            Self::Exact | Self::Contextual | Self::Stale | Self::ImportedStatic
        )
    }

    /// Returns true when a non-exact cue must be visible.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Exact | Self::NotApplicable)
    }
}

/// Surface consuming diagnostic bus state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSurfaceClass {
    /// Editor inline markers and hover-linked marker metadata.
    EditorInline,
    /// Search, symbol, and result-facet surfaces.
    SearchIndex,
    /// Support or diagnostics export packet projection.
    SupportExport,
    /// CLI or headless JSON projection.
    CliJson,
}

impl DiagnosticSurfaceClass {
    /// Returns the stable schema token for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorInline => "editor_inline",
            Self::SearchIndex => "search_index",
            Self::SupportExport => "support_export",
            Self::CliJson => "cli_json",
        }
    }
}

/// Evidence role attached to a diagnostic envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticEvidenceRoleClass {
    /// Primary source record for the finding.
    PrimarySource,
    /// Router decision that selected or downgraded a provider.
    RouterDecision,
    /// Provider status or health row.
    ProviderStatus,
    /// Build, run, test, or import session evidence.
    ProducerSession,
    /// Anchor remap or coordinate mapping evidence.
    RemapEvidence,
    /// Support replay or export evidence.
    SupportReplay,
}

/// Export-safe evidence reference carried by one diagnostic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticEvidenceRef {
    /// Opaque evidence reference.
    pub evidence_ref: String,
    /// Role this evidence plays for the diagnostic.
    pub evidence_role_class: DiagnosticEvidenceRoleClass,
    /// Export-safe evidence summary.
    pub summary: String,
}

/// Source descriptor for one diagnostic producer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceDescriptor {
    /// Stable source descriptor id.
    pub source_descriptor_id: String,
    /// Normalized diagnostic source family.
    pub source_family: DiagnosticSourceFamily,
    /// Plane of evidence behind the finding.
    pub evidence_plane_class: DiagnosticEvidencePlaneClass,
    /// Origin of the evidence copy currently held by the bus.
    pub origin_class: DiagnosticOriginClass,
    /// Opaque producer or tool reference.
    pub producer_ref: String,
    /// Opaque producer version reference, when known.
    pub producer_version_ref: Option<String>,
    /// Router provider id, when the producer is routed.
    pub provider_id: Option<String>,
    /// Router host identity ref, when the producer is a supervised host.
    pub router_host_ref: Option<String>,
    /// Locality where the evidence was produced or imported.
    pub locality_class: LocalityClass,
    /// Authority posture for this diagnostic source.
    pub support_class: SupportClass,
    /// Export-safe source summary.
    pub summary: String,
}

impl DiagnosticSourceDescriptor {
    /// Returns true when this source represents imported evidence.
    pub fn is_imported(&self) -> bool {
        matches!(self.origin_class, DiagnosticOriginClass::ImportedSnapshot)
            || self.source_family == DiagnosticSourceFamily::ScannerImport
            || self.evidence_plane_class == DiagnosticEvidencePlaneClass::ImportedSnapshotEvidence
    }

    /// Returns true when this source represents local live or local cached evidence.
    pub fn is_local(&self) -> bool {
        matches!(
            self.origin_class,
            DiagnosticOriginClass::LiveLocalSession | DiagnosticOriginClass::LocalCache
        )
    }
}

/// Freshness and epoch data for one diagnostic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticFreshness {
    /// Freshness class for the diagnostic.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Timestamp or opaque clock ref for when evidence was observed.
    pub observed_at: String,
    /// Current or source epoch reference.
    pub epoch_ref: Option<String>,
    /// Invalidation or stale reason reference, when known.
    pub invalidation_ref: Option<String>,
    /// Export-safe freshness summary.
    pub summary: String,
}

impl DiagnosticFreshness {
    /// Returns true when this freshness row is cache-backed.
    pub const fn is_cached(&self) -> bool {
        self.freshness_class.is_cached()
    }

    /// Returns true when stale or non-current state must be disclosed.
    pub const fn requires_disclosure(&self) -> bool {
        self.freshness_class.requires_disclosure()
    }
}

/// Scope and partiality labels for one diagnostic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticScope {
    /// Scope claimed by the diagnostic producer.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness for the claimed scope.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits that explain partiality.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Target, workset, root, run, or import scope reference.
    pub target_ref: String,
    /// Execution context anchoring target and toolchain identity.
    pub execution_context_id: String,
    /// Export-safe scope summary.
    pub summary: String,
}

impl DiagnosticScope {
    /// Returns true when the diagnostic is partial for its claimed scope.
    pub const fn is_partial(&self) -> bool {
        matches!(
            self.completeness_class,
            CompletenessClass::PartialForClaimedScope
                | CompletenessClass::UnavailableForClaimedScope
        )
    }

    /// Returns true when partial scope must be disclosed.
    pub fn requires_disclosure(&self) -> bool {
        self.is_partial() || !self.scope_limit_classes.is_empty()
    }
}

/// Current anchor projection for one diagnostic row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticAnchor {
    /// Stable anchor family id that survives remaps when possible.
    pub anchor_family_id: String,
    /// Current anchor reference, when one can safely be shown.
    pub current_anchor_ref: Option<String>,
    /// Opaque path or structured object reference, when known.
    pub path_ref: Option<String>,
    /// Current anchor remap state.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Export-safe anchor summary.
    pub summary: String,
}

impl DiagnosticAnchor {
    /// Returns true when editor-inline consumers may cite this anchor.
    pub fn allows_inline_projection(&self) -> bool {
        self.current_anchor_ref.is_some() && self.remap_state_class.allows_inline_projection()
    }

    /// Returns true when the anchor must render with a non-exact cue.
    pub const fn requires_disclosure(&self) -> bool {
        self.remap_state_class.requires_disclosure()
    }
}

/// Normalized diagnostic row carried by the bus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticEnvelope {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_bus_schema_version: DiagnosticBusSchemaVersion,
    /// Stable diagnostic id.
    pub diagnostic_id: String,
    /// Diagnostic collection id.
    pub collection_id: String,
    /// Workspace id covered by this diagnostic.
    pub workspace_id: String,
    /// Source and provenance descriptor.
    pub source: DiagnosticSourceDescriptor,
    /// Normalized severity.
    pub severity_class: DiagnosticSeverityClass,
    /// Opaque rule id reference, when available.
    pub rule_id_ref: Option<String>,
    /// Opaque category reference, when available.
    pub category_ref: Option<String>,
    /// Freshness and epoch state.
    pub freshness: DiagnosticFreshness,
    /// Scope and partiality state.
    pub scope: DiagnosticScope,
    /// Current anchor state.
    pub anchor: DiagnosticAnchor,
    /// Evidence refs that explain the row without raw payloads.
    pub evidence_refs: Vec<DiagnosticEvidenceRef>,
    /// Provider ids linked to this row.
    pub provider_status_refs: Vec<String>,
    /// Router decision ref linked to this row, when available.
    pub router_decision_ref: Option<String>,
    /// Redaction posture for export.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticEnvelope {
    /// Stable record-kind tag for diagnostic envelopes.
    pub const RECORD_KIND: &'static str = "diagnostic_bus_envelope_record";

    /// Returns true when the diagnostic came from imported evidence.
    pub fn is_imported(&self) -> bool {
        self.source.is_imported() || self.freshness.freshness_class.is_imported()
    }

    /// Returns true when the diagnostic is live or cached local evidence.
    pub fn is_local(&self) -> bool {
        self.source.is_local()
    }

    /// Returns true when the diagnostic is cache-backed.
    pub fn is_cached(&self) -> bool {
        self.freshness.is_cached()
            || matches!(self.source.origin_class, DiagnosticOriginClass::LocalCache)
    }

    /// Returns true when scope partiality must be visible.
    pub fn is_partial(&self) -> bool {
        self.scope.is_partial()
    }

    /// Returns true when the diagnostic is stale, superseded, or unverified.
    pub const fn is_stale_or_unverified(&self) -> bool {
        matches!(
            self.freshness.freshness_class,
            DiagnosticFreshnessClass::Stale
                | DiagnosticFreshnessClass::Superseded
                | DiagnosticFreshnessClass::Unverified
        )
    }

    /// Returns true when downstream surfaces must disclose degraded state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.is_imported()
            || self.is_cached()
            || self.is_partial()
            || self.freshness.requires_disclosure()
            || self.scope.requires_disclosure()
            || self.anchor.requires_disclosure()
    }

    /// Returns true when an editor inline projection may show this row.
    pub fn allows_editor_inline_projection(&self) -> bool {
        self.anchor.allows_inline_projection()
            && self.scope.completeness_class != CompletenessClass::UnavailableForClaimedScope
    }
}

/// Provider availability row consumed by the diagnostic bus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticProviderAvailabilityRow {
    /// Provider id.
    pub provider_id: String,
    /// Plain-language provider label.
    pub provider_display_label: String,
    /// Source family this provider can produce for diagnostics.
    pub source_family: DiagnosticSourceFamily,
    /// Language-router provider kind.
    pub provider_kind: ProviderKind,
    /// Provider support posture.
    pub support_class: SupportClass,
    /// Provider health.
    pub health_state: HealthState,
    /// Provider freshness.
    pub freshness_class: RouterFreshnessClass,
    /// Scope claimed by the provider.
    pub scope_claim_class: ScopeClaimClass,
    /// Completeness for the provider's claimed scope.
    pub completeness_class: CompletenessClass,
    /// Concrete scope limits.
    pub scope_limit_classes: Vec<ScopeLimitClass>,
    /// Provider locality.
    pub locality_class: LocalityClass,
    /// Fault domain owning restart accounting.
    pub fault_domain_id: FaultDomainId,
    /// Restart strikes counted by the supervisor.
    pub restart_strike_count: u32,
    /// Quarantine reference, when active.
    pub quarantine_ref: Option<String>,
    /// Router decision that produced this availability row, when any.
    pub router_decision_ref: Option<String>,
    /// Export-safe provider availability summary.
    pub summary: String,
}

impl DiagnosticProviderAvailabilityRow {
    /// Builds a diagnostic provider row from a router provider-stack row.
    pub fn from_provider_stack_row(
        row: &ProviderStackRow,
        decision: &RouterDecisionRecord,
    ) -> Self {
        Self {
            provider_id: row.provider_id.clone(),
            provider_display_label: row.provider_display_label.clone(),
            source_family: DiagnosticSourceFamily::from_provider_kind(row.provider_kind),
            provider_kind: row.provider_kind,
            support_class: row.support_class,
            health_state: row.health_state,
            freshness_class: row.freshness_class,
            scope_claim_class: decision.request_context.requested_scope_claim_class,
            completeness_class: completeness_from_health(row.health_state),
            scope_limit_classes: Vec::new(),
            locality_class: row.locality_class,
            fault_domain_id: row.fault_domain_id,
            restart_strike_count: row.restart_strike_count,
            quarantine_ref: row.quarantine_ref.clone(),
            router_decision_ref: Some(decision.router_decision_id.clone()),
            summary: row.summary.clone(),
        }
    }

    /// Returns true when this provider cannot provide full current diagnostics.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.health_state.requires_disclosure()
            || self.freshness_class != RouterFreshnessClass::AuthoritativeLive
            || self.completeness_class != CompletenessClass::CompleteForClaimedScope
            || self.quarantine_ref.is_some()
            || !self.scope_limit_classes.is_empty()
    }

    /// Returns true when this provider is unavailable for the claimed scope.
    pub const fn is_unavailable(&self) -> bool {
        matches!(
            self.health_state,
            HealthState::Unavailable
                | HealthState::CrashLoopQuarantined
                | HealthState::PolicyBlocked
                | HealthState::CapabilityMissing
        ) || matches!(
            self.completeness_class,
            CompletenessClass::UnavailableForClaimedScope
        )
    }
}

/// Aggregate counts used by editor, search, and support projections.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DiagnosticBusAggregateCounts {
    /// Total diagnostics in the snapshot.
    pub total_count: usize,
    /// Error diagnostics.
    pub error_count: usize,
    /// Warning diagnostics.
    pub warning_count: usize,
    /// Notice diagnostics.
    pub notice_count: usize,
    /// Hint diagnostics.
    pub hint_count: usize,
    /// Diagnostics with local live or cached origin.
    pub local_count: usize,
    /// Diagnostics with imported source or freshness.
    pub imported_count: usize,
    /// Diagnostics backed by cache.
    pub cached_count: usize,
    /// Diagnostics whose claimed scope is partial or unavailable.
    pub partial_count: usize,
    /// Diagnostics that are stale, superseded, or unverified.
    pub stale_or_unverified_count: usize,
    /// Providers that are unavailable or quarantined.
    pub unavailable_provider_count: usize,
}

impl DiagnosticBusAggregateCounts {
    /// Builds counts from normalized diagnostics and provider availability rows.
    pub fn from_rows(
        diagnostics: &[DiagnosticEnvelope],
        providers: &[DiagnosticProviderAvailabilityRow],
    ) -> Self {
        let mut counts = Self {
            total_count: diagnostics.len(),
            unavailable_provider_count: providers
                .iter()
                .filter(|provider| provider.is_unavailable())
                .count(),
            ..Self::default()
        };

        for diagnostic in diagnostics {
            match diagnostic.severity_class {
                DiagnosticSeverityClass::Error => counts.error_count += 1,
                DiagnosticSeverityClass::Warning => counts.warning_count += 1,
                DiagnosticSeverityClass::Notice => counts.notice_count += 1,
                DiagnosticSeverityClass::Hint => counts.hint_count += 1,
            }
            if diagnostic.is_local() {
                counts.local_count += 1;
            }
            if diagnostic.is_imported() {
                counts.imported_count += 1;
            }
            if diagnostic.is_cached() {
                counts.cached_count += 1;
            }
            if diagnostic.is_partial() {
                counts.partial_count += 1;
            }
            if diagnostic.is_stale_or_unverified() {
                counts.stale_or_unverified_count += 1;
            }
        }

        counts
    }
}

/// Snapshot of all diagnostics and provider states in one collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticBusSnapshot {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_bus_schema_version: DiagnosticBusSchemaVersion,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Workspace id covered by the snapshot.
    pub workspace_id: String,
    /// Diagnostic collection id.
    pub collection_id: String,
    /// Normalized diagnostic rows.
    pub diagnostics: Vec<DiagnosticEnvelope>,
    /// Provider availability rows visible to diagnostics consumers.
    pub provider_availability_rows: Vec<DiagnosticProviderAvailabilityRow>,
    /// Aggregate counts for compact surfaces.
    pub aggregate_counts: DiagnosticBusAggregateCounts,
    /// Redaction posture for the snapshot.
    pub redaction_class: RedactionClass,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe snapshot summary.
    pub export_safe_summary: String,
}

impl DiagnosticBusSnapshot {
    /// Stable record-kind tag for diagnostic bus snapshots.
    pub const RECORD_KIND: &'static str = "diagnostic_bus_snapshot_record";

    /// Returns true when any diagnostic or provider row needs a degraded label.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.diagnostics
            .iter()
            .any(DiagnosticEnvelope::requires_degraded_disclosure)
            || self
                .provider_availability_rows
                .iter()
                .any(DiagnosticProviderAvailabilityRow::requires_degraded_disclosure)
    }

    /// Builds a surface-specific projection from this snapshot.
    pub fn surface_projection(
        &self,
        surface_class: DiagnosticSurfaceClass,
        captured_at: impl Into<String>,
    ) -> DiagnosticSurfaceProjection {
        let included_diagnostic_ids = self
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.diagnostic_id.clone())
            .collect::<Vec<_>>();
        let inline_diagnostic_ids = self
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.allows_editor_inline_projection())
            .map(|diagnostic| diagnostic.diagnostic_id.clone())
            .collect::<Vec<_>>();
        let withheld_diagnostic_ids = self
            .diagnostics
            .iter()
            .filter(|diagnostic| !diagnostic.allows_editor_inline_projection())
            .map(|diagnostic| diagnostic.diagnostic_id.clone())
            .collect::<Vec<_>>();
        let provider_availability_refs = self
            .provider_availability_rows
            .iter()
            .map(|provider| provider.provider_id.clone())
            .collect::<Vec<_>>();
        let freshness_summary_class = summarize_freshness(&self.diagnostics);
        let completeness_summary_class = summarize_completeness(&self.diagnostics);

        let visible_inline_count = match surface_class {
            DiagnosticSurfaceClass::EditorInline => inline_diagnostic_ids.len(),
            DiagnosticSurfaceClass::SearchIndex
            | DiagnosticSurfaceClass::SupportExport
            | DiagnosticSurfaceClass::CliJson => included_diagnostic_ids.len(),
        };

        DiagnosticSurfaceProjection {
            record_kind: DiagnosticSurfaceProjection::RECORD_KIND.into(),
            diagnostic_bus_schema_version: DIAGNOSTIC_BUS_SCHEMA_VERSION,
            projection_id: format!(
                "diagnostic_bus_projection:{}:{}",
                surface_class.as_str(),
                sanitize_id(&self.snapshot_id)
            ),
            snapshot_id: self.snapshot_id.clone(),
            surface_class,
            included_diagnostic_ids,
            inline_diagnostic_ids,
            withheld_diagnostic_ids,
            provider_availability_refs,
            disclosure_required: self.requires_degraded_disclosure(),
            freshness_summary_class,
            completeness_summary_class,
            visible_count: visible_inline_count,
            captured_at: captured_at.into(),
            export_safe_summary: format!(
                "{} diagnostic rows projected for {} with freshness {:?} and completeness {:?}.",
                visible_inline_count,
                surface_class.as_str(),
                freshness_summary_class,
                completeness_summary_class
            ),
        }
    }
}

/// Surface-specific view over a diagnostic bus snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_bus_schema_version: DiagnosticBusSchemaVersion,
    /// Stable projection id.
    pub projection_id: String,
    /// Source snapshot id.
    pub snapshot_id: String,
    /// Surface consuming the projection.
    pub surface_class: DiagnosticSurfaceClass,
    /// Diagnostic ids included in the projection.
    pub included_diagnostic_ids: Vec<String>,
    /// Diagnostic ids safe to show as inline editor markers.
    pub inline_diagnostic_ids: Vec<String>,
    /// Diagnostic ids withheld from inline marker rendering.
    pub withheld_diagnostic_ids: Vec<String>,
    /// Provider availability ids attached to the projection.
    pub provider_availability_refs: Vec<String>,
    /// Whether the surface must show source, freshness, or partiality disclosure.
    pub disclosure_required: bool,
    /// Worst visible freshness class in the projection.
    pub freshness_summary_class: DiagnosticFreshnessClass,
    /// Worst visible completeness class in the projection.
    pub completeness_summary_class: CompletenessClass,
    /// Count visible in the surface's primary row set.
    pub visible_count: usize,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe projection summary.
    pub export_safe_summary: String,
}

impl DiagnosticSurfaceProjection {
    /// Stable record-kind tag for surface projections.
    pub const RECORD_KIND: &'static str = "diagnostic_surface_projection_record";
}

pub(crate) fn summarize_freshness(diagnostics: &[DiagnosticEnvelope]) -> DiagnosticFreshnessClass {
    diagnostics
        .iter()
        .map(|diagnostic| diagnostic.freshness.freshness_class)
        .max_by_key(|freshness_class| freshness_class.rank())
        .unwrap_or(DiagnosticFreshnessClass::Current)
}

pub(crate) fn summarize_completeness(diagnostics: &[DiagnosticEnvelope]) -> CompletenessClass {
    if diagnostics.iter().any(|diagnostic| {
        diagnostic.scope.completeness_class == CompletenessClass::UnavailableForClaimedScope
    }) {
        CompletenessClass::UnavailableForClaimedScope
    } else if diagnostics.iter().any(|diagnostic| {
        diagnostic.scope.completeness_class == CompletenessClass::PartialForClaimedScope
    }) {
        CompletenessClass::PartialForClaimedScope
    } else {
        CompletenessClass::CompleteForClaimedScope
    }
}

fn completeness_from_health(health_state: HealthState) -> CompletenessClass {
    match health_state {
        HealthState::Ready => CompletenessClass::CompleteForClaimedScope,
        HealthState::Warming | HealthState::Degraded | HealthState::CachedOnly => {
            CompletenessClass::PartialForClaimedScope
        }
        HealthState::PolicyBlocked
        | HealthState::CapabilityMissing
        | HealthState::CrashLoopQuarantined
        | HealthState::Unavailable => CompletenessClass::UnavailableForClaimedScope,
    }
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}
