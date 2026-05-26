//! Hardened graph freshness and confidence propagation packet.
//!
//! This module is the graph-owned contract for the M4 stable lane that
//! propagates freshness, confidence, and provenance from the workspace
//! graph onto every consumer that previously reduced graph truth to a
//! row-local label. The packet binds the v24 promise: every row must
//! carry a stable graph object or query handle, producer identity,
//! schema version, visibility scope, retention class, the graph epoch
//! the row was drawn from (`local_live`, `remote_synced`,
//! `imported_provider`, `cached_snapshot`, or `mixed_epoch_unresolvable`),
//! the invalidation scope class
//! (`smallest_subgraph`, `full_rebuild_schema_boundary`,
//! `full_rebuild_producer_version_boundary`,
//! `full_rebuild_workspace_epoch_boundary`, or `no_invalidation_needed`),
//! and a closed disclosure for any hidden-graph dependency a stable
//! consumer still relies on.
//!
//! The packet is intentionally metadata-only — it carries no raw
//! node bodies, no raw provider payloads, no secrets, and no ambient
//! credentials — and is read verbatim by the search shell, navigation
//! shell, docs/help surface, AI-context inspector, review bundle,
//! topology surface, CLI/headless emitter, support export, and the
//! release proof index instead of reconstructing freshness or
//! confidence from row-local prose.
//!
//! The vocabulary mirrors the v24 contract:
//!
//! - [`GraphHandle`] binds the stable graph object or query identity:
//!   handle class, target ref, schema version, producer id, producer
//!   version, and an optional opaque query digest so search, AI, review,
//!   and support paths reason about the same handle without re-running
//!   the producer.
//! - [`EpochLabel`] preserves the local, remote, imported-provider, and
//!   cached snapshot epoch refs and refuses to merge incompatible
//!   epochs into one unlabeled confidence state.
//! - [`InvalidationScope`] resolves to one closed class and pins the
//!   affected-subgraph refs; full-rebuild boundaries must record the
//!   boundary class and a non-empty rebuild reason so a consumer can
//!   tell a localized recompute from a workspace-epoch rebuild.
//! - [`FreshnessPropagationPacket`] preserves the visibility scope,
//!   retention class, hidden-graph dependency disclosure, and consumer
//!   projections so support and AI consumers cannot silently downgrade
//!   freshness or confidence after the row leaves the graph runtime.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`FreshnessPropagationPacket`].
pub const FRESHNESS_PROPAGATION_PACKET_RECORD_KIND: &str = "graph_freshness_propagation_packet";

/// Stable record-kind tag for [`FreshnessPropagationPacketSupportExport`].
pub const FRESHNESS_PROPAGATION_PACKET_SUPPORT_EXPORT_RECORD_KIND: &str =
    "graph_freshness_propagation_support_export";

/// Integer schema version for the stable freshness-propagation packet.
pub const FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the reviewer doc.
pub const FRESHNESS_PROPAGATION_PACKET_DOC_REF: &str =
    "docs/search/m4/freshness_propagation_packet.md";

/// Repo-relative path of the human-readable artifact narrative.
pub const FRESHNESS_PROPAGATION_PACKET_ARTIFACT_DOC_REF: &str =
    "artifacts/search/m4/freshness_propagation_packet.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const FRESHNESS_PROPAGATION_PACKET_FIXTURE_DIR: &str =
    "fixtures/search/m4/freshness_propagation_packet";

/// Repo-relative path of the checked-in stable propagation packet.
pub const FRESHNESS_PROPAGATION_PACKET_ARTIFACT_REF: &str =
    "artifacts/search/m4/freshness_propagation_packet.json";

/// Closed graph-handle class. A handle is either a stable graph object
/// (node, edge, or compound subject) or a stable query handle the
/// producer minted for a query-family answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphHandleClass {
    /// Stable graph node (symbol, file, route, owner, etc.).
    GraphNode,
    /// Stable graph edge (definition, reference, import, owns, etc.).
    GraphEdge,
    /// Stable graph subject covering a compound impact / explainer subject.
    GraphSubject,
    /// Stable query handle minted by the alpha query-family runtime.
    GraphQuery,
}

impl GraphHandleClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphNode => "graph_node",
            Self::GraphEdge => "graph_edge",
            Self::GraphSubject => "graph_subject",
            Self::GraphQuery => "graph_query",
        }
    }
}

/// Closed freshness class for a propagated graph row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Row reflects the live workspace graph at capture time.
    Live,
    /// Row reflects a hot-set lane while cold paths are still warming.
    HotSet,
    /// Row is from a warming lane that is still partial.
    Warming,
    /// Row came from a cached snapshot of the live workspace graph.
    Cached,
    /// Row is from a known-stale graph slice.
    Stale,
    /// Row was replayed from a captured fixture or snapshot.
    Replayed,
    /// Row came from an imported provider lane (e.g. external SBOM, registry).
    Imported,
    /// Freshness is unknown because the answering lane could not be polled.
    Unknown,
}

impl FreshnessClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::HotSet => "hot_set",
            Self::Warming => "warming",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Replayed => "replayed",
            Self::Imported => "imported",
            Self::Unknown => "unknown",
        }
    }

    /// True when this freshness state requires a visible caveat on the row.
    pub const fn requires_visible_caveat(self) -> bool {
        !matches!(self, Self::Live | Self::HotSet)
    }
}

/// Closed confidence class for a propagated graph row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceClass {
    /// Row is an exact, locally-authoritative graph fact.
    Exact,
    /// Row is authoritative under an imported provider's authority.
    ImportedAuthoritative,
    /// Row is derived (e.g. analyzed, inferred from cross-references).
    InferredDerived,
    /// Row is a heuristic match (no producer authority).
    Heuristic,
    /// Row was withheld by latency or policy; confidence is intentionally not asserted.
    Withheld,
}

impl ConfidenceClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::ImportedAuthoritative => "imported_authoritative",
            Self::InferredDerived => "inferred_derived",
            Self::Heuristic => "heuristic",
            Self::Withheld => "withheld",
        }
    }
}

