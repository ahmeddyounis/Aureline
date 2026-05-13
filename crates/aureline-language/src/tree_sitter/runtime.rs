use super::records::{
    BufferRef, CacheRecord, CacheStatusClass, DerivedCueClass, DerivedCuePostureClass,
    DerivedCueRecord, EpochBinding, EpochRoleClass, ExportPolicy, ExportPolicyClass,
    FailureReasonClass, GrammarResolution, GrammarResolutionStateClass, GrammarSourceClass,
    IncrementalBudget, ParseFreshnessClass, ParseLifecycleStateClass, ParseQualityClass,
    ParseRequestClass, ParseSessionRecord, ParseState, ParserHost, ParserHostClass,
    ParserSubstrateClass, SyntaxTreeIdentity, TrustState,
};
use super::registry::{
    default_launch_grammar_registry, GrammarDescriptor, TreeSitterGrammarRegistry,
};

/// Cache context supplied by callers that reuse or invalidate syntax trees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCacheContext {
    /// Cache status observed by this parse request.
    pub cache_status_class: CacheStatusClass,
    /// Cache key reference for support joins.
    pub cache_key_ref: String,
    /// Previous syntax-tree reference, or `syntax-tree:none`.
    pub previous_tree_ref: String,
    /// Cache invalidation reasons attached to this parse.
    pub invalidation_reason_classes: Vec<CacheStatusClass>,
    /// Reviewer-facing cache summary.
    pub summary: String,
}

impl ParseCacheContext {
    /// Builds the default cache-miss context for a fresh parse.
    pub fn cache_miss(buffer_ref: &BufferRef) -> Self {
        Self {
            cache_status_class: CacheStatusClass::CacheMiss,
            cache_key_ref: format!(
                "cache-key:syntax:{}:{}",
                buffer_ref.buffer_id, buffer_ref.buffer_version
            ),
            previous_tree_ref: "syntax-tree:none".into(),
            invalidation_reason_classes: Vec::new(),
            summary: "No reusable previous tree was available for this parse.".into(),
        }
    }

    /// Builds an edit-invalidation context for an incremental parse.
    pub fn invalidated_by_edit(
        buffer_ref: &BufferRef,
        previous_tree_ref: impl Into<String>,
    ) -> Self {
        Self {
            cache_status_class: CacheStatusClass::InvalidatedByEdit,
            cache_key_ref: format!(
                "cache-key:syntax:{}:{}",
                buffer_ref.buffer_id, buffer_ref.buffer_version
            ),
            previous_tree_ref: previous_tree_ref.into(),
            invalidation_reason_classes: vec![CacheStatusClass::InvalidatedByEdit],
            summary: "Previous syntax tree was edited and reused for an incremental parse.".into(),
        }
    }
}

/// Visible runtime state for a parser handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ParserRuntimeStateClass {
    /// Parser startup was requested.
    Starting,
    /// Grammar loaded and the parser is ready.
    Ready,
    /// Parser is running a request.
    Running,
    /// Parser completed the latest request.
    Completed,
    /// Parser failed to load or parse.
    Failed,
    /// Parser could not run because no grammar was available.
    DegradedNoGrammar,
    /// Parser was explicitly shut down.
    Shutdown,
}

impl ParserRuntimeStateClass {
    /// Returns the stable runtime-state token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Starting => "starting",
            Self::Ready => "ready",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::DegradedNoGrammar => "degraded_no_grammar",
            Self::Shutdown => "shutdown",
        }
    }
}

/// Visible lifecycle snapshot for parser startup, parse, and shutdown.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserLifecycleSnapshot {
    /// Stable parser runtime session id.
    pub runtime_session_id: String,
    /// Requested or resolved language id.
    pub language_id: String,
    /// Grammar id when grammar resolution succeeded.
    pub grammar_id: Option<String>,
    /// Current runtime state.
    pub runtime_state_class: ParserRuntimeStateClass,
    /// Current parse lifecycle state when a parse exists.
    pub parse_lifecycle_state_class: Option<ParseLifecycleStateClass>,
    /// Typed failure reasons visible to consumers.
    pub failure_reason_classes: Vec<FailureReasonClass>,
    /// Reviewer-facing lifecycle summary.
    pub summary: String,
}

