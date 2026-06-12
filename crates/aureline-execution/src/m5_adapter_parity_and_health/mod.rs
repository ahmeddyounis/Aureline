//! Canonical M5 adapter-parity-and-health matrix with a non-inheriting health gate that
//! keeps every M5 flow that can mix live build-event data, protocol-backed adapters,
//! imported artifacts, and heuristic fallback explicit about *how* it sourced its
//! execution truth and *how authoritative* that source is.
//!
//! Each [`AdapterHealthRow`] names one M5 flow that consumes adapter-sourced execution
//! truth — pipeline build runs, preview routes, notebook executions, framework tooling
//! actions, incident replays, and support-bundle joins — and answers, for that flow, what
//! kind of adapter sourced its truth ([`AdapterSource`]), how fresh that data is
//! ([`FreshnessState`]), how complete its coverage is ([`CoverageState`]), how stable its
//! connection is ([`ConnectionState`]), and whether the truth was verified
//! ([`VerificationState`]). The row then publishes a [`HealthClass`] no input can exceed.
//!
//! The [`HealthClass`] a flow may publish is the weakest ceiling implied by its observed
//! states, so an imported or heuristic source, a stale or expired snapshot, partial or
//! absent coverage, a reconnecting, bridged, or disconnected connection, or an unverified
//! or unverifiable source all narrow or withhold the published health automatically. The
//! guardrail this enforces: an imported or heuristic adapter can never replace authoritative
//! live state silently just because it is faster or easier to render. A flow whose adapter
//! is only imported or heuristic stays usable but visibly narrower than a live native or
//! protocol-backed adapter, and a stale or partial route is downgraded rather than left
//! quietly green. The [`HealthDecision`] records the gate's action — publish authoritative
//! health, qualify it to an import-backed claim, mark it provisional, or withhold it — and
//! the recomputed [`FallbackReason`]s explain it; all are validated against the gate.
//!
//! The health model is a strip and a receipt, not a status badge. Every flow resolves
//! through one adapter-health strip instead of a hidden per-feature health chip: the row
//! surfaces the adapter, the source kind, freshness, coverage, connection, verification,
//! and a recovery path before the flow consumes the truth, and it exports a machine-readable
//! health receipt so support bundles, issue reports, and release evidence can reconstruct
//! how the product actually sourced execution truth without replaying the flow.
//!
//! The adapter-flow and adapter-source vocabularies are closed and shared. [`AdapterFlow`]
//! is the single controlled vocabulary every M5 flow reuses instead of inventing
//! feature-local health chips, and a flow that joins a support bundle or incident report
//! must carry a support-bundle ref so field triage inherits the same health and fallback
//! truth the user saw.
//!
//! The packet is checked in at
//! `artifacts/execution/m5/m5-adapter-parity-and-health.json` and embedded here. It is
//! metadata-only: every field is a typed state or an opaque ref, and it carries no
//! credential bodies, raw provider payloads, host tokens, or control-plane secrets.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported M5 adapter-parity-and-health matrix schema version.
pub const M5_ADAPTER_HEALTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ADAPTER_HEALTH_RECORD_KIND: &str = "m5_adapter_parity_and_health_matrix";

/// Repo-relative path to the checked-in packet.
pub const M5_ADAPTER_HEALTH_PATH: &str = "artifacts/execution/m5/m5-adapter-parity-and-health.json";

/// Embedded checked-in packet JSON.
pub const M5_ADAPTER_HEALTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/execution/m5/m5-adapter-parity-and-health.json"
));

/// An M5 flow that consumes adapter-sourced execution truth and so must resolve its
/// health through the adapter-health strip before it renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterFlow {
    /// Pipeline build run driven by a build-event stream.
    PipelineBuildRun,
    /// Preview-route execution.
    PreviewRoute,
    /// Notebook execution.
    NotebookExecution,
    /// Framework tooling action.
    FrameworkToolingAction,
    /// Incident replay or incident-linked rerun.
    IncidentReplay,
    /// Support-bundle join.
    SupportBundleJoin,
}

impl AdapterFlow {
    /// Every adapter flow, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PipelineBuildRun,
        Self::PreviewRoute,
        Self::NotebookExecution,
        Self::FrameworkToolingAction,
        Self::IncidentReplay,
        Self::SupportBundleJoin,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PipelineBuildRun => "pipeline_build_run",
            Self::PreviewRoute => "preview_route",
            Self::NotebookExecution => "notebook_execution",
            Self::FrameworkToolingAction => "framework_tooling_action",
            Self::IncidentReplay => "incident_replay",
            Self::SupportBundleJoin => "support_bundle_join",
        }
    }

    /// Whether this flow joins a support bundle or incident report and therefore must carry
    /// a support-bundle ref so field triage inherits the same health and fallback truth.
    ///
    /// This is the pinned relationship the gate validates a support join against, so an
    /// incident or support surface can never restate adapter health by hand instead of
    /// joining the canonical packet.
    pub const fn joins_support_bundle(self) -> bool {
        matches!(self, Self::IncidentReplay | Self::SupportBundleJoin)
    }
}

