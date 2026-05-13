//! Query-envelope contract for search, graph, and docs surfaces.
//!
//! The query envelope projects the lower-level subscription envelope into the
//! state vocabulary that live surfaces, support artifacts, and benchmark traces
//! share. Search, graph, and docs consumers all subscribe to this module's
//! runtime and observe the same `ready`, `warming`, `partial`, `stale`, and
//! `failed` state tokens.

use std::cell::RefCell;
use std::rc::Rc;

use crate::envelope::{
    AuthorityClass, BackpressureMode, CausedBy, Completeness, DerivationClass, FrameClass,
    Freshness, Invalidation, JsonValue, ProducerRef, ScopeClass, ScopeRef, StaleReason,
    SubscriptionEnvelope, TerminalReason, ViewClass,
};
use crate::producers::synthetic_instance;
use crate::store::{Emission, Producer, ReactiveStore, SamplePayload, StoreError};

/// Schema version for query-envelope records and artifacts.
pub const QUERY_ENVELOPE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`QueryEnvelopeRecord`].
pub const QUERY_ENVELOPE_RECORD_KIND: &str = "query_envelope_alpha_record";

/// Stable record-kind tag for [`QueryEnvelopeSupportArtifact`].
pub const QUERY_ENVELOPE_SUPPORT_ARTIFACT_RECORD_KIND: &str = "query_envelope_support_artifact";

/// Stable record-kind tag for [`QueryEnvelopeBenchmarkTrace`].
pub const QUERY_ENVELOPE_BENCHMARK_TRACE_RECORD_KIND: &str = "query_envelope_benchmark_trace";

/// Consumer surface that receives query-envelope frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryConsumerSurface {
    /// Search result and quick-navigation surfaces.
    Search,
    /// Code, symbol, dependency, and docs graph surfaces.
    Graph,
    /// Documentation search and docs-browser surfaces.
    Docs,
}

impl QueryConsumerSurface {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Docs => "docs",
        }
    }

    /// Returns the default query family for this surface.
    pub const fn default_query_family(self) -> &'static str {
        match self {
            Self::Search => "search.results",
            Self::Graph => "graph.neighborhood",
            Self::Docs => "docs.results",
        }
    }

    /// Returns the producer identity advertised in support artifacts.
    pub const fn producer_id(self) -> &'static str {
        match self {
            Self::Search => "aureline.search.query",
            Self::Graph => "aureline.graph.query",
            Self::Docs => "aureline.docs.query",
        }
    }

    /// Returns the default materialized-view class for this surface.
    pub const fn default_view_class(self) -> ViewClass {
        match self {
            Self::Search | Self::Docs => ViewClass::DurableLocalMaterialization,
            Self::Graph => ViewClass::ExportableSnapshot,
        }
    }

    /// Returns the default backpressure mode for this surface.
    pub const fn default_backpressure_mode(self) -> BackpressureMode {
        match self {
            Self::Search => BackpressureMode::Realtime,
            Self::Graph | Self::Docs => BackpressureMode::Coalesced,
        }
    }

    /// Returns a deterministic producer instance suffix for fixtures.
    pub const fn producer_pid(self) -> u32 {
        match self {
            Self::Search => 6201,
            Self::Graph => 6202,
            Self::Docs => 6203,
        }
    }
}

/// Query state observed by live surfaces and exported artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryEnvelopeState {
    /// The requested scope is fully represented by current data.
    Ready,
    /// The producer is still preparing the requested scope.
    Warming,
    /// The producer has returned a usable subset of the requested scope.
    Partial,
    /// The producer returned data that is known to be out of date.
    Stale,
    /// The subscription cannot continue or the producer reported failure.
    Failed,
}

impl QueryEnvelopeState {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Warming => "warming",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Failed => "failed",
        }
    }

    /// Returns the human-readable label shown by shell consumers.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Warming => "Warming",
            Self::Partial => "Partial",
            Self::Stale => "Stale",
            Self::Failed => "Failed",
        }
    }

    /// Returns whether this state must avoid claiming full current results.
    pub const fn narrows_current_claim(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// Classifies a subscription envelope into the query-envelope vocabulary.
    pub fn from_envelope(
        envelope: &SubscriptionEnvelope,
        has_cancellation_reason: bool,
        has_failure_reason: bool,
    ) -> Self {
        if has_failure_reason
            || has_cancellation_reason
            || matches!(envelope.frame_class, FrameClass::Terminal)
            || matches!(envelope.completeness, Completeness::Unavailable)
        {
            return Self::Failed;
        }

        if matches!(
            envelope.freshness,
            Freshness::Cached | Freshness::Stale | Freshness::Imported | Freshness::Replayed
        ) || matches!(envelope.frame_class, FrameClass::ResyncRequired)
        {
            return Self::Stale;
        }

        if matches!(envelope.completeness, Completeness::Partial) {
            return Self::Partial;
        }

        if matches!(envelope.freshness, Freshness::Warming)
            || matches!(envelope.completeness, Completeness::Unloaded)
        {
            return Self::Warming;
        }

        Self::Ready
    }
}

/// User-visible reason a query subscription refreshed or became stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryRefreshReason {
    /// The consumer opened a new subscription.
    InitialSubscribe,
    /// The user explicitly requested a refresh.
    UserRefresh,
    /// The query scope changed.
    ScopeChanged,
    /// The search index epoch advanced.
    IndexEpochAdvanced,
    /// The graph epoch advanced.
    GraphEpochAdvanced,
    /// The docs pack or docs revision changed.
    DocsPackChanged,
    /// The policy epoch changed.
    PolicyEpochChanged,
    /// The producer restarted.
    ProducerRestart,
    /// Backpressure required a fresh snapshot.
    BackpressureResync,
    /// A benchmark replay produced this frame.
    BenchmarkReplay,
}

