use serde::{Deserialize, Serialize};

/// Integer schema version for [`ParseSessionRecord`] payloads.
pub type ParseSessionSchemaVersion = u32;

/// Parser substrate used by a parse session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParserSubstrateClass {
    /// The syntax tree was produced by the Tree-sitter runtime.
    TreeSitter,
    /// The syntax tree came from a Tree-sitter-compatible implementation.
    TreeSitterCompatible,
    /// Only a lightweight bracket scanner was available.
    LightweightBracketScanner,
    /// Only lexical/plain-text structure was available.
    PlainTextLexical,
    /// Parse metadata was imported from a sealed external projection.
    ExternalProjectionImport,
    /// No parser substrate was available.
    NotAvailable,
}

impl ParserSubstrateClass {
    /// Returns the stable schema token for this parser substrate.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TreeSitter => "tree_sitter",
            Self::TreeSitterCompatible => "tree_sitter_compatible",
            Self::LightweightBracketScanner => "lightweight_bracket_scanner",
            Self::PlainTextLexical => "plain_text_lexical",
            Self::ExternalProjectionImport => "external_projection_import",
            Self::NotAvailable => "not_available",
        }
    }
}

/// Source class for an admitted grammar package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrammarSourceClass {
    /// A curated upstream grammar bundled with the product.
    BundledCuratedUpstream,
    /// A first-party grammar maintained directly by Aureline.
    BundledFirstParty,
    /// A grammar supplied by an extension package.
    ExtensionPack,
    /// A workspace-pinned grammar admitted by trust policy.
    WorkspacePinned,
    /// A grammar mirrored by a remote workspace agent.
    RemoteAgentMirror,
    /// No grammar source applies to the degraded path.
    NotApplicable,
}

impl GrammarSourceClass {
    /// Returns the stable schema token for this grammar source.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundledCuratedUpstream => "bundled_curated_upstream",
            Self::BundledFirstParty => "bundled_first_party",
            Self::ExtensionPack => "extension_pack",
            Self::WorkspacePinned => "workspace_pinned",
            Self::RemoteAgentMirror => "remote_agent_mirror",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Outcome of resolving a grammar before parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrammarResolutionStateClass {
    /// The bundled or admitted grammar matched the current request.
    ResolvedCurrent,
    /// A trusted workspace override supplied the grammar.
    ResolvedWithWorkspaceOverride,
    /// A remote mirror supplied the grammar.
    ResolvedRemoteMirror,
    /// No grammar is available for the language id.
    MissingForLanguage,
    /// The grammar ABI is incompatible with the runtime.
    AbiMismatch,
    /// The grammar provenance or signature failed validation.
    SignatureUnverified,
    /// Trust or policy denied the grammar.
    BlockedByTrustOrPolicy,
    /// The plain-text fallback does not use a grammar.
    NotApplicablePlainText,
}

impl GrammarResolutionStateClass {
    /// Returns the stable schema token for this grammar resolution state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedCurrent => "resolved_current",
            Self::ResolvedWithWorkspaceOverride => "resolved_with_workspace_override",
            Self::ResolvedRemoteMirror => "resolved_remote_mirror",
            Self::MissingForLanguage => "missing_for_language",
            Self::AbiMismatch => "abi_mismatch",
            Self::SignatureUnverified => "signature_unverified",
            Self::BlockedByTrustOrPolicy => "blocked_by_trust_or_policy",
            Self::NotApplicablePlainText => "not_applicable_plain_text",
        }
    }
}

/// Host placement class for parser work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParserHostClass {
    /// Bounded parser work in the foreground editor worker.
    EditorProcessForegroundWorker,
    /// Parser work hosted in a local sidecar worker.
    LocalSidecarWorker,
    /// Parser work hosted by a workspace remote agent.
    WorkspaceRemoteAgent,
    /// Parse metadata imported from a sealed snapshot.
    ImportedSnapshot,
    /// No parser work was scheduled.
    NotScheduled,
}

impl ParserHostClass {
    /// Returns the stable schema token for this parser host.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorProcessForegroundWorker => "editor_process_foreground_worker",
            Self::LocalSidecarWorker => "local_sidecar_worker",
            Self::WorkspaceRemoteAgent => "workspace_remote_agent",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::NotScheduled => "not_scheduled",
        }
    }
}

