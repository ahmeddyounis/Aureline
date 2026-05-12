//! Target graph state card: graph-readiness truth on one bounded wedge.
//!
//! The card is the M1 bounded-prototype surface a user opens to ask "what
//! does the workspace graph claim about this target right now, and how much
//! authority does that claim carry?" without trusting the chrome to be
//! truthful on its own. It is a thin projection over two upstream contracts:
//!
//! - [`aureline_reactive_state::ReadinessProjection`] — the canonical
//!   readiness label vocabulary (`exact`, `imported`, `heuristic`, `stale`,
//!   `partial`, `unavailable`, `out_of_scope`) frozen in
//!   `docs/filesystem/semantic_readiness_projection.md`. The card reads this
//!   label verbatim; it never re-derives readiness from raw freshness
//!   booleans.
//! - [`aureline_graph_proto`] — the workspace-graph node / edge / source /
//!   provenance / confidence / query-family vocabulary frozen in
//!   `docs/graph/workspace_graph_seed.md`. The card quotes those tokens
//!   verbatim so chrome, support exports, and machine output never invent a
//!   private synonym.
//!
//! ## What the wedge owns
//!
//! - One canonical [`GraphStateCardRecord`] data shape, serialized verbatim
//!   for support exports and proof captures.
//! - A typed [`GraphBasisClass`] that classifies what kind of basis the
//!   current target-graph claim rests on: `live_workspace_authoritative`,
//!   `imported_bundle`, `heuristic_inference`, `cached_warming`,
//!   `stale_after_invalidation`, `partial_subscope`,
//!   `unavailable_no_basis`, or `out_of_scope_for_current_workspace`.
//!   The basis class is derived mechanically from the readiness projection
//!   and the graph-side provenance / freshness inputs — surfaces cannot
//!   advertise `live_workspace_authoritative` over a `Partial` projection.
//! - An explicit [`PrototypeLabel`] chip carried on every card so the chrome
//!   cannot quietly drop the wedge label and pretend the surface is a
//!   production graph platform.
//! - Stable claim-limit strings the chrome MUST render verbatim so the
//!   wedge does not overclaim cross-workspace accuracy.
//! - A live [`GraphStateCardMount`] that subscribes to the shared
//!   [`aureline_reactive_state::LiveReactiveStore`] and refreshes the card
//!   record whenever the readiness projection moves, so the card is tied to
//!   real reactive state rather than static mock data.
//!
//! ## Out of scope (deliberately)
//!
//! - Broad graph-query platform features (semantic refactor scope expansion,
//!   cross-workspace traversal, public-graph queries).
//! - Mutation, write, or apply flows. The card is read-only; the destructive
//!   core wedge owns its own preview/apply/revert lifecycle.
//! - Synthesizing graph evidence the upstream graph crate cannot prove. If
//!   the upstream basis is unavailable, the card surfaces
//!   `unavailable_no_basis` rather than fabricating a confidence level.

use std::cell::RefCell;
use std::rc::Rc;

use serde::{Deserialize, Serialize};

use aureline_graph_proto::{
    ConfidenceLevel, NodeClass, ProvenanceClass, QueryFamilyTag, ShardAffinityTag, SourceClass,
    Visibility, WorksetScopeClass,
};
use aureline_reactive_state::{
    LiveReactiveStore, LiveSubscriptionToken, ReadinessLabel, ReadinessProjection, StoreError,
    WorkspaceReadinessSnapshot,
};

use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried in serialized graph-state-card payloads.
pub const GRAPH_STATE_CARD_RECORD_KIND: &str = "graph_state_card_record";

/// Schema version for the [`GraphStateCardRecord`] payload shape.
pub const GRAPH_STATE_CARD_SCHEMA_VERSION: u32 = 1;

/// Prototype label carried on every card. The chrome quotes the token
/// verbatim; surfaces MUST NOT drop the chip even when the card's readiness
/// label is `exact`, because the wedge as a whole is a bounded prototype.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype of the target graph state card. Read-only. Not a
    /// production graph platform; not cross-workspace; not authoritative for
    /// refactor scope expansion.
    M1PrototypeGraphReadinessCard,
}

impl PrototypeLabel {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeGraphReadinessCard => "m1_prototype_graph_readiness_card",
        }
    }

    /// Human-readable chip label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeGraphReadinessCard => {
                "Prototype — graph readiness card (workspace-local only)"
            }
        }
    }
}

