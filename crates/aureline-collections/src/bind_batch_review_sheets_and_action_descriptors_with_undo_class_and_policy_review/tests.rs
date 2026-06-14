use super::*;

const PACKET_ID: &str = "m5-batch-review-sheet:test:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn member(id: &str, disposition: BatchMemberDisposition, reason: &str) -> BatchReviewMemberRow {
    BatchReviewMemberRow {
        stable_item_id: id.to_owned(),
        review_label: format!("Row {id}"),
        disposition,
        disposition_reason: reason.to_owned(),
        in_current_filter: true,
    }
}

fn counts(
    included: u64,
    excluded: u64,
    blocked: u64,
    skipped: u64,
    hidden: u64,
) -> BatchScopeCounts {
    BatchScopeCounts {
        included,
        excluded,
        blocked,
        skipped,
        hidden,
        total_reviewed: included + excluded + blocked + skipped + hidden,
    }
}

fn local_reversible_action() -> BatchActionScopeDescriptor {
    BatchActionScopeDescriptor {
        action_id: "action:suppress".to_owned(),
        action_kind: BatchActionKind::Suppress,
        scope_class: BatchActionScopeClass::LocalReversibleBatch,
        execution_origin: ExecutionOriginClass::LocalClient,
        undo_recovery_class: UndoRecoveryClass::FullyReversible,
        counts: counts(3, 0, 0, 0, 0),
        mutates_state: true,
        provider_backed: false,
        select_all_expansion_was_explicit: false,
        undo_recovery_label: "Un-suppress to restore each incident.".to_owned(),
    }
}

fn simple_sheet(
    sheet_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    action: BatchActionScopeDescriptor,
) -> BatchReviewSheet {
    let scope_blocks = if action.counts.blocked > 0 {
        vec![ScopeBlock {
            cause: BatchScopeNarrowingCause::PolicyBlocked,
            member_count: action.counts.blocked,
            reason_label: "blocked by policy; needs a second approver".to_owned(),
            visible_to_operator: true,
        }]
    } else {
        vec![]
    };
    BatchReviewSheet {
        sheet_id: sheet_id.to_owned(),
        surface,
        view_kind,
        selection_id_ref: format!("selection:{sheet_id}"),
        action,
        sheet_title: "Batch action".to_owned(),
        member_rows: vec![member(
            "item:1",
            BatchMemberDisposition::Included,
            "selected and eligible",
        )],
        scope_blocks,
        requires_review_before_commit: true,
        blocks_generic_continue: true,
        names_included_excluded_blocked_skipped: true,
        recovery_posture_label: "Fully reversible; un-suppress to restore.".to_owned(),
        recovery_posture_exportable: true,
        result_summary: None,
        accessibility_summary: "3 included; recoverable.".to_owned(),
        evidence_refs: refs(&[&format!("evidence:{sheet_id}")]),
    }
}

fn provider_backed_action() -> BatchActionScopeDescriptor {
    BatchActionScopeDescriptor {
        action_id: "action:install".to_owned(),
        action_kind: BatchActionKind::Install,
        scope_class: BatchActionScopeClass::MixedClientProviderBatch,
        execution_origin: ExecutionOriginClass::MixedClientProvider,
        undo_recovery_class: UndoRecoveryClass::CompensatableViaInverse,
        counts: counts(6, 1, 2, 0, 3),
        mutates_state: true,
        provider_backed: true,
        select_all_expansion_was_explicit: true,
        undo_recovery_label: "Uninstall to reverse.".to_owned(),
    }
}

fn irreversible_action() -> BatchActionScopeDescriptor {
    BatchActionScopeDescriptor {
        action_id: "action:delete".to_owned(),
        action_kind: BatchActionKind::Delete,
        scope_class: BatchActionScopeClass::DestructiveGatedBatch,
        execution_origin: ExecutionOriginClass::ProviderAuthoritative,
        undo_recovery_class: UndoRecoveryClass::Irreversible,
        counts: counts(4, 0, 1, 0, 0),
        mutates_state: true,
        provider_backed: true,
        select_all_expansion_was_explicit: true,
        undo_recovery_label: "Permanent; export first.".to_owned(),
    }
}

