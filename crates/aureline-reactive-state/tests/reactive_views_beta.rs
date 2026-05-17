//! Integration drill for the reactive-state and materialized-view
//! invalidation beta lane.
//!
//! Loads the checked-in corpus, re-proves the on-disk schema, doc,
//! report, and fixture presence, round-trips every case through serde,
//! and exercises the cross-surface epoch-parity acceptance contract.

use std::path::PathBuf;

use aureline_reactive_state::reactive_views::{
    current_materialized_view_corpus, current_materialized_view_fixture_refs,
    load_materialized_view_case, AuthorityLabel, DowngradeLabel, EpochParityState,
    InvalidationCauseClass, MaterializedViewClass, MaterializedViewCorpusEntry,
    MaterializedViewReport, OpenGapClass, ReactiveViewEvaluator, SubscriberFreshness, SurfaceKind,
    SupportExportPosture, REACTIVE_VIEW_CORPUS_DIR, REACTIVE_VIEW_CORPUS_MANIFEST_REF,
    REACTIVE_VIEW_DOC_REF, REACTIVE_VIEW_REPORT_RECORD_KIND, REACTIVE_VIEW_REPORT_REF,
    REACTIVE_VIEW_SCHEMA_REF, REQUIRED_SURFACE_KINDS, REQUIRED_VIEW_CLASSES,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn corpus_entries() -> Vec<MaterializedViewCorpusEntry> {
    current_materialized_view_corpus()
        .expect("checked-in reactive-views corpus must parse")
        .entries
}

#[test]
fn corpus_loads_and_validates() {
    let corpus = current_materialized_view_corpus().expect("checked-in corpus must parse");
    ReactiveViewEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
    assert!(!corpus.entries.is_empty(), "corpus must not be empty");
}

#[test]
fn corpus_covers_every_required_view_class() {
    let entries = corpus_entries();
    for view_class in REQUIRED_VIEW_CLASSES {
        assert!(
            entries
                .iter()
                .any(|entry| entry.case.view_class == view_class),
            "required view_class {} has no seeded case",
            view_class.as_str()
        );
    }
}

#[test]
fn corpus_declares_cross_surface_drift_or_resync_row() {
    let entries = corpus_entries();
    assert!(
        entries.iter().any(|entry| matches!(
            entry.case.parity_state,
            EpochParityState::DriftDetected | EpochParityState::AwaitingResync
        )),
        "corpus must seed at least one case with parity_state in {{drift_detected, awaiting_resync}}"
    );
}

#[test]
fn every_required_surface_appears_in_every_case() {
    for entry in corpus_entries() {
        for required in REQUIRED_SURFACE_KINDS {
            assert!(
                entry
                    .case
                    .subscriber_epochs
                    .iter()
                    .any(|s| s.surface_kind == required),
                "case {} must declare a subscriber_epoch for surface_kind = {}",
                entry.case.view_id,
                required.as_str()
            );
        }
    }
}

#[test]
fn fixture_files_exist_on_disk() {
    let root = repo_root();
    assert!(
        root.join(REACTIVE_VIEW_CORPUS_MANIFEST_REF).exists(),
        "manifest must exist on disk"
    );
    assert!(
        root.join(REACTIVE_VIEW_SCHEMA_REF).exists(),
        "schema must exist on disk"
    );
    assert!(
        root.join(REACTIVE_VIEW_DOC_REF).exists(),
        "doc must exist on disk"
    );
    assert!(
        root.join(REACTIVE_VIEW_REPORT_REF).exists(),
        "baseline report must exist on disk"
    );
    assert!(
        root.join(REACTIVE_VIEW_CORPUS_DIR).is_dir(),
        "corpus directory must exist on disk"
    );
    for fixture_ref in current_materialized_view_fixture_refs() {
        let path = root.join(fixture_ref);
        assert!(
            path.exists(),
            "fixture must exist on disk: {}",
            path.display()
        );
    }
}

#[test]
fn cases_round_trip_through_serde() {
    for entry in corpus_entries() {
        let yaml = serde_yaml::to_string(&entry.case).expect("case must serialize to yaml");
        let restored =
            load_materialized_view_case(&yaml).expect("case must round-trip through yaml");
        assert_eq!(restored, entry.case, "{} round-trip", entry.case.view_id);
    }
}

#[test]
fn report_preserves_cross_surface_matrix_truth() {
    let corpus = current_materialized_view_corpus().unwrap();
    let report: MaterializedViewReport = ReactiveViewEvaluator::new()
        .report(
            "report:reactive_views_beta:drill",
            "2026-05-16T10:00:00Z",
            &corpus,
        )
        .expect("report must build");
    assert_eq!(report.record_kind, REACTIVE_VIEW_REPORT_RECORD_KIND);
    assert!(report.is_export_safe(), "report must be export-safe");
    assert!(report.raw_private_material_excluded);
    assert!(report.ambient_authority_excluded);
    assert_eq!(report.matrix_rows.len(), corpus.entries.len());
    assert_eq!(
        report.view_class_summaries.len(),
        REQUIRED_VIEW_CLASSES.len()
    );
    let total: u32 = report
        .view_class_summaries
        .iter()
        .map(|s| s.case_count)
        .sum();
    assert_eq!(total as usize, corpus.entries.len());
    for summary in &report.view_class_summaries {
        assert_eq!(
            summary.case_count,
            summary.aligned_count
                + summary.drift_detected_count
                + summary.awaiting_resync_count
                + summary.terminal_unavailable_count,
            "view_class {} summary must reconcile parity counts",
            summary.view_class.as_str()
        );
    }
    for row in &report.matrix_rows {
        assert!(
            row.min_subscriber_epoch <= row.authority_epoch,
            "matrix row {} must not report a subscriber epoch above authority",
            row.view_id
        );
        assert!(
            row.max_subscriber_epoch <= row.authority_epoch,
            "matrix row {} must not report a subscriber epoch above authority",
            row.view_id
        );
    }
}

#[test]
fn aligned_rows_have_no_downgrade_or_open_gaps() {
    for entry in corpus_entries() {
        if entry.case.parity_state != EpochParityState::Aligned {
            continue;
        }
        assert_eq!(
            entry.case.downgrade_label,
            DowngradeLabel::None,
            "aligned case {} must declare downgrade_label = none",
            entry.case.view_id
        );
        assert!(
            entry
                .case
                .open_gaps
                .iter()
                .all(|gap| gap.gap_class == OpenGapClass::None),
            "aligned case {} must not record any non-none open_gap",
            entry.case.view_id
        );
    }
}

#[test]
fn non_aligned_rows_declare_a_downgrade_and_open_gap() {
    for entry in corpus_entries() {
        match entry.case.parity_state {
            EpochParityState::Aligned => continue,
            EpochParityState::DriftDetected
            | EpochParityState::AwaitingResync
            | EpochParityState::TerminalUnavailable => {
                assert_ne!(
                    entry.case.downgrade_label,
                    DowngradeLabel::None,
                    "non-aligned case {} must declare a non-none downgrade_label",
                    entry.case.view_id
                );
                assert!(
                    entry
                        .case
                        .open_gaps
                        .iter()
                        .any(|gap| gap.gap_class != OpenGapClass::None),
                    "non-aligned case {} must record at least one non-none open_gap",
                    entry.case.view_id
                );
            }
        }
    }
}

#[test]
fn exportable_classes_preserve_support_export_metadata() {
    for entry in corpus_entries() {
        let case = &entry.case;
        if !matches!(
            case.view_class,
            MaterializedViewClass::ExportableSnapshot
                | MaterializedViewClass::ManagedReplicatedView
        ) {
            continue;
        }
        assert!(
            matches!(
                case.support_export.posture,
                SupportExportPosture::MetadataSafeExport | SupportExportPosture::HeldRecord
            ),
            "exportable/managed case {} must declare metadata_safe_export or held_record",
            case.view_id
        );
        assert!(case.support_export.includes_view_class);
        assert!(case.support_export.includes_authority_label);
        assert!(case.support_export.includes_authority_epoch);
        assert!(case.support_export.includes_subscriber_epochs);
        assert!(case.support_export.raw_private_material_excluded);
        assert!(case.support_export.ambient_authority_excluded);
        assert!(case.support_export.preserves_user_authored_files);
    }
}

#[test]
fn drift_rows_record_an_epoch_lag() {
    for entry in corpus_entries() {
        if entry.case.parity_state != EpochParityState::DriftDetected {
            continue;
        }
        let authority = entry.case.authority_epoch;
        assert!(
            entry
                .case
                .subscriber_epochs
                .iter()
                .any(|s| s.observed_epoch < authority),
            "drift_detected case {} must record at least one subscriber.observed_epoch < authority_epoch",
            entry.case.view_id
        );
    }
}

#[test]
fn refuses_aligned_with_downgrade_label() {
    let mut corpus = current_materialized_view_corpus().unwrap();
    for entry in corpus.entries.iter_mut() {
        if entry.case.parity_state == EpochParityState::Aligned {
            entry.case.downgrade_label = DowngradeLabel::YellowSurfacePartial;
            break;
        }
    }
    let err = ReactiveViewEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("aligned with downgrade must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.outcome.aligned_must_not_carry_downgrade"));
}

