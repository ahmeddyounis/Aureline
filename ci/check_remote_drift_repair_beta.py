#!/usr/bin/env python3
"""Validate the beta remote drift-repair fixtures and diagnostics packet."""

from __future__ import annotations

import argparse
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/workspace/remote_drift_repair.schema.json"
DEFAULT_MANIFEST_REL = "fixtures/runtime/m3/remote_drift/manifest.yaml"

FAILS_CLOSED_PRIMARY_ACTIONS = {
    "continue_narrowed_posture",
    "run_drift_probe",
    "reconnect",
    "upgrade",
    "downgrade",
    "continue_local_only",
    "contact_admin_or_support",
}

WIDENING_ACTIONS = {"upgrade"}
NARROWING_ACTIONS = {"continue_narrowed_posture", "continue_local_only", "downgrade"}
NEUTRAL_ACTIONS = {
    "no_repair_required",
    "run_drift_probe",
    "reconnect",
    "contact_admin_or_support",
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


def schema_validate(schema: dict[str, Any], payload: dict[str, Any], label: str) -> list[Finding]:
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema, format_checker=FormatChecker())
    findings: list[Finding] = []
    for error in sorted(validator.iter_errors(payload), key=lambda item: list(item.path)):
        path = ".".join(str(part) for part in error.path) or "<root>"
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.schema.validation_failed",
                message=f"{label}: {path}: {error.message}",
                remediation="Update the fixture or schema so the drift-repair record validates.",
                details={"schema_path": list(error.schema_path)},
                ref=label,
            )
        )
    return findings


def case_id_of(case: dict[str, Any]) -> str:
    expectations = ensure_dict(case.get("harness_expectations"), "case.harness_expectations")
    return ensure_str(expectations.get("case_id"), "case.harness_expectations.case_id")


def validate_authority_for_action(
    findings: list[Finding], cid: str, label: str, action: dict[str, Any]
) -> None:
    action_class = ensure_str(action.get("action_class"), f"{cid}.{label}.action_class")
    impact = ensure_str(action.get("authority_impact"), f"{cid}.{label}.authority_impact")
    requires = bool(action.get("requires_reapproval"))

    if action_class in WIDENING_ACTIONS:
        if impact != "requires_reapproval":
            findings.append(
                Finding(
                    severity="error",
                    check_id="remote_drift_repair_beta.authority_impact_widens_silently",
                    message=(
                        f"{cid}.{label} action={action_class} must declare authority_impact=requires_reapproval; "
                        f"got {impact}"
                    ),
                    remediation="Authority-widening actions must require explicit re-approval.",
                    ref=cid,
                )
            )
        if not requires:
            findings.append(
                Finding(
                    severity="error",
                    check_id="remote_drift_repair_beta.requires_reapproval_flag_mismatch",
                    message=(
                        f"{cid}.{label} action={action_class} must set requires_reapproval=true"
                    ),
                    remediation="Mirror authority_impact in the requires_reapproval flag.",
                    ref=cid,
                )
            )
    elif action_class in NARROWING_ACTIONS:
        if impact != "narrows_authority":
            findings.append(
                Finding(
                    severity="error",
                    check_id="remote_drift_repair_beta.authority_impact_should_narrow",
                    message=(
                        f"{cid}.{label} action={action_class} expected authority_impact=narrows_authority; got {impact}"
                    ),
                    remediation="Narrowing actions must stamp narrows_authority.",
                    ref=cid,
                )
            )
    elif action_class in NEUTRAL_ACTIONS:
        if impact != "maintains_current":
            findings.append(
                Finding(
                    severity="error",
                    check_id="remote_drift_repair_beta.authority_impact_should_maintain",
                    message=(
                        f"{cid}.{label} action={action_class} expected authority_impact=maintains_current; got {impact}"
                    ),
                    remediation="Neutral actions must stamp maintains_current.",
                    ref=cid,
                )
            )

    if impact == "requires_reapproval" and not requires:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.requires_reapproval_flag_missing",
                message=(
                    f"{cid}.{label} authority_impact=requires_reapproval but requires_reapproval flag is false"
                ),
                remediation="Set requires_reapproval=true whenever authority_impact=requires_reapproval.",
                ref=cid,
            )
        )


