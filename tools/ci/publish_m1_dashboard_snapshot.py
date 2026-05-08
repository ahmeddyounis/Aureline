#!/usr/bin/env python3
"""Publish and validate the protected-functions proof dashboard snapshot.

This tool consumes:
  - artifacts/dashboards/m1_protected_functions.json
  - artifacts/milestones/m1/artifact_index.yaml

And emits:
  - a machine-readable snapshot (optional via --snapshot)
  - a machine-readable validation report (optional via --report)

The intent is to export a stable, check-in-able dashboard snapshot into the
milestone evidence roots so review packets never depend on live dashboards or
spreadsheets.
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


DEFAULT_DASHBOARD_REL = "artifacts/dashboards/m1_protected_functions.json"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"


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
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--dashboard", default=DEFAULT_DASHBOARD_REL)
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
    parser.add_argument(
        "--snapshot",
        default=None,
        help="Write a machine-readable dashboard snapshot to this repo-relative path.",
    )
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable validation report to this repo-relative path.",
    )
    return parser.parse_args()


def utc_now_iso() -> str:
    return dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z")


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be an object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be an array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ensure_int(value: Any, label: str) -> int:
    if not isinstance(value, int):
        raise SystemExit(f"{label} must be an integer")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def path_exists(repo_root: Path, ref: str) -> bool:
    ref = strip_fragment(ref)
    return bool(ref) and (repo_root / ref).exists()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


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


def validate_owner_handle(value: str, label: str, findings: list[Finding]) -> None:
    if not value.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id=f"{label}.owner.format",
                message=f"{label} owner does not look like a handle: {value!r}",
                remediation="Use an @handle so review routing is explicit.",
            )
        )


def validate_dashboard_definition(
    repo_root: Path,
    dashboard: dict[str, Any],
    dashboard_ref: str,
    findings: list[Finding],
) -> dict[str, Any]:
    schema_version = ensure_int(dashboard.get("schema_version"), "dashboard.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard.schema_version.unsupported",
                message=f"unsupported dashboard schema_version {schema_version} (expected 1)",
                remediation="Bump the validator in the same change that bumps the dashboard schema version.",
            )
        )

    as_of = ensure_str(dashboard.get("as_of"), "dashboard.as_of")
    parse_iso_date(as_of, "dashboard.as_of")

    owner = ensure_str(dashboard.get("owner"), "dashboard.owner")
    validate_owner_handle(owner, "dashboard", findings)

    ensure_str(dashboard.get("dashboard_id"), "dashboard.dashboard_id")
    ensure_str(dashboard.get("title"), "dashboard.title")
    artifact_index_ref = ensure_str(dashboard.get("artifact_index_ref"), "dashboard.artifact_index_ref")
    if not path_exists(repo_root, artifact_index_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard.artifact_index_ref.missing",
                message=f"artifact_index_ref does not exist: {artifact_index_ref}",
                remediation="Point artifact_index_ref at the canonical proof index path.",
                ref=artifact_index_ref,
            )
        )

    review_entrypoint_ref = ensure_str(dashboard.get("review_entrypoint_ref"), "dashboard.review_entrypoint_ref")
    if not path_exists(repo_root, review_entrypoint_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard.review_entrypoint_ref.missing",
                message=f"review_entrypoint_ref does not exist: {review_entrypoint_ref}",
                remediation="Seed the reviewer-facing workflow doc so dashboard exports have a stable entrypoint.",
                ref=review_entrypoint_ref,
            )
        )

    vocab = ensure_list(dashboard.get("freshness_state_vocabulary"), "dashboard.freshness_state_vocabulary")
    vocab = [ensure_str(value, "dashboard.freshness_state_vocabulary[]") for value in vocab]
    if len(set(vocab)) != len(vocab):
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard.freshness_state_vocabulary.duplicate",
                message="freshness_state_vocabulary must not contain duplicates",
                remediation="Keep the dashboard state vocabulary a unique list so consumers can rely on stable values.",
                ref=dashboard_ref,
            )
        )

    precedence = ensure_list(dashboard.get("row_state_precedence"), "dashboard.row_state_precedence")
    precedence = [ensure_str(value, "dashboard.row_state_precedence[]") for value in precedence]
    missing = sorted(set(vocab).difference(precedence))
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="dashboard.row_state_precedence.missing_states",
                message=f"row_state_precedence is missing states: {', '.join(missing)}",
                remediation="Include all freshness_state_vocabulary values so rollups are well-defined.",
                ref=dashboard_ref,
            )
        )

    thresholds = ensure_dict(dashboard.get("default_thresholds"), "dashboard.default_thresholds")
    for key in ("pass_freshness_states", "warn_freshness_states", "fail_freshness_states", "gap_freshness_states"):
        values = ensure_list(thresholds.get(key), f"dashboard.default_thresholds.{key}")
        for value in values:
            value = ensure_str(value, f"dashboard.default_thresholds.{key}[]")
            if vocab and value not in vocab:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="dashboard.default_thresholds.out_of_vocab",
                        message=f"default_thresholds.{key} references {value!r} which is not in freshness_state_vocabulary",
                        remediation="Use only freshness_state_vocabulary values in threshold sets so the mapping stays stable.",
                        ref=dashboard_ref,
                    )
                )

    rows = ensure_list(dashboard.get("rows"), "dashboard.rows")
    seen_row_ids: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"dashboard.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"dashboard.rows[{idx}].row_id")
        if row_id in seen_row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="dashboard.rows.duplicate_row_id",
                    message=f"duplicate row_id: {row_id}",
                    remediation="Row IDs must be stable and unique.",
                    ref=row_id,
                )
            )
        seen_row_ids.add(row_id)

        ensure_str(row.get("title"), f"dashboard.rows[{idx}].title")
        owner_dri = ensure_str(row.get("owner_dri"), f"dashboard.rows[{idx}].owner_dri")
        validate_owner_handle(owner_dri, f"dashboard.rows[{idx}]", findings)

        lane_ids = ensure_list(row.get("proof_lane_ids"), f"dashboard.rows[{idx}].proof_lane_ids")
        if not lane_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="dashboard.rows.missing_lane_ids",
                    message=f"row {row_id} must list at least one proof_lane_id",
                    remediation="Deep-link each row to at least one proof lane in artifacts/milestones/m1/artifact_index.yaml.",
                    ref=row_id,
                )
            )
        for lane_id in lane_ids:
            _ = ensure_str(lane_id, f"dashboard.rows[{idx}].proof_lane_ids[]")

        evidence_refs = ensure_list(row.get("evidence_refs"), f"dashboard.rows[{idx}].evidence_refs")
        for evidence_ref in evidence_refs:
            evidence_ref = ensure_str(evidence_ref, f"dashboard.rows[{idx}].evidence_refs[]")
            if not path_exists(repo_root, evidence_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="dashboard.rows.evidence_ref.missing",
                        message=f"row {row_id} evidence_ref does not exist: {evidence_ref}",
                        remediation="Fix the path or seed the referenced artifact so dashboard rows can deep-link to real evidence.",
                        ref=evidence_ref,
                    )
                )

    return {"vocab": vocab, "precedence": precedence, "thresholds": thresholds}


def classify_panel_state(row_state: str, thresholds: dict[str, Any]) -> str:
    def values(key: str) -> set[str]:
        raw = thresholds.get(key, [])
        if not isinstance(raw, list):
            return set()
        return {v for v in raw if isinstance(v, str)}

    if row_state in values("fail_freshness_states"):
        return "fail"
    if row_state in values("warn_freshness_states"):
        return "warn"
    if row_state in values("gap_freshness_states"):
        return "gap"
    if row_state in values("pass_freshness_states"):
        return "pass"
    return "unknown"


def derive_row_state(lane_states: list[str], precedence: list[str]) -> str:
    if not lane_states:
        return "unknown"
    order = {state: idx for idx, state in enumerate(precedence)}
    best = None
    for state in lane_states:
        idx = order.get(state)
        if idx is None:
            continue
        if best is None or idx < best:
            best = idx
    if best is None:
        return "unknown"
    return precedence[best]


def ensure_latest_capture_present(
    repo_root: Path,
    lane_id: str,
    lane: dict[str, Any],
    freshness_state: str,
    findings: list[Finding],
) -> None:
    if freshness_state == "planned_not_yet_seeded":
        return
    latest_capture = lane.get("latest_capture")
    if not isinstance(latest_capture, dict):
        findings.append(
            Finding(
                severity="error",
                check_id="artifact_index.lane.latest_capture.missing",
                message=f"proof lane {lane_id} freshness.state is {freshness_state} but latest_capture is missing",
                remediation="Record the latest capture (captured_at/command/report_ref) so dashboards can deep-link to current evidence.",
                ref=lane_id,
            )
        )
        return
    report_ref = latest_capture.get("report_ref")
    if not isinstance(report_ref, str) or not report_ref.strip():
        findings.append(
            Finding(
                severity="error",
                check_id="artifact_index.lane.latest_capture.report_ref.missing",
                message=f"proof lane {lane_id} latest_capture.report_ref is missing",
                remediation="Point report_ref at the latest machine-readable capture under artifacts/milestones/m1/captures/.",
                ref=lane_id,
            )
        )
        return
    report_ref = report_ref.strip()
    if not path_exists(repo_root, report_ref):
        findings.append(
            Finding(
                severity="error",
                check_id="artifact_index.lane.latest_capture.report_ref.not_found",
                message=f"proof lane {lane_id} latest_capture.report_ref does not exist: {report_ref}",
                remediation="Seed the referenced capture file or update the report_ref to match the latest run.",
                ref=report_ref,
            )
        )


def build_snapshot(
    repo_root: Path,
    dashboard: dict[str, Any],
    dashboard_ref: str,
    index: dict[str, Any],
    index_ref: str,
    meta: dict[str, Any],
    findings: list[Finding],
) -> dict[str, Any]:
    lanes = index.get("proof_lanes", [])
    if not isinstance(lanes, list):
        raise SystemExit("artifact_index.proof_lanes must be an array")
    lanes_by_id: dict[str, dict[str, Any]] = {}
    for raw_lane in lanes:
        if isinstance(raw_lane, dict) and isinstance(raw_lane.get("lane_id"), str):
            lanes_by_id[raw_lane["lane_id"]] = raw_lane

    rows = ensure_list(dashboard.get("rows"), "dashboard.rows")
    out_rows: list[dict[str, Any]] = []
    row_counts_by_state: dict[str, int] = {}
    row_counts_by_panel: dict[str, int] = {}

    precedence = meta["precedence"]
    thresholds = meta["thresholds"]

    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"dashboard.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"dashboard.rows[{idx}].row_id")
        title = ensure_str(row.get("title"), f"dashboard.rows[{idx}].title")
        lane_ids = ensure_list(row.get("proof_lane_ids"), f"dashboard.rows[{idx}].proof_lane_ids")
        lane_ids = [ensure_str(lane_id, f"dashboard.rows[{idx}].proof_lane_ids[]") for lane_id in lane_ids]

        lane_states: list[str] = []
        lane_summaries: list[dict[str, Any]] = []
        for lane_id in lane_ids:
            lane = lanes_by_id.get(lane_id)
            if lane is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="dashboard.rows.proof_lane_id.unknown",
                        message=f"row {row_id} references unknown proof_lane_id {lane_id!r}",
                        remediation="Use a lane_id that exists in artifacts/milestones/m1/artifact_index.yaml proof_lanes.",
                        ref=lane_id,
                    )
                )
                continue
            freshness = lane.get("freshness", {})
            freshness_state = ""
            if isinstance(freshness, dict):
                freshness_state = str(freshness.get("state") or "").strip()
            lane_states.append(freshness_state)
            ensure_latest_capture_present(repo_root, lane_id, lane, freshness_state, findings)
            owning_packet_ref = lane.get("owning_packet_ref")
            lane_summaries.append(
                {
                    "lane_id": lane_id,
                    "title": lane.get("title"),
                    "freshness_state": freshness_state,
                    "owning_packet_ref": owning_packet_ref,
                    "latest_capture": lane.get("latest_capture"),
                }
            )

        row_state = derive_row_state(lane_states, precedence)
        panel_state = classify_panel_state(row_state, thresholds)
        row_counts_by_state[row_state] = row_counts_by_state.get(row_state, 0) + 1
        row_counts_by_panel[panel_state] = row_counts_by_panel.get(panel_state, 0) + 1

        out_rows.append(
            {
                "row_id": row_id,
                "title": title,
                "state": row_state,
                "panel_state": panel_state,
                "proof_lane_summaries": lane_summaries,
                "evidence_refs": row.get("evidence_refs", []),
            }
        )

    return {
        "schema_version": 1,
        "dashboard_id": dashboard.get("dashboard_id"),
        "generated_at": utc_now_iso(),
        "dashboard_definition_ref": dashboard_ref,
        "artifact_index_ref": index_ref,
        "row_count_by_state": row_counts_by_state,
        "row_count_by_panel_state": row_counts_by_panel,
        "rows": out_rows,
    }


def write_json(path: Path, payload: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def print_summary(findings: list[Finding], snapshot: dict[str, Any]) -> None:
    counts = {"error": 0, "warning": 0}
    for finding in findings:
        if finding.severity in counts:
            counts[finding.severity] += 1
    row_counts = snapshot.get("row_count_by_panel_state", {})
    print("[proof-dashboards] snapshot exported summary")
    print(f"[proof-dashboards] findings: {counts['error']} error, {counts['warning']} warning")
    print(
        "[proof-dashboards] rows:"
        f" {row_counts.get('pass', 0)} pass,"
        f" {row_counts.get('warn', 0)} warn,"
        f" {row_counts.get('fail', 0)} fail,"
        f" {row_counts.get('gap', 0)} gap,"
        f" {row_counts.get('unknown', 0)} unknown"
    )
    if counts["error"]:
        for finding in findings:
            if finding.severity != "error":
                continue
            ref = f" ({finding.ref})" if finding.ref else ""
            print(f"[proof-dashboards] error: {finding.check_id}: {finding.message}{ref}", file=sys.stderr)


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    dashboard_ref = ensure_str(args.dashboard, "args.dashboard")
    index_ref = ensure_str(args.index, "args.index")

    dashboard_path = repo_root / dashboard_ref
    index_path = repo_root / index_ref

    dashboard = ensure_dict(load_json(dashboard_path), "dashboard")
    index = ensure_dict(render_yaml_as_json(index_path), "artifact_index")

    findings: list[Finding] = []
    meta = validate_dashboard_definition(repo_root, dashboard, dashboard_ref, findings)

    snapshot = build_snapshot(repo_root, dashboard, dashboard_ref, index, index_ref, meta, findings)
    print_summary(findings, snapshot)

    has_error = any(finding.severity == "error" for finding in findings)

    if args.snapshot and not has_error:
        snapshot_ref = ensure_str(args.snapshot, "args.snapshot")
        write_json(repo_root / snapshot_ref, snapshot)

    if args.report:
        report_ref = ensure_str(args.report, "args.report")
        counts = {"error": 0, "warning": 0}
        for finding in findings:
            if finding.severity in counts:
                counts[finding.severity] += 1
        report = {
            "schema_version": 1,
            "check_id": "m1_proof_dashboards_snapshot",
            "finding_counts": counts,
            "findings": [finding.as_report() for finding in findings],
            "generated_at": utc_now_iso(),
            "dashboard_ref": dashboard_ref,
            "index_ref": index_ref,
            "snapshot_ref": args.snapshot,
        }
        write_json(repo_root / report_ref, report)

    return 1 if has_error else 0


if __name__ == "__main__":
    raise SystemExit(main())
