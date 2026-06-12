//! Canonical M5 explainer-snapshots-and-architecture-maps packet: the honest answer the M5 graph
//! surfaces give to *what is this part of the workspace, and what backs that explanation?*
//!
//! Where [`crate::m5_workset_scope`] answers *what slice am I looking at?*,
//! [`crate::m5_topology_identity`] answers *which exact graph object is this?*,
//! [`crate::m5_impact_query`] answers *is this empty impact answer safe?*,
//! [`crate::m5_ownership_and_contracts`] answers *who owns this, and what kind of source said so?*,
//! and [`crate::m5_graph_governance`] freezes *which depth claim a lane may publish*, this packet
//! answers the question onboarding tours, review explainers, the docs/browser surface, and AI
//! context inspectors all ask of an architecture explanation: **is this generated prose or curated
//! truth, what files, symbols, docs packs, ADRs, curated notes, and graph objects back it, and how
//! fresh and confident is it?** It carries one [`ExplainerSnapshot`] per explained subject — each
//! with the topology object it explains, a [`ExplanationSourceClass`] generated-versus-curated
//! label, freshness and confidence tokens, the [`ExplainerCitation`] ids that back it, the
//! [`NavigationAffordance`]s that keep its architecture map navigable without a canvas, and the
//! open [`FollowUpAction`] ids it surfaces — plus one [`ExplainerConsumerBinding`] per surface that
//! carries the same snapshot beyond a single panel render.
//!
//! Four invariants hold across the packet:
//!
//! - **Generated prose never stands without citations.** Every snapshot must cite at least one
//!   [`ExplainerCitation`] and carry a non-empty freshness and confidence token, so a generated
//!   architecture explanation never reads as a free-floating source of truth.
//! - **Source kind stays visible.** Every snapshot carries an explicit
//!   [`ExplanationSourceClass`], and every binding preserves that label rather than flattening it,
//!   so generated prose never silently reads as curated truth downstream.
//! - **Architecture maps are never canvas-only.** Every snapshot must offer the keyboard,
//!   list/table, and screen-reader [`NavigationAffordance`]s alongside any canvas, so topology
//!   exploration has an equivalent accessible path.
//! - **Exports never leak restricted prose.** Every binding declares the visibility ceiling it may
//!   carry, the support-export binding carries every export-safe snapshot and no private one, and
//!   the export projection redacts private snapshots entirely, so support and enterprise review can
//!   inspect an explanation without private product state or a canvas-only assumption.
//!
//! The packet reuses the stable topology identity space ([`TopologyNodeKind`]) and the active scope
//! snapshot ([`TopologyScopeAnchor`]) rather than minting one-off explainer strings per surface,
//! binds upstream to the canonical graph-depth governance matrix, the workset-scope packet, and the
//! topology-identity packet it extends, and stamps every consumer binding with the active scope
//! snapshot so replay can reconstruct the exact slice the user queried.
//!
//! The packet is checked in at `artifacts/graph/m5/m5-explainer-and-architecture-maps.json` and
//! embedded here. It is metadata-only: every field is a typed state, a count, a label, or an opaque
//! ref, and it carries no raw prose bodies, raw provider payloads, or graph node contents.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_topology_identity::{TopologyNodeKind, TopologyScopeAnchor};

/// Supported M5 explainer-and-architecture-maps packet schema version.
pub const M5_EXPLAINER_ARCHITECTURE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_EXPLAINER_ARCHITECTURE_RECORD_KIND: &str = "m5_explainer_and_architecture_maps_packet";

/// Repo-relative path to the checked-in packet.
pub const M5_EXPLAINER_ARCHITECTURE_PATH: &str =
    "artifacts/graph/m5/m5-explainer-and-architecture-maps.json";

/// Repo-relative path to the JSON Schema validating the packet.
pub const M5_EXPLAINER_ARCHITECTURE_SCHEMA_REF: &str =
    "schemas/graph/m5-explainer-and-architecture-maps.schema.json";

/// Repo-relative path to the companion document.
pub const M5_EXPLAINER_ARCHITECTURE_DOC_REF: &str =
    "docs/graph/m5/m5-explainer-and-architecture-maps.md";

/// Repo-relative path to the fixture corpus directory.
pub const M5_EXPLAINER_ARCHITECTURE_FIXTURE_DIR: &str =
    "fixtures/graph/m5/m5-explainer-and-architecture-maps";

/// Repo-relative path to the canonical graph-depth governance matrix this packet extends.
pub const M5_EXPLAINER_ARCHITECTURE_GOVERNANCE_MATRIX_REF: &str =
    "artifacts/graph/m5/m5-graph-governance.json";

/// Repo-relative path to the canonical workset-scope packet this packet is bound to.
pub const M5_EXPLAINER_ARCHITECTURE_SCOPE_PACKET_REF: &str =
    "artifacts/graph/m5/m5-workset-scope.json";

/// Repo-relative path to the canonical topology-identity packet whose id space this packet reuses.
pub const M5_EXPLAINER_ARCHITECTURE_TOPOLOGY_PACKET_REF: &str =
    "artifacts/graph/m5/m5-topology-identity.json";

/// Embedded checked-in packet JSON.
pub const M5_EXPLAINER_ARCHITECTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/graph/m5/m5-explainer-and-architecture-maps.json"
));

/// Whether an explanation is curated first-party prose, imported from a doc pack or provider, or
/// generated by an inference or summarization producer.
///
/// The generated-versus-curated label is the headline disclosure: a generated explanation must
/// never read as curated truth merely because it is fluent. The ordering is a precedence:
/// [`ExplanationSourceClass::Curated`] is the most authoritative and
/// [`ExplanationSourceClass::Generated`] the least.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplanationSourceClass {
    /// Curated first-party prose authored or reviewed by a maintainer.
    Curated,
    /// Prose imported from a connected doc pack or provider rather than authored locally.
    Imported,
    /// Prose generated by a heuristic, summarization, or AI producer; a synthesis, not declared
    /// truth.
    Generated,
}

impl ExplanationSourceClass {
    /// Every source class, in declaration (precedence) order.
    pub const ALL: [Self; 3] = [Self::Curated, Self::Imported, Self::Generated];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Curated => "curated",
            Self::Imported => "imported",
            Self::Generated => "generated",
        }
    }

    /// Precedence rank; lower is more authoritative.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::Curated => 0,
            Self::Imported => 1,
            Self::Generated => 2,
        }
    }

    /// Whether this class is curated first-party truth rather than imported or generated.
    pub const fn is_curated(self) -> bool {
        matches!(self, Self::Curated)
    }

    /// Whether this class is generated by an inference or summarization producer.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::Generated)
    }
}

