use super::*;

const PACKET_ID: &str = "m5-scope-receipt:test:0001";
const MINTED_AT: &str = "2026-06-13T00:00:00Z";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

#[allow(clippy::too_many_arguments)]
fn counts(
    selected: u64,
    visible: u64,
    loaded: u64,
    matching: Option<u64>,
    provider_side: Option<u64>,
    approximate: bool,
    acted_on: u64,
    omitted: u64,
) -> ScopeReceiptCounts {
    ScopeReceiptCounts {
        selected_count: selected,
        visible_count: visible,
        loaded_count: loaded,
        matching_count: matching,
        provider_side_count: provider_side,
        matching_is_approximate: approximate,
        acted_on_count: acted_on,
        omitted_count: omitted,
    }
}

#[allow(clippy::too_many_arguments)]
fn receipt(
    id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    action_kind: BatchActionKind,
    scope_class: ScopeReceiptClass,
    execution_origin: ExecutionOriginClass,
    counts: ScopeReceiptCounts,
    provider_backed: bool,
) -> ScopeReceipt {
    let beyond_loaded = scope_class.requires_explicit_expansion();
    ScopeReceipt {
        receipt_id: id.to_owned(),
        surface,
        view_kind,
        action_kind,
        scope_class,
        execution_origin,
        selection_id_ref: format!("selection:{id}"),
        query_snapshot_id_ref: beyond_loaded.then(|| format!("snapshot:{id}")),
        counts,
        scope_label: format!("Acted on the {} scope for {id}.", scope_class.as_str()),
        expansion_was_explicit: beyond_loaded,
        mutates_state: !matches!(action_kind, BatchActionKind::Export | BatchActionKind::Copy),
        provider_backed,
        redaction_class_token: "metadata_safe_default".to_owned(),
        evidence_refs: refs(&[&format!("evidence:{id}")]),
    }
}

fn baseline_receipts() -> Vec<ScopeReceipt> {
    vec![
        receipt(
            "receipt:pipeline-rerun",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            BatchActionKind::Rerun,
            ScopeReceiptClass::SelectedItems,
            ExecutionOriginClass::LocalClient,
            counts(8, 10, 10, Some(25), None, false, 8, 0),
            false,
        ),
        receipt(
            "receipt:review-update",
            DenseCollectionSurface::ReviewQueue,
            CollectionViewKind::Queue,
            BatchActionKind::Update,
            ScopeReceiptClass::LoadedRows,
            ExecutionOriginClass::MixedClientProvider,
            counts(0, 12, 30, Some(120), None, false, 30, 0),
            true,
        ),
        receipt(
            "receipt:incident-suppress",
            DenseCollectionSurface::IncidentList,
            CollectionViewKind::List,
            BatchActionKind::Suppress,
            ScopeReceiptClass::VisibleRows,
            ExecutionOriginClass::LocalClient,
            counts(0, 15, 40, Some(40), None, false, 15, 0),
            false,
        ),
        receipt(
            "receipt:marketplace-install",
            DenseCollectionSurface::MarketplaceResults,
            CollectionViewKind::Table,
            BatchActionKind::Install,
            ScopeReceiptClass::ProviderSideSelection,
            ExecutionOriginClass::MixedClientProvider,
            counts(0, 20, 20, None, Some(6), true, 6, 0),
            true,
        ),
        receipt(
            "receipt:admin-delete",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            BatchActionKind::Delete,
            ScopeReceiptClass::AllMatchingQuery,
            ExecutionOriginClass::ProviderAuthoritative,
            counts(0, 25, 50, Some(140), None, false, 138, 2),
            true,
        ),
        receipt(
            "receipt:query-export",
            DenseCollectionSurface::QueryBackedResultSet,
            CollectionViewKind::Table,
            BatchActionKind::Export,
            ScopeReceiptClass::AllMatchingQuery,
            ExecutionOriginClass::LocalClient,
            counts(0, 50, 50, None, None, true, 1240, 0),
            false,
        ),
        receipt(
            "receipt:activity-copy",
            DenseCollectionSurface::ActivityRows,
            CollectionViewKind::List,
            BatchActionKind::Copy,
            ScopeReceiptClass::VisibleRows,
            ExecutionOriginClass::LocalClient,
            counts(0, 8, 60, Some(60), None, false, 8, 0),
            false,
        ),
        receipt(
            "receipt:admin-update",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            BatchActionKind::Update,
            ScopeReceiptClass::SelectedItems,
            ExecutionOriginClass::ProviderAuthoritative,
            counts(5, 25, 50, Some(140), None, false, 5, 0),
            true,
        ),
    ]
}

