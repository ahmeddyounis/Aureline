//! Fixture generator helper for the reactive-state lineage replay gate.
//!
//! Only runs when `REACTIVE_STATE_LINEAGE_GEN_FIXTURES=1` is set in the
//! environment. Emits the canonical fixture JSON files into
//! `fixtures/workspace/m4/reactive_state_lineage/` so the replay gate has
//! a deterministic, checked-in corpus.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    default_reactive_state_inspection_hooks, project_reactive_state_lineage_with_hooks,
    ConsumerSurfaceKind, EpochParityState, MaterializedViewObservation, ReactiveAuthorityLabel,
    ReactiveDowngradeLabel, ReactiveInvalidationCauseClass, ReactiveMaterializedViewClass,
    ReactiveOpenGapClass, ReactiveOpenGapEntry, ReactiveStateInputs, ReactiveStateInspectionHook,
    ReactiveStateInspectionHookClass, ReactiveStateLineageRecord, ReactiveSubscriberFreshness,
    ReactiveSupportExportInputs, ReactiveSupportExportPosture, SubscriberEpochObservation,
    REQUIRED_CONSUMER_SURFACES,
};
use serde::Serialize;

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/workspace/m4/reactive_state_lineage")
}

fn authoritative_subscribers(epoch: u64, captured_at: &str) -> Vec<SubscriberEpochObservation> {
    REQUIRED_CONSUMER_SURFACES
        .iter()
        .map(|surface| SubscriberEpochObservation {
            surface_kind: *surface,
            observed_epoch: epoch,
            observed_freshness: ReactiveSubscriberFreshness::Authoritative,
            last_invalidation_cause: ReactiveInvalidationCauseClass::AuthorityWrite,
            observed_at: captured_at.to_owned(),
        })
        .collect()
}

fn aligned_view(
    view_id: &str,
    title: &str,
    view_class: ReactiveMaterializedViewClass,
    authority_label: ReactiveAuthorityLabel,
    posture: ReactiveSupportExportPosture,
    captured_at: &str,
) -> MaterializedViewObservation {
    MaterializedViewObservation {
        view_id: view_id.to_owned(),
        title: title.to_owned(),
        view_class,
        authority_label,
        authority_epoch: 7,
        subscriber_epochs: authoritative_subscribers(7, captured_at),
        declared_parity_state: EpochParityState::Aligned,
        support_export: ReactiveSupportExportInputs::metadata_safe_baseline(posture),
        declared_downgrade_label: ReactiveDowngradeLabel::None,
        open_gaps: vec![],
        captured_at: captured_at.to_owned(),
    }
}

fn override_subscriber(
    view: &mut MaterializedViewObservation,
    surface_kind: ConsumerSurfaceKind,
    observed_epoch: u64,
    observed_freshness: ReactiveSubscriberFreshness,
    last_invalidation_cause: ReactiveInvalidationCauseClass,
) {
    for sub in &mut view.subscriber_epochs {
        if sub.surface_kind == surface_kind {
            sub.observed_epoch = observed_epoch;
            sub.observed_freshness = observed_freshness;
            sub.last_invalidation_cause = last_invalidation_cause;
        }
    }
}

fn aligned_corpus(captured_at: &str) -> Vec<MaterializedViewObservation> {
    vec![
        aligned_view(
            "view-ephemeral-shell-status",
            "Shell status ephemeral projection",
            ReactiveMaterializedViewClass::EphemeralProjection,
            ReactiveAuthorityLabel::WorkspaceVfs,
            ReactiveSupportExportPosture::LocalOnly,
            captured_at,
        ),
        aligned_view(
            "view-durable-workspace-index",
            "Durable workspace search index",
            ReactiveMaterializedViewClass::DurableLocalMaterialization,
            ReactiveAuthorityLabel::DerivedKnowledge,
            ReactiveSupportExportPosture::LocalOnly,
            captured_at,
        ),
        aligned_view(
            "view-exportable-support-snapshot",
            "Exportable support snapshot",
            ReactiveMaterializedViewClass::ExportableSnapshot,
            ReactiveAuthorityLabel::PolicyEntitlement,
            ReactiveSupportExportPosture::MetadataSafeExport,
            captured_at,
        ),
        aligned_view(
            "view-managed-review-state",
            "Managed replicated review state",
            ReactiveMaterializedViewClass::ManagedReplicatedView,
            ReactiveAuthorityLabel::ProviderOverlay,
            ReactiveSupportExportPosture::MetadataSafeExport,
            captured_at,
        ),
    ]
}

fn base_inputs(
    workspace_ref: &str,
    corpus_ref: &str,
    captured_at: &str,
    views: Vec<MaterializedViewObservation>,
) -> ReactiveStateInputs {
    ReactiveStateInputs {
        workspace_ref: workspace_ref.to_owned(),
        producer_ref: "producer-aureline-fixtures-0001".to_owned(),
        corpus_ref: corpus_ref.to_owned(),
        captured_at: captured_at.to_owned(),
        views,
    }
}

#[derive(Debug, Serialize)]
struct FixtureEnvelope<'a> {
    posture_id: &'a str,
    inputs: &'a ReactiveStateInputs,
    inspection_hooks: &'a Vec<ReactiveStateInspectionHook>,
    expected: &'a ReactiveStateLineageRecord,
}

fn write_fixture(
    name: &str,
    posture_id: &str,
    inputs: ReactiveStateInputs,
    inspection_hooks: Vec<ReactiveStateInspectionHook>,
) {
    let record =
        project_reactive_state_lineage_with_hooks(posture_id, &inputs, inspection_hooks.clone());
    let envelope = FixtureEnvelope {
        posture_id,
        inputs: &inputs,
        inspection_hooks: &inspection_hooks,
        expected: &record,
    };
    let path = fixtures_dir().join(format!("{name}.json"));
    let json = serde_json::to_string_pretty(&envelope).expect("envelope serializes");
    std::fs::write(&path, json + "\n").expect("fixture write");
    eprintln!("wrote {}", path.display());
}

