#!/usr/bin/env python3
"""Validate the external alpha scope matrix and go/no-go scoreboard."""

from __future__ import annotations

import argparse
import datetime as dt
import json
import subprocess
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any


DEFAULT_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_GRAPH_REL = "artifacts/milestones/m2/dependency_graph.yaml"

REQUIRED_WEDGES = {
    "alpha_wedge:typescript_javascript",
    "alpha_wedge:python",
}

REQUIRED_SCOREBOARD_ROWS = {
    "scoreboard_row:alpha_scope.scope_change_control",
    "scoreboard_row:alpha_scope.ts_js_navigation",
    "scoreboard_row:alpha_scope.ts_js_run_test_debug",
    "scoreboard_row:alpha_scope.ts_js_git_review",
    "scoreboard_row:alpha_scope.python_environment_tests",
    "scoreboard_row:alpha_scope.python_debug_refactor",
    "scoreboard_row:alpha_scope.local_deployment",
    "scoreboard_row:alpha_scope.helper_backed_deployment",
    "scoreboard_row:alpha_scope.migration_parity",
    "scoreboard_row:alpha_scope.design_partner_intake",
    "scoreboard_row:alpha_scope.benchmark_fixtures",
    "scoreboard_row:alpha_scope.schema_record_registry",
    "scoreboard_row:alpha_scope.docs_known_limits",
    "scoreboard_row:alpha_scope.supportability",
}

UPSTREAM_REGISTRY_REFS = {
    "language_bundles": "artifacts/product/language_bundle_rows.yaml",
    "personas": "artifacts/product/p0_persona_rows.yaml",
    "reference_workspaces": "artifacts/compat/reference_workspace_rows.yaml",
    "scoreboard_families": "artifacts/qe/workflow_bundle_ids.yaml",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "alpha_wedge:",
    "archetype_row:",
    "claim_row:",
    "compat_row:",
    "deployment.",
    "fixture.",
    "framework_pack:",
    "launch_bundle:",
    "lane:",
    "persona:",
    "proof_packet:",
    "scoreboard_row:",
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
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--graph", default=DEFAULT_GRAPH_REL)
    parser.add_argument("--report", default=None)
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


def parse_iso_date(value: str, label: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError as exc:
        raise SystemExit(f"{label} must be a YYYY-MM-DD date, got {value!r}") from exc


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


def collect_values_by_key(value: Any, key: str) -> set[str]:
    found: set[str] = set()
    if isinstance(value, dict):
        for current_key, current_value in value.items():
            if current_key == key and isinstance(current_value, str):
                found.add(current_value)
            else:
                found.update(collect_values_by_key(current_value, key))
    elif isinstance(value, list):
        for item in value:
            found.update(collect_values_by_key(item, key))
    return found


def load_upstream_ids(repo_root: Path) -> dict[str, set[str]]:
    language_bundles = ensure_dict(
        render_yaml_as_json(repo_root / UPSTREAM_REGISTRY_REFS["language_bundles"]),
        "language_bundles",
    )
    personas = ensure_dict(
        render_yaml_as_json(repo_root / UPSTREAM_REGISTRY_REFS["personas"]),
        "personas",
    )
    reference_workspaces = ensure_dict(
        render_yaml_as_json(repo_root / UPSTREAM_REGISTRY_REFS["reference_workspaces"]),
        "reference_workspaces",
    )
    scoreboard_families = ensure_dict(
        render_yaml_as_json(repo_root / UPSTREAM_REGISTRY_REFS["scoreboard_families"]),
        "scoreboard_families",
    )

    family_ids = collect_values_by_key(scoreboard_families, "family_id")
    vocabulary = scoreboard_families.get("scoreboard_family_vocabulary")
    if isinstance(vocabulary, list):
        family_ids.update(item for item in vocabulary if isinstance(item, str))

    return {
        "launch_bundles": collect_values_by_key(language_bundles, "bundle_id"),
        "personas": collect_values_by_key(personas, "persona_id"),
        "archetype_rows": collect_values_by_key(reference_workspaces, "archetype_row_id"),
        "scoreboard_families": family_ids,
    }


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that bumps the artifact schema.",
            )
        )
    parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of")
    ensure_str(payload.get("owner"), f"{label}.owner")


