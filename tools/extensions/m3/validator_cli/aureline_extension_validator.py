#!/usr/bin/env python3
"""Validate beta extension manifests and checked-in conformance fixtures."""

from __future__ import annotations

import argparse
import json
import re
import sys
from dataclasses import asdict, dataclass
from pathlib import Path
from typing import Any


REPORT_SCHEMA_VERSION = 1
MANIFEST_SCHEMA_VERSION = 1
VALIDATOR_ID = "aureline.extension.validator.beta"
VALIDATOR_VERSION = "0.1.0"

PACKAGE_RE = re.compile(r"^[a-z][a-z0-9-]*(?:\.[a-z0-9-]+)+$")
SEMVER_RE = re.compile(
    r"^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$"
)
WORLD_RE = re.compile(r"^aureline:[a-z][a-z0-9-]*[a-z0-9]@[0-9]+\.[0-9]+\.[0-9]+$")

PERMISSION_SCOPES = {
    "filesystem_read",
    "filesystem_write",
    "shell_execute",
    "network_egress",
    "ai_provider_access",
    "connected_provider_access",
    "secret_handle_use",
    "workspace_settings_read",
    "workspace_settings_write",
    "execution_context_bind",
    "subscription_subscribe",
    "ui_command_contribute",
    "capability_inherit",
}
PRIVILEGED_SCOPES = {
    "filesystem_write",
    "shell_execute",
    "network_egress",
    "secret_handle_use",
    "workspace_settings_write",
    "execution_context_bind",
}
TRUST_MODE_CLASSES = {
    "allowed_in_trusted_workspace",
    "read_only_degrade",
    "disabled_in_restricted_mode",
    "explicit_approval_required",
}
RUNTIME_ORIGINS = {
    "wasm",
    "external_host",
    "helper_binary",
    "remote_side_component",
    "bridge",
}
HOST_CONTRACT_FAMILIES = {
    "wasm_component_model",
    "wasm_core_module",
    "external_host_process",
    "helper_binary",
    "remote_side_component",
    "compatibility_bridge",
}
HOST_ABI_WINDOWS = {
    "component_model_abi_window_beta_1",
    "core_module_abi_window_beta_1",
    "external_host_process_window_documented",
    "compatibility_bridge_window_documented",
}
SUPPORT_CLASSES = {
    "certified",
    "supported",
    "limited",
    "experimental",
    "community",
    "retest_pending",
    "evidence_stale",
    "unsupported",
}
BRIDGE_STATES = {"native", "bridge", "partial", "unsupported", "retest_pending"}
LIFECYCLE_STATES = {
    "verified",
    "resolved",
    "activated",
    "degraded",
    "disabled",
    "removed",
}
DEGRADED_BEHAVIORS = {
    "read_only_degrade",
    "disable_background_work",
    "disable_until_review",
    "quarantine_pending_review",
}
SCENARIO_CLASSES = {
    "install",
    "activation",
    "permission_prompt",
    "degraded_path",
    "disable_rollback",
}
NETWORK_ENDPOINT_CLASSES = {
    "metadata_fetch",
    "package_registry",
    "vendor_api",
    "user_configured_url",
}
RED_FLAGS = {
    "missing_manifest_shape",
    "opaque_publisher_identity",
    "undeclared_privileged_permission",
    "missing_lifecycle_metadata",
    "incompatible_sdk_target",
    "missing_disable_or_rollback",
}


@dataclass
class Finding:
    check_id: str
    suite: str
    status: str
    severity: str
    message: str
    field: str | None = None
    fix: str | None = None

    def as_report(self) -> dict[str, Any]:
        payload = asdict(self)
        if payload["field"] is None:
            payload.pop("field")
        if payload["fix"] is None:
            payload.pop("fix")
        return payload


def add(
    findings: list[Finding],
    *,
    check_id: str,
    suite: str,
    ok: bool,
    severity: str = "blocker",
    message: str,
    field: str | None = None,
    fix: str | None = None,
) -> None:
    findings.append(
        Finding(
            check_id=check_id,
            suite=suite,
            status="pass" if ok else "fail",
            severity=severity,
            message=message,
            field=field,
            fix=fix,
        )
    )


def load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise SystemExit(f"missing JSON file: {path}") from exc
    except json.JSONDecodeError as exc:
        raise SystemExit(f"invalid JSON at {path}: {exc}") from exc


