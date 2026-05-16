#!/usr/bin/env python3
"""Validate the beta remote-helper capability and skew-window fixtures."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/providers/remote_capabilities.schema.json"
DEFAULT_MANIFEST_REL = "fixtures/runtime/m3/remote_helper_skew_beta/manifest.yaml"

FAILS_CLOSED_VISIBILITIES = {"probe_required_untested", "outside_supported_window"}

REPAIR_PATH_BY_VISIBILITY = {
    "adjacent_supported": {"no_repair_required"},
    "narrowed_supported_window": {"continue_narrowed_posture", "continue_local_only"},
    "probe_required_untested": {"run_drift_probe_or_reattach"},
    "outside_supported_window": {
        "upgrade_or_repin",
        "continue_local_only",
        "contact_admin_or_support",
    },
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
                check_id="remote_helper_skew_beta.schema.validation_failed",
                message=f"{path}: {error.message}",
                remediation="Update the fixture or schema so the beta envelope validates.",
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


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
    for idx, ref in enumerate(ensure_list(manifest.get("alpha_lane_refs"), "manifest.alpha_lane_refs")):
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
    for idx, ref in enumerate(ensure_list(manifest.get("case_refs"), "manifest.case_refs")):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(ref, f"manifest.case_refs[{idx}]"),
            "manifest.case_refs",
        )

    actual_case_ids = {case_id_of(case) for case in cases}
    expected_case_ids = {
        ensure_str(item, "manifest.expected_case_ids[]")
        for item in ensure_list(manifest.get("expected_case_ids"), "manifest.expected_case_ids")
    }
    if actual_case_ids != expected_case_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.case_ids.mismatch",
                message="manifest expected_case_ids must exactly match loaded case ids",
                remediation="Update the manifest or fixture set so the protected cases are explicit.",
                details={
                    "actual": sorted(actual_case_ids),
                    "expected": sorted(expected_case_ids),
                },
            )
        )

    actual_phases = {ensure_str(case.get("lifecycle_phase"), "case.lifecycle_phase") for case in cases}
    required_phases = {
        ensure_str(item, "manifest.required_lifecycle_phases[]")
        for item in ensure_list(
            manifest.get("required_lifecycle_phases"), "manifest.required_lifecycle_phases"
        )
    }
    missing_phases = sorted(required_phases - actual_phases)
    if missing_phases:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.lifecycle_phase_coverage_missing",
                message=f"missing required lifecycle phases: {', '.join(missing_phases)}",
                remediation="Add or repair fixtures so attach and reconnect are both covered.",
            )
        )

    actual_visibilities = {
        ensure_str(case.get("skew_visibility"), "case.skew_visibility") for case in cases
    }
    required_visibilities = {
        ensure_str(item, "manifest.required_skew_visibility_classes[]")
        for item in ensure_list(
            manifest.get("required_skew_visibility_classes"),
            "manifest.required_skew_visibility_classes",
        )
    }
    missing_visibilities = sorted(required_visibilities - actual_visibilities)
    if missing_visibilities:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.skew_visibility_coverage_missing",
                message=f"missing required skew_visibility classes: {', '.join(missing_visibilities)}",
                remediation="Add or repair fixtures so every required visibility class is exercised.",
            )
        )

    actual_repairs = {
        ensure_str(case.get("repair_path"), "case.repair_path") for case in cases
    }
    required_repairs = {
        ensure_str(item, "manifest.required_repair_path_classes[]")
        for item in ensure_list(
            manifest.get("required_repair_path_classes"),
            "manifest.required_repair_path_classes",
        )
    }
    missing_repairs = sorted(required_repairs - actual_repairs)
    if missing_repairs:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.repair_path_coverage_missing",
                message=f"missing required repair_path classes: {', '.join(missing_repairs)}",
                remediation="Add or repair fixtures so every required repair-path class is exercised.",
            )
        )

    acceptance_seen: set[str] = set()
    for case in cases:
        expectations = ensure_dict(case.get("harness_expectations"), "case.harness_expectations")
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
                check_id="remote_helper_skew_beta.acceptance_coverage_missing",
                message=f"missing required acceptance states: {', '.join(missing_acceptance)}",
                remediation="Add fixtures that exercise every protected acceptance state.",
            )
        )


def case_id_of(case: dict[str, Any]) -> str:
    expectations = ensure_dict(case.get("harness_expectations"), "case.harness_expectations")
    return ensure_str(expectations.get("case_id"), "case.harness_expectations.case_id")


def validate_case(repo_root: Path, case: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    cid = case_id_of(case)

    visibility = ensure_str(case.get("skew_visibility"), f"{cid}.skew_visibility")
    repair = ensure_str(case.get("repair_path"), f"{cid}.repair_path")
    outcome = ensure_str(case.get("negotiation_outcome"), f"{cid}.negotiation_outcome")
    mutation_allowed = bool(case.get("mutation_allowed"))

    allowed_repairs = REPAIR_PATH_BY_VISIBILITY.get(visibility, set())
    if repair not in allowed_repairs:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.repair_path_visibility_mismatch",
                message=(
                    f"{cid} visibility={visibility} cannot use repair_path={repair}; "
                    f"expected one of {sorted(allowed_repairs)}"
                ),
                remediation="Align skew_visibility and repair_path with the canonical derivation table.",
                ref=cid,
            )
        )

    if visibility in FAILS_CLOSED_VISIBILITIES and mutation_allowed:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.fails_closed_violation",
                message=(
                    f"{cid} visibility={visibility} must fail closed for mutating remote work, "
                    "but mutation_allowed=true"
                ),
                remediation="Set mutation_allowed=false for probe-required or outside-window records.",
                ref=cid,
            )
        )

    if outcome != "match" and mutation_allowed:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.outcome_mutation_mismatch",
                message=f"{cid} mutation_allowed=true but negotiation_outcome={outcome}",
                remediation="Mutation requires negotiation_outcome=match.",
                ref=cid,
            )
        )

    if outcome == "refuse" and case.get("negotiated_capabilities"):
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.refused_negotiated_not_empty",
                message=f"{cid} refused negotiation must have empty negotiated_capabilities",
                remediation="Drop the negotiated capabilities on refused records.",
                ref=cid,
            )
        )

    expectations = ensure_dict(case.get("harness_expectations"), f"{cid}.harness_expectations")
    if expectations.get("expected_skew_visibility") != visibility:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.expected_skew_visibility_mismatch",
                message=f"{cid} skew_visibility does not match harness expectation",
                remediation="Update the fixture visibility or expectation together.",
                ref=cid,
            )
        )
    if expectations.get("expected_repair_path") != repair:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.expected_repair_path_mismatch",
                message=f"{cid} repair_path does not match harness expectation",
                remediation="Update the fixture repair path or expectation together.",
                ref=cid,
            )
        )
    if expectations.get("expected_negotiation_outcome") != outcome:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.expected_negotiation_outcome_mismatch",
                message=f"{cid} negotiation_outcome does not match harness expectation",
                remediation="Update the fixture outcome or expectation together.",
                ref=cid,
            )
        )
    if bool(expectations.get("expected_mutation_allowed")) != mutation_allowed:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.expected_mutation_allowed_mismatch",
                message=f"{cid} mutation_allowed does not match harness expectation",
                remediation="Update the fixture mutation flag or expectation together.",
                ref=cid,
            )
        )

    support_refs = ensure_list(case.get("support_packet_refs"), f"{cid}.support_packet_refs")
    compat_refs = ensure_list(
        case.get("compatibility_report_row_refs"), f"{cid}.compatibility_report_row_refs"
    )
    if not support_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.support_packet_refs_missing",
                message=f"{cid} must reference at least one support packet ref",
                remediation="Add a support_packet ref so the export bundle has a target.",
                ref=cid,
            )
        )
    if not compat_refs:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_helper_skew_beta.compatibility_report_row_refs_missing",
                message=f"{cid} must reference at least one compatibility-report row ref",
                remediation="Add a compat_report_row ref so the report and support consumers share truth.",
                ref=cid,
            )
        )

    evidence = ensure_dict(case.get("evidence_refs"), f"{cid}.evidence_refs")
    for group in ("schema_refs", "fixture_refs", "doc_refs", "compat_refs"):
        for idx, ref in enumerate(ensure_list(evidence.get(group), f"{cid}.evidence_refs.{group}")):
            add_missing_ref(
                findings,
                repo_root,
                ensure_str(ref, f"{cid}.evidence_refs.{group}[{idx}]"),
                f"{cid}.evidence_refs.{group}",
            )
    add_missing_ref(
        findings,
        repo_root,
        ensure_str(evidence.get("alpha_envelope_ref"), f"{cid}.evidence_refs.alpha_envelope_ref"),
        f"{cid}.evidence_refs.alpha_envelope_ref",
    )

    return findings


def render_summary(findings: list[Finding], cases: list[dict[str, Any]]) -> str:
    lines = ["[remote-helper-skew-beta] summary"]
    if not findings:
        lines.append(
            f"[remote-helper-skew-beta] OK: {len(cases)} fixture cases validated"
        )
        return "\n".join(lines) + "\n"
    lines.append(f"[remote-helper-skew-beta] FAIL: {len(findings)} finding(s)")
    for finding in findings:
        ref = f" [{finding.ref}]" if finding.ref else ""
        lines.append(f"- {finding.severity}: {finding.check_id}{ref}: {finding.message}")
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()
    schema = ensure_dict(load_json(repo_root / args.schema), args.schema)
    manifest = ensure_dict(load_yaml(repo_root / args.manifest), args.manifest)

    case_refs = [
        ensure_str(ref, f"manifest.case_refs[{idx}]")
        for idx, ref in enumerate(ensure_list(manifest.get("case_refs"), "manifest.case_refs"))
    ]
    cases = [ensure_dict(load_json(repo_root / ref), ref) for ref in case_refs]

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
            "record_kind": "remote_helper_skew_beta_validation_report",
            "schema_ref": args.schema,
            "manifest_ref": args.manifest,
            "case_count": len(cases),
            "findings": [finding.as_report() for finding in findings],
        }
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