def validate_path_refs(repo_root: Path, refs: list[Any], label: str, findings: list[Finding], required: bool = True) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Replace the empty or non-string ref with a repo-relative artifact path or stable row id.",
                )
            )
            continue
        ref = raw_ref.strip()
        if looks_like_path(ref) and required and not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.missing_ref",
                    message=f"{label}[{idx}] does not resolve: {ref}",
                    remediation="Fix the path or seed the referenced artifact so the alpha scope graph remains inspectable.",
                    ref=ref,
                )
            )


def validate_known_id_refs(
    refs: list[Any],
    allowed_ids: set[str],
    label: str,
    row_ref: str,
    findings: list[Finding],
) -> None:
    for idx, raw_ref in enumerate(refs):
        if not isinstance(raw_ref, str) or not raw_ref.strip():
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.invalid_ref",
                    message=f"{label}[{idx}] must be a non-empty string",
                    remediation="Use a stable id from the upstream register.",
                    ref=row_ref,
                )
            )
            continue
        ref = raw_ref.strip()
        if ref not in allowed_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"{label}.unknown_ref",
                    message=f"{label}[{idx}] is not present in its upstream register: {ref}",
                    remediation="Fix the id or add the upstream register row in the same change.",
                    ref=ref,
                )
            )


def validate_matrix(
    repo_root: Path,
    matrix: dict[str, Any],
    scoreboard_ids: set[str],
    upstream_ids: dict[str, set[str]],
    findings: list[Finding],
) -> set[str]:
    validate_header(matrix, "matrix", findings)
    ensure_str(matrix.get("matrix_id"), "matrix.matrix_id")
    if ensure_str(matrix.get("scope_state"), "matrix.scope_state") != "frozen":
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.scope_state.not_frozen",
                message="matrix.scope_state must be frozen",
                remediation="Set scope_state to frozen once the alpha claim surface is locked.",
            )
        )

    validate_path_refs(
        repo_root,
        [
            matrix.get("human_entrypoint_ref"),
            matrix.get("scoreboard_ref"),
            matrix.get("dependency_graph_ref"),
        ],
        "matrix.primary_refs",
        findings,
    )

    change_control = ensure_dict(matrix.get("change_control"), "matrix.change_control")
    if ensure_str(change_control.get("late_addition_policy"), "matrix.change_control.late_addition_policy") != "explicit_scope_review_or_waiver":
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.change_control.late_addition_policy",
                message="late additions must require explicit scope review or waiver",
                remediation="Set late_addition_policy to explicit_scope_review_or_waiver.",
            )
        )
    required_addition_refs = {
        "artifacts/milestones/m2/alpha_wedge_matrix.yaml",
        "artifacts/milestones/m2/exit_gate_scoreboard.yaml",
        "artifacts/milestones/m2/dependency_graph.yaml",
        "artifacts/milestones/m2/proof_packets/alpha_scope.md#known-limits",
    }
    addition_refs = set(ensure_list(change_control.get("addition_requires_refs"), "matrix.change_control.addition_requires_refs"))
    missing_addition_refs = required_addition_refs - addition_refs
    if missing_addition_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.change_control.missing_addition_refs",
                message="change_control.addition_requires_refs is missing required alpha scope artifacts",
                remediation="Add the matrix, scoreboard, dependency graph, and known-limits packet refs.",
                details={"missing": sorted(missing_addition_refs)},
            )
        )

    protected_fixtures = ensure_list(matrix.get("protected_fixture_refs"), "matrix.protected_fixture_refs")
    if len(protected_fixtures) < 2:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.protected_fixtures.too_few",
                message="protected_fixture_refs must include at least TS/JS and Python fixtures",
                remediation="Add protected fixture refs for both claimed alpha wedges.",
            )
        )
    for idx, raw_fixture in enumerate(protected_fixtures):
        fixture = ensure_dict(raw_fixture, f"matrix.protected_fixture_refs[{idx}]")
        ensure_str(fixture.get("fixture_id"), f"matrix.protected_fixture_refs[{idx}].fixture_id")
        ensure_str(fixture.get("wedge_ref"), f"matrix.protected_fixture_refs[{idx}].wedge_ref")
        fixture_ref = ensure_str(fixture.get("fixture_ref"), f"matrix.protected_fixture_refs[{idx}].fixture_ref")
        if not artifact_ref_exists(repo_root, fixture_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.protected_fixture_refs.missing",
                    message=f"protected fixture ref does not exist: {fixture_ref}",
                    remediation="Seed the fixture or fix the path.",
                    ref=fixture_ref,
                )
            )
        for row_ref in ensure_list(fixture.get("exercises_scoreboard_rows"), f"matrix.protected_fixture_refs[{idx}].exercises_scoreboard_rows"):
            if row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.protected_fixture_refs.unknown_scoreboard_row",
                        message=f"protected fixture cites unknown scoreboard row: {row_ref}",
                        remediation="Add the scoreboard row or correct the fixture binding.",
                        ref=str(row_ref),
                    )
                )

    wedge_rows = ensure_list(matrix.get("wedge_rows"), "matrix.wedge_rows")
    wedge_ids: set[str] = set()
    referenced_scoreboard_rows: set[str] = set()
    for idx, raw_row in enumerate(wedge_rows):
        row = ensure_dict(raw_row, f"matrix.wedge_rows[{idx}]")
        wedge_id = ensure_str(row.get("wedge_id"), f"matrix.wedge_rows[{idx}].wedge_id")
        if wedge_id in wedge_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.wedge_rows.duplicate",
                    message=f"duplicate wedge_id: {wedge_id}",
                    remediation="Use one row per alpha wedge id.",
                    ref=wedge_id,
                )
            )
        wedge_ids.add(wedge_id)
        scope_state = ensure_str(row.get("scope_state"), f"matrix.wedge_rows[{idx}].scope_state")
        ensure_str(row.get("claim_posture"), f"matrix.wedge_rows[{idx}].claim_posture")
        if scope_state != "in_scope_alpha":
            continue

        for field_name in ("launch_bundle_refs", "archetype_row_refs", "persona_refs", "required_packet_refs"):
            values = ensure_list(row.get(field_name), f"matrix.wedge_rows[{idx}].{field_name}")
            if not values:
                findings.append(
                    Finding(
                        severity="error",
                        check_id=f"matrix.wedge_rows.{field_name}.empty",
                        message=f"{wedge_id} must declare non-empty {field_name}",
                        remediation=f"Add at least one {field_name} entry to the in-scope wedge row.",
                        ref=wedge_id,
                    )
                )
        validate_known_id_refs(
            ensure_list(row.get("launch_bundle_refs"), f"matrix.wedge_rows[{idx}].launch_bundle_refs"),
            upstream_ids["launch_bundles"],
            "matrix.wedge_rows.launch_bundle_refs",
            wedge_id,
            findings,
        )
        validate_known_id_refs(
            ensure_list(row.get("archetype_row_refs"), f"matrix.wedge_rows[{idx}].archetype_row_refs"),
            upstream_ids["archetype_rows"],
            "matrix.wedge_rows.archetype_row_refs",
            wedge_id,
            findings,
        )
        validate_known_id_refs(
            ensure_list(row.get("persona_refs"), f"matrix.wedge_rows[{idx}].persona_refs"),
            upstream_ids["personas"],
            "matrix.wedge_rows.persona_refs",
            wedge_id,
            findings,
        )

        required_scoreboard_rows = ensure_list(row.get("required_scoreboard_rows"), f"matrix.wedge_rows[{idx}].required_scoreboard_rows")
        if not required_scoreboard_rows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.wedge_rows.required_scoreboard_rows.empty",
                    message=f"{wedge_id} must declare required_scoreboard_rows",
                    remediation="Bind the wedge to one or more scoreboard rows.",
                    ref=wedge_id,
                )
            )
        for row_ref in required_scoreboard_rows:
            referenced_scoreboard_rows.add(str(row_ref))
            if row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.wedge_rows.required_scoreboard_rows.unknown",
                        message=f"{wedge_id} cites unknown scoreboard row: {row_ref}",
                        remediation="Add the scoreboard row or correct the matrix reference.",
                        ref=str(row_ref),
                    )
                )

        workflows = ensure_list(row.get("protected_workflows"), f"matrix.wedge_rows[{idx}].protected_workflows")
        if not workflows:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.wedge_rows.protected_workflows.empty",
                    message=f"{wedge_id} must declare protected workflows",
                    remediation="Add the workflows this alpha wedge is allowed to claim.",
                    ref=wedge_id,
                )
            )
        for workflow_idx, raw_workflow in enumerate(workflows):
            workflow = ensure_dict(raw_workflow, f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}]")
            ensure_str(workflow.get("workflow_id"), f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].workflow_id")
            ensure_str(workflow.get("title"), f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].title")
            scoreboard_row_ref = ensure_str(
                workflow.get("scoreboard_row_ref"),
                f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].scoreboard_row_ref",
            )
            referenced_scoreboard_rows.add(scoreboard_row_ref)
            if scoreboard_row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.protected_workflows.unknown_scoreboard_row",
                        message=f"workflow cites unknown scoreboard row: {scoreboard_row_ref}",
                        remediation="Add the scoreboard row or correct the workflow binding.",
                        ref=scoreboard_row_ref,
                    )
                )
            proof_packet = ensure_str(
                workflow.get("proof_packet_required"),
                f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].proof_packet_required",
            )
            if not proof_packet.startswith("proof_packet:"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.protected_workflows.invalid_proof_packet",
                        message=f"workflow proof_packet_required must use proof_packet: id, got {proof_packet}",
                        remediation="Use a stable proof_packet: id so downstream packets can cite it.",
                        ref=proof_packet,
                    )
                )
            validate_path_refs(
                repo_root,
                ensure_list(workflow.get("acceptance_refs"), f"matrix.wedge_rows[{idx}].protected_workflows[{workflow_idx}].acceptance_refs"),
                "matrix.protected_workflows.acceptance_refs",
                findings,
            )

        deployments = ensure_list(row.get("claimed_deployment_rows"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows")
        if not deployments:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.wedge_rows.claimed_deployment_rows.empty",
                    message=f"{wedge_id} must declare claimed deployment rows",
                    remediation="Add at least one local or helper-backed deployment row.",
                    ref=wedge_id,
                )
            )
        for deployment_idx, raw_deployment in enumerate(deployments):
            deployment = ensure_dict(raw_deployment, f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}]")
            ensure_str(deployment.get("deployment_claim_id"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].deployment_claim_id")
            ensure_str(deployment.get("claim_class"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].claim_class")
            ensure_str(deployment.get("support_state"), f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].support_state")
            validate_path_refs(
                repo_root,
                [deployment.get("deployment_profile_ref")],
                "matrix.claimed_deployment_rows.deployment_profile_ref",
                findings,
            )
            scoreboard_row_ref = ensure_str(
                deployment.get("scoreboard_row_ref"),
                f"matrix.wedge_rows[{idx}].claimed_deployment_rows[{deployment_idx}].scoreboard_row_ref",
            )
            referenced_scoreboard_rows.add(scoreboard_row_ref)
            if scoreboard_row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="matrix.claimed_deployment_rows.unknown_scoreboard_row",
                        message=f"deployment row cites unknown scoreboard row: {scoreboard_row_ref}",
                        remediation="Add the scoreboard row or correct the deployment binding.",
                        ref=scoreboard_row_ref,
                    )
                )

        if not ensure_list(row.get("explicit_out_of_scope"), f"matrix.wedge_rows[{idx}].explicit_out_of_scope"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.wedge_rows.explicit_out_of_scope.empty",
                    message=f"{wedge_id} must name explicit out-of-scope items",
                    remediation="Add at least one explicit out-of-scope row so alpha claims cannot widen silently.",
                    ref=wedge_id,
                )
            )

    missing_wedges = REQUIRED_WEDGES - wedge_ids
    if missing_wedges:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.wedge_rows.missing_required",
                message="matrix is missing required alpha wedge rows",
                remediation="Add TypeScript / JavaScript and Python wedge rows.",
                details={"missing": sorted(missing_wedges)},
            )
        )

    non_claimed_rows = ensure_list(matrix.get("non_claimed_rows"), "matrix.non_claimed_rows")
    if not non_claimed_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="matrix.non_claimed_rows.empty",
                message="matrix must declare held or out-of-scope rows",
                remediation="Add non_claimed_rows so scope expansion cannot happen by omission.",
            )
        )
    for idx, raw_row in enumerate(non_claimed_rows):
        row = ensure_dict(raw_row, f"matrix.non_claimed_rows[{idx}]")
        ensure_str(row.get("row_id"), f"matrix.non_claimed_rows[{idx}].row_id")
        ensure_str(row.get("scope_state"), f"matrix.non_claimed_rows[{idx}].scope_state")
        widening_requires = set(ensure_list(row.get("widening_requires"), f"matrix.non_claimed_rows[{idx}].widening_requires"))
        required_widening = {"scope_review", "scoreboard_row", "proof_packet", "known_limit_note"}
        missing = required_widening - widening_requires
        if missing:
            findings.append(
                Finding(
                    severity="error",
                    check_id="matrix.non_claimed_rows.widening_requires",
                    message=f"non-claimed row is missing widening requirements: {row.get('row_id')}",
                    remediation="Require scope_review, scoreboard_row, proof_packet, and known_limit_note before widening.",
                    ref=str(row.get("row_id")),
                    details={"missing": sorted(missing)},
                )
            )

    return referenced_scoreboard_rows