/// Closed basis-class vocabulary. Classifies what kind of basis the current
/// target-graph claim rests on. Surfaces MUST quote the token verbatim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraphBasisClass {
    /// The producer is the authoritative workspace-VFS / symbol resolver
    /// for the current target, the freshness is authoritative, and the
    /// coverage is full. The chrome MAY render the target as Ready.
    LiveWorkspaceAuthoritative,
    /// The current basis was carried in from an external snapshot
    /// (`Imported` readiness). Surfaces MUST badge the card as `Cached`
    /// and MUST NOT claim authority for it.
    ImportedBundle,
    /// Heuristic inference (`Heuristic` readiness). The chrome MUST badge
    /// the card as `Limited` and MUST surface the confidence rollup.
    HeuristicInference,
    /// Cached / warming basis served while the producer reindexes. The
    /// chrome MUST badge the card as `Cached` or `Partial` (depending on
    /// completeness) and MUST surface the warming hint.
    CachedWarming,
    /// The previous basis was invalidated by an upstream signal and the
    /// producer has not republished. Surfaces MUST badge the card as
    /// `Stale` and MUST surface the not-ready reason.
    StaleAfterInvalidation,
    /// The current scope is narrower than the workspace OR coverage is
    /// incomplete. Surfaces MUST badge the card as `Partial` and MUST
    /// quote `claim_limits` so the user knows what is missing.
    PartialSubscope,
    /// The producer cannot serve a claim for the current target. The
    /// chrome MUST surface `Offline` and an explicit reason.
    UnavailableNoBasis,
    /// The current subject is outside the active workspace scope. Not a
    /// failure: the chrome MUST surface `Out of scope` and offer the user
    /// a path to widen scope.
    OutOfScopeForCurrentWorkspace,
}

impl GraphBasisClass {
    /// Stable token used in exported evidence.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveWorkspaceAuthoritative => "live_workspace_authoritative",
            Self::ImportedBundle => "imported_bundle",
            Self::HeuristicInference => "heuristic_inference",
            Self::CachedWarming => "cached_warming",
            Self::StaleAfterInvalidation => "stale_after_invalidation",
            Self::PartialSubscope => "partial_subscope",
            Self::UnavailableNoBasis => "unavailable_no_basis",
            Self::OutOfScopeForCurrentWorkspace => "out_of_scope_for_current_workspace",
        }
    }

    /// Human-readable label suitable for the chip body.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LiveWorkspaceAuthoritative => "Live workspace basis",
            Self::ImportedBundle => "Imported bundle basis",
            Self::HeuristicInference => "Heuristic inference",
            Self::CachedWarming => "Cached / warming basis",
            Self::StaleAfterInvalidation => "Stale after invalidation",
            Self::PartialSubscope => "Partial subscope coverage",
            Self::UnavailableNoBasis => "Unavailable — no basis",
            Self::OutOfScopeForCurrentWorkspace => "Out of scope for current workspace",
        }
    }

    /// True when this basis is authoritative for the target. Only one
    /// variant returns true; the chrome MUST NOT advertise authority on any
    /// other basis class.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::LiveWorkspaceAuthoritative)
    }

    /// Map the basis class to the canonical chrome [`DegradedStateToken`].
    /// The mapping is the sole place that crosses the graph-readiness
    /// vocabulary into the chrome's degraded-state vocabulary; the chrome
    /// MUST quote the resulting token verbatim. `LiveWorkspaceAuthoritative`
    /// returns `None` (no degraded badge); every other basis class returns
    /// the closest existing degraded token without inventing a synonym.
    pub const fn to_degraded_token(self) -> Option<DegradedStateToken> {
        match self {
            Self::LiveWorkspaceAuthoritative => None,
            Self::ImportedBundle => Some(DegradedStateToken::Cached),
            Self::HeuristicInference => Some(DegradedStateToken::Limited),
            Self::CachedWarming => Some(DegradedStateToken::Warming),
            Self::StaleAfterInvalidation => Some(DegradedStateToken::Stale),
            Self::PartialSubscope => Some(DegradedStateToken::Partial),
            Self::UnavailableNoBasis => Some(DegradedStateToken::Offline),
            Self::OutOfScopeForCurrentWorkspace => Some(DegradedStateToken::Limited),
        }
    }
}

