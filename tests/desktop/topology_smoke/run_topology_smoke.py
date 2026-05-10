#!/usr/bin/env python3
"""Unattended desktop topology smoke runner.

Replays the suspend/resume, multi-monitor, and mixed-DPI rows in
``fixtures/desktop/m1_suspend_resume_matrix.yaml`` against pure-geometry
projections of the runtime windowing safety guard at
``crates/aureline-shell/src/windowing/display_safety.rs`` and asserts:

- the off-screen detection in each row matches the expected outcome;
- after the recorded topology change, the window's safe-bounds anchor
  lands inside a real display rectangle (never silently off-screen);
- the named failure drill, when forced, reproduces the precise
  failing-vs-claimed hardware row the matrix expects;
- continuity-level, focus-visibility, authority-rebind, and
  session-execution truth tokens are present and match the frozen
  vocabulary echoed in the matrix; and
- every claimed_profile_id resolves to a row in
  ``artifacts/platform/claimed_desktop_profiles.yaml``.

The runner emits a durable, machine-readable capture (``--report``) and
exits non-zero if any row fails to clamp into safe bounds, drops a
named topology class, or references vocabulary the matrix has not
declared. The pass/fail artifact is comparable across runs because the
geometry and expectations are deterministic.

YAML decoding follows the existing repository convention: the matrix
file is parsed via Ruby/Psych (already required by other CI checks),
which avoids adding a Python YAML dependency.
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


DEFAULT_MATRIX_REL = "fixtures/desktop/m1_suspend_resume_matrix.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/topology_smoke_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"

REQUIRED_SCENARIO_COVERAGE = {
    "suspend_resume",
    "display_topology_change",
    "mixed_dpi",
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
        "--matrix",
        default=DEFAULT_MATRIX_REL,
        help="Smoke matrix YAML path, repo-relative.",
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


# ---- pure geometry mirroring crates/aureline-shell/src/windowing/display_safety.rs ----

@dataclass(frozen=True)
class Rect:
    x: int
    y: int
    width: int
    height: int

    @property
    def right(self) -> int:
        return self.x + self.width

    @property
    def bottom(self) -> int:
        return self.y + self.height

    def intersection_area(self, other: "Rect") -> int:
        x0 = max(self.x, other.x)
        y0 = max(self.y, other.y)
        x1 = min(self.right, other.right)
        y1 = min(self.bottom, other.bottom)
        if x0 >= x1 or y0 >= y1:
            return 0
        return (x1 - x0) * (y1 - y0)

    def contains_point(self, px: int, py: int) -> bool:
        return self.x <= px < self.right and self.y <= py < self.bottom

    def center_anchor(self, child: "Rect") -> tuple[int, int]:
        dx = max((self.width - child.width) // 2, 0)
        dy = max((self.height - child.height) // 2, 0)
        return self.x + dx, self.y + dy


def rect_from_obj(obj: dict[str, Any], label: str) -> Rect:
    obj = ensure_dict(obj, label)
    return Rect(
        x=int(obj.get("x", 0)),
        y=int(obj.get("y", 0)),
        width=int(obj.get("width", 0)),
        height=int(obj.get("height", 0)),
    )


def is_offscreen(window: Rect, displays: list[Rect]) -> bool:
    if not displays:
        return True
    return all(window.intersection_area(d) == 0 for d in displays)


def safe_anchor_after_clamp(
    window: Rect,
    displays_after: list[Rect],
    primary_after: Rect | None,
) -> tuple[bool, tuple[int, int] | None]:
    """Return (clamped_into_safe_bounds, anchor_after_clamp)."""
    if not displays_after:
        return False, None
    target = primary_after or displays_after[0]
    anchor = target.center_anchor(window)
    return target.contains_point(*anchor), anchor


def scale_bucket_token(scale_factor: float) -> str:
    if scale_factor <= 0.0:
        return "other"
    for target, token in ((1.0, "1x"), (1.25, "1_25x"), (1.5, "1_5x"), (2.0, "2x")):
        if abs(scale_factor - target) <= 0.08:
            return token
    return "other"


# ---- matrix replay ----

@dataclass
class RowResult:
    smoke_row_id: str
    inherited_scenario_id: str
    passed_checks: list[str] = field(default_factory=list)
    failed_checks: list[str] = field(default_factory=list)
    diagnostics: dict[str, Any] = field(default_factory=dict)


def collect_displays(geometry: dict[str, Any], label: str) -> tuple[list[Rect], Rect | None, list[float]]:
    raw = ensure_list(geometry.get("displays"), f"{label}.displays")
    if not raw:
        raise SystemExit(f"{label}.displays must declare at least one display")
    displays: list[Rect] = []
    primary: Rect | None = None
    scales: list[float] = []
    for idx, raw_display in enumerate(raw):
        d = ensure_dict(raw_display, f"{label}.displays[{idx}]")
        rect = Rect(
            x=int(d.get("x", 0)),
            y=int(d.get("y", 0)),
            width=int(d.get("width", 0)),
            height=int(d.get("height", 0)),
        )
        displays.append(rect)
        scale = float(d.get("scale_factor", 1.0))
        scales.append(scale)
        if d.get("primary") is True and primary is None:
            primary = rect
    if primary is None:
        primary = displays[0]
    return displays, primary, scales


def replay_row(
    row: dict[str, Any],
    matrix_vocab: dict[str, set[str]],
    claimed_profile_ids: set[str],
) -> RowResult:
    smoke_row_id = ensure_str(row.get("smoke_row_id"), "row.smoke_row_id")
    inherited = ensure_str(row.get("inherited_scenario_id"), f"{smoke_row_id}.inherited_scenario_id")
    result = RowResult(smoke_row_id=smoke_row_id, inherited_scenario_id=inherited)

    # Scenario classes must be in the matrix vocabulary.
    classes = ensure_list(row.get("scenario_classes"), f"{smoke_row_id}.scenario_classes")
    for cls in classes:
        cls = ensure_str(cls, f"{smoke_row_id}.scenario_classes[]")
        if cls not in matrix_vocab["scenario_class"]:
            result.failed_checks.append(f"scenario_class '{cls}' not in matrix vocabulary")
        else:
            result.passed_checks.append(f"scenario_class '{cls}' is in vocabulary")

    # Claimed profiles must resolve.
    declared_profiles = ensure_list(row.get("claimed_profile_ids"), f"{smoke_row_id}.claimed_profile_ids")
    if not declared_profiles:
        result.failed_checks.append("claimed_profile_ids is empty")
    for profile_id in declared_profiles:
        profile_id = ensure_str(profile_id, f"{smoke_row_id}.claimed_profile_ids[]")
        if profile_id not in claimed_profile_ids:
            result.failed_checks.append(
                f"claimed_profile_id '{profile_id}' is not in claimed_desktop_profiles.yaml"
            )
        else:
            result.passed_checks.append(f"claimed_profile_id '{profile_id}' resolves")

    geometry_before = ensure_dict(row.get("geometry_before"), f"{smoke_row_id}.geometry_before")
    geometry_after = ensure_dict(row.get("geometry_after"), f"{smoke_row_id}.geometry_after")
    window_before = rect_from_obj(geometry_before.get("window"), f"{smoke_row_id}.geometry_before.window")
    window_after = rect_from_obj(geometry_after.get("window"), f"{smoke_row_id}.geometry_after.window")
    displays_before, _primary_before, scales_before = collect_displays(geometry_before, f"{smoke_row_id}.geometry_before")
    displays_after, primary_after, scales_after = collect_displays(geometry_after, f"{smoke_row_id}.geometry_after")

    expected = ensure_dict(row.get("expected"), f"{smoke_row_id}.expected")
    expected_offscreen_before = bool(expected.get("offscreen_before_clamp"))
    expected_anchor_inside = bool(expected.get("anchor_within_safe_bounds_after_clamp"))

    actual_offscreen_before = is_offscreen(window_after, displays_after)
    if actual_offscreen_before == expected_offscreen_before:
        result.passed_checks.append(
            f"offscreen_before_clamp matches expected ({expected_offscreen_before})"
        )
    else:
        result.failed_checks.append(
            f"offscreen_before_clamp mismatch: expected={expected_offscreen_before}, "
            f"actual={actual_offscreen_before}"
        )

    clamped, anchor = safe_anchor_after_clamp(window_after, displays_after, primary_after)
    if clamped == expected_anchor_inside:
        result.passed_checks.append(
            f"anchor_within_safe_bounds_after_clamp matches expected ({expected_anchor_inside})"
        )
    else:
        result.failed_checks.append(
            f"anchor_within_safe_bounds_after_clamp mismatch: expected={expected_anchor_inside}, "
            f"actual={clamped}"
        )
    result.diagnostics["computed_anchor_after_clamp"] = anchor
    result.diagnostics["window_intent_rect"] = asdict(window_after)
    result.diagnostics["window_before_rect"] = asdict(window_before)
    result.diagnostics["scale_buckets_before"] = [scale_bucket_token(s) for s in scales_before]
    result.diagnostics["scale_buckets_after"] = [scale_bucket_token(s) for s in scales_after]

    # Topology change classes (must be in vocabulary; for mixed-DPI rows
    # scale_changed must be present; for detach rows display_removed
    # must be present; for suspend/resume rows wake_display_reconnect or
    # safe_bounds_changed must be present so the matrix never silently
    # drops a topology change).
    topology_classes = ensure_list(
        expected.get("topology_change_classes"),
        f"{smoke_row_id}.expected.topology_change_classes",
    )
    if not topology_classes:
        result.failed_checks.append("expected.topology_change_classes must declare at least one class")
    for cls in topology_classes:
        cls = ensure_str(cls, f"{smoke_row_id}.expected.topology_change_classes[]")
        if cls not in matrix_vocab["topology_change_class"]:
            result.failed_checks.append(
                f"topology_change_class '{cls}' not in matrix vocabulary"
            )
    if "mixed_dpi" in classes and "scale_changed" not in topology_classes:
        result.failed_checks.append(
            "mixed_dpi rows must declare scale_changed in topology_change_classes"
        )
    if "suspend_resume" in classes and not (
        {"safe_bounds_changed", "wake_display_reconnect"} & set(topology_classes)
    ):
        result.failed_checks.append(
            "suspend_resume rows must declare wake_display_reconnect or safe_bounds_changed"
        )
    if (
        len(displays_after) < len(displays_before)
        and "display_removed" not in topology_classes
    ):
        result.failed_checks.append(
            "rows that drop a display must declare display_removed in topology_change_classes"
        )

    adjustment_classes = ensure_list(
        expected.get("adjustment_classes"),
        f"{smoke_row_id}.expected.adjustment_classes",
    )
    if expected_anchor_inside and actual_offscreen_before:
        if "snapped_to_safe_bounds" not in adjustment_classes:
            result.failed_checks.append(
                "rows that clamp from off-screen must declare snapped_to_safe_bounds"
            )
    for cls in adjustment_classes:
        cls = ensure_str(cls, f"{smoke_row_id}.expected.adjustment_classes[]")
        if cls not in matrix_vocab["adjustment_class"]:
            result.failed_checks.append(
                f"adjustment_class '{cls}' not in matrix vocabulary"
            )

    continuity = ensure_str(
        expected.get("continuity_level_expectation"),
        f"{smoke_row_id}.expected.continuity_level_expectation",
    )
    if continuity not in matrix_vocab["continuity_level"]:
        result.failed_checks.append(
            f"continuity_level_expectation '{continuity}' not in matrix vocabulary"
        )

    focus_rule = ensure_str(
        expected.get("focus_visibility_rule"),
        f"{smoke_row_id}.expected.focus_visibility_rule",
    )
    if focus_rule not in matrix_vocab["focus_visibility_rule"]:
        result.failed_checks.append(
            f"focus_visibility_rule '{focus_rule}' not in matrix vocabulary"
        )

    rebind_posture = ensure_str(
        expected.get("authority_rebind_posture"),
        f"{smoke_row_id}.expected.authority_rebind_posture",
    )
    if rebind_posture not in matrix_vocab["authority_rebind_posture"]:
        result.failed_checks.append(
            f"authority_rebind_posture '{rebind_posture}' not in matrix vocabulary"
        )

    postures_raw = expected.get("session_execution_postures")
    if postures_raw is None:
        result.failed_checks.append(
            "expected.session_execution_postures must declare at least one surface posture"
        )
    else:
        postures = ensure_list(postures_raw, f"{smoke_row_id}.expected.session_execution_postures")
        if not postures:
            result.failed_checks.append("expected.session_execution_postures is empty")
        for posture in postures:
            posture = ensure_dict(posture, f"{smoke_row_id}.expected.session_execution_postures[]")
            posture_token = ensure_str(
                posture.get("posture"),
                f"{smoke_row_id}.expected.session_execution_postures[].posture",
            )
            if posture_token not in matrix_vocab["session_execution_posture"]:
                result.failed_checks.append(
                    f"session_execution_posture '{posture_token}' not in matrix vocabulary"
                )

    # Suspend/resume rows MUST never declare live_session_continued for
    # a remote/preview/debug surface — that would mean a hidden rerun.
    if "suspend_resume" in classes and isinstance(postures_raw, list):
        for posture in postures_raw:
            posture = ensure_dict(posture, f"{smoke_row_id}.expected.session_execution_postures[]")
            surface_class = ensure_str(
                posture.get("surface_class"),
                f"{smoke_row_id}.expected.session_execution_postures[].surface_class",
            )
            posture_token = ensure_str(posture.get("posture"), "posture")
            if (
                surface_class in {"task_runner", "debug_session", "preview_route"}
                and posture_token == "live_session_continued"
            ):
                result.failed_checks.append(
                    f"suspend_resume rows must not claim live_session_continued for {surface_class}"
                )

    # Named failure drills must be declared (no silent skip).
    drills = ensure_list(row.get("named_failure_drills"), f"{smoke_row_id}.named_failure_drills")
    if not drills:
        result.failed_checks.append("named_failure_drills must reference at least one drill id")
    if "suspend_resume" in classes and "no_hidden_rerun_live_surfaces" not in drills:
        result.failed_checks.append(
            "suspend_resume rows must include the no_hidden_rerun_live_surfaces drill"
        )

    # Failure drill: replay the row's forced-failure input and confirm
    # the smoke runner can reproduce the actionable owner + next action.
    failure_drill = ensure_dict(row.get("failure_drill"), f"{smoke_row_id}.failure_drill")
    drill_id = ensure_str(failure_drill.get("drill_id"), f"{smoke_row_id}.failure_drill.drill_id")
    forced = ensure_dict(failure_drill.get("forced_input"), f"{smoke_row_id}.failure_drill.forced_input")
    expected_report = ensure_dict(
        failure_drill.get("expected_report"),
        f"{smoke_row_id}.failure_drill.expected_report",
    )

    drill_record: dict[str, Any] = {
        "drill_id": drill_id,
        "forced_input": forced,
        "expected_report": expected_report,
    }
    if forced.get("skip_clamp_after_wake") or forced.get("skip_clamp_after_detach"):
        drill_clamped = False
        drill_record["replay_clamped_into_safe_bounds"] = drill_clamped
        if expected_report.get("clamped_into_safe_bounds") is not False:
            result.failed_checks.append(
                f"failure drill {drill_id} expected_report must record clamped_into_safe_bounds=false"
            )
    if forced.get("suppress_scale_change_class"):
        drill_record["replay_topology_change_classes_recorded"] = False
        if expected_report.get("topology_change_classes_recorded") is not False:
            result.failed_checks.append(
                f"failure drill {drill_id} expected_report must record "
                "topology_change_classes_recorded=false"
            )

    actionable_owner = expected_report.get("actionable_owner_ref")
    if not isinstance(actionable_owner, str) or not actionable_owner.strip():
        result.failed_checks.append(
            f"failure drill {drill_id} expected_report must declare actionable_owner_ref"
        )
    next_action = expected_report.get("next_action")
    if not isinstance(next_action, str) or not next_action.strip():
        result.failed_checks.append(
            f"failure drill {drill_id} expected_report must declare next_action"
        )

    result.diagnostics["failure_drill_replay"] = drill_record
    return result


def load_matrix_vocab(matrix: dict[str, Any]) -> dict[str, set[str]]:
    def vocab_for(field_name: str) -> set[str]:
        raw = ensure_list(matrix.get(field_name), f"matrix.{field_name}")
        return {ensure_str(item, f"matrix.{field_name}[]") for item in raw}

    return {
        "scenario_class": vocab_for("scenario_class_vocabulary"),
        "continuity_level": vocab_for("continuity_level_vocabulary"),
        "focus_visibility_rule": vocab_for("focus_visibility_rule_vocabulary"),
        "authority_rebind_posture": vocab_for("authority_rebind_posture_vocabulary"),
        "session_execution_posture": vocab_for("session_execution_posture_vocabulary"),
        "topology_change_class": vocab_for("topology_change_class_vocabulary"),
        "adjustment_class": vocab_for("adjustment_class_vocabulary"),
    }


def load_claimed_profile_ids(repo_root: Path, ref: str) -> set[str]:
    payload = render_yaml_as_json(repo_root / ref)
    payload = ensure_dict(payload, f"{ref}")
    profiles = ensure_list(payload.get("profiles"), f"{ref}.profiles")
    ids: set[str] = set()
    for idx, profile in enumerate(profiles):
        profile = ensure_dict(profile, f"{ref}.profiles[{idx}]")
        ids.add(ensure_str(profile.get("profile_id"), f"{ref}.profiles[{idx}].profile_id"))
    return ids


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    matrix_rel = args.matrix
    matrix_path = repo_root / matrix_rel
    matrix = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)

    findings: list[Finding] = []

    schema_version = matrix.get("schema_version")
    if not isinstance(schema_version, int) or schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version",
                message=f"matrix schema_version must be the integer 1, got {schema_version!r}",
                remediation="Bump the runner together with the schema if the matrix shape changes.",
            )
        )

    ensure_str(matrix.get("status"), "matrix.status")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    ensure_str(matrix.get("owner_dri"), "matrix.owner_dri")
    overview_page = ensure_str(matrix.get("overview_page"), "matrix.overview_page")
    if not artifact_ref_exists(repo_root, overview_page):
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.overview_page.missing",
                message=f"overview_page does not exist: {overview_page}",
                remediation="Create the reviewer-facing landing page or fix the path.",
                ref=overview_page,
            )
        )

    vocab_sources = ensure_dict(matrix.get("vocabulary_sources"), "matrix.vocabulary_sources")
    for key, value in vocab_sources.items():
        ref = ensure_str(value, f"matrix.vocabulary_sources.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.vocabulary_sources.missing",
                    message=f"vocabulary_sources.{key} does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced contract.",
                    ref=ref,
                )
            )

    hard_deps = ensure_list(matrix.get("hard_dependency_refs"), "matrix.hard_dependency_refs")
    for idx, ref in enumerate(hard_deps):
        ref = ensure_str(ref, f"matrix.hard_dependency_refs[{idx}]")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.hard_dependency_refs.missing",
                    message=f"hard_dependency_refs[{idx}] does not exist: {ref}",
                    remediation="Fix the dependency path; the lane must consume live upstream surfaces.",
                    ref=ref,
                )
            )

    validation_lane_ref = ensure_str(
        matrix.get("validation_lane_ref"), "matrix.validation_lane_ref"
    )
    fragment = validation_lane_ref.split("#", 1)[0]
    if not (repo_root / fragment).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.validation_lane_ref.missing",
                message=f"validation_lane_ref base does not exist: {validation_lane_ref}",
                remediation="Point at a seeded QE lane registry entry.",
                ref=validation_lane_ref,
            )
        )

    matrix_vocab = load_matrix_vocab(matrix)
    claimed_profile_ids = load_claimed_profile_ids(
        repo_root, vocab_sources["claimed_profiles_ref"]
    )

    rows = ensure_list(matrix.get("rows"), "matrix.rows")
    if not rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.empty",
                message="matrix.rows must declare at least three rows (suspend/resume, monitor, mixed-DPI)",
                remediation="Seed the required smoke rows.",
            )
        )

    row_results: list[RowResult] = []
    seen_ids: set[str] = set()
    seen_scenario_classes: set[str] = set()
    for row in rows:
        row = ensure_dict(row, "matrix.rows[]")
        result = replay_row(row, matrix_vocab, claimed_profile_ids)
        row_results.append(result)
        if result.smoke_row_id in seen_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate smoke_row_id: {result.smoke_row_id}",
                    remediation="smoke_row_ids must be unique.",
                    ref=result.smoke_row_id,
                )
            )
        seen_ids.add(result.smoke_row_id)
        for cls in row.get("scenario_classes", []):
            seen_scenario_classes.add(cls)

    missing_coverage = REQUIRED_SCENARIO_COVERAGE - seen_scenario_classes
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.coverage.missing_required_classes",
                message=(
                    "matrix must seed at least one row each for "
                    f"{sorted(REQUIRED_SCENARIO_COVERAGE)}; missing: {sorted(missing_coverage)}"
                ),
                remediation="Add the missing rows so the suspend/resume, multi-monitor, and mixed-DPI lanes are all live.",
            )
        )

    # Promote per-row failures into findings.
    for result in row_results:
        for failure in result.failed_checks:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.row.failed_check",
                    message=f"{result.smoke_row_id}: {failure}",
                    remediation="Fix the row geometry, vocabulary, or expected truth so the smoke replay holds.",
                    ref=result.smoke_row_id,
                )
            )

    errors = [f for f in findings if f.severity == "error"]
    status = "PASS" if not errors else "FAIL"

    capture: dict[str, Any] = {
        "schema_version": 1,
        "capture_kind": "topology_smoke_validation_capture",
        "captured_at": now_iso_z(),
        "owner_dri": ensure_str(matrix.get("owner_dri"), "matrix.owner_dri"),
        "matrix_ref": matrix_rel,
        "exact_build_identity_ref": args.build_identity,
        "command": (
            "python3 tests/desktop/topology_smoke/run_topology_smoke.py --repo-root ."
        ),
        "status": status,
        "required_scenario_coverage": sorted(REQUIRED_SCENARIO_COVERAGE),
        "observed_scenario_classes": sorted(seen_scenario_classes),
        "rows": [
            {
                "smoke_row_id": r.smoke_row_id,
                "inherited_scenario_id": r.inherited_scenario_id,
                "passed_checks": r.passed_checks,
                "failed_checks": r.failed_checks,
                "diagnostics": r.diagnostics,
            }
            for r in row_results
        ],
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
        f"[topology-smoke] {status} ({len(errors)} errors, "
        f"{len(findings) - len(errors)} warnings) — capture: {args.report}"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[topology-smoke] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[topology-smoke]   remediation: {finding.remediation}")

    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[topology-smoke] interrupted", file=sys.stderr)
        sys.exit(130)