fn omission(cause: SnapshotOmissionCause, count: u64) -> SnapshotOmission {
    SnapshotOmission {
        cause,
        member_count: count,
        reason_label: format!("{count} members omitted: {}", cause.as_str()),
        visible_to_operator: true,
    }
}

fn snapshot(
    id: &str,
    surface: DenseCollectionSurface,
    captured: ScopeReceiptClass,
    posture: DeepLinkReopenPosture,
    reopened: bool,
    omissions: Vec<SnapshotOmission>,
) -> SavedQueryDeepLinkSnapshot {
    SavedQueryDeepLinkSnapshot {
        snapshot_id: id.to_owned(),
        surface,
        captured_scope_class: captured,
        query_snapshot_id_ref: format!("snapshot:{id}"),
        captured_at: "2026-06-12T00:00:00Z".to_owned(),
        captured_matching_count: Some(120),
        captured_is_approximate: false,
        reopened_at: reopened.then(|| "2026-06-13T00:00:00Z".to_owned()),
        current_matching_count: reopened.then_some(117),
        reopen_posture: posture,
        omissions,
        preserves_current_versus_captured: true,
        implies_frozen_certainty: false,
        reopen_rebinds_to_live_query: true,
        snapshot_label: format!("Shared {} scope for {id}.", captured.as_str()),
        evidence_refs: refs(&[&format!("evidence:{id}")]),
    }
}

fn baseline_snapshots() -> Vec<SavedQueryDeepLinkSnapshot> {
    vec![
        snapshot(
            "snapshot:query-export",
            DenseCollectionSurface::QueryBackedResultSet,
            ScopeReceiptClass::AllMatchingQuery,
            DeepLinkReopenPosture::CurrentDivergedFromCaptured,
            true,
            vec![omission(SnapshotOmissionCause::NoLongerMatchesQuery, 3)],
        ),
        snapshot(
            "snapshot:marketplace",
            DenseCollectionSurface::MarketplaceResults,
            ScopeReceiptClass::ProviderSideSelection,
            DeepLinkReopenPosture::ProviderResultsMayDiffer,
            true,
            vec![omission(SnapshotOmissionCause::ProviderRemoved, 2)],
        ),
        snapshot(
            "snapshot:pipeline",
            DenseCollectionSurface::PipelineRunList,
            ScopeReceiptClass::SelectedItems,
            DeepLinkReopenPosture::CapturedMatchesCurrent,
            true,
            vec![],
        ),
        snapshot(
            "snapshot:incident-stale",
            DenseCollectionSurface::IncidentList,
            ScopeReceiptClass::AllMatchingQuery,
            DeepLinkReopenPosture::CapturedSnapshotStale,
            false,
            vec![],
        ),
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
    ])
}

fn baseline_packet() -> ScopeReceiptPacket {
    ScopeReceiptPacket::new(ScopeReceiptPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test scope receipt packet".to_owned(),
        receipts: baseline_receipts(),
        deep_link_snapshots: baseline_snapshots(),
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
fn counts_reconcile_against_known_population() {
    let known = counts(8, 10, 10, Some(25), None, false, 8, 0);
    assert!(known.reconciles(ScopeReceiptClass::SelectedItems));
    assert!(!known.reconciles(ScopeReceiptClass::VisibleRows)); // visible=10, acted_on=8

    let partition = counts(0, 25, 50, Some(140), None, false, 138, 2);
    assert!(partition.reconciles(ScopeReceiptClass::AllMatchingQuery));

    let mut packet = baseline_packet();
    packet.receipts[0].counts.acted_on_count = 99;
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::CountsDoNotReconcile));
}

#[test]
fn unknown_population_requires_approximate_flag() {
    let approximate = counts(0, 50, 50, None, None, true, 1240, 0);
    assert!(approximate.reconciles(ScopeReceiptClass::AllMatchingQuery));

    let exact_but_unknown = counts(0, 50, 50, None, None, false, 1240, 0);
    assert!(!exact_but_unknown.reconciles(ScopeReceiptClass::AllMatchingQuery));
}

