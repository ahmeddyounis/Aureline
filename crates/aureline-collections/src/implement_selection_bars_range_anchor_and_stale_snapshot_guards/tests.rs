use super::*;

const PACKET_ID: &str = "m5-selection-bar-continuity:test:0001";
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

fn fresh_guard() -> StaleQuerySnapshotGuard {
    StaleQuerySnapshotGuard {
        selection_dataset_identity: "dataset:v1".to_owned(),
        current_dataset_identity: "dataset:v1".to_owned(),
        query_snapshot_id_ref: None,
        dataset_identity_change: DatasetIdentityChange::Unchanged,
        guard_outcome: StaleGuardOutcome::ProceedFresh,
        broad_action_cannot_bypass_preview: true,
        guidance_label: None,
    }
}

fn stale_guard() -> StaleQuerySnapshotGuard {
    StaleQuerySnapshotGuard {
        selection_dataset_identity: "dataset:v7".to_owned(),
        current_dataset_identity: "dataset:v9".to_owned(),
        query_snapshot_id_ref: Some("query-snapshot:v7".to_owned()),
        dataset_identity_change: DatasetIdentityChange::RowsAddedOrRemoved,
        guard_outcome: StaleGuardOutcome::RequireReopenReview,
        broad_action_cannot_bypass_preview: true,
        guidance_label: Some("12 rows changed since you selected; reopen to re-review".to_owned()),
    }
}

fn simple_bar(
    bar_id: &str,
    surface: DenseCollectionSurface,
    view_kind: CollectionViewKind,
    data_mode: CollectionDataMode,
) -> SelectionBar {
    SelectionBar {
        bar_id: bar_id.to_owned(),
        surface,
        view_kind,
        data_mode,
        label_summary: "Selection bar".to_owned(),
        membership: SelectionMembership {
            basis: SelectionMembershipBasis::StableIdentitySet,
            by_stable_identity: true,
            members: vec![item("item:1", "Row 1", true), item("item:2", "Row 2", true)],
            query_snapshot_id_ref: None,
        },
        range_anchor: None,
        counts: SelectionBarCounts {
            selected_total: 2,
            selected_visible: 2,
            selected_outside_filter: 0,
            selected_from_prior_snapshot: 0,
            selected_blocked: 0,
        },
        snapshot_guard: fresh_guard(),
        survives_sort_filter_virtualization: true,
        accessibility_summary: "2 selected; all visible.".to_owned(),
        evidence_refs: refs(&[&format!("evidence:{bar_id}")]),
    }
}

fn hidden_selected_bar() -> SelectionBar {
    let mut bar = simple_bar(
        "bar:review",
        DenseCollectionSurface::ReviewQueue,
        CollectionViewKind::Queue,
        CollectionDataMode::FilteredSorted,
    );
    bar.membership.members = vec![
        item("item:1", "Row 1", true),
        item("item:2", "Row 2", true),
        item("item:hidden", "Row hidden", false),
    ];
    bar.counts = SelectionBarCounts {
        selected_total: 3,
        selected_visible: 2,
        selected_outside_filter: 1,
        selected_from_prior_snapshot: 0,
        selected_blocked: 0,
    };
    bar.accessibility_summary = "3 selected; 1 outside the current filter.".to_owned();
    bar
}

fn range_anchor_bar() -> SelectionBar {
    let mut bar = simple_bar(
        "bar:graph",
        DenseCollectionSurface::GraphList,
        CollectionViewKind::Tree,
        CollectionDataMode::Virtualized,
    );
    bar.membership.basis = SelectionMembershipBasis::RangeAnchorExpansion;
    bar.range_anchor = Some(RangeAnchor {
        anchor_item_id: "item:1".to_owned(),
        focus_item_id: "item:2".to_owned(),
        anchored_by_stable_identity: true,
        anchor_still_present: true,
        visible_traversal_order: true,
        reresolution_label: None,
    });
    bar
}