/// Trust state applied to parser host placement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    /// The workspace and parser inputs are trusted for this parse lane.
    Trusted,
    /// Trust policy restricts parser behavior or grammar loading.
    Restricted,
}

impl TrustState {
    /// Returns the stable schema token for this trust state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Trusted => "trusted",
            Self::Restricted => "restricted",
        }
    }
}

/// Request class for a parse session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseRequestClass {
    /// Initial parse for an opened buffer.
    OpenBufferInitial,
    /// Incremental parse after a visible edit.
    VisibleEditIncremental,
    /// Refresh parse after a save snapshot.
    SaveSnapshotRefresh,
    /// Background parse for index or search warming.
    BackgroundIndexRefresh,
    /// Reparse after a language-id switch.
    LanguageSwitchReparse,
    /// Reparse after a grammar package update.
    GrammarUpdateReparse,
    /// Bounded parse during support export replay.
    SupportExportReplay,
}

impl ParseRequestClass {
    /// Returns the stable schema token for this parse request.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenBufferInitial => "open_buffer_initial",
            Self::VisibleEditIncremental => "visible_edit_incremental",
            Self::SaveSnapshotRefresh => "save_snapshot_refresh",
            Self::BackgroundIndexRefresh => "background_index_refresh",
            Self::LanguageSwitchReparse => "language_switch_reparse",
            Self::GrammarUpdateReparse => "grammar_update_reparse",
            Self::SupportExportReplay => "support_export_replay",
        }
    }
}

/// Lifecycle state for one parse request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseLifecycleStateClass {
    /// The parse request has been admitted but not started.
    Queued,
    /// The parser is currently running.
    Running,
    /// The parser yielded because the budget was exhausted.
    YieldedBudgetExhausted,
    /// The parse completed and published its visible state.
    Completed,
    /// The parse failed with a typed reason.
    Failed,
    /// No grammar was available and syntax fell back explicitly.
    DegradedNoGrammar,
    /// Decode recovery prevented full parser projections.
    DegradedDecodeRecovery,
    /// A stale tree remains visible while a new parse is pending.
    StaleAwaitingReparse,
    /// A newer buffer version superseded this parse.
    CancelledSuperseded,
}

impl ParseLifecycleStateClass {
    /// Returns the stable schema token for this lifecycle state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::YieldedBudgetExhausted => "yielded_budget_exhausted",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::DegradedNoGrammar => "degraded_no_grammar",
            Self::DegradedDecodeRecovery => "degraded_decode_recovery",
            Self::StaleAwaitingReparse => "stale_awaiting_reparse",
            Self::CancelledSuperseded => "cancelled_superseded",
        }
    }
}

/// Quality of the syntax structure produced by a parse session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseQualityClass {
    /// A complete syntax tree without parser error nodes.
    FullTree,
    /// A syntax tree exists but contains parser error or missing nodes.
    PartialTreeWithErrors,
    /// Only lexical structure is available.
    LexicalStructureOnly,
    /// Only plain-text behavior is available.
    PlainTextOnly,
    /// No structure is available.
    Unavailable,
}

impl ParseQualityClass {
    /// Returns the stable schema token for this quality class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTree => "full_tree",
            Self::PartialTreeWithErrors => "partial_tree_with_errors",
            Self::LexicalStructureOnly => "lexical_structure_only",
            Self::PlainTextOnly => "plain_text_only",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Freshness class for syntax-tree or cue state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParseFreshnessClass {
    /// The tree matches the current admitted buffer version.
    CurrentBufferVersion,
    /// A warm cached tree is intentionally reused.
    WarmCached,
    /// The tree belongs to an older buffer version.
    StaleBufferVersion,
    /// The grammar or query identity is stale.
    StaleGrammarVersion,
    /// Imported parse metadata has not been verified as current.
    UnverifiedImported,
}

impl ParseFreshnessClass {
    /// Returns the stable schema token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentBufferVersion => "current_buffer_version",
            Self::WarmCached => "warm_cached",
            Self::StaleBufferVersion => "stale_buffer_version",
            Self::StaleGrammarVersion => "stale_grammar_version",
            Self::UnverifiedImported => "unverified_imported",
        }
    }
}

