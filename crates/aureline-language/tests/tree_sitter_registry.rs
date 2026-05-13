use std::path::Path;

use aureline_language::{
    default_launch_grammar_registry, FailureReasonClass, GrammarRegistryRecord,
    ParseLifecycleStateClass, ParseQualityClass, ParseRequest, ParseSessionRecord,
    ParserRuntimeStateClass, TreeSitterParserSupervisor,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct RegistryFixture {
    record_kind: String,
    schema_version: u32,
    expected_language_ids: Vec<String>,
    expected_extension_routes: Vec<ExtensionRoute>,
    parse_cases: Vec<ParseCase>,
    missing_grammar_case: MissingGrammarCase,
}

#[derive(Debug, Deserialize)]
struct ExtensionRoute {
    extension: String,
    language_id: String,
}

#[derive(Debug, Deserialize)]
struct ParseCase {
    language_id: String,
    source: String,
    expected_lifecycle: String,
    expected_quality: String,
}

#[derive(Debug, Deserialize)]
struct MissingGrammarCase {
    language_id: String,
    expected_lifecycle: String,
    expected_quality: String,
    expected_failure: String,
}

#[test]
fn launch_grammars_resolve_and_load_through_one_registry() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "tree_sitter_registry_case");
    assert_eq!(fixture.schema_version, 1);

    let registry = default_launch_grammar_registry();
    let record = registry.registry_record("2026-05-13T00:00:00Z");
    assert_eq!(
        record.record_kind,
        GrammarRegistryRecord::RECORD_KIND,
        "registry record kind must stay stable"
    );
    assert_eq!(record.entries.len(), fixture.expected_language_ids.len());

    for expected_language_id in &fixture.expected_language_ids {
        let descriptor = registry
            .require_language_id(expected_language_id)
            .unwrap_or_else(|err| panic!("resolve {expected_language_id}: {err}"));
        let language = descriptor.load_language();
        assert!(
            language.version() >= tree_sitter::MIN_COMPATIBLE_LANGUAGE_VERSION,
            "{expected_language_id} ABI should be accepted by the runtime"
        );
        assert!(
            language.node_kind_count() > 0,
            "{expected_language_id} grammar should report node kinds"
        );
    }

    for route in &fixture.expected_extension_routes {
        let descriptor = registry
            .require_file_extension(&route.extension)
            .unwrap_or_else(|err| panic!("resolve extension {}: {err}", route.extension));
        assert_eq!(descriptor.language_id, route.language_id);
    }
}

#[test]
fn parser_lifecycle_publishes_completed_partial_missing_and_shutdown_states() {
    let fixture = load_fixture();
    let supervisor = TreeSitterParserSupervisor::with_default_registry();

    for case in &fixture.parse_cases {
        let output = supervisor.parse_text(request_for(&case.language_id), &case.source);
        assert_eq!(
            output
                .record
                .parse_state
                .parse_lifecycle_state_class
                .as_str(),
            case.expected_lifecycle,
            "lifecycle mismatch for {}",
            case.language_id
        );
        assert_eq!(
            output.record.parse_state.parse_quality_class.as_str(),
            case.expected_quality,
            "quality mismatch for {}",
            case.language_id
        );
        assert!(
            output.tree().is_some(),
            "tree missing for {}",
            case.language_id
        );
        assert!(
            output.record.has_current_full_tree(),
            "{} should publish a current full tree",
            case.language_id
        );

        let serialized =
            serde_json::to_string(&output.record).expect("parse-session record serializes");
        let round_trip: ParseSessionRecord =
            serde_json::from_str(&serialized).expect("parse-session record deserializes");
        assert_eq!(round_trip, output.record);
    }

    let partial = supervisor.parse_text(
        request_for("language:typescript"),
        "export function broken( {",
    );
    assert_eq!(
        partial.record.parse_state.parse_lifecycle_state_class,
        ParseLifecycleStateClass::Completed
    );
    assert_eq!(
        partial.record.parse_state.parse_quality_class,
        ParseQualityClass::PartialTreeWithErrors
    );
    assert!(partial.record.requires_degraded_disclosure());
    assert!(partial
        .record
        .parse_state
        .failure_reason_classes
        .contains(&FailureReasonClass::ParserErrorNodesPresent));

    let missing_case = &fixture.missing_grammar_case;
    let missing = supervisor.parse_text(request_for(&missing_case.language_id), "value = true");
    assert_eq!(
        missing
            .record
            .parse_state
            .parse_lifecycle_state_class
            .as_str(),
        missing_case.expected_lifecycle
    );
    assert_eq!(
        missing.record.parse_state.parse_quality_class.as_str(),
        missing_case.expected_quality
    );
    assert_eq!(
        missing.record.parse_state.failure_reason_classes[0].as_str(),
        missing_case.expected_failure
    );
    assert!(missing.tree().is_none());
    assert!(missing.record.requires_degraded_disclosure());

    let handle = supervisor
        .start_parser("parser-runtime:test:shutdown", "language:python")
        .expect("python parser should start");
    assert_eq!(
        handle.lifecycle().runtime_state_class,
        ParserRuntimeStateClass::Ready
    );
    let shutdown = handle.shutdown();
    assert_eq!(
        shutdown.runtime_state_class,
        ParserRuntimeStateClass::Shutdown
    );
    assert_eq!(
        shutdown.failure_reason_classes,
        vec![FailureReasonClass::None]
    );
}

fn request_for(language_id: &str) -> ParseRequest {
    let safe_language = language_id.replace(':', "-");
    ParseRequest::foreground_file(
        format!("parse-session:test:{safe_language}"),
        format!("doc:test:{safe_language}"),
        format!("buffer:test:{safe_language}"),
        1,
        language_id,
        "2026-05-13T00:00:00Z",
    )
}

fn load_fixture() -> RegistryFixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/language/tree_sitter_registry/launch_registry_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
