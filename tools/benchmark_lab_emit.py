#!/usr/bin/env python3
"""Emit a benchmark-lab run-result record and human-readable summary.

Reads the committed protected corpus manifest, protected-metrics file,
fitness-function catalog, and benchmark environment manifests,
assembles one record per invocation conforming to
``schemas/benchmarks/run_result.schema.json``, and writes the companion
Markdown summary.

The script is intentionally conservative: it uses only the Python
standard library, writes deterministic bytes (sorted keys, explicit
2-space indentation, ``SOURCE_DATE_EPOCH``-pinned timestamps, pinned
``TZ`` / ``LC_ALL``), and never invents a fitness-row id, corpus id,
or metric name outside the registers under ``artifacts/bench/`` and
``fixtures/benchmarks/``.

Seed verification (``--verify-seed``) re-emits the two committed
seed records plus the dashboard snapshot under a temporary directory
and diffs them against the files under
``artifacts/benchmarks/dashboard_seed/`` so a change to this script,
to the protected-metrics file, to the fitness-function catalog, or to
the corpus manifest either lands with a matching seed refresh or fails
the lane.
"""

from __future__ import annotations

import argparse
import datetime as _dt
import difflib
import json
import os
import shutil
import sys
import tempfile
from pathlib import Path
from typing import Any, Iterable

RUN_RESULT_SCHEMA_VERSION = 2
RECORD_KIND = "benchmark_run_result_record"

FITNESS_CATALOG_REL = "artifacts/bench/fitness_function_catalog.yaml"
CORPUS_MANIFEST_REL = "fixtures/benchmarks/corpus_manifest.yaml"
PROTECTED_METRICS_REL = "artifacts/bench/protected_metrics.yaml"
SCHEMA_REL = "schemas/benchmarks/run_result.schema.json"

SEED_OUT_DIR_REL = "artifacts/benchmarks/dashboard_seed"