/// What kind of object an explainer citation points at, kept distinct so an explanation can cite a
/// file, symbol, docs pack, ADR, curated note, or graph object without collapsing them into one
/// generic ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CitationKind {
    /// A workspace file.
    File,
    /// A code symbol such as a function, type, or method.
    Symbol,
    /// A documentation or knowledge pack.
    DocPack,
    /// An architecture decision record.
    Adr,
    /// A curated maintainer note.
    CuratedNote,
    /// A canonical graph node or edge object.
    GraphObject,
}

impl CitationKind {
    /// Every citation kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::File,
        Self::Symbol,
        Self::DocPack,
        Self::Adr,
        Self::CuratedNote,
        Self::GraphObject,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Symbol => "symbol",
            Self::DocPack => "doc_pack",
            Self::Adr => "adr",
            Self::CuratedNote => "curated_note",
            Self::GraphObject => "graph_object",
        }
    }
}

/// A way to navigate an architecture map, kept explicit so topology exploration is never
/// canvas-only.
///
/// [`NavigationAffordance::Keyboard`], [`NavigationAffordance::ListTable`], and
/// [`NavigationAffordance::ScreenReader`] are the required non-canvas paths every snapshot must
/// offer; [`NavigationAffordance::Canvas`] is the optional visual map.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NavigationAffordance {
    /// The visual canvas map.
    Canvas,
    /// Full keyboard navigation of the map.
    Keyboard,
    /// An equivalent list or table projection of the map.
    ListTable,
    /// An accessible screen-reader path through the map.
    ScreenReader,
}

impl NavigationAffordance {
    /// Every affordance, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Canvas,
        Self::Keyboard,
        Self::ListTable,
        Self::ScreenReader,
    ];

    /// The non-canvas affordances every snapshot must offer for an accessible, non-canvas-only path.
    pub const REQUIRED_NON_CANVAS: [Self; 3] =
        [Self::Keyboard, Self::ListTable, Self::ScreenReader];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Canvas => "canvas",
            Self::Keyboard => "keyboard",
            Self::ListTable => "list_table",
            Self::ScreenReader => "screen_reader",
        }
    }

    /// Whether this affordance is one of the required non-canvas paths.
    pub const fn is_non_canvas(self) -> bool {
        !matches!(self, Self::Canvas)
    }
}

/// An open follow-up an explainer snapshot surfaces, so an explanation records the next step rather
/// than leaving a stale or narrowed answer to read as complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FollowUpActionClass {
    /// Open a cited source at its anchor (a file, symbol, docs pack, or ADR).
    OpenCitedSource,
    /// Open the non-canvas architecture map projection.
    OpenArchitectureMap,
    /// Refresh a stale snapshot against the current index.
    RefreshStaleSnapshot,
    /// Widen the active workset scope to broaden the explanation.
    WidenScope,
    /// Request curated review to promote a generated explanation to curated truth.
    RequestCuratedReview,
}

impl FollowUpActionClass {
    /// Every follow-up action class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenCitedSource,
        Self::OpenArchitectureMap,
        Self::RefreshStaleSnapshot,
        Self::WidenScope,
        Self::RequestCuratedReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCitedSource => "open_cited_source",
            Self::OpenArchitectureMap => "open_architecture_map",
            Self::RefreshStaleSnapshot => "refresh_stale_snapshot",
            Self::WidenScope => "widen_scope",
            Self::RequestCuratedReview => "request_curated_review",
        }
    }
}

/// The visibility scope a snapshot may be shown within, keeping private or policy-scoped prose from
/// widening into exports or public-facing surfaces beyond its declared scope.
///
/// The ordering is a restrictiveness: [`ExplainerVisibility::Public`] is the least restricted and
/// [`ExplainerVisibility::Private`] the most.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainerVisibility {
    /// Shown anywhere, including public issue reports.
    Public,
    /// Shown in-product and in enterprise/support exports, but not public-facing surfaces.
    Internal,
    /// Restricted; shown only in-product to authorized users and never exported.
    Private,
}

impl ExplainerVisibility {
    /// Every visibility, in declaration (restrictiveness) order.
    pub const ALL: [Self; 3] = [Self::Public, Self::Internal, Self::Private];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::Internal => "internal",
            Self::Private => "private",
        }
    }

    /// Restrictiveness rank; higher is more restricted.
    pub const fn restrictiveness_rank(self) -> u8 {
        match self {
            Self::Public => 0,
            Self::Internal => 1,
            Self::Private => 2,
        }
    }

    /// Whether a snapshot at this visibility may appear in an export (support or enterprise);
    /// private snapshots never may.
    pub const fn is_export_safe(self) -> bool {
        matches!(self, Self::Public | Self::Internal)
    }

    /// Whether a snapshot at this visibility fits within a binding capped at `ceiling`.
    pub const fn fits_within(self, ceiling: Self) -> bool {
        self.restrictiveness_rank() <= ceiling.restrictiveness_rank()
    }
}

/// A surface that carries an explainer snapshot beyond the panel that first rendered it.
///
/// The closed vocabulary is exhaustive: every M5 surface that shows an architecture explanation
/// plus the durable support-export bundle, so onboarding, review, docs, and AI all point at one
/// shared snapshot rather than regenerating private prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExplainerConsumerSurface {
    /// The onboarding tour for a new contributor.
    OnboardingTour,
    /// The review-explanation surface.
    ReviewExplainer,
    /// The docs/browser surface.
    DocsBrowser,
    /// The AI context inspector.
    AiContextInspector,
    /// The support/export bundle.
    SupportExport,
}

impl ExplainerConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OnboardingTour,
        Self::ReviewExplainer,
        Self::DocsBrowser,
        Self::AiContextInspector,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingTour => "onboarding_tour",
            Self::ReviewExplainer => "review_explainer",
            Self::DocsBrowser => "docs_browser",
            Self::AiContextInspector => "ai_context_inspector",
            Self::SupportExport => "support_export",
        }
    }

    /// Whether this is the durable support-export surface that must carry every export-safe snapshot.
    pub const fn is_support_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// One citation backing an explainer snapshot: a file, symbol, docs pack, ADR, curated note, or
