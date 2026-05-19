use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_editor::{
    build_beta_orientation_aid_state_record, BetaOrientationAidInput, FoldSummaryStateRecord,
    MarkerFamilyClass, MultiCursorModePosture, OrientationAidAvailabilityClass,
    OrientationAidStateRecord, OrientationSurfaceClass, OverviewAidKindClass,
    FOLD_SUMMARY_STATE_SCHEMA_VERSION, ORIENTATION_AID_STATE_SCHEMA_VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture_dir() -> PathBuf {
    repo_root().join("fixtures/editor/m3/orientation_aids")
}

fn load_fixture(name: &str) -> OrientationAidStateRecord {
    let path = fixture_dir().join(name);
    let raw = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&raw).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}

fn assert_record_passes_beta_invariants(record: &OrientationAidStateRecord) {
    assert_eq!(record.record_kind, OrientationAidStateRecord::RECORD_KIND);
    assert_eq!(record.schema_version, ORIENTATION_AID_STATE_SCHEMA_VERSION);
    assert!(record.multi_cursor_attribution_is_visible());
    assert!(record.fold_summaries_preserve_hidden_state());
    assert!(record.breadcrumb_preserves_continuity());
    assert!(record.degraded_aids_have_alternate_paths());
    assert!(record.marker_vocabulary_is_consistent());
    assert!(record.degraded_mode_labeling_is_explicit());

    for fold in &record.fold_summaries {
        assert_eq!(fold.record_kind, FoldSummaryStateRecord::RECORD_KIND);
        assert_eq!(fold.schema_version, FOLD_SUMMARY_STATE_SCHEMA_VERSION);
    }
}

#[test]
fn source_editor_fixture_satisfies_beta_invariants() {
    let record = load_fixture("source_editor_beta.json");
    assert_eq!(record.surface_class, OrientationSurfaceClass::EditorSource);
    assert_eq!(
        record.multi_cursor.mode_posture,
        MultiCursorModePosture::MultipleCarets
    );
    assert!(record.multi_cursor.caret_count >= 2);

    let families: BTreeSet<_> = record.shared_marker_families.iter().copied().collect();
    for required in [
        MarkerFamilyClass::DiagnosticError,
        MarkerFamilyClass::MergeConflict,
        MarkerFamilyClass::StagedHunk,
        MarkerFamilyClass::SearchHit,
        MarkerFamilyClass::ReviewThread,
        MarkerFamilyClass::TrustOrPolicyWarning,
        MarkerFamilyClass::FoldHiddenState,
    ] {
        assert!(
            families.contains(&required),
            "shared marker vocabulary must include {}",
            required.as_str()
        );
    }

    let aid_kinds: BTreeSet<_> = record
        .overview_aids
        .iter()
        .map(|aid| aid.aid_kind)
        .collect();
    assert!(aid_kinds.contains(&OverviewAidKindClass::Minimap));
    assert!(aid_kinds.contains(&OverviewAidKindClass::OverviewRuler));

    assert_record_passes_beta_invariants(&record);
}

#[test]
fn diff_surface_fixture_attributes_column_selection_and_staged_hunks() {
    let record = load_fixture("diff_surface_beta.json");
    assert_eq!(record.surface_class, OrientationSurfaceClass::EditorDiff);
    assert_eq!(
        record.multi_cursor.mode_posture,
        MultiCursorModePosture::ColumnSelection
    );
    assert!(record.multi_cursor.column_mode_active);

    let hidden_families: BTreeSet<_> = record
        .fold_summaries
        .iter()
        .flat_map(|fold| fold.hidden_marker_counts.iter().map(|count| count.family))
        .collect();
    assert!(hidden_families.contains(&MarkerFamilyClass::StagedHunk));

    assert_record_passes_beta_invariants(&record);
}

#[test]
fn review_surface_fixture_keeps_review_thread_family_first_class() {
    let record = load_fixture("review_surface_beta.json");
    assert_eq!(record.surface_class, OrientationSurfaceClass::ReviewThread);

    assert!(record
        .gutter
        .marker_families
        .contains(&MarkerFamilyClass::ReviewThread));
    for aid in &record.overview_aids {
        assert!(aid
            .marker_families
            .contains(&MarkerFamilyClass::ReviewThread));
    }

    assert_record_passes_beta_invariants(&record);
}

