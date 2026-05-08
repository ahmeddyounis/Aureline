#!/usr/bin/env python3
"""Validate the proof artifact index and its storage conventions.

This check keeps the proof index executable by ensuring the canonical
artifact index:

- exists and parses;
- carries a stable schema/owner header;
- declares storage roots for M1 review artifacts; and
- registers required proof lanes with owner + packet + freshness metadata.
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


DEFAULT_INDEX_REL = "artifacts/milestones/m1/artifact_index.yaml"

SENTINEL_REFS = {
    "not_yet_seeded",
    "outline_only",
    "planned_not_yet_seeded",
}

FRESHNESS_STATES = {
    "current",
    "needs_refresh",
    "stale",
    "planned_not_yet_seeded",
}

REQUIRED_LANES = {
    "renderer",
    "save_recovery",
    "start_center",
    "terminal",
    "support_bundle",
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
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--index", default=DEFAULT_INDEX_REL)
    parser.add_argument(
        "--report",
        default=None,
        help="Write a machine-readable JSON report to this repo-relative path.",
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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = strip_fragment(ref)
    return bool(ref) and (repo_root / ref).exists()


def is_sentinel_ref(ref: str) -> bool:
    return ref.strip() in SENTINEL_REFS


def is_under_any_root(ref: str, roots: list[str]) -> bool:
    target = strip_fragment(ref)
    if not target:
        return False
    for root in roots:
        root_path = root.rstrip("/").strip()
        if not root_path:
            continue
        if target == root_path or target.startswith(root_path + "/"):
            return True
    return False


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"schema_version must be 1, got {schema_version}",
                remediation="Bump the validator in the same change that bumps the schema version.",
            )
        )

    as_of = ensure_str(payload.get("as_of"), f"{label}.as_of")
    _ = parse_iso_date(as_of, f"{label}.as_of")

    _ = ensure_str(payload.get("owner"), f"{label}.owner")


def validate_storage_conventions(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> list[str]:
    roots = ensure_list(index.get("storage_roots"), "index.storage_roots")
    root_refs: list[str] = []
    for idx, raw_root in enumerate(roots):
        row = ensure_dict(raw_root, f"index.storage_roots[{idx}]")
        root_id = ensure_str(row.get("root_id"), f"index.storage_roots[{idx}].root_id")
        root_ref = ensure_str(row.get("root_ref"), f"index.storage_roots[{idx}].root_ref")
        root_refs.append(root_ref)
        if is_sentinel_ref(root_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.storage_roots.sentinel",
                    message=f"storage root {root_id} cannot be a sentinel ref: {root_ref}",
                    remediation="Seed the storage root directory and reference its repo-relative path.",
                    ref=root_ref,
                )
            )
            continue
        if not artifact_ref_exists(repo_root, root_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.storage_roots.missing",
                    message=f"storage root {root_id} does not exist: {root_ref}",
                    remediation="Create the directory (or fix the path) so evidence has a governed sink.",
                    ref=root_ref,
                )
            )
    if not root_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="index.storage_roots.empty",
                message="storage_roots must declare at least one governed evidence root",
                remediation="Declare the M1 evidence roots under artifacts/milestones/m1/.",
            )
        )
    return root_refs


def validate_canonical_artifacts(repo_root: Path, index: dict[str, Any], findings: list[Finding]) -> dict[str, str]:
    canonical_artifacts = ensure_list(index.get("canonical_artifacts"), "index.canonical_artifacts")
    refs_by_id: dict[str, str] = {}
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        ensure_str(row.get("title"), f"index.canonical_artifacts[{idx}].title")
        ensure_str(row.get("owner_dri"), f"index.canonical_artifacts[{idx}].owner_dri")
        refs_by_id[artifact_id] = artifact_ref
        if not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifacts.missing_ref",
                    message=f"canonical_artifacts[{idx}] artifact_ref does not exist: {artifact_ref}",
                    remediation="Fix the path or seed the referenced artifact so consumers can resolve it.",
                    ref=artifact_ref,
                )
            )
    return refs_by_id


def validate_proof_lanes(
    repo_root: Path,
    index: dict[str, Any],
    storage_roots: list[str],
    findings: list[Finding],
) -> None:
    lanes = ensure_list(index.get("proof_lanes"), "index.proof_lanes")
    seen: set[str] = set()
    for idx, raw_lane in enumerate(lanes):
        lane = ensure_dict(raw_lane, f"index.proof_lanes[{idx}]")
        lane_id = ensure_str(lane.get("lane_id"), f"index.proof_lanes[{idx}].lane_id")
        if lane_id in seen:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.duplicate",
                    message=f"duplicate lane_id: {lane_id}",
                    remediation="Lane IDs must be stable and unique.",
                    ref=lane_id,
                )
            )
        seen.add(lane_id)

        ensure_str(lane.get("title"), f"index.proof_lanes[{idx}].title")
        ensure_str(lane.get("owner_dri"), f"index.proof_lanes[{idx}].owner_dri")

        freshness = ensure_dict(lane.get("freshness"), f"index.proof_lanes[{idx}].freshness")
        freshness_state = ensure_str(freshness.get("state"), f"index.proof_lanes[{idx}].freshness.state")
        if freshness_state not in FRESHNESS_STATES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.freshness.invalid",
                    message=f"lane {lane_id} freshness.state must be one of {sorted(FRESHNESS_STATES)}, got {freshness_state}",
                    remediation="Use a supported freshness state so dashboards can interpret lane status consistently.",
                    ref=lane_id,
                )
            )

        owning_packet_ref = ensure_str(lane.get("owning_packet_ref"), f"index.proof_lanes[{idx}].owning_packet_ref")
        if not artifact_ref_exists(repo_root, owning_packet_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.owning_packet.missing",
                    message=f"lane {lane_id} owning_packet_ref does not exist: {owning_packet_ref}",
                    remediation="Seed the packet so reviewers can anchor the lane to one owning artifact.",
                    ref=owning_packet_ref,
                )
            )
        if not is_under_any_root(owning_packet_ref, storage_roots):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.owning_packet.outside_roots",
                    message=f"lane {lane_id} owning_packet_ref is outside declared storage roots: {owning_packet_ref}",
                    remediation="Store proof packets under the governed roots and update the storage_roots list if required.",
                    ref=owning_packet_ref,
                )
            )

        exact_build_identity_ref = ensure_str(
            lane.get("exact_build_identity_ref"),
            f"index.proof_lanes[{idx}].exact_build_identity_ref",
        )
        if not artifact_ref_exists(repo_root, exact_build_identity_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.build_identity.missing",
                    message=f"lane {lane_id} exact_build_identity_ref does not exist: {exact_build_identity_ref}",
                    remediation="Point to the checked-in build identity artifact so proof can join against one build identity.",
                    ref=exact_build_identity_ref,
                )
            )

        validation_lane_refs = ensure_list(
            lane.get("validation_lane_refs"),
            f"index.proof_lanes[{idx}].validation_lane_refs",
        )
        if not validation_lane_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.validation_lane_refs.empty",
                    message=f"lane {lane_id} must declare at least one validation lane ref",
                    remediation="Reference the protected QE lane(s) that back this proof lane.",
                    ref=lane_id,
                )
            )
        else:
            for lane_ref in validation_lane_refs:
                lane_ref = ensure_str(lane_ref, f"index.proof_lanes[{idx}].validation_lane_refs[]")
                if is_sentinel_ref(lane_ref):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="index.proof_lanes.validation_lane_refs.sentinel",
                            message=f"lane {lane_id} validation_lane_refs cannot be a sentinel value: {lane_ref}",
                            remediation="Point to a seeded QE lane registry ref (or seed the missing lane first).",
                            ref=lane_ref,
                        )
                    )
                elif not artifact_ref_exists(repo_root, lane_ref):
                    findings.append(
                        Finding(
                            severity="error",
                            check_id="index.proof_lanes.validation_lane_refs.missing",
                            message=f"lane {lane_id} validation_lane_ref does not exist: {lane_ref}",
                            remediation="Fix the ref path (or seed the referenced lane) so validation remains traceable.",
                            ref=lane_ref,
                        )
                    )

        evidence = ensure_list(lane.get("evidence_refs"), f"index.proof_lanes[{idx}].evidence_refs")
        for ref_idx, evidence_ref in enumerate(evidence):
            evidence_ref = ensure_str(evidence_ref, f"index.proof_lanes[{idx}].evidence_refs[{ref_idx}]")
            if is_sentinel_ref(evidence_ref):
                continue
            if not artifact_ref_exists(repo_root, evidence_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.evidence.missing",
                        message=f"lane {lane_id} evidence ref does not exist: {evidence_ref}",
                        remediation="Seed the evidence artifact (or mark it planned-only with a sentinel).",
                        ref=evidence_ref,
                    )
                )
                continue
            if not is_under_any_root(evidence_ref, storage_roots):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.evidence.outside_roots",
                        message=f"lane {lane_id} evidence ref is outside declared storage roots: {evidence_ref}",
                        remediation="Store evidence under governed roots and update storage_roots if needed.",
                        ref=evidence_ref,
                    )
                )

        latest_capture = lane.get("latest_capture")
        if freshness_state != "planned_not_yet_seeded" and latest_capture is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.proof_lanes.latest_capture.missing",
                    message=f"lane {lane_id} must register latest_capture when freshness.state is {freshness_state}",
                    remediation="Register the latest capture (or set freshness.state to planned_not_yet_seeded).",
                    ref=lane_id,
                )
            )
        if isinstance(latest_capture, dict):
            captured_at = ensure_str(latest_capture.get("captured_at"), f"index.proof_lanes[{idx}].latest_capture.captured_at")
            try:
                dt.datetime.fromisoformat(captured_at.replace("Z", "+00:00"))
            except ValueError:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.latest_capture.captured_at.invalid",
                        message=f"lane {lane_id} latest_capture.captured_at must be ISO-8601, got {captured_at!r}",
                        remediation="Use an ISO-8601 timestamp (UTC Z preferred).",
                        ref=lane_id,
                    )
                )
            _ = ensure_str(latest_capture.get("command"), f"index.proof_lanes[{idx}].latest_capture.command")
            report_ref = ensure_str(latest_capture.get("report_ref"), f"index.proof_lanes[{idx}].latest_capture.report_ref")
            if not artifact_ref_exists(repo_root, report_ref):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.latest_capture.report_ref.missing",
                        message=f"lane {lane_id} latest_capture.report_ref does not exist: {report_ref}",
                        remediation="Re-run the lane and check in the capture under the governed roots.",
                        ref=report_ref,
                    )
                )
            elif not is_under_any_root(report_ref, storage_roots):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.proof_lanes.latest_capture.report_ref.outside_roots",
                        message=f"lane {lane_id} latest_capture.report_ref is outside declared storage roots: {report_ref}",
                        remediation="Store captures under governed roots and update storage_roots if needed.",
                        ref=report_ref,
                    )
                )

    missing = REQUIRED_LANES - seen
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="index.proof_lanes.required.missing",
                message=f"proof_lanes is missing required lanes: {sorted(missing)}",
                remediation="Register the missing lanes so M1 proof does not fragment across ad hoc folders.",
            )
        )


def write_report(repo_root: Path, report_rel: str, index_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_proof_artifact_index",
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "index_ref": index_rel,
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
    }
    report_path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    index_rel = str(args.index)
    index_path = repo_root / index_rel
    index_payload = ensure_dict(render_yaml_as_json(index_path), index_rel)

    findings: list[Finding] = []
    validate_header(index_payload, "index", findings)

    _ = ensure_str(index_payload.get("index_id"), "index.index_id")
    human_entry = ensure_str(index_payload.get("human_entrypoint_ref"), "index.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="index.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {human_entry}",
                remediation="Create the reviewer entrypoint doc (or fix the ref path).",
                ref=human_entry,
            )
        )

    _ = validate_canonical_artifacts(repo_root, index_payload, findings)
    storage_roots = validate_storage_conventions(repo_root, index_payload, findings)
    validate_proof_lanes(repo_root, index_payload, storage_roots, findings)

    if args.report:
        write_report(repo_root, str(args.report), index_rel=index_rel, findings=findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"

    print(f"[m1-artifact-index] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[m1-artifact-index] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[m1-artifact-index]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m1-artifact-index] interrupted", file=sys.stderr)
        sys.exit(130)

