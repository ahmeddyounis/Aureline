#!/usr/bin/env python3
"""Validate the external alpha design-partner intake and feedback loop."""

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


DEFAULT_GUIDE_REL = "docs/alpha/design_partner_guide.md"
DEFAULT_INTAKE_REL = "artifacts/milestones/m2/design_partner_intake_packet.md"
DEFAULT_TASK_PACK_REL = "artifacts/milestones/m2/design_partner_task_pack.md"
DEFAULT_TAXONOMY_REL = "artifacts/feedback/design_partner_feedback_taxonomy.yaml"
DEFAULT_KNOWN_LIMITS_REL = "artifacts/feedback/external_alpha_known_limits.md"
DEFAULT_MATRIX_REL = "artifacts/milestones/m2/alpha_wedge_matrix.yaml"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_ISSUE_ROUTING_REL = "artifacts/governance/issue_routing.yaml"
DEFAULT_CASES_DIR_REL = "fixtures/feedback/external_alpha_cases"

REQUIRED_CATEGORIES = {
    "task_completion",
    "alpha_task_blocker",
    "scope_or_known_limit",
    "privacy_redaction",
    "support_export_or_diagnostics",
    "docs_or_copy_truth",
    "migration_import_gap",
    "benchmark_fixture_gap",
    "trust_security_boundary",
}

REQUIRED_SEVERITIES = {
    "external_alpha_blocker",
    "task_blocking",
    "scoped_workaround",
    "clarity_gap",
    "observation",
}

REQUIRED_PRIVACY_STATES = {
    "redaction_review_not_required",
    "redaction_review_required",
    "redaction_cleared",
    "raw_content_blocked",
    "partner_consent_required",
}

REQUIRED_PASS_FAIL_STATES = {
    "pass",
    "pass_with_scoped_workaround",
    "blocked_pending_packet",
    "known_limit",
    "fail",
}

REQUIRED_INTAKE_FIELDS = {
    "partner_role",
    "stack_profile",
    "privacy_posture",
    "privacy_review_state",
    "reproducible_task_scripts",
    "completion_criteria",
    "blocker_severity",
    "known_limit_refs",
    "rollback_posture",
}

REQUIRED_DOC_REFS = {
    DEFAULT_GUIDE_REL,
    DEFAULT_INTAKE_REL,
    DEFAULT_TASK_PACK_REL,
    DEFAULT_TAXONOMY_REL,
    DEFAULT_KNOWN_LIMITS_REL,
    DEFAULT_MATRIX_REL,
    DEFAULT_SCOREBOARD_REL,
    "artifacts/program/design_partner_intake_checklist.yaml",
}

REQUIRED_KNOWN_LIMIT_IDS = {
    "known_limit:external_alpha.scope.claimed_wedges_only",
    "known_limit:external_alpha.deployment.local_or_helper_only",
    "known_limit:external_alpha.notebook_handoff_only",
    "known_limit:external_alpha.browser_mobile_companion_out",
    "known_limit:external_alpha.support_export_redaction_required",
    "known_limit:external_alpha.migration_evidence_seeded",
    "known_limit:external_alpha.no_raw_partner_content",
}

EXPECTED_CASE_STATES = {
    "pass",
    "pass_with_redaction_block",
    "pass_with_known_limit_route",
}

PRIVATE_ROUTE_CLASSES = {
    "private_partner_channel",
    "private_support_channel",
    "private_security_channel",
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
    parser.add_argument("--guide", default=DEFAULT_GUIDE_REL)
    parser.add_argument("--intake", default=DEFAULT_INTAKE_REL)
    parser.add_argument("--task-pack", default=DEFAULT_TASK_PACK_REL)
    parser.add_argument("--taxonomy", default=DEFAULT_TAXONOMY_REL)
    parser.add_argument("--known-limits", default=DEFAULT_KNOWN_LIMITS_REL)
    parser.add_argument("--matrix", default=DEFAULT_MATRIX_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--issue-routing", default=DEFAULT_ISSUE_ROUTING_REL)
    parser.add_argument("--cases-dir", default=DEFAULT_CASES_DIR_REL)
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


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def read_text(path: Path, label: str, findings: list[Finding]) -> str:
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing",
                message=f"{path} does not exist",
                remediation="Create the referenced artifact.",
                ref=str(path),
            )
        )
        return ""
    try:
        return path.read_text(encoding="utf-8")
    except Exception as exc:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.read_failed",
                message=f"failed to read {path}: {exc}",
                remediation="Ensure the artifact is readable as UTF-8.",
                ref=str(path),
            )
        )
        return ""


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


