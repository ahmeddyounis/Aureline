//! M5 operator overview boards: concrete, trustworthy summaries over many
//! operator objects, bound to the frozen operator-surface matrix.
//!
//! The [operator-surface matrix](crate::m5_operator_surfaces) freezes the
//! *families* of operator surface — what an overview board, triage inbox, or
//! failover notice is, the one shared state vocabulary, and the invariants every
//! surface must hold. This lane builds the first real **overview boards** on top
//! of it: the incident-response, support-queue, admin-approvals, and
//! release-readiness boards an operator actually opens.
//!
//! An overview board is a summary over many operator objects. The hard part is
//! not drawing tiles — it is keeping the summary honest as it scales:
//!
//! 1. **Canonical object identity, never a dashboard-only id.** Every
//!    [`BoardTile`] carries an `object_ref` — the same canonical
//!    `aureline://` handle the incident, support, admin, and release detail
//!    surfaces use — and its open-detail route is that exact ref. Boards do not
//!    invent a parallel identity layer for the objects they summarize.
//! 2. **No silent green.** Each tile's [`BoardTile::effective_state`] is
//!    *computed* from its displayed state, freshness, and blocker/waiver state by
//!    [`compute_effective_state`]: a tile is `clear` only when its evidence is
//!    fresh and nothing is blocked or waived. A stale or waived tile can never be
//!    frozen as green — flipping one in the fixture flips an invariant and fails
//!    CI.
//! 3. **Owner and blocker/waiver state stay first-class.** Owner, decision
//!    right, and a visible blocker/waiver reason are required tile fields, not
//!    hover-only chrome.
//! 4. **Shared filters and saved views.** One [`FilterFacet`] vocabulary spans
//!    every board, and a [`SavedView`] names its filters and order (with a stated
//!    reason). Applying a view is deterministic.
//! 5. **Export parity.** [`BoardExportView`] freezes a board's default view as a
//!    machine-readable, ordered, filtered snapshot that preserves the exact
//!    filters, order, scope, freshness, ownership, and blocker/waiver semantics
//!    outside the live UI. [`export_board_view`] recomputes it, and an invariant
//!    asserts the frozen export equals re-applying the view.
//!
//! [`operator_board_set`] is the canonical binding: it builds the boards
//! deterministically and computes each [`BoardInvariant`]'s `holds` flag from the
//! built data, so the checked-in fixture and the replay gate freeze the contract
//! byte-for-byte. The record carries no endpoint URLs, hostnames, credentials,
//! raw payloads, or absolute paths — only opaque object refs, stable tokens, and
//! short reviewable sentences — so it is safe for support export.

use serde::{Deserialize, Serialize};

use crate::m5_operator_surfaces::{
    ConsumerClass, LiveSnapshotClass, OperatorStateClass, OperatorSurfaceClass, RedactionClass,
    ScopeClass, TokenDef,
};

#[cfg(test)]
mod tests;

/// Schema version for the operator-board set.
pub const M5_OPERATOR_BOARDS_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the operator-board set.
pub const M5_OPERATOR_BOARDS_SCHEMA_REF: &str = "schemas/ops/m5-operator-boards.schema.json";

/// Stable record-kind tag for the operator-board set.
pub const M5_OPERATOR_BOARDS_RECORD_KIND: &str = "m5_operator_board_set";

/// Stable id for the canonical operator-board set.
pub const M5_OPERATOR_BOARDS_SET_ID: &str = "m5-operator-boards:set:0001";

/// Evaluation stamp for the canonical set. Held as a constant so the canonical
/// binding stays deterministic and the fixture freezes byte-for-byte.
pub const M5_OPERATOR_BOARDS_AS_OF: &str = "2026-06-22T00:00:00Z";

/// The operator-surface matrix fixture this board set binds for object identity.
pub const M5_OPERATOR_BOARDS_MATRIX_REF: &str =
    "fixtures/ops/m5-operator-surfaces/canonical_matrix.json";

/// The matrix record kind this board set binds.
pub const M5_OPERATOR_BOARDS_MATRIX_RECORD_KIND: &str = "m5_operator_surface_matrix";

// ---------------------------------------------------------------------------
// Board families.
// ---------------------------------------------------------------------------

/// The first real overview boards this lane proves the shared contract with.
///
/// Each board is one operator-facing summary over many canonical objects, bound
/// to a surface family from the operator-surface matrix. Adding a board is a
/// breaking change to the set; the tokens are frozen here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardClass {
    /// Incident-response board: open incidents and review items, triaged.
    IncidentResponse,
    /// Support-queue board: open support cases, triaged.
    SupportQueue,
    /// Admin-approvals board: pending governance approval requests.
    AdminApprovals,
    /// Release-readiness board: release gates and the services they depend on.
    ReleaseReadiness,
}

impl BoardClass {
    /// All boards, in set order.
    pub const ALL: [Self; 4] = [
        Self::IncidentResponse,
        Self::SupportQueue,
        Self::AdminApprovals,
        Self::ReleaseReadiness,
    ];

    /// Stable snake_case token for this board.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentResponse => "incident_response",
            Self::SupportQueue => "support_queue",
            Self::AdminApprovals => "admin_approvals",
            Self::ReleaseReadiness => "release_readiness",
        }
    }

    /// Stable, namespaced board id.
    pub fn board_id(self) -> String {
        format!("operator_board.{}", self.as_str())
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IncidentResponse => "Incident response",
            Self::SupportQueue => "Support queue",
            Self::AdminApprovals => "Admin approvals",
            Self::ReleaseReadiness => "Release readiness",
        }
    }

    /// The operator-surface matrix family this board is an instance of, so the
    /// board renders the same surface contract rather than a per-surface clone.
    pub const fn surface(self) -> OperatorSurfaceClass {
        match self {
            Self::IncidentResponse | Self::SupportQueue | Self::AdminApprovals => {
                OperatorSurfaceClass::TriageInbox
            }
            Self::ReleaseReadiness => OperatorSurfaceClass::OperationalOverviewBoard,
        }
    }
}

// ---------------------------------------------------------------------------
// Canonical object kinds.
// ---------------------------------------------------------------------------

/// The kinds of canonical object a board tile summarizes.
///
/// Every kind resolves to an object the incident/support/admin/release detail
/// surfaces already own; a board never invents a new object type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectKind {
    /// A canonical incident record.
    IncidentRecord,
    /// A canonical support case.
    SupportCase,
    /// A canonical admin / governance approval request.
    AdminApprovalRequest,
    /// A canonical release gate.
    ReleaseGate,
    /// A canonical service-health record.
    ServiceHealthRecord,
    /// A canonical review item.
    ReviewItem,
}

impl ObjectKind {
    /// All object kinds, in set order.
    pub const ALL: [Self; 6] = [
        Self::IncidentRecord,
        Self::SupportCase,
        Self::AdminApprovalRequest,
        Self::ReleaseGate,
        Self::ServiceHealthRecord,
        Self::ReviewItem,
    ];

    /// Stable snake_case token for this object kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentRecord => "incident_record",
            Self::SupportCase => "support_case",
            Self::AdminApprovalRequest => "admin_approval_request",
            Self::ReleaseGate => "release_gate",
            Self::ServiceHealthRecord => "service_health_record",
            Self::ReviewItem => "review_item",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::IncidentRecord => "Incident record",
            Self::SupportCase => "Support case",
            Self::AdminApprovalRequest => "Admin approval request",
            Self::ReleaseGate => "Release gate",
            Self::ServiceHealthRecord => "Service-health record",
            Self::ReviewItem => "Review item",
        }
    }
}

// ---------------------------------------------------------------------------
// Freshness and blocker/waiver state.
// ---------------------------------------------------------------------------