fn prior_snapshot_bar() -> SelectionBar {
    let mut bar = simple_bar(
        "bar:marketplace",
        DenseCollectionSurface::MarketplaceResults,
        CollectionViewKind::Table,
        CollectionDataMode::Paginated,
    );
    bar.membership.basis = SelectionMembershipBasis::QuerySnapshot;
    bar.membership.query_snapshot_id_ref = Some("query-snapshot:marketplace:v3".to_owned());
    bar.membership.members = vec![item("item:1", "Row 1", true), item("item:2", "Row 2", true)];
    bar.counts = SelectionBarCounts {
        selected_total: 240,
        selected_visible: 240,
        selected_outside_filter: 0,
        selected_from_prior_snapshot: 240,
        selected_blocked: 6,
    };
    bar.snapshot_guard = stale_guard();
    bar.accessibility_summary =
        "240 selected from a prior snapshot; reopen required after changes.".to_owned();
    bar
}

fn baseline_bars() -> Vec<SelectionBar> {
    vec![
        simple_bar(
            "bar:pipeline",
            DenseCollectionSurface::PipelineRunList,
            CollectionViewKind::List,
            CollectionDataMode::StaticComplete,
        ),
        hidden_selected_bar(),
        {
            let mut bar = simple_bar(
                "bar:incident",
                DenseCollectionSurface::IncidentList,
                CollectionViewKind::List,
                CollectionDataMode::Streaming,
            );
            bar.snapshot_guard = StaleQuerySnapshotGuard {
                selection_dataset_identity: "dataset:incident:v2".to_owned(),
                current_dataset_identity: "dataset:incident:v2".to_owned(),
                query_snapshot_id_ref: None,
                dataset_identity_change: DatasetIdentityChange::ReorderedOnly,
                guard_outcome: StaleGuardOutcome::ProceedFresh,
                broad_action_cannot_bypass_preview: true,
                guidance_label: None,
            };
            bar
        },
        range_anchor_bar(),
        prior_snapshot_bar(),
        simple_bar(
            "bar:admin",
            DenseCollectionSurface::ProviderAdminTable,
            CollectionViewKind::Table,
            CollectionDataMode::Virtualized,
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
    ])
}

fn baseline_packet() -> SelectionBarContinuityPacket {
    SelectionBarContinuityPacket::new(SelectionBarContinuityPacketInput {
        packet_id: PACKET_ID.to_owned(),
        packet_label: "Test selection bar continuity packet".to_owned(),
        bars: baseline_bars(),
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
fn membership_by_row_position_is_rejected() {
    let mut packet = baseline_packet();
    packet.bars[0].membership.by_stable_identity = false;
    let violations = packet.validate();
    assert!(violations.contains(&SelectionBarContinuityViolation::MembershipNotByStableIdentity));
}

#[test]
fn counts_must_reconcile() {
    let mut bar = simple_bar(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
        CollectionDataMode::FilteredSorted,
    );
    // visible 2 + outside 3 != total 2.
    bar.counts.selected_outside_filter = 3;
    assert!(!bar.counts.reconciles());

    let mut packet = baseline_packet();
    packet.bars[0] = bar;
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::CountsDoNotReconcile));
}

#[test]
fn hidden_selection_must_be_disclosed() {
    let mut bar = hidden_selected_bar();
    bar.accessibility_summary = "3 selected.".to_owned();
    assert!(!bar.hidden_selection_disclosed());
    assert!(!bar.is_complete());
}

#[test]
fn material_change_cannot_proceed_silently() {
    let mut bar = prior_snapshot_bar();
    bar.snapshot_guard.guard_outcome = StaleGuardOutcome::ProceedFresh;
    assert!(!bar.snapshot_guard.is_consistent());

    let mut packet = baseline_packet();
    packet.bars[4] = bar;
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::StaleSnapshotProceedsSilently));
}