impl QueryRefreshReason {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InitialSubscribe => "initial_subscribe",
            Self::UserRefresh => "user_refresh",
            Self::ScopeChanged => "scope_changed",
            Self::IndexEpochAdvanced => "index_epoch_advanced",
            Self::GraphEpochAdvanced => "graph_epoch_advanced",
            Self::DocsPackChanged => "docs_pack_changed",
            Self::PolicyEpochChanged => "policy_epoch_changed",
            Self::ProducerRestart => "producer_restart",
            Self::BackpressureResync => "backpressure_resync",
            Self::BenchmarkReplay => "benchmark_replay",
        }
    }

    fn stale_reason(self) -> StaleReason {
        match self {
            Self::InitialSubscribe | Self::UserRefresh => StaleReason::ExplicitRefreshRequested,
            Self::ScopeChanged => StaleReason::ScopeRemoved,
            Self::IndexEpochAdvanced | Self::GraphEpochAdvanced | Self::DocsPackChanged => {
                StaleReason::AuthorityEpochRolled
            }
            Self::PolicyEpochChanged => StaleReason::PolicyEpochChanged,
            Self::ProducerRestart => StaleReason::ProducerRestart,
            Self::BackpressureResync | Self::BenchmarkReplay => StaleReason::CausalityLost,
        }
    }
}

/// User-visible reason a query subscription ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QueryCancellationReason {
    /// The user cancelled the in-flight query.
    UserCancelled,
    /// A newer query superseded this subscription.
    SupersededByNewQuery,
    /// The requested scope was removed.
    ScopeRemoved,
    /// The query exceeded its deadline.
    DeadlineExceeded,
    /// Policy terminated the query.
    PolicyTerminated,
    /// The producer shut down.
    ProducerShutdown,
}

impl QueryCancellationReason {
    /// Returns the stable token used in exported records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserCancelled => "user_cancelled",
            Self::SupersededByNewQuery => "superseded_by_new_query",
            Self::ScopeRemoved => "scope_removed",
            Self::DeadlineExceeded => "deadline_exceeded",
            Self::PolicyTerminated => "policy_terminated",
            Self::ProducerShutdown => "producer_shutdown",
        }
    }

    fn terminal_reason(self) -> TerminalReason {
        match self {
            Self::UserCancelled | Self::SupersededByNewQuery | Self::DeadlineExceeded => {
                TerminalReason::ConsumerCancelled
            }
            Self::ScopeRemoved => TerminalReason::ScopeRemoved,
            Self::PolicyTerminated => TerminalReason::PolicyTerminated,
            Self::ProducerShutdown => TerminalReason::ProducerShutdown,
        }
    }

    fn stale_reason(self) -> StaleReason {
        match self {
            Self::ScopeRemoved => StaleReason::ScopeRemoved,
            Self::PolicyTerminated => StaleReason::PolicyEpochChanged,
            Self::ProducerShutdown => StaleReason::ProducerRestart,
            Self::UserCancelled | Self::SupersededByNewQuery | Self::DeadlineExceeded => {
                StaleReason::ExplicitRefreshRequested
            }
        }
    }
}

/// Payload rollup emitted by query-envelope producers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct QueryEnvelopePayload {
    /// Human-readable payload summary.
    pub summary: String,
    /// Number of results or graph/docs rows represented by this frame.
    pub result_count: u64,
    /// Number of rows currently covered by the producer.
    pub coverage_ready: u64,
    /// Total number of rows or scope units requested by the consumer.
    pub coverage_total: u64,
    /// Additional redaction-safe payload detail lines.
    pub detail_lines: Vec<(String, String)>,
}

impl QueryEnvelopePayload {
    /// Converts this payload into the store's sample payload shape.
    pub fn to_sample_payload(&self) -> SamplePayload {
        SamplePayload {
            summary: self.summary.clone(),
            entry_count: self.result_count,
            coverage_ready: self.coverage_ready,
            coverage_total: self.coverage_total,
            detail_lines: self.detail_lines.clone(),
        }
    }
}

/// Subscription inputs needed to register a query-envelope producer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeSubscriptionInput {
    /// Surface that will consume the subscription.
    pub surface: QueryConsumerSurface,
    /// Query family used by the lower-level subscription store.
    pub query_family: String,
    /// Scope requested by the consumer.
    pub scope_ref: ScopeRef,
    /// Producer version copied into support artifacts.
    pub producer_version: String,
    /// Backpressure mode requested by the consumer.
    pub backpressure_mode: BackpressureMode,
}

impl QueryEnvelopeSubscriptionInput {
    /// Builds a default workspace-scoped subscription input.
    pub fn for_workspace(surface: QueryConsumerSurface, workspace_id: impl Into<String>) -> Self {
        Self {
            surface,
            query_family: surface.default_query_family().to_string(),
            scope_ref: ScopeRef {
                class: ScopeClass::Workspace,
                id: workspace_id.into(),
            },
            producer_version: env!("CARGO_PKG_VERSION").to_string(),
            backpressure_mode: surface.default_backpressure_mode(),
        }
    }

    fn to_producer(&self) -> Producer {
        Producer {
            query_family: self.query_family.clone(),
            scope_ref: self.scope_ref.clone(),
            authority_class: AuthorityClass::DerivedKnowledge,
            derivation_class: DerivationClass::Authoritative,
            view_class: self.surface.default_view_class(),
            backpressure_mode: self.backpressure_mode,
            producer_refs: vec![ProducerRef {
                producer_id: self.surface.producer_id().to_string(),
                producer_instance: synthetic_instance(
                    self.surface.producer_id(),
                    2026051300,
                    self.surface.producer_pid(),
                ),
                producer_version: Some(self.producer_version.clone()),
                input_digests: Vec::new(),
                derivation_epoch: None,
                source: None,
            }],
        }
    }
}

