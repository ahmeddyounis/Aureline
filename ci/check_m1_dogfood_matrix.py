#!/usr/bin/env python3
"""Validate the protected small-project fixture-repo set and dogfood matrix.

This check keeps the daily small-project dogfood lane executable by ensuring
the canonical dogfood-matrix artifact:

- exists and parses;
- carries a stable schema/owner header;
- defines a protected row for each required fixture category; and
- declares the required action kinds (open/edit/save/restore/etc) with concrete
  commands and expected outcomes.

It also validates the proof-index consumer that downstream review packets and
dashboards should read instead of cloning the fixture list into ad hoc notes.
"""

from __future__ import annotations

import argparse
import datetime as dt
import json
import re
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

DEFAULT_MATRIX_REL = "artifacts/milestones/m1/dogfood_matrix.yaml"
DEFAULT_INDEX_REL = "artifacts/milestones/m1/dogfood_matrix_index.yaml"

SENTINEL_REFS = {
    "not_yet_seeded",
    "outline_only",
    "contract_not_yet_seeded",
    "planned_not_yet_seeded",
}

REQUIRED_ACTION_KINDS = [
    "open",
    "quick_open",
    "edit_save",
    "terminal",
    "restore_session",
    "missing_target_recovery",
]

REQUIRED_FIXTURE_CATEGORIES = [
    "plain_text",
    "nested_source_tree",
    "path_and_encoding",
    "missing_target_restore",
]

BUILD_IDENTITY_REQUIRED_KEYS = {
    "schema_version",
    "commit",
    "commit_short",
    "dirty",
    "toolchain_channel",
    "rustc_version",
    "cargo_version",
    "host_triple",
    "target_triple",
    "profile",
    "workspace_version",
    "source_date_epoch",
    "build_timestamp_utc",
}

COMMIT_FULL_RE = re.compile(r"^(?:[0-9a-f]{40}|unknown)$")


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
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
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


def ensure_bool(value: Any, label: str) -> bool:
    if not isinstance(value, bool):
        raise SystemExit(f"{label} must be a boolean")
    return value


def parse_iso_date(value: str, label: str) -> dt.date:
    try:
        return dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    ref = ref.strip()
    if ref in SENTINEL_REFS:
        return False
    path = strip_fragment(ref)
    if not path:
        return False
    return (repo_root / path).exists()


def validate_required_refs(repo_root: Path, refs: list[Any], check_id: str, label: str) -> list[Finding]:
    findings: list[Finding] = []
    for idx, ref in enumerate(refs):
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative path (optionally with a #fragment).",
                )
            )
            continue
        ref = ref.strip()
        if ref in SENTINEL_REFS:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.sentinel_in_required",
                    message=f"{label}[{idx}] is a sentinel ref but is marked required: {ref}",
                    remediation="Replace the sentinel with a real artifact path, or move this ref under an optional list.",
                    ref=ref,
                )
            )
            continue
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.missing_ref",
                    message=f"{label}[{idx}] does not exist: {ref}",
                    remediation="Fix the path (or seed the referenced artifact) so the dogfood matrix does not rely on missing prerequisites.",
                    ref=ref,
                )
            )
    return findings


def validate_optional_refs(repo_root: Path, refs: list[Any], check_id: str, label: str) -> list[Finding]:
    findings: list[Finding] = []
    for idx, ref in enumerate(refs):
        if not isinstance(ref, str) or not ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{check_id}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative path or a sentinel value.",
                )
            )
            continue
        ref = ref.strip()
        if ref in SENTINEL_REFS:
            continue
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="warning",
                    check_id=f"{check_id}.missing_optional_ref",
                    message=f"{label}[{idx}] does not exist (optional): {ref}",
                    remediation="Either seed the referenced artifact or change it to an explicit sentinel until it is ready.",
                    ref=ref,
                )
            )
    return findings