def validate_scoreboard(
    repo_root: Path,
    scoreboard: dict[str, Any],
    upstream_ids: dict[str, set[str]],
    findings: list[Finding],
) -> set[str]:
    validate_header(scoreboard, "scoreboard", findings)
    ensure_str(scoreboard.get("scoreboard_id"), "scoreboard.scoreboard_id")
    validate_path_refs(
        repo_root,
        [
            scoreboard.get("human_entrypoint_ref"),
            scoreboard.get("matrix_ref"),
            scoreboard.get("dependency_graph_ref"),
        ],
        "scoreboard.primary_refs",
        findings,
    )

    current_states = set(ensure_list(scoreboard.get("current_state_vocabulary"), "scoreboard.current_state_vocabulary"))
    go_no_go_states = set(ensure_list(scoreboard.get("go_no_go_vocabulary"), "scoreboard.go_no_go_vocabulary"))
    proof_lane_classes = set(ensure_list(scoreboard.get("proof_lane_class_vocabulary"), "scoreboard.proof_lane_class_vocabulary"))

    rows = ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")
    row_ids: set[str] = set()
    for idx, raw_row in enumerate(rows):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        row_id = ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id")
        if row_id in row_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.duplicate",
                    message=f"duplicate scoreboard row_id: {row_id}",
                    remediation="Use one row per scoreboard row id.",
                    ref=row_id,
                )
            )
        row_ids.add(row_id)
        ensure_str(row.get("title"), f"scoreboard.scoreboard_rows[{idx}].title")
        ensure_str(row.get("lane"), f"scoreboard.scoreboard_rows[{idx}].lane")
        ensure_str(row.get("owner_dri"), f"scoreboard.scoreboard_rows[{idx}].owner_dri")
        ensure_str(row.get("consumer_rule"), f"scoreboard.scoreboard_rows[{idx}].consumer_rule")
        proof_lane_class = ensure_str(row.get("proof_lane_class"), f"scoreboard.scoreboard_rows[{idx}].proof_lane_class")
        if proof_lane_class not in proof_lane_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.invalid_proof_lane_class",
                    message=f"{row_id} uses unknown proof_lane_class: {proof_lane_class}",
                    remediation="Use a proof_lane_class listed in proof_lane_class_vocabulary.",
                    ref=row_id,
                )
            )
        scoreboard_family_ref = row.get("scoreboard_family_ref")
        if scoreboard_family_ref is not None:
            family_ref = ensure_str(scoreboard_family_ref, f"scoreboard.scoreboard_rows[{idx}].scoreboard_family_ref")
            if family_ref not in upstream_ids["scoreboard_families"]:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.unknown_scoreboard_family",
                        message=f"{row_id} cites unknown scoreboard_family_ref: {family_ref}",
                        remediation="Use a family id from artifacts/qe/workflow_bundle_ids.yaml.",
                        ref=row_id,
                    )
                )
        current_state = ensure_str(row.get("current_state"), f"scoreboard.scoreboard_rows[{idx}].current_state")
        if current_state not in current_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.invalid_current_state",
                    message=f"{row_id} uses unknown current_state: {current_state}",
                    remediation="Use a state listed in current_state_vocabulary.",
                    ref=row_id,
                )
            )
        go_no_go_state = ensure_str(row.get("go_no_go_state"), f"scoreboard.scoreboard_rows[{idx}].go_no_go_state")
        if go_no_go_state not in go_no_go_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.invalid_go_no_go_state",
                    message=f"{row_id} uses unknown go_no_go_state: {go_no_go_state}",
                    remediation="Use a state listed in go_no_go_vocabulary.",
                    ref=row_id,
                )
            )
        proof_packet_refs = ensure_list(row.get("proof_packet_refs"), f"scoreboard.scoreboard_rows[{idx}].proof_packet_refs")
        if not proof_packet_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="scoreboard.rows.proof_packet_refs.empty",
                    message=f"{row_id} must name proof_packet_refs",
                    remediation="Add at least one proof_packet: ref to the scoreboard row.",
                    ref=row_id,
                )
            )
        for packet_ref in proof_packet_refs:
            if not isinstance(packet_ref, str) or not packet_ref.startswith("proof_packet:"):
                findings.append(
                    Finding(
                        severity="error",
                        check_id="scoreboard.rows.invalid_proof_packet_ref",
                        message=f"{row_id} has invalid proof packet ref: {packet_ref}",
                        remediation="Use stable proof_packet: ids so later packets can cite the row.",
                        ref=row_id,
                    )
                )
        validate_path_refs(
            repo_root,
            ensure_list(row.get("required_evidence_refs"), f"scoreboard.scoreboard_rows[{idx}].required_evidence_refs"),
            "scoreboard.rows.required_evidence_refs",
            findings,
        )

    missing_rows = REQUIRED_SCOREBOARD_ROWS - row_ids
    if missing_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.rows.missing_required",
                message="scoreboard is missing required alpha proof lane rows",
                remediation="Add the missing row ids so downstream proof lanes can cite them.",
                details={"missing": sorted(missing_rows)},
            )
        )

    acceptance_coverage = ensure_list(scoreboard.get("acceptance_state_coverage"), "scoreboard.acceptance_state_coverage")
    exercised_states = {ensure_str(ensure_dict(row, "scoreboard.acceptance_state_coverage[]").get("exercises_state"), "scoreboard.acceptance_state_coverage[].exercises_state") for row in acceptance_coverage}
    required_exercised = {"seeded", "blocked_pending_packet", "scope_review_required"}
    missing_exercised = required_exercised - exercised_states
    if missing_exercised:
        findings.append(
            Finding(
                severity="error",
                check_id="scoreboard.acceptance_state_coverage.missing",
                message="acceptance_state_coverage is missing required states",
                remediation="Add coverage rows for seeded, blocked_pending_packet, and scope_review_required.",
                details={"missing": sorted(missing_exercised)},
            )
        )
    for idx, raw_case in enumerate(acceptance_coverage):
        case = ensure_dict(raw_case, f"scoreboard.acceptance_state_coverage[{idx}]")
        validate_path_refs(repo_root, [case.get("fixture_ref")], "scoreboard.acceptance_state_coverage.fixture_ref", findings)
        ensure_str(case.get("expected_validator_result"), f"scoreboard.acceptance_state_coverage[{idx}].expected_validator_result")

    return row_ids


