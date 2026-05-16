//! Rust port of `ci/check_activity_center_alpha.py`.
//!
//! Asserts the same artifact-contract invariants the Python validator does so
//! the activity-center alpha schema, fixtures, runtime consumer, support-seed
//! consumer, and doc stay aligned. The Python validator is kept in place; this
//! test is additive and can be wired into Rust-only CI.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::Value;

const SCHEMA_REL: &str = "schemas/events/activity_row.schema.json";
const SNAPSHOT_REL: &str = "fixtures/ux/activity_center_alpha/activity_center_alpha_snapshot.json";
const SUPPORT_EXPORT_REL: &str =
    "fixtures/ux/activity_center_alpha/support_export_activity_rows.json";
const DOCS_REL: &str = "docs/ux/activity_center_alpha.md";
const RUNTIME_REL: &str = "crates/aureline-shell/src/activity_center/alpha.rs";
const SUPPORT_CONSUMER_REL: &str = "crates/aureline-shell/src/support_seed/mod.rs";

const REQUIRED_FAMILIES: &[&str] = &[
    "indexing",
    "restore",
    "install_update",
    "task_run",
    "test_run",
];
const REQUIRED_STATES: &[&str] = &[
    "running",
    "queued_waiting",
    "partially_completed",
    "failed",
    "completed",
];
const REQUIRED_PARTITIONS: &[&str] = &["current_work", "needs_attention", "completed"];

const REQUIRED_RUNTIME_MARKERS: &[&str] = &[
    "ActivityCenterAlphaRuntime",
    "ActivityCenterAlphaStore",
    "ActivityCenterSupportExport",
    "has_exact_reopen_identity",
    "satisfies_sensitive_detail_rule",
];
const REQUIRED_SUPPORT_MARKERS: &[&str] = &[
    "activity_center_preview",
    "activity_center_seed",
    "support.item.activity_center_alpha_rows",
];

const SENSITIVE_IMPACT_FLAGS: &[&str] = &[
    "affects_cost",
    "affects_policy",
    "affects_network",
    "affects_trust",
    "affects_provider_state",
    "affects_recovery_posture",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_json(rel: &str) -> Value {
    let path = repo_root().join(rel);
    let payload = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

fn read_text(rel: &str) -> String {
    let path = repo_root().join(rel);
    fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn rows<'a>(record: &'a Value, label: &str) -> &'a [Value] {
    record
        .get("rows")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_else(|| panic!("{label}.rows must be a non-empty array"))
}

fn nonempty_str<'a>(value: &'a Value, label: &str) -> &'a str {
    let text = value
        .as_str()
        .unwrap_or_else(|| panic!("{label} must be a non-empty string"));
    if text.is_empty() {
        panic!("{label} must be a non-empty string");
    }
    text
}

#[test]
fn required_artifacts_exist() {
    for rel in [
        SCHEMA_REL,
        SNAPSHOT_REL,
        SUPPORT_EXPORT_REL,
        DOCS_REL,
        RUNTIME_REL,
        SUPPORT_CONSUMER_REL,
    ] {
        let path = repo_root().join(rel);
        assert!(
            path.exists(),
            "required activity-center alpha artifact is missing: {}",
            path.display()
        );
    }
}

#[test]
fn schema_declares_canonical_id_and_record_defs() {
    let schema = read_json(SCHEMA_REL);
    assert_eq!(
        schema.get("$id").and_then(Value::as_str),
        Some("https://aureline.dev/schemas/events/activity_row.schema.json"),
        "activity row schema $id is not the canonical events schema"
    );
    let defs = schema
        .get("$defs")
        .and_then(Value::as_object)
        .expect("schema $defs must be an object");
    for record in [
        "activity_row_record",
        "activity_center_alpha_snapshot_record",
        "activity_center_support_export_record",
    ] {
        assert!(
            defs.contains_key(record),
            "schema $defs missing record definition: {record}"
        );
    }
}