/// Cache state for a parse request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheStatusClass {
    /// A current syntax-tree cache entry was reused.
    CacheHitCurrent,
    /// A stale syntax-tree cache entry was reused with disclosure.
    CacheHitStale,
    /// No syntax-tree cache entry was available.
    CacheMiss,
    /// The cache was invalidated by an edit.
    InvalidatedByEdit,
    /// The cache was invalidated by a grammar update.
    InvalidatedByGrammarUpdate,
    /// The cache was invalidated by a query-pack update.
    InvalidatedByQueryPackUpdate,
    /// The cache was invalidated by encoding changes.
    InvalidatedByEncodingChange,
    /// The cache was invalidated by trust or policy.
    InvalidatedByTrustOrPolicy,
    /// The cache was invalidated because a cache record was corrupt.
    InvalidatedByCacheCorruption,
    /// This parse result is not cacheable.
    NotCacheable,
}

impl CacheStatusClass {
    /// Returns the stable schema token for this cache status.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CacheHitCurrent => "cache_hit_current",
            Self::CacheHitStale => "cache_hit_stale",
            Self::CacheMiss => "cache_miss",
            Self::InvalidatedByEdit => "invalidated_by_edit",
            Self::InvalidatedByGrammarUpdate => "invalidated_by_grammar_update",
            Self::InvalidatedByQueryPackUpdate => "invalidated_by_query_pack_update",
            Self::InvalidatedByEncodingChange => "invalidated_by_encoding_change",
            Self::InvalidatedByTrustOrPolicy => "invalidated_by_trust_or_policy",
            Self::InvalidatedByCacheCorruption => "invalidated_by_cache_corruption",
            Self::NotCacheable => "not_cacheable",
        }
    }
}

/// Typed reason a parse session failed or degraded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureReasonClass {
    /// No failure occurred.
    None,
    /// No grammar exists for the language id.
    GrammarMissing,
    /// The grammar ABI is not compatible with the runtime.
    GrammarAbiMismatch,
    /// Grammar provenance or signature is unverified.
    GrammarSignatureUnverified,
    /// Trust or policy denied grammar loading.
    GrammarLoadDenied,
    /// The parser exceeded its time budget.
    ParseTimeoutBudgetExhausted,
    /// The parser produced error or missing nodes.
    ParserErrorNodesPresent,
    /// The Tree-sitter runtime returned an unexpected parse error.
    ParseRuntimeError,
    /// Decode recovery is still pending.
    DecodeRecoveryPending,
    /// Mixed encoding has not been resolved.
    MixedEncodingUnresolved,
    /// File size policy prevented a full parse.
    FileTooLargeForFullParse,
    /// Policy blocked parser work.
    PolicyBlocked,
    /// An extension grammar is not trusted.
    ExtensionUntrusted,
    /// Syntax cache data is corrupt.
    CacheCorrupt,
    /// The selected parser host is unavailable.
    HostUnavailable,
    /// A newer buffer version superseded the parse.
    SupersededByNewerBufferVersion,
}

impl FailureReasonClass {
    /// Returns the stable schema token for this failure reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::GrammarMissing => "grammar_missing",
            Self::GrammarAbiMismatch => "grammar_abi_mismatch",
            Self::GrammarSignatureUnverified => "grammar_signature_unverified",
            Self::GrammarLoadDenied => "grammar_load_denied",
            Self::ParseTimeoutBudgetExhausted => "parse_timeout_budget_exhausted",
            Self::ParserErrorNodesPresent => "parser_error_nodes_present",
            Self::ParseRuntimeError => "parse_runtime_error",
            Self::DecodeRecoveryPending => "decode_recovery_pending",
            Self::MixedEncodingUnresolved => "mixed_encoding_unresolved",
            Self::FileTooLargeForFullParse => "file_too_large_for_full_parse",
            Self::PolicyBlocked => "policy_blocked",
            Self::ExtensionUntrusted => "extension_untrusted",
            Self::CacheCorrupt => "cache_corrupt",
            Self::HostUnavailable => "host_unavailable",
            Self::SupersededByNewerBufferVersion => "superseded_by_newer_buffer_version",
        }
    }
}

