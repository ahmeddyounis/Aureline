//! Cross-surface subscription contract: one typed publish/subscribe
//! fabric shared by the shell, search, graph, AI, review, and support
//! surfaces.
//!
//! Where [`crate::m5_reactive_governance`] freezes the *declarative
//! matrix* of which authority owns each reactive surface and how a
//! claim is narrowed, this module materializes the *runtime contract*:
//! a small, deterministic bus that takes one frame from an owning
//! authority, builds the canonical [`crate::envelope::SubscriptionEnvelope`],
//! and fans an identical set of stable subscription fields out to every
//! subscribed consumer surface. The point is that shell, search, graph,
//! AI, review, and support never grow private caches, private epochs, or
//! private stale-state language — their visible state is one projection
//! of one envelope.
//!
//! The contract carries exactly what the milestone reactive-state lane
//! requires of a subscription:
//!
//! - **query family** and **authority ownership**
//!   ([`AuthorityClass`]) — workspace/VFS, buffers, derived knowledge,
//!   execution, policy/entitlement, and provider overlays;
//! - **scope** ([`ScopeClass`] + a concrete scope id) — workspace,
//!   window, review workspace, remote session, tenant, or companion
//!   surface; an ambient unscoped subscription is rejected by the bus;
//! - **snapshot epoch** and **delta sequence** — the snapshot-plus-delta
//!   stream coordinates;
//! - **freshness / completeness** metadata and **backpressure mode**;
//! - the narrowed **truth claim** computed by the one canonical
//!   [`narrow_truth_claim`] engine, so a degraded frame downgrades
//!   identically on every surface.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/cross_surface_subscription.schema.json`](../../../../schemas/state/cross_surface_subscription.schema.json)
//! - [`/docs/state/cross_surface_subscription.md`](../../../../docs/state/cross_surface_subscription.md)
//! - [`/artifacts/state/cross_surface_subscription.json`](../../../../artifacts/state/cross_surface_subscription.json)
//! - [`/artifacts/state/cross_surface_subscription_proof.md`](../../../../artifacts/state/cross_surface_subscription_proof.md)
//! - [`/fixtures/state/cross_surface_subscription/`](../../../../fixtures/state/cross_surface_subscription/)

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::envelope::{
    self as env, CausedBy, Invalidation, JsonValue, ProducerRef, SubscriptionEnvelope,
    SUBSCRIPTION_SCHEMA_VERSION,
};
use crate::m5_reactive_governance::{
    narrow_truth_claim, AuthorityClass, BackpressureMode, Completeness, DerivationClass, Freshness,
    NarrowingTrigger, ObservedReactiveState, ScopeClass, TerminalReason, TruthClaim, ViewClass,
};

/// Schema version stamped onto packets and fixtures.
pub const CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const CROSS_SURFACE_SUBSCRIPTION_PACKET_RECORD_KIND: &str =
    "cross_surface_subscription_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const CROSS_SURFACE_SUBSCRIPTION_FIXTURE_RECORD_KIND: &str =
    "cross_surface_subscription_fixture_record";

/// Stable record-kind tag carried by an inspector report.
pub const CROSS_SURFACE_SUBSCRIPTION_INSPECTOR_RECORD_KIND: &str =
    "cross_surface_subscription_inspector_report";

/// Repo-relative schema ref.
pub const CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF: &str =
    "schemas/state/cross_surface_subscription.schema.json";

/// Repo-relative reviewer doc ref.
pub const CROSS_SURFACE_SUBSCRIPTION_DOC_REF: &str = "docs/state/cross_surface_subscription.md";

/// Repo-relative machine-readable artifact packet.
pub const CROSS_SURFACE_SUBSCRIPTION_PACKET_REF: &str =
    "artifacts/state/cross_surface_subscription.json";

/// Repo-relative reviewer proof report.
pub const CROSS_SURFACE_SUBSCRIPTION_PROOF_REF: &str =
    "artifacts/state/cross_surface_subscription_proof.md";

/// Repo-relative fixture directory.
pub const CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR: &str =
    "fixtures/state/cross_surface_subscription";

/// Repo-relative fixture manifest.
pub const CROSS_SURFACE_SUBSCRIPTION_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/cross_surface_subscription/manifest.yaml";

// ---------------------------------------------------------------------------
// Cross-surface vocabulary.
// ---------------------------------------------------------------------------

/// One consumer surface that subscribes through the common contract.
///
/// These are the product surfaces the milestone reactive-state lane
/// requires to share one subscription envelope rather than caching their
/// own truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// The shell (workspace tree, status, activity center).
    Shell,
    /// The search results surface.
    Search,
    /// The code/knowledge graph surface.
    Graph,
    /// The AI context panel.
    Ai,
    /// The review workspace.
    Review,
    /// The support / export surface.
    Support,
}

impl ConsumerSurface {
    /// Stable token mirrored by the schema.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Search => "search",
            Self::Graph => "graph",
            Self::Ai => "ai",
            Self::Review => "review",
            Self::Support => "support",
        }
    }

    /// All six consumer surfaces, in stable order.
    pub const fn all() -> [ConsumerSurface; 6] {
        [
            Self::Shell,
            Self::Search,
            Self::Graph,
            Self::Ai,
            Self::Review,
            Self::Support,
        ]
    }
}

/// Lifecycle frame class. Mirrors [`crate::envelope::FrameClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameClass {
    Snapshot,
    Delta,
    ResyncRequired,
    Terminal,
}

impl FrameClass {
    /// Stable token mirrored by the schema and [`crate::envelope`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Snapshot => "snapshot",
            Self::Delta => "delta",
            Self::ResyncRequired => "resync_required",
            Self::Terminal => "terminal",
        }
    }
}

// ---------------------------------------------------------------------------
// Declarative publish/subscribe wiring.
// ---------------------------------------------------------------------------