/// The freshness age of the evidence behind a tile.
///
/// Mirrors the shared freshness age tokens used across the operator-surface
/// matrix. Only [`FreshnessClass::Fresh`] and [`FreshnessClass::Recent`] keep a
/// would-be-green tile green.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Fresh: confirmed within the live window.
    Fresh,
    /// Recent: confirmed recently, still green-eligible.
    Recent,
    /// Stale: past the green window.
    Stale,
    /// Very stale: well past the green window.
    VeryStale,
    /// Never: no confirmation has been seen.
    Never,
}

impl FreshnessClass {
    /// All freshness ages, oldest path last.
    pub const ALL: [Self; 5] = [
        Self::Fresh,
        Self::Recent,
        Self::Stale,
        Self::VeryStale,
        Self::Never,
    ];

    /// Stable snake_case token for this freshness age.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Stale => "stale",
            Self::VeryStale => "very_stale",
            Self::Never => "never",
        }
    }

    /// Whether this age still lets a would-be-green tile stay green.
    pub const fn green_eligible(self) -> bool {
        matches!(self, Self::Fresh | Self::Recent)
    }

    /// Age rank, ascending from freshest, used for freshness-ordered views.
    pub const fn age_rank(self) -> i64 {
        match self {
            Self::Fresh => 0,
            Self::Recent => 1,
            Self::Stale => 2,
            Self::VeryStale => 3,
            Self::Never => 4,
        }
    }
}

/// The blocker / waiver state of the object behind a tile.
///
/// A waiver never reads as green: a waived finding is acknowledged risk, so the
/// tile downgrades to attention and surfaces the waiver reason rather than
/// hiding it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerWaiverClass {
    /// No blocker and no waiver.
    None,
    /// An active blocker: the object cannot proceed and says why.
    Blocked,
    /// A blocker waived under acknowledged risk; never reads as green.
    Waived,
    /// A waiver whose validity has lapsed; the blocker is in force again.
    WaiverExpired,
}

impl BlockerWaiverClass {
    /// All blocker/waiver states, in set order.
    pub const ALL: [Self; 4] = [Self::None, Self::Blocked, Self::Waived, Self::WaiverExpired];

    /// Stable snake_case token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Blocked => "blocked",
            Self::Waived => "waived",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether a visible blocker/waiver reason is required for this state.
    pub const fn requires_reason(self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Computes a tile's effective state from its displayed state, freshness, and
/// blocker/waiver state.
///
/// This is the no-silent-green rule made executable: a tile is reported `clear`
/// only when its displayed state is `clear`, its evidence is fresh or recent, and
/// nothing is blocked or waived. An active blocker (or an expired waiver) forces
/// `blocked`; a live waiver forces `attention`; stale evidence downgrades a
/// would-be-green tile to `unconfirmed`.
pub fn compute_effective_state(
    displayed: OperatorStateClass,
    freshness: FreshnessClass,
    blocker_waiver: BlockerWaiverClass,
) -> OperatorStateClass {
    match blocker_waiver {
        BlockerWaiverClass::Blocked | BlockerWaiverClass::WaiverExpired => {
            OperatorStateClass::Blocked
        }
        BlockerWaiverClass::Waived => OperatorStateClass::Attention,
        BlockerWaiverClass::None => {
            if displayed == OperatorStateClass::Clear && !freshness.green_eligible() {
                OperatorStateClass::Unconfirmed
            } else {
                displayed
            }
        }
    }
}

/// Severity rank of an operator state, higher being more urgent. Used to order
/// boards by effective-state severity.
fn state_severity_rank(state: OperatorStateClass) -> i64 {
    use OperatorStateClass::*;
    match state {
        Blocked => 100,
        FailoverInProgress => 95,
        MigrationInProgress => 90,
        BoundaryDriftRecheckRequired => 85,
        ReadOnlyWindow => 70,
        DrainWindow => 65,
        Attention => 60,
        Unconfirmed => 50,
        Reconciling => 40,
        ScheduledWindow => 35,
        EmbeddedBoundaryHandoff => 30,
        ImportedSnapshotNoLive => 20,
        UnknownRequiresReview => 15,
        Clear => 0,
    }
}

// ---------------------------------------------------------------------------
// Shared filter / saved-view vocabulary.
// ---------------------------------------------------------------------------

/// The facets a saved view can filter a board on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterFacetClass {
    /// Filter by computed effective state.
    State,
    /// Filter by evidence freshness age.
    Freshness,
    /// Filter by owner.
    Owner,
    /// Filter by blocker / waiver state.
    BlockerWaiver,
    /// Filter by local-versus-shared scope.
    Scope,
    /// Filter by canonical object kind.
    ObjectKind,
}

impl FilterFacetClass {
    /// All facets, in vocabulary order.
    pub const ALL: [Self; 6] = [
        Self::State,
        Self::Freshness,
        Self::Owner,
        Self::BlockerWaiver,
        Self::Scope,
        Self::ObjectKind,
    ];

    /// Stable snake_case token for this facet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::State => "state",
            Self::Freshness => "freshness",
            Self::Owner => "owner",
            Self::BlockerWaiver => "blocker_waiver",
            Self::Scope => "scope",
            Self::ObjectKind => "object_kind",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::State => "Effective state",
            Self::Freshness => "Freshness",
            Self::Owner => "Owner",
            Self::BlockerWaiver => "Blocker / waiver",
            Self::Scope => "Scope",
            Self::ObjectKind => "Object kind",
        }
    }

    /// Whether the facet's values come from a closed token vocabulary (and so can
    /// be validated against [`FilterFacet::allowed_tokens`]). Owner is open.
    pub const fn closed_vocabulary(self) -> bool {
        !matches!(self, Self::Owner)
    }

    /// The closed token vocabulary for this facet, or an empty list when open.
    fn allowed_tokens(self) -> Vec<String> {
        let tokens: Vec<String> = match self {
            Self::State => OperatorStateClass::ALL
                .iter()
                .map(|s| s.as_str().to_owned())
                .collect(),
            Self::Freshness => FreshnessClass::ALL
                .iter()
                .map(|f| f.as_str().to_owned())
                .collect(),
            Self::Owner => Vec::new(),
            Self::BlockerWaiver => BlockerWaiverClass::ALL
                .iter()
                .map(|b| b.as_str().to_owned())
                .collect(),
            Self::Scope => strvec(&["local_private", "shared_team", "managed_org"]),
            Self::ObjectKind => ObjectKind::ALL
                .iter()
                .map(|k| k.as_str().to_owned())
                .collect(),
        };
        tokens
    }
}

/// The orders a saved view can sort a board by.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderKeyClass {
    /// Order by computed effective-state severity.
    EffectiveStateSeverity,
    /// Order by evidence freshness age.
    Freshness,
    /// Order by the tile's explicit rank.
    ExplicitRank,
    /// Order by owner.
    Owner,
}

impl OrderKeyClass {
    /// All order keys, in vocabulary order.
    pub const ALL: [Self; 4] = [
        Self::EffectiveStateSeverity,
        Self::Freshness,
        Self::ExplicitRank,
        Self::Owner,
    ];

    /// Stable snake_case token for this order key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectiveStateSeverity => "effective_state_severity",
            Self::Freshness => "freshness",
            Self::ExplicitRank => "explicit_rank",
            Self::Owner => "owner",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::EffectiveStateSeverity => "Effective-state severity",
            Self::Freshness => "Freshness",
            Self::ExplicitRank => "Explicit rank",
            Self::Owner => "Owner",
        }
    }
}

/// The actions a board exposes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoardActionClass {
    /// Open the canonical detail object behind a tile.
    OpenDetail,
    /// Add tiles to a compare set, each resolving to its canonical object.
    Compare,
    /// Export the current view as a frozen, machine-readable snapshot.
    ExportView,
    /// Save the current filters and order as a named view.
    SaveView,
    /// Apply a saved view.
    ApplyView,
    /// Adjust the live filters without saving.
    Filter,
}

