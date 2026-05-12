#!/usr/bin/env python3
"""Headless binary smoke for protected fixture repositories.

Reads ``artifacts/milestones/m1/dogfood_matrix.yaml``, launches the
``aureline_shell`` binary against each protected fixture repository for the
currently supported action kinds, validates the startup trace, and exercises
the binary's private headless edit/save hook on temporary fixture copies.

YAML decoding goes through Ruby/Psych, matching the repository convention used
by adjacent audit runners.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/milestones/m1/dogfood_matrix.yaml"
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/integrated_flow_smoke_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"
DEFAULT_BINARY_REL = "target/debug/aureline_shell"

SUPPORTED_ACTIONS = {"open", "quick_open", "edit_save"}
PENDING_ACTIONS = {
    "terminal": "pending_upstream: integrated terminal command execution is not yet claim-bearing",
    "restore_session": "pending_upstream: restore-session execution is not yet claim-bearing",
    "missing_target_recovery": "pending_upstream: missing-target restore recovery is not yet claim-bearing",
}

CHECK_ID_BINARY_BUILD_FAILED = "integrated_flow_smoke.binary.build_failed"
CHECK_ID_BINARY_MISSING = "integrated_flow_smoke.binary.missing"
CHECK_ID_FIXTURE_MISSING = "integrated_flow_smoke.fixture.missing"
CHECK_ID_MATRIX_SCHEMA = "integrated_flow_smoke.matrix.schema_violation"
CHECK_ID_QUICK_OPEN_TARGET_MISSING = "integrated_flow_smoke.quick_open.target_missing"
CHECK_ID_EDIT_TARGET_MISSING = "integrated_flow_smoke.edit_save.target_missing"
CHECK_ID_SHELL_EXIT_NONZERO = "integrated_flow_smoke.shell.exit_nonzero"
CHECK_ID_TRACE_MISSING = "integrated_flow_smoke.trace.missing"
CHECK_ID_TRACE_PARSE_FAILED = "integrated_flow_smoke.trace.parse_failed"
CHECK_ID_FIRST_FRAME_MISSING = "integrated_flow_smoke.trace.first_frame_missing"
CHECK_ID_PROTECTED_BUDGETS_MISSING = "integrated_flow_smoke.trace.protected_budgets_missing"
CHECK_ID_HEADLESS_EDIT_SAVE_FAILED = "integrated_flow_smoke.edit_save.headless_failed"
CHECK_ID_ROUND_TRIP_MISMATCH = "integrated_flow_smoke.edit_save.round_trip_mismatch"
CHECK_ID_FORCE_DRILL_EXPECTED_FAILURE = (
    "integrated_flow_smoke.force_drill.workspace_path_missing"
)
CHECK_ID_FORCE_DRILL_NOT_REPRODUCED = "integrated_flow_smoke.force_drill.not_reproduced"


@dataclass
class Finding:
    severity: str
    check_id: str
    message: str
    remediation: str
    row_id: str | None = None
    ref: str | None = None
    details: dict[str, Any] = field(default_factory=dict)

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["row_id"] is None:
            payload.pop("row_id")
        if payload["ref"] is None:
            payload.pop("ref")
        if not payload["details"]:
            payload.pop("details")
        return payload


@dataclass
class ActionObservation:
    row_id: str
    fixture_row_id: str
    action_kind: str
    status: str
    fixture_root_ref: str
    entry_file_ref: str | None = None
    pending_reason: str | None = None
    shell_exit_code: int | None = None
    observed_startup_event_sequence: list[dict[str, Any]] = field(default_factory=list)
    protected_journey_budgets: list[dict[str, str]] = field(default_factory=list)
    exact_build_identity_ref: str | None = None
    quick_open_target: str | None = None
    edit_save_target: str | None = None
    edit_save_round_trip_bytes: int | None = None
    headless_report: dict[str, Any] | None = None

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        for key in list(payload.keys()):
            if payload[key] is None:
                payload.pop(key)
        return payload


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--report", default=DEFAULT_REPORT_REL)
    parser.add_argument("--build-identity", default=DEFAULT_BUILD_IDENTITY_REL)
    parser.add_argument(
        "--binary",
        default=None,
        help="Path to aureline_shell. Defaults to target/debug/aureline_shell.",
    )
    parser.add_argument(
        "--no-build",
        action="store_true",
        help="Do not build aureline_shell when the default binary is missing.",
    )
    parser.add_argument(
        "--timeout-seconds",
        type=int,
        default=30,
        help="Per-binary-launch timeout.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help="Inject the missing-workspace failure for one fixture row id.",
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


def now_iso_z() -> str:
    return (
        dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z")
    )


def load_json_file(repo_root: Path, rel: str, label: str) -> dict[str, Any]:
    path = repo_root / rel
    if not path.exists():
        raise SystemExit(f"missing {label}: {path}")
    return ensure_dict(json.loads(path.read_text(encoding="utf-8")), label)


def repo_display_path(repo_root: Path, path: Path) -> str:
    try:
        return path.resolve().relative_to(repo_root).as_posix()
    except ValueError:
        return str(path)


def resolve_binary(repo_root: Path, args: argparse.Namespace) -> tuple[Path | None, list[Finding]]:
    findings: list[Finding] = []
    if args.binary:
        binary = Path(args.binary)
        if not binary.is_absolute():
            binary = repo_root / binary
        if not binary.exists():
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_BINARY_MISSING,
                    message=f"configured aureline_shell binary does not exist: {binary}",
                    remediation="Build the binary or pass --binary with the correct path.",
                    ref=str(binary),
                )
            )
            return None, findings
        return binary, findings

    binary = repo_root / DEFAULT_BINARY_REL
    if binary.exists():
        return binary, findings
    if args.no_build:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_BINARY_MISSING,
                message=f"default aureline_shell binary does not exist: {binary}",
                remediation="Run cargo build -p aureline-shell --bin aureline_shell or omit --no-build.",
                ref=DEFAULT_BINARY_REL,
            )
        )
        return None, findings

    build = subprocess.run(
        ["cargo", "build", "-p", "aureline-shell", "--bin", "aureline_shell"],
        cwd=repo_root,
        capture_output=True,
        text=True,
    )
    if build.returncode != 0:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_BINARY_BUILD_FAILED,
                message="cargo build for aureline_shell failed",
                remediation="Fix the Rust build, then re-run the integrated flow smoke.",
                details={
                    "stdout_tail": build.stdout[-4000:],
                    "stderr_tail": build.stderr[-4000:],
                },
            )
        )
        return None, findings
    if not binary.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_BINARY_MISSING,
                message=f"cargo build completed but binary was not found: {binary}",
                remediation="Check the workspace target directory and pass --binary explicitly.",
                ref=DEFAULT_BINARY_REL,
            )
        )
        return None, findings
    return binary, findings


def load_matrix(repo_root: Path, matrix_rel: str) -> tuple[dict[str, Any], list[dict[str, Any]], list[str]]:
    matrix = ensure_dict(render_yaml_as_json(repo_root / matrix_rel), matrix_rel)
    matrix_meta = ensure_dict(matrix.get("matrix"), f"{matrix_rel}.matrix")
    action_kinds = [
        ensure_str(action, f"{matrix_rel}.matrix.required_action_kinds[]")
        for action in ensure_list(
            matrix_meta.get("required_action_kinds"),
            f"{matrix_rel}.matrix.required_action_kinds",
        )
    ]
    rows = [
        ensure_dict(row, f"{matrix_rel}.rows[]")
        for row in ensure_list(matrix.get("rows"), f"{matrix_rel}.rows")
    ]
    return matrix, rows, action_kinds


def run_shell_startup(
    *,
    binary: Path,
    fixture_root: Path,
    trace_path: Path,
    timeout_seconds: int,
) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [
            str(binary),
            "--open",
            str(fixture_root),
            "--exit-after-first-frame",
            "--emit-startup-trace",
            str(trace_path),
            "--renderer",
            "software",
        ],
        capture_output=True,
        text=True,
        timeout=timeout_seconds,
    )


def analyze_trace(trace_path: Path) -> tuple[list[dict[str, Any]], list[dict[str, str]], str | None, list[str]]:
    raw = json.loads(trace_path.read_text(encoding="utf-8"))
    events = ensure_list(raw, str(trace_path))
    sequence: list[dict[str, Any]] = []
    budgets: list[dict[str, str]] = []
    exact_ref: str | None = None
    notes: list[str] = []
    for idx, raw_event in enumerate(events):
        event = ensure_dict(raw_event, f"{trace_path}[{idx}]")
        note = event.get("note")
        if isinstance(note, str):
            notes.append(note)
        protected_journey = event.get("protected_journey")
        budget_ref = event.get("budget_ref")
        event_id = event.get("event_id")
        event_class = event.get("event_class")
        sequence.append(
            {
                "event_id": event_id,
                "event_class": event_class,
                "note": note,
                "protected_journey": protected_journey,
                "budget_ref": budget_ref,
            }
        )
        if isinstance(protected_journey, str) and protected_journey and isinstance(budget_ref, str) and budget_ref:
            budgets.append(
                {
                    "protected_journey": protected_journey,
                    "budget_ref": budget_ref,
                }
            )
        if exact_ref is None and isinstance(event.get("exact_build_identity_ref"), str):
            exact_ref = event["exact_build_identity_ref"]
    return sequence, budgets, exact_ref, notes


def add_startup_findings(
    *,
    row_id: str,
    result: subprocess.CompletedProcess[str],
    trace_path: Path,
    findings: list[Finding],
) -> tuple[list[dict[str, Any]], list[dict[str, str]], str | None]:
    if result.returncode != 0:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_SHELL_EXIT_NONZERO,
                message=f"aureline_shell exited {result.returncode}",
                remediation="Fix the shell startup path for protected fixture repositories.",
                row_id=row_id,
                details={
                    "stdout_tail": result.stdout[-2000:],
                    "stderr_tail": result.stderr[-2000:],
                },
            )
        )
        return [], [], None
    if not trace_path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_TRACE_MISSING,
                message="startup trace file was not emitted",
                remediation="Ensure --emit-startup-trace writes the trace before first-frame exit.",
                row_id=row_id,
                ref=str(trace_path),
            )
        )
        return [], [], None
    try:
        sequence, budgets, exact_ref, notes = analyze_trace(trace_path)
    except (json.JSONDecodeError, SystemExit) as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_TRACE_PARSE_FAILED,
                message=f"startup trace could not be parsed: {exc}",
                remediation="Emit valid JSON trace events from aureline_shell.",
                row_id=row_id,
                ref=str(trace_path),
            )
        )
        return [], [], None
    if "shell.first_frame_submit" not in notes:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FIRST_FRAME_MISSING,
                message="startup trace did not contain shell.first_frame_submit",
                remediation="Keep first-frame milestone emission wired to the native shell submit path.",
                row_id=row_id,
            )
        )
    if not budgets:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_PROTECTED_BUDGETS_MISSING,
                message="startup trace did not report protected_journey + budget_ref pairs",
                remediation="Keep protected journey budget fields populated on startup trace events.",
                row_id=row_id,
            )
        )
    return sequence, budgets, exact_ref


def extract_quick_open_target(action: dict[str, Any]) -> str | None:
    command = action.get("command")
    if not isinstance(command, str):
        return None
    match = re.search(r"type\s+([^→`]+?)(?:\s*→|$)", command)
    if not match:
        return None
    return match.group(1).strip()


def extract_edit_target(action: dict[str, Any]) -> str | None:
    command = action.get("command")
    if not isinstance(command, str):
        return None
    match = re.search(r"Edit\s+(.+?)\s+by\s+", command)
    if not match:
        return None
    return match.group(1).strip()


def fixture_contains_target(fixture_root: Path, target: str) -> bool:
    target = target.replace("\\", "/")
    if "/" in target:
        return (fixture_root / target).is_file()
    for root, _dirs, files in os.walk(fixture_root):
        if target in files:
            return True
    return False


def run_headless_edit_save(
    *,
    binary: Path,
    fixture_root: Path,
    target_rel: str,
    timeout_seconds: int,
) -> tuple[subprocess.CompletedProcess[str], bytes, Path, dict[str, Any] | None]:
    payload = (
        b"aureline-integrated-flow-smoke\n"
        b"known-byte-sequence: 617572656c696e65\n"
    )
    report_path = fixture_root.parent / "headless_edit_save_report.json"
    result = subprocess.run(
        [
            str(binary),
            "--open",
            str(fixture_root),
            "--headless-test-edit-save",
            target_rel,
            "--headless-test-write-hex",
            payload.hex(),
            "--headless-test-report",
            str(report_path),
        ],
        capture_output=True,
        text=True,
        timeout=timeout_seconds,
    )
    report: dict[str, Any] | None = None
    if report_path.exists():
        report = ensure_dict(json.loads(report_path.read_text(encoding="utf-8")), str(report_path))
    return result, payload, fixture_root / target_rel, report


def copy_fixture_to_temp(fixture_root: Path, temp_root: Path) -> Path:
    copied = temp_root / fixture_root.name
    copied.parent.mkdir(parents=True, exist_ok=True)
    shutil.copytree(fixture_root, copied)
    return copied


def observe_action(
    *,
    binary: Path,
    repo_root: Path,
    row: dict[str, Any],
    action_kind: str,
    timeout_seconds: int,
    temp_root: Path,
    findings: list[Finding],
) -> ActionObservation:
    fixture_row_id = ensure_str(row.get("row_id"), "row.row_id")
    row_id = f"{fixture_row_id}:{action_kind}"
    fixture_root_ref = ensure_str(row.get("repo_root_ref"), f"{fixture_row_id}.repo_root_ref")
    entry_file_ref = row.get("entry_file_ref")
    fixture_root = repo_root / fixture_root_ref
    observation = ActionObservation(
        row_id=row_id,
        fixture_row_id=fixture_row_id,
        action_kind=action_kind,
        status="failed",
        fixture_root_ref=fixture_root_ref,
        entry_file_ref=entry_file_ref if isinstance(entry_file_ref, str) else None,
    )

    if action_kind in PENDING_ACTIONS:
        observation.status = "pending_upstream"
        observation.pending_reason = PENDING_ACTIONS[action_kind]
        return observation

    if action_kind not in SUPPORTED_ACTIONS:
        observation.status = "pending_upstream"
        observation.pending_reason = "pending_upstream: action kind has no binary smoke contract yet"
        return observation

    if not fixture_root.is_dir():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FIXTURE_MISSING,
                message=f"fixture root does not exist: {fixture_root_ref}",
                remediation="Restore the protected fixture repository or update the matrix ref.",
                row_id=row_id,
                ref=fixture_root_ref,
            )
        )
        return observation

    trace_path = temp_root / f"{fixture_row_id.replace('.', '_')}_{action_kind}_startup_trace.json"
    try:
        result = run_shell_startup(
            binary=binary,
            fixture_root=fixture_root,
            trace_path=trace_path,
            timeout_seconds=timeout_seconds,
        )
    except subprocess.TimeoutExpired as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_SHELL_EXIT_NONZERO,
                message=f"aureline_shell timed out after {timeout_seconds}s",
                remediation="Fix the first-frame exit path for protected fixture repositories.",
                row_id=row_id,
                details={
                    "stdout_tail": (exc.stdout or "")[-2000:],
                    "stderr_tail": (exc.stderr or "")[-2000:],
                },
            )
        )
        return observation
    observation.shell_exit_code = result.returncode
    sequence, budgets, exact_ref = add_startup_findings(
        row_id=row_id,
        result=result,
        trace_path=trace_path,
        findings=findings,
    )
    observation.observed_startup_event_sequence = sequence
    observation.protected_journey_budgets = budgets
    observation.exact_build_identity_ref = exact_ref

    actions = ensure_dict(row.get("actions"), f"{fixture_row_id}.actions")
    action = ensure_dict(actions.get(action_kind), f"{fixture_row_id}.actions.{action_kind}")

    if action_kind == "quick_open":
        target = extract_quick_open_target(action)
        observation.quick_open_target = target
        if not target or not fixture_contains_target(fixture_root, target):
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_QUICK_OPEN_TARGET_MISSING,
                    message=f"quick-open target is not present in fixture: {target!r}",
                    remediation="Align the dogfood quick-open command with a real fixture file.",
                    row_id=row_id,
                    ref=fixture_root_ref,
                )
            )

    if action_kind == "edit_save":
        target = extract_edit_target(action)
        observation.edit_save_target = target
        if not target or not (fixture_root / target).is_file():
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_EDIT_TARGET_MISSING,
                    message=f"edit/save target is not present in fixture: {target!r}",
                    remediation="Align the dogfood edit/save command with a real fixture file.",
                    row_id=row_id,
                    ref=fixture_root_ref,
                )
            )
        else:
            fixture_copy = copy_fixture_to_temp(fixture_root, temp_root / "edit_save")
            try:
                edit_result, payload, edited_path, headless_report = run_headless_edit_save(
                    binary=binary,
                    fixture_root=fixture_copy,
                    target_rel=target,
                    timeout_seconds=timeout_seconds,
                )
            except subprocess.TimeoutExpired as exc:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_HEADLESS_EDIT_SAVE_FAILED,
                        message=f"headless edit/save timed out after {timeout_seconds}s",
                        remediation="Keep the binary headless edit/save hook bounded and non-interactive.",
                        row_id=row_id,
                        details={
                            "stdout_tail": (exc.stdout or "")[-2000:],
                            "stderr_tail": (exc.stderr or "")[-2000:],
                        },
                    )
                )
                return observation
            observation.headless_report = headless_report
            observation.edit_save_round_trip_bytes = len(payload)
            if edit_result.returncode != 0:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_HEADLESS_EDIT_SAVE_FAILED,
                        message=f"headless edit/save exited {edit_result.returncode}",
                        remediation="Keep the binary headless edit/save hook wired to the staged save path.",
                        row_id=row_id,
                        details={
                            "stdout_tail": edit_result.stdout[-2000:],
                            "stderr_tail": edit_result.stderr[-2000:],
                        },
                    )
                )
            elif edited_path.read_bytes() != payload:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=CHECK_ID_ROUND_TRIP_MISMATCH,
                        message="headless edit/save bytes did not round-trip on disk",
                        remediation="Fix the edit/save path so saved bytes match the staged payload exactly.",
                        row_id=row_id,
                        ref=str(edited_path),
                    )
                )

    row_error = any(f.severity == "error" and f.row_id == row_id for f in findings)
    observation.status = "passed" if not row_error else "failed"
    return observation


def run_force_drill(
    *,
    binary: Path,
    repo_root: Path,
    rows: list[dict[str, Any]],
    requested_row_id: str,
    timeout_seconds: int,
) -> tuple[list[ActionObservation], list[Finding]]:
    findings: list[Finding] = []
    fixture_row_id = requested_row_id.split(":", 1)[0]
    selected = None
    for row in rows:
        if row.get("row_id") == fixture_row_id:
            selected = row
            break
    if selected is None:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_MATRIX_SCHEMA,
                message=f"force-drill row id was not found: {requested_row_id}",
                remediation="Pass a row_id from the dogfood matrix.",
                row_id=requested_row_id,
            )
        )
        return [], findings

    missing_root = repo_root / ".tmp" / "integrated_flow_smoke_missing_fixture"
    trace_path = repo_root / ".tmp" / "integrated_flow_smoke_force_drill_trace.json"
    row_id = f"{fixture_row_id}:force_drill"
    observation = ActionObservation(
        row_id=row_id,
        fixture_row_id=fixture_row_id,
        action_kind="force_drill",
        status="failed",
        fixture_root_ref=str(missing_root),
    )
    try:
        result = subprocess.run(
            [
                str(binary),
                "--open",
                str(missing_root),
                "--exit-after-first-frame",
                "--emit-startup-trace",
                str(trace_path),
            ],
            capture_output=True,
            text=True,
            timeout=timeout_seconds,
        )
        observation.shell_exit_code = result.returncode
    except subprocess.TimeoutExpired as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FORCE_DRILL_NOT_REPRODUCED,
                message=f"force drill timed out after {timeout_seconds}s",
                remediation="Ensure missing --open targets fail before shell startup blocks.",
                row_id=row_id,
                details={
                    "stdout_tail": (exc.stdout or "")[-2000:],
                    "stderr_tail": (exc.stderr or "")[-2000:],
                },
            )
        )
        return [observation], findings
    if result.returncode != 0 and "workspace path does not exist" in result.stderr:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FORCE_DRILL_EXPECTED_FAILURE,
                message="force drill reproduced the missing-workspace failure",
                remediation="This is the expected force-drill failure; run without --force-drill for the passing lane.",
                row_id=row_id,
                details={"stderr_tail": result.stderr[-2000:]},
            )
        )
    else:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FORCE_DRILL_NOT_REPRODUCED,
                message="force drill did not reproduce the expected missing-workspace failure",
                remediation="Ensure aureline_shell rejects missing --open targets with a typed startup failure.",
                row_id=row_id,
                details={
                    "exit_code": result.returncode,
                    "stdout_tail": result.stdout[-2000:],
                    "stderr_tail": result.stderr[-2000:],
                },
            )
        )
    return [observation], findings


def write_capture(
    *,
    repo_root: Path,
    report_rel: str,
    command: str,
    matrix_rel: str,
    build_identity_rel: str,
    build_identity: dict[str, Any],
    binary: Path | None,
    observations: list[ActionObservation],
    findings: list[Finding],
    force_drill: str | None,
) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    exact_refs = sorted(
        {
            obs.exact_build_identity_ref
            for obs in observations
            if isinstance(obs.exact_build_identity_ref, str)
        }
    )
    payload = {
        "schema_version": 1,
        "capture_kind": "integrated_flow_smoke_validation_capture",
        "generated_at": now_iso_z(),
        "command": command,
        "matrix_ref": matrix_rel,
        "build_identity_ref": build_identity_rel,
        "build_identity": build_identity,
        "binary_ref": repo_display_path(repo_root, binary) if binary is not None else None,
        "exact_build_identity_refs": exact_refs,
        "supported_action_kinds": sorted(SUPPORTED_ACTIONS),
        "pending_action_kinds": sorted(PENDING_ACTIONS),
        "force_drill": force_drill,
        "row_counts": {
            "passed": sum(1 for obs in observations if obs.status == "passed"),
            "failed": sum(1 for obs in observations if obs.status == "failed"),
            "pending_upstream": sum(
                1 for obs in observations if obs.status == "pending_upstream"
            ),
        },
        "finding_counts": {
            "error": sum(1 for finding in findings if finding.severity == "error"),
            "warning": sum(1 for finding in findings if finding.severity == "warning"),
        },
        "rows": [obs.as_report() for obs in observations],
        "findings": [finding.as_report() for finding in findings],
    }
    if payload["binary_ref"] is None:
        payload.pop("binary_ref")
    if payload["force_drill"] is None:
        payload.pop("force_drill")
    report_path.write_text(json.dumps(payload, indent=2) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    matrix, rows, action_kinds = load_matrix(repo_root, args.matrix)
    _ = matrix
    build_identity = load_json_file(repo_root, args.build_identity, "build identity")
    binary, findings = resolve_binary(repo_root, args)
    observations: list[ActionObservation] = []

    if binary is not None:
        if args.force_drill:
            observations, drill_findings = run_force_drill(
                binary=binary,
                repo_root=repo_root,
                rows=rows,
                requested_row_id=args.force_drill,
                timeout_seconds=args.timeout_seconds,
            )
            findings.extend(drill_findings)
        else:
            with tempfile.TemporaryDirectory(prefix="aureline_integrated_flow_") as temp_dir:
                temp_root = Path(temp_dir)
                for row in rows:
                    for action_kind in action_kinds:
                        observations.append(
                            observe_action(
                                binary=binary,
                                repo_root=repo_root,
                                row=row,
                                action_kind=action_kind,
                                timeout_seconds=args.timeout_seconds,
                                temp_root=temp_root,
                                findings=findings,
                            )
                        )

    command = " ".join(["python3", "tests/desktop/m1_integrated_flow_smoke/run_integrated_flow_smoke.py"] + sys.argv[1:])
    write_capture(
        repo_root=repo_root,
        report_rel=args.report,
        command=command,
        matrix_rel=args.matrix,
        build_identity_rel=args.build_identity,
        build_identity=build_identity,
        binary=binary,
        observations=observations,
        findings=findings,
        force_drill=args.force_drill,
    )

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(
        f"[integrated-flow-smoke] {status} "
        f"({len(errors)} errors, {len(warnings)} warnings, {len(observations)} rows)"
    )
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        row_suffix = f" ({finding.row_id})" if finding.row_id else ""
        print(
            f"[integrated-flow-smoke] {prefix} {finding.check_id}: "
            f"{finding.message}{row_suffix}{ref_suffix}"
        )
        print(f"[integrated-flow-smoke]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[integrated-flow-smoke] interrupted", file=sys.stderr)
        sys.exit(130)