/// graph object the explanation rests on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainerCitation {
    /// Stable citation id inside the packet.
    pub citation_id: String,
    /// What kind of object this citation points at.
    pub kind: CitationKind,
    /// Canonical, stable ref to the cited object (a topology node id, docs anchor, or ADR ref).
    pub target_ref: String,
    /// Redaction-aware display label for the cited object.
    pub display_label: String,
    /// Freshness token copied from the cited object.
    pub freshness: String,
    /// Confidence token copied from the cited object.
    pub confidence: String,
    /// Export-safe, copy-safe permalink that embeds the canonical citation id.
    pub export_permalink: String,
}

impl ExplainerCitation {
    /// Whether the permalink is non-empty and embeds the canonical citation id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty()
            && self.export_permalink.contains(&self.citation_id)
    }
}

/// One open follow-up an explainer snapshot surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FollowUpAction {
    /// Stable action id inside the packet.
    pub action_id: String,
    /// What kind of follow-up this is.
    pub action_class: FollowUpActionClass,
    /// Ref the action opens or operates on.
    pub subject_ref: String,
    /// Redaction-aware reviewer-facing label.
    pub label: String,
}

/// One architecture-map explainer snapshot: what a topology object is, and what backs the
/// explanation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainerSnapshot {
    /// Stable snapshot id inside the packet.
    pub snapshot_id: String,
    /// Canonical, stable topology node id the snapshot explains.
    pub subject_id: String,
    /// Node kind of the explained subject.
    pub subject_kind: TopologyNodeKind,
    /// Redaction-aware title for the snapshot.
    pub title: String,
    /// Redaction-aware ref to the explanation prose; never the raw prose body.
    pub prose_ref: String,
    /// Whether the explanation is curated, imported, or generated.
    pub source_class: ExplanationSourceClass,
    /// Freshness token copied from the graph object.
    pub freshness: String,
    /// Confidence token copied from the graph object.
    pub confidence: String,
    /// Visibility scope this snapshot may be shown within.
    pub visibility: ExplainerVisibility,
    /// Citation ids backing this snapshot; every snapshot must cite at least one.
    #[serde(default)]
    pub cited_ids: Vec<String>,
    /// Navigation affordances this snapshot offers; the non-canvas paths are required.
    pub navigation_affordances: Vec<NavigationAffordance>,
    /// Open follow-up action ids this snapshot surfaces.
    #[serde(default)]
    pub follow_up_ids: Vec<String>,
    /// Export-safe, copy-safe permalink that embeds the canonical snapshot id.
    pub export_permalink: String,
}

impl ExplainerSnapshot {
    /// Whether the snapshot cites at least one citation, so generated prose never stands alone.
    pub fn is_cited(&self) -> bool {
        !self.cited_ids.is_empty()
    }

    /// Whether the snapshot carries non-empty freshness and confidence cues.
    pub fn carries_freshness_and_confidence(&self) -> bool {
        !self.freshness.trim().is_empty() && !self.confidence.trim().is_empty()
    }

    /// Whether the snapshot offers every required non-canvas navigation path.
    pub fn has_accessible_navigation(&self) -> bool {
        NavigationAffordance::REQUIRED_NON_CANVAS
            .iter()
            .all(|required| self.navigation_affordances.contains(required))
    }

    /// Whether the permalink is non-empty and embeds the canonical snapshot id.
    pub fn permalink_is_export_safe(&self) -> bool {
        !self.export_permalink.trim().is_empty()
            && self.export_permalink.contains(&self.snapshot_id)
    }

    /// Whether this snapshot may appear in an export.
    pub const fn is_export_safe(&self) -> bool {
        self.visibility.is_export_safe()
    }
}

/// One surface bound to the active scope snapshot, carrying a set of explainer snapshots forward.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExplainerConsumerBinding {
    /// Stable binding id inside the packet.
    pub binding_id: String,
    /// Surface this binding carries snapshots into.
    pub surface: ExplainerConsumerSurface,
    /// Snapshot id this surface is bound to; must equal the active scope snapshot id.
    pub snapshot_id: String,
    /// Scope id this surface renders; must equal the active scope id.
    pub scope_id: String,
    /// Visibility ceiling this surface may carry; a carried snapshot may not exceed it.
    pub max_visibility: ExplainerVisibility,
    /// Whether this surface preserves the snapshots' generated-versus-curated source labels rather
    /// than flattening them; must be true so generated prose never reads as curated truth downstream.
    pub preserves_source_labels: bool,
    /// Canonical snapshot ids this surface carries; every id must be declared in the packet.
    #[serde(default)]
    pub carries_snapshot_ids: Vec<String>,
    /// Ref to the surface artifact that ingests these snapshots.
    pub consumer_ref: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ExplainerArchitectureSummary {
    /// Total declared snapshots.
    pub snapshot_count: usize,
    /// Total declared citations.
    pub citation_count: usize,
    /// Total declared follow-up actions.
    pub follow_up_action_count: usize,
    /// Total consumer bindings.
    pub consumer_binding_count: usize,
    /// Number of distinct surfaces bound.
    pub surface_count: usize,
    /// Snapshots sourced as `curated`.
    pub curated_count: usize,
    /// Snapshots sourced as `imported`.
    pub imported_count: usize,
    /// Snapshots sourced as `generated`.
    pub generated_count: usize,
    /// Snapshots at `public` visibility.
    pub public_count: usize,
    /// Snapshots at `internal` visibility.
    pub internal_count: usize,
    /// Snapshots at `private` visibility.
    pub private_count: usize,
    /// Snapshots that are export-safe (public or internal).
    pub export_safe_snapshot_count: usize,
    /// Snapshots that cite at least one citation.
    pub cited_snapshot_count: usize,
    /// Snapshots that offer every required non-canvas navigation path.
    pub accessible_snapshot_count: usize,
    /// Citations of kind `file`.
    pub file_citation_count: usize,
    /// Citations of kind `symbol`.
    pub symbol_citation_count: usize,
    /// Citations of kind `doc_pack`.
    pub doc_pack_citation_count: usize,
    /// Citations of kind `adr`.
    pub adr_citation_count: usize,
    /// Citations of kind `curated_note`.
    pub curated_note_citation_count: usize,
    /// Citations of kind `graph_object`.
    pub graph_object_citation_count: usize,
    /// Total citation references across every snapshot.
    pub citation_ref_link_count: usize,
    /// Total follow-up references across every snapshot.
    pub follow_up_ref_link_count: usize,
}