#[test]
fn receipt_names_selected_versus_matching() {
    let mut packet = baseline_packet();
    // Strip every cross-reference that lets the operator tell selected from matching.
    packet.receipts[0].counts.matching_count = None;
    packet.receipts[0].counts.provider_side_count = None;
    packet.receipts[0].counts.matching_is_approximate = false;
    assert!(!packet.receipts[0].names_selected_versus_matching());
    let violations = packet.validate();
    assert!(violations.contains(&ScopeReceiptViolation::SelectedVersusMatchingNotNamed));
    assert!(violations.contains(&ScopeReceiptViolation::ReceiptInvalid));
}

#[test]
fn beyond_loaded_scope_requires_query_snapshot() {
    let mut packet = baseline_packet();
    // admin-delete is all-matching-query: drop its snapshot ref.
    packet.receipts[4].query_snapshot_id_ref = None;
    assert!(!packet.receipts[4].is_valid());
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::ScopeSnapshotMissing));
}

#[test]
fn beyond_loaded_scope_requires_explicit_expansion() {
    let mut packet = baseline_packet();
    packet.receipts[5].expansion_was_explicit = false;
    assert!(!packet.receipts[5].is_valid());
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::ExpansionNotExplicit));
}

#[test]
fn provider_side_scope_must_be_provider_backed() {
    let mut receipt = receipt(
        "r",
        DenseCollectionSurface::MarketplaceResults,
        CollectionViewKind::Table,
        BatchActionKind::Install,
        ScopeReceiptClass::ProviderSideSelection,
        ExecutionOriginClass::MixedClientProvider,
        counts(0, 20, 20, None, Some(6), true, 6, 0),
        true,
    );
    assert!(receipt.is_valid());
    receipt.provider_backed = false;
    assert!(!receipt.is_valid());
}

#[test]
fn provider_backed_cannot_be_local_origin() {
    let mut receipt = receipt(
        "r",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
        BatchActionKind::Update,
        ScopeReceiptClass::LoadedRows,
        ExecutionOriginClass::MixedClientProvider,
        counts(0, 12, 30, Some(120), None, false, 30, 0),
        true,
    );
    assert!(receipt.is_valid());
    receipt.execution_origin = ExecutionOriginClass::LocalClient;
    assert!(!receipt.is_valid());
}

#[test]
fn generic_scope_label_is_rejected() {
    let mut packet = baseline_packet();
    packet.receipts[0].scope_label = "selected".to_owned();
    assert!(!packet.receipts[0].is_valid());
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::ReceiptInvalid));
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .receipts
        .retain(|receipt| receipt.surface != DenseCollectionSurface::ReviewQueue);
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_required_action_kind_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .receipts
        .retain(|receipt| receipt.action_kind != BatchActionKind::Delete);
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::RequiredActionKindMissing));
}

#[test]
fn missing_scope_class_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .receipts
        .retain(|receipt| receipt.scope_class != ScopeReceiptClass::ProviderSideSelection);
    let violations = packet.validate();
    assert!(violations.contains(&ScopeReceiptViolation::ScopeClassCoverageMissing));
    assert!(violations.contains(&ScopeReceiptViolation::ProviderSideCaseMissing));
}

#[test]
fn missing_all_matching_case_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .receipts
        .retain(|receipt| receipt.scope_class != ScopeReceiptClass::AllMatchingQuery);
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::AllMatchingCaseMissing));
}

#[test]
fn deep_link_cannot_imply_frozen_certainty() {
    let mut packet = baseline_packet();
    packet.deep_link_snapshots[0].implies_frozen_certainty = true;
    assert!(!packet.deep_link_snapshots[0].is_valid());
    let violations = packet.validate();
    assert!(violations.contains(&ScopeReceiptViolation::DeepLinkImpliesFrozenCertainty));
    assert!(violations.contains(&ScopeReceiptViolation::DeepLinkSnapshotInvalid));
}

#[test]
fn deep_link_must_preserve_captured_versus_current() {
    let mut packet = baseline_packet();
    packet.deep_link_snapshots[0].preserves_current_versus_captured = false;
    assert!(!packet.deep_link_snapshots[0].honesty_holds());
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::DeepLinkSnapshotInvalid));
}

#[test]
fn deep_link_must_rebind_to_live_query() {
    let mut snapshot = baseline_snapshots().remove(0);
    assert!(snapshot.honesty_holds());
    snapshot.reopen_rebinds_to_live_query = false;
    assert!(!snapshot.honesty_holds());
}