def validate_build_identity_record(repo_root: Path, ref: str, check_id: str) -> list[Finding]:
    findings: list[Finding] = []
    rel = strip_fragment(ref)
    path = repo_root / rel
    try:
        payload = json.loads(path.read_text(encoding="utf-8"))
    except Exception as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.parse_failed",
                message=f"failed to parse build identity JSON at {rel}: {exc}",
                remediation="Regenerate the build identity record (or fix the referenced path) so consumers can read it mechanically.",
                ref=rel,
            )
        )
        return findings

    if not isinstance(payload, dict):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.not_object",
                message=f"build identity payload must be a JSON object: {rel}",
                remediation="Replace the payload with a JSON object matching schemas/build/build_identity.schema.json.",
                ref=rel,
            )
        )
        return findings

    keys = set(payload.keys())
    missing = sorted(BUILD_IDENTITY_REQUIRED_KEYS - keys)
    extra = sorted(keys - BUILD_IDENTITY_REQUIRED_KEYS)
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.missing_fields",
                message=f"build identity payload is missing required fields: {', '.join(missing)}",
                remediation="Regenerate the build identity record so it matches schemas/build/build_identity.schema.json.",
                ref=rel,
            )
        )
    if extra:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.extra_fields",
                message=f"build identity payload has unknown fields: {', '.join(extra)}",
                remediation="Remove unknown fields (or bump the schema and update validators) so the artifact stays stable.",
                ref=rel,
            )
        )

    schema_version = payload.get("schema_version")
    if not isinstance(schema_version, int) or schema_version < 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.schema_version",
                message=f"build identity schema_version must be an integer >= 1, got {schema_version!r}",
                remediation="Regenerate the artifact with tools/build/print_build_identity.sh or update the schema and validators together.",
                ref=rel,
            )
        )

    commit = payload.get("commit")
    if not isinstance(commit, str) or not COMMIT_FULL_RE.match(commit):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.commit",
                message=f"build identity commit must be a 40-hex hash or 'unknown', got {commit!r}",
                remediation="Regenerate the artifact so commit is a full hash (or 'unknown' outside git).",
                ref=rel,
            )
        )

    dirty = payload.get("dirty")
    if not isinstance(dirty, bool):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.dirty",
                message=f"build identity dirty must be a boolean, got {dirty!r}",
                remediation="Regenerate the artifact so tree state is explicit.",
                ref=rel,
            )
        )

    profile = payload.get("profile")
    if profile not in {"dev", "release"}:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.profile",
                message=f"build identity profile must be 'dev' or 'release', got {profile!r}",
                remediation="Regenerate the artifact so profile is normalized to the schema vocabulary.",
                ref=rel,
            )
        )

    source_date_epoch = payload.get("source_date_epoch")
    if not isinstance(source_date_epoch, int) or source_date_epoch < 0:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.source_date_epoch",
                message=f"build identity source_date_epoch must be an integer >= 0, got {source_date_epoch!r}",
                remediation="Regenerate the artifact so SOURCE_DATE_EPOCH is recorded deterministically.",
                ref=rel,
            )
        )

    build_timestamp_utc = payload.get("build_timestamp_utc")
    if not isinstance(build_timestamp_utc, str) or not build_timestamp_utc.strip():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{check_id}.build_timestamp_utc",
                message="build identity build_timestamp_utc must be a non-empty string",
                remediation="Regenerate the artifact so a deterministic build timestamp is recorded.",
                ref=rel,
            )
        )

    return findings