#[test]
fn snapshot_record_kind_and_row_coverage() {
    let snapshot = read_json(SNAPSHOT_REL);
    assert_eq!(
        snapshot.get("record_kind").and_then(Value::as_str),
        Some("activity_center_alpha_snapshot_record"),
        "snapshot record_kind must be activity_center_alpha_snapshot_record"
    );
    let rows = rows(&snapshot, "snapshot");
    assert!(!rows.is_empty(), "snapshot must contain activity rows");

    let families: BTreeSet<&str> = rows
        .iter()
        .filter_map(|row| row.get("job_family").and_then(Value::as_str))
        .collect();
    for family in REQUIRED_FAMILIES {
        assert!(
            families.contains(family),
            "snapshot misses required job_family: {family}"
        );
    }

    let states: BTreeSet<&str> = rows
        .iter()
        .filter_map(|row| row.get("state_class").and_then(Value::as_str))
        .collect();
    for state in REQUIRED_STATES {
        assert!(
            states.contains(state),
            "snapshot misses required state_class: {state}"
        );
    }

    let partitions: BTreeSet<&str> = rows
        .iter()
        .filter_map(|row| row.get("activity_partition").and_then(Value::as_str))
        .collect();
    for partition in REQUIRED_PARTITIONS {
        assert!(
            partitions.contains(partition),
            "snapshot misses required activity_partition: {partition}"
        );
    }
}

#[test]
fn every_snapshot_row_carries_required_invariants() {
    let snapshot = read_json(SNAPSHOT_REL);
    let rows = rows(&snapshot, "snapshot");

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    let mut exact_rows = 0usize;
    let mut exportable_rows = 0usize;
    let mut sensitive_rows_with_detail = 0usize;
    let mut retry_rows = 0usize;
    let mut cancel_rows = 0usize;

    for (idx, row) in rows.iter().enumerate() {
        let obj = row
            .as_object()
            .unwrap_or_else(|| panic!("snapshot row {idx} must be an object"));

        let row_id = nonempty_str(
            obj.get("activity_row_id").unwrap_or(&Value::Null),
            &format!("snapshot.row[{idx}].activity_row_id"),
        );
        for field in [
            "durable_job_id",
            "canonical_event_id",
            "actor_identity_ref",
            "actor_or_subsystem_label",
        ] {
            nonempty_str(
                obj.get(field).unwrap_or(&Value::Null),
                &format!("snapshot.row[{idx}].{field}"),
            );
        }
        assert!(
            seen_ids.insert(row_id),
            "duplicate activity_row_id in snapshot: {row_id}"
        );

        let exact_ref = obj.get("exact_reopen_identity_ref").and_then(Value::as_str);
        let reopen_exact = obj
            .get("reopen_target")
            .and_then(|target| target.get("exact_target_identity_ref"))
            .and_then(Value::as_str);
        if reopen_exact.is_some() && reopen_exact == exact_ref {
            exact_rows += 1;
        }

        let actions = obj
            .get("actions")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        let action_kinds: BTreeSet<&str> = actions
            .iter()
            .filter_map(|action| action.get("action_kind").and_then(Value::as_str))
            .collect();
        assert!(
            action_kinds.contains("open_details"),
            "row {row_id} has no open_details action"
        );
        if action_kinds.contains("retry_job") {
            retry_rows += 1;
            for action in &actions {
                if action.get("action_kind").and_then(Value::as_str) == Some("retry_job") {
                    assert_eq!(
                        action
                            .get("reissues_original_side_effect")
                            .and_then(Value::as_bool),
                        Some(true),
                        "row {row_id} retry action must declare reissues_original_side_effect=true"
                    );
                }
            }
        }
        if action_kinds.contains("cancel_job") {
            cancel_rows += 1;
        }

        let impact_obj = obj.get("impact").and_then(Value::as_object);
        let sensitive = impact_obj
            .map(|impact| {
                SENSITIVE_IMPACT_FLAGS
                    .iter()
                    .any(|flag| impact.get(*flag).and_then(Value::as_bool) == Some(true))
            })
            .unwrap_or(false);
        if sensitive {
            let detail_required = impact_obj
                .and_then(|impact| impact.get("detail_or_evidence_required"))
                .and_then(Value::as_bool)
                == Some(true);
            let progress = obj.get("progress").and_then(Value::as_object);
            let detail_ref = progress
                .and_then(|p| p.get("detail_or_evidence_ref"))
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty());
            assert!(
                detail_required && detail_ref.is_some(),
                "row {row_id}: sensitive-impact row must carry detail_or_evidence_required=true and progress.detail_or_evidence_ref"
            );
            sensitive_rows_with_detail += 1;
        }

        let support_link = obj.get("support_link").and_then(Value::as_object);
        if support_link
            .and_then(|s| s.get("exportable"))
            .and_then(Value::as_bool)
            == Some(true)
        {
            exportable_rows += 1;
            assert_eq!(
                support_link
                    .and_then(|s| s.get("raw_private_material_excluded"))
                    .and_then(Value::as_bool),
                Some(true),
                "row {row_id}: support-exportable row must set raw_private_material_excluded=true"
            );
        }
    }

    assert_eq!(
        exact_rows,
        rows.len(),
        "every row must preserve exact reopen identity (got {exact_rows} of {} rows)",
        rows.len()
    );
    assert!(
        exportable_rows >= 1,
        "snapshot must have at least one support-exportable row"
    );
    assert!(
        sensitive_rows_with_detail >= 1,
        "snapshot must prove impact flags with evidence detail at least once"
    );
    assert!(
        retry_rows >= 1,
        "snapshot must cover retry posture (no retry_job action found)"
    );
    assert!(
        cancel_rows >= 1,
        "snapshot must cover cancel posture (no cancel_job action found)"
    );
}