/// One query-envelope frame projected from a subscription envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeRecord {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha record.
    pub schema_version: u32,
    /// Stable frame identity built from subscription, epoch, sequence, and frame class.
    pub query_envelope_id: String,
    /// Lower-level subscription identity.
    pub subscription_id: u64,
    /// Consumer surface token.
    pub consumer_surface: String,
    /// Query family token from the subscription store.
    pub query_family: String,
    /// Scope class token copied from the lower-level envelope.
    pub scope_class: String,
    /// Scope identity copied from the lower-level envelope.
    pub scope_id: String,
    /// Query-envelope state classification.
    pub state: QueryEnvelopeState,
    /// Stable state token copied to logs, fixtures, and support artifacts.
    pub state_token: String,
    /// Human-readable state label for shell projections.
    pub state_label: String,
    /// Snapshot epoch copied from the lower-level envelope.
    pub snapshot_epoch: u64,
    /// Delta sequence copied from the lower-level envelope.
    pub delta_seq: u64,
    /// Frame class token copied from the lower-level envelope.
    pub frame_class: String,
    /// Freshness token copied from the lower-level envelope.
    pub freshness: String,
    /// Completeness token copied from the lower-level envelope.
    pub completeness: String,
    /// Backpressure mode copied from the lower-level envelope.
    pub backpressure_mode: String,
    /// Producer identity copied from the first producer reference.
    pub producer_id: String,
    /// Producer version copied from the first producer reference when present.
    pub producer_version: Option<String>,
    /// Invalidation reason copied from the lower-level envelope when present.
    pub invalidation_reason: Option<String>,
    /// Refresh reason that caused this frame when known.
    pub refresh_reason: Option<String>,
    /// Cancellation reason that ended this subscription when known.
    pub cancellation_reason: Option<String>,
    /// Failure reason reported by the producer when known.
    pub failure_reason: Option<String>,
    /// Stable observation timestamp supplied by the caller.
    pub observed_at: String,
    /// Number of results or graph/docs rows represented by this frame.
    pub result_count: u64,
    /// Number of rows currently covered by the producer.
    pub coverage_ready: u64,
    /// Total number of rows or scope units requested by the consumer.
    pub coverage_total: u64,
}

impl QueryEnvelopeRecord {
    /// Projects a subscription envelope into a query-envelope record.
    pub fn from_subscription_envelope(
        surface: QueryConsumerSurface,
        envelope: &SubscriptionEnvelope,
        observed_at: impl Into<String>,
        refresh_reason: Option<QueryRefreshReason>,
        cancellation_reason: Option<QueryCancellationReason>,
        failure_reason: Option<impl Into<String>>,
    ) -> Self {
        let failure_reason = failure_reason.map(Into::into);
        let state = QueryEnvelopeState::from_envelope(
            envelope,
            cancellation_reason.is_some(),
            failure_reason.is_some(),
        );
        let (summary_count, coverage_ready, coverage_total) = payload_rollup(&envelope.payload);
        let producer = envelope.producer_refs.first();
        Self {
            record_kind: QUERY_ENVELOPE_RECORD_KIND.to_string(),
            schema_version: QUERY_ENVELOPE_ALPHA_SCHEMA_VERSION,
            query_envelope_id: format!(
                "query-envelope:{}:{}:{}:{}",
                envelope.subscription_id,
                envelope.snapshot_epoch,
                envelope.delta_seq,
                envelope.frame_class.as_str()
            ),
            subscription_id: envelope.subscription_id,
            consumer_surface: surface.as_str().to_string(),
            query_family: envelope.query_family.clone(),
            scope_class: envelope.scope_ref.class.as_str().to_string(),
            scope_id: envelope.scope_ref.id.clone(),
            state,
            state_token: state.as_str().to_string(),
            state_label: state.label().to_string(),
            snapshot_epoch: envelope.snapshot_epoch,
            delta_seq: envelope.delta_seq,
            frame_class: envelope.frame_class.as_str().to_string(),
            freshness: envelope.freshness.as_str().to_string(),
            completeness: envelope.completeness.as_str().to_string(),
            backpressure_mode: envelope.backpressure_mode.as_str().to_string(),
            producer_id: producer
                .map(|p| p.producer_id.clone())
                .unwrap_or_else(|| surface.producer_id().to_string()),
            producer_version: producer.and_then(|p| p.producer_version.clone()),
            invalidation_reason: envelope
                .invalidation
                .as_ref()
                .map(|invalidation| invalidation.stale_reason.as_str().to_string()),
            refresh_reason: refresh_reason.map(|reason| reason.as_str().to_string()),
            cancellation_reason: cancellation_reason.map(|reason| reason.as_str().to_string()),
            failure_reason,
            observed_at: observed_at.into(),
            result_count: summary_count,
            coverage_ready,
            coverage_total,
        }
    }

    /// Returns redaction-safe fields suitable for structured support logs.
    pub fn support_log_fields(&self) -> Vec<(String, String)> {
        let mut fields = vec![
            (
                "query_envelope_id".to_string(),
                self.query_envelope_id.clone(),
            ),
            (
                "consumer_surface".to_string(),
                self.consumer_surface.clone(),
            ),
            ("query_family".to_string(), self.query_family.clone()),
            ("state_token".to_string(), self.state_token.clone()),
            (
                "snapshot_epoch".to_string(),
                self.snapshot_epoch.to_string(),
            ),
            ("delta_seq".to_string(), self.delta_seq.to_string()),
        ];
        push_optional(
            &mut fields,
            "refresh_reason",
            self.refresh_reason.as_deref(),
        );
        push_optional(
            &mut fields,
            "cancellation_reason",
            self.cancellation_reason.as_deref(),
        );
        push_optional(
            &mut fields,
            "failure_reason",
            self.failure_reason.as_deref(),
        );
        push_optional(
            &mut fields,
            "invalidation_reason",
            self.invalidation_reason.as_deref(),
        );
        fields
    }
}

/// One redaction-safe support row for a query-envelope frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeSupportRow {
    /// Stable query-envelope frame identity.
    pub query_envelope_id: String,
    /// Consumer surface token.
    pub consumer_surface: String,
    /// Query family token.
    pub query_family: String,
    /// Scope class token.
    pub scope_class: String,
    /// Scope identity.
    pub scope_id: String,
    /// Query-envelope state token.
    pub state_token: String,
    /// Freshness token copied from the lower-level envelope.
    pub freshness: String,
    /// Completeness token copied from the lower-level envelope.
    pub completeness: String,
    /// Backpressure mode copied from the lower-level envelope.
    pub backpressure_mode: String,
    /// Invalidation reason when present.
    pub invalidation_reason: Option<String>,
    /// Refresh reason when present.
    pub refresh_reason: Option<String>,
    /// Cancellation reason when present.
    pub cancellation_reason: Option<String>,
    /// Failure reason when present.
    pub failure_reason: Option<String>,
    /// Stable observation timestamp supplied by the caller.
    pub observed_at: String,
}