#[test]
fn generate_fixtures() {
    if std::env::var("REACTIVE_STATE_LINEAGE_GEN_FIXTURES")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }
    std::fs::create_dir_all(fixtures_dir()).expect("ensure fixture dir");

    // Aligned corpus: every required view class is aligned at the same
    // authority epoch.
    let aligned_inputs = base_inputs(
        "workspace-rust-service-0001",
        "reactive-state-corpus-aligned-0001",
        "mono:1700000100",
        aligned_corpus("mono:1700000100"),
    );
    write_fixture(
        "aligned_stable",
        "posture:aligned",
        aligned_inputs,
        default_reactive_state_inspection_hooks(),
    );

    // Drift detected: durable workspace index lags on the shell surface.
    let mut drift_views = aligned_corpus("mono:1700000110");
    {
        let durable = drift_views
            .iter_mut()
            .find(|v| v.view_class == ReactiveMaterializedViewClass::DurableLocalMaterialization)
            .expect("seed durable view");
        override_subscriber(
            durable,
            ConsumerSurfaceKind::Shell,
            6,
            ReactiveSubscriberFreshness::Cached,
            ReactiveInvalidationCauseClass::AuthorityWrite,
        );
        durable.declared_parity_state = EpochParityState::DriftDetected;
        durable.declared_downgrade_label = ReactiveDowngradeLabel::YellowSurfacePartial;
        durable.open_gaps.push(ReactiveOpenGapEntry {
            gap_class: ReactiveOpenGapClass::SubscriberPending,
            summary: "shell subscriber lag awaiting refresh".to_owned(),
        });
    }
    let drift_inputs = base_inputs(
        "workspace-rust-service-0001",
        "reactive-state-corpus-drift-0001",
        "mono:1700000110",
        drift_views,
    );
    write_fixture(
        "drift_detected_stable",
        "posture:drift_detected",
        drift_inputs,
        default_reactive_state_inspection_hooks(),
    );

    // Awaiting resync: exportable support snapshot received a resync-required
    // terminal on the support surface.
    let mut awaiting_views = aligned_corpus("mono:1700000120");
    {
        let exportable = awaiting_views
            .iter_mut()
            .find(|v| v.view_class == ReactiveMaterializedViewClass::ExportableSnapshot)
            .expect("seed exportable view");
        override_subscriber(
            exportable,
            ConsumerSurfaceKind::Support,
            6,
            ReactiveSubscriberFreshness::Warming,
            ReactiveInvalidationCauseClass::ResyncRequired,
        );
        exportable.declared_parity_state = EpochParityState::AwaitingResync;
        exportable.declared_downgrade_label = ReactiveDowngradeLabel::YellowAuthoritySkew;
        exportable.open_gaps.push(ReactiveOpenGapEntry {
            gap_class: ReactiveOpenGapClass::DriftRecoveryManual,
            summary: "support surface awaiting manual resync".to_owned(),
        });
    }
    let awaiting_inputs = base_inputs(
        "workspace-rust-service-0001",
        "reactive-state-corpus-awaiting-0001",
        "mono:1700000120",
        awaiting_views,
    );
    write_fixture(
        "awaiting_resync_stable",
        "posture:awaiting_resync",
        awaiting_inputs,
        default_reactive_state_inspection_hooks(),
    );

    // Terminal unavailable: managed replicated review state is unavailable
    // on the review surface (managed-service outage).
    let mut terminal_views = aligned_corpus("mono:1700000130");
    {
        let managed = terminal_views
            .iter_mut()
            .find(|v| v.view_class == ReactiveMaterializedViewClass::ManagedReplicatedView)
            .expect("seed managed view");
        override_subscriber(
            managed,
            ConsumerSurfaceKind::Review,
            6,
            ReactiveSubscriberFreshness::Unavailable,
            ReactiveInvalidationCauseClass::ExternalChange,
        );
        managed.declared_parity_state = EpochParityState::TerminalUnavailable;
        managed.declared_downgrade_label = ReactiveDowngradeLabel::DegradedToAuthorityOnly;
        managed.open_gaps.push(ReactiveOpenGapEntry {
            gap_class: ReactiveOpenGapClass::ReplicationPending,
            summary: "managed replication backend offline".to_owned(),
        });
    }
    let terminal_inputs = base_inputs(
        "workspace-rust-service-0001",
        "reactive-state-corpus-terminal-0001",
        "mono:1700000130",
        terminal_views,
    );
    write_fixture(
        "terminal_unavailable_stable",
        "posture:terminal_unavailable",
        terminal_inputs,
        default_reactive_state_inspection_hooks(),
    );

    // Narrowed: same aligned corpus but the compare-epochs inspection hook
    // is unavailable.
    let narrowed_inputs = base_inputs(
        "workspace-rust-service-0001",
        "reactive-state-corpus-narrowed-0001",
        "mono:1700000140",
        aligned_corpus("mono:1700000140"),
    );
    let mut narrowed_hooks = default_reactive_state_inspection_hooks();
    for hook in &mut narrowed_hooks {
        if hook.hook_class == ReactiveStateInspectionHookClass::CompareEpochs {
            hook.available = false;
            hook.disclosure = "Compare-epochs unavailable on this posture.".to_owned();
        }
    }
    write_fixture(
        "missing_compare_hook_narrowed",
        "posture:missing_compare_hook",
        narrowed_inputs,
        narrowed_hooks,
    );
}
