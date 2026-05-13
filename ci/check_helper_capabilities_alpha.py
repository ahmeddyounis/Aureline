#!/usr/bin/env python3
"""Validate and render the helper capability negotiation alpha fixtures."""

from __future__ import annotations

import argparse
import copy
import json
import sys
from dataclasses import asdict, dataclass, field
from pathlib import Path
from typing import Any

import yaml
from jsonschema import Draft202012Validator, FormatChecker


DEFAULT_SCHEMA_REL = "schemas/remote/helper_capabilities_alpha.schema.json"
DEFAULT_MANIFEST_REL = "fixtures/remote/mixed_version_drift_alpha/manifest.yaml"
DEFAULT_COMPAT_MATRIX_REL = "artifacts/compat/qualification_matrix_seed.yaml"
DEFAULT_SKEW_REGISTER_REL = "artifacts/compat/version_skew_register.yaml"
DEFAULT_SKEW_WINDOWS_REL = "artifacts/compat/skew_windows.yaml"

MUTATING_EFFECT_CLASSES = {
    "remote_write",
    "process",
    "terminal",
    "debug",
    "ai_runtime",
    "managed_control_plane_write",
}

BLOCKED_OR_RETRY_LABELS = {"retry_required", "unsupported_skew"}
NON_MUTATING_LABELS = {"limited", "retry_required", "unsupported_skew"}


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
    parser.add_argument("--compat-matrix", default=DEFAULT_COMPAT_MATRIX_REL)
    parser.add_argument("--skew-register", default=DEFAULT_SKEW_REGISTER_REL)
    parser.add_argument("--skew-windows", default=DEFAULT_SKEW_WINDOWS_REL)
    parser.add_argument("--report", default=None)
    parser.add_argument(
        "--render-negotiation-projection",
        action="store_true",
        help="Print the support/export-safe negotiation projection.",
    )
    parser.add_argument(
        "--force-drill",
        default=None,
        help=(
            "Replay one failure drill as '<case_id>:<drill_id>'. The command "
            "exits 0 only when the forced input reproduces the drill's "
            "expected_check_id."
        ),
    )
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


def strip_fragment(ref: str) -> str:
    return ref.split("#", 1)[0].strip()


def looks_like_path(ref: str) -> bool:
    clean = strip_fragment(ref)
    if not clean:
        return False
    if "/" in clean:
        return True
    return clean.endswith((".json", ".yaml", ".yml", ".md", ".py", ".rs", ".toml"))


def artifact_ref_exists(repo_root: Path, ref: str) -> bool:
    clean = strip_fragment(ref)
    return bool(clean) and (repo_root / clean).exists()


def add_missing_ref(
    findings: list[Finding], repo_root: Path, ref: str, label: str
) -> None:
    if looks_like_path(ref) and not artifact_ref_exists(repo_root, ref):
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
                check_id="helper_capabilities.schema.validation_failed",
                message=f"{path}: {error.message}",
                remediation="Update the fixture or schema so the alpha envelope validates.",
                details={"schema_path": list(error.schema_path)},
            )
        )
    return findings


def load_compat_row_ids(repo_root: Path, rel: str) -> set[str]:
    payload = ensure_dict(load_yaml(repo_root / rel), rel)
    rows = ensure_list(payload.get("qualification_rows"), f"{rel}.qualification_rows")
    return {
        ensure_str(ensure_dict(row, f"{rel}.qualification_rows[]").get("row_id"), "row_id")
        for row in rows
    }


