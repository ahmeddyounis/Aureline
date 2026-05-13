use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use aureline_editor::{EditorStructuralSnapshot, StructuralEditorAnalyzer};
use aureline_language::{DerivedCueClass, ParseRequest};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct StructuralFixture {
    record_kind: String,
    schema_version: u32,
    cases: Vec<StructuralCase>,
}

#[derive(Debug, Deserialize)]
struct StructuralCase {
    name: String,
    language_id: String,
    source: String,
    expected: ExpectedStructuralState,
}

#[derive(Debug, Deserialize)]
struct ExpectedStructuralState {
    highlighting: String,
    folds: String,
    outline: String,
    provider: String,
    min_highlights: usize,
    highlight_kinds: Vec<String>,
    highlight_sources: Vec<String>,
    outline_labels: Vec<String>,
    outline_kinds: Vec<String>,
    fold_labels: Vec<String>,
    min_folds: usize,
    #[serde(default)]
    degraded_reasons: Vec<String>,
    raw_source_excluded: bool,
}

#[test]
fn structural_editor_alpha_fixture_set_stays_tree_sitter_sourced() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "editor_structural_alpha_fixture");
    assert_eq!(fixture.schema_version, 1);
    assert!(!fixture.cases.is_empty(), "fixture set must not be empty");

    let analyzer = StructuralEditorAnalyzer::with_default_registry();
    for case in fixture.cases {
        let snapshot = analyzer.analyze_text(request_for(&case), &case.source);

        assert_eq!(
            snapshot.record_kind,
            EditorStructuralSnapshot::RECORD_KIND,
            "record kind mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.structural_snapshot_schema_version,
            EditorStructuralSnapshot::SCHEMA_VERSION,
            "schema version mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.state.highlighting.as_str(),
            case.expected.highlighting,
            "highlighting state mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.state.folds.as_str(),
            case.expected.folds,
            "fold state mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.state.outline.as_str(),
            case.expected.outline,
            "outline state mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.state.provider_class.as_str(),
            case.expected.provider,
            "provider mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.state.raw_source_excluded, case.expected.raw_source_excluded,
            "raw-source export policy mismatch for {}",
            case.name
        );
        assert_eq!(
            snapshot.parse_session.export_policy.raw_source_excluded,
            case.expected.raw_source_excluded,
            "parse-session export policy mismatch for {}",
            case.name
        );
        assert!(
            snapshot
                .parse_session
                .requested_derived_cue_classes
                .contains(&DerivedCueClass::Outline),
            "outline cue should be requested for {}",
            case.name
        );

        assert!(
            snapshot.highlights.len() >= case.expected.min_highlights,
            "too few highlight spans for {}: got {}",
            case.name,
            snapshot.highlights.len()
        );
        assert!(
            snapshot.folds.len() >= case.expected.min_folds,
            "too few folds for {}: got {}",
            case.name,
            snapshot.folds.len()
        );

        let highlight_kinds: BTreeSet<&str> = snapshot
            .highlights
            .iter()
            .map(|span| span.kind.as_str())
            .collect();
        for expected_kind in &case.expected.highlight_kinds {
            assert!(
                highlight_kinds.contains(expected_kind.as_str()),
                "missing highlight kind {expected_kind} for {} in {:?}",
                case.name,
                highlight_kinds
            );
        }

        let highlight_sources: BTreeSet<&str> = snapshot
            .highlights
            .iter()
            .map(|span| span.source_class.as_str())
            .collect();
        for expected_source in &case.expected.highlight_sources {
            assert!(
                highlight_sources.contains(expected_source.as_str()),
                "missing highlight source {expected_source} for {}",
                case.name
            );
        }

        let outline_labels: BTreeSet<&str> = snapshot
            .outline
            .iter()
            .map(|node| node.label.as_str())
            .collect();
        for expected_label in &case.expected.outline_labels {
            assert!(
                outline_labels.contains(expected_label.as_str()),
                "missing outline label {expected_label} for {} in {:?}",
                case.name,
                outline_labels
            );
        }

        let outline_kinds: BTreeSet<&str> = snapshot
            .outline
            .iter()
            .map(|node| node.kind.as_str())
            .collect();
        for expected_kind in &case.expected.outline_kinds {
            assert!(
                outline_kinds.contains(expected_kind.as_str()),
                "missing outline kind {expected_kind} for {}",
                case.name
            );
        }

        let fold_labels: BTreeSet<&str> = snapshot
            .folds
            .iter()
            .map(|fold| fold.label.as_str())
            .collect();
        for expected_label in &case.expected.fold_labels {
            assert!(
                fold_labels.contains(expected_label.as_str()),
                "missing fold label {expected_label} for {} in {:?}",
                case.name,
                fold_labels
            );
        }

        let degraded_reasons: BTreeSet<&str> = snapshot
            .state
            .degraded_reason_classes
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        for expected_reason in &case.expected.degraded_reasons {
            assert!(
                degraded_reasons.contains(expected_reason.as_str()),
                "missing degraded reason {expected_reason} for {}",
                case.name
            );
        }

        for fold in &snapshot.folds {
            assert!(
                !fold.keyboard_toggle_command.trim().is_empty(),
                "fold keyboard command missing for {}",
                case.name
            );
            assert!(
                !fold.accessibility_label.trim().is_empty(),
                "fold accessibility label missing for {}",
                case.name
            );
        }
        for node in &snapshot.outline {
            assert!(
                !node.accessibility_label.trim().is_empty(),
                "outline accessibility label missing for {}",
                case.name
            );
        }
        for span in &snapshot.highlights {
            assert!(
                !span.accessibility_label.trim().is_empty(),
                "highlight accessibility label missing for {}",
                case.name
            );
        }

        let serialized =
            serde_json::to_string(&snapshot).expect("structural snapshot should serialize");
        assert!(
            !serialized.contains(&case.source),
            "serialized snapshot should not embed raw source for {}",
            case.name
        );
    }
}

fn request_for(case: &StructuralCase) -> ParseRequest {
    let safe_name = case.name.replace('_', "-");
    ParseRequest::foreground_file(
        format!("parse-session:test:structural:{safe_name}"),
        format!("doc:test:structural:{safe_name}"),
        format!("buffer:test:structural:{safe_name}"),
        1,
        &case.language_id,
        "2026-05-13T00:00:00Z",
    )
}

fn load_fixture() -> StructuralFixture {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/structural_editor_alpha/launch_wedge_structural_cases.json");
    let payload = fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