impl ParserLifecycleSnapshot {
    fn ready(runtime_session_id: String, descriptor: &GrammarDescriptor) -> Self {
        Self {
            runtime_session_id,
            language_id: descriptor.language_id.into(),
            grammar_id: Some(descriptor.grammar_id.into()),
            runtime_state_class: ParserRuntimeStateClass::Ready,
            parse_lifecycle_state_class: Some(ParseLifecycleStateClass::Queued),
            failure_reason_classes: vec![FailureReasonClass::None],
            summary: format!(
                "{} parser loaded through the shared Tree-sitter registry.",
                descriptor.display_name
            ),
        }
    }

    fn missing(runtime_session_id: String, language_id: String) -> Self {
        Self {
            runtime_session_id,
            language_id,
            grammar_id: None,
            runtime_state_class: ParserRuntimeStateClass::DegradedNoGrammar,
            parse_lifecycle_state_class: Some(ParseLifecycleStateClass::DegradedNoGrammar),
            failure_reason_classes: vec![FailureReasonClass::GrammarMissing],
            summary: "No admitted Tree-sitter grammar exists, so syntax falls back explicitly."
                .into(),
        }
    }

    fn failed(
        runtime_session_id: String,
        descriptor: &GrammarDescriptor,
        reason: FailureReasonClass,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            runtime_session_id,
            language_id: descriptor.language_id.into(),
            grammar_id: Some(descriptor.grammar_id.into()),
            runtime_state_class: ParserRuntimeStateClass::Failed,
            parse_lifecycle_state_class: Some(ParseLifecycleStateClass::Failed),
            failure_reason_classes: vec![reason],
            summary: summary.into(),
        }
    }

    fn shutdown(runtime_session_id: String, descriptor: &GrammarDescriptor) -> Self {
        Self {
            runtime_session_id,
            language_id: descriptor.language_id.into(),
            grammar_id: Some(descriptor.grammar_id.into()),
            runtime_state_class: ParserRuntimeStateClass::Shutdown,
            parse_lifecycle_state_class: None,
            failure_reason_classes: vec![FailureReasonClass::None],
            summary: format!("{} parser runtime was shut down.", descriptor.display_name),
        }
    }
}

/// Startup failure that preserves the visible parser lifecycle snapshot.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserStartupError {
    snapshot: Box<ParserLifecycleSnapshot>,
}

impl ParserStartupError {
    /// Builds an error from the lifecycle snapshot consumers must surface.
    pub fn new(snapshot: ParserLifecycleSnapshot) -> Self {
        Self {
            snapshot: Box::new(snapshot),
        }
    }

    /// Returns the lifecycle snapshot for inspection.
    pub fn snapshot(&self) -> &ParserLifecycleSnapshot {
        &self.snapshot
    }

    /// Consumes the error and returns its lifecycle snapshot.
    pub fn into_snapshot(self) -> ParserLifecycleSnapshot {
        *self.snapshot
    }
}

impl std::fmt::Display for ParserStartupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.snapshot.summary)
    }
}

impl std::error::Error for ParserStartupError {}

/// Request admitted by the Tree-sitter parser runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseRequest {
    /// Stable parse-session id copied onto the parse-session record.
    pub parse_session_id: String,
    /// Stable parser runtime session id used for lifecycle snapshots.
    pub runtime_session_id: String,
    /// Document reference parsed by this request.
    pub document_ref: String,
    /// Buffer identity parsed by this request.
    pub buffer_ref: BufferRef,
    /// Requested language id or alias.
    pub language_id: String,
    /// Workspace or support scope where grammar resolution applies.
    pub scope_ref: String,
    /// Coordinate profile attached before parser offsets are projected.
    pub coordinate_profile_ref: String,
    /// Request class admitted by the parser.
    pub parse_request_class: ParseRequestClass,
    /// Cue classes requested by the caller.
    pub requested_derived_cue_classes: Vec<DerivedCueClass>,
    /// Budget applied to parser work.
    pub incremental_budget: IncrementalBudget,
    /// Capture timestamp used by deterministic fixtures.
    pub captured_at: String,
}