def validate_case(repo_root: Path, case: dict[str, Any]) -> list[Finding]:
    findings: list[Finding] = []
    cid = case_id_of(case)

    primary = ensure_dict(case.get("primary_action"), f"{cid}.primary_action")
    validate_authority_for_action(findings, cid, "primary_action", primary)
    for idx, action in enumerate(
        ensure_list(case.get("alternative_actions"), f"{cid}.alternative_actions")
    ):
        validate_authority_for_action(
            findings, cid, f"alternative_actions[{idx}]", ensure_dict(action, f"{cid}.alternative_actions[{idx}]")
        )

    primary_class = ensure_str(primary.get("action_class"), f"{cid}.primary_action.action_class")
    fails_closed_flag = bool(case.get("fails_closed_for_mutation"))
    if primary_class in FAILS_CLOSED_PRIMARY_ACTIONS and not fails_closed_flag:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.fails_closed_flag_mismatch",
                message=(
                    f"{cid} primary action={primary_class} requires fails_closed_for_mutation=true"
                ),
                remediation="Non-baseline primary actions must stamp fails_closed_for_mutation=true.",
                ref=cid,
            )
        )
    if primary_class == "no_repair_required" and fails_closed_flag:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.no_repair_should_not_fail_closed",
                message=f"{cid} no_repair_required must not stamp fails_closed_for_mutation=true",
                remediation="Adjacent supported cases never fail closed.",
                ref=cid,
            )
        )

    any_requires_field = bool(case.get("any_action_requires_reapproval"))
    any_requires_actual = bool(primary.get("requires_reapproval")) or any(
        bool(action.get("requires_reapproval"))
        for action in ensure_list(case.get("alternative_actions"), f"{cid}.alternative_actions")
    )
    if any_requires_field != any_requires_actual:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.any_action_requires_reapproval_mismatch",
                message=(
                    f"{cid} any_action_requires_reapproval={any_requires_field} but "
                    f"derived value={any_requires_actual}"
                ),
                remediation="Recompute any_action_requires_reapproval from the action authority impacts.",
                ref=cid,
            )
        )

    expectations = ensure_dict(case.get("harness_expectations"), f"{cid}.harness_expectations")
    if expectations.get("expected_primary_action_class") != primary_class:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.expected_primary_action_class_mismatch",
                message=f"{cid} primary_action.action_class does not match harness expectation",
                remediation="Update the fixture primary action or expectation together.",
                ref=cid,
            )
        )
    if expectations.get("expected_authority_impact") != primary.get("authority_impact"):
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.expected_authority_impact_mismatch",
                message=f"{cid} primary_action.authority_impact does not match harness expectation",
                remediation="Update the fixture authority impact or expectation together.",
                ref=cid,
            )
        )
    expected_reasons = list(
        ensure_list(
            expectations.get("expected_drift_reasons"),
            f"{cid}.harness_expectations.expected_drift_reasons",
        )
    )
    actual_reasons = list(
        ensure_list(case.get("drift_reasons"), f"{cid}.drift_reasons")
    )
    if expected_reasons != actual_reasons:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.expected_drift_reasons_mismatch",
                message=f"{cid} drift_reasons does not match harness expectation",
                remediation="Update the fixture drift reasons or expectation together.",
                ref=cid,
                details={"actual": actual_reasons, "expected": expected_reasons},
            )
        )
    if bool(expectations.get("expected_fails_closed_for_mutation")) != fails_closed_flag:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.expected_fails_closed_mismatch",
                message=f"{cid} fails_closed_for_mutation does not match harness expectation",
                remediation="Update the fixture flag or expectation together.",
                ref=cid,
            )
        )
    if (
        bool(expectations.get("expected_any_action_requires_reapproval"))
        != any_requires_field
    ):
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.expected_any_action_requires_reapproval_mismatch",
                message=f"{cid} any_action_requires_reapproval does not match harness expectation",
                remediation="Update the fixture flag or expectation together.",
                ref=cid,
            )
        )

    evidence = ensure_dict(case.get("evidence_refs"), f"{cid}.evidence_refs")
    add_missing_ref(
        findings,
        repo_root,
        ensure_str(evidence.get("source_record_ref"), f"{cid}.evidence_refs.source_record_ref"),
        f"{cid}.evidence_refs.source_record_ref",
    )
    for group in ("schema_refs", "fixture_refs", "doc_refs"):
        for idx, ref in enumerate(ensure_list(evidence.get(group), f"{cid}.evidence_refs.{group}")):
            add_missing_ref(
                findings,
                repo_root,
                ensure_str(ref, f"{cid}.evidence_refs.{group}[{idx}]"),
                f"{cid}.evidence_refs.{group}",
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
    for idx, ref in enumerate(ensure_list(manifest.get("source_lane_refs"), "manifest.source_lane_refs")):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(ref, f"manifest.source_lane_refs[{idx}]"),
            "manifest.source_lane_refs",
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
                check_id="remote_drift_repair_beta.case_ids.mismatch",
                message="manifest expected_case_ids must exactly match loaded case ids",
                remediation="Update the manifest or fixture set so the protected cases are explicit.",
                details={"actual": sorted(actual_case_ids), "expected": sorted(expected_case_ids)},
            )
        )

    actual_reason_classes: set[str] = set()
    for case in cases:
        for reason in ensure_list(case.get("drift_reasons"), "case.drift_reasons"):
            actual_reason_classes.add(ensure_str(reason, "case.drift_reasons[]"))
    required_reasons = {
        ensure_str(item, "manifest.required_drift_reason_classes[]")
        for item in ensure_list(
            manifest.get("required_drift_reason_classes"),
            "manifest.required_drift_reason_classes",
        )
    }
    missing_reasons = sorted(required_reasons - actual_reason_classes)
    if missing_reasons:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.drift_reason_coverage_missing",
                message=f"missing required drift_reason classes: {', '.join(missing_reasons)}",
                remediation="Add or repair fixtures so every required drift-reason class is exercised.",
            )
        )

    actual_primary_actions: set[str] = set()
    actual_alternative_actions: set[str] = set()
    actual_authority_impacts: set[str] = set()
    for case in cases:
        primary = ensure_dict(case.get("primary_action"), "case.primary_action")
        actual_primary_actions.add(
            ensure_str(primary.get("action_class"), "case.primary_action.action_class")
        )
        actual_authority_impacts.add(
            ensure_str(primary.get("authority_impact"), "case.primary_action.authority_impact")
        )
        for action in ensure_list(case.get("alternative_actions"), "case.alternative_actions"):
            actual_alternative_actions.add(
                ensure_str(action.get("action_class"), "case.alternative_actions[].action_class")
            )
            actual_authority_impacts.add(
                ensure_str(
                    action.get("authority_impact"),
                    "case.alternative_actions[].authority_impact",
                )
            )

    required_primary = {
        ensure_str(item, "manifest.required_primary_action_classes[]")
        for item in ensure_list(
            manifest.get("required_primary_action_classes"),
            "manifest.required_primary_action_classes",
        )
    }
    missing_primary = sorted(required_primary - actual_primary_actions)
    if missing_primary:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.primary_action_coverage_missing",
                message=f"missing required primary_action_class entries: {', '.join(missing_primary)}",
                remediation="Add fixtures whose primary action covers every required class.",
            )
        )

    required_alternative = {
        ensure_str(item, "manifest.required_alternative_action_classes[]")
        for item in ensure_list(
            manifest.get("required_alternative_action_classes"),
            "manifest.required_alternative_action_classes",
        )
    }
    missing_alternative = sorted(required_alternative - actual_alternative_actions)
    if missing_alternative:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.alternative_action_coverage_missing",
                message=(
                    f"missing required alternative_action_class entries: {', '.join(missing_alternative)}"
                ),
                remediation="Add fixtures whose alternative actions cover every required class.",
            )
        )

    required_impacts = {
        ensure_str(item, "manifest.required_authority_impact_classes[]")
        for item in ensure_list(
            manifest.get("required_authority_impact_classes"),
            "manifest.required_authority_impact_classes",
        )
    }
    missing_impacts = sorted(required_impacts - actual_authority_impacts)
    if missing_impacts:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.authority_impact_coverage_missing",
                message=f"missing required authority_impact entries: {', '.join(missing_impacts)}",
                remediation="Add fixtures that exercise every authority-impact class.",
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
                check_id="remote_drift_repair_beta.acceptance_coverage_missing",
                message=f"missing required acceptance states: {', '.join(missing_acceptance)}",
                remediation="Add fixtures that exercise every protected acceptance state.",
            )
        )


