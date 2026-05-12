#!/usr/bin/env python3
"""Validate and render the external alpha reference-workspace dry run."""

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


DEFAULT_CASES_REL = "fixtures/reference_workspaces/m2/dry_run_rehearsal_cases.yaml"
DEFAULT_DRY_RUN_REL = "artifacts/milestones/m2/reference_workspace_dry_run.md"
DEFAULT_REHEARSAL_REL = "artifacts/benchmarks/m2_publication_rehearsal.md"
DEFAULT_KNOWN_LIMITS_REL = "artifacts/milestones/m2/known_limits_alpha.yaml"
DEFAULT_KNOWN_LIMITS_MARKDOWN_REL = "artifacts/feedback/external_alpha_known_limits.md"
DEFAULT_FIXTURE_REGISTER_REL = "artifacts/benchmarks/m2_fixture_register.yaml"
DEFAULT_SCOREBOARD_REL = "artifacts/milestones/m2/exit_gate_scoreboard.yaml"
DEFAULT_TSJS_BUNDLE_REL = "artifacts/bundles/tsjs_launch_bundle_alpha.yaml"
DEFAULT_PYTHON_BUNDLE_REL = "artifacts/bundles/python_launch_bundle_alpha.yaml"
DEFAULT_PUBLICATION_CHECKLIST_REL = "artifacts/bench/publication_rehearsal_checklist.yaml"

REQUIRED_REFERENCE_WORKSPACES = {
    "refws.ts_web_app_archetype_seed": {
        "case_id": "dry_run_case:external_alpha.ts_web_app_reference",
        "wedge_ref": "alpha_wedge:typescript_javascript",
        "fixture_register_row_ref": "fixture_register:external_alpha.ts_web_app_reference",
        "workflow_bundle_ref": "launch_bundle:typescript_web_app.seed",
        "bundle_manifest_ref": DEFAULT_TSJS_BUNDLE_REL,
    },
    "refws.python_data_app_archetype_seed": {
        "case_id": "dry_run_case:external_alpha.python_service_data_reference",
        "wedge_ref": "alpha_wedge:python",
        "fixture_register_row_ref": "fixture_register:external_alpha.python_service_data_reference",
        "workflow_bundle_ref": "launch_bundle:python_service_or_data_app.seed",
        "bundle_manifest_ref": DEFAULT_PYTHON_BUNDLE_REL,
    },
}

REQUIRED_ACCEPTANCE_STATES = {
    "ts_js_reference_workspace_dry_run_completed",
    "python_reference_workspace_dry_run_completed",
    "current_known_limits_attached",
    "fixture_register_bundle_scoreboard_contract_used",
    "methodology_only_publication_rehearsal",
}

REQUIRED_LIMIT_IDS = {
    "known_limit:external_alpha.scope.claimed_wedges_only",
    "known_limit:external_alpha.deployment.local_or_helper_only",
    "known_limit:external_alpha.support_export_redaction_required",
    "known_limit:external_alpha.no_raw_partner_content",
    "known_limit:external_alpha.launch_bundle_seed_not_certified",
    "known_limit:external_alpha.reference_workspace_dry_run_synthetic_only",
    "known_limit:external_alpha.publication_rehearsal_methodology_only",
}