/// Budget policy applied to a parse request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BudgetPolicyClass {
    /// Short visible-edit parser slices.
    VisibleEditInteractive,
    /// Foreground parse for a visible file open or reveal.
    ForegroundVisibleFile,
    /// Background parse for workspace or search warming.
    BackgroundWorkspace,
    /// Reduced parse budget for large or hostile files.
    LargeFileReduced,
    /// Bounded replay budget for support export.
    ExportReplayBounded,
}

impl BudgetPolicyClass {
    /// Returns the stable schema token for this budget policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleEditInteractive => "visible_edit_interactive",
            Self::ForegroundVisibleFile => "foreground_visible_file",
            Self::BackgroundWorkspace => "background_workspace",
            Self::LargeFileReduced => "large_file_reduced",
            Self::ExportReplayBounded => "export_replay_bounded",
        }
    }
}

/// Parse-derived cue class controlled by parse state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedCueClass {
    /// Syntax highlighting ranges.
    SyntaxHighlighting,
    /// Syntax-aware fold ranges.
    Folds,
    /// Indentation guide hints.
    IndentGuides,
    /// Structural selection expansion.
    StructuralSelection,
    /// Breadcrumb segments.
    Breadcrumbs,
    /// Outline rows.
    Outline,
    /// Local symbol rows.
    LocalSymbols,
    /// Bracket-matching cues.
    BracketMatching,
    /// Minimap structural markers.
    MinimapMarkers,
    /// Diagnostic anchors.
    Diagnostics,
    /// Refactor preview anchors.
    RefactorPreview,
    /// Semantic graph ingest input.
    SemanticGraphIngest,
    /// Support export projection.
    SupportExport,
}

impl DerivedCueClass {
    /// Returns the stable schema token for this cue class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyntaxHighlighting => "syntax_highlighting",
            Self::Folds => "folds",
            Self::IndentGuides => "indent_guides",
            Self::StructuralSelection => "structural_selection",
            Self::Breadcrumbs => "breadcrumbs",
            Self::Outline => "outline",
            Self::LocalSymbols => "local_symbols",
            Self::BracketMatching => "bracket_matching",
            Self::MinimapMarkers => "minimap_markers",
            Self::Diagnostics => "diagnostics",
            Self::RefactorPreview => "refactor_preview",
            Self::SemanticGraphIngest => "semantic_graph_ingest",
            Self::SupportExport => "support_export",
        }
    }
}

/// Availability posture for a parse-derived cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivedCuePostureClass {
    /// Cue is exact for the current syntax tree.
    AvailableExact,
    /// Cue is available only for a partial tree or subset.
    AvailablePartial,
    /// Cue is served only from cache.
    CachedOnly,
    /// Cue is supplied by an explicit heuristic fallback.
    FallbackHeuristic,
    /// Cue is suppressed because parse state degraded.
    SuppressedDueToDegradation,
    /// Cue is blocked by policy, trust, or unavailable structure.
    Blocked,
}

impl DerivedCuePostureClass {
    /// Returns the stable schema token for this cue posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AvailableExact => "available_exact",
            Self::AvailablePartial => "available_partial",
            Self::CachedOnly => "cached_only",
            Self::FallbackHeuristic => "fallback_heuristic",
            Self::SuppressedDueToDegradation => "suppressed_due_to_degradation",
            Self::Blocked => "blocked",
        }
    }
}

/// Export policy for parse state and parse-derived cues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPolicyClass {
    /// Metadata-only export is safe by default.
    MetadataSafeDefault,
    /// Export may include normalized range references only.
    RangeRefsOnly,
    /// Export is safe for support bundles.
    SupportExportSafe,
    /// Raw source is excluded from the export.
    BlockedRawSourceExcluded,
    /// The record cannot be exported.
    NotExportable,
}

impl ExportPolicyClass {
    /// Returns the stable schema token for this export policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::RangeRefsOnly => "range_refs_only",
            Self::SupportExportSafe => "support_export_safe",
            Self::BlockedRawSourceExcluded => "blocked_raw_source_excluded",
            Self::NotExportable => "not_exportable",
        }
    }
}

