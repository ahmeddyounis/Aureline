//! Notebook-aware search, outline, breadcrumbs, and cell-target navigation.
//!
//! This module materializes the typed records that keep notebook search,
//! outline, breadcrumb, and deep-link navigation honest about cell identity,
//! scope, and degraded state. Every record anchors to stable cell IDs from
//! the canonical document model so search results, outline entries, breadcrumb
//! segments, and navigation targets survive reorder, diff, and comment
//! anchoring without requiring a selected kernel.
//!
//! The module exposes:
//!
//! - the [`NotebookSearchQuery`] record that carries search scope, match class,
//!   result cell refs, and truncation state so the chrome never presents a
//!   scoped search as a full-document search;
//! - the [`NotebookOutlineItem`] record that carries outline hierarchy —
//!   headings and cell boundaries — anchored to durable cell IDs so the outline
//!   survives notebook edits;
//! - the [`NotebookBreadcrumb`] record that carries a breadcrumb segment with
//!   class, label, and target ref so the user always knows where they are in
//!   the notebook hierarchy;
//! - the [`NotebookCellTarget`] record that carries a precise navigation target
//!   (cell id, cell index, output index, heading anchor, or search match) so
//!   deep links and goto actions never silently fall back to a different cell;
//! - the [`NotebookSearchOutlineNavigationPacket`] checked-in artifact that
//!   downstream docs, help, support, and CI surfaces ingest instead of cloning
//!   status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookSearchQuery`] payloads.
pub const NOTEBOOK_SEARCH_QUERY_RECORD_KIND: &str = "notebook_search_query";

/// Stable record-kind tag for serialized [`NotebookOutlineItem`] payloads.
pub const NOTEBOOK_OUTLINE_ITEM_RECORD_KIND: &str = "notebook_outline_item";

/// Stable record-kind tag for serialized [`NotebookBreadcrumb`] payloads.
pub const NOTEBOOK_BREADCRUMB_RECORD_KIND: &str = "notebook_breadcrumb";

/// Stable record-kind tag for serialized [`NotebookCellTarget`] payloads.
pub const NOTEBOOK_CELL_TARGET_RECORD_KIND: &str = "notebook_cell_target";

/// Stable record-kind tag for the checked-in
/// [`NotebookSearchOutlineNavigationPacket`].
pub const NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND: &str =
    "notebook_search_outline_navigation_packet";

/// Repo-relative path to the checked-in search/outline/navigation packet JSON.
pub const NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_PATH: &str =
    "artifacts/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation.json";

/// Embedded checked-in search/outline/navigation packet JSON.
pub const NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/add_notebook_aware_search_outline_breadcrumbs_and_cell_target_navigation.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Where the search is scoped. Pinned so the chrome never presents a
    /// scoped search as a full-document search.
    NotebookSearchScopeClass {
        CurrentCell => "current_cell",
        AllCells => "all_cells",
        SelectedCells => "selected_cells",
        CellOutputsOnly => "cell_outputs_only",
        MarkdownCellsOnly => "markdown_cells_only",
        CodeCellsOnly => "code_cells_only",
    }
);

closed_vocab!(
    /// How query text is matched. Pinned so the user knows whether results
    /// are exact, fuzzy, regex, or semantic.
    NotebookSearchMatchClass {
        Exact => "exact",
        Fuzzy => "fuzzy",
        Regex => "regex",
        Semantic => "semantic",
    }
);

closed_vocab!(
    /// What kind of structure the outline item represents. Headings are
    /// extracted from markdown cell content; cell boundaries are structural.
    NotebookOutlineItemClass {
        Heading1 => "heading_1",
        Heading2 => "heading_2",
        Heading3 => "heading_3",
        Heading4 => "heading_4",
        Heading5 => "heading_5",
        Heading6 => "heading_6",
        MarkdownCellBoundary => "markdown_cell_boundary",
        CodeCellBoundary => "code_cell_boundary",
        RawCellBoundary => "raw_cell_boundary",
    }
);

closed_vocab!(
    /// What kind of breadcrumb segment this is. Pinned so the chrome never
    /// invents ad hoc breadcrumb levels.
    NotebookBreadcrumbClass {
        DocumentRoot => "document_root",
        SectionHeading => "section_heading",
        CellBoundary => "cell_boundary",
        OutputBoundary => "output_boundary",
        SearchResultSet => "search_result_set",
    }
);