PATH_LIKE_SUFFIXES = (".yaml", ".yml", ".json", ".md", ".toml", ".rs", ".py")
ID_PREFIXES = (
    "alpha_wedge:",
    "case:",
    "corpus.",
    "dry_run_case:",
    "fixture_register:",
    "known_limit:",
    "launch_bundle:",
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
    parser.add_argument("--cases", default=DEFAULT_CASES_REL)
    parser.add_argument("--dry-run", default=DEFAULT_DRY_RUN_REL)
    parser.add_argument("--publication-rehearsal", default=DEFAULT_REHEARSAL_REL)
    parser.add_argument("--known-limits", default=DEFAULT_KNOWN_LIMITS_REL)
    parser.add_argument("--known-limits-markdown", default=DEFAULT_KNOWN_LIMITS_MARKDOWN_REL)
    parser.add_argument("--fixture-register", default=DEFAULT_FIXTURE_REGISTER_REL)
    parser.add_argument("--scoreboard", default=DEFAULT_SCOREBOARD_REL)
    parser.add_argument("--tsjs-bundle", default=DEFAULT_TSJS_BUNDLE_REL)
    parser.add_argument("--python-bundle", default=DEFAULT_PYTHON_BUNDLE_REL)
    parser.add_argument("--publication-checklist", default=DEFAULT_PUBLICATION_CHECKLIST_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-publication-summary",
        action="store_true",
        help="Print the export-safe dry-run and publication-rehearsal summary.",
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


def parse_iso_date(value: str, label: str, findings: list[Finding], ref: str) -> None:
    try:
        dt.date.fromisoformat(value)
    except ValueError:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.invalid_date",
                message=f"{label} must be a YYYY-MM-DD date, got {value!r}",
                remediation="Use an ISO date without time.",
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


def read_text(repo_root: Path, rel: str, label: str, findings: list[Finding]) -> str:
    path = repo_root / rel
    if not path.exists():
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing",
                message=f"{rel} does not exist",
                remediation="Seed the referenced artifact.",
                ref=rel,
            )
        )
        return ""
    return path.read_text(encoding="utf-8")


def validate_header(payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    schema_version = ensure_int(payload.get("schema_version"), f"{label}.schema_version")
    if schema_version != 1:
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.schema_version.unsupported",
                message=f"{label}.schema_version must be 1, got {schema_version}",
                remediation="Update the validator in the same change that bumps the schema.",
            )
        )
    if "as_of" in payload:
        parse_iso_date(ensure_str(payload.get("as_of"), f"{label}.as_of"), f"{label}.as_of", findings, label)
    if "owner" in payload:
        ensure_str(payload.get("owner"), f"{label}.owner")


def collect_scoreboard_ids(scoreboard: dict[str, Any]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(scoreboard.get("scoreboard_rows"), "scoreboard.scoreboard_rows")):
        row = ensure_dict(raw_row, f"scoreboard.scoreboard_rows[{idx}]")
        ids.add(ensure_str(row.get("row_id"), f"scoreboard.scoreboard_rows[{idx}].row_id"))
    return ids