/// One declarative subscription binding: which authority publishes which
/// query family at which scope, the view class it materializes, and the
/// set of consumer surfaces that subscribe to it.
///
/// A binding is the unit of publish/subscribe wiring. It does not carry a
/// live frame; the bus pairs a binding with a concrete scope id and an
/// authority frame to produce per-surface [`ConsumerView`]s.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Canonical query family the binding carries.
    pub query_family: String,
    /// Authority class that owns and publishes the canonical truth.
    pub authority_class: AuthorityClass,
    /// Whether the binding is authoritative or a derived projection.
    pub derivation_class: DerivationClass,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Materialized-view class for the binding.
    pub view_class: ViewClass,
    /// Default backpressure mode for the subscription.
    pub default_backpressure_mode: BackpressureMode,
    /// Consumer surfaces subscribed to this binding, sorted and unique.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// Strongest claim a healthy frame on this binding may present.
    pub healthy_claim: TruthClaim,
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
    /// Proof report ref.
    pub proof_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the cross-surface subscription contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceSubscriptionPacket {
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
    /// Declarative subscription bindings.
    pub bindings: Vec<SubscriptionBinding>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

// ---------------------------------------------------------------------------
// Published frame and projected consumer view.
// ---------------------------------------------------------------------------

/// One frame an owning authority publishes onto a binding at a scope.
///
/// This is the runtime input the bus turns into a canonical
/// [`SubscriptionEnvelope`]. It carries the snapshot-plus-delta
/// coordinates, freshness/completeness metadata, backpressure mode, and
/// producer attribution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublishedFrame {
    /// Concrete scope id. Must be non-empty: an ambient unscoped
    /// subscription is rejected by the bus.
    pub scope_id: String,
    /// Lifecycle frame class.
    pub frame_class: FrameClass,
    /// Snapshot epoch the frame belongs to.
    pub snapshot_epoch: u64,
    /// Delta sequence within the snapshot epoch.
    pub delta_seq: u64,
    /// Observed freshness.
    pub freshness: Freshness,
    /// Observed completeness.
    pub completeness: Completeness,
    /// Observed backpressure mode.
    pub backpressure_mode: BackpressureMode,
    /// Terminal reason, when the subscription has ended.
    pub terminal_reason: Option<TerminalReason>,
    /// Whether policy / entitlement limits the visible projection.
    pub policy_limited: bool,
    /// Producer id of the publishing authority.
    pub producer_id: String,
    /// Producer instance (host/pid/boot) of the publishing authority.
    pub producer_instance: String,
    /// Synthetic monotonic observation tag (no wall clock).
    pub observed_at: String,
}

impl PublishedFrame {
    /// The minimal observed state the narrowing engine consumes.
    pub fn observed(&self) -> ObservedReactiveState {
        ObservedReactiveState {
            freshness: self.freshness,
            completeness: self.completeness,
            backpressure_mode: self.backpressure_mode,
            terminal_reason: self.terminal_reason,
            policy_limited: self.policy_limited,
        }
    }
}

/// The stable subscription fields every subscribed surface observes for a
/// published frame. They are identical across surfaces: that identity is
/// the contract — no surface may derive a richer or staler view than the
/// shared envelope permits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSubscriptionFields {
    /// Binding the frame was published on.
    pub binding_id: String,
    /// Subscription id allocated for the `(binding, scope)` pair.
    pub subscription_id: u64,
    /// Canonical query family.
    pub query_family: String,
    /// Authority that published the frame.
    pub authority_class: AuthorityClass,
    /// Whether the view is authoritative or derived.
    pub derivation_class: DerivationClass,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Concrete scope id.
    pub scope_id: String,
    /// Materialized-view class.
    pub view_class: ViewClass,
    /// Snapshot epoch the frame belongs to.
    pub snapshot_epoch: u64,
    /// Delta sequence within the snapshot epoch.
    pub delta_seq: u64,
    /// Lifecycle frame class.
    pub frame_class: FrameClass,
    /// Observed freshness.
    pub freshness: Freshness,
    /// Observed completeness.
    pub completeness: Completeness,
    /// Observed backpressure mode.
    pub backpressure_mode: BackpressureMode,
    /// Narrowed truth claim the surface may present for this frame.
    pub truth_claim: TruthClaim,
    /// Every trigger that narrowed the claim, in stable order.
    pub narrowing_triggers: Vec<NarrowingTrigger>,
    /// Producer id of the publishing authority.
    pub producer_id: String,
    /// Synthetic monotonic observation tag.
    pub observed_at: String,
}

/// One consumer surface's view of a published frame. The
/// [`StableSubscriptionFields`] are shared verbatim with every other
/// subscribed surface; only [`ConsumerView::consumer_surface`] differs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerView {
    /// The consumer surface this view was projected for.
    pub consumer_surface: ConsumerSurface,
    /// The stable subscription fields shared by all subscribed surfaces.
    pub subscription: StableSubscriptionFields,
}

/// The result of publishing one frame: the canonical envelope JSON every
/// surface consumed, the stable fields, and the per-surface views.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublishOutcome {
    /// Canonical [`SubscriptionEnvelope`] JSON the surfaces consumed. All
    /// consumer views are a projection of this one frame.
    pub envelope_json: String,
    /// The stable subscription fields shared by every subscribed surface.
    pub stable: StableSubscriptionFields,
    /// One view per subscribed consumer surface, in stable order.
    pub views: Vec<ConsumerView>,
}

// ---------------------------------------------------------------------------
// Inspector.
// ---------------------------------------------------------------------------