impl QueryEnvelopeSupportRow {
    /// Materializes a support row from one query-envelope record.
    pub fn from_record(record: &QueryEnvelopeRecord) -> Self {
        Self {
            query_envelope_id: record.query_envelope_id.clone(),
            consumer_surface: record.consumer_surface.clone(),
            query_family: record.query_family.clone(),
            scope_class: record.scope_class.clone(),
            scope_id: record.scope_id.clone(),
            state_token: record.state_token.clone(),
            freshness: record.freshness.clone(),
            completeness: record.completeness.clone(),
            backpressure_mode: record.backpressure_mode.clone(),
            invalidation_reason: record.invalidation_reason.clone(),
            refresh_reason: record.refresh_reason.clone(),
            cancellation_reason: record.cancellation_reason.clone(),
            failure_reason: record.failure_reason.clone(),
            observed_at: record.observed_at.clone(),
        }
    }
}

/// Redaction-safe support artifact for query-envelope state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeSupportArtifact {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha artifact.
    pub schema_version: u32,
    /// Stable support artifact identity.
    pub artifact_id: String,
    /// Stable generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity inferred from the first workspace-scoped row.
    pub workspace_id: Option<String>,
    /// True because raw query text and result payloads are not copied into this artifact.
    pub raw_query_payload_excluded: bool,
    /// Query-envelope rows copied from live frames.
    pub rows: Vec<QueryEnvelopeSupportRow>,
}

impl QueryEnvelopeSupportArtifact {
    /// Materializes a support artifact from query-envelope records.
    pub fn from_records(
        artifact_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: &[QueryEnvelopeRecord],
    ) -> Self {
        Self {
            record_kind: QUERY_ENVELOPE_SUPPORT_ARTIFACT_RECORD_KIND.to_string(),
            schema_version: QUERY_ENVELOPE_ALPHA_SCHEMA_VERSION,
            artifact_id: artifact_id.into(),
            generated_at: generated_at.into(),
            workspace_id: records
                .iter()
                .find(|record| record.scope_class == "workspace")
                .map(|record| record.scope_id.clone()),
            raw_query_payload_excluded: true,
            rows: records
                .iter()
                .map(QueryEnvelopeSupportRow::from_record)
                .collect(),
        }
    }

    /// Returns true when refresh, cancellation, or failure reasons are visible.
    pub fn exposes_reason_tokens(&self) -> bool {
        self.rows.iter().any(|row| {
            row.refresh_reason.is_some()
                || row.cancellation_reason.is_some()
                || row.failure_reason.is_some()
        })
    }
}

/// One benchmark trace frame derived from a query-envelope record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeBenchmarkFrame {
    /// Zero-based position in the benchmark trace.
    pub event_index: u64,
    /// Stable query-envelope frame identity.
    pub query_envelope_id: String,
    /// Consumer surface token.
    pub consumer_surface: String,
    /// Query-envelope state token.
    pub state_token: String,
    /// Snapshot epoch copied from the query-envelope record.
    pub snapshot_epoch: u64,
    /// Delta sequence copied from the query-envelope record.
    pub delta_seq: u64,
    /// Frame class copied from the query-envelope record.
    pub frame_class: String,
    /// Number of results or graph/docs rows represented by this frame.
    pub result_count: u64,
    /// Number of rows currently covered by the producer.
    pub coverage_ready: u64,
    /// Total number of rows or scope units requested by the consumer.
    pub coverage_total: u64,
    /// Refresh reason when present.
    pub refresh_reason: Option<String>,
    /// Cancellation reason when present.
    pub cancellation_reason: Option<String>,
    /// Failure reason when present.
    pub failure_reason: Option<String>,
}

/// Benchmark trace backed by the same records that live consumers observe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryEnvelopeBenchmarkTrace {
    /// Stable record-kind tag for support and fixture exports.
    pub record_kind: String,
    /// Integer schema version for this alpha trace.
    pub schema_version: u32,
    /// Stable trace identity.
    pub trace_id: String,
    /// Stable generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace identity inferred from the first workspace-scoped row.
    pub workspace_id: Option<String>,
    /// Benchmark frames projected from query-envelope records.
    pub frames: Vec<QueryEnvelopeBenchmarkFrame>,
}

impl QueryEnvelopeBenchmarkTrace {
    /// Materializes a benchmark trace from query-envelope records.
    pub fn from_records(
        trace_id: impl Into<String>,
        generated_at: impl Into<String>,
        records: &[QueryEnvelopeRecord],
    ) -> Self {
        Self {
            record_kind: QUERY_ENVELOPE_BENCHMARK_TRACE_RECORD_KIND.to_string(),
            schema_version: QUERY_ENVELOPE_ALPHA_SCHEMA_VERSION,
            trace_id: trace_id.into(),
            generated_at: generated_at.into(),
            workspace_id: records
                .iter()
                .find(|record| record.scope_class == "workspace")
                .map(|record| record.scope_id.clone()),
            frames: records
                .iter()
                .enumerate()
                .map(|(event_index, record)| QueryEnvelopeBenchmarkFrame {
                    event_index: event_index as u64,
                    query_envelope_id: record.query_envelope_id.clone(),
                    consumer_surface: record.consumer_surface.clone(),
                    state_token: record.state_token.clone(),
                    snapshot_epoch: record.snapshot_epoch,
                    delta_seq: record.delta_seq,
                    frame_class: record.frame_class.clone(),
                    result_count: record.result_count,
                    coverage_ready: record.coverage_ready,
                    coverage_total: record.coverage_total,
                    refresh_reason: record.refresh_reason.clone(),
                    cancellation_reason: record.cancellation_reason.clone(),
                    failure_reason: record.failure_reason.clone(),
                })
                .collect(),
        }
    }

    /// Returns the state token sequence for compact benchmark assertions.
    pub fn state_sequence(&self) -> Vec<&str> {
        self.frames
            .iter()
            .map(|frame| frame.state_token.as_str())
            .collect()
    }
}

/// Observer callback invoked for every query-envelope record.
pub type QueryEnvelopeObserver = Rc<dyn Fn(&QueryEnvelopeRecord)>;

/// Token returned when a consumer subscribes to query-envelope records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueryEnvelopeSubscriptionToken {
    /// Lower-level subscription identity being observed.
    pub subscription_id: u64,
    /// Observer identity within the query-envelope runtime.
    pub observer_id: u64,
}