def collect_fixture_register_rows(register: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for idx, raw_row in enumerate(ensure_list(register.get("reference_workspaces"), "fixture_register.reference_workspaces")):
        row = ensure_dict(raw_row, f"fixture_register.reference_workspaces[{idx}]")
        rows[ensure_str(row.get("register_row_id"), f"fixture_register.reference_workspaces[{idx}].register_row_id")] = row
    return rows


def collect_bundle_ids(*bundles: dict[str, Any]) -> dict[str, dict[str, Any]]:
    rows: dict[str, dict[str, Any]] = {}
    for bundle in bundles:
        rows[ensure_str(bundle.get("bundle_id"), "bundle.bundle_id")] = bundle
    return rows


def collect_workflow_packet_refs(repo_root: Path, packet_ref: str) -> set[str]:
    packet = ensure_dict(render_yaml_as_json(repo_root / strip_fragment(packet_ref)), f"fixture_packet[{packet_ref}]")
    workflow_ids: set[str] = set()
    for idx, raw_workflow in enumerate(ensure_list(packet.get("protected_workflows"), "fixture_packet.protected_workflows")):
        workflow = ensure_dict(raw_workflow, f"fixture_packet.protected_workflows[{idx}]")
        workflow_ids.add(ensure_str(workflow.get("workflow_id"), f"fixture_packet.protected_workflows[{idx}].workflow_id"))
    return workflow_ids


def collect_known_limits(packet: dict[str, Any], markdown: str, findings: list[Finding]) -> set[str]:
    ids: set[str] = set()
    for idx, raw_limit in enumerate(ensure_list(packet.get("known_limits"), "known_limits.known_limits")):
        row = ensure_dict(raw_limit, f"known_limits.known_limits[{idx}]")
        limit_id = ensure_str(row.get("known_limit_id"), f"known_limits.known_limits[{idx}].known_limit_id")
        ids.add(limit_id)
        ensure_str(row.get("limitation_class"), f"{limit_id}.limitation_class")
        ensure_str(row.get("severity"), f"{limit_id}.severity")
        ensure_str(row.get("note_state"), f"{limit_id}.note_state")
        ensure_str(row.get("review_rubric_class"), f"{limit_id}.review_rubric_class")
        ensure_str(row.get("partner_summary"), f"{limit_id}.partner_summary")
        if not ensure_list(row.get("explicit_exclusions"), f"{limit_id}.explicit_exclusions"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="known_limits.explicit_exclusions.empty",
                    message=f"{limit_id} must name at least one explicit exclusion",
                    remediation="Add explicit_exclusions so publication packets know what is narrowed.",
                    ref=limit_id,
                )
            )
        if not ensure_list(row.get("mandatory_publication_destinations"), f"{limit_id}.mandatory_publication_destinations"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="known_limits.destinations.empty",
                    message=f"{limit_id} must name mandatory publication destinations",
                    remediation="Add the docs, support, release, or public-proof destinations this note controls.",
                    ref=limit_id,
                )
            )
        freshness = ensure_dict(row.get("freshness"), f"{limit_id}.freshness")
        ensure_str(freshness.get("captured_at"), f"{limit_id}.freshness.captured_at")
        ensure_str(freshness.get("stale_after"), f"{limit_id}.freshness.stale_after")
        ensure_str(freshness.get("proof_class"), f"{limit_id}.freshness.proof_class")

    missing_required = REQUIRED_LIMIT_IDS - ids
    if missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="known_limits.missing_required",
                message="known-limits packet is missing required external alpha limits",
                remediation="Add the current scope, redaction, launch-bundle, dry-run, and rehearsal limits.",
                details={"missing": sorted(missing_required)},
            )
        )

    markdown_ids = set(re.findall(r"known_limit:external_alpha\.[A-Za-z0-9_.:-]+", markdown))
    missing_from_markdown = ids - markdown_ids
    if missing_from_markdown:
        findings.append(
            Finding(
                severity="error",
                check_id="known_limits.markdown_parity",
                message="machine known-limit ids must appear in the markdown companion",
                remediation="Add every machine known_limit_id to artifacts/feedback/external_alpha_known_limits.md.",
                details={"missing": sorted(missing_from_markdown)},
            )
        )
    return ids


def validate_source_refs(repo_root: Path, payload: dict[str, Any], label: str, findings: list[Finding]) -> None:
    for key, raw_ref in ensure_dict(payload.get("source_contract_refs"), f"{label}.source_contract_refs").items():
        validate_path_ref(repo_root, ensure_str(raw_ref, f"{label}.source_contract_refs.{key}"), f"{label}.source_contract_refs.{key}", findings)