/// How authoritative the execution truth a flow publishes is.
///
/// Ordered low-to-high by [`HealthClass::rank`]: an [`HealthClass::Unavailable`] flow has
/// no usable adapter truth, and an [`HealthClass::LiveAuthoritative`] flow is backed by a
/// live native or protocol-backed adapter that is fresh, complete, connected, and verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthClass {
    /// Backed by a live native or protocol-backed authoritative adapter.
    LiveAuthoritative,
    /// Backed by a structured or imported artifact; usable but narrower than live.
    ImportQualified,
    /// Backed by heuristic inference or degraded data; narrowest usable claim.
    HeuristicProvisional,
    /// No usable adapter truth; the health is withheld.
    Unavailable,
}

impl HealthClass {
    /// Every health class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveAuthoritative,
        Self::ImportQualified,
        Self::HeuristicProvisional,
        Self::Unavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveAuthoritative => "live_authoritative",
            Self::ImportQualified => "import_qualified",
            Self::HeuristicProvisional => "heuristic_provisional",
            Self::Unavailable => "unavailable",
        }
    }

    /// Monotonic rank; higher means more authoritative.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::HeuristicProvisional => 1,
            Self::ImportQualified => 2,
            Self::LiveAuthoritative => 3,
        }
    }

    /// The weaker (lower-rank) of two health classes.
    pub const fn min(self, other: Self) -> Self {
        if other.rank() < self.rank() {
            other
        } else {
            self
        }
    }
}

/// The kind of adapter that sourced a flow's execution truth.
///
/// These states are the guardrail against silent substitution: a flow sourced from an
/// imported artifact or heuristic inference can never publish live authoritative health,
/// so an imported or heuristic state never replaces a live native or protocol-backed feed
/// merely because it is faster or easier to render.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdapterSource {
    /// A live native adapter (e.g. a build-event stream).
    Native,
    /// A protocol-backed adapter; authoritative in the same way a native one is.
    ProtocolBacked,
    /// A structured import; a qualified source, capped at import-qualified.
    StructuredImport,
    /// An imported artifact; a qualified source, capped at import-qualified.
    Imported,
    /// Heuristic inference; capped at heuristic-provisional.
    Heuristic,
}

impl AdapterSource {
    /// Every adapter source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Native,
        Self::ProtocolBacked,
        Self::StructuredImport,
        Self::Imported,
        Self::Heuristic,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Native => "native",
            Self::ProtocolBacked => "protocol_backed",
            Self::StructuredImport => "structured_import",
            Self::Imported => "imported",
            Self::Heuristic => "heuristic",
        }
    }

    /// Highest health class this adapter source permits a flow to publish.
    pub const fn health_ceiling(self) -> HealthClass {
        match self {
            Self::Native | Self::ProtocolBacked => HealthClass::LiveAuthoritative,
            Self::StructuredImport | Self::Imported => HealthClass::ImportQualified,
            Self::Heuristic => HealthClass::HeuristicProvisional,
        }
    }

    /// Whether this source is a live, authoritative adapter rather than an artifact or
    /// heuristic fallback.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::Native | Self::ProtocolBacked)
    }

    /// Whether this source must carry a source-snapshot ref backing its imported or
    /// heuristic truth.
    pub const fn requires_snapshot(self) -> bool {
        matches!(
            self,
            Self::StructuredImport | Self::Imported | Self::Heuristic
        )
    }

    /// Whether this state raises the [`FallbackReason::ImportedArtifact`] trigger.
    pub const fn is_imported_trigger(self) -> bool {
        matches!(self, Self::Imported)
    }

    /// Whether this state raises the [`FallbackReason::HeuristicInference`] trigger.
    pub const fn is_heuristic_trigger(self) -> bool {
        matches!(self, Self::Heuristic)
    }
}

/// How fresh the adapter data backing a flow is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessState {
    /// The data is live and current.
    Live,
    /// The data is recent and within tolerance.
    Recent,
    /// The data is stale; caps at heuristic-provisional.
    Stale,
    /// The data is expired; caps at unavailable.
    Expired,
}

impl FreshnessState {
    /// Every freshness state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Live, Self::Recent, Self::Stale, Self::Expired];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::Expired => "expired",
        }
    }

    /// Highest health class this freshness state permits a flow to publish.
    pub const fn health_ceiling(self) -> HealthClass {
        match self {
            Self::Live | Self::Recent => HealthClass::LiveAuthoritative,
            Self::Stale => HealthClass::HeuristicProvisional,
            Self::Expired => HealthClass::Unavailable,
        }
    }

    /// Whether this state raises the [`FallbackReason::StaleSnapshot`] trigger.
    pub const fn is_stale_trigger(self) -> bool {
        matches!(self, Self::Stale | Self::Expired)
    }
}

/// How complete the adapter's coverage of the flow's execution truth is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageState {
    /// Coverage is complete.
    Complete,
    /// Coverage is partial; caps at import-qualified.
    Partial,
    /// Coverage is degraded; caps at heuristic-provisional.
    Degraded,
    /// Coverage is absent; caps at unavailable.
    Absent,
}

impl CoverageState {
    /// Every coverage state, in declaration order.
    pub const ALL: [Self; 4] = [Self::Complete, Self::Partial, Self::Degraded, Self::Absent];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Partial => "partial",
            Self::Degraded => "degraded",
            Self::Absent => "absent",
        }
    }

    /// Highest health class this coverage state permits a flow to publish.
    pub const fn health_ceiling(self) -> HealthClass {
        match self {
            Self::Complete => HealthClass::LiveAuthoritative,
            Self::Partial => HealthClass::ImportQualified,
            Self::Degraded => HealthClass::HeuristicProvisional,
            Self::Absent => HealthClass::Unavailable,
        }
    }

    /// Whether this state raises the [`FallbackReason::PartialCoverage`] trigger.
    pub const fn is_partial_trigger(self) -> bool {
        matches!(self, Self::Partial | Self::Degraded | Self::Absent)
    }
}