def ensure_obj(value: Any, label: str, findings: list[Finding]) -> dict[str, Any]:
    if isinstance(value, dict):
        return value
    add(
        findings,
        check_id="manifest_shape.object_required",
        suite="manifest_shape",
        ok=False,
        message=f"{label} must be a JSON object",
        field=label,
        fix="Use an object for this manifest section.",
    )
    return {}


def as_list(value: Any) -> list[Any]:
    return value if isinstance(value, list) else []


def non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value.strip())


def normalize_rel(path: Path, repo_root: Path) -> str:
    try:
        return path.resolve().relative_to(repo_root.resolve()).as_posix()
    except ValueError:
        return path.as_posix()


def host_family_allowed(runtime_origin: str, host_family: str) -> bool:
    allowed = {
        "wasm": {"wasm_component_model", "wasm_core_module"},
        "external_host": {"external_host_process"},
        "helper_binary": {"helper_binary"},
        "remote_side_component": {"remote_side_component"},
        "bridge": {"compatibility_bridge"},
    }
    return host_family in allowed.get(runtime_origin, set())


def validate_manifest_payload(
    payload: Any,
    *,
    subject_manifest_ref: str,
    generated_at: str,
) -> dict[str, Any]:
    findings: list[Finding] = []
    manifest = ensure_obj(payload, "(root)", findings)

    required_top = {
        "manifest_version",
        "package_id",
        "publisher_id",
        "version",
        "runtime_origin",
        "host_contract_family",
        "sdk",
        "compatibility",
        "permissions",
        "lifecycle",
        "conformance",
    }
    missing_top = sorted(required_top - set(manifest))
    add(
        findings,
        check_id="manifest_shape.required_fields",
        suite="manifest_shape",
        ok=not missing_top,
        message=(
            "manifest carries every required top-level section"
            if not missing_top
            else "manifest is missing required top-level fields: " + ", ".join(missing_top)
        ),
        fix="Add the missing section before publishing.",
    )

    add(
        findings,
        check_id="manifest_shape.schema_version",
        suite="manifest_shape",
        ok=manifest.get("manifest_version") == MANIFEST_SCHEMA_VERSION,
        message="manifest_version pins the beta manifest schema",
        field="manifest_version",
        fix=f"Set manifest_version to {MANIFEST_SCHEMA_VERSION}.",
    )

    package_id = manifest.get("package_id")
    publisher_id = manifest.get("publisher_id")
    extension_version = manifest.get("version")
    runtime_origin = manifest.get("runtime_origin")
    host_family = manifest.get("host_contract_family")

    add(
        findings,
        check_id="manifest_shape.package_id",
        suite="manifest_shape",
        ok=non_empty_string(package_id) and bool(PACKAGE_RE.match(package_id)),
        message="package_id is a stable reverse-DNS style package id",
        field="package_id",
        fix="Use a stable package id such as com.example.tooling.",
    )
    add(
        findings,
        check_id="manifest_shape.publisher_identity",
        suite="manifest_shape",
        ok=non_empty_string(publisher_id) and publisher_id not in {"anonymous", "unknown"},
        message="publisher_id is explicit and not anonymous",
        field="publisher_id",
        fix="Bind the package to a publisher identity before validation.",
    )
    add(
        findings,
        check_id="manifest_shape.version_semver",
        suite="manifest_shape",
        ok=non_empty_string(extension_version) and bool(SEMVER_RE.match(extension_version)),
        message="version follows semver",
        field="version",
        fix="Use MAJOR.MINOR.PATCH with an optional prerelease suffix.",
    )
    add(
        findings,
        check_id="manifest_shape.runtime_origin_known",
        suite="manifest_shape",
        ok=runtime_origin in RUNTIME_ORIGINS,
        message="runtime_origin uses the closed beta vocabulary",
        field="runtime_origin",
        fix="Choose one of: " + ", ".join(sorted(RUNTIME_ORIGINS)),
    )
    add(
        findings,
        check_id="manifest_shape.host_contract_family_known",
        suite="manifest_shape",
        ok=host_family in HOST_CONTRACT_FAMILIES,
        message="host_contract_family uses the closed beta vocabulary",
        field="host_contract_family",
        fix="Choose one of: " + ", ".join(sorted(HOST_CONTRACT_FAMILIES)),
    )

    sdk = ensure_obj(manifest.get("sdk"), "sdk", findings)
    sdk_line_id = sdk.get("line_id")
    sdk_line_semver = sdk.get("line_semver")
    host_abi_window = sdk.get("host_abi_window")
    wit_world_refs = as_list(sdk.get("wit_world_refs"))
    external_host_contract_ref = sdk.get("external_host_contract_ref")
    compatibility = ensure_obj(manifest.get("compatibility"), "compatibility", findings)
    aureline_versions = ensure_obj(compatibility.get("aureline_versions"), "compatibility.aureline_versions", findings)
    platforms = as_list(compatibility.get("platforms"))
    support_class = compatibility.get("support_class")
    bridge_state = compatibility.get("bridge_state")

    add(
        findings,
        check_id="compatibility_targets.sdk_line_id",
        suite="compatibility_targets",
        ok=sdk_line_id == "aureline.sdk.beta",
        message="sdk.line_id targets the published beta SDK line",
        field="sdk.line_id",
        fix="Set sdk.line_id to aureline.sdk.beta for this validator lane.",
    )
    add(
        findings,
        check_id="compatibility_targets.sdk_semver",
        suite="compatibility_targets",
        ok=non_empty_string(sdk_line_semver) and bool(SEMVER_RE.match(sdk_line_semver)),
        message="sdk.line_semver follows semver",
        field="sdk.line_semver",
        fix="Use the SDK line semver published with the beta host.",
    )
    add(
        findings,
        check_id="compatibility_targets.host_abi_window",
        suite="compatibility_targets",
        ok=host_abi_window in HOST_ABI_WINDOWS,
        message="sdk.host_abi_window uses a supported beta ABI window",
        field="sdk.host_abi_window",
        fix="Use a documented beta ABI window for the selected runtime origin.",
    )
    add(
        findings,
        check_id="compatibility_targets.host_family_matches_runtime_origin",
        suite="compatibility_targets",
        ok=isinstance(runtime_origin, str)
        and isinstance(host_family, str)
        and host_family_allowed(runtime_origin, host_family),
        message="host contract family is compatible with runtime origin",
        field="host_contract_family",
        fix="Align runtime_origin and host_contract_family before validation.",
    )
    add(
        findings,
        check_id="compatibility_targets.wit_worlds_declared",
        suite="compatibility_targets",
        ok=(
            runtime_origin != "wasm"
            or (
                bool(wit_world_refs)
                and all(isinstance(ref, str) and WORLD_RE.match(ref) for ref in wit_world_refs)
            )
        ),
        message="wasm manifests declare WIT world refs",
        field="sdk.wit_world_refs",
        fix="List each claimed WIT world as aureline:<world>@MAJOR.MINOR.PATCH.",
    )
    add(
        findings,
        check_id="compatibility_targets.external_host_contract_declared",
        suite="compatibility_targets",
        ok=runtime_origin != "external_host" or non_empty_string(external_host_contract_ref),
        message="external-host manifests declare an external host contract ref",
        field="sdk.external_host_contract_ref",
        fix="Add the supervised external-host contract ref.",
    )
    add(
        findings,
        check_id="compatibility_targets.aureline_version_range",
        suite="compatibility_targets",
        ok=non_empty_string(aureline_versions.get("min"))
        and non_empty_string(aureline_versions.get("max")),
        message="compatibility target declares min and max Aureline versions",
        field="compatibility.aureline_versions",
        fix="Declare the supported host version range.",
    )
    add(
        findings,
        check_id="compatibility_targets.platforms_declared",
        suite="compatibility_targets",
        ok=bool(platforms) and all(non_empty_string(item) for item in platforms),
        message="compatibility target declares at least one platform",
        field="compatibility.platforms",
        fix="List the supported OS/architecture target rows.",
    )
    add(
        findings,
        check_id="compatibility_targets.support_class",
        suite="compatibility_targets",
        ok=support_class in SUPPORT_CLASSES,
        message="support_class uses the shared compatibility vocabulary",
        field="compatibility.support_class",
        fix="Use a support class from the compatibility report vocabulary.",
    )
    add(
        findings,
        check_id="compatibility_targets.bridge_state",
        suite="compatibility_targets",
        ok=bridge_state in BRIDGE_STATES,
        message="bridge_state uses the shared bridge compatibility vocabulary",
        field="compatibility.bridge_state",
        fix="Use native, bridge, partial, unsupported, or retest_pending.",
    )

    permissions = as_list(manifest.get("permissions"))
    add(
        findings,
        check_id="permission_declarations.present",
        suite="permission_declarations",
        ok=bool(permissions),
        message="manifest declares at least one permission entry",
        field="permissions",
        fix="Declare every permission scope the extension may request.",
    )
    declared_scope_targets: set[tuple[str, str]] = set()
    for idx, entry_raw in enumerate(permissions):
        entry = entry_raw if isinstance(entry_raw, dict) else {}
        scope = entry.get("scope")
        targets = as_list(entry.get("targets"))
        purpose = entry.get("purpose")
        trust_mode = entry.get("trust_mode")
        field = f"permissions[{idx}]"
        add(
            findings,
            check_id="permission_declarations.scope_known",
            suite="permission_declarations",
            ok=scope in PERMISSION_SCOPES,
            message=f"{field}.scope uses the closed permission vocabulary",
            field=f"{field}.scope",
            fix="Use a scope from schemas/extensions/permission_manifest.schema.json.",
        )
        add(
            findings,
            check_id="permission_declarations.targets_declared",
            suite="permission_declarations",
            ok=bool(targets) and all(non_empty_string(item) for item in targets),
            message=f"{field}.targets declares at least one target",
            field=f"{field}.targets",
            fix="Declare the target scope the permission applies to.",
        )
        add(
            findings,
            check_id="permission_declarations.purpose_text",
            suite="permission_declarations",
            ok=non_empty_string(purpose),
            message=f"{field}.purpose is non-empty",
            field=f"{field}.purpose",
            fix="Explain why the extension needs this permission.",
        )
        add(
            findings,
            check_id="permission_declarations.trust_mode",
            suite="permission_declarations",
            ok=trust_mode in TRUST_MODE_CLASSES,
            message=f"{field}.trust_mode uses the trust-mode vocabulary",
            field=f"{field}.trust_mode",
            fix="Declare how this permission behaves in restricted mode.",
        )
        prompt = entry.get("prompt")
        prompt_obj = prompt if isinstance(prompt, dict) else {}
        add(
            findings,
            check_id="permission_declarations.prompt_copy",
            suite="permission_declarations",
            ok=(
                scope not in PRIVILEGED_SCOPES
                or (isinstance(prompt, dict) and non_empty_string(prompt_obj.get("summary")))
            ),
            message=f"{field} carries prompt copy when the scope is privileged",
            field=f"{field}.prompt.summary",
            fix="Add prompt.summary for privileged scopes.",
        )
        add(
            findings,
            check_id="permission_declarations.review_required_for_privileged_scope",
            suite="permission_declarations",
            ok=scope not in PRIVILEGED_SCOPES or entry.get("review_required") is True,
            message=f"{field} marks privileged scopes review-required",
            field=f"{field}.review_required",
            fix="Set review_required to true for privileged scopes.",
        )
        if scope == "network_egress":
            network = entry.get("network") if isinstance(entry.get("network"), dict) else {}
            endpoint_class = network.get("endpoint_class")
            add(
                findings,
                check_id="permission_declarations.network_endpoint_class",
                suite="permission_declarations",
                ok=endpoint_class in NETWORK_ENDPOINT_CLASSES,
                message=f"{field}.network.endpoint_class is declared",
                field=f"{field}.network.endpoint_class",
                fix="Declare the network endpoint class.",
            )
        if scope == "secret_handle_use":
            add(
                findings,
                check_id="permission_declarations.secret_handle_only",
                suite="permission_declarations",
                ok=entry.get("handle_only") is True,
                message=f"{field} uses brokered secret handles instead of raw secrets",
                field=f"{field}.handle_only",
                fix="Set handle_only to true for secret access.",
            )
        for target in targets:
            if isinstance(scope, str) and isinstance(target, str):
                declared_scope_targets.add((scope, target))
    add(
        findings,
        check_id="permission_declarations.no_duplicate_scope_target",
        suite="permission_declarations",
        ok=len(declared_scope_targets) == sum(
            len(as_list(entry.get("targets"))) for entry in permissions if isinstance(entry, dict)
        ),
        message="permission scope-target pairs are not duplicated",
        field="permissions",
        fix="Collapse duplicate permission target rows into one entry.",
    )

    lifecycle = ensure_obj(manifest.get("lifecycle"), "lifecycle", findings)
    activation = lifecycle.get("activation") if isinstance(lifecycle.get("activation"), dict) else {}
    degraded_path = lifecycle.get("degraded_path") if isinstance(lifecycle.get("degraded_path"), dict) else {}
    disable = lifecycle.get("disable") if isinstance(lifecycle.get("disable"), dict) else {}
    rollback = lifecycle.get("rollback") if isinstance(lifecycle.get("rollback"), dict) else {}
    state = lifecycle.get("state")
    triggers = as_list(activation.get("triggers"))
    add(
        findings,
        check_id="lifecycle_metadata.state_known",
        suite="lifecycle_metadata",
        ok=state in LIFECYCLE_STATES,
        message="lifecycle.state uses the shared extension lifecycle vocabulary",
        field="lifecycle.state",
        fix="Use verified, resolved, activated, degraded, disabled, or removed.",
    )
    add(
        findings,
        check_id="lifecycle_metadata.activation_triggers",
        suite="lifecycle_metadata",
        ok=bool(triggers) and all(non_empty_string(item) for item in triggers),
        message="activation triggers are declared",
        field="lifecycle.activation.triggers",
        fix="Declare the events that can activate the extension.",
    )
    add(
        findings,
        check_id="lifecycle_metadata.activation_budget",
        suite="lifecycle_metadata",
        ok=isinstance(activation.get("budget_ms"), int) and activation.get("budget_ms") > 0,
        message="activation budget is declared in milliseconds",
        field="lifecycle.activation.budget_ms",
        fix="Set a positive activation budget in milliseconds.",
    )
    add(
        findings,
        check_id="lifecycle_metadata.degraded_path",
        suite="lifecycle_metadata",
        ok=degraded_path.get("behavior_class") in DEGRADED_BEHAVIORS
        and degraded_path.get("preserves_core_editing") is True,
        message="degraded path preserves core editing and has a typed behavior",
        field="lifecycle.degraded_path",
        fix="Declare a typed degraded behavior that preserves local editing.",
    )
    add(
        findings,
        check_id="lifecycle_metadata.disable_support",
        suite="lifecycle_metadata",
        ok=disable.get("supported") is True and disable.get("preserves_user_state") is True,
        message="disable behavior is supported and preserves user state",
        field="lifecycle.disable",
        fix="Declare disable.supported and disable.preserves_user_state as true.",
    )
    add(
        findings,
        check_id="lifecycle_metadata.rollback_support",
        suite="lifecycle_metadata",
        ok=rollback.get("supported") is True and non_empty_string(rollback.get("last_known_good_ref")),
        message="rollback behavior names a last-known-good target",
        field="lifecycle.rollback",
        fix="Declare rollback.supported and rollback.last_known_good_ref.",
    )

    conformance = ensure_obj(manifest.get("conformance"), "conformance", findings)
    fixture_rows = as_list(conformance.get("fixtures"))
    observed_scenarios = {
        row.get("scenario_class")
        for row in fixture_rows
        if isinstance(row, dict) and isinstance(row.get("scenario_class"), str)
    }
    add(
        findings,
        check_id="conformance_fixtures.required_scenario_coverage",
        suite="conformance_fixtures",
        ok=SCENARIO_CLASSES.issubset(observed_scenarios),
        message="conformance fixtures cover install, activation, permission prompts, degraded paths, and disable or rollback",
        field="conformance.fixtures",
        fix="Add fixture rows for every required scenario class.",
    )
    for idx, row_raw in enumerate(fixture_rows):
        row = row_raw if isinstance(row_raw, dict) else {}
        scenario = row.get("scenario_class")
        field = f"conformance.fixtures[{idx}]"
        add(
            findings,
            check_id="conformance_fixtures.scenario_class_known",
            suite="conformance_fixtures",
            ok=scenario in SCENARIO_CLASSES,
            message=f"{field}.scenario_class uses the conformance scenario vocabulary",
            field=f"{field}.scenario_class",
            fix="Use one of: " + ", ".join(sorted(SCENARIO_CLASSES)),
        )
        add(
            findings,
            check_id="conformance_fixtures.fixture_ref",
            suite="conformance_fixtures",
            ok=non_empty_string(row.get("fixture_ref")),
            message=f"{field}.fixture_ref is non-empty",
            field=f"{field}.fixture_ref",
            fix="Point the row at a replayable fixture or evidence ref.",
        )

    failed = [finding for finding in findings if finding.status == "fail"]
    warnings = [finding for finding in findings if finding.status == "warn"]
    result_class = "fail" if any(f.severity == "blocker" for f in failed) else "pass"
    if result_class == "pass" and warnings:
        result_class = "warn"
    compatibility_badge = (
        "compatible_on_declared_targets"
        if result_class == "pass"
        else "unsupported_pending_qualification"
    )
    red_flag_classes = sorted(
        {
            flag
            for finding in failed
            for flag in classify_red_flags(finding.check_id)
        }
    )

    return {
        "record_kind": "extension_conformance_report",
        "conformance_kit_report_schema_version": REPORT_SCHEMA_VERSION,
        "validator_id": VALIDATOR_ID,
        "validator_version": VALIDATOR_VERSION,
        "generated_at": generated_at,
        "subject_manifest_ref": subject_manifest_ref,
        "package_id": package_id if isinstance(package_id, str) else None,
        "publisher_id": publisher_id if isinstance(publisher_id, str) else None,
        "extension_version": extension_version if isinstance(extension_version, str) else None,
        "runtime_origin": runtime_origin if isinstance(runtime_origin, str) else None,
        "host_contract_family": host_family if isinstance(host_family, str) else None,
        "result_class": result_class,
        "compatibility_badge_class": compatibility_badge,
        "red_flag_classes": red_flag_classes,
        "summary": {
            "passed": sum(1 for finding in findings if finding.status == "pass"),
            "failed": len(failed),
            "warnings": len(warnings),
            "blockers": sum(1 for finding in failed if finding.severity == "blocker"),
        },
        "checks": [finding.as_report() for finding in findings],
    }