#[derive(Clone)]
struct QueryEnvelopeSubscription {
    surface: QueryConsumerSurface,
    query_family: String,
    scope_ref: ScopeRef,
    subscription_id: u64,
}

struct QueryEnvelopeObserverEntry {
    observer_id: u64,
    subscription_id: u64,
    observer: QueryEnvelopeObserver,
}

/// Live query-envelope runtime backed by [`ReactiveStore`].
pub struct LiveQueryEnvelopeRuntime {
    store: RefCell<ReactiveStore>,
    subscriptions: RefCell<Vec<QueryEnvelopeSubscription>>,
    observers: RefCell<Vec<QueryEnvelopeObserverEntry>>,
    next_observer_id: RefCell<u64>,
}

impl Default for LiveQueryEnvelopeRuntime {
    fn default() -> Self {
        Self::new()
    }
}

impl LiveQueryEnvelopeRuntime {
    /// Creates an empty query-envelope runtime.
    pub fn new() -> Self {
        Self {
            store: RefCell::new(ReactiveStore::new()),
            subscriptions: RefCell::new(Vec::new()),
            observers: RefCell::new(Vec::new()),
            next_observer_id: RefCell::new(1),
        }
    }

    /// Opens a surface subscription and returns its subscription id.
    pub fn open_surface(&self, input: QueryEnvelopeSubscriptionInput) -> Result<u64, StoreError> {
        let producer = input.to_producer();
        self.store.borrow_mut().register_producer(producer);
        let subscription_id = self.store.borrow_mut().subscribe(
            &input.query_family,
            &input.scope_ref,
            input.backpressure_mode,
        )?;
        self.subscriptions
            .borrow_mut()
            .push(QueryEnvelopeSubscription {
                surface: input.surface,
                query_family: input.query_family,
                scope_ref: input.scope_ref,
                subscription_id,
            });
        Ok(subscription_id)
    }

    /// Opens a default workspace subscription for a surface.
    pub fn open_workspace_surface(
        &self,
        surface: QueryConsumerSurface,
        workspace_id: impl Into<String>,
    ) -> Result<u64, StoreError> {
        self.open_surface(QueryEnvelopeSubscriptionInput::for_workspace(
            surface,
            workspace_id,
        ))
    }

    /// Subscribes an observer to records for one subscription id.
    pub fn subscribe(
        &self,
        subscription_id: u64,
        observer: QueryEnvelopeObserver,
    ) -> QueryEnvelopeSubscriptionToken {
        let observer_id = {
            let mut next = self.next_observer_id.borrow_mut();
            let observer_id = *next;
            *next += 1;
            observer_id
        };
        self.observers
            .borrow_mut()
            .push(QueryEnvelopeObserverEntry {
                observer_id,
                subscription_id,
                observer,
            });
        QueryEnvelopeSubscriptionToken {
            subscription_id,
            observer_id,
        }
    }

    /// Removes an observer registered by [`LiveQueryEnvelopeRuntime::subscribe`].
    pub fn unsubscribe(&self, token: QueryEnvelopeSubscriptionToken) {
        self.observers.borrow_mut().retain(|entry| {
            !(entry.subscription_id == token.subscription_id
                && entry.observer_id == token.observer_id)
        });
    }

    /// Publishes a snapshot frame and returns the projected query-envelope record.
    pub fn publish_snapshot(
        &self,
        subscription_id: u64,
        payload: QueryEnvelopePayload,
        freshness: Freshness,
        completeness: Completeness,
        refresh_reason: Option<QueryRefreshReason>,
        observed_at: impl Into<String>,
    ) -> Result<QueryEnvelopeRecord, StoreError> {
        let invalidation = refresh_reason.and_then(invalidation_for_refresh);
        let emission = self.store.borrow_mut().emit_snapshot(
            subscription_id,
            freshness,
            completeness,
            payload.to_sample_payload(),
            invalidation,
        )?;
        Ok(self.record_and_notify(
            subscription_id,
            emission,
            observed_at,
            refresh_reason,
            None,
            None::<String>,
        ))
    }

    /// Publishes a delta frame and returns the projected query-envelope record.
    pub fn publish_delta(
        &self,
        subscription_id: u64,
        payload: QueryEnvelopePayload,
        freshness: Freshness,
        completeness: Completeness,
        refresh_reason: Option<QueryRefreshReason>,
        observed_at: impl Into<String>,
    ) -> Result<QueryEnvelopeRecord, StoreError> {
        let invalidation = refresh_reason.and_then(invalidation_for_refresh);
        let emission = self.store.borrow_mut().emit_delta(
            subscription_id,
            freshness,
            completeness,
            payload.to_sample_payload(),
            invalidation,
        )?;
        Ok(self.record_and_notify(
            subscription_id,
            emission,
            observed_at,
            refresh_reason,
            None,
            None::<String>,
        ))
    }

    /// Emits a stale resync frame for an explicit refresh reason.
    pub fn request_refresh(
        &self,
        subscription_id: u64,
        refresh_reason: QueryRefreshReason,
        observed_at: impl Into<String>,
    ) -> Result<QueryEnvelopeRecord, StoreError> {
        let emission = self.store.borrow_mut().emit_resync_required(
            subscription_id,
            refresh_reason.stale_reason(),
            Some(CausedBy {
                note: Some(format!("query_refresh_reason={}", refresh_reason.as_str())),
                ..CausedBy::default()
            }),
            Completeness::Partial,
        )?;
        Ok(self.record_and_notify(
            subscription_id,
            emission,
            observed_at,
            Some(refresh_reason),
            None,
            None::<String>,
        ))
    }

    /// Emits a terminal frame for a cancelled subscription.
    pub fn cancel(
        &self,
        subscription_id: u64,
        cancellation_reason: QueryCancellationReason,
        observed_at: impl Into<String>,
    ) -> Result<QueryEnvelopeRecord, StoreError> {
        let emission = self.store.borrow_mut().emit_terminal(
            subscription_id,
            cancellation_reason.terminal_reason(),
            Freshness::Stale,
            Completeness::Unavailable,
            Some(Invalidation {
                stale_reason: cancellation_reason.stale_reason(),
                caused_by: Some(CausedBy {
                    note: Some(format!(
                        "query_cancellation_reason={}",
                        cancellation_reason.as_str()
                    )),
                    ..CausedBy::default()
                }),
            }),
        )?;
        Ok(self.record_and_notify(
            subscription_id,
            emission,
            observed_at,
            None,
            Some(cancellation_reason),
            None::<String>,
        ))
    }