/// Closed retention class for a propagated graph row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Row is persistent in the workspace graph store.
    Persistent,
    /// Row is bound to the current session and not persisted across restarts.
    SessionScoped,
    /// Row is held only in the warm hot-set lane.
    EphemeralWarm,
    /// Row is fully transient (e.g. live query result snapshot).
    Transient,
    /// Row retention is withheld by policy / trust posture.
    Withheld,
}

impl RetentionClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Persistent => "persistent",
            Self::SessionScoped => "session_scoped",
            Self::EphemeralWarm => "ephemeral_warm",
            Self::Transient => "transient",
            Self::Withheld => "withheld",
        }
    }
}

/// Closed visibility scope class. Visibility states what scope the row's
/// truth was drawn from, and is preserved by every consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VisibilityScopeClass {
    /// Row is workspace-public; visible to every consumer of the workspace graph.
    WorkspacePublic,
    /// Row is workspace-private; visible only to the local user / actor.
    WorkspacePrivate,
    /// Row is shared via a remote graph mirror.
    RemoteShared,
    /// Row is sourced from an imported external graph.
    ImportedExternal,
    /// Row is hidden by policy / trust posture but the row itself still surfaces a placeholder.
    PolicyHidden,
}

impl VisibilityScopeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspacePublic => "workspace_public",
            Self::WorkspacePrivate => "workspace_private",
            Self::RemoteShared => "remote_shared",
            Self::ImportedExternal => "imported_external",
            Self::PolicyHidden => "policy_hidden",
        }
    }
}

/// Closed graph-epoch class. Tracks where the row's graph truth was
/// produced so search, navigation, review, AI, topology, and support
/// surfaces never silently merge a local-live row with an imported or
/// cached row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphEpochClass {
    /// Row was produced from the live local workspace graph.
    LocalLive,
    /// Row was produced from a remote-synced graph slice that is in lockstep with the local epoch.
    RemoteSynced,
    /// Row came from an imported provider (registry, SBOM, external graph mirror).
    ImportedProvider,
    /// Row came from a cached snapshot replayed under the current session.
    CachedSnapshot,
    /// Row spans two or more incompatible epochs and must declare a mixed-epoch disclosure.
    MixedEpochUnresolvable,
}

impl GraphEpochClass {
    /// Every closed epoch token in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalLive,
        Self::RemoteSynced,
        Self::ImportedProvider,
        Self::CachedSnapshot,
        Self::MixedEpochUnresolvable,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalLive => "local_live",
            Self::RemoteSynced => "remote_synced",
            Self::ImportedProvider => "imported_provider",
            Self::CachedSnapshot => "cached_snapshot",
            Self::MixedEpochUnresolvable => "mixed_epoch_unresolvable",
        }
    }

    /// True when the epoch state is at the live edge.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LocalLive | Self::RemoteSynced)
    }
}

/// Closed invalidation-scope class. Records the smallest invalidation
/// the producer attempted; full-rebuild classes are reserved for
/// schema, producer-version, or workspace-epoch boundaries and MUST
/// surface as such.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationScopeClass {
    /// No invalidation was needed (e.g. row read from the warm hot-set).
    NoInvalidationNeeded,
    /// Invalidation targeted the smallest affected subgraph.
    SmallestSubgraph,
    /// Invalidation required a full rebuild because the schema version changed.
    FullRebuildSchemaBoundary,
    /// Invalidation required a full rebuild because the producer version changed.
    FullRebuildProducerVersionBoundary,
    /// Invalidation required a full rebuild because the workspace epoch changed.
    FullRebuildWorkspaceEpochBoundary,
}

impl InvalidationScopeClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoInvalidationNeeded => "no_invalidation_needed",
            Self::SmallestSubgraph => "smallest_subgraph",
            Self::FullRebuildSchemaBoundary => "full_rebuild_schema_boundary",
            Self::FullRebuildProducerVersionBoundary => "full_rebuild_producer_version_boundary",
            Self::FullRebuildWorkspaceEpochBoundary => "full_rebuild_workspace_epoch_boundary",
        }
    }

    /// True when the row reflects a full-rebuild boundary that must be surfaced.
    pub const fn is_full_rebuild_boundary(self) -> bool {
        matches!(
            self,
            Self::FullRebuildSchemaBoundary
                | Self::FullRebuildProducerVersionBoundary
                | Self::FullRebuildWorkspaceEpochBoundary
        )
    }
}

/// Closed hidden-graph dependency disclosure state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenGraphDependencyState {
    /// No hidden-graph dependency exists. The published graph/query contract is sufficient.
    PublishedContractOnly,
    /// A hidden-graph dependency exists and is disclosed with a closed reason.
    HiddenDependencyDisclosed,
    /// A hidden-graph dependency exists but has NOT been disclosed; row must be narrowed below stable.
    HiddenDependencyUndisclosed,
}

impl HiddenGraphDependencyState {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishedContractOnly => "published_contract_only",
            Self::HiddenDependencyDisclosed => "hidden_dependency_disclosed",
            Self::HiddenDependencyUndisclosed => "hidden_dependency_undisclosed",
        }
    }
}

/// Closed captured-vs-live status for a propagation packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedVsLiveClass {
    /// Packet was produced by a live session.
    Live,
    /// Packet was produced by replaying a captured snapshot.
    CapturedSnapshot,
    /// Packet was produced by a rerun that replaced a prior snapshot.
    RerunReplacedSnapshot,
    /// Packet was narrowed below scope and the captured rows are not live.
    NarrowedScopeRerun,
}

impl CapturedVsLiveClass {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::RerunReplacedSnapshot => "rerun_replaced_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Consumer surface that must inherit the propagation packet verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessPropagationConsumerSurface {
    /// Search shell quick-open, file, symbol, and command-search panes.
    SearchShell,
    /// Navigation shell (go-to / peek / hierarchy / continuity panels).
    NavigationShell,
    /// Docs/help surface that explains graph freshness and confidence.
    DocsHelp,
    /// AI-context inspector / picker.
    AiContextInspector,
    /// Review-pack bundle (review or PR-style payload consumer).
    ReviewBundle,
    /// Topology surface (graph map view).
    TopologySurface,
    /// CLI / headless emitter for graph queries.
    CliHeadless,
    /// Support export bundle.
    SupportExport,
    /// Release proof index entry.
    ReleaseProofIndex,
}