/// Epoch role attached to a parse-session record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochRoleClass {
    /// Workspace-scope epoch.
    WorkspaceScope,
    /// Buffer snapshot epoch.
    BufferSnapshot,
    /// Syntax-tree epoch.
    SyntaxTree,
    /// Grammar package epoch.
    GrammarPackage,
    /// Query-pack epoch.
    QueryPack,
    /// Coordinate-profile epoch.
    CoordinateProfile,
    /// Trust-policy epoch.
    TrustPolicy,
    /// Remote-workspace epoch.
    RemoteWorkspace,
}

impl EpochRoleClass {
    /// Returns the stable schema token for this epoch role.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceScope => "workspace_scope",
            Self::BufferSnapshot => "buffer_snapshot",
            Self::SyntaxTree => "syntax_tree",
            Self::GrammarPackage => "grammar_package",
            Self::QueryPack => "query_pack",
            Self::CoordinateProfile => "coordinate_profile",
            Self::TrustPolicy => "trust_policy",
            Self::RemoteWorkspace => "remote_workspace",
        }
    }
}

/// Buffer identity fields copied into a parse-session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BufferRef {
    /// Stable buffer identity for the admitted snapshot.
    pub buffer_id: String,
    /// Monotonic version of the admitted buffer.
    pub buffer_version: u64,
    /// Export-safe hash reference for the raw buffer bytes.
    pub buffer_content_hash_ref: String,
    /// Export-safe hash reference for decoded text.
    pub decoded_text_hash_ref: String,
    /// Encoding-state reference for coordinate and decode guarantees.
    pub encoding_state_ref: String,
    /// Decode-recovery state token from the text fidelity path.
    pub decode_recovery_state: String,
}

/// Grammar-provenance fields copied into a parse-session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GrammarResolution {
    /// Stable grammar family id.
    pub grammar_id: String,
    /// Canonical language id selected for the buffer.
    pub language_id: String,
    /// Source class for the grammar package.
    pub grammar_source_class: GrammarSourceClass,
    /// Resolution state before parsing.
    pub grammar_resolution_state_class: GrammarResolutionStateClass,
    /// Exact grammar version reference.
    pub grammar_version_ref: String,
    /// Runtime ABI reference for the grammar.
    pub grammar_abi_ref: String,
    /// Query-pack identity used for parse-derived cues.
    pub query_pack_ref: String,
    /// Export-safe artifact hash reference.
    pub artifact_hash_ref: String,
    /// Export-safe signature or provenance reference.
    pub signature_ref: String,
    /// Upstream project and revision reference.
    pub upstream_ref: String,
    /// Local patch reference, or `not_applicable`.
    pub local_patch_ref: String,
    /// Scope where this grammar resolution applies.
    pub scope_ref: String,
    /// Reviewer-facing resolution summary.
    pub summary: String,
}

/// Parser host fields copied into a parse-session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParserHost {
    /// Host class selected for this parse.
    pub parser_host_class: ParserHostClass,
    /// Export-safe parser host identity.
    pub host_identity_ref: String,
    /// Locality class used by support and routing surfaces.
    pub locality_class: String,
    /// Trust state applied to the host.
    pub trust_state: TrustState,
    /// Reviewer-facing host-placement summary.
    pub summary: String,
}

/// Incremental parse budget applied to a parse request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncrementalBudget {
    /// Budget policy selected for the request.
    pub budget_policy_class: BudgetPolicyClass,
    /// Soft slice budget in milliseconds.
    pub slice_budget_ms: u64,
    /// Burst ceiling in milliseconds.
    pub burst_ceiling_ms: u64,
    /// Maximum bytes admitted by the request budget.
    pub byte_budget: usize,
    /// Maximum syntax nodes admitted by the request budget.
    pub node_budget: usize,
    /// Whether the parser should yield after one slice.
    pub yield_after_slice: bool,
    /// Export-safe cancellation-token reference.
    pub cancellation_token_ref: String,
    /// Reviewer-facing budget summary.
    pub summary: String,
}

impl IncrementalBudget {
    /// Returns an alpha budget for a visible edit parse.
    pub fn visible_edit(cancellation_token_ref: impl Into<String>) -> Self {
        Self {
            budget_policy_class: BudgetPolicyClass::VisibleEditInteractive,
            slice_budget_ms: 3,
            burst_ceiling_ms: 8,
            byte_budget: 65_536,
            node_budget: 20_000,
            yield_after_slice: true,
            cancellation_token_ref: cancellation_token_ref.into(),
            summary: "Interactive parse budget admits one short visible-edit slice.".into(),
        }
    }