VALID_RUN_CONTEXTS = {
    "reference_capture",
    "provisional_capture",
    "self_capture",
    "smoke_subset",
}
VALID_LANES = {"ci_nightly", "ci_merge_queue", "ci_preview", "developer_local"}
VALID_TRIGGERS = {
    "scheduled_nightly",
    "manual_dispatch",
    "commit_gated",
    "developer_invocation",
}
VALID_CORPUS_SUBSETS = {"full", "smoke"}
VALID_ENVIRONMENT_PRESETS = {
    "self_capture_current_machine",
    "ref_macos_arm64_nominal",
    "ref_windows_x86_64_nominal",
    "ref_linux_x86_64_nominal",
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--out-dir", default=None)
    parser.add_argument("--corpus-subset", default="smoke", choices=sorted(VALID_CORPUS_SUBSETS))
    parser.add_argument("--run-context", default="self_capture", choices=sorted(VALID_RUN_CONTEXTS))
    parser.add_argument("--lane", default="developer_local", choices=sorted(VALID_LANES))
    parser.add_argument("--trigger", default="developer_invocation", choices=sorted(VALID_TRIGGERS))
    parser.add_argument(
        "--environment-preset",
        default="self_capture_current_machine",
        choices=sorted(VALID_ENVIRONMENT_PRESETS),
    )
    parser.add_argument("--regression-demo", action="store_true")
    parser.add_argument("--verify-seed", action="store_true")
    return parser.parse_args()


def read_text(path: Path) -> str:
    with path.open("r", encoding="utf-8") as fh:
        return fh.read()


def parse_corpus_manifest(text: str) -> dict[str, Any]:
    """Very small YAML reader tailored to the corpus manifest header.

    The manifest header only uses top-level ``key: value`` pairs for
    ``manifest_id`` and ``manifest_revision`` before any structured
    content. A full YAML parser is not available in the stdlib; reading
    the handful of fields we need by line is enough and keeps the tool
    stdlib-only.
    """

    manifest_id = None
    manifest_revision = None
    for raw in text.splitlines():
        stripped = raw.strip()
        if stripped.startswith("manifest_id:") and manifest_id is None:
            manifest_id = stripped.split(":", 1)[1].strip()
        elif stripped.startswith("manifest_revision:") and manifest_revision is None:
            value = stripped.split(":", 1)[1].strip()
            manifest_revision = int(value)
        if manifest_id is not None and manifest_revision is not None:
            break
    if manifest_id is None or manifest_revision is None:
        raise SystemExit(
            f"corpus manifest at {CORPUS_MANIFEST_REL} missing manifest_id or manifest_revision"
        )
    return {"manifest_id": manifest_id, "manifest_revision": manifest_revision}


def parse_fitness_catalog(text: str) -> dict[str, Any]:
    catalog_id = None
    catalog_revision = None
    for raw in text.splitlines():
        stripped = raw.strip()
        if stripped.startswith("catalog_id:") and catalog_id is None:
            catalog_id = stripped.split(":", 1)[1].strip()
        elif stripped.startswith("catalog_revision:") and catalog_revision is None:
            value = stripped.split(":", 1)[1].strip()
            catalog_revision = int(value)
        if catalog_id is not None and catalog_revision is not None:
            break
    if catalog_id is None or catalog_revision is None:
        raise SystemExit(
            f"fitness-function catalog at {FITNESS_CATALOG_REL} missing catalog_id or catalog_revision"
        )
    return {"catalog_id": catalog_id, "catalog_revision": catalog_revision}


def parse_protected_metrics(text: str) -> dict[str, Any]:
    metrics_file_id = None
    metrics_file_revision = None
    for raw in text.splitlines():
        stripped = raw.strip()
        if stripped.startswith("metrics_file_id:") and metrics_file_id is None:
            metrics_file_id = stripped.split(":", 1)[1].strip()
        elif stripped.startswith("metrics_file_revision:") and metrics_file_revision is None:
            value = stripped.split(":", 1)[1].strip()
            metrics_file_revision = int(value)
        if metrics_file_id is not None and metrics_file_revision is not None:
            break
    if metrics_file_id is None or metrics_file_revision is None:
        raise SystemExit(
            "protected-metrics file at "
            f"{PROTECTED_METRICS_REL} missing metrics_file_id or metrics_file_revision"
        )
    return {
        "metrics_file_id": metrics_file_id,
        "metrics_file_revision": metrics_file_revision,
    }


def pinned_epoch_timestamp() -> str:
    """ISO-8601 UTC timestamp pinned to ``SOURCE_DATE_EPOCH``."""

    epoch = int(os.environ.get("SOURCE_DATE_EPOCH", "0"))
    return _dt.datetime.fromtimestamp(epoch, tz=_dt.timezone.utc).strftime(
        "%Y-%m-%dT%H:%M:%SZ"
    )


def pinned_epoch_date() -> str:
    epoch = int(os.environ.get("SOURCE_DATE_EPOCH", "0"))
    return _dt.datetime.fromtimestamp(epoch, tz=_dt.timezone.utc).strftime("%Y-%m-%d")


def self_capture_rows() -> list[dict[str, Any]]:
    """Rows emitted by the seeded self-capture run.

    Values mirror the committed seed at
    ``artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json``
    so the two paths agree byte-for-byte.
    """

    return [
        {
            "fitness_row_id": "ff.warm_start_to_first_paint",
            "corpus_refs": [
                "corpus.workflow.startup_warm_to_first_paint",
                "corpus.reference.micro_local_folder",
            ],
            "sli_kind": "structural_digest",
            "threshold_mode": "to_be_set_by_benchmark_council",
            "threshold_at_measurement": {
                "p50_ms": "to_be_set_by_benchmark_council",
                "p95_ms": "to_be_set_by_benchmark_council",
            },
            "measured_value": {
                "digest_ref": "trace_digest.full_scene.warm_start_to_first_paint.seed",
                "sample_size": 1,
            },
            "measurement_status": "captured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "shell_spike_timing_trace",
            "data_source_ref": "artifacts/traces/examples/full_scene.json",
            "trace_bundle_ref": "trace_bundle.shell_spike.fixture_v1.full_scene",
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.first_paint",
            "corpus_refs": [
                "corpus.workflow.startup_warm_to_first_paint",
                "corpus.workflow.first_useful_edit_rust_self_host",
                "corpus.reference.micro_local_folder",
            ],
            "sli_kind": "structural_digest",
            "threshold_mode": "absolute_p50_and_p95",
            "threshold_at_measurement": {
                "p50_ms": "to_be_set_by_benchmark_council",
                "p95_ms": "to_be_set_by_benchmark_council",
            },
            "measured_value": {
                "digest_ref": "trace_digest.full_scene.first_paint.seed",
                "sample_size": 1,
            },
            "measurement_status": "captured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "shell_spike_timing_trace",
            "data_source_ref": "artifacts/traces/examples/full_scene.json",
            "trace_bundle_ref": "trace_bundle.shell_spike.fixture_v1.full_scene",
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.input_to_paint",
            "corpus_refs": [
                "corpus.micro.interaction_safety_cases",
                "corpus.workflow.startup_warm_to_first_paint",
            ],
            "sli_kind": "structural_digest",
            "threshold_mode": "absolute_p50_and_p95",
            "threshold_at_measurement": {
                "p50_ms": "to_be_set_by_benchmark_council",
                "p95_ms": "to_be_set_by_benchmark_council",
            },
            "measured_value": {
                "digest_ref": "trace_digest.full_scene.input_to_paint.seed",
                "sample_size": 8,
            },
            "measurement_status": "captured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "shell_spike_timing_trace",
            "data_source_ref": "artifacts/traces/examples/full_scene.json",
            "trace_bundle_ref": "trace_bundle.shell_spike.fixture_v1.full_scene",
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.buffer_operations",
            "corpus_refs": [
                "corpus.micro.interaction_safety_cases",
                "corpus.workflow.first_useful_edit_rust_self_host",
                "corpus.large_file.decode_recovery",
            ],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "boolean_gate",
            "threshold_at_measurement": {"boolean_must_pass": True},
            "measured_value": {"boolean_outcome": True, "sample_size": 1},
            "measurement_status": "captured",
            "result": "pass",
            "trend_direction": "unchanged",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "corpus_fixture_measurement",
            "data_source_ref": "artifacts/buffer/buffer_metrics_seed.json",
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.vfs_save_conflict_handling",
            "corpus_refs": [
                "corpus.boundary.filesystem_identity_cases",
                "corpus.boundary.mutation_lineage_cases",
                "corpus.workflow.first_useful_edit_rust_self_host",
            ],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "boolean_gate",
            "threshold_at_measurement": {"boolean_must_pass": True},
            "measured_value": {"boolean_outcome": True, "sample_size": 1},
            "measurement_status": "captured",
            "result": "pass",
            "trend_direction": "unchanged",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "corpus_fixture_measurement",
            "data_source_ref": "fixtures/runtime/vfs_decision_examples/",
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.benchmark_lab_health",
            "corpus_refs": [
                "corpus.workflow.startup_warm_to_first_paint",
                "corpus.workflow.first_useful_edit_rust_self_host",
                "corpus.workflow.plain_open_unknown_archetype",
            ],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "boolean_gate",
            "threshold_at_measurement": {"boolean_must_pass": True},
            "measured_value": {"boolean_outcome": True, "sample_size": 1},
            "measurement_status": "captured",
            "result": "pass",
            "trend_direction": "unchanged",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "governance_packet_self_audit",
            "data_source_ref": "artifacts/governance/governance_packet_template.yaml",
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.power_thermal_posture",
            "corpus_refs": [
                "corpus.workflow.startup_warm_to_first_paint",
                "corpus.workflow.first_useful_edit_rust_self_host",
            ],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "to_be_set_by_benchmark_council",
            "threshold_at_measurement": None,
            "measured_value": None,
            "measurement_status": "not_measured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "to_be_wired_by_benchmark_council",
            "data_source_ref": None,
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.restore_fidelity",
            "corpus_refs": [
                "corpus.recovery.restore_last_session_compatible",
                "corpus.recovery.recent_work_missing_target",
                "corpus.recovery.resume_managed_workspace_reauth",
            ],
            "sli_kind": "ratio_unit_interval",
            "threshold_mode": "to_be_set_by_benchmark_council",
            "threshold_at_measurement": None,
            "measured_value": None,
            "measurement_status": "not_measured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "to_be_wired_by_benchmark_council",
            "data_source_ref": None,
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
        {
            "fitness_row_id": "ff.command_parity",
            "corpus_refs": ["corpus.boundary.filesystem_identity_cases"],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "to_be_set_by_benchmark_council",
            "threshold_at_measurement": None,
            "measured_value": None,
            "measurement_status": "not_measured",
            "result": "provisional",
            "trend_direction": "unknown_insufficient_history",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "to_be_wired_by_benchmark_council",
            "data_source_ref": None,
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": None,
            "error_reason": None,
            "notes_ref": None,
        },
    ]


def regression_example_rows() -> list[dict[str, Any]]:
    return [
        {
            "fitness_row_id": "ff.benchmark_lab_health",
            "corpus_refs": ["corpus.workflow.startup_warm_to_first_paint"],
            "sli_kind": "boolean_pass_fail",
            "threshold_mode": "boolean_gate",
            "threshold_at_measurement": {"boolean_must_pass": True},
            "measured_value": {"boolean_outcome": False, "sample_size": 1},
            "measurement_status": "captured",
            "result": "fail",
            "trend_direction": "regressing",
            "host_platform_class": "host_independent",
            "host_os": None,
            "rustc_target_triple": None,
            "data_source_kind": "governance_packet_self_audit",
            "data_source_ref": "artifacts/governance/governance_packet_template.yaml",
            "trace_bundle_ref": None,
            "waiver_refs": [],
            "expires_on": None,
            "regression_trigger_ref": {
                "kind": "corpus_row_missing",
                "summary_ref": "benchmark_run_note.seed.regression_example.corpus_row_missing",
            },
            "error_reason": None,
            "notes_ref": "benchmark_run_note.seed.regression_example.benchmark_lab_health",
        }
    ]


def summarise(rows: Iterable[dict[str, Any]]) -> dict[str, Any]:
    row_count = {
        "pass": 0,
        "warn": 0,
        "fail": 0,
        "not_measured": 0,
        "waived": 0,
        "provisional": 0,
    }
    trigger_count = {
        "threshold_exceeded": 0,
        "boolean_gate_failed": 0,
        "ratio_below_floor": 0,
        "corpus_row_missing": 0,
        "trace_schema_nonconforming": 0,
        "ad_hoc_metric_name_observed": 0,
        "toolchain_pin_drift": 0,
        "hardware_definition_mismatch": 0,
        "fitness_catalog_row_status_provisional": 0,
    }
    for row in rows:
        result = row["result"]
        if result == "not_measured" and row["measurement_status"] == "not_measured":
            # not_measured is routed to the provisional bucket only when
            # the row itself is provisional; otherwise it increments the
            # not_measured bucket to match row_result_class.
            if row["result"] == "provisional":
                row_count["provisional"] += 1
            else:
                row_count["not_measured"] += 1
        else:
            row_count[result] += 1
        trig = row.get("regression_trigger_ref")
        if trig is not None:
            trigger_count[trig["kind"]] += 1
    return row_count, trigger_count


def build_identity_for(run_id: str) -> dict[str, Any]:
    return {
        "exact_build_identity_ref": f"exact_build_identity.seed.{run_id.split('.')[-1]}",
        "release_channel_class": "dev_local",
        "producer_lane_class": "developer_local",
        "workspace_version": "0.0.0",
        "commit_short_hash": "000000000000",
    }


def toolchain_pin_seed() -> dict[str, Any]:
    return {
        "channel": "pinned-in-rust-toolchain-toml",
        "rustc_version": "pinned-in-rust-toolchain-toml",
        "toolchain_pin_digest": "sha256:seed-toolchain-pin-digest",
        "lockfile_digest": "sha256:seed-lockfile-digest",
    }


ENVIRONMENT_PRESET_ROWS: dict[str, dict[str, dict[str, Any]]] = {
    "self_capture_current_machine": {
        "hardware_definition": {
            "definition_id": "hardware_definition.self_capture.current_machine_reported",
            "definition_revision": 1,
            "is_council_approved_baseline": False,
            "notes_ref": "hardware_definition_note.current_machine.self_capture",
        },
        "environment_definition": {
            "definition_id": "environment_definition.self_capture.current_machine_default",
            "definition_revision": 1,
            "display_class_id": "display_class.self_capture.current_machine_reported",
            "lab_image_id": "lab_image.self_capture.unmanaged_local.rev1",
            "lab_image_revision": 1,
            "power_posture_id": "power_posture.self_capture.reported_out_of_band",
            "thermal_posture_id": "thermal_posture.self_capture.reported_out_of_band",
            "calibration_rule_set_id": "calibration_rule_set.self_capture_disclosure",
            "calibration_rule_set_revision": 1,
            "is_council_approved_reference_environment": False,
            "comparability_note_ref": "benchmark_environment_note.self_capture.current_machine_default",
        },
    },
    "ref_macos_arm64_nominal": {
        "hardware_definition": {
            "definition_id": "hardware_definition.ref.macos15.arm64.apple_silicon_14in",
            "definition_revision": 1,
            "is_council_approved_baseline": True,
            "notes_ref": "hardware_definition_note.ref.macos15.arm64.apple_silicon_14in",
        },
        "environment_definition": {
            "definition_id": "environment_definition.ref.macos15.arm64.internal_14in_nominal",
            "definition_revision": 1,
            "display_class_id": "display_class.internal_14in_retina_3024x1964_sdr60",
            "lab_image_id": "lab_image.macos15.arm64.rev1",
            "lab_image_revision": 1,
            "power_posture_id": "power_posture.ac_balanced",
            "thermal_posture_id": "thermal_posture.nominal",
            "calibration_rule_set_id": "calibration_rule_set.reference_lab",
            "calibration_rule_set_revision": 1,
            "is_council_approved_reference_environment": True,
            "comparability_note_ref": None,
        },
    },
    "ref_windows_x86_64_nominal": {
        "hardware_definition": {
            "definition_id": "hardware_definition.ref.windows11.x86_64.thinkpad_t14_gen5",
            "definition_revision": 1,
            "is_council_approved_baseline": True,
            "notes_ref": "hardware_definition_note.ref.windows11.x86_64.thinkpad_t14_gen5",
        },
        "environment_definition": {
            "definition_id": "environment_definition.ref.windows11.x86_64.internal_14in_nominal",
            "definition_revision": 1,
            "display_class_id": "display_class.internal_14in_1920x1200_sdr60",
            "lab_image_id": "lab_image.windows11.x86_64.rev1",
            "lab_image_revision": 1,
            "power_posture_id": "power_posture.ac_balanced",
            "thermal_posture_id": "thermal_posture.nominal",
            "calibration_rule_set_id": "calibration_rule_set.reference_lab",
            "calibration_rule_set_revision": 1,
            "is_council_approved_reference_environment": True,
            "comparability_note_ref": None,
        },
    },
    "ref_linux_x86_64_nominal": {
        "hardware_definition": {
            "definition_id": "hardware_definition.ref.ubuntu24_04.x86_64.framework13",
            "definition_revision": 1,
            "is_council_approved_baseline": True,
            "notes_ref": "hardware_definition_note.ref.ubuntu24_04.x86_64.framework13",
        },
        "environment_definition": {
            "definition_id": "environment_definition.ref.ubuntu24_04.x86_64.internal_13_5in_nominal",
            "definition_revision": 1,
            "display_class_id": "display_class.internal_13_5in_2256x1504_sdr60",
            "lab_image_id": "lab_image.ubuntu24_04.x86_64.rev1",
            "lab_image_revision": 1,
            "power_posture_id": "power_posture.ac_balanced",
            "thermal_posture_id": "thermal_posture.nominal",
            "calibration_rule_set_id": "calibration_rule_set.reference_lab",
            "calibration_rule_set_revision": 1,
            "is_council_approved_reference_environment": True,
            "comparability_note_ref": None,
        },
    },
}


def hardware_definition_seed(environment_preset: str) -> dict[str, Any]:
    return ENVIRONMENT_PRESET_ROWS[environment_preset]["hardware_definition"].copy()


def environment_definition_seed(environment_preset: str) -> dict[str, Any]:
    return ENVIRONMENT_PRESET_ROWS[environment_preset]["environment_definition"].copy()


def build_run_record(
    *,
    run_id: str,
    run_context: str,
    lane: str,
    trigger: str,
    corpus_manifest: dict[str, Any],
    protected_metrics: dict[str, Any],
    fitness_catalog: dict[str, Any],
    rows: list[dict[str, Any]],
    evidence_channels: list[str],
    notes_ref: str,
    out_dir_rel: str,
    dashboard_ref: str | None,
    environment_preset: str,
) -> dict[str, Any]:
    row_count, trigger_count = summarise(rows)
    raw_ref = f"{out_dir_rel}/raw/{run_id}.json"
    report_ref = f"{out_dir_rel}/report/{run_id}.md"
    ts = pinned_epoch_timestamp()
    return {
        "run_result_schema_version": RUN_RESULT_SCHEMA_VERSION,
        "record_kind": RECORD_KIND,
        "run_id": run_id,
        "run_context": run_context,
        "lane_class": lane,
        "trigger_kind": trigger,
        "started_at": ts,
        "finished_at": ts,
        "measured_on": pinned_epoch_date(),
        "build_identity": build_identity_for(run_id),
        "toolchain_pin": toolchain_pin_seed(),
        "corpus_manifest": corpus_manifest,
        "protected_metrics": protected_metrics,
        "fitness_function_catalog": fitness_catalog,
        "hardware_definition": hardware_definition_seed(environment_preset),
        "environment_definition": environment_definition_seed(environment_preset),
        "comparability": {
            "comparability_class": "not_yet_comparable",
            "quarantine_reasons": [],
            "trend_window": None,
        },
        "rows": rows,
        "summary": {
            "row_count_by_result": row_count,
            "regression_trigger_count_by_kind": trigger_count,
            "human_readable_summary_ref": report_ref,
            "raw_artifact_ref": raw_ref,
            "dashboard_ref": dashboard_ref,
        },
        "evidence_consumer_channels": evidence_channels,
        "notes_ref": notes_ref,
    }


def dump_json(path: Path, body: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    text = json.dumps(body, indent=2, ensure_ascii=False, sort_keys=False)
    if not text.endswith("\n"):
        text += "\n"
    path.write_text(text, encoding="utf-8")


def relative_ref(from_ref: str, to_ref: str | None) -> str | None:
    if to_ref is None:
        return None
    return os.path.relpath(to_ref, start=Path(from_ref).parent).replace(os.sep, "/")


def render_self_capture_markdown(record: dict[str, Any]) -> str:
    summary = record["summary"]
    report_ref = summary["human_readable_summary_ref"]
    raw_link = relative_ref(report_ref, summary["raw_artifact_ref"])
    dashboard_link = relative_ref(report_ref, summary["dashboard_ref"])
    protected_metrics_link = relative_ref(report_ref, PROTECTED_METRICS_REL)
    fitness_catalog_link = relative_ref(report_ref, FITNESS_CATALOG_REL)
    corpus_manifest_link = relative_ref(report_ref, CORPUS_MANIFEST_REL)
    hardware_manifest_link = relative_ref(
        report_ref, "artifacts/perf/reference_hardware_manifest.yaml"
    )
    lab_image_manifest_link = relative_ref(
        report_ref, "artifacts/perf/lab_image_manifest.yaml"
    )
    parity_doc_link = relative_ref(report_ref, "docs/perf/self_capture_parity.md")
    trace_bundle_link = relative_ref(report_ref, "artifacts/traces/examples/full_scene.json")
    if record["run_context"] == "self_capture":
        context_note = (
            "**This run is `self_capture`, not a reference capture.** The numbers\n"
            "below describe harness self-health on a seeded input set; they are\n"
            "not admissible as release-evidence input. A reference capture MUST\n"
            "run on the `ci_nightly` lane against a council-approved hardware\n"
            "row and benchmark environment row.\n\n"
        )
    else:
        context_note = (
            f"**This run is `{record['run_context']}`.** The row results below describe\n"
            "the current harness input set and still depend on the cited hardware row,\n"
            "benchmark environment row, and comparability posture before they can widen\n"
            "into release or publication evidence.\n\n"
        )
    header = (
        f"# Benchmark run: `{record['run_id']}`\n\n"
        "| Field                              | Value                                                                 |\n"
        "|------------------------------------|-----------------------------------------------------------------------|\n"
        f"| Run context                        | `{record['run_context']}`                                                        |\n"
        f"| Lane                               | `{record['lane_class']}`                                                     |\n"
        f"| Trigger                            | `{record['trigger_kind']}`                                                |\n"
        f"| Measured on                        | {record['measured_on']}                                                            |\n"
        f"| Build identity                     | `{record['build_identity']['exact_build_identity_ref']}`                              |\n"
        f"| Release channel                    | `{record['build_identity']['release_channel_class']}`                                                           |\n"
        f"| Workspace version                  | {record['build_identity']['workspace_version']}                                                                 |\n"
        f"| Corpus manifest revision           | {record['corpus_manifest']['manifest_revision']}                                                                     |\n"
        f"| Protected metrics revision         | {record['protected_metrics']['metrics_file_revision']}                                                                     |\n"
        f"| Fitness-function catalog revision  | {record['fitness_function_catalog']['catalog_revision']}                                                                     |\n"
        f"| Hardware definition                | `{record['hardware_definition']['definition_id']}` (council-approved: {'yes' if record['hardware_definition']['is_council_approved_baseline'] else 'no'})  |\n"
        f"| Environment definition             | `{record['environment_definition']['definition_id']}`                                   |\n"
        f"| Display class                      | `{record['environment_definition']['display_class_id']}`                                  |\n"
        f"| Lab image                          | `{record['environment_definition']['lab_image_id']}` @ rev {record['environment_definition']['lab_image_revision']}                |\n"
        f"| Power / thermal posture            | `{record['environment_definition']['power_posture_id']}` / `{record['environment_definition']['thermal_posture_id']}` |\n"
        f"| Comparability                      | `{record['comparability']['comparability_class']}` (no quarantine reasons)                          |\n\n"
        + context_note
        + "## Row results\n\n"
        + "| Fitness row                          | Result        | Trend                         | Threshold mode                    | Notes                                                                                          |\n"
        + "|--------------------------------------|---------------|-------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------|\n"
        + "| `ff.warm_start_to_first_paint`       | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Trace digest seeded from `artifacts/traces/examples/full_scene.json`.                          |\n"
        + "| `ff.first_paint`                     | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Same trace digest; numeric SLO bars deferred to benchmark-council ratification.                |\n"
        + "| `ff.input_to_paint`                  | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Eight hot-path marks from the fixture scene.                                                   |\n"
        + "| `ff.buffer_operations`               | `pass`        | `unchanged`                   | `boolean_gate`                    | Undo-class correctness gate reads `artifacts/buffer/buffer_metrics_seed.json`.                 |\n"
        + "| `ff.vfs_save_conflict_handling`      | `pass`        | `unchanged`                   | `boolean_gate`                    | Compare-before-write floor held against the frozen VFS decision examples.                      |\n"
        + "| `ff.benchmark_lab_health`            | `pass`        | `unchanged`                   | `boolean_gate`                    | Self-audit across governance-packet, corpus-manifest, and fitness-catalog resolution.          |\n"
        + "| `ff.power_thermal_posture`           | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Data source `to_be_wired_by_benchmark_council`; row reserved by the fitness catalog.           |\n"
        + "| `ff.restore_fidelity`                | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Entry-restore harness not yet wired; reserves the onboarding-metric name.                      |\n"
        + "| `ff.command_parity`                  | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Command-graph parity harness not yet wired; ADR landing required.                              |\n\n"
        + "Row-count totals: **3 pass**, 0 warn, 0 fail, 0 not_measured, 0 waived,\n"
        + "**6 provisional**.\n\n"
        + "## Links\n\n"
        f"- Raw artifact: [`{summary['raw_artifact_ref']}`]({raw_link})\n"
        + (
            f"- Dashboard snapshot: [`{summary['dashboard_ref']}`]({dashboard_link})\n"
            if dashboard_link is not None
            else ""
        )
        + f"- Protected metrics: [`{PROTECTED_METRICS_REL}`]({protected_metrics_link})\n"
        + f"- Fitness-function catalog: [`{FITNESS_CATALOG_REL}`]({fitness_catalog_link})\n"
        + f"- Corpus manifest: [`{CORPUS_MANIFEST_REL}`]({corpus_manifest_link})\n"
        + f"- Reference hardware manifest: [`artifacts/perf/reference_hardware_manifest.yaml`]({hardware_manifest_link})\n"
        + f"- Lab-image manifest: [`artifacts/perf/lab_image_manifest.yaml`]({lab_image_manifest_link})\n"
        + f"- Self-capture parity guidance: [`docs/perf/self_capture_parity.md`]({parity_doc_link})\n"
        + f"- Trace bundle seed: [`artifacts/traces/examples/full_scene.json`]({trace_bundle_link})\n"
    )
    return header


def render_regression_markdown(record: dict[str, Any]) -> str:
    summary = record["summary"]
    report_ref = summary["human_readable_summary_ref"]
    raw_link = relative_ref(report_ref, summary["raw_artifact_ref"])
    dashboard_link = relative_ref(report_ref, summary["dashboard_ref"])
    protected_metrics_link = relative_ref(report_ref, PROTECTED_METRICS_REL)
    fitness_catalog_link = relative_ref(report_ref, FITNESS_CATALOG_REL)
    corpus_manifest_link = relative_ref(report_ref, CORPUS_MANIFEST_REL)
    hardware_manifest_link = relative_ref(
        report_ref, "artifacts/perf/reference_hardware_manifest.yaml"
    )
    lab_image_manifest_link = relative_ref(
        report_ref, "artifacts/perf/lab_image_manifest.yaml"
    )
    parity_doc_link = relative_ref(report_ref, "docs/perf/self_capture_parity.md")
    if record["run_context"] == "self_capture":
        regression_scope_note = (
            "artifact. It is `self_capture`, not a reference capture — the\n"
            "fail verdict below is a harness demonstration, not a release signal.\n\n"
        )
    else:
        regression_scope_note = (
            f"artifact. It is `{record['run_context']}` — the\n"
            "fail verdict below is still a harness demonstration, not a release signal.\n\n"
        )
    return (
        f"# Benchmark run: `{record['run_id']}`\n\n"
        "| Field                              | Value                                                                  |\n"
        "|------------------------------------|------------------------------------------------------------------------|\n"
        f"| Run context                        | `{record['run_context']}`                                                         |\n"
        f"| Lane                               | `{record['lane_class']}`                                                      |\n"
        f"| Trigger                            | `{record['trigger_kind']}`                                                 |\n"
        f"| Measured on                        | {record['measured_on']}                                                             |\n"
        f"| Build identity                     | `{record['build_identity']['exact_build_identity_ref']}`                         |\n"
        f"| Release channel                    | `{record['build_identity']['release_channel_class']}`                                                            |\n"
        f"| Workspace version                  | {record['build_identity']['workspace_version']}                                                                  |\n"
        f"| Corpus manifest revision           | {record['corpus_manifest']['manifest_revision']}                                                                      |\n"
        f"| Protected metrics revision         | {record['protected_metrics']['metrics_file_revision']}                                                                      |\n"
        f"| Fitness-function catalog revision  | {record['fitness_function_catalog']['catalog_revision']}                                                                      |\n"
        f"| Hardware definition                | `{record['hardware_definition']['definition_id']}` (council-approved: {'yes' if record['hardware_definition']['is_council_approved_baseline'] else 'no'})   |\n"
        f"| Environment definition             | `{record['environment_definition']['definition_id']}`                                    |\n"
        f"| Display class                      | `{record['environment_definition']['display_class_id']}`                                   |\n"
        f"| Lab image                          | `{record['environment_definition']['lab_image_id']}` @ rev {record['environment_definition']['lab_image_revision']}                 |\n"
        f"| Power / thermal posture            | `{record['environment_definition']['power_posture_id']}` / `{record['environment_definition']['thermal_posture_id']}`  |\n"
        f"| Comparability                      | `{record['comparability']['comparability_class']}` (no quarantine reasons)                           |\n\n"
        "**This run intentionally demonstrates the regression path.** The\n"
        "lane emits it so the benchmark-lab wrappers and the nightly workflow\n"
        "can show a non-zero exit code end to end, with a named fitness row,\n"
        "a named regression-trigger kind, and a pointer back to the raw\n"
        + regression_scope_note
        + "## Row results\n\n"
        + "| Fitness row                 | Result | Trend        | Threshold mode | Regression trigger    | Notes                                                                                   |\n"
        + "|-----------------------------|--------|--------------|----------------|-----------------------|-----------------------------------------------------------------------------------------|\n"
        + "| `ff.benchmark_lab_health`   | `fail` | `regressing` | `boolean_gate` | `corpus_row_missing`  | The harness pretended a cited corpus id failed to resolve; the gate reported a fail.    |\n\n"
        + "Row-count totals: 0 pass, 0 warn, **1 fail**, 0 not_measured, 0 waived,\n"
        + "0 provisional. The regression-trigger bucket increments\n"
        + "`corpus_row_missing` by one.\n\n"
        + "## Why this is the regression demonstration\n\n"
        + "`ff.benchmark_lab_health` is the fitness row whose threshold reads\n"
        + "\"every fixture cited by a benchmark report resolves to an id in the\n"
        + "corpus manifest\". The fixture id this run cites\n"
        + "(`corpus.workflow.startup_warm_to_first_paint`) resolves cleanly\n"
        + "against the real manifest revision — the `fail` here comes from the\n"
        + "harness deliberately flipping the boolean outcome on emit to exercise\n"
        + "the wiring. A real nightly lane that hit the same regression would\n"
        + "fail the run, record the `corpus_row_missing` trigger, and flag the\n"
        + "row on the dashboard under the same summary_ref that this file\n"
        + "carries.\n\n"
        + "## Links\n\n"
        f"- Raw artifact: [`{summary['raw_artifact_ref']}`]({raw_link})\n"
        + (
            f"- Dashboard snapshot: [`{summary['dashboard_ref']}`]({dashboard_link})\n"
            if dashboard_link is not None
            else ""
        )
        + f"- Protected metrics: [`{PROTECTED_METRICS_REL}`]({protected_metrics_link})\n"
        + f"- Fitness-function catalog row: `ff.benchmark_lab_health` in [`{FITNESS_CATALOG_REL}`]({fitness_catalog_link})\n"
        + f"- Corpus manifest: [`{CORPUS_MANIFEST_REL}`]({corpus_manifest_link})\n"
        + f"- Reference hardware manifest: [`artifacts/perf/reference_hardware_manifest.yaml`]({hardware_manifest_link})\n"
        + f"- Lab-image manifest: [`artifacts/perf/lab_image_manifest.yaml`]({lab_image_manifest_link})\n"
        + f"- Self-capture parity guidance: [`docs/perf/self_capture_parity.md`]({parity_doc_link})\n"
    )


def dashboard_snapshot(records: list[dict[str, Any]]) -> dict[str, Any]:
    source_runs = []
    for rec in records:
        source_runs.append(
            {
                "run_id": rec["run_id"],
                "run_context": rec["run_context"],
                "raw_artifact_ref": rec["summary"]["raw_artifact_ref"],
                "human_readable_summary_ref": rec["summary"][
                    "human_readable_summary_ref"
                ],
                "exact_build_identity_ref": rec["build_identity"][
                    "exact_build_identity_ref"
                ],
                "corpus_manifest_revision": rec["corpus_manifest"]["manifest_revision"],
                "protected_metrics_revision": rec["protected_metrics"][
                    "metrics_file_revision"
                ],
                "fitness_function_catalog_revision": rec["fitness_function_catalog"][
                    "catalog_revision"
                ],
                "hardware_definition_id": rec["hardware_definition"]["definition_id"],
                "is_council_approved_baseline": rec["hardware_definition"][
                    "is_council_approved_baseline"
                ],
                "environment_definition_id": rec["environment_definition"]["definition_id"],
                "display_class_id": rec["environment_definition"]["display_class_id"],
                "lab_image_id": rec["environment_definition"]["lab_image_id"],
                "lab_image_revision": rec["environment_definition"][
                    "lab_image_revision"
                ],
                "is_council_approved_reference_environment": rec[
                    "environment_definition"
                ]["is_council_approved_reference_environment"],
                "comparability_class": rec["comparability"]["comparability_class"],
                "row_count_by_result": rec["summary"]["row_count_by_result"],
                "regression_trigger_count_by_kind": rec["summary"][
                    "regression_trigger_count_by_kind"
                ],
            }
        )

    # by_fitness_row: latest record per fitness row across the input list.
    by_fitness_row: dict[str, dict[str, Any]] = {}
    for rec in records:
        for row in rec["rows"]:
            by_fitness_row[row["fitness_row_id"]] = {
                "latest_run_id": rec["run_id"],
                "result": row["result"],
                "trend_direction": row["trend_direction"],
                "threshold_mode": row["threshold_mode"],
                "comparability_class": rec["comparability"]["comparability_class"],
            }

    # Enforce deterministic key order keyed on the committed seed so
    # re-emits stay byte-stable.
    fitness_row_order = [
        "ff.warm_start_to_first_paint",
        "ff.first_paint",
        "ff.input_to_paint",
        "ff.buffer_operations",
        "ff.vfs_save_conflict_handling",
        "ff.benchmark_lab_health",
        "ff.power_thermal_posture",
        "ff.restore_fidelity",
        "ff.command_parity",
    ]
    by_fitness_row_ordered: dict[str, Any] = {}
    for key in fitness_row_order:
        if key in by_fitness_row:
            by_fitness_row_ordered[key] = by_fitness_row[key]
    for key, value in by_fitness_row.items():
        if key not in by_fitness_row_ordered:
            by_fitness_row_ordered[key] = value

    return {
        "schema_version": 1,
        "dashboard_id": "aureline.benchmark_lab_dashboard",
        "generated_at": pinned_epoch_timestamp(),
        "source_run_refs": source_runs,
        "by_fitness_row": by_fitness_row_ordered,
        "notes_ref": "benchmark_lab_dashboard_note.seed",
    }


def evidence_channels_self_capture() -> list[str]:
    return [
        "benchmark_lab.bootstrap_entry_parity",
        "benchmark_lab.text_stack",
        "benchmark_lab.large_file",
        "benchmark_lab.vfs_save_pipeline",
        "support_bundle.performance_summary",
        "release_evidence.claim_manifest",
    ]


def evidence_channels_regression_example() -> list[str]:
    return [
        "release_evidence.claim_manifest",
        "support_bundle.performance_summary",
    ]


def emit_seed_set(repo_root: Path, out_dir: Path) -> tuple[dict[str, Any], dict[str, Any], dict[str, Any]]:
    """Emit the two seeded run records plus the dashboard snapshot."""

    corpus_manifest = parse_corpus_manifest(
        read_text(repo_root / CORPUS_MANIFEST_REL)
    )
    protected_metrics = parse_protected_metrics(
        read_text(repo_root / PROTECTED_METRICS_REL)
    )
    fitness_catalog = parse_fitness_catalog(
        read_text(repo_root / FITNESS_CATALOG_REL)
    )
    # The committed seed always carries stable, repo-relative pointers into
    # artifacts/benchmarks/dashboard_seed/. verify_seed writes to a tmpdir but
    # still compares against those stable strings, so the embedded path is the
    # seed location regardless of where bytes land on disk.
    out_rel = SEED_OUT_DIR_REL

    self_rec = build_run_record(
        run_id="benchmark_run.seed.self_capture",
        run_context="self_capture",
        lane="developer_local",
        trigger="developer_invocation",
        corpus_manifest=corpus_manifest,
        protected_metrics=protected_metrics,
        fitness_catalog=fitness_catalog,
        rows=self_capture_rows(),
        evidence_channels=evidence_channels_self_capture(),
        notes_ref="benchmark_run_note.seed.self_capture",
        out_dir_rel=out_rel,
        dashboard_ref=f"{out_rel}/dashboard.json",
        environment_preset="self_capture_current_machine",
    )
    regr_rec = build_run_record(
        run_id="benchmark_run.seed.regression_example",
        run_context="self_capture",
        lane="developer_local",
        trigger="developer_invocation",
        corpus_manifest=corpus_manifest,
        protected_metrics=protected_metrics,
        fitness_catalog=fitness_catalog,
        rows=regression_example_rows(),
        evidence_channels=evidence_channels_regression_example(),
        notes_ref="benchmark_run_note.seed.regression_example",
        out_dir_rel=out_rel,
        dashboard_ref=f"{out_rel}/dashboard.json",
        environment_preset="self_capture_current_machine",
    )
    dashboard = dashboard_snapshot([self_rec, regr_rec])

    dump_json(out_dir / "raw" / f"{self_rec['run_id']}.json", self_rec)
    dump_json(out_dir / "raw" / f"{regr_rec['run_id']}.json", regr_rec)
    dump_json(out_dir / "dashboard.json", dashboard)

    (out_dir / "report").mkdir(parents=True, exist_ok=True)
    (out_dir / "report" / f"{self_rec['run_id']}.md").write_text(
        render_self_capture_markdown(self_rec), encoding="utf-8"
    )
    (out_dir / "report" / f"{regr_rec['run_id']}.md").write_text(
        render_regression_markdown(regr_rec), encoding="utf-8"
    )

    return self_rec, regr_rec, dashboard


def verify_seed(repo_root: Path) -> int:
    committed_dir = repo_root / SEED_OUT_DIR_REL
    with tempfile.TemporaryDirectory() as tmp:
        emit_dir = Path(tmp) / "emit"
        emit_seed_set(repo_root, emit_dir)
        mismatches: list[str] = []
        relative_paths = [
            "raw/benchmark_run.seed.self_capture.json",
            "raw/benchmark_run.seed.regression_example.json",
            "dashboard.json",
            "report/benchmark_run.seed.self_capture.md",
            "report/benchmark_run.seed.regression_example.md",
        ]
        for rel in relative_paths:
            committed = (committed_dir / rel).read_text(encoding="utf-8")
            fresh = (emit_dir / rel).read_text(encoding="utf-8")
            if committed != fresh:
                diff = "\n".join(
                    difflib.unified_diff(
                        committed.splitlines(),
                        fresh.splitlines(),
                        fromfile=f"committed:{rel}",
                        tofile=f"fresh:{rel}",
                        lineterm="",
                    )
                )
                mismatches.append(f"{rel}:\n{diff}")
    if mismatches:
        for m in mismatches:
            print(m, file=sys.stderr)
        print(
            "\nbenchmark_lab_emit: committed seed under "
            f"{SEED_OUT_DIR_REL}/ does not match fresh emission; refresh "
            "the seed or fix the emitter so the lane stays honest.",
            file=sys.stderr,
        )
        return 1
    print("benchmark_lab_emit: committed seed matches fresh emission")
    return 0


def resolve_out_dir_rel(repo_root: Path, out_dir: Path) -> str:
    try:
        return out_dir.relative_to(repo_root).as_posix()
    except ValueError:
        # Developer pointed --out-dir outside the repo (e.g. a tmpdir);
        # fall back to the absolute posix path so the record's raw and
        # report refs still resolve on the producing host.
        return out_dir.as_posix()


def emit_standard_run(
    repo_root: Path,
    out_dir: Path,
    *,
    run_context: str,
    lane: str,
    trigger: str,
    corpus_subset: str,
    environment_preset: str,
) -> int:
    corpus_manifest = parse_corpus_manifest(
        read_text(repo_root / CORPUS_MANIFEST_REL)
    )
    protected_metrics = parse_protected_metrics(
        read_text(repo_root / PROTECTED_METRICS_REL)
    )
    fitness_catalog = parse_fitness_catalog(
        read_text(repo_root / FITNESS_CATALOG_REL)
    )

    run_id_suffix = corpus_subset if corpus_subset == "smoke" else "full"
    run_id = f"benchmark_run.local.{run_context}.{run_id_suffix}"
    rec = build_run_record(
        run_id=run_id,
        run_context=run_context,
        lane=lane,
        trigger=trigger,
        corpus_manifest=corpus_manifest,
        protected_metrics=protected_metrics,
        fitness_catalog=fitness_catalog,
        rows=self_capture_rows(),
        evidence_channels=evidence_channels_self_capture(),
        notes_ref=f"benchmark_run_note.local.{run_context}.{run_id_suffix}",
        out_dir_rel=resolve_out_dir_rel(repo_root, out_dir),
        dashboard_ref=None,
        environment_preset=environment_preset,
    )
    dump_json(out_dir / "raw" / f"{run_id}.json", rec)
    (out_dir / "report").mkdir(parents=True, exist_ok=True)
    (out_dir / "report" / f"{run_id}.md").write_text(
        render_self_capture_markdown(rec), encoding="utf-8"
    )
    print(f"benchmark_lab_emit: wrote {out_dir / 'raw' / f'{run_id}.json'}")
    return 0


def emit_regression_run(
    repo_root: Path,
    out_dir: Path,
    *,
    run_context: str,
    lane: str,
    trigger: str,
    environment_preset: str,
) -> int:
    corpus_manifest = parse_corpus_manifest(
        read_text(repo_root / CORPUS_MANIFEST_REL)
    )
    protected_metrics = parse_protected_metrics(
        read_text(repo_root / PROTECTED_METRICS_REL)
    )
    fitness_catalog = parse_fitness_catalog(
        read_text(repo_root / FITNESS_CATALOG_REL)
    )
    run_id = "benchmark_run.local.regression_demo"
    rec = build_run_record(
        run_id=run_id,
        run_context=run_context,
        lane=lane,
        trigger=trigger,
        corpus_manifest=corpus_manifest,
        protected_metrics=protected_metrics,
        fitness_catalog=fitness_catalog,
        rows=regression_example_rows(),
        evidence_channels=evidence_channels_regression_example(),
        notes_ref=f"benchmark_run_note.local.regression_demo",
        out_dir_rel=resolve_out_dir_rel(repo_root, out_dir),
        dashboard_ref=None,
        environment_preset=environment_preset,
    )
    dump_json(out_dir / "raw" / f"{run_id}.json", rec)
    (out_dir / "report").mkdir(parents=True, exist_ok=True)
    (out_dir / "report" / f"{run_id}.md").write_text(
        render_regression_markdown(rec), encoding="utf-8"
    )
    print(
        f"benchmark_lab_emit: wrote {out_dir / 'raw' / f'{run_id}.json'} "
        "(fail: one row tripped `corpus_row_missing` on ff.benchmark_lab_health)",
        file=sys.stderr,
    )
    return 1


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if args.verify_seed:
        return verify_seed(repo_root)
    if args.out_dir is None:
        raise SystemExit("--out-dir is required unless --verify-seed is set")
    out_dir = Path(args.out_dir)
    if not out_dir.is_absolute():
        out_dir = (repo_root / out_dir).resolve()
    else:
        out_dir = out_dir.resolve()
    if args.regression_demo:
        return emit_regression_run(
            repo_root,
            out_dir,
            run_context=args.run_context,
            lane=args.lane,
            trigger=args.trigger,
            environment_preset=args.environment_preset,
        )
    return emit_standard_run(
        repo_root,
        out_dir,
        run_context=args.run_context,
        lane=args.lane,
        trigger=args.trigger,
        corpus_subset=args.corpus_subset,
        environment_preset=args.environment_preset,
    )


if __name__ == "__main__":
    sys.exit(main())