impl ParseRequest {
    /// Builds a foreground parse request for clean UTF-8 text.
    pub fn foreground_file(
        parse_session_id: impl Into<String>,
        document_ref: impl Into<String>,
        buffer_id: impl Into<String>,
        buffer_version: u64,
        language_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> Self {
        let parse_session_id = parse_session_id.into();
        let buffer_id = buffer_id.into();
        Self {
            runtime_session_id: format!("parser-runtime:{parse_session_id}"),
            parse_session_id,
            document_ref: document_ref.into(),
            buffer_ref: clean_utf8_buffer_ref(buffer_id, buffer_version),
            language_id: language_id.into(),
            scope_ref: "scope:workspace:local".into(),
            coordinate_profile_ref: "coord-profile:local:utf8".into(),
            parse_request_class: ParseRequestClass::OpenBufferInitial,
            requested_derived_cue_classes: default_requested_cues(),
            incremental_budget: IncrementalBudget::foreground_visible_file(
                "cancel:parse:foreground",
            ),
            captured_at: captured_at.into(),
        }
    }
}

/// Result of a parse request, including the reusable syntax tree when present.
pub struct ParseOutput {
    /// Boundary record exposing parser state to downstream consumers.
    pub record: ParseSessionRecord,
    /// Root node kind reported by Tree-sitter, when a tree was produced.
    pub root_kind: Option<String>,
    tree: Option<::tree_sitter::Tree>,
}

impl ParseOutput {
    /// Returns the parsed Tree-sitter tree when parsing produced one.
    pub fn tree(&self) -> Option<&::tree_sitter::Tree> {
        self.tree.as_ref()
    }
}

/// Supervises Tree-sitter parser startup and one-shot parse lifecycle records.
#[derive(Debug, Clone)]
pub struct TreeSitterParserSupervisor {
    registry: TreeSitterGrammarRegistry,
    parser_host_class: ParserHostClass,
    host_identity_ref: String,
}

impl TreeSitterParserSupervisor {
    /// Builds a supervisor over the curated launch grammar registry.
    pub fn with_default_registry() -> Self {
        Self::new(default_launch_grammar_registry())
    }

    /// Builds a supervisor over a caller-provided grammar registry.
    pub fn new(registry: TreeSitterGrammarRegistry) -> Self {
        Self {
            registry,
            parser_host_class: ParserHostClass::EditorProcessForegroundWorker,
            host_identity_ref: "host:local:tree-sitter-foreground".into(),
        }
    }

    /// Returns the shared registry consumed by this supervisor.
    pub fn registry(&self) -> &TreeSitterGrammarRegistry {
        &self.registry
    }

    /// Starts a parser for one language and returns a visible handle.
    ///
    /// # Errors
    ///
    /// Returns a lifecycle snapshot when grammar resolution fails or the
    /// Tree-sitter runtime rejects the loaded grammar.
    pub fn start_parser(
        &self,
        runtime_session_id: impl Into<String>,
        language_id: &str,
    ) -> Result<ParserRuntimeHandle, ParserStartupError> {
        let runtime_session_id = runtime_session_id.into();
        let descriptor = self
            .registry
            .resolve_language_id(language_id)
            .cloned()
            .ok_or_else(|| {
                ParserStartupError::new(ParserLifecycleSnapshot::missing(
                    runtime_session_id.clone(),
                    language_id.to_owned(),
                ))
            })?;

        let mut parser = ::tree_sitter::Parser::new();
        let language = descriptor.load_language();
        if parser.set_language(&language).is_err() {
            return Err(ParserStartupError::new(ParserLifecycleSnapshot::failed(
                runtime_session_id,
                &descriptor,
                FailureReasonClass::GrammarAbiMismatch,
                "Tree-sitter rejected the grammar ABI during parser startup.",
            )));
        }

        let lifecycle = ParserLifecycleSnapshot::ready(runtime_session_id.clone(), &descriptor);
        Ok(ParserRuntimeHandle {
            runtime_session_id,
            descriptor,
            parser: Some(parser),
            parser_host_class: self.parser_host_class,
            host_identity_ref: self.host_identity_ref.clone(),
            lifecycle,
        })
    }

    /// Parses text through a short-lived parser handle and returns state plus tree.
    pub fn parse_text(&self, request: ParseRequest, source_text: &str) -> ParseOutput {
        match self.start_parser(request.runtime_session_id.clone(), &request.language_id) {
            Ok(mut handle) => handle.parse_text(request, source_text),
            Err(error) => startup_failure_output(request, error.into_snapshot(), &self.registry),
        }
    }
}

/// Live parser handle with explicit parse and shutdown transitions.
pub struct ParserRuntimeHandle {
    runtime_session_id: String,
    descriptor: GrammarDescriptor,
    parser: Option<::tree_sitter::Parser>,
    parser_host_class: ParserHostClass,
    host_identity_ref: String,
    lifecycle: ParserLifecycleSnapshot,
}

impl ParserRuntimeHandle {
    /// Returns the latest lifecycle snapshot for this parser handle.
    pub fn lifecycle(&self) -> &ParserLifecycleSnapshot {
        &self.lifecycle
    }