/// One inspector row: the latest published frame for a `(binding, scope)`
/// pair, naming which authority published it and which scope/epoch it
/// belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionInspectorRow {
    /// The stable subscription fields of the latest frame.
    pub subscription: StableSubscriptionFields,
    /// Consumer surfaces subscribed to the binding, in stable order.
    pub consumer_surfaces: Vec<ConsumerSurface>,
}

/// A round-trippable inspector report: which authority published the
/// current view of every active `(binding, scope)` pair.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SubscriptionInspectorReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// One row per active `(binding, scope)` pair, in stable order.
    pub rows: Vec<SubscriptionInspectorRow>,
}

// ---------------------------------------------------------------------------
// Runtime bus.
// ---------------------------------------------------------------------------

/// Error returned when a publish violates the subscription contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptionError {
    /// The binding id is not declared in the bus.
    UnknownBinding(String),
    /// An ambient unscoped subscription was attempted (empty scope id).
    AmbientScopeForbidden(String),
    /// A derived binding attempted to publish exact current truth.
    ExactTruthOverclaim(String),
}

impl fmt::Display for SubscriptionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownBinding(id) => write!(f, "unknown subscription binding: {id}"),
            Self::AmbientScopeForbidden(id) => write!(
                f,
                "ambient unscoped subscription forbidden on binding {id}: a concrete scope id is required"
            ),
            Self::ExactTruthOverclaim(id) => write!(
                f,
                "derived binding {id} may not present exact current truth"
            ),
        }
    }
}

impl std::error::Error for SubscriptionError {}

/// Per-`(binding, scope)` runtime state.
#[derive(Debug, Clone)]
struct BusEntry {
    binding_id: String,
    scope_id: String,
    subscription_id: u64,
    latest: StableSubscriptionFields,
    consumer_surfaces: Vec<ConsumerSurface>,
}

/// The cross-surface subscription bus.
///
/// The bus owns the declarative bindings and, for each `(binding, scope)`
/// pair seen, a stable subscription id and the latest published frame. It
/// is the single fan-out point: one [`publish`](CrossSurfaceSubscriptionBus::publish)
/// produces one canonical envelope and one identical set of stable fields
/// for every subscribed surface.
#[derive(Debug, Clone)]
pub struct CrossSurfaceSubscriptionBus {
    bindings: Vec<SubscriptionBinding>,
    entries: Vec<BusEntry>,
    next_subscription_id: u64,
}

impl CrossSurfaceSubscriptionBus {
    /// Builds a bus from a packet's bindings.
    pub fn from_packet(packet: &CrossSurfaceSubscriptionPacket) -> Self {
        Self {
            bindings: packet.bindings.clone(),
            entries: Vec::new(),
            next_subscription_id: 1,
        }
    }

    /// Returns the binding with the given id, if declared.
    pub fn binding(&self, binding_id: &str) -> Option<&SubscriptionBinding> {
        self.bindings.iter().find(|b| b.binding_id == binding_id)
    }

    /// Publishes one frame from the binding's owning authority and fans an
    /// identical set of stable subscription fields out to every subscribed
    /// consumer surface.
    ///
    /// # Errors
    ///
    /// Returns [`SubscriptionError::UnknownBinding`] if the binding is not
    /// declared, [`SubscriptionError::AmbientScopeForbidden`] if the scope
    /// id is empty, and [`SubscriptionError::ExactTruthOverclaim`] if a
    /// derived binding's healthy claim is exact current truth.
    pub fn publish(
        &mut self,
        binding_id: &str,
        frame: &PublishedFrame,
    ) -> Result<PublishOutcome, SubscriptionError> {
        let binding = self
            .bindings
            .iter()
            .find(|b| b.binding_id == binding_id)
            .ok_or_else(|| SubscriptionError::UnknownBinding(binding_id.to_owned()))?
            .clone();

        if frame.scope_id.trim().is_empty() {
            return Err(SubscriptionError::AmbientScopeForbidden(
                binding_id.to_owned(),
            ));
        }
        if binding.derivation_class == DerivationClass::Derived
            && binding.healthy_claim == TruthClaim::ExactCurrentTruth
        {
            return Err(SubscriptionError::ExactTruthOverclaim(
                binding_id.to_owned(),
            ));
        }

        let subscription_id = self.subscription_id_for(&binding.binding_id, &frame.scope_id);

        let narrowed = narrow_truth_claim(binding.derivation_class, &frame.observed());
        let envelope = build_envelope(&binding, frame, subscription_id);
        let envelope_json = envelope.to_json();

        let stable = StableSubscriptionFields {
            binding_id: binding.binding_id.clone(),
            subscription_id,
            query_family: binding.query_family.clone(),
            authority_class: binding.authority_class,
            derivation_class: binding.derivation_class,
            scope_class: binding.scope_class,
            scope_id: frame.scope_id.clone(),
            view_class: binding.view_class,
            snapshot_epoch: frame.snapshot_epoch,
            delta_seq: frame.delta_seq,
            frame_class: frame.frame_class,
            freshness: frame.freshness,
            completeness: frame.completeness,
            backpressure_mode: frame.backpressure_mode,
            truth_claim: narrowed.claim,
            narrowing_triggers: narrowed.triggers,
            producer_id: frame.producer_id.clone(),
            observed_at: frame.observed_at.clone(),
        };

        let mut consumer_surfaces = binding.consumer_surfaces.clone();
        consumer_surfaces.sort();
        consumer_surfaces.dedup();

        let views: Vec<ConsumerView> = consumer_surfaces
            .iter()
            .map(|surface| ConsumerView {
                consumer_surface: *surface,
                subscription: stable.clone(),
            })
            .collect();

        self.record_latest(&binding, &frame.scope_id, &stable, &consumer_surfaces);

        Ok(PublishOutcome {
            envelope_json,
            stable,
            views,
        })
    }