#[test]
fn large_file_degraded_fixture_labels_disabled_state_and_alternate_routes() {
    let record = load_fixture("large_file_degraded_beta.json");

    assert_eq!(
        record.gutter.availability,
        OrientationAidAvailabilityClass::DisabledLargeFile
    );
    for aid in &record.overview_aids {
        assert_eq!(
            aid.availability,
            OrientationAidAvailabilityClass::DisabledLargeFile
        );
        assert!(!aid.replacement_route_refs.is_empty());
    }
    assert!(record
        .degraded_mode_classes
        .contains(&OrientationAidAvailabilityClass::DisabledLargeFile));

    assert_record_passes_beta_invariants(&record);
}

#[test]
fn constructed_beta_record_matches_source_editor_fixture_for_default_posture() {
    let constructed = build_beta_orientation_aid_state_record(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:beta:source-editor:0001".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:orders/src/controller.ts".into(),
        surface_ref: "surface:editor.source.beta".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });

    let fixture = load_fixture("source_editor_beta.json");
    assert_eq!(constructed, fixture);

    assert_record_passes_beta_invariants(&constructed);
}

#[test]
fn constructed_beta_record_for_large_file_marks_aids_disabled() {
    let constructed = build_beta_orientation_aid_state_record(BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:beta:large-file:test".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:logs/giant.log".into(),
        surface_ref: "surface:editor.source.beta".into(),
        large_file_mode: true,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    });

    assert_eq!(
        constructed.gutter.availability,
        OrientationAidAvailabilityClass::DisabledLargeFile
    );
    for aid in &constructed.overview_aids {
        assert_eq!(
            aid.availability,
            OrientationAidAvailabilityClass::DisabledLargeFile
        );
    }
    assert!(constructed
        .degraded_mode_classes
        .contains(&OrientationAidAvailabilityClass::DisabledLargeFile));
    assert!(constructed.degraded_aids_have_alternate_paths());
    assert!(constructed.marker_vocabulary_is_consistent());
}

#[test]
fn constructed_beta_record_resolves_each_degraded_posture_to_distinct_class() {
    let postures: [(BetaOrientationAidInput, OrientationAidAvailabilityClass); 6] = [
        (
            posture_with(|input| input.low_resource_mode = true),
            OrientationAidAvailabilityClass::DisabledLowResource,
        ),
        (
            posture_with(|input| input.reduced_motion = true),
            OrientationAidAvailabilityClass::DisabledReducedMotion,
        ),
        (
            posture_with(|input| input.high_contrast = true),
            OrientationAidAvailabilityClass::DisabledHighContrast,
        ),
        (
            posture_with(|input| input.battery_saver = true),
            OrientationAidAvailabilityClass::DisabledBatterySaver,
        ),
        (
            posture_with(|input| input.restricted_mode = true),
            OrientationAidAvailabilityClass::DisabledRestrictedMode,
        ),
        (
            posture_with(|input| input.large_file_mode = true),
            OrientationAidAvailabilityClass::DisabledLargeFile,
        ),
    ];

    for (input, expected) in postures {
        let record = build_beta_orientation_aid_state_record(input);
        assert_eq!(record.gutter.availability, expected);
        for aid in &record.overview_aids {
            assert_eq!(aid.availability, expected);
        }
        assert!(record.degraded_aids_have_alternate_paths());
        assert!(record.marker_vocabulary_is_consistent());
        assert!(record.degraded_mode_labeling_is_explicit());
    }
}

fn posture_with(mutate: impl FnOnce(&mut BetaOrientationAidInput)) -> BetaOrientationAidInput {
    let mut input = BetaOrientationAidInput {
        orientation_state_id: "orientation-aid-state:beta:posture:test".into(),
        surface_class: OrientationSurfaceClass::EditorSource,
        document_ref: "document:posture:test".into(),
        surface_ref: "surface:editor.source.beta".into(),
        large_file_mode: false,
        low_resource_mode: false,
        reduced_motion: false,
        high_contrast: false,
        battery_saver: false,
        restricted_mode: false,
    };
    mutate(&mut input);
    input
}
