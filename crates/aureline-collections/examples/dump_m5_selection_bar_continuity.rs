//! Conformance dump for the M5 selection-bar continuity packet — selection bars,
//! range-anchor identity, stale-query-snapshot guards, and hidden-selected-count
//! continuity across filtered, sorted, and streaming dense collections.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::implement_selection_bars_range_anchor_and_stale_snapshot_guards::*;
use aureline_collections::{CollectionViewKind, DenseCollectionSurface};

const PACKET_ID: &str = "m5-selection-bar-continuity:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn item(id: &str, label: &str, in_current_filter: bool) -> StableSelectionItem {
    StableSelectionItem {
        stable_item_id: id.to_owned(),
        review_label: label.to_owned(),
        in_current_filter,
    }
}

fn counts(
    total: u64,
    visible: u64,
    outside_filter: u64,
    prior_snapshot: u64,
    blocked: u64,
) -> SelectionBarCounts {
    SelectionBarCounts {
        selected_total: total,
        selected_visible: visible,
        selected_outside_filter: outside_filter,
        selected_from_prior_snapshot: prior_snapshot,
        selected_blocked: blocked,
    }
}

#[allow(clippy::too_many_arguments)]
fn bar(
    bar_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    data_mode: CollectionDataMode,
    label: &str,
    membership: SelectionMembership,
    range_anchor: Option<RangeAnchor>,
    counts: SelectionBarCounts,
    snapshot_guard: StaleQuerySnapshotGuard,
    accessibility_summary: &str,
) -> SelectionBar {
    SelectionBar {
        bar_id: bar_id.to_owned(),
        surface,
        view_kind,
        data_mode,
        label_summary: label.to_owned(),
        membership,
        range_anchor,
        counts,
        snapshot_guard,
        survives_sort_filter_virtualization: true,
        accessibility_summary: accessibility_summary.to_owned(),
        evidence_refs: refs(&[&format!("evidence:bar:{bar_id}")]),
    }
}