def validate_diagnostics_packet(
    repo_root: Path,
    packet: dict[str, Any],
    cases: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    refs = ensure_list(packet.get("guidance_record_refs"), "packet.guidance_record_refs")
    for idx, ref in enumerate(refs):
        add_missing_ref(
            findings,
            repo_root,
            ensure_str(ref, f"packet.guidance_record_refs[{idx}]"),
            "packet.guidance_record_refs",
        )
    fixture_paths = {ensure_str(ref, "ref") for ref in refs}
    case_paths = {
        f"fixtures/runtime/m3/remote_drift/{Path(case_id_of(case)).name.split(':')[-1]}.json"
        for case in cases
    }
    if not case_paths.issubset(fixture_paths):
        missing = sorted(case_paths - fixture_paths)
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.diagnostics_packet_missing_record",
                message=f"diagnostics_packet does not reference all cases: {missing}",
                remediation="List every fixture case in the diagnostics packet so support reads the full bundle.",
            )
        )

    reason_summary = set(
        ensure_str(item, "packet.drift_reason_summary_tokens[]")
        for item in ensure_list(
            packet.get("drift_reason_summary_tokens"), "packet.drift_reason_summary_tokens"
        )
    )
    derived_reasons: set[str] = set()
    derived_actions: set[str] = set()
    derived_any_fails_closed = False
    derived_any_requires_reapproval = False
    for case in cases:
        for reason in ensure_list(case.get("drift_reasons"), "case.drift_reasons"):
            derived_reasons.add(ensure_str(reason, "case.drift_reasons[]"))
        primary = ensure_dict(case.get("primary_action"), "case.primary_action")
        derived_actions.add(
            ensure_str(primary.get("action_class"), "case.primary_action.action_class")
        )
        for action in ensure_list(case.get("alternative_actions"), "case.alternative_actions"):
            derived_actions.add(
                ensure_str(action.get("action_class"), "case.alternative_actions[].action_class")
            )
        if bool(case.get("fails_closed_for_mutation")):
            derived_any_fails_closed = True
        if bool(case.get("any_action_requires_reapproval")):
            derived_any_requires_reapproval = True

    if reason_summary != derived_reasons:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.diagnostics_packet_reason_summary_mismatch",
                message="diagnostics_packet drift_reason_summary_tokens must match the union of case drift_reasons",
                remediation="Regenerate the diagnostics packet from the canonical case set.",
                details={"actual": sorted(reason_summary), "expected": sorted(derived_reasons)},
            )
        )

    action_summary = set(
        ensure_str(item, "packet.repair_action_summary_tokens[]")
        for item in ensure_list(
            packet.get("repair_action_summary_tokens"), "packet.repair_action_summary_tokens"
        )
    )
    if action_summary != derived_actions:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.diagnostics_packet_action_summary_mismatch",
                message="diagnostics_packet repair_action_summary_tokens must match the union of case repair actions",
                remediation="Regenerate the diagnostics packet from the canonical case set.",
                details={"actual": sorted(action_summary), "expected": sorted(derived_actions)},
            )
        )

    if bool(packet.get("any_record_fails_closed_for_mutation")) != derived_any_fails_closed:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.diagnostics_packet_fails_closed_mismatch",
                message="diagnostics_packet any_record_fails_closed_for_mutation does not match derived value",
                remediation="Recompute the diagnostics packet flag from the case set.",
            )
        )
    if bool(packet.get("any_record_requires_reapproval")) != derived_any_requires_reapproval:
        findings.append(
            Finding(
                severity="error",
                check_id="remote_drift_repair_beta.diagnostics_packet_requires_reapproval_mismatch",
                message="diagnostics_packet any_record_requires_reapproval does not match derived value",
                remediation="Recompute the diagnostics packet flag from the case set.",
            )
        )