/// How stable the adapter's connection to the live truth is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    /// The adapter is connected directly to the live truth.
    Connected,
    /// The adapter is reconnecting; caps at import-qualified.
    Reconnecting,
    /// The adapter is reached only over a bridge; caps at heuristic-provisional.
    Bridged,
    /// The adapter is disconnected; caps at unavailable.
    Disconnected,
}

impl ConnectionState {
    /// Every connection state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::Reconnecting,
        Self::Bridged,
        Self::Disconnected,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::Reconnecting => "reconnecting",
            Self::Bridged => "bridged",
            Self::Disconnected => "disconnected",
        }
    }

    /// Highest health class this connection state permits a flow to publish.
    pub const fn health_ceiling(self) -> HealthClass {
        match self {
            Self::Connected => HealthClass::LiveAuthoritative,
            Self::Reconnecting => HealthClass::ImportQualified,
            Self::Bridged => HealthClass::HeuristicProvisional,
            Self::Disconnected => HealthClass::Unavailable,
        }
    }

    /// Whether this state raises the [`FallbackReason::ConnectionUnstable`] trigger.
    pub const fn is_unstable_trigger(self) -> bool {
        matches!(
            self,
            Self::Reconnecting | Self::Bridged | Self::Disconnected
        )
    }
}

/// Whether the adapter truth backing a flow was verified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    /// The truth was verified against the live adapter.
    Verified,
    /// The truth is attested by a recorded snapshot; caps at import-qualified.
    Attested,
    /// The truth is unverified; caps at heuristic-provisional.
    Unverified,
    /// The truth cannot be verified; caps at unavailable.
    Unverifiable,
}

impl VerificationState {
    /// Every verification state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Attested,
        Self::Unverified,
        Self::Unverifiable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Attested => "attested",
            Self::Unverified => "unverified",
            Self::Unverifiable => "unverifiable",
        }
    }

    /// Highest health class this verification state permits a flow to publish.
    pub const fn health_ceiling(self) -> HealthClass {
        match self {
            Self::Verified => HealthClass::LiveAuthoritative,
            Self::Attested => HealthClass::ImportQualified,
            Self::Unverified => HealthClass::HeuristicProvisional,
            Self::Unverifiable => HealthClass::Unavailable,
        }
    }

    /// Whether this state raises the [`FallbackReason::UnverifiedSource`] trigger.
    pub const fn is_unverified_trigger(self) -> bool {
        matches!(self, Self::Unverified | Self::Unverifiable)
    }
}

/// The recovery path surfaced when a flow's health is narrowed or withheld.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPath {
    /// Wait for the live adapter to reconnect and republish authoritative health.
    AwaitLiveAdapter,
    /// Re-import the structured or imported artifact.
    ReimportArtifact,
    /// Open the flow in the provider's surface.
    OpenInProvider,
    /// No recovery is needed; only valid when the flow publishes authoritative health.
    #[serde(rename = "none")]
    NoneNeeded,
}

impl RecoveryPath {
    /// Every recovery path, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AwaitLiveAdapter,
        Self::ReimportArtifact,
        Self::OpenInProvider,
        Self::NoneNeeded,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AwaitLiveAdapter => "await_live_adapter",
            Self::ReimportArtifact => "reimport_artifact",
            Self::OpenInProvider => "open_in_provider",
            Self::NoneNeeded => "none",
        }
    }

    /// Whether this is a real recovery path the user can take.
    pub const fn is_offered(self) -> bool {
        !matches!(self, Self::NoneNeeded)
    }
}

/// A headline reason the health gate narrows a flow.
///
/// These are the canonical fallback reasons: an imported artifact, heuristic inference, a
/// stale snapshot, partial coverage, an unstable connection, and an unverified source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackReason {
    /// The flow is driven by an imported artifact rather than a live adapter.
    ImportedArtifact,
    /// The flow is driven by heuristic inference.
    HeuristicInference,
    /// The adapter snapshot is stale or expired.
    StaleSnapshot,
    /// The adapter coverage is partial, degraded, or absent.
    PartialCoverage,
    /// The adapter connection is reconnecting, bridged, or disconnected.
    ConnectionUnstable,
    /// The adapter truth is unverified or unverifiable.
    UnverifiedSource,
}

impl FallbackReason {
    /// Every fallback reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ImportedArtifact,
        Self::HeuristicInference,
        Self::StaleSnapshot,
        Self::PartialCoverage,
        Self::ConnectionUnstable,
        Self::UnverifiedSource,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedArtifact => "imported_artifact",
            Self::HeuristicInference => "heuristic_inference",
            Self::StaleSnapshot => "stale_snapshot",
            Self::PartialCoverage => "partial_coverage",
            Self::ConnectionUnstable => "connection_unstable",
            Self::UnverifiedSource => "unverified_source",
        }
    }
}