fn mixed_result() -> BatchResultSummary {
    BatchResultSummary {
        succeeded_count: 7,
        failed_count: 1,
        skipped_count: 0,
        blocked_count: 1,
        summary_label: "7 succeeded, 1 failed, 1 blocked.".to_owned(),
        per_item_results: vec![
            BatchItemResultRow {
                stable_item_id: "item:7".to_owned(),
                outcome: BatchItemOutcome::Failed,
                outcome_label: "runner pool exhausted".to_owned(),
                recovery_action_ref: Some("retry:item:7".to_owned()),
            },
            BatchItemResultRow {
                stable_item_id: "item:8".to_owned(),
                outcome: BatchItemOutcome::Blocked,
                outcome_label: "locked by a deployment".to_owned(),
                recovery_action_ref: None,
            },
        ],
        collapses_to_single_toast: false,
    }
}

fn baseline_sheets() -> Vec<BatchReviewSheet> {
    let mut pipeline = simple_sheet(
        "sheet:pipeline",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
        BatchActionScopeDescriptor {
            action_id: "action:rerun".to_owned(),
            action_kind: BatchActionKind::Rerun,
            scope_class: BatchActionScopeClass::LocalReversibleBatch,
            execution_origin: ExecutionOriginClass::LocalClient,
            undo_recovery_class: UndoRecoveryClass::ReversibleWithinWindow,
            counts: counts(8, 2, 1, 0, 0),
            mutates_state: true,
            provider_backed: false,
            select_all_expansion_was_explicit: false,
            undo_recovery_label: "Cancel within 5 minutes.".to_owned(),
        },
    );
    pipeline.scope_blocks = vec![ScopeBlock {
        cause: BatchScopeNarrowingCause::ClientCapability,
        member_count: 1,
        reason_label: "1 run is locked by a deployment.".to_owned(),
        visible_to_operator: true,
    }];
    pipeline.result_summary = Some(mixed_result());

    vec![
        pipeline,
        simple_sheet(
            "sheet:review",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            provider_backed_action(),
        ),
        simple_sheet(
            "sheet:incident",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            local_reversible_action(),
        ),
        simple_sheet(
            "sheet:marketplace",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            provider_backed_action(),
        ),
        simple_sheet(
            "sheet:admin",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            irreversible_action(),
        ),
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
    ])
}

fn baseline_packet() -> BatchReviewSheetPacket {
    BatchReviewSheetPacket::new(BatchReviewSheetPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test batch review sheet packet".to_owned(),
        sheets: baseline_sheets(),
        guardrails: guardrails(),
        consumer_projection: consumer_projection(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: MINTED_AT.to_owned(),
    })
}

#[test]
fn baseline_packet_validates() {
    assert!(baseline_packet().validate().is_empty());
}

#[test]
fn counts_must_reconcile() {
    let mut counts = counts(3, 0, 0, 0, 0);
    assert!(counts.reconciles());
    counts.total_reviewed = 9;
    assert!(!counts.reconciles());

    let mut packet = baseline_packet();
    packet.sheets[2].action.counts.total_reviewed = 99;
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::CountsDoNotReconcile));
}

#[test]
fn consequential_action_cannot_bypass_review() {
    let mut packet = baseline_packet();
    // Delete sheet is consequential; drop the generic-Continue gate.
    packet.sheets[4].blocks_generic_continue = false;
    assert!(!packet.sheets[4].gate_holds());
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::ConsequentialActionBypassesReview));
}

#[test]
fn consequential_action_must_name_dispositions() {
    let mut packet = baseline_packet();
    packet.sheets[4].names_included_excluded_blocked_skipped = false;
    assert!(!packet.sheets[4].gate_holds());
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::ConsequentialActionBypassesReview));
}

#[test]
fn undo_class_must_match_mutation() {
    let mut action = irreversible_action();
    action.undo_recovery_class = UndoRecoveryClass::NoMutation;
    // mutates_state stays true while claiming no mutation.
    assert!(!action.is_valid());
}