def validate_matrix(repo_root: Path, payload: dict[str, Any], matrix_rel: str) -> list[Finding]:
    findings: list[Finding] = []

    schema_version = ensure_int(payload.get("schema_version"), "matrix.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.schema_version.unsupported",
                message=f"unsupported schema_version {schema_version} (expected 1)",
                remediation="Bump the validator and document the new schema before changing schema_version.",
            )
        )

    as_of = ensure_str(payload.get("as_of"), "matrix.as_of")
    parse_iso_date(as_of, "matrix.as_of")

    owner = ensure_str(payload.get("owner"), "matrix.owner")
    if not owner.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id="matrix.owner.format",
                message=f"owner does not look like a handle: {owner!r}",
                remediation="Use an @handle so review routing is explicit.",
            )
        )

    matrix = ensure_dict(payload.get("matrix"), "matrix.matrix")
    ensure_str(matrix.get("matrix_id"), "matrix.matrix.matrix_id")
    ensure_int(matrix.get("matrix_revision"), "matrix.matrix.matrix_revision")
    ensure_str(matrix.get("status"), "matrix.matrix.status")

    overview_page = ensure_str(matrix.get("overview_page"), "matrix.matrix.overview_page")
    issue_taxonomy_ref = ensure_str(matrix.get("issue_taxonomy_ref"), "matrix.matrix.issue_taxonomy_ref")
    fixture_root_ref = ensure_str(matrix.get("fixture_root_ref"), "matrix.matrix.fixture_root_ref")

    for required_ref, label in [
        (overview_page, "matrix.matrix.overview_page"),
        (issue_taxonomy_ref, "matrix.matrix.issue_taxonomy_ref"),
        (fixture_root_ref, "matrix.matrix.fixture_root_ref"),
    ]:
        if not artifact_ref_exists(repo_root, required_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.matrix.missing_ref",
                    message=f"{label} does not exist: {required_ref}",
                    remediation="Fix the path or seed the referenced artifact so the dogfood lane has a stable entry point.",
                    ref=required_ref,
                )
            )

    required_action_kinds = ensure_list(matrix.get("required_action_kinds"), "matrix.matrix.required_action_kinds")
    action_kind_values: list[str] = []
    for idx, value in enumerate(required_action_kinds):
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.required_action_kinds.invalid",
                    message=f"required_action_kinds[{idx}] must be a non-empty string",
                    remediation="List action kinds as non-empty strings.",
                )
            )
            continue
        action_kind_values.append(value.strip())
    missing_actions = [kind for kind in REQUIRED_ACTION_KINDS if kind not in action_kind_values]
    if missing_actions:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.required_action_kinds.missing",
                message=f"required_action_kinds is missing: {', '.join(missing_actions)}",
                remediation="Restore the missing action kinds so each protected row has a complete dogfood recipe.",
                details={"required_action_kinds": REQUIRED_ACTION_KINDS, "present": action_kind_values},
            )
        )

    required_categories = ensure_list(matrix.get("required_fixture_categories"), "matrix.matrix.required_fixture_categories")
    category_values: list[str] = []
    for idx, value in enumerate(required_categories):
        if not isinstance(value, str) or not value.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.required_fixture_categories.invalid",
                    message=f"required_fixture_categories[{idx}] must be a non-empty string",
                    remediation="List fixture categories as non-empty strings.",
                )
            )
            continue
        category_values.append(value.strip())
    missing_categories = [cat for cat in REQUIRED_FIXTURE_CATEGORIES if cat not in category_values]
    if missing_categories:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.required_fixture_categories.missing",
                message=f"required_fixture_categories is missing: {', '.join(missing_categories)}",
                remediation="Restore the missing category labels so the matrix covers each required fixture class.",
                details={"required_fixture_categories": REQUIRED_FIXTURE_CATEGORIES, "present": category_values},
            )
        )

    rows = ensure_list(payload.get("rows"), "matrix.rows")
    row_ids: set[str] = set()
    covered_categories: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"matrix.rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"matrix.rows[{idx}].row_id")
        if row_id in row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.duplicate_id",
                    message=f"duplicate row_id: {row_id}",
                    remediation="Give each dogfood row a unique row_id so consumers can join reliably.",
                    ref=row_id,
                )
            )
        row_ids.add(row_id)

        ensure_str(row.get("title"), f"matrix.rows[{idx}].title")
        ensure_bool(row.get("protected"), f"matrix.rows[{idx}].protected")
        owner_dri = ensure_str(row.get("owner_dri"), f"matrix.rows[{idx}].owner_dri")
        if not owner_dri.startswith("@"):
            findings.append(
                Finding(
                    severity="warning",
                    check_id="matrix.rows.owner_dri.format",
                    message=f"row owner_dri does not look like a handle: {owner_dri!r}",
                    remediation="Use an @handle so missing-fixture failures route quickly to an owner.",
                    ref=row_id,
                )
            )

        categories = ensure_list(row.get("categories"), f"matrix.rows[{idx}].categories")
        for category in categories:
            if isinstance(category, str) and category.strip():
                covered_categories.add(category.strip())

        repo_root_ref = ensure_str(row.get("repo_root_ref"), f"matrix.rows[{idx}].repo_root_ref")
        entry_file_ref = ensure_str(row.get("entry_file_ref"), f"matrix.rows[{idx}].entry_file_ref")
        expected_smoke_suites = ensure_list(row.get("expected_smoke_suites"), f"matrix.rows[{idx}].expected_smoke_suites")
        if not expected_smoke_suites:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.expected_smoke_suites.empty",
                    message="expected_smoke_suites must include at least one smoke-suite reference",
                    remediation="Add at least one smoke-suite reference (for example a QE lane ref) so reviewers know what enforces this row.",
                    ref=row_id,
                )
            )
        expected_proof_packets = ensure_list(
            row.get("expected_proof_packet_refs"), f"matrix.rows[{idx}].expected_proof_packet_refs"
        )
        if not expected_proof_packets:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.expected_proof_packet_refs.empty",
                    message="expected_proof_packet_refs must include at least one proof-packet reference",
                    remediation="Add at least one proof-packet reference so downstream review can join against a concrete proof artifact.",
                    ref=row_id,
                )
            )
        findings.extend(
            validate_required_refs(
                repo_root,
                [repo_root_ref, entry_file_ref, *expected_smoke_suites, *expected_proof_packets],
                check_id="matrix.rows.refs",
                label=f"matrix.rows[{idx}]",
            )
        )

        repo_dir = repo_root / strip_fragment(repo_root_ref)
        if repo_dir.exists() and not repo_dir.is_dir():
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.rows.repo_root_ref.not_dir",
                    message=f"repo_root_ref is not a directory: {repo_root_ref}",
                    remediation="Point repo_root_ref at a directory containing the fixture source tree.",
                    ref=repo_root_ref,
                )
            )

        actions = ensure_dict(row.get("actions"), f"matrix.rows[{idx}].actions")
        for action_kind in REQUIRED_ACTION_KINDS:
            action = actions.get(action_kind)
            if action is None:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.rows.actions.missing",
                        message=f"row is missing required action kind: {action_kind}",
                        remediation="Add the missing action-kind block so every protected row has a complete dogfood recipe.",
                        ref=row_id,
                        details={"required_action_kinds": REQUIRED_ACTION_KINDS},
                    )
                )
                continue
            action_obj = ensure_dict(action, f"matrix.rows[{idx}].actions.{action_kind}")
            ensure_str(action_obj.get("command"), f"matrix.rows[{idx}].actions.{action_kind}.command")
            ensure_str(action_obj.get("expected_outcome"), f"matrix.rows[{idx}].actions.{action_kind}.expected_outcome")

    missing_category_coverage = [cat for cat in REQUIRED_FIXTURE_CATEGORIES if cat not in covered_categories]
    if missing_category_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.rows.category_coverage.missing",
                message=f"rows do not cover required fixture categories: {', '.join(missing_category_coverage)}",
                remediation="Add at least one protected row for each missing category so daily dogfooding covers the required fixture classes.",
                details={"required_fixture_categories": REQUIRED_FIXTURE_CATEGORIES, "covered": sorted(covered_categories)},
            )
        )

    if not (repo_root / matrix_rel).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.file.missing",
                message=f"matrix file does not exist on disk: {matrix_rel}",
                remediation="Restore the dogfood matrix file.",
                ref=matrix_rel,
            )
        )

    return findings