/// The action the health gate takes on a flow relative to a clean authoritative publish.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthDecision {
    /// No narrowing; the flow publishes live authoritative health.
    Publish,
    /// The flow publishes an import-qualified claim.
    Qualify,
    /// The flow publishes a heuristic-provisional claim.
    Provisional,
    /// The flow's health is withheld; no usable truth.
    Withhold,
}

impl HealthDecision {
    /// Every health decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Publish,
        Self::Qualify,
        Self::Provisional,
        Self::Withhold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Publish => "publish",
            Self::Qualify => "qualify",
            Self::Provisional => "provisional",
            Self::Withhold => "withhold",
        }
    }

    /// Whether the gate narrowed or withheld the flow's health.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::Publish)
    }

    /// The decision implied by a published health class.
    pub const fn for_health(health: HealthClass) -> Self {
        match health {
            HealthClass::LiveAuthoritative => Self::Publish,
            HealthClass::ImportQualified => Self::Qualify,
            HealthClass::HeuristicProvisional => Self::Provisional,
            HealthClass::Unavailable => Self::Withhold,
        }
    }
}

/// One adapter-parity-and-health row for an M5 flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdapterHealthRow {
    /// Stable adapter-health flow id.
    pub flow_id: String,
    /// M5 flow this row governs.
    pub adapter_flow: AdapterFlow,
    /// Kind of adapter that sourced the flow's execution truth.
    pub adapter_source: AdapterSource,
    /// How fresh the adapter data is.
    pub freshness: FreshnessState,
    /// How complete the adapter's coverage is.
    pub coverage: CoverageState,
    /// How stable the adapter's connection is.
    pub connection: ConnectionState,
    /// Whether the adapter truth was verified.
    pub verification: VerificationState,
    /// Health the flow's own evidence asserts, before the gate.
    pub declared_health: HealthClass,
    /// Health actually published after the gate narrows the flow.
    ///
    /// Must equal [`AdapterHealthRow::effective_health`].
    pub published_health: HealthClass,
    /// Decision the gate takes; must equal the recomputed decision.
    pub health_decision: HealthDecision,
    /// Headline fallback reasons; must equal the recomputed set.
    #[serde(default)]
    pub fallback_reasons: Vec<FallbackReason>,
    /// Recovery path surfaced when the health is narrowed or withheld.
    pub recovery_path: RecoveryPath,
    /// Ref to the adapter the flow's truth was sourced from.
    pub adapter_ref: String,
    /// Ref to the target context the flow runs against.
    pub target_context_ref: String,
    /// Ref to the adapter-health strip the user saw.
    pub health_strip_ref: String,
    /// Ref to the machine-readable health receipt for support, audit, and release evidence.
    pub health_receipt_ref: String,
    /// Ref to the source snapshot; required when the source is structured-import, imported,
    /// or heuristic.
    #[serde(default)]
    pub source_snapshot_ref: String,
    /// Ref to the support bundle; required when the flow joins a support bundle or incident
    /// report.
    #[serde(default)]
    pub support_bundle_ref: String,
    /// Ref to the in-product execution this health applies to.
    pub execution_ref: String,
    /// Ref binding this row into desktop, CLI, support, and release surfaces.
    pub support_export_ref: String,
    /// Additional source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl AdapterHealthRow {
    /// The health the flow's own evidence asserted, before environmental narrowing.
    pub fn capability_floor(&self) -> HealthClass {
        self.declared_health
    }

    /// The health the gate permits this flow to publish.
    ///
    /// Lowers the capability floor to the weakest ceiling implied by the adapter source,
    /// freshness, coverage, connection, and verification states, so an imported or
    /// heuristic source, a stale snapshot, partial coverage, an unstable connection, or an
    /// unverified source can never publish live authoritative health.
    pub fn effective_health(&self) -> HealthClass {
        self.capability_floor()
            .min(self.adapter_source.health_ceiling())
            .min(self.freshness.health_ceiling())
            .min(self.coverage.health_ceiling())
            .min(self.connection.health_ceiling())
            .min(self.verification.health_ceiling())
    }

    /// The headline fallback reasons recomputed from the flow's observed states.
    pub fn computed_fallback_reasons(&self) -> Vec<FallbackReason> {
        let mut reasons = Vec::new();
        if self.adapter_source.is_imported_trigger() {
            reasons.push(FallbackReason::ImportedArtifact);
        }
        if self.adapter_source.is_heuristic_trigger() {
            reasons.push(FallbackReason::HeuristicInference);
        }
        if self.freshness.is_stale_trigger() {
            reasons.push(FallbackReason::StaleSnapshot);
        }
        if self.coverage.is_partial_trigger() {
            reasons.push(FallbackReason::PartialCoverage);
        }
        if self.connection.is_unstable_trigger() {
            reasons.push(FallbackReason::ConnectionUnstable);
        }
        if self.verification.is_unverified_trigger() {
            reasons.push(FallbackReason::UnverifiedSource);
        }
        reasons
    }

    /// The decision the gate must record for this flow, derived from its effective health.
    pub fn required_decision(&self) -> HealthDecision {
        HealthDecision::for_health(self.effective_health())
    }

    /// Whether the flow publishes live authoritative health.
    pub fn is_authoritative(&self) -> bool {
        self.effective_health() == HealthClass::LiveAuthoritative
    }

    /// Whether the gate narrowed the published health below what the flow declared.
    ///
    /// This is the automatic downgrade: a stale, partial, imported, or heuristic flow that
    /// declared a stronger claim has its published health lowered rather than left green.
    pub fn is_downgraded(&self) -> bool {
        self.effective_health().rank() < self.capability_floor().rank()
    }

    /// Whether the flow carries its own non-empty adapter, target, strip, receipt,
    /// execution, and support-export refs.
    pub fn has_required_evidence(&self) -> bool {
        !self.adapter_ref.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && !self.health_strip_ref.trim().is_empty()
            && !self.health_receipt_ref.trim().is_empty()
            && !self.execution_ref.trim().is_empty()
            && !self.support_export_ref.trim().is_empty()
    }

    /// Whether the stored published health, decision, and fallback reasons all agree with
    /// the recomputed gate decision.
    pub fn gate_consistent(&self) -> bool {
        self.published_health == self.effective_health()
            && self.health_decision == self.required_decision()
            && self.fallback_reasons == self.computed_fallback_reasons()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdapterHealthSummary {
    /// Total flow rows.
    pub total_flows: usize,
    /// Number of claimed flows.
    pub flow_count: usize,
    /// Flows published with live authoritative health.
    pub authoritative_flows: usize,
    /// Flows published with import-qualified health.
    pub import_qualified_flows: usize,
    /// Flows published with heuristic-provisional health.
    pub heuristic_provisional_flows: usize,
    /// Flows published as unavailable.
    pub unavailable_flows: usize,
    /// Flows the gate cleared to publish authoritative health.
    pub published_flows: usize,
    /// Flows the gate narrowed to a qualified claim.
    pub qualified_flows: usize,
    /// Flows the gate narrowed to a provisional claim.
    pub provisional_flows: usize,
    /// Flows the gate withheld entirely.
    pub withheld_flows: usize,
    /// Flows whose published health was downgraded below what they declared.
    pub downgraded_flows: usize,
    /// Flows that join a support bundle or incident report.
    pub export_join_flows: usize,
    /// Flows backed by a live native or protocol-backed adapter.
    pub live_source_flows: usize,
    /// Flows backed by an imported or heuristic source.
    pub imported_or_heuristic_flows: usize,
    /// Flows whose snapshot is stale or expired.
    pub stale_flows: usize,
    /// Flows carrying at least one fallback reason.
    pub flows_with_fallback_reasons: usize,
}

/// A redaction-safe export row projected from an adapter-health row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterHealthExportRow {
    /// Adapter-health flow id.
    pub flow_id: String,
    /// Adapter-flow token.
    pub adapter_flow: String,
    /// Adapter-source token.
    pub adapter_source: String,
    /// Freshness-state token.
    pub freshness: String,
    /// Coverage-state token.
    pub coverage: String,
    /// Connection-state token.
    pub connection: String,
    /// Verification-state token.
    pub verification: String,
    /// Declared-health token.
    pub declared_health: String,
    /// Published-health token.
    pub published_health: String,
    /// Health-decision token.
    pub health_decision: String,
    /// Fallback-reason tokens.
    pub fallback_reasons: Vec<String>,
    /// Recovery-path token.
    pub recovery_path: String,
    /// Adapter ref.
    pub adapter_ref: String,
    /// Health-receipt ref.
    pub health_receipt_ref: String,
    /// Execution ref the health applies to.
    pub execution_ref: String,
    /// Support-bundle ref, when the flow joins a bundle.
    pub support_bundle_ref: String,
    /// Whether the flow joins a support bundle or incident report.
    pub joins_support_bundle: bool,
    /// Whether the flow publishes live authoritative health.
    pub authoritative: bool,
    /// Whether the published health was downgraded below the declared claim.
    pub downgraded: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AdapterHealthExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Projected rows.
    pub flows: Vec<M5AdapterHealthExportRow>,
    /// Whether every flow's published health and decision agree with the gate.
    pub all_flows_gate_consistent: bool,
    /// Flows that publish live authoritative health.
    pub authoritative_count: usize,
    /// Flows the gate narrowed or withheld.
    pub narrowed_count: usize,
    /// Flows the gate withheld entirely.
    pub withheld_count: usize,
}

