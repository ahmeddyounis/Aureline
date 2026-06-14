//! Conformance dump for the M5 batch-review sheet packet — batch-action
//! descriptors with included / excluded / blocked / skipped counts, an
//! undo/recovery class, and provider/policy-scope review for broad M5 actions
//! across pipeline, review, incident, marketplace, and provider/admin surfaces.
//!
//! Prints the canonical support export (default) or the Markdown summary
//! (`summary` argument) so the checked-in artifact stays byte-aligned with the
//! in-crate builder.

use aureline_collections::bind_batch_review_sheets_and_action_descriptors_with_undo_class_and_policy_review::*;
use aureline_collections::stabilize_selection_scope_and_batch_result_truth::BatchMemberDisposition;
use aureline_collections::{
    BatchActionKind, BatchActionScopeClass, BatchItemOutcome, CollectionViewKind,
    DenseCollectionSurface, ExecutionOriginClass,
};

const PACKET_ID: &str = "m5-batch-review-sheet:stable:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn member(
    id: &str,
    label: &str,
    disposition: BatchMemberDisposition,
    reason: &str,
    in_current_filter: bool,
) -> BatchReviewMemberRow {
    BatchReviewMemberRow {
        stable_item_id: id.to_owned(),
        review_label: label.to_owned(),
        disposition,
        disposition_reason: reason.to_owned(),
        in_current_filter,
    }
}