#[test]
fn material_change_may_downgrade_or_block() {
    for outcome in [
        StaleGuardOutcome::RequireReopenReview,
        StaleGuardOutcome::DowngradeToVisibleOnly,
        StaleGuardOutcome::BlockUntilResynced,
    ] {
        let mut guard = stale_guard();
        guard.guard_outcome = outcome;
        assert!(guard.is_consistent(), "outcome {outcome:?} should be valid");
        assert!(guard.is_stale());
    }
}

#[test]
fn unchanged_identity_must_proceed() {
    let mut guard = fresh_guard();
    guard.guard_outcome = StaleGuardOutcome::RequireReopenReview;
    guard.guidance_label = Some("spurious downgrade".to_owned());
    assert!(!guard.is_consistent());
}

#[test]
fn differing_identity_marked_unchanged_is_inconsistent() {
    let mut guard = fresh_guard();
    guard.current_dataset_identity = "dataset:v2".to_owned();
    // change still claims unchanged while identities differ.
    assert!(!guard.is_consistent());
}

#[test]
fn reordered_only_may_proceed() {
    let guard = StaleQuerySnapshotGuard {
        selection_dataset_identity: "dataset:v1".to_owned(),
        current_dataset_identity: "dataset:v1".to_owned(),
        query_snapshot_id_ref: None,
        dataset_identity_change: DatasetIdentityChange::ReorderedOnly,
        guard_outcome: StaleGuardOutcome::ProceedFresh,
        broad_action_cannot_bypass_preview: true,
        guidance_label: None,
    };
    assert!(guard.is_consistent());
    assert!(!guard.is_stale());
}

#[test]
fn broad_action_cannot_bypass_preview() {
    let mut bar = simple_bar(
        "b",
        DenseCollectionSurface::PipelineRunList,
        CollectionViewKind::List,
        CollectionDataMode::Virtualized,
    );
    bar.snapshot_guard.broad_action_cannot_bypass_preview = false;
    assert!(!bar.is_complete());

    let mut packet = baseline_packet();
    packet.bars[0] = bar;
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::BroadActionBypassesPreview));
}

#[test]
fn range_anchor_by_row_position_is_rejected() {
    let mut bar = range_anchor_bar();
    bar.range_anchor
        .as_mut()
        .unwrap()
        .anchored_by_stable_identity = false;
    assert!(!bar.range_anchor.as_ref().unwrap().is_valid());

    let mut packet = baseline_packet();
    packet.bars[3] = bar;
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::RangeAnchorNotByStableIdentity));
}

#[test]
fn departed_anchor_requires_reresolution_label() {
    let mut anchor = RangeAnchor {
        anchor_item_id: "item:1".to_owned(),
        focus_item_id: "item:2".to_owned(),
        anchored_by_stable_identity: true,
        anchor_still_present: false,
        visible_traversal_order: true,
        reresolution_label: None,
    };
    assert!(!anchor.is_valid());
    anchor.reresolution_label =
        Some("anchor left the filter; range re-anchors on the next visible item".to_owned());
    assert!(anchor.is_valid());
}

#[test]
fn query_backed_membership_requires_snapshot() {
    let mut membership = SelectionMembership {
        basis: SelectionMembershipBasis::QuerySnapshot,
        by_stable_identity: true,
        members: vec![item("item:1", "Row 1", true)],
        query_snapshot_id_ref: None,
    };
    assert!(!membership.is_valid());
    membership.query_snapshot_id_ref = Some("query-snapshot:v1".to_owned());
    assert!(membership.is_valid());
}

#[test]
fn membership_must_match_outside_filter_count() {
    let mut bar = hidden_selected_bar();
    // Mark every member in-filter while claiming one outside.
    for member in &mut bar.membership.members {
        member.in_current_filter = true;
    }
    assert!(!bar.membership_matches_counts());
}