/// The typed M5 adapter-parity-and-health matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AdapterHealthMatrix {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Claimed flows; one row per flow.
    pub adapter_flows: Vec<AdapterFlow>,
    /// Closed health-class vocabulary.
    pub health_classes: Vec<HealthClass>,
    /// Closed adapter-source vocabulary.
    pub adapter_sources: Vec<AdapterSource>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessState>,
    /// Closed coverage-state vocabulary.
    pub coverage_states: Vec<CoverageState>,
    /// Closed connection-state vocabulary.
    pub connection_states: Vec<ConnectionState>,
    /// Closed verification-state vocabulary.
    pub verification_states: Vec<VerificationState>,
    /// Closed recovery-path vocabulary.
    pub recovery_paths: Vec<RecoveryPath>,
    /// Closed fallback-reason vocabulary.
    pub fallback_reasons: Vec<FallbackReason>,
    /// Closed health-decision vocabulary.
    pub health_decisions: Vec<HealthDecision>,
    /// Adapter-health rows, one per claimed flow.
    #[serde(default)]
    pub flows: Vec<AdapterHealthRow>,
    /// Summary counts.
    pub summary: M5AdapterHealthSummary,
}

impl M5AdapterHealthMatrix {
    /// Returns the row for a claimed flow.
    pub fn flow(&self, flow: AdapterFlow) -> Option<&AdapterHealthRow> {
        self.flows.iter().find(|f| f.adapter_flow == flow)
    }