impl FreshnessPropagationConsumerSurface {
    /// Every required consumer surface in declaration order.
    pub const REQUIRED: [Self; 9] = [
        Self::SearchShell,
        Self::NavigationShell,
        Self::DocsHelp,
        Self::AiContextInspector,
        Self::ReviewBundle,
        Self::TopologySurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::ReleaseProofIndex,
    ];

    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SearchShell => "search_shell",
            Self::NavigationShell => "navigation_shell",
            Self::DocsHelp => "docs_help",
            Self::AiContextInspector => "ai_context_inspector",
            Self::ReviewBundle => "review_bundle",
            Self::TopologySurface => "topology_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::ReleaseProofIndex => "release_proof_index",
        }
    }
}

/// Closed promotion state for [`FreshnessPropagationPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessPropagationPromotionState {
    /// Packet certifies a stable claim.
    Stable,
    /// Packet must remain narrowed below stable until a recorded gap closes.
    NarrowedBelowStable,
    /// Packet has a blocker finding and cannot publish on stable surfaces.
    BlocksStable,
}

impl FreshnessPropagationPromotionState {
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
pub enum FreshnessPropagationFindingSeverity {
    /// Informational finding.
    Info,
    /// Reviewable finding that narrows the packet below stable.
    Warning,
    /// Blocker that prevents stable publication.
    Blocker,
}

/// Closed validation-finding vocabulary for [`FreshnessPropagationPacket`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessPropagationFindingKind {
    /// Record kind does not match the schema.
    WrongRecordKind,
    /// Schema version does not match the frozen schema.
    WrongSchemaVersion,
    /// Packet identity refs are empty.
    MissingPacketIdentity,
    /// Packet has no rows.
    MissingRow,
    /// A row has no graph handle identity.
    MissingGraphHandle,
    /// A row's graph handle has no producer identity.
    MissingProducerIdentity,
    /// A row's graph handle has no schema version.
    MissingSchemaVersionOnRow,
    /// A row has no visibility scope.
    MissingVisibilityScope,
    /// A row has no retention class.
    MissingRetentionClass,
    /// A row has no invalidation scope.
    MissingInvalidationScope,
    /// A row uses `MixedEpochUnresolvable` without disclosing the mixed epochs.
    MixedEpochUnlabeled,
    /// A row uses a full-rebuild invalidation class without surfacing its boundary reason.
    FullRebuildNotSurfaced,
    /// A row admits a hidden-graph dependency that is not disclosed.
    HiddenGraphDependencyUndisclosed,
    /// A required consumer projection is missing.
    MissingConsumerProjection,
    /// A consumer projection drops or remints freshness / confidence truth.
    ConsumerProjectionDrift,
    /// A consumer projection drops the epoch label.
    EpochLabelDropped,
    /// A consumer projection drops confidence into an unlabeled state.
    ConfidenceCollapsed,
    /// A row admits raw query text, raw node bodies, secrets, or provider payloads.
    RawBoundaryMaterialPresent,
    /// Packet drops one of the closed epoch tokens from its coverage vocabulary.
    MissingEpochCoverage,
    /// A row carries an epoch class that is not listed in `covered_epoch_classes`.
    EpochSilentlyDropped,
    /// A non-`live` row drops its partiality / withheld note.
    MissingPartialityNote,
    /// Stored promotion state disagrees with derived findings.
    PromotionStateMismatch,
}

impl FreshnessPropagationFindingKind {
    /// Stable token used in fixtures, schemas, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingPacketIdentity => "missing_packet_identity",
            Self::MissingRow => "missing_row",
            Self::MissingGraphHandle => "missing_graph_handle",
            Self::MissingProducerIdentity => "missing_producer_identity",
            Self::MissingSchemaVersionOnRow => "missing_schema_version_on_row",
            Self::MissingVisibilityScope => "missing_visibility_scope",
            Self::MissingRetentionClass => "missing_retention_class",
            Self::MissingInvalidationScope => "missing_invalidation_scope",
            Self::MixedEpochUnlabeled => "mixed_epoch_unlabeled",
            Self::FullRebuildNotSurfaced => "full_rebuild_not_surfaced",
            Self::HiddenGraphDependencyUndisclosed => "hidden_graph_dependency_undisclosed",
            Self::MissingConsumerProjection => "missing_consumer_projection",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::EpochLabelDropped => "epoch_label_dropped",
            Self::ConfidenceCollapsed => "confidence_collapsed",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
            Self::MissingEpochCoverage => "missing_epoch_coverage",
            Self::EpochSilentlyDropped => "epoch_silently_dropped",
            Self::MissingPartialityNote => "missing_partiality_note",
            Self::PromotionStateMismatch => "promotion_state_mismatch",
        }
    }
}

/// One validation finding emitted by the propagation validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationValidationFinding {
    /// Closed finding kind.
    pub finding_kind: FreshnessPropagationFindingKind,
    /// Finding severity.
    pub severity: FreshnessPropagationFindingSeverity,
    /// Short support-safe summary.
    pub summary: String,
}

impl FreshnessPropagationValidationFinding {
    fn new(
        finding_kind: FreshnessPropagationFindingKind,
        severity: FreshnessPropagationFindingSeverity,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            finding_kind,
            severity,
            summary: summary.into(),
        }
    }
}

/// Stable graph object or query handle attached to every row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphHandle {
    /// Stable handle id (URN-style, deterministic across presentational changes).
    pub handle_id: String,
    /// Handle class.
    pub handle_class: GraphHandleClass,
    /// Canonical ref to the underlying graph object or query (node id, edge id, query id).
    pub target_ref: String,
    /// Schema version under which the row's truth is interpreted.
    pub schema_version: u32,
    /// Stable producer identity.
    pub producer_id: String,
    /// Producer version that minted the row.
    pub producer_version: String,
    /// Optional opaque query digest pinned for query-handles.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_digest_ref: Option<String>,
}