def load_skew_case_statuses(repo_root: Path, rel: str) -> tuple[dict[str, str], dict[str, str]]:
    payload = ensure_dict(load_yaml(repo_root / rel), rel)
    register = ensure_list(payload.get("register"), f"{rel}.register")
    case_status: dict[str, str] = {}
    case_register: dict[str, str] = {}
    for idx, raw_entry in enumerate(register):
        entry = ensure_dict(raw_entry, f"{rel}.register[{idx}]")
        register_id = ensure_str(entry.get("register_id"), f"{rel}.register[{idx}].register_id")
        for bucket in ("supported", "best_effort", "untested", "unsupported"):
            cases = entry.get(bucket) or []
            if not isinstance(cases, list):
                continue
            for jdx, raw_case in enumerate(cases):
                case = ensure_dict(raw_case, f"{rel}.register[{idx}].{bucket}[{jdx}]")
                case_id = ensure_str(
                    case.get("skew_case_id"),
                    f"{rel}.register[{idx}].{bucket}[{jdx}].skew_case_id",
                )
                case_status[case_id] = bucket
                case_register[case_id] = register_id
    return case_status, case_register


def load_skew_windows(repo_root: Path, rel: str) -> dict[str, dict[str, Any]]:
    payload = ensure_dict(load_yaml(repo_root / rel), rel)
    declarations = ensure_list(payload.get("declarations"), f"{rel}.declarations")
    windows: dict[str, dict[str, Any]] = {}
    for idx, raw_decl in enumerate(declarations):
        decl = ensure_dict(raw_decl, f"{rel}.declarations[{idx}]")
        window_id = ensure_str(decl.get("skew_window_id"), f"{rel}.declarations[{idx}].skew_window_id")
        windows[window_id] = decl
    return windows


def load_manifest(repo_root: Path, rel: str) -> dict[str, Any]:
    manifest = ensure_dict(load_yaml(repo_root / rel), rel)
    return manifest


def load_cases(repo_root: Path, manifest: dict[str, Any]) -> list[dict[str, Any]]:
    cases: list[dict[str, Any]] = []
    for idx, ref in enumerate(ensure_list(manifest.get("case_refs"), "manifest.case_refs")):
        case_ref = ensure_str(ref, f"manifest.case_refs[{idx}]")
        case = ensure_dict(load_yaml(repo_root / case_ref), case_ref)
        cases.append(case)
    return cases


def drill_index(manifest: dict[str, Any]) -> dict[tuple[str, str], dict[str, Any]]:
    drills: dict[tuple[str, str], dict[str, Any]] = {}
    for idx, raw_drill in enumerate(manifest.get("failure_drills") or []):
        drill = ensure_dict(raw_drill, f"manifest.failure_drills[{idx}]")
        case_id = ensure_str(drill.get("case_id"), f"manifest.failure_drills[{idx}].case_id")
        drill_id = ensure_str(drill.get("drill_id"), f"manifest.failure_drills[{idx}].drill_id")
        drills[(case_id, drill_id)] = drill
    return drills


def parse_force_drill(value: str) -> tuple[str, str]:
    marker = ":"
    if marker not in value:
        raise SystemExit("--force-drill must be '<case_id>:<drill_id>'")
    case_id, drill_id = value.rsplit(marker, 1)
    if not case_id or not drill_id:
        raise SystemExit("--force-drill must be '<case_id>:<drill_id>'")
    return case_id, drill_id


def apply_forced_input(case: dict[str, Any], forced_input: dict[str, Any]) -> dict[str, Any]:
    forced = copy.deepcopy(case)
    negotiation = ensure_dict(forced.get("negotiation"), "case.negotiation")
    if "rewrite_result_label" in forced_input:
        negotiation["result_label"] = forced_input["rewrite_result_label"]
    if "set_mutation_allowed" in forced_input:
        negotiation["mutation_allowed"] = forced_input["set_mutation_allowed"]
    if forced_input.get("drop_dropped_capabilities"):
        negotiation["dropped_capabilities"] = []
    if "append_negotiated_capability" in forced_input:
        negotiated = list(negotiation.get("negotiated_capabilities") or [])
        negotiated.append(forced_input["append_negotiated_capability"])
        negotiation["negotiated_capabilities"] = negotiated
    return forced


def case_id(case: dict[str, Any]) -> str:
    expectations = ensure_dict(case.get("harness_expectations"), "case.harness_expectations")
    return ensure_str(expectations.get("case_id"), "case.harness_expectations.case_id")


