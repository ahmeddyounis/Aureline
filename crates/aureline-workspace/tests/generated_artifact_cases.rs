//! Fixture-driven coverage for the generated-artifact lineage catalog.
//!
//! Walks every case under `fixtures/workspace/generated_artifact_cases/*.json`
//! and asserts the default catalog emits the expected lineage records (or no
//! record for the failure-drill paths).

use std::path::Path;

use aureline_workspace::{
    default_generated_artifact_catalog, GeneratedArtifactClass, LineageFreshnessClass,
    LineageHintRecord,
};
use serde::Deserialize;
use serde_json::Value as JsonValue;

#[derive(Debug, Deserialize)]
struct CaseFixture {
    case_id: String,
    title: String,
    input: CaseInput,
    expected_lineage_records: Vec<ExpectedRecord>,
}

#[derive(Debug, Deserialize)]
struct CaseInput {
    relative_paths: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedRecord {
    generated_relative_path: String,
    #[serde(default = "default_expect_match")]
    expect_match: bool,
    #[serde(default)]
    generated_class: Option<String>,
    #[serde(default)]
    source_canonical_relative_path: Option<String>,
    #[serde(default)]
    producer_id: Option<String>,
    #[serde(default)]
    producer_label: Option<String>,
    #[serde(default)]
    freshness_class: Option<String>,
    #[serde(default)]
    rule_id: Option<String>,
}

fn default_expect_match() -> bool {
    true
}

fn class_token(class: GeneratedArtifactClass) -> &'static str {
    class.as_str()
}

fn freshness_token(freshness: LineageFreshnessClass) -> &'static str {
    freshness.as_str()
}

fn assert_record_matches(case_id: &str, expected: &ExpectedRecord, got: &LineageHintRecord) {
    if let Some(class) = expected.generated_class.as_deref() {
        assert_eq!(
            class_token(got.generated_class),
            class,
            "{case_id}: generated_class for {} mismatch",
            expected.generated_relative_path
        );
    }
    assert_eq!(
        got.source_canonical_relative_path.as_deref(),
        expected.source_canonical_relative_path.as_deref(),
        "{case_id}: source_canonical_relative_path for {} mismatch",
        expected.generated_relative_path
    );
    if let Some(producer_id) = expected.producer_id.as_deref() {
        assert_eq!(
            got.producer_id.as_str(),
            producer_id,
            "{case_id}: producer_id for {} mismatch",
            expected.generated_relative_path
        );
    }
    if let Some(producer_label) = expected.producer_label.as_deref() {
        assert_eq!(
            got.producer_label.as_str(),
            producer_label,
            "{case_id}: producer_label for {} mismatch",
            expected.generated_relative_path
        );
    }
    if let Some(freshness) = expected.freshness_class.as_deref() {
        assert_eq!(
            freshness_token(got.freshness_class),
            freshness,
            "{case_id}: freshness_class for {} mismatch",
            expected.generated_relative_path
        );
    }
    if let Some(rule_id) = expected.rule_id.as_deref() {
        assert_eq!(
            got.rule_id.as_str(),
            rule_id,
            "{case_id}: rule_id for {} mismatch",
            expected.generated_relative_path
        );
    }
}

#[test]
fn fixture_corpus_drives_generated_artifact_catalog() {
    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/workspace/generated_artifact_cases");
    let catalog = default_generated_artifact_catalog();

    let mut case_count = 0usize;
    for entry in std::fs::read_dir(&root_dir).expect("generated_artifact_cases dir must exist") {
        let entry = entry.expect("generated_artifact_cases entry must read");
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let payload = std::fs::read_to_string(&path).expect("fixture must read");
        let mut value: JsonValue = serde_json::from_str(&payload).expect("fixture must parse");
        if let Some(obj) = value.as_object_mut() {
            obj.remove("$schema");
            obj.remove("__fixture__");
            obj.remove("scenario");
        }
        let fixture: CaseFixture = serde_json::from_value(value).unwrap_or_else(|err| {
            panic!(
                "fixture {} did not deserialize: {err}",
                path.display(),
            )
        });

        // Cover every input path with the catalog and confirm the result
        // matches the expected record (or that no record is produced when
        // expect_match=false).
        let expected_by_path: std::collections::BTreeMap<&str, &ExpectedRecord> = fixture
            .expected_lineage_records
            .iter()
            .map(|expected| (expected.generated_relative_path.as_str(), expected))
            .collect();

        for relative_path in &fixture.input.relative_paths {
            let expected = expected_by_path
                .get(relative_path.as_str())
                .copied()
                .unwrap_or_else(|| {
                    panic!(
                        "case {} ({}): no expectation declared for input path {relative_path}",
                        fixture.case_id, fixture.title
                    )
                });

            let got = catalog.detect(relative_path);
            if !expected.expect_match {
                assert!(
                    got.is_none(),
                    "case {}: expected no lineage hint for {relative_path}, got {:?}",
                    fixture.case_id,
                    got
                );
                continue;
            }
            let got = got.unwrap_or_else(|| {
                panic!(
                    "case {}: expected a lineage hint for {relative_path}, got none",
                    fixture.case_id
                )
            });
            assert_record_matches(&fixture.case_id, expected, &got);
            assert_eq!(
                got.generated_relative_path, *relative_path,
                "{}: generated_relative_path must echo the input path",
                fixture.case_id
            );
            assert_eq!(
                got.schema_version,
                LineageHintRecord::SCHEMA_VERSION,
                "{}: schema_version must match seeded value",
                fixture.case_id
            );
        }

        case_count += 1;
    }

    assert!(
        case_count >= 4,
        "expected at least 4 generated_artifact fixture cases, found {case_count}",
    );
}
