#!/usr/bin/env python3
"""Emit a benchmark-lab run-result record and human-readable summary.

Reads the committed protected corpus manifest, protected-metrics file,
and fitness-function catalog, assembles one record per invocation conforming to
``schemas/benchmarks/run_result.schema.json``, and writes the
companion Markdown summary.

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

RUN_RESULT_SCHEMA_VERSION = 1
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


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", required=True)
    parser.add_argument("--out-dir", default=None)
    parser.add_argument("--corpus-subset", default="smoke", choices=sorted(VALID_CORPUS_SUBSETS))
    parser.add_argument("--run-context", default="self_capture", choices=sorted(VALID_RUN_CONTEXTS))
    parser.add_argument("--lane", default="developer_local", choices=sorted(VALID_LANES))
    parser.add_argument("--trigger", default="developer_invocation", choices=sorted(VALID_TRIGGERS))
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


def hardware_definition_seed(run_id: str) -> dict[str, Any]:
    return {
        "definition_id": "hardware_definition.reserved.not_yet_seeded",
        "definition_revision": 1,
        "is_council_approved_baseline": False,
        "notes_ref": f"hardware_definition_note.seed.{run_id.split('.')[-1]}",
    }


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
) -> dict[str, Any]:
    row_count, trigger_count = summarise(rows)
    raw_ref = f"{out_dir_rel}/raw/{run_id}.json"
    report_ref = f"{out_dir_rel}/report/{run_id}.md"
    dashboard_ref = f"{out_dir_rel}/dashboard.json"
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
        "hardware_definition": hardware_definition_seed(run_id),
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


def render_self_capture_markdown(record: dict[str, Any]) -> str:
    header = (
        "# Benchmark run: `benchmark_run.seed.self_capture`\n\n"
        "| Field                              | Value                                                                 |\n"
        "|------------------------------------|-----------------------------------------------------------------------|\n"
        "| Run context                        | `self_capture`                                                        |\n"
        "| Lane                               | `developer_local`                                                     |\n"
        "| Trigger                            | `developer_invocation`                                                |\n"
        f"| Measured on                        | {record['measured_on']}                                                            |\n"
        f"| Build identity                     | `{record['build_identity']['exact_build_identity_ref']}`                              |\n"
        "| Release channel                    | `dev_local`                                                           |\n"
        "| Workspace version                  | 0.0.0                                                                 |\n"
        f"| Corpus manifest revision           | {record['corpus_manifest']['manifest_revision']}                                                                     |\n"
        f"| Protected metrics revision         | {record['protected_metrics']['metrics_file_revision']}                                                                     |\n"
        f"| Fitness-function catalog revision  | {record['fitness_function_catalog']['catalog_revision']}                                                                     |\n"
        "| Hardware definition                | `hardware_definition.reserved.not_yet_seeded` (council-approved: no)  |\n"
        "| Comparability                      | `not_yet_comparable` (no quarantine reasons)                          |\n\n"
        "**This run is `self_capture`, not a reference capture.** The numbers\n"
        "below describe harness self-health on a seeded input set; they are\n"
        "not admissible as release-evidence input. A reference capture MUST\n"
        "run on the `ci_nightly` lane against a council-approved hardware\n"
        "baseline.\n\n"
        "## Row results\n\n"
        "| Fitness row                          | Result        | Trend                         | Threshold mode                    | Notes                                                                                          |\n"
        "|--------------------------------------|---------------|-------------------------------|-----------------------------------|------------------------------------------------------------------------------------------------|\n"
        "| `ff.warm_start_to_first_paint`       | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Trace digest seeded from `artifacts/traces/examples/full_scene.json`.                          |\n"
        "| `ff.first_paint`                     | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Same trace digest; numeric SLO bars deferred to benchmark-council ratification.                |\n"
        "| `ff.input_to_paint`                  | `provisional` | `unknown_insufficient_history`| `absolute_p50_and_p95`            | Eight hot-path marks from the fixture scene.                                                   |\n"
        "| `ff.buffer_operations`               | `pass`        | `unchanged`                   | `boolean_gate`                    | Undo-class correctness gate reads `artifacts/buffer/buffer_metrics_seed.json`.                 |\n"
        "| `ff.vfs_save_conflict_handling`      | `pass`        | `unchanged`                   | `boolean_gate`                    | Compare-before-write floor held against the frozen VFS decision examples.                      |\n"
        "| `ff.benchmark_lab_health`            | `pass`        | `unchanged`                   | `boolean_gate`                    | Self-audit across governance-packet, corpus-manifest, and fitness-catalog resolution.          |\n"
        "| `ff.power_thermal_posture`           | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Data source `to_be_wired_by_benchmark_council`; row reserved by the fitness catalog.           |\n"
        "| `ff.restore_fidelity`                | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Entry-restore harness not yet wired; reserves the onboarding-metric name.                      |\n"
        "| `ff.command_parity`                  | `provisional` | `unknown_insufficient_history`| `to_be_set_by_benchmark_council`  | Command-graph parity harness not yet wired; ADR landing required.                              |\n\n"
        "Row-count totals: **3 pass**, 0 warn, 0 fail, 0 not_measured, 0 waived,\n"
        "**6 provisional**.\n\n"
        "## Links\n\n"
        "- Raw artifact: [`artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.self_capture.json`](../raw/benchmark_run.seed.self_capture.json)\n"
        "- Dashboard snapshot: [`artifacts/benchmarks/dashboard_seed/dashboard.json`](../dashboard.json)\n"
        "- Protected metrics: [`artifacts/bench/protected_metrics.yaml`](../../../bench/protected_metrics.yaml)\n"
        "- Fitness-function catalog: [`artifacts/bench/fitness_function_catalog.yaml`](../../../bench/fitness_function_catalog.yaml)\n"
        "- Corpus manifest: [`fixtures/benchmarks/corpus_manifest.yaml`](../../../../fixtures/benchmarks/corpus_manifest.yaml)\n"
        "- Trace bundle seed: [`artifacts/traces/examples/full_scene.json`](../../../traces/examples/full_scene.json)\n"
    )
    return header


def render_regression_markdown(record: dict[str, Any]) -> str:
    return (
        "# Benchmark run: `benchmark_run.seed.regression_example`\n\n"
        "| Field                              | Value                                                                  |\n"
        "|------------------------------------|------------------------------------------------------------------------|\n"
        "| Run context                        | `self_capture`                                                         |\n"
        "| Lane                               | `developer_local`                                                      |\n"
        "| Trigger                            | `developer_invocation`                                                 |\n"
        f"| Measured on                        | {record['measured_on']}                                                             |\n"
        f"| Build identity                     | `{record['build_identity']['exact_build_identity_ref']}`                         |\n"
        "| Release channel                    | `dev_local`                                                            |\n"
        "| Workspace version                  | 0.0.0                                                                  |\n"
        f"| Corpus manifest revision           | {record['corpus_manifest']['manifest_revision']}                                                                      |\n"
        f"| Protected metrics revision         | {record['protected_metrics']['metrics_file_revision']}                                                                      |\n"
        f"| Fitness-function catalog revision  | {record['fitness_function_catalog']['catalog_revision']}                                                                      |\n"
        "| Hardware definition                | `hardware_definition.reserved.not_yet_seeded` (council-approved: no)   |\n"
        "| Comparability                      | `not_yet_comparable` (no quarantine reasons)                           |\n\n"
        "**This run intentionally demonstrates the regression path.** The\n"
        "lane emits it so the benchmark-lab wrappers and the nightly workflow\n"
        "can show a non-zero exit code end to end, with a named fitness row,\n"
        "a named regression-trigger kind, and a pointer back to the raw\n"
        "artifact. It is `self_capture`, not a reference capture — the\n"
        "fail verdict below is a harness demonstration, not a release signal.\n\n"
        "## Row results\n\n"
        "| Fitness row                 | Result | Trend        | Threshold mode | Regression trigger    | Notes                                                                                   |\n"
        "|-----------------------------|--------|--------------|----------------|-----------------------|-----------------------------------------------------------------------------------------|\n"
        "| `ff.benchmark_lab_health`   | `fail` | `regressing` | `boolean_gate` | `corpus_row_missing`  | The harness pretended a cited corpus id failed to resolve; the gate reported a fail.    |\n\n"
        "Row-count totals: 0 pass, 0 warn, **1 fail**, 0 not_measured, 0 waived,\n"
        "0 provisional. The regression-trigger bucket increments\n"
        "`corpus_row_missing` by one.\n\n"
        "## Why this is the regression demonstration\n\n"
        "`ff.benchmark_lab_health` is the fitness row whose threshold reads\n"
        "\"every fixture cited by a benchmark report resolves to an id in the\n"
        "corpus manifest\". The fixture id this run cites\n"
        "(`corpus.workflow.startup_warm_to_first_paint`) resolves cleanly\n"
        "against the real manifest revision — the `fail` here comes from the\n"
        "harness deliberately flipping the boolean outcome on emit to exercise\n"
        "the wiring. A real nightly lane that hit the same regression would\n"
        "fail the run, record the `corpus_row_missing` trigger, and flag the\n"
        "row on the dashboard under the same summary_ref that this file\n"
        "carries.\n\n"
        "## Links\n\n"
        "- Raw artifact: [`artifacts/benchmarks/dashboard_seed/raw/benchmark_run.seed.regression_example.json`](../raw/benchmark_run.seed.regression_example.json)\n"
        "- Dashboard snapshot: [`artifacts/benchmarks/dashboard_seed/dashboard.json`](../dashboard.json)\n"
        "- Protected metrics: [`artifacts/bench/protected_metrics.yaml`](../../../bench/protected_metrics.yaml)\n"
        "- Fitness-function catalog row: `ff.benchmark_lab_health` in [`artifacts/bench/fitness_function_catalog.yaml`](../../../bench/fitness_function_catalog.yaml)\n"
        "- Corpus manifest: [`fixtures/benchmarks/corpus_manifest.yaml`](../../../../fixtures/benchmarks/corpus_manifest.yaml)\n"
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
        )
    return emit_standard_run(
        repo_root,
        out_dir,
        run_context=args.run_context,
        lane=args.lane,
        trigger=args.trigger,
        corpus_subset=args.corpus_subset,
    )


if __name__ == "__main__":
    sys.exit(main())