/// Inputs the graph wedge supplies for the target subject. The card joins
/// these with the live readiness projection; it never invents a graph-side
/// field the wedge cannot prove.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GraphStateCardSubject {
    /// Stable id of the target whose graph state the card describes. The
    /// chrome quotes this verbatim (e.g. `topology_walk:workset:hot_path`).
    pub target_id: String,
    /// Workspace-graph node-class the target maps to (from the closed graph
    /// vocab). The chrome renders this as part of the card body.
    pub node_class: NodeClass,
    /// Workspace-graph query-family tag the wedge is serving. The chrome
    /// renders this verbatim.
    pub query_family: QueryFamilyTag,
    /// Shard the wedge consults for this target.
    pub shard_affinity: ShardAffinityTag,
    /// Source class of the underlying producer for the target.
    pub source_class: SourceClass,
    /// Provenance class of the current frame. `AuthoritativeProducer` is
    /// the only class that can pair with [`GraphBasisClass::LiveWorkspaceAuthoritative`].
    pub provenance_class: ProvenanceClass,
    /// Workset / scope class of the current view.
    pub scope_class: WorksetScopeClass,
    /// Scope visibility for the target. `MissingInScope` always forces the
    /// card into [`GraphBasisClass::OutOfScopeForCurrentWorkspace`].
    pub scope_visibility: Visibility,
    /// Rolled-up confidence level of the underlying claim, if the producer
    /// can supply one. Mandatory when the basis is heuristic.
    pub rolled_up_confidence: Option<ConfidenceLevel>,
    /// Human-readable note that explains a partial / heuristic / stale
    /// basis (e.g. "Backend folders are excluded from the workset"). The
    /// chrome MUST quote this verbatim when present.
    pub partial_note: Option<String>,
}

/// One claim-limit row. The chrome quotes these verbatim under the card so
/// the wedge cannot overclaim cross-workspace accuracy or refactor-scope
/// reach. Each row carries a stable token plus a human-readable label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphStateClaimLimit {
    pub token: String,
    pub label: String,
}

impl GraphStateClaimLimit {
    fn new(token: &str, label: &str) -> Self {
        Self {
            token: token.to_owned(),
            label: label.to_owned(),
        }
    }

    /// Canonical claim-limit set carried on every M1 card. The order is
    /// stable; chrome MUST render them in this order.
    pub fn canonical_limits() -> Vec<Self> {
        vec![
            Self::new(
                "workspace_local_only",
                "Workspace-local only; not cross-workspace.",
            ),
            Self::new(
                "no_refactor_scope_expansion",
                "Not authoritative for refactor scope expansion.",
            ),
            Self::new(
                "no_public_graph_queries",
                "Not a public graph-query platform.",
            ),
        ]
    }
}

/// Serializable graph-state-card payload. The chrome renders this struct
/// directly; export and proof flows quote it verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GraphStateCardRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub workspace_id: String,
    pub target_id: String,
    pub node_class_token: String,
    pub query_family_token: String,
    pub shard_affinity_token: String,
    pub source_class_token: String,
    pub provenance_class_token: String,
    pub scope_class_token: String,
    pub scope_visibility_token: String,
    pub readiness_label_token: String,
    pub readiness_label_display: String,
    pub basis_class_token: String,
    pub basis_class_display: String,
    pub is_authoritative: bool,
    pub freshness_token: String,
    pub completeness_token: String,
    pub frame_class_token: String,
    pub snapshot_epoch: u64,
    pub delta_seq: u64,
    pub producer_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub producer_version: Option<String>,
    pub observed_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_ready_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rolled_up_confidence_token: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub partial_note: Option<String>,
    pub claim_limits: Vec<GraphStateClaimLimit>,
    pub summary_line: String,
}

impl GraphStateCardRecord {
    /// Render a deterministic plaintext block. Support exports and proof
    /// captures quote this verbatim — the format is stable across hosts and
    /// never bakes in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display
        ));
        out.push_str(&format!(
            "target {} ({}) — query_family={} shard={} scope={}\n",
            self.target_id,
            self.node_class_token,
            self.query_family_token,
            self.shard_affinity_token,
            self.scope_class_token,
        ));
        out.push_str(&format!(
            "readiness={} basis={} authoritative={}\n",
            self.readiness_label_token, self.basis_class_token, self.is_authoritative,
        ));
        out.push_str(&format!(
            "producer={} source={} provenance={} freshness={} completeness={} frame={}\n",
            self.producer_id,
            self.source_class_token,
            self.provenance_class_token,
            self.freshness_token,
            self.completeness_token,
            self.frame_class_token,
        ));
        out.push_str(&format!(
            "scope_visibility={} observed_at={} epoch={} seq={}\n",
            self.scope_visibility_token, self.observed_at, self.snapshot_epoch, self.delta_seq,
        ));
        if let Some(reason) = &self.not_ready_reason {
            out.push_str(&format!("not_ready_reason={}\n", reason));
        }
        if let Some(token) = &self.degraded_token {
            out.push_str(&format!("degraded_token={}\n", token));
        }
        if let Some(token) = &self.rolled_up_confidence_token {
            out.push_str(&format!("confidence={}\n", token));
        }
        if let Some(note) = &self.partial_note {
            out.push_str(&format!("partial_note={}\n", note));
        }
        out.push_str("claim_limits:\n");
        for limit in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", limit.token, limit.label));
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out
    }
}