    /// Flows that publish live authoritative health.
    pub fn authoritative_flows(&self) -> impl Iterator<Item = &AdapterHealthRow> {
        self.flows.iter().filter(|f| f.is_authoritative())
    }

    /// Flows the gate narrowed or withheld in any way.
    pub fn narrowed_flows(&self) -> impl Iterator<Item = &AdapterHealthRow> {
        self.flows
            .iter()
            .filter(|f| f.required_decision().is_narrowed())
    }

    /// Flows the gate withheld entirely.
    pub fn withheld_flows(&self) -> impl Iterator<Item = &AdapterHealthRow> {
        self.flows
            .iter()
            .filter(|f| f.required_decision() == HealthDecision::Withhold)
    }

    /// Whether every flow's stored published health, decision, and reasons agree with the
    /// recomputed gate decision.
    pub fn all_flows_gate_consistent(&self) -> bool {
        self.flows.iter().all(|f| f.gate_consistent())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> M5AdapterHealthSummary {
        let count_published = |health: HealthClass| {
            self.flows
                .iter()
                .filter(|f| f.published_health == health)
                .count()
        };
        let count_decision = |decision: HealthDecision| {
            self.flows
                .iter()
                .filter(|f| f.health_decision == decision)
                .count()
        };
        M5AdapterHealthSummary {
            total_flows: self.flows.len(),
            flow_count: self.adapter_flows.len(),
            authoritative_flows: count_published(HealthClass::LiveAuthoritative),
            import_qualified_flows: count_published(HealthClass::ImportQualified),
            heuristic_provisional_flows: count_published(HealthClass::HeuristicProvisional),
            unavailable_flows: count_published(HealthClass::Unavailable),
            published_flows: count_decision(HealthDecision::Publish),
            qualified_flows: count_decision(HealthDecision::Qualify),
            provisional_flows: count_decision(HealthDecision::Provisional),
            withheld_flows: count_decision(HealthDecision::Withhold),
            downgraded_flows: self.flows.iter().filter(|f| f.is_downgraded()).count(),
            export_join_flows: self
                .flows
                .iter()
                .filter(|f| f.adapter_flow.joins_support_bundle())
                .count(),
            live_source_flows: self
                .flows
                .iter()
                .filter(|f| f.adapter_source.is_live())
                .count(),
            imported_or_heuristic_flows: self
                .flows
                .iter()
                .filter(|f| {
                    f.adapter_source.is_imported_trigger()
                        || f.adapter_source.is_heuristic_trigger()
                })
                .count(),
            stale_flows: self
                .flows
                .iter()
                .filter(|f| f.freshness.is_stale_trigger())
                .count(),
            flows_with_fallback_reasons: self
                .flows
                .iter()
                .filter(|f| !f.fallback_reasons.is_empty())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces — desktop and CLI
    /// adapter-health strips, pipeline, preview, notebook, framework, and incident lanes,
    /// support bundles, issue reports, and release/audit evidence — render instead of
    /// restating each flow's health posture by hand.
    pub fn export_projection(&self) -> M5AdapterHealthExportProjection {
        let flows = self
            .flows
            .iter()
            .map(|f| M5AdapterHealthExportRow {
                flow_id: f.flow_id.clone(),
                adapter_flow: f.adapter_flow.as_str().to_owned(),
                adapter_source: f.adapter_source.as_str().to_owned(),
                freshness: f.freshness.as_str().to_owned(),
                coverage: f.coverage.as_str().to_owned(),
                connection: f.connection.as_str().to_owned(),
                verification: f.verification.as_str().to_owned(),
                declared_health: f.declared_health.as_str().to_owned(),
                published_health: f.published_health.as_str().to_owned(),
                health_decision: f.health_decision.as_str().to_owned(),
                fallback_reasons: f
                    .fallback_reasons
                    .iter()
                    .map(|r| r.as_str().to_owned())
                    .collect(),
                recovery_path: f.recovery_path.as_str().to_owned(),
                adapter_ref: f.adapter_ref.clone(),
                health_receipt_ref: f.health_receipt_ref.clone(),
                execution_ref: f.execution_ref.clone(),
                support_bundle_ref: f.support_bundle_ref.clone(),
                joins_support_bundle: f.adapter_flow.joins_support_bundle(),
                authoritative: f.is_authoritative(),
                downgraded: f.is_downgraded(),
                summary: format!(
                    "{}: source {}, freshness {}, coverage {}, connection {}, verification {}, declared {}, published {} ({}), recovery {}",
                    f.adapter_flow.as_str(),
                    f.adapter_source.as_str(),
                    f.freshness.as_str(),
                    f.coverage.as_str(),
                    f.connection.as_str(),
                    f.verification.as_str(),
                    f.declared_health.as_str(),
                    f.published_health.as_str(),
                    f.health_decision.as_str(),
                    f.recovery_path.as_str()
                ),
            })
            .collect();
        M5AdapterHealthExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            flows,
            all_flows_gate_consistent: self.all_flows_gate_consistent(),
            authoritative_count: self.authoritative_flows().count(),
            narrowed_count: self.narrowed_flows().count(),
            withheld_count: self.withheld_flows().count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5AdapterHealthViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let claimed: BTreeSet<AdapterFlow> = self.adapter_flows.iter().copied().collect();

        let mut seen_ids = BTreeSet::new();
        let mut seen_flows = BTreeSet::new();
        for row in &self.flows {
            if !seen_ids.insert(row.flow_id.clone()) {
                violations.push(M5AdapterHealthViolation::DuplicateFlowId {
                    flow_id: row.flow_id.clone(),
                });
            }
            if !seen_flows.insert(row.adapter_flow) {
                violations.push(M5AdapterHealthViolation::DuplicateFlowRow {
                    flow: row.adapter_flow.as_str(),
                });
            }
            if !claimed.contains(&row.adapter_flow) {
                violations.push(M5AdapterHealthViolation::UnclaimedFlowRow {
                    flow_id: row.flow_id.clone(),
                    flow: row.adapter_flow.as_str(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        // Every claimed flow must carry its own row, so a flow never inherits a live
        // authoritative claim from an adjacent one.
        for &flow in &self.adapter_flows {
            if !seen_flows.contains(&flow) {
                violations.push(M5AdapterHealthViolation::MissingFlowRow {
                    flow: flow.as_str(),
                });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(M5AdapterHealthViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AdapterHealthViolation>) {
        if self.schema_version != M5_ADAPTER_HEALTH_SCHEMA_VERSION {
            violations.push(M5AdapterHealthViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_ADAPTER_HEALTH_RECORD_KIND {
            violations.push(M5AdapterHealthViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AdapterHealthViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "adapter_flows",
                self.adapter_flows == AdapterFlow::ALL.to_vec(),
            ),
            (
                "health_classes",
                self.health_classes == HealthClass::ALL.to_vec(),
            ),
            (
                "adapter_sources",
                self.adapter_sources == AdapterSource::ALL.to_vec(),
            ),
            (
                "freshness_states",
                self.freshness_states == FreshnessState::ALL.to_vec(),
            ),
            (
                "coverage_states",
                self.coverage_states == CoverageState::ALL.to_vec(),
            ),
            (
                "connection_states",
                self.connection_states == ConnectionState::ALL.to_vec(),
            ),
            (
                "verification_states",
                self.verification_states == VerificationState::ALL.to_vec(),
            ),
            (
                "recovery_paths",
                self.recovery_paths == RecoveryPath::ALL.to_vec(),
            ),
            (
                "fallback_reasons",
                self.fallback_reasons == FallbackReason::ALL.to_vec(),
            ),
            (
                "health_decisions",
                self.health_decisions == HealthDecision::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5AdapterHealthViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_row(&self, row: &AdapterHealthRow, violations: &mut Vec<M5AdapterHealthViolation>) {
        for (field, value) in [
            ("flow_id", &row.flow_id),
            ("adapter_ref", &row.adapter_ref),
            ("target_context_ref", &row.target_context_ref),
            ("health_strip_ref", &row.health_strip_ref),
            ("health_receipt_ref", &row.health_receipt_ref),
            ("execution_ref", &row.execution_ref),
            ("support_export_ref", &row.support_export_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AdapterHealthViolation::EmptyField {
                    id: row.flow_id.clone(),
                    field_name: field,
                });
            }
        }

        // A structured-import, imported, or heuristic source must carry its source-snapshot
        // ref so the imported truth the strip showed can always be inspected.
        if row.adapter_source.requires_snapshot() && row.source_snapshot_ref.trim().is_empty() {
            violations.push(M5AdapterHealthViolation::EmptyField {
                id: row.flow_id.clone(),
                field_name: "source_snapshot_ref",
            });
        }

        // A flow that joins a support bundle or incident report must carry its
        // support-bundle ref so field triage inherits the same health and fallback truth.
        if row.adapter_flow.joins_support_bundle() && row.support_bundle_ref.trim().is_empty() {
            violations.push(M5AdapterHealthViolation::SupportJoinMissing {
                flow_id: row.flow_id.clone(),
                flow: row.adapter_flow.as_str(),
            });
        }

        let mut seen_reasons = BTreeSet::new();
        for reason in &row.fallback_reasons {
            if !seen_reasons.insert(*reason) {
                violations.push(M5AdapterHealthViolation::DuplicateFallbackReason {
                    flow_id: row.flow_id.clone(),
                    reason: reason.as_str(),
                });
            }
        }

        // The published health must equal the gate's recomputed ceiling, so an imported,
        // heuristic, stale, partial, or unverified flow can never read as live authoritative.
        let effective = row.effective_health();
        if row.published_health != effective {
            violations.push(M5AdapterHealthViolation::OverstatedHealth {
                flow_id: row.flow_id.clone(),
                published: row.published_health.as_str(),
                computed: effective.as_str(),
            });
        }

        // The recorded decision must match the gate's recomputed decision.
        let required = row.required_decision();
        if row.health_decision != required {
            violations.push(M5AdapterHealthViolation::DecisionMismatch {
                flow_id: row.flow_id.clone(),
                declared: row.health_decision.as_str(),
                required: required.as_str(),
            });
        }

        // The recorded fallback reasons must equal the reasons recomputed from the observed
        // states, so a fallback can never be asserted or hidden by hand.
        let computed = row.computed_fallback_reasons();
        if row.fallback_reasons != computed {
            violations.push(M5AdapterHealthViolation::FallbackReasonsMismatch {
                flow_id: row.flow_id.clone(),
            });
        }

        // A narrowed or withheld flow must offer a real recovery path, so a degraded flow
        // never drops its recovery semantics.
        if row.health_decision.is_narrowed() && !row.recovery_path.is_offered() {
            violations.push(M5AdapterHealthViolation::MissingRecovery {
                flow_id: row.flow_id.clone(),
            });
        }

        // An authoritative flow must be genuinely clean: a live authoritative ceiling on
        // every input, a live authoritative capability floor, and no fallback reason. This
        // is the non-substitution guardrail — imported or heuristic state can never present
        // as authoritative live truth.
        if row.is_authoritative()
            && (row.adapter_source.health_ceiling() != HealthClass::LiveAuthoritative
                || row.freshness.health_ceiling() != HealthClass::LiveAuthoritative
                || row.coverage.health_ceiling() != HealthClass::LiveAuthoritative
                || row.connection.health_ceiling() != HealthClass::LiveAuthoritative
                || row.verification.health_ceiling() != HealthClass::LiveAuthoritative
                || row.capability_floor() != HealthClass::LiveAuthoritative
                || !row.fallback_reasons.is_empty())
        {
            violations.push(M5AdapterHealthViolation::AuthoritativeFlowNotClean {
                flow_id: row.flow_id.clone(),
            });
        }
    }
}

/// A validation violation for the M5 adapter-parity-and-health packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AdapterHealthViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// An adapter-health flow id appears more than once.
    DuplicateFlowId {
        /// Duplicate flow id.
        flow_id: String,
    },
    /// A claimed flow carries more than one row.
    DuplicateFlowRow {
        /// Flow token.
        flow: &'static str,
    },
    /// A claimed flow has no row.
    MissingFlowRow {
        /// Flow token.
        flow: &'static str,
    },
    /// A row covers a flow the packet does not claim.
    UnclaimedFlowRow {
        /// Row id.
        flow_id: String,
        /// Flow token.
        flow: &'static str,
    },
    /// A support-joining flow does not carry a support-bundle ref.
    SupportJoinMissing {
        /// Row id.
        flow_id: String,
        /// Flow token.
        flow: &'static str,
    },
    /// A row lists a fallback reason more than once.
    DuplicateFallbackReason {
        /// Row id.
        flow_id: String,
        /// Reason token.
        reason: &'static str,
    },
    /// A flow publishes a health beyond what its evidence supports.
    OverstatedHealth {
        /// Row id.
        flow_id: String,
        /// Published health token.
        published: &'static str,
        /// Computed effective health token.
        computed: &'static str,
    },
    /// A flow's decision disagrees with its gate decision.
    DecisionMismatch {
        /// Row id.
        flow_id: String,
        /// Declared decision token.
        declared: &'static str,
        /// Required decision token.
        required: &'static str,
    },
    /// A flow's fallback reasons disagree with the recomputed reasons.
    FallbackReasonsMismatch {
        /// Row id.
        flow_id: String,
    },
    /// A narrowed or withheld flow offers no recovery path.
    MissingRecovery {
        /// Row id.
        flow_id: String,
    },
    /// An authoritative flow still carries a fallback reason or a non-clean state.
    AuthoritativeFlowNotClean {
        /// Row id.
        flow_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for M5AdapterHealthViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateFlowId { flow_id } => {
                write!(f, "duplicate flow id {flow_id}")
            }
            Self::DuplicateFlowRow { flow } => {
                write!(f, "duplicate row for flow {flow}")
            }
            Self::MissingFlowRow { flow } => {
                write!(f, "missing row for claimed flow {flow}")
            }
            Self::UnclaimedFlowRow { flow_id, flow } => {
                write!(f, "row {flow_id} covers unclaimed flow {flow}")
            }
            Self::SupportJoinMissing { flow_id, flow } => {
                write!(
                    f,
                    "row {flow_id} joins support flow {flow} but carries no support-bundle ref"
                )
            }
            Self::DuplicateFallbackReason { flow_id, reason } => {
                write!(f, "row {flow_id} repeats fallback reason {reason}")
            }
            Self::OverstatedHealth {
                flow_id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "row {flow_id} publishes health {published} but the gate computes {computed}"
                )
            }
            Self::DecisionMismatch {
                flow_id,
                declared,
                required,
            } => {
                write!(
                    f,
                    "row {flow_id} records decision {declared} but the gate requires {required}"
                )
            }
            Self::FallbackReasonsMismatch { flow_id } => {
                write!(f, "row {flow_id} fallback reasons disagree with the gate")
            }
            Self::MissingRecovery { flow_id } => {
                write!(
                    f,
                    "row {flow_id} is narrowed or withheld but offers no recovery path"
                )
            }
            Self::AuthoritativeFlowNotClean { flow_id } => {
                write!(
                    f,
                    "row {flow_id} is authoritative but carries a fallback reason or non-clean state"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for M5AdapterHealthViolation {}

/// Loads the embedded M5 adapter-parity-and-health matrix packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5AdapterHealthMatrix`].
pub fn current_m5_adapter_health_matrix() -> Result<M5AdapterHealthMatrix, serde_json::Error> {
    serde_json::from_str(M5_ADAPTER_HEALTH_JSON)
}

#[cfg(test)]
mod tests;
