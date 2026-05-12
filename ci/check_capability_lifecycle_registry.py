#!/usr/bin/env python3
"""Validate and render the external alpha capability-lifecycle registry."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_REGISTRY_REL = "artifacts/governance/capability_lifecycle_registry.yaml"
DEFAULT_MARKER_SCHEMA_REL = "schemas/governance/dependency_marker.schema.json"
DEFAULT_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_FIXTURE_REL = "fixtures/governance/capability_lifecycle_registry_cases/manifest.yaml"

EXPECTED_LIFECYCLE_VOCABULARY = [
    "Labs",
    "Preview",
    "Beta",
    "Stable",
    "Deprecated",
    "DisabledByPolicy",
    "Retired",
]

EXPECTED_SCHEMA_PROJECTION = {
    "Labs": "labs",
    "Preview": "preview",
    "Beta": "beta",
    "Stable": "stable",
    "Deprecated": "deprecated",
    "DisabledByPolicy": "disabled_by_policy",
    "Retired": "retired",
}

REQUIRED_ROW_FIELDS = {
    "row_id",
    "surface_ref",
    "surface_kind",
    "title",
    "declared_lifecycle_state",
    "effective_lifecycle_state",
    "owner",
    "target_persona_or_workflow",
    "default_posture",
    "migration_note",
    "support_promise",
    "review_or_expiry_date",
    "kill_switch_or_policy_disable_ref",
    "source_scope_refs",
    "scoreboard_row_refs",
    "dependency_marker_refs",
    "consumer_surfaces",
}

REQUIRED_MARKER_FIELDS = {
    "marker_id",
    "marker_schema_version",
    "marker_kind",
    "parent_row_ref",
    "artifact_classes",
    "dependency_ref",
    "dependency_lifecycle_state",
    "effect_on_parent",
    "reason_code",
    "disclosure_summary",
    "repair_or_review_ref",
    "review_or_expiry_date",
    "export_visibility",
    "source_refs",
}

REQUIRED_ARTIFACT_CLASSES = {
    "profile_artifact",
    "workspace_manifest",
    "saved_view",
    "export",
    "migration_packet",
    "bundle_claim",
    "archetype_claim",
}

REQUIRED_ACCEPTANCE_STATES = {
    "claimed_alpha_surface_coverage",
    "dependency_marker_disclosure",
    "no_lifecycle_unknown",
    "policy_disable_blocks_claim",
    "consumer_projection_fields",
}

REQUIRED_CONSUMER_FIELDS = {
    "row_id",
    "surface_ref",
    "effective_lifecycle_state",
    "owner",
    "review_or_expiry_date",
    "dependency_markers",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "alpha_wedge:",
    "archetype_certification_seed:",
    "archetype_row:",
    "capability_lifecycle:",
    "deployment.",
    "dependency_marker:",
    "fixture_register:",
    "launch_bundle:",
    "persona:",
    "policy:",
    "profile_preset:",
    "scoreboard_row:",
    "saved_view:",
    "workflow.",
)


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
    parser.add_argument("--registry", default=DEFAULT_REGISTRY_REL)
    parser.add_argument("--marker-schema", default=DEFAULT_MARKER_SCHEMA_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--fixtures", default=DEFAULT_FIXTURE_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-help-projection",
        action="store_true",
        help="Print the Help/About and diagnostics projection after validation.",
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
        raise SystemExit(f"failed to parse YAML at {path}: {stderr}")
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


def parse_iso_date(value: str, label: str, findings: list[Finding], ref: str | None = None) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_date",
                message=f"{label} must be a YYYY-MM-DD date, got {value!r}",
                remediation="Use an ISO-8601 date such as 2026-06-09.",
                ref=ref,
            )
        )


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean or clean.startswith(ID_PREFIXES):
        return False
    return "/" in clean or clean.endswith(PATH_LIKE_SUFFIXES)


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def validate_path_ref(repo_root: Path, ref: str, label: str, findings: list[Finding]) -> None:
    if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the path or seed the referenced artifact.",
                ref=ref,
            )
        )


def validate_header(registry: dict[str, Any], findings: list[Finding]) -> None:
    schema_version = ensure_int(registry.get("schema_version"), "registry.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.schema_version.unsupported",
                message=f"registry.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that changes the registry schema.",
            )
        )
    parse_iso_date(ensure_str(registry.get("as_of"), "registry.as_of"), "registry.as_of", findings)
    ensure_str(registry.get("owner"), "registry.owner")
    ensure_str(registry.get("registry_id"), "registry.registry_id")


def validate_registry_refs(repo_root: Path, registry: dict[str, Any], findings: list[Finding]) -> None:
    for field_name in (
        "human_entrypoint_ref",
        "dependency_marker_schema_ref",
        "alpha_wedge_matrix_ref",
        "alpha_exit_scoreboard_ref",
        "known_limits_ref",
    ):
        ref = ensure_str(registry.get(field_name), f"registry.{field_name}")
        validate_path_ref(repo_root, ref, f"registry.{field_name}", findings)
    for ref in ensure_list(registry.get("source_contract_refs"), "registry.source_contract_refs"):
        validate_path_ref(repo_root, ensure_str(ref, "registry.source_contract_refs[]"), "registry.source_contract_refs", findings)


def validate_vocabulary(registry: dict[str, Any], marker_schema: dict[str, Any], findings: list[Finding]) -> set[str]:
    lifecycle_vocabulary = [ensure_str(item, "registry.lifecycle_vocabulary[]") for item in ensure_list(registry.get("lifecycle_vocabulary"), "registry.lifecycle_vocabulary")]
    if lifecycle_vocabulary != EXPECTED_LIFECYCLE_VOCABULARY:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.vocabulary.lifecycle_mismatch",
                message="registry.lifecycle_vocabulary must match the controlled lifecycle vocabulary exactly",
                remediation="Use Labs, Preview, Beta, Stable, Deprecated, DisabledByPolicy, and Retired in that order.",
                details={"expected": EXPECTED_LIFECYCLE_VOCABULARY, "actual": lifecycle_vocabulary},
            )
        )

    projection = ensure_dict(registry.get("schema_projection"), "registry.schema_projection")
    if projection != EXPECTED_SCHEMA_PROJECTION:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.vocabulary.schema_projection_mismatch",
                message="registry.schema_projection must map the controlled display states to the existing schema states",
                remediation="Keep the registry display vocabulary and schema projection in lockstep.",
                details={"expected": EXPECTED_SCHEMA_PROJECTION, "actual": projection},
            )
        )

    marker_states = (
        marker_schema.get("properties", {})
        .get("dependency_lifecycle_state", {})
        .get("enum", [])
    )
    if marker_states != EXPECTED_LIFECYCLE_VOCABULARY:
        findings.append(
            Finding(
                severity="error",
                check_id="marker_schema.lifecycle_enum_mismatch",
                message="dependency marker schema lifecycle enum must match the registry lifecycle vocabulary",
                remediation="Update schemas/governance/dependency_marker.schema.json in the same change.",
                details={"expected": EXPECTED_LIFECYCLE_VOCABULARY, "actual": marker_states},
            )
        )

    return set(lifecycle_vocabulary)


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def collect_claimed_matrix_refs(matrix: dict[str, Any]) -> tuple[set[str], set[str]]:
    claimed_refs: set[str] = set()
    scoreboard_refs: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(matrix.get("wedge_rows"), "matrix.wedge_rows")):
        row = ensure_dict(raw_row, f"matrix.wedge_rows[{idx}]")
        if ensure_str(row.get("claim_posture"), f"matrix.wedge_rows[{idx}].claim_posture") != "alpha_claim":
            continue
        claimed_refs.add(ensure_str(row.get("wedge_id"), f"matrix.wedge_rows[{idx}].wedge_id"))
        for field_name in ("launch_bundle_refs", "archetype_row_refs", "required_scoreboard_rows"):
            for raw_ref in ensure_list(row.get(field_name), f"matrix.wedge_rows[{idx}].{field_name}"):
                ref = ensure_str(raw_ref, f"matrix.wedge_rows[{idx}].{field_name}[]")
                if ref.startswith("scoreboard_row:"):
                    scoreboard_refs.add(ref)
                else:
                    claimed_refs.add(ref)
        for workflow_idx, raw_workflow in enumerate(ensure_list(row.get("protected_workflows"), f"matrix.wedge_rows[{idx}].protected_workflows")):
            workflow = ensure_dict(raw_workflow, f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}]")
            claimed_refs.add(ensure_str(workflow.get("workflow_id"), f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].workflow_id"))
            scoreboard_refs.add(ensure_str(workflow.get("scoreboard_row_ref"), f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].scoreboard_row_ref"))
        for deployment_idx, raw_deployment in enumerate(ensure_list(row.get("claimed_deployment_rows"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows")):
            deployment = ensure_dict(raw_deployment, f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}]")
            claimed_refs.add(ensure_str(deployment.get("deployment_claim_id"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].deployment_claim_id"))
            scoreboard_refs.add(ensure_str(deployment.get("scoreboard_row_ref"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].scoreboard_row_ref"))
    return claimed_refs, scoreboard_refs


def collect_rows(registry: dict[str, Any], lifecycle_vocabulary: set[str], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    default_postures = set(ensure_list(registry.get("default_posture_vocabulary"), "registry.default_posture_vocabulary"))
    support_promises = set(ensure_list(registry.get("support_promise_vocabulary"), "registry.support_promise_vocabulary"))
    for idx, raw_row in enumerate(ensure_list(registry.get("surface_rows"), "registry.surface_rows")):
        row = ensure_dict(raw_row, f"registry.surface_rows[{idx}]")
        missing_fields = sorted(REQUIRED_ROW_FIELDS - set(row))
        row_id = str(row.get("row_id", f"registry.surface_rows[{idx}]"))
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.rows.missing_fields",
                    message=f"surface row is missing required fields: {row_id}",
                    remediation="Add owner, target workflow, default posture, migration note, support promise, review date, policy ref, and marker refs.",
                    ref=row_id,
                    details={"missing": missing_fields},
                )
            )
            continue
        row_id = ensure_str(row.get("row_id"), f"registry.surface_rows[{idx}].row_id")
        if row_id in rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.rows.duplicate",
                    message=f"duplicate lifecycle row id: {row_id}",
                    remediation="Use one stable row id per lifecycle row.",
                    ref=row_id,
                )
            )
        rows[row_id] = row

        for state_field in ("declared_lifecycle_state", "effective_lifecycle_state"):
            state = ensure_str(row.get(state_field), f"registry.surface_rows[{idx}].{state_field}")
            if state not in lifecycle_vocabulary:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.lifecycle_state.invalid",
                        message=f"{row_id} uses an unknown lifecycle state: {state}",
                        remediation="Use the controlled lifecycle vocabulary.",
                        ref=row_id,
                    )
                )
        parse_iso_date(ensure_str(row.get("review_or_expiry_date"), f"registry.surface_rows[{idx}].review_or_expiry_date"), "registry.rows.review_or_expiry_date", findings, row_id)
        if row.get("default_posture") not in default_postures:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.rows.default_posture.invalid",
                    message=f"{row_id} uses an unknown default posture: {row.get('default_posture')}",
                    remediation="Use a default posture from registry.default_posture_vocabulary.",
                    ref=row_id,
                )
            )
        if row.get("support_promise") not in support_promises:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.rows.support_promise.invalid",
                    message=f"{row_id} uses an unknown support promise: {row.get('support_promise')}",
                    remediation="Use a support promise from registry.support_promise_vocabulary.",
                    ref=row_id,
                )
            )
    return rows


def collect_markers(registry: dict[str, Any], lifecycle_vocabulary: set[str], rows: dict[str, dict[str, Any]], findings: list[Finding]) -> dict[str, dict[str, Any]]:
    markers: dict[str, dict[str, Any]] = {}
    artifact_classes_allowed = set(ensure_list(registry.get("marker_artifact_class_vocabulary"), "registry.marker_artifact_class_vocabulary"))
    for idx, raw_marker in enumerate(ensure_list(registry.get("dependency_markers"), "registry.dependency_markers")):
        marker = ensure_dict(raw_marker, f"registry.dependency_markers[{idx}]")
        missing_fields = sorted(REQUIRED_MARKER_FIELDS - set(marker))
        marker_id = str(marker.get("marker_id", f"registry.dependency_markers[{idx}]"))
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.missing_fields",
                    message=f"dependency marker is missing required fields: {marker_id}",
                    remediation="Add all fields required by schemas/governance/dependency_marker.schema.json.",
                    ref=marker_id,
                    details={"missing": missing_fields},
                )
            )
            continue
        marker_id = ensure_str(marker.get("marker_id"), f"registry.dependency_markers[{idx}].marker_id")
        if marker_id in markers:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.duplicate",
                    message=f"duplicate dependency marker id: {marker_id}",
                    remediation="Use one stable id per marker.",
                    ref=marker_id,
                )
            )
        markers[marker_id] = marker
        if ensure_int(marker.get("marker_schema_version"), f"registry.dependency_markers[{idx}].marker_schema_version") != 1:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.schema_version.unsupported",
                    message=f"{marker_id} must use marker_schema_version 1",
                    remediation="Update this validator in the same change that changes marker schema version.",
                    ref=marker_id,
                )
            )
        parent_ref = ensure_str(marker.get("parent_row_ref"), f"registry.dependency_markers[{idx}].parent_row_ref")
        if parent_ref not in rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.parent_row_unknown",
                    message=f"{marker_id} cites unknown parent row: {parent_ref}",
                    remediation="Add the parent lifecycle row or correct parent_row_ref.",
                    ref=marker_id,
                )
            )
        dependency_state = ensure_str(marker.get("dependency_lifecycle_state"), f"registry.dependency_markers[{idx}].dependency_lifecycle_state")
        if dependency_state not in lifecycle_vocabulary:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.dependency_lifecycle_state.invalid",
                    message=f"{marker_id} uses an unknown dependency lifecycle state: {dependency_state}",
                    remediation="Use the controlled lifecycle vocabulary.",
                    ref=marker_id,
                )
            )
        marker_artifact_classes = {
            ensure_str(item, f"registry.dependency_markers[{idx}].artifact_classes[]")
            for item in ensure_list(marker.get("artifact_classes"), f"registry.dependency_markers[{idx}].artifact_classes")
        }
        unknown_classes = sorted(marker_artifact_classes - artifact_classes_allowed)
        if unknown_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.markers.artifact_class.invalid",
                    message=f"{marker_id} uses unknown artifact classes",
                    remediation="Use classes from registry.marker_artifact_class_vocabulary.",
                    ref=marker_id,
                    details={"unknown": unknown_classes},
                )
            )
        parse_iso_date(ensure_str(marker.get("review_or_expiry_date"), f"registry.dependency_markers[{idx}].review_or_expiry_date"), "registry.markers.review_or_expiry_date", findings, marker_id)
    return markers


def validate_row_marker_links(rows: dict[str, dict[str, Any]], markers: dict[str, dict[str, Any]], findings: list[Finding]) -> None:
    rendered_marker_ids: set[str] = set()
    for row_id, row in rows.items():
        row_marker_refs = [
            ensure_str(item, f"registry.surface_rows[{row_id}].dependency_marker_refs[]")
            for item in ensure_list(row.get("dependency_marker_refs"), f"registry.surface_rows[{row_id}].dependency_marker_refs")
        ]
        for marker_ref in row_marker_refs:
            if marker_ref not in markers:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.dependency_marker_refs.unknown",
                        message=f"{row_id} cites unknown dependency marker: {marker_ref}",
                        remediation="Add the marker record or correct the row dependency_marker_refs.",
                        ref=row_id,
                    )
                )
                continue
            marker = markers[marker_ref]
            if marker.get("parent_row_ref") != row_id:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.dependency_marker_refs.parent_mismatch",
                        message=f"{row_id} cites marker owned by {marker.get('parent_row_ref')}: {marker_ref}",
                        remediation="Attach markers only to their parent row or mint a row-specific marker.",
                        ref=row_id,
                    )
                )
            rendered_marker_ids.add(marker_ref)

        effective_state = row.get("effective_lifecycle_state")
        declared_state = row.get("declared_lifecycle_state")
        if declared_state == "Stable" and row_marker_refs and effective_state == "Stable":
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.rows.stable_with_marker_not_narrowed",
                    message=f"{row_id} declares Stable and carries markers but still renders Stable effectively",
                    remediation="Narrow effective_lifecycle_state or clear the markers after evidence is stable.",
                    ref=row_id,
                )
            )
        if effective_state == "DisabledByPolicy":
            if not str(row.get("kill_switch_or_policy_disable_ref", "")).startswith("policy:"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.disabled_by_policy_missing_policy_ref",
                        message=f"{row_id} is DisabledByPolicy but does not carry a policy disable ref",
                        remediation="Add a policy: reference naming the disable or kill-switch path.",
                        ref=row_id,
                    )
                )
            if not any(markers.get(marker_ref, {}).get("marker_kind") == "DisabledByPolicyDependency" for marker_ref in row_marker_refs):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.disabled_by_policy_missing_marker",
                        message=f"{row_id} is DisabledByPolicy but has no policy-disable marker",
                        remediation="Attach a DisabledByPolicyDependency marker with a repair or scope-review route.",
                        ref=row_id,
                    )
                )

    unrendered = sorted(set(markers) - rendered_marker_ids)
    if unrendered:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.markers.unrendered",
                message="one or more dependency markers are not referenced by any surface row",
                remediation="Attach every marker to the row that must render it.",
                details={"unrendered": unrendered},
            )
        )


def validate_coverage(
    rows: dict[str, dict[str, Any]],
    claimed_refs: set[str],
    required_scoreboard_refs: set[str],
    known_scoreboard_refs: set[str],
    markers: dict[str, dict[str, Any]],
    findings: list[Finding],
) -> None:
    covered_refs: set[str] = set()
    covered_scoreboard_refs: set[str] = set()
    for row in rows.values():
        covered_refs.add(ensure_str(row.get("surface_ref"), "row.surface_ref"))
        covered_refs.update(ensure_str(item, "row.source_scope_refs[]") for item in ensure_list(row.get("source_scope_refs"), "row.source_scope_refs"))
        scoreboard_refs = {
            ensure_str(item, "row.scoreboard_row_refs[]")
            for item in ensure_list(row.get("scoreboard_row_refs"), "row.scoreboard_row_refs")
        }
        covered_scoreboard_refs.update(scoreboard_refs)
        for scoreboard_ref in scoreboard_refs:
            if scoreboard_ref not in known_scoreboard_refs:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="registry.rows.scoreboard_row_refs.unknown",
                        message=f"lifecycle row cites unknown scoreboard row: {scoreboard_ref}",
                        remediation="Correct the scoreboard row ref or add it to the exit scoreboard.",
                        ref=ensure_str(row.get("row_id"), "row.row_id"),
                    )
                )

    missing_claimed = sorted(claimed_refs - covered_refs)
    if missing_claimed:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.coverage.claimed_surface_missing",
                message="claimed alpha surfaces from the wedge matrix are missing lifecycle coverage",
                remediation="Add each missing wedge, workflow, deployment, launch-bundle, or archetype ref to a lifecycle row.",
                details={"missing": missing_claimed},
            )
        )

    missing_scoreboard = sorted(required_scoreboard_refs - covered_scoreboard_refs)
    if missing_scoreboard:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.coverage.scoreboard_row_missing",
                message="marketed alpha scoreboard rows are missing lifecycle coverage",
                remediation="Add each missing scoreboard row to a lifecycle row.",
                details={"missing": missing_scoreboard},
            )
        )

    covered_artifact_classes: set[str] = set()
    for marker in markers.values():
        covered_artifact_classes.update(ensure_list(marker.get("artifact_classes"), "marker.artifact_classes"))
    missing_artifact_classes = sorted(REQUIRED_ARTIFACT_CLASSES - covered_artifact_classes)
    if missing_artifact_classes:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.markers.required_artifact_class_missing",
                message="dependency markers do not cover every required launch-wedge artifact class",
                remediation="Add markers for profile artifacts, workspace manifests, saved views, exports, migration packets, bundle claims, and archetype claims.",
                details={"missing": missing_artifact_classes},
            )
        )


def validate_acceptance_fixtures(repo_root: Path, registry: dict[str, Any], fixture_manifest: dict[str, Any], findings: list[Finding]) -> None:
    acceptance_states = {
        ensure_str(row.get("exercises_state"), "registry.acceptance_state_coverage[].exercises_state")
        for row in ensure_list(registry.get("acceptance_state_coverage"), "registry.acceptance_state_coverage")
    }
    fixture_states = {
        ensure_str(row.get("case_id"), "fixtures.acceptance_cases[].case_id")
        for row in ensure_list(fixture_manifest.get("acceptance_cases"), "fixtures.acceptance_cases")
    }
    missing_states = sorted(REQUIRED_ACCEPTANCE_STATES - acceptance_states)
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.acceptance_state_coverage.missing",
                message="registry acceptance_state_coverage does not exercise all required states",
                remediation="Add coverage for lifecycle coverage, marker disclosure, no unknown lifecycle, policy disable, and consumer projection.",
                details={"missing": missing_states},
            )
        )
    missing_cases = sorted(REQUIRED_ACCEPTANCE_STATES - fixture_states)
    if missing_cases:
        findings.append(
            Finding(
                severity="error",
                check_id="fixtures.acceptance_cases.missing",
                message="fixture manifest is missing required acceptance cases",
                remediation="Add the protected fixture cases that the registry references.",
                details={"missing": missing_cases},
            )
        )
    for idx, raw_row in enumerate(ensure_list(fixture_manifest.get("acceptance_cases"), "fixtures.acceptance_cases")):
        row = ensure_dict(raw_row, f"fixtures.acceptance_cases[{idx}]")
        fixture_ref = ensure_str(row.get("registry_fixture_ref"), f"fixtures.acceptance_cases[{idx}].registry_fixture_ref")
        validate_path_ref(repo_root, fixture_ref, "fixtures.acceptance_cases.registry_fixture_ref", findings)


def validate_first_consumers(registry: dict[str, Any], findings: list[Finding]) -> None:
    consumers = ensure_list(registry.get("first_consumers"), "registry.first_consumers")
    if not consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="registry.first_consumers.empty",
                message="registry must name at least one first consumer",
                remediation="Add a Help/About, settings, diagnostics, or support-export consumer that reads the registry.",
            )
        )
    for idx, raw_consumer in enumerate(consumers):
        consumer = ensure_dict(raw_consumer, f"registry.first_consumers[{idx}]")
        consumer_id = ensure_str(consumer.get("consumer_id"), f"registry.first_consumers[{idx}].consumer_id")
        rendered_fields = {
            ensure_str(item, f"registry.first_consumers[{idx}].required_rendered_fields[]")
            for item in ensure_list(consumer.get("required_rendered_fields"), f"registry.first_consumers[{idx}].required_rendered_fields")
        }
        missing_fields = sorted(REQUIRED_CONSUMER_FIELDS - rendered_fields)
        if missing_fields:
            findings.append(
                Finding(
                    severity="error",
                    check_id="registry.first_consumers.required_fields_missing",
                    message=f"{consumer_id} does not render all required fields",
                    remediation="Render lifecycle state, owner, review or expiry, and dependency markers directly from the registry.",
                    ref=consumer_id,
                    details={"missing": missing_fields},
                )
            )


def build_help_projection(rows: dict[str, dict[str, Any]], markers: dict[str, dict[str, Any]]) -> list[dict[str, Any]]:
    projection: list[dict[str, Any]] = []
    for row_id in sorted(rows):
        row = rows[row_id]
        marker_entries = []
        for marker_ref in ensure_list(row.get("dependency_marker_refs"), f"{row_id}.dependency_marker_refs"):
            marker = markers.get(str(marker_ref))
            if marker is None:
                continue
            marker_entries.append(
                {
                    "marker_id": marker["marker_id"],
                    "marker_kind": marker["marker_kind"],
                    "artifact_classes": marker["artifact_classes"],
                    "dependency_lifecycle_state": marker["dependency_lifecycle_state"],
                    "effect_on_parent": marker["effect_on_parent"],
                    "disclosure_summary": marker["disclosure_summary"],
                    "repair_or_review_ref": marker["repair_or_review_ref"],
                }
            )
        projection.append(
            {
                "row_id": row_id,
                "surface_ref": row["surface_ref"],
                "surface_kind": row["surface_kind"],
                "title": row["title"],
                "declared_lifecycle_state": row["declared_lifecycle_state"],
                "effective_lifecycle_state": row["effective_lifecycle_state"],
                "owner": row["owner"],
                "review_or_expiry_date": row["review_or_expiry_date"],
                "support_promise": row["support_promise"],
                "migration_note": row["migration_note"],
                "kill_switch_or_policy_disable_ref": row["kill_switch_or_policy_disable_ref"],
                "dependency_markers": marker_entries,
            }
        )
    return projection


def write_report(
    path: Path,
    registry_rel: str,
    rows: dict[str, dict[str, Any]],
    markers: dict[str, dict[str, Any]],
    projection: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "registry_ref": registry_rel,
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_rows": sorted(rows),
            "checked_markers": sorted(markers),
            "projected_rows": len(projection),
        },
        "consumer_projection": projection,
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    if not (repo_root / ".git").exists():
        raise SystemExit(f"--repo-root does not look like a repository root: {repo_root}")

    registry_rel = str(args.registry)
    registry = ensure_dict(render_yaml_as_json(repo_root / registry_rel), "registry")
    marker_schema = ensure_dict(json.loads((repo_root / args.marker_schema).read_text(encoding="utf-8")), "marker_schema")
    matrix = ensure_dict(render_yaml_as_json(repo_root / args.matrix), "matrix")
    scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.scoreboard), "scoreboard")
    fixture_manifest = ensure_dict(render_yaml_as_json(repo_root / args.fixtures), "fixture_manifest")

    findings: list[Finding] = []
    validate_header(registry, findings)
    validate_registry_refs(repo_root, registry, findings)
    lifecycle_vocabulary = validate_vocabulary(registry, marker_schema, findings)
    rows = collect_rows(registry, lifecycle_vocabulary, findings)
    markers = collect_markers(registry, lifecycle_vocabulary, rows, findings)
    validate_row_marker_links(rows, markers, findings)
    claimed_refs, required_scoreboard_refs = collect_claimed_matrix_refs(matrix)
    validate_coverage(
        rows=rows,
        claimed_refs=claimed_refs,
        required_scoreboard_refs=required_scoreboard_refs,
        known_scoreboard_refs=collect_scoreboard_ids(scoreboard),
        markers=markers,
        findings=findings,
    )
    validate_acceptance_fixtures(repo_root, registry, fixture_manifest, findings)
    validate_first_consumers(registry, findings)

    projection = build_help_projection(rows, markers)
    if args.report:
        write_report(repo_root / args.report, registry_rel, rows, markers, projection, findings)
    if args.render_help_projection:
        print(json.dumps({"schema_version": 1, "surface_class": "help_about_diagnostics_projection", "rows": projection}, indent=2, sort_keys=True))

    errors = [item for item in findings if item.severity == "error"]
    warnings = [item for item in findings if item.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[capability-lifecycle-registry] {status} ({len(errors)} errors, {len(warnings)} warnings)", file=sys.stderr)
    for finding in findings:
        print(
            f"[capability-lifecycle-registry] {finding.severity.upper()} {finding.check_id}: {finding.message}",
            file=sys.stderr,
        )
        print(f"[capability-lifecycle-registry]   remediation: {finding.remediation}", file=sys.stderr)
        if finding.ref:
            print(f"[capability-lifecycle-registry]   ref: {finding.ref}", file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