/// Derive a [`GraphBasisClass`] mechanically from the readiness projection
/// plus the graph-side subject. The mapping is total and never widens
/// authority — the only way to reach `LiveWorkspaceAuthoritative` is for
/// the projection to read `exact`, the subject's provenance to be
/// `AuthoritativeProducer`, AND the scope visibility to be `FullyVisible`.
pub fn classify_graph_basis(
    projection: &ReadinessProjection,
    subject: &GraphStateCardSubject,
) -> GraphBasisClass {
    if matches!(subject.scope_visibility, Visibility::MissingInScope) {
        return GraphBasisClass::OutOfScopeForCurrentWorkspace;
    }
    match projection.readiness_label {
        ReadinessLabel::OutOfScope => GraphBasisClass::OutOfScopeForCurrentWorkspace,
        ReadinessLabel::Unavailable => GraphBasisClass::UnavailableNoBasis,
        ReadinessLabel::Stale => GraphBasisClass::StaleAfterInvalidation,
        ReadinessLabel::Imported => GraphBasisClass::ImportedBundle,
        ReadinessLabel::Heuristic => GraphBasisClass::HeuristicInference,
        ReadinessLabel::Partial => GraphBasisClass::PartialSubscope,
        ReadinessLabel::Exact => {
            // Authoritative is only honoured when every side aligns. Any
            // mismatch downgrades to partial-subscope so the wedge cannot
            // silently overclaim authority.
            let provenance_ok = matches!(
                subject.provenance_class,
                ProvenanceClass::AuthoritativeProducer
            );
            let scope_ok = matches!(subject.scope_visibility, Visibility::FullyVisible);
            if provenance_ok && scope_ok {
                GraphBasisClass::LiveWorkspaceAuthoritative
            } else {
                GraphBasisClass::PartialSubscope
            }
        }
    }
}

/// Materialize a [`GraphStateCardRecord`] from a readiness projection plus
/// the graph wedge's typed subject inputs. The card pins a graph basis
/// class through [`classify_graph_basis`] and quotes every upstream token
/// verbatim.
pub fn materialize_graph_state_card(
    projection: &ReadinessProjection,
    subject: &GraphStateCardSubject,
) -> GraphStateCardRecord {
    let basis = classify_graph_basis(projection, subject);
    let degraded = basis.to_degraded_token();
    let prototype = PrototypeLabel::M1PrototypeGraphReadinessCard;
    let summary = compose_summary_line(projection, subject, basis, degraded);
    GraphStateCardRecord {
        record_kind: GRAPH_STATE_CARD_RECORD_KIND.to_owned(),
        schema_version: GRAPH_STATE_CARD_SCHEMA_VERSION,
        prototype_label_token: prototype.as_str().to_owned(),
        prototype_label_display: prototype.label().to_owned(),
        workspace_id: projection.scope_ref.id.clone(),
        target_id: subject.target_id.clone(),
        node_class_token: subject.node_class.as_str().to_owned(),
        query_family_token: subject.query_family.as_str().to_owned(),
        shard_affinity_token: subject.shard_affinity.as_str().to_owned(),
        source_class_token: subject.source_class.as_str().to_owned(),
        provenance_class_token: subject.provenance_class.as_str().to_owned(),
        scope_class_token: subject.scope_class.as_str().to_owned(),
        scope_visibility_token: subject.scope_visibility.as_str().to_owned(),
        readiness_label_token: projection.readiness_label.as_str().to_owned(),
        readiness_label_display: projection.readiness_label.label().to_owned(),
        basis_class_token: basis.as_str().to_owned(),
        basis_class_display: basis.label().to_owned(),
        is_authoritative: basis.is_authoritative(),
        freshness_token: projection.freshness.as_str().to_owned(),
        completeness_token: projection.completeness.as_str().to_owned(),
        frame_class_token: projection.frame_class.as_str().to_owned(),
        snapshot_epoch: projection.snapshot_epoch,
        delta_seq: projection.delta_seq,
        producer_id: projection.producer_id.clone(),
        producer_version: projection.producer_version.clone(),
        observed_at: projection.observed_at.clone(),
        not_ready_reason: projection.not_ready_reason.map(|r| r.as_str().to_owned()),
        degraded_token: degraded.map(|t| t.token().to_owned()),
        rolled_up_confidence_token: subject.rolled_up_confidence.map(|c| c.as_str().to_owned()),
        partial_note: subject.partial_note.clone(),
        claim_limits: GraphStateClaimLimit::canonical_limits(),
        summary_line: summary,
    }
}