#[test]
fn no_mutation_cannot_claim_mutation() {
    let mut action = local_reversible_action();
    action.mutates_state = false;
    action.undo_recovery_class = UndoRecoveryClass::NoMutation;
    assert!(action.is_valid());
    action.undo_recovery_class = UndoRecoveryClass::FullyReversible;
    // not mutating but claims a mutating recovery class.
    assert!(!action.is_valid());
}

#[test]
fn provider_backed_cannot_be_local_origin() {
    let mut action = provider_backed_action();
    action.execution_origin = ExecutionOriginClass::LocalClient;
    assert!(!action.is_valid());
}

#[test]
fn recovery_posture_must_be_exportable_and_precise() {
    let mut sheet = simple_sheet(
        "s",
        DenseCollectionSurface::IncidentList,
        CollectionViewKind::List,
        local_reversible_action(),
    );
    assert!(sheet.recovery_disclosed());

    sheet.recovery_posture_exportable = false;
    assert!(!sheet.recovery_disclosed());

    sheet.recovery_posture_exportable = true;
    sheet.recovery_posture_label = "n/a".to_owned();
    assert!(!sheet.recovery_disclosed());

    let mut packet = baseline_packet();
    packet.sheets[2].recovery_posture_label = "done".to_owned();
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::RecoveryPostureUndisclosed));
}

#[test]
fn blocked_members_require_explicit_scope_blocks() {
    let mut packet = baseline_packet();
    // marketplace install has blocked=2; drop the explaining block.
    packet.sheets[3].scope_blocks.clear();
    assert!(!packet.sheets[3].narrowing_surfaced());
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::ProviderPolicyNarrowingHidden));
}

#[test]
fn hidden_scope_block_is_rejected() {
    let mut packet = baseline_packet();
    packet.sheets[3].scope_blocks[0].visible_to_operator = false;
    assert!(!packet.sheets[3].narrowing_surfaced());
}

#[test]
fn mixed_outcome_is_preserved_not_collapsed() {
    let summary = mixed_result();
    assert!(summary.is_mixed());
    assert!(summary.preserves_per_item_truth());

    let mut collapsed = mixed_result();
    collapsed.collapses_to_single_toast = true;
    assert!(!collapsed.preserves_per_item_truth());

    let mut packet = baseline_packet();
    packet.sheets[0]
        .result_summary
        .as_mut()
        .unwrap()
        .collapses_to_single_toast = true;
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::MixedOutcomeCollapsed));
}

#[test]
fn failed_members_must_be_individually_accounted() {
    let mut summary = mixed_result();
    // Claim a failure but enumerate no failed/blocked per-item rows.
    summary.per_item_results.clear();
    assert!(!summary.preserves_per_item_truth());
}

#[test]
fn member_rows_cannot_exceed_counts() {
    let mut packet = baseline_packet();
    // incident sheet claims 0 excluded; enumerate an excluded row.
    packet.sheets[2].member_rows.push(member(
        "item:extra",
        BatchMemberDisposition::Excluded,
        "excluded by operator",
    ));
    assert!(!packet.sheets[2].member_rows_consistent_with_counts());
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::MemberRowsExceedCounts));
}

#[test]
fn non_included_member_requires_reason() {
    let row = member("item:x", BatchMemberDisposition::Blocked, "");
    assert!(!row.is_valid());
    let row = member(
        "item:x",
        BatchMemberDisposition::Blocked,
        "locked by provider",
    );
    assert!(row.is_valid());
    // Included rows do not require a reason.
    let row = BatchReviewMemberRow {
        stable_item_id: "item:y".to_owned(),
        review_label: "Row".to_owned(),
        disposition: BatchMemberDisposition::Included,
        disposition_reason: String::new(),
        in_current_filter: true,
    };
    assert!(row.is_valid());
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .sheets
        .retain(|sheet| sheet.surface != DenseCollectionSurface::ProviderAdminTable);
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_irreversible_case_is_rejected() {
    let mut packet = baseline_packet();
    for sheet in &mut packet.sheets {
        if sheet.action.undo_recovery_class == UndoRecoveryClass::Irreversible {
            sheet.action.undo_recovery_class = UndoRecoveryClass::CompensatableViaInverse;
            sheet.action.undo_recovery_label = "Restore from backup.".to_owned();
        }
    }
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::IrreversibleCaseMissing));
}