impl GraphHandle {
    fn has_identity(&self) -> bool {
        !self.handle_id.trim().is_empty() && !self.target_ref.trim().is_empty()
    }

    fn has_producer_identity(&self) -> bool {
        !self.producer_id.trim().is_empty() && !self.producer_version.trim().is_empty()
    }
}

/// Mixed-epoch disclosure for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MixedEpochDisclosure {
    /// Short summary of the incompatible epochs the row spans.
    pub summary: String,
    /// Epoch refs the row spans.
    pub epoch_refs: Vec<String>,
}

impl MixedEpochDisclosure {
    fn is_valid(&self) -> bool {
        !self.summary.trim().is_empty() && !self.epoch_refs.is_empty()
    }
}

/// Graph-epoch labeling for a propagated row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochLabel {
    /// Closed epoch class.
    pub epoch_class: GraphEpochClass,
    /// Optional local workspace epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_epoch_ref: Option<String>,
    /// Optional remote-synced epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_epoch_ref: Option<String>,
    /// Optional imported provider epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_provider_epoch_ref: Option<String>,
    /// Optional cached snapshot epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cached_snapshot_epoch_ref: Option<String>,
    /// Required disclosure when `epoch_class` is `mixed_epoch_unresolvable`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mixed_epoch_disclosure: Option<MixedEpochDisclosure>,
}

/// Invalidation scope pinned to every row. Tracks the smallest
/// invalidation the producer attempted; full-rebuild boundaries must
/// declare a non-empty reason so a consumer can tell a localized
/// recompute from a workspace-epoch rebuild.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationScope {
    /// Closed invalidation-scope class.
    pub invalidation_class: InvalidationScopeClass,
    /// Affected subgraph refs (when the class is `smallest_subgraph`).
    #[serde(default)]
    pub affected_subgraph_refs: Vec<String>,
    /// Required reason when a full-rebuild boundary fired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_rebuild_reason: Option<String>,
}

impl InvalidationScope {
    fn requires_subgraph_refs(&self) -> bool {
        matches!(self.invalidation_class, InvalidationScopeClass::SmallestSubgraph)
    }

    fn requires_full_rebuild_reason(&self) -> bool {
        self.invalidation_class.is_full_rebuild_boundary()
    }

    fn is_valid(&self) -> bool {
        if self.requires_subgraph_refs() && self.affected_subgraph_refs.is_empty() {
            return false;
        }
        if self.requires_full_rebuild_reason()
            && self
                .full_rebuild_reason
                .as_deref()
                .map(|reason| reason.trim().is_empty())
                .unwrap_or(true)
        {
            return false;
        }
        true
    }
}

/// Hidden-graph dependency disclosure attached to a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenGraphDependencyDisclosure {
    /// Closed disclosure state.
    pub state: HiddenGraphDependencyState,
    /// Optional reason / dependency ref (required when the state is `hidden_dependency_disclosed`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disclosed_reason: Option<String>,
}

impl HiddenGraphDependencyDisclosure {
    fn is_valid(&self) -> bool {
        match self.state {
            HiddenGraphDependencyState::PublishedContractOnly => true,
            HiddenGraphDependencyState::HiddenDependencyDisclosed => self
                .disclosed_reason
                .as_deref()
                .map(|reason| !reason.trim().is_empty())
                .unwrap_or(false),
            HiddenGraphDependencyState::HiddenDependencyUndisclosed => false,
        }
    }
}

/// One propagated row in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationRow {
    /// Stable row id inside the packet.
    pub row_id: String,
    /// Stable graph handle binding the row to its producer truth.
    pub graph_handle: GraphHandle,
    /// Closed freshness class.
    pub freshness: FreshnessClass,
    /// Closed confidence class.
    pub confidence: ConfidenceClass,
    /// Closed retention class.
    pub retention: RetentionClass,
    /// Closed visibility scope class.
    pub visibility_scope: VisibilityScopeClass,
    /// Graph epoch labeling for the row.
    pub epoch_label: EpochLabel,
    /// Invalidation scope pinned to the row.
    pub invalidation_scope: InvalidationScope,
    /// Hidden-graph dependency disclosure.
    pub hidden_graph_dependency: HiddenGraphDependencyDisclosure,
    /// Closed consumer surface this row was rendered for.
    pub consumer_surface: FreshnessPropagationConsumerSurface,
    /// Short partiality / withheld note required when freshness is non-`live`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partiality_note: Option<String>,
    /// True when raw query text, raw node bodies, secrets, and provider payloads are excluded.
    pub raw_boundary_material_excluded: bool,
    /// Capture timestamp for this row.
    pub captured_at: String,
}

/// Consumer projection proving a surface reads the same packet without
/// reinventing freshness or confidence locally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationConsumerProjection {
    /// Consumer surface class.
    pub consumer_surface: FreshnessPropagationConsumerSurface,
    /// Stable projection ref.
    pub projection_ref: String,
    /// Packet id consumed by the projection.
    pub packet_id_ref: String,
    /// Render timestamp.
    pub rendered_at: String,
    /// True when the surface preserves the same packet id.
    pub preserves_same_packet: bool,
    /// True when the surface preserves the graph handle and producer identity.
    pub preserves_graph_handle: bool,
    /// True when the surface preserves the schema version pinned on each row.
    pub preserves_schema_version: bool,
    /// True when the surface preserves the visibility scope verbatim.
    pub preserves_visibility_scope: bool,
    /// True when the surface preserves the retention class verbatim.
    pub preserves_retention_class: bool,
    /// True when the surface preserves the epoch label verbatim.
    pub preserves_epoch_label: bool,
    /// True when the surface preserves the invalidation scope verbatim.
    pub preserves_invalidation_scope: bool,
    /// True when the surface preserves the hidden-graph dependency disclosure verbatim.
    pub preserves_hidden_dependency_state: bool,
    /// True when the surface preserves the freshness class verbatim.
    pub preserves_freshness_class: bool,
    /// True when the surface preserves the confidence class verbatim.
    pub preserves_confidence_class: bool,
    /// True when JSON export is available from the projection.
    pub supports_json_export: bool,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient authority / credentials are excluded.
    pub ambient_authority_excluded: bool,
}