def collect_matrix_refs(matrix: dict[str, Any]) -> dict[str, set[str]]:
    wedge_ids: set[str] = set()
    workflow_ids: set[str] = set()
    scoreboard_refs: set[str] = set()
    task_ids: set[str] = set()
    fixture_refs: set[str] = set()
    out_of_scope_refs: set[str] = set()

    for raw_fixture in ensure_list(matrix.get("protected_fixture_refs"), "matrix.protected_fixture_refs"):
        fixture = ensure_dict(raw_fixture, "matrix.protected_fixture_refs[]")
        fixture_refs.add(ensure_str(fixture.get("fixture_ref"), "matrix.protected_fixture_refs[].fixture_ref"))
        for row_ref in ensure_list(fixture.get("exercises_scoreboard_rows"), "matrix.protected_fixture_refs[].exercises_scoreboard_rows"):
            scoreboard_refs.add(str(row_ref))

    for raw_row in ensure_list(matrix.get("wedge_rows"), "matrix.wedge_rows"):
        row = ensure_dict(raw_row, "matrix.wedge_rows[]")
        wedge_ids.add(ensure_str(row.get("wedge_id"), "matrix.wedge_rows[].wedge_id"))
        for row_ref in ensure_list(row.get("required_scoreboard_rows"), "matrix.wedge_rows[].required_scoreboard_rows"):
            scoreboard_refs.add(str(row_ref))
        for raw_workflow in ensure_list(row.get("protected_workflows"), "matrix.wedge_rows[].protected_workflows"):
            workflow = ensure_dict(raw_workflow, "matrix.wedge_rows[].protected_workflows[]")
            workflow_id = ensure_str(workflow.get("workflow_id"), "matrix.wedge_rows[].protected_workflows[].workflow_id")
            workflow_ids.add(workflow_id)
            task_ids.add(workflow_id.replace("workflow.alpha.", "task.alpha.", 1))
            scoreboard_refs.add(ensure_str(workflow.get("scoreboard_row_ref"), "matrix.wedge_rows[].protected_workflows[].scoreboard_row_ref"))
        for raw_limit in ensure_list(row.get("explicit_out_of_scope"), "matrix.wedge_rows[].explicit_out_of_scope"):
            limit = ensure_dict(raw_limit, "matrix.wedge_rows[].explicit_out_of_scope[]")
            if "out_of_scope_id" in limit:
                out_of_scope_refs.add(ensure_str(limit.get("out_of_scope_id"), "matrix.wedge_rows[].explicit_out_of_scope[].out_of_scope_id"))

    for raw_row in ensure_list(matrix.get("non_claimed_rows"), "matrix.non_claimed_rows"):
        row = ensure_dict(raw_row, "matrix.non_claimed_rows[]")
        out_of_scope_refs.add(ensure_str(row.get("row_id"), "matrix.non_claimed_rows[].row_id"))

    return {
        "wedge_ids": wedge_ids,
        "workflow_ids": workflow_ids,
        "scoreboard_refs": scoreboard_refs,
        "task_ids": task_ids,
        "fixture_refs": fixture_refs,
        "out_of_scope_refs": out_of_scope_refs,
    }


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    rows = ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")
    return {
        ensure_str(ensure_dict(row, "scoreboard.scoreboard_rows[]").get("row_id"), "scoreboard.scoreboard_rows[].row_id")
        for row in rows
    }