#[test]
fn omissions_force_divergence_posture() {
    let mut packet = baseline_packet();
    // A snapshot with omissions cannot claim the captured scope still matches.
    packet.deep_link_snapshots[0].reopen_posture = DeepLinkReopenPosture::CapturedMatchesCurrent;
    assert!(!packet.deep_link_snapshots[0].divergence_consistent());
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::DeepLinkDivergenceHidden));
}

#[test]
fn hidden_omission_is_rejected() {
    let mut packet = baseline_packet();
    packet.deep_link_snapshots[1].omissions[0].visible_to_operator = false;
    assert!(!packet.deep_link_snapshots[1].is_valid());
}

#[test]
fn missing_divergence_case_is_rejected() {
    let mut packet = baseline_packet();
    for snapshot in &mut packet.deep_link_snapshots {
        snapshot.reopen_posture = DeepLinkReopenPosture::CapturedMatchesCurrent;
        snapshot.omissions.clear();
    }
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::DeepLinkDivergenceCaseMissing));
}

#[test]
fn missing_provider_posture_case_is_rejected() {
    let mut packet = baseline_packet();
    for snapshot in &mut packet.deep_link_snapshots {
        if snapshot.reopen_posture == DeepLinkReopenPosture::ProviderResultsMayDiffer {
            snapshot.reopen_posture = DeepLinkReopenPosture::CurrentDivergedFromCaptured;
        }
    }
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::DeepLinkProviderCaseMissing));
}

#[test]
fn guardrails_must_all_hold() {
    let mut packet = baseline_packet();
    packet.guardrails.deep_link_never_implies_frozen_certainty = false;
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::GuardrailsIncomplete));
}

#[test]
fn consumer_projection_must_all_hold() {
    let mut packet = baseline_packet();
    packet
        .consumer_projection
        .diagnostics_reconstructs_scope_class = false;
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::ConsumerProjectionIncomplete));
}

#[test]
fn missing_source_contracts_is_rejected() {
    let mut packet = baseline_packet();
    packet.source_contract_refs = refs(&[SCOPE_RECEIPT_SCHEMA_REF]);
    assert!(packet
        .validate()
        .contains(&ScopeReceiptViolation::MissingSourceContracts));
}

#[test]
fn reconstruction_recovers_scope_class() {
    let packet = baseline_packet();
    let reconstructions = packet.reconstructions();
    assert_eq!(reconstructions.len(), packet.receipts.len());

    let delete = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.receipt_id == "receipt:admin-delete")
        .expect("delete reconstruction present");
    assert_eq!(delete.scope_class_token, "all_matching_query");
    assert_eq!(delete.acted_on_count, 138);
    assert_eq!(delete.omitted_count, 2);
    assert!(delete.expansion_was_explicit);
    assert!(delete.has_query_snapshot);

    let install = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.receipt_id == "receipt:marketplace-install")
        .expect("install reconstruction present");
    assert!(install.provider_side);
    assert!(install.matching_is_approximate);
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
    assert_eq!(packet.record_kind, SCOPE_RECEIPT_RECORD_KIND);
    assert_eq!(packet.schema_version, SCOPE_RECEIPT_SCHEMA_VERSION);
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: ScopeReceiptPacket = serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn scope_class_expansion_semantics() {
    assert!(ScopeReceiptClass::AllMatchingQuery.requires_explicit_expansion());
    assert!(ScopeReceiptClass::ProviderSideSelection.requires_explicit_expansion());
    assert!(!ScopeReceiptClass::SelectedItems.requires_explicit_expansion());
    assert!(!ScopeReceiptClass::VisibleRows.requires_explicit_expansion());
    assert!(ScopeReceiptClass::ProviderSideSelection.is_provider_side());
    assert!(!ScopeReceiptClass::AllMatchingQuery.is_provider_side());
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_scope_receipt_export()
        .expect("checked-in scope receipt export parses and validates");
    assert_eq!(packet.packet_id, "m5-scope-receipt:stable:0001");
    assert!(packet.validate().is_empty());
    for required in REQUIRED_RECEIPT_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    for required in ScopeReceiptClass::ALL {
        assert!(packet.represented_scope_classes().contains(&required));
    }
    for required in REQUIRED_RECEIPT_ACTION_KINDS {
        assert!(packet.represented_action_kinds().contains(&required));
    }
    assert!(packet
        .represented_reopen_postures()
        .contains(&DeepLinkReopenPosture::ProviderResultsMayDiffer));
}
