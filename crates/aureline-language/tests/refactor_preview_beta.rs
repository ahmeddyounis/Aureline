use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use aureline_language::refactor_preview::{
    current_refactor_preview_corpus, current_refactor_preview_fixture_refs, RefactorClass,
    RefactorCorpusRowState, RefactorFallbackReasonClass, RefactorPreviewEvaluator,
    RefactorPreviewRecord, RefactorRuntimeConditionClass, RefactorValidationResult,
};

#[test]
fn corpus_validates_and_covers_refactor_classes_and_conditions() {
    let corpus = current_refactor_preview_corpus().expect("corpus parses");
    let report = RefactorPreviewEvaluator::new()
        .report(
            "language:refactor-preview:report:test",
            "2026-05-18T10:00:00Z",
            &corpus,
        )
        .expect("corpus validates");

    assert!(report.is_export_safe());
    assert_eq!(
        report.aggregate_counts.total_rows,
        corpus.entries.len() as u32
    );
    assert!(report.aggregate_counts.green_rows >= 2);
    assert!(report.aggregate_counts.downgraded_rows >= 2);
    assert!(report.aggregate_counts.unsupported_rows >= 1);
    assert!(report.aggregate_counts.fallback_labeled_rows >= 4);
    assert!(report.aggregate_counts.grouped_rollback_rows >= 3);

    let classes = corpus
        .entries
        .iter()
        .map(|entry| entry.preview.refactor_class)
        .collect::<BTreeSet<_>>();
    for required in [
        RefactorClass::RenameSymbol,
        RefactorClass::ExtractFunction,
        RefactorClass::MoveSymbol,
        RefactorClass::UpdateImports,
        RefactorClass::CrossFileSignatureChange,
    ] {
        assert!(classes.contains(&required), "missing {required:?}");
    }

    let conditions = corpus
        .entries
        .iter()
        .map(|entry| entry.preview.runtime_condition_class)
        .collect::<BTreeSet<_>>();
    for required in [
        RefactorRuntimeConditionClass::WarmSemantic,
        RefactorRuntimeConditionClass::PartialIndex,
        RefactorRuntimeConditionClass::CachedSemantic,
        RefactorRuntimeConditionClass::RemoteAssisted,
    ] {
        assert!(conditions.contains(&required), "missing {required:?}");
    }
}

#[test]
fn fallback_labels_are_required_for_degraded_sources() {
    let corpus = current_refactor_preview_corpus().expect("corpus parses");
    for entry in corpus.entries {
        if entry.preview.requires_fallback_label() {
            assert!(
                entry.preview.fallback_label.is_actionable(),
                "{} needs an actionable fallback label",
                entry.preview.preview_id
            );
        }
        if entry
            .preview
            .semantic_source_class
            .requires_fallback_label()
            && !entry.preview.semantic_source_class.can_back_green_claim()
        {
            assert_ne!(
                entry.validation_result.corpus_row_state,
                RefactorCorpusRowState::Green,
                "non-current source must not be green unless explicitly qualified"
            );
        }
    }
}

#[test]
fn green_remote_assisted_row_is_labeled_but_semantic() {
    let corpus = current_refactor_preview_corpus().expect("corpus parses");
    let remote = corpus
        .entries
        .iter()
        .find(|entry| {
            entry.preview.runtime_condition_class == RefactorRuntimeConditionClass::RemoteAssisted
        })
        .expect("remote-assisted row exists");

    assert_eq!(
        remote.validation_result.corpus_row_state,
        RefactorCorpusRowState::Green
    );
    assert!(remote.preview.fallback_label.required);
    assert!(remote
        .preview
        .fallback_label
        .reason_classes
        .contains(&RefactorFallbackReasonClass::RemoteAssistedSemantic));
    assert!(remote.preview.rollback_handle.is_grouped_rollback_ready());
}

#[test]
fn records_round_trip_through_json() {
    let corpus = current_refactor_preview_corpus().expect("corpus parses");
    for entry in corpus.entries {
        let preview_json = serde_json::to_string(&entry.preview).expect("preview serializes");
        let preview: RefactorPreviewRecord =
            serde_json::from_str(&preview_json).expect("preview deserializes");
        assert_eq!(preview, entry.preview);

        let validation_json =
            serde_json::to_string(&entry.validation_result).expect("validation serializes");
        let validation: RefactorValidationResult =
            serde_json::from_str(&validation_json).expect("validation deserializes");
        assert_eq!(validation, entry.validation_result);
    }
}

#[test]
fn checked_in_fixture_refs_are_loaded() {
    let fixture_refs = current_refactor_preview_fixture_refs().collect::<Vec<_>>();
    assert_eq!(fixture_refs.len(), 6);

    let root = repo_root();
    for fixture_ref in fixture_refs {
        assert!(
            root.join(fixture_ref).exists(),
            "fixture ref should exist: {fixture_ref}"
        );
    }
}

#[test]
fn schema_files_parse_as_json() {
    let root = repo_root();
    for schema_ref in [
        "schemas/language/refactor_preview.schema.json",
        "schemas/language/refactor_validation_result.schema.json",
    ] {
        let payload = fs::read_to_string(root.join(schema_ref))
            .unwrap_or_else(|err| panic!("{schema_ref}: {err}"));
        let parsed: serde_json::Value =
            serde_json::from_str(&payload).unwrap_or_else(|err| panic!("{schema_ref}: {err}"));
        assert_eq!(
            parsed
                .get("$schema")
                .and_then(serde_json::Value::as_str)
                .expect("schema declares draft"),
            "https://json-schema.org/draft/2020-12/schema"
        );
    }
}

#[test]
fn validator_flags_missing_required_fallback_label() {
    let mut corpus = current_refactor_preview_corpus().expect("corpus parses");
    let row = corpus
        .entries
        .iter_mut()
        .find(|entry| entry.preview.requires_fallback_label())
        .expect("degraded row exists");
    row.preview.fallback_label.required = false;
    row.preview.fallback_label.reason_classes.clear();

    let report = RefactorPreviewEvaluator::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| { defect.check_id == "fallback_label.required_but_not_actionable" }));
}

#[test]
fn validator_flags_missing_grouped_rollback_for_applyable_multi_file_preview() {
    let mut corpus = current_refactor_preview_corpus().expect("corpus parses");
    let row = corpus
        .entries
        .iter_mut()
        .find(|entry| entry.preview.rollback_handle.is_grouped_rollback_ready())
        .expect("grouped rollback row exists");
    row.preview.rollback_handle.rollback_handle_ref = None;
    row.preview.rollback_handle.local_history_group_ref = None;
    row.preview.rollback_handle.mutation_journal_ref = None;

    let report = RefactorPreviewEvaluator::new().validate(&corpus);
    assert!(report
        .defects
        .iter()
        .any(|defect| defect.check_id == "rollback.grouped_handle_missing"));
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}
