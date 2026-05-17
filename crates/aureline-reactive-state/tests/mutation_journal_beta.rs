//! Integration drill for the mutation-journal beta lane.
//!
//! Loads the checked-in corpus, re-proves the on-disk schema, doc,
//! report, and fixture presence, round-trips every case through serde,
//! and exercises the grouped-write attribution and replayability
//! acceptance contract.

use std::path::PathBuf;

use aureline_reactive_state::mutation_journal::{
    current_mutation_journal_corpus, current_mutation_journal_fixture_refs,
    load_mutation_journal_case, ActorClass, AttributionState, AuthorityClass, ConsumerSurface,
    DowngradeLabel, EntryKind, MutationJournalCorpusEntry, MutationJournalEvaluator,
    MutationJournalReport, OpenGapClass, RecoveryClass, ReplayabilityState, SourceLane,
    MUTATION_JOURNAL_CORPUS_DIR, MUTATION_JOURNAL_CORPUS_MANIFEST_REF, MUTATION_JOURNAL_DOC_REF,
    MUTATION_JOURNAL_REPORT_RECORD_KIND, MUTATION_JOURNAL_REPORT_REF, MUTATION_JOURNAL_SCHEMA_REF,
    REQUIRED_CONSUMER_SURFACES, REQUIRED_RECOVERY_CLASSES, REQUIRED_SOURCE_LANES,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn corpus_entries() -> Vec<MutationJournalCorpusEntry> {
    current_mutation_journal_corpus()
        .expect("checked-in mutation-journal corpus must parse")
        .entries
}

#[test]
fn corpus_loads_and_validates() {
    let corpus = current_mutation_journal_corpus().expect("checked-in corpus must parse");
    MutationJournalEvaluator::new()
        .validate_corpus(&corpus)
        .expect("checked-in corpus must validate");
    assert!(!corpus.entries.is_empty(), "corpus must not be empty");
}

#[test]
fn corpus_covers_every_required_source_lane() {
    let entries = corpus_entries();
    for lane in REQUIRED_SOURCE_LANES {
        assert!(
            entries.iter().any(|entry| entry.case.source_lane == lane),
            "required source_lane {} has no seeded case",
            lane.as_str()
        );
    }
}

#[test]
fn corpus_covers_every_required_recovery_class() {
    let entries = corpus_entries();
    for cls in REQUIRED_RECOVERY_CLASSES {
        assert!(
            entries
                .iter()
                .any(|entry| entry.case.recovery_class == cls),
            "required recovery_class {} has no seeded case",
            cls.as_str()
        );
    }
}

#[test]
fn corpus_covers_every_required_consumer_surface() {
    let entries = corpus_entries();
    for surface in REQUIRED_CONSUMER_SURFACES {
        assert!(
            entries
                .iter()
                .any(|entry| entry.case.consumer_surface == surface),
            "required consumer_surface {} has no seeded case",
            surface.as_str()
        );
    }
}

#[test]
fn corpus_seeds_attribution_gap_rows() {
    let entries = corpus_entries();
    assert!(
        entries
            .iter()
            .any(|entry| entry.case.attribution_state == AttributionState::PartiallyAttributed),
        "corpus must seed a partially_attributed case"
    );
    assert!(
        entries
            .iter()
            .any(|entry| entry.case.attribution_state == AttributionState::Unattributed),
        "corpus must seed an unattributed case"
    );
}

#[test]
fn fixture_files_exist_on_disk() {
    let root = repo_root();
    assert!(
        root.join(MUTATION_JOURNAL_CORPUS_MANIFEST_REF).exists(),
        "manifest must exist on disk"
    );
    assert!(
        root.join(MUTATION_JOURNAL_SCHEMA_REF).exists(),
        "schema must exist on disk"
    );
    assert!(
        root.join(MUTATION_JOURNAL_DOC_REF).exists(),
        "doc must exist on disk"
    );
    assert!(
        root.join(MUTATION_JOURNAL_REPORT_REF).exists(),
        "baseline report must exist on disk"
    );
    assert!(
        root.join(MUTATION_JOURNAL_CORPUS_DIR).is_dir(),
        "corpus directory must exist on disk"
    );
    for fixture_ref in current_mutation_journal_fixture_refs() {
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
            load_mutation_journal_case(&yaml).expect("case must round-trip through yaml");
        assert_eq!(restored, entry.case, "{} round-trip", entry.case.entry_id);
    }
}

#[test]
fn report_preserves_matrix_and_summary_truth() {
    let corpus = current_mutation_journal_corpus().unwrap();
    let report: MutationJournalReport = MutationJournalEvaluator::new()
        .report(
            "report:mutation_journal_beta:drill",
            "2026-05-16T10:30:00Z",
            &corpus,
        )
        .expect("report must build");
    assert_eq!(report.record_kind, MUTATION_JOURNAL_REPORT_RECORD_KIND);
    assert!(report.is_export_safe(), "report must be export-safe");
    assert!(report.raw_payload_excluded);
    assert!(report.raw_private_material_excluded);
    assert!(report.ambient_authority_excluded);
    assert_eq!(report.matrix_rows.len(), corpus.entries.len());
    assert_eq!(
        report.source_lane_summaries.len(),
        REQUIRED_SOURCE_LANES.len()
    );
    assert_eq!(
        report.recovery_class_summaries.len(),
        REQUIRED_RECOVERY_CLASSES.len()
    );

    let total_lane_cases: u32 = report
        .source_lane_summaries
        .iter()
        .map(|s| s.case_count)
        .sum();
    let required_lane_total: u32 = corpus
        .entries
        .iter()
        .filter(|entry| REQUIRED_SOURCE_LANES.contains(&entry.case.source_lane))
        .count() as u32;
    assert_eq!(total_lane_cases, required_lane_total);

    for summary in &report.source_lane_summaries {
        assert_eq!(
            summary.case_count,
            summary.attributed_count
                + summary.partially_attributed_count
                + summary.unattributed_count,
            "source_lane {} summary must reconcile attribution counts",
            summary.source_lane.as_str()
        );
    }
    for summary in &report.recovery_class_summaries {
        assert_eq!(
            summary.case_count,
            summary.replay_ready_count
                + summary.replay_with_compensation_count
                + summary.regenerate_only_count
                + summary.requires_manual_inspection_count,
            "recovery_class {} summary must reconcile replayability counts",
            summary.recovery_class.as_str()
        );
    }
}

#[test]
fn aligned_rows_have_no_downgrade_or_open_gaps() {
    for entry in corpus_entries() {
        if entry.case.downgrade_label != DowngradeLabel::None {
            continue;
        }
        assert_eq!(
            entry.case.attribution_state,
            AttributionState::Attributed,
            "aligned case {} must be attributed",
            entry.case.entry_id
        );
        assert!(
            entry
                .case
                .open_gaps
                .iter()
                .all(|gap| gap.gap_class == OpenGapClass::None),
            "aligned case {} must not record any non-none open_gap",
            entry.case.entry_id
        );
    }
}

#[test]
fn non_aligned_rows_declare_a_downgrade_and_open_gap() {
    for entry in corpus_entries() {
        if entry.case.downgrade_label == DowngradeLabel::None {
            continue;
        }
        assert!(
            entry
                .case
                .open_gaps
                .iter()
                .any(|gap| gap.gap_class != OpenGapClass::None),
            "non-aligned case {} must record at least one non-none open_gap",
            entry.case.entry_id
        );
    }
}

#[test]
fn support_export_projections_preserve_audit_fields() {
    for entry in corpus_entries() {
        let s = &entry.case.support_export;
        assert!(s.includes_entry_id);
        assert!(s.includes_source_lane);
        assert!(s.includes_actor_class);
        assert!(s.includes_authority_class);
        assert!(s.includes_recovery_class);
        assert!(s.includes_replayability_state);
        assert!(s.includes_affected_paths);
        assert!(s.raw_payload_excluded);
        assert!(s.raw_private_material_excluded);
        assert!(s.ambient_authority_excluded);
        assert!(s.preserves_user_authored_files);
    }
}

#[test]
fn multi_file_writes_declare_multiple_affected_paths() {
    for entry in corpus_entries() {
        if entry.case.entry_kind != EntryKind::MultiFileWrite {
            continue;
        }
        assert!(
            entry.case.affected_paths.len() >= 2,
            "multi_file_write case {} must declare at least two affected_paths",
            entry.case.entry_id
        );
        assert!(
            entry.case.group_size >= 2,
            "multi_file_write case {} must declare group_size >= 2",
            entry.case.entry_id
        );
    }
}

#[test]
fn refuses_aligned_with_downgrade_label() {
    let mut corpus = current_mutation_journal_corpus().unwrap();
    for entry in corpus.entries.iter_mut() {
        if entry.case.downgrade_label == DowngradeLabel::None {
            entry.case.downgrade_label = DowngradeLabel::YellowPartialAttribution;
            break;
        }
    }
    let err = MutationJournalEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("aligned with downgrade must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.outcome.yellow_partial_requires_partial"));
}