def requested_by_capability(case: dict[str, Any]) -> dict[str, dict[str, Any]]:
    requests: dict[str, dict[str, Any]] = {}
    for idx, raw_request in enumerate(
        ensure_list(case.get("requested_capabilities"), f"{case_id(case)}.requested_capabilities")
    ):
        request = ensure_dict(raw_request, f"{case_id(case)}.requested_capabilities[{idx}]")
        capability = ensure_str(request.get("capability"), f"{case_id(case)}.requested_capabilities[{idx}].capability")
        requests[capability] = request
    return requests


def validate_manifest(
    repo_root: Path,
    manifest: dict[str, Any],
    cases: list[dict[str, Any]],
    findings: list[Finding],
) -> None:
    for ref_label in ("schema_ref", "doc_ref"):
        add_missing_ref(findings, repo_root, ensure_str(manifest.get(ref_label), f"manifest.{ref_label}"), f"manifest.{ref_label}")
    validator = ensure_dict(manifest.get("validator"), "manifest.validator")
    add_missing_ref(
        findings,
        repo_root,
        ensure_str(validator.get("script_ref"), "manifest.validator.script_ref"),
        "manifest.validator.script_ref",
    )
    for idx, ref in enumerate(ensure_list(manifest.get("canonical_source_refs"), "manifest.canonical_source_refs")):
        add_missing_ref(findings, repo_root, ensure_str(ref, f"manifest.canonical_source_refs[{idx}]"), "manifest.canonical_source_refs")
    for idx, ref in enumerate(ensure_list(manifest.get("case_refs"), "manifest.case_refs")):
        add_missing_ref(findings, repo_root, ensure_str(ref, f"manifest.case_refs[{idx}]"), "manifest.case_refs")

    actual_case_ids = {case_id(case) for case in cases}
    expected_case_ids = {
        ensure_str(item, "manifest.expected_case_ids[]")
        for item in ensure_list(manifest.get("expected_case_ids"), "manifest.expected_case_ids")
    }
    if actual_case_ids != expected_case_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.case_ids.mismatch",
                message="manifest expected_case_ids must exactly match loaded case ids",
                remediation="Update the manifest or fixture set so the protected cases are explicit.",
                details={
                    "actual": sorted(actual_case_ids),
                    "expected": sorted(expected_case_ids),
                },
            )
        )

    actual_labels = {
        ensure_dict(case.get("negotiation"), f"{case_id(case)}.negotiation").get("result_label")
        for case in cases
    }
    required_labels = {
        ensure_str(item, "manifest.required_result_labels[]")
        for item in ensure_list(manifest.get("required_result_labels"), "manifest.required_result_labels")
    }
    missing_labels = sorted(required_labels - actual_labels)
    if missing_labels:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.result_label_coverage_missing",
                message=f"missing required result labels: {', '.join(missing_labels)}",
                remediation="Add or repair fixtures so supported, limited, retry_required, and unsupported_skew are covered.",
            )
        )

    acceptance_seen: set[str] = set()
    for case in cases:
        expectations = ensure_dict(case.get("harness_expectations"), f"{case_id(case)}.harness_expectations")
        acceptance_seen.update(
            ensure_str(item, f"{case_id(case)}.harness_expectations.acceptance_states[]")
            for item in ensure_list(expectations.get("acceptance_states"), f"{case_id(case)}.acceptance_states")
        )
    required_acceptance = {
        ensure_str(item, "manifest.required_acceptance_states[]")
        for item in ensure_list(manifest.get("required_acceptance_states"), "manifest.required_acceptance_states")
    }
    missing_acceptance = sorted(required_acceptance - acceptance_seen)
    if missing_acceptance:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.acceptance_coverage_missing",
                message=f"missing required acceptance states: {', '.join(missing_acceptance)}",
                remediation="Add fixtures that exercise every protected acceptance state.",
            )
        )