def validate_case(
    repo_root: Path,
    case: dict[str, Any],
    fixture_rows: dict[str, dict[str, Any]],
    bundle_rows: dict[str, dict[str, Any]],
    scoreboard_ids: set[str],
    known_limit_ids: set[str],
    result_vocabulary: set[str],
    rehearsal_vocabulary: set[str],
    dry_run_text: str,
    rehearsal_text: str,
    findings: list[Finding],
) -> str:
    case_id = ensure_str(case.get("case_id"), "case.case_id")
    reference_workspace_id = ensure_str(case.get("reference_workspace_id"), f"{case_id}.reference_workspace_id")
    expected = REQUIRED_REFERENCE_WORKSPACES.get(reference_workspace_id)
    if expected is None:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.unknown_reference_workspace",
                message=f"{case_id} uses an unexpected reference workspace id: {reference_workspace_id}",
                remediation="Use the TypeScript / JavaScript or Python external alpha reference workspace ids.",
                ref=case_id,
            )
        )
        return reference_workspace_id

    for field_name, expected_value in expected.items():
        actual = ensure_str(case.get(field_name), f"{case_id}.{field_name}")
        if actual != expected_value:
            findings.append(
                Finding(
                    severity="error",
                    check_id=f"cases.{field_name}.mismatch",
                    message=f"{case_id} has {field_name}={actual}, expected {expected_value}",
                    remediation="Bind the case to the canonical fixture register, bundle, and wedge rows.",
                    ref=case_id,
                )
            )

    fixture_row_ref = ensure_str(case.get("fixture_register_row_ref"), f"{case_id}.fixture_register_row_ref")
    fixture_row = fixture_rows.get(fixture_row_ref)
    if fixture_row is None:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.fixture_register_row.unknown",
                message=f"{case_id} cites unknown fixture register row {fixture_row_ref}",
                remediation="Use a register_row_id from artifacts/benchmarks/m2_fixture_register.yaml.",
                ref=case_id,
            )
        )
        return reference_workspace_id

    if ensure_str(fixture_row.get("reference_workspace_id"), f"{fixture_row_ref}.reference_workspace_id") != reference_workspace_id:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.fixture_register_row.workspace_mismatch",
                message=f"{case_id} does not match fixture register reference workspace id",
                remediation="Use the same reference_workspace_id as the fixture register row.",
                ref=case_id,
            )
        )

    bundle_ref = ensure_str(case.get("workflow_bundle_ref"), f"{case_id}.workflow_bundle_ref")
    if bundle_ref not in bundle_rows:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.bundle_ref.unknown",
                message=f"{case_id} cites unknown bundle {bundle_ref}",
                remediation="Use the checked external alpha launch bundle manifests.",
                ref=case_id,
            )
        )

    case_corpus_refs = set(ensure_list(case.get("corpus_refs"), f"{case_id}.corpus_refs"))
    register_corpus_refs = set(ensure_list(fixture_row.get("corpus_refs"), f"{fixture_row_ref}.corpus_refs"))
    if not case_corpus_refs or not case_corpus_refs <= register_corpus_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.corpus_refs.not_from_register",
                message=f"{case_id} corpus refs must come from its fixture register row",
                remediation="Copy corpus refs from the fixture register instead of adding local ids.",
                ref=case_id,
                details={"unknown": sorted(case_corpus_refs - register_corpus_refs)},
            )
        )

    packet_ref = ensure_str(case.get("fixture_packet_ref"), f"{case_id}.fixture_packet_ref")
    validate_path_ref(repo_root, packet_ref, "cases.fixture_packet_ref", findings)
    packet_workflows = collect_workflow_packet_refs(repo_root, packet_ref) if artifact_ref_exists(repo_root, packet_ref) else set()
    case_workflows = set(ensure_list(case.get("protected_workflow_refs"), f"{case_id}.protected_workflow_refs"))
    register_workflows = set(ensure_list(fixture_row.get("protected_workflow_refs"), f"{fixture_row_ref}.protected_workflow_refs"))
    if not case_workflows or not case_workflows <= register_workflows or not case_workflows <= packet_workflows:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.protected_workflows.not_current",
                message=f"{case_id} protected workflows must be present in the fixture register and workflow packet",
                remediation="Use workflow ids from the current fixture register and workflow packet.",
                ref=case_id,
                details={
                    "not_in_register": sorted(case_workflows - register_workflows),
                    "not_in_packet": sorted(case_workflows - packet_workflows),
                },
            )
        )

    for row_ref in ensure_list(case.get("scoreboard_row_refs"), f"{case_id}.scoreboard_row_refs"):
        row_ref = ensure_str(row_ref, f"{case_id}.scoreboard_row_refs[]")
        if row_ref not in scoreboard_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.scoreboard_row_refs.unknown",
                    message=f"{case_id} cites unknown scoreboard row {row_ref}",
                    remediation="Use rows from the alpha exit-gate scoreboard.",
                    ref=case_id,
                )
            )

    result = ensure_str(case.get("dry_run_result"), f"{case_id}.dry_run_result")
    if result not in result_vocabulary:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.dry_run_result.invalid",
                message=f"{case_id} uses unknown dry run result {result}",
                remediation="Use a dry-run result from result_vocabulary.",
                ref=case_id,
            )
        )
    if result != "completed_with_known_limits":
        findings.append(
            Finding(
                severity="error",
                check_id="cases.dry_run_result.not_completed",
                message=f"{case_id} must complete with known limits",
                remediation="Set dry_run_result to completed_with_known_limits after attaching current known limits.",
                ref=case_id,
            )
        )

    rehearsal_result = ensure_str(case.get("publication_rehearsal_result"), f"{case_id}.publication_rehearsal_result")
    if rehearsal_result not in rehearsal_vocabulary:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.publication_rehearsal_result.invalid",
                message=f"{case_id} uses unknown publication rehearsal result {rehearsal_result}",
                remediation="Use a result from publication_rehearsal_result_vocabulary.",
                ref=case_id,
            )
        )
    if rehearsal_result == "publish_packet":
        findings.append(
            Finding(
                severity="error",
                check_id="cases.publication_rehearsal_result.overclaims",
                message=f"{case_id} cannot publish a claim-bearing packet from this rehearsal",
                remediation="Keep the rehearsal methodology-only until measured benchmark and support export proof exists.",
                ref=case_id,
            )
        )

    case_known_limits = set(ensure_list(case.get("known_limit_refs"), f"{case_id}.known_limit_refs"))
    missing_case_limits = REQUIRED_LIMIT_IDS - case_known_limits
    if missing_case_limits - {"known_limit:external_alpha.notebook_handoff_only"}:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.known_limit_refs.missing_required",
                message=f"{case_id} is missing required dry-run known limits",
                remediation="Attach scope, deployment, redaction, seed-status, synthetic-dry-run, and methodology-only known limits.",
                ref=case_id,
                details={"missing": sorted(missing_case_limits)},
            )
        )
    for known_limit in case_known_limits:
        if known_limit not in known_limit_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.known_limit_refs.unknown",
                    message=f"{case_id} cites unknown known limit {known_limit}",
                    remediation="Use known_limit_id values from artifacts/milestones/m2/known_limits_alpha.yaml.",
                    ref=case_id,
                )
            )

    for marker_text, label in ((case_id, "case id"), (reference_workspace_id, "reference workspace id")):
        if marker_text not in dry_run_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="dry_run_report.missing_case_marker",
                    message=f"dry-run report does not mention {label} {marker_text}",
                    remediation="Add the case id and reference workspace id to the dry-run report.",
                    ref=case_id,
                )
            )
    if bundle_ref not in rehearsal_text:
        findings.append(
            Finding(
                severity="error",
                check_id="publication_rehearsal.missing_bundle_marker",
                message=f"publication rehearsal does not mention bundle {bundle_ref}",
                remediation="Add each dry-run bundle to the rehearsal binding table.",
                ref=case_id,
            )
        )
    return reference_workspace_id