    /// Builds an inspector report of the latest frame for every active
    /// `(binding, scope)` pair, in stable order.
    pub fn inspector_report(&self) -> SubscriptionInspectorReport {
        let mut rows: Vec<SubscriptionInspectorRow> = self
            .entries
            .iter()
            .map(|entry| SubscriptionInspectorRow {
                subscription: entry.latest.clone(),
                consumer_surfaces: entry.consumer_surfaces.clone(),
            })
            .collect();
        rows.sort_by(|a, b| {
            (&a.subscription.binding_id, &a.subscription.scope_id)
                .cmp(&(&b.subscription.binding_id, &b.subscription.scope_id))
        });
        SubscriptionInspectorReport {
            record_kind: CROSS_SURFACE_SUBSCRIPTION_INSPECTOR_RECORD_KIND.to_owned(),
            schema_version: CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION,
            rows,
        }
    }

    fn subscription_id_for(&mut self, binding_id: &str, scope_id: &str) -> u64 {
        if let Some(entry) = self
            .entries
            .iter()
            .find(|e| e.binding_id == binding_id && e.scope_id == scope_id)
        {
            return entry.subscription_id;
        }
        let id = self.next_subscription_id;
        self.next_subscription_id += 1;
        id
    }

    fn record_latest(
        &mut self,
        binding: &SubscriptionBinding,
        scope_id: &str,
        stable: &StableSubscriptionFields,
        consumer_surfaces: &[ConsumerSurface],
    ) {
        if let Some(entry) = self
            .entries
            .iter_mut()
            .find(|e| e.binding_id == binding.binding_id && e.scope_id == scope_id)
        {
            entry.latest = stable.clone();
            return;
        }
        self.entries.push(BusEntry {
            binding_id: binding.binding_id.clone(),
            scope_id: scope_id.to_owned(),
            subscription_id: stable.subscription_id,
            latest: stable.clone(),
            consumer_surfaces: consumer_surfaces.to_vec(),
        });
    }
}

// ---------------------------------------------------------------------------
// Canonical envelope construction.
// ---------------------------------------------------------------------------

/// Builds the canonical [`SubscriptionEnvelope`] every subscribed surface
/// consumes for one published frame. The bus projects this single frame
/// into per-surface views, so no surface forks the shared truth.
fn build_envelope(
    binding: &SubscriptionBinding,
    frame: &PublishedFrame,
    subscription_id: u64,
) -> SubscriptionEnvelope {
    let invalidation = invalidation_for(frame);
    SubscriptionEnvelope {
        subscription_schema_version: SUBSCRIPTION_SCHEMA_VERSION,
        subscription_id,
        query_family: binding.query_family.clone(),
        scope_ref: env::ScopeRef {
            class: env_scope_class(binding.scope_class),
            id: frame.scope_id.clone(),
        },
        authority_class: env_authority_class(binding.authority_class),
        derivation_class: env_derivation_class(binding.derivation_class),
        snapshot_epoch: frame.snapshot_epoch,
        delta_seq: frame.delta_seq,
        frame_class: env_frame_class(frame.frame_class),
        freshness: env_freshness(frame.freshness),
        completeness: env_completeness(frame.completeness),
        backpressure_mode: env_backpressure_mode(frame.backpressure_mode),
        view_class: env_view_class(binding.view_class),
        producer_refs: vec![ProducerRef {
            producer_id: frame.producer_id.clone(),
            producer_instance: frame.producer_instance.clone(),
            producer_version: None,
            input_digests: vec![],
            derivation_epoch: None,
            source: None,
        }],
        invalidation,
        terminal_reason: frame.terminal_reason.map(env_terminal_reason),
        payload: Some(JsonValue::obj(vec![(
            "truth_claim",
            JsonValue::str(
                narrow_truth_claim(binding.derivation_class, &frame.observed())
                    .claim
                    .as_str(),
            ),
        )])),
    }
}

/// Attaches an invalidation body to non-authoritative frames, naming the
/// stale reason that matches the observed freshness.
fn invalidation_for(frame: &PublishedFrame) -> Option<Invalidation> {
    let stale_reason = match frame.freshness {
        Freshness::Authoritative | Freshness::Warming => None,
        Freshness::Cached => Some(env::StaleReason::CacheServed),
        Freshness::Stale => Some(env::StaleReason::UpstreamInputStale),
        Freshness::Replayed => Some(env::StaleReason::ReplayedFromBundle),
        Freshness::Imported => Some(env::StaleReason::ImportedFromExternal),
    };
    stale_reason.map(|reason| Invalidation {
        stale_reason: reason,
        caused_by: Some(CausedBy {
            note: Some(format!("freshness={}", frame.freshness.as_str())),
            ..CausedBy::default()
        }),
    })
}

fn env_authority_class(value: AuthorityClass) -> env::AuthorityClass {
    match value {
        AuthorityClass::WorkspaceVfs => env::AuthorityClass::WorkspaceVfs,
        AuthorityClass::BufferEditor => env::AuthorityClass::BufferEditor,
        AuthorityClass::DerivedKnowledge => env::AuthorityClass::DerivedKnowledge,
        AuthorityClass::Execution => env::AuthorityClass::Execution,
        AuthorityClass::PolicyEntitlement => env::AuthorityClass::PolicyEntitlement,
        AuthorityClass::ProviderOverlay => env::AuthorityClass::ProviderOverlay,
    }
}

fn env_derivation_class(value: DerivationClass) -> env::DerivationClass {
    match value {
        DerivationClass::Authoritative => env::DerivationClass::Authoritative,
        DerivationClass::Derived => env::DerivationClass::Derived,
    }
}

fn env_freshness(value: Freshness) -> env::Freshness {
    match value {
        Freshness::Authoritative => env::Freshness::Authoritative,
        Freshness::Warming => env::Freshness::Warming,
        Freshness::Cached => env::Freshness::Cached,
        Freshness::Stale => env::Freshness::Stale,
        Freshness::Replayed => env::Freshness::Replayed,
        Freshness::Imported => env::Freshness::Imported,
    }
}