def validate_case(
    repo_root: Path,
    case: dict[str, Any],
    *,
    compat_row_ids: set[str],
    skew_case_statuses: dict[str, str],
    skew_case_registers: dict[str, str],
    skew_windows: dict[str, dict[str, Any]],
) -> list[Finding]:
    findings: list[Finding] = []
    cid = case_id(case)
    boundary = ensure_dict(case.get("boundary"), f"{cid}.boundary")
    negotiation = ensure_dict(case.get("negotiation"), f"{cid}.negotiation")
    drift = ensure_dict(case.get("drift"), f"{cid}.drift")
    expectations = ensure_dict(case.get("harness_expectations"), f"{cid}.harness_expectations")

    compat_ref = ensure_str(boundary.get("compatibility_row_ref"), f"{cid}.boundary.compatibility_row_ref")
    if compat_ref not in compat_row_ids:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.compatibility_row_missing",
                message=f"{cid} references unknown compatibility row {compat_ref}",
                remediation="Use a row from artifacts/compat/qualification_matrix_seed.yaml.",
                ref=cid,
            )
        )

    skew_case_ref = ensure_str(boundary.get("skew_case_ref"), f"{cid}.boundary.skew_case_ref")
    declared_status = ensure_str(boundary.get("skew_status_class"), f"{cid}.boundary.skew_status_class")
    actual_status = skew_case_statuses.get(skew_case_ref)
    if actual_status is None:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.skew_case_missing",
                message=f"{cid} references unknown skew case {skew_case_ref}",
                remediation="Use a skew_case from artifacts/compat/version_skew_register.yaml.",
                ref=cid,
            )
        )
    elif declared_status != actual_status:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.skew_status_mismatch",
                message=f"{cid} declares skew_status_class={declared_status}, but the register bucket is {actual_status}",
                remediation="Align the fixture status with the version-skew register.",
                ref=cid,
            )
        )

    skew_window_ref = ensure_str(boundary.get("skew_window_ref"), f"{cid}.boundary.skew_window_ref")
    window = skew_windows.get(skew_window_ref)
    if window is None:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.skew_window_missing",
                message=f"{cid} references unknown skew window {skew_window_ref}",
                remediation="Use a declaration from artifacts/compat/skew_windows.yaml.",
                ref=cid,
            )
        )
    else:
        if window.get("qualification_row_ref") != compat_ref:
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.skew_window_compat_mismatch",
                    message=f"{cid} skew window does not bind to {compat_ref}",
                    remediation="Use the skew-window declaration that binds the remote attach compatibility row.",
                    ref=cid,
                )
            )
        expected_register = window.get("version_skew_register_ref")
        actual_register = skew_case_registers.get(skew_case_ref)
        if actual_register is not None and expected_register != actual_register:
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.skew_window_register_mismatch",
                    message=f"{cid} skew case belongs to {actual_register}, not {expected_register}",
                    remediation="Use a skew case from the window's declared version-skew register.",
                    ref=cid,
                )
            )

    for group_name in ("schema_refs", "fixture_refs", "doc_refs", "compat_refs", "runtime_refs"):
        evidence = ensure_dict(case.get("evidence_refs"), f"{cid}.evidence_refs")
        for idx, ref in enumerate(ensure_list(evidence.get(group_name), f"{cid}.evidence_refs.{group_name}")):
            add_missing_ref(
                findings,
                repo_root,
                ensure_str(ref, f"{cid}.evidence_refs.{group_name}[{idx}]"),
                f"{cid}.evidence_refs.{group_name}",
            )

    for boundary_ref_name in (
        "mixed_version_envelope_ref",
        "helper_version_negotiation_ref",
        "attach_session_ref",
        "remote_agent_hello_ref",
        "prebuild_descriptor_ref",
    ):
        raw_ref = boundary.get(boundary_ref_name)
        if isinstance(raw_ref, str):
            add_missing_ref(findings, repo_root, raw_ref, f"{cid}.boundary.{boundary_ref_name}")

    if negotiation.get("result_label") != expectations.get("expected_result_label"):
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.expected_result_label_mismatch",
                message=f"{cid} result_label does not match harness expectation",
                remediation="Update the fixture result or expectation together.",
                ref=cid,
                details={
                    "actual": negotiation.get("result_label"),
                    "expected": expectations.get("expected_result_label"),
                },
            )
        )
    if negotiation.get("decision_class") != expectations.get("expected_decision_class"):
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.expected_decision_mismatch",
                message=f"{cid} decision_class does not match harness expectation",
                remediation="Update the fixture decision or expectation together.",
                ref=cid,
            )
        )
    if drift.get("safe_continuation") != expectations.get("expected_safe_continuation"):
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.expected_safe_continuation_mismatch",
                message=f"{cid} safe_continuation does not match harness expectation",
                remediation="Update the fixture drift state or expectation together.",
                ref=cid,
            )
        )

    requests = requested_by_capability(case)
    client_caps = set(ensure_dict(case.get("client"), f"{cid}.client").get("capability_set") or [])
    helper_caps = set(ensure_dict(case.get("helper"), f"{cid}.helper").get("capability_set") or [])
    negotiated = set(negotiation.get("negotiated_capabilities") or [])
    intersection = client_caps & helper_caps
    negotiated_outside_intersection = sorted(negotiated - intersection)
    if negotiated_outside_intersection:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.negotiated_capability_not_in_intersection",
                message=f"{cid} negotiates capabilities not offered by both sides: {', '.join(negotiated_outside_intersection)}",
                remediation="Negotiate only the client/helper capability intersection.",
                ref=cid,
            )
        )

    dropped = ensure_list(negotiation.get("dropped_capabilities"), f"{cid}.negotiation.dropped_capabilities")
    dropped_caps = {
        ensure_str(ensure_dict(item, f"{cid}.negotiation.dropped_capabilities[]").get("capability"), "dropped.capability")
        for item in dropped
    }
    required_caps = {
        capability
        for capability, request in requests.items()
        if request.get("requirement") == "required"
    }
    missing_required = sorted(required_caps - negotiated)
    for capability in missing_required:
        if capability not in dropped_caps:
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.required_missing_without_drop_reason",
                    message=f"{cid} missing required capability {capability} without a dropped_capability reason",
                    remediation="Record the missing capability and a typed reason.",
                    ref=cid,
                )
            )

    result_label = ensure_str(negotiation.get("result_label"), f"{cid}.negotiation.result_label")
    mutation_allowed = bool(negotiation.get("mutation_allowed"))
    mutating_negotiated = sorted(
        capability
        for capability in negotiated
        if (requests.get(capability) or {}).get("effect_class") in MUTATING_EFFECT_CLASSES
    )

    if result_label == "supported" and missing_required:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.supported_label_has_missing_required_capability",
                message=f"{cid} is supported but missing required capabilities: {', '.join(missing_required)}",
                remediation="Use limited, retry_required, or unsupported_skew until all required capabilities are admitted.",
                ref=cid,
            )
        )

    if result_label in NON_MUTATING_LABELS and mutation_allowed:
        check_id = (
            "helper_capabilities.blocked_state_allows_mutation"
            if result_label in BLOCKED_OR_RETRY_LABELS
            else "helper_capabilities.limited_state_allows_mutation"
        )
        findings.append(
            Finding(
                severity="error",
                check_id=check_id,
                message=f"{cid} has result_label={result_label} but mutation_allowed=true",
                remediation="Keep mutation disabled until the negotiated result is supported.",
                ref=cid,
            )
        )

    if result_label in BLOCKED_OR_RETRY_LABELS and mutating_negotiated:
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.blocked_state_negotiates_mutating_capability",
                message=f"{cid} negotiates mutating capabilities while {result_label}: {', '.join(mutating_negotiated)}",
                remediation="Drop mutating capabilities when the lane is retry-required or unsupported.",
                ref=cid,
            )
        )

    drift_label = ensure_str(drift.get("drift_label"), f"{cid}.drift.drift_label")
    if result_label == "unsupported_skew":
        if declared_status != "unsupported" or drift_label != "unsupported_skew":
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.unsupported_skew_label_mismatch",
                    message=f"{cid} unsupported skew must align result, drift, and register status",
                    remediation="Use unsupported_skew for both result and drift when the skew-register bucket is unsupported.",
                    ref=cid,
                )
            )
        if negotiation.get("decision_class") not in {"block_attach", "require_upgrade_or_repin"}:
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.unsupported_skew_decision_mismatch",
                    message=f"{cid} unsupported skew must block attach or require upgrade/repin",
                    remediation="Use block_attach or require_upgrade_or_repin.",
                    ref=cid,
                )
            )
    elif declared_status == "unsupported":
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.unsupported_skew_label_mismatch",
                message=f"{cid} is in the unsupported skew-register bucket but result_label={result_label}",
                remediation="Use unsupported_skew until the canonical register moves the case.",
                ref=cid,
            )
        )

    retryability = ensure_dict(negotiation.get("retryability"), f"{cid}.negotiation.retryability")
    if result_label == "retry_required":
        retry_class = ensure_str(retryability.get("retry_class"), f"{cid}.negotiation.retryability.retry_class")
        if declared_status != "untested" and not retryability.get("probe_required"):
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.retry_required_label_mismatch",
                    message=f"{cid} retry_required must be backed by an untested skew case or an explicit probe requirement",
                    remediation="Use retry_required only for probe or reattach states.",
                    ref=cid,
                )
            )
        if retry_class == "no_retry_required":
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.retry_required_without_retry",
                    message=f"{cid} retry_required has retry_class=no_retry_required",
                    remediation="Set a concrete retry class and recovery action.",
                    ref=cid,
                )
            )
    elif declared_status == "untested":
        findings.append(
            Finding(
                severity="error",
                check_id="helper_capabilities.retry_required_label_mismatch",
                message=f"{cid} is untested in the skew register but result_label={result_label}",
                remediation="Keep the case retry_required until a probe or reattach result moves it.",
                ref=cid,
            )
        )

    return findings