#[test]
fn refuses_dropped_user_authored_files_preservation() {
    let mut corpus = current_mutation_journal_corpus().unwrap();
    corpus.entries[0].case.safety.preserves_user_authored_files = false;
    let err = MutationJournalEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("dropped user-files preservation must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.preserves_user_authored_files"));
}

#[test]
fn refuses_admitted_destructive_reset() {
    let mut corpus = current_mutation_journal_corpus().unwrap();
    corpus.entries[0].case.safety.destructive_resets_present = true;
    let err = MutationJournalEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("admitted destructive reset must fail validation");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.safety.destructive_resets_present"));
}

#[test]
fn refuses_corpus_missing_required_recovery_class() {
    let full = current_mutation_journal_corpus().unwrap();
    let mut truncated = full.clone();
    truncated
        .entries
        .retain(|entry| entry.case.recovery_class != RecoveryClass::CheckpointRestore);
    let err = MutationJournalEvaluator::new()
        .validate_corpus(&truncated)
        .expect_err("removing a required recovery_class must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "corpus.required_recovery_class_missing"));
}

#[test]
fn refuses_unattributed_with_known_actor() {
    let mut corpus = current_mutation_journal_corpus().unwrap();
    for entry in corpus.entries.iter_mut() {
        if entry.case.attribution_state == AttributionState::Unattributed {
            entry.case.actor_class = ActorClass::AiAgent;
            break;
        }
    }
    let err = MutationJournalEvaluator::new()
        .validate_corpus(&corpus)
        .expect_err("unattributed with known actor must fail");
    assert!(err
        .violations
        .iter()
        .any(|v| v.check_id == "case.attribution.unattributed_requires_unknown_actor"));
}

#[test]
fn vocab_tokens_match_schema_constants() {
    assert_eq!(SourceLane::AiAssistant.as_str(), "ai_assistant");
    assert_eq!(SourceLane::InteractiveRefactor.as_str(), "interactive_refactor");
    assert_eq!(SourceLane::AutomatedTooling.as_str(), "automated_tooling");
    assert_eq!(ActorClass::AiAgent.as_str(), "ai_agent");
    assert_eq!(ActorClass::UnknownActor.as_str(), "unknown_actor");
    assert_eq!(AuthorityClass::BufferEditor.as_str(), "buffer_editor");
    assert_eq!(EntryKind::MultiFileWrite.as_str(), "multi_file_write");
    assert_eq!(RecoveryClass::ExactUndo.as_str(), "exact_undo");
    assert_eq!(RecoveryClass::Compensation.as_str(), "compensation");
    assert_eq!(RecoveryClass::Regeneration.as_str(), "regeneration");
    assert_eq!(RecoveryClass::CheckpointRestore.as_str(), "checkpoint_restore");
    assert_eq!(
        RecoveryClass::RequiresUserResolution.as_str(),
        "requires_user_resolution"
    );
    assert_eq!(AttributionState::Attributed.as_str(), "attributed");
    assert_eq!(
        AttributionState::PartiallyAttributed.as_str(),
        "partially_attributed"
    );
    assert_eq!(ReplayabilityState::ReplayReady.as_str(), "replay_ready");
    assert_eq!(
        ReplayabilityState::RequiresManualInspection.as_str(),
        "requires_manual_inspection"
    );
    assert_eq!(ConsumerSurface::IncidentPacket.as_str(), "incident_packet");
    assert_eq!(ConsumerSurface::CrashReport.as_str(), "crash_report");
    assert_eq!(DowngradeLabel::RedBlocksBetaRow.as_str(), "red_blocks_beta_row");
    assert_eq!(
        DowngradeLabel::DegradedToCheckpointRestoreOnly.as_str(),
        "degraded_to_checkpoint_restore_only"
    );
    assert_eq!(OpenGapClass::AttributionPending.as_str(), "attribution_pending");
    assert_eq!(
        OpenGapClass::CompensationClassPending.as_str(),
        "compensation_class_pending"
    );
}