def validate_cases(
    repo_root: Path,
    cases: dict[str, Any],
    fixture_register: dict[str, Any],
    scoreboard: dict[str, Any],
    bundles_by_id: dict[str, dict[str, Any]],
    known_limit_ids: set[str],
    dry_run_text: str,
    rehearsal_text: str,
    findings: list[Finding],
) -> list[str]:
    validate_header(cases, "cases", findings)
    validate_source_refs(repo_root, cases, "cases", findings)
    result_vocabulary = set(ensure_list(cases.get("result_vocabulary"), "cases.result_vocabulary"))
    rehearsal_vocabulary = set(ensure_list(cases.get("publication_rehearsal_result_vocabulary"), "cases.publication_rehearsal_result_vocabulary"))
    required_acceptance = set(ensure_list(cases.get("required_acceptance_states"), "cases.required_acceptance_states"))
    missing_required_acceptance = REQUIRED_ACCEPTANCE_STATES - required_acceptance
    if missing_required_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.required_acceptance_states.missing",
                message="required_acceptance_states is missing required dry-run states",
                remediation="Add all acceptance states for TS/JS, Python, known limits, contracts, and methodology-only rehearsal.",
                details={"missing": sorted(missing_required_acceptance)},
            )
        )

    fixture_rows = collect_fixture_register_rows(fixture_register)
    scoreboard_ids = collect_scoreboard_ids(scoreboard)
    checked_workspace_ids: list[str] = []
    seen_case_ids: set[str] = set()
    acceptance_refs: set[str] = set()
    for idx, raw_case in enumerate(ensure_list(cases.get("cases"), "cases.cases")):
        case = ensure_dict(raw_case, f"cases.cases[{idx}]")
        case_id = ensure_str(case.get("case_id"), f"cases.cases[{idx}].case_id")
        if case_id in seen_case_ids:
            findings.append(
                Finding(
                    severity="error",
                    check_id="cases.duplicate",
                    message=f"duplicate dry-run case id {case_id}",
                    remediation="Use one row per dry-run case.",
                    ref=case_id,
                )
            )
        seen_case_ids.add(case_id)
        checked_workspace_ids.append(
            validate_case(
                repo_root=repo_root,
                case=case,
                fixture_rows=fixture_rows,
                bundle_rows=bundles_by_id,
                scoreboard_ids=scoreboard_ids,
                known_limit_ids=known_limit_ids,
                result_vocabulary=result_vocabulary,
                rehearsal_vocabulary=rehearsal_vocabulary,
                dry_run_text=dry_run_text,
                rehearsal_text=rehearsal_text,
                findings=findings,
            )
        )
        acceptance_refs.update(ensure_list(case.get("acceptance_state_refs"), f"{case_id}.acceptance_state_refs"))

    missing_workspaces = set(REQUIRED_REFERENCE_WORKSPACES) - set(checked_workspace_ids)
    if missing_workspaces:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.required_workspace_missing",
                message="dry-run cases must cover both required reference workspaces",
                remediation="Add one TypeScript / JavaScript and one Python dry-run case.",
                details={"missing": sorted(missing_workspaces)},
            )
        )

    missing_acceptance_refs = REQUIRED_ACCEPTANCE_STATES - acceptance_refs
    if missing_acceptance_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.acceptance_state_refs.missing",
                message="case acceptance_state_refs do not cover all required states",
                remediation="Attach every required acceptance state to at least one dry-run case.",
                details={"missing": sorted(missing_acceptance_refs)},
            )
        )

    coverage_states: set[str] = set()
    for idx, raw_row in enumerate(ensure_list(cases.get("acceptance_state_coverage"), "cases.acceptance_state_coverage")):
        row = ensure_dict(raw_row, f"cases.acceptance_state_coverage[{idx}]")
        coverage_states.add(ensure_str(row.get("exercises_state"), f"cases.acceptance_state_coverage[{idx}].exercises_state"))
        validate_path_ref(
            repo_root,
            ensure_str(row.get("fixture_ref"), f"cases.acceptance_state_coverage[{idx}].fixture_ref"),
            "cases.acceptance_state_coverage.fixture_ref",
            findings,
        )
        ensure_str(row.get("expected_validator_result"), f"cases.acceptance_state_coverage[{idx}].expected_validator_result")
    missing_coverage = REQUIRED_ACCEPTANCE_STATES - coverage_states
    if missing_coverage:
        findings.append(
            Finding(
                severity="error",
                check_id="cases.acceptance_state_coverage.missing",
                message="acceptance_state_coverage does not exercise all required dry-run states",
                remediation="Add acceptance coverage rows for the missing states.",
                details={"missing": sorted(missing_coverage)},
            )
        )
    return sorted(set(checked_workspace_ids))


