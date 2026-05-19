//! Integration test that walks the worked fixture corpus and confirms each
//! positive record validates clean while each negative record surfaces the
//! expected violation token.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use aureline_preview::preview_origin::{
    BrowserRuntimeSessionOrigin, HotReloadStateDescriptor, PreviewOriginDescriptor,
    PreviewOriginFinding, PreviewTargetDescriptor, RuntimeMutationActionPlan,
};
use serde_json::Value;

fn fixtures_dir() -> PathBuf {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir
        .join("..")
        .join("..")
        .join("fixtures")
        .join("preview")
        .join("m3")
        .join("preview_origin_and_browser_runtime")
}

fn read_fixture(path: &Path) -> Value {
    let raw =
        std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {}", path.display(), e));
    serde_json::from_str(&raw).unwrap_or_else(|e| panic!("parse {}: {}", path.display(), e))
}

fn strip_fixture_block(mut value: Value) -> (Value, BTreeMap<String, Value>) {
    let mut meta = BTreeMap::new();
    if let Some(object) = value.as_object_mut() {
        if let Some(block) = object.remove("__fixture__") {
            if let Some(map) = block.as_object() {
                for (k, v) in map.iter() {
                    meta.insert(k.clone(), v.clone());
                }
            }
        }
    }
    (value, meta)
}

fn validate_findings(record_kind: &str, value: Value) -> Vec<PreviewOriginFinding> {
    match record_kind {
        "preview_origin_descriptor_record" => {
            let record: PreviewOriginDescriptor = serde_json::from_value(value)
                .expect("deserialize preview_origin_descriptor_record");
            record.validate()
        }
        "preview_target_descriptor_record" => {
            let record: PreviewTargetDescriptor = serde_json::from_value(value)
                .expect("deserialize preview_target_descriptor_record");
            record.validate()
        }
        "hot_reload_state_descriptor_record" => {
            let record: HotReloadStateDescriptor = serde_json::from_value(value)
                .expect("deserialize hot_reload_state_descriptor_record");
            record.validate()
        }
        "browser_runtime_session_origin_record" => {
            let record: BrowserRuntimeSessionOrigin = serde_json::from_value(value)
                .expect("deserialize browser_runtime_session_origin_record");
            record.validate()
        }
        "runtime_mutation_action_plan_record" => {
            let record: RuntimeMutationActionPlan = serde_json::from_value(value)
                .expect("deserialize runtime_mutation_action_plan_record");
            record.validate()
        }
        other => panic!("unknown record_kind {}", other),
    }
}

#[test]
fn fixture_corpus_round_trips() {
    let dir = fixtures_dir();
    let entries: Vec<_> = std::fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {}", dir.display(), e))
        .filter_map(Result::ok)
        .collect();
    assert!(!entries.is_empty(), "fixture corpus is empty");

    let mut positive_seen = 0_usize;
    let mut negative_seen = 0_usize;

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_owned();
        let (record, meta) = strip_fixture_block(read_fixture(&path));
        let record_kind = record
            .get("record_kind")
            .and_then(|v| v.as_str())
            .unwrap_or_else(|| panic!("{} missing record_kind", file_name))
            .to_owned();

        let findings = validate_findings(&record_kind, record);

        if file_name.contains(".positive.") {
            positive_seen += 1;
            assert!(
                findings.is_empty(),
                "positive fixture {} should validate clean; findings = {:?}",
                file_name,
                findings
            );
        } else if file_name.contains(".negative.") {
            negative_seen += 1;
            let expected = meta
                .get("expected_violation_check_id")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| {
                    panic!(
                        "negative fixture {} missing __fixture__.expected_violation_check_id",
                        file_name
                    )
                });
            assert!(
                findings.iter().any(|f| f.check_id == expected),
                "negative fixture {} expected violation {}; got {:?}",
                file_name,
                expected,
                findings
            );
        }
    }

    assert!(positive_seen >= 8, "expected at least 8 positive fixtures");
    assert!(negative_seen >= 5, "expected at least 5 negative fixtures");
}