def build_projection(cases: list[dict[str, Any]]) -> list[dict[str, Any]]:
    projection: list[dict[str, Any]] = []
    for case in cases:
        boundary = ensure_dict(case.get("boundary"), "case.boundary")
        client = ensure_dict(case.get("client"), "case.client")
        helper = ensure_dict(case.get("helper"), "case.helper")
        negotiation = ensure_dict(case.get("negotiation"), "case.negotiation")
        drift = ensure_dict(case.get("drift"), "case.drift")
        dropped = []
        for raw_drop in negotiation.get("dropped_capabilities") or []:
            drop = ensure_dict(raw_drop, "drop")
            dropped.append(
                {
                    "capability": drop.get("capability"),
                    "reason_class": drop.get("reason_class"),
                    "retryable": drop.get("retryable"),
                }
            )
        projection.append(
            {
                "case_id": case_id(case),
                "envelope_id": case.get("envelope_id"),
                "client_version": client.get("version"),
                "helper_version": helper.get("version"),
                "skew_case_ref": boundary.get("skew_case_ref"),
                "result_label": negotiation.get("result_label"),
                "decision_class": negotiation.get("decision_class"),
                "effective_posture": negotiation.get("effective_posture"),
                "safe_continuation": drift.get("safe_continuation"),
                "repair_actions": drift.get("repair_actions"),
                "dropped_capabilities": dropped,
            }
        )
    return projection