def validate_publication_checklist(checklist: dict[str, Any], findings: list[Finding]) -> None:
    validate_header(checklist, "publication_checklist", findings)
    results = set(ensure_list(checklist.get("result_vocabulary"), "publication_checklist.result_vocabulary"))
    required = {"methodology_only", "narrow_claim_before_publish", "retest_pending", "blocked"}
    missing = required - results
    if missing:
        findings.append(
            Finding(
                severity="error",
                check_id="publication_checklist.result_vocabulary.missing",
                message="publication rehearsal checklist does not expose required result states",
                remediation="Keep methodology-only, narrow, retest, and blocked states available to dry runs.",
                details={"missing": sorted(missing)},
            )
        )


def validate_text_artifacts(dry_run_text: str, rehearsal_text: str, known_limits_text: str, findings: list[Finding]) -> None:
    required_dry_run_markers = {
        "completed_with_known_limits",
        "artifacts/milestones/m2/known_limits_alpha.yaml",
        "fixtures/reference_workspaces/m2/dry_run_rehearsal_cases.yaml",
        "ci/check_reference_workspace_dry_run.py",
    }
    missing_dry_run = sorted(marker for marker in required_dry_run_markers if marker not in dry_run_text)
    if missing_dry_run:
        findings.append(
            Finding(
                severity="error",
                check_id="dry_run_report.required_markers_missing",
                message="reference dry-run report is missing required markers",
                remediation="Add the completion state, known-limits packet, fixture manifest, and validator refs.",
                details={"missing": missing_dry_run},
            )
        )

    required_rehearsal_markers = {
        "methodology_only",
        "keep_methodology_only",
        "known_limit:external_alpha.reference_workspace_dry_run_synthetic_only",
        "known_limit:external_alpha.publication_rehearsal_methodology_only",
    }
    missing_rehearsal = sorted(marker for marker in required_rehearsal_markers if marker not in rehearsal_text)
    if missing_rehearsal:
        findings.append(
            Finding(
                severity="error",
                check_id="publication_rehearsal.required_markers_missing",
                message="publication rehearsal is missing required methodology-only and known-limit markers",
                remediation="Keep the rehearsal explicit about methodology-only status and dry-run limitations.",
                details={"missing": missing_rehearsal},
            )
        )

    for marker in REQUIRED_LIMIT_IDS:
        if marker not in known_limits_text:
            findings.append(
                Finding(
                    severity="error",
                    check_id="known_limits_markdown.required_marker_missing",
                    message=f"markdown known-limits companion does not mention {marker}",
                    remediation="Keep the human-readable known-limits packet aligned with the machine packet.",
                    ref=marker,
                )
            )


