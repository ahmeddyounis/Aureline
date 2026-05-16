#!/usr/bin/env python3
"""Validate the beta managed-workspace lifecycle and lineage fixtures."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/providers/managed_workspace_lifecycle.schema.json"
DEFAULT_MANIFEST_REL = "fixtures/runtime/m3/managed_workspace_lifecycle/manifest.yaml"

LIVE_STATES = {"live"}
RECONNECT_STATES = {"reconnect_required"}
TERMINAL_STATES = {"retired"}
NOT_APPLICABLE_STATES = {"starting", "retired"}


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
    parser.add_argument("--schema", default=DEFAULT_SCHEMA_REL)
    parser.add_argument("--manifest", default=DEFAULT_MANIFEST_REL)
    parser.add_argument("--report", default=None)
    return parser.parse_args()


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def load_yaml(path: Path) -> Any:
    try:
        return yaml.safe_load(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing YAML file: {path}") from exc
    except yaml.YAMLError as exc:
        raise SystemExit(f"invalid YAML at {path}: {exc}") from exc


def ensure_dict(value: Any, label: str) -> dict[str, Any]:
    if not isinstance(value, dict):
        raise SystemExit(f"{label} must be a mapping/object")
    return value


def ensure_list(value: Any, label: str) -> list[Any]:
    if not isinstance(value, list):
        raise SystemExit(f"{label} must be a list/array")
    return value


def ensure_str(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise SystemExit(f"{label} must be a non-empty string")
    return value.strip()


def ref_exists(repo_root: Path, ref: str) -> bool:
    clean = ref.split("#", 1)[0].strip()
    return bool(clean) and (repo_root / clean).exists()


def add_missing_ref(
    findings: list[Finding], repo_root: Path, ref: str, label: str
) -> None:
    if "/" not in ref and not ref.endswith((".json", ".yaml", ".yml", ".md", ".py", ".rs")):
        return
    if not ref_exists(repo_root, ref):
        findings.append(
            Finding(
                severity="error",
                check_id=f"{label}.missing_ref",
                message=f"{label} does not resolve: {ref}",
                remediation="Fix the reference or seed the referenced artifact.",
                ref=ref,
            )
        )


def schema_validate(schema: dict[str, Any], payload: dict[str, Any]) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.schema.validation_failed",
                message=f"{path}: {error.message}",
                remediation="Update the fixture or schema so the beta record validates.",
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def case_id_of(case: dict[str, Any]) -> str:
    expectations = ensure_dict(
        case.get("harness_expectations"), "case.harness_expectations"
    )
    return ensure_str(
        expectations.get("case_id"), "case.harness_expectations.case_id"
    )


def validate_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    cases: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    for ref_label in ("schema_ref", "doc_ref", "runtime_module_ref"):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(manifest.get(ref_label), f"manifest.{ref_label}"),
            f"manifest.{ref_label}",
        )
    for idx, ref in enumerate(
        ensure_list(manifest.get("alpha_lane_refs"), "manifest.alpha_lane_refs")
    ):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(ref, f"manifest.alpha_lane_refs[{idx}]"),
            "manifest.alpha_lane_refs",
        )
    validator = ensure_dict(manifest.get("validator"), "manifest.validator")
    add_missing_ref(
        findings,
        repo_root,
        ensure_str(validator.get("script_ref"), "manifest.validator.script_ref"),
        "manifest.validator.script_ref",
    )
    for idx, ref in enumerate(
        ensure_list(manifest.get("case_refs"), "manifest.case_refs")
    ):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(ref, f"manifest.case_refs[{idx}]"),
            "manifest.case_refs",
        )

    actual_case_ids = {case_id_of(case) for case in cases}
    expected_case_ids = {
        ensure_str(item, "manifest.expected_case_ids[]")
        for item in ensure_list(
            manifest.get("expected_case_ids"), "manifest.expected_case_ids"
        )
    }
    if actual_case_ids != expected_case_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.case_ids.mismatch",
                message="manifest expected_case_ids must exactly match loaded case ids",
                remediation=(
                    "Update the manifest or fixture set so the protected cases are explicit."
                ),
                details={
                    "actual": sorted(actual_case_ids),
                    "expected": sorted(expected_case_ids),
                },
            )
        )

    actual_phases = {
        ensure_str(case.get("current_phase"), "case.current_phase") for case in cases
    }
    required_phases = {
        ensure_str(item, "manifest.required_lifecycle_phases[]")
        for item in ensure_list(
            manifest.get("required_lifecycle_phases"),
            "manifest.required_lifecycle_phases",
        )
    }
    missing_phases = sorted(required_phases - actual_phases)
    if missing_phases:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.lifecycle_phase_coverage_missing",
                message=(
                    f"missing required lifecycle phases: {', '.join(missing_phases)}"
                ),
                remediation=(
                    "Add or repair fixtures so every required phase is covered."
                ),
            )
        )

    actual_states = {
        ensure_str(case.get("current_state"), "case.current_state") for case in cases
    }
    required_states = {
        ensure_str(item, "manifest.required_lifecycle_states[]")
        for item in ensure_list(
            manifest.get("required_lifecycle_states"),
            "manifest.required_lifecycle_states",
        )
    }
    missing_states = sorted(required_states - actual_states)
    if missing_states:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.lifecycle_state_coverage_missing",
                message=(
                    f"missing required lifecycle states: {', '.join(missing_states)}"
                ),
                remediation=(
                    "Add or repair fixtures so every required state is covered."
                ),
            )
        )

    actual_continuity = {
        ensure_str(
            case.get("local_editing_continuity"),
            "case.local_editing_continuity",
        )
        for case in cases
    }
    required_continuity = {
        ensure_str(item, "manifest.required_local_editing_continuity_classes[]")
        for item in ensure_list(
            manifest.get("required_local_editing_continuity_classes"),
            "manifest.required_local_editing_continuity_classes",
        )
    }
    missing_continuity = sorted(required_continuity - actual_continuity)
    if missing_continuity:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.local_editing_continuity_coverage_missing"
                ),
                message=(
                    "missing required local-editing continuity classes: "
                    f"{', '.join(missing_continuity)}"
                ),
                remediation=(
                    "Add or repair fixtures so every required continuity class is covered."
                ),
            )
        )

    acceptance_seen: set[str] = set()
    for case in cases:
        expectations = ensure_dict(
            case.get("harness_expectations"), "case.harness_expectations"
        )
        acceptance_seen.update(
            ensure_str(item, "case.harness_expectations.acceptance_states[]")
            for item in ensure_list(
                expectations.get("acceptance_states"),
                "case.harness_expectations.acceptance_states",
            )
        )
    required_acceptance = {
        ensure_str(item, "manifest.required_acceptance_states[]")
        for item in ensure_list(
            manifest.get("required_acceptance_states"),
            "manifest.required_acceptance_states",
        )
    }
    missing_acceptance = sorted(required_acceptance - acceptance_seen)
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.acceptance_coverage_missing"
                ),
                message=(
                    "missing required acceptance states: "
                    f"{', '.join(missing_acceptance)}"
                ),
                remediation=(
                    "Add fixtures that exercise every protected acceptance state."
                ),
            )
        )


def validate_case(repo_root: Path, case: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    cid = case_id_of(case)

    phase = ensure_str(case.get("current_phase"), f"{cid}.current_phase")
    state = ensure_str(case.get("current_state"), f"{cid}.current_state")
    continuity = ensure_str(
        case.get("local_editing_continuity"),
        f"{cid}.local_editing_continuity",
    )
    mutation_allowed = bool(case.get("mutation_allowed"))
    reconnect_required = bool(case.get("reconnect_required"))
    lineage = ensure_list(case.get("lineage"), f"{cid}.lineage")
    if not lineage:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.empty_lineage",
                message=f"{cid} must carry a non-empty lineage",
                remediation="Add at least the current phase to the lineage.",
                ref=cid,
            )
        )
        return findings

    tail = ensure_dict(lineage[-1], f"{cid}.lineage[last]")
    tail_phase = ensure_str(tail.get("phase"), f"{cid}.lineage[last].phase")
    tail_state = ensure_str(tail.get("state"), f"{cid}.lineage[last].state")
    if tail_phase != phase:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.lineage_tail_phase_mismatch",
                message=(
                    f"{cid} lineage tail phase={tail_phase} does not match "
                    f"current_phase={phase}"
                ),
                remediation="Align lineage tail with current_phase.",
                ref=cid,
            )
        )
    if tail_state != state:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.lineage_tail_state_mismatch",
                message=(
                    f"{cid} lineage tail state={tail_state} does not match "
                    f"current_state={state}"
                ),
                remediation="Align lineage tail with current_state.",
                ref=cid,
            )
        )

    if mutation_allowed and state not in LIVE_STATES:
        findings.append(
            Finding(
                severity="error",
                check_id="managed_workspace_lifecycle_beta.mutation_in_non_live_state",
                message=(
                    f"{cid} mutation_allowed=true but current_state={state}; "
                    "mutation is reserved for state=live"
                ),
                remediation="Set mutation_allowed=false unless current_state is live.",
                ref=cid,
            )
        )
    if reconnect_required and state not in RECONNECT_STATES:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.reconnect_required_state_mismatch"
                ),
                message=(
                    f"{cid} reconnect_required=true but current_state={state}"
                ),
                remediation=(
                    "Set reconnect_required=true only when current_state is reconnect_required."
                ),
                ref=cid,
            )
        )
    if state in RECONNECT_STATES and not reconnect_required:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.reconnect_required_flag_missing"
                ),
                message=(
                    f"{cid} current_state=reconnect_required must set reconnect_required=true"
                ),
                remediation="Set reconnect_required=true for reconnect_required rows.",
                ref=cid,
            )
        )
    if continuity == "not_applicable" and state not in NOT_APPLICABLE_STATES:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.continuity_not_applicable_outside_terminal"
                ),
                message=(
                    f"{cid} continuity=not_applicable but current_state={state}; "
                    "not_applicable is reserved for starting or retired rows"
                ),
                remediation=(
                    "Choose a preserved or inspect-only continuity class for non-terminal rows."
                ),
                ref=cid,
            )
        )

    expectations = ensure_dict(
        case.get("harness_expectations"), f"{cid}.harness_expectations"
    )
    if expectations.get("expected_current_phase") != phase:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_current_phase_mismatch"
                ),
                message=f"{cid} current_phase does not match harness expectation",
                remediation="Update the fixture phase or expectation together.",
                ref=cid,
            )
        )
    if expectations.get("expected_current_state") != state:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_current_state_mismatch"
                ),
                message=f"{cid} current_state does not match harness expectation",
                remediation="Update the fixture state or expectation together.",
                ref=cid,
            )
        )
    if expectations.get("expected_local_editing_continuity") != continuity:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_continuity_mismatch"
                ),
                message=(
                    f"{cid} local_editing_continuity does not match harness expectation"
                ),
                remediation="Update the fixture continuity or expectation together.",
                ref=cid,
            )
        )
    if bool(expectations.get("expected_mutation_allowed")) != mutation_allowed:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_mutation_allowed_mismatch"
                ),
                message=(
                    f"{cid} mutation_allowed does not match harness expectation"
                ),
                remediation="Update the fixture mutation flag or expectation together.",
                ref=cid,
            )
        )
    if bool(expectations.get("expected_reconnect_required")) != reconnect_required:
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_reconnect_required_mismatch"
                ),
                message=(
                    f"{cid} reconnect_required does not match harness expectation"
                ),
                remediation="Update the fixture reconnect flag or expectation together.",
                ref=cid,
            )
        )
    expected_lineage_length = expectations.get("expected_lineage_length")
    if expected_lineage_length != len(lineage):
        findings.append(
            Finding(
                severity="error",
                check_id=(
                    "managed_workspace_lifecycle_beta.expected_lineage_length_mismatch"
                ),
                message=(
                    f"{cid} lineage length {len(lineage)} does not match expectation "
                    f"{expected_lineage_length}"
                ),
                remediation="Update the fixture lineage or expectation together.",
                ref=cid,
            )
        )

    evidence = ensure_dict(case.get("evidence_refs"), f"{cid}.evidence_refs")
    for group in ("schema_refs", "fixture_refs", "doc_refs"):
        for idx, ref in enumerate(
            ensure_list(evidence.get(group), f"{cid}.evidence_refs.{group}")
        ):
            add_missing_ref(
                findings,
                repo_root,
                ensure_str(ref, f"{cid}.evidence_refs.{group}[{idx}]"),
                f"{cid}.evidence_refs.{group}",
            )

    return findings


def render_summary(findings: list[Finding], cases: list[dict[str, Any]]) -> str:
    lines = ["[managed-workspace-lifecycle-beta] summary"]
    if not findings:
        lines.append(
            "[managed-workspace-lifecycle-beta] OK: "
            f"{len(cases)} fixture cases validated"
        )
        return "\n".join(lines) + "\n"
    lines.append(
        f"[managed-workspace-lifecycle-beta] FAIL: {len(findings)} finding(s)"
    )
    for finding in findings:
        ref = f" [{finding.ref}]" if finding.ref else ""
        lines.append(
            f"- {finding.severity}: {finding.check_id}{ref}: {finding.message}"
        )
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema = ensure_dict(load_json(repo_root / args.schema), args.schema)
    manifest = ensure_dict(load_yaml(repo_root / args.manifest), args.manifest)

    case_refs = [
        ensure_str(ref, f"manifest.case_refs[{idx}]")
        for idx, ref in enumerate(
            ensure_list(manifest.get("case_refs"), "manifest.case_refs")
        )
    ]
    cases = [
        ensure_dict(load_json(repo_root / ref), ref) for ref in case_refs
    ]

    findings: list[Finding] = []
    validate_manifest(repo_root, manifest, cases, findings)
    for case in cases:
        findings.extend(schema_validate(schema, case))
        findings.extend(validate_case(repo_root, case))

    sys.stdout.write(render_summary(findings, cases))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report = {
            "record_kind": "managed_workspace_lifecycle_beta_validation_report",
            "schema_ref": args.schema,
            "manifest_ref": args.manifest,
            "case_count": len(cases),
            "findings": [finding.as_report() for finding in findings],
        }
        report_path.write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