fn bars() -> Vec<SelectionBar> {
    vec![
        // Pipeline run list: static complete dataset, plain identity membership.
        bar(
            "bar:pipeline-run-list:0001",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            CollectionDataMode::StaticComplete,
            "Pipeline run list selection over a complete, fresh dataset",
            SelectionMembership {
                basis: SelectionMembershipBasis::StableIdentitySet,
                by_stable_identity: true,
                members: vec![
                    item("pipeline:run:1", "Run 1", true),
                    item("pipeline:run:2", "Run 2", true),
                    item("pipeline:run:3", "Run 3", true),
                ],
                query_snapshot_id_ref: None,
            },
            None,
            counts(3, 3, 0, 0, 0),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:pipeline:v12".to_owned(),
                current_dataset_identity: "dataset:pipeline:v12".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::Unchanged,
                guard_outcome: StaleGuardOutcome::ProceedFresh,
                broad_action_cannot_bypass_preview: true,
                guidance_label: None,
            },
            "3 runs selected; all visible; dataset unchanged.",
        ),
        // Review queue: filtered/sorted with hidden-selected members outside the
        // current filter.
        bar(
            "bar:review-queue:0001",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            CollectionDataMode::FilteredSorted,
            "Review queue selection with members outside the active filter",
            SelectionMembership {
                basis: SelectionMembershipBasis::StableIdentitySet,
                by_stable_identity: true,
                members: vec![
                    item("review:item:1", "Review item 1", true),
                    item("review:item:2", "Review item 2", true),
                    item("review:item:hidden", "Review item now filtered out", false),
                ],
                query_snapshot_id_ref: None,
            },
            None,
            counts(12, 9, 3, 0, 1),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:review:v4".to_owned(),
                current_dataset_identity: "dataset:review:v4".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::ReorderedOnly,
                guard_outcome: StaleGuardOutcome::ProceedFresh,
                broad_action_cannot_bypass_preview: true,
                guidance_label: None,
            },
            "12 selected; 3 are outside the current filter; 1 blocked.",
        ),
        // Incident list: streaming live, but selection holds by stable identity and
        // a reorder does not invalidate it.
        bar(
            "bar:incident-list:0001",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            CollectionDataMode::Streaming,
            "Incident list selection that survives live streaming reorders",
            SelectionMembership {
                basis: SelectionMembershipBasis::StableIdentitySet,
                by_stable_identity: true,
                members: vec![
                    item("incident:1", "Incident 1", true),
                    item("incident:2", "Incident 2", true),
                ],
                query_snapshot_id_ref: None,
            },
            None,
            counts(2, 2, 0, 0, 0),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:incident:v8".to_owned(),
                current_dataset_identity: "dataset:incident:v8".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::ReorderedOnly,
                guard_outcome: StaleGuardOutcome::ProceedFresh,
                broad_action_cannot_bypass_preview: true,
                guidance_label: None,
            },
            "2 incidents selected; selection holds by id as new rows stream in.",
        ),
        // Graph tree: virtualized, holding a shift-range anchor by stable identity.
        bar(
            "bar:graph-list:0001",
            DenseCollectionSurface::GraphList,
            CollectionViewKind::Tree,
            CollectionDataMode::Virtualized,
            "Reference graph tree shift-range selection anchored by stable identity",
            SelectionMembership {
                basis: SelectionMembershipBasis::RangeAnchorExpansion,
                by_stable_identity: true,
                members: vec![
                    item("graph:node:a", "Node A (anchor)", true),
                    item("graph:node:b", "Node B", true),
                    item("graph:node:c", "Node C (focus)", true),
                ],
                query_snapshot_id_ref: None,
            },
            Some(RangeAnchor {
                anchor_item_id: "graph:node:a".to_owned(),
                focus_item_id: "graph:node:c".to_owned(),
                anchored_by_stable_identity: true,
                anchor_still_present: true,
                visible_traversal_order: true,
                reresolution_label: None,
            }),
            counts(3, 3, 0, 0, 0),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:graph:v2".to_owned(),
                current_dataset_identity: "dataset:graph:v2".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::Unchanged,
                guard_outcome: StaleGuardOutcome::ProceedFresh,
                broad_action_cannot_bypass_preview: true,
                guidance_label: None,
            },
            "3 nodes selected in visible range from anchor Node A to focus Node C.",
        ),
        // Marketplace table: query-snapshot-backed, paginated, and stale — the
        // guard forces re-review before a broad action.
        bar(
            "bar:marketplace-results:0001",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            CollectionDataMode::Paginated,
            "Marketplace results selection from a prior snapshot that went stale",
            SelectionMembership {
                basis: SelectionMembershipBasis::QuerySnapshot,
                by_stable_identity: true,
                members: vec![
                    item("market:item:1", "Extension 1", true),
                    item("market:item:offpage", "Extension off the current page", false),
                ],
                query_snapshot_id_ref: Some("query-snapshot:marketplace:v3".to_owned()),
            },
            None,
            counts(240, 50, 190, 240, 6),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:marketplace:v3".to_owned(),
                current_dataset_identity: "dataset:marketplace:v5".to_owned(),
                query_snapshot_id_ref: Some("query-snapshot:marketplace:v3".to_owned()),
                dataset_identity_change: DatasetIdentityChange::RowsAddedOrRemoved,
                guard_outcome: StaleGuardOutcome::RequireReopenReview,
                broad_action_cannot_bypass_preview: true,
                guidance_label: Some(
                    "the catalog changed since you selected; reopen to re-review the 240 matches"
                        .to_owned(),
                ),
            },
            "240 selected from a prior snapshot; 190 outside the current page; reopen required before install.",
        ),
        // Provider/admin table: virtualized, with a departed range anchor that
        // re-resolves, and a provider epoch change downgraded to visible-only.
        bar(
            "bar:provider-admin-table:0001",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            CollectionDataMode::Virtualized,
            "Provider/admin table selection downgraded after a provider epoch change",
            SelectionMembership {
                basis: SelectionMembershipBasis::RangeAnchorExpansion,
                by_stable_identity: true,
                members: vec![
                    item("admin:row:1", "Admin row 1", true),
                    item("admin:row:gone", "Admin row left the window", false),
                ],
                query_snapshot_id_ref: None,
            },
            Some(RangeAnchor {
                anchor_item_id: "admin:row:gone".to_owned(),
                focus_item_id: "admin:row:1".to_owned(),
                anchored_by_stable_identity: true,
                anchor_still_present: false,
                visible_traversal_order: true,
                reresolution_label: Some(
                    "the anchor row left the window; the range re-anchors on the next visible row"
                        .to_owned(),
                ),
            }),
            counts(18, 12, 6, 0, 0),
            StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:admin:epoch-7".to_owned(),
                current_dataset_identity: "dataset:admin:epoch-8".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::ProviderEpochChanged,
                guard_outcome: StaleGuardOutcome::DowngradeToVisibleOnly,
                broad_action_cannot_bypass_preview: true,
                guidance_label: Some(
                    "provider data advanced an epoch; the action is narrowed to the 12 visible rows"
                        .to_owned(),
                ),
            },
            "18 selected; 6 outside the current filter; downgraded to visible rows after a provider change.",
        ),
    ]
}

fn guardrails() -> SelectionBarGuardrails {
    SelectionBarGuardrails {
        selection_survives_sort_filter_virtualization: true,
        hidden_selected_count_always_visible: true,
        stale_snapshot_triggers_review_or_downgrade: true,
        broad_action_cannot_bypass_preview: true,
        range_anchor_by_stable_identity: true,
    }
}

fn consumer_projection() -> SelectionBarConsumerProjection {
    SelectionBarConsumerProjection {
        product_renders_selection_bar: true,
        diagnostics_reconstructs_selection_truth: true,
        support_export_reuses_records: true,
        docs_help_reuses_vocabulary: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        SELECTION_BAR_CONTINUITY_SCHEMA_REF,
        SELECTION_BAR_CONTINUITY_DOC_REF,
        SELECTION_BAR_CONTINUITY_ARTIFACT_REF,
        "schemas/collections/selection-scope.schema.json",
        "schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json",
    ])
}

fn packet() -> SelectionBarContinuityPacket {
    SelectionBarContinuityPacket::new(SelectionBarContinuityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Selection Bars And Stale-Query-Snapshot Guards".to_owned(),
        bars: bars(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "support".to_owned());
    let packet = packet();

    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "packet must validate: {violations:?}"
    );

    if which == "summary" {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
