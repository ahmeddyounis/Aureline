#!/usr/bin/env python3
"""Unattended nightly hot-path fitness gate runner.

Reads the canonical gate definition from
``ci/perf/nightly_hot_path.yml`` and joins it against the latest
unattended hot-path trace-suite capture
(``artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json``)
to project one regression-status row per (gate metric row, reference
hardware row) cell.

The runner stays unattended-runnable: it never opens a window, never
runs the live shell binary, and is byte-stable across reruns when the
inputs are unchanged. It emits:

- ``artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json``
  — the durable JSON capture pinned to the exact-build identity, the
  trace-suite capture, the fitness-function catalog rows, and the gate
  revision.
- ``dashboards/m1/hot_path_fitness.json`` — the dashboard snapshot the
  M1 exit packet embeds: one row per fitness function carrying current
  value, prior baseline, warning band, regression floor, active waiver
  state, owner, and owning lane.

The runner exits non-zero if any protected gate row crosses its
regression floor without a non-null waiver record, if any
``upstream_contracts`` ref does not resolve, if the latest trace-suite
capture is missing or its scenario coverage drifts from the gate
definition, or if the named failure drill does not reproduce a typed
regression-fail finding.

YAML decoding follows the existing repository convention: gate / matrix
files are parsed via Ruby/Psych, which is already required by other CI
checks and avoids a Python YAML dependency.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_GATE_REL = "ci/perf/nightly_hot_path.yml"
DEFAULT_TRACE_CAPTURE_REL = (
    "artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/nightly_hot_path_validation_capture.json"
)
DEFAULT_DASHBOARD_REL = "dashboards/m1/hot_path_fitness.json"

REGRESSION_STATUSES = {
    "pass",
    "warn",
    "fail",
    "waived",
    "missing_observation",
    "pending_scenario_seed",
}

WAIVER_STATES = {
    "no_active_waiver",
    "active_waiver",
    "expired_waiver",
    "threshold_provisional_pending_council",
}


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--repo-root",
        default=".",
        help="Repository root (must contain a .git directory).",
    )
    parser.add_argument(
        "--gate",
        default=DEFAULT_GATE_REL,
        help="Gate definition YAML, repo-relative.",
    )
    parser.add_argument(
        "--trace-capture",
        default=DEFAULT_TRACE_CAPTURE_REL,
        help="Latest hot-path trace-suite validation capture, repo-relative.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Build identity record JSON, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture, repo-relative.",
    )
    parser.add_argument(
        "--dashboard",
        default=DEFAULT_DASHBOARD_REL,
        help="Where to write the dashboard snapshot, repo-relative.",
    )
    parser.add_argument(
        "--force-drill",
        action="store_true",
        help=(
            "Replay the gate definition's failure drill (override the named "
            "row's observed value) and assert the expected check_id is "
            "produced. Used by the lane's failure-drill validation."
        ),
    )
    return parser.parse_args()


def render_yaml_as_json(path: Path) -> Any:
    if not path.exists():
        raise SystemExit(f"missing YAML file: {path}")
    ruby = subprocess.run(
        [
            "ruby",
            "-rjson",
            "-ryaml",
            "-e",
            (
                "payload = YAML.safe_load(File.read(ARGV[0]), permitted_classes: [], aliases: false); "
                "STDOUT.write(JSON.generate(payload))"
            ),
            str(path),
        ],
        capture_output=True,
        text=True,
    )
    if ruby.returncode != 0:
        stderr = ruby.stderr.strip() or "unknown Ruby/Psych failure"
        raise SystemExit(f"failed to parse YAML at {path} via Ruby/Psych: {stderr}")
    try:
        return json.loads(ruby.stdout)
    except json.JSONDecodeError as exc:
        raise SystemExit(f"Ruby/Psych emitted invalid JSON for {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a YAML mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a YAML list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    path = ref.split("#", 1)[0].strip()
    if not path:
        return False
    return (repo_root / path).exists()


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_json(path: Path, label: str) -> dict[str, Any]:
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    return ensure_dict(json.loads(path.read_text(encoding="utf-8")), label)


def project_status(
    *,
    observed_ms: int | None,
    target_ms: int,
    warning_band_ms: int,
    regression_floor_ms: int,
    waiver_state: str,
    waiver_record_ref: str | None,
    coverage_state: str,
) -> tuple[str, str | None]:
    """Project a regression status + optional failed check_id.

    Returns (status, check_id_when_failing). ``check_id_when_failing`` is
    non-None only when the row's status is ``fail`` and the gate should
    propagate a failed-check finding to the durable capture.
    """
    if coverage_state == "pending_scenario_seed":
        return "pending_scenario_seed", None
    if observed_ms is None:
        return "missing_observation", "nightly_hot_path.observation_missing"
    if observed_ms > regression_floor_ms:
        if waiver_state == "active_waiver" and waiver_record_ref:
            return "waived", None
        return "fail", "nightly_hot_path.regression_floor.exceeded"
    if observed_ms > warning_band_ms:
        return "warn", None
    return "pass", None


def join_trace_observation(
    *,
    trace_packets: list[dict[str, Any]],
    scenario_id: str | None,
    metric_id: str,
    hardware_row_id: str,
) -> tuple[int | None, dict[str, Any] | None]:
    """Find the observed_ms for (scenario, metric, hardware row).

    Returns (observed_ms, matched_packet) — observed_ms is None when no
    matching packet/metric is found.
    """
    if scenario_id is None:
        return None, None
    for packet in trace_packets:
        if packet.get("scenario_id") != scenario_id:
            continue
        if packet.get("hardware_row_id") != hardware_row_id:
            continue
        for metric in ensure_list(packet.get("metrics", []), "trace_packet.metrics"):
            if not isinstance(metric, dict):
                continue
            if metric.get("metric_id") != metric_id:
                continue
            observed = metric.get("observed_ms")
            if isinstance(observed, int):
                return observed, packet
            return None, packet
    return None, None


def hardware_rows_for_scenario(
    trace_packets: list[dict[str, Any]], scenario_id: str
) -> list[str]:
    seen: list[str] = []
    for packet in trace_packets:
        if packet.get("scenario_id") != scenario_id:
            continue
        row = packet.get("hardware_row_id")
        if isinstance(row, str) and row not in seen:
            seen.append(row)
    return seen


def all_hardware_rows(trace_packets: list[dict[str, Any]]) -> list[str]:
    seen: list[str] = []
    for packet in trace_packets:
        row = packet.get("hardware_row_id")
        if isinstance(row, str) and row not in seen:
            seen.append(row)
    return seen


def project_dashboard_cell(
    *,
    gate_row: dict[str, Any],
    hardware_row_id: str,
    observed_ms: int | None,
    matched_packet: dict[str, Any] | None,
    status: str,
    exact_build_identity_token: str,
) -> dict[str, Any]:
    threshold = ensure_dict(gate_row.get("threshold"), "gate_row.threshold")
    waiver = ensure_dict(gate_row.get("waiver"), "gate_row.waiver")
    cell: dict[str, Any] = {
        "gate_row_id": gate_row["gate_row_id"],
        "title": gate_row["title"],
        "fitness_function_row_ref": gate_row["fitness_function_row_ref"],
        "scenario_id": gate_row.get("scenario_id"),
        "metric_id": gate_row["metric_id"],
        "budget_ref": gate_row["budget_ref"],
        "owner_dri": gate_row["owner_dri"],
        "owning_lane": gate_row["owning_lane"],
        "coverage_state": gate_row["coverage_state"],
        "hardware_row_id": hardware_row_id,
        "current_value_ms": observed_ms,
        "prior_baseline_ms": (
            matched_packet.get("metrics", [{}])[0].get("bucket_p50_ms")
            if matched_packet
            else None
        ),
        "target_ms": ensure_int(threshold.get("target_ms"), "threshold.target_ms"),
        "warning_band_ms": ensure_int(
            threshold.get("warning_band_ms"), "threshold.warning_band_ms"
        ),
        "regression_floor_ms": ensure_int(
            threshold.get("regression_floor_ms"),
            "threshold.regression_floor_ms",
        ),
        "regression_status": status,
        "waiver_state": waiver["waiver_state"],
        "waiver_record_ref": waiver.get("waiver_record_ref"),
        "waiver_authority_ref": waiver.get("waiver_authority_ref"),
        "waiver_expiry_at": waiver.get("expiry_at"),
        "exact_build_identity_token": exact_build_identity_token,
    }
    return cell


def derive_overall_status(rows: list[dict[str, Any]], findings: list[Finding]) -> str:
    if any(f.severity == "error" for f in findings):
        return "FAIL"
    if any(row["regression_status"] == "fail" for row in rows):
        return "FAIL"
    return "PASS"


def evaluate_gate(
    *,
    gate: dict[str, Any],
    trace_capture: dict[str, Any],
    exact_build_identity_token: str,
    force_drill: bool,
) -> tuple[list[dict[str, Any]], list[Finding], dict[str, Any]]:
    findings: list[Finding] = []
    trace_packets = ensure_list(
        trace_capture.get("trace_packets"), "trace_capture.trace_packets"
    )

    gate_rows = ensure_list(
        gate.get("protected_metric_rows"), "gate.protected_metric_rows"
    )
    if not gate_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.gate.empty",
                message="gate definition must declare at least one protected_metric_row",
                remediation="Seed the protected gate rows for startup, typing, quick-open, save, and recovery.",
            )
        )

    forced = None
    if force_drill:
        drill = ensure_dict(gate.get("failure_drill"), "gate.failure_drill")
        forced_input = ensure_dict(drill.get("forced_input"), "failure_drill.forced_input")
        forced = {
            "gate_row_id": ensure_str(
                forced_input.get("gate_row_id"),
                "failure_drill.forced_input.gate_row_id",
            ),
            "hardware_row_id": ensure_str(
                forced_input.get("hardware_row_id"),
                "failure_drill.forced_input.hardware_row_id",
            ),
            "forced_observed_ms": ensure_int(
                forced_input.get("forced_observed_ms"),
                "failure_drill.forced_input.forced_observed_ms",
            ),
        }

    rendered: list[dict[str, Any]] = []
    rendered_gate_row_ids: set[str] = set()
    for idx, row in enumerate(gate_rows):
        row = ensure_dict(row, f"gate.protected_metric_rows[{idx}]")
        gate_row_id = ensure_str(
            row.get("gate_row_id"), f"gate.protected_metric_rows[{idx}].gate_row_id"
        )
        rendered_gate_row_ids.add(gate_row_id)
        scenario_id = row.get("scenario_id")
        metric_id = ensure_str(row.get("metric_id"), "gate_row.metric_id")
        coverage_state = ensure_str(
            row.get("coverage_state"), "gate_row.coverage_state"
        )
        waiver = ensure_dict(row.get("waiver"), "gate_row.waiver")
        waiver_state = ensure_str(waiver.get("waiver_state"), "waiver.waiver_state")
        if waiver_state not in WAIVER_STATES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="nightly_hot_path.waiver_state.unknown",
                    message=f"unknown waiver_state {waiver_state!r} on {gate_row_id}",
                    remediation="Restrict waiver_state to the closed vocabulary in the gate file.",
                    ref=gate_row_id,
                )
            )

        # Determine which hardware rows to project against.
        if scenario_id is None:
            target_hw_rows: list[str] = [None]  # type: ignore[list-item]
        else:
            target_hw_rows = hardware_rows_for_scenario(trace_packets, scenario_id)
            if not target_hw_rows and coverage_state != "pending_scenario_seed":
                findings.append(
                    Finding(
                        severity="error",
                        check_id="nightly_hot_path.scenario.not_in_trace_suite",
                        message=(
                            f"gate row {gate_row_id} names scenario {scenario_id!r} "
                            "but no hot-path trace-suite packet covers it"
                        ),
                        remediation=(
                            "Seed the missing scenario in "
                            "benchmarks/m1/hot_path_trace_suite/scenarios.yaml "
                            "and re-run the trace suite, or fix the scenario_id."
                        ),
                        ref=gate_row_id,
                    )
                )

        for hw_row_id in target_hw_rows:
            observed_ms, matched_packet = join_trace_observation(
                trace_packets=trace_packets,
                scenario_id=scenario_id,
                metric_id=metric_id,
                hardware_row_id=hw_row_id or "",
            )

            # Apply the failure-drill override if present.
            if (
                forced is not None
                and forced["gate_row_id"] == gate_row_id
                and forced["hardware_row_id"] == hw_row_id
            ):
                observed_ms = forced["forced_observed_ms"]

            threshold = ensure_dict(row.get("threshold"), "gate_row.threshold")
            status, check_id = project_status(
                observed_ms=observed_ms,
                target_ms=ensure_int(
                    threshold.get("target_ms"), "threshold.target_ms"
                ),
                warning_band_ms=ensure_int(
                    threshold.get("warning_band_ms"),
                    "threshold.warning_band_ms",
                ),
                regression_floor_ms=ensure_int(
                    threshold.get("regression_floor_ms"),
                    "threshold.regression_floor_ms",
                ),
                waiver_state=waiver_state,
                waiver_record_ref=waiver.get("waiver_record_ref"),
                coverage_state=coverage_state,
            )
            if status not in REGRESSION_STATUSES:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="nightly_hot_path.regression_status.unknown",
                        message=(
                            f"runner produced unknown regression_status {status!r} "
                            f"for {gate_row_id}"
                        ),
                        remediation="Restrict status to the closed gate vocabulary.",
                        ref=gate_row_id,
                    )
                )

            if check_id == "nightly_hot_path.regression_floor.exceeded":
                findings.append(
                    Finding(
                        severity="error",
                        check_id=check_id,
                        message=(
                            f"gate row {gate_row_id} on {hw_row_id} observed "
                            f"{observed_ms} ms, exceeding regression floor "
                            f"{threshold.get('regression_floor_ms')} ms with no "
                            "active waiver"
                        ),
                        remediation=(
                            "Bisect the regression on the named scenario, fix or mint "
                            "a waiver through the performance council, and re-run "
                            "the nightly hot-path lane."
                        ),
                        ref=gate_row_id,
                        details={
                            "hardware_row_id": hw_row_id,
                            "scenario_id": scenario_id,
                            "observed_ms": observed_ms,
                            "regression_floor_ms": threshold.get(
                                "regression_floor_ms"
                            ),
                            "owner_dri": row["owner_dri"],
                            "owning_lane": row["owning_lane"],
                        },
                    )
                )

            cell = project_dashboard_cell(
                gate_row=row,
                hardware_row_id=hw_row_id or "n/a",
                observed_ms=observed_ms,
                matched_packet=matched_packet,
                status=status,
                exact_build_identity_token=exact_build_identity_token,
            )
            rendered.append(cell)

    # Required-coverage check.
    required = sorted(
        ensure_str(s, "gate.required_gate_row_coverage[]")
        for s in ensure_list(
            gate.get("required_gate_row_coverage"),
            "gate.required_gate_row_coverage",
        )
    )
    missing = sorted(set(required) - rendered_gate_row_ids)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.required_coverage.missing",
                message=(
                    "gate must render every required_gate_row_coverage id; "
                    f"missing: {missing}"
                ),
                remediation=(
                    "Add the missing gate_row_id entries to "
                    "protected_metric_rows or drop them from required coverage."
                ),
            )
        )

    drill_record: dict[str, Any] = {}
    if force_drill:
        drill = ensure_dict(gate.get("failure_drill"), "gate.failure_drill")
        expected = ensure_dict(
            drill.get("expected_report"), "failure_drill.expected_report"
        )
        expected_check_id = ensure_str(
            expected.get("expected_check_id"),
            "failure_drill.expected_report.expected_check_id",
        )
        expected_overall = ensure_str(
            expected.get("expected_overall_status"),
            "failure_drill.expected_report.expected_overall_status",
        )
        reproduced = any(
            f.severity == "error" and f.check_id == expected_check_id
            for f in findings
        )
        replay_overall = derive_overall_status(rendered, findings)
        drill_record = {
            "drill_id": ensure_str(drill.get("drill_id"), "failure_drill.drill_id"),
            "forced_input": ensure_dict(
                drill.get("forced_input"), "failure_drill.forced_input"
            ),
            "expected_report": expected,
            "reproduced_expected_check_id": reproduced,
            "replay_overall_status": replay_overall,
            "replay_passed": (
                reproduced and replay_overall == expected_overall
            ),
        }

    return rendered, findings, drill_record


def write_dashboard(
    *,
    repo_root: Path,
    dashboard_rel: str,
    gate: dict[str, Any],
    rows: list[dict[str, Any]],
    overall_status: str,
    trace_capture: dict[str, Any],
    exact_build_identity_token: str,
) -> str:
    dashboard = {
        "schema": "aureline.nightly_hot_path_dashboard.v1",
        "schema_version": 1,
        "dashboard_id": "aureline.dashboards.m1.hot_path_fitness",
        "gate_id": gate["gate_id"],
        "gate_revision": gate["gate_revision"],
        "owner_dri": gate["owner_dri"],
        "reviewer_entrypoint": gate["reviewer_entrypoint"],
        "exact_build_identity_token": exact_build_identity_token,
        "exact_build_identity_ref": gate["upstream_contracts"][
            "exact_build_identity_ref"
        ],
        "trace_capture_ref": (
            "artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json"
        ),
        "trace_capture_captured_at": trace_capture.get("captured_at"),
        "overall_status": overall_status,
        "generated_at": now_iso_z(),
        "rows": rows,
    }
    path = repo_root / dashboard_rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(
        json.dumps(dashboard, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    return dashboard_rel


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    findings: list[Finding] = []

    gate = ensure_dict(render_yaml_as_json(repo_root / args.gate), args.gate)
    if gate.get("schema_version") != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.schema_version",
                message=(
                    f"gate.schema_version must be the integer 1, got "
                    f"{gate.get('schema_version')!r}"
                ),
                remediation="Bump the runner together with the gate file if the schema changes.",
            )
        )

    upstream = ensure_dict(
        gate.get("upstream_contracts"), "gate.upstream_contracts"
    )
    for key, value in upstream.items():
        ref = ensure_str(value, f"gate.upstream_contracts.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="nightly_hot_path.upstream_contracts.missing",
                    message=f"upstream_contracts.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        gate.get("validation_lane_ref"), "gate.validation_lane_ref"
    )
    if not artifact_ref_exists(repo_root, validation_lane_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.validation_lane_ref.missing",
                message=f"validation_lane_ref does not resolve: {validation_lane_ref}",
                remediation="Fix the path or register the lane in the QE test-lane registry.",
                ref=validation_lane_ref,
            )
        )

    trace_capture_path = repo_root / args.trace_capture
    trace_capture = load_json(trace_capture_path, args.trace_capture)
    if trace_capture.get("capture_kind") != "hot_path_trace_suite_validation_capture":
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.trace_capture.wrong_kind",
                message=(
                    f"{args.trace_capture} is not a hot_path_trace_suite_validation_capture"
                ),
                remediation="Regenerate the trace-suite capture before running the gate.",
                ref=args.trace_capture,
            )
        )
    if trace_capture.get("status") != "PASS":
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.trace_capture.not_passing",
                message=(
                    "trace-suite capture must be PASS before the gate joins it; "
                    f"current status is {trace_capture.get('status')!r}"
                ),
                remediation=(
                    "Run python3 benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py "
                    "and address the failing trace packets first."
                ),
                ref=args.trace_capture,
            )
        )

    build_identity = load_json(repo_root / args.build_identity, args.build_identity)
    exact_build_identity_token = ensure_str(
        trace_capture.get("exact_build_identity_token"),
        "trace_capture.exact_build_identity_token",
    )
    build_commit = ensure_str(
        build_identity.get("commit_short"), "build_identity.commit_short"
    )
    if build_commit not in exact_build_identity_token:
        findings.append(
            Finding(
                severity="error",
                check_id="nightly_hot_path.build_identity.mismatch",
                message=(
                    "trace-suite capture exact_build_identity_token does not match "
                    f"the running build commit {build_commit!r}"
                ),
                remediation=(
                    "Refresh the trace-suite capture against the current build "
                    "identity before re-running the gate."
                ),
            )
        )

    rendered_rows, gate_findings, drill_record = evaluate_gate(
        gate=gate,
        trace_capture=trace_capture,
        exact_build_identity_token=exact_build_identity_token,
        force_drill=args.force_drill,
    )
    findings.extend(gate_findings)

    overall_status = derive_overall_status(rendered_rows, findings)

    dashboard_rel = write_dashboard(
        repo_root=repo_root,
        dashboard_rel=args.dashboard,
        gate=gate,
        rows=rendered_rows,
        overall_status=overall_status,
        trace_capture=trace_capture,
        exact_build_identity_token=exact_build_identity_token,
    )

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "nightly_hot_path_validation_capture",
        "captured_at": now_iso_z(),
        "command": (
            "python3 ci/perf/run_nightly_hot_path.py --repo-root ."
            + (" --force-drill" if args.force_drill else "")
        ),
        "owner_dri": gate["owner_dri"],
        "gate_id": gate["gate_id"],
        "gate_revision": gate["gate_revision"],
        "gate_ref": args.gate,
        "trace_capture_ref": args.trace_capture,
        "dashboard_ref": dashboard_rel,
        "exact_build_identity_ref": args.build_identity,
        "exact_build_identity_token": exact_build_identity_token,
        "validation_lane_ref": validation_lane_ref,
        "status": overall_status,
        "rendered_dashboard_rows": rendered_rows,
        "failure_drill_replay": drill_record,
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(
        json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )

    errors = [f for f in findings if f.severity == "error"]
    print(
        f"[nightly-hot-path] {overall_status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — "
        f"capture: {args.report} dashboard: {dashboard_rel}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[nightly-hot-path] {prefix} {finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(f"[nightly-hot-path]   remediation: {finding.remediation}")

    if args.force_drill:
        if not drill_record.get("replay_passed"):
            print(
                "[nightly-hot-path] failure drill did not reproduce the expected "
                "check_id; the gate is not protecting against the named regression."
            )
            return 1
        expected_check_id = (
            drill_record.get("expected_report", {}).get("expected_check_id")
        )
        print(
            "[nightly-hot-path] failure drill reproduced expected check_id "
            f"{expected_check_id!r}; the drill validates the gate's "
            "regression-floor behavior."
        )
        # In force-drill mode, the gate is *expected* to FAIL — exit 0 so the
        # drill itself can be wired into CI as a positive verification.
        return 0

    return 0 if overall_status == "PASS" else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[nightly-hot-path] interrupted", file=sys.stderr)
        sys.exit(130)