fn env_completeness(value: Completeness) -> env::Completeness {
    match value {
        Completeness::Full => env::Completeness::Full,
        Completeness::Partial => env::Completeness::Partial,
        Completeness::Unloaded => env::Completeness::Unloaded,
        Completeness::Unavailable => env::Completeness::Unavailable,
    }
}

fn env_backpressure_mode(value: BackpressureMode) -> env::BackpressureMode {
    match value {
        BackpressureMode::Realtime => env::BackpressureMode::Realtime,
        BackpressureMode::Coalesced => env::BackpressureMode::Coalesced,
        BackpressureMode::SnapshotRequired => env::BackpressureMode::SnapshotRequired,
    }
}

fn env_view_class(value: ViewClass) -> env::ViewClass {
    match value {
        ViewClass::EphemeralProjection => env::ViewClass::EphemeralProjection,
        ViewClass::DurableLocalMaterialization => env::ViewClass::DurableLocalMaterialization,
        ViewClass::ExportableSnapshot => env::ViewClass::ExportableSnapshot,
        ViewClass::ManagedReplicatedView => env::ViewClass::ManagedReplicatedView,
    }
}

fn env_scope_class(value: ScopeClass) -> env::ScopeClass {
    match value {
        ScopeClass::Workspace => env::ScopeClass::Workspace,
        ScopeClass::Window => env::ScopeClass::Window,
        ScopeClass::ReviewWorkspace => env::ScopeClass::ReviewWorkspace,
        ScopeClass::RemoteSession => env::ScopeClass::RemoteSession,
        ScopeClass::Tenant => env::ScopeClass::Tenant,
        ScopeClass::CompanionSurface => env::ScopeClass::CompanionSurface,
    }
}

fn env_frame_class(value: FrameClass) -> env::FrameClass {
    match value {
        FrameClass::Snapshot => env::FrameClass::Snapshot,
        FrameClass::Delta => env::FrameClass::Delta,
        FrameClass::ResyncRequired => env::FrameClass::ResyncRequired,
        FrameClass::Terminal => env::FrameClass::Terminal,
    }
}

fn env_terminal_reason(value: TerminalReason) -> env::TerminalReason {
    match value {
        TerminalReason::ConsumerCancelled => env::TerminalReason::ConsumerCancelled,
        TerminalReason::ProducerShutdown => env::TerminalReason::ProducerShutdown,
        TerminalReason::ScopeRemoved => env::TerminalReason::ScopeRemoved,
        TerminalReason::PolicyTerminated => env::TerminalReason::PolicyTerminated,
        TerminalReason::Unavailable => env::TerminalReason::Unavailable,
    }
}

// ---------------------------------------------------------------------------
// Fixtures.
// ---------------------------------------------------------------------------