def validate_index(repo_root: Path, payload: dict[str, Any], matrix_rel: str, index_rel: str) -> list[Finding]:
    findings: list[Finding] = []

    schema_version = ensure_int(payload.get("schema_version"), "index.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="index.schema_version.unsupported",
                message=f"unsupported schema_version {schema_version} (expected 1)",
                remediation="Bump the validator and document the new schema before changing schema_version.",
            )
        )

    as_of = ensure_str(payload.get("as_of"), "index.as_of")
    parse_iso_date(as_of, "index.as_of")

    owner = ensure_str(payload.get("owner"), "index.owner")
    if not owner.startswith("@"):
        findings.append(
            Finding(
                severity="warning",
                check_id="index.owner.format",
                message=f"owner does not look like a handle: {owner!r}",
                remediation="Use an @handle so review routing is explicit.",
            )
        )

    human_entry = ensure_str(payload.get("human_entrypoint_ref"), "index.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="index.human_entrypoint.missing",
                message=f"human_entrypoint_ref does not exist: {human_entry}",
                remediation="Fix the path so reviewers have a stable landing page.",
                ref=human_entry,
            )
        )

    canonical_artifacts = ensure_list(payload.get("canonical_artifacts"), "index.canonical_artifacts")
    matrix_row: dict[str, Any] | None = None
    validator_row: dict[str, Any] | None = None
    for idx, raw_row in enumerate(canonical_artifacts):
        row = ensure_dict(raw_row, f"index.canonical_artifacts[{idx}]")
        artifact_id = ensure_str(row.get("artifact_id"), f"index.canonical_artifacts[{idx}].artifact_id")
        artifact_ref = ensure_str(row.get("artifact_ref"), f"index.canonical_artifacts[{idx}].artifact_ref")
        ensure_str(row.get("title"), f"index.canonical_artifacts[{idx}].title")
        ensure_str(row.get("owner_dri"), f"index.canonical_artifacts[{idx}].owner_dri")

        if artifact_id == "dogfood_matrix":
            matrix_row = row
            if strip_fragment(artifact_ref) != matrix_rel:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="index.dogfood_matrix.mismatch",
                        message=f"dogfood_matrix artifact_ref must be {matrix_rel}, got {artifact_ref}",
                        remediation="Update dogfood_matrix_index.yaml so consumers always point at the canonical matrix path.",
                        ref=artifact_ref,
                    )
                )
        if artifact_id == "validator":
            validator_row = row

        if artifact_id in {"dogfood_matrix", "validator", "fixture_repos"} and not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="index.canonical_artifact.missing",
                    message=f"canonical_artifacts[{idx}] artifact_ref does not exist: {artifact_ref}",
                    remediation="Fix the path or seed the artifact so the proof index remains a reliable join point.",
                    ref=artifact_ref,
                )
            )

    if matrix_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.dogfood_matrix.missing_row",
                message="canonical_artifacts must include artifact_id: dogfood_matrix",
                remediation="Add a dogfood_matrix row pointing at the canonical matrix artifact.",
            )
        )
    if validator_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="index.validator.missing_row",
                message="canonical_artifacts must include artifact_id: validator",
                remediation="Add a validator row pointing at the matrix validation script.",
            )
        )

    validation_lane = ensure_dict(payload.get("validation_lane"), "index.validation_lane")
    fixture_root = ensure_str(validation_lane.get("protected_fixture_root"), "index.validation_lane.protected_fixture_root")
    lane_refs = ensure_list(validation_lane.get("proof_lane_refs"), "index.validation_lane.proof_lane_refs")
    findings.extend(
        validate_required_refs(
            repo_root,
            [fixture_root, *lane_refs],
            check_id="index.validation_lane",
            label="index.validation_lane",
        )
    )

    exact_build = ensure_dict(payload.get("exact_build_identity"), "index.exact_build_identity")
    contract_ref = ensure_str(exact_build.get("contract_ref"), "index.exact_build_identity.contract_ref")
    baseline_ref = ensure_str(exact_build.get("baseline_ref"), "index.exact_build_identity.baseline_ref")
    findings.extend(
        validate_required_refs(
            repo_root,
            [contract_ref, baseline_ref],
            check_id="index.exact_build_identity",
            label="index.exact_build_identity",
        )
    )
    latest_identity_ref = ensure_str(exact_build.get("latest_identity_ref"), "index.exact_build_identity.latest_identity_ref")
    findings.extend(
        validate_required_refs(
            repo_root,
            [latest_identity_ref],
            check_id="index.exact_build_identity.latest",
            label="index.exact_build_identity.latest_identity_ref",
        )
    )
    if latest_identity_ref not in SENTINEL_REFS and artifact_ref_exists(repo_root, latest_identity_ref):
        findings.extend(
            validate_build_identity_record(
                repo_root,
                latest_identity_ref,
                check_id="index.exact_build_identity.latest_identity_ref",
            )
        )

    latest_capture = ensure_dict(payload.get("latest_validation_capture"), "index.latest_validation_capture")
    ensure_str(latest_capture.get("captured_at"), "index.latest_validation_capture.captured_at")
    ensure_str(latest_capture.get("command"), "index.latest_validation_capture.command")
    capture_ref = ensure_str(latest_capture.get("report_ref"), "index.latest_validation_capture.report_ref")
    findings.extend(
        validate_optional_refs(
            repo_root,
            [capture_ref],
            check_id="index.latest_validation_capture",
            label="index.latest_validation_capture.report_ref",
        )
    )

    if not (repo_root / index_rel).exists():
        findings.append(
            Finding(
                severity="error",
                check_id="index.file.missing",
                message=f"index file does not exist on disk: {index_rel}",
                remediation="Restore the dogfood-matrix index file.",
                ref=index_rel,
            )
        )

    return findings