/// A redaction-safe export row projected from one explainer snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplainerArchitectureExportRow {
    /// Canonical snapshot id.
    pub snapshot_id: String,
    /// Canonical subject id.
    pub subject_id: String,
    /// Snapshot title.
    pub title: String,
    /// Source-class token.
    pub source_class: String,
    /// Visibility token.
    pub visibility: String,
    /// Freshness token.
    pub freshness: String,
    /// Confidence token.
    pub confidence: String,
    /// Citation ids backing the snapshot.
    pub cited_ids: Vec<String>,
    /// Navigation affordance tokens the snapshot offers.
    pub navigation_affordances: Vec<String>,
    /// Open follow-up action ids the snapshot surfaces.
    pub follow_up_ids: Vec<String>,
    /// Export-safe permalink that points at the exact snapshot.
    pub permalink: String,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet — the explainer index downstream surfaces
/// render instead of re-describing architecture by hand. Private snapshots are redacted entirely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ExplainerArchitectureExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Active snapshot id every binding is stamped with.
    pub snapshot_id: String,
    /// Active scope id.
    pub scope_id: String,
    /// Active scope-mode token.
    pub scope_mode: String,
    /// Projected export-safe snapshot rows.
    pub snapshots: Vec<M5ExplainerArchitectureExportRow>,
    /// Count of private snapshots withheld from the export.
    pub redacted_private_count: usize,
    /// Whether every snapshot cites at least one citation.
    pub every_snapshot_cited: bool,
    /// Whether every snapshot offers the required non-canvas navigation paths.
    pub every_snapshot_accessible: bool,
    /// Whether every binding preserves the snapshots' source-class labels.
    pub source_labels_preserved_everywhere: bool,
    /// Whether every export-safe snapshot is carried by the support-export binding.
    pub every_export_safe_snapshot_in_support_export: bool,
    /// Whether the support-export binding carries no private snapshot.
    pub no_private_in_support_export: bool,
}

/// The typed M5 explainer-and-architecture-maps packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5ExplainerArchitecturePacket {
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
    /// Ref to the canonical graph-depth governance matrix this packet extends.
    pub governance_matrix_ref: String,
    /// Ref to the canonical workset-scope packet this packet is bound to.
    pub scope_packet_ref: String,
    /// Ref to the canonical topology-identity packet whose id space this packet reuses.
    pub topology_packet_ref: String,
    /// Ref to the graph-conformance suite backing the packet.
    pub conformance_ref: String,
    /// Ref binding this packet into the release-evidence surface.
    pub release_evidence_ref: String,
    /// Ref binding this packet into the help/service-health surface.
    pub help_surface_ref: String,
    /// Ref binding this packet into the docs-badge surface.
    pub docs_badge_ref: String,
    /// Ref binding this packet into the support-export surface.
    pub support_export_ref: String,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<ExplanationSourceClass>,
    /// Closed citation-kind vocabulary.
    pub citation_kinds: Vec<CitationKind>,
    /// Closed navigation-affordance vocabulary.
    pub navigation_affordances: Vec<NavigationAffordance>,
    /// Closed follow-up-action vocabulary.
    pub follow_up_action_classes: Vec<FollowUpActionClass>,
    /// Closed visibility vocabulary.
    pub visibilities: Vec<ExplainerVisibility>,
    /// Closed consumer-surface vocabulary.
    pub consumer_surfaces: Vec<ExplainerConsumerSurface>,
    /// The active scope snapshot every binding is stamped with.
    pub active_scope: TopologyScopeAnchor,
    /// Declared citations.
    #[serde(default)]
    pub citations: Vec<ExplainerCitation>,
    /// Declared follow-up actions.
    #[serde(default)]
    pub follow_up_actions: Vec<FollowUpAction>,
    /// Declared explainer snapshots.
    #[serde(default)]
    pub snapshots: Vec<ExplainerSnapshot>,
    /// Consumer bindings, one per surface.
    #[serde(default)]
    pub consumer_bindings: Vec<ExplainerConsumerBinding>,
    /// Summary counts.
    pub summary: M5ExplainerArchitectureSummary,
}

impl M5ExplainerArchitecturePacket {
    /// Returns the snapshot for a snapshot id.
    pub fn snapshot(&self, snapshot_id: &str) -> Option<&ExplainerSnapshot> {
        self.snapshots.iter().find(|s| s.snapshot_id == snapshot_id)
    }

    /// Returns the citation for a citation id.
    pub fn citation(&self, citation_id: &str) -> Option<&ExplainerCitation> {
        self.citations.iter().find(|c| c.citation_id == citation_id)
    }

    /// Returns the follow-up action for an action id.
    pub fn follow_up(&self, action_id: &str) -> Option<&FollowUpAction> {
        self.follow_up_actions
            .iter()
            .find(|a| a.action_id == action_id)
    }

    /// Returns the binding for a surface.
    pub fn consumer_binding(
        &self,
        surface: ExplainerConsumerSurface,
    ) -> Option<&ExplainerConsumerBinding> {
        self.consumer_bindings.iter().find(|b| b.surface == surface)
    }

    /// Returns the export-safe permalink for a snapshot id.
    pub fn permalink_for_snapshot(&self, snapshot_id: &str) -> Option<&str> {
        self.snapshot(snapshot_id)
            .map(|s| s.export_permalink.as_str())
    }

    /// Whether every snapshot cites at least one citation, so a generated explanation never stands
    /// without citations.
    pub fn every_snapshot_cited(&self) -> bool {
        self.snapshots.iter().all(ExplainerSnapshot::is_cited)
    }

    /// Whether every snapshot offers the required non-canvas navigation paths.
    pub fn every_snapshot_accessible(&self) -> bool {
        self.snapshots
            .iter()
            .all(ExplainerSnapshot::has_accessible_navigation)
    }

    /// Whether every binding preserves the snapshots' source-class labels.
    pub fn source_labels_preserved_everywhere(&self) -> bool {
        self.consumer_bindings
            .iter()
            .all(|b| b.preserves_source_labels)
    }

