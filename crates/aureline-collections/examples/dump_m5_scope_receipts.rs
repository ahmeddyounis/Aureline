//! Conformance dump for the M5 scope-receipt and saved-query deep-link snapshot
//! packet — scope receipts for export / copy / rerun / suppress / install /
//! update / delete flows that name selected items versus all matching items, plus
//! saved-query deep-link snapshots that preserve current-versus-captured scope on
//! reopen across pipeline, review, incident, marketplace, provider/admin,
//! query-backed, and activity surfaces.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::emit_scope_receipts_and_saved_query_deep_link_snapshot_truth::*;
use aureline_collections::{
    BatchActionKind, CollectionViewKind, DenseCollectionSurface, ExecutionOriginClass,
};

const PACKET_ID: &str = "m5-scope-receipt:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn receipts() -> Vec<ScopeReceipt> {
    vec![
        // Pipeline run list: re-run the explicitly selected failed runs. The receipt
        // names the selected scope (8 of 25 matching) so a rerun never silently
        // expands to all matching runs.
        ScopeReceipt {
            receipt_id: "receipt:pipeline-rerun:0001".to_owned(),
            surface: DenseCollectionSurface::PipelineRunList,
            view_kind: CollectionViewKind::List,
            action_kind: BatchActionKind::Rerun,
            scope_class: ScopeReceiptClass::SelectedItems,
            execution_origin: ExecutionOriginClass::LocalClient,
            selection_id_ref: "selection:pipeline:rerun:0001".to_owned(),
            query_snapshot_id_ref: None,
            counts: ScopeReceiptCounts {
                selected_count: 8,
                visible_count: 10,
                loaded_count: 10,
                matching_count: Some(25),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 8,
                omitted_count: 0,
            },
            scope_label: "Re-ran the 8 selected runs, not all 25 matching the failed filter."
                .to_owned(),
            expansion_was_explicit: false,
            mutates_state: true,
            provider_backed: false,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:pipeline-rerun:0001"]),
        },
        // Review queue: update priority on every loaded reviewed item. Provider
        // completes the write; the receipt names the loaded scope (30 of 120).
        ScopeReceipt {
            receipt_id: "receipt:review-update:0001".to_owned(),
            surface: DenseCollectionSurface::ReviewQueue,
            view_kind: CollectionViewKind::Queue,
            action_kind: BatchActionKind::Update,
            scope_class: ScopeReceiptClass::LoadedRows,
            execution_origin: ExecutionOriginClass::MixedClientProvider,
            selection_id_ref: "selection:review:update:0001".to_owned(),
            query_snapshot_id_ref: None,
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 12,
                loaded_count: 30,
                matching_count: Some(120),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 30,
                omitted_count: 0,
            },
            scope_label: "Updated the 30 loaded reviewed items, not all 120 matching the queue."
                .to_owned(),
            expansion_was_explicit: false,
            mutates_state: true,
            provider_backed: true,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:review-update:0001"]),
        },
        // Incident list: suppress only the visible incidents. The receipt names the
        // visible scope (15 rows) so visible rows are never treated as all matching.
        ScopeReceipt {
            receipt_id: "receipt:incident-suppress:0001".to_owned(),
            surface: DenseCollectionSurface::IncidentList,
            view_kind: CollectionViewKind::List,
            action_kind: BatchActionKind::Suppress,
            scope_class: ScopeReceiptClass::VisibleRows,
            execution_origin: ExecutionOriginClass::LocalClient,
            selection_id_ref: "selection:incident:suppress:0001".to_owned(),
            query_snapshot_id_ref: None,
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 15,
                loaded_count: 40,
                matching_count: Some(40),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 15,
                omitted_count: 0,
            },
            scope_label: "Suppressed the 15 visible incidents, not all 40 loaded or matching."
                .to_owned(),
            expansion_was_explicit: false,
            mutates_state: true,
            provider_backed: false,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:incident-suppress:0001"]),
        },
        // Marketplace results: install a provider-side selection the client never
        // enumerated row by row. Pinned to a snapshot and reached by an explicit
        // expansion step; the provider count is approximate.
        ScopeReceipt {
            receipt_id: "receipt:marketplace-install:0001".to_owned(),
            surface: DenseCollectionSurface::MarketplaceResults,
            view_kind: CollectionViewKind::Table,
            action_kind: BatchActionKind::Install,
            scope_class: ScopeReceiptClass::ProviderSideSelection,
            execution_origin: ExecutionOriginClass::MixedClientProvider,
            selection_id_ref: "selection:marketplace:install:0001".to_owned(),
            query_snapshot_id_ref: Some("snapshot:marketplace:install:0001".to_owned()),
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 20,
                loaded_count: 20,
                matching_count: None,
                provider_side_count: Some(6),
                matching_is_approximate: true,
                acted_on_count: 6,
                omitted_count: 0,
            },
            scope_label:
                "Installed the provider-side selection of 6 extensions resolved by the marketplace."
                    .to_owned(),
            expansion_was_explicit: true,
            mutates_state: true,
            provider_backed: true,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:marketplace-install:0001"]),
        },
        // Provider/admin table: delete every matching record. Destructive and
        // provider-authoritative; 2 of 140 are provider-locked and omitted, named
        // explicitly so the all-matching scope is not mistaken for a clean sweep.
        ScopeReceipt {
            receipt_id: "receipt:admin-delete:0001".to_owned(),
            surface: DenseCollectionSurface::ProviderAdminTable,
            view_kind: CollectionViewKind::Table,
            action_kind: BatchActionKind::Delete,
            scope_class: ScopeReceiptClass::AllMatchingQuery,
            execution_origin: ExecutionOriginClass::ProviderAuthoritative,
            selection_id_ref: "selection:admin:delete:0001".to_owned(),
            query_snapshot_id_ref: Some("snapshot:admin:delete:0001".to_owned()),
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 25,
                loaded_count: 50,
                matching_count: Some(140),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 138,
                omitted_count: 2,
            },
            scope_label:
                "Deleted 138 of all 140 matching records; 2 provider-locked records were omitted."
                    .to_owned(),
            expansion_was_explicit: true,
            mutates_state: true,
            provider_backed: true,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:admin-delete:0001"]),
        },
        // Query-backed result set: export every matching row. The matching count is
        // streaming and approximate; the receipt flags it rather than implying the
        // visible page was the whole set.
        ScopeReceipt {
            receipt_id: "receipt:query-export:0001".to_owned(),
            surface: DenseCollectionSurface::QueryBackedResultSet,
            view_kind: CollectionViewKind::Table,
            action_kind: BatchActionKind::Export,
            scope_class: ScopeReceiptClass::AllMatchingQuery,
            execution_origin: ExecutionOriginClass::LocalClient,
            selection_id_ref: "selection:query:export:0001".to_owned(),
            query_snapshot_id_ref: Some("snapshot:query:export:0001".to_owned()),
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 50,
                loaded_count: 50,
                matching_count: None,
                provider_side_count: None,
                matching_is_approximate: true,
                acted_on_count: 1240,
                omitted_count: 0,
            },
            scope_label:
                "Exported all ~1,240 rows matching the query snapshot, not just the 50 on screen."
                    .to_owned(),
            expansion_was_explicit: true,
            mutates_state: false,
            provider_backed: false,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:query-export:0001"]),
        },
        // Activity rows: copy only the visible rows to the clipboard. Read-only but
        // still receipted so a copy names the visible scope (8 of 60 loaded).
        ScopeReceipt {
            receipt_id: "receipt:activity-copy:0001".to_owned(),
            surface: DenseCollectionSurface::ActivityRows,
            view_kind: CollectionViewKind::List,
            action_kind: BatchActionKind::Copy,
            scope_class: ScopeReceiptClass::VisibleRows,
            execution_origin: ExecutionOriginClass::LocalClient,
            selection_id_ref: "selection:activity:copy:0001".to_owned(),
            query_snapshot_id_ref: None,
            counts: ScopeReceiptCounts {
                selected_count: 0,
                visible_count: 8,
                loaded_count: 60,
                matching_count: Some(60),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 8,
                omitted_count: 0,
            },
            scope_label: "Copied the 8 visible activity rows, not all 60 loaded or matching."
                .to_owned(),
            expansion_was_explicit: false,
            mutates_state: false,
            provider_backed: false,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:activity-copy:0001"]),
        },
        // Provider/admin table: update the 5 explicitly selected records. Provider
        // completes the write; the receipt names the selected scope (5 of 140).
        ScopeReceipt {
            receipt_id: "receipt:admin-update:0001".to_owned(),
            surface: DenseCollectionSurface::ProviderAdminTable,
            view_kind: CollectionViewKind::Table,
            action_kind: BatchActionKind::Update,
            scope_class: ScopeReceiptClass::SelectedItems,
            execution_origin: ExecutionOriginClass::ProviderAuthoritative,
            selection_id_ref: "selection:admin:update:0001".to_owned(),
            query_snapshot_id_ref: None,
            counts: ScopeReceiptCounts {
                selected_count: 5,
                visible_count: 25,
                loaded_count: 50,
                matching_count: Some(140),
                provider_side_count: None,
                matching_is_approximate: false,
                acted_on_count: 5,
                omitted_count: 0,
            },
            scope_label: "Updated the 5 selected records, not all 140 matching the admin filter."
                .to_owned(),
            expansion_was_explicit: false,
            mutates_state: true,
            provider_backed: true,
            redaction_class_token: "metadata_safe_default".to_owned(),
            evidence_refs: refs(&["evidence:receipt:admin-update:0001"]),
        },
    ]
}

