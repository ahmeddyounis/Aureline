#!/usr/bin/env python3
"""Unattended operator-truth demo smoke for M1 exit review.

Walks the canonical operator-truth acceptance checklist at
``artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml``
and proves that every bounded M1 prototype wedge listed there:

- has a reviewer-facing landing page reachable from the repo;
- has an owning proof packet reachable from the repo;
- has a named protected-walk fixture the smoke can replay;
- carries every closed ``expected_tokens`` value verbatim on that
  fixture so the demo path quotes the same vocabulary the upstream
  wedge owns (rather than forking a local copy);
- has a named failure drill (drill id + forced mutation + expected
  ``check_id``) so this lane fails loudly when vocabulary drifts.

The runner emits a durable JSON capture (``--report``) and exits
non-zero on any regression. ``--force-drill <drill_id>`` replays
the named drill and exits 0 only when the runner reproduced
exactly the expected ``check_id``.

This smoke is an acceptance harness over already-landed bounded
wedges. It does NOT replace any upstream wedge's own validation
lane; each row points at the upstream ``validation_command``
reviewers can rerun directly to refresh the per-wedge evidence.

YAML decoding goes through Ruby/Psych, matching the repo-wide
convention used by adjacent audit runners.
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


DEFAULT_CHECKLIST_REL = (
    "artifacts/milestones/m1/operator_truth_acceptance_checklist.yaml"
)
DEFAULT_REPORT_REL = (
    "artifacts/milestones/m1/captures/"
    "operator_truth_demo_smoke_validation_capture.json"
)
DEFAULT_BUILD_IDENTITY_REL = "artifacts/build/build_identity.json"


# Stable check ids. Failure-drill `expected_check_id` values resolve
# against these.
CHECK_ID_CHECKLIST_SCHEMA_VIOLATION = (
    "operator_truth_demo_smoke.checklist.schema_violation"
)
CHECK_ID_REVIEWER_DOC_MISSING = (
    "operator_truth_demo_smoke.reviewer_doc.missing"
)
CHECK_ID_PROOF_PACKET_MISSING = (
    "operator_truth_demo_smoke.proof_packet.missing"
)
CHECK_ID_FIXTURE_MISSING = (
    "operator_truth_demo_smoke.protected_walk_fixture.missing"
)
CHECK_ID_FIXTURE_PARSE_FAILED = (
    "operator_truth_demo_smoke.protected_walk_fixture.parse_failed"
)
CHECK_ID_FIELD_PATH_MISSING = (
    "operator_truth_demo_smoke.acceptance_row.field_path_missing"
)
CHECK_ID_TOKEN_MISMATCH = (
    "operator_truth_demo_smoke.acceptance_row.expected_token_mismatch"
)
CHECK_ID_DUPLICATE_ROW = (
    "operator_truth_demo_smoke.acceptance_row.duplicate_row_id"
)
CHECK_ID_CLAIM_LIMITS_MISSING = (
    "operator_truth_demo_smoke.checklist.claim_limits_missing"
)
CHECK_ID_VALIDATION_COMMAND_MISSING = (
    "operator_truth_demo_smoke.acceptance_row.validation_command_missing"
)
CHECK_ID_FAILURE_DRILL_MISSING = (
    "operator_truth_demo_smoke.acceptance_row.failure_drill_missing"
)
CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING = (
    "operator_truth_demo_smoke.failure_drill.expected_finding_missing"
)


REQUIRED_CLAIM_LIMITS: list[str] = [
    "single_bounded_acceptance_harness_only",
    "no_new_product_surface_invention",
    "no_replacement_for_per_wedge_proof_packets",
    "no_market_facing_claims_beyond_bounded_wedges",
]


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
class RowObservation:
    row_id: str
    wedge_id: str
    reviewer_doc_ref: str
    proof_packet_ref: str
    validation_command: str
    protected_walk_fixture_ref: str
    expected_tokens_observed: list[dict[str, Any]] = field(default_factory=list)
    failure_drill_id: str | None = None


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    parser.add_argument("--checklist", default=DEFAULT_CHECKLIST_REL)
    parser.add_argument(
        "--report",
        default=DEFAULT_REPORT_REL,
        help="Write a machine-readable JSON capture to this repo-relative path.",
    )
    parser.add_argument(
        "--build-identity",
        default=DEFAULT_BUILD_IDENTITY_REL,
        help="Repo-relative path to the build-identity artifact to record on the capture.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help="Replay one named failure drill (by drill_id) and exit 0 only when the expected check_id is reproduced.",
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


def repo_ref_exists(repo_root: Path, ref: str) -> bool:
    if not ref:
        return False
    return (repo_root / ref).exists()


# Resolve a dot-path like ``a.b.0.c`` against a JSON-shaped tree.
# Integer segments index into lists; non-integer segments index into
# mappings. Returns the value or raises KeyError on a missing path.
def resolve_field_path(data: Any, field_path: str) -> Any:
    segments = field_path.split(".") if field_path else []
    cursor: Any = data
    for segment in segments:
        if isinstance(cursor, list):
            try:
                idx = int(segment)
            except ValueError as exc:
                raise KeyError(
                    f"non-integer segment {segment!r} against list at {field_path!r}"
                ) from exc
            if idx < 0 or idx >= len(cursor):
                raise KeyError(
                    f"list index {idx} out of range at {field_path!r}"
                )
            cursor = cursor[idx]
        elif isinstance(cursor, dict):
            if segment not in cursor:
                raise KeyError(
                    f"missing key {segment!r} at {field_path!r}"
                )
            cursor = cursor[segment]
        else:
            raise KeyError(
                f"cannot descend into {type(cursor).__name__} at {field_path!r}"
            )
    return cursor


def validate_claim_limits(
    checklist: dict[str, Any], findings: list[Finding]
) -> None:
    declared = checklist.get("claim_limits")
    if not isinstance(declared, list) or not declared:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_CLAIM_LIMITS_MISSING,
                message="checklist.claim_limits must declare at least one bounded claim limit",
                remediation=(
                    "Add the canonical claim-limit set ("
                    + ", ".join(REQUIRED_CLAIM_LIMITS)
                    + ") to the checklist."
                ),
            )
        )
        return
    missing = [limit for limit in REQUIRED_CLAIM_LIMITS if limit not in declared]
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_CLAIM_LIMITS_MISSING,
                message=(
                    "checklist.claim_limits is missing required claim-limit rows: "
                    + ", ".join(missing)
                ),
                remediation=(
                    "Add the canonical claim-limit set so the smoke quotes them back verbatim."
                ),
                details={"missing": missing},
            )
        )


def validate_row(
    repo_root: Path,
    row: dict[str, Any],
    idx: int,
    findings: list[Finding],
    seen_row_ids: set[str],
    observations: list[RowObservation],
) -> None:
    label = f"acceptance_rows[{idx}]"
    row_id = ensure_str(row.get("row_id"), f"{label}.row_id")
    if row_id in seen_row_ids:
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_DUPLICATE_ROW,
                message=f"duplicate row_id: {row_id}",
                remediation="Pick a unique row_id per checklist row.",
                row_id=row_id,
                ref=row_id,
            )
        )
        return
    seen_row_ids.add(row_id)

    wedge_id = ensure_str(row.get("wedge_id"), f"{label}.wedge_id")
    reviewer_doc_ref = ensure_str(
        row.get("reviewer_doc_ref"), f"{label}.reviewer_doc_ref"
    )
    proof_packet_ref = ensure_str(
        row.get("proof_packet_ref"), f"{label}.proof_packet_ref"
    )
    fixture_ref = ensure_str(
        row.get("protected_walk_fixture_ref"),
        f"{label}.protected_walk_fixture_ref",
    )

    validation_command = row.get("validation_command")
    if not isinstance(validation_command, str) or not validation_command.strip():
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_VALIDATION_COMMAND_MISSING,
                message=(
                    f"row {row_id} must declare a validation_command "
                    "so reviewers can rerun the upstream wedge's lane directly"
                ),
                remediation=(
                    "Add the upstream wedge's cargo or python validation command."
                ),
                row_id=row_id,
                ref=row_id,
            )
        )
        validation_command = ""

    observation = RowObservation(
        row_id=row_id,
        wedge_id=wedge_id,
        reviewer_doc_ref=reviewer_doc_ref,
        proof_packet_ref=proof_packet_ref,
        validation_command=validation_command.strip(),
        protected_walk_fixture_ref=fixture_ref,
    )

    if not repo_ref_exists(repo_root, reviewer_doc_ref):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_REVIEWER_DOC_MISSING,
                message=f"row {row_id} reviewer_doc_ref does not exist: {reviewer_doc_ref}",
                remediation="Create the reviewer-facing landing page or fix the ref.",
                row_id=row_id,
                ref=reviewer_doc_ref,
            )
        )

    if not repo_ref_exists(repo_root, proof_packet_ref):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_PROOF_PACKET_MISSING,
                message=f"row {row_id} proof_packet_ref does not exist: {proof_packet_ref}",
                remediation="Create the owning proof packet or fix the ref.",
                row_id=row_id,
                ref=proof_packet_ref,
            )
        )

    fixture_payload: Any | None = None
    if not repo_ref_exists(repo_root, fixture_ref):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FIXTURE_MISSING,
                message=f"row {row_id} protected_walk_fixture_ref does not exist: {fixture_ref}",
                remediation="Seed the protected-walk fixture or fix the ref.",
                row_id=row_id,
                ref=fixture_ref,
            )
        )
    else:
        try:
            fixture_payload = json.loads((repo_root / fixture_ref).read_text())
        except (json.JSONDecodeError, OSError) as exc:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIXTURE_PARSE_FAILED,
                    message=(
                        f"row {row_id} protected_walk_fixture_ref could not be parsed as JSON: {exc}"
                    ),
                    remediation="Repair the fixture JSON.",
                    row_id=row_id,
                    ref=fixture_ref,
                )
            )

    expected_tokens = ensure_list(
        row.get("expected_tokens"), f"{label}.expected_tokens"
    )
    for token_idx, raw_token in enumerate(expected_tokens):
        token = ensure_dict(raw_token, f"{label}.expected_tokens[{token_idx}]")
        field_path = ensure_str(
            token.get("field_path"),
            f"{label}.expected_tokens[{token_idx}].field_path",
        )
        expected_value = token.get("expected_value")
        if not isinstance(expected_value, str) or not expected_value.strip():
            raise SystemExit(
                f"{label}.expected_tokens[{token_idx}].expected_value must be a non-empty string"
            )
        observation_entry: dict[str, Any] = {
            "field_path": field_path,
            "expected_value": expected_value,
        }
        if fixture_payload is None:
            observation_entry["observed_value"] = None
            observation_entry["status"] = "fixture_unavailable"
            observation.expected_tokens_observed.append(observation_entry)
            continue
        try:
            observed = resolve_field_path(fixture_payload, field_path)
        except KeyError as exc:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIELD_PATH_MISSING,
                    message=(
                        f"row {row_id} expected_tokens field_path {field_path!r} "
                        f"is not present on fixture {fixture_ref}: {exc}"
                    ),
                    remediation=(
                        "Update the checklist field_path to match the fixture, or update the fixture."
                    ),
                    row_id=row_id,
                    ref=fixture_ref,
                    details={"field_path": field_path},
                )
            )
            observation_entry["observed_value"] = None
            observation_entry["status"] = "field_path_missing"
            observation.expected_tokens_observed.append(observation_entry)
            continue
        observation_entry["observed_value"] = observed
        if observed != expected_value:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_TOKEN_MISMATCH,
                    message=(
                        f"row {row_id} expected_tokens field_path {field_path!r} "
                        f"observed {observed!r} but checklist claims {expected_value!r}"
                    ),
                    remediation=(
                        "Re-align the checklist row with the upstream wedge's protected-walk fixture, "
                        "or update the fixture so it carries the expected token."
                    ),
                    row_id=row_id,
                    ref=fixture_ref,
                    details={
                        "field_path": field_path,
                        "expected": expected_value,
                        "observed": observed,
                    },
                )
            )
            observation_entry["status"] = "mismatch"
        else:
            observation_entry["status"] = "match"
        observation.expected_tokens_observed.append(observation_entry)

    failure_drill = row.get("failure_drill")
    if not isinstance(failure_drill, dict):
        findings.append(
            Finding(
                severity="error",
                check_id=CHECK_ID_FAILURE_DRILL_MISSING,
                message=(
                    f"row {row_id} must declare a failure_drill with drill_id + "
                    "forced_value_path + forced_value + expected_check_id so the lane fails loudly"
                ),
                remediation="Add a named failure drill block to the row.",
                row_id=row_id,
                ref=row_id,
            )
        )
    else:
        drill_id = ensure_str(
            failure_drill.get("drill_id"), f"{label}.failure_drill.drill_id"
        )
        _ = ensure_str(
            failure_drill.get("forced_value_path"),
            f"{label}.failure_drill.forced_value_path",
        )
        _ = failure_drill.get("forced_value")
        _ = ensure_str(
            failure_drill.get("expected_check_id"),
            f"{label}.failure_drill.expected_check_id",
        )
        observation.failure_drill_id = drill_id

    observations.append(observation)


def replay_failure_drill(
    repo_root: Path,
    checklist: dict[str, Any],
    drill_id: str,
    findings: list[Finding],
) -> str | None:
    rows = ensure_list(
        checklist.get("acceptance_rows"), "checklist.acceptance_rows"
    )
    for raw_row in rows:
        row = ensure_dict(raw_row, "checklist.acceptance_rows[]")
        failure_drill = row.get("failure_drill")
        if not isinstance(failure_drill, dict):
            continue
        if failure_drill.get("drill_id") != drill_id:
            continue
        row_id = ensure_str(row.get("row_id"), "row.row_id")
        forced_path = ensure_str(
            failure_drill.get("forced_value_path"),
            "failure_drill.forced_value_path",
        )
        forced_value = failure_drill.get("forced_value")
        expected_check_id = ensure_str(
            failure_drill.get("expected_check_id"),
            "failure_drill.expected_check_id",
        )
        fixture_ref = ensure_str(
            row.get("protected_walk_fixture_ref"),
            "row.protected_walk_fixture_ref",
        )
        if not repo_ref_exists(repo_root, fixture_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIXTURE_MISSING,
                    message=(
                        f"drill {drill_id} cannot replay: fixture {fixture_ref} missing"
                    ),
                    remediation="Seed the protected-walk fixture or fix the ref.",
                    row_id=row_id,
                    ref=fixture_ref,
                )
            )
            return expected_check_id
        try:
            fixture_payload = json.loads((repo_root / fixture_ref).read_text())
        except (json.JSONDecodeError, OSError) as exc:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIXTURE_PARSE_FAILED,
                    message=(
                        f"drill {drill_id} cannot replay: fixture {fixture_ref} failed to parse: {exc}"
                    ),
                    remediation="Repair the fixture JSON.",
                    row_id=row_id,
                    ref=fixture_ref,
                )
            )
            return expected_check_id
        # Locate the matching expected_tokens row (so we know which
        # field_path the drill mutates).
        expected_tokens = ensure_list(
            row.get("expected_tokens"), "row.expected_tokens"
        )
        matching_token = None
        for raw_token in expected_tokens:
            token = ensure_dict(raw_token, "row.expected_tokens[]")
            if token.get("field_path") == forced_path:
                matching_token = token
                break
        if matching_token is None:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING,
                    message=(
                        f"drill {drill_id} forced_value_path {forced_path!r} does not "
                        "match any expected_tokens row on this acceptance row; the drill "
                        "cannot drift a token that the row never claimed"
                    ),
                    remediation=(
                        "Set forced_value_path to one of the expected_tokens field_paths on the row."
                    ),
                    row_id=row_id,
                    ref=forced_path,
                )
            )
            return expected_check_id
        # Drive the smoke comparison with the forced value.
        try:
            observed = resolve_field_path(fixture_payload, forced_path)
        except KeyError as exc:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FIELD_PATH_MISSING,
                    message=(
                        f"drill {drill_id} field_path {forced_path!r} not present on fixture: {exc}"
                    ),
                    remediation=(
                        "Update the drill forced_value_path to match the fixture, or update the fixture."
                    ),
                    row_id=row_id,
                    ref=forced_path,
                )
            )
            return expected_check_id
        # The "drift" the smoke records: the row's expected_value is
        # replaced by forced_value; the fixture is unchanged.
        if observed != forced_value:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_TOKEN_MISMATCH,
                    message=(
                        f"drill {drill_id} reproduces vocabulary drift on {forced_path!r}: "
                        f"checklist forced_value {forced_value!r} disagrees with fixture {observed!r}"
                    ),
                    remediation=(
                        "Re-align the checklist row with the upstream wedge's protected-walk fixture, "
                        "or update the fixture so it carries the expected token."
                    ),
                    row_id=row_id,
                    ref=forced_path,
                    details={
                        "field_path": forced_path,
                        "forced_value": forced_value,
                        "observed_value": observed,
                    },
                )
            )
        else:
            findings.append(
                Finding(
                    severity="error",
                    check_id=CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING,
                    message=(
                        f"drill {drill_id} forced_value {forced_value!r} happens to match the "
                        "fixture value; the drill cannot prove drift detection"
                    ),
                    remediation=(
                        "Pick a forced_value that contradicts the upstream protected-walk fixture."
                    ),
                    row_id=row_id,
                    ref=forced_path,
                )
            )
        return expected_check_id
    findings.append(
        Finding(
            severity="error",
            check_id=CHECK_ID_FAILURE_DRILL_EXPECTED_FINDING_MISSING,
            message=f"drill {drill_id} not declared on any acceptance row",
            remediation="Pick a drill_id that exists in the checklist.",
            ref=drill_id,
        )
    )
    return None


def write_report(
    repo_root: Path,
    report_rel: str,
    checklist_rel: str,
    build_identity_rel: str,
    findings: list[Finding],
    observations: list[RowObservation],
    drill_id: str | None,
    expected_check_id: str | None,
) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload: dict[str, Any] = {
        "schema_version": 1,
        "check_id": "m1_operator_truth_demo_smoke",
        "generated_at": dt.datetime.now(dt.timezone.utc)
        .replace(microsecond=0)
        .isoformat()
        .replace("+00:00", "Z"),
        "checklist_ref": checklist_rel,
        "build_identity_ref": build_identity_rel,
        "finding_counts": {
            "error": sum(1 for f in findings if f.severity == "error"),
            "warning": sum(1 for f in findings if f.severity == "warning"),
        },
        "findings": [f.as_report() for f in findings],
        "row_observations": [
            {
                "row_id": obs.row_id,
                "wedge_id": obs.wedge_id,
                "reviewer_doc_ref": obs.reviewer_doc_ref,
                "proof_packet_ref": obs.proof_packet_ref,
                "validation_command": obs.validation_command,
                "protected_walk_fixture_ref": obs.protected_walk_fixture_ref,
                "expected_tokens_observed": obs.expected_tokens_observed,
                "failure_drill_id": obs.failure_drill_id,
            }
            for obs in observations
        ],
    }
    if drill_id is not None:
        payload["forced_drill"] = {
            "drill_id": drill_id,
            "expected_check_id": expected_check_id,
            "observed_check_ids": sorted(
                {f.check_id for f in findings if f.check_id}
            ),
        }
    report_path.write_text(
        json.dumps(payload, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(
            f"--repo-root does not look like a repository root: {repo_root}"
        )

    checklist_rel = str(args.checklist)
    checklist_payload = ensure_dict(
        render_yaml_as_json(repo_root / checklist_rel), checklist_rel
    )

    findings: list[Finding] = []
    observations: list[RowObservation] = []

    validate_claim_limits(checklist_payload, findings)

    rows = ensure_list(
        checklist_payload.get("acceptance_rows"),
        "checklist.acceptance_rows",
    )
    seen_row_ids: set[str] = set()

    drill_expected_check_id: str | None = None
    if args.force_drill is not None:
        drill_expected_check_id = replay_failure_drill(
            repo_root, checklist_payload, args.force_drill, findings
        )
    else:
        for idx, raw_row in enumerate(rows):
            row = ensure_dict(raw_row, f"checklist.acceptance_rows[{idx}]")
            validate_row(
                repo_root,
                row,
                idx,
                findings,
                seen_row_ids,
                observations,
            )

    if args.report:
        write_report(
            repo_root,
            args.report,
            checklist_rel,
            args.build_identity,
            findings,
            observations,
            args.force_drill,
            drill_expected_check_id,
        )

    if args.force_drill is not None:
        observed_ids = {f.check_id for f in findings if f.check_id}
        if drill_expected_check_id is None:
            return 1
        if drill_expected_check_id in observed_ids:
            print(
                f"drill {args.force_drill}: reproduced expected check_id "
                f"{drill_expected_check_id!r}"
            )
            return 0
        print(
            f"drill {args.force_drill}: expected {drill_expected_check_id!r}, observed {sorted(observed_ids)!r}",
            file=sys.stderr,
        )
        return 1

    errors = [f for f in findings if f.severity == "error"]
    if errors:
        print(
            f"operator-truth demo smoke failed with {len(errors)} error(s); see {args.report}",
            file=sys.stderr,
        )
        return 1
    print(
        f"operator-truth demo smoke passed: {len(observations)} row(s) verified; capture written to {args.report}"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
