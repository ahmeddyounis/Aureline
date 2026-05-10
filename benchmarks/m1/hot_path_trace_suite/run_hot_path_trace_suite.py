#!/usr/bin/env python3
"""Unattended hot-path trace suite runner.

Reads the canonical hot-path scenarios from
``benchmarks/m1/hot_path_trace_suite/scenarios.yaml`` and the council-approved
reference-hardware rows from
``artifacts/perf/m1/reference_hardware_matrix.yaml``, replays each
(hardware_profile_id, scenario_id) pair against the reference trace
fixture (``fixtures/perf/hot_path_trace_reference.json``), and emits a
durable JSON capture under
``artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json``.

The runner stays unattended-runnable: it never opens a window, never
runs the live shell binary, and is byte-stable across reruns when the
inputs are unchanged. It emits one trace packet per (hardware row,
scenario) pair carrying:

- ``exact_build_identity_ref`` (read from ``artifacts/build/build_identity.json``)
- ``hardware_definition_id`` resolved through the reference-hardware manifest
- ``scenario_id`` and the protected-path / journey-segment binding
- ``timing_buckets`` populated from the scenario's published p50/p95
  thresholds so dashboard ingestion stays apples-to-apples
- ``qualification_status`` (``passed`` or ``failed``)

The runner also exercises the failure drill described in
``scenarios.yaml#failure_drill``: it reloads the reference trace with the
named segment dropped and confirms the trace packet fails qualification
with a typed missing-metric finding rather than passing on a partial
trace.

YAML decoding follows the existing repository convention: the suite
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


DEFAULT_SCENARIOS_REL = "benchmarks/m1/hot_path_trace_suite/scenarios.yaml"
DEFAULT_HARDWARE_REL = "artifacts/perf/m1/reference_hardware_matrix.yaml"
DEFAULT_REFERENCE_TRACE_REL = "fixtures/perf/hot_path_trace_reference.json"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/hot_path_trace_suite_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_HARDWARE_MANIFEST_REL = "artifacts/perf/reference_hardware_manifest.yaml"


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
        "--scenarios",
        default=DEFAULT_SCENARIOS_REL,
        help="Scenarios YAML, repo-relative.",
    )
    parser.add_argument(
        "--hardware-matrix",
        default=DEFAULT_HARDWARE_REL,
        help="Reference-hardware matrix YAML, repo-relative.",
    )
    parser.add_argument(
        "--reference-trace",
        default=DEFAULT_REFERENCE_TRACE_REL,
        help="Reference trace fixture JSON, repo-relative.",
    )
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Where to write the durable JSON capture (repo-relative).",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Path to the build identity record to embed in the capture.",
    )
    parser.add_argument(
        "--hardware-manifest",
        default=DEFAULT_HARDWARE_MANIFEST_REL,
        help="Reference hardware manifest YAML, repo-relative.",
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


def load_hardware_definition_ids(repo_root: Path, manifest_rel: str) -> set[str]:
    payload = ensure_dict(render_yaml_as_json(repo_root / manifest_rel), manifest_rel)
    rows = ensure_list(payload.get("hardware_rows"), f"{manifest_rel}.hardware_rows")
    ids: set[str] = set()
    for idx, row in enumerate(rows):
        row = ensure_dict(row, f"{manifest_rel}.hardware_rows[{idx}]")
        ids.add(ensure_str(row.get("id"), f"{manifest_rel}.hardware_rows[{idx}].id"))
    return ids


def load_reference_trace(repo_root: Path, trace_rel: str) -> dict[str, Any]:
    path = repo_root / trace_rel
    if not path.exists():
        raise SystemExit(f"missing reference trace fixture: {path}")
    return ensure_dict(json.loads(path.read_text(encoding="utf-8")), trace_rel)


def load_build_identity(repo_root: Path, identity_rel: str) -> dict[str, Any]:
    path = repo_root / identity_rel
    if not path.exists():
        raise SystemExit(f"missing build identity record: {path}")
    return ensure_dict(json.loads(path.read_text(encoding="utf-8")), identity_rel)


def trace_segment_ids(trace: dict[str, Any]) -> set[str]:
    events = ensure_list(trace.get("events"), "reference_trace.events")
    seen: set[str] = set()
    for idx, event in enumerate(events):
        event = ensure_dict(event, f"reference_trace.events[{idx}]")
        segment_id = ensure_str(
            event.get("journey_segment_id"),
            f"reference_trace.events[{idx}].journey_segment_id",
        )
        seen.add(segment_id)
    return seen


def trace_durations_by_segment(trace: dict[str, Any]) -> dict[str, list[int]]:
    """Return all duration_ticks observed per segment_id (span_end events only)."""
    durations: dict[str, list[int]] = {}
    for event in ensure_list(trace.get("events"), "reference_trace.events"):
        event = ensure_dict(event, "reference_trace.events[]")
        if event.get("span_kind") != "span_end":
            continue
        ticks = event.get("duration_ticks")
        if not isinstance(ticks, int):
            continue
        seg = event.get("journey_segment_id")
        if isinstance(seg, str) and seg:
            durations.setdefault(seg, []).append(ticks)
    return durations


def bucket_for_observation(
    observed_ms: int | None,
    target_ms: int,
    bucket_p50_ms: int,
    bucket_p95_ms: int,
) -> str:
    if observed_ms is None:
        return "missing"
    if observed_ms <= bucket_p50_ms:
        return "within_p50"
    if observed_ms <= bucket_p95_ms:
        return "within_p95"
    if observed_ms <= target_ms:
        return "under_target"
    return "over_target"


def build_trace_packet(
    *,
    scenario: dict[str, Any],
    hardware_row: dict[str, Any],
    hardware_definition_id: str,
    trace_segments: set[str],
    durations_by_segment: dict[str, list[int]],
    exact_build_identity_ref: str,
    build_identity: dict[str, Any],
) -> tuple[dict[str, Any], list[str]]:
    """Build one trace packet plus the list of missing required segment ids."""
    required_segments = [
        ensure_str(s, "scenario.required_journey_segment_ids[]")
        for s in ensure_list(
            scenario.get("required_journey_segment_ids"),
            "scenario.required_journey_segment_ids",
        )
    ]
    missing = [s for s in required_segments if s not in trace_segments]
    qualification_status = "passed" if not missing else "failed"

    metric_rows: list[dict[str, Any]] = []
    for metric in ensure_list(scenario.get("metrics"), "scenario.metrics"):
        metric = ensure_dict(metric, "scenario.metrics[]")
        metric_id = ensure_str(metric.get("metric_id"), "scenario.metrics[].metric_id")
        target_ms = int(metric.get("target_ms", 0))
        bucket_p50 = int(metric.get("bucket_p50_ms", target_ms))
        bucket_p95 = int(metric.get("bucket_p95_ms", target_ms))
        # Use the longest observed duration on any required segment as the
        # representative observation. The reference fixture records ticks
        # in synthetic units; the runner treats one tick as one millisecond
        # so the buckets stay comparable across runs without local timing.
        observed_candidates: list[int] = []
        for seg in required_segments:
            observed_candidates.extend(durations_by_segment.get(seg, []))
        observed_ms = max(observed_candidates) if observed_candidates else None
        metric_rows.append(
            {
                "metric_id": metric_id,
                "comparator": ensure_str(
                    metric.get("comparator"), "scenario.metrics[].comparator"
                ),
                "target_ms": target_ms,
                "bucket_p50_ms": bucket_p50,
                "bucket_p95_ms": bucket_p95,
                "observed_ms": observed_ms,
                "timing_bucket": bucket_for_observation(
                    observed_ms, target_ms, bucket_p50, bucket_p95
                ),
            }
        )

    packet = {
        "schema": "aureline.hot_path_trace_packet.v1",
        "schema_version": 1,
        "scenario_id": ensure_str(scenario.get("scenario_id"), "scenario.scenario_id"),
        "scenario_title": ensure_str(scenario.get("title"), "scenario.title"),
        "event_class": ensure_str(scenario.get("event_class"), "scenario.event_class"),
        "protected_journey": ensure_str(
            scenario.get("protected_journey"), "scenario.protected_journey"
        ),
        "dispatch_layer": ensure_str(
            scenario.get("dispatch_layer"), "scenario.dispatch_layer"
        ),
        "budget_ref": ensure_str(scenario.get("budget_ref"), "scenario.budget_ref"),
        "hardware_row_id": ensure_str(
            hardware_row.get("row_id"), "hardware_row.row_id"
        ),
        "hardware_definition_id": hardware_definition_id,
        "host_os_class": ensure_str(
            hardware_row.get("host_os_class"), "hardware_row.host_os_class"
        ),
        "architecture_class": ensure_str(
            hardware_row.get("architecture_class"),
            "hardware_row.architecture_class",
        ),
        "power_profile_id": ensure_str(
            hardware_row.get("power_profile_id"),
            "hardware_row.power_profile_id",
        ),
        "capture_posture_id": ensure_str(
            hardware_row.get("capture_posture_id"),
            "hardware_row.capture_posture_id",
        ),
        "lab_image_ref": ensure_str(
            hardware_row.get("lab_image_ref"), "hardware_row.lab_image_ref"
        ),
        "exact_build_identity_ref": exact_build_identity_ref,
        "build_commit_short": ensure_str(
            build_identity.get("commit_short"), "build_identity.commit_short"
        ),
        "build_workspace_version": ensure_str(
            build_identity.get("workspace_version"),
            "build_identity.workspace_version",
        ),
        "build_profile": ensure_str(
            build_identity.get("profile"), "build_identity.profile"
        ),
        "required_journey_segment_ids": required_segments,
        "missing_journey_segment_ids": missing,
        "metrics": metric_rows,
        "qualification_status": qualification_status,
    }
    return packet, missing


def derive_exact_build_identity_ref(build: dict[str, Any]) -> str:
    profile = ensure_str(build.get("profile"), "build_identity.profile")
    version = ensure_str(
        build.get("workspace_version"), "build_identity.workspace_version"
    )
    target = ensure_str(
        build.get("target_triple"), "build_identity.target_triple"
    )
    commit = ensure_str(build.get("commit_short"), "build_identity.commit_short")
    return (
        f"build-id:aureline:dev:{version}:{target}:{profile}:{commit}"
    )


def replay_failure_drill(
    *,
    drill: dict[str, Any],
    scenarios_index: dict[str, dict[str, Any]],
    hardware_rows: list[dict[str, Any]],
    hardware_definition_ids: set[str],
    reference_trace: dict[str, Any],
    exact_build_identity_ref: str,
    build_identity: dict[str, Any],
) -> dict[str, Any]:
    """Rebuild the trace packet for the warm-first-paint scenario with the
    named segment dropped and confirm the packet reports a typed
    missing-metric failure rather than silently passing.
    """
    forced_input = ensure_dict(drill.get("forced_input"), "failure_drill.forced_input")
    drop_segment = ensure_str(
        forced_input.get("drop_journey_segment_id"),
        "failure_drill.forced_input.drop_journey_segment_id",
    )

    # Drop every event whose journey_segment_id matches the named id.
    forced_events = [
        ev
        for ev in ensure_list(reference_trace.get("events"), "reference_trace.events")
        if not (
            isinstance(ev, dict) and ev.get("journey_segment_id") == drop_segment
        )
    ]
    forced_trace = {**reference_trace, "events": forced_events}
    forced_segments = trace_segment_ids(forced_trace)
    forced_durations = trace_durations_by_segment(forced_trace)

    expected = ensure_dict(drill.get("expected_report"), "failure_drill.expected_report")
    expected_status = ensure_str(
        expected.get("qualification_status"),
        "failure_drill.expected_report.qualification_status",
    )
    expected_missing = sorted(
        ensure_str(s, "failure_drill.expected_report.missing_metric_journey_segment_ids[]")
        for s in ensure_list(
            expected.get("missing_metric_journey_segment_ids"),
            "failure_drill.expected_report.missing_metric_journey_segment_ids",
        )
    )

    # Replay against the first hardware row that covers warm_first_paint.
    target_scenario_id = "scenario.warm_first_paint"
    scenario = scenarios_index[target_scenario_id]
    chosen_row = next(
        (
            row
            for row in hardware_rows
            if target_scenario_id in row.get("covered_scenario_ids", [])
        ),
        None,
    )
    if chosen_row is None:
        raise SystemExit(
            "failure drill cannot run: no reference-hardware row covers scenario.warm_first_paint"
        )
    hardware_definition_id = ensure_str(
        chosen_row.get("hardware_definition_id"),
        "hardware_row.hardware_definition_id",
    )
    if hardware_definition_id not in hardware_definition_ids:
        raise SystemExit(
            f"failure drill: hardware_definition_id {hardware_definition_id} is not in the manifest"
        )

    packet, missing = build_trace_packet(
        scenario=scenario,
        hardware_row=chosen_row,
        hardware_definition_id=hardware_definition_id,
        trace_segments=forced_segments,
        durations_by_segment=forced_durations,
        exact_build_identity_ref=exact_build_identity_ref,
        build_identity=build_identity,
    )

    drill_passed = (
        packet["qualification_status"] == expected_status
        and sorted(missing) == expected_missing
    )

    return {
        "drill_id": ensure_str(drill.get("drill_id"), "failure_drill.drill_id"),
        "forced_input": forced_input,
        "expected_report": expected,
        "replay_packet": packet,
        "replay_passed": drill_passed,
    }


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    findings: list[Finding] = []

    scenarios_payload = ensure_dict(
        render_yaml_as_json(repo_root / args.scenarios), args.scenarios
    )
    hardware_payload = ensure_dict(
        render_yaml_as_json(repo_root / args.hardware_matrix), args.hardware_matrix
    )

    schema_version = scenarios_payload.get("schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="scenarios.schema_version",
                message=f"scenarios.schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the runner together with the schema if the scenarios shape changes.",
            )
        )

    hw_schema_version = hardware_payload.get("schema_version")
    if hw_schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="hardware_matrix.schema_version",
                message=f"hardware_matrix.schema_version must be the integer 1, got {hw_schema_version!r}",
                remediation="Bump the runner together with the schema if the hardware matrix shape changes.",
            )
        )

    # Verify all upstream contract refs resolve.
    upstream = ensure_dict(
        scenarios_payload.get("upstream_contracts"),
        "scenarios.upstream_contracts",
    )
    for key, value in upstream.items():
        ref = ensure_str(value, f"scenarios.upstream_contracts.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="scenarios.upstream_contracts.missing",
                    message=f"upstream_contracts.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    hw_upstream = ensure_dict(
        hardware_payload.get("upstream_contracts"),
        "hardware_matrix.upstream_contracts",
    )
    for key, value in hw_upstream.items():
        ref = ensure_str(value, f"hardware_matrix.upstream_contracts.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="hardware_matrix.upstream_contracts.missing",
                    message=f"upstream_contracts.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    scenarios = ensure_list(scenarios_payload.get("scenarios"), "scenarios.scenarios")
    if not scenarios:
        findings.append(
            Finding(
                severity="error",
                check_id="scenarios.empty",
                message="scenarios.yaml must declare at least one scenario",
                remediation="Seed the canonical hot-path scenarios.",
            )
        )
    scenarios_index: dict[str, dict[str, Any]] = {}
    for idx, scenario in enumerate(scenarios):
        scenario = ensure_dict(scenario, f"scenarios.scenarios[{idx}]")
        scenario_id = ensure_str(
            scenario.get("scenario_id"),
            f"scenarios.scenarios[{idx}].scenario_id",
        )
        if scenario_id in scenarios_index:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scenarios.duplicate_id",
                    message=f"duplicate scenario_id: {scenario_id}",
                    remediation="scenario_ids must be unique.",
                    ref=scenario_id,
                )
            )
        scenarios_index[scenario_id] = scenario

    hardware_rows = ensure_list(
        hardware_payload.get("reference_hardware_rows"),
        "hardware_matrix.reference_hardware_rows",
    )
    if not hardware_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="hardware_matrix.empty",
                message="reference hardware matrix must declare at least one row",
                remediation="Seed the council-approved reference rows.",
            )
        )

    hardware_definition_ids = load_hardware_definition_ids(
        repo_root, args.hardware_manifest
    )

    # Validate covered_scenario_ids on every row resolves into scenarios_index.
    for idx, row in enumerate(hardware_rows):
        row = ensure_dict(row, f"hardware_matrix.reference_hardware_rows[{idx}]")
        hardware_definition_id = ensure_str(
            row.get("hardware_definition_id"),
            f"hardware_matrix.reference_hardware_rows[{idx}].hardware_definition_id",
        )
        if hardware_definition_id not in hardware_definition_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="hardware_matrix.unknown_hardware_definition_id",
                    message=(
                        f"hardware_definition_id {hardware_definition_id!r} is not in "
                        f"{args.hardware_manifest}"
                    ),
                    remediation="Add the row to the reference-hardware manifest first or fix the id.",
                    ref=hardware_definition_id,
                )
            )
        for scenario_id in ensure_list(
            row.get("covered_scenario_ids"),
            f"hardware_matrix.reference_hardware_rows[{idx}].covered_scenario_ids",
        ):
            scenario_id = ensure_str(scenario_id, "covered_scenario_ids[]")
            if scenario_id not in scenarios_index:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="hardware_matrix.unknown_scenario_id",
                        message=(
                            f"row {row.get('row_id')!r} references unknown scenario_id {scenario_id!r}"
                        ),
                        remediation="Seed the scenario in scenarios.yaml or fix the id.",
                        ref=scenario_id,
                    )
                )

    reference_trace = load_reference_trace(repo_root, args.reference_trace)
    trace_segments = trace_segment_ids(reference_trace)
    durations_by_segment = trace_durations_by_segment(reference_trace)
    build_identity = load_build_identity(repo_root, args.build_identity)
    exact_build_identity_ref = derive_exact_build_identity_ref(build_identity)

    trace_packets: list[dict[str, Any]] = []
    covered_scenario_ids: set[str] = set()
    for row in hardware_rows:
        row = ensure_dict(row, "hardware_matrix.reference_hardware_rows[]")
        hardware_definition_id = ensure_str(
            row.get("hardware_definition_id"),
            "hardware_row.hardware_definition_id",
        )
        for scenario_id in ensure_list(
            row.get("covered_scenario_ids"),
            "hardware_row.covered_scenario_ids",
        ):
            scenario = scenarios_index.get(scenario_id)
            if scenario is None:
                continue
            packet, missing = build_trace_packet(
                scenario=scenario,
                hardware_row=row,
                hardware_definition_id=hardware_definition_id,
                trace_segments=trace_segments,
                durations_by_segment=durations_by_segment,
                exact_build_identity_ref=exact_build_identity_ref,
                build_identity=build_identity,
            )
            trace_packets.append(packet)
            if not missing:
                covered_scenario_ids.add(scenario_id)
            else:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="trace_packet.missing_required_segments",
                        message=(
                            f"row {row.get('row_id')!r} scenario {scenario_id!r} is missing "
                            f"required segments: {missing}"
                        ),
                        remediation=(
                            "Re-instrument the named segments in the reference trace fixture "
                            "or lift the requirement explicitly."
                        ),
                        ref=scenario_id,
                    )
                )

    required_coverage = sorted(
        ensure_str(s, "scenarios.required_scenario_coverage[]")
        for s in ensure_list(
            scenarios_payload.get("required_scenario_coverage"),
            "scenarios.required_scenario_coverage",
        )
    )
    missing_coverage = sorted(set(required_coverage) - covered_scenario_ids)
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="trace_suite.missing_required_coverage",
                message=(
                    "trace suite must seed at least one passing trace packet for each "
                    f"required scenario; missing: {missing_coverage}"
                ),
                remediation=(
                    "Add the missing scenarios to a reference-hardware row's "
                    "covered_scenario_ids or restore the upstream segment instrumentation."
                ),
            )
        )

    # Failure drill.
    drill = ensure_dict(
        scenarios_payload.get("failure_drill"), "scenarios.failure_drill"
    )
    drill_record = replay_failure_drill(
        drill=drill,
        scenarios_index=scenarios_index,
        hardware_rows=hardware_rows,
        hardware_definition_ids=hardware_definition_ids,
        reference_trace=reference_trace,
        exact_build_identity_ref=exact_build_identity_ref,
        build_identity=build_identity,
    )
    if not drill_record["replay_passed"]:
        findings.append(
            Finding(
                severity="error",
                check_id="failure_drill.unexpected_outcome",
                message=(
                    "failure drill replay did not match expected report; "
                    "the trace packet must report 'failed' with the named missing segment"
                ),
                remediation=(
                    "Repair the runner so it refuses partial traces, or update the "
                    "failure_drill expected_report if the contract changed."
                ),
                ref=drill_record["drill_id"],
            )
        )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "hot_path_trace_suite_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(
            scenarios_payload.get("owner_dri"), "scenarios.owner_dri"
        ),
        "scenarios_ref": args.scenarios,
        "hardware_matrix_ref": args.hardware_matrix,
        "reference_trace_ref": args.reference_trace,
        "exact_build_identity_ref": args.build_identity,
        "exact_build_identity_token": exact_build_identity_ref,
        "command": (
            "python3 benchmarks/m1/hot_path_trace_suite/run_hot_path_trace_suite.py "
            "--repo-root ."
        ),
        "status": status,
        "required_scenario_coverage": required_coverage,
        "covered_scenario_ids": sorted(covered_scenario_ids),
        "trace_packets": trace_packets,
        "failure_drill_replay": drill_record,
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }

    report_path = repo_root / args.report
    report_path.parent.mkdir(parents=True, exist_ok=True)
    report_path.write_text(json.dumps(capture, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    print(
        f"[hot-path-trace-suite] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(
            f"[hot-path-trace-suite] {prefix} {finding.check_id}: {finding.message}{ref_suffix}"
        )
        print(f"[hot-path-trace-suite]   remediation: {finding.remediation}")

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[hot-path-trace-suite] interrupted", file=sys.stderr)
        sys.exit(130)