fn sheets() -> Vec<BatchReviewSheet> {
    vec![
        // Pipeline run list: rerun failed runs. Mutating, locally reversible within
        // a cancel window, with a mixed-outcome result summary after execution.
        BatchReviewSheet {
            sheet_id: "sheet:pipeline-rerun:0001".to_owned(),
            surface: DenseCollectionSurface::PipelineRunList,
            view_kind: CollectionViewKind::List,
            selection_id_ref: "selection:pipeline:rerun:0001".to_owned(),
            action: BatchActionScopeDescriptor {
                action_id: "action:pipeline:rerun".to_owned(),
                action_kind: BatchActionKind::Rerun,
                scope_class: BatchActionScopeClass::LocalReversibleBatch,
                execution_origin: ExecutionOriginClass::LocalClient,
                undo_recovery_class: UndoRecoveryClass::ReversibleWithinWindow,
                counts: BatchScopeCounts {
                    included: 8,
                    excluded: 2,
                    blocked: 1,
                    skipped: 0,
                    hidden: 0,
                    total_reviewed: 11,
                },
                mutates_state: true,
                provider_backed: false,
                select_all_expansion_was_explicit: false,
                undo_recovery_label:
                    "Queued reruns can be cancelled within 5 minutes before they start.".to_owned(),
            },
            sheet_title: "Re-run 8 failed pipeline runs".to_owned(),
            member_rows: vec![
                member(
                    "pipeline:run:1",
                    "Run 1 (failed)",
                    BatchMemberDisposition::Included,
                    "selected and eligible to re-run",
                    true,
                ),
                member(
                    "pipeline:run:9",
                    "Run 9 (passed)",
                    BatchMemberDisposition::Excluded,
                    "excluded: last run already passed",
                    true,
                ),
                member(
                    "pipeline:run:locked",
                    "Run 12 (locked)",
                    BatchMemberDisposition::Blocked,
                    "blocked: run is locked by an in-progress deployment",
                    true,
                ),
            ],
            scope_blocks: vec![ScopeBlock {
                cause: BatchScopeNarrowingCause::ClientCapability,
                member_count: 1,
                reason_label: "1 run is locked by an in-progress deployment and cannot re-run yet."
                    .to_owned(),
                visible_to_operator: true,
            }],
            requires_review_before_commit: true,
            blocks_generic_continue: true,
            names_included_excluded_blocked_skipped: true,
            recovery_posture_label:
                "Reversible within a 5-minute cancel window; cancelled reruns leave runs untouched."
                    .to_owned(),
            recovery_posture_exportable: true,
            result_summary: Some(BatchResultSummary {
                succeeded_count: 7,
                failed_count: 1,
                skipped_count: 0,
                blocked_count: 1,
                summary_label:
                    "7 reruns queued, 1 failed to enqueue, 1 blocked by an active deployment."
                        .to_owned(),
                per_item_results: vec![
                    BatchItemResultRow {
                        stable_item_id: "pipeline:run:7".to_owned(),
                        outcome: BatchItemOutcome::Failed,
                        outcome_label: "rerun rejected: runner pool exhausted".to_owned(),
                        recovery_action_ref: Some("retry:pipeline:run:7".to_owned()),
                    },
                    BatchItemResultRow {
                        stable_item_id: "pipeline:run:locked".to_owned(),
                        outcome: BatchItemOutcome::Blocked,
                        outcome_label: "blocked: locked by an in-progress deployment".to_owned(),
                        recovery_action_ref: None,
                    },
                ],
                collapses_to_single_toast: false,
            }),
            accessibility_summary:
                "8 of 11 runs included; 2 excluded, 1 blocked; reversible within 5 minutes."
                    .to_owned(),
            evidence_refs: refs(&["evidence:sheet:pipeline-rerun:0001"]),
        },
        // Review queue: approve items. Provider-backed, mixed client/provider,
        // compensatable by re-opening, with a policy block surfaced explicitly.
        BatchReviewSheet {
            sheet_id: "sheet:review-approve:0001".to_owned(),
            surface: DenseCollectionSurface::ReviewQueue,
            view_kind: CollectionViewKind::Queue,
            selection_id_ref: "selection:review:approve:0001".to_owned(),
            action: BatchActionScopeDescriptor {
                action_id: "action:review:approve".to_owned(),
                action_kind: BatchActionKind::Approve,
                scope_class: BatchActionScopeClass::MixedClientProviderBatch,
                execution_origin: ExecutionOriginClass::MixedClientProvider,
                undo_recovery_class: UndoRecoveryClass::CompensatableViaInverse,
                counts: BatchScopeCounts {
                    included: 20,
                    excluded: 0,
                    blocked: 3,
                    skipped: 0,
                    hidden: 5,
                    total_reviewed: 28,
                },
                mutates_state: true,
                provider_backed: true,
                select_all_expansion_was_explicit: true,
                undo_recovery_label:
                    "Approvals can be reversed by re-opening each item from its history.".to_owned(),
            },
            sheet_title: "Approve 20 reviewed items".to_owned(),
            member_rows: vec![
                member(
                    "review:item:1",
                    "Review item 1",
                    BatchMemberDisposition::Included,
                    "selected and ready to approve",
                    true,
                ),
                member(
                    "review:item:policy",
                    "Review item 14",
                    BatchMemberDisposition::Blocked,
                    "blocked: a second approver is required by policy",
                    true,
                ),
                member(
                    "review:item:offfilter",
                    "Review item 22",
                    BatchMemberDisposition::Hidden,
                    "hidden: selected earlier but outside the current filter",
                    false,
                ),
            ],
            scope_blocks: vec![ScopeBlock {
                cause: BatchScopeNarrowingCause::PolicyBlocked,
                member_count: 3,
                reason_label:
                    "3 items require a second approver before they can be approved in bulk."
                        .to_owned(),
                visible_to_operator: true,
            }],
            requires_review_before_commit: true,
            blocks_generic_continue: true,
            names_included_excluded_blocked_skipped: true,
            recovery_posture_label:
                "Compensatable: approvals are reversed by re-opening each item from its history."
                    .to_owned(),
            recovery_posture_exportable: true,
            result_summary: None,
            accessibility_summary:
                "20 of 28 items approved; 3 blocked by policy, 5 hidden outside the filter."
                    .to_owned(),
            evidence_refs: refs(&["evidence:sheet:review-approve:0001"]),
        },
        // Incident list: suppress incidents. Mutating but fully reversible by
        // un-suppressing; some already-suppressed members are skipped no-ops.
        BatchReviewSheet {
            sheet_id: "sheet:incident-suppress:0001".to_owned(),
            surface: DenseCollectionSurface::IncidentList,
            view_kind: CollectionViewKind::List,
            selection_id_ref: "selection:incident:suppress:0001".to_owned(),
            action: BatchActionScopeDescriptor {
                action_id: "action:incident:suppress".to_owned(),
                action_kind: BatchActionKind::Suppress,
                scope_class: BatchActionScopeClass::LocalReversibleBatch,
                execution_origin: ExecutionOriginClass::LocalClient,
                undo_recovery_class: UndoRecoveryClass::FullyReversible,
                counts: BatchScopeCounts {
                    included: 12,
                    excluded: 3,
                    blocked: 0,
                    skipped: 2,
                    hidden: 0,
                    total_reviewed: 17,
                },
                mutates_state: true,
                provider_backed: false,
                select_all_expansion_was_explicit: false,
                undo_recovery_label:
                    "Suppressed incidents can be restored at any time by un-suppressing them."
                        .to_owned(),
            },
            sheet_title: "Suppress 12 incidents".to_owned(),
            member_rows: vec![
                member(
                    "incident:1",
                    "Incident 1",
                    BatchMemberDisposition::Included,
                    "selected and active",
                    true,
                ),
                member(
                    "incident:kept",
                    "Incident 5",
                    BatchMemberDisposition::Excluded,
                    "excluded: deselected by the operator",
                    true,
                ),
                member(
                    "incident:already",
                    "Incident 8",
                    BatchMemberDisposition::Skipped,
                    "skipped: already suppressed (no-op)",
                    true,
                ),
            ],
            scope_blocks: vec![],
            requires_review_before_commit: true,
            blocks_generic_continue: true,
            names_included_excluded_blocked_skipped: true,
            recovery_posture_label:
                "Fully reversible: un-suppress restores each incident with no data loss.".to_owned(),
            recovery_posture_exportable: true,
            result_summary: None,
            accessibility_summary:
                "12 of 17 incidents suppressed; 3 excluded, 2 already suppressed and skipped."
                    .to_owned(),
            evidence_refs: refs(&["evidence:sheet:incident-suppress:0001"]),
        },
        // Marketplace results: install extensions. Provider-backed, compensatable
        // by uninstall, with a provider-incompatibility block surfaced explicitly.
        BatchReviewSheet {
            sheet_id: "sheet:marketplace-install:0001".to_owned(),
            surface: DenseCollectionSurface::MarketplaceResults,
            view_kind: CollectionViewKind::Table,
            selection_id_ref: "selection:marketplace:install:0001".to_owned(),
            action: BatchActionScopeDescriptor {
                action_id: "action:marketplace:install".to_owned(),
                action_kind: BatchActionKind::Install,
                scope_class: BatchActionScopeClass::MixedClientProviderBatch,
                execution_origin: ExecutionOriginClass::MixedClientProvider,
                undo_recovery_class: UndoRecoveryClass::CompensatableViaInverse,
                counts: BatchScopeCounts {
                    included: 6,
                    excluded: 1,
                    blocked: 2,
                    skipped: 0,
                    hidden: 3,
                    total_reviewed: 12,
                },
                mutates_state: true,
                provider_backed: true,
                select_all_expansion_was_explicit: true,
                undo_recovery_label: "Installed extensions can be removed by uninstalling them."
                    .to_owned(),
            },
            sheet_title: "Install 6 extensions".to_owned(),
            member_rows: vec![
                member(
                    "market:item:1",
                    "Extension 1",
                    BatchMemberDisposition::Included,
                    "selected and compatible",
                    true,
                ),
                member(
                    "market:item:incompat",
                    "Extension 4",
                    BatchMemberDisposition::Blocked,
                    "blocked: not compatible with the current provider version",
                    true,
                ),
                member(
                    "market:item:offpage",
                    "Extension 9",
                    BatchMemberDisposition::Hidden,
                    "hidden: matched the query but is off the current page",
                    false,
                ),
            ],
            scope_blocks: vec![ScopeBlock {
                cause: BatchScopeNarrowingCause::ProviderBlocked,
                member_count: 2,
                reason_label:
                    "2 extensions are not compatible with the current provider and cannot install."
                        .to_owned(),
                visible_to_operator: true,
            }],
            requires_review_before_commit: true,
            blocks_generic_continue: true,
            names_included_excluded_blocked_skipped: true,
            recovery_posture_label:
                "Compensatable: uninstall each extension to reverse the install.".to_owned(),
            recovery_posture_exportable: true,
            result_summary: None,
            accessibility_summary:
                "6 of 12 extensions included; 1 excluded, 2 provider-blocked, 3 hidden off-page."
                    .to_owned(),
            evidence_refs: refs(&["evidence:sheet:marketplace-install:0001"]),
        },
        // Provider/admin table: delete records. Provider-authoritative and
        // irreversible — the undo posture is named and a provider lock is surfaced.
        BatchReviewSheet {
            sheet_id: "sheet:admin-delete:0001".to_owned(),
            surface: DenseCollectionSurface::ProviderAdminTable,
            view_kind: CollectionViewKind::Table,
            selection_id_ref: "selection:admin:delete:0001".to_owned(),
            action: BatchActionScopeDescriptor {
                action_id: "action:admin:delete".to_owned(),
                action_kind: BatchActionKind::Delete,
                scope_class: BatchActionScopeClass::DestructiveGatedBatch,
                execution_origin: ExecutionOriginClass::ProviderAuthoritative,
                undo_recovery_class: UndoRecoveryClass::Irreversible,
                counts: BatchScopeCounts {
                    included: 4,
                    excluded: 0,
                    blocked: 1,
                    skipped: 0,
                    hidden: 0,
                    total_reviewed: 5,
                },
                mutates_state: true,
                provider_backed: true,
                select_all_expansion_was_explicit: true,
                undo_recovery_label:
                    "Deletion is permanent; export the records first to retain a copy.".to_owned(),
            },
            sheet_title: "Delete 4 provider records".to_owned(),
            member_rows: vec![
                member(
                    "admin:row:1",
                    "Record 1",
                    BatchMemberDisposition::Included,
                    "selected and owned by you",
                    true,
                ),
                member(
                    "admin:row:locked",
                    "Record 5",
                    BatchMemberDisposition::Blocked,
                    "blocked: the provider has locked this record",
                    true,
                ),
            ],
            scope_blocks: vec![ScopeBlock {
                cause: BatchScopeNarrowingCause::ProviderBlocked,
                member_count: 1,
                reason_label: "1 record is locked by the provider and cannot be deleted."
                    .to_owned(),
                visible_to_operator: true,
            }],
            requires_review_before_commit: true,
            blocks_generic_continue: true,
            names_included_excluded_blocked_skipped: true,
            recovery_posture_label:
                "Irreversible: deletion is permanent. Export the 4 records first to retain a copy."
                    .to_owned(),
            recovery_posture_exportable: true,
            result_summary: None,
            accessibility_summary:
                "4 of 5 records will be permanently deleted; 1 is provider-locked and blocked."
                    .to_owned(),
            evidence_refs: refs(&["evidence:sheet:admin-delete:0001"]),
        },
    ]
}

fn guardrails() -> BatchReviewGuardrails {
    BatchReviewGuardrails {
        row_highlight_is_not_durable_selection: true,
        provider_policy_narrowing_never_hidden: true,
        visible_rows_not_all_matching_without_explicit_step: true,
        broad_action_cannot_bypass_preview: true,
        undo_recovery_class_visible_and_exportable: true,
    }
}

fn consumer_projection() -> BatchReviewConsumerProjection {
    BatchReviewConsumerProjection {
        product_renders_review_sheet: true,
        diagnostics_reconstructs_batch_truth: true,
        support_export_reuses_records: true,
        docs_help_reuses_vocabulary: true,
    }
}

fn source_contract_refs() -> Vec<String> {
    refs(&[
        BATCH_REVIEW_SHEET_SCHEMA_REF,
        BATCH_REVIEW_SHEET_DOC_REF,
        BATCH_REVIEW_SHEET_ARTIFACT_REF,
        "schemas/collections/selection-scope.schema.json",
        "schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json",
    ])
}

fn packet() -> BatchReviewSheetPacket {
    BatchReviewSheetPacket::new(BatchReviewSheetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "M5 Batch-Review Sheets And Batch-Action Descriptors".to_owned(),
        sheets: sheets(),
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