fn omission(
    cause: SnapshotOmissionCause,
    member_count: u64,
    reason_label: &str,
) -> SnapshotOmission {
    SnapshotOmission {
        cause,
        member_count,
        reason_label: reason_label.to_owned(),
        visible_to_operator: true,
    }
}

fn deep_link_snapshots() -> Vec<SavedQueryDeepLinkSnapshot> {
    vec![
        // A shared export link captured an all-matching scope; on reopen the live
        // query has diverged and 3 captured rows no longer match. Both counts shown.
        SavedQueryDeepLinkSnapshot {
            snapshot_id: "snapshot:query-export:0001".to_owned(),
            surface: DenseCollectionSurface::QueryBackedResultSet,
            captured_scope_class: ScopeReceiptClass::AllMatchingQuery,
            query_snapshot_id_ref: "snapshot:query:export:0001".to_owned(),
            captured_at: "2026-06-12T09:00:00Z".to_owned(),
            captured_matching_count: Some(1240),
            captured_is_approximate: true,
            reopened_at: Some("2026-06-13T08:00:00Z".to_owned()),
            current_matching_count: Some(1237),
            reopen_posture: DeepLinkReopenPosture::CurrentDivergedFromCaptured,
            omissions: vec![omission(
                SnapshotOmissionCause::NoLongerMatchesQuery,
                3,
                "3 captured rows no longer match the live query and were dropped on reopen.",
            )],
            preserves_current_versus_captured: true,
            implies_frozen_certainty: false,
            reopen_rebinds_to_live_query: true,
            snapshot_label:
                "Shared export scope: captured ~1,240 rows; 1,237 still match the live query."
                    .to_owned(),
            evidence_refs: refs(&["evidence:snapshot:query-export:0001"]),
        },
        // A provider-side marketplace selection cannot be guaranteed to match live
        // results on reopen; the provider removed 2 captured extensions.
        SavedQueryDeepLinkSnapshot {
            snapshot_id: "snapshot:marketplace-install:0001".to_owned(),
            surface: DenseCollectionSurface::MarketplaceResults,
            captured_scope_class: ScopeReceiptClass::ProviderSideSelection,
            query_snapshot_id_ref: "snapshot:marketplace:install:0001".to_owned(),
            captured_at: "2026-06-12T10:30:00Z".to_owned(),
            captured_matching_count: Some(6),
            captured_is_approximate: true,
            reopened_at: Some("2026-06-13T07:15:00Z".to_owned()),
            current_matching_count: Some(4),
            reopen_posture: DeepLinkReopenPosture::ProviderResultsMayDiffer,
            omissions: vec![omission(
                SnapshotOmissionCause::ProviderRemoved,
                2,
                "2 captured extensions were removed by the provider since the link was shared.",
            )],
            preserves_current_versus_captured: true,
            implies_frozen_certainty: false,
            reopen_rebinds_to_live_query: true,
            snapshot_label:
                "Shared install scope: provider-side set may differ; 4 of 6 captured remain."
                    .to_owned(),
            evidence_refs: refs(&["evidence:snapshot:marketplace-install:0001"]),
        },
        // A selected-items deep link whose captured scope still matches the live
        // results — reported as an observation, never as frozen certainty.
        SavedQueryDeepLinkSnapshot {
            snapshot_id: "snapshot:pipeline-rerun:0001".to_owned(),
            surface: DenseCollectionSurface::PipelineRunList,
            captured_scope_class: ScopeReceiptClass::SelectedItems,
            query_snapshot_id_ref: "snapshot:pipeline:rerun:0001".to_owned(),
            captured_at: "2026-06-13T08:55:00Z".to_owned(),
            captured_matching_count: Some(8),
            captured_is_approximate: false,
            reopened_at: Some("2026-06-13T09:00:00Z".to_owned()),
            current_matching_count: Some(8),
            reopen_posture: DeepLinkReopenPosture::CapturedMatchesCurrent,
            omissions: vec![],
            preserves_current_versus_captured: true,
            implies_frozen_certainty: false,
            reopen_rebinds_to_live_query: true,
            snapshot_label:
                "Shared rerun scope: all 8 selected runs still match; verified on reopen."
                    .to_owned(),
            evidence_refs: refs(&["evidence:snapshot:pipeline-rerun:0001"]),
        },
        // A stale, not-yet-reopened all-matching snapshot that must be re-resolved
        // against the current query before any batch action.
        SavedQueryDeepLinkSnapshot {
            snapshot_id: "snapshot:incident-stale:0001".to_owned(),
            surface: DenseCollectionSurface::IncidentList,
            captured_scope_class: ScopeReceiptClass::AllMatchingQuery,
            query_snapshot_id_ref: "snapshot:incident:stale:0001".to_owned(),
            captured_at: "2026-05-30T12:00:00Z".to_owned(),
            captured_matching_count: Some(40),
            captured_is_approximate: false,
            reopened_at: None,
            current_matching_count: None,
            reopen_posture: DeepLinkReopenPosture::CapturedSnapshotStale,
            omissions: vec![],
            preserves_current_versus_captured: true,
            implies_frozen_certainty: false,
            reopen_rebinds_to_live_query: true,
            snapshot_label:
                "Saved incident scope is stale; it will be re-resolved against the live query."
                    .to_owned(),
            evidence_refs: refs(&["evidence:snapshot:incident-stale:0001"]),
        },
    ]
}

fn guardrails() -> ScopeReceiptGuardrails {
    ScopeReceiptGuardrails {
        row_highlight_is_not_durable_selection: true,
        provider_policy_narrowing_never_hidden: true,
        visible_rows_not_all_matching_without_explicit_step: true,
        broad_action_cannot_bypass_preview: true,
        deep_link_never_implies_frozen_certainty: true,
        receipt_names_selected_versus_matching: true,
    }
}

fn consumer_projection() -> ScopeReceiptConsumerProjection {
    ScopeReceiptConsumerProjection {
        product_renders_scope_receipt: true,
        diagnostics_reconstructs_scope_class: true,
        support_export_reuses_records: true,
        docs_help_reuses_vocabulary: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        SCOPE_RECEIPT_SCHEMA_REF,
        SCOPE_RECEIPT_DOC_REF,
        SCOPE_RECEIPT_ARTIFACT_REF,
        "schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json",
        "schemas/collections/selection-scope.schema.json",
    ])
}

fn packet() -> ScopeReceiptPacket {
    ScopeReceiptPacket::new(ScopeReceiptPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Scope Receipts And Saved-Query Deep-Link Snapshots".to_owned(),
        receipts: receipts(),
        deep_link_snapshots: deep_link_snapshots(),
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