impl BoardActionClass {
    /// Stable snake_case token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::Compare => "compare",
            Self::ExportView => "export_view",
            Self::SaveView => "save_view",
            Self::ApplyView => "apply_view",
            Self::Filter => "filter",
        }
    }

    /// Human-readable label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OpenDetail => "Open detail",
            Self::Compare => "Compare",
            Self::ExportView => "Export view",
            Self::SaveView => "Save view",
            Self::ApplyView => "Apply view",
            Self::Filter => "Filter",
        }
    }

    /// Whether the action resolves to a canonical detail object rather than only
    /// rearranging the board's own view state.
    pub const fn routes_to_canonical_object(self) -> bool {
        matches!(self, Self::OpenDetail | Self::Compare)
    }
}

// ---------------------------------------------------------------------------
// Record structs.
// ---------------------------------------------------------------------------

/// One filter facet in the shared filter vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterFacet {
    /// The facet.
    pub facet: FilterFacetClass,
    /// Stable token (equals `facet.as_str()`).
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// Whether the facet's values come from a closed token vocabulary.
    pub closed_vocabulary: bool,
    /// The closed token vocabulary, or an empty list for open facets like owner.
    pub allowed_tokens: Vec<String>,
}

/// One filter clause: a facet and the values that pass it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FilterClause {
    /// The facet this clause filters on.
    pub facet: FilterFacetClass,
    /// The values that pass; a tile passes the clause if its facet value is one
    /// of these (logical OR within a clause, AND across clauses).
    pub include_tokens: Vec<String>,
}

/// The order a saved view applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardOrder {
    /// The order key.
    pub key: OrderKeyClass,
    /// Whether the order is descending.
    pub descending: bool,
    /// One reviewable sentence naming the order, so the board never sorts by a
    /// hidden rule.
    pub reason: String,
}

/// A named, shareable filter-and-order over a board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedView {
    /// Stable, namespaced view id.
    pub view_id: String,
    /// Stable token, unique within the board.
    pub token: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the view.
    pub summary: String,
    /// Whether the view is shared with the team or private to its owner.
    pub shared: bool,
    /// Local-versus-shared scope of the view.
    pub scope: ScopeClass,
    /// The owner of the saved view.
    pub owner: String,
    /// The filter clauses, applied with AND across clauses.
    pub filters: Vec<FilterClause>,
    /// The order applied after filtering.
    pub order: BoardOrder,
}

/// One tile summarizing one canonical object on a board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardTile {
    /// Stable, board-namespaced presentation handle for the tile. This is not the
    /// object's identity; [`BoardTile::object_ref`] is.
    pub tile_id: String,
    /// The canonical object handle this tile summarizes — the same ref the detail
    /// surfaces use. Never a dashboard-only id.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// Short title.
    pub title: String,
    /// The owner, shown first-class and never hover-only.
    pub owner: String,
    /// Who holds the decision right for this object.
    pub decision_right: String,
    /// The state the board would headline before applying the no-silent-green
    /// downgrade.
    pub displayed_state: OperatorStateClass,
    /// The canonical evidence object behind the displayed state.
    pub evidence_ref: String,
    /// The freshness age of that evidence.
    pub freshness: FreshnessClass,
    /// The blocker / waiver state of the object.
    pub blocker_waiver: BlockerWaiverClass,
    /// A visible blocker/waiver reason; required whenever something is blocked or
    /// waived, never hidden behind hover-only chrome.
    pub blocker_reason: String,
    /// The computed effective state ([`compute_effective_state`]); a stale or
    /// waived tile can never be `clear`.
    pub effective_state: OperatorStateClass,
    /// Local-versus-shared scope of the object.
    pub scope: ScopeClass,
    /// The tile's explicit rank within its board's default order.
    pub rank: u32,
    /// The open-detail route; equals [`BoardTile::object_ref`] so open-detail and
    /// the summarized object stay the same canonical thing.
    pub open_detail_ref: String,
    /// Whether the tile can join a compare set.
    pub comparable: bool,
}

/// One action a board exposes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardAction {
    /// The action.
    pub action: BoardActionClass,
    /// Human-readable label.
    pub label: String,
    /// Whether the action resolves to a canonical detail object.
    pub routes_to_canonical_object: bool,
    /// One reviewable sentence describing the action.
    pub summary: String,
}

/// One row of a frozen board export, preserving the truth fields outside the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportedTileRow {
    /// 1-based position in the exported order.
    pub rank_in_export: u32,
    /// The tile presentation handle.
    pub tile_id: String,
    /// The canonical object handle.
    pub object_ref: String,
    /// The kind of canonical object.
    pub object_kind: ObjectKind,
    /// Short title.
    pub title: String,
    /// The owner.
    pub owner: String,
    /// Who holds the decision right.
    pub decision_right: String,
    /// The computed effective state.
    pub effective_state: OperatorStateClass,
    /// The evidence freshness age.
    pub freshness: FreshnessClass,
    /// The blocker / waiver state.
    pub blocker_waiver: BlockerWaiverClass,
    /// The visible blocker/waiver reason, preserved verbatim.
    pub blocker_reason: String,
    /// The open-detail route, preserved so the export still points at the
    /// canonical object.
    pub open_detail_ref: String,
}

/// A board's saved view, frozen as a machine-readable, ordered, filtered export.
///
/// The export preserves the exact filters, order, scope, ownership, freshness,
/// and blocker/waiver semantics of the live board so the truth survives outside
/// the UI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardExportView {
    /// Stable, namespaced export id.
    pub export_id: String,
    /// The board this export belongs to.
    pub board: BoardClass,
    /// The board's stable id.
    pub board_id: String,
    /// The applied view's id.
    pub applied_view_id: String,
    /// The applied view's token.
    pub applied_view_token: String,
    /// The applied filters, preserved verbatim.
    pub filters: Vec<FilterClause>,
    /// The applied order, preserved verbatim.
    pub order: BoardOrder,
    /// The applied view's scope.
    pub scope: ScopeClass,
    /// The applied view's owner.
    pub view_owner: String,
    /// The redaction posture of the export.
    pub redaction_class: RedactionClass,
    /// Live-versus-snapshot posture; always snapshot for a frozen export.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// One reviewable sentence summarizing the export.
    pub summary: String,
    /// The number of rows in the export.
    pub row_count: u32,
    /// The resolved, ordered rows.
    pub rows: Vec<ExportedTileRow>,
}

/// One overview board.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewBoard {
    /// The board family.
    pub board: BoardClass,
    /// Stable, namespaced board id.
    pub board_id: String,
    /// Human-readable label.
    pub label: String,
    /// One reviewable sentence describing the board.
    pub summary: String,
    /// The operator-surface matrix family this board is an instance of.
    pub surface: OperatorSurfaceClass,
    /// The bound surface's stable id (equals `surface.surface_id()`), so the
    /// board points at the matrix's surface rather than a parallel identity.
    pub surface_id: String,
    /// Local-versus-shared scope of the board's objects.
    pub scope: ScopeClass,
    /// The consumers that render this board.
    pub consumed_by: Vec<ConsumerClass>,
    /// The default redaction posture on export.
    pub default_redaction: RedactionClass,
    /// Live-versus-snapshot posture of the live board.
    pub live_vs_snapshot: LiveSnapshotClass,
    /// The token of the default saved view.
    pub default_view: String,
    /// The saved views over this board.
    pub saved_views: Vec<SavedView>,
    /// The actions this board exposes.
    pub actions: Vec<BoardAction>,
    /// The tiles summarizing canonical objects.
    pub tiles: Vec<BoardTile>,
    /// The frozen export of the default view, proving export parity.
    pub export: BoardExportView,
}

/// One frozen invariant, with a computed `holds` flag.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoardInvariant {
    /// Stable invariant id.
    pub invariant_id: String,
    /// The invariant statement.
    pub statement: String,
    /// Whether the built set satisfies the invariant.
    pub holds: bool,
}