    /// Parses text and emits a parse-session record plus syntax tree.
    pub fn parse_text(&mut self, request: ParseRequest, source_text: &str) -> ParseOutput {
        let cache_context = ParseCacheContext::cache_miss(&request.buffer_ref);
        self.parse_text_with_cache_context(request, source_text, None, cache_context)
    }

    /// Parses text with an optional edited previous tree and explicit cache context.
    pub fn parse_text_with_cache_context(
        &mut self,
        request: ParseRequest,
        source_text: &str,
        old_tree: Option<&::tree_sitter::Tree>,
        cache_context: ParseCacheContext,
    ) -> ParseOutput {
        if source_text.len() > request.incremental_budget.byte_budget {
            self.lifecycle.runtime_state_class = ParserRuntimeStateClass::Failed;
            self.lifecycle.parse_lifecycle_state_class =
                Some(ParseLifecycleStateClass::YieldedBudgetExhausted);
            self.lifecycle.failure_reason_classes =
                vec![FailureReasonClass::FileTooLargeForFullParse];

            let grammar_resolution = self
                .descriptor
                .grammar_resolution(request.scope_ref.clone());
            return degraded_output(
                request,
                grammar_resolution,
                parser_host(
                    self.parser_host_class,
                    &self.host_identity_ref,
                    TrustState::Trusted,
                    "Parser work was admitted, but the byte budget blocked a full parse.",
                ),
                ParseLifecycleStateClass::YieldedBudgetExhausted,
                ParseQualityClass::LexicalStructureOnly,
                vec![FailureReasonClass::FileTooLargeForFullParse],
                "The file exceeded the admitted byte budget; structural cues are narrowed.",
            );
        }

        self.lifecycle.runtime_state_class = ParserRuntimeStateClass::Running;
        self.lifecycle.parse_lifecycle_state_class = Some(ParseLifecycleStateClass::Running);

        let parser = match self.parser.as_mut() {
            Some(parser) => parser,
            None => {
                self.lifecycle.runtime_state_class = ParserRuntimeStateClass::Failed;
                self.lifecycle.parse_lifecycle_state_class = Some(ParseLifecycleStateClass::Failed);
                self.lifecycle.failure_reason_classes = vec![FailureReasonClass::HostUnavailable];
                let grammar_resolution = self
                    .descriptor
                    .grammar_resolution(request.scope_ref.clone());
                return degraded_output(
                    request,
                    grammar_resolution,
                    parser_host(
                        self.parser_host_class,
                        &self.host_identity_ref,
                        TrustState::Trusted,
                        "Parser handle was unavailable before parse execution.",
                    ),
                    ParseLifecycleStateClass::Failed,
                    ParseQualityClass::Unavailable,
                    vec![FailureReasonClass::HostUnavailable],
                    "Parser host was unavailable before parse execution.",
                );
            }
        };

        parser.set_timeout_micros(request.incremental_budget.burst_ceiling_ms * 1_000);
        let Some(tree) = parser.parse(source_text, old_tree) else {
            self.lifecycle.runtime_state_class = ParserRuntimeStateClass::Failed;
            self.lifecycle.parse_lifecycle_state_class = Some(ParseLifecycleStateClass::Failed);
            self.lifecycle.failure_reason_classes =
                vec![FailureReasonClass::ParseTimeoutBudgetExhausted];
            let grammar_resolution = self
                .descriptor
                .grammar_resolution(request.scope_ref.clone());
            return degraded_output(
                request,
                grammar_resolution,
                parser_host(
                    self.parser_host_class,
                    &self.host_identity_ref,
                    TrustState::Trusted,
                    "Parser work failed inside the admitted time budget.",
                ),
                ParseLifecycleStateClass::Failed,
                ParseQualityClass::Unavailable,
                vec![FailureReasonClass::ParseTimeoutBudgetExhausted],
                "Tree-sitter did not publish a tree inside the admitted budget.",
            );
        };

        let root = tree.root_node();
        let root_kind = root.kind().to_owned();
        let parser_error_node_count = count_error_nodes(root);
        let has_errors = parser_error_node_count > 0;
        let parse_quality = if has_errors {
            ParseQualityClass::PartialTreeWithErrors
        } else {
            ParseQualityClass::FullTree
        };
        let failure_reasons = if has_errors {
            vec![FailureReasonClass::ParserErrorNodesPresent]
        } else {
            vec![FailureReasonClass::None]
        };
        let syntax_tree_id = syntax_tree_id(&request, self.descriptor.language_id);
        let parse_state = ParseState {
            parse_lifecycle_state_class: ParseLifecycleStateClass::Completed,
            parse_quality_class: parse_quality,
            parse_freshness_class: ParseFreshnessClass::CurrentBufferVersion,
            failure_reason_classes: failure_reasons.clone(),
            parser_error_node_count,
            unresolved_byte_range_count: 0,
            degraded_surface_classes: degraded_surfaces(
                &request.requested_derived_cue_classes,
                parse_quality,
            ),
            summary: if has_errors {
                "Tree-sitter produced a partial tree with parser error or missing nodes.".into()
            } else {
                "Tree-sitter produced a current syntax tree for the admitted buffer version.".into()
            },
        };

        let grammar_resolution = self
            .descriptor
            .grammar_resolution(request.scope_ref.clone());
        let parser_host_record = parser_host(
            self.parser_host_class,
            &self.host_identity_ref,
            TrustState::Trusted,
            "Bounded parser work ran in the foreground parser worker.",
        );
        let syntax_identity = SyntaxTreeIdentity {
            syntax_tree_id: syntax_tree_id.clone(),
            tree_epoch_ref: format!("epoch:{syntax_tree_id}"),
            document_ref: request.document_ref.clone(),
            buffer_id: request.buffer_ref.buffer_id.clone(),
            parser_substrate_class: ParserSubstrateClass::TreeSitter,
            grammar_id: self.descriptor.grammar_id.into(),
            language_id: self.descriptor.language_id.into(),
            grammar_version_ref: grammar_resolution.grammar_version_ref.clone(),
            grammar_abi_ref: grammar_resolution.grammar_abi_ref.clone(),
            query_pack_ref: grammar_resolution.query_pack_ref.clone(),
            parser_host_class: self.parser_host_class,
            buffer_version: request.buffer_ref.buffer_version,
            buffer_content_hash_ref: request.buffer_ref.buffer_content_hash_ref.clone(),
            decoded_text_hash_ref: request.buffer_ref.decoded_text_hash_ref.clone(),
            parse_session_id: request.parse_session_id.clone(),
            parse_started_at: request.captured_at.clone(),
            parse_completed_at: request.captured_at.clone(),
            parse_quality_class: parse_quality,
            parse_freshness_class: ParseFreshnessClass::CurrentBufferVersion,
        };

        self.lifecycle.runtime_state_class = ParserRuntimeStateClass::Completed;
        self.lifecycle.parse_lifecycle_state_class = Some(ParseLifecycleStateClass::Completed);
        self.lifecycle.failure_reason_classes = failure_reasons.clone();

        let record = ParseSessionRecord {
            record_kind: ParseSessionRecord::RECORD_KIND.into(),
            parse_session_schema_version: ParseSessionRecord::SCHEMA_VERSION,
            parse_session_id: request.parse_session_id.clone(),
            document_ref: request.document_ref.clone(),
            buffer_ref: request.buffer_ref.clone(),
            parse_request_class: request.parse_request_class,
            requested_derived_cue_classes: request.requested_derived_cue_classes.clone(),
            grammar_resolution,
            parser_host: parser_host_record,
            coordinate_profile_ref: request.coordinate_profile_ref.clone(),
            incremental_budget: request.incremental_budget.clone(),
            parse_state,
            syntax_tree_identity: Some(syntax_identity),
            cache_record: cache_record(cache_context),
            derived_cues: derived_cues(&request, &syntax_tree_id, parse_quality, &failure_reasons),
            current_epoch_bindings: epoch_bindings(
                &request,
                &self.descriptor,
                Some(&syntax_tree_id),
            ),
            export_policy: default_export_policy(),
            captured_at: request.captured_at.clone(),
            export_safe_summary: format!(
                "{} syntax tree rooted at `{}` is {} for buffer version {}.",
                self.descriptor.display_name,
                root_kind,
                if has_errors { "partial" } else { "current" },
                request.buffer_ref.buffer_version
            ),
        };

        ParseOutput {
            record,
            root_kind: Some(root_kind),
            tree: Some(tree),
        }
    }