    /// Emits a terminal frame for a producer failure.
    pub fn fail(
        &self,
        subscription_id: u64,
        failure_reason: impl Into<String>,
        observed_at: impl Into<String>,
    ) -> Result<QueryEnvelopeRecord, StoreError> {
        let failure_reason = failure_reason.into();
        let emission = self.store.borrow_mut().emit_terminal(
            subscription_id,
            TerminalReason::Unavailable,
            Freshness::Stale,
            Completeness::Unavailable,
            Some(Invalidation {
                stale_reason: StaleReason::CausalityLost,
                caused_by: Some(CausedBy {
                    note: Some(format!("query_failure_reason={failure_reason}")),
                    ..CausedBy::default()
                }),
            }),
        )?;
        Ok(self.record_and_notify(
            subscription_id,
            emission,
            observed_at,
            None,
            None,
            Some(failure_reason),
        ))
    }

    /// Returns the number of registered surface subscriptions.
    pub fn subscription_count(&self) -> usize {
        self.subscriptions.borrow().len()
    }

    fn record_and_notify(
        &self,
        subscription_id: u64,
        emission: Emission,
        observed_at: impl Into<String>,
        refresh_reason: Option<QueryRefreshReason>,
        cancellation_reason: Option<QueryCancellationReason>,
        failure_reason: Option<String>,
    ) -> QueryEnvelopeRecord {
        let surface = self.surface_for(subscription_id);
        let record = QueryEnvelopeRecord::from_subscription_envelope(
            surface,
            &emission.envelope,
            observed_at,
            refresh_reason,
            cancellation_reason,
            failure_reason,
        );
        for entry in self.observers.borrow().iter() {
            if entry.subscription_id == subscription_id {
                (entry.observer)(&record);
            }
        }
        record
    }

    fn surface_for(&self, subscription_id: u64) -> QueryConsumerSurface {
        self.subscriptions
            .borrow()
            .iter()
            .find(|subscription| {
                subscription.subscription_id == subscription_id
                    && !subscription.query_family.is_empty()
                    && !subscription.scope_ref.id.is_empty()
            })
            .map(|subscription| subscription.surface)
            .unwrap_or(QueryConsumerSurface::Search)
    }
}

fn invalidation_for_refresh(reason: QueryRefreshReason) -> Option<Invalidation> {
    if matches!(reason, QueryRefreshReason::InitialSubscribe) {
        return None;
    }
    Some(Invalidation {
        stale_reason: reason.stale_reason(),
        caused_by: Some(CausedBy {
            note: Some(format!("query_refresh_reason={}", reason.as_str())),
            ..CausedBy::default()
        }),
    })
}

fn push_optional(fields: &mut Vec<(String, String)>, key: &str, value: Option<&str>) {
    if let Some(value) = value {
        fields.push((key.to_string(), value.to_string()));
    }
}

fn payload_rollup(payload: &Option<JsonValue>) -> (u64, u64, u64) {
    let Some(JsonValue::Object(entries)) = payload else {
        return (0, 0, 0);
    };
    let result_count = object_u64(entries, "entry_count").unwrap_or(0);
    let (coverage_ready, coverage_total) = entries
        .iter()
        .find_map(|(key, value)| {
            if key != "coverage" {
                return None;
            }
            match value {
                JsonValue::Object(coverage) => Some((
                    object_u64(coverage, "ready").unwrap_or(0),
                    object_u64(coverage, "total").unwrap_or(0),
                )),
                _ => None,
            }
        })
        .unwrap_or((0, 0));
    (result_count, coverage_ready, coverage_total)
}