/// The frozen operator-board set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverviewBoardSet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub m5_operator_boards_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable set id.
    pub set_id: String,
    /// Evaluation stamp.
    pub as_of: String,
    /// One reviewable sentence summarizing the set.
    pub summary: String,
    /// The operator-surface matrix fixture this set binds for object identity.
    pub matrix_ref: String,
    /// The matrix record kind this set binds.
    pub matrix_record_kind: String,
    /// The shared filter facets every board reuses.
    pub filter_facets: Vec<FilterFacet>,
    /// The order keys every board reuses.
    pub order_keys: Vec<TokenDef>,
    /// The canonical object kinds tiles point at.
    pub object_kinds: Vec<TokenDef>,
    /// The boards.
    pub boards: Vec<OverviewBoard>,
    /// The computed invariants.
    pub invariants: Vec<BoardInvariant>,
    /// Whether raw payloads are excluded (always true for this record).
    pub raw_payload_excluded: bool,
}

/// Error returned when the board set fails a structural consistency check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardValidationError {
    /// The failed check.
    pub reason: String,
}

impl std::fmt::Display for BoardValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "operator-board set invalid: {}", self.reason)
    }
}

impl std::error::Error for BoardValidationError {}

impl OverviewBoardSet {
    /// Returns the board, if present.
    pub fn board(&self, board: BoardClass) -> Option<&OverviewBoard> {
        self.boards.iter().find(|b| b.board == board)
    }

    /// Returns the filter facet, if present.
    pub fn facet(&self, facet: FilterFacetClass) -> Option<&FilterFacet> {
        self.filter_facets.iter().find(|f| f.facet == facet)
    }

    /// Whether every computed invariant holds.
    pub fn all_invariants_hold(&self) -> bool {
        self.invariants.iter().all(|i| i.holds)
    }

    /// Whether the record is safe to place in a support export: raw payloads are
    /// excluded and every ref is a repo-relative object ref or `aureline://`
    /// handle, never a URL, host, credential, or absolute path.
    pub fn is_support_export_safe(&self) -> bool {
        if !self.raw_payload_excluded {
            return false;
        }
        self.all_refs().iter().all(|r| is_export_safe_ref(r))
    }

    /// Every ref string carried by the set, for export-safety auditing.
    fn all_refs(&self) -> Vec<&str> {
        let mut refs = vec![self.matrix_ref.as_str(), self.schema_ref.as_str()];
        for board in &self.boards {
            for tile in &board.tiles {
                refs.push(tile.object_ref.as_str());
                refs.push(tile.evidence_ref.as_str());
                refs.push(tile.open_detail_ref.as_str());
            }
            for row in &board.export.rows {
                refs.push(row.object_ref.as_str());
                refs.push(row.open_detail_ref.as_str());
            }
        }
        refs
    }

    /// Re-checks structural consistency and returns an error on the first
    /// failure. Complements the computed [`BoardInvariant`]s with the uniqueness
    /// and parity checks a consumer relies on.
    pub fn validate(&self) -> Result<(), BoardValidationError> {
        let fail = |reason: String| Err(BoardValidationError { reason });

        if self.record_kind != M5_OPERATOR_BOARDS_RECORD_KIND {
            return fail(format!("unexpected record_kind {}", self.record_kind));
        }
        if self.schema_ref != M5_OPERATOR_BOARDS_SCHEMA_REF {
            return fail(format!("unexpected schema_ref {}", self.schema_ref));
        }
        if !self.raw_payload_excluded {
            return fail("raw_payload_excluded must be true".to_owned());
        }
        if self.matrix_record_kind != M5_OPERATOR_BOARDS_MATRIX_RECORD_KIND {
            return fail("matrix_record_kind must bind the operator-surface matrix".to_owned());
        }

        // Every board is present exactly once.
        for board in BoardClass::ALL {
            if self.boards.iter().filter(|b| b.board == board).count() != 1 {
                return fail(format!("board {} not present exactly once", board.as_str()));
            }
        }

        // Ids are unique across the whole set.
        if !all_unique(self.boards.iter().map(|b| b.board_id.as_str())) {
            return fail("board ids are not unique".to_owned());
        }
        if !all_unique(
            self.boards
                .iter()
                .flat_map(|b| b.saved_views.iter().map(|v| v.view_id.as_str())),
        ) {
            return fail("view ids are not unique".to_owned());
        }
        if !all_unique(
            self.boards
                .iter()
                .flat_map(|b| b.tiles.iter().map(|t| t.tile_id.as_str())),
        ) {
            return fail("tile ids are not unique".to_owned());
        }

        let matrix = crate::m5_operator_surfaces::operator_surface_matrix();

        for board in &self.boards {
            if board.board_id != board.board.board_id() {
                return fail(format!("board id mismatch for {}", board.board.as_str()));
            }
            // The board binds a matrix surface that exists, by the matrix's id.
            if board.surface != board.board.surface()
                || board.surface_id != board.surface.surface_id()
                || matrix.surface(board.surface).is_none()
            {
                return fail(format!(
                    "board {} does not bind a canonical matrix surface",
                    board.board.as_str()
                ));
            }
            if board.tiles.is_empty() {
                return fail(format!("board {} has no tiles", board.board.as_str()));
            }
            if board.saved_views.is_empty() {
                return fail(format!("board {} has no saved views", board.board.as_str()));
            }
            // The default view resolves to a saved view.
            let default = board
                .saved_views
                .iter()
                .find(|v| v.token == board.default_view);
            let Some(default) = default else {
                return fail(format!(
                    "board {} default view {} is not a saved view",
                    board.board.as_str(),
                    board.default_view
                ));
            };
            // Open-detail must be offered and route to canonical objects.
            if !board
                .actions
                .iter()
                .any(|a| a.action == BoardActionClass::OpenDetail && a.routes_to_canonical_object)
            {
                return fail(format!(
                    "board {} must offer a canonical open-detail action",
                    board.board.as_str()
                ));
            }
            for tile in &board.tiles {
                if !tile.object_ref.starts_with("aureline://") {
                    return fail(format!(
                        "tile {} object_ref is not a canonical handle",
                        tile.tile_id
                    ));
                }
                if tile.open_detail_ref != tile.object_ref {
                    return fail(format!(
                        "tile {} open-detail does not route to its object",
                        tile.tile_id
                    ));
                }
                if tile.owner.is_empty() || tile.decision_right.is_empty() {
                    return fail(format!("tile {} hides owner/decision-right", tile.tile_id));
                }
                if tile.blocker_waiver.requires_reason() && tile.blocker_reason.is_empty() {
                    return fail(format!(
                        "tile {} is blocked/waived without a visible reason",
                        tile.tile_id
                    ));
                }
                let expected = compute_effective_state(
                    tile.displayed_state,
                    tile.freshness,
                    tile.blocker_waiver,
                );
                if tile.effective_state != expected {
                    return fail(format!(
                        "tile {} effective state is not the computed no-silent-green state",
                        tile.tile_id
                    ));
                }
            }
            // Every saved view filters on defined facets, with valid values.
            for view in &board.saved_views {
                for clause in &view.filters {
                    let Some(facet) = self.facet(clause.facet) else {
                        return fail(format!(
                            "view {} filters on undefined facet {}",
                            view.view_id,
                            clause.facet.as_str()
                        ));
                    };
                    if facet.closed_vocabulary {
                        for value in &clause.include_tokens {
                            if !facet.allowed_tokens.contains(value) {
                                return fail(format!(
                                    "view {} uses value {} outside facet {}",
                                    view.view_id,
                                    value,
                                    clause.facet.as_str()
                                ));
                            }
                        }
                    }
                }
            }
            // Export parity: the frozen export equals re-applying the default view.
            let recomputed = compute_export(
                board.board,
                &board.board_id,
                board.default_redaction,
                &board.tiles,
                default,
            );
            if board.export != recomputed {
                return fail(format!(
                    "board {} export does not match its default view",
                    board.board.as_str()
                ));
            }
        }

        if !self.is_support_export_safe() {
            return fail("board set is not support-export safe".to_owned());
        }
        if !self.all_invariants_hold() {
            let failed: Vec<&str> = self
                .invariants
                .iter()
                .filter(|i| !i.holds)
                .map(|i| i.invariant_id.as_str())
                .collect();
            return fail(format!("invariants do not hold: {}", failed.join(", ")));
        }
        Ok(())
    }
}