def render_summary(findings: list[Finding], cases: list[dict[str, Any]]) -> str:
    lines = ["[helper-capabilities-alpha] summary"]
    if not findings:
        lines.append(
            f"[helper-capabilities-alpha] OK: {len(cases)} fixture cases validated"
        )
        return "\n".join(lines) + "\n"
    lines.append(f"[helper-capabilities-alpha] FAIL: {len(findings)} finding(s)")
    for finding in findings:
        ref = f" [{finding.ref}]" if finding.ref else ""
        lines.append(f"- {finding.severity}: {finding.check_id}{ref}: {finding.message}")
    return "\n".join(lines) + "\n"


def main() -> int:
    args = parse_args()
    repo_root = Path(args.repo_root).resolve()

    schema = ensure_dict(load_json(repo_root / args.schema), args.schema)
    manifest = load_manifest(repo_root, args.manifest)
    cases = load_cases(repo_root, manifest)

    compat_row_ids = load_compat_row_ids(repo_root, args.compat_matrix)
    skew_case_statuses, skew_case_registers = load_skew_case_statuses(
        repo_root, args.skew_register
    )
    skew_windows = load_skew_windows(repo_root, args.skew_windows)

    findings: list[Finding] = []
    validate_manifest(repo_root, manifest, cases, findings)

    forced_case_id: str | None = None
    expected_forced_check_id: str | None = None
    forced_reproduced = False
    if args.force_drill:
        forced_case_id, forced_drill_id = parse_force_drill(args.force_drill)
        drills = drill_index(manifest)
        drill = drills.get((forced_case_id, forced_drill_id))
        if drill is None:
            raise SystemExit(f"unknown failure drill: {args.force_drill}")
        expected_forced_check_id = ensure_str(
            drill.get("expected_check_id"), f"{args.force_drill}.expected_check_id"
        )
        forced_input = ensure_dict(drill.get("forced_input"), f"{args.force_drill}.forced_input")
        cases = [
            apply_forced_input(case, forced_input) if case_id(case) == forced_case_id else case
            for case in cases
        ]

    for case in cases:
        findings.extend(schema_validate(schema, case))
        findings.extend(
            validate_case(
                repo_root,
                case,
                compat_row_ids=compat_row_ids,
                skew_case_statuses=skew_case_statuses,
                skew_case_registers=skew_case_registers,
                skew_windows=skew_windows,
            )
        )

    if args.force_drill:
        forced_reproduced = any(
            finding.check_id == expected_forced_check_id
            and (forced_case_id is None or finding.ref == forced_case_id)
            for finding in findings
        )
        if not forced_reproduced:
            findings.append(
                Finding(
                    severity="error",
                    check_id="helper_capabilities.failure_drill_not_reproduced",
                    message=(
                        f"forced drill {args.force_drill} did not reproduce "
                        f"{expected_forced_check_id}"
                    ),
                    remediation="Fix the drill fixture or harness rule so the forced regression fails loudly.",
                    ref=forced_case_id,
                )
            )
        else:
            findings = [
                finding
                for finding in findings
                if finding.check_id == expected_forced_check_id and finding.ref == forced_case_id
            ]

    if args.render_negotiation_projection:
        sys.stdout.write(json.dumps(build_projection(cases), indent=2, sort_keys=True) + "\n")
    elif args.force_drill and forced_reproduced and expected_forced_check_id:
        sys.stdout.write(
            "[helper-capabilities-alpha] OK forced drill reproduced "
            f"{expected_forced_check_id}\n"
        )
    else:
        sys.stdout.write(render_summary(findings, cases))

    if args.report:
        report_path = repo_root / args.report
        report_path.parent.mkdir(parents=True, exist_ok=True)
        report = {
            "record_kind": "helper_capabilities_alpha_validation_report",
            "schema_ref": args.schema,
            "manifest_ref": args.manifest,
            "case_count": len(cases),
            "forced_drill": args.force_drill,
            "findings": [finding.as_report() for finding in findings],
            "projection": build_projection(cases),
        }
        report_path.write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")

    has_errors = any(finding.severity == "error" for finding in findings)
    if args.force_drill and expected_forced_check_id:
        return 0 if forced_reproduced else 1
    return 1 if has_errors else 0


if __name__ == "__main__":
    raise SystemExit(main())