impl FreshnessPropagationConsumerProjection {
    fn preserves_truth_for(&self, packet_id: &str) -> bool {
        self.packet_id_ref == packet_id
            && self.preserves_same_packet
            && self.preserves_graph_handle
            && self.preserves_schema_version
            && self.preserves_visibility_scope
            && self.preserves_retention_class
            && self.preserves_epoch_label
            && self.preserves_invalidation_scope
            && self.preserves_hidden_dependency_state
            && self.preserves_freshness_class
            && self.preserves_confidence_class
            && self.supports_json_export
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && !self.projection_ref.trim().is_empty()
    }
}

/// Constructor input for [`FreshnessPropagationPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Capture timestamp for the packet as a whole.
    pub generated_at: String,
    /// Captured-vs-live status.
    pub captured_vs_live: CapturedVsLiveClass,
    /// Epoch classes covered by the packet (must include every epoch
    /// represented by a row in `rows`).
    #[serde(default)]
    pub covered_epoch_classes: Vec<GraphEpochClass>,
    /// Propagated rows.
    #[serde(default)]
    pub rows: Vec<FreshnessPropagationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FreshnessPropagationConsumerProjection>,
    /// Source contract refs (docs / fixtures / artifact) consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
}

/// Graph-owned packet for hardened freshness and confidence propagation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationPacket {
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
    /// Captured-vs-live status.
    pub captured_vs_live: CapturedVsLiveClass,
    /// Epoch classes covered by the packet.
    #[serde(default)]
    pub covered_epoch_classes: Vec<GraphEpochClass>,
    /// Propagated rows.
    #[serde(default)]
    pub rows: Vec<FreshnessPropagationRow>,
    /// Consumer projections preserving this packet.
    #[serde(default)]
    pub consumer_projections: Vec<FreshnessPropagationConsumerProjection>,
    /// Source contract refs consumed by the packet.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Derived promotion state.
    pub promotion_state: FreshnessPropagationPromotionState,
    /// Validation findings captured at materialization.
    #[serde(default)]
    pub validation_findings: Vec<FreshnessPropagationValidationFinding>,
}

impl FreshnessPropagationPacket {
    /// Materialize a packet and record derived validation findings.
    pub fn materialize(input: FreshnessPropagationPacketInput) -> Self {
        let mut packet = Self {
            record_kind: FRESHNESS_PROPAGATION_PACKET_RECORD_KIND.to_owned(),
            schema_version: FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            generated_at: input.generated_at,
            captured_vs_live: input.captured_vs_live,
            covered_epoch_classes: input.covered_epoch_classes,
            rows: input.rows,
            consumer_projections: input.consumer_projections,
            source_contract_refs: input.source_contract_refs,
            promotion_state: FreshnessPropagationPromotionState::Stable,
            validation_findings: Vec::new(),
        };
        let findings = packet.derived_findings(false);
        packet.promotion_state = promotion_state_for_findings(&findings);
        packet.validation_findings = findings;
        packet
    }

    /// Re-validate the packet against stable propagation invariants.
    pub fn validate(&self) -> Vec<FreshnessPropagationValidationFinding> {
        self.derived_findings(true)
    }

    /// True when this packet has no blocker-level finding.
    pub fn is_stable(&self) -> bool {
        !self
            .validate()
            .iter()
            .any(|finding| finding.severity == FreshnessPropagationFindingSeverity::Blocker)
    }

    /// Returns true when a consumer projection preserves this packet for `surface`.
    pub fn has_projection_for(&self, surface: FreshnessPropagationConsumerSurface) -> bool {
        self.consumer_projections.iter().any(|projection| {
            projection.consumer_surface == surface
                && projection.preserves_truth_for(&self.packet_id)
        })
    }