def validate_graph(repo_root: Path, graph: dict[str, Any], findings: list[Finding]) -> None:
    validate_header(graph, "graph", findings)
    ensure_str(graph.get("graph_id"), "graph.graph_id")
    validate_path_refs(
        repo_root,
        [
            graph.get("human_entrypoint_ref"),
            graph.get("matrix_ref"),
            graph.get("scoreboard_ref"),
        ],
        "graph.primary_refs",
        findings,
    )

    nodes = ensure_list(graph.get("nodes"), "graph.nodes")
    node_ids: set[str] = set()
    for idx, raw_node in enumerate(nodes):
        node = ensure_dict(raw_node, f"graph.nodes[{idx}]")
        node_id = ensure_str(node.get("node_id"), f"graph.nodes[{idx}].node_id")
        if node_id in node_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.nodes.duplicate",
                    message=f"duplicate graph node_id: {node_id}",
                    remediation="Use one graph node per node id.",
                    ref=node_id,
                )
            )
        node_ids.add(node_id)
        ensure_str(node.get("kind"), f"graph.nodes[{idx}].kind")
        if not isinstance(node.get("required"), bool):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.nodes.required_not_bool",
                    message=f"graph node required must be boolean: {node_id}",
                    remediation="Set required to true or false.",
                    ref=node_id,
                )
            )
        artifact_ref = ensure_str(node.get("artifact_ref"), f"graph.nodes[{idx}].artifact_ref")
        if node.get("required") is True and not artifact_ref_exists(repo_root, artifact_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="graph.nodes.missing_required_ref",
                    message=f"required graph node does not resolve: {artifact_ref}",
                    remediation="Seed the artifact or fix the graph node artifact_ref.",
                    ref=artifact_ref,
                )
            )

    required_nodes = {
        "alpha_wedge_matrix",
        "alpha_exit_gate_scoreboard",
        "alpha_scope_validator",
        "workflow_bundle_register",
        "launch_language_bundles",
        "reference_workspace_rows",
        "design_partner_intake",
        "benchmark_corpus_manifest",
        "known_limits_contract",
        "alpha_scope_proof_packet",
    }
    missing_nodes = required_nodes - node_ids
    if missing_nodes:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.nodes.missing_required",
                message="dependency graph is missing required nodes",
                remediation="Add nodes for the matrix, scoreboard, validator, upstream contracts, fixtures, and proof packet.",
                details={"missing": sorted(missing_nodes)},
            )
        )

    edges = ensure_list(graph.get("edges"), "graph.edges")
    for idx, raw_edge in enumerate(edges):
        edge = ensure_dict(raw_edge, f"graph.edges[{idx}]")
        from_node = ensure_str(edge.get("from"), f"graph.edges[{idx}].from")
        to_node = ensure_str(edge.get("to"), f"graph.edges[{idx}].to")
        ensure_str(edge.get("rationale"), f"graph.edges[{idx}].rationale")
        for edge_node in (from_node, to_node):
            if edge_node not in node_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="graph.edges.unknown_node",
                        message=f"edge references unknown graph node: {edge_node}",
                        remediation="Add the node or correct the edge endpoint.",
                        ref=edge_node,
                    )
                )

    consumer_surfaces = ensure_list(graph.get("consumer_surfaces"), "graph.consumer_surfaces")
    validator_consumers = []
    for idx, raw_consumer in enumerate(consumer_surfaces):
        consumer = ensure_dict(raw_consumer, f"graph.consumer_surfaces[{idx}]")
        consumer_id = ensure_str(consumer.get("consumer_id"), f"graph.consumer_surfaces[{idx}].consumer_id")
        command = ensure_str(consumer.get("command"), f"graph.consumer_surfaces[{idx}].command")
        ensure_list(consumer.get("reads"), f"graph.consumer_surfaces[{idx}].reads")
        validate_path_refs(repo_root, [consumer.get("report_ref")], "graph.consumer_surfaces.report_ref", findings, required=False)
        if "check_alpha_scope.py" in command or consumer_id == "alpha_scope_validator":
            validator_consumers.append(consumer_id)
    if not validator_consumers:
        findings.append(
            Finding(
                severity="error",
                check_id="graph.consumer_surfaces.no_validator",
                message="dependency graph must declare the validator as the first consumer",
                remediation="Add a consumer_surfaces row for ci/check_alpha_scope.py.",
            )
        )


