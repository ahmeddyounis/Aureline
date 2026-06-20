//! Canonical M5 reactive-state, subscription-envelope, and
//! materialized-view governance matrix.
//!
//! This module freezes one typed contract for every reactive M5
//! surface — shell, search, graph, docs, AI, review, preview,
//! companion, policy/trust, editor-adjacent, headless mirror, and
//! support/export — so those surfaces stop growing private caches,
//! private epochs, and private stale-state language.
//!
//! The matrix is grounded in the authoritative source documents:
//!
//! - the authority-class, subscription-envelope, and
//!   materialized-view tables in Appendix DB of the technical
//!   architecture document (`DB.1`/`DB.2`/`DB.3`);
//! - the frozen subscription-envelope vocabulary in
//!   [`crate::envelope`], which this matrix mirrors token-for-token
//!   (`authority_class`, `freshness`, `completeness`,
//!   `backpressure_mode`, `view_class`, invalidation reasons, and
//!   terminal reasons) so the two never drift.
//!
//! Where [`crate::envelope`] models a single subscription *frame*,
//! this module models the *governance matrix*: one row per reactive
//! surface that declares which authority owns its truth, which
//! materialized-view class it is, which freshness / completeness /
//! backpressure states it can present, and — critically — how a
//! presented [`TruthClaim`] is automatically narrowed when the
//! surface outruns its authoritative epoch, scope, or invalidation
//! guarantee. The narrowing function is the single source of truth
//! release and support tooling use to detect underqualified rows and
//! downgrade claims without surface-specific prose.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/m5_reactive_governance.schema.json`](../../../../schemas/state/m5_reactive_governance.schema.json)
//! - [`/docs/state/m5_reactive_governance.md`](../../../../docs/state/m5_reactive_governance.md)
//! - [`/artifacts/state/m5_reactive_governance.json`](../../../../artifacts/state/m5_reactive_governance.json)
//! - [`/artifacts/state/m5_reactive_governance.md`](../../../../artifacts/state/m5_reactive_governance.md)
//! - [`/fixtures/state/m5_reactive_governance/`](../../../../fixtures/state/m5_reactive_governance/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const M5_REACTIVE_GOVERNANCE_PACKET_RECORD_KIND: &str = "m5_reactive_governance_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const M5_REACTIVE_GOVERNANCE_FIXTURE_RECORD_KIND: &str =
    "m5_reactive_governance_fixture_record";

/// Repo-relative schema ref.
pub const M5_REACTIVE_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/state/m5_reactive_governance.schema.json";

/// Repo-relative reviewer doc ref.
pub const M5_REACTIVE_GOVERNANCE_DOC_REF: &str = "docs/state/m5_reactive_governance.md";

/// Repo-relative machine-readable artifact packet.
pub const M5_REACTIVE_GOVERNANCE_PACKET_REF: &str = "artifacts/state/m5_reactive_governance.json";

/// Repo-relative reviewer artifact report.
pub const M5_REACTIVE_GOVERNANCE_REPORT_REF: &str = "artifacts/state/m5_reactive_governance.md";

/// Repo-relative fixture directory.
pub const M5_REACTIVE_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/state/m5_reactive_governance";

/// Repo-relative fixture manifest.
pub const M5_REACTIVE_GOVERNANCE_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/m5_reactive_governance/manifest.yaml";

// ---------------------------------------------------------------------------
// Mirrored subscription-envelope vocabulary (serde-friendly).
//
// Token strings here MUST equal the `as_str` tokens emitted by the
// matching enums in `crate::envelope`. The `vocabulary_matches_envelope`
// test in `tests/m5_reactive_governance.rs` asserts that parity so the
// matrix cannot silently fork the ADR vocabulary.
// ---------------------------------------------------------------------------

/// Authority class that owns the canonical truth for a surface.
///
/// Mirrors Appendix DB.1 and [`crate::envelope::AuthorityClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    WorkspaceVfs,
    BufferEditor,
    DerivedKnowledge,
    Execution,
    PolicyEntitlement,
    ProviderOverlay,
}

impl AuthorityClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceVfs => "workspace_vfs",
            Self::BufferEditor => "buffer_editor",
            Self::DerivedKnowledge => "derived_knowledge",
            Self::Execution => "execution",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Whether a surface presents authoritative truth or a derived
/// projection. Derived surfaces may never claim exact current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivationClass {
    Authoritative,
    Derived,
}

impl DerivationClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Derived => "derived",
        }
    }
}

/// Frozen freshness vocabulary. Mirrors [`crate::envelope::Freshness`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Freshness {
    Authoritative,
    Warming,
    Cached,
    Stale,
    Replayed,
    Imported,
}

impl Freshness {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Warming => "warming",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Replayed => "replayed",
            Self::Imported => "imported",
        }
    }
}

/// Frozen completeness vocabulary. Mirrors [`crate::envelope::Completeness`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Completeness {
    Full,
    Partial,
    Unloaded,
    Unavailable,
}

impl Completeness {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Partial => "partial",
            Self::Unloaded => "unloaded",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Frozen backpressure-mode vocabulary. Mirrors
/// [`crate::envelope::BackpressureMode`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackpressureMode {
    Realtime,
    Coalesced,
    SnapshotRequired,
}

impl BackpressureMode {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Realtime => "realtime",
            Self::Coalesced => "coalesced",
            Self::SnapshotRequired => "snapshot_required",
        }
    }
}

/// Frozen materialized-view-class vocabulary. Mirrors Appendix DB.3
/// and [`crate::envelope::ViewClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewClass {
    EphemeralProjection,
    DurableLocalMaterialization,
    ExportableSnapshot,
    ManagedReplicatedView,
}

impl ViewClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralProjection => "ephemeral_projection",
            Self::DurableLocalMaterialization => "durable_local_materialization",
            Self::ExportableSnapshot => "exportable_snapshot",
            Self::ManagedReplicatedView => "managed_replicated_view",
        }
    }
}

/// Frozen invalidation-reason vocabulary. Mirrors
/// [`crate::envelope::StaleReason`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationReason {
    ProducerRestart,
    AuthorityEpochRolled,
    PolicyEpochChanged,
    WatcherDropped,
    QueueSaturation,
    UpstreamInputStale,
    ExplicitRefreshRequested,
    CacheServed,
    ReplayedFromBundle,
    ImportedFromExternal,
    ScopeRemoved,
    CausalityLost,
}

impl InvalidationReason {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProducerRestart => "producer_restart",
            Self::AuthorityEpochRolled => "authority_epoch_rolled",
            Self::PolicyEpochChanged => "policy_epoch_changed",
            Self::WatcherDropped => "watcher_dropped",
            Self::QueueSaturation => "queue_saturation",
            Self::UpstreamInputStale => "upstream_input_stale",
            Self::ExplicitRefreshRequested => "explicit_refresh_requested",
            Self::CacheServed => "cache_served",
            Self::ReplayedFromBundle => "replayed_from_bundle",
            Self::ImportedFromExternal => "imported_from_external",
            Self::ScopeRemoved => "scope_removed",
            Self::CausalityLost => "causality_lost",
        }
    }
}

/// Frozen terminal-reason vocabulary. Mirrors
/// [`crate::envelope::TerminalReason`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReason {
    ConsumerCancelled,
    ProducerShutdown,
    ScopeRemoved,
    PolicyTerminated,
    Unavailable,
}