    /// Returns the unique epoch-class tokens carried across rows.
    pub fn epoch_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.epoch_label.epoch_class);
        }
        set.into_iter().map(GraphEpochClass::as_str).collect()
    }

    /// Returns the unique freshness-class tokens carried across rows.
    pub fn freshness_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.freshness);
        }
        set.into_iter().map(FreshnessClass::as_str).collect()
    }

    /// Returns the unique confidence-class tokens carried across rows.
    pub fn confidence_class_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.confidence);
        }
        set.into_iter().map(ConfidenceClass::as_str).collect()
    }

    /// Returns the unique consumer surfaces present across rows.
    pub fn consumer_surface_tokens(&self) -> Vec<&'static str> {
        let mut set = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.consumer_surface);
        }
        set.into_iter()
            .map(FreshnessPropagationConsumerSurface::as_str)
            .collect()
    }

    /// Build a support export wrapping the exact product packet.
    pub fn support_export(
        &self,
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
    ) -> FreshnessPropagationPacketSupportExport {
        FreshnessPropagationPacketSupportExport {
            record_kind: FRESHNESS_PROPAGATION_PACKET_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION,
            export_id: export_id.into(),
            export_packet_id_ref: self.packet_id.clone(),
            exported_at: exported_at.into(),
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            export_packet: self.clone(),
        }
    }

    fn derived_findings(
        &self,
        include_record_fields: bool,
    ) -> Vec<FreshnessPropagationValidationFinding> {
        let mut findings = Vec::new();

        if include_record_fields
            && self.record_kind != FRESHNESS_PROPAGATION_PACKET_RECORD_KIND
        {
            findings.push(FreshnessPropagationValidationFinding::new(
                FreshnessPropagationFindingKind::WrongRecordKind,
                FreshnessPropagationFindingSeverity::Blocker,
                "freshness-propagation packet has the wrong record kind",
            ));
        }
        if include_record_fields
            && self.schema_version != FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION
        {
            findings.push(FreshnessPropagationValidationFinding::new(
                FreshnessPropagationFindingKind::WrongSchemaVersion,
                FreshnessPropagationFindingSeverity::Blocker,
                "freshness-propagation packet has the wrong schema version",
            ));
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.generated_at.trim().is_empty()
        {
            findings.push(FreshnessPropagationValidationFinding::new(
                FreshnessPropagationFindingKind::MissingPacketIdentity,
                FreshnessPropagationFindingSeverity::Blocker,
                "packet, workflow, and timestamp refs are required",
            ));
        }

        if self.rows.is_empty() {
            findings.push(FreshnessPropagationValidationFinding::new(
                FreshnessPropagationFindingKind::MissingRow,
                FreshnessPropagationFindingSeverity::Blocker,
                "packet must include at least one propagated row",
            ));
        }

        let row_epoch_set: BTreeSet<GraphEpochClass> = self
            .rows
            .iter()
            .map(|row| row.epoch_label.epoch_class)
            .collect();
        let covered_epoch_set: BTreeSet<GraphEpochClass> =
            self.covered_epoch_classes.iter().copied().collect();
        for epoch in &row_epoch_set {
            if !covered_epoch_set.contains(epoch) {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::EpochSilentlyDropped,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row carries epoch class {} but packet covered_epoch_classes drops it",
                        epoch.as_str()
                    ),
                ));
            }
        }
        for epoch in &covered_epoch_set {
            if !row_epoch_set.contains(epoch) {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingEpochCoverage,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "packet declares epoch class {} in coverage but no row carries it",
                        epoch.as_str()
                    ),
                ));
            }
        }

        for row in &self.rows {
            let target = row.row_id.as_str();
            if row.row_id.trim().is_empty() || row.captured_at.trim().is_empty() {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingPacketIdentity,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!("row {} identity or capture timestamp is empty", target),
                ));
            }
            if !row.graph_handle.has_identity() {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingGraphHandle,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!("row {} graph handle has no id or target ref", target),
                ));
            }
            if !row.graph_handle.has_producer_identity() {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingProducerIdentity,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} graph handle is missing producer id or producer version",
                        target
                    ),
                ));
            }
            if row.graph_handle.schema_version == 0 {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingSchemaVersionOnRow,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!("row {} graph handle has no schema version", target),
                ));
            }
            // Visibility and retention are enum-typed; an additional check ensures
            // policy-hidden rows always disclose a hidden-dependency or partiality note.
            if matches!(row.visibility_scope, VisibilityScopeClass::PolicyHidden)
                && row.partiality_note.is_none()
            {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingVisibilityScope,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} is policy_hidden but carries no partiality note",
                        target
                    ),
                ));
            }
            if matches!(row.retention, RetentionClass::Withheld)
                && row.partiality_note.is_none()
            {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingRetentionClass,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} retention is withheld but carries no partiality note",
                        target
                    ),
                ));
            }
            if !row.invalidation_scope.is_valid() {
                if row.invalidation_scope.requires_full_rebuild_reason() {
                    findings.push(FreshnessPropagationValidationFinding::new(
                        FreshnessPropagationFindingKind::FullRebuildNotSurfaced,
                        FreshnessPropagationFindingSeverity::Blocker,
                        format!(
                            "row {} uses {} but does not surface a full-rebuild reason",
                            target,
                            row.invalidation_scope.invalidation_class.as_str()
                        ),
                    ));
                } else {
                    findings.push(FreshnessPropagationValidationFinding::new(
                        FreshnessPropagationFindingKind::MissingInvalidationScope,
                        FreshnessPropagationFindingSeverity::Blocker,
                        format!(
                            "row {} invalidation scope is incomplete for class {}",
                            target,
                            row.invalidation_scope.invalidation_class.as_str()
                        ),
                    ));
                }
            }
            if matches!(
                row.epoch_label.epoch_class,
                GraphEpochClass::MixedEpochUnresolvable
            ) && row
                .epoch_label
                .mixed_epoch_disclosure
                .as_ref()
                .map(|disclosure| !disclosure.is_valid())
                .unwrap_or(true)
            {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MixedEpochUnlabeled,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} declares mixed_epoch_unresolvable but does not disclose its mixed epochs",
                        target
                    ),
                ));
            }
            if !row.hidden_graph_dependency.is_valid() {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::HiddenGraphDependencyUndisclosed,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} relies on a hidden graph dependency that is not disclosed",
                        target
                    ),
                ));
            }
            if matches!(row.confidence, ConfidenceClass::Withheld)
                && row.partiality_note.is_none()
            {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::ConfidenceCollapsed,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} confidence is withheld but no partiality note explains why",
                        target
                    ),
                ));
            }
            if row.freshness.requires_visible_caveat() && row.partiality_note.is_none() {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingPartialityNote,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} freshness is {} but carries no partiality note",
                        target,
                        row.freshness.as_str()
                    ),
                ));
            }
            if !row.raw_boundary_material_excluded {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::RawBoundaryMaterialPresent,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "row {} admits raw query text, node bodies, or provider payloads",
                        target
                    ),
                ));
            }
        }

        for required_surface in FreshnessPropagationConsumerSurface::REQUIRED {
            if !self.has_projection_for(required_surface) {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::MissingConsumerProjection,
                    FreshnessPropagationFindingSeverity::Blocker,
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
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::ConsumerProjectionDrift,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "projection {} does not preserve freshness/confidence truth",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_epoch_label {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::EpochLabelDropped,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "projection {} drops the epoch label",
                        projection.projection_ref
                    ),
                ));
            }
            if !projection.preserves_confidence_class {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::ConfidenceCollapsed,
                    FreshnessPropagationFindingSeverity::Blocker,
                    format!(
                        "projection {} collapses confidence into an unlabeled state",
                        projection.projection_ref
                    ),
                ));
            }
        }

        if include_record_fields {
            let mut without_promotion = findings.clone();
            without_promotion.retain(|finding| {
                finding.finding_kind != FreshnessPropagationFindingKind::PromotionStateMismatch
            });
            let derived = promotion_state_for_findings(&without_promotion);
            if self.promotion_state != derived {
                findings.push(FreshnessPropagationValidationFinding::new(
                    FreshnessPropagationFindingKind::PromotionStateMismatch,
                    FreshnessPropagationFindingSeverity::Blocker,
                    "stored promotion state does not match derived findings",
                ));
            }
        }

        findings
    }
}