def collect_route_classes(issue_routing: dict[str, Any]) -> dict[str, str]:
    route_classes: dict[str, str] = {}
    for raw_route in ensure_list(issue_routing.get("route_classes"), "issue_routing.route_classes"):
        route = ensure_dict(raw_route, "issue_routing.route_classes[]")
        route_id = ensure_str(route.get("id"), "issue_routing.route_classes[].id")
        route_classes[route_id] = ensure_str(route.get("kind"), "issue_routing.route_classes[].kind")
    return route_classes


def validate_text_artifact(
    text: str,
    path: str,
    required_refs: set[str],
    findings: list[Finding],
    required_terms: set[str] | None = None,
) -> None:
    for ref in sorted(required_refs):
        if ref == path:
            continue
        if ref not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="markdown.required_ref_missing",
                    message=f"{path} is missing required canonical ref: {ref}",
                    remediation="Add the canonical ref so the packet can be reviewed without side channels.",
                    ref=ref,
                )
            )
    for term in sorted(required_terms or set()):
        if term not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="markdown.required_term_missing",
                    message=f"{path} is missing required term: {term}",
                    remediation="Add the term to the packet so the validator can prove the acceptance state.",
                    ref=term,
                )
            )


def validate_known_limits(text: str, findings: list[Finding]) -> set[str]:
    known_limit_ids = set(re.findall(r"known_limit:external_alpha\.[A-Za-z0-9_.:-]+", text))
    missing = REQUIRED_KNOWN_LIMIT_IDS - known_limit_ids
    for ref in sorted(missing):
        findings.append(
            Finding(
                severity="error",
                check_id="known_limits.missing_required_id",
                message=f"known-limits packet is missing required id: {ref}",
                remediation="Add the known-limit id so feedback categories can route to it.",
                ref=ref,
            )
        )
    return known_limit_ids