#[test]
fn missing_required_surface_is_rejected() {
    let mut packet = baseline_packet();
    packet
        .bars
        .retain(|bar| bar.surface != DenseCollectionSurface::ProviderAdminTable);
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::RequiredSurfaceMissing));
}

#[test]
fn missing_view_kind_is_rejected() {
    let mut packet = baseline_packet();
    for bar in &mut packet.bars {
        if bar.view_kind == CollectionViewKind::Tree {
            bar.view_kind = CollectionViewKind::List;
        }
    }
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::RequiredViewKindMissing));
}

#[test]
fn missing_data_mode_is_rejected() {
    let mut packet = baseline_packet();
    for bar in &mut packet.bars {
        if bar.data_mode == CollectionDataMode::Streaming {
            bar.data_mode = CollectionDataMode::StaticComplete;
        }
    }
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::RequiredDataModeMissing));
}

#[test]
fn missing_stale_guard_case_is_rejected() {
    let mut packet = baseline_packet();
    for bar in &mut packet.bars {
        bar.snapshot_guard = fresh_guard();
        bar.counts.selected_from_prior_snapshot = 0;
        bar.counts.selected_blocked = 0;
        if bar.bar_id == "bar:marketplace" {
            bar.membership.basis = SelectionMembershipBasis::StableIdentitySet;
            bar.membership.query_snapshot_id_ref = None;
        }
    }
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::StaleGuardCaseMissing));
}

#[test]
fn missing_range_anchor_case_is_rejected() {
    let mut packet = baseline_packet();
    for bar in &mut packet.bars {
        bar.range_anchor = None;
    }
    assert!(packet
        .validate()
        .contains(&SelectionBarContinuityViolation::RangeAnchorCaseMissing));
}

#[test]
fn reconstruction_recovers_selection_truth() {
    let packet = baseline_packet();
    let reconstructions = packet.reconstructions();
    assert_eq!(reconstructions.len(), packet.bars.len());

    let review = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.bar_id == "bar:review")
        .expect("review reconstruction present");
    assert_eq!(review.selected_total, 3);
    assert_eq!(review.selected_outside_filter, 1);

    let marketplace = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.bar_id == "bar:marketplace")
        .expect("marketplace reconstruction present");
    assert_eq!(marketplace.selected_from_prior_snapshot, 240);
    assert!(marketplace.is_stale);
    assert_eq!(marketplace.guard_outcome_token, "require_reopen_review");

    let graph = reconstructions
        .iter()
        .find(|reconstruction| reconstruction.bar_id == "bar:graph")
        .expect("graph reconstruction present");
    assert!(graph.has_range_anchor);
    assert!(graph.range_anchor_present);
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
    assert_eq!(packet.record_kind, SELECTION_BAR_CONTINUITY_RECORD_KIND);
    assert_eq!(
        packet.schema_version,
        SELECTION_BAR_CONTINUITY_SCHEMA_VERSION
    );
}

#[test]
fn round_trips_through_json() {
    let packet = baseline_packet();
    let json = packet.export_safe_json();
    let parsed: SelectionBarContinuityPacket =
        serde_json::from_str(&json).expect("packet round-trips");
    assert_eq!(parsed, packet);
}

#[test]
fn checked_in_export_validates() {
    let packet = current_m5_selection_bar_continuity_export()
        .expect("checked-in selection bar continuity export parses and validates");
    assert_eq!(packet.packet_id, "m5-selection-bar-continuity:stable:0001");
    assert!(packet.validate().is_empty());
    for required in REQUIRED_BAR_SURFACES {
        assert!(packet.represented_surfaces().contains(&required));
    }
    for required in CollectionViewKind::ALL {
        assert!(packet.represented_view_kinds().contains(&required));
    }
    for required in REQUIRED_DATA_MODES {
        assert!(packet.represented_data_modes().contains(&required));
    }
    assert!(packet.stale_bar_count() >= 1);
}