#[test]
fn missing_provider_backed_case_is_rejected() {
    let mut packet = baseline_packet();
    for sheet in &mut packet.sheets {
        sheet.action.provider_backed = false;
        if sheet.action.execution_origin != ExecutionOriginClass::LocalClient {
            sheet.action.execution_origin = ExecutionOriginClass::LocalClient;
        }
    }
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::ProviderBackedCaseMissing));
}

#[test]
fn missing_provider_policy_block_case_is_rejected() {
    let mut packet = baseline_packet();
    // Replace every provider/policy block with a non-provider/policy cause while
    // keeping the blocked counts explained.
    for sheet in &mut packet.sheets {
        for block in &mut sheet.scope_blocks {
            block.cause = BatchScopeNarrowingCause::ClientCapability;
        }
    }
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::ProviderPolicyBlockCaseMissing));
}

#[test]
fn missing_mixed_outcome_case_is_rejected() {
    let mut packet = baseline_packet();
    for sheet in &mut packet.sheets {
        sheet.result_summary = None;
    }
    assert!(packet
        .validate()
        .contains(&BatchReviewSheetViolation::MixedOutcomeCaseMissing));
}

#[test]
fn undo_class_recoverability_semantics() {
    assert!(!UndoRecoveryClass::Irreversible.is_recoverable());
    assert!(UndoRecoveryClass::FullyReversible.is_recoverable());
    assert!(!UndoRecoveryClass::NoMutation.mutates());
    assert!(UndoRecoveryClass::Irreversible.requires_explicit_review());
    assert!(UndoRecoveryClass::PartiallyReversible.requires_explicit_review());
    assert!(!UndoRecoveryClass::FullyReversible.requires_explicit_review());
}

#[test]
fn read_only_action_is_consequential_when_exporting() {
    let action = BatchActionScopeDescriptor {
        action_id: "action:export".to_owned(),
        action_kind: BatchActionKind::Export,
        scope_class: BatchActionScopeClass::LocalReversibleBatch,
        execution_origin: ExecutionOriginClass::LocalClient,
        undo_recovery_class: UndoRecoveryClass::NoMutation,
        counts: counts(5, 0, 0, 0, 0),
        mutates_state: false,
        provider_backed: false,
        select_all_expansion_was_explicit: false,
        undo_recovery_label: "Nothing to undo; data was copied out.".to_owned(),
    };
    assert!(action.is_valid());
    assert!(action.is_consequential());
}

#[test]
fn reconstruction_recovers_batch_truth() {
    let packet = baseline_packet();
    let reconstructions = packet.reconstructions();
    assert_eq!(reconstructions.len(), packet.sheets.len());

    let admin = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.sheet_id == "sheet:admin")
        .expect("admin reconstruction present");
    assert_eq!(admin.undo_recovery_class_token, "irreversible");
    assert!(admin.is_consequential);
    assert!(admin.blocks_generic_continue);
    assert_eq!(admin.provider_policy_block_tokens, vec!["policy_blocked"]);

    let pipeline = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.sheet_id == "sheet:pipeline")
        .expect("pipeline reconstruction present");
    assert!(pipeline.has_result_summary);
    assert!(pipeline.result_is_mixed);
    assert_eq!(pipeline.included, 8);
}

#[test]
fn export_is_metadata_safe() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("bearer "));
    assert!(packet.validate().is_empty());
}

#[test]
fn record_kind_and_schema_version_are_pinned() {
    let packet = baseline_packet();
    assert_eq!(packet.record_kind, BATCH_REVIEW_SHEET_RECORD_KIND);
    assert_eq!(packet.schema_version, BATCH_REVIEW_SHEET_SCHEMA_VERSION);
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: BatchReviewSheetPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_batch_review_sheet_export()
        .expect("checked-in batch review sheet export parses and validates");
    assert_eq!(packet.packet_id, "m5-batch-review-sheet:stable:0001");
    assert!(packet.validate().is_empty());
    for required in REQUIRED_SHEET_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    assert_eq!(packet.gated_sheet_count(), packet.sheets.len());
    assert!(packet
        .represented_undo_classes()
        .contains(&UndoRecoveryClass::Irreversible));
}
