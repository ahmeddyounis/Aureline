#!/usr/bin/env python3
"""Shared helpers for power / thermal capture auditing."""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

HIDDEN_VISIBILITY_STATES = {
    "occluded_window",
    "hidden_tab",
    "collapsed_split",
    "detached_offscreen",
}


class CaptureError(Exception):
    """Raised when a capture cannot be loaded or is malformed."""


@dataclass
class AuditResult:
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)

    @property
    def ok(self) -> bool:
        return not self.errors


def load_capture(path_str: str) -> dict[str, Any]:
    path = Path(path_str)
    try:
        raw = path.read_text(encoding="utf-8")
    except OSError as exc:
        raise CaptureError(f"{path}: unable to read capture: {exc}") from exc
    try:
        capture = json.loads(raw)
    except json.JSONDecodeError as exc:
        raise CaptureError(f"{path}: invalid JSON: {exc}") from exc
    validate_capture_shape(capture, path)
    return capture


def validate_capture_shape(capture: dict[str, Any], path: Path) -> None:
    required_top = [
        "schema",
        "schema_version",
        "record_kind",
        "capture_id",
        "reference_profile_id",
        "reference_profile_revision",
        "reference_posture_id",
        "run_context_class",
        "capture_class",
        "scenario_id",
        "duration_seconds",
        "host",
        "environment",
        "policy_expectations",
        "efficiency_state_timeline",
        "samples",
        "events",
    ]
    missing = [field for field in required_top if field not in capture]
    if missing:
        raise CaptureError(f"{path}: missing top-level fields: {', '.join(missing)}")
    if capture.get("schema") != "aureline.power_thermal_capture.v1":
        raise CaptureError(f"{path}: unsupported schema token {capture.get('schema')!r}")
    if capture.get("schema_version") != 1:
        raise CaptureError(
            f"{path}: unsupported schema_version {capture.get('schema_version')!r}"
        )
    if capture.get("record_kind") != "power_thermal_capture_record":
        raise CaptureError(
            f"{path}: unsupported record_kind {capture.get('record_kind')!r}"
        )
    if not isinstance(capture.get("samples"), list) or not capture["samples"]:
        raise CaptureError(f"{path}: samples[] must be a non-empty array")
    if not isinstance(capture.get("events"), list) or not capture["events"]:
        raise CaptureError(f"{path}: events[] must be a non-empty array")
    if (
        not isinstance(capture.get("efficiency_state_timeline"), list)
        or not capture["efficiency_state_timeline"]
    ):
        raise CaptureError(
            f"{path}: efficiency_state_timeline[] must be a non-empty array"
        )


def duration_hours(capture: dict[str, Any]) -> float:
    return capture["duration_seconds"] / 3600.0


def energy_delta_wh(capture: dict[str, Any]) -> float:
    env = capture["environment"]
    return env["initial_battery_energy_wh"] - env["final_battery_energy_wh"]


def battery_drain_percent(capture: dict[str, Any]) -> float:
    env = capture["environment"]
    return env["initial_battery_percent"] - env["final_battery_percent"]


def average_power_from_energy(capture: dict[str, Any]) -> float:
    hours = duration_hours(capture)
    if hours <= 0:
        return 0.0
    return energy_delta_wh(capture) / hours


def drain_per_hour_percent(capture: dict[str, Any]) -> float:
    hours = duration_hours(capture)
    if hours <= 0:
        return 0.0
    return battery_drain_percent(capture) / hours


def peak_hot_path_p95_ms(capture: dict[str, Any]) -> float:
    return max(sample["hot_path_p95_ms"] for sample in capture["samples"])


def max_hidden_pane_work(capture: dict[str, Any]) -> int:
    return max(sample["hidden_pane_work"] for sample in capture["samples"])


def max_offscreen_suppression_eligible(capture: dict[str, Any]) -> int:
    return max(sample["offscreen_suppression_eligible"] for sample in capture["samples"])


def max_optional_assistance_running(capture: dict[str, Any]) -> int:
    return max(
        sample["background_running_by_class"]["optional_assistance"]
        for sample in capture["samples"]
    )


def timeline_segment_for_second(
    capture: dict[str, Any], second: int
) -> dict[str, Any] | None:
    timeline = capture["efficiency_state_timeline"]
    for index, segment in enumerate(timeline):
        is_last = index == len(timeline) - 1
        start = segment["start_second"]
        end = segment["end_second"]
        if start <= second < end or (is_last and second == end):
            return segment
    return None


def expected_workloads_for_state(capture: dict[str, Any], state: str) -> list[str]:
    expectations = capture["policy_expectations"]["expected_workloads_by_state"]
    return list(expectations.get(state, []))


def transition_settle_window_seconds(capture: dict[str, Any]) -> int:
    return int(capture["policy_expectations"]["transition_settle_window_seconds"])


