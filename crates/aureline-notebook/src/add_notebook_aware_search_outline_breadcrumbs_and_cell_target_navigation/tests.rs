use super::*;

fn sample_search_query() -> NotebookSearchQuery {
    NotebookSearchQuery {
        record_kind: NOTEBOOK_SEARCH_QUERY_RECORD_KIND.to_owned(),
        notebook_search_outline_navigation_schema_version: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION,
        search_query_id: "nb.search.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        search_scope_class: NotebookSearchScopeClass::AllCells,
        query_label: "data frame".to_owned(),
        match_class: NotebookSearchMatchClass::Fuzzy,
        result_cell_id_refs: vec!["nb.cell.01".to_owned(), "nb.cell.03".to_owned()],
        result_count_visible: 2,
        result_count_total: 2,
        truncated: false,
        kernel_required_for_match_class: false,
        degraded_no_kernel: false,
        summary: "Fuzzy search for 'data frame' across all cells.".to_owned(),
    }
}

fn sample_outline_item() -> NotebookOutlineItem {
    NotebookOutlineItem {
        record_kind: NOTEBOOK_OUTLINE_ITEM_RECORD_KIND.to_owned(),
        notebook_search_outline_navigation_schema_version: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION,
        outline_item_id: "nb.outline.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        item_class: NotebookOutlineItemClass::Heading2,
        cell_id_ref: "nb.cell.01".to_owned(),
        heading_level: Some(2),
        title_label: "Data loading".to_owned(),
        child_item_refs: vec!["nb.outline.02".to_owned()],
        collapsed: false,
        summary: "H2 outline item for data loading section.".to_owned(),
    }
}

fn sample_breadcrumb() -> NotebookBreadcrumb {
    NotebookBreadcrumb {
        record_kind: NOTEBOOK_BREADCRUMB_RECORD_KIND.to_owned(),
        notebook_search_outline_navigation_schema_version: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION,
        breadcrumb_id: "nb.breadcrumb.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        segment_index: 1,
        breadcrumb_class: NotebookBreadcrumbClass::SectionHeading,
        label: "Data loading".to_owned(),
        target_ref: "nb.target.01".to_owned(),
        active: false,
        summary: "Breadcrumb segment pointing to data loading section.".to_owned(),
    }
}

fn sample_cell_target() -> NotebookCellTarget {
    NotebookCellTarget {
        record_kind: NOTEBOOK_CELL_TARGET_RECORD_KIND.to_owned(),
        notebook_search_outline_navigation_schema_version: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION,
        cell_target_id: "nb.target.01".to_owned(),
        document_id_ref: "nb.doc.example".to_owned(),
        target_class: NotebookCellTargetClass::CellIdAnchor,
        cell_id_ref: Some("nb.cell.01".to_owned()),
        cell_index: None,
        output_index: None,
        heading_anchor_ref: None,
        search_match_ref: None,
        scroll_behavior_class: NotebookScrollBehaviorClass::CenterInView,
        focus_cell: true,
        summary: "Navigate to cell 01 and center it in view.".to_owned(),
    }
}

#[test]
fn search_query_validates_clean() {
    let q = sample_search_query();
    assert!(
        q.validate().is_empty(),
        "search query should be clean: {:?}",
        q.validate()
    );
}

#[test]
fn outline_item_validates_clean() {
    let o = sample_outline_item();
    assert!(
        o.validate().is_empty(),
        "outline item should be clean: {:?}",
        o.validate()
    );
}

#[test]
fn breadcrumb_validates_clean() {
    let b = sample_breadcrumb();
    assert!(
        b.validate().is_empty(),
        "breadcrumb should be clean: {:?}",
        b.validate()
    );
}

#[test]
fn cell_target_validates_clean() {
    let t = sample_cell_target();
    assert!(
        t.validate().is_empty(),
        "cell target should be clean: {:?}",
        t.validate()
    );
}

#[test]
fn search_query_rejects_visible_exceeds_total() {
    let mut q = sample_search_query();
    q.result_count_visible = 5;
    q.result_count_total = 3;
    let findings = q.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_search_query.visible_exceeds_total"));
}

#[test]
fn search_query_rejects_truncated_with_equal_counts() {
    let mut q = sample_search_query();
    q.truncated = true;
    let findings = q.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_search_query.truncated_but_counts_equal"));
}

#[test]
fn search_query_rejects_not_truncated_with_differing_counts() {
    let mut q = sample_search_query();
    q.result_count_total = 5;
    let findings = q.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_search_query.not_truncated_but_counts_differ"));
}

#[test]
fn search_query_warns_semantic_without_kernel_flag() {
    let mut q = sample_search_query();
    q.match_class = NotebookSearchMatchClass::Semantic;
    q.kernel_required_for_match_class = false;
    let findings = q.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_search_query.semantic_requires_kernel_flag"));
}

#[test]
fn outline_item_rejects_empty_cell_id_ref() {
    let mut o = sample_outline_item();
    o.cell_id_ref = "".to_owned();
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.cell_id_ref_required"));
}

#[test]
fn outline_item_rejects_heading_without_level() {
    let mut o = sample_outline_item();
    o.heading_level = None;
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.heading_level_required"));
}

#[test]
fn outline_item_rejects_non_heading_with_level() {
    let mut o = sample_outline_item();
    o.item_class = NotebookOutlineItemClass::CodeCellBoundary;
    o.heading_level = Some(2);
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.heading_level_unexpected"));
}