/// One fixture binding a published frame to the expected cross-surface
/// projection: the narrowed truth claim, the triggers, and the consumer
/// surfaces that all observe the identical stable fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossSurfaceSubscriptionFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Binding the frame is published on.
    pub binding_id: String,
    /// The published frame.
    pub frame: PublishedFrame,
    /// Expected narrowed truth claim every surface presents.
    pub expected_truth_claim: TruthClaim,
    /// Expected narrowing triggers.
    pub expected_triggers: Vec<NarrowingTrigger>,
    /// Expected consumer surfaces that observe identical stable fields.
    pub expected_consumer_surfaces: Vec<ConsumerSurface>,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

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
        writeln!(f, "cross-surface subscription validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// Validates the cross-surface subscription packet.
///
/// # Errors
///
/// Returns a [`ValidationReport`] naming every contract violation.
pub fn validate_cross_surface_subscription_packet(
    packet: &CrossSurfaceSubscriptionPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != CROSS_SURFACE_SUBSCRIPTION_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            format!(
                "record_kind must be {CROSS_SURFACE_SUBSCRIPTION_PACKET_RECORD_KIND}, got {}",
                packet.record_kind
            ),
        );
    }
    if packet.schema_version != CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION {
        report.push(
            "packet.schema_version",
            format!(
                "schema_version must be {CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION}, got {}",
                packet.schema_version
            ),
        );
    }
    let refs = &packet.source_contract_refs;
    if refs.doc_ref != CROSS_SURFACE_SUBSCRIPTION_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted");
    }
    if refs.schema_ref != CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF {
        report.push("packet.schema_ref", "schema_ref drifted");
    }
    if refs.packet_ref != CROSS_SURFACE_SUBSCRIPTION_PACKET_REF {
        report.push("packet.packet_ref", "packet_ref drifted");
    }
    if refs.proof_ref != CROSS_SURFACE_SUBSCRIPTION_PROOF_REF {
        report.push("packet.proof_ref", "proof_ref drifted");
    }
    if refs.fixture_manifest_ref != CROSS_SURFACE_SUBSCRIPTION_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted",
        );
    }

    if packet.bindings.is_empty() {
        report.push(
            "packet.bindings",
            "packet must declare at least one binding",
        );
    }

    let mut binding_ids = BTreeSet::new();
    let mut authority_classes = BTreeSet::new();
    let mut view_classes = BTreeSet::new();
    let mut covered_surfaces = BTreeSet::new();
    let mut has_all_six_binding = false;

    for binding in &packet.bindings {
        if !binding_ids.insert(binding.binding_id.clone()) {
            report.push(
                "binding.duplicate",
                format!("duplicate binding id {}", binding.binding_id),
            );
        }
        if binding.binding_id.trim().is_empty() {
            report.push("binding.binding_id", "binding id must be non-empty");
        }
        if binding.query_family.trim().is_empty() {
            report.push(
                "binding.query_family",
                format!("binding {} must carry a query family", binding.binding_id),
            );
        }
        authority_classes.insert(binding.authority_class);
        view_classes.insert(binding.view_class);

        if binding.consumer_surfaces.is_empty() {
            report.push(
                "binding.consumer_surfaces",
                format!(
                    "binding {} must declare at least one consumer surface",
                    binding.binding_id
                ),
            );
        }
        if !is_strictly_sorted_unique(&binding.consumer_surfaces) {
            report.push(
                "binding.surface_order",
                format!(
                    "binding {} consumer surfaces must be sorted and unique",
                    binding.binding_id
                ),
            );
        }
        for surface in &binding.consumer_surfaces {
            covered_surfaces.insert(*surface);
        }
        if binding.consumer_surfaces.len() == ConsumerSurface::all().len() {
            has_all_six_binding = true;
        }

        // Guardrail: a derived binding may never advertise exact current truth.
        if binding.derivation_class == DerivationClass::Derived
            && binding.healthy_claim == TruthClaim::ExactCurrentTruth
        {
            report.push(
                "binding.exact_truth_overclaim",
                format!(
                    "derived binding {} may not present exact_current_truth",
                    binding.binding_id
                ),
            );
        }

        // The healthy claim must equal the engine's output for a healthy state.
        let healthy =
            narrow_truth_claim(binding.derivation_class, &ObservedReactiveState::healthy());
        if binding.healthy_claim != healthy.claim {
            report.push(
                "binding.healthy_claim",
                format!(
                    "binding {} healthy_claim {} must equal engine output {}",
                    binding.binding_id,
                    binding.healthy_claim.as_str(),
                    healthy.claim.as_str()
                ),
            );
        }
    }

    for required in [
        AuthorityClass::WorkspaceVfs,
        AuthorityClass::BufferEditor,
        AuthorityClass::DerivedKnowledge,
        AuthorityClass::Execution,
        AuthorityClass::PolicyEntitlement,
        AuthorityClass::ProviderOverlay,
    ] {
        if !authority_classes.contains(&required) {
            report.push(
                "packet.authority_missing",
                format!("contract must cover authority class {}", required.as_str()),
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
                format!("contract must cover view class {}", required.as_str()),
            );
        }
    }
    for required in ConsumerSurface::all() {
        if !covered_surfaces.contains(&required) {
            report.push(
                "packet.surface_missing",
                format!("contract must wire consumer surface {}", required.as_str()),
            );
        }
    }
    if !has_all_six_binding {
        report.push(
            "packet.cross_surface_binding",
            "contract must declare at least one binding subscribed by all six consumer surfaces",
        );
    }

    if packet.invariants.iter().all(|inv| inv.trim().is_empty()) {
        report.push("packet.invariants", "invariants must be non-empty");
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one fixture against the packet by replaying it through the
/// bus and confirming the cross-surface projection matches.
///
/// # Errors
///
/// Returns a [`ValidationReport`] naming every contract violation.
pub fn validate_cross_surface_subscription_fixture(
    packet: &CrossSurfaceSubscriptionPacket,
    fixture: &CrossSurfaceSubscriptionFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };
    if fixture.record_kind != CROSS_SURFACE_SUBSCRIPTION_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            format!(
                "fixture {} record_kind must be {}",
                fixture.fixture_id, CROSS_SURFACE_SUBSCRIPTION_FIXTURE_RECORD_KIND
            ),
        );
    }
    if fixture.schema_version != CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION {
        report.push(
            "fixture.schema_version",
            format!(
                "fixture {} schema_version must be {}",
                fixture.fixture_id, CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION
            ),
        );
    }

    let Some(binding) = packet
        .bindings
        .iter()
        .find(|b| b.binding_id == fixture.binding_id)
    else {
        report.push(
            "fixture.binding_missing",
            format!(
                "fixture {} points to binding {} missing from the contract",
                fixture.fixture_id, fixture.binding_id
            ),
        );
        return Err(report);
    };

    if fixture.frame.scope_id.trim().is_empty() {
        report.push(
            "fixture.ambient_scope",
            format!(
                "fixture {} must carry a concrete scope id; ambient subscriptions are forbidden",
                fixture.fixture_id
            ),
        );
    }

    let mut expected_surfaces = binding.consumer_surfaces.clone();
    expected_surfaces.sort();
    expected_surfaces.dedup();
    if fixture.expected_consumer_surfaces != expected_surfaces {
        report.push(
            "fixture.consumer_surfaces",
            format!(
                "fixture {} expected consumer surfaces must equal binding {} subscribers",
                fixture.fixture_id, binding.binding_id
            ),
        );
    }

    let narrowed = narrow_truth_claim(binding.derivation_class, &fixture.frame.observed());
    if narrowed.claim != fixture.expected_truth_claim {
        report.push(
            "fixture.expected_claim",
            format!(
                "fixture {} expected claim {} but engine produced {}",
                fixture.fixture_id,
                fixture.expected_truth_claim.as_str(),
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

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Seeded contract.
// ---------------------------------------------------------------------------

/// Returns the checked-in cross-surface subscription contract this lane
/// freezes.
pub fn seeded_cross_surface_subscription_packet() -> CrossSurfaceSubscriptionPacket {
    use ConsumerSurface::{Ai, Graph, Review, Search, Shell, Support};

    let bindings = vec![
        binding(
            "binding:workspace_tree",
            "vfs.workspace_tree",
            AuthorityClass::WorkspaceVfs,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            BackpressureMode::Coalesced,
            &[Shell, Search, Graph, Ai, Review, Support],
            "The workspace tree is the one anchor every surface reads to scope its derived view; all six subscribe to the same VFS authority frame instead of caching their own tree.",
        ),
        binding(
            "binding:execution_activity",
            "execution.activity_stream",
            AuthorityClass::Execution,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            BackpressureMode::Coalesced,
            &[Shell, Review, Support],
            "Shell, review, and support read one execution activity stream; under saturation it coalesces rather than presenting a saturated stream as current.",
        ),
        binding(
            "binding:buffer_outline",
            "buffer.outline",
            AuthorityClass::BufferEditor,
            ScopeClass::Window,
            ViewClass::EphemeralProjection,
            BackpressureMode::Coalesced,
            &[Shell, Ai],
            "The buffer outline is a window-scoped ephemeral projection shared by the shell and the AI context panel.",
        ),
        binding(
            "binding:search_index",
            "search.results",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::DurableLocalMaterialization,
            BackpressureMode::Coalesced,
            &[Shell, Search, Ai],
            "The search index is a derived-knowledge view; shell, search, and AI read one partial/stale-labeled projection rather than three private indices.",
        ),
        binding(
            "binding:graph_neighborhood",
            "graph.neighborhood",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::EphemeralProjection,
            BackpressureMode::Coalesced,
            &[Shell, Graph],
            "Graph neighborhoods are ephemeral derived projections shared by the shell and the graph surface.",
        ),
        binding(
            "binding:policy_trust",
            "policy.trust_state",
            AuthorityClass::PolicyEntitlement,
            ScopeClass::Workspace,
            ViewClass::EphemeralProjection,
            BackpressureMode::Realtime,
            &[Shell, Ai, Review, Support],
            "The policy/trust state is one entitlement authority every gating surface reads; a policy-limited frame narrows identically across shell, AI, review, and support.",
        ),
        binding(
            "binding:review_overlay",
            "review.workspace_overlay",
            AuthorityClass::ProviderOverlay,
            ScopeClass::ReviewWorkspace,
            ViewClass::ManagedReplicatedView,
            BackpressureMode::Coalesced,
            &[Shell, Review, Support],
            "The review overlay is a managed replicated provider view; when the provider is unavailable it says so across shell, review, and support instead of replacing local truth.",
        ),
        binding(
            "binding:support_export",
            "support.reactive_state_export",
            AuthorityClass::DerivedKnowledge,
            ScopeClass::Workspace,
            ViewClass::ExportableSnapshot,
            BackpressureMode::Realtime,
            &[Support],
            "The support export is an exportable snapshot of captured reactive state; replayed and imported frames stay labeled for release and procurement readers.",
        ),
    ];

    CrossSurfaceSubscriptionPacket {
        record_kind: CROSS_SURFACE_SUBSCRIPTION_PACKET_RECORD_KIND.to_owned(),
        schema_version: CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION,
        packet_id: "state.cross_surface_subscription.v1".to_owned(),
        title: "Cross-surface subscription contract for shell, search, graph, AI, review, and support"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: CROSS_SURFACE_SUBSCRIPTION_DOC_REF.to_owned(),
            schema_ref: CROSS_SURFACE_SUBSCRIPTION_SCHEMA_REF.to_owned(),
            packet_ref: CROSS_SURFACE_SUBSCRIPTION_PACKET_REF.to_owned(),
            proof_ref: CROSS_SURFACE_SUBSCRIPTION_PROOF_REF.to_owned(),
            fixture_manifest_ref: CROSS_SURFACE_SUBSCRIPTION_FIXTURE_MANIFEST_REF.to_owned(),
        },
        bindings,
        invariants: vec![
            "Every consumer surface (shell, search, graph, AI, review, support) reads one typed subscription envelope — query family, authority, scope, snapshot epoch, delta sequence, freshness, completeness, backpressure mode — instead of a private cache.".to_owned(),
            "All surfaces subscribed to a binding observe identical stable subscription fields for a published frame; no surface derives a richer or staler view than the shared envelope permits.".to_owned(),
            "Every subscription is scoped by a concrete scope id (workspace, window, review workspace, remote session, tenant, or companion surface); an ambient unscoped subscription is rejected by the bus.".to_owned(),
            "A degraded frame narrows its truth claim through one canonical engine, so stale, warming, partial, cached, replayed, imported, coalesced, policy-limited, and provider-unavailable frames downgrade identically on every surface.".to_owned(),
            "The inspector names which authority published the current view of each binding and which scope and epoch it belongs to, and review/export/support round-trip the same stable subscription fields.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixture rows this lane freezes.
pub fn seeded_cross_surface_subscription_fixtures() -> Vec<CrossSurfaceSubscriptionFixture> {
    let packet = seeded_cross_surface_subscription_packet();
    vec![
        fixture(
            &packet,
            "fixture:subscription:workspace_tree_warming",
            "binding:workspace_tree",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Snapshot,
                snapshot_epoch: 1,
                delta_seq: 0,
                freshness: Freshness::Warming,
                completeness: Completeness::Unloaded,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.vfs.workspace_tree".to_owned(),
                producer_instance: "synthetic-host/pid-4812/boot-2026041900".to_owned(),
                observed_at: "mono:1".to_owned(),
            },
            "A warming, unloaded workspace tree narrows to warming-no-truth-yet for all six surfaces at once.",
        ),
        fixture(
            &packet,
            "fixture:subscription:execution_activity_coalesced",
            "binding:execution_activity",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Delta,
                snapshot_epoch: 4,
                delta_seq: 31,
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.execution.activity".to_owned(),
                producer_instance: "synthetic-host/pid-4800/boot-2026041900".to_owned(),
                observed_at: "mono:2".to_owned(),
            },
            "A coalesced execution stream narrows to a coalesced-stream claim while it lags the producer delta rate.",
        ),
        fixture(
            &packet,
            "fixture:subscription:buffer_outline_stale",
            "binding:buffer_outline",
            PublishedFrame {
                scope_id: "win-7".to_owned(),
                frame_class: FrameClass::Delta,
                snapshot_epoch: 9,
                delta_seq: 2,
                freshness: Freshness::Stale,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.buffer.outline".to_owned(),
                producer_instance: "synthetic-host/pid-5001/boot-2026041900".to_owned(),
                observed_at: "mono:3".to_owned(),
            },
            "A stale buffer outline narrows to a stale snapshot for the shell and the AI panel identically.",
        ),
        fixture(
            &packet,
            "fixture:subscription:search_index_partial",
            "binding:search_index",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Snapshot,
                snapshot_epoch: 12,
                delta_seq: 0,
                freshness: Freshness::Authoritative,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.search.index".to_owned(),
                producer_instance: "synthetic-host/pid-5200/boot-2026041900".to_owned(),
                observed_at: "mono:4".to_owned(),
            },
            "A partially loaded search index narrows to a partial projection; coalesced backpressure does not override the narrower partial claim.",
        ),
        fixture(
            &packet,
            "fixture:subscription:graph_neighborhood_replayed",
            "binding:graph_neighborhood",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Snapshot,
                snapshot_epoch: 6,
                delta_seq: 0,
                freshness: Freshness::Replayed,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.graph.neighborhood".to_owned(),
                producer_instance: "synthetic-host/pid-5401/boot-2026041900".to_owned(),
                observed_at: "mono:5".to_owned(),
            },
            "A replayed graph neighborhood narrows to a replayed snapshot, the narrowest of replayed-versus-partial-versus-coalesced.",
        ),
        fixture(
            &packet,
            "fixture:subscription:policy_trust_policy_limited",
            "binding:policy_trust",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Snapshot,
                snapshot_epoch: 3,
                delta_seq: 0,
                freshness: Freshness::Authoritative,
                completeness: Completeness::Full,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: true,
                producer_id: "aureline.policy.trust".to_owned(),
                producer_instance: "synthetic-host/pid-5300/boot-2026041900".to_owned(),
                observed_at: "mono:6".to_owned(),
            },
            "A policy-limited trust frame narrows to a policy-limited projection for shell, AI, review, and support identically.",
        ),
        fixture(
            &packet,
            "fixture:subscription:review_overlay_unavailable",
            "binding:review_overlay",
            PublishedFrame {
                scope_id: "review-22".to_owned(),
                frame_class: FrameClass::Terminal,
                snapshot_epoch: 5,
                delta_seq: 0,
                freshness: Freshness::Authoritative,
                completeness: Completeness::Unavailable,
                backpressure_mode: BackpressureMode::Coalesced,
                terminal_reason: Some(TerminalReason::Unavailable),
                policy_limited: false,
                producer_id: "aureline.provider.review".to_owned(),
                producer_instance: "synthetic-host/pid-5500/boot-2026041900".to_owned(),
                observed_at: "mono:7".to_owned(),
            },
            "An unavailable review provider narrows to provider-unavailable across shell, review, and support instead of replacing local truth.",
        ),
        fixture(
            &packet,
            "fixture:subscription:support_export_imported",
            "binding:support_export",
            PublishedFrame {
                scope_id: "ws-core".to_owned(),
                frame_class: FrameClass::Snapshot,
                snapshot_epoch: 2,
                delta_seq: 0,
                freshness: Freshness::Imported,
                completeness: Completeness::Partial,
                backpressure_mode: BackpressureMode::Realtime,
                terminal_reason: None,
                policy_limited: false,
                producer_id: "aureline.support.export".to_owned(),
                producer_instance: "synthetic-host/pid-5600/boot-2026041900".to_owned(),
                observed_at: "mono:8".to_owned(),
            },
            "An imported support-export frame narrows to an imported snapshot for release and procurement readers.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Builders / helpers.
// ---------------------------------------------------------------------------

// Subscription fixtures intentionally declare each authority and delivery axis inline.
#[allow(clippy::too_many_arguments)]
fn binding(
    binding_id: &str,
    query_family: &str,
    authority_class: AuthorityClass,
    scope_class: ScopeClass,
    view_class: ViewClass,
    default_backpressure_mode: BackpressureMode,
    consumer_surfaces: &[ConsumerSurface],
    notes: &str,
) -> SubscriptionBinding {
    // Every cross-surface subscription is a derived projection of an authority.
    let derivation_class = DerivationClass::Derived;
    SubscriptionBinding {
        binding_id: binding_id.to_owned(),
        query_family: query_family.to_owned(),
        authority_class,
        derivation_class,
        scope_class,
        view_class,
        default_backpressure_mode,
        consumer_surfaces: sorted_unique(consumer_surfaces),
        healthy_claim: narrow_truth_claim(derivation_class, &ObservedReactiveState::healthy())
            .claim,
        notes: notes.to_owned(),
    }
}

fn fixture(
    packet: &CrossSurfaceSubscriptionPacket,
    fixture_id: &str,
    binding_id: &str,
    frame: PublishedFrame,
    notes: &str,
) -> CrossSurfaceSubscriptionFixture {
    let binding = packet
        .bindings
        .iter()
        .find(|b| b.binding_id == binding_id)
        .unwrap_or_else(|| panic!("fixture {fixture_id} references unknown binding {binding_id}"));
    let narrowed = narrow_truth_claim(binding.derivation_class, &frame.observed());
    CrossSurfaceSubscriptionFixture {
        record_kind: CROSS_SURFACE_SUBSCRIPTION_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: CROSS_SURFACE_SUBSCRIPTION_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        binding_id: binding_id.to_owned(),
        frame,
        expected_truth_claim: narrowed.claim,
        expected_triggers: narrowed.triggers,
        expected_consumer_surfaces: binding.consumer_surfaces.clone(),
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