def render_summary(findings: list[Finding], cases: list[dict[str, Any]]) -> str:
    lines = ["[remote-drift-repair-beta] summary"]
    if not findings:
        lines.append(
            f"[remote-drift-repair-beta] OK: {len(cases)} fixture cases validated"
        )
        return "\n".join(lines) + "\n"
    lines.append(f"[remote-drift-repair-beta] FAIL: {len(findings)} finding(s)")
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
    packet_ref = ensure_str(
        manifest.get("diagnostics_packet_ref"), "manifest.diagnostics_packet_ref"
    )
    packet = ensure_dict(load_json(repo_root / packet_ref), packet_ref)

    findings: list[Finding] = []
    validate_manifest(repo_root, manifest, cases, findings)
    for case_ref, case in zip(case_refs, cases):
        findings.extend(schema_validate(schema, case, case_ref))
        findings.extend(validate_case(repo_root, case))
    findings.extend(schema_validate(schema, packet, packet_ref))
    validate_diagnostics_packet(repo_root, packet, cases, findings)

    sys.stdout.write(render_summary(findings, cases))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report = {
            "record_kind": "remote_drift_repair_beta_validation_report",
            "schema_ref": args.schema,
            "manifest_ref": args.manifest,
            "case_count": len(cases),
            "findings": [finding.as_report() for finding in findings],
        }
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    return 1 if any(finding.severity == "error" for finding in findings) else 0


if __name__ == "__main__":
    raise SystemExit(main())