def write_report(path: Path, findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    matrix = ensure_dict(render_yaml_as_json(repo_root / args.matrix), "matrix")
    scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.scoreboard), "scoreboard")
    graph = ensure_dict(render_yaml_as_json(repo_root / args.graph), "graph")
    upstream_ids = load_upstream_ids(repo_root)

    findings: list[Finding] = []
    scoreboard_ids = validate_scoreboard(repo_root, scoreboard, upstream_ids, findings)
    referenced_scoreboard_ids = validate_matrix(repo_root, matrix, scoreboard_ids, upstream_ids, findings)
    validate_graph(repo_root, graph, findings)

    unused_required_refs = REQUIRED_SCOREBOARD_ROWS - referenced_scoreboard_ids - {"scoreboard_row:alpha_scope.scope_change_control", "scoreboard_row:alpha_scope.design_partner_intake", "scoreboard_row:alpha_scope.benchmark_fixtures", "scoreboard_row:alpha_scope.schema_record_registry", "scoreboard_row:alpha_scope.migration_parity"}
    if unused_required_refs:
        findings.append(
            Finding(
                severity="warning",
                check_id="matrix.scoreboard_refs.unused_required",
                message="some required scoreboard rows are not directly referenced by wedge workflow/deployment rows",
                remediation="This is acceptable for cross-wedge baselines, but review whether the matrix should cite them explicitly.",
                details={"row_ids": sorted(unused_required_refs)},
            )
        )

    if args.report:
        write_report(repo_root / args.report, findings)

    errors = [item for item in findings if item.severity == "error"]
    if errors:
        for item in errors:
            ref = f" ({item.ref})" if item.ref else ""
            print(f"ERROR [{item.check_id}]{ref}: {item.message}", file=sys.stderr)
            print(f"  remediation: {item.remediation}", file=sys.stderr)
        return 1

    warnings = [item for item in findings if item.severity == "warning"]
    for item in warnings:
        ref = f" ({item.ref})" if item.ref else ""
        print(f"WARNING [{item.check_id}]{ref}: {item.message}", file=sys.stderr)

    print("alpha scope artifacts validated")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