fn all_unique<'a>(iter: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    iter.into_iter().all(|item| seen.insert(item))
}

/// Whether a ref is safe to export: a repo-relative object ref or opaque
/// `aureline://` handle, never a URL, host, credential, or absolute path.
fn is_export_safe_ref(r: &str) -> bool {
    if r.is_empty() || r.starts_with('/') || (r.contains("://") && !r.starts_with("aureline://")) {
        return false;
    }
    r.starts_with("schemas/")
        || r.starts_with("crates/")
        || r.starts_with("artifacts/")
        || r.starts_with("fixtures/")
        || r.starts_with("docs/")
        || r.starts_with("aureline://")
}

fn scope_token(scope: ScopeClass) -> &'static str {
    match scope {
        ScopeClass::LocalPrivate => "local_private",
        ScopeClass::SharedTeam => "shared_team",
        ScopeClass::ManagedOrg => "managed_org",
    }
}

// ---------------------------------------------------------------------------
// View application and export.
// ---------------------------------------------------------------------------

/// The value a tile presents for a filter facet.
fn tile_facet_value(tile: &BoardTile, facet: FilterFacetClass) -> String {
    match facet {
        FilterFacetClass::State => tile.effective_state.as_str().to_owned(),
        FilterFacetClass::Freshness => tile.freshness.as_str().to_owned(),
        FilterFacetClass::Owner => tile.owner.clone(),
        FilterFacetClass::BlockerWaiver => tile.blocker_waiver.as_str().to_owned(),
        FilterFacetClass::Scope => scope_token(tile.scope).to_owned(),
        FilterFacetClass::ObjectKind => tile.object_kind.as_str().to_owned(),
    }
}

/// Whether a tile passes every clause of a saved view (AND across clauses).
fn tile_passes(tile: &BoardTile, view: &SavedView) -> bool {
    view.filters.iter().all(|clause| {
        clause
            .include_tokens
            .contains(&tile_facet_value(tile, clause.facet))
    })
}

/// Applies a saved view to a tile set, returning the filtered, ordered tiles.
///
/// Deterministic: tiles are filtered by the view's clauses, ordered by the view's
/// order key, and tie-broken by `tile_id` so the result never depends on input
/// order.
pub fn apply_view<'a>(tiles: &'a [BoardTile], view: &SavedView) -> Vec<&'a BoardTile> {
    let mut kept: Vec<&BoardTile> = tiles.iter().filter(|t| tile_passes(t, view)).collect();
    kept.sort_by(|a, b| {
        let primary = match view.order.key {
            OrderKeyClass::EffectiveStateSeverity => {
                state_severity_rank(a.effective_state).cmp(&state_severity_rank(b.effective_state))
            }
            OrderKeyClass::Freshness => a.freshness.age_rank().cmp(&b.freshness.age_rank()),
            OrderKeyClass::ExplicitRank => a.rank.cmp(&b.rank),
            OrderKeyClass::Owner => a.owner.cmp(&b.owner),
        };
        let primary = if view.order.descending {
            primary.reverse()
        } else {
            primary
        };
        primary.then_with(|| a.tile_id.cmp(&b.tile_id))
    });
    kept
}

/// Builds the frozen export of a saved view over a tile set.
fn compute_export(
    board: BoardClass,
    board_id: &str,
    redaction_class: RedactionClass,
    tiles: &[BoardTile],
    view: &SavedView,
) -> BoardExportView {
    let ordered = apply_view(tiles, view);
    let rows: Vec<ExportedTileRow> = ordered
        .iter()
        .enumerate()
        .map(|(idx, tile)| ExportedTileRow {
            rank_in_export: (idx as u32) + 1,
            tile_id: tile.tile_id.clone(),
            object_ref: tile.object_ref.clone(),
            object_kind: tile.object_kind,
            title: tile.title.clone(),
            owner: tile.owner.clone(),
            decision_right: tile.decision_right.clone(),
            effective_state: tile.effective_state,
            freshness: tile.freshness,
            blocker_waiver: tile.blocker_waiver,
            blocker_reason: tile.blocker_reason.clone(),
            open_detail_ref: tile.open_detail_ref.clone(),
        })
        .collect();
    let row_count = rows.len() as u32;
    BoardExportView {
        export_id: format!("{board_id}.export.{}", view.token),
        board,
        board_id: board_id.to_owned(),
        applied_view_id: view.view_id.clone(),
        applied_view_token: view.token.clone(),
        filters: view.filters.clone(),
        order: view.order.clone(),
        scope: view.scope,
        view_owner: view.owner.clone(),
        redaction_class,
        live_vs_snapshot: LiveSnapshotClass::SnapshotOnly,
        summary: format!(
            "Frozen export of board {board_id} via saved view '{}' — {row_count} rows; filters, \
             order, scope, freshness, ownership, and blocker/waiver state preserved.",
            view.token
        ),
        row_count,
        rows,
    }
}

/// Exports a board's saved view by token, recomputing it from the live tiles.
///
/// Returns `None` if the token names no saved view on the board. Consumers use
/// this to produce a fresh export of any saved view; the canonical default-view
/// export frozen on each board is the same function applied to its default view.
pub fn export_board_view(board: &OverviewBoard, view_token: &str) -> Option<BoardExportView> {
    let view = board.saved_views.iter().find(|v| v.token == view_token)?;
    Some(compute_export(
        board.board,
        &board.board_id,
        board.default_redaction,
        &board.tiles,
        view,
    ))
}

// ---------------------------------------------------------------------------
// Canonical binding.
// ---------------------------------------------------------------------------

/// Builds the canonical operator-board set.
///
/// Deterministic: the same bytes every call. Tile effective states and each
/// board's default-view export are computed, and the invariant `holds` flags are
/// computed from the built boards, so an inconsistent edit flips an invariant
/// rather than silently passing.
pub fn operator_board_set() -> OverviewBoardSet {
    let filter_facets = build_filter_facets();
    let boards = build_boards();
    let invariants = compute_invariants(&boards);

    OverviewBoardSet {
        record_kind: M5_OPERATOR_BOARDS_RECORD_KIND.to_owned(),
        m5_operator_boards_schema_version: M5_OPERATOR_BOARDS_SCHEMA_VERSION,
        schema_ref: M5_OPERATOR_BOARDS_SCHEMA_REF.to_owned(),
        set_id: M5_OPERATOR_BOARDS_SET_ID.to_owned(),
        as_of: M5_OPERATOR_BOARDS_AS_OF.to_owned(),
        summary: "The first real Aureline operator overview boards — incident response, support \
                  queue, admin approvals, and release readiness — as trustworthy summaries over \
                  many canonical objects, with one shared filter/saved-view vocabulary, computed \
                  no-silent-green tile state, first-class owner and blocker/waiver state, canonical \
                  open-detail routing, and export parity, all bound to the operator-surface matrix."
            .to_owned(),
        matrix_ref: M5_OPERATOR_BOARDS_MATRIX_REF.to_owned(),
        matrix_record_kind: M5_OPERATOR_BOARDS_MATRIX_RECORD_KIND.to_owned(),
        filter_facets,
        order_keys: OrderKeyClass::ALL
            .iter()
            .map(|k| TokenDef {
                token: k.as_str().to_owned(),
                label: k.label().to_owned(),
            })
            .collect(),
        object_kinds: ObjectKind::ALL
            .iter()
            .map(|k| TokenDef {
                token: k.as_str().to_owned(),
                label: k.label().to_owned(),
            })
            .collect(),
        boards,
        invariants,
        raw_payload_excluded: true,
    }
}