    /// Whether every export-safe snapshot is carried by the support-export binding.
    pub fn every_export_safe_snapshot_in_support_export(&self) -> bool {
        let Some(binding) = self.consumer_binding(ExplainerConsumerSurface::SupportExport) else {
            return self.snapshots.iter().all(|s| !s.is_export_safe());
        };
        let carried: BTreeSet<&str> = binding
            .carries_snapshot_ids
            .iter()
            .map(String::as_str)
            .collect();
        self.snapshots
            .iter()
            .filter(|s| s.is_export_safe())
            .all(|s| carried.contains(s.snapshot_id.as_str()))
    }

    /// Whether the support-export binding carries no private snapshot.
    pub fn no_private_in_support_export(&self) -> bool {
        let Some(binding) = self.consumer_binding(ExplainerConsumerSurface::SupportExport) else {
            return true;
        };
        binding
            .carries_snapshot_ids
            .iter()
            .all(|id| match self.snapshot(id) {
                Some(s) => s.visibility.is_export_safe(),
                None => true,
            })
    }

    /// Recomputes the summary block from the snapshots, citations, follow-ups, and bindings.
    pub fn computed_summary(&self) -> M5ExplainerArchitectureSummary {
        let source_count = |class: ExplanationSourceClass| {
            self.snapshots
                .iter()
                .filter(|s| s.source_class == class)
                .count()
        };
        let visibility_count = |visibility: ExplainerVisibility| {
            self.snapshots
                .iter()
                .filter(|s| s.visibility == visibility)
                .count()
        };
        let citation_kind_count =
            |kind: CitationKind| self.citations.iter().filter(|c| c.kind == kind).count();
        let distinct_surfaces: BTreeSet<ExplainerConsumerSurface> =
            self.consumer_bindings.iter().map(|b| b.surface).collect();
        M5ExplainerArchitectureSummary {
            snapshot_count: self.snapshots.len(),
            citation_count: self.citations.len(),
            follow_up_action_count: self.follow_up_actions.len(),
            consumer_binding_count: self.consumer_bindings.len(),
            surface_count: distinct_surfaces.len(),
            curated_count: source_count(ExplanationSourceClass::Curated),
            imported_count: source_count(ExplanationSourceClass::Imported),
            generated_count: source_count(ExplanationSourceClass::Generated),
            public_count: visibility_count(ExplainerVisibility::Public),
            internal_count: visibility_count(ExplainerVisibility::Internal),
            private_count: visibility_count(ExplainerVisibility::Private),
            export_safe_snapshot_count: self
                .snapshots
                .iter()
                .filter(|s| s.is_export_safe())
                .count(),
            cited_snapshot_count: self.snapshots.iter().filter(|s| s.is_cited()).count(),
            accessible_snapshot_count: self
                .snapshots
                .iter()
                .filter(|s| s.has_accessible_navigation())
                .count(),
            file_citation_count: citation_kind_count(CitationKind::File),
            symbol_citation_count: citation_kind_count(CitationKind::Symbol),
            doc_pack_citation_count: citation_kind_count(CitationKind::DocPack),
            adr_citation_count: citation_kind_count(CitationKind::Adr),
            curated_note_citation_count: citation_kind_count(CitationKind::CuratedNote),
            graph_object_citation_count: citation_kind_count(CitationKind::GraphObject),
            citation_ref_link_count: self.snapshots.iter().map(|s| s.cited_ids.len()).sum(),
            follow_up_ref_link_count: self.snapshots.iter().map(|s| s.follow_up_ids.len()).sum(),
        }
    }