    /// Returns an alpha budget for a foreground file parse.
    pub fn foreground_visible_file(cancellation_token_ref: impl Into<String>) -> Self {
        Self {
            budget_policy_class: BudgetPolicyClass::ForegroundVisibleFile,
            slice_budget_ms: 10,
            burst_ceiling_ms: 25,
            byte_budget: 262_144,
            node_budget: 80_000,
            yield_after_slice: true,
            cancellation_token_ref: cancellation_token_ref.into(),
            summary: "Foreground parse budget allows visible-file work while preserving shell responsiveness.".into(),
        }
    }

    /// Returns an alpha budget for background parser work.
    pub fn background_workspace(cancellation_token_ref: impl Into<String>) -> Self {
        Self {
            budget_policy_class: BudgetPolicyClass::BackgroundWorkspace,
            slice_budget_ms: 25,
            burst_ceiling_ms: 75,
            byte_budget: 1_048_576,
            node_budget: 200_000,
            yield_after_slice: true,
            cancellation_token_ref: cancellation_token_ref.into(),
            summary: "Background parse budget yields to visible parser work and cancellation."
                .into(),
        }
    }
}

/// Current state of one parse request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseState {
    /// Parse lifecycle state.
    pub parse_lifecycle_state_class: ParseLifecycleStateClass,
    /// Quality of produced structure.
    pub parse_quality_class: ParseQualityClass,
    /// Freshness of produced structure.
    pub parse_freshness_class: ParseFreshnessClass,
    /// Failure or degradation reasons.
    pub failure_reason_classes: Vec<FailureReasonClass>,
    /// Number of Tree-sitter error or missing nodes.
    pub parser_error_node_count: usize,
    /// Number of unresolved byte ranges.
    pub unresolved_byte_range_count: usize,
    /// Cue classes degraded by this parse state.
    pub degraded_surface_classes: Vec<DerivedCueClass>,
    /// Reviewer-facing parse-state summary.
    pub summary: String,
}

/// Value identity for a produced syntax tree.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SyntaxTreeIdentity {
    /// Stable syntax-tree id.
    pub syntax_tree_id: String,
    /// Epoch reference for the syntax-tree value.
    pub tree_epoch_ref: String,
    /// Document reference parsed by the session.
    pub document_ref: String,
    /// Buffer id parsed by the session.
    pub buffer_id: String,
    /// Parser substrate that produced the tree.
    pub parser_substrate_class: ParserSubstrateClass,
    /// Grammar id used for the tree.
    pub grammar_id: String,
    /// Language id used for the tree.
    pub language_id: String,
    /// Grammar version reference used for the tree.
    pub grammar_version_ref: String,
    /// Grammar ABI reference used for the tree.
    pub grammar_abi_ref: String,
    /// Query-pack reference used for derived cues.
    pub query_pack_ref: String,
    /// Parser host class used for the tree.
    pub parser_host_class: ParserHostClass,
    /// Buffer version parsed for the tree.
    pub buffer_version: u64,
    /// Export-safe raw-buffer hash reference.
    pub buffer_content_hash_ref: String,
    /// Export-safe decoded-text hash reference.
    pub decoded_text_hash_ref: String,
    /// Parse session that produced the tree.
    pub parse_session_id: String,
    /// Timestamp when parsing started.
    pub parse_started_at: String,
    /// Timestamp when parsing completed.
    pub parse_completed_at: String,
    /// Quality of the produced tree.
    pub parse_quality_class: ParseQualityClass,
    /// Freshness of the produced tree.
    pub parse_freshness_class: ParseFreshnessClass,
}

/// Cache record for one parse request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CacheRecord {
    /// Cache status observed by the parse.
    pub cache_status_class: CacheStatusClass,
    /// Cache key reference for support joins.
    pub cache_key_ref: String,
    /// Previous syntax-tree reference, or `syntax-tree:none`.
    pub previous_tree_ref: String,
    /// Cache invalidation reasons.
    pub invalidation_reason_classes: Vec<CacheStatusClass>,
    /// Reviewer-facing cache summary.
    pub summary: String,
}

