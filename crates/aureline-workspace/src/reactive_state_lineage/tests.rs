//! Unit tests for the reactive-state lineage projection.

use super::*;

fn aligned_subscribers(epoch: u64) -> Vec<SubscriberEpochObservation> {
    REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| SubscriberEpochObservation {
            surface_kind: *surface,
            observed_epoch: epoch,
            observed_freshness: SubscriberFreshness::Authoritative,
            last_invalidation_cause: InvalidationCauseClass::AuthorityWrite,
            observed_at: "mono:1700000050".to_owned(),
        })
        .collect()
}

fn aligned_view(
    view_id: &str,
    view_class: MaterializedViewClass,
    posture: SupportExportPosture,
) -> MaterializedViewObservation {
    MaterializedViewObservation {
        view_id: view_id.to_owned(),
        title: format!("Aligned view {view_id}"),
        view_class,
        authority_label: AuthorityLabel::WorkspaceVfs,
        authority_epoch: 7,
        subscriber_epochs: aligned_subscribers(7),
        declared_parity_state: EpochParityState::Aligned,
        support_export: SupportExportInputs::metadata_safe_baseline(posture),
        declared_downgrade_label: ReactiveDowngradeLabel::None,
        open_gaps: vec![],
        captured_at: "mono:1700000050".to_owned(),
    }
}

fn drift_view(view_id: &str) -> MaterializedViewObservation {
    let mut subs = aligned_subscribers(7);
    subs[0].observed_epoch = 6;
    subs[0].observed_freshness = SubscriberFreshness::Cached;
    MaterializedViewObservation {
        view_id: view_id.to_owned(),
        title: format!("Drift view {view_id}"),
        view_class: MaterializedViewClass::DurableLocalMaterialization,
        authority_label: AuthorityLabel::DerivedKnowledge,
        authority_epoch: 7,
        subscriber_epochs: subs,
        declared_parity_state: EpochParityState::DriftDetected,
        support_export: SupportExportInputs::metadata_safe_baseline(
            SupportExportPosture::LocalOnly,
        ),
        declared_downgrade_label: ReactiveDowngradeLabel::YellowSurfacePartial,
        open_gaps: vec![OpenGapEntry {
            gap_class: OpenGapClass::SubscriberPending,
            summary: "shell subscriber lag".to_owned(),
        }],
        captured_at: "mono:1700000051".to_owned(),
    }
}

fn awaiting_view(view_id: &str) -> MaterializedViewObservation {
    let mut subs = aligned_subscribers(7);
    subs[1].observed_epoch = 6;
    subs[1].observed_freshness = SubscriberFreshness::Warming;
    subs[1].last_invalidation_cause = InvalidationCauseClass::ResyncRequired;
    MaterializedViewObservation {
        view_id: view_id.to_owned(),
        title: format!("Awaiting view {view_id}"),
        view_class: MaterializedViewClass::ExportableSnapshot,
        authority_label: AuthorityLabel::Execution,
        authority_epoch: 7,
        subscriber_epochs: subs,
        declared_parity_state: EpochParityState::AwaitingResync,
        support_export: SupportExportInputs::metadata_safe_baseline(
            SupportExportPosture::MetadataSafeExport,
        ),
        declared_downgrade_label: ReactiveDowngradeLabel::YellowAuthoritySkew,
        open_gaps: vec![OpenGapEntry {
            gap_class: OpenGapClass::DriftRecoveryManual,
            summary: "manual resync required".to_owned(),
        }],
        captured_at: "mono:1700000052".to_owned(),
    }
}

fn terminal_view(view_id: &str) -> MaterializedViewObservation {
    let mut subs = aligned_subscribers(7);
    subs[2].observed_freshness = SubscriberFreshness::Unavailable;
    MaterializedViewObservation {
        view_id: view_id.to_owned(),
        title: format!("Terminal view {view_id}"),
        view_class: MaterializedViewClass::ManagedReplicatedView,
        authority_label: AuthorityLabel::ProviderOverlay,
        authority_epoch: 7,
        subscriber_epochs: subs,
        declared_parity_state: EpochParityState::TerminalUnavailable,
        support_export: SupportExportInputs::metadata_safe_baseline(
            SupportExportPosture::MetadataSafeExport,
        ),
        declared_downgrade_label: ReactiveDowngradeLabel::DegradedToAuthorityOnly,
        open_gaps: vec![OpenGapEntry {
            gap_class: OpenGapClass::ReplicationPending,
            summary: "managed replication unavailable".to_owned(),
        }],
        captured_at: "mono:1700000053".to_owned(),
    }
}