closed_vocab!(
    /// What kind of navigation target this is. Pinned so deep links and goto
    /// actions never silently fall back to a different cell.
    NotebookCellTargetClass {
        CellIdAnchor => "cell_id_anchor",
        CellIndexAnchor => "cell_index_anchor",
        OutputIndexAnchor => "output_index_anchor",
        HeadingAnchor => "heading_anchor",
        SearchMatchAnchor => "search_match_anchor",
    }
);

closed_vocab!(
    /// How the view should scroll when navigating to a target. Pinned so the
    /// chrome behaves predictably.
    NotebookScrollBehaviorClass {
        CenterInView => "center_in_view",
        ScrollToTop => "scroll_to_top",
        ScrollToNearest => "scroll_to_nearest",
        NoScroll => "no_scroll",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchOutlineNavigationFinding {
    /// Stable check id (e.g. `notebook_search_query.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, query id, item id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl SearchOutlineNavigationFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookSearchQuery`].
pub type NotebookSearchQueryFinding = SearchOutlineNavigationFinding;

/// Typed validation finding for a [`NotebookOutlineItem`].
pub type NotebookOutlineItemFinding = SearchOutlineNavigationFinding;

/// Typed validation finding for a [`NotebookBreadcrumb`].
pub type NotebookBreadcrumbFinding = SearchOutlineNavigationFinding;

/// Typed validation finding for a [`NotebookCellTarget`].
pub type NotebookCellTargetFinding = SearchOutlineNavigationFinding;

/// Typed validation finding for a [`NotebookSearchOutlineNavigationPacket`].
pub type NotebookSearchOutlineNavigationPacketFinding = SearchOutlineNavigationFinding;

/// Notebook search query record. Carries scope, match class, result refs, and
/// truncation state so the chrome renders search results honestly.
///
/// The `query_label` is export-safe and does not carry raw user query text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSearchQuery {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_search_outline_navigation_schema_version: u32,
    /// Stable opaque search-query id.
    pub search_query_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Search scope class.
    pub search_scope_class: NotebookSearchScopeClass,
    /// Export-safe search query label. Raw query text MUST NOT appear here.
    pub query_label: String,
    /// Match class.
    pub match_class: NotebookSearchMatchClass,
    /// Opaque refs to cells that matched the query.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub result_cell_id_refs: Vec<String>,
    /// Number of results currently visible (after truncation).
    pub result_count_visible: u32,
    /// Total number of results before truncation.
    pub result_count_total: u32,
    /// Whether results were truncated.
    pub truncated: bool,
    /// Whether a kernel is required for this match class. Semantic search may
    /// require an active kernel or local model; the chrome must show a
    /// degraded label when the requirement is unmet.
    pub kernel_required_for_match_class: bool,
    /// Whether the query is degraded because no kernel is available.
    pub degraded_no_kernel: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookSearchQuery {
    /// Returns typed truth-rule findings; an empty vector means the query is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookSearchQueryFinding> {
        let mut findings = Vec::new();
        let subject = self.search_query_id.as_str();

        if self.record_kind != NOTEBOOK_SEARCH_QUERY_RECORD_KIND {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SEARCH_QUERY_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_search_outline_navigation_schema_version
            != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION
        {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION}, found {}",
                    self.notebook_search_outline_navigation_schema_version
                ),
            ));
        }

        if self.result_count_visible > self.result_count_total {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.visible_exceeds_total",
                subject,
                "result_count_visible must not exceed result_count_total",
            ));
        }

        if self.truncated && self.result_count_visible == self.result_count_total {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.truncated_but_counts_equal",
                subject,
                "truncated=true requires result_count_visible < result_count_total",
            ));
        }

        if !self.truncated && self.result_count_visible != self.result_count_total {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.not_truncated_but_counts_differ",
                subject,
                "truncated=false requires result_count_visible == result_count_total",
            ));
        }

        if self.kernel_required_for_match_class && self.degraded_no_kernel {
            // Degraded label is correct when kernel is required but absent.
        }

        if self.match_class == NotebookSearchMatchClass::Semantic && !self.kernel_required_for_match_class {
            findings.push(NotebookSearchQueryFinding::new(
                "notebook_search_query.semantic_requires_kernel_flag",
                subject,
                "semantic match class should set kernel_required_for_match_class=true",
            ));
        }

        findings
    }
}