def comparable_context_fields(capture: dict[str, Any]) -> dict[str, Any]:
    env = capture["environment"]
    host = capture["host"]
    return {
        "reference_profile_id": capture["reference_profile_id"],
        "reference_profile_revision": capture["reference_profile_revision"],
        "reference_posture_id": capture["reference_posture_id"],
        "capture_class": capture["capture_class"],
        "scenario_id": capture["scenario_id"],
        "host_platform_class": host["host_platform_class"],
        "architecture_class": host["architecture_class"],
        "os_image_id": host["os_image_id"],
        "power_source": env["power_source"],
        "battery_mode": env["battery_mode"],
        "user_power_mode": env["user_power_mode"],
        "display_brightness_nits": env["display_brightness_nits"],
        "display_refresh_hz": env["display_refresh_hz"],
        "network_profile": env["network_profile"],
        "duration_seconds": capture["duration_seconds"],
    }


def summarize_capture(capture: dict[str, Any]) -> str:
    env = capture["environment"]
    host = capture["host"]
    lines = [
        f"capture_id: {capture['capture_id']}",
        f"reference_profile: {capture['reference_profile_id']}@{capture['reference_profile_revision']}",
        f"reference_posture: {capture['reference_posture_id']}",
        f"capture_class: {capture['capture_class']}",
        f"scenario_id: {capture['scenario_id']}",
        f"run_context: {capture['run_context_class']}",
        f"host: {host['host_platform_class']} {host['architecture_class']} ({host['os_image_id']})",
        (
            "environment: "
            f"{env['power_source']}, battery_mode={env['battery_mode']}, "
            f"user_power_mode={env['user_power_mode']}, "
            f"brightness={env['display_brightness_nits']} nits, "
            f"refresh={env['display_refresh_hz']} Hz, ambient={env['ambient_temperature_c']} C"
        ),
        f"duration_seconds: {capture['duration_seconds']}",
        f"battery_drain_percent: {battery_drain_percent(capture):.2f}",
        f"battery_drain_percent_per_hour: {drain_per_hour_percent(capture):.2f}",
        f"energy_delta_wh: {energy_delta_wh(capture):.2f}",
        f"average_power_w: {average_power_from_energy(capture):.2f}",
        f"peak_hot_path_p95_ms: {peak_hot_path_p95_ms(capture):.2f}",
        f"max_hidden_pane_work: {max_hidden_pane_work(capture)}",
        (
            "max_offscreen_suppression_eligible: "
            f"{max_offscreen_suppression_eligible(capture)}"
        ),
        f"max_optional_assistance_running: {max_optional_assistance_running(capture)}",
        (
            "efficiency_states: "
            + " -> ".join(segment["state"] for segment in capture["efficiency_state_timeline"])
        ),
    ]
    return "\n".join(lines)