/// Record for one parse-derived cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedCueRecord {
    /// Cue class represented by this record.
    pub derived_cue_class: DerivedCueClass,
    /// Availability posture for the cue.
    pub derived_cue_posture_class: DerivedCuePostureClass,
    /// Parse freshness attached to the cue.
    pub parse_freshness_class: ParseFreshnessClass,
    /// Parse session that produced the cue.
    pub producer_parse_session_id: String,
    /// Syntax tree used by the cue, or `syntax-tree:none`.
    pub syntax_tree_id: String,
    /// Coordinate mapping id required for exported ranges.
    pub coordinate_mapping_id: String,
    /// Reasons the cue is blocked or degraded.
    pub blocked_or_degraded_reason_classes: Vec<FailureReasonClass>,
    /// Export policy for this cue.
    pub export_policy_class: ExportPolicyClass,
    /// Reviewer-facing cue summary.
    pub summary: String,
}

/// Epoch binding carried by parse-session records.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EpochBinding {
    /// Role of the bound epoch.
    pub epoch_role_class: EpochRoleClass,
    /// Export-safe epoch reference.
    pub epoch_ref: String,
}

/// Export policy for a parse-session record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportPolicy {
    /// Export policy class for the record.
    pub export_policy_class: ExportPolicyClass,
    /// Redaction class applied to the record.
    pub redaction_class: String,
    /// Whether raw source text is excluded.
    pub raw_source_excluded: bool,
    /// Whether raw parser logs are excluded.
    pub raw_parser_logs_excluded: bool,
    /// Whether range exports require coordinate mapping.
    pub range_export_requires_coordinate_mapping: bool,
    /// Reviewer-facing export summary.
    pub summary: String,
}

/// Boundary record for one parse request and its visible parser state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParseSessionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for parse-session records.
    pub parse_session_schema_version: ParseSessionSchemaVersion,
    /// Stable parse-session id.
    pub parse_session_id: String,
    /// Document reference parsed by this session.
    pub document_ref: String,
    /// Buffer identity parsed by this session.
    pub buffer_ref: BufferRef,
    /// Request class admitted by the parser.
    pub parse_request_class: ParseRequestClass,
    /// Cue classes requested by the caller.
    pub requested_derived_cue_classes: Vec<DerivedCueClass>,
    /// Grammar resolution outcome.
    pub grammar_resolution: GrammarResolution,
    /// Parser host placement.
    pub parser_host: ParserHost,
    /// Coordinate profile attached to this parse.
    pub coordinate_profile_ref: String,
    /// Budget applied to this parse.
    pub incremental_budget: IncrementalBudget,
    /// Visible parse state.
    pub parse_state: ParseState,
    /// Syntax-tree identity, when a tree was produced.
    pub syntax_tree_identity: Option<SyntaxTreeIdentity>,
    /// Cache state for this parse.
    pub cache_record: CacheRecord,
    /// Parse-derived cue posture records.
    pub derived_cues: Vec<DerivedCueRecord>,
    /// Epoch bindings current when the parse completed.
    pub current_epoch_bindings: Vec<EpochBinding>,
    /// Export policy for this record.
    pub export_policy: ExportPolicy,
    /// Timestamp when the record was captured.
    pub captured_at: String,
    /// Export-safe reviewer summary.
    pub export_safe_summary: String,
}

impl ParseSessionRecord {
    /// Stable record-kind tag carried in serialized parse sessions.
    pub const RECORD_KIND: &'static str = "parse_session_record";

    /// Integer schema version matching `/schemas/language/parse_session.schema.json`.
    pub const SCHEMA_VERSION: ParseSessionSchemaVersion = 1;

    /// Returns true when the record published a full current syntax tree.
    pub fn has_current_full_tree(&self) -> bool {
        self.parse_state.parse_lifecycle_state_class == ParseLifecycleStateClass::Completed
            && self.parse_state.parse_quality_class == ParseQualityClass::FullTree
            && self.syntax_tree_identity.is_some()
    }

    /// Returns true when consumers must show degraded or fallback state.
    pub fn requires_degraded_disclosure(&self) -> bool {
        self.parse_state
            .failure_reason_classes
            .iter()
            .any(|reason| *reason != FailureReasonClass::None)
            || self.parse_state.parse_quality_class != ParseQualityClass::FullTree
    }
}