/// Notebook outline item record. Represents one entry in the outline / table
/// of contents — a heading or a cell boundary — anchored to a stable cell ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutlineItem {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_search_outline_navigation_schema_version: u32,
    /// Stable opaque outline-item id.
    pub outline_item_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// What kind of structure this item represents.
    pub item_class: NotebookOutlineItemClass,
    /// Opaque ref to the cell this item is anchored to.
    pub cell_id_ref: String,
    /// Heading level (1–6) when item_class is a heading; null otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading_level: Option<u8>,
    /// Export-safe title label for the outline entry. Raw heading text MUST NOT
    /// appear here if it contains sensitive content.
    pub title_label: String,
    /// Opaque refs to child outline items.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub child_item_refs: Vec<String>,
    /// Whether this item is collapsed in the outline view.
    pub collapsed: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookOutlineItem {
    /// Returns typed truth-rule findings; an empty vector means the item is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookOutlineItemFinding> {
        let mut findings = Vec::new();
        let subject = self.outline_item_id.as_str();

        if self.record_kind != NOTEBOOK_OUTLINE_ITEM_RECORD_KIND {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_OUTLINE_ITEM_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_search_outline_navigation_schema_version
            != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION
        {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION}, found {}",
                    self.notebook_search_outline_navigation_schema_version
                ),
            ));
        }

        if self.cell_id_ref.trim().is_empty() {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.cell_id_ref_required",
                subject,
                "cell_id_ref must be non-empty",
            ));
        }

        let is_heading = matches!(
            self.item_class,
            NotebookOutlineItemClass::Heading1
                | NotebookOutlineItemClass::Heading2
                | NotebookOutlineItemClass::Heading3
                | NotebookOutlineItemClass::Heading4
                | NotebookOutlineItemClass::Heading5
                | NotebookOutlineItemClass::Heading6
        );

        if is_heading && self.heading_level.is_none() {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.heading_level_required",
                subject,
                "heading items must carry a heading_level",
            ));
        }

        if !is_heading && self.heading_level.is_some() {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.heading_level_unexpected",
                subject,
                "non-heading items must not carry a heading_level",
            ));
        }

        if let Some(level) = self.heading_level {
            if level < 1 || level > 6 {
                findings.push(NotebookOutlineItemFinding::new(
                    "notebook_outline_item.heading_level_range",
                    subject,
                    "heading_level must be between 1 and 6",
                ));
            }
        }

        if self.title_label.trim().is_empty() {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.title_label_required",
                subject,
                "title_label must be non-empty",
            ));
        }

        // Detect self-reference in child_item_refs.
        if self.child_item_refs.iter().any(|r| r == &self.outline_item_id) {
            findings.push(NotebookOutlineItemFinding::new(
                "notebook_outline_item.self_reference",
                subject,
                "child_item_refs must not contain self",
            ));
        }

        findings
    }
}