def audit_capture(capture: dict[str, Any]) -> AuditResult:
    result = AuditResult()
    env = capture["environment"]
    duration = capture["duration_seconds"]
    timeline = capture["efficiency_state_timeline"]
    settle_window = transition_settle_window_seconds(capture)

    required_env_fields = [
        "power_source",
        "battery_mode",
        "user_power_mode",
        "display_brightness_nits",
        "display_refresh_hz",
        "network_profile",
        "ambient_temperature_c",
        "ambient_tolerance_c",
        "initial_battery_percent",
        "final_battery_percent",
        "initial_battery_energy_wh",
        "final_battery_energy_wh",
    ]
    for field in required_env_fields:
        value = env.get(field)
        if value is None or value == "":
            result.errors.append(f"environment.{field} is missing")

    sorted_timeline = sorted(timeline, key=lambda item: item["start_second"])
    if sorted_timeline[0]["start_second"] != 0:
        result.errors.append("efficiency_state_timeline does not start at second 0")
    previous_end = None
    for segment in sorted_timeline:
        start = segment["start_second"]
        end = segment["end_second"]
        if end < start:
            result.errors.append(
                f"timeline segment {segment['state']} has end before start ({start}>{end})"
            )
        if previous_end is not None and start != previous_end:
            result.errors.append(
                f"efficiency_state_timeline gap or overlap around second {start}"
            )
        if not segment.get("source"):
            result.errors.append(
                f"timeline segment {segment['state']} at {start}s is missing source"
            )
        if not segment.get("reason"):
            result.errors.append(
                f"timeline segment {segment['state']} at {start}s is missing reason"
            )
        previous_end = end
    if sorted_timeline[-1]["end_second"] != duration:
        result.errors.append(
            f"efficiency_state_timeline does not end at duration_seconds ({duration})"
        )

    for sample in capture["samples"]:
        second = sample["second"]
        segment = timeline_segment_for_second(capture, second)
        if segment is None:
            result.errors.append(f"sample at {second}s falls outside the timeline")
            continue
        if segment["state"] != sample["efficiency_state"]:
            result.errors.append(
                f"sample at {second}s says {sample['efficiency_state']} but timeline says {segment['state']}"
            )
        if sample["hidden_pane_work"] > 0:
            result.errors.append(
                f"sample at {second}s reports hidden_pane_work={sample['hidden_pane_work']}"
            )
        if sample["offscreen_suppression_eligible"] > 0:
            result.warnings.append(
                "sample at "
                f"{second}s reports offscreen_suppression_eligible="
                f"{sample['offscreen_suppression_eligible']}"
            )
        if (
            sample["efficiency_state"] in {"ThermalConstrained", "ProtectCore"}
            and sample["background_running_by_class"]["optional_assistance"] > 0
        ):
            result.errors.append(
                "sample at "
                f"{second}s keeps optional_assistance running during {sample['efficiency_state']}"
            )
        for surface in sample["surfaces"]:
            if surface["visibility_state"] in HIDDEN_VISIBILITY_STATES:
                if surface["committed_paint_count"] > 0:
                    result.errors.append(
                        "surface "
                        f"{surface['surface_id']} painted while {surface['visibility_state']} "
                        f"at {second}s"
                    )
                if surface["hidden_pane_work"] > 0:
                    result.errors.append(
                        "surface "
                        f"{surface['surface_id']} reported hidden_pane_work="
                        f"{surface['hidden_pane_work']} at {second}s"
                    )
                if surface["offscreen_suppression_eligible"] > 0:
                    result.warnings.append(
                        "surface "
                        f"{surface['surface_id']} reported offscreen_suppression_eligible="
                        f"{surface['offscreen_suppression_eligible']} at {second}s"
                    )

    transitions = [
        event
        for event in capture["events"]
        if event["event_id"] == "efficiency_state_transition"
    ]
    for event in transitions:
        second = event["second"]
        if not event.get("from_state") or not event.get("to_state"):
            result.errors.append(f"transition event at {second}s is missing from_state or to_state")
        if not event.get("source"):
            result.errors.append(f"transition event at {second}s is missing source")
        if not event.get("reason"):
            result.errors.append(f"transition event at {second}s is missing reason")
        if event.get("context_present") is not True:
            result.errors.append(f"transition event at {second}s is missing efficiency-state context")

    budget_events = [
        event
        for event in capture["events"]
        if event["event_id"] == "workload_budget_decision"
    ]
    for segment in sorted_timeline:
        state = segment["state"]
        expected_workloads = expected_workloads_for_state(capture, state)
        if not expected_workloads:
            continue
        start = segment["start_second"]
        window_end = start + settle_window
        for workload_id in expected_workloads:
            matched = any(
                event["efficiency_state"] == state
                and event.get("workload_id") == workload_id
                and start <= event["second"] <= window_end
                for event in budget_events
            )
            if not matched:
                result.errors.append(
                    f"{state} entry at {start}s missing workload_budget_decision for {workload_id} "
                    f"inside {settle_window}s"
                )

    return result


def compare_captures(
    baseline: dict[str, Any], candidate: dict[str, Any]
) -> tuple[list[str], str]:
    mismatches: list[str] = []
    left = comparable_context_fields(baseline)
    right = comparable_context_fields(candidate)
    for field, left_value in left.items():
        right_value = right[field]
        if left_value != right_value:
            mismatches.append(
                f"{field}: baseline={left_value!r} candidate={right_value!r}"
            )

    left_env = baseline["environment"]
    right_env = candidate["environment"]
    ambient_delta = abs(
        left_env["ambient_temperature_c"] - right_env["ambient_temperature_c"]
    )
    tolerance = min(left_env["ambient_tolerance_c"], right_env["ambient_tolerance_c"])
    if ambient_delta > tolerance:
        mismatches.append(
            "ambient_temperature_c differs by "
            f"{ambient_delta:.2f} C which exceeds tolerance {tolerance:.2f} C"
        )

    summary_lines = [
        f"baseline_capture_id: {baseline['capture_id']}",
        f"candidate_capture_id: {candidate['capture_id']}",
        f"reference_profile: {baseline['reference_profile_id']}@{baseline['reference_profile_revision']}",
        f"reference_posture: {baseline['reference_posture_id']}",
        f"baseline_average_power_w: {average_power_from_energy(baseline):.2f}",
        f"candidate_average_power_w: {average_power_from_energy(candidate):.2f}",
        (
            "delta_average_power_w: "
            f"{average_power_from_energy(candidate) - average_power_from_energy(baseline):+.2f}"
        ),
        f"baseline_drain_pct_per_hour: {drain_per_hour_percent(baseline):.2f}",
        f"candidate_drain_pct_per_hour: {drain_per_hour_percent(candidate):.2f}",
        (
            "delta_drain_pct_per_hour: "
            f"{drain_per_hour_percent(candidate) - drain_per_hour_percent(baseline):+.2f}"
        ),
        f"baseline_peak_hot_path_p95_ms: {peak_hot_path_p95_ms(baseline):.2f}",
        f"candidate_peak_hot_path_p95_ms: {peak_hot_path_p95_ms(candidate):.2f}",
        (
            "delta_peak_hot_path_p95_ms: "
            f"{peak_hot_path_p95_ms(candidate) - peak_hot_path_p95_ms(baseline):+.2f}"
        ),
    ]
    return mismatches, "\n".join(summary_lines)