fn promotion_state_for_findings(
    findings: &[FreshnessPropagationValidationFinding],
) -> FreshnessPropagationPromotionState {
    if findings
        .iter()
        .any(|finding| finding.severity == FreshnessPropagationFindingSeverity::Blocker)
    {
        FreshnessPropagationPromotionState::BlocksStable
    } else if findings
        .iter()
        .any(|finding| finding.severity == FreshnessPropagationFindingSeverity::Warning)
    {
        FreshnessPropagationPromotionState::NarrowedBelowStable
    } else {
        FreshnessPropagationPromotionState::Stable
    }
}

/// Support-export wrapper preserving the product propagation packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessPropagationPacketSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export packet id preserved by the export.
    pub export_packet_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// True when raw private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when ambient credentials / authority are excluded.
    pub ambient_authority_excluded: bool,
    /// Exact product packet preserved by the export.
    pub export_packet: FreshnessPropagationPacket,
}

impl FreshnessPropagationPacketSupportExport {
    /// True when the export preserves the same packet id safely.
    pub fn is_export_safe(&self) -> bool {
        self.record_kind == FRESHNESS_PROPAGATION_PACKET_SUPPORT_EXPORT_RECORD_KIND
            && self.schema_version == FRESHNESS_PROPAGATION_PACKET_SCHEMA_VERSION
            && self.export_packet_id_ref == self.export_packet.packet_id
            && self.raw_private_material_excluded
            && self.ambient_authority_excluded
            && self.export_packet.validate().is_empty()
    }
}

/// Errors emitted when reading the checked-in stable propagation packet.
#[derive(Debug)]
pub enum FreshnessPropagationArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(Vec<FreshnessPropagationValidationFinding>),
}