fn build_filter_facets() -> Vec<FilterFacet> {
    FilterFacetClass::ALL
        .iter()
        .map(|facet| FilterFacet {
            facet: *facet,
            token: facet.as_str().to_owned(),
            label: facet.label().to_owned(),
            closed_vocabulary: facet.closed_vocabulary(),
            allowed_tokens: facet.allowed_tokens(),
        })
        .collect()
}

fn strvec(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn tile(
    board: BoardClass,
    n: u32,
    object_ref: &str,
    kind: ObjectKind,
    title: &str,
    owner: &str,
    decision_right: &str,
    displayed: OperatorStateClass,
    evidence_ref: &str,
    freshness: FreshnessClass,
    blocker_waiver: BlockerWaiverClass,
    blocker_reason: &str,
    scope: ScopeClass,
    rank: u32,
    comparable: bool,
) -> BoardTile {
    BoardTile {
        tile_id: format!("{}.tile.{n:04}", board.board_id()),
        object_ref: object_ref.to_owned(),
        object_kind: kind,
        title: title.to_owned(),
        owner: owner.to_owned(),
        decision_right: decision_right.to_owned(),
        displayed_state: displayed,
        evidence_ref: evidence_ref.to_owned(),
        freshness,
        blocker_waiver,
        blocker_reason: blocker_reason.to_owned(),
        effective_state: compute_effective_state(displayed, freshness, blocker_waiver),
        scope,
        rank,
        open_detail_ref: object_ref.to_owned(),
        comparable,
    }
}

fn clause(facet: FilterFacetClass, values: &[&str]) -> FilterClause {
    FilterClause {
        facet,
        include_tokens: strvec(values),
    }
}

fn order(key: OrderKeyClass, descending: bool, reason: &str) -> BoardOrder {
    BoardOrder {
        key,
        descending,
        reason: reason.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn view(
    board: BoardClass,
    token: &str,
    label: &str,
    summary: &str,
    shared: bool,
    scope: ScopeClass,
    owner: &str,
    filters: Vec<FilterClause>,
    order: BoardOrder,
) -> SavedView {
    SavedView {
        view_id: format!("{}.view.{token}", board.board_id()),
        token: token.to_owned(),
        label: label.to_owned(),
        summary: summary.to_owned(),
        shared,
        scope,
        owner: owner.to_owned(),
        filters,
        order,
    }
}

fn default_actions() -> Vec<BoardAction> {
    [
        (
            BoardActionClass::OpenDetail,
            "Open the canonical incident/support/admin/release object behind a tile.",
        ),
        (
            BoardActionClass::Compare,
            "Add tiles to a compare set; each entry resolves to its canonical object.",
        ),
        (
            BoardActionClass::ApplyView,
            "Apply a saved view's filters and order.",
        ),
        (
            BoardActionClass::SaveView,
            "Save the current filters and order as a named, shareable view.",
        ),
        (
            BoardActionClass::Filter,
            "Adjust the live filters across the shared facet vocabulary.",
        ),
        (
            BoardActionClass::ExportView,
            "Export the current view as a frozen, machine-readable snapshot.",
        ),
    ]
    .into_iter()
    .map(|(action, summary)| BoardAction {
        action,
        label: action.label().to_owned(),
        routes_to_canonical_object: action.routes_to_canonical_object(),
        summary: summary.to_owned(),
    })
    .collect()
}

/// Assembles a board, computing its default-view export from its own tiles.
#[allow(clippy::too_many_arguments)]
fn assemble_board(
    board: BoardClass,
    summary: &str,
    scope: ScopeClass,
    consumed_by: Vec<ConsumerClass>,
    default_redaction: RedactionClass,
    default_view: &str,
    saved_views: Vec<SavedView>,
    tiles: Vec<BoardTile>,
) -> OverviewBoard {
    let default = saved_views
        .iter()
        .find(|v| v.token == default_view)
        .expect("default view must be one of the saved views");
    let export = compute_export(board, &board.board_id(), default_redaction, &tiles, default);
    OverviewBoard {
        board,
        board_id: board.board_id(),
        label: board.label().to_owned(),
        summary: summary.to_owned(),
        surface: board.surface(),
        surface_id: board.surface().surface_id(),
        scope,
        consumed_by,
        default_redaction,
        live_vs_snapshot: LiveSnapshotClass::SnapshotCapable,
        default_view: default_view.to_owned(),
        saved_views,
        actions: default_actions(),
        tiles,
        export,
    }
}

fn build_boards() -> Vec<OverviewBoard> {
    use BlockerWaiverClass as BW;
    use ConsumerClass::*;
    use FreshnessClass as F;
    use OperatorStateClass as S;

    let incident = {
        let b = BoardClass::IncidentResponse;
        let tiles = vec![
            tile(
                b,
                1,
                "aureline://incident/inc-2048",
                ObjectKind::IncidentRecord,
                "Auth provider latency",
                "on_call_driver",
                "incident_commander",
                S::Attention,
                "aureline://evidence/inc-2048-alert",
                F::Fresh,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                2,
                true,
            ),
            tile(
                b,
                2,
                "aureline://incident/inc-2049",
                ObjectKind::IncidentRecord,
                "Index rebuild backlog",
                "on_call_driver",
                "incident_commander",
                S::Clear,
                "aureline://evidence/inc-2049-metric",
                F::Stale,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                3,
                true,
            ),
            tile(
                b,
                3,
                "aureline://incident/inc-2050",
                ObjectKind::IncidentRecord,
                "Managed control-plane errors",
                "on_call_driver",
                "incident_commander",
                S::Attention,
                "aureline://evidence/inc-2050-alert",
                F::Recent,
                BW::Blocked,
                "Mitigation needs a managed approval that has not been granted.",
                ScopeClass::SharedTeam,
                1,
                true,
            ),
            tile(
                b,
                4,
                "aureline://review-item/rev-771",
                ObjectKind::ReviewItem,
                "Post-incident review: token refresh",
                "review_lead",
                "review_lead",
                S::Clear,
                "aureline://evidence/rev-771-summary",
                F::Fresh,
                BW::Waived,
                "Follow-up waived by the incident commander until 2026-07-01.",
                ScopeClass::SharedTeam,
                4,
                true,
            ),
        ];
        let views = vec![
            view(
                b,
                "all_by_severity",
                "All, most severe first",
                "Every open item ordered by computed effective-state severity; stale and waived \
                 items never sort as clear.",
                true,
                ScopeClass::SharedTeam,
                "on_call_driver",
                vec![],
                order(
                    OrderKeyClass::EffectiveStateSeverity,
                    true,
                    "Most severe effective state first.",
                ),
            ),
            view(
                b,
                "blocked_and_waived",
                "Blocked and waived",
                "Only items whose blocker/waiver state needs an owner decision.",
                true,
                ScopeClass::SharedTeam,
                "on_call_driver",
                vec![clause(
                    FilterFacetClass::BlockerWaiver,
                    &["blocked", "waived", "waiver_expired"],
                )],
                order(
                    OrderKeyClass::EffectiveStateSeverity,
                    true,
                    "Blocking items above waived items.",
                ),
            ),
        ];
        assemble_board(
            b,
            "Open incidents and post-incident review items, triaged against the canonical incident \
             objects the incident workspace owns.",
            ScopeClass::SharedTeam,
            vec![ShellUi, CliHeadless, IncidentWorkspace, SupportExport, AdminQueue],
            RedactionClass::MetadataSafeDefault,
            "all_by_severity",
            views,
            tiles,
        )
    };

    let support = {
        let b = BoardClass::SupportQueue;
        let tiles = vec![
            tile(
                b,
                1,
                "aureline://support-case/case-7741",
                ObjectKind::SupportCase,
                "Export bundle stuck preparing",
                "support_lead",
                "support_lead",
                S::Attention,
                "aureline://evidence/case-7741-trace",
                F::Recent,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                1,
                true,
            ),
            tile(
                b,
                2,
                "aureline://support-case/case-7799",
                ObjectKind::SupportCase,
                "Redaction policy question",
                "support_triage",
                "support_lead",
                S::Clear,
                "aureline://evidence/case-7799-note",
                F::Fresh,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                3,
                true,
            ),
            tile(
                b,
                3,
                "aureline://support-case/case-7802",
                ObjectKind::SupportCase,
                "Cannot reach managed control plane",
                "support_lead",
                "support_lead",
                S::Attention,
                "aureline://evidence/case-7802-route",
                F::Fresh,
                BW::Blocked,
                "Boundary drift detected; recheck required before a managed reply.",
                ScopeClass::SharedTeam,
                2,
                true,
            ),
        ];
        let views = vec![
            view(
                b,
                "open_oldest_first",
                "Open, oldest first",
                "Every open case ordered by evidence age so the stalest case surfaces first.",
                true,
                ScopeClass::SharedTeam,
                "support_lead",
                vec![],
                order(OrderKeyClass::Freshness, true, "Oldest evidence first."),
            ),
            view(
                b,
                "blocking_only",
                "Blocking only",
                "Cases an owner must unblock, ordered by severity.",
                true,
                ScopeClass::SharedTeam,
                "support_lead",
                vec![clause(
                    FilterFacetClass::BlockerWaiver,
                    &["blocked", "waiver_expired"],
                )],
                order(
                    OrderKeyClass::EffectiveStateSeverity,
                    true,
                    "Most severe first.",
                ),
            ),
        ];
        assemble_board(
            b,
            "Open support cases, triaged against the canonical support-case objects the support \
             center owns.",
            ScopeClass::SharedTeam,
            vec![ShellUi, CliHeadless, SupportExport, IncidentWorkspace],
            RedactionClass::InternalSupportRestricted,
            "open_oldest_first",
            views,
            tiles,
        )
    };

    let admin = {
        let b = BoardClass::AdminApprovals;
        let tiles = vec![
            tile(
                b,
                1,
                "aureline://admin-approval/req-301",
                ObjectKind::AdminApprovalRequest,
                "Policy change: telemetry retention",
                "org_admin",
                "org_admin",
                S::Attention,
                "aureline://evidence/req-301-diff",
                F::Fresh,
                BW::None,
                "",
                ScopeClass::ManagedOrg,
                1,
                true,
            ),
            tile(
                b,
                2,
                "aureline://admin-approval/req-318",
                ObjectKind::AdminApprovalRequest,
                "Provider credential rotation",
                "security_admin",
                "security_admin",
                S::Clear,
                "aureline://evidence/req-318-plan",
                F::VeryStale,
                BW::None,
                "",
                ScopeClass::ManagedOrg,
                3,
                true,
            ),
            tile(
                b,
                3,
                "aureline://admin-approval/req-322",
                ObjectKind::AdminApprovalRequest,
                "Tenant migration sign-off",
                "org_admin",
                "org_admin",
                S::Attention,
                "aureline://evidence/req-322-boundary",
                F::Fresh,
                BW::Blocked,
                "Awaiting a boundary recheck after the tenant migration.",
                ScopeClass::ManagedOrg,
                2,
                true,
            ),
        ];
        let views = vec![
            view(
                b,
                "pending_by_severity",
                "Pending, most severe first",
                "Every pending request ordered by effective-state severity; a very-stale request \
                 never sorts as clear.",
                true,
                ScopeClass::ManagedOrg,
                "org_admin",
                vec![],
                order(
                    OrderKeyClass::EffectiveStateSeverity,
                    true,
                    "Most severe effective state first.",
                ),
            ),
            view(
                b,
                "mine_org_admin",
                "Owned by org admin",
                "Requests this owner holds the decision right for, in explicit rank order.",
                false,
                ScopeClass::LocalPrivate,
                "org_admin",
                vec![clause(FilterFacetClass::Owner, &["org_admin"])],
                order(
                    OrderKeyClass::ExplicitRank,
                    false,
                    "Operator's explicit rank order.",
                ),
            ),
        ];
        assemble_board(
            b,
            "Pending governance approval requests, triaged against the canonical admin objects the \
             admin console owns.",
            ScopeClass::ManagedOrg,
            vec![ShellUi, CliHeadless, AdminQueue, SupportExport],
            RedactionClass::OperatorOnlyRestricted,
            "pending_by_severity",
            views,
            tiles,
        )
    };

    let release = {
        let b = BoardClass::ReleaseReadiness;
        let tiles = vec![
            tile(
                b,
                1,
                "aureline://release-gate/gate-evidence",
                ObjectKind::ReleaseGate,
                "Release evidence complete",
                "release_owner",
                "release_owner",
                S::Clear,
                "aureline://evidence/gate-evidence-bundle",
                F::Fresh,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                3,
                true,
            ),
            tile(
                b,
                2,
                "aureline://release-gate/gate-perf",
                ObjectKind::ReleaseGate,
                "Performance budget",
                "release_owner",
                "release_owner",
                S::Clear,
                "aureline://evidence/gate-perf-run",
                F::Stale,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                2,
                true,
            ),
            tile(
                b,
                3,
                "aureline://release-gate/gate-license",
                ObjectKind::ReleaseGate,
                "License scan",
                "release_owner",
                "security_admin",
                S::Clear,
                "aureline://evidence/gate-license-scan",
                F::Fresh,
                BW::Waived,
                "Finding waived by security until 2026-07-15.",
                ScopeClass::SharedTeam,
                1,
                true,
            ),
            tile(
                b,
                4,
                "aureline://service-health/svc-build-farm",
                ObjectKind::ServiceHealthRecord,
                "Build farm",
                "platform_oncall",
                "platform_oncall",
                S::Attention,
                "aureline://evidence/svc-build-farm-card",
                F::Recent,
                BW::None,
                "",
                ScopeClass::SharedTeam,
                4,
                true,
            ),
        ];
        let views = vec![
            view(
                b,
                "readiness_overview",
                "Readiness overview",
                "Every gate and dependency ordered by effective-state severity; a stale or waived \
                 gate never shows as a clear green tile.",
                true,
                ScopeClass::SharedTeam,
                "release_owner",
                vec![],
                order(
                    OrderKeyClass::EffectiveStateSeverity,
                    true,
                    "Most severe effective state first.",
                ),
            ),
            view(
                b,
                "needs_attention",
                "Needs attention",
                "Only gates and dependencies that are not confirmed clear.",
                true,
                ScopeClass::SharedTeam,
                "release_owner",
                vec![clause(
                    FilterFacetClass::State,
                    &["unconfirmed", "attention", "blocked"],
                )],
                order(OrderKeyClass::Freshness, true, "Oldest evidence first."),
            ),
        ];
        assemble_board(
            b,
            "Release gates and the services they depend on, summarized against the canonical \
             release and service-health objects the release surfaces own.",
            ScopeClass::SharedTeam,
            vec![ShellUi, CliHeadless, ReleaseEvidence, SupportExport],
            RedactionClass::MetadataSafeDefault,
            "readiness_overview",
            views,
            tiles,
        )
    };

    vec![incident, support, admin, release]
}

// ---------------------------------------------------------------------------
// Invariants.
// ---------------------------------------------------------------------------

fn invariant(id: &str, statement: &str, holds: bool) -> BoardInvariant {
    BoardInvariant {
        invariant_id: id.to_owned(),
        statement: statement.to_owned(),
        holds,
    }
}

fn compute_invariants(boards: &[OverviewBoard]) -> Vec<BoardInvariant> {
    let matrix = crate::m5_operator_surfaces::operator_surface_matrix();
    let facets: Vec<FilterFacetClass> = FilterFacetClass::ALL.to_vec();
    let mut out = Vec::new();

    // Boards summarize canonical objects, not dashboard-only ids.
    out.push(invariant(
        "operator_boards.canonical_object_identity",
        "Every tile carries a canonical aureline:// object handle and routes open-detail to that \
         exact handle, so boards never invent a separate identity layer for the objects they \
         summarize.",
        boards.iter().all(|b| {
            b.tiles.iter().all(|t| {
                t.object_ref.starts_with("aureline://") && t.open_detail_ref == t.object_ref
            })
        }),
    ));

    // Boards bind a real matrix surface family.
    out.push(invariant(
        "operator_boards.surface_binding",
        "Every board binds a surface family that exists in the operator-surface matrix, by the \
         matrix's own surface id, rather than cloning a per-surface model.",
        boards.iter().all(|b| {
            b.surface == b.board.surface()
                && b.surface_id == b.surface.surface_id()
                && matrix.surface(b.surface).is_some()
        }),
    ));

    // No silent green: effective state is the computed downgrade.
    out.push(invariant(
        "operator_boards.no_silent_green",
        "Every tile's effective state equals the computed no-silent-green state, so a stale or \
         waived tile can never be frozen as a clear green tile.",
        boards.iter().all(|b| {
            b.tiles.iter().all(|t| {
                t.effective_state
                    == compute_effective_state(t.displayed_state, t.freshness, t.blocker_waiver)
            })
        }),
    ));

    // Owner and blocker/waiver state stay first-class.
    out.push(invariant(
        "operator_boards.owner_blocker_visible",
        "Every tile names an owner and decision right, and any blocked or waived tile carries a \
         visible reason rather than hiding it behind hover-only chrome.",
        boards.iter().all(|b| {
            b.tiles.iter().all(|t| {
                !t.owner.is_empty()
                    && !t.decision_right.is_empty()
                    && (!t.blocker_waiver.requires_reason() || !t.blocker_reason.is_empty())
            })
        }),
    ));

    // Saved views and the default-view contract.
    out.push(invariant(
        "operator_boards.saved_views_present",
        "Every board offers at least one saved view, its default view resolves to one, and every \
         view names its order with a stated reason.",
        boards.iter().all(|b| {
            !b.saved_views.is_empty()
                && b.saved_views.iter().any(|v| v.token == b.default_view)
                && b.saved_views.iter().all(|v| !v.order.reason.is_empty())
        }),
    ));

    // Shared filter vocabulary.
    out.push(invariant(
        "operator_boards.shared_filter_vocabulary",
        "Every filter clause references a facet from the shared vocabulary, and every value on a \
         closed-vocabulary facet is one of that facet's allowed tokens.",
        boards.iter().all(|b| {
            b.saved_views.iter().all(|v| {
                v.filters.iter().all(|c| {
                    facets.contains(&c.facet)
                        && (!c.facet.closed_vocabulary()
                            || c.include_tokens
                                .iter()
                                .all(|val| c.facet.allowed_tokens().contains(val)))
                })
            })
        }),
    ));

    // Open-detail parity.
    out.push(invariant(
        "operator_boards.open_detail_parity",
        "Every board offers a canonical open-detail action and every tile's open-detail route is \
         its canonical object handle.",
        boards.iter().all(|b| {
            b.actions
                .iter()
                .any(|a| a.action == BoardActionClass::OpenDetail && a.routes_to_canonical_object)
                && b.tiles.iter().all(|t| t.open_detail_ref == t.object_ref)
        }),
    ));

    // Export parity: the frozen export equals re-applying the default view.
    out.push(invariant(
        "operator_boards.export_parity",
        "Every board's frozen export equals re-applying its default saved view, so the export \
         preserves the exact filters, order, and per-tile state.",
        boards
            .iter()
            .all(|b| export_board_view(b, &b.default_view).is_some_and(|e| e == b.export)),
    ));

    // Export preserves scope/freshness/ownership/blocker truth as a labeled snapshot.
    out.push(invariant(
        "operator_boards.export_preserves_truth",
        "Every export carries the applied view's scope, owner, filters, and order, labels itself a \
         snapshot, and preserves each row's effective state, freshness, ownership, and \
         blocker/waiver reason.",
        boards.iter().all(|b| {
            let e = &b.export;
            let view = b.saved_views.iter().find(|v| v.token == b.default_view);
            view.is_some_and(|v| {
                e.scope == v.scope
                    && e.view_owner == v.owner
                    && e.filters == v.filters
                    && e.order == v.order
                    && e.live_vs_snapshot == LiveSnapshotClass::SnapshotOnly
                    && e.row_count as usize == e.rows.len()
                    && e.rows.iter().all(|r| {
                        b.tiles.iter().any(|t| {
                            t.tile_id == r.tile_id
                                && t.object_ref == r.object_ref
                                && t.effective_state == r.effective_state
                                && t.freshness == r.freshness
                                && t.blocker_waiver == r.blocker_waiver
                                && t.blocker_reason == r.blocker_reason
                        })
                    })
            })
        }),
    ));

    // The first real boards are all present.
    out.push(invariant(
        "operator_boards.first_real_boards_present",
        "The incident-response, support-queue, admin-approvals, and release-readiness boards are \
         all present, proving the shared contract across real boards.",
        BoardClass::ALL
            .iter()
            .all(|class| boards.iter().filter(|b| b.board == *class).count() == 1),
    ));

    // Stable ids unique.
    out.push(invariant(
        "operator_boards.stable_ids_unique",
        "Board ids, saved-view ids, and tile ids are each defined once and unique across the set.",
        all_unique(boards.iter().map(|b| b.board_id.as_str()))
            && all_unique(
                boards
                    .iter()
                    .flat_map(|b| b.saved_views.iter().map(|v| v.view_id.as_str())),
            )
            && all_unique(
                boards
                    .iter()
                    .flat_map(|b| b.tiles.iter().map(|t| t.tile_id.as_str())),
            ),
    ));

    out
}

// ---------------------------------------------------------------------------
// Human-readable projection.
// ---------------------------------------------------------------------------

/// Renders the board set as human-readable lines for CLI/headless and support.
pub fn operator_board_lines(set: &OverviewBoardSet) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Operator overview boards — {} ({})",
        set.set_id, set.as_of
    ));
    lines.push(set.summary.clone());
    lines.push(format!(
        "Boards: {}  Facets: {}  Object kinds: {}  bound to {}",
        set.boards.len(),
        set.filter_facets.len(),
        set.object_kinds.len(),
        set.matrix_ref,
    ));

    for board in &set.boards {
        lines.push(format!(
            "Board {} [{}] surface={} scope={} redaction={:?}",
            board.board.as_str(),
            board.board_id,
            board.surface.as_str(),
            scope_token(board.scope),
            board.default_redaction,
        ));
        lines.push(format!("  {}", board.summary));
        lines.push(format!(
            "  default view: {} ({} saved views)",
            board.default_view,
            board.saved_views.len()
        ));
        lines.push("  tiles:".to_owned());
        for t in &board.tiles {
            lines.push(format!(
                "    - {} [{}] owner={} displayed={} freshness={} blocker={} -> effective={}",
                t.title,
                t.object_ref,
                t.owner,
                t.displayed_state.as_str(),
                t.freshness.as_str(),
                t.blocker_waiver.as_str(),
                t.effective_state.as_str(),
            ));
        }
        lines.push(format!(
            "  export {} via '{}': {} rows (snapshot)",
            board.export.export_id, board.export.applied_view_token, board.export.row_count,
        ));
    }

    lines.push("Invariants:".to_owned());
    for i in &set.invariants {
        lines.push(format!(
            "  - [{}] {}",
            if i.holds { "ok" } else { "FAIL" },
            i.invariant_id
        ));
    }

    lines
}