impl TerminalReason {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerCancelled => "consumer_cancelled",
            Self::ProducerShutdown => "producer_shutdown",
            Self::ScopeRemoved => "scope_removed",
            Self::PolicyTerminated => "policy_terminated",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Frozen scope-class vocabulary. Mirrors [`crate::envelope::ScopeClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    Workspace,
    Window,
    ReviewWorkspace,
    RemoteSession,
    Tenant,
    CompanionSurface,
}

impl ScopeClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Window => "window",
            Self::ReviewWorkspace => "review_workspace",
            Self::RemoteSession => "remote_session",
            Self::Tenant => "tenant",
            Self::CompanionSurface => "companion_surface",
        }
    }
}

// ---------------------------------------------------------------------------
// M5-specific governance vocabulary.
// ---------------------------------------------------------------------------

/// One reactive M5 surface governed by the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReactiveSurfaceClass {
    ShellWorkspaceTree,
    ShellActivityCenter,
    EditorBufferOutline,
    SearchResults,
    GraphNeighborhood,
    DocsBrowser,
    AiContextPanel,
    ReviewWorkspace,
    PreviewOutput,
    CompanionPanel,
    PolicyTrustBanner,
    HeadlessWorkspaceMirror,
    SupportExportView,
}

impl ReactiveSurfaceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellWorkspaceTree => "shell_workspace_tree",
            Self::ShellActivityCenter => "shell_activity_center",
            Self::EditorBufferOutline => "editor_buffer_outline",
            Self::SearchResults => "search_results",
            Self::GraphNeighborhood => "graph_neighborhood",
            Self::DocsBrowser => "docs_browser",
            Self::AiContextPanel => "ai_context_panel",
            Self::ReviewWorkspace => "review_workspace",
            Self::PreviewOutput => "preview_output",
            Self::CompanionPanel => "companion_panel",
            Self::PolicyTrustBanner => "policy_trust_banner",
            Self::HeadlessWorkspaceMirror => "headless_workspace_mirror",
            Self::SupportExportView => "support_export_view",
        }
    }
}

/// Presentation channel a reactive surface can render through. The
/// matrix as a whole must cover all four so UI, CLI/headless, export,
/// and release surfaces share one stale-state grammar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationChannel {
    Ui,
    CliHeadless,
    Export,
    Release,
}

impl PresentationChannel {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ui => "ui",
            Self::CliHeadless => "cli_headless",
            Self::Export => "export",
            Self::Release => "release",
        }
    }
}

/// The strongest factual claim a surface is allowed to present given a
/// subscription state. Lower [`TruthClaim::strength`] is a narrower,
/// more honest claim; narrowing always moves toward lower strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthClaim {
    /// Reserved ceiling for the owning authority only. A derived M5
    /// surface may never present this.
    ExactCurrentTruth,
    /// A consistent snapshot at the surface's authoritative epoch.
    ConsistentSnapshot,
    /// Updates are coalesced; the stream lags the producer delta rate.
    CoalescedStream,
    /// Scope is only partially loaded.
    PartialProjection,
    /// Nothing authoritative has been observed yet for the scope.
    WarmingNoTruthYet,
    /// Served from a local cache, not a live producer.
    CachedProjection,
    /// A known-stale snapshot behind the current authoritative epoch.
    StaleSnapshot,
    /// Replayed from a captured bundle, not a live producer.
    ReplayedSnapshot,
    /// Imported from an external source, not a live producer.
    ImportedSnapshot,
    /// A policy- or entitlement-limited projection of the truth.
    PolicyLimitedProjection,
    /// The backing producer is unavailable; no current truth is known.
    ProviderUnavailable,
}

impl TruthClaim {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCurrentTruth => "exact_current_truth",
            Self::ConsistentSnapshot => "consistent_snapshot",
            Self::CoalescedStream => "coalesced_stream",
            Self::PartialProjection => "partial_projection",
            Self::WarmingNoTruthYet => "warming_no_truth_yet",
            Self::CachedProjection => "cached_projection",
            Self::StaleSnapshot => "stale_snapshot",
            Self::ReplayedSnapshot => "replayed_snapshot",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::PolicyLimitedProjection => "policy_limited_projection",
            Self::ProviderUnavailable => "provider_unavailable",
        }
    }

    /// Relative confidence of the claim. Lower is narrower / more
    /// honest; the narrowing function always selects the lowest
    /// strength among the triggered candidates.
    pub const fn strength(self) -> u8 {
        match self {
            Self::ProviderUnavailable => 0,
            Self::PolicyLimitedProjection => 1,
            Self::ImportedSnapshot => 2,
            Self::ReplayedSnapshot => 3,
            Self::StaleSnapshot => 4,
            Self::CachedProjection => 5,
            Self::WarmingNoTruthYet => 6,
            Self::PartialProjection => 7,
            Self::CoalescedStream => 8,
            Self::ConsistentSnapshot => 9,
            Self::ExactCurrentTruth => 10,
        }
    }
}

/// One axis along which an observed subscription state forces a
/// reactive surface to narrow its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingTrigger {
    FreshnessWarming,
    FreshnessCached,
    FreshnessStale,
    FreshnessReplayed,
    FreshnessImported,
    CompletenessPartial,
    CompletenessUnloaded,
    CompletenessUnavailable,
    BackpressureCoalesced,
    BackpressureSnapshotRequired,
    TerminalUnavailable,
    TerminalTerminated,
    PolicyLimited,
}

impl NarrowingTrigger {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshnessWarming => "freshness_warming",
            Self::FreshnessCached => "freshness_cached",
            Self::FreshnessStale => "freshness_stale",
            Self::FreshnessReplayed => "freshness_replayed",
            Self::FreshnessImported => "freshness_imported",
            Self::CompletenessPartial => "completeness_partial",
            Self::CompletenessUnloaded => "completeness_unloaded",
            Self::CompletenessUnavailable => "completeness_unavailable",
            Self::BackpressureCoalesced => "backpressure_coalesced",
            Self::BackpressureSnapshotRequired => "backpressure_snapshot_required",
            Self::TerminalUnavailable => "terminal_unavailable",
            Self::TerminalTerminated => "terminal_terminated",
            Self::PolicyLimited => "policy_limited",
        }
    }
}

/// Persistence semantics for a materialized view (Appendix DB.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClass {
    MemoryOnly,
    LocalCacheOrDb,
    SavedArtifact,
    ServiceOrLocalMirror,
}

impl PersistenceClass {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MemoryOnly => "memory_only",
            Self::LocalCacheOrDb => "local_cache_or_db",
            Self::SavedArtifact => "saved_artifact",
            Self::ServiceOrLocalMirror => "service_or_local_mirror",
        }
    }
}

/// Deletion semantics for a materialized view (Appendix DB.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteSemantics {
    EvictOnScopeChange,
    ClearOrRebuild,
    ReplacedByNewSnapshot,
    ReconcileOnReconnect,
}

impl DeleteSemantics {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvictOnScopeChange => "evict_on_scope_change",
            Self::ClearOrRebuild => "clear_or_rebuild",
            Self::ReplacedByNewSnapshot => "replaced_by_new_snapshot",
            Self::ReconcileOnReconnect => "reconcile_on_reconnect",
        }
    }
}

// ---------------------------------------------------------------------------
// Narrowing engine: the single source of truth for claim downgrade.
// ---------------------------------------------------------------------------

/// One observed subscription state for a reactive surface. This is the
/// minimal projection of [`crate::envelope::SubscriptionEnvelope`] the
/// narrowing engine needs to decide what the surface may claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ObservedReactiveState {
    /// Observed freshness of the frame.
    pub freshness: Freshness,
    /// Observed completeness of the scope.
    pub completeness: Completeness,
    /// Observed backpressure mode of the stream.
    pub backpressure_mode: BackpressureMode,
    /// Terminal reason, when the subscription has ended.
    pub terminal_reason: Option<TerminalReason>,
    /// Whether policy / entitlement limits what truth is visible.
    pub policy_limited: bool,
}

impl ObservedReactiveState {
    /// A fully-healthy observed state: authoritative, full, realtime,
    /// live, and not policy-limited.
    pub const fn healthy() -> Self {
        Self {
            freshness: Freshness::Authoritative,
            completeness: Completeness::Full,
            backpressure_mode: BackpressureMode::Realtime,
            terminal_reason: None,
            policy_limited: false,
        }
    }
}

/// Result of narrowing a presented claim against an observed state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NarrowedClaim {
    /// The narrowest claim the surface may present.
    pub claim: TruthClaim,
    /// Every trigger that forced narrowing, in stable order. Empty
    /// when the observed state is fully healthy.
    pub triggers: Vec<NarrowingTrigger>,
}

fn narrowing_candidates(observed: &ObservedReactiveState) -> Vec<(TruthClaim, NarrowingTrigger)> {
    let mut out = Vec::new();
    match observed.freshness {
        Freshness::Authoritative => {}
        Freshness::Warming => {
            out.push((
                TruthClaim::WarmingNoTruthYet,
                NarrowingTrigger::FreshnessWarming,
            ));
        }
        Freshness::Cached => {
            out.push((
                TruthClaim::CachedProjection,
                NarrowingTrigger::FreshnessCached,
            ));
        }
        Freshness::Stale => {
            out.push((TruthClaim::StaleSnapshot, NarrowingTrigger::FreshnessStale));
        }
        Freshness::Replayed => {
            out.push((
                TruthClaim::ReplayedSnapshot,
                NarrowingTrigger::FreshnessReplayed,
            ));
        }
        Freshness::Imported => {
            out.push((
                TruthClaim::ImportedSnapshot,
                NarrowingTrigger::FreshnessImported,
            ));
        }
    }
    match observed.completeness {
        Completeness::Full => {}
        Completeness::Partial => {
            out.push((
                TruthClaim::PartialProjection,
                NarrowingTrigger::CompletenessPartial,
            ));
        }
        Completeness::Unloaded => {
            out.push((
                TruthClaim::WarmingNoTruthYet,
                NarrowingTrigger::CompletenessUnloaded,
            ));
        }
        Completeness::Unavailable => {
            out.push((
                TruthClaim::ProviderUnavailable,
                NarrowingTrigger::CompletenessUnavailable,
            ));
        }
    }
    match observed.backpressure_mode {
        BackpressureMode::Realtime => {}
        BackpressureMode::Coalesced => {
            out.push((
                TruthClaim::CoalescedStream,
                NarrowingTrigger::BackpressureCoalesced,
            ));
        }
        BackpressureMode::SnapshotRequired => {
            out.push((
                TruthClaim::CoalescedStream,
                NarrowingTrigger::BackpressureSnapshotRequired,
            ));
        }
    }
    if let Some(reason) = observed.terminal_reason {
        match reason {
            TerminalReason::Unavailable => {
                out.push((
                    TruthClaim::ProviderUnavailable,
                    NarrowingTrigger::TerminalUnavailable,
                ));
            }
            _ => {
                out.push((
                    TruthClaim::ProviderUnavailable,
                    NarrowingTrigger::TerminalTerminated,
                ));
            }
        }
    }
    if observed.policy_limited {
        out.push((
            TruthClaim::PolicyLimitedProjection,
            NarrowingTrigger::PolicyLimited,
        ));
    }
    out
}

/// Narrows the claim a surface of the given derivation class may
/// present, given an observed subscription state.
///
/// This is the canonical downgrade rule the whole matrix, every real
/// consumer, and release/support tooling share. A derived surface
/// starts from [`TruthClaim::ConsistentSnapshot`] (never
/// [`TruthClaim::ExactCurrentTruth`]); every observed degradation
/// contributes a candidate claim, and the narrowest (lowest-strength)
/// candidate wins. The returned [`NarrowedClaim::triggers`] names every
/// degradation that fired so support and docs surfaces can explain the
/// downgrade without inventing local prose.
pub fn narrow_truth_claim(
    derivation: DerivationClass,
    observed: &ObservedReactiveState,
) -> NarrowedClaim {
    let baseline = match derivation {
        DerivationClass::Authoritative => TruthClaim::ExactCurrentTruth,
        DerivationClass::Derived => TruthClaim::ConsistentSnapshot,
    };
    let candidates = narrowing_candidates(observed);
    let mut claim = baseline;
    for (candidate, _) in &candidates {
        if candidate.strength() < claim.strength() {
            claim = *candidate;
        }
    }
    let mut triggers: Vec<NarrowingTrigger> = candidates.iter().map(|(_, t)| *t).collect();
    triggers.sort();
    triggers.dedup();
    NarrowedClaim { claim, triggers }
}

// ---------------------------------------------------------------------------
// Packet structures.
// ---------------------------------------------------------------------------

/// One declared narrowing rule on a reactive surface row. The rule set
/// is computed from the surface's supported state sets and the
/// canonical [`narrow_truth_claim`] engine, so it can never drift from
/// the engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimNarrowingRule {
    /// The single-axis degradation that triggers this rule.
    pub trigger: NarrowingTrigger,
    /// The claim the surface narrows to under this trigger alone.
    pub narrowed_claim: TruthClaim,
}

/// One reactive M5 surface governed by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveSurfaceRow {
    /// Reactive surface class.
    pub surface_class: ReactiveSurfaceClass,
    /// Canonical query family the surface subscribes to.
    pub query_family: String,
    /// Authority class that owns the canonical truth.
    pub authority_class: AuthorityClass,
    /// Whether the surface is authoritative or a derived projection.
    pub derivation_class: DerivationClass,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Materialized-view class for this surface.
    pub view_class: ViewClass,
    /// Default backpressure mode for the subscription.
    pub default_backpressure_mode: BackpressureMode,
    /// Presentation channels that render this surface.
    pub presentation_channels: Vec<PresentationChannel>,
    /// Freshness states the surface can present.
    pub supported_freshness: Vec<Freshness>,
    /// Completeness states the surface can present.
    pub supported_completeness: Vec<Completeness>,
    /// Backpressure modes the surface can experience.
    pub supported_backpressure: Vec<BackpressureMode>,
    /// Whether the backing producer can go terminally unavailable.
    pub supports_terminal_unavailable: bool,
    /// Whether policy / entitlement can limit the visible projection.
    pub supports_policy_limited: bool,
    /// Invalidation reasons the surface honors.
    pub honored_invalidation_reasons: Vec<InvalidationReason>,
    /// The strongest claim the surface may present when fully healthy.
    pub healthy_claim: TruthClaim,
    /// Computed narrowing rules over the supported state sets.
    pub claim_narrowing_rules: Vec<ClaimNarrowingRule>,
    /// Real consumer surfaces that ingest this row.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One cross-surface epoch-parity group. All surfaces sharing an
/// authority class read the same authoritative epoch; a member that
/// lags MUST narrow rather than present a parallel epoch as truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochParityGroup {
    /// Stable group id.
    pub group_id: String,
    /// Authority class shared by the members.
    pub authority_class: AuthorityClass,
    /// Member surfaces, in stable order.
    pub member_surfaces: Vec<ReactiveSurfaceClass>,
    /// Human-readable parity rule.
    pub parity_rule: String,
}

/// One materialized-view declaration. Persistence and delete semantics
/// are computed from the view class per Appendix DB.3, so they cannot
/// drift from the architecture matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MaterializedViewDecl {
    /// Stable view id.
    pub view_id: String,
    /// Surface this view backs.
    pub backing_surface: ReactiveSurfaceClass,
    /// View class.
    pub view_class: ViewClass,
    /// Persistence semantics.
    pub persistence: PersistenceClass,
    /// Authority a read gets: authoritative truth or a derived view.
    pub authority_on_read: DerivationClass,
    /// Delete / eviction semantics.
    pub delete_semantics: DeleteSemantics,
    /// Whether the view can be rebuilt from authoritative inputs.
    pub rebuildable_from_authority: bool,
    /// Short reviewer note.
    pub notes: String,
}

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the M5 reactive-governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Reactive surface rows.
    pub surfaces: Vec<ReactiveSurfaceRow>,
    /// Cross-surface epoch-parity groups.
    pub epoch_parity_groups: Vec<EpochParityGroup>,
    /// Materialized-view declarations.
    pub materialized_views: Vec<MaterializedViewDecl>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// One fixture binding a surface and an observed state to the expected
/// narrowed claim, proving the canonical downgrade behavior.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ReactiveGovernanceFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Surface under test.
    pub surface_class: ReactiveSurfaceClass,
    /// Observed subscription state.
    pub observed: ObservedReactiveState,
    /// Expected narrowed claim.
    pub expected_claim: TruthClaim,
    /// Expected narrowing triggers.
    pub expected_triggers: Vec<NarrowingTrigger>,
    /// One consumer that quotes this surface row.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "m5 reactive governance validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// ---------------------------------------------------------------------------
// Seeded packet.
// ---------------------------------------------------------------------------

/// Returns the checked-in matrix packet this lane freezes.
pub fn seeded_m5_reactive_governance_packet() -> M5ReactiveGovernancePacket {
    let surfaces = vec![
        surface_row(
            ReactiveSurfaceClass::ShellWorkspaceTree,
            "vfs.workspace_tree",
            AuthorityClass::WorkspaceVfs,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            &[PresentationChannel::Ui, PresentationChannel::CliHeadless],
            &[Freshness::Warming, Freshness::Cached, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced],
            false,
            false,
            &[
                InvalidationReason::AuthorityEpochRolled,
                InvalidationReason::ProducerRestart,
                InvalidationReason::WatcherDropped,
                InvalidationReason::ScopeRemoved,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-shell/src/preview_truth/mod.rs",
            ],
            "The workspace tree projects VFS authority; it labels warming, cached, stale, partial, or coalesced state instead of implying exact current truth.",
        ),
        surface_row(
            ReactiveSurfaceClass::ShellActivityCenter,
            "execution.activity_stream",
            AuthorityClass::Execution,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            &[PresentationChannel::Ui, PresentationChannel::CliHeadless],
            &[Freshness::Warming, Freshness::Cached, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced, BackpressureMode::SnapshotRequired],
            true,
            false,
            &[
                InvalidationReason::ProducerRestart,
                InvalidationReason::QueueSaturation,
                InvalidationReason::AuthorityEpochRolled,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-shell/src/activity_center/deferred_publish.rs",
            ],
            "The activity center reflects execution authority; under queue saturation it coalesces or requests a fresh snapshot rather than presenting a saturated stream as current.",
        ),
        surface_row(
            ReactiveSurfaceClass::EditorBufferOutline,
            "buffer.outline",
            AuthorityClass::BufferEditor,
            ScopeClass::Window,
            ViewClass::EphemeralProjection,
            &[PresentationChannel::Ui],
            &[Freshness::Warming, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced],
            false,
            false,
            &[
                InvalidationReason::ProducerRestart,
                InvalidationReason::UpstreamInputStale,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-editor/src/lib.rs",
            ],
            "The outline is an ephemeral projection of buffer authority; it warms or marks stale rather than claiming to mirror every keystroke exactly.",
        ),
        surface_row(
            ReactiveSurfaceClass::SearchResults,
            "search.results",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            &[PresentationChannel::Ui, PresentationChannel::CliHeadless],
            &[Freshness::Warming, Freshness::Cached, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced, BackpressureMode::SnapshotRequired],
            false,
            false,
            &[
                InvalidationReason::UpstreamInputStale,
                InvalidationReason::ProducerRestart,
                InvalidationReason::ExplicitRefreshRequested,
                InvalidationReason::AuthorityEpochRolled,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-search/src/lib.rs",
            ],
            "Search results label warming, partial, cached, and stale states and pair them with a rerun path instead of presenting a partial index as complete.",
        ),
        surface_row(
            ReactiveSurfaceClass::GraphNeighborhood,
            "graph.neighborhood",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::EphemeralProjection,
            &[PresentationChannel::Ui],
            &[Freshness::Warming, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced],
            false,
            false,
            &[
                InvalidationReason::UpstreamInputStale,
                InvalidationReason::ProducerRestart,
                InvalidationReason::ScopeRemoved,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-graph/src/lib.rs",
            ],
            "Graph neighborhoods are ephemeral derived projections; partial graphs are labeled partial, never implied to be the whole neighborhood.",
        ),
        surface_row(
            ReactiveSurfaceClass::DocsBrowser,
            "docs.browser_index",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            &[PresentationChannel::Ui, PresentationChannel::CliHeadless],
            &[Freshness::Warming, Freshness::Cached, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced],
            false,
            false,
            &[
                InvalidationReason::UpstreamInputStale,
                InvalidationReason::CacheServed,
                InvalidationReason::ExplicitRefreshRequested,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-docs/src/lib.rs",
            ],
            "The docs browser serves cached projections explicitly and offers refresh rather than presenting cache as live.",
        ),
        surface_row(
            ReactiveSurfaceClass::AiContextPanel,
            "ai.context_projection",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::EphemeralProjection,
            &[PresentationChannel::Ui],
            &[Freshness::Warming, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::Coalesced],
            false,
            true,
            &[
                InvalidationReason::UpstreamInputStale,
                InvalidationReason::PolicyEpochChanged,
                InvalidationReason::ExplicitRefreshRequested,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-ai/src/lib.rs",
            ],
            "The AI context panel narrows to a policy-limited projection when entitlement or policy restricts the visible context.",
        ),
        surface_row(
            ReactiveSurfaceClass::ReviewWorkspace,
            "review.workspace_overlay",
            AuthorityClass::ProviderOverlay,
            ScopeClass::ReviewWorkspace,
            ViewClass::ManagedReplicatedView,
            &[PresentationChannel::Ui, PresentationChannel::Export],
            &[
                Freshness::Warming,
                Freshness::Cached,
                Freshness::Stale,
                Freshness::Imported,
            ],
            &[Completeness::Partial, Completeness::Unloaded, Completeness::Unavailable],
            &[BackpressureMode::Coalesced, BackpressureMode::SnapshotRequired],
            true,
            true,
            &[
                InvalidationReason::AuthorityEpochRolled,
                InvalidationReason::PolicyEpochChanged,
                InvalidationReason::ProducerRestart,
                InvalidationReason::ImportedFromExternal,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-review/src/lib.rs",
            ],
            "The review workspace is a managed replicated overlay; when the provider is unavailable it says so instead of replacing local truth.",
        ),
        surface_row(
            ReactiveSurfaceClass::PreviewOutput,
            "preview.output_snapshot",
            AuthorityClass::Execution,
            ScopeClass::Window,
            ViewClass::ExportableSnapshot,
            &[PresentationChannel::Ui, PresentationChannel::Export],
            &[Freshness::Replayed, Freshness::Imported],
            &[Completeness::Partial],
            &[],
            false,
            false,
            &[
                InvalidationReason::ReplayedFromBundle,
                InvalidationReason::ImportedFromExternal,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-preview/src/lib.rs",
            ],
            "Preview output is an exportable snapshot; replayed or imported snapshots are labeled as such and never updated in place.",
        ),
        surface_row(
            ReactiveSurfaceClass::CompanionPanel,
            "companion.surface_mirror",
            AuthorityClass::ProviderOverlay,
            ScopeClass::CompanionSurface,
            ViewClass::ManagedReplicatedView,
            &[PresentationChannel::Ui],
            &[
                Freshness::Warming,
                Freshness::Cached,
                Freshness::Stale,
                Freshness::Imported,
            ],
            &[Completeness::Partial, Completeness::Unloaded, Completeness::Unavailable],
            &[BackpressureMode::Coalesced],
            true,
            true,
            &[
                InvalidationReason::PolicyEpochChanged,
                InvalidationReason::ProducerRestart,
                InvalidationReason::CacheServed,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-companion/src/lib.rs",
            ],
            "The companion panel mirrors a managed provider view; it reconciles on reconnect and labels unavailable, cached, or policy-limited state.",
        ),
        surface_row(
            ReactiveSurfaceClass::PolicyTrustBanner,
            "policy.trust_state",
            AuthorityClass::PolicyEntitlement,
            ScopeClass::Workspace,
            ViewClass::EphemeralProjection,
            &[PresentationChannel::Ui, PresentationChannel::CliHeadless],
            &[Freshness::Warming, Freshness::Stale],
            &[Completeness::Unloaded],
            &[],
            false,
            true,
            &[
                InvalidationReason::PolicyEpochChanged,
                InvalidationReason::ExplicitRefreshRequested,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-policy/src/lib.rs",
            ],
            "The trust banner labels stale policy epochs so mutating affordances are not left enabled against an outdated entitlement.",
        ),
        surface_row(
            ReactiveSurfaceClass::HeadlessWorkspaceMirror,
            "vfs.workspace_tree",
            AuthorityClass::WorkspaceVfs,
            ScopeClass::RemoteSession,
            ViewClass::EphemeralProjection,
            &[PresentationChannel::CliHeadless, PresentationChannel::Release],
            &[Freshness::Warming, Freshness::Stale],
            &[Completeness::Partial, Completeness::Unloaded],
            &[BackpressureMode::SnapshotRequired],
            false,
            false,
            &[
                InvalidationReason::AuthorityEpochRolled,
                InvalidationReason::ProducerRestart,
            ],
            &[
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
                "crates/aureline-cli/src/lib.rs",
            ],
            "The headless mirror shares the workspace query family and narrows identically to the UI so CLI and release surfaces never present a richer claim than the UI would.",
        ),
        surface_row(
            ReactiveSurfaceClass::SupportExportView,
            "support.reactive_state_export",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::ExportableSnapshot,
            &[PresentationChannel::Export, PresentationChannel::Release],
            &[Freshness::Replayed, Freshness::Imported],
            &[Completeness::Partial],
            &[],
            false,
            false,
            &[
                InvalidationReason::ReplayedFromBundle,
                InvalidationReason::ImportedFromExternal,
            ],
            &[
                "crates/aureline-support/src/m5_reactive_governance/mod.rs",
                "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
            ],
            "The support export is an exportable snapshot of captured reactive state; it carries the same narrowing so release and procurement readers see the captured claim, not live truth.",
        ),
    ];

    let epoch_parity_groups = epoch_parity_groups_from_surfaces(&surfaces);
    let materialized_views = materialized_views_from_surfaces(&surfaces);

    M5ReactiveGovernancePacket {
        record_kind: M5_REACTIVE_GOVERNANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION,
        packet_id: "state.m5_reactive_governance.v1".to_owned(),
        title: "Canonical M5 reactive-state, subscription-envelope, and materialized-view governance matrix"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: M5_REACTIVE_GOVERNANCE_DOC_REF.to_owned(),
            schema_ref: M5_REACTIVE_GOVERNANCE_SCHEMA_REF.to_owned(),
            packet_ref: M5_REACTIVE_GOVERNANCE_PACKET_REF.to_owned(),
            report_ref: M5_REACTIVE_GOVERNANCE_REPORT_REF.to_owned(),
            fixture_manifest_ref: M5_REACTIVE_GOVERNANCE_FIXTURE_MANIFEST_REF.to_owned(),
        },
        surfaces,
        epoch_parity_groups,
        materialized_views,
        invariants: vec![
            "Every reactive M5 surface subscribes through one typed envelope (query family, scope, snapshot epoch, delta sequence, freshness, completeness, backpressure mode) instead of a private cache.".to_owned(),
            "No derived M5 surface presents exact current truth; the strongest a derived surface may claim is a consistent snapshot at its authoritative epoch.".to_owned(),
            "Claim narrowing is computed from one canonical engine, so stale, warming, partial, cached, replayed, imported, coalesced, policy-limited, and provider-unavailable states downgrade identically across UI, CLI/headless, export, and release channels.".to_owned(),
            "Surfaces sharing an authority class form one epoch-parity group and read the same authoritative epoch; a lagging member narrows rather than presenting a parallel epoch as truth.".to_owned(),
            "Materialized views declare persistence, read authority, and delete semantics per view class, so ephemeral, durable-local, exportable-snapshot, and managed-replicated views never blur their lifecycle.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture rows this lane freezes.
pub fn seeded_m5_reactive_governance_fixtures() -> Vec<M5ReactiveGovernanceFixture> {
    vec![
        fixture(
            "fixture:m5:reactive:shell_warming",
            ReactiveSurfaceClass::ShellWorkspaceTree,
            ObservedReactiveState {
                freshness: Freshness::Warming,
                completeness: Completeness::Unloaded,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-shell/src/m5_reactive_state_explainer/mod.rs",
            "A warming, unloaded workspace tree narrows to warming-no-truth-yet instead of an empty-but-authoritative claim.",
        ),
        fixture(
            "fixture:m5:reactive:shell_healthy",
            ReactiveSurfaceClass::ShellWorkspaceTree,
            ObservedReactiveState::healthy(),
            "crates/aureline-shell/src/preview_truth/mod.rs",
            "A fully healthy derived workspace tree presents a consistent snapshot, never exact current truth.",
        ),
        fixture(
            "fixture:m5:reactive:search_stale",
            ReactiveSurfaceClass::SearchResults,
            ObservedReactiveState {
                freshness: Freshness::Stale,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-search/src/lib.rs",
            "Stale search results narrow to a stale snapshot with a rerun path.",
        ),
        fixture(
            "fixture:m5:reactive:docs_cached",
            ReactiveSurfaceClass::DocsBrowser,
            ObservedReactiveState {
                freshness: Freshness::Cached,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-docs/src/lib.rs",
            "A cache-served docs index narrows to a cached projection, not live truth.",
        ),
        fixture(
            "fixture:m5:reactive:graph_partial",
            ReactiveSurfaceClass::GraphNeighborhood,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-graph/src/lib.rs",
            "A partially loaded graph neighborhood narrows to a partial projection.",
        ),
        fixture(
            "fixture:m5:reactive:ai_policy_limited",
            ReactiveSurfaceClass::AiContextPanel,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: true,
            },
            "crates/aureline-ai/src/lib.rs",
            "A policy-limited AI context narrows to a policy-limited projection even when otherwise healthy.",
        ),
        fixture(
            "fixture:m5:reactive:review_coalesced",
            ReactiveSurfaceClass::ReviewWorkspace,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-review/src/lib.rs",
            "A coalesced review overlay narrows to a coalesced-stream claim while it lags the producer delta rate.",
        ),
        fixture(
            "fixture:m5:reactive:companion_unavailable",
            ReactiveSurfaceClass::CompanionPanel,
            ObservedReactiveState {
                freshness: Freshness::Authoritative,
                completeness: Completeness::Unavailable,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: Some(TerminalReason::Unavailable),
                policy_limited: false,
            },
            "crates/aureline-companion/src/lib.rs",
            "An unavailable companion provider narrows to provider-unavailable instead of replacing local truth.",
        ),
        fixture(
            "fixture:m5:reactive:preview_replayed",
            ReactiveSurfaceClass::PreviewOutput,
            ObservedReactiveState {
                freshness: Freshness::Replayed,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-preview/src/lib.rs",
            "A replayed preview snapshot narrows to a replayed snapshot, the narrowest of replayed-versus-partial.",
        ),
        fixture(
            "fixture:m5:reactive:support_imported",
            ReactiveSurfaceClass::SupportExportView,
            ObservedReactiveState {
                freshness: Freshness::Imported,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
            },
            "crates/aureline-support/src/m5_reactive_governance/mod.rs",
            "An imported support-export snapshot narrows to an imported snapshot for release and procurement readers.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// Validates the seeded packet or an on-disk copy of it.
pub fn validate_m5_reactive_governance_packet(
    packet: &M5ReactiveGovernancePacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != M5_REACTIVE_GOVERNANCE_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {M5_REACTIVE_GOVERNANCE_PACKET_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {}, got {}",
                M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION, packet.schema_version
            ),
        );
    }
    if packet.source_contract_refs.doc_ref != M5_REACTIVE_GOVERNANCE_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted");
    }
    if packet.source_contract_refs.schema_ref != M5_REACTIVE_GOVERNANCE_SCHEMA_REF {
        report.push("packet.schema_ref", "schema_ref drifted");
    }
    if packet.source_contract_refs.packet_ref != M5_REACTIVE_GOVERNANCE_PACKET_REF {
        report.push("packet.packet_ref", "packet_ref drifted");
    }
    if packet.source_contract_refs.report_ref != M5_REACTIVE_GOVERNANCE_REPORT_REF {
        report.push("packet.report_ref", "report_ref drifted");
    }
    if packet.source_contract_refs.fixture_manifest_ref
        != M5_REACTIVE_GOVERNANCE_FIXTURE_MANIFEST_REF
    {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted",
        );
    }

    let mut surface_classes = BTreeSet::new();
    let mut authority_classes = BTreeSet::new();
    let mut view_classes = BTreeSet::new();
    let mut channels = BTreeSet::new();

    for row in &packet.surfaces {
        if !surface_classes.insert(row.surface_class) {
            report.push(
                "surface.duplicate",
                format!("duplicate surface {}", row.surface_class.as_str()),
            );
        }
        authority_classes.insert(row.authority_class);
        view_classes.insert(row.view_class);
        for channel in &row.presentation_channels {
            channels.insert(*channel);
        }

        if row.query_family.trim().is_empty() {
            report.push(
                "surface.query_family",
                format!(
                    "surface {} must carry a query family",
                    row.surface_class.as_str()
                ),
            );
        }
        if row.presentation_channels.is_empty() {
            report.push(
                "surface.channels",
                format!(
                    "surface {} must declare a presentation channel",
                    row.surface_class.as_str()
                ),
            );
        }
        if row.consumer_refs.is_empty() {
            report.push(
                "surface.consumer_refs",
                format!(
                    "surface {} must carry a consumer ref",
                    row.surface_class.as_str()
                ),
            );
        }
        if row.honored_invalidation_reasons.is_empty() {
            report.push(
                "surface.invalidation_reasons",
                format!(
                    "surface {} must honor at least one invalidation reason",
                    row.surface_class.as_str()
                ),
            );
        }

        // Guardrail: a derived surface may never advertise exact current truth.
        if row.derivation_class == DerivationClass::Derived
            && row.healthy_claim == TruthClaim::ExactCurrentTruth
        {
            report.push(
                "surface.exact_truth_overclaim",
                format!(
                    "derived surface {} may not present exact_current_truth",
                    row.surface_class.as_str()
                ),
            );
        }

        // The healthy claim must equal what the engine returns for a healthy state.
        let healthy = narrow_truth_claim(row.derivation_class, &ObservedReactiveState::healthy());
        if row.healthy_claim != healthy.claim {
            report.push(
                "surface.healthy_claim",
                format!(
                    "surface {} healthy_claim {} must equal engine output {}",
                    row.surface_class.as_str(),
                    row.healthy_claim.as_str(),
                    healthy.claim.as_str()
                ),
            );
        }

        // Declared narrowing rules must equal the engine's projection.
        let expected_rules = narrowing_rules_for(row);
        if row.claim_narrowing_rules != expected_rules {
            report.push(
                "surface.claim_narrowing_rules",
                format!(
                    "surface {} narrowing rules drifted from the canonical engine",
                    row.surface_class.as_str()
                ),
            );
        }

        validate_supported_sets(row, &mut report);
    }

    for required in [
        PresentationChannel::Ui,
        PresentationChannel::CliHeadless,
        PresentationChannel::Export,
        PresentationChannel::Release,
    ] {
        if !channels.contains(&required) {
            report.push(
                "packet.channel_missing",
                format!(
                    "matrix must cover presentation channel {}",
                    required.as_str()
                ),
            );
        }
    }

    for required in [
        ViewClass::EphemeralProjection,
        ViewClass::DurableLocalMaterialization,
        ViewClass::ExportableSnapshot,
        ViewClass::ManagedReplicatedView,
    ] {
        if !view_classes.contains(&required) {
            report.push(
                "packet.view_class_missing",
                format!("matrix must cover view class {}", required.as_str()),
            );
        }
    }

    validate_epoch_parity_groups(packet, &authority_classes, &mut report);
    validate_materialized_views(packet, &surface_classes, &mut report);

    if packet.invariants.iter().all(|inv| inv.trim().is_empty()) {
        report.push("packet.invariants", "invariants must be non-empty");
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_supported_sets(row: &ReactiveSurfaceRow, report: &mut ValidationReport) {
    if row.supported_freshness.contains(&Freshness::Authoritative) {
        report.push(
            "surface.supported_freshness",
            format!(
                "surface {} must list only degraded freshness states; authoritative is implicit",
                row.surface_class.as_str()
            ),
        );
    }
    if row.supported_completeness.contains(&Completeness::Full) {
        report.push(
            "surface.supported_completeness",
            format!(
                "surface {} must list only degraded completeness states; full is implicit",
                row.surface_class.as_str()
            ),
        );
    }
    if row
        .supported_backpressure
        .contains(&BackpressureMode::Realtime)
    {
        report.push(
            "surface.supported_backpressure",
            format!(
                "surface {} must list only non-realtime backpressure modes; realtime is implicit",
                row.surface_class.as_str()
            ),
        );
    }
    if !is_strictly_sorted_unique(&row.supported_freshness)
        || !is_strictly_sorted_unique(&row.supported_completeness)
        || !is_strictly_sorted_unique(&row.supported_backpressure)
        || !is_strictly_sorted_unique(&row.honored_invalidation_reasons)
        || !is_strictly_sorted_unique(&row.presentation_channels)
    {
        report.push(
            "surface.set_order",
            format!(
                "surface {} sets must be sorted and unique",
                row.surface_class.as_str()
            ),
        );
    }
}

fn validate_epoch_parity_groups(
    packet: &M5ReactiveGovernancePacket,
    authority_classes: &BTreeSet<AuthorityClass>,
    report: &mut ValidationReport,
) {
    let expected = epoch_parity_groups_from_surfaces(&packet.surfaces);
    if packet.epoch_parity_groups != expected {
        report.push(
            "packet.epoch_parity_groups",
            "epoch parity groups drifted from the surface authority classes",
        );
    }
    let grouped: BTreeSet<_> = packet
        .epoch_parity_groups
        .iter()
        .map(|group| group.authority_class)
        .collect();
    for authority in authority_classes {
        if !grouped.contains(authority) {
            report.push(
                "packet.epoch_parity_coverage",
                format!(
                    "authority class {} has no epoch parity group",
                    authority.as_str()
                ),
            );
        }
    }
}

fn validate_materialized_views(
    packet: &M5ReactiveGovernancePacket,
    surface_classes: &BTreeSet<ReactiveSurfaceClass>,
    report: &mut ValidationReport,
) {
    let expected = materialized_views_from_surfaces(&packet.surfaces);
    if packet.materialized_views != expected {
        report.push(
            "packet.materialized_views",
            "materialized view declarations drifted from the surface view classes",
        );
    }
    let mut backed = BTreeSet::new();
    for view in &packet.materialized_views {
        if !backed.insert(view.backing_surface) {
            report.push(
                "view.duplicate_backing",
                format!(
                    "duplicate materialized view for surface {}",
                    view.backing_surface.as_str()
                ),
            );
        }
    }
    for surface in surface_classes {
        if !backed.contains(surface) {
            report.push(
                "view.missing_backing",
                format!(
                    "surface {} has no materialized view declaration",
                    surface.as_str()
                ),
            );
        }
    }
}

/// Validates one fixture against the packet.
pub fn validate_m5_reactive_governance_fixture(
    packet: &M5ReactiveGovernancePacket,
    fixture: &M5ReactiveGovernanceFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != M5_REACTIVE_GOVERNANCE_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, M5_REACTIVE_GOVERNANCE_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION
            ),
        );
    }
    let Some(row) = packet
        .surfaces
        .iter()
        .find(|row| row.surface_class == fixture.surface_class)
    else {
        report.push(
            "fixture.surface_missing",
            format!(
                "fixture {} points to surface {} missing from the matrix",
                fixture.fixture_id,
                fixture.surface_class.as_str()
            ),
        );
        return Err(report);
    };

    validate_observed_supported(fixture, row, &mut report);

    let narrowed = narrow_truth_claim(row.derivation_class, &fixture.observed);
    if narrowed.claim != fixture.expected_claim {
        report.push(
            "fixture.expected_claim",
            format!(
                "fixture {} expected claim {} but engine produced {}",
                fixture.fixture_id,
                fixture.expected_claim.as_str(),
                narrowed.claim.as_str()
            ),
        );
    }
    if narrowed.triggers != fixture.expected_triggers {
        report.push(
            "fixture.expected_triggers",
            format!(
                "fixture {} expected triggers drifted from engine output",
                fixture.fixture_id
            ),
        );
    }
    if !row.consumer_refs.iter().any(|c| c == &fixture.consumer_ref) {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} consumer_ref {} must be declared by surface {}",
                fixture.fixture_id,
                fixture.consumer_ref,
                row.surface_class.as_str()
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_observed_supported(
    fixture: &M5ReactiveGovernanceFixture,
    row: &ReactiveSurfaceRow,
    report: &mut ValidationReport,
) {
    let observed = &fixture.observed;
    if observed.freshness != Freshness::Authoritative
        && !row.supported_freshness.contains(&observed.freshness)
    {
        report.push(
            "fixture.unsupported_freshness",
            format!(
                "fixture {} observes freshness {} not supported by surface {}",
                fixture.fixture_id,
                observed.freshness.as_str(),
                row.surface_class.as_str()
            ),
        );
    }
    if observed.completeness != Completeness::Full
        && !row.supported_completeness.contains(&observed.completeness)
    {
        report.push(
            "fixture.unsupported_completeness",
            format!(
                "fixture {} observes completeness {} not supported by surface {}",
                fixture.fixture_id,
                observed.completeness.as_str(),
                row.surface_class.as_str()
            ),
        );
    }
    if observed.backpressure_mode != BackpressureMode::Realtime
        && !row
            .supported_backpressure
            .contains(&observed.backpressure_mode)
    {
        report.push(
            "fixture.unsupported_backpressure",
            format!(
                "fixture {} observes backpressure {} not supported by surface {}",
                fixture.fixture_id,
                observed.backpressure_mode.as_str(),
                row.surface_class.as_str()
            ),
        );
    }
    if observed.terminal_reason == Some(TerminalReason::Unavailable)
        && !row.supports_terminal_unavailable
    {
        report.push(
            "fixture.unsupported_terminal",
            format!(
                "fixture {} observes terminal unavailable not supported by surface {}",
                fixture.fixture_id,
                row.surface_class.as_str()
            ),
        );
    }
    if observed.policy_limited && !row.supports_policy_limited {
        report.push(
            "fixture.unsupported_policy_limited",
            format!(
                "fixture {} observes policy-limited not supported by surface {}",
                fixture.fixture_id,
                row.surface_class.as_str()
            ),
        );
    }
}

// ---------------------------------------------------------------------------
// Builders / helpers (all deterministic, all computed from one source).
// ---------------------------------------------------------------------------

#[allow(clippy::too_many_arguments)]
fn surface_row(
    surface_class: ReactiveSurfaceClass,
    query_family: &str,
    authority_class: AuthorityClass,
    scope_class: ScopeClass,
    view_class: ViewClass,
    presentation_channels: &[PresentationChannel],
    supported_freshness: &[Freshness],
    supported_completeness: &[Completeness],
    supported_backpressure: &[BackpressureMode],
    supports_terminal_unavailable: bool,
    supports_policy_limited: bool,
    honored_invalidation_reasons: &[InvalidationReason],
    consumer_refs: &[&str],
    notes: &str,
) -> ReactiveSurfaceRow {
    // Every reactive M5 surface is a derived projection of an authority.
    let derivation_class = DerivationClass::Derived;
    let mut row = ReactiveSurfaceRow {
        surface_class,
        query_family: query_family.to_owned(),
        authority_class,
        derivation_class,
        scope_class,
        view_class,
        default_backpressure_mode: BackpressureMode::Realtime,
        presentation_channels: sorted_unique(presentation_channels),
        supported_freshness: sorted_unique(supported_freshness),
        supported_completeness: sorted_unique(supported_completeness),
        supported_backpressure: sorted_unique(supported_backpressure),
        supports_terminal_unavailable,
        supports_policy_limited,
        honored_invalidation_reasons: sorted_unique(honored_invalidation_reasons),
        healthy_claim: TruthClaim::ExactCurrentTruth, // replaced below
        claim_narrowing_rules: Vec::new(),            // replaced below
        consumer_refs: consumer_refs.iter().map(|c| (*c).to_owned()).collect(),
        notes: notes.to_owned(),
    };
    row.healthy_claim =
        narrow_truth_claim(derivation_class, &ObservedReactiveState::healthy()).claim;
    row.claim_narrowing_rules = narrowing_rules_for(&row);
    row
}

fn narrowing_rules_for(row: &ReactiveSurfaceRow) -> Vec<ClaimNarrowingRule> {
    let mut rules = Vec::new();
    let mut push_rule = |trigger: NarrowingTrigger, observed: ObservedReactiveState| {
        let narrowed = narrow_truth_claim(row.derivation_class, &observed);
        rules.push(ClaimNarrowingRule {
            trigger,
            narrowed_claim: narrowed.claim,
        });
    };

    for freshness in &row.supported_freshness {
        let mut observed = ObservedReactiveState::healthy();
        observed.freshness = *freshness;
        if let Some(trigger) = freshness_trigger(*freshness) {
            push_rule(trigger, observed);
        }
    }
    for completeness in &row.supported_completeness {
        let mut observed = ObservedReactiveState::healthy();
        observed.completeness = *completeness;
        if let Some(trigger) = completeness_trigger(*completeness) {
            push_rule(trigger, observed);
        }
    }
    for backpressure in &row.supported_backpressure {
        let mut observed = ObservedReactiveState::healthy();
        observed.backpressure_mode = *backpressure;
        if let Some(trigger) = backpressure_trigger(*backpressure) {
            push_rule(trigger, observed);
        }
    }
    if row.supports_terminal_unavailable {
        let mut observed = ObservedReactiveState::healthy();
        observed.terminal_reason = Some(TerminalReason::Unavailable);
        push_rule(NarrowingTrigger::TerminalUnavailable, observed);
    }
    if row.supports_policy_limited {
        let mut observed = ObservedReactiveState::healthy();
        observed.policy_limited = true;
        push_rule(NarrowingTrigger::PolicyLimited, observed);
    }

    rules.sort_by(|a, b| a.trigger.cmp(&b.trigger));
    rules
}

fn freshness_trigger(freshness: Freshness) -> Option<NarrowingTrigger> {
    match freshness {
        Freshness::Authoritative => None,
        Freshness::Warming => Some(NarrowingTrigger::FreshnessWarming),
        Freshness::Cached => Some(NarrowingTrigger::FreshnessCached),
        Freshness::Stale => Some(NarrowingTrigger::FreshnessStale),
        Freshness::Replayed => Some(NarrowingTrigger::FreshnessReplayed),
        Freshness::Imported => Some(NarrowingTrigger::FreshnessImported),
    }
}

fn completeness_trigger(completeness: Completeness) -> Option<NarrowingTrigger> {
    match completeness {
        Completeness::Full => None,
        Completeness::Partial => Some(NarrowingTrigger::CompletenessPartial),
        Completeness::Unloaded => Some(NarrowingTrigger::CompletenessUnloaded),
        Completeness::Unavailable => Some(NarrowingTrigger::CompletenessUnavailable),
    }
}

fn backpressure_trigger(mode: BackpressureMode) -> Option<NarrowingTrigger> {
    match mode {
        BackpressureMode::Realtime => None,
        BackpressureMode::Coalesced => Some(NarrowingTrigger::BackpressureCoalesced),
        BackpressureMode::SnapshotRequired => Some(NarrowingTrigger::BackpressureSnapshotRequired),
    }
}

fn epoch_parity_groups_from_surfaces(surfaces: &[ReactiveSurfaceRow]) -> Vec<EpochParityGroup> {
    let mut authorities: Vec<AuthorityClass> =
        surfaces.iter().map(|row| row.authority_class).collect();
    authorities.sort();
    authorities.dedup();
    authorities
        .into_iter()
        .map(|authority| {
            let mut members: Vec<ReactiveSurfaceClass> = surfaces
                .iter()
                .filter(|row| row.authority_class == authority)
                .map(|row| row.surface_class)
                .collect();
            members.sort();
            members.dedup();
            EpochParityGroup {
                group_id: format!("epoch_parity:{}", authority.as_str()),
                authority_class: authority,
                member_surfaces: members,
                parity_rule: format!(
                    "All {} surfaces read the same authoritative snapshot epoch; a member that lags its authority epoch narrows its claim instead of presenting a parallel epoch as current truth.",
                    authority.as_str()
                ),
            }
        })
        .collect()
}

fn materialized_views_from_surfaces(surfaces: &[ReactiveSurfaceRow]) -> Vec<MaterializedViewDecl> {
    let mut views: Vec<MaterializedViewDecl> = surfaces
        .iter()
        .map(|row| {
            let (persistence, delete_semantics, rebuildable) = view_lifecycle(row.view_class);
            MaterializedViewDecl {
                view_id: format!("view:{}", row.surface_class.as_str()),
                backing_surface: row.surface_class,
                view_class: row.view_class,
                persistence,
                authority_on_read: row.derivation_class,
                delete_semantics,
                rebuildable_from_authority: rebuildable,
                notes: format!(
                    "{} backs the {} surface and follows {} lifecycle semantics.",
                    row.view_class.as_str(),
                    row.surface_class.as_str(),
                    persistence.as_str()
                ),
            }
        })
        .collect();
    views.sort_by(|a, b| a.backing_surface.cmp(&b.backing_surface));
    views
}

/// Persistence, delete, and rebuildability semantics per view class
/// (Appendix DB.3).
fn view_lifecycle(view_class: ViewClass) -> (PersistenceClass, DeleteSemantics, bool) {
    match view_class {
        ViewClass::EphemeralProjection => (
            PersistenceClass::MemoryOnly,
            DeleteSemantics::EvictOnScopeChange,
            true,
        ),
        ViewClass::DurableLocalMaterialization => (
            PersistenceClass::LocalCacheOrDb,
            DeleteSemantics::ClearOrRebuild,
            true,
        ),
        ViewClass::ExportableSnapshot => (
            PersistenceClass::SavedArtifact,
            DeleteSemantics::ReplacedByNewSnapshot,
            false,
        ),
        ViewClass::ManagedReplicatedView => (
            PersistenceClass::ServiceOrLocalMirror,
            DeleteSemantics::ReconcileOnReconnect,
            true,
        ),
    }
}

fn fixture(
    fixture_id: &str,
    surface_class: ReactiveSurfaceClass,
    observed: ObservedReactiveState,
    consumer_ref: &str,
    notes: &str,
) -> M5ReactiveGovernanceFixture {
    let narrowed = narrow_truth_claim(DerivationClass::Derived, &observed);
    M5ReactiveGovernanceFixture {
        record_kind: M5_REACTIVE_GOVERNANCE_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: M5_REACTIVE_GOVERNANCE_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        surface_class,
        observed,
        expected_claim: narrowed.claim,
        expected_triggers: narrowed.triggers,
        consumer_ref: consumer_ref.to_owned(),
        notes: notes.to_owned(),
    }
}

fn sorted_unique<T: Copy + Ord>(items: &[T]) -> Vec<T> {
    let mut out: Vec<T> = items.to_vec();
    out.sort();
    out.dedup();
    out
}

fn is_strictly_sorted_unique<T: Ord>(items: &[T]) -> bool {
    items.windows(2).all(|pair| pair[0] < pair[1])
}

#[cfg(test)]
mod tests;