fn stable_inputs() -> ReactiveStateInputs {
    ReactiveStateInputs {
        workspace_ref: "workspace-stable-0001".to_owned(),
        producer_ref: "producer-aureline-0001".to_owned(),
        corpus_ref: "reactive-state-corpus-stable-0001".to_owned(),
        captured_at: "mono:1700000049".to_owned(),
        views: vec![
            aligned_view(
                "view-ephemeral-shell",
                MaterializedViewClass::EphemeralProjection,
                SupportExportPosture::LocalOnly,
            ),
            drift_view("view-durable-index"),
            awaiting_view("view-exportable-support"),
            terminal_view("view-managed-replicated"),
        ],
    }
}

#[test]
fn clean_inputs_project_stable_record() {
    let inputs = stable_inputs();
    let record = project_reactive_state_lineage("posture.clean", &inputs);

    assert!(
        record.is_stable_qualified(),
        "narrow: {:?}",
        record.stable_qualification.narrow_reasons
    );
    assert!(record.is_support_export_safe());
    assert_eq!(record.record_kind, REACTIVE_STATE_LINEAGE_RECORD_KIND);
    assert_eq!(record.schema_ref, REACTIVE_STATE_LINEAGE_SCHEMA_REF);
    assert!(record.view_class_coverage.all_required_view_classes_present);
    assert!(
        record
            .view_class_coverage
            .all_required_consumer_surfaces_present
    );
    assert!(
        record
            .view_class_coverage
            .no_subscriber_epoch_exceeds_authority
    );
    assert!(
        record
            .stale_view_downgrade
            .all_non_aligned_views_carry_downgrade
    );
    assert!(record.stale_view_downgrade.all_aligned_views_carry_none);
    assert!(
        record
            .stale_view_downgrade
            .all_downgraded_views_record_open_gap
    );
    assert!(
        record
            .epoch_parity_honesty
            .all_views_declared_parity_matches_observation
    );
    assert!(record.support_export_honesty.all_views_preserve_epoch_state);
    assert!(
        record
            .support_export_honesty
            .all_exportable_or_replicated_views_have_safe_posture
    );
    assert_eq!(record.inspection_hooks.len(), 6);
    assert!(record
        .producer_attribution
        .integrity_hash
        .starts_with("rsl:"));
}

#[test]
fn missing_view_class_narrows_record() {
    let mut inputs = stable_inputs();
    inputs
        .views
        .retain(|view| view.view_class != MaterializedViewClass::ManagedReplicatedView);

    let record = project_reactive_state_lineage("posture.missing_class", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::RequiredViewClassMissing));
}

#[test]
fn missing_consumer_surface_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0]
        .subscriber_epochs
        .retain(|sub| sub.surface_kind != ConsumerSurfaceKind::Support);

    let record = project_reactive_state_lineage("posture.missing_surface", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::RequiredConsumerSurfaceMissing));
}

#[test]
fn subscriber_epoch_exceeding_authority_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0].subscriber_epochs[0].observed_epoch = inputs.views[0].authority_epoch + 1;

    let record = project_reactive_state_lineage("posture.subscriber_ahead", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::SubscriberEpochExceedsAuthority));
}

#[test]
fn drift_without_epoch_lag_narrows_record() {
    let mut inputs = stable_inputs();
    let drift = &mut inputs.views[1];
    for sub in &mut drift.subscriber_epochs {
        sub.observed_epoch = drift.authority_epoch;
        sub.observed_freshness = SubscriberFreshness::Authoritative;
        sub.last_invalidation_cause = InvalidationCauseClass::AuthorityWrite;
    }
    // declared_parity_state stays DriftDetected — that's the dishonesty.

    let record = project_reactive_state_lineage("posture.drift_no_lag", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::DriftWithoutEpochLag));
}