def classify_red_flags(check_id: str) -> set[str]:
    flags: set[str] = set()
    if check_id.startswith("manifest_shape.required") or check_id.startswith("manifest_shape.schema"):
        flags.add("missing_manifest_shape")
    if check_id == "manifest_shape.publisher_identity":
        flags.add("opaque_publisher_identity")
    if check_id.startswith("permission_declarations."):
        flags.add("undeclared_privileged_permission")
    if check_id.startswith("lifecycle_metadata."):
        flags.add("missing_lifecycle_metadata")
    if check_id.startswith("compatibility_targets."):
        flags.add("incompatible_sdk_target")
    if check_id in {
        "lifecycle_metadata.disable_support",
        "lifecycle_metadata.rollback_support",
    }:
        flags.add("missing_disable_or_rollback")
    return flags & RED_FLAGS


def validate_suite(
    suite_path: Path,
    *,
    repo_root: Path,
) -> dict[str, Any]:
    suite = load_json(suite_path)
    if not isinstance(suite, dict):
        raise SystemExit(f"{suite_path} must contain a JSON object")
    generated_at = suite.get("generated_at")
    if not non_empty_string(generated_at):
        raise SystemExit(f"{suite_path}: generated_at must be a non-empty timestamp")
    cases = suite.get("cases")
    if not isinstance(cases, list) or not cases:
        raise SystemExit(f"{suite_path}: cases must be a non-empty array")

    suite_dir = suite_path.parent
    case_results: list[dict[str, Any]] = []
    aggregate_scenarios: set[str] = set()
    unexpected: list[str] = []
    blocker_count = 0

    for idx, case in enumerate(cases):
        if not isinstance(case, dict):
            raise SystemExit(f"{suite_path}: cases[{idx}] must be an object")
        case_id = case.get("case_id")
        manifest_rel = case.get("manifest")
        expected = case.get("expected_result_class")
        if not non_empty_string(case_id) or not non_empty_string(manifest_rel):
            raise SystemExit(f"{suite_path}: cases[{idx}] requires case_id and manifest")
        if expected not in {"pass", "warn", "fail"}:
            raise SystemExit(
                f"{suite_path}: cases[{idx}].expected_result_class must be pass, warn, or fail"
            )
        manifest_path = (suite_dir / manifest_rel).resolve()
        manifest_payload = load_json(manifest_path)
        report = validate_manifest_payload(
            manifest_payload,
            subject_manifest_ref=normalize_rel(manifest_path, repo_root),
            generated_at=generated_at,
        )
        observed = report["result_class"]
        matched = observed == expected
        if not matched:
            unexpected.append(f"{case_id}: expected {expected}, observed {observed}")
        blocker_count += int(report["summary"]["blockers"])
        conformance = manifest_payload.get("conformance") if isinstance(manifest_payload, dict) else {}
        for row in as_list(conformance.get("fixtures") if isinstance(conformance, dict) else None):
            if isinstance(row, dict) and isinstance(row.get("scenario_class"), str):
                aggregate_scenarios.add(row["scenario_class"])
        case_results.append(
            {
                "case_id": case_id,
                "manifest_ref": normalize_rel(manifest_path, repo_root),
                "expected_result_class": expected,
                "observed_result_class": observed,
                "matched_expectation": matched,
                "blocker_count": report["summary"]["blockers"],
                "failed_check_count": report["summary"]["failed"],
                "compatibility_badge_class": report["compatibility_badge_class"],
                "red_flag_classes": report["red_flag_classes"],
            }
        )

    missing_scenarios = sorted(SCENARIO_CLASSES - aggregate_scenarios)
    if missing_scenarios:
        unexpected.append(
            "suite missing required scenario coverage: " + ", ".join(missing_scenarios)
        )

    return {
        "record_kind": "extension_conformance_suite_report",
        "conformance_kit_report_schema_version": REPORT_SCHEMA_VERSION,
        "validator_id": VALIDATOR_ID,
        "validator_version": VALIDATOR_VERSION,
        "suite_id": suite.get("suite_id"),
        "generated_at": generated_at,
        "suite_manifest_ref": normalize_rel(suite_path, repo_root),
        "suite_result_class": "fail" if unexpected else "pass",
        "required_scenario_classes": sorted(SCENARIO_CLASSES),
        "observed_scenario_classes": sorted(aggregate_scenarios),
        "case_count": len(case_results),
        "case_results": case_results,
        "unexpected_results": unexpected,
        "aggregate_blocker_count": blocker_count,
    }