/// Notebook breadcrumb segment record. Represents one segment in the
/// breadcrumb trail that shows the user where they are in the notebook
/// hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookBreadcrumb {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_search_outline_navigation_schema_version: u32,
    /// Stable opaque breadcrumb id.
    pub breadcrumb_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Position of this segment in the breadcrumb trail (0 = root).
    pub segment_index: u32,
    /// Breadcrumb class.
    pub breadcrumb_class: NotebookBreadcrumbClass,
    /// Export-safe display label.
    pub label: String,
    /// Opaque ref to the navigation target this segment points to.
    pub target_ref: String,
    /// Whether this segment is the active (current) segment.
    pub active: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookBreadcrumb {
    /// Returns typed truth-rule findings; an empty vector means the breadcrumb
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookBreadcrumbFinding> {
        let mut findings = Vec::new();
        let subject = self.breadcrumb_id.as_str();

        if self.record_kind != NOTEBOOK_BREADCRUMB_RECORD_KIND {
            findings.push(NotebookBreadcrumbFinding::new(
                "notebook_breadcrumb.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_BREADCRUMB_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_search_outline_navigation_schema_version
            != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION
        {
            findings.push(NotebookBreadcrumbFinding::new(
                "notebook_breadcrumb.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION}, found {}",
                    self.notebook_search_outline_navigation_schema_version
                ),
            ));
        }

        if self.label.trim().is_empty() {
            findings.push(NotebookBreadcrumbFinding::new(
                "notebook_breadcrumb.label_required",
                subject,
                "label must be non-empty",
            ));
        }

        if self.target_ref.trim().is_empty() {
            findings.push(NotebookBreadcrumbFinding::new(
                "notebook_breadcrumb.target_ref_required",
                subject,
                "target_ref must be non-empty",
            ));
        }

        // Document root must be at segment_index 0.
        if self.breadcrumb_class == NotebookBreadcrumbClass::DocumentRoot && self.segment_index != 0 {
            findings.push(NotebookBreadcrumbFinding::new(
                "notebook_breadcrumb.document_root_index",
                subject,
                "document_root breadcrumb must be at segment_index 0",
            ));
        }

        findings
    }
}

/// Notebook cell-target navigation record. Represents a precise navigation
/// target within a notebook so deep links and goto actions never silently fall
/// back to a different cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCellTarget {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_search_outline_navigation_schema_version: u32,
    /// Stable opaque cell-target id.
    pub cell_target_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Target class.
    pub target_class: NotebookCellTargetClass,
    /// Opaque ref to the target cell id. Required when target_class is
    /// `cell_id_anchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_id_ref: Option<String>,
    /// Zero-based cell index. Required when target_class is `cell_index_anchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_index: Option<u32>,
    /// Zero-based output index within the cell. Required when target_class is
    /// `output_index_anchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output_index: Option<u32>,
    /// Opaque ref to a heading anchor within a markdown cell. Required when
    /// target_class is `heading_anchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading_anchor_ref: Option<String>,
    /// Opaque ref to a search match. Required when target_class is
    /// `search_match_anchor`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search_match_ref: Option<String>,
    /// How the view should scroll when navigating to this target.
    pub scroll_behavior_class: NotebookScrollBehaviorClass,
    /// Whether the cell should receive focus after navigation.
    pub focus_cell: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCellTarget {
    /// Returns typed truth-rule findings; an empty vector means the target is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCellTargetFinding> {
        let mut findings = Vec::new();
        let subject = self.cell_target_id.as_str();

        if self.record_kind != NOTEBOOK_CELL_TARGET_RECORD_KIND {
            findings.push(NotebookCellTargetFinding::new(
                "notebook_cell_target.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CELL_TARGET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_search_outline_navigation_schema_version
            != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION
        {
            findings.push(NotebookCellTargetFinding::new(
                "notebook_cell_target.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION}, found {}",
                    self.notebook_search_outline_navigation_schema_version
                ),
            ));
        }

        // Target-class / field consistency.
        match self.target_class {
            NotebookCellTargetClass::CellIdAnchor => {
                if self.cell_id_ref.is_none() || self.cell_id_ref.as_ref().unwrap().trim().is_empty()
                {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.cell_id_ref_required",
                        subject,
                        "cell_id_anchor target_class requires a non-empty cell_id_ref",
                    ));
                }
            }
            NotebookCellTargetClass::CellIndexAnchor => {
                if self.cell_index.is_none() {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.cell_index_required",
                        subject,
                        "cell_index_anchor target_class requires a cell_index",
                    ));
                }
            }
            NotebookCellTargetClass::OutputIndexAnchor => {
                if self.cell_id_ref.is_none() || self.cell_id_ref.as_ref().unwrap().trim().is_empty()
                {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.cell_id_ref_required_for_output",
                        subject,
                        "output_index_anchor target_class requires a non-empty cell_id_ref",
                    ));
                }
                if self.output_index.is_none() {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.output_index_required",
                        subject,
                        "output_index_anchor target_class requires an output_index",
                    ));
                }
            }
            NotebookCellTargetClass::HeadingAnchor => {
                if self.heading_anchor_ref.is_none()
                    || self.heading_anchor_ref.as_ref().unwrap().trim().is_empty()
                {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.heading_anchor_ref_required",
                        subject,
                        "heading_anchor target_class requires a non-empty heading_anchor_ref",
                    ));
                }
            }
            NotebookCellTargetClass::SearchMatchAnchor => {
                if self.search_match_ref.is_none()
                    || self.search_match_ref.as_ref().unwrap().trim().is_empty()
                {
                    findings.push(NotebookCellTargetFinding::new(
                        "notebook_cell_target.search_match_ref_required",
                        subject,
                        "search_match_anchor target_class requires a non-empty search_match_ref",
                    ));
                }
            }
        }

        // At least one concrete locator must be present.
        let has_locator = self.cell_id_ref.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false)
            || self.cell_index.is_some()
            || self.output_index.is_some()
            || self.heading_anchor_ref.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false)
            || self.search_match_ref.as_ref().map(|s| !s.trim().is_empty()).unwrap_or(false);

        if !has_locator {
            findings.push(NotebookCellTargetFinding::new(
                "notebook_cell_target.at_least_one_locator",
                subject,
                "at least one locator field must be present",
            ));
        }

        findings
    }
}