#[test]
fn aligned_with_downgrade_label_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0].declared_downgrade_label = ReactiveDowngradeLabel::YellowSurfacePartial;
    inputs.views[0].open_gaps.push(OpenGapEntry {
        gap_class: OpenGapClass::SubscriberPending,
        summary: "stowed".to_owned(),
    });

    let record = project_reactive_state_lineage("posture.aligned_with_downgrade", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::AlignedCarriesDowngrade));
    assert!(!record.stale_view_downgrade.all_aligned_views_carry_none);
}

#[test]
fn non_aligned_missing_downgrade_label_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[1].declared_downgrade_label = ReactiveDowngradeLabel::None;

    let record = project_reactive_state_lineage("posture.drift_no_downgrade", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::NonAlignedMissingDowngrade));
}

#[test]
fn downgraded_view_without_open_gap_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[1].open_gaps.clear();

    let record = project_reactive_state_lineage("posture.no_gap", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::DowngradedWithoutOpenGap));
}

#[test]
fn aligned_view_with_open_gap_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0].open_gaps.push(OpenGapEntry {
        gap_class: OpenGapClass::SubscriberPending,
        summary: "stray gap".to_owned(),
    });

    let record = project_reactive_state_lineage("posture.aligned_gap", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::AlignedCarriesOpenGap));
}

#[test]
fn support_export_dropping_epoch_fields_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0].support_export.includes_subscriber_epochs = false;

    let record = project_reactive_state_lineage("posture.support_dropped", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::SupportExportEpochFieldsDropped));
}

#[test]
fn exportable_view_with_local_only_posture_narrows_record() {
    let mut inputs = stable_inputs();
    let exportable = inputs
        .views
        .iter_mut()
        .find(|view| view.view_class == MaterializedViewClass::ExportableSnapshot)
        .expect("seeded");
    exportable.support_export.posture = SupportExportPosture::LocalOnly;

    let record = project_reactive_state_lineage("posture.local_only_export", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::SupportExportPostureUnsafe));
}

#[test]
fn support_export_redaction_unsafe_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.views[0].support_export.ambient_authority_excluded = false;

    let record = project_reactive_state_lineage("posture.ambient_authority", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::SupportExportRedactionUnsafe));
}

#[test]
fn missing_inspection_hook_narrows_record() {
    let inputs = stable_inputs();
    let mut hooks = default_reactive_state_inspection_hooks();
    for hook in &mut hooks {
        if hook.hook_class == ReactiveStateInspectionHookClass::CompareEpochs {
            hook.available = false;
        }
    }

    let record = project_reactive_state_lineage_with_hooks("posture.no_compare", &inputs, hooks);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::InspectionHookUnavailable));
}

#[test]
fn empty_workspace_ref_narrows_record() {
    let mut inputs = stable_inputs();
    inputs.workspace_ref = "".to_owned();

    let record = project_reactive_state_lineage("posture.empty_workspace", &inputs);
    assert!(!record.is_stable_qualified());
    assert!(record
        .stable_qualification
        .narrow_reasons
        .contains(&ReactiveStateLineageNarrowReason::LineageExportUnsafe));
}

#[test]
fn lines_projection_renders_required_sections() {
    let inputs = stable_inputs();
    let record = project_reactive_state_lineage("posture.lines", &inputs);
    let lines = reactive_state_lineage_lines(&record);

    assert!(lines
        .iter()
        .any(|line| line.contains("Reactive-state lineage")));
    assert!(lines
        .iter()
        .any(|line| line.contains("view_class_coverage")));
    assert!(lines.iter().any(|line| line == "View rows:"));
    assert!(lines
        .iter()
        .any(|line| line.contains("Stale-view downgrade")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Epoch parity honesty")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Support-export honesty")));
    assert!(lines.iter().any(|line| line == "Inspection hooks:"));
}

#[test]
fn record_round_trips_through_json() {
    let inputs = stable_inputs();
    let record = project_reactive_state_lineage("posture.round_trip", &inputs);
    let serialized = serde_json::to_string(&record).expect("record must serialize");
    let parsed: ReactiveStateLineageRecord =
        serde_json::from_str(&serialized).expect("record must deserialize");
    assert_eq!(record, parsed);
}