def render_summary(cases: dict[str, Any]) -> str:
    lines = [
        "External alpha reference-workspace dry run",
        "case_id | reference_workspace | bundle | result | publication_rehearsal | known_limits",
    ]
    for raw_case in ensure_list(cases.get("cases"), "cases.cases"):
        case = ensure_dict(raw_case, "cases.cases[]")
        lines.append(
            " | ".join(
                [
                    ensure_str(case.get("case_id"), "case.case_id"),
                    ensure_str(case.get("reference_workspace_id"), "case.reference_workspace_id"),
                    ensure_str(case.get("workflow_bundle_ref"), "case.workflow_bundle_ref"),
                    ensure_str(case.get("dry_run_result"), "case.dry_run_result"),
                    ensure_str(case.get("publication_rehearsal_result"), "case.publication_rehearsal_result"),
                    ",".join(ensure_list(case.get("known_limit_refs"), "case.known_limit_refs")),
                ]
            )
        )
    return "\n".join(lines) + "\n"


def write_report(path: Path, checked_workspaces: list[str], findings: list[Finding]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    payload = {
        "schema_version": 1,
        "status": "pass" if not any(item.severity == "error" for item in findings) else "fail",
        "generated_at": dt.datetime.now(dt.UTC).replace(microsecond=0).isoformat().replace("+00:00", "Z"),
        "summary": {
            "errors": sum(1 for item in findings if item.severity == "error"),
            "warnings": sum(1 for item in findings if item.severity == "warning"),
            "checked_reference_workspace_ids": checked_workspaces,
            "acceptance_states": sorted(REQUIRED_ACCEPTANCE_STATES),
        },
        "findings": [item.as_report() for item in findings],
    }
    path.write_text(json.dumps(payload, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    findings: list[Finding] = []

    cases = ensure_dict(render_yaml_as_json(repo_root / args.cases), "cases")
    known_limits = ensure_dict(render_yaml_as_json(repo_root / args.known_limits), "known_limits")
    fixture_register = ensure_dict(render_yaml_as_json(repo_root / args.fixture_register), "fixture_register")
    scoreboard = ensure_dict(render_yaml_as_json(repo_root / args.scoreboard), "scoreboard")
    tsjs_bundle = ensure_dict(render_yaml_as_json(repo_root / args.tsjs_bundle), "tsjs_bundle")
    python_bundle = ensure_dict(render_yaml_as_json(repo_root / args.python_bundle), "python_bundle")
    publication_checklist = ensure_dict(render_yaml_as_json(repo_root / args.publication_checklist), "publication_checklist")

    dry_run_text = read_text(repo_root, args.dry_run, "dry_run_report", findings)
    rehearsal_text = read_text(repo_root, args.publication_rehearsal, "publication_rehearsal", findings)
    known_limits_markdown = read_text(repo_root, args.known_limits_markdown, "known_limits_markdown", findings)

    validate_header(known_limits, "known_limits", findings)
    validate_source_refs(repo_root, known_limits, "known_limits", findings)
    known_limit_ids = collect_known_limits(known_limits, known_limits_markdown, findings)
    validate_publication_checklist(publication_checklist, findings)
    validate_text_artifacts(dry_run_text, rehearsal_text, known_limits_markdown, findings)
    checked_workspaces = validate_cases(
        repo_root=repo_root,
        cases=cases,
        fixture_register=fixture_register,
        scoreboard=scoreboard,
        bundles_by_id=collect_bundle_ids(tsjs_bundle, python_bundle),
        known_limit_ids=known_limit_ids,
        dry_run_text=dry_run_text,
        rehearsal_text=rehearsal_text,
        findings=findings,
    )

    if args.report:
        write_report(repo_root / args.report, checked_workspaces, findings)

    if args.render_publication_summary:
        print(render_summary(cases), end="")

    errors = [finding for finding in findings if finding.severity == "error"]
    warnings = [finding for finding in findings if finding.severity == "warning"]
    status = "PASS" if not errors else "FAIL"
    print(f"[reference-workspace-dry-run] {status} ({len(errors)} errors, {len(warnings)} warnings)")
    for finding in findings:
        prefix = "ERROR" if finding.severity == "error" else "WARN"
        ref_suffix = f" [{finding.ref}]" if finding.ref else ""
        print(f"[reference-workspace-dry-run] {prefix} {finding.check_id}: {finding.message}{ref_suffix}")
        print(f"[reference-workspace-dry-run]   remediation: {finding.remediation}")
    return 0 if not errors else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except KeyboardInterrupt:
        print("[reference-workspace-dry-run] interrupted", file=sys.stderr)
        sys.exit(130)