    /// Shuts down this parser handle and returns the final lifecycle snapshot.
    pub fn shutdown(mut self) -> ParserLifecycleSnapshot {
        self.parser.take();
        let snapshot =
            ParserLifecycleSnapshot::shutdown(self.runtime_session_id.clone(), &self.descriptor);
        self.lifecycle = snapshot.clone();
        snapshot
    }
}

fn clean_utf8_buffer_ref(buffer_id: String, buffer_version: u64) -> BufferRef {
    BufferRef {
        buffer_content_hash_ref: format!("hash:buffer:{buffer_id}:v{buffer_version}"),
        decoded_text_hash_ref: format!("hash:decoded:{buffer_id}:v{buffer_version}"),
        encoding_state_ref: "encoding:utf8:lf:no-bom".into(),
        decode_recovery_state: "clean_decode".into(),
        buffer_id,
        buffer_version,
    }
}

fn default_requested_cues() -> Vec<DerivedCueClass> {
    vec![
        DerivedCueClass::SyntaxHighlighting,
        DerivedCueClass::Folds,
        DerivedCueClass::IndentGuides,
        DerivedCueClass::StructuralSelection,
        DerivedCueClass::Breadcrumbs,
        DerivedCueClass::LocalSymbols,
        DerivedCueClass::BracketMatching,
        DerivedCueClass::SupportExport,
    ]
}

fn parser_host(
    parser_host_class: ParserHostClass,
    host_identity_ref: &str,
    trust_state: TrustState,
    summary: impl Into<String>,
) -> ParserHost {
    ParserHost {
        parser_host_class,
        host_identity_ref: host_identity_ref.into(),
        locality_class: match parser_host_class {
            ParserHostClass::EditorProcessForegroundWorker => "local_in_process",
            ParserHostClass::LocalSidecarWorker => "local_sidecar",
            ParserHostClass::WorkspaceRemoteAgent => "workspace_remote_agent",
            ParserHostClass::ImportedSnapshot => "imported_snapshot",
            ParserHostClass::NotScheduled => "not_applicable",
        }
        .into(),
        trust_state,
        summary: summary.into(),
    }
}

fn missing_grammar_output(request: ParseRequest, snapshot: ParserLifecycleSnapshot) -> ParseOutput {
    let language_id = request.language_id.clone();
    let grammar_resolution = GrammarResolution {
        grammar_id: format!("grammar:unavailable:{}", sanitize_id(&language_id)),
        language_id,
        grammar_source_class: GrammarSourceClass::NotApplicable,
        grammar_resolution_state_class: GrammarResolutionStateClass::MissingForLanguage,
        grammar_version_ref: "not_applicable".into(),
        grammar_abi_ref: "not_applicable".into(),
        query_pack_ref: "not_applicable".into(),
        artifact_hash_ref: "not_applicable".into(),
        signature_ref: "not_applicable".into(),
        upstream_ref: "not_applicable".into(),
        local_patch_ref: "not_applicable".into(),
        scope_ref: request.scope_ref.clone(),
        summary: "No admitted grammar package exists for this language id.".into(),
    };
    degraded_output(
        request,
        grammar_resolution,
        parser_host(
            ParserHostClass::NotScheduled,
            "host:not_scheduled",
            TrustState::Trusted,
            snapshot.summary,
        ),
        ParseLifecycleStateClass::DegradedNoGrammar,
        ParseQualityClass::PlainTextOnly,
        vec![FailureReasonClass::GrammarMissing],
        "No grammar is available, so syntax-aware cues use explicit fallback labels.",
    )
}

fn startup_failure_output(
    request: ParseRequest,
    snapshot: ParserLifecycleSnapshot,
    registry: &TreeSitterGrammarRegistry,
) -> ParseOutput {
    if snapshot
        .failure_reason_classes
        .contains(&FailureReasonClass::GrammarMissing)
    {
        return missing_grammar_output(request, snapshot);
    }

    let grammar_resolution = registry
        .resolve_language_id(&request.language_id)
        .map(|descriptor| {
            let mut resolution = descriptor.grammar_resolution(request.scope_ref.clone());
            resolution.grammar_resolution_state_class = GrammarResolutionStateClass::AbiMismatch;
            resolution.summary = snapshot.summary.clone();
            resolution
        })
        .unwrap_or_else(|| GrammarResolution {
            grammar_id: snapshot.grammar_id.clone().unwrap_or_else(|| {
                format!("grammar:unavailable:{}", sanitize_id(&request.language_id))
            }),
            language_id: request.language_id.clone(),
            grammar_source_class: GrammarSourceClass::NotApplicable,
            grammar_resolution_state_class: GrammarResolutionStateClass::AbiMismatch,
            grammar_version_ref: "not_applicable".into(),
            grammar_abi_ref: "not_applicable".into(),
            query_pack_ref: "not_applicable".into(),
            artifact_hash_ref: "not_applicable".into(),
            signature_ref: "not_applicable".into(),
            upstream_ref: "not_applicable".into(),
            local_patch_ref: "not_applicable".into(),
            scope_ref: request.scope_ref.clone(),
            summary: snapshot.summary.clone(),
        });

    degraded_output(
        request,
        grammar_resolution,
        parser_host(
            ParserHostClass::NotScheduled,
            "host:not_scheduled",
            TrustState::Trusted,
            snapshot.summary,
        ),
        ParseLifecycleStateClass::Failed,
        ParseQualityClass::Unavailable,
        snapshot.failure_reason_classes,
        "Parser startup failed before a syntax tree could be produced.",
    )
}

fn degraded_output(
    request: ParseRequest,
    grammar_resolution: GrammarResolution,
    parser_host: ParserHost,
    lifecycle_state: ParseLifecycleStateClass,
    quality: ParseQualityClass,
    failure_reasons: Vec<FailureReasonClass>,
    summary: &str,
) -> ParseOutput {
    let parse_state = ParseState {
        parse_lifecycle_state_class: lifecycle_state,
        parse_quality_class: quality,
        parse_freshness_class: ParseFreshnessClass::CurrentBufferVersion,
        failure_reason_classes: failure_reasons.clone(),
        parser_error_node_count: 0,
        unresolved_byte_range_count: 0,
        degraded_surface_classes: request.requested_derived_cue_classes.clone(),
        summary: summary.into(),
    };

    let record = ParseSessionRecord {
        record_kind: ParseSessionRecord::RECORD_KIND.into(),
        parse_session_schema_version: ParseSessionRecord::SCHEMA_VERSION,
        parse_session_id: request.parse_session_id.clone(),
        document_ref: request.document_ref.clone(),
        buffer_ref: request.buffer_ref.clone(),
        parse_request_class: request.parse_request_class,
        requested_derived_cue_classes: request.requested_derived_cue_classes.clone(),
        grammar_resolution,
        parser_host,
        coordinate_profile_ref: request.coordinate_profile_ref.clone(),
        incremental_budget: request.incremental_budget.clone(),
        parse_state,
        syntax_tree_identity: None,
        cache_record: cache_record(ParseCacheContext {
            cache_status_class: CacheStatusClass::NotCacheable,
            cache_key_ref: format!("cache-key:none:{}", sanitize_id(&request.language_id)),
            previous_tree_ref: "syntax-tree:none".into(),
            invalidation_reason_classes: Vec::new(),
            summary: "No reusable syntax tree is cacheable for this degraded parse.".into(),
        }),
        derived_cues: derived_cues(&request, "syntax-tree:none", quality, &failure_reasons),
        current_epoch_bindings: vec![
            EpochBinding {
                epoch_role_class: EpochRoleClass::WorkspaceScope,
                epoch_ref: format!("epoch:{}", request.scope_ref),
            },
            EpochBinding {
                epoch_role_class: EpochRoleClass::BufferSnapshot,
                epoch_ref: format!(
                    "epoch:buffer:{}:v{}",
                    request.buffer_ref.buffer_id, request.buffer_ref.buffer_version
                ),
            },
        ],
        export_policy: default_export_policy(),
        captured_at: request.captured_at.clone(),
        export_safe_summary: summary.into(),
    };

    ParseOutput {
        record,
        root_kind: None,
        tree: None,
    }
}

fn cache_record(cache_context: ParseCacheContext) -> CacheRecord {
    CacheRecord {
        cache_status_class: cache_context.cache_status_class,
        cache_key_ref: cache_context.cache_key_ref,
        previous_tree_ref: cache_context.previous_tree_ref,
        invalidation_reason_classes: cache_context.invalidation_reason_classes,
        summary: cache_context.summary,
    }
}

fn derived_cues(
    request: &ParseRequest,
    syntax_tree_id: &str,
    quality: ParseQualityClass,
    failure_reasons: &[FailureReasonClass],
) -> Vec<DerivedCueRecord> {
    request
        .requested_derived_cue_classes
        .iter()
        .copied()
        .map(|cue| {
            let posture = cue_posture(cue, quality);
            DerivedCueRecord {
                derived_cue_class: cue,
                derived_cue_posture_class: posture,
                parse_freshness_class: ParseFreshnessClass::CurrentBufferVersion,
                producer_parse_session_id: request.parse_session_id.clone(),
                syntax_tree_id: syntax_tree_id.into(),
                coordinate_mapping_id: format!(
                    "coordmap:{}:{}",
                    request.buffer_ref.buffer_id,
                    cue.as_str()
                ),
                blocked_or_degraded_reason_classes: if posture
                    == DerivedCuePostureClass::AvailableExact
                {
                    Vec::new()
                } else {
                    failure_reasons.to_vec()
                },
                export_policy_class: ExportPolicyClass::RangeRefsOnly,
                summary: cue_summary(cue, posture),
            }
        })
        .collect()
}

fn cue_posture(cue: DerivedCueClass, quality: ParseQualityClass) -> DerivedCuePostureClass {
    match quality {
        ParseQualityClass::FullTree => DerivedCuePostureClass::AvailableExact,
        ParseQualityClass::PartialTreeWithErrors => match cue {
            DerivedCueClass::SyntaxHighlighting
            | DerivedCueClass::BracketMatching
            | DerivedCueClass::SupportExport => DerivedCuePostureClass::AvailablePartial,
            _ => DerivedCuePostureClass::SuppressedDueToDegradation,
        },
        ParseQualityClass::LexicalStructureOnly | ParseQualityClass::PlainTextOnly => match cue {
            DerivedCueClass::SyntaxHighlighting => DerivedCuePostureClass::FallbackHeuristic,
            DerivedCueClass::SupportExport => DerivedCuePostureClass::AvailablePartial,
            _ => DerivedCuePostureClass::Blocked,
        },
        ParseQualityClass::Unavailable => DerivedCuePostureClass::Blocked,
    }
}

fn cue_summary(cue: DerivedCueClass, posture: DerivedCuePostureClass) -> String {
    format!(
        "{} cue posture is {} for this parse session.",
        cue.as_str(),
        posture.as_str()
    )
}

fn degraded_surfaces(
    requested: &[DerivedCueClass],
    quality: ParseQualityClass,
) -> Vec<DerivedCueClass> {
    requested
        .iter()
        .copied()
        .filter(|cue| cue_posture(*cue, quality) != DerivedCuePostureClass::AvailableExact)
        .collect()
}

fn epoch_bindings(
    request: &ParseRequest,
    descriptor: &GrammarDescriptor,
    syntax_tree_id: Option<&str>,
) -> Vec<EpochBinding> {
    let mut bindings = vec![
        EpochBinding {
            epoch_role_class: EpochRoleClass::WorkspaceScope,
            epoch_ref: format!("epoch:{}", request.scope_ref),
        },
        EpochBinding {
            epoch_role_class: EpochRoleClass::BufferSnapshot,
            epoch_ref: format!(
                "epoch:buffer:{}:v{}",
                request.buffer_ref.buffer_id, request.buffer_ref.buffer_version
            ),
        },
        EpochBinding {
            epoch_role_class: EpochRoleClass::GrammarPackage,
            epoch_ref: format!("epoch:{}", descriptor.grammar_version_ref()),
        },
        EpochBinding {
            epoch_role_class: EpochRoleClass::QueryPack,
            epoch_ref: format!("epoch:{}", descriptor.query_pack_ref),
        },
    ];

    if let Some(syntax_tree_id) = syntax_tree_id {
        bindings.push(EpochBinding {
            epoch_role_class: EpochRoleClass::SyntaxTree,
            epoch_ref: format!("epoch:{syntax_tree_id}"),
        });
    }

    bindings
}

fn default_export_policy() -> ExportPolicy {
    ExportPolicy {
        export_policy_class: ExportPolicyClass::MetadataSafeDefault,
        redaction_class: "metadata_safe_default".into(),
        raw_source_excluded: true,
        raw_parser_logs_excluded: true,
        range_export_requires_coordinate_mapping: true,
        summary: "Parse state export includes metadata and range refs, never raw source text."
            .into(),
    }
}

fn syntax_tree_id(request: &ParseRequest, language_id: &str) -> String {
    format!(
        "syntax-tree:{}:v{}:{}",
        sanitize_id(&request.buffer_ref.buffer_id),
        request.buffer_ref.buffer_version,
        sanitize_id(language_id)
    )
}

fn sanitize_id(value: &str) -> String {
    value
        .trim()
        .trim_start_matches("language:")
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '-'
            }
        })
        .collect()
}

fn count_error_nodes(node: ::tree_sitter::Node<'_>) -> usize {
    let self_count = usize::from(node.is_error() || node.is_missing());
    self_count
        + (0..node.child_count())
            .filter_map(|index| node.child(index))
            .map(count_error_nodes)
            .sum::<usize>()
}