#[test]
fn outline_item_rejects_heading_level_out_of_range() {
    let mut o = sample_outline_item();
    o.heading_level = Some(7);
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.heading_level_range"));
}

#[test]
fn outline_item_rejects_empty_title_label() {
    let mut o = sample_outline_item();
    o.title_label = "".to_owned();
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.title_label_required"));
}

#[test]
fn outline_item_rejects_self_reference() {
    let mut o = sample_outline_item();
    o.child_item_refs = vec!["nb.outline.01".to_owned()];
    let findings = o.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_outline_item.self_reference"));
}

#[test]
fn breadcrumb_rejects_empty_label() {
    let mut b = sample_breadcrumb();
    b.label = "".to_owned();
    let findings = b.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_breadcrumb.label_required"));
}

#[test]
fn breadcrumb_rejects_empty_target_ref() {
    let mut b = sample_breadcrumb();
    b.target_ref = "".to_owned();
    let findings = b.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_breadcrumb.target_ref_required"));
}

#[test]
fn breadcrumb_rejects_document_root_not_at_index_zero() {
    let mut b = sample_breadcrumb();
    b.breadcrumb_class = NotebookBreadcrumbClass::DocumentRoot;
    b.segment_index = 1;
    let findings = b.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_breadcrumb.document_root_index"));
}

#[test]
fn cell_target_rejects_cell_id_anchor_without_cell_id() {
    let mut t = sample_cell_target();
    t.cell_id_ref = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.cell_id_ref_required"));
}

#[test]
fn cell_target_rejects_cell_index_anchor_without_index() {
    let mut t = sample_cell_target();
    t.target_class = NotebookCellTargetClass::CellIndexAnchor;
    t.cell_id_ref = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.cell_index_required"));
}

#[test]
fn cell_target_rejects_output_index_anchor_without_output_index() {
    let mut t = sample_cell_target();
    t.target_class = NotebookCellTargetClass::OutputIndexAnchor;
    t.output_index = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.output_index_required"));
}

#[test]
fn cell_target_rejects_heading_anchor_without_heading_ref() {
    let mut t = sample_cell_target();
    t.target_class = NotebookCellTargetClass::HeadingAnchor;
    t.cell_id_ref = None;
    t.heading_anchor_ref = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.heading_anchor_ref_required"));
}

#[test]
fn cell_target_rejects_search_match_anchor_without_match_ref() {
    let mut t = sample_cell_target();
    t.target_class = NotebookCellTargetClass::SearchMatchAnchor;
    t.cell_id_ref = None;
    t.search_match_ref = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.search_match_ref_required"));
}

#[test]
fn cell_target_rejects_no_locator() {
    let mut t = sample_cell_target();
    t.cell_id_ref = None;
    t.cell_index = None;
    t.output_index = None;
    t.heading_anchor_ref = None;
    t.search_match_ref = None;
    let findings = t.validate();
    assert!(findings
        .iter()
        .any(|f| f.check_id == "notebook_cell_target.at_least_one_locator"));
}

#[test]
fn closed_vocabularies_expose_stable_tokens() {
    assert_eq!(NotebookSearchScopeClass::AllCells.as_str(), "all_cells");
    assert_eq!(NotebookSearchMatchClass::Exact.as_str(), "exact");
    assert_eq!(NotebookOutlineItemClass::Heading3.as_str(), "heading_3");
    assert_eq!(NotebookBreadcrumbClass::CellBoundary.as_str(), "cell_boundary");
    assert_eq!(NotebookCellTargetClass::HeadingAnchor.as_str(), "heading_anchor");
    assert_eq!(
        NotebookScrollBehaviorClass::ScrollToNearest.as_str(),
        "scroll_to_nearest"
    );
}

#[test]
fn packet_validates_clean() {
    let packet = NotebookSearchOutlineNavigationPacket {
        schema_version: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION,
        record_kind: NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND.to_owned(),
        packet_id: "nb.son.packet.01".to_owned(),
        as_of: "2026-06-09T00:00:00Z".to_owned(),
        search_scope_classes: NotebookSearchScopeClass::ALL.to_vec(),
        search_match_classes: NotebookSearchMatchClass::ALL.to_vec(),
        outline_item_classes: NotebookOutlineItemClass::ALL.to_vec(),
        breadcrumb_classes: NotebookBreadcrumbClass::ALL.to_vec(),
        cell_target_classes: NotebookCellTargetClass::ALL.to_vec(),
        scroll_behavior_classes: NotebookScrollBehaviorClass::ALL.to_vec(),
        example_search_queries: vec![sample_search_query()],
        example_outline_items: vec![sample_outline_item()],
        example_breadcrumbs: vec![sample_breadcrumb()],
        example_cell_targets: vec![sample_cell_target()],
        summary: "Search/outline/navigation packet v1.".to_owned(),
    };
    assert!(
        packet.validate().is_empty(),
        "packet should be clean: {:?}",
        packet.validate()
    );
}

#[test]
fn embedded_packet_parses() {
    let packet = current_notebook_search_outline_navigation_packet()
        .expect("embedded packet must parse");
    assert_eq!(
        packet.schema_version,
        NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_SCHEMA_VERSION
    );
    assert_eq!(
        packet.record_kind,
        NOTEBOOK_SEARCH_OUTLINE_NAVIGATION_PACKET_RECORD_KIND
    );
}