fn compose_summary_line(
    projection: &ReadinessProjection,
    subject: &GraphStateCardSubject,
    basis: GraphBasisClass,
    degraded: Option<DegradedStateToken>,
) -> String {
    let badge = degraded
        .map(|t| format!(" [{}]", t.label()))
        .unwrap_or_default();
    let reason = projection
        .not_ready_reason
        .map(|r| format!(" — {}", r.as_str()))
        .unwrap_or_default();
    format!(
        "{label}{badge}: {target} via {family} ({basis}){reason}",
        label = projection.readiness_label.label(),
        target = subject.target_id,
        family = subject.query_family.as_str(),
        basis = basis.as_str(),
    )
}

/// Live mount: subscribes to a shared [`LiveReactiveStore`] subscription
/// and refreshes the graph-state card whenever the underlying readiness
/// projection moves. The mount is the wedge's tie to real reactive state;
/// the chrome reads `latest_record()` rather than caching its own copy.
pub struct GraphStateCardMount {
    workspace_id: String,
    subject: GraphStateCardSubject,
    token: LiveSubscriptionToken,
    latest: Rc<RefCell<Option<GraphStateCardRecord>>>,
}

impl GraphStateCardMount {
    /// Mount a graph-state card onto an existing shared workspace-readiness
    /// subscription. The card starts populated with the latest projection
    /// cached on the store, if one is present.
    pub fn mount_existing(
        store: &LiveReactiveStore,
        subscription_id: u64,
        workspace_id: impl Into<String>,
        subject: GraphStateCardSubject,
    ) -> Result<Self, StoreError> {
        let latest: Rc<RefCell<Option<GraphStateCardRecord>>> = Rc::new(RefCell::new(None));
        let latest_inner = Rc::clone(&latest);
        let subject_inner = subject.clone();
        let token = store.subscribe(
            subscription_id,
            Rc::new(move |projection: &ReadinessProjection| {
                *latest_inner.borrow_mut() =
                    Some(materialize_graph_state_card(projection, &subject_inner));
            }),
        )?;
        Ok(Self {
            workspace_id: workspace_id.into(),
            subject,
            token,
            latest,
        })
    }

    /// Convenience constructor: open a shared workspace-readiness
    /// subscription on the live store, publish the initial snapshot, and
    /// mount one graph-state card on it. Returns the mount and the
    /// subscription id so a workspace-readiness chip can attach to the
    /// same subscription without inventing a private cache.
    pub fn open_and_mount(
        store: &LiveReactiveStore,
        snapshot: &WorkspaceReadinessSnapshot,
        subject: GraphStateCardSubject,
    ) -> Result<(Self, u64), StoreError> {
        let (sid, _initial) = aureline_reactive_state::open_workspace_readiness(store, snapshot)?;
        let mount = Self::mount_existing(store, sid, snapshot.workspace_id.clone(), subject)?;
        Ok((mount, sid))
    }

    /// Returns the latest rendered card record, if any frame has been
    /// published.
    pub fn latest_record(&self) -> Option<GraphStateCardRecord> {
        self.latest.borrow().clone()
    }

    /// Returns the workspace id this card was mounted for.
    pub fn workspace_id(&self) -> &str {
        &self.workspace_id
    }

    /// Returns the subject this card was mounted for.
    pub fn subject(&self) -> &GraphStateCardSubject {
        &self.subject
    }

    /// Returns the underlying shared subscription token.
    pub const fn token(&self) -> LiveSubscriptionToken {
        self.token
    }

    /// Detach the card from the live store. Subsequent publishes will not
    /// refresh `latest_record`.
    pub fn unmount(self, store: &LiveReactiveStore) -> Result<(), StoreError> {
        store.unsubscribe(self.token)
    }
}

#[cfg(test)]
mod tests;