#[test]
fn refuses_dropped_user_authored_files_preservation() {
    let mut corpus = current_materialized_view_corpus().unwrap();
    corpus.entries[0].case.safety.preserves_user_authored_files = false;
    let err = ReactiveViewEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("dropped user-files preservation must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.preserves_user_authored_files"));
}

#[test]
fn refuses_admitted_destructive_reset() {
    let mut corpus = current_materialized_view_corpus().unwrap();
    corpus.entries[0].case.safety.destructive_resets_present = true;
    let err = ReactiveViewEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("admitted destructive reset must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.destructive_resets_present"));
}

#[test]
fn refuses_corpus_missing_required_view_class() {
    let full = current_materialized_view_corpus().unwrap();
    let mut truncated = full.clone();
    truncated
        .entries
        .retain(|entry| entry.case.view_class != MaterializedViewClass::ManagedReplicatedView);
    let err = ReactiveViewEvaluator::new()
        .validate_corpus(&truncated)
        .expect_err("removing a required view_class must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "corpus.required_view_class_missing"));
}

#[test]
fn refuses_subscriber_epoch_above_authority() {
    let mut corpus = current_materialized_view_corpus().unwrap();
    let case = &mut corpus.entries[0].case;
    let surface = case.subscriber_epochs[0].surface_kind;
    case.subscriber_epochs[0].observed_epoch = case.authority_epoch + 1;
    let err = ReactiveViewEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("subscriber epoch above authority must fail");
    assert!(
        err.violations
            .iter()
            .any(|v| v.check_id == "case.subscriber.epoch_exceeds_authority"),
        "expected epoch_exceeds_authority violation for surface {}",
        surface.as_str()
    );
}

#[test]
fn vocab_tokens_match_schema_constants() {
    assert_eq!(MaterializedViewClass::EphemeralProjection.as_str(), "ephemeral_projection");
    assert_eq!(MaterializedViewClass::DurableLocalMaterialization.as_str(), "durable_local_materialization");
    assert_eq!(MaterializedViewClass::ExportableSnapshot.as_str(), "exportable_snapshot");
    assert_eq!(MaterializedViewClass::ManagedReplicatedView.as_str(), "managed_replicated_view");
    assert_eq!(AuthorityLabel::WorkspaceVfs.as_str(), "workspace_vfs");
    assert_eq!(SurfaceKind::Support.as_str(), "support");
    assert_eq!(EpochParityState::DriftDetected.as_str(), "drift_detected");
    assert_eq!(SubscriberFreshness::Authoritative.as_str(), "authoritative");
    assert_eq!(InvalidationCauseClass::ResyncRequired.as_str(), "resync_required");
    assert_eq!(SupportExportPosture::MetadataSafeExport.as_str(), "metadata_safe_export");
    assert_eq!(DowngradeLabel::YellowSurfacePartial.as_str(), "yellow_surface_partial");
    assert_eq!(OpenGapClass::ReplicationPending.as_str(), "replication_pending");
}