impl fmt::Display for FreshnessPropagationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => write!(
                formatter,
                "graph freshness-propagation packet parse failed: {error}"
            ),
            Self::Validation(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "graph freshness-propagation packet failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FreshnessPropagationArtifactError {}

/// Returns the checked-in stable propagation packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or validate.
pub fn current_stable_freshness_propagation_packet(
) -> Result<FreshnessPropagationPacket, FreshnessPropagationArtifactError> {
    let packet: FreshnessPropagationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/search/m4/freshness_propagation_packet.json"
    )))
    .map_err(FreshnessPropagationArtifactError::Packet)?;
    let findings = packet.validate();
    if findings.is_empty() {
        Ok(packet)
    } else {
        Err(FreshnessPropagationArtifactError::Validation(findings))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_handle(label: &str) -> GraphHandle {
        GraphHandle {
            handle_id: format!("graph:handle:{label}"),
            handle_class: GraphHandleClass::GraphNode,
            target_ref: format!("graph:node:{label}"),
            schema_version: 1,
            producer_id: "producer:graph.workspace.alpha".to_owned(),
            producer_version: "0.1.0".to_owned(),
            query_digest_ref: None,
        }
    }

    fn sample_epoch_label() -> EpochLabel {
        EpochLabel {
            epoch_class: GraphEpochClass::LocalLive,
            local_epoch_ref: Some("epoch:local:sha-abc".to_owned()),
            remote_epoch_ref: None,
            imported_provider_epoch_ref: None,
            cached_snapshot_epoch_ref: None,
            mixed_epoch_disclosure: None,
        }
    }

    fn sample_invalidation() -> InvalidationScope {
        InvalidationScope {
            invalidation_class: InvalidationScopeClass::SmallestSubgraph,
            affected_subgraph_refs: vec!["graph:subgraph:router".to_owned()],
            full_rebuild_reason: None,
        }
    }

    fn sample_hidden_dependency() -> HiddenGraphDependencyDisclosure {
        HiddenGraphDependencyDisclosure {
            state: HiddenGraphDependencyState::PublishedContractOnly,
            disclosed_reason: None,
        }
    }

    fn sample_row(
        surface: FreshnessPropagationConsumerSurface,
        label: &str,
    ) -> FreshnessPropagationRow {
        FreshnessPropagationRow {
            row_id: format!("row:{label}:{}", surface.as_str()),
            graph_handle: sample_handle(label),
            freshness: FreshnessClass::Live,
            confidence: ConfidenceClass::Exact,
            retention: RetentionClass::Persistent,
            visibility_scope: VisibilityScopeClass::WorkspacePublic,
            epoch_label: sample_epoch_label(),
            invalidation_scope: sample_invalidation(),
            hidden_graph_dependency: sample_hidden_dependency(),
            consumer_surface: surface,
            partiality_note: None,
            raw_boundary_material_excluded: true,
            captured_at: "2026-05-26T12:00:00Z".to_owned(),
        }
    }

    fn sample_projection(
        surface: FreshnessPropagationConsumerSurface,
        packet_id: &str,
    ) -> FreshnessPropagationConsumerProjection {
        FreshnessPropagationConsumerProjection {
            consumer_surface: surface,
            projection_ref: format!("projection:{}", surface.as_str()),
            packet_id_ref: packet_id.to_owned(),
            rendered_at: "2026-05-26T12:00:01Z".to_owned(),
            preserves_same_packet: true,
            preserves_graph_handle: true,
            preserves_schema_version: true,
            preserves_visibility_scope: true,
            preserves_retention_class: true,
            preserves_epoch_label: true,
            preserves_invalidation_scope: true,
            preserves_hidden_dependency_state: true,
            preserves_freshness_class: true,
            preserves_confidence_class: true,
            supports_json_export: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    fn baseline_input(packet_id: &str) -> FreshnessPropagationPacketInput {
        FreshnessPropagationPacketInput {
            packet_id: packet_id.to_owned(),
            workflow_or_surface_id: "workflow.graph.freshness_propagation.baseline".to_owned(),
            generated_at: "2026-05-26T12:00:00Z".to_owned(),
            captured_vs_live: CapturedVsLiveClass::Live,
            covered_epoch_classes: vec![GraphEpochClass::LocalLive],
            rows: FreshnessPropagationConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| sample_row(surface, "baseline"))
                .collect(),
            consumer_projections: FreshnessPropagationConsumerSurface::REQUIRED
                .iter()
                .copied()
                .map(|surface| sample_projection(surface, packet_id))
                .collect(),
            source_contract_refs: vec![FRESHNESS_PROPAGATION_PACKET_DOC_REF.to_owned()],
        }
    }

    #[test]
    fn closed_tokens_are_pinned() {
        assert_eq!(GraphEpochClass::LocalLive.as_str(), "local_live");
        assert_eq!(GraphEpochClass::RemoteSynced.as_str(), "remote_synced");
        assert_eq!(
            GraphEpochClass::ImportedProvider.as_str(),
            "imported_provider"
        );
        assert_eq!(GraphEpochClass::CachedSnapshot.as_str(), "cached_snapshot");
        assert_eq!(
            GraphEpochClass::MixedEpochUnresolvable.as_str(),
            "mixed_epoch_unresolvable"
        );
        assert_eq!(
            InvalidationScopeClass::SmallestSubgraph.as_str(),
            "smallest_subgraph"
        );
        assert_eq!(
            InvalidationScopeClass::FullRebuildSchemaBoundary.as_str(),
            "full_rebuild_schema_boundary"
        );
        assert_eq!(ConfidenceClass::ImportedAuthoritative.as_str(), "imported_authoritative");
        assert_eq!(
            HiddenGraphDependencyState::HiddenDependencyUndisclosed.as_str(),
            "hidden_dependency_undisclosed"
        );
        assert_eq!(
            FreshnessPropagationConsumerSurface::NavigationShell.as_str(),
            "navigation_shell"
        );
    }

    #[test]
    fn baseline_packet_certifies_stable() {
        let packet = FreshnessPropagationPacket::materialize(baseline_input(
            "packet:m4:freshness_propagation:baseline",
        ));
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::Stable,
            "unexpected findings: {:?}",
            packet
                .validation_findings
                .iter()
                .map(|finding| finding.finding_kind.as_str())
                .collect::<Vec<_>>()
        );
        assert!(packet.validation_findings.is_empty());
        assert_eq!(packet.epoch_class_tokens(), vec!["local_live"]);
    }

    #[test]
    fn mixed_epoch_without_disclosure_blocks_stable() {
        let mut input = baseline_input("packet:m4:freshness_propagation:mixed_epoch");
        input
            .covered_epoch_classes
            .push(GraphEpochClass::MixedEpochUnresolvable);
        let mut mixed_row = sample_row(
            FreshnessPropagationConsumerSurface::ReviewBundle,
            "mixed_epoch",
        );
        mixed_row.epoch_label.epoch_class = GraphEpochClass::MixedEpochUnresolvable;
        mixed_row.freshness = FreshnessClass::Cached;
        mixed_row.partiality_note = Some("mixed epoch".to_owned());
        input.rows.push(mixed_row);
        let packet = FreshnessPropagationPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == FreshnessPropagationFindingKind::MixedEpochUnlabeled));
    }

    #[test]
    fn full_rebuild_without_reason_blocks_stable() {
        let mut input = baseline_input("packet:m4:freshness_propagation:full_rebuild");
        if let Some(row) = input.rows.first_mut() {
            row.invalidation_scope = InvalidationScope {
                invalidation_class: InvalidationScopeClass::FullRebuildWorkspaceEpochBoundary,
                affected_subgraph_refs: vec![],
                full_rebuild_reason: None,
            };
        }
        let packet = FreshnessPropagationPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == FreshnessPropagationFindingKind::FullRebuildNotSurfaced));
    }

    #[test]
    fn hidden_dependency_undisclosed_blocks_stable() {
        let mut input = baseline_input("packet:m4:freshness_propagation:hidden_dep");
        if let Some(row) = input.rows.first_mut() {
            row.hidden_graph_dependency.state =
                HiddenGraphDependencyState::HiddenDependencyUndisclosed;
        }
        let packet = FreshnessPropagationPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == FreshnessPropagationFindingKind::HiddenGraphDependencyUndisclosed));
    }

    #[test]
    fn epoch_drop_in_coverage_blocks_stable() {
        let mut input = baseline_input("packet:m4:freshness_propagation:epoch_drop");
        if let Some(row) = input.rows.first_mut() {
            row.epoch_label.epoch_class = GraphEpochClass::ImportedProvider;
            row.epoch_label.imported_provider_epoch_ref =
                Some("epoch:imported:registry-v1".to_owned());
            row.freshness = FreshnessClass::Imported;
            row.confidence = ConfidenceClass::ImportedAuthoritative;
            row.visibility_scope = VisibilityScopeClass::ImportedExternal;
            row.retention = RetentionClass::Persistent;
            row.partiality_note =
                Some("Imported from external provider; pending refresh.".to_owned());
        }
        let packet = FreshnessPropagationPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == FreshnessPropagationFindingKind::EpochSilentlyDropped));
    }

    #[test]
    fn missing_consumer_projection_blocks_stable() {
        let mut input = baseline_input("packet:m4:freshness_propagation:missing_projection");
        input.consumer_projections.pop();
        let packet = FreshnessPropagationPacket::materialize(input);
        assert_eq!(
            packet.promotion_state,
            FreshnessPropagationPromotionState::BlocksStable
        );
        assert!(packet
            .validation_findings
            .iter()
            .any(|finding| finding.finding_kind
                == FreshnessPropagationFindingKind::MissingConsumerProjection));
    }
}