#[test]
fn support_export_maps_to_snapshot_and_excludes_raw_material() {
    let snapshot = read_json(SNAPSHOT_REL);
    let export = read_json(SUPPORT_EXPORT_REL);
    assert_eq!(
        export.get("record_kind").and_then(Value::as_str),
        Some("activity_center_support_export_record"),
        "support export record_kind must be activity_center_support_export_record"
    );
    let snapshot_ids: BTreeSet<&str> = rows(&snapshot, "snapshot")
        .iter()
        .filter_map(|row| row.get("activity_row_id").and_then(Value::as_str))
        .collect();
    let export_rows = rows(&export, "support_export");
    assert!(
        !export_rows.is_empty(),
        "support export must include at least one structured row"
    );

    let mut exported_families: BTreeSet<&str> = BTreeSet::new();
    for (idx, row) in export_rows.iter().enumerate() {
        let obj = row
            .as_object()
            .unwrap_or_else(|| panic!("support_export.row[{idx}] must be an object"));
        let row_id = obj
            .get("activity_row_id")
            .and_then(Value::as_str)
            .unwrap_or_else(|| {
                panic!("support_export.row[{idx}].activity_row_id must be a string")
            });
        assert!(
            snapshot_ids.contains(row_id),
            "support export row does not map to snapshot row: {row_id}"
        );
        assert_eq!(
            obj.get("raw_private_material_excluded")
                .and_then(Value::as_bool),
            Some(true),
            "support export row {row_id} must exclude raw private material"
        );
        if let Some(family) = obj.get("job_family").and_then(Value::as_str) {
            exported_families.insert(family);
        }
    }
    assert!(
        exported_families.contains("test_run") || exported_families.contains("restore"),
        "support export must include a meaningful test_run or restore family row"
    );
}

#[test]
fn runtime_and_support_consumers_carry_required_markers() {
    let runtime = read_text(RUNTIME_REL);
    for marker in REQUIRED_RUNTIME_MARKERS {
        assert!(
            runtime.contains(marker),
            "runtime consumer {RUNTIME_REL} is missing required marker: {marker}"
        );
    }
    let support = read_text(SUPPORT_CONSUMER_REL);
    for marker in REQUIRED_SUPPORT_MARKERS {
        assert!(
            support.contains(marker),
            "support consumer {SUPPORT_CONSUMER_REL} is missing required marker: {marker}"
        );
    }
}