    /// Produces the explainer index downstream surfaces — release evidence, help/service-health,
    /// docs badges, onboarding tours, review explainers, the docs/browser surface, AI context
    /// inspectors, and support exports — render instead of re-describing architecture by hand.
    /// Private snapshots are redacted entirely.
    pub fn export_projection(&self) -> M5ExplainerArchitectureExportProjection {
        let snapshots = self
            .snapshots
            .iter()
            .filter(|s| s.is_export_safe())
            .map(|s| M5ExplainerArchitectureExportRow {
                snapshot_id: s.snapshot_id.clone(),
                subject_id: s.subject_id.clone(),
                title: s.title.clone(),
                source_class: s.source_class.as_str().to_owned(),
                visibility: s.visibility.as_str().to_owned(),
                freshness: s.freshness.clone(),
                confidence: s.confidence.clone(),
                cited_ids: s.cited_ids.clone(),
                navigation_affordances: s
                    .navigation_affordances
                    .iter()
                    .map(|a| a.as_str().to_owned())
                    .collect(),
                follow_up_ids: s.follow_up_ids.clone(),
                permalink: s.export_permalink.clone(),
                summary: format!(
                    "{} explainer for {} ({}): {} citation(s) [{}/{}]",
                    s.source_class.as_str(),
                    s.subject_id,
                    s.visibility.as_str(),
                    s.cited_ids.len(),
                    s.freshness,
                    s.confidence
                ),
            })
            .collect();
        M5ExplainerArchitectureExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            snapshot_id: self.active_scope.snapshot_id.clone(),
            scope_id: self.active_scope.scope_id.clone(),
            scope_mode: self.active_scope.scope_mode.as_str().to_owned(),
            snapshots,
            redacted_private_count: self
                .snapshots
                .iter()
                .filter(|s| !s.is_export_safe())
                .count(),
            every_snapshot_cited: self.every_snapshot_cited(),
            every_snapshot_accessible: self.every_snapshot_accessible(),
            source_labels_preserved_everywhere: self.source_labels_preserved_everywhere(),
            every_export_safe_snapshot_in_support_export: self
                .every_export_safe_snapshot_in_support_export(),
            no_private_in_support_export: self.no_private_in_support_export(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5ExplainerArchitectureViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_anchor(&mut violations);
        self.validate_citations(&mut violations);
        self.validate_follow_ups(&mut violations);
        self.validate_snapshots(&mut violations);
        self.validate_bindings(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5ExplainerArchitectureViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        if self.schema_version != M5_EXPLAINER_ARCHITECTURE_SCHEMA_VERSION {
            violations.push(M5ExplainerArchitectureViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_EXPLAINER_ARCHITECTURE_RECORD_KIND {
            violations.push(M5ExplainerArchitectureViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("governance_matrix_ref", &self.governance_matrix_ref),
            ("scope_packet_ref", &self.scope_packet_ref),
            ("topology_packet_ref", &self.topology_packet_ref),
            ("conformance_ref", &self.conformance_ref),
            ("release_evidence_ref", &self.release_evidence_ref),
            ("help_surface_ref", &self.help_surface_ref),
            ("docs_badge_ref", &self.docs_badge_ref),
            ("support_export_ref", &self.support_export_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ExplainerArchitectureViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        // The packet must bind upstream to the canonical governance matrix, workset-scope packet,
        // and topology-identity packet it extends, so an explanation has one provenance root.
        if self.governance_matrix_ref != M5_EXPLAINER_ARCHITECTURE_GOVERNANCE_MATRIX_REF {
            violations.push(M5ExplainerArchitectureViolation::GovernanceMatrixRefMismatch);
        }
        if self.scope_packet_ref != M5_EXPLAINER_ARCHITECTURE_SCOPE_PACKET_REF {
            violations.push(M5ExplainerArchitectureViolation::ScopePacketRefMismatch);
        }
        if self.topology_packet_ref != M5_EXPLAINER_ARCHITECTURE_TOPOLOGY_PACKET_REF {
            violations.push(M5ExplainerArchitectureViolation::TopologyPacketRefMismatch);
        }
        for (field, ok) in [
            (
                "source_classes",
                self.source_classes == ExplanationSourceClass::ALL.to_vec(),
            ),
            (
                "citation_kinds",
                self.citation_kinds == CitationKind::ALL.to_vec(),
            ),
            (
                "navigation_affordances",
                self.navigation_affordances == NavigationAffordance::ALL.to_vec(),
            ),
            (
                "follow_up_action_classes",
                self.follow_up_action_classes == FollowUpActionClass::ALL.to_vec(),
            ),
            (
                "visibilities",
                self.visibilities == ExplainerVisibility::ALL.to_vec(),
            ),
            (
                "consumer_surfaces",
                self.consumer_surfaces == ExplainerConsumerSurface::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5ExplainerArchitectureViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_anchor(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        for (field, value) in [
            ("snapshot_id", &self.active_scope.snapshot_id),
            ("scope_id", &self.active_scope.scope_id),
            ("taken_as_of", &self.active_scope.taken_as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5ExplainerArchitectureViolation::EmptyField {
                    id: "<active_scope>".to_owned(),
                    field_name: field,
                });
            }
        }
    }

    fn validate_citations(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        let mut seen_ids = BTreeSet::new();
        for citation in &self.citations {
            if !seen_ids.insert(citation.citation_id.clone()) {
                violations.push(M5ExplainerArchitectureViolation::DuplicateCitationId {
                    citation_id: citation.citation_id.clone(),
                });
            }
            for (field, value) in [
                ("citation_id", &citation.citation_id),
                ("target_ref", &citation.target_ref),
                ("display_label", &citation.display_label),
                ("freshness", &citation.freshness),
                ("confidence", &citation.confidence),
                ("export_permalink", &citation.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ExplainerArchitectureViolation::EmptyField {
                        id: citation.citation_id.clone(),
                        field_name: field,
                    });
                }
            }
            if !citation.permalink_is_export_safe() {
                violations.push(M5ExplainerArchitectureViolation::UnsafeCitationPermalink {
                    citation_id: citation.citation_id.clone(),
                });
            }
        }
    }

    fn validate_follow_ups(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        let mut seen_ids = BTreeSet::new();
        for action in &self.follow_up_actions {
            if !seen_ids.insert(action.action_id.clone()) {
                violations.push(M5ExplainerArchitectureViolation::DuplicateFollowUpId {
                    action_id: action.action_id.clone(),
                });
            }
            for (field, value) in [
                ("action_id", &action.action_id),
                ("subject_ref", &action.subject_ref),
                ("label", &action.label),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ExplainerArchitectureViolation::EmptyField {
                        id: action.action_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_snapshots(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        let declared_citations: BTreeSet<&str> = self
            .citations
            .iter()
            .map(|c| c.citation_id.as_str())
            .collect();
        let declared_follow_ups: BTreeSet<&str> = self
            .follow_up_actions
            .iter()
            .map(|a| a.action_id.as_str())
            .collect();

        let mut seen_ids = BTreeSet::new();
        for snapshot in &self.snapshots {
            if !seen_ids.insert(snapshot.snapshot_id.clone()) {
                violations.push(M5ExplainerArchitectureViolation::DuplicateSnapshotId {
                    snapshot_id: snapshot.snapshot_id.clone(),
                });
            }
            for (field, value) in [
                ("snapshot_id", &snapshot.snapshot_id),
                ("subject_id", &snapshot.subject_id),
                ("title", &snapshot.title),
                ("prose_ref", &snapshot.prose_ref),
                ("freshness", &snapshot.freshness),
                ("confidence", &snapshot.confidence),
                ("export_permalink", &snapshot.export_permalink),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ExplainerArchitectureViolation::EmptyField {
                        id: snapshot.snapshot_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Headline guardrail: a generated (or any) explanation must cite at least one citation,
            // so generated prose never stands as a free-floating source of architecture truth.
            if !snapshot.is_cited() {
                violations.push(M5ExplainerArchitectureViolation::SnapshotMissingCitations {
                    snapshot_id: snapshot.snapshot_id.clone(),
                    source_class: snapshot.source_class.as_str(),
                });
            }
            // An explanation must always carry freshness and confidence cues alongside its prose.
            if !snapshot.carries_freshness_and_confidence() {
                violations.push(
                    M5ExplainerArchitectureViolation::SnapshotMissingFreshnessOrConfidence {
                        snapshot_id: snapshot.snapshot_id.clone(),
                    },
                );
            }
            // Architecture maps are never canvas-only: every snapshot must offer the keyboard,
            // list/table, and screen-reader paths.
            if !snapshot.has_accessible_navigation() {
                violations.push(M5ExplainerArchitectureViolation::CanvasOnlyNavigation {
                    snapshot_id: snapshot.snapshot_id.clone(),
                });
            }
            if !snapshot.permalink_is_export_safe() {
                violations.push(M5ExplainerArchitectureViolation::UnsafeSnapshotPermalink {
                    snapshot_id: snapshot.snapshot_id.clone(),
                });
            }
            for citation_id in &snapshot.cited_ids {
                if !declared_citations.contains(citation_id.as_str()) {
                    violations.push(M5ExplainerArchitectureViolation::UnresolvedCitationRef {
                        snapshot_id: snapshot.snapshot_id.clone(),
                        citation_id: citation_id.clone(),
                    });
                }
            }
            for action_id in &snapshot.follow_up_ids {
                if !declared_follow_ups.contains(action_id.as_str()) {
                    violations.push(M5ExplainerArchitectureViolation::UnresolvedFollowUpRef {
                        snapshot_id: snapshot.snapshot_id.clone(),
                        action_id: action_id.clone(),
                    });
                }
            }
        }
    }

    fn validate_bindings(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        let snapshot_id = &self.active_scope.snapshot_id;
        let scope_id = &self.active_scope.scope_id;

        let mut seen_ids = BTreeSet::new();
        let mut seen_surfaces = BTreeSet::new();
        for binding in &self.consumer_bindings {
            if !seen_ids.insert(binding.binding_id.clone()) {
                violations.push(M5ExplainerArchitectureViolation::DuplicateBindingId {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if !seen_surfaces.insert(binding.surface) {
                violations.push(M5ExplainerArchitectureViolation::DuplicateSurfaceBinding {
                    surface: binding.surface.as_str(),
                });
            }
            for (field, value) in [
                ("binding_id", &binding.binding_id),
                ("snapshot_id", &binding.snapshot_id),
                ("scope_id", &binding.scope_id),
                ("consumer_ref", &binding.consumer_ref),
                ("note", &binding.note),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5ExplainerArchitectureViolation::EmptyField {
                        id: binding.binding_id.clone(),
                        field_name: field,
                    });
                }
            }
            // Every surface must preserve generated-versus-curated labels so a generated
            // explanation never reads as curated truth once it leaves the originating panel.
            if !binding.preserves_source_labels {
                violations.push(M5ExplainerArchitectureViolation::SourceLabelsNotPreserved {
                    binding_id: binding.binding_id.clone(),
                });
            }
            // Every binding must be stamped with the active snapshot and scope so support export
            // and replay can reconstruct the slice the user queried.
            if &binding.snapshot_id != snapshot_id {
                violations.push(M5ExplainerArchitectureViolation::SnapshotBindingMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            if &binding.scope_id != scope_id {
                violations.push(M5ExplainerArchitectureViolation::ScopeIdMismatch {
                    binding_id: binding.binding_id.clone(),
                });
            }
            for snapshot_ref in &binding.carries_snapshot_ids {
                let Some(snapshot) = self.snapshot(snapshot_ref) else {
                    violations.push(M5ExplainerArchitectureViolation::UnresolvedSnapshotRef {
                        binding_id: binding.binding_id.clone(),
                        snapshot_id: snapshot_ref.clone(),
                    });
                    continue;
                };
                // A binding may not carry a snapshot more restricted than its declared ceiling, so
                // private or policy-scoped prose never widens past its declared scope.
                if !snapshot.visibility.fits_within(binding.max_visibility) {
                    violations.push(M5ExplainerArchitectureViolation::VisibilityExceedsBinding {
                        binding_id: binding.binding_id.clone(),
                        snapshot_id: snapshot_ref.clone(),
                        visibility: snapshot.visibility.as_str(),
                        max_visibility: binding.max_visibility.as_str(),
                    });
                }
            }
        }

        // Every surface must carry a binding so onboarding, review, docs, and AI all point at the
        // same shared snapshot.
        for surface in ExplainerConsumerSurface::ALL {
            if !seen_surfaces.contains(&surface) {
                violations.push(M5ExplainerArchitectureViolation::MissingSurfaceBinding {
                    surface: surface.as_str(),
                });
            }
        }

        self.validate_support_export(violations);
    }

    fn validate_support_export(&self, violations: &mut Vec<M5ExplainerArchitectureViolation>) {
        let Some(binding) = self.consumer_binding(ExplainerConsumerSurface::SupportExport) else {
            return;
        };
        let carried: BTreeSet<&str> = binding
            .carries_snapshot_ids
            .iter()
            .map(String::as_str)
            .collect();
        for snapshot in &self.snapshots {
            // Guardrail: every export-safe snapshot must be carried by the durable support-export
            // surface, so support and enterprise review can cite it without a private lookup.
            if snapshot.is_export_safe() && !carried.contains(snapshot.snapshot_id.as_str()) {
                violations.push(
                    M5ExplainerArchitectureViolation::ExportSafeSnapshotMissingFromSupportExport {
                        snapshot_id: snapshot.snapshot_id.clone(),
                    },
                );
            }
            // Out-of-scope guardrail: a private snapshot must never widen into the export.
            if !snapshot.is_export_safe() && carried.contains(snapshot.snapshot_id.as_str()) {
                violations.push(
                    M5ExplainerArchitectureViolation::PrivateSnapshotInSupportExport {
                        snapshot_id: snapshot.snapshot_id.clone(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the M5 explainer-and-architecture-maps packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5ExplainerArchitectureViolation {
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
    /// A closed vocabulary is not canonical.
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
    /// The packet does not bind to the canonical governance matrix.
    GovernanceMatrixRefMismatch,
    /// The packet does not bind to the canonical workset-scope packet.
    ScopePacketRefMismatch,
    /// The packet does not bind to the canonical topology-identity packet.
    TopologyPacketRefMismatch,
    /// A citation id appears more than once.
    DuplicateCitationId {
        /// Duplicate citation id.
        citation_id: String,
    },
    /// A citation carries a permalink that is empty or does not embed its id.
    UnsafeCitationPermalink {
        /// Citation id.
        citation_id: String,
    },
    /// A follow-up action id appears more than once.
    DuplicateFollowUpId {
        /// Duplicate action id.
        action_id: String,
    },
    /// A snapshot id appears more than once.
    DuplicateSnapshotId {
        /// Duplicate snapshot id.
        snapshot_id: String,
    },
    /// A snapshot cites no citation, so its explanation would stand without backing.
    SnapshotMissingCitations {
        /// Snapshot id.
        snapshot_id: String,
        /// Source-class token.
        source_class: &'static str,
    },
    /// A snapshot carries no freshness or confidence cue.
    SnapshotMissingFreshnessOrConfidence {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A snapshot offers no equivalent non-canvas navigation path.
    CanvasOnlyNavigation {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A snapshot carries a permalink that is empty or does not embed its id.
    UnsafeSnapshotPermalink {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A snapshot cites a citation id not declared in the packet.
    UnresolvedCitationRef {
        /// Snapshot id.
        snapshot_id: String,
        /// Unresolved citation id.
        citation_id: String,
    },
    /// A snapshot references a follow-up action id not declared in the packet.
    UnresolvedFollowUpRef {
        /// Snapshot id.
        snapshot_id: String,
        /// Unresolved action id.
        action_id: String,
    },
    /// A binding does not preserve the snapshots' source-class labels.
    SourceLabelsNotPreserved {
        /// Binding id.
        binding_id: String,
    },
    /// A binding id appears more than once.
    DuplicateBindingId {
        /// Duplicate binding id.
        binding_id: String,
    },
    /// A surface carries more than one binding.
    DuplicateSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A surface has no binding.
    MissingSurfaceBinding {
        /// Surface token.
        surface: &'static str,
    },
    /// A binding is not stamped with the active snapshot id.
    SnapshotBindingMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding renders a scope id other than the active scope.
    ScopeIdMismatch {
        /// Binding id.
        binding_id: String,
    },
    /// A binding carries a snapshot id not declared in the packet.
    UnresolvedSnapshotRef {
        /// Binding id.
        binding_id: String,
        /// Unresolved snapshot id.
        snapshot_id: String,
    },
    /// A binding carries a snapshot more restricted than its declared visibility ceiling.
    VisibilityExceedsBinding {
        /// Binding id.
        binding_id: String,
        /// Carried snapshot id.
        snapshot_id: String,
        /// Snapshot visibility token.
        visibility: &'static str,
        /// Binding ceiling token.
        max_visibility: &'static str,
    },
    /// An export-safe snapshot is not carried by the support-export binding.
    ExportSafeSnapshotMissingFromSupportExport {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// A private snapshot is carried by the support-export binding.
    PrivateSnapshotInSupportExport {
        /// Snapshot id.
        snapshot_id: String,
    },
    /// The summary counts disagree with the packet body.
    SummaryMismatch,
}

impl fmt::Display for M5ExplainerArchitectureViolation {
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
            Self::GovernanceMatrixRefMismatch => write!(
                f,
                "packet governance_matrix_ref must be the canonical graph-depth governance matrix"
            ),
            Self::ScopePacketRefMismatch => write!(
                f,
                "packet scope_packet_ref must be the canonical workset-scope packet"
            ),
            Self::TopologyPacketRefMismatch => write!(
                f,
                "packet topology_packet_ref must be the canonical topology-identity packet"
            ),
            Self::DuplicateCitationId { citation_id } => {
                write!(f, "duplicate citation id {citation_id}")
            }
            Self::UnsafeCitationPermalink { citation_id } => write!(
                f,
                "citation {citation_id} has an empty permalink or one that does not embed its id"
            ),
            Self::DuplicateFollowUpId { action_id } => {
                write!(f, "duplicate follow-up action id {action_id}")
            }
            Self::DuplicateSnapshotId { snapshot_id } => {
                write!(f, "duplicate snapshot id {snapshot_id}")
            }
            Self::SnapshotMissingCitations {
                snapshot_id,
                source_class,
            } => write!(
                f,
                "{source_class} snapshot {snapshot_id} ships without any citation"
            ),
            Self::SnapshotMissingFreshnessOrConfidence { snapshot_id } => write!(
                f,
                "snapshot {snapshot_id} carries no freshness or confidence cue"
            ),
            Self::CanvasOnlyNavigation { snapshot_id } => write!(
                f,
                "snapshot {snapshot_id} offers no equivalent keyboard, list/table, and screen-reader path"
            ),
            Self::UnsafeSnapshotPermalink { snapshot_id } => write!(
                f,
                "snapshot {snapshot_id} has an empty permalink or one that does not embed its id"
            ),
            Self::UnresolvedCitationRef {
                snapshot_id,
                citation_id,
            } => write!(
                f,
                "snapshot {snapshot_id} cites {citation_id} that is not declared in the packet"
            ),
            Self::UnresolvedFollowUpRef {
                snapshot_id,
                action_id,
            } => write!(
                f,
                "snapshot {snapshot_id} references follow-up {action_id} that is not declared in the packet"
            ),
            Self::SourceLabelsNotPreserved { binding_id } => write!(
                f,
                "binding {binding_id} does not preserve source-class labels"
            ),
            Self::DuplicateBindingId { binding_id } => {
                write!(f, "duplicate binding id {binding_id}")
            }
            Self::DuplicateSurfaceBinding { surface } => {
                write!(f, "duplicate binding for surface {surface}")
            }
            Self::MissingSurfaceBinding { surface } => {
                write!(f, "missing binding for surface {surface}")
            }
            Self::SnapshotBindingMismatch { binding_id } => write!(
                f,
                "binding {binding_id} is not stamped with the active snapshot id"
            ),
            Self::ScopeIdMismatch { binding_id } => write!(
                f,
                "binding {binding_id} renders a scope other than the active scope"
            ),
            Self::UnresolvedSnapshotRef {
                binding_id,
                snapshot_id,
            } => write!(
                f,
                "binding {binding_id} carries snapshot {snapshot_id} that is not declared in the packet"
            ),
            Self::VisibilityExceedsBinding {
                binding_id,
                snapshot_id,
                visibility,
                max_visibility,
            } => write!(
                f,
                "binding {binding_id} carries {visibility} snapshot {snapshot_id} beyond its {max_visibility} ceiling"
            ),
            Self::ExportSafeSnapshotMissingFromSupportExport { snapshot_id } => write!(
                f,
                "export-safe snapshot {snapshot_id} is not carried by the support-export binding"
            ),
            Self::PrivateSnapshotInSupportExport { snapshot_id } => write!(
                f,
                "private snapshot {snapshot_id} may not be carried by the support-export binding"
            ),
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the packet body")
            }
        }
    }
}

impl Error for M5ExplainerArchitectureViolation {}

/// Loads the embedded M5 explainer-and-architecture-maps packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5ExplainerArchitecturePacket`].
pub fn current_m5_explainer_and_architecture_maps_packet(
) -> Result<M5ExplainerArchitecturePacket, serde_json::Error> {
    serde_json::from_str(M5_EXPLAINER_ARCHITECTURE_JSON)
}

#[cfg(test)]
mod tests;