def render_report(report: dict[str, Any]) -> str:
    return json.dumps(report, indent=2, sort_keys=True) + "\n"


def command_validate_manifest(args: argparse.Namespace) -> int:
    repo_root = Path(args.repo_root).resolve()
    manifest_path = Path(args.manifest).resolve()
    generated_at = args.generated_at or "2026-05-16T18:00:00Z"
    payload = load_json(manifest_path)
    report = validate_manifest_payload(
        payload,
        subject_manifest_ref=normalize_rel(manifest_path, repo_root),
        generated_at=generated_at,
    )
    if args.format == "json":
        sys.stdout.write(render_report(report))
    else:
        print(
            f"{report['subject_manifest_ref']}: {report['result_class']} "
            f"({report['summary']['failed']} failed checks)"
        )
        for check in report["checks"]:
            if check["status"] != "pass":
                field = f" [{check.get('field')}]" if check.get("field") else ""
                print(f"- {check['severity']} {check['check_id']}{field}: {check['message']}")
    return 0 if report["result_class"] in {"pass", "warn"} else 1


def command_validate_suite(args: argparse.Namespace) -> int:
    repo_root = Path(args.repo_root).resolve()
    suite_path = Path(args.suite).resolve()
    report = validate_suite(suite_path, repo_root=repo_root)
    rendered = render_report(report)
    if args.report:
        report_path = Path(args.report).resolve()
        if args.check:
            if not report_path.exists():
                print(f"[extension-validator] missing report: {report_path}", file=sys.stderr)
                return 1
            existing = report_path.read_text(encoding="utf-8")
            if existing != rendered:
                print(
                    f"[extension-validator] report drift detected: {report_path}",
                    file=sys.stderr,
                )
                return 1
        else:
            report_path.parent.mkdir(parents=True, exist_ok=True)
            report_path.write_text(rendered, encoding="utf-8")
    else:
        sys.stdout.write(rendered)
    return 0 if report["suite_result_class"] == "pass" else 1


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", default=".")
    sub = parser.add_subparsers(dest="command", required=True)

    one = sub.add_parser("validate-manifest", help="validate one extension manifest")
    one.add_argument("--manifest", required=True)
    one.add_argument("--generated-at")
    one.add_argument("--format", choices=["json", "text"], default="json")
    one.set_defaults(func=command_validate_manifest)

    suite = sub.add_parser("validate-suite", help="validate a conformance fixture suite")
    suite.add_argument("--suite", required=True)
    suite.add_argument("--report")
    suite.add_argument(
        "--check",
        action="store_true",
        help="fail when --report does not match the generated suite report",
    )
    suite.set_defaults(func=command_validate_suite)
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    return args.func(args)


if __name__ == "__main__":
    raise SystemExit(main())