def write_report(repo_root: Path, report_rel: str, matrix_rel: str, index_rel: str, findings: list[Finding]) -> None:
    report_path = repo_root / report_rel
    report_path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "check_id": "m1_small_project_dogfood_matrix",
        "matrix_ref": matrix_rel,
        "index_ref": index_rel,
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
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

    matrix_rel = str(args.matrix)
    index_rel = str(args.index)

    matrix_path = repo_root / matrix_rel
    index_path = repo_root / index_rel

    matrix_payload = ensure_dict(render_yaml_as_json(matrix_path), matrix_rel)
    index_payload = ensure_dict(render_yaml_as_json(index_path), index_rel)

    findings: list[Finding] = []
    findings.extend(validate_matrix(repo_root, matrix_payload, matrix_rel=matrix_rel))
    findings.extend(validate_index(repo_root, index_payload, matrix_rel=matrix_rel, index_rel=index_rel))

    matrix_meta = ensure_dict(matrix_payload.get("matrix"), "matrix.matrix")
    expected_human_entry = ensure_str(matrix_meta.get("overview_page"), "matrix.matrix.overview_page")
    actual_human_entry = ensure_str(index_payload.get("human_entrypoint_ref"), "index.human_entrypoint_ref")
    if strip_fragment(expected_human_entry) != strip_fragment(actual_human_entry):
        findings.append(
            Finding(
                severity="error",
                check_id="cross.human_entrypoint.mismatch",
                message=f"matrix and proof index disagree on the human entrypoint: {expected_human_entry} vs {actual_human_entry}",
                remediation="Update both files so reviewers have exactly one stable entry page for the dogfood matrix.",
                details={"matrix_overview_page": expected_human_entry, "index_human_entrypoint_ref": actual_human_entry},
            )
        )

    if args.report:
        write_report(repo_root, str(args.report), matrix_rel=matrix_rel, index_rel=index_rel, findings=findings)

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"

    print(f"[m1-dogfood-matrix] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[m1-dogfood-matrix] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[m1-dogfood-matrix]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[m1-dogfood-matrix] interrupted", file=sys.stderr)
        sys.exit(130)