def validate_taxonomy(
    repo_root: Path,
    taxonomy: dict[str, Any],
    route_classes: dict[str, str],
    scoreboard_ids: set[str],
    known_limit_ids: set[str],
    findings: list[Finding],
) -> dict[str, dict[str, Any]]:
    validate_header(taxonomy, "taxonomy", findings)
    ensure_str(taxonomy.get("taxonomy_id"), "taxonomy.taxonomy_id")

    entrypoint = ensure_str(taxonomy.get("human_entrypoint_ref"), "taxonomy.human_entrypoint_ref")
    if not artifact_ref_exists(repo_root, entrypoint):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.human_entrypoint_ref.missing",
                message=f"human_entrypoint_ref does not exist: {entrypoint}",
                remediation="Fix the ref so partners and reviewers have a stable guide.",
                ref=entrypoint,
            )
        )

    inputs = ensure_dict(taxonomy.get("inputs"), "taxonomy.inputs")
    for key, raw_ref in inputs.items():
        ref = ensure_str(raw_ref, f"taxonomy.inputs.{key}")
        if not artifact_ref_exists(repo_root, ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.inputs.missing_ref",
                    message=f"taxonomy input ref does not exist: {ref}",
                    remediation="Fix the ref or seed the referenced artifact.",
                    ref=ref,
                    details={"input_key": str(key)},
                )
            )

    severities = set(ensure_list(taxonomy.get("severity_vocabulary"), "taxonomy.severity_vocabulary"))
    for missing in sorted(REQUIRED_SEVERITIES - severities):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.severity_vocabulary.missing",
                message=f"severity_vocabulary is missing {missing}",
                remediation="Add the required severity so partner reports share one blocker scale.",
                ref=missing,
            )
        )

    privacy_states = set(ensure_list(taxonomy.get("privacy_review_state_vocabulary"), "taxonomy.privacy_review_state_vocabulary"))
    for missing in sorted(REQUIRED_PRIVACY_STATES - privacy_states):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.privacy_state_vocabulary.missing",
                message=f"privacy_review_state_vocabulary is missing {missing}",
                remediation="Add the required privacy state so raw artifacts cannot bypass review.",
                ref=missing,
            )
        )

    pass_fail_states = set(ensure_list(taxonomy.get("pass_fail_state_vocabulary"), "taxonomy.pass_fail_state_vocabulary"))
    for missing in sorted(REQUIRED_PASS_FAIL_STATES - pass_fail_states):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.pass_fail_state_vocabulary.missing",
                message=f"pass_fail_state_vocabulary is missing {missing}",
                remediation="Add the required task result state so task pack outcomes remain comparable.",
                ref=missing,
            )
        )

    categories: dict[str, dict[str, Any]] = {}
    for idx, raw_category in enumerate(ensure_list(taxonomy.get("feedback_categories"), "taxonomy.feedback_categories")):
        category = ensure_dict(raw_category, f"taxonomy.feedback_categories[{idx}]")
        category_id = ensure_str(category.get("category_id"), f"taxonomy.feedback_categories[{idx}].category_id")
        categories[category_id] = category
        route_class = ensure_str(category.get("default_route_class"), f"taxonomy.feedback_categories[{idx}].default_route_class")
        if route_class not in route_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.feedback_categories.unknown_route_class",
                    message=f"{category_id} cites unknown route class {route_class}",
                    remediation="Use a route_class from artifacts/governance/issue_routing.yaml.",
                    ref=route_class,
                )
            )
        severity_floor = ensure_str(category.get("severity_floor"), f"taxonomy.feedback_categories[{idx}].severity_floor")
        if severity_floor not in severities:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.feedback_categories.unknown_severity_floor",
                    message=f"{category_id} cites unknown severity floor {severity_floor}",
                    remediation="Use a value from severity_vocabulary.",
                    ref=severity_floor,
                )
            )
        for row_ref in ensure_list(category.get("scoreboard_row_refs"), f"taxonomy.feedback_categories[{idx}].scoreboard_row_refs"):
            if row_ref not in scoreboard_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="taxonomy.feedback_categories.unknown_scoreboard_row",
                        message=f"{category_id} cites unknown scoreboard row {row_ref}",
                        remediation="Use a row from the alpha exit-gate scoreboard.",
                        ref=str(row_ref),
                    )
                )

    for missing in sorted(REQUIRED_CATEGORIES - set(categories)):
        findings.append(
            Finding(
                severity="error",
                check_id="taxonomy.feedback_categories.missing",
                message=f"feedback_categories is missing {missing}",
                remediation="Add the category so partner reports do not need ad hoc labels.",
                ref=missing,
            )
        )

    for idx, raw_route in enumerate(ensure_list(taxonomy.get("known_limit_routing"), "taxonomy.known_limit_routing")):
        route = ensure_dict(raw_route, f"taxonomy.known_limit_routing[{idx}]")
        category_id = ensure_str(route.get("category_id"), f"taxonomy.known_limit_routing[{idx}].category_id")
        if category_id not in categories:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.known_limit_routing.unknown_category",
                    message=f"known_limit_routing cites unknown category {category_id}",
                    remediation="Add the category or fix the routing row.",
                    ref=category_id,
                )
            )
        refs = ensure_list(route.get("required_known_limit_refs"), f"taxonomy.known_limit_routing[{idx}].required_known_limit_refs")
        if not refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="taxonomy.known_limit_routing.empty_refs",
                    message=f"known_limit_routing for {category_id} must cite at least one known limit",
                    remediation="Add one or more known_limit:external_alpha.* refs.",
                    ref=category_id,
                )
            )
        for ref in refs:
            if ref not in known_limit_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="taxonomy.known_limit_routing.unknown_ref",
                        message=f"known_limit_routing cites unknown known-limit ref {ref}",
                        remediation="Add the id to the known-limits packet or fix the taxonomy ref.",
                        ref=str(ref),
                    )
                )

    return categories


def validate_task_pack(
    text: str,
    matrix_refs: dict[str, set[str]],
    findings: list[Finding],
) -> None:
    for workflow_id in sorted(matrix_refs["workflow_ids"]):
        if workflow_id not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="task_pack.workflow_missing",
                    message=f"task pack is missing matrix workflow {workflow_id}",
                    remediation="Add a task row for every protected workflow in the alpha matrix.",
                    ref=workflow_id,
                )
            )
    for task_id in sorted(matrix_refs["task_ids"]):
        if task_id not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="task_pack.task_id_missing",
                    message=f"task pack is missing expected task id {task_id}",
                    remediation="Use task ids derived from the protected workflow ids.",
                    ref=task_id,
                )
            )
    for fixture_ref in sorted(matrix_refs["fixture_refs"]):
        if fixture_ref not in text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="task_pack.fixture_ref_missing",
                    message=f"task pack is missing required fixture ref {fixture_ref}",
                    remediation="Name the fixture required to run the partner task.",
                    ref=fixture_ref,
                )
            )