/// Checked-in search/outline/navigation packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookSearchOutlineNavigationPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: search scope classes.
    pub search_scope_classes: Vec<NotebookSearchScopeClass>,
    /// Closed vocabulary: search match classes.
    pub search_match_classes: Vec<NotebookSearchMatchClass>,
    /// Closed vocabulary: outline item classes.
    pub outline_item_classes: Vec<NotebookOutlineItemClass>,
    /// Closed vocabulary: breadcrumb classes.
    pub breadcrumb_classes: Vec<NotebookBreadcrumbClass>,
    /// Closed vocabulary: cell target classes.
    pub cell_target_classes: Vec<NotebookCellTargetClass>,
    /// Closed vocabulary: scroll behavior classes.
    pub scroll_behavior_classes: Vec<NotebookScrollBehaviorClass>,
    /// Worked example search queries.
    pub example_search_queries: Vec<NotebookSearchQuery>,
    /// Worked example outline items.
    pub example_outline_items: Vec<NotebookOutlineItem>,
    /// Worked example breadcrumbs.
    pub example_breadcrumbs: Vec<NotebookBreadcrumb>,
    /// Worked example cell targets.
    pub example_cell_targets: Vec<NotebookCellTarget>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookSearchOutlineNavigationPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookSearchOutlineNavigationPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.search_scope_classes.len() != NotebookSearchScopeClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.search_scope_classes_coverage",
                subject,
                "search_scope_classes must list every variant",
            ));
        }
        if self.search_match_classes.len() != NotebookSearchMatchClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.search_match_classes_coverage",
                subject,
                "search_match_classes must list every variant",
            ));
        }
        if self.outline_item_classes.len() != NotebookOutlineItemClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.outline_item_classes_coverage",
                subject,
                "outline_item_classes must list every variant",
            ));
        }
        if self.breadcrumb_classes.len() != NotebookBreadcrumbClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.breadcrumb_classes_coverage",
                subject,
                "breadcrumb_classes must list every variant",
            ));
        }
        if self.cell_target_classes.len() != NotebookCellTargetClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.cell_target_classes_coverage",
                subject,
                "cell_target_classes must list every variant",
            ));
        }
        if self.scroll_behavior_classes.len() != NotebookScrollBehaviorClass::ALL.len() {
            findings.push(NotebookSearchOutlineNavigationPacketFinding::new(
                "notebook_search_outline_navigation_packet.scroll_behavior_classes_coverage",
                subject,
                "scroll_behavior_classes must list every variant",
            ));
        }

        for query in &self.example_search_queries {
            findings.extend(query.validate().into_iter().map(|f| {
                NotebookSearchOutlineNavigationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for item in &self.example_outline_items {
            findings.extend(item.validate().into_iter().map(|f| {
                NotebookSearchOutlineNavigationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for breadcrumb in &self.example_breadcrumbs {
            findings.extend(breadcrumb.validate().into_iter().map(|f| {
                NotebookSearchOutlineNavigationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }
        for target in &self.example_cell_targets {
            findings.extend(target.validate().into_iter().map(|f| {
                NotebookSearchOutlineNavigationPacketFinding::new(
                    &f.check_id,
                    &f.subject_ref,
                    &f.message,
                )
            }));
        }

        findings
    }
}

impl NotebookSearchScopeClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CurrentCell,
        Self::AllCells,
        Self::SelectedCells,
        Self::CellOutputsOnly,
        Self::MarkdownCellsOnly,
        Self::CodeCellsOnly,
    ];
}

impl NotebookSearchMatchClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [Self::Exact, Self::Fuzzy, Self::Regex, Self::Semantic];
}

impl NotebookOutlineItemClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Heading1,
        Self::Heading2,
        Self::Heading3,
        Self::Heading4,
        Self::Heading5,
        Self::Heading6,
        Self::MarkdownCellBoundary,
        Self::CodeCellBoundary,
        Self::RawCellBoundary,
    ];
}

impl NotebookBreadcrumbClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocumentRoot,
        Self::SectionHeading,
        Self::CellBoundary,
        Self::OutputBoundary,
        Self::SearchResultSet,
    ];
}

impl NotebookCellTargetClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CellIdAnchor,
        Self::CellIndexAnchor,
        Self::OutputIndexAnchor,
        Self::HeadingAnchor,
        Self::SearchMatchAnchor,
    ];
}

impl NotebookScrollBehaviorClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CenterInView,
        Self::ScrollToTop,
        Self::ScrollToNearest,
        Self::NoScroll,
    ];
}

/// Parses the checked-in search/outline/navigation packet JSON.
pub fn current_notebook_search_outline_navigation_packet(
) -> Result<NotebookSearchOutlineNavigationPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_JSON)
}

#[cfg(test)]
mod tests;