fn object_u64(entries: &[(String, JsonValue)], key: &str) -> Option<u64> {
    entries
        .iter()
        .find(|(candidate, _)| candidate == key)
        .and_then(|(_, value)| match value {
            JsonValue::U64(value) => Some(*value),
            _ => None,
        })
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize)]
    struct ExpectedRecord {
        query_envelope_id: String,
        subscription_id: u64,
        consumer_surface: String,
        query_family: String,
        scope_ref: ExpectedScopeRef,
        state_token: String,
        snapshot_epoch: u64,
        delta_seq: u64,
        frame_class: String,
        freshness: String,
        completeness: String,
        backpressure_mode: String,
        producer_id: String,
        producer_version: Option<String>,
        invalidation_reason: Option<String>,
        refresh_reason: Option<String>,
        cancellation_reason: Option<String>,
        failure_reason: Option<String>,
        observed_at: String,
        result_count: u64,
        coverage: ExpectedCoverage,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedScopeRef {
        class: String,
        id: String,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedCoverage {
        ready: u64,
        total: u64,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedSupportArtifact {
        record_kind: String,
        schema_version: u32,
        artifact_id: String,
        generated_at: String,
        workspace_id: Option<String>,
        raw_query_payload_excluded: bool,
        rows: Vec<ExpectedSupportRow>,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedSupportRow {
        query_envelope_id: String,
        consumer_surface: String,
        query_family: String,
        scope_ref: ExpectedScopeRef,
        state_token: String,
        freshness: String,
        completeness: String,
        backpressure_mode: String,
        invalidation_reason: Option<String>,
        refresh_reason: Option<String>,
        cancellation_reason: Option<String>,
        failure_reason: Option<String>,
        observed_at: String,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedBenchmarkTrace {
        record_kind: String,
        schema_version: u32,
        trace_id: String,
        generated_at: String,
        workspace_id: Option<String>,
        frames: Vec<ExpectedBenchmarkFrame>,
    }

    #[derive(Debug, Deserialize)]
    struct ExpectedBenchmarkFrame {
        event_index: u64,
        query_envelope_id: String,
        consumer_surface: String,
        state_token: String,
        snapshot_epoch: u64,
        delta_seq: u64,
        frame_class: String,
        result_count: u64,
        coverage: ExpectedCoverage,
        refresh_reason: Option<String>,
        cancellation_reason: Option<String>,
        failure_reason: Option<String>,
    }

    #[test]
    fn surfaces_observe_one_state_vocabulary() {
        let records = fixture_records();
        let states: Vec<&str> = records
            .iter()
            .map(|record| record.state_token.as_str())
            .collect();
        assert_eq!(
            states,
            vec!["ready", "partial", "warming", "stale", "failed"]
        );
        assert_eq!(records[0].consumer_surface, "search");
        assert_eq!(records[2].consumer_surface, "graph");
        assert_eq!(records[4].consumer_surface, "docs");
    }

    #[test]
    fn observers_receive_live_records() {
        let runtime = LiveQueryEnvelopeRuntime::new();
        let search_id = runtime
            .open_workspace_surface(QueryConsumerSurface::Search, "ws-alpha")
            .unwrap();
        let observed = Rc::new(RefCell::new(Vec::new()));
        let observed_for_callback = observed.clone();
        let token = runtime.subscribe(
            search_id,
            Rc::new(move |record| {
                observed_for_callback
                    .borrow_mut()
                    .push(record.state_token.clone());
            }),
        );
        runtime
            .publish_snapshot(
                search_id,
                payload("ready search rows", 12, 12, 12),
                Freshness::Authoritative,
                Completeness::Full,
                Some(QueryRefreshReason::InitialSubscribe),
                "mono:query:observer-1",
            )
            .unwrap();
        runtime.unsubscribe(token);
        runtime
            .publish_delta(
                search_id,
                payload("partial search rows", 16, 16, 24),
                Freshness::Authoritative,
                Completeness::Partial,
                None,
                "mono:query:observer-2",
            )
            .unwrap();
        assert_eq!(observed.borrow().as_slice(), ["ready"]);
    }

    #[test]
    fn support_artifact_exposes_refresh_and_cancellation_reasons() {
        let records = fixture_records();
        let artifact = QueryEnvelopeSupportArtifact::from_records(
            "artifact:query-envelope:alpha",
            "mono:query:artifact",
            &records,
        );
        assert!(artifact.exposes_reason_tokens());
        assert_eq!(artifact.workspace_id.as_deref(), Some("ws-alpha"));
        assert!(artifact
            .rows
            .iter()
            .any(|row| row.refresh_reason.as_deref() == Some("graph_epoch_advanced")));
        assert!(artifact
            .rows
            .iter()
            .any(|row| { row.cancellation_reason.as_deref() == Some("superseded_by_new_query") }));
        assert!(artifact.raw_query_payload_excluded);
    }

    #[test]
    fn benchmark_trace_reuses_live_records() {
        let records = fixture_records();
        let trace = QueryEnvelopeBenchmarkTrace::from_records(
            "trace:query-envelope:alpha",
            "mono:query:trace",
            &records,
        );
        assert_eq!(
            trace.state_sequence(),
            vec!["ready", "partial", "warming", "stale", "failed"]
        );
        assert_eq!(trace.frames[1].frame_class, "delta");
        assert_eq!(
            trace.frames[3].refresh_reason.as_deref(),
            Some("graph_epoch_advanced")
        );
    }

    #[test]
    fn fixtures_match_runtime_records() {
        let records = fixture_records();
        let fixtures = [
            (
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../fixtures/search/query_envelope_alpha/search_ready_snapshot.json"
                ),
                &records[0],
            ),
            (
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../fixtures/search/query_envelope_alpha/search_partial_delta.json"
                ),
                &records[1],
            ),
            (
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../fixtures/search/query_envelope_alpha/graph_warming_snapshot.json"
                ),
                &records[2],
            ),
            (
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../fixtures/search/query_envelope_alpha/graph_refresh_stale.json"
                ),
                &records[3],
            ),
            (
                concat!(
                    env!("CARGO_MANIFEST_DIR"),
                    "/../../fixtures/search/query_envelope_alpha/docs_cancelled_failed.json"
                ),
                &records[4],
            ),
        ];

        for (path, record) in fixtures {
            let json = std::fs::read_to_string(path)
                .unwrap_or_else(|err| panic!("failed to read fixture {path}: {err}"));
            let expected: ExpectedRecord = serde_json::from_str(&json)
                .unwrap_or_else(|err| panic!("failed to parse fixture {path}: {err}"));
            assert_record_matches_fixture(record, &expected);
        }
    }

    #[test]
    fn support_and_benchmark_fixtures_match_runtime_records() {
        let records = fixture_records();
        let support = QueryEnvelopeSupportArtifact::from_records(
            "artifact:query-envelope:alpha",
            "mono:query:artifact",
            &records,
        );
        let trace = QueryEnvelopeBenchmarkTrace::from_records(
            "trace:query-envelope:alpha",
            "mono:query:trace",
            &records,
        );

        let support_json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/search/query_envelope_alpha/support_artifact.json"
        ))
        .expect("support artifact fixture is readable");
        let expected_support: ExpectedSupportArtifact =
            serde_json::from_str(&support_json).expect("support artifact fixture parses");
        assert_support_matches_fixture(&support, &expected_support);

        let trace_json = std::fs::read_to_string(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/search/query_envelope_alpha/benchmark_trace.json"
        ))
        .expect("benchmark trace fixture is readable");
        let expected_trace: ExpectedBenchmarkTrace =
            serde_json::from_str(&trace_json).expect("benchmark trace fixture parses");
        assert_trace_matches_fixture(&trace, &expected_trace);
    }

    fn fixture_records() -> Vec<QueryEnvelopeRecord> {
        let runtime = LiveQueryEnvelopeRuntime::new();
        let search_id = runtime
            .open_workspace_surface(QueryConsumerSurface::Search, "ws-alpha")
            .unwrap();
        let graph_id = runtime
            .open_workspace_surface(QueryConsumerSurface::Graph, "ws-alpha")
            .unwrap();
        let docs_id = runtime
            .open_workspace_surface(QueryConsumerSurface::Docs, "ws-alpha")
            .unwrap();
        assert_eq!(runtime.subscription_count(), 3);

        vec![
            runtime
                .publish_snapshot(
                    search_id,
                    payload("ready search rows", 12, 12, 12),
                    Freshness::Authoritative,
                    Completeness::Full,
                    Some(QueryRefreshReason::InitialSubscribe),
                    "mono:query:0001",
                )
                .unwrap(),
            runtime
                .publish_delta(
                    search_id,
                    payload("partial search rows", 16, 16, 24),
                    Freshness::Authoritative,
                    Completeness::Partial,
                    None,
                    "mono:query:0002",
                )
                .unwrap(),
            runtime
                .publish_snapshot(
                    graph_id,
                    payload("graph warming", 0, 0, 8),
                    Freshness::Warming,
                    Completeness::Unloaded,
                    Some(QueryRefreshReason::InitialSubscribe),
                    "mono:query:0003",
                )
                .unwrap(),
            runtime
                .request_refresh(
                    graph_id,
                    QueryRefreshReason::GraphEpochAdvanced,
                    "mono:query:0004",
                )
                .unwrap(),
            runtime
                .cancel(
                    docs_id,
                    QueryCancellationReason::SupersededByNewQuery,
                    "mono:query:0005",
                )
                .unwrap(),
        ]
    }

    fn payload(
        summary: impl Into<String>,
        result_count: u64,
        coverage_ready: u64,
        coverage_total: u64,
    ) -> QueryEnvelopePayload {
        QueryEnvelopePayload {
            summary: summary.into(),
            result_count,
            coverage_ready,
            coverage_total,
            detail_lines: Vec::new(),
        }
    }

    fn assert_record_matches_fixture(record: &QueryEnvelopeRecord, expected: &ExpectedRecord) {
        assert_eq!(record.query_envelope_id, expected.query_envelope_id);
        assert_eq!(record.subscription_id, expected.subscription_id);
        assert_eq!(record.consumer_surface, expected.consumer_surface);
        assert_eq!(record.query_family, expected.query_family);
        assert_eq!(record.scope_class, expected.scope_ref.class);
        assert_eq!(record.scope_id, expected.scope_ref.id);
        assert_eq!(record.state_token, expected.state_token);
        assert_eq!(record.snapshot_epoch, expected.snapshot_epoch);
        assert_eq!(record.delta_seq, expected.delta_seq);
        assert_eq!(record.frame_class, expected.frame_class);
        assert_eq!(record.freshness, expected.freshness);
        assert_eq!(record.completeness, expected.completeness);
        assert_eq!(record.backpressure_mode, expected.backpressure_mode);
        assert_eq!(record.producer_id, expected.producer_id);
        assert_eq!(record.producer_version, expected.producer_version);
        assert_eq!(record.invalidation_reason, expected.invalidation_reason);
        assert_eq!(record.refresh_reason, expected.refresh_reason);
        assert_eq!(record.cancellation_reason, expected.cancellation_reason);
        assert_eq!(record.failure_reason, expected.failure_reason);
        assert_eq!(record.observed_at, expected.observed_at);
        assert_eq!(record.result_count, expected.result_count);
        assert_eq!(record.coverage_ready, expected.coverage.ready);
        assert_eq!(record.coverage_total, expected.coverage.total);
    }

    fn assert_support_matches_fixture(
        support: &QueryEnvelopeSupportArtifact,
        expected: &ExpectedSupportArtifact,
    ) {
        assert_eq!(support.record_kind, expected.record_kind);
        assert_eq!(support.schema_version, expected.schema_version);
        assert_eq!(support.artifact_id, expected.artifact_id);
        assert_eq!(support.generated_at, expected.generated_at);
        assert_eq!(support.workspace_id, expected.workspace_id);
        assert_eq!(
            support.raw_query_payload_excluded,
            expected.raw_query_payload_excluded
        );
        assert_eq!(support.rows.len(), expected.rows.len());
        for (row, expected) in support.rows.iter().zip(expected.rows.iter()) {
            assert_eq!(row.query_envelope_id, expected.query_envelope_id);
            assert_eq!(row.consumer_surface, expected.consumer_surface);
            assert_eq!(row.query_family, expected.query_family);
            assert_eq!(row.scope_class, expected.scope_ref.class);
            assert_eq!(row.scope_id, expected.scope_ref.id);
            assert_eq!(row.state_token, expected.state_token);
            assert_eq!(row.freshness, expected.freshness);
            assert_eq!(row.completeness, expected.completeness);
            assert_eq!(row.backpressure_mode, expected.backpressure_mode);
            assert_eq!(row.invalidation_reason, expected.invalidation_reason);
            assert_eq!(row.refresh_reason, expected.refresh_reason);
            assert_eq!(row.cancellation_reason, expected.cancellation_reason);
            assert_eq!(row.failure_reason, expected.failure_reason);
            assert_eq!(row.observed_at, expected.observed_at);
        }
    }

    fn assert_trace_matches_fixture(
        trace: &QueryEnvelopeBenchmarkTrace,
        expected: &ExpectedBenchmarkTrace,
    ) {
        assert_eq!(trace.record_kind, expected.record_kind);
        assert_eq!(trace.schema_version, expected.schema_version);
        assert_eq!(trace.trace_id, expected.trace_id);
        assert_eq!(trace.generated_at, expected.generated_at);
        assert_eq!(trace.workspace_id, expected.workspace_id);
        assert_eq!(trace.frames.len(), expected.frames.len());
        for (frame, expected) in trace.frames.iter().zip(expected.frames.iter()) {
            assert_eq!(frame.event_index, expected.event_index);
            assert_eq!(frame.query_envelope_id, expected.query_envelope_id);
            assert_eq!(frame.consumer_surface, expected.consumer_surface);
            assert_eq!(frame.state_token, expected.state_token);
            assert_eq!(frame.snapshot_epoch, expected.snapshot_epoch);
            assert_eq!(frame.delta_seq, expected.delta_seq);
            assert_eq!(frame.frame_class, expected.frame_class);
            assert_eq!(frame.result_count, expected.result_count);
            assert_eq!(frame.coverage_ready, expected.coverage.ready);
            assert_eq!(frame.coverage_total, expected.coverage.total);
            assert_eq!(frame.refresh_reason, expected.refresh_reason);
            assert_eq!(frame.cancellation_reason, expected.cancellation_reason);
            assert_eq!(frame.failure_reason, expected.failure_reason);
        }
    }
}