def validate_case_files(
    repo_root: Path,
    cases_dir: Path,
    categories: dict[str, dict[str, Any]],
    route_classes: dict[str, str],
    matrix_refs: dict[str, set[str]],
    scoreboard_ids: set[str],
    known_limit_ids: set[str],
    taxonomy: dict[str, Any],
    findings: list[Finding],
) -> None:
    if not cases_dir.exists():
        findings.append(
            Finding(
                severity="error",
                check_id="cases_dir.missing",
                message=f"case directory does not exist: {cases_dir}",
                remediation="Seed protected feedback cases for the design-partner proof path.",
                ref=str(cases_dir),
            )
        )
        return

    severities = set(ensure_list(taxonomy.get("severity_vocabulary"), "taxonomy.severity_vocabulary"))
    privacy_states = set(ensure_list(taxonomy.get("privacy_review_state_vocabulary"), "taxonomy.privacy_review_state_vocabulary"))
    pass_fail_states = set(ensure_list(taxonomy.get("pass_fail_state_vocabulary"), "taxonomy.pass_fail_state_vocabulary"))
    covered_categories: set[str] = set()
    covered_states: set[str] = set()
    covered_wedges: set[str] = set()

    case_paths = sorted(cases_dir.glob("*.yaml"))
    if not case_paths:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.empty",
                message="no protected feedback case YAML files found",
                remediation="Add metadata-only cases that exercise task, redaction, and known-limit routing.",
                ref=str(cases_dir),
            )
        )
        return

    for case_path in case_paths:
        case = ensure_dict(render_yaml_as_json(case_path), f"case:{case_path.name}")
        case_id = ensure_str(case.get("case_id"), f"{case_path.name}.case_id")
        category_id = ensure_str(case.get("category_id"), f"{case_path.name}.category_id")
        covered_categories.add(category_id)
        if category_id not in categories:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_category",
                    message=f"{case_id} cites unknown category {category_id}",
                    remediation="Use a category from the feedback taxonomy.",
                    ref=case_id,
                )
            )
            continue

        category = categories[category_id]
        route_class = ensure_str(case.get("default_route_class"), f"{case_path.name}.default_route_class")
        if route_class not in route_classes:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_route_class",
                    message=f"{case_id} cites unknown route class {route_class}",
                    remediation="Use a route class from the issue routing matrix.",
                    ref=case_id,
                )
            )
        expected_route = ensure_str(category.get("default_route_class"), f"category:{category_id}.default_route_class")
        if route_class != expected_route:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.route_class_mismatch",
                    message=f"{case_id} route {route_class} does not match taxonomy default {expected_route}",
                    remediation="Align the fixture route with the taxonomy or update the taxonomy intentionally.",
                    ref=case_id,
                )
            )

        wedge_ref = ensure_str(case.get("wedge_ref"), f"{case_path.name}.wedge_ref")
        covered_wedges.add(wedge_ref)
        if wedge_ref not in matrix_refs["wedge_ids"]:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_wedge_ref",
                    message=f"{case_id} cites unknown wedge {wedge_ref}",
                    remediation="Use a claimed alpha wedge from the matrix.",
                    ref=case_id,
                )
            )

        workflow_id = ensure_str(case.get("workflow_id"), f"{case_path.name}.workflow_id")
        if workflow_id not in matrix_refs["workflow_ids"]:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_workflow_id",
                    message=f"{case_id} cites unknown workflow {workflow_id}",
                    remediation="Use a protected workflow from the alpha matrix.",
                    ref=case_id,
                )
            )

        task_script_id = ensure_str(case.get("task_script_id"), f"{case_path.name}.task_script_id")
        if task_script_id not in matrix_refs["task_ids"]:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_task_script_id",
                    message=f"{case_id} cites unknown task script {task_script_id}",
                    remediation="Use a task id from the design-partner task pack.",
                    ref=case_id,
                )
            )

        scoreboard_row_ref = ensure_str(case.get("scoreboard_row_ref"), f"{case_path.name}.scoreboard_row_ref")
        if scoreboard_row_ref not in scoreboard_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_scoreboard_row_ref",
                    message=f"{case_id} cites unknown scoreboard row {scoreboard_row_ref}",
                    remediation="Use a row from the alpha exit-gate scoreboard.",
                    ref=case_id,
                )
            )

        fixture_ref = ensure_str(case.get("fixture_ref"), f"{case_path.name}.fixture_ref")
        if not artifact_ref_exists(repo_root, fixture_ref):
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.fixture_missing",
                    message=f"{case_id} fixture does not exist: {fixture_ref}",
                    remediation="Fix the fixture path or seed the referenced fixture.",
                    ref=case_id,
                )
            )

        blocker_severity = ensure_str(case.get("blocker_severity"), f"{case_path.name}.blocker_severity")
        if blocker_severity not in severities:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_blocker_severity",
                    message=f"{case_id} cites unknown blocker severity {blocker_severity}",
                    remediation="Use a severity from the feedback taxonomy.",
                    ref=case_id,
                )
            )

        privacy_state = ensure_str(case.get("privacy_review_state"), f"{case_path.name}.privacy_review_state")
        if privacy_state not in privacy_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_privacy_review_state",
                    message=f"{case_id} cites unknown privacy state {privacy_state}",
                    remediation="Use a privacy_review_state from the feedback taxonomy.",
                    ref=case_id,
                )
            )
        if privacy_state in {"redaction_review_required", "raw_content_blocked", "partner_consent_required"} and route_class not in PRIVATE_ROUTE_CLASSES:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.private_privacy_state_public_route",
                    message=f"{case_id} has privacy state {privacy_state} but route {route_class} is not private",
                    remediation="Route sensitive reports through a private partner, support, or security lane.",
                    ref=case_id,
                )
            )

        pass_fail_state = ensure_str(case.get("pass_fail_state"), f"{case_path.name}.pass_fail_state")
        if pass_fail_state not in pass_fail_states:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.unknown_pass_fail_state",
                    message=f"{case_id} cites unknown pass/fail state {pass_fail_state}",
                    remediation="Use a pass_fail_state from the feedback taxonomy.",
                    ref=case_id,
                )
            )

        known_refs = ensure_list(case.get("known_limit_refs"), f"{case_path.name}.known_limit_refs")
        requires_known_ref = bool(category.get("requires_known_limit_ref"))
        if requires_known_ref and not known_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.required_known_limit_missing",
                    message=f"{case_id} category {category_id} requires known_limit_refs",
                    remediation="Add one or more known_limit:external_alpha.* refs.",
                    ref=case_id,
                )
            )
        for ref in known_refs:
            if ref not in known_limit_ids:
                findings.append(
                    Finding(
                        severity="error",
                        check_id="cases.unknown_known_limit_ref",
                        message=f"{case_id} cites unknown known-limit ref {ref}",
                        remediation="Use a known-limit id from the external alpha known-limits packet.",
                        ref=case_id,
                    )
                )

        evidence_refs = ensure_list(case.get("evidence_refs"), f"{case_path.name}.evidence_refs")
        if not evidence_refs:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.evidence_refs.empty",
                    message=f"{case_id} must cite at least one redaction-safe evidence ref",
                    remediation="Add a metadata, packet, capture, or redacted evidence ref.",
                    ref=case_id,
                )
            )

        expected_state = ensure_str(case.get("expected_validator_state"), f"{case_path.name}.expected_validator_state")
        covered_states.add(expected_state)

    for missing in sorted({"task_completion", "privacy_redaction", "scope_or_known_limit"} - covered_categories):
        findings.append(
            Finding(
                severity="error",
                check_id="cases.coverage.category_missing",
                message=f"protected cases do not cover category {missing}",
                remediation="Add a case exercising this required acceptance path.",
                ref=missing,
            )
        )

    for missing in sorted(EXPECTED_CASE_STATES - covered_states):
        findings.append(
            Finding(
                severity="error",
                check_id="cases.coverage.state_missing",
                message=f"protected cases do not cover expected validator state {missing}",
                remediation="Add a case exercising the missing validator state.",
                ref=missing,
            )
        )

    for missing in sorted({"alpha_wedge:typescript_javascript", "alpha_wedge:python"} - covered_wedges):
        findings.append(
            Finding(
                severity="error",
                check_id="cases.coverage.wedge_missing",
                message=f"protected cases do not cover wedge {missing}",
                remediation="Add a case for both claimed alpha wedges.",
                ref=missing,
            )
        )


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    guide_path = repo_root / args.guide
    intake_path = repo_root / args.intake
    task_pack_path = repo_root / args.task_pack
    taxonomy_path = repo_root / args.taxonomy
    known_limits_path = repo_root / args.known_limits
    matrix_path = repo_root / args.matrix
    scoreboard_path = repo_root / args.scoreboard
    issue_routing_path = repo_root / args.issue_routing
    cases_dir = repo_root / args.cases_dir

    guide_text = read_text(guide_path, "guide", findings)
    intake_text = read_text(intake_path, "intake", findings)
    task_pack_text = read_text(task_pack_path, "task_pack", findings)
    known_limits_text = read_text(known_limits_path, "known_limits", findings)

    taxonomy = ensure_dict(render_yaml_as_json(taxonomy_path), "taxonomy")
    matrix = ensure_dict(render_yaml_as_json(matrix_path), "matrix")
    scoreboard = ensure_dict(render_yaml_as_json(scoreboard_path), "scoreboard")
    issue_routing = ensure_dict(render_yaml_as_json(issue_routing_path), "issue_routing")

    matrix_refs = collect_matrix_refs(matrix)
    scoreboard_ids = collect_scoreboard_ids(scoreboard)
    route_classes = collect_route_classes(issue_routing)
    known_limit_ids = validate_known_limits(known_limits_text, findings)

    validate_text_artifact(guide_text, args.guide, REQUIRED_DOC_REFS, findings)
    validate_text_artifact(intake_text, args.intake, REQUIRED_DOC_REFS, findings, REQUIRED_INTAKE_FIELDS)
    validate_text_artifact(task_pack_text, args.task_pack, REQUIRED_DOC_REFS, findings)
    validate_text_artifact(known_limits_text, args.known_limits, REQUIRED_DOC_REFS, findings)
    validate_task_pack(task_pack_text, matrix_refs, findings)

    categories = validate_taxonomy(
        repo_root,
        taxonomy,
        route_classes,
        scoreboard_ids,
        known_limit_ids,
        findings,
    )
    validate_case_files(
        repo_root,
        cases_dir,
        categories,
        route_classes,
        matrix_refs,
        scoreboard_ids,
        known_limit_ids,
        taxonomy,
        findings,
    )

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    report = {
        "schema_version": 1,
        "generated_at": dt.datetime.now(dt.timezone.utc).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "status": "pass" if not errors else "fail",
        "summary": {
            "errors": len(errors),
            "warnings": len(warnings),
            "checked_categories": sorted(categories),
            "checked_known_limit_ids": sorted(known_limit_ids),
            "checked_task_ids": sorted(matrix_refs["task_ids"]),
        },
        "findings": [finding.as_report() for finding in findings],
    }

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    if errors:
        for finding in errors:
            print(f"ERROR {finding.check_id}: {finding.message}", file=sys.stderr)
            print(f"  remediation: {finding.remediation}", file=sys.stderr)
            if finding.ref:
                print(f"  ref: {finding.ref}", file=sys.stderr)
        return 1

    if warnings:
        for finding in warnings:
            print(f"WARNING {finding.check_id}: {finding.message}", file=sys.stderr)

    print("external alpha design-partner intake validation passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

